//! Free gradient ascent in R^{4F} on general polytopes.
//!
//! Goal: Run unconstrained gradient ascent on general random polytopes and
//! record both the summary outcomes and per-step traces.
//! Input Artifacts: experiments/sys-landscape/cache.jsonl
//! Output Artifacts: experiments/sys-landscape/cache.jsonl
//!         experiments/sys-landscape/gradient-ascent-general/gradient-ascent-general.jsonl
//!         experiments/sys-landscape/gradient-ascent-general/gradient-ascent-general-trace.jsonl
//!
//! At each iteration, builds the active-orbit first-order model for `sys`.
//! With one active orbit, uses that branch gradient directly; at switching
//! points, chooses a maximin ascent direction from the active gradient family.
//! It then steps directly in a-space: a_k(t) = a_k + t * d_k.
//! Boundary-crossing via overshoot (multiples of t_max) and wiggle (random
//! perturbation of dual vertices).
//!
//! Predecessor: boundary-crossing-search (split into gradient-ascent-general
//! and gradient-ascent-products, 2026-04-04).
//!
//! CLI (all optional):
//! - `--n <count>`        number of seeds this invocation processes   (default: 10)
//! - `--n-start <offset>` starting global seed index                  (default: 0)
//! - `--seed <u64>`       base RNG seed                               (default: 42)
//! - `--out <path>`       output summary .jsonl                       (default: untracked temp smoke path)
//! - `--fresh`            delete existing summary + trace files before running
//! - `--db-update`        load and save the sys-landscape family cache
//! - `--no-db-update`     do not load or save the sys-landscape family cache
//!                        (set by LICCA to avoid concurrent write races)
//!
//! Architecture B (2026-04-12): rayon `par_iter` over `[n_start, n_start+n)`
//! at the dataset level. Seed i uses its own RNG stream
//! `ChaCha8Rng::seed_from_u64(seed + i)`, so the output for index i is
//! byte-reproducible regardless of thread assignment. Shared CLI / writer /
//! resume plumbing lives in `exp_sys_landscape::{parse_ascent_args,
//! open_ascent_writers, run_parallel_seeds, ...}`.

use exp_sys_landscape::{
    apply_dual_step, ascent_direction, compute_active_sys_state, compute_step_bound, compute_sys,
    dual_vertices_rational_strings, finalize_ascent_output, open_ascent_writers,
    orbit_scalars_from_result, parse_ascent_args, run_parallel_seeds, smoke_output_path,
    trace_path_for, AscentArgs, AscentMode, SeedResult, SummaryRow, TraceRow, MAX_STEP_SIZE,
};
use nalgebra::Vector4;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use rand_distr::{Distribution, StandardNormal};
use std::collections::HashMap;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use symplectic::database::{load_many, save, DualVerticesKey, PolytopeRecord, SigmaAction};
use symplectic::geom::polytope::Polytope4D;
use symplectic::random::sample_random_polytope;

// ============================================================================
// Configuration
// ============================================================================

const DEFAULT_SEED: u64 = 42;

/// Facet count for fresh polytopes.
const FACET_COUNT: usize = 10;

/// Height range for random generation.
const H_MIN: f64 = 0.8;
const H_MAX: f64 = 1.2;

/// Maximum attempts per seed index to generate a valid polytope before giving up
/// on that index. Each retry draws new numbers from the same per-seed RNG stream,
/// so output for a given global index is still byte-reproducible.
const MAX_POLYTOPE_ATTEMPTS: usize = 100;

/// Maximum gradient ascent iterations per phase.
const MAX_ITERATIONS: usize = 30;

/// Minimum improvement per iteration to continue. Well above f64 noise
/// (~1e-15) but small enough to capture meaningful steps. Matches
/// gradient-descent. If changed, re-check convergence rates in trace data.
const CONVERGENCE_THRESHOLD: f64 = 1e-6;

/// Step fractions of t_max for within-bound line search.
const STEP_FRACTIONS: &[f64] = &[0.1, 0.25, 0.5, 0.75, 0.95];

/// Multipliers beyond t_max for crossing combinatorial boundaries.
/// Land in neighboring combinatorial cells.
const OVERSHOOT_MULTIPLIERS: &[f64] = &[1.5, 2.0, 3.0];

/// Number of random dual-vertex perturbations per escape round.
const N_WIGGLES: usize = 5;

/// Multiplicative perturbation scale for dual vertex components: a_k[i] -> a_k[i] * (1 + 0.05 * N(0,1)).
/// Per-facet displacement has expected norm ~0.05 * |a_k| (unit-scale dual vertices: ~0.05).
/// Cell-widths data (experiments/combinatorial-cells/cell-widths/): median non-orbit cell
/// width = 0.124, median
/// orbit cell width = 0.258. So per-facet displacement ~0.05 is ~40% of the narrowest
/// median cell width. With F=10 facets all perturbed simultaneously, boundary crossing
/// is highly likely — confirmed by data: wiggle dominated overshoot as escape strategy
/// (41/42 seeds, experiments/sys-landscape/gradient-ascent-general/job.sh).
/// If changed: much smaller (e.g. 0.01) reduces boundary-crossing probability and escape
/// effectiveness. Much larger (e.g. 0.2) risks producing degenerate polytopes
/// (Polytope4D::from_f64 failure) or landing too far from the current optimum.
const WIGGLE_STRENGTH: f64 = 0.05;

/// Maximum rounds of escape attempts after convergence.
const MAX_ESCAPE_ROUNDS: usize = 3;

/// Per-seed time budget. 120s is generous: most seeds converge in <30s, but
/// F=10 with multiple escape rounds can take longer.
const SEED_TIME_BUDGET_SECS: f64 = 120.0;

/// Numerical zero threshold for gradient norms, rates, and slack comparisons.
/// Near machine epsilon for unit-scale f64. Matches gradient-descent.
const EPS: f64 = 1e-15;

// ============================================================================
// Gradient step in a-space
// ============================================================================

// ============================================================================
// Gradient ascent with integrated overshoot
// ============================================================================

struct AscentResult {
    final_polytope: Polytope4D,
    final_sys: f64,
    n_iters: usize,
    n_overshoot_improvements: usize,
    trace: Vec<TraceRow>,
}

/// Ascent in dual-vertex space with overshoot at every iteration.
///
/// At each step:
/// 1. Builds the active-orbit first-order model of `sys`
/// 2. Tries STEP_FRACTIONS of t_max (within cell) and OVERSHOOT_MULTIPLIERS (crosses boundary)
/// 3. Picks the candidate with highest sys
// TODO: add [lem:sys-sensitivity] to formal math (see gradient-correctness experiment)
fn gradient_ascent(
    name: &str,
    phase: usize,
    start: &Polytope4D,
    t0: Instant,
    budget: f64,
) -> Option<AscentResult> {
    let mut current = Polytope4D::from_f64(start.dual_vertices_f64().to_vec()).ok()?;

    let sys_init = compute_sys(&current)?;

    let mut current_sys = sys_init;
    let mut n_iters = 0usize;
    let mut n_overshoot = 0usize;
    let mut trace = Vec::new();

    for iter in 0..MAX_ITERATIONS {
        if t0.elapsed().as_secs_f64() > budget {
            break;
        }

        // 1. Shared local state
        let state = compute_active_sys_state(&current)?;
        let sys = state.sys;
        let duals = current.dual_vertices_f64();

        // 2. Ascent direction: single branch when unique, nonsmooth maximin
        // direction when several active orbit branches tie.
        let d_sys_a = ascent_direction(&current, &state, AscentMode::General)?;

        let gradient_norm = d_sys_a.iter().map(|d| d.norm_squared()).sum::<f64>().sqrt();
        if gradient_norm < EPS {
            break;
        }

        // 3. Step bound
        let t_max = compute_step_bound(&current, &d_sys_a);
        if t_max <= 0.0 {
            break;
        }

        // 4. Line search: within-bound + overshoot
        let mut best: Option<(Polytope4D, f64, String, f64, f64)> = None;

        for &frac in STEP_FRACTIONS {
            let t = frac * t_max;
            if let Some((p, new_sys)) = apply_dual_step(duals, &d_sys_a, t) {
                if new_sys > sys && best.as_ref().is_none_or(|b| new_sys > b.1) {
                    best = Some((p, new_sys, "within".into(), frac, t));
                }
            }
        }

        if t_max < MAX_STEP_SIZE {
            for &mult in OVERSHOOT_MULTIPLIERS {
                let t = mult * t_max;
                if let Some((p, new_sys)) = apply_dual_step(duals, &d_sys_a, t) {
                    if new_sys > sys && best.as_ref().is_none_or(|b| new_sys > b.1) {
                        best = Some((p, new_sys, format!("overshoot_{mult}x"), mult, t));
                    }
                }
            }
        }

        // 5. Take best step or stop
        match best {
            Some((new_polytope, new_sys, step_type, frac, t)) => {
                let delta = new_sys - sys;
                if step_type.starts_with("overshoot") {
                    n_overshoot += 1;
                }

                trace.push(TraceRow {
                    name: name.to_string(),
                    phase,
                    iteration: iter,
                    step_type,
                    t_fraction: frac,
                    t_actual: t,
                    sys_before: sys,
                    sys_after: new_sys,
                    delta_sys: delta,
                    gradient_norm,
                });

                current = new_polytope;
                current_sys = new_sys;
                n_iters = iter + 1;

                if delta < CONVERGENCE_THRESHOLD {
                    break;
                }
            }
            None => break,
        }
    }

    Some(AscentResult {
        final_polytope: current,
        final_sys: current_sys,
        n_iters,
        n_overshoot_improvements: n_overshoot,
        trace,
    })
}

// ============================================================================
// Escape via wiggle
// ============================================================================

/// Perturb dual vertices by Gaussian noise to escape local optimum.
/// Returns None if the perturbed dual vertices don't form a valid polytope.
fn wiggle(polytope: &Polytope4D, rng: &mut ChaCha8Rng) -> Option<Polytope4D> {
    let duals = polytope.dual_vertices_f64();
    let new_duals: Vec<Vector4<f64>> = duals
        .iter()
        .map(|a| {
            a.map(|c| {
                let noise: f64 = StandardNormal.sample(rng);
                c * (1.0 + WIGGLE_STRENGTH * noise)
            })
        })
        .collect();
    Polytope4D::from_f64(new_duals).ok()
}

// ============================================================================
// Per-seed processing
// ============================================================================

fn process_seed(
    name: &str,
    seed_index: usize,
    polytope_type: &str,
    polytope: &Polytope4D,
    rng: &mut ChaCha8Rng,
) -> Option<SeedResult> {
    let t0 = Instant::now();
    let budget = SEED_TIME_BUDGET_SECS;

    let starting_sys = compute_sys(polytope)?;

    let mut best_polytope = Polytope4D::from_f64(polytope.dual_vertices_f64().to_vec()).ok()?;
    let mut best_sys = starting_sys;
    let mut n_phases = 0usize;
    let mut n_iters_total = 0usize;
    let mut n_escape_overshoot = 0usize;
    let mut n_escape_wiggle = 0usize;
    let mut best_strategy = "none".to_string();
    let mut all_trace = Vec::new();

    // Phase 0: initial gradient ascent (with overshoot at each step)
    if let Some(result) = gradient_ascent(name, n_phases, polytope, t0, budget) {
        n_phases += 1;
        n_iters_total += result.n_iters;
        n_escape_overshoot += result.n_overshoot_improvements;
        all_trace.extend(result.trace);

        if result.final_sys > best_sys {
            best_sys = result.final_sys;
            best_polytope = result.final_polytope;
            best_strategy = if result.n_overshoot_improvements > 0 {
                "overshoot".to_string()
            } else {
                "within_cell".to_string()
            };
        }
    }

    // Escape rounds: wiggle + re-ascent
    for _round in 0..MAX_ESCAPE_ROUNDS {
        if t0.elapsed().as_secs_f64() > budget {
            break;
        }
        let mut escaped = false;

        for _ in 0..N_WIGGLES {
            if t0.elapsed().as_secs_f64() > budget {
                break;
            }
            if let Some(wiggled) = wiggle(&best_polytope, rng) {
                if let Some(result) = gradient_ascent(name, n_phases, &wiggled, t0, budget) {
                    n_phases += 1;
                    n_iters_total += result.n_iters;
                    n_escape_overshoot += result.n_overshoot_improvements;
                    all_trace.extend(result.trace);

                    if result.final_sys > best_sys + CONVERGENCE_THRESHOLD {
                        best_sys = result.final_sys;
                        best_polytope = result.final_polytope;
                        n_escape_wiggle += 1;
                        best_strategy = "wiggle".to_string();
                        escaped = true;
                        break;
                    }
                }
            }
        }
        if !escaped {
            break;
        }
    }

    let total_time_ms = t0.elapsed().as_secs_f64() * 1000.0;
    let final_state = compute_active_sys_state(&best_polytope)?;
    let final_capacity = final_state.capacity.capacity();
    let mut final_record = PolytopeRecord::from_polytope(&best_polytope);
    final_record = final_record.with_computed_fields(final_state.vol, 0.0, final_capacity, 0.0);
    final_record = final_record.with_sigmas(
        vec![SigmaAction {
            perm: final_state.capacity.best_sigma().to_vec(),
            action: final_capacity,
        }],
        0.0,
    );
    final_record =
        final_record.with_orbit_scalars(orbit_scalars_from_result(&final_state.capacity));
    let starting_dual_vertices_rational = dual_vertices_rational_strings(polytope);
    let final_dual_vertices_rational = dual_vertices_rational_strings(&best_polytope);
    let final_dvs: Vec<[f64; 4]> = best_polytope
        .dual_vertices_f64()
        .iter()
        .map(|a| [a[0], a[1], a[2], a[3]])
        .collect();

    Some(SeedResult {
        summary: SummaryRow {
            name: name.to_string(),
            seed_index,
            source_name: name.to_string(),
            lineage_id: format!("general::{name}"),
            polytope_type: polytope_type.to_string(),
            facet_count: best_polytope.facet_count(),
            starting_sys,
            final_sys: best_sys,
            total_delta: best_sys - starting_sys,
            n_ascent_phases: n_phases,
            n_gradient_iters_total: n_iters_total,
            n_escape_overshoot,
            n_escape_wiggle,
            best_strategy,
            total_time_ms,
            starting_dual_vertices_rational,
            final_dual_vertices_rational,
            final_dual_vertices: final_dvs,
        },
        trace: all_trace,
        final_record,
        final_polytope: best_polytope,
    })
}

// ============================================================================
// Polytope database helper
// ============================================================================

/// Insert a polytope into the database if not already present.
/// Stores rational geometry for future vertex-enumeration-free reconstruction.
fn insert_polytope_to_db(db: &mut HashMap<DualVerticesKey, PolytopeRecord>, polytope: &Polytope4D) {
    let key: DualVerticesKey = polytope.dual_vertices().to_vec();
    if db.contains_key(&key) {
        return;
    }
    let record = PolytopeRecord::from_polytope(polytope);
    db.insert(key, record);
}

// ============================================================================
// Main
// ============================================================================

fn main() {
    let default_out = smoke_output_path(
        "sys-gradient-ascent-general",
        "smoke-gradient-ascent-general.jsonl",
    );
    let args: AscentArgs = parse_ascent_args(DEFAULT_SEED, 10, default_out, "general");
    let t_global = Instant::now();

    let summary_path = args.out.clone();
    let trace_path = trace_path_for(&summary_path);

    println!("gradient-ascent-general: free gradient ascent on general polytopes");
    println!("  n:            {}", args.n);
    println!("  n-start:      {}", args.n_start);
    println!("  seed:         {}", args.seed);
    println!("  out:          {}", summary_path.display());
    println!("  trace:        {}", trace_path.display());
    println!("  fresh:        {}", args.fresh);
    println!("  no-db-update: {}\n", args.no_db_update);

    let completed = if args.fresh {
        std::collections::HashSet::new()
    } else {
        exp_sys_landscape::load_completed_names(&summary_path)
    };

    if completed.is_empty() {
        println!("Starting fresh run.");
    } else {
        println!("Resuming: {} seeds already completed.", completed.len());
    }

    let writers = open_ascent_writers(&summary_path, &trace_path, args.fresh);
    let best = Arc::new(Mutex::new((0.0f64, String::new())));

    // DB state: loaded once, shared across threads under a Mutex when !no_db_update.
    // On LICCA (--no-db-update), both load and insertion are skipped entirely.
    let family_cache_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("cache.jsonl");
    let db_arc: Arc<Mutex<HashMap<DualVerticesKey, PolytopeRecord>>> = if args.no_db_update {
        Arc::new(Mutex::new(HashMap::new()))
    } else {
        let db = load_many(&[family_cache_path.as_path()])
            .expect("failed to load sys-landscape family cache");
        println!("Loaded family cache: {} entries", db.len());
        Arc::new(Mutex::new(db))
    };

    let no_db_update = args.no_db_update;
    let db_for_closure = Arc::clone(&db_arc);

    run_parallel_seeds(&args, &completed, &writers, &best, move |i, seed_i| {
        let mut rng_i = ChaCha8Rng::seed_from_u64(seed_i);

        let mut polytope_opt: Option<Polytope4D> = None;
        for _ in 0..MAX_POLYTOPE_ATTEMPTS {
            if let Ok(p) = sample_random_polytope(FACET_COUNT, H_MIN, H_MAX, &mut rng_i) {
                polytope_opt = Some(p);
                break;
            }
        }
        let polytope = match polytope_opt {
            Some(p) => p,
            None => {
                eprintln!(
                    "WARNING: seed {i}: no valid polytope after {MAX_POLYTOPE_ATTEMPTS} attempts, skipping"
                );
                return None;
            }
        };

        if !no_db_update {
            let mut db = db_for_closure.lock().expect("lock db for insert");
            insert_polytope_to_db(&mut db, &polytope);
        }

        let name = format!("general_{i}");
        let result = process_seed(&name, i, "general", &polytope, &mut rng_i)?;

        if !no_db_update {
            let mut db = db_for_closure.lock().expect("lock db for final insert");
            let key = result.final_record.key();
            db.entry(key)
                .and_modify(|record| {
                    if record.volume.is_none() {
                        record.volume = result.final_record.volume;
                    }
                    if record.volume_err.is_none() {
                        record.volume_err = result.final_record.volume_err;
                    }
                    if record.capacity.is_none() {
                        record.capacity = result.final_record.capacity;
                    }
                    if record.capacity_err.is_none() {
                        record.capacity_err = result.final_record.capacity_err;
                    }
                    if record.sigma_gap_cutoff.is_none() {
                        record.sigma_gap_cutoff = result.final_record.sigma_gap_cutoff;
                    }
                    if record.sigmas.is_none() {
                        record.sigmas = result.final_record.sigmas.clone();
                    }
                    if record.orbit_scalars.is_none() {
                        record.orbit_scalars = result.final_record.orbit_scalars.clone();
                    }
                })
                .or_insert_with(|| result.final_record.clone());
        }

        // Per-seed progress print from inside the closure. Writing to stdout
        // via println! is thread-safe (line-buffered, each call flushes its
        // own line), so two threads cannot interleave within a single line.
        let s = &result.summary;
        println!(
            "[seed {i}] {name}: sys: {:.4}->{:.4} (d={:.4}), strategy={}, phases={}, {:.1}s",
            s.starting_sys,
            s.final_sys,
            s.total_delta,
            s.best_strategy,
            s.n_ascent_phases,
            s.total_time_ms / 1000.0,
        );
        if s.final_sys > 1.0 {
            eprintln!(
                "*** VITERBO VIOLATION: {} sys={:.6} ***",
                s.name, s.final_sys
            );
        }

        Some(result)
    });

    // Drop writers (consumed by finalize), sort + rewrite both output files
    // so row order is deterministic regardless of rayon thread scheduling and
    // any crash-resume history. See `finalize_ascent_output` for details.
    finalize_ascent_output(&summary_path, &trace_path, writers);

    if !no_db_update {
        let db = db_arc.lock().expect("lock db for save");
        save(&family_cache_path, &db).expect("failed to save sys-landscape family cache");
    }

    let (best_sys, best_name) = {
        let b = best.lock().expect("lock best for report");
        (b.0, b.1.clone())
    };

    let end = args.n_start + args.n;
    println!("\n========================================");
    println!("Processed indices: {}..{end}", args.n_start);
    println!("Best sys: {best_sys:.6} ({best_name})");
    if !no_db_update {
        let db = db_arc.lock().expect("lock db for count");
        println!(
            "Cache: {} entries ({})",
            db.len(),
            family_cache_path.display()
        );
    }
    println!("Total time: {:.1}s", t_global.elapsed().as_secs_f64());
    println!("Output: {}", summary_path.display());
    println!("Trace: {}", trace_path.display());
}

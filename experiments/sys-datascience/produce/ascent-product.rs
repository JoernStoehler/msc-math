//! Dataset producer: fixed-`F` ascent on the Lagrangian-product submanifold.
//!
//! Goal: Run projected gradient ascent on random Lagrangian products and
//! record both the summary outcomes and per-step traces.
//! Input Artifacts: experiments/sys-datascience/produce/shared-cache.jsonl
//! Output Artifacts: experiments/sys-datascience/produce/shared-cache.jsonl
//!         experiments/sys-datascience/produce/ascent-product-endpoints.jsonl
//!         experiments/sys-datascience/produce/ascent-product-trace.jsonl
//!
//! At each iteration, builds the active-orbit first-order model for `sys`.
//! With one active orbit, it uses that branch gradient directly; at switching
//! points, it chooses a maximin ascent direction under LP-preserving coordinate
//! bounds. It then steps directly in a-space: a_k(t) = a_k + t * d_k.
//! Boundary-crossing via overshoot (multiples of t_max) and wiggle (random
//! perturbation of dual vertices).
//!
//! Predecessor: boundary-crossing-search (split into gradient-ascent-general
//! and gradient-ascent-products, 2026-04-04).
//!
//! CLI (all optional):
//! - `--n <count>`        number of seeds this invocation processes   (default: 12)
//! - `--n-start <offset>` starting global seed index                  (default: 0)
//! - `--seed <u64>`       base RNG seed                               (default: 42)
//! - `--out <path>`       output endpoint summary .jsonl                       (default: untracked temp smoke path)
//! - `--seed-time-budget-secs <f64>` per-seed wall-clock budget       (default: 120)
//! - `--fresh`            delete existing endpoint summary + trace + cache + computed-polytope files before running
//! - `--db-update`        load and save the sys-landscape family cache
//! - `--no-db-update`     do not load or save the sys-landscape family cache
//!                        (set by LICCA shards to avoid concurrent write races)
//!
//! Canonical refresh example:
//! `cargo run -p exp-sys-landscape --release --bin sys-dataset-ascent-product -- --out experiments/sys-datascience/produce/ascent-product-endpoints.jsonl --db-update`
//!
//! Architecture B (2026-04-12): rayon `par_iter` over `[n_start, n_start+n)`
//! at the dataset level. Seed i uses its own RNG stream
//! `ChaCha8Rng::seed_from_u64(seed + i)`, is named `ascent_product_{i}`, and has
//! bucket `i mod LAGRANGIAN_SPLITS.len()`. The output for index i is
//! byte-reproducible regardless of thread assignment. Shared CLI / writer /
//! resume plumbing lives in `exp_sys_landscape::{parse_ascent_args,
//! open_ascent_writers, run_parallel_seeds, ...}`.

use exp_sys_landscape::SysLandscapePolytopeCache;
use exp_sys_landscape::{
    apply_dual_step_with_cached_computation, ascent_direction, compute_active_sys_state_cached,
    compute_step_bound, compute_sys_computation_cached, dual_vertices_rational_strings,
    finalize_ascent_output, open_ascent_writers, orbit_scalars_from_result, parse_ascent_args,
    polytope_key, run_parallel_seeds, shared_family_cache_path, smoke_output_path,
    write_expensive_computation_cache_rows, AscentArgs, AscentEventRow, AscentMode,
    AscentOutputPaths, ComputedPolytopeMeta, ComputedPolytopeRecorder, ExpensiveComputationCache,
    SeedResult, SummaryRow, TraceRow, MAX_STEP_SIZE,
};
use nalgebra::Vector4;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use rand_distr::{Distribution, StandardNormal};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use symplectic::algorithms::billiard::facet_classification::FacetClassification;
use symplectic::classify_facets_from_dual_vertices;
use symplectic::database::{load_many, save, DualVerticesKey, PolytopeRecord, SigmaAction};
use symplectic::geom::polygon::random_polygon_2d;

// ============================================================================
// Configuration
// ============================================================================

const DEFAULT_SEED: u64 = 42;

/// Lagrangian product splits (q_facets, p_facets) summing to 10.
const LAGRANGIAN_SPLITS: &[(usize, usize)] = &[(3, 7), (4, 6), (5, 5)];

/// Height range for random generation.
const H_MIN: f64 = 0.8;
const H_MAX: f64 = 1.2;

/// Maximum attempts per seed index to generate a valid Lagrangian product
/// before giving up on that index. Retries draw new numbers from the same
/// per-seed RNG stream, so output remains byte-reproducible.
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
/// Local cell-width probes and prior ascent runs suggest this scale is large
/// enough to cross boundaries often, while still small enough to keep most
/// perturbed polytopes constructible. See
/// `experiments/combinatorial-cells/README.md` and
/// `research/sys-landscape.md`.
/// If changed: much smaller (e.g. 0.01) reduces boundary-crossing probability and escape
/// effectiveness. Much larger (e.g. 0.2) risks producing degenerate polytopes
/// (SysLandscapePolytopeCache::from_f64_dual_vertices failure) or landing too far from the current optimum.
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
    final_polytope: SysLandscapePolytopeCache,
    final_sys: f64,
    n_iters: usize,
    n_overshoot_improvements: usize,
    trace: Vec<TraceRow>,
}

/// Ascent in dual-vertex space with overshoot at every iteration.
///
/// At each step:
/// 1. Builds the active-orbit first-order model of `sys`
/// 2. Enforces LP-preserving coordinate bounds on the ascent direction
/// 3. Tries STEP_FRACTIONS of t_max (within cell) and OVERSHOOT_MULTIPLIERS (crosses boundary)
/// 4. Picks the candidate with highest sys
// TODO: add [lem:sys-sensitivity] to formal math (see gradient-correctness experiment)
fn gradient_ascent(
    name: &str,
    seed_index: usize,
    phase: usize,
    start: &SysLandscapePolytopeCache,
    lagrangian_class: &FacetClassification,
    t0: Instant,
    budget: f64,
    initial_role: &str,
    computed_polytopes: &mut ComputedPolytopeRecorder,
    expensive_cache: &ExpensiveComputationCache,
) -> Option<AscentResult> {
    let mut current =
        SysLandscapePolytopeCache::from_f64_dual_vertices(start.dual_vertices_f64.to_vec())?;

    let initial = compute_sys_computation_cached(&current, expensive_cache)?;
    computed_polytopes.push(
        ComputedPolytopeMeta {
            phase: Some(phase),
            iteration: None,
            role: initial_role,
            ..ComputedPolytopeMeta::role(initial_role)
        },
        &current,
        &initial.capacity,
        initial.vol,
        initial.sys,
    );

    let mut current_sys = initial.sys;
    let mut n_iters = 0usize;
    let mut n_overshoot = 0usize;
    let mut trace = Vec::new();

    for iter in 0..MAX_ITERATIONS {
        if t0.elapsed().as_secs_f64() > budget {
            break;
        }

        // 1. Shared local state
        let state = compute_active_sys_state_cached(&current, expensive_cache)?;
        let sys = state.sys;
        let duals = &current.dual_vertices_f64;
        computed_polytopes.push(
            ComputedPolytopeMeta {
                phase: Some(phase),
                iteration: Some(iter),
                ..ComputedPolytopeMeta::role("current_state")
            },
            &current,
            &state.capacity,
            state.vol,
            state.sys,
        );

        // 2. Ascent direction with explicit LP-preserving coordinate bounds.
        let d_sys_a = ascent_direction(
            &current,
            &state,
            AscentMode::LagrangianProduct {
                classification: lagrangian_class,
            },
        )?;

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
        let mut best: Option<(SysLandscapePolytopeCache, f64, String, f64, f64)> = None;
        let mut best_result_idx: Option<usize> = None;

        for &frac in STEP_FRACTIONS {
            let t = frac * t_max;
            if let Some((p, computation)) =
                apply_dual_step_with_cached_computation(duals, &d_sys_a, t, expensive_cache)
            {
                let result_idx = computed_polytopes.push(
                    ComputedPolytopeMeta {
                        phase: Some(phase),
                        iteration: Some(iter),
                        role: "line_search_candidate",
                        step_type: Some("within"),
                        t_fraction: Some(frac),
                        t_actual: Some(t),
                        accepted_in_iteration: false,
                        became_run_final: false,
                    },
                    &p,
                    &computation.capacity,
                    computation.vol,
                    computation.sys,
                );
                let new_sys = computation.sys;
                if new_sys > sys && best.as_ref().is_none_or(|b| new_sys > b.1) {
                    best = Some((p, new_sys, "within".into(), frac, t));
                    best_result_idx = Some(result_idx);
                }
            }
        }

        if t_max < MAX_STEP_SIZE {
            for &mult in OVERSHOOT_MULTIPLIERS {
                let t = mult * t_max;
                if let Some((p, computation)) =
                    apply_dual_step_with_cached_computation(duals, &d_sys_a, t, expensive_cache)
                {
                    let step_type = format!("overshoot_{mult}x");
                    let result_idx = computed_polytopes.push(
                        ComputedPolytopeMeta {
                            phase: Some(phase),
                            iteration: Some(iter),
                            role: "line_search_candidate",
                            step_type: Some(&step_type),
                            t_fraction: Some(mult),
                            t_actual: Some(t),
                            accepted_in_iteration: false,
                            became_run_final: false,
                        },
                        &p,
                        &computation.capacity,
                        computation.vol,
                        computation.sys,
                    );
                    let new_sys = computation.sys;
                    if new_sys > sys && best.as_ref().is_none_or(|b| new_sys > b.1) {
                        best = Some((p, new_sys, step_type, mult, t));
                        best_result_idx = Some(result_idx);
                    }
                }
            }
        }

        // 5. Take best step or stop
        match best {
            Some((new_polytope, new_sys, step_type, frac, t)) => {
                if let Some(result_idx) = best_result_idx {
                    computed_polytopes.mark_accepted(result_idx);
                }
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
fn wiggle(
    polytope: &SysLandscapePolytopeCache,
    rng: &mut ChaCha8Rng,
) -> Option<SysLandscapePolytopeCache> {
    let duals = &polytope.dual_vertices_f64;
    let new_duals: Vec<Vector4<f64>> = duals
        .iter()
        .map(|a| {
            a.map(|c| {
                let noise: f64 = StandardNormal.sample(rng);
                c * (1.0 + WIGGLE_STRENGTH * noise)
            })
        })
        .collect();
    SysLandscapePolytopeCache::from_f64_dual_vertices(new_duals)
}

// ============================================================================
// Per-seed processing
// ============================================================================

fn process_seed(
    name: &str,
    seed_index: usize,
    polytope_type: &str,
    polytope: &SysLandscapePolytopeCache,
    lagrangian_class: &FacetClassification,
    seed_time_budget_secs: f64,
    rng: &mut ChaCha8Rng,
    expensive_cache: &ExpensiveComputationCache,
) -> Option<SeedResult> {
    let t0 = Instant::now();
    let budget = seed_time_budget_secs;

    let starting_computation = compute_sys_computation_cached(polytope, expensive_cache)?;
    let starting_sys = starting_computation.sys;
    let mut computed_polytopes =
        ComputedPolytopeRecorder::new("gradient_ascent_products", name, seed_index);
    computed_polytopes.push(
        ComputedPolytopeMeta::role("start"),
        polytope,
        &starting_computation.capacity,
        starting_computation.vol,
        starting_computation.sys,
    );

    let mut best_polytope =
        SysLandscapePolytopeCache::from_f64_dual_vertices(polytope.dual_vertices_f64.to_vec())?;
    let mut best_sys = starting_sys;
    let mut n_phases = 0usize;
    let mut n_iters_total = 0usize;
    let mut n_escape_overshoot = 0usize;
    let mut n_escape_wiggle = 0usize;
    let mut best_strategy = "none".to_string();
    let mut all_trace = Vec::new();

    // Phase 0: initial gradient ascent (with overshoot at each step)
    if let Some(result) = gradient_ascent(
        name,
        seed_index,
        n_phases,
        polytope,
        lagrangian_class,
        t0,
        budget,
        "current_state",
        &mut computed_polytopes,
        expensive_cache,
    ) {
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
                if let Some(result) = gradient_ascent(
                    name,
                    seed_index,
                    n_phases,
                    &wiggled,
                    lagrangian_class,
                    t0,
                    budget,
                    "wiggle_start",
                    &mut computed_polytopes,
                    expensive_cache,
                ) {
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
    let final_state = compute_active_sys_state_cached(&best_polytope, expensive_cache)?;
    let final_capacity = final_state.capacity.capacity();
    computed_polytopes.push(
        ComputedPolytopeMeta {
            became_run_final: true,
            ..ComputedPolytopeMeta::role("final")
        },
        &best_polytope,
        &final_state.capacity,
        final_state.vol,
        final_state.sys,
    );
    let mut final_record = best_polytope.to_record();
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
        .dual_vertices_f64
        .iter()
        .map(|a| [a[0], a[1], a[2], a[3]])
        .collect();

    let (computed_polytope_rows, mut ascent_event_rows) = computed_polytopes.into_outputs();
    let summary = SummaryRow {
        name: name.to_string(),
        seed_index,
        source_name: name.to_string(),
        lineage_id: format!("products::{name}"),
        polytope_type: polytope_type.to_string(),
        facet_count: best_polytope.facet_count(),
        starting_sys,
        final_capacity,
        final_volume: final_state.vol,
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
    };
    ascent_event_rows.push(AscentEventRow {
        event_id: format!("gradient_ascent_products:{name}:run_completed"),
        dataset: "gradient_ascent_products".to_string(),
        run_id: name.to_string(),
        seed_index,
        phase: None,
        iteration: None,
        role: "run_completed".to_string(),
        step_type: None,
        t_fraction: None,
        t_actual: None,
        accepted_in_iteration: false,
        became_run_final: true,
        polytope_key: polytope_key(&best_polytope),
        source_name: Some(summary.source_name.clone()),
        lineage_id: Some(summary.lineage_id.clone()),
        polytope_type: Some(summary.polytope_type.clone()),
        facet_count: Some(summary.facet_count),
        starting_sys: Some(summary.starting_sys),
        final_capacity: Some(summary.final_capacity),
        final_volume: Some(summary.final_volume),
        final_sys: Some(summary.final_sys),
        total_delta: Some(summary.total_delta),
        n_ascent_phases: Some(summary.n_ascent_phases),
        n_gradient_iters_total: Some(summary.n_gradient_iters_total),
        n_escape_overshoot: Some(summary.n_escape_overshoot),
        n_escape_wiggle: Some(summary.n_escape_wiggle),
        best_strategy: Some(summary.best_strategy.clone()),
        total_time_ms: Some(summary.total_time_ms),
    });
    Some(SeedResult {
        summary,
        trace: all_trace,
        computed_polytopes: computed_polytope_rows,
        ascent_events: ascent_event_rows,
        final_record,
        final_polytope: best_polytope,
    })
}

/// Generate a Lagrangian product for global seed index i from its own RNG
/// stream. Bucket (q_f, p_f) is determined by `i mod LAGRANGIAN_SPLITS.len()`,
/// so contiguous index ranges are evenly spread across buckets (10k total ->
/// ~3333 per bucket for a 3-way split).
fn generate_for_seed(
    i: usize,
    rng: &mut ChaCha8Rng,
) -> Option<(String, SysLandscapePolytopeCache)> {
    let bucket_idx = i % LAGRANGIAN_SPLITS.len();
    let (q_f, p_f) = LAGRANGIAN_SPLITS[bucket_idx];
    let bucket_name = format!("lagrangian_{q_f}x{p_f}");

    for _ in 0..MAX_POLYTOPE_ATTEMPTS {
        let (qn, qh) = random_polygon_2d(q_f, H_MIN, H_MAX, rng);
        let (pn, ph) = random_polygon_2d(p_f, H_MIN, H_MAX, rng);
        if let Some(p) = SysLandscapePolytopeCache::from_lagrangian_product(&qn, &qh, &pn, &ph) {
            return Some((bucket_name, p));
        }
    }
    None
}

// ============================================================================
// Main
// ============================================================================

/// Insert a polytope into the database if not already present.
/// Stores rational geometry for future vertex-enumeration-free reconstruction.
fn insert_polytope_to_db(
    db: &mut HashMap<DualVerticesKey, PolytopeRecord>,
    polytope: &SysLandscapePolytopeCache,
) {
    let key: DualVerticesKey = polytope.dual_vertices.to_vec();
    if db.contains_key(&key) {
        return;
    }
    let record = polytope.to_record();
    db.insert(key, record);
}

fn main() {
    let default_out = smoke_output_path(
        "sys-dataset-ascent-product",
        "smoke-ascent-product-endpoints.jsonl",
    );
    let args: AscentArgs = parse_ascent_args(
        DEFAULT_SEED,
        12,
        SEED_TIME_BUDGET_SECS,
        default_out,
        "ascent_product",
    );
    let t_global = Instant::now();

    let output_paths = AscentOutputPaths::from_summary_path(args.out.clone());

    println!("dataset-ascent-product: fixed-F ascent on Lagrangian products");
    println!("  n:            {}", args.n);
    println!("  n-start:      {}", args.n_start);
    println!("  seed:         {}", args.seed);
    println!("  out:          {}", output_paths.summary.display());
    println!("  trace:        {}", output_paths.trace.display());
    println!("  cache:        {}", output_paths.cache.display());
    println!(
        "  computed:     {}",
        output_paths.computed_polytopes.display()
    );
    println!("  events:       {}", output_paths.ascent_events.display());
    println!(
        "  expensive-cache-out: {}",
        output_paths.expensive_computations_cache.display()
    );
    for path in &args.expensive_computation_caches {
        println!("  expensive-cache-in:  {}", path.display());
    }
    println!("  fresh:        {}", args.fresh);
    println!("  budget:       {:.1}s/seed", args.seed_time_budget_secs);
    println!("  no-db-update: {}", args.no_db_update);
    println!("  buckets:      {LAGRANGIAN_SPLITS:?}\n");

    let completed = std::collections::HashSet::new();
    println!("Rerunning shard control flow; expensive computations use read-only cache hits.");

    let writers = open_ascent_writers(&output_paths, args.fresh);
    let best = Arc::new(Mutex::new((0.0f64, String::new())));
    let mut expensive_cache_inputs = args.expensive_computation_caches.clone();
    expensive_cache_inputs.push(output_paths.expensive_computations_cache.clone());
    let expensive_cache = Arc::new(ExpensiveComputationCache::load(&expensive_cache_inputs));

    // DB state: loaded once, shared across threads under a Mutex when !no_db_update.
    // On LICCA (--no-db-update), both load and insertion are skipped entirely.
    let family_cache_path = shared_family_cache_path();
    let db_arc: Arc<Mutex<HashMap<DualVerticesKey, PolytopeRecord>>> = if args.no_db_update {
        Arc::new(Mutex::new(HashMap::new()))
    } else {
        let db = load_many(&[family_cache_path.as_path()])
            .expect("failed to load sys-landscape family cache");
        println!("Loaded family cache: {} entries", db.len());
        Arc::new(Mutex::new(db))
    };

    let no_db_update = args.no_db_update;
    let seed_time_budget_secs = args.seed_time_budget_secs;
    let db_for_closure = Arc::clone(&db_arc);
    let expensive_cache_for_closure = Arc::clone(&expensive_cache);

    run_parallel_seeds(&args, &completed, &writers, &best, move |i, seed_i| {
        let mut rng_i = ChaCha8Rng::seed_from_u64(seed_i);

        let (bucket_name, polytope) = generate_for_seed(i, &mut rng_i)?;

        if !no_db_update {
            let mut db = db_for_closure.lock().expect("lock db for insert");
            insert_polytope_to_db(&mut db, &polytope);
        }

        let class = classify_facets_from_dual_vertices(&polytope.dual_vertices_f64)
            .expect("should classify as Lagrangian");

        let name = format!("ascent_product_{i}");
        let result = process_seed(
            &name,
            i,
            &bucket_name,
            &polytope,
            &class,
            seed_time_budget_secs,
            &mut rng_i,
            &expensive_cache_for_closure,
        )?;

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
            "[seed {i}] {name} ({bucket_name}): sys: {:.4}->{:.4} (d={:.4}), strategy={}, phases={}, {:.1}s",
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

    // Drop writers (consumed by finalize), sort + rewrite summary, trace, and
    // cache files so row order is deterministic regardless of rayon thread
    // scheduling and any crash-resume history. See `finalize_ascent_output`
    // for details.
    finalize_ascent_output(&output_paths, writers);
    let cache_rows = expensive_cache.used_rows();
    write_expensive_computation_cache_rows(&output_paths, &cache_rows);

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
    let expensive_stats = expensive_cache.stats();
    println!(
        "Expensive-computation cache: hits={}, misses={}, used_rows={}",
        expensive_stats.hits,
        expensive_stats.misses,
        cache_rows.len()
    );
    println!("Output: {}", output_paths.summary.display());
    println!("Trace: {}", output_paths.trace.display());
    println!("Cache: {}", output_paths.cache.display());
    println!(
        "Expensive computations cache: {}",
        output_paths.expensive_computations_cache.display()
    );
    println!(
        "Computed polytopes: {}",
        output_paths.computed_polytopes.display()
    );
    println!("Ascent events: {}", output_paths.ascent_events.display());
}

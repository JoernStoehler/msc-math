//! Projected gradient ascent on the Lagrangian product submanifold.
//!
//! At each iteration, computes d(sys)/d(a_k) via the library's dual-vertex
//! derivatives, projects the direction to preserve Lagrangian product structure
//! (q-facets keep zero p-components, p-facets keep zero q-components), then
//! steps directly in a-space: a_k(t) = a_k + t * d_k.
//! Boundary-crossing via overshoot (multiples of t_max) and wiggle (random
//! perturbation of dual vertices).
//!
//! Architecture: single binary, inline polytope generation + optimization.
//! Seeds: fresh random Lagrangian products (standard master seed, low attempt numbers).
//!
//! Predecessor: boundary-crossing-search (split into gradient-ascent-general
//! and gradient-ascent-products, 2026-04-04).
//!
//! Usage: cargo run -p exp-sys-landscape --release --bin sys-gradient-ascent-products
//! Flags: --fresh  (clear existing data and rerun)
//! Output: gradient-ascent-products/gradient-ascent-products.jsonl      (per-seed summary)
//!         gradient-ascent-products/gradient-ascent-products-trace.jsonl (per-iteration trace)

use database::{DualVerticesKey, PolytopeRecord};
use exp_sys_landscape::compute_step_bound;
use nalgebra::Vector4;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use rand_distr::{Distribution, StandardNormal};
use serde::Serialize;
use std::collections::{HashMap, HashSet};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;
use std::time::Instant;
use symplectic::algorithms::billiard::billiard_capacity;
use symplectic::algorithms::billiard::facet_classification::{classify_facets, FacetClassification};
use symplectic::derivatives::{capacity_derivatives_a, volume_derivatives_a};
use symplectic::geom::lagrangian_product::lagrangian_product;
use symplectic::geom::polygon::random_polygon_2d;
use symplectic::geom::polytope::Polytope4D;
use symplectic::geom::volume::volume;
use symplectic::kkt::saddle_point_solver::solve_kkt_for;

// ============================================================================
// Configuration
// ============================================================================

/// Reproducible RNG seed. Arbitrary choice; re-run with different seeds to
/// check robustness of findings.
const SEED: u64 = 42;

/// Development-scale counts. Increase for production runs.
const N_LAGRANGIAN_PER_BUCKET: usize = 4;

/// Lagrangian product splits (q_facets, p_facets) summing to 10.
const LAGRANGIAN_SPLITS: &[(usize, usize)] = &[(3, 7), (4, 6), (5, 5)];

/// Height range for random generation.
const H_MIN: f64 = 0.8;
const H_MAX: f64 = 1.2;

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

/// Prevents pathological steps when t_max is huge.
const MAX_STEP_SIZE: f64 = 100.0;

/// Number of random dual-vertex perturbations per escape round.
const N_WIGGLES: usize = 5;

/// Multiplicative perturbation scale for dual vertex components: a_k[i] -> a_k[i] * (1 + 0.05 * N(0,1)).
/// Per-facet displacement has expected norm ~0.05 * |a_k| (unit-scale dual vertices: ~0.05).
/// Cell-widths data (cell-widths/logbook.md): median non-orbit cell width = 0.124, median
/// orbit cell width = 0.258. So per-facet displacement ~0.05 is ~40% of the narrowest
/// median cell width. With F=10 facets all perturbed simultaneously, boundary crossing
/// is highly likely — confirmed by data: wiggle dominated overshoot as escape strategy
/// (gradient-ascent-products/logbook.md, gradient-ascent-general/logbook.md).
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
// Output schemas
// ============================================================================

/// One row per seed — the main analysis dataset.
#[derive(Debug, Serialize)]
struct SummaryRow {
    name: String,
    polytope_type: String,
    facet_count: usize,
    starting_sys: f64,
    final_sys: f64,
    total_delta: f64,
    n_ascent_phases: usize,
    n_gradient_iters_total: usize,
    n_escape_overshoot: usize,
    n_escape_wiggle: usize,
    best_strategy: String,
    total_time_ms: f64,
    final_dual_vertices: Vec<[f64; 4]>,
}

/// One row per iteration per ascent phase — diagnostic trace.
#[derive(Debug, Serialize)]
struct TraceRow {
    name: String,
    phase: usize,
    iteration: usize,
    step_type: String,
    t_fraction: f64,
    t_actual: f64,
    sys_before: f64,
    sys_after: f64,
    delta_sys: f64,
    gradient_norm: f64,
}

// ============================================================================
// Gradient step in a-space
// ============================================================================

/// Compute sys = c_EHZ(K)^2 / (2 vol(K)) for a polytope using billiard capacity.
fn compute_sys(polytope: &Polytope4D) -> Option<f64> {
    let vol = volume(polytope).ok().filter(|&v| v > 0.0)?;
    let cap = compute_capacity(polytope)?;
    let sys = cap * cap / (2.0 * vol);
    sys.is_finite().then_some(sys)
}

/// Try a step in dual-vertex space: a_k(t) = a_k + t * d_k.
fn try_step_a(
    duals: &[Vector4<f64>],
    direction: &[Vector4<f64>],
    t: f64,
) -> Option<(Polytope4D, f64)> {
    let new_duals: Vec<Vector4<f64>> = duals
        .iter()
        .zip(direction)
        .map(|(a, d)| a + t * d)
        .collect();
    let polytope = Polytope4D::from_f64(new_duals).ok()?;
    let sys = compute_sys(&polytope)?;
    Some((polytope, sys))
}

fn compute_capacity(polytope: &Polytope4D) -> Option<f64> {
    billiard_capacity(polytope).ok()?.map(|r| r.result.capacity)
}

fn compute_capacity_result(polytope: &Polytope4D) -> Option<(f64, Vec<usize>)> {
    let r = billiard_capacity(polytope).ok()??;
    Some((r.result.capacity, r.result.best_permutation))
}

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

/// Gradient ascent in dual-vertex space with overshoot at every iteration.
/// Direction is projected to preserve Lagrangian product structure.
///
/// At each step:
/// 1. Computes d(sys)/d(a_k) = (cap * d(cap)/d(a_k) - sys * d(vol)/d(a_k)) / vol
/// 2. Projects direction: q-facets zero out p-components, p-facets zero out q-components
/// 3. Tries STEP_FRACTIONS of t_max (within cell) and OVERSHOOT_MULTIPLIERS (crosses boundary)
/// 4. Picks the candidate with highest sys
// TODO: add [lem:sys-sensitivity] to math.tex (see gradient-correctness experiment)
fn gradient_ascent(
    name: &str,
    phase: usize,
    start: &Polytope4D,
    lagrangian_class: &FacetClassification,
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

        // 1. Capacity + KKT
        let (cap, best_perm) = compute_capacity_result(&current)?;
        let kkt = solve_kkt_for(&current, &best_perm).feasible()?;
        let vol = volume(&current).ok().filter(|&v| v > 0.0)?;
        let sys = cap * cap / (2.0 * vol);

        // 2. Gradient d(sys)/d(a_k)
        let duals = current.dual_vertices_f64();
        let d_vol_a = volume_derivatives_a(&current);
        let d_cap_a =
            capacity_derivatives_a(&kkt.beta, kkt.q_corrected, &kkt.mu, &best_perm, duals);
        let mut d_sys_a: Vec<Vector4<f64>> = d_vol_a
            .iter()
            .zip(d_cap_a.iter())
            .map(|(dv, dc)| (cap * dc - sys * dv) / vol)
            .collect();

        // Project direction to preserve Lagrangian product structure:
        // q-facets zero out p-components, p-facets zero out q-components
        for &k in &lagrangian_class.q_indices {
            d_sys_a[k][2] = 0.0;
            d_sys_a[k][3] = 0.0;
        }
        for &k in &lagrangian_class.p_indices {
            d_sys_a[k][0] = 0.0;
            d_sys_a[k][1] = 0.0;
        }

        let gradient_norm = d_sys_a
            .iter()
            .map(|d| d.norm_squared())
            .sum::<f64>()
            .sqrt();
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
            if let Some((p, new_sys)) = try_step_a(duals, &d_sys_a, t) {
                if new_sys > sys && best.as_ref().is_none_or(|b| new_sys > b.1) {
                    best = Some((p, new_sys, "within".into(), frac, t));
                }
            }
        }

        if t_max < MAX_STEP_SIZE {
            for &mult in OVERSHOOT_MULTIPLIERS {
                let t = mult * t_max;
                if let Some((p, new_sys)) = try_step_a(duals, &d_sys_a, t) {
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

struct SeedResult {
    summary: SummaryRow,
    trace: Vec<TraceRow>,
}

fn process_seed(
    name: &str,
    polytope_type: &str,
    polytope: &Polytope4D,
    lagrangian_class: &FacetClassification,
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
    if let Some(result) = gradient_ascent(name, n_phases, polytope, lagrangian_class, t0, budget) {
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
                    n_phases,
                    &wiggled,
                    lagrangian_class,
                    t0,
                    budget,
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
    let final_dvs: Vec<[f64; 4]> = best_polytope
        .dual_vertices_f64()
        .iter()
        .map(|a| [a[0], a[1], a[2], a[3]])
        .collect();

    Some(SeedResult {
        summary: SummaryRow {
            name: name.to_string(),
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
            final_dual_vertices: final_dvs,
        },
        trace: all_trace,
    })
}

// ============================================================================
// Polytope generation
// ============================================================================

fn generate_lagrangian_polytopes(rng: &mut ChaCha8Rng) -> Vec<(String, String, Polytope4D)> {
    let mut polytopes = Vec::new();
    for &(q_f, p_f) in LAGRANGIAN_SPLITS {
        let bucket_name = format!("lagrangian_{}x{}", q_f, p_f);
        let mut count = 0usize;
        let mut attempts = 0usize;
        while count < N_LAGRANGIAN_PER_BUCKET {
            attempts += 1;
            if attempts > N_LAGRANGIAN_PER_BUCKET * 100 {
                eprintln!(
                    "WARNING: gave up after {attempts} attempts for {bucket_name}, got {count}"
                );
                break;
            }
            let (qn, qh) = random_polygon_2d(q_f, H_MIN, H_MAX, rng);
            let (pn, ph) = random_polygon_2d(p_f, H_MIN, H_MAX, rng);
            if let Ok(p) = lagrangian_product(&qn, &qh, &pn, &ph) {
                let name = format!("{bucket_name}_{count}");
                polytopes.push((name, bucket_name.clone(), p));
                count += 1;
            }
        }
    }
    polytopes
}

// ============================================================================
// Resume support
// ============================================================================

fn load_completed_names(path: &std::path::Path) -> HashSet<String> {
    let mut names = HashSet::new();
    if let Ok(file) = File::open(path) {
        let reader = BufReader::new(file);
        for line in reader.lines().map_while(Result::ok) {
            let line = line.trim().to_string();
            if line.is_empty() {
                continue;
            }
            if let Ok(v) = serde_json::from_str::<serde_json::Value>(&line) {
                if let Some(name) = v.get("name").and_then(|n| n.as_str()) {
                    names.insert(name.to_string());
                }
            }
        }
    }
    names
}

// ============================================================================
// Main
// ============================================================================

/// Insert a polytope into the database if not already present.
/// Stores rational geometry for future vertex-enumeration-free reconstruction.
fn insert_polytope_to_db(
    db: &mut HashMap<DualVerticesKey, PolytopeRecord>,
    polytope: &Polytope4D,
) {
    let key: DualVerticesKey = polytope.dual_vertices().to_vec();
    if db.contains_key(&key) {
        return;
    }
    let record = PolytopeRecord::from_polytope(polytope);
    db.insert(key, record);
}

/// Write a seed result to the summary and trace JSONL files.
fn write_result(
    result: &SeedResult,
    summary_writer: &mut BufWriter<File>,
    trace_writer: &mut BufWriter<File>,
) {
    serde_json::to_writer(&mut *summary_writer, &result.summary).expect("write summary");
    writeln!(summary_writer).expect("newline");
    for row in &result.trace {
        serde_json::to_writer(&mut *trace_writer, row).expect("write trace");
        writeln!(trace_writer).expect("newline");
    }
}

fn main() {
    let t_global = Instant::now();
    let base =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("gradient-ascent-products");
    let summary_path = base.join("gradient-ascent-products.jsonl");
    let trace_path = base.join("gradient-ascent-products-trace.jsonl");

    println!("gradient-ascent-products: projected gradient ascent on Lagrangian products\n");

    // CLI args
    let args: Vec<String> = std::env::args().collect();
    let fresh = args.iter().any(|a| a == "--fresh");

    // Resume support
    let completed = if fresh {
        let _ = std::fs::remove_file(&summary_path);
        let _ = std::fs::remove_file(&trace_path);
        HashSet::new()
    } else {
        load_completed_names(&summary_path)
    };

    if completed.is_empty() {
        println!("Starting fresh run.");
    } else {
        println!("Resuming: {} seeds already completed.", completed.len());
    }

    let summary_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&summary_path)
        .expect("open summary JSONL");
    let mut summary_writer = BufWriter::new(summary_file);

    let trace_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&trace_path)
        .expect("open trace JSONL");
    let mut trace_writer = BufWriter::new(trace_file);

    let mut rng = ChaCha8Rng::seed_from_u64(SEED);
    let mut best_global = 0.0f64;
    let mut best_name = String::new();

    // Load polytope database for caching
    let db_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../data/polytopes.jsonl");
    let mut db: HashMap<DualVerticesKey, PolytopeRecord> =
        database::load(&db_path).expect("failed to load database");
    println!("Loaded database: {} entries\n", db.len());

    // =========================================================================
    // Phase 1: Lagrangian products
    // =========================================================================

    println!(
        "Generating Lagrangian products (splits: {:?}, {} per bucket)...",
        LAGRANGIAN_SPLITS, N_LAGRANGIAN_PER_BUCKET
    );
    let lagrangian = generate_lagrangian_polytopes(&mut rng);
    println!("Generated {} Lagrangian products.\n", lagrangian.len());

    for (idx, (name, bucket, polytope)) in lagrangian.iter().enumerate() {
        insert_polytope_to_db(&mut db, polytope);

        if completed.contains(name) {
            continue;
        }
        print!("[lagrangian {}/{}] {}: ", idx + 1, lagrangian.len(), name);

        let class = classify_facets(polytope).expect("should classify as Lagrangian");

        match process_seed(name, bucket, polytope, &class, &mut rng) {
            Some(result) => {
                write_result(&result, &mut summary_writer, &mut trace_writer);
                let s = &result.summary;
                if s.final_sys > best_global {
                    best_global = s.final_sys;
                    best_name = s.name.clone();
                }
                println!(
                    "sys: {:.4}->{:.4} (d={:.4}), strategy={}, phases={}, {:.1}s",
                    s.starting_sys, s.final_sys, s.total_delta, s.best_strategy,
                    s.n_ascent_phases, s.total_time_ms / 1000.0,
                );
                if s.final_sys > 1.0 {
                    eprintln!("*** VITERBO VIOLATION: {} sys={:.6} ***", s.name, s.final_sys);
                }
            }
            None => println!("FAILED"),
        }
    }

    // =========================================================================
    // Final summary
    // =========================================================================

    summary_writer.flush().expect("flush summary");
    trace_writer.flush().expect("flush trace");
    database::save(&db_path, &db).expect("failed to save database");

    println!("\n========================================");
    println!("Best sys: {:.6} ({})", best_global, best_name);
    println!("Database: {} entries", db.len());
    println!("Total time: {:.1}s", t_global.elapsed().as_secs_f64());
    println!("Output: {}", summary_path.display());
    println!("Trace: {}", trace_path.display());
}

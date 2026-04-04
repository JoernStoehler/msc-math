//! Free gradient ascent in R^{4F} on general polytopes.
//!
//! At each iteration, computes d(sys)/d(a_k) via the library's dual-vertex
//! derivatives, then steps directly in a-space: a_k(t) = a_k + t * d_k.
//! Boundary-crossing via overshoot (multiples of t_max) and wiggle (random
//! perturbation of dual vertices).
//!
//! Architecture: single binary, inline polytope generation + optimization.
//! Seeds: fresh random general polytopes (standard master seed, low attempt numbers).
//!
//! Predecessor: boundary-crossing-search (split into gradient-ascent-general
//! and gradient-ascent-products, 2026-04-04).
//!
//! Usage: cargo run -p exp-sys-landscape --release --bin sys-gradient-ascent-general
//! Flags: --fresh  (clear existing data and rerun)
//! Output: gradient-ascent-general/gradient-ascent-general.jsonl      (per-seed summary)
//!         gradient-ascent-general/gradient-ascent-general-trace.jsonl (per-iteration trace)

use database::{DualVerticesKey, PolytopeRecord, SigmaAction};
use nalgebra::{Matrix4, Vector4};
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use rand_distr::{Distribution, StandardNormal};
use serde::Serialize;
use std::collections::{HashMap, HashSet};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;
use std::time::Instant;
use symplectic::derivatives::{capacity_derivatives_a, volume_derivatives_a};
use symplectic::geom::polytope::Polytope4D;
use symplectic::geom::skeleton::Skeleton;
use symplectic::geom::volume::volume;
use symplectic::kkt::saddle_point_solver::solve_kkt_for;
use symplectic::random::sample_random_polytope;

// ============================================================================
// Configuration
// ============================================================================

/// Reproducible RNG seed. Arbitrary choice; re-run with different seeds to
/// check robustness of findings.
const SEED: u64 = 42;

/// Development-scale counts. Increase for production runs.
const N_GENERAL: usize = 10;

/// Facet count for fresh polytopes.
const FACET_COUNT: usize = 10;

/// Height range for random generation.
const H_MIN: f64 = 0.8;
const H_MAX: f64 = 1.2;

/// Maximum gradient ascent iterations per phase.
const MAX_ITERATIONS: usize = 30;

/// Minimum improvement per iteration to continue. Well above f64 noise
/// (~1e-15) but small enough to capture meaningful steps. Matches
/// gradient-descent and gradient-search. If changed, re-check convergence
/// rates in trace data.
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

/// ~5% perturbation of dual vertex components. Small enough to stay near the
/// current optimum, large enough to cross combinatorial boundaries.
const WIGGLE_STRENGTH: f64 = 0.05;

/// Maximum rounds of escape attempts after convergence.
const MAX_ESCAPE_ROUNDS: usize = 3;

/// Per-seed time budget. 120s is generous: most seeds converge in <30s, but
/// F=10 with multiple escape rounds can take longer. From gradient-search.
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
// Step bound in a-space (linearized)
// ============================================================================

/// Maximum step t > 0 along direction d_k in dual-vertex space before the
/// combinatorial type changes.
///
/// For a vertex v defined by 4 facets D = {d_1, ..., d_4} with a_{d_i} · v = 1,
/// the linearized vertex velocity is:
///   dv/dt = -A_D^{-1} · w,  where w_i = d_{d_i} · v
///
/// The slack for non-defining facet j is s_j = 1 - a_j · v, with rate:
///   ds_j/dt = -(d_j · v + a_j · dv/dt)
///
/// The step bound is t_max = min(s_j / |rate_j|) over all (vertex, facet)
/// pairs where rate < 0.
// TODO: add [lem:step-bound-a] to math.tex (see combinatorial-boundaries experiment)
fn compute_step_bound_a(polytope: &Polytope4D, direction: &[Vector4<f64>]) -> f64 {
    let duals = polytope.dual_vertices_f64();
    let vertices = polytope.vertices_f64();
    let f = polytope.facet_count();
    let skeleton = Skeleton::compute(polytope);

    let mut t_max = f64::INFINITY;

    for (vi, vertex_facets) in skeleton.vertex_facets.iter().enumerate() {
        let v = &vertices[vi];

        if vertex_facets.len() == 4 {
            // Simple vertex: exact linearization via A_D^{-1}
            let a_mat = Matrix4::from_rows(&[
                duals[vertex_facets[0]].transpose(),
                duals[vertex_facets[1]].transpose(),
                duals[vertex_facets[2]].transpose(),
                duals[vertex_facets[3]].transpose(),
            ]);

            let a_inv = match a_mat.try_inverse() {
                Some(inv) => inv,
                None => continue,
            };

            // w_i = d_{d_i} · v
            let w = Vector4::new(
                direction[vertex_facets[0]].dot(v),
                direction[vertex_facets[1]].dot(v),
                direction[vertex_facets[2]].dot(v),
                direction[vertex_facets[3]].dot(v),
            );
            let dv_dt = -(a_inv * w);

            for j in 0..f {
                if vertex_facets.contains(&j) {
                    continue;
                }
                // s_j = 1 - a_j · v
                let slack = 1.0 - duals[j].dot(v);
                // ds_j/dt = -(d_j · v + a_j · dv/dt)
                let rate = -(direction[j].dot(v) + duals[j].dot(&dv_dt));
                if rate < -EPS {
                    let t_crit = slack / (-rate);
                    if t_crit > 0.0 && t_crit < t_max {
                        t_max = t_crit;
                    }
                }
            }
        } else {
            // Non-simple vertex: conservative bound using max rates
            let max_d_norm = direction.iter().map(|d| d.norm()).fold(0.0f64, f64::max);
            if max_d_norm > EPS {
                for j in 0..f {
                    if vertex_facets.contains(&j) {
                        continue;
                    }
                    let slack = 1.0 - duals[j].dot(v);
                    // Conservative: |ds_j/dt| ≤ |d_j| · |v| + |a_j| · |dv/dt|
                    // Upper bound |dv/dt| by max_d_norm · |v| (rough)
                    let max_rate = max_d_norm * v.norm() * (1.0 + duals[j].norm());
                    if max_rate > EPS {
                        let t_crit = slack / max_rate;
                        if t_crit > 0.0 && t_crit < t_max {
                            t_max = t_crit;
                        }
                    }
                }
            }
        }
    }

    t_max.min(MAX_STEP_SIZE)
}

// ============================================================================
// Gradient step in a-space
// ============================================================================

/// Compute sys = c_EHZ(K)^2 / (2 vol(K)) for a polytope using HK2017.
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
    symplectic::algorithms::hk2017::ehz_capacity(polytope).map(|r| r.result.capacity)
}

fn compute_capacity_result(polytope: &Polytope4D) -> Option<(f64, Vec<usize>)> {
    let r = symplectic::algorithms::hk2017::ehz_capacity(polytope)?;
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
///
/// At each step:
/// 1. Computes d(sys)/d(a_k) = (cap * d(cap)/d(a_k) - sys * d(vol)/d(a_k)) / vol
/// 2. Tries STEP_FRACTIONS of t_max (within cell) and OVERSHOOT_MULTIPLIERS (crosses boundary)
/// 3. Picks the candidate with highest sys
// TODO: add [lem:sys-sensitivity] to math.tex (see gradient-correctness experiment)
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
        let d_sys_a: Vec<Vector4<f64>> = d_vol_a
            .iter()
            .zip(d_cap_a.iter())
            .map(|(dv, dc)| (cap * dc - sys * dv) / vol)
            .collect();

        let gradient_norm = d_sys_a
            .iter()
            .map(|d| d.norm_squared())
            .sum::<f64>()
            .sqrt();
        if gradient_norm < EPS {
            break;
        }

        // 3. Step bound
        let t_max = compute_step_bound_a(&current, &d_sys_a);
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
                if let Some(result) = gradient_ascent(
                    name,
                    n_phases,
                    &wiggled,
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

fn generate_general_polytopes(rng: &mut ChaCha8Rng) -> Vec<(String, Polytope4D)> {
    let mut polytopes = Vec::new();
    let mut attempts = 0usize;
    while polytopes.len() < N_GENERAL {
        attempts += 1;
        if attempts > N_GENERAL * 100 {
            eprintln!(
                "WARNING: gave up after {attempts} attempts, got {} general polytopes",
                polytopes.len()
            );
            break;
        }
        if let Ok(p) = sample_random_polytope(FACET_COUNT, H_MIN, H_MAX, rng) {
            let name = format!("general_{}", polytopes.len());
            polytopes.push((name, p));
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
/// Computes and stores capacity + volume if the record is new.
fn insert_polytope_to_db(
    db: &mut HashMap<DualVerticesKey, PolytopeRecord>,
    polytope: &Polytope4D,
    capacity: Option<f64>,
    volume_val: Option<f64>,
    best_perm: Option<&[usize]>,
) {
    let key: DualVerticesKey = polytope.dual_vertices().to_vec();
    if db.contains_key(&key) {
        return;
    }
    let mut record = PolytopeRecord::from_polytope(polytope);
    if let (Some(cap), Some(vol)) = (capacity, volume_val) {
        record = record.with_computed_fields(vol, 0.0, cap, 0.0);
        if let Some(perm) = best_perm {
            record = record.with_sigmas(
                vec![SigmaAction { perm: perm.to_vec(), action: cap }],
                0.0,
            );
        }
    }
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
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("gradient-ascent-general");
    let summary_path = base.join("gradient-ascent-general.jsonl");
    let trace_path = base.join("gradient-ascent-general-trace.jsonl");

    println!("gradient-ascent-general: free gradient ascent on general polytopes\n");

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
    // Phase 1: General random polytopes
    // =========================================================================

    println!("Generating {N_GENERAL} general random F={FACET_COUNT} polytopes...");
    let general = generate_general_polytopes(&mut rng);
    println!("Generated {} polytopes.\n", general.len());

    for (idx, (name, polytope)) in general.iter().enumerate() {
        // Insert starting seed into database
        insert_polytope_to_db(&mut db, polytope, None, None, None);

        if completed.contains(name) {
            continue;
        }
        print!("[general {}/{}] {}: ", idx + 1, general.len(), name);

        match process_seed(name, "general", polytope, &mut rng) {
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

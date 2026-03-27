//! First-order prediction test for analytical gradients ∂c/∂a_k, ∂vol/∂a_k, ∂sys/∂a_k.
//!
//! Tests the defining property of a gradient: f(a+td) − f(a) − t·g·d = o(t).
//! The residual r(t) = |f(a+td) − f(a) − t·g·d| should decrease as t → 0.
//! The log-log slope of r(t) vs t reveals smoothness: slope ≈ 2 for C², slope ≈ 1
//! for C¹ not C², slope ≈ 0 at non-differentiable points (orbit switching boundary).
//!
//! Q1: Generic random polytopes — convergence rates, dimension scaling
//! Q2: Non-generic geometry — Lagrangian products with symmetry-degenerate orbits
//! Q3: Near-degeneracy — small action gap between best and second-best orbit
//! Q4: Barely-cutting facets — near-redundant halfspaces
//! Q5: Orbit-switching — subdifferential prediction at near-tied orbits
//!
//! Methodology (Q1-Q4):
//! - For each polytope, compute base values and analytical gradients
//! - Sample random directions d in R^{4F} (unit vectors via Muller's method)
//! - Sweep perturbation size t geometrically from 1e-1 to 1e-7
//! - For capacity: solve_kkt_for with the base orbit on the perturbed polytope
//!   (tests per-orbit envelope theorem prediction; equals the capacity gradient at
//!   generic points where the minimizing orbit is unique)
//! - For volume: volume() of the perturbed polytope
//! - For sys = c²/(2·vol): derived from perturbed cap and vol
//!
//! Methodology (Q5):
//! - Enumerate all certified orbits within generous action gap of the best
//! - Compute per-orbit gradient g_i via capacity_derivatives_a for each
//! - Subdifferential prediction: D_d c = min_i(g_i · d)
//! - Compare against actual capacity change via full ehz_capacity on perturbed polytope
//! - Records orbit switching (which orbit wins in the perturbed polytope)
//! - [prop:capacity-piecewise-smooth](d): at switching boundaries, D_d c = min_i(∇A_i · d)
//!
//! Mathematical correspondence:
//! - [lem:cap-derivative] (unverified): envelope theorem formula for ∂c/∂a_k
//! - [lem:vol-derivative] (unverified): chain rule formula for ∂vol/∂a_k
//! - [prop:capacity-piecewise-smooth] (unverified): piecewise C^∞, generic differentiability
//!
//! Architecture:
//! 1. `cargo run --release --bin gradient_correctness [q1 q2 q3 q4 q5]` → JSONL files
//! 2. Python analyze.py → convergence plots and slope analysis
//!
//! Self-contained: generates all polytopes internally.

use nalgebra::Vector4;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use rand_distr::{Distribution, StandardNormal, Uniform};
use serde::Serialize;
use std::f64::consts::PI;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::panic;
use std::time::Instant;
use symplectic::algorithms::hk2017::combinations;
use symplectic::algorithms::hk2017::permutations::for_each_cyclic_permutation;
use symplectic::derivatives::{capacity_derivatives_a, volume_derivatives_a};
use symplectic::geom::facet_volume::facet_volume_3d;
use symplectic::geom::polygon::random_polygon_2d;
use symplectic::kkt::saddle_point_solver::{solve_kkt_for, KktResult, EPS_Q_POSITIVE};
use symplectic::random::generate_random_polytopes;
use symplectic::{ehz_capacity, lagrangian_product, regular_polygon_2d, rotate_polygon_2d};
use symplectic::{volume, Polytope4D};

// ============================================================================
// Constants
// ============================================================================

/// Base seed for deterministic RNG across all phases.
const SEED_BASE: u64 = 7777;

/// Number of random perturbation directions per polytope.
/// 5 directions in R^{4F} provides reasonable coverage for detecting
/// direction-dependent issues with isotropic sampling. Increasing to 10+
/// would tighten the slope distribution but 5 already gives IQR width < 0.1
/// for capacity. Decreasing below 3 risks missing direction-dependent bugs.
const N_DIRS: usize = 5;

/// Perturbation sizes for the first-order prediction test.
/// Geometric sweep from 1e-1 to 1e-7 with half-decade spacing (13 values).
/// Large t: tests robustness far from base point.
/// Small t: tests convergence to zero (the defining gradient property).
/// Below ~1e-7, floating-point cancellation in f(a+td)−f(a) dominates.
const T_VALUES: &[f64] = &[
    1e-1, 3e-2, 1e-2, 3e-3, 1e-3, 3e-4, 1e-4, 3e-5, 1e-5, 3e-6, 1e-6, 3e-7, 1e-7,
];

/// Q1: polytopes per facet count. 20 gives 600 traces total (6 F-values × 20 ×
/// 5 dirs), enough for stable slope medians. Runtime scales linearly.
const Q1_POLYTOPES_PER_F: usize = 20;

/// Q3: max candidates to generate when filling gap bins.
/// 2000 is enough to fill all bins at F=6 (verified in v1).
const Q3_MAX_CANDIDATES: usize = 2000;

/// Q3: max polytopes per gap bin. 20 gives ~100 traces per bin (×5 dirs),
/// enough for meaningful per-bin slope statistics.
const Q3_PER_BIN: usize = 20;

/// Q4: base polytopes to augment with barely-cutting facets.
/// 10 × 5 deltas × 5 dirs = 250 traces. Runtime is fast (F=7, ~12s total).
const Q4_BASE_COUNT: usize = 10;

/// Q4: barely-cutting delta values. Range 1e-1 to 1e-5 spans from "substantial cut"
/// to "facet volume near zero". Below 1e-5, Polytope4D::from_f64 may reject as
/// degenerate.
const Q4_DELTAS: &[f64] = &[1e-1, 1e-2, 1e-3, 1e-4, 1e-5];

/// Q3: gap bins (lower_bound, upper_bound, label).
const Q3_GAP_BINS: [(f64, f64, &str); 4] = [
    (1e-1, f64::INFINITY, "large"),
    (1e-2, 1e-1, "medium"),
    (1e-4, 1e-2, "small"),
    (0.0, 1e-4, "tiny"),
];

/// Minimum beta for certified orbit in Q3/Q5 enumeration.
/// Matches the library's EPS_MARGIN_TRUE (1e-9) from kkt/mod.rs — orbits with
/// beta below this are Indeterminate in the production accumulator.
const EPS_BETA_CERTIFIED: f64 = 1e-9;

/// Skip Q2 polytopes with F > this to avoid slow ehz_capacity calls.
/// LP(5,5) has F=10 (~3 min per ehz_capacity call in v1). F≤8 is tractable.
const MAX_FACET_Q2: usize = 8;

/// Q5: generous action gap threshold for orbit enumeration.
/// All orbits with action ≤ best_action + this threshold are kept.
/// Analysis filters to tighter thresholds in post-processing (no recomputation).
/// Value 0.1 chosen to include orbits up to ~10% of typical capacities (O(1)–O(10)),
/// matching the upper boundary of the "medium" gap bin (1e-3 to 1e-1). Orbits further
/// than 0.1 from the best have very different gradients and are not subdifferential
/// candidates at any realistic step size.
const Q5_GAP_THRESHOLD: f64 = 0.1;

/// Q5: polytopes per gap bin per facet count.
/// 15 per bin × 4 bins × 2 F-values = 120 polytopes (target), × 5 dirs × 13 t ≈ 7800
/// ehz_capacity calls on perturbed polytopes. 131s total (run 2026-03-27, F=6-7).
const Q5_PER_BIN: usize = 15;

/// Q5: max candidates to generate when filling gap bins.
/// 3000 fills all non-tiny bins at F=6-7 (run 2026-03-27: 47/60 at F=6, 53/60 at F=7).
/// Tiny bins (gap < 1e-5) are structurally rare and underfill regardless of budget.
const Q5_MAX_CANDIDATES: usize = 3000;

/// Q5: gap bins for polytope selection (by gap between best and second-best orbit).
const Q5_GAP_BINS: [(f64, f64, &str); 4] = [
    (1e-1, f64::INFINITY, "large"),
    (1e-3, 1e-1, "medium"),
    (1e-5, 1e-3, "small"),
    (0.0, 1e-5, "tiny"),
];

// ============================================================================
// Output schema
// ============================================================================

#[derive(Debug, Serialize)]
struct PredictionRow {
    phase: String,
    polytope_id: String,
    facet_count: usize,
    polytope_class: String,

    target: String,
    dir_idx: usize,
    t: f64,

    f_base: f64,
    f_perturbed: f64,
    grad_dot_d: f64,
    predicted_change: f64,
    actual_change: f64,
    residual: f64,
    residual_over_t: f64,

    log_t: f64,
    log_residual: f64,

    action_gap: Option<f64>,
    barely_cutting_delta: Option<f64>,
    min_facet_volume: Option<f64>,

    time_ms: f64,
}

/// Q5: per-orbit info embedded in each JSONL row for post-hoc gap-threshold filtering.
#[derive(Debug, Serialize)]
struct OrbitGradInfo {
    action: f64,
    grad_dot_d: f64,
}

/// Q5 output row: subdifferential prediction test for orbit-switching behavior.
#[derive(Debug, Serialize)]
struct SubdiffRow {
    phase: String,
    polytope_id: String,
    facet_count: usize,

    n_orbits: usize,
    action_gap: f64,

    dir_idx: usize,
    t: f64,
    log_t: f64,

    c_base: f64,
    c_perturbed: f64,
    actual_change: f64,

    subdiff_dot_d: f64,
    subdiff_predicted: f64,
    subdiff_residual: f64,
    subdiff_log_residual: f64,

    single_dot_d: f64,
    single_predicted: f64,
    single_residual: f64,
    single_log_residual: f64,

    base_best_perm: String,
    perturbed_best_perm: String,
    orbit_switched: bool,

    /// JSON array of {action, grad_dot_d} per orbit — for post-hoc gap-threshold analysis.
    orbit_grads: String,

    time_ms: f64,
}

// ============================================================================
// Helper functions
// ============================================================================

/// Sample a random unit vector in R^{4F} (isotropic: standard normals, then normalize).
fn random_direction(f: usize, rng: &mut ChaCha8Rng) -> Vec<Vector4<f64>> {
    let mut dir: Vec<Vector4<f64>> = (0..f)
        .map(|_| {
            Vector4::new(
                StandardNormal.sample(rng),
                StandardNormal.sample(rng),
                StandardNormal.sample(rng),
                StandardNormal.sample(rng),
            )
        })
        .collect();
    let norm = dir.iter().map(|v| v.norm_squared()).sum::<f64>().sqrt();
    if norm > 1e-10 {
        for v in &mut dir {
            *v /= norm;
        }
    }
    dir
}

/// Dot product of gradient and direction in R^{4F}: Σ_k g_k · d_k.
fn dot_grad_dir(g: &[Vector4<f64>], d: &[Vector4<f64>]) -> f64 {
    g.iter().zip(d.iter()).map(|(gk, dk)| gk.dot(dk)).sum()
}

/// Safe wrapper around ehz_capacity that catches panics (Q-correction panic
/// on near-degenerate polytopes).
fn ehz_capacity_safe(polytope: &Polytope4D) -> Option<symplectic::EhzResult> {
    let polytope = polytope.clone();
    panic::catch_unwind(panic::AssertUnwindSafe(|| ehz_capacity(&polytope)))
        .ok()
        .flatten()
}

/// Safe wrapper around solve_kkt_for that catches panics (Q-correction panic).
fn solve_kkt_safe(polytope: &Polytope4D, perm: &[usize]) -> Option<KktResult> {
    let polytope = polytope.clone();
    let perm = perm.to_vec();
    panic::catch_unwind(panic::AssertUnwindSafe(|| solve_kkt_for(&polytope, &perm)))
        .ok()
        .flatten()
}

/// Compute ∂sys/∂a_k via quotient rule: sys = c²/(2·vol).
/// ∂sys/∂a_k = (c·∂c/∂a_k − sys·∂vol/∂a_k) / vol.
/// [cor:sys-derivative] quotient-rule derivative of the systolic ratio.
/// In experiments/sys-optimization/math.tex.
fn sys_derivatives_a(
    d_cap: &[Vector4<f64>],
    d_vol: &[Vector4<f64>],
    cap: f64,
    vol: f64,
    sys: f64,
) -> Vec<Vector4<f64>> {
    d_vol
        .iter()
        .zip(d_cap.iter())
        .map(|(dv, dc)| (cap * dc - sys * dv) / vol)
        .collect()
}

/// Polytope with precomputed base values and KKT solution.
struct PolytopeInfo {
    polytope: Polytope4D,
    cap: f64,
    vol: f64,
    sys: f64,
    best_perm: Vec<usize>,
    kkt: KktResult,
}

/// Compute capacity, volume, sys, and KKT for a polytope's best orbit.
fn analyze_polytope(polytope: &Polytope4D) -> Option<PolytopeInfo> {
    let ehz = ehz_capacity_safe(polytope)?;
    let cap = ehz.result.capacity;
    let vol = volume(polytope).ok()?;
    if vol <= 0.0 {
        return None;
    }
    let sys = cap * cap / (2.0 * vol);
    let best_perm = ehz.result.best_permutation.clone();
    let kkt = solve_kkt_safe(polytope, &best_perm)?;
    Some(PolytopeInfo {
        polytope: polytope.clone(),
        cap,
        vol,
        sys,
        best_perm,
        kkt,
    })
}

/// Values of capacity, volume, and sys at a perturbed point a + t·d.
struct PerturbedValues {
    capacity: Option<f64>,
    volume: Option<f64>,
    sys: Option<f64>,
}

/// Compute cap, vol, sys at perturbed dual vertices a + t·d.
///
/// Capacity: solve_kkt_for with the base orbit on the perturbed polytope.
/// This tests the per-orbit envelope theorem prediction (equals the capacity
/// gradient at generic points where the minimizing orbit is unique).
fn compute_perturbed(
    base_duals: &[Vector4<f64>],
    direction: &[Vector4<f64>],
    t: f64,
    base_perm: &[usize],
) -> PerturbedValues {
    let perturbed: Vec<Vector4<f64>> = base_duals
        .iter()
        .zip(direction.iter())
        .map(|(a, d)| a + t * d)
        .collect();

    let polytope = match Polytope4D::from_f64(perturbed) {
        Ok(p) => p,
        Err(_) => {
            return PerturbedValues {
                capacity: None,
                volume: None,
                sys: None,
            }
        }
    };

    let cap = solve_kkt_safe(&polytope, base_perm)
        .filter(|kkt| kkt.q_corrected > EPS_Q_POSITIVE && kkt.beta.iter().all(|&b| b > 0.0))
        .map(|kkt| 0.5 / kkt.q_corrected);

    let vol = volume(&polytope).ok().filter(|&v| v > 0.0);

    let sys = match (cap, vol) {
        (Some(c), Some(v)) => Some(c * c / (2.0 * v)),
        _ => None,
    };

    PerturbedValues { capacity: cap, volume: vol, sys }
}

// ============================================================================
// Core: first-order prediction test
// ============================================================================

/// Run first-order prediction test for all three targets on a single polytope.
/// Returns one JSONL row per (target, direction, t) combination where the
/// perturbed value could be computed.
fn first_order_test(
    info: &PolytopeInfo,
    phase: &str,
    polytope_id: &str,
    polytope_class: &str,
    n_dirs: usize,
    rng: &mut ChaCha8Rng,
    action_gap: Option<f64>,
    barely_cutting_delta: Option<f64>,
    min_facet_volume: Option<f64>,
) -> Vec<PredictionRow> {
    let duals = info.polytope.dual_vertices_f64();
    let f = duals.len();

    // Analytical gradients for all three targets
    let g_cap = capacity_derivatives_a(
        &info.kkt.beta,
        info.kkt.q_corrected,
        &info.kkt.mu,
        &info.best_perm,
        &duals,
    );
    let g_vol = volume_derivatives_a(&info.polytope);
    let g_sys = sys_derivatives_a(&g_cap, &g_vol, info.cap, info.vol, info.sys);

    let targets: [(&str, f64, &[Vector4<f64>]); 3] = [
        ("capacity", info.cap, &g_cap),
        ("volume", info.vol, &g_vol),
        ("sys", info.sys, &g_sys),
    ];

    let mut rows = Vec::new();

    for dir_idx in 0..n_dirs {
        let direction = random_direction(f, rng);
        let gd: Vec<f64> = targets.iter().map(|(_, _, g)| dot_grad_dir(g, &direction)).collect();

        for &t in T_VALUES {
            let t0 = Instant::now();
            let perturbed = compute_perturbed(&duals, &direction, t, &info.best_perm);
            let elapsed = t0.elapsed().as_secs_f64() * 1000.0;

            let f_perturbed = [perturbed.capacity, perturbed.volume, perturbed.sys];

            for (i, &(target_name, f_base, _)) in targets.iter().enumerate() {
                if let Some(f_pert) = f_perturbed[i] {
                    let actual = f_pert - f_base;
                    let predicted = t * gd[i];
                    let residual = (actual - predicted).abs();
                    let rot = residual / t.abs();
                    // Floor at 1e-300 to avoid log10(0) = -inf (not valid JSON).
                    let log_residual = residual.max(1e-300).log10();

                    rows.push(PredictionRow {
                        phase: phase.to_string(),
                        polytope_id: polytope_id.to_string(),
                        facet_count: f,
                        polytope_class: polytope_class.to_string(),
                        target: target_name.to_string(),
                        dir_idx,
                        t,
                        f_base,
                        f_perturbed: f_pert,
                        grad_dot_d: gd[i],
                        predicted_change: predicted,
                        actual_change: actual,
                        residual,
                        residual_over_t: rot,
                        log_t: t.abs().log10(),
                        log_residual,
                        action_gap,
                        barely_cutting_delta,
                        min_facet_volume,
                        time_ms: elapsed,
                    });
                }
            }
        }
    }

    rows
}

/// Write rows to a JSONL writer.
fn write_rows(writer: &mut BufWriter<File>, rows: &[PredictionRow]) {
    for row in rows {
        let json = serde_json::to_string(row).expect("serialize row");
        writeln!(writer, "{}", json).expect("write row");
    }
}

// ============================================================================
// Phase-specific helpers
// ============================================================================

/// Sample a random unit vector on S³ (for Q4 facet normal generation).
fn random_unit_s3(rng: &mut ChaCha8Rng) -> Vector4<f64> {
    loop {
        let x: f64 = StandardNormal.sample(rng);
        let y: f64 = StandardNormal.sample(rng);
        let z: f64 = StandardNormal.sample(rng);
        let w: f64 = StandardNormal.sample(rng);
        let v = Vector4::new(x, y, z, w);
        let norm = v.norm();
        if norm > 1e-10 {
            return v / norm;
        }
    }
}

/// Add a barely-cutting facet near a random vertex of the polytope.
/// The new halfspace passes delta inside the vertex. Returns None if
/// construction fails after 50 attempts.
fn add_barely_cutting_facet(
    polytope: &Polytope4D,
    delta: f64,
    rng: &mut ChaCha8Rng,
) -> Option<Polytope4D> {
    let vertices = polytope.vertices_f64();
    let duals = polytope.dual_vertices_f64();

    // 50 attempts: success rate ~80% at delta≥1e-3, ~50% at delta=1e-5 (v1 data).
    for _ in 0..50 {
        let idx = Uniform::from(0..vertices.len()).sample(rng);
        let v = &vertices[idx];
        let n = random_unit_s3(rng);
        // h = n·v − δ: hyperplane passes δ inside vertex v
        let h = n.dot(v) - delta;
        if h <= 0.0 {
            continue;
        }
        let a_new = n / h;
        let mut new_duals = duals.to_vec();
        new_duals.push(a_new);
        if let Ok(p) = Polytope4D::from_f64(new_duals) {
            return Some(p);
        }
    }
    None
}

/// Enumerate all certified orbits for a polytope.
/// Returns (action, permutation, kkt_result) sorted by action ascending.
fn enumerate_all_orbits(polytope: &Polytope4D) -> Vec<(f64, Vec<usize>, KktResult)> {
    let f = polytope.facet_count();
    let mut orbits = Vec::new();

    for m in 2..=f {
        for subset in combinations(f, m) {
            for_each_cyclic_permutation(&subset, &mut |perm| {
                if let Some(kkt) = solve_kkt_safe(polytope, perm) {
                    let min_beta = kkt.beta.iter().copied().fold(f64::INFINITY, f64::min);
                    if min_beta > EPS_BETA_CERTIFIED && kkt.q_corrected > EPS_Q_POSITIVE {
                        let action = 0.5 / kkt.q_corrected;
                        orbits.push((action, perm.to_vec(), kkt));
                    }
                }
            });
        }
    }

    orbits.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    orbits
}



// ============================================================================
// Phases
// ============================================================================

fn run_q1(base_dir: &str) {
    let path = format!("{}/gradient-correctness-q1-generic.jsonl", base_dir);
    let file = File::create(&path).expect("create Q1 JSONL");
    let mut writer = BufWriter::new(file);
    let mut total_rows = 0;

    let facet_counts = [5, 6, 7, 8, 9, 10];

    for &f_count in &facet_counts {
        let mut rng = ChaCha8Rng::seed_from_u64(SEED_BASE + f_count as u64);
        let polytopes =
            generate_random_polytopes(Q1_POLYTOPES_PER_F, f_count, 0.5, 2.0, &mut rng);

        for (i, polytope) in polytopes.iter().enumerate() {
            let info = match analyze_polytope(polytope) {
                Some(info) => info,
                None => {
                    eprintln!("  Q1: F={} polytope {} — failed, skipping", f_count, i);
                    continue;
                }
            };

            let id = format!("generic_F{}_{:03}", f_count, i);
            let rows = first_order_test(
                &info, "q1", &id, "random", N_DIRS, &mut rng, None, None, None,
            );
            write_rows(&mut writer, &rows);
            total_rows += rows.len();

            if (i + 1) % 5 == 0 {
                println!(
                    "  Q1: F={} — {}/{} polytopes done",
                    f_count,
                    i + 1,
                    Q1_POLYTOPES_PER_F
                );
            }
        }
    }

    writer.flush().expect("flush Q1");
    println!("Q1 done: {} rows written to {}", total_rows, path);
}

fn run_q2(base_dir: &str) {
    let path = format!("{}/gradient-correctness-q2-nongeneric.jsonl", base_dir);
    let file = File::create(&path).expect("create Q2 JSONL");
    let mut writer = BufWriter::new(file);
    let mut total_rows = 0;

    let regular_pairs = [(3, 3), (3, 4), (4, 4), (3, 5), (4, 5), (5, 5)];
    let rotation_angles = [PI / 7.0, PI / 5.0, PI / 3.0];
    let random_pairs = [(3, 3), (3, 4), (4, 4), (5, 5)];
    let random_per_pair = 5;

    let mut rng = ChaCha8Rng::seed_from_u64(SEED_BASE + 200);

    // Regular Lagrangian products
    for &(n1, n2) in &regular_pairs {
        if n1 + n2 > MAX_FACET_Q2 {
            println!(
                "  Q2: skipping LP({},{}) — F={} > {}",
                n1,
                n2,
                n1 + n2,
                MAX_FACET_Q2
            );
            continue;
        }
        let (qn, qh) = regular_polygon_2d(n1, 1.0);
        let (pn, ph) = regular_polygon_2d(n2, 1.0);
        let polytope = lagrangian_product(&qn, &qh, &pn, &ph).expect("regular LP");
        let id = format!("lp_regular_{}_{}", n1, n2);

        if let Some(info) = analyze_polytope(&polytope) {
            let rows = first_order_test(
                &info,
                "q2",
                &id,
                "lagrangian_regular",
                N_DIRS,
                &mut rng,
                None,
                None,
                None,
            );
            write_rows(&mut writer, &rows);
            total_rows += rows.len();
        } else {
            eprintln!("  Q2: regular LP({},{}) — failed", n1, n2);
        }
    }

    // Rotated Lagrangian products
    for &(n1, n2) in &regular_pairs {
        if n1 + n2 > MAX_FACET_Q2 {
            continue;
        }
        let (qn, qh) = regular_polygon_2d(n1, 1.0);
        for (ai, &theta) in rotation_angles.iter().enumerate() {
            let (pn, ph) = regular_polygon_2d(n2, 1.0);
            let (pn_rot, ph_rot) = rotate_polygon_2d(&pn, &ph, theta);
            let polytope =
                lagrangian_product(&qn, &qh, &pn_rot, &ph_rot).expect("rotated LP");
            let id = format!("lp_rotated_{}_{}_{}", n1, n2, ai);

            if let Some(info) = analyze_polytope(&polytope) {
                let rows = first_order_test(
                    &info,
                    "q2",
                    &id,
                    "lagrangian_rotated",
                    N_DIRS,
                    &mut rng,
                    None,
                    None,
                    None,
                );
                write_rows(&mut writer, &rows);
                total_rows += rows.len();
            } else {
                eprintln!("  Q2: rotated LP({},{},θ={:.3}) — failed", n1, n2, theta);
            }
        }
    }

    // Random Lagrangian products
    for &(n1, n2) in &random_pairs {
        if n1 + n2 > MAX_FACET_Q2 {
            continue;
        }
        for j in 0..random_per_pair {
            let (qn, qh) = random_polygon_2d(n1, 0.5, 2.0, &mut rng);
            let (pn, ph) = random_polygon_2d(n2, 0.5, 2.0, &mut rng);
            let polytope = match lagrangian_product(&qn, &qh, &pn, &ph) {
                Ok(p) => p,
                Err(e) => {
                    eprintln!(
                        "  Q2: random LP({},{},{}) — construction: {:?}",
                        n1, n2, j, e
                    );
                    continue;
                }
            };
            let id = format!("lp_random_{}_{}_{:02}", n1, n2, j);

            if let Some(info) = analyze_polytope(&polytope) {
                let rows = first_order_test(
                    &info,
                    "q2",
                    &id,
                    "lagrangian_random",
                    N_DIRS,
                    &mut rng,
                    None,
                    None,
                    None,
                );
                write_rows(&mut writer, &rows);
                total_rows += rows.len();
            } else {
                eprintln!("  Q2: random LP({},{},{}) — failed", n1, n2, j);
            }
        }
    }

    writer.flush().expect("flush Q2");
    println!("Q2 done: {} rows written to {}", total_rows, path);
}

fn run_q3(base_dir: &str) {
    let path = format!("{}/gradient-correctness-q3-degeneracy.jsonl", base_dir);
    let file = File::create(&path).expect("create Q3 JSONL");
    let mut writer = BufWriter::new(file);
    let mut bin_counts = [0usize; 4];
    let mut total_rows = 0;
    let mut generated = 0;

    let mut rng = ChaCha8Rng::seed_from_u64(SEED_BASE + 300);
    let f_count = 6; // Small F for tractable orbit enumeration

    println!("  Q3: Generating candidates (F={})...", f_count);

    while generated < Q3_MAX_CANDIDATES && bin_counts.iter().any(|&c| c < Q3_PER_BIN) {
        let polytopes = generate_random_polytopes(10, f_count, 0.5, 2.0, &mut rng);

        for polytope in &polytopes {
            generated += 1;
            if bin_counts.iter().all(|&c| c >= Q3_PER_BIN) {
                break;
            }

            let orbits = enumerate_all_orbits(polytope);
            if orbits.len() < 2 {
                continue;
            }

            let best_action = orbits[0].0;
            let second_action = orbits[1].0;
            let gap = second_action - best_action;

            let bin_idx = Q3_GAP_BINS
                .iter()
                .position(|&(lo, hi, _)| gap >= lo && gap < hi);
            let bin_idx = match bin_idx {
                Some(idx) if bin_counts[idx] < Q3_PER_BIN => idx,
                _ => continue,
            };

            let info = match analyze_polytope(polytope) {
                Some(info) => info,
                None => continue,
            };

            let id = format!(
                "degeneracy_{}_{:03}",
                Q3_GAP_BINS[bin_idx].2, bin_counts[bin_idx]
            );
            let rows = first_order_test(
                &info,
                "q3",
                &id,
                "near_degenerate",
                N_DIRS,
                &mut rng,
                Some(gap),
                None,
                None,
            );
            write_rows(&mut writer, &rows);
            total_rows += rows.len();
            bin_counts[bin_idx] += 1;

            if generated % 100 == 0 {
                println!(
                    "  Q3: {} candidates, bins: large={}, medium={}, small={}, tiny={}",
                    generated, bin_counts[0], bin_counts[1], bin_counts[2], bin_counts[3],
                );
            }
        }
    }

    writer.flush().expect("flush Q3");
    println!(
        "Q3 done: {} rows, {} candidates, bins: large={}, medium={}, small={}, tiny={}",
        total_rows, generated, bin_counts[0], bin_counts[1], bin_counts[2], bin_counts[3],
    );
}

fn run_q4(base_dir: &str) {
    let path = format!("{}/gradient-correctness-q4-redundant.jsonl", base_dir);
    let file = File::create(&path).expect("create Q4 JSONL");
    let mut writer = BufWriter::new(file);
    let mut total_rows = 0;

    let mut rng = ChaCha8Rng::seed_from_u64(SEED_BASE + 400);
    let f_count = 6;
    let base_polytopes =
        generate_random_polytopes(Q4_BASE_COUNT, f_count, 0.5, 2.0, &mut rng);

    for (i, base) in base_polytopes.iter().enumerate() {
        for &delta in Q4_DELTAS {
            let augmented = match add_barely_cutting_facet(base, delta, &mut rng) {
                Some(p) => p,
                None => {
                    eprintln!(
                        "  Q4: base {} delta={:.0e} — construction failed",
                        i, delta
                    );
                    continue;
                }
            };

            let info = match analyze_polytope(&augmented) {
                Some(info) => info,
                None => {
                    eprintln!("  Q4: base {} delta={:.0e} — capacity failed", i, delta);
                    continue;
                }
            };

            let min_fv = (0..augmented.facet_count())
                .map(|k| facet_volume_3d(&augmented, k))
                .filter(|&fv| fv > 0.0)
                .fold(f64::INFINITY, f64::min);

            let id = format!("barely_cutting_{:02}_d{:.0e}", i, delta);
            let rows = first_order_test(
                &info,
                "q4",
                &id,
                "barely_cutting",
                N_DIRS,
                &mut rng,
                None,
                Some(delta),
                Some(min_fv),
            );
            write_rows(&mut writer, &rows);
            total_rows += rows.len();
        }
        println!("  Q4: base polytope {}/{} done", i + 1, Q4_BASE_COUNT);
    }

    writer.flush().expect("flush Q4");
    println!("Q4 done: {} rows written", total_rows);
}

// ============================================================================
// Q5: Orbit-switching and subdifferential prediction
// ============================================================================

fn run_q5(base_dir: &str) {
    let path = format!("{}/gradient-correctness-q5-subdiff.jsonl", base_dir);
    let file = File::create(&path).expect("create Q5 JSONL");
    let mut writer = BufWriter::new(file);
    let mut total_rows = 0;

    let facet_counts = [6, 7];

    for &f_count in &facet_counts {
        // Benchmark ehz_capacity at this F
        let mut bench_rng = ChaCha8Rng::seed_from_u64(SEED_BASE + 550 + f_count as u64);
        let bench_polys = generate_random_polytopes(5, f_count, 0.5, 2.0, &mut bench_rng);
        let t0 = Instant::now();
        for p in &bench_polys {
            ehz_capacity_safe(p);
        }
        let bench_ms = t0.elapsed().as_secs_f64() * 1000.0 / bench_polys.len() as f64;
        println!("  Q5: F={} ehz_capacity benchmark: {:.2}ms/call", f_count, bench_ms);

        // Fill gap bins: find polytopes with different action gap levels
        let mut rng = ChaCha8Rng::seed_from_u64(SEED_BASE + 500 + f_count as u64);
        let mut bin_counts = [0usize; 4];
        let mut generated = 0;

        struct PolytopeWithOrbits {
            polytope: Polytope4D,
            orbits: Vec<(f64, Vec<usize>, KktResult)>,
            gap: f64,
        }
        let mut polytope_data: Vec<PolytopeWithOrbits> = Vec::new();

        println!("  Q5: Finding polytopes with near-tied orbits at F={}...", f_count);

        while generated < Q5_MAX_CANDIDATES && bin_counts.iter().any(|&c| c < Q5_PER_BIN) {
            let polytopes = generate_random_polytopes(10, f_count, 0.5, 2.0, &mut rng);

            for polytope in &polytopes {
                generated += 1;
                if bin_counts.iter().all(|&c| c >= Q5_PER_BIN) {
                    break;
                }

                // Use enumerate_all_orbits for binning (need second-best action).
                // Then filter to gap threshold for storage.
                let all_orbits = enumerate_all_orbits(polytope);
                if all_orbits.len() < 2 {
                    continue;
                }

                let best_action = all_orbits[0].0;
                let second_action = all_orbits[1].0;
                let gap = second_action - best_action;

                let bin_idx = Q5_GAP_BINS
                    .iter()
                    .position(|&(lo, hi, _)| gap >= lo && gap < hi);
                let bin_idx = match bin_idx {
                    Some(idx) if bin_counts[idx] < Q5_PER_BIN => idx,
                    _ => continue,
                };

                // Keep only orbits within generous gap threshold
                let filtered: Vec<_> = all_orbits
                    .into_iter()
                    .filter(|(action, _, _)| *action <= best_action + Q5_GAP_THRESHOLD)
                    .collect();

                polytope_data.push(PolytopeWithOrbits {
                    polytope: polytope.clone(),
                    orbits: filtered,
                    gap,
                });
                bin_counts[bin_idx] += 1;

                if generated % 200 == 0 {
                    println!(
                        "    {} candidates, bins: large={}, medium={}, small={}, tiny={}",
                        generated,
                        bin_counts[0],
                        bin_counts[1],
                        bin_counts[2],
                        bin_counts[3],
                    );
                }
            }
        }

        println!(
            "  Q5: F={} — {} polytopes (bins: {}/{}/{}/{}), from {} candidates",
            f_count,
            polytope_data.len(),
            bin_counts[0],
            bin_counts[1],
            bin_counts[2],
            bin_counts[3],
            generated,
        );

        // Process each polytope
        let mut dir_rng = ChaCha8Rng::seed_from_u64(SEED_BASE + 600 + f_count as u64);

        for (pi, pd) in polytope_data.iter().enumerate() {
            let duals = pd.polytope.dual_vertices_f64();
            let best_perm = &pd.orbits[0].1;
            let c_base = pd.orbits[0].0; // capacity = action of best orbit

            // Compute per-orbit gradients
            let orbit_grads: Vec<Vec<Vector4<f64>>> = pd
                .orbits
                .iter()
                .map(|(_action, perm, kkt)| {
                    capacity_derivatives_a(&kkt.beta, kkt.q_corrected, &kkt.mu, perm, &duals)
                })
                .collect();

            let id = format!("q5_F{}_{:03}", f_count, pi);
            let base_perm_str = serde_json::to_string(best_perm).unwrap();

            for dir_idx in 0..N_DIRS {
                let direction = random_direction(duals.len(), &mut dir_rng);

                // g_i · d for each orbit
                let orbit_gd: Vec<f64> = orbit_grads
                    .iter()
                    .map(|g| dot_grad_dir(g, &direction))
                    .collect();

                // [prop:capacity-piecewise-smooth](d): at switching boundaries,
                // the directional derivative D_d c = min_i(∇_a A_i · d).
                let subdiff_gd = orbit_gd
                    .iter()
                    .copied()
                    .fold(f64::INFINITY, f64::min);
                // [lem:cap-derivative]: single best-orbit gradient prediction.
                let single_gd = orbit_gd[0];

                // Per-orbit details for post-hoc gap-threshold analysis
                let orbit_info: Vec<OrbitGradInfo> = pd
                    .orbits
                    .iter()
                    .zip(orbit_gd.iter())
                    .map(|((action, _, _), &gd)| OrbitGradInfo {
                        action: *action,
                        grad_dot_d: gd,
                    })
                    .collect();
                let orbit_grads_json = serde_json::to_string(&orbit_info).unwrap();

                for &t in T_VALUES {
                    let t0 = Instant::now();

                    // Perturb dual vertices
                    let perturbed_duals: Vec<Vector4<f64>> = duals
                        .iter()
                        .zip(direction.iter())
                        .map(|(a, d)| a + t * d)
                        .collect();

                    let perturbed_polytope = match Polytope4D::from_f64(perturbed_duals) {
                        Ok(p) => p,
                        Err(_) => continue,
                    };

                    // Full ehz_capacity on perturbed polytope — the key difference from Q1-Q4
                    let perturbed_ehz = match ehz_capacity_safe(&perturbed_polytope) {
                        Some(r) => r,
                        None => continue,
                    };

                    let c_perturbed = perturbed_ehz.result.capacity;
                    let perturbed_perm = &perturbed_ehz.result.best_permutation;
                    let perturbed_perm_str = serde_json::to_string(perturbed_perm).unwrap();
                    let orbit_switched = perturbed_perm != best_perm;

                    let actual = c_perturbed - c_base;

                    let subdiff_pred = t * subdiff_gd;
                    let subdiff_res = (actual - subdiff_pred).abs();

                    let single_pred = t * single_gd;
                    let single_res = (actual - single_pred).abs();

                    let elapsed = t0.elapsed().as_secs_f64() * 1000.0;

                    let row = SubdiffRow {
                        phase: "q5".to_string(),
                        polytope_id: id.clone(),
                        facet_count: f_count,
                        n_orbits: pd.orbits.len(),
                        action_gap: pd.gap,
                        dir_idx,
                        t,
                        log_t: t.abs().log10(),
                        c_base,
                        c_perturbed,
                        actual_change: actual,
                        subdiff_dot_d: subdiff_gd,
                        subdiff_predicted: subdiff_pred,
                        subdiff_residual: subdiff_res,
                        subdiff_log_residual: subdiff_res.max(1e-300).log10(),
                        single_dot_d: single_gd,
                        single_predicted: single_pred,
                        single_residual: single_res,
                        single_log_residual: single_res.max(1e-300).log10(),
                        base_best_perm: base_perm_str.clone(),
                        perturbed_best_perm: perturbed_perm_str,
                        orbit_switched,
                        orbit_grads: orbit_grads_json.clone(),
                        time_ms: elapsed,
                    };

                    let json = serde_json::to_string(&row).expect("serialize Q5 row");
                    writeln!(writer, "{}", json).expect("write Q5 row");
                    total_rows += 1;
                }
            }

            if (pi + 1) % 10 == 0 {
                println!(
                    "  Q5: F={} — {}/{} polytopes done",
                    f_count,
                    pi + 1,
                    polytope_data.len()
                );
            }
        }
    }

    writer.flush().expect("flush Q5");
    println!("Q5 done: {} rows written to {}", total_rows, path);
}

// ============================================================================
// Main
// ============================================================================

fn main() {
    let base_dir = "gradient-correctness";
    let args: Vec<String> = std::env::args().collect();
    let run_all = args.len() <= 1;
    let phases: Vec<&str> = if run_all {
        vec!["q1", "q2", "q3", "q4", "q5"]
    } else {
        args[1..].iter().map(|s| s.as_str()).collect()
    };

    println!("=== Gradient Correctness: First-Order Prediction Test ===");
    println!("Phases: {:?}\n", phases);

    let t0 = Instant::now();

    for &phase in &phases {
        let tp = Instant::now();
        match phase {
            "q1" => {
                println!("--- Q1: Generic random polytopes ---");
                run_q1(base_dir);
            }
            "q2" => {
                println!("--- Q2: Non-generic geometry (Lagrangian products) ---");
                run_q2(base_dir);
            }
            "q3" => {
                println!("--- Q3: Near-degeneracy ---");
                run_q3(base_dir);
            }
            "q4" => {
                println!("--- Q4: Barely-cutting facets ---");
                run_q4(base_dir);
            }
            "q5" => {
                println!("--- Q5: Orbit-switching (subdifferential prediction) ---");
                run_q5(base_dir);
            }
            other => eprintln!("Unknown phase: {}", other),
        }
        println!(
            "  {} time: {:.1}s\n",
            phase.to_uppercase(),
            tp.elapsed().as_secs_f64()
        );
    }

    println!("=== Total time: {:.1}s ===", t0.elapsed().as_secs_f64());
}

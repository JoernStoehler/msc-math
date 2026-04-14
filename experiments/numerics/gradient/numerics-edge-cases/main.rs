//! First-order prediction test for analytical gradients (Q3 near-degeneracy + Q4 barely-cutting).
//!
//! Tests the defining property of a gradient: f(a+td) - f(a) - t*g*d = o(t).
//! The residual r(t) = |f(a+td) - f(a) - t*g*d| should decrease as t -> 0.
//! The log-log slope of r(t) vs t reveals smoothness: slope ~ 2 for C^2.
//!
//! Q3: Near-degeneracy -- small action gap between best and second-best orbit
//! Q4: Barely-cutting facets -- near-redundant halfspaces
//!
//! Split from gradient-validation/main.rs (Q1-Q4 shared the first_order_test framework;
//! Q3+Q4 test edge cases with special polytope construction).
//!
//! Methodology:
//! - For each polytope, compute base values and analytical gradients
//! - Sample random directions d in R^{4F} (unit vectors via Muller's method)
//! - Sweep perturbation size t geometrically from 1e-1 to 1e-7
//! - For capacity: solve_kkt_for with the base orbit on the perturbed polytope
//! - For volume: volume() of the perturbed polytope
//! - For sys = c^2/(2*vol): derived from perturbed cap and vol
//!
//! Mathematical correspondence:
//! - [lem:cap-derivative] (unverified): envelope theorem formula for dc/da_k.
//!   In library/src/algorithms/math.tex.
//! - [lem:vol-derivative] (unverified): chain rule formula for dvol/da_k.
//!   In library/src/algorithms/math.tex.
//! - [prop:capacity-piecewise-smooth] (unverified): piecewise C^inf, generic differentiability.
//!   In library/src/algorithms/math.tex.
//!
//! Architecture:
//! 1. `cargo run --release --bin gradient-edge-cases` -> JSONL files
//! 2. Python analyze.py -> convergence plots and slope analysis
//!
//! Self-contained: generates all polytopes internally.

use nalgebra::Vector4;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use rand_distr::{Distribution, StandardNormal, Uniform};
use serde::Serialize;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::time::Instant;
use symplectic::algorithms::hk2017::combinations;
use symplectic::algorithms::hk2017::permutations::for_each_cyclic_permutation;
use symplectic::derivatives::{capacity_derivatives_a, volume_derivatives_a};
use symplectic::geom::facet_volume::facet_volume_3d;
use symplectic::kkt::saddle_point_solver::{solve_kkt_for, KktResult, EPS_Q_POSITIVE};
use symplectic::random::generate_random_polytopes;
use symplectic::{ehz_capacity, volume, Polytope4D};

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
/// Below ~1e-7, floating-point cancellation in f(a+td)-f(a) dominates.
const T_VALUES: &[f64] = &[
    1e-1, 3e-2, 1e-2, 3e-3, 1e-3, 3e-4, 1e-4, 3e-5, 1e-5, 3e-6, 1e-6, 3e-7, 1e-7,
];

/// Q3: max candidates to generate when filling gap bins.
/// 2000 is enough to fill all bins at F=6 (verified in v1).
const Q3_MAX_CANDIDATES: usize = 2000;

/// Q3: max polytopes per gap bin. 20 gives ~100 traces per bin (x5 dirs),
/// enough for meaningful per-bin slope statistics.
const Q3_PER_BIN: usize = 20;

/// Q4: base polytopes to augment with barely-cutting facets.
/// 10 x 5 deltas x 5 dirs = 250 traces. Runtime is fast (F=7, ~12s total).
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

/// Minimum beta for certified orbit in Q3 enumeration.
/// Matches the library's EPS_MARGIN_TRUE (1e-9) from kkt/mod.rs -- orbits with
/// beta below this are Indeterminate in the production accumulator.
const EPS_BETA_CERTIFIED: f64 = 1e-9;

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

/// Dot product of gradient and direction in R^{4F}: Sigma_k g_k . d_k.
fn dot_grad_dir(g: &[Vector4<f64>], d: &[Vector4<f64>]) -> f64 {
    g.iter().zip(d.iter()).map(|(gk, dk)| gk.dot(dk)).sum()
}

fn ehz_capacity_safe(polytope: &Polytope4D) -> Option<symplectic::EhzResult> {
    ehz_capacity(polytope)
}

fn solve_kkt_safe(polytope: &Polytope4D, perm: &[usize]) -> Option<KktResult> {
    solve_kkt_for(polytope, perm).feasible()
}

/// Compute dsys/da_k via quotient rule: sys = c^2/(2*vol).
/// dsys/da_k = (c*dc/da_k - sys*dvol/da_k) / vol.
/// [cor:sys-derivative] quotient-rule derivative of the systolic ratio.
/// In library/src/algorithms/math.tex.
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

/// Values of capacity, volume, and sys at a perturbed point a + t*d.
struct PerturbedValues {
    capacity: Option<f64>,
    volume: Option<f64>,
    sys: Option<f64>,
}

/// Compute cap, vol, sys at perturbed dual vertices a + t*d.
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

/// Sample a random unit vector on S^3 (for Q4 facet normal generation).
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

    // 50 attempts: success rate ~80% at delta>=1e-3, ~50% at delta=1e-5 (v1 data).
    for _ in 0..50 {
        let idx = Uniform::from(0..vertices.len()).sample(rng);
        let v = &vertices[idx];
        let n = random_unit_s3(rng);
        // h = n*v - delta: hyperplane passes delta inside vertex v
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

/// Enumerate all certified orbits for a polytope (strict: beta > EPS, Q > EPS).
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
// Main
// ============================================================================

fn main() {
    let base_dir = ".";

    println!("=== Gradient Correctness: Edge Cases (Q3 + Q4) ===\n");
    let t0 = Instant::now();

    println!("--- Q3: Near-degeneracy ---");
    let tp = Instant::now();
    run_q3(base_dir);
    println!("  Q3 time: {:.1}s\n", tp.elapsed().as_secs_f64());

    println!("--- Q4: Barely-cutting facets ---");
    let tp = Instant::now();
    run_q4(base_dir);
    println!("  Q4 time: {:.1}s\n", tp.elapsed().as_secs_f64());

    println!("=== Total time: {:.1}s ===", t0.elapsed().as_secs_f64());
}

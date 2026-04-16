//! First-order prediction test for analytical gradients (Q1 generic + Q2 non-generic).
//!
//! Tests the defining property of a gradient: f(a+td) - f(a) - t*g*d = o(t).
//! The residual r(t) = |f(a+td) - f(a) - t*g*d| should decrease as t -> 0.
//! The log-log slope of r(t) vs t reveals smoothness: slope ~ 2 for C^2.
//!
//! Q1: Generic random polytopes -- convergence rates, dimension scaling
//! Q2: Non-generic geometry -- Lagrangian products with symmetry-degenerate orbits
//!
//! Split from gradient-validation/main.rs (Q1-Q4 shared the first_order_test framework;
//! Q1+Q2 are the basic validation cases with no special polytope construction).
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
//!   In formal/library/algorithms.tex.
//! - [lem:vol-derivative] (unverified): chain rule formula for dvol/da_k.
//!   In formal/library/algorithms.tex.
//! - [prop:capacity-piecewise-smooth] (unverified): piecewise C^inf, generic differentiability.
//!   In formal/library/algorithms.tex.
//!
//! Architecture:
//! 1. `cargo run --release --bin gradient-basic-validation` -> JSONL files
//! 2. Python analyze.py -> convergence plots and slope analysis
//!
//! Self-contained: generates all polytopes internally.

use nalgebra::Vector4;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use rand_distr::{Distribution, StandardNormal};
use serde::Serialize;
use std::f64::consts::PI;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::time::{Instant, SystemTime, UNIX_EPOCH};
use symplectic::derivatives::{
    capacity_derivatives_a_from_kkt_result,
    directional_derivative_a,
    volume_derivatives_a,
};
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
/// Below ~1e-7, floating-point cancellation in f(a+td)-f(a) dominates.
const T_VALUES: &[f64] = &[
    1e-1, 3e-2, 1e-2, 3e-3, 1e-3, 3e-4, 1e-4, 3e-5, 1e-5, 3e-6, 1e-6, 3e-7, 1e-7,
];

/// Q1: polytopes per facet count. 20 gives 600 traces total (6 F-values x 20 x
/// 5 dirs), enough for stable slope medians. Runtime scales linearly.
const Q1_POLYTOPES_PER_F: usize = 20;

/// Skip Q2 polytopes with F > this to avoid slow ehz_capacity calls.
/// LP(5,5) has F=10 (~3 min per ehz_capacity call in v1). F<=8 is tractable.
const MAX_FACET_Q2: usize = 8;

/// Smoke-test settings for Phase 2.
const SMOKE_Q1_FACET_COUNTS: &[usize] = &[5];
const SMOKE_Q1_POLYTOPES_PER_F: usize = 1;
const SMOKE_Q2_REGULAR_PAIRS: &[(usize, usize)] = &[(3, 3)];
const SMOKE_Q2_ROTATION_ANGLES: &[f64] = &[PI / 7.0];
const SMOKE_Q2_RANDOM_PAIRS: &[(usize, usize)] = &[(3, 3)];
const SMOKE_Q2_RANDOM_PER_PAIR: usize = 1;
const SMOKE_N_DIRS: usize = 1;

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

fn ehz_capacity_safe(polytope: &Polytope4D) -> Option<symplectic::EhzResult> {
    ehz_capacity(polytope)
}

fn solve_kkt_safe(polytope: &Polytope4D, perm: &[usize]) -> Option<KktResult> {
    solve_kkt_for(polytope, perm).feasible()
}

/// Compute dsys/da_k via quotient rule: sys = c^2/(2*vol).
/// dsys/da_k = (c*dc/da_k - sys*dvol/da_k) / vol.
/// [cor:sys-derivative] quotient-rule derivative of the systolic ratio.
/// In formal/library/algorithms.tex.
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

    PerturbedValues {
        capacity: cap,
        volume: vol,
        sys,
    }
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
    let g_cap = capacity_derivatives_a_from_kkt_result(&info.polytope, &info.best_perm, &info.kkt);
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
        let gd: Vec<f64> = targets
            .iter()
            .map(|(_, _, g)| directional_derivative_a(g, &direction))
            .collect();

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

fn smoke_mode() -> bool {
    std::env::args().skip(1).any(|arg| arg == "--smoke")
}

fn smoke_output_dir(label: &str) -> String {
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock before UNIX_EPOCH")
        .as_millis();
    let dir = std::env::temp_dir().join(format!("{label}-{}-{stamp}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("create smoke output dir");
    dir.to_string_lossy().into_owned()
}

struct BasicValidationConfig {
    q1_facet_counts: &'static [usize],
    q1_polytopes_per_f: usize,
    q2_regular_pairs: &'static [(usize, usize)],
    q2_rotation_angles: &'static [f64],
    q2_random_pairs: &'static [(usize, usize)],
    q2_random_per_pair: usize,
    n_dirs: usize,
}

fn basic_validation_config(smoke: bool) -> BasicValidationConfig {
    if smoke {
        BasicValidationConfig {
            q1_facet_counts: SMOKE_Q1_FACET_COUNTS,
            q1_polytopes_per_f: SMOKE_Q1_POLYTOPES_PER_F,
            q2_regular_pairs: SMOKE_Q2_REGULAR_PAIRS,
            q2_rotation_angles: SMOKE_Q2_ROTATION_ANGLES,
            q2_random_pairs: SMOKE_Q2_RANDOM_PAIRS,
            q2_random_per_pair: SMOKE_Q2_RANDOM_PER_PAIR,
            n_dirs: SMOKE_N_DIRS,
        }
    } else {
        BasicValidationConfig {
            q1_facet_counts: &[5, 6, 7, 8, 9, 10],
            q1_polytopes_per_f: Q1_POLYTOPES_PER_F,
            q2_regular_pairs: &[(3, 3), (3, 4), (4, 4), (3, 5), (4, 5), (5, 5)],
            q2_rotation_angles: &[PI / 7.0, PI / 5.0, PI / 3.0],
            q2_random_pairs: &[(3, 3), (3, 4), (4, 4), (5, 5)],
            q2_random_per_pair: 5,
            n_dirs: N_DIRS,
        }
    }
}

// ============================================================================
// Phases
// ============================================================================

fn run_q1(base_dir: &str, cfg: &BasicValidationConfig) {
    let path = format!("{}/gradient-correctness-q1-generic.jsonl", base_dir);
    let file = File::create(&path).expect("create Q1 JSONL");
    let mut writer = BufWriter::new(file);
    let mut total_rows = 0;

    for &f_count in cfg.q1_facet_counts {
        let mut rng = ChaCha8Rng::seed_from_u64(SEED_BASE + f_count as u64);
        let polytopes =
            generate_random_polytopes(cfg.q1_polytopes_per_f, f_count, 0.5, 2.0, &mut rng);

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
                &info, "q1", &id, "random", cfg.n_dirs, &mut rng, None, None, None,
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

fn run_q2(base_dir: &str, cfg: &BasicValidationConfig) {
    let path = format!("{}/gradient-correctness-q2-nongeneric.jsonl", base_dir);
    let file = File::create(&path).expect("create Q2 JSONL");
    let mut writer = BufWriter::new(file);
    let mut total_rows = 0;

    let mut rng = ChaCha8Rng::seed_from_u64(SEED_BASE + 200);

    // Regular Lagrangian products
    for &(n1, n2) in cfg.q2_regular_pairs {
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
                cfg.n_dirs,
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
    for &(n1, n2) in cfg.q2_regular_pairs {
        if n1 + n2 > MAX_FACET_Q2 {
            continue;
        }
        let (qn, qh) = regular_polygon_2d(n1, 1.0);
        for (ai, &theta) in cfg.q2_rotation_angles.iter().enumerate() {
            let (pn, ph) = regular_polygon_2d(n2, 1.0);
            let (pn_rot, ph_rot) = rotate_polygon_2d(&pn, &ph, theta);
            let polytope = lagrangian_product(&qn, &qh, &pn_rot, &ph_rot).expect("rotated LP");
            let id = format!("lp_rotated_{}_{}_{}", n1, n2, ai);

            if let Some(info) = analyze_polytope(&polytope) {
                let rows = first_order_test(
                    &info,
                    "q2",
                    &id,
                    "lagrangian_rotated",
                    cfg.n_dirs,
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
    for &(n1, n2) in cfg.q2_random_pairs {
        if n1 + n2 > MAX_FACET_Q2 {
            continue;
        }
        for j in 0..cfg.q2_random_per_pair {
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
                    cfg.n_dirs,
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

// ============================================================================
// Main
// ============================================================================

fn main() {
    let smoke = smoke_mode();
    let cfg = basic_validation_config(smoke);
    let smoke_dir;
    let base_dir = if smoke {
        smoke_dir = smoke_output_dir("dev-numerics-smoke");
        println!("Smoke output: {smoke_dir}");
        smoke_dir.as_str()
    } else {
        "."
    };

    println!(
        "=== Gradient Correctness: Basic Validation (Q1 + Q2){} ===\n",
        if smoke { " [smoke]" } else { "" }
    );
    let t0 = Instant::now();

    println!("--- Q1: Generic random polytopes ---");
    let tp = Instant::now();
    run_q1(base_dir, &cfg);
    println!("  Q1 time: {:.1}s\n", tp.elapsed().as_secs_f64());

    println!("--- Q2: Non-generic geometry (Lagrangian products) ---");
    let tp = Instant::now();
    run_q2(base_dir, &cfg);
    println!("  Q2 time: {:.1}s\n", tp.elapsed().as_secs_f64());

    println!("=== Total time: {:.1}s ===", t0.elapsed().as_secs_f64());
}

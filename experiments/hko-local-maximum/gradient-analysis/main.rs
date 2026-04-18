//! Gradient analysis of HKO2024: analytical sensitivity + gradient ascent in F=10 space.
//!
//! Goal: Measure first-order sensitivity at HKO2024 and run local gradient-ascent
//! probes within the 10-facet space.
//! Input Artifacts: None (starts from the hardcoded HKO2024 polytope).
//! Output Artifacts: experiments/hko-local-maximum/gradient-analysis/hko-neighborhood-sensitivity.jsonl
//!         experiments/hko-local-maximum/gradient-analysis/hko-neighborhood-ascent.jsonl
//!
//! Computes analytical derivatives d_sys/d_h_k and d_sys/d_n_k at the exact HKO2024
//! polytope, then runs gradient ascent in joint (h, n) space. Tracks all near-optimal
//! Reeb orbits (subdifferential structure).
//!
//! Split from gradient-is-zero/main.rs (Phase A).
//!
//! Architecture:
//! 1. `cargo run --bin hko-gradient-analysis --release` generates datasets
//! 2. Writes hko-neighborhood-sensitivity.jsonl and hko-neighborhood-ascent.jsonl
//! 3. Python script (analyze.py) reads JSONL, produces figures
//!
//! KKT convention: this experiment now stores the shared `OrbitKktData` payload
//! directly. That keeps the library's symmetric multiplier convention
//! (`Hβ + Nμ + ηξ = 0`) instead of re-labeling it into a local asymmetric
//! variant before derivative consumers use it again.

use nalgebra::{Matrix4, Vector4};
use serde::Serialize;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::time::Instant;
use exp_hko_local_maximum::ehz_capacity_instrumented;
use real_algebraic::{Algebraic, OrderedField, TanPiFifth};
use symplectic::algorithms::OrbitKktData;
use symplectic::derivatives::{capacity_derivatives_a_from_orbit, volume_derivatives_a};
use symplectic::ehz_capacity;
use symplectic::exact::{capacity_derivatives_a_exact, solve_orbit_sigma_exact, ExactPolytope4D};
use symplectic::geom::known_polytopes;
use symplectic::geom::polytope::Polytope4D;
use symplectic::geom::skeleton::Skeleton;
use symplectic::geom::volume::volume;
use symplectic::omega0;

/// Gap threshold for near-optimal orbits: collect orbits within δ of best.
/// 1% is generous — in practice, HKO2024's 44 near-optimal orbits all have
/// gaps < 5e-14 (machine precision). The threshold just needs to be well above
/// machine epsilon to capture all degenerate orbits, while small enough to
/// exclude genuinely suboptimal ones. Any value in [1e-6, 0.1] gives the same
/// result on HKO2024.
const NEAR_OPTIMAL_GAP: f64 = 0.01;

/// Maximum step size cap.
const MAX_STEP_SIZE: f64 = 100.0;

/// Maximum number of gradient ascent iterations.
const MAX_ASCENT_ITERATIONS: usize = 50;

/// Convergence threshold for gradient ascent (minimum improvement per iteration).
const CONVERGENCE_THRESHOLD: f64 = 1e-8;

/// Armijo sufficient decrease parameter (c in f(x + t*d) >= f(x) + c*t*∇f·d).
const ARMIJO_C: f64 = 1e-4;

/// Backtracking factor for Armijo line search.
const BACKTRACKING_FACTOR: f64 = 0.5;

/// Minimum step fraction (give up below this).
const MIN_STEP_FRACTION: f64 = 1e-12;

/// Numerical zero threshold for gradient components and rates.
/// Near machine epsilon (~1e-16); guards against treating f64 noise as
/// a meaningful direction or rate. Used in step bounds and gradient checks.
const EPS_NUMERICAL_ZERO: f64 = 1e-15;

// ============================================================================
// Output schemas
// ============================================================================

#[derive(Debug, Serialize)]
struct SensitivityRow {
    name: String,
    facet_count: usize,
    dual_vertices: Vec<[f64; 4]>,
    volume: f64,
    capacity: f64,
    sys: f64,
    // Near-optimal orbit tracking
    n_valid_orbits: usize,
    n_near_optimal: usize,
    near_optimal_gap: f64,
    orbits: Vec<OrbitInfo>,
    // Derived h/n gradients
    d_sys_h: Vec<f64>,
    gradient_norm_h: f64,
    d_sys_n: Vec<[f64; 4]>,
    gradient_norm_n: f64,
    // Combined
    gradient_norm_hn: f64,
    // Step bounds
    t_max_h: f64,
    t_max_hn: f64,
    // Per-orbit gradients (subdifferential)
    per_orbit_d_sys_h: Vec<Vec<f64>>,
    per_orbit_gradient_norm_h: Vec<f64>,
    exact_best_sigma: Option<ExactBestSigmaDiagnostics>,
    // Timing
    time_instrumented_ms: f64,
    time_sensitivity_ms: f64,
}

#[derive(Debug, Serialize)]
struct OrbitInfo {
    subset: Vec<usize>,
    permutation: Vec<usize>,
    action: f64,
    relative_gap: f64,
    beta: Vec<f64>,
    q_value: f64,
}

#[derive(Debug, Serialize)]
struct AscentRow {
    iteration: usize,
    step_type: String, // "h_only" or "h_n"
    t_actual: f64,
    t_max: f64,
    sys_before: f64,
    sys_after: f64,
    delta_sys: f64,
    volume: f64,
    capacity: f64,
    // Orbit tracking
    best_subset: Vec<usize>,
    best_permutation: Vec<usize>,
    orbit_switched: bool,
    n_near_optimal: usize,
    // Gradient info
    gradient_norm_h: f64,
    gradient_norm_hn: f64,
    // State
    dual_vertices: Vec<[f64; 4]>,
    time_ms: f64,
}

#[derive(Debug, Serialize)]
struct ExactBestSigmaDiagnostics {
    sigma: Vec<usize>,
    q_exact_f64: f64,
    action_exact_f64: f64,
    max_abs_q_diff_vs_float: f64,
    max_abs_beta_diff_vs_float: f64,
    exact_capacity_gradient_a: Vec<[f64; 4]>,
    float_capacity_gradient_a: Vec<[f64; 4]>,
    max_abs_capacity_gradient_diff: f64,
}

// ============================================================================
// Instrumented HK2017 — collects ALL valid orbits
// ============================================================================

fn subset_of_sigma(sigma: &[usize]) -> Vec<usize> {
    let mut subset = sigma.to_vec();
    subset.sort_unstable();
    subset
}

// ============================================================================
// Sensitivity computation — uses library derivative functions
// ============================================================================

struct SensitivityResult {
    d_cap_a: Vec<Vector4<f64>>,
    d_sys_h: Vec<f64>,
    gradient_norm_h: f64,
    d_sys_n: Vec<Vector4<f64>>,
    gradient_norm_n: f64,
    gradient_norm_hn: f64,
}

fn compute_sensitivity(
    polytope: &Polytope4D,
    vol: f64,
    cap: f64,
    sys: f64,
    orbit: &OrbitKktData,
) -> SensitivityResult {
    let duals = polytope.dual_vertices_f64();
    let f = duals.len();

    let d_vol_a = volume_derivatives_a(polytope);
    let d_cap_a = capacity_derivatives_a_from_orbit(polytope, orbit)
        .expect("gradient-analysis stores orbit payloads with closure multipliers");

    let d_sys_a: Vec<Vector4<f64>> = d_vol_a
        .iter()
        .zip(d_cap_a.iter())
        .map(|(dv, dc)| (cap * dc - sys * dv) / vol)
        .collect();

    // Derive h/n gradients from dual vertex gradient
    let d_sys_h: Vec<f64> = (0..f)
        .map(|k| {
            let a_norm = duals[k].norm();
            let n = duals[k] / a_norm;
            let h = 1.0 / a_norm;
            d_sys_a[k].dot(&(-n / (h * h)))
        })
        .collect();

    let gradient_norm_h = d_sys_h.iter().map(|x| x * x).sum::<f64>().sqrt();

    let d_sys_n: Vec<Vector4<f64>> = (0..f)
        .map(|k| {
            let a_norm = duals[k].norm();
            let n = duals[k] / a_norm;
            let h = 1.0 / a_norm;
            let proj = d_sys_a[k] / h - (d_sys_a[k].dot(&n) / h) * n;
            proj
        })
        .collect();

    let gradient_norm_n = d_sys_n.iter().map(|v| v.norm_squared()).sum::<f64>().sqrt();

    let gradient_norm_hn =
        (gradient_norm_h * gradient_norm_h + gradient_norm_n * gradient_norm_n).sqrt();

    SensitivityResult {
        d_cap_a,
        d_sys_h,
        gradient_norm_h,
        d_sys_n,
        gradient_norm_n,
        gradient_norm_hn,
    }
}

type TanPiFifthField = Algebraic<TanPiFifth>;

fn exact_hko_polytope() -> ExactPolytope4D<TanPiFifthField> {
    let z = TanPiFifthField::zero();
    let one = TanPiFifthField::one();
    let t = TanPiFifthField::generator();
    let t2 = t.clone() * t.clone();
    let t3 = t2.clone() * t.clone();

    let a = (TanPiFifthField::one() + t2.clone()) / TanPiFifthField::from_i64(4);
    let b = (TanPiFifthField::from_i64(7) * t.clone() - t3.clone()) / TanPiFifthField::from_i64(4);
    let sec36 = (TanPiFifthField::from_i64(3) - t2.clone()) / TanPiFifthField::from_i64(2);

    ExactPolytope4D::new(vec![
        [one.clone(), t.clone(), z.clone(), z.clone()],
        [-a.clone(), b.clone(), z.clone(), z.clone()],
        [-sec36.clone(), z.clone(), z.clone(), z.clone()],
        [-a.clone(), -b.clone(), z.clone(), z.clone()],
        [one.clone(), -t.clone(), z.clone(), z.clone()],
        [z.clone(), z.clone(), t.clone(), -one.clone()],
        [z.clone(), z.clone(), b.clone(), a.clone()],
        [z.clone(), z.clone(), z.clone(), sec36.clone()],
        [z.clone(), z.clone(), -b, a],
        [z.clone(), z.clone(), -t, -one],
    ])
    .expect("exact HKO pentagon polytope")
}

fn max_abs_vector_diff(lhs: &[Vector4<f64>], rhs: &[[f64; 4]]) -> f64 {
    lhs.iter()
        .zip(rhs.iter())
        .flat_map(|(left, right)| {
            [0usize, 1, 2, 3]
                .into_iter()
                .map(move |idx| (left[idx] - right[idx]).abs())
        })
        .fold(0.0, f64::max)
}

fn compute_exact_best_sigma_diagnostics(best_orbit: &OrbitKktData, float_d_cap_a: &[Vector4<f64>]) -> Option<ExactBestSigmaDiagnostics> {
    let exact_polytope = exact_hko_polytope();
    let exact_orbit = solve_orbit_sigma_exact(&exact_polytope, &best_orbit.sigma)?;
    let exact_capacity_gradient = capacity_derivatives_a_exact(&exact_polytope, &exact_orbit);
    let exact_capacity_gradient_f64: Vec<[f64; 4]> = exact_capacity_gradient
        .iter()
        .map(|grad| std::array::from_fn(|idx| grad[idx].to_f64()))
        .collect();

    let exact_beta_f64: Vec<f64> = exact_orbit.beta.iter().map(OrderedField::to_f64).collect();
    let max_abs_beta_diff_vs_float = exact_beta_f64
        .iter()
        .zip(best_orbit.beta.iter())
        .map(|(lhs, rhs)| (lhs - rhs).abs())
        .fold(0.0, f64::max);
    let q_exact_f64 = exact_orbit.q.to_f64();
    let action_exact_f64 = exact_orbit.action().to_f64();

    Some(ExactBestSigmaDiagnostics {
        sigma: exact_orbit.sigma,
        q_exact_f64,
        action_exact_f64,
        max_abs_q_diff_vs_float: (q_exact_f64 - best_orbit.q).abs(),
        max_abs_beta_diff_vs_float,
        max_abs_capacity_gradient_diff: max_abs_vector_diff(float_d_cap_a, &exact_capacity_gradient_f64),
        exact_capacity_gradient_a: exact_capacity_gradient_f64,
        float_capacity_gradient_a: float_d_cap_a
            .iter()
            .map(|grad| [grad[0], grad[1], grad[2], grad[3]])
            .collect(),
    })
}

// ============================================================================
// Step bounds computation (experiment-specific: topology-aware step size limits)
// Same math as [lem:step-bound-incidence] and [lem:step-bound-omega] in
// formal/combinatorial-cells/boundary-characterization.tex, adapted for (h,n) space.
// TODO: add [lem:step-bound-hn] to formal/hko-local-maximum/gradient-analysis.tex for the (h,n) variant.
// ============================================================================

fn compute_step_bound(polytope: &Polytope4D, direction: &[f64]) -> f64 {
    let duals = polytope.dual_vertices_f64();
    let normals: Vec<Vector4<f64>> = duals.iter().map(|a| a / a.norm()).collect();
    let heights: Vec<f64> = duals.iter().map(|a| 1.0 / a.norm()).collect();
    let vertices = polytope.vertices_f64();
    let f = polytope.facet_count();
    let skeleton = Skeleton::compute(polytope);

    let mut t_max = f64::INFINITY;

    for (vi, vertex_facets) in skeleton.vertex_facets.iter().enumerate() {
        let v = &vertices[vi];

        if vertex_facets.len() == 4 {
            let det_facets = &vertex_facets;
            let n_mat = Matrix4::from_rows(&[
                normals[det_facets[0]].transpose(),
                normals[det_facets[1]].transpose(),
                normals[det_facets[2]].transpose(),
                normals[det_facets[3]].transpose(),
            ]);

            let n_inv = match n_mat.try_inverse() {
                Some(inv) => inv,
                None => continue,
            };

            let g_det = Vector4::new(
                direction[det_facets[0]],
                direction[det_facets[1]],
                direction[det_facets[2]],
                direction[det_facets[3]],
            );
            let dv_dt = n_inv * g_det;

            for j in 0..f {
                if vertex_facets.contains(&j) {
                    continue;
                }
                let slack = heights[j] - normals[j].dot(v);
                let rate = direction[j] - normals[j].dot(&dv_dt);
                if rate < -EPS_NUMERICAL_ZERO {
                    let t_crit = slack / (-rate);
                    if t_crit > 0.0 && t_crit < t_max {
                        t_max = t_crit;
                    }
                }
            }
        } else {
            // Over-determined vertex (>4 incident facets): we cannot invert the
            // normal matrix, so we use a conservative bound: t ≤ slack / max|g_k|.
            // This is safe but may over-tighten the step. In practice, HKO2024 is
            // a simple polytope (all vertices have exactly 4 incident facets), so
            // this branch is never reached.
            for j in 0..f {
                if vertex_facets.contains(&j) {
                    continue;
                }
                let slack = heights[j] - normals[j].dot(v);
                if direction[j] < -EPS_NUMERICAL_ZERO {
                    continue;
                }
                let max_g = direction.iter().map(|x| x.abs()).fold(0.0f64, f64::max);
                if max_g > EPS_NUMERICAL_ZERO {
                    let t_crit = slack / max_g;
                    if t_crit > 0.0 && t_crit < t_max {
                        t_max = t_crit;
                    }
                }
            }
        }
    }

    for k in 0..f {
        if direction[k] < -EPS_NUMERICAL_ZERO {
            let t_crit = heights[k] / (-direction[k]);
            if t_crit > 0.0 && t_crit < t_max {
                t_max = t_crit;
            }
        }
    }

    t_max.min(MAX_STEP_SIZE)
}

fn compute_step_bound_hn(polytope: &Polytope4D, g_h: &[f64], g_n: &[Vector4<f64>]) -> f64 {
    let duals = polytope.dual_vertices_f64();
    let normals: Vec<Vector4<f64>> = duals.iter().map(|a| a / a.norm()).collect();
    let heights: Vec<f64> = duals.iter().map(|a| 1.0 / a.norm()).collect();
    let vertices = polytope.vertices_f64();
    let f = polytope.facet_count();
    let skeleton = Skeleton::compute(polytope);

    let mut t_max = f64::INFINITY;

    for (vi, vertex_facets) in skeleton.vertex_facets.iter().enumerate() {
        let v = &vertices[vi];

        if vertex_facets.len() == 4 {
            let det_facets = vertex_facets;
            let n_mat = Matrix4::from_rows(&[
                normals[det_facets[0]].transpose(),
                normals[det_facets[1]].transpose(),
                normals[det_facets[2]].transpose(),
                normals[det_facets[3]].transpose(),
            ]);

            let n_inv = match n_mat.try_inverse() {
                Some(inv) => inv,
                None => continue,
            };

            let rhs = Vector4::new(
                g_h[det_facets[0]] - g_n[det_facets[0]].dot(v),
                g_h[det_facets[1]] - g_n[det_facets[1]].dot(v),
                g_h[det_facets[2]] - g_n[det_facets[2]].dot(v),
                g_h[det_facets[3]] - g_n[det_facets[3]].dot(v),
            );
            let dv_dt = n_inv * rhs;

            for j in 0..f {
                if vertex_facets.contains(&j) {
                    continue;
                }
                let slack = heights[j] - normals[j].dot(v);
                let rate = g_h[j] - g_n[j].dot(v) - normals[j].dot(&dv_dt);
                if rate < -EPS_NUMERICAL_ZERO {
                    let t_crit = slack / (-rate);
                    if t_crit > 0.0 && t_crit < t_max {
                        t_max = t_crit;
                    }
                }
            }
        } else {
            for j in 0..f {
                if vertex_facets.contains(&j) {
                    continue;
                }
                let slack = heights[j] - normals[j].dot(v);
                let max_g_h = g_h.iter().map(|x| x.abs()).fold(0.0f64, f64::max);
                let max_g_n = g_n.iter().map(|g| g.norm()).fold(0.0f64, f64::max);
                let max_rate = max_g_h + max_g_n * v.norm();
                if max_rate > EPS_NUMERICAL_ZERO {
                    let t_crit = slack / max_rate;
                    if t_crit > 0.0 && t_crit < t_max {
                        t_max = t_crit;
                    }
                }
            }
        }
    }

    for k in 0..f {
        if g_h[k] < -EPS_NUMERICAL_ZERO {
            let t_crit = heights[k] / (-g_h[k]);
            if t_crit > 0.0 && t_crit < t_max {
                t_max = t_crit;
            }
        }
    }

    // ω₀ sign preservation for ridge-adjacent pairs
    for ridge in &skeleton.ridges {
        let i = ridge.facets[0];
        let j = ridge.facets[1];
        let omega_ij = omega0(&normals[i], &normals[j]);
        let d_omega = omega0(&g_n[i], &normals[j]) + omega0(&normals[i], &g_n[j]);
        if omega_ij.abs() > EPS_NUMERICAL_ZERO && d_omega.abs() > EPS_NUMERICAL_ZERO {
            let t_flip = -omega_ij / d_omega;
            if t_flip > 0.0 && t_flip < t_max {
                t_max = t_flip;
            }
        }
    }

    t_max.min(MAX_STEP_SIZE)
}

// ============================================================================
// Gradient step helpers
// ============================================================================

/// Safely compute sys for a polytope, catching panics from degenerate geometry.
fn safe_sys(polytope: &Polytope4D) -> Option<(f64, f64, f64)> {
    let vol = volume(polytope).unwrap_or(0.0);
    if vol <= 0.0 {
        return None;
    }
    let cap = ehz_capacity(polytope)
        .ok()
        .map(|r| r.capacity())
        .unwrap_or(f64::NAN);
    if !cap.is_finite() {
        return None;
    }
    let sys = cap * cap / (2.0 * vol);
    if sys.is_finite() {
        Some((sys, vol, cap))
    } else {
        None
    }
}

fn try_step_h(
    normals: &[Vector4<f64>],
    heights: &[f64],
    direction: &[f64],
    t: f64,
) -> Option<(Polytope4D, f64, f64, f64)> {
    let f = normals.len();
    let new_heights: Vec<f64> = (0..f).map(|k| heights[k] + t * direction[k]).collect();

    let new_polytope = Polytope4D::from_f64(
        normals
            .iter()
            .zip(new_heights.iter())
            .map(|(n, &h)| n / h)
            .collect(),
    )
    .ok()?;
    let (sys, vol, cap) = safe_sys(&new_polytope)?;
    Some((new_polytope, sys, vol, cap))
}

fn try_step_hn(
    normals: &[Vector4<f64>],
    heights: &[f64],
    g_h: &[f64],
    g_n: &[Vector4<f64>],
    t: f64,
) -> Option<(Polytope4D, f64, f64, f64)> {
    let f = normals.len();
    let new_heights: Vec<f64> = (0..f).map(|k| heights[k] + t * g_h[k]).collect();
    let new_normals: Vec<Vector4<f64>> = (0..f)
        .map(|k| {
            let n = normals[k] + t * g_n[k];
            n / n.norm()
        })
        .collect();

    let new_polytope = Polytope4D::from_f64(
        new_normals
            .iter()
            .zip(new_heights.iter())
            .map(|(n, &h)| n / h)
            .collect(),
    )
    .ok()?;
    let (sys, vol, cap) = safe_sys(&new_polytope)?;
    Some((new_polytope, sys, vol, cap))
}

// ============================================================================
// Armijo backtracking line search
// ============================================================================

/// Armijo backtracking line search for height-only steps.
/// Returns (polytope, sys, vol, cap, t_actual) or None if no improvement.
fn armijo_step_h(
    polytope: &Polytope4D,
    d_sys_h: &[f64],
    t_max: f64,
    current_sys: f64,
) -> Option<(Polytope4D, f64, f64, f64, f64)> {
    let duals = polytope.dual_vertices_f64();
    let normals: Vec<Vector4<f64>> = duals.iter().map(|a| a / a.norm()).collect();
    let heights: Vec<f64> = duals.iter().map(|a| 1.0 / a.norm()).collect();
    let grad_dot_dir: f64 = d_sys_h.iter().map(|x| x * x).sum(); // ∇f · d = |∇f|² (ascending)

    let mut t = 0.95 * t_max;
    while t > MIN_STEP_FRACTION * t_max {
        if let Some((new_poly, new_sys, vol, cap)) = try_step_h(&normals, &heights, d_sys_h, t) {
            // Armijo condition: f(x + td) >= f(x) + c·t·∇f·d
            if new_sys >= current_sys + ARMIJO_C * t * grad_dot_dir {
                return Some((new_poly, new_sys, vol, cap, t));
            }
        }
        t *= BACKTRACKING_FACTOR;
    }
    None
}

/// Armijo backtracking line search for (h,n) steps.
fn armijo_step_hn(
    polytope: &Polytope4D,
    d_sys_h: &[f64],
    d_sys_n: &[Vector4<f64>],
    t_max: f64,
    current_sys: f64,
) -> Option<(Polytope4D, f64, f64, f64, f64)> {
    let duals = polytope.dual_vertices_f64();
    let normals: Vec<Vector4<f64>> = duals.iter().map(|a| a / a.norm()).collect();
    let heights: Vec<f64> = duals.iter().map(|a| 1.0 / a.norm()).collect();
    let grad_dot_dir: f64 = d_sys_h.iter().map(|x| x * x).sum::<f64>()
        + d_sys_n.iter().map(|v| v.norm_squared()).sum::<f64>();

    let mut t = 0.95 * t_max;
    while t > MIN_STEP_FRACTION * t_max {
        if let Some((new_poly, new_sys, vol, cap)) =
            try_step_hn(&normals, &heights, d_sys_h, d_sys_n, t)
        {
            if new_sys >= current_sys + ARMIJO_C * t * grad_dot_dir {
                return Some((new_poly, new_sys, vol, cap, t));
            }
        }
        t *= BACKTRACKING_FACTOR;
    }
    None
}

// ============================================================================
// Phase A: Main analysis
// ============================================================================

fn run_phase_a(base_dir: &std::path::Path, smoke: bool) {
    println!("═══════════════════════════════════════════════════════════");
    println!("Phase A: HKO2024 sensitivity + gradient ascent (F=10)");
    println!("═══════════════════════════════════════════════════════════\n");

    // Load HKO2024
    let known = known_polytopes::hko_pentagon();
    let polytope = &known.polytope;
    let f = polytope.facet_count();
    println!("HKO2024: F={f}, known capacity={:.6}", known.capacity);

    // Cross-check with library
    let lib_result = ehz_capacity(polytope).expect("library ehz_capacity failed");
    println!(
        "  Library capacity: {:.10} (diff from known: {:.2e})",
        lib_result.capacity(),
        (lib_result.capacity() - known.capacity).abs()
    );

    // Instrumented HK2017
    println!("\nRunning instrumented HK2017...");
    let t_instr = Instant::now();
    let instrumented = ehz_capacity_instrumented(polytope).expect("no valid orbits for HKO2024");
    let time_instrumented_ms = t_instr.elapsed().as_secs_f64() * 1000.0;

    // Cross-check
    let cap_diff = (instrumented.capacity - lib_result.capacity()).abs();
    assert!(
        cap_diff < 1e-8,
        "Capacity mismatch: instrumented={:.10}, library={:.10}",
        instrumented.capacity,
        lib_result.capacity()
    );
    println!(
        "  Instrumented capacity: {:.10} (matches library, diff={:.2e})",
        instrumented.capacity, cap_diff
    );

    let cap = instrumented.capacity;
    let vol = volume(polytope).expect("volume failed");
    let sys = cap * cap / (2.0 * vol);
    println!("  Volume: {vol:.10}");
    println!("  Sys: {sys:.10}");
    println!("  Total valid orbits: {}", instrumented.orbits.len());
    println!("  Computation time: {time_instrumented_ms:.1}ms");

    // Near-optimal orbit analysis
    let best_action = instrumented.orbits[0].action;
    let near_optimal: Vec<&OrbitKktData> = instrumented
        .orbits
        .iter()
        .filter(|o| (o.action - best_action) / best_action < NEAR_OPTIMAL_GAP)
        .collect();

    println!("\n--- Near-optimal orbits (gap < {NEAR_OPTIMAL_GAP}) ---");
    println!(
        "  Count: {} (of {} total)",
        near_optimal.len(),
        instrumented.orbits.len()
    );
    for (i, orbit) in near_optimal.iter().enumerate() {
        let gap = (orbit.action - best_action) / best_action;
        println!(
            "  #{}: S={:?}, σ={:?}, action={:.10}, gap={:.6e}",
            i,
            subset_of_sigma(&orbit.sigma),
            orbit.sigma,
            orbit.action,
            gap
        );
    }

    // Also show a few more orbits for context
    println!("\n--- Orbit action distribution (first 10) ---");
    for (i, orbit) in instrumented.orbits.iter().take(10).enumerate() {
        let gap = (orbit.action - best_action) / best_action;
        println!(
            "  #{}: action={:.6}, gap={:.4e}, |S|={}, S={:?}",
            i,
            orbit.action,
            gap,
            orbit.sigma.len(),
            subset_of_sigma(&orbit.sigma)
        );
    }

    // Sensitivity for best orbit
    println!("\n--- Sensitivity (best orbit) ---");
    let t_sens = Instant::now();
    let best_orbit = &instrumented.orbits[0];
    let sensitivity = compute_sensitivity(polytope, vol, cap, sys, best_orbit);
    let exact_best_sigma = compute_exact_best_sigma_diagnostics(best_orbit, &sensitivity.d_cap_a);
    let time_sensitivity_ms = t_sens.elapsed().as_secs_f64() * 1000.0;

    println!("  ∂sys/∂h:");
    for k in 0..f {
        println!("    k={}: d_sys={:.6e}", k, sensitivity.d_sys_h[k]);
    }
    println!("  |∇sys_h| = {:.6e}", sensitivity.gradient_norm_h);
    println!("  |∇sys_n| = {:.6e}", sensitivity.gradient_norm_n);
    println!("  |∇sys_hn| = {:.6e}", sensitivity.gradient_norm_hn);
    if let Some(exact) = &exact_best_sigma {
        println!(
            "  exact best-sigma sidecar: q={:.15}, action={:.15}, |Δq|={:.3e}, |Δbeta|_max={:.3e}, |Δ∂c/∂a|_max={:.3e}",
            exact.q_exact_f64,
            exact.action_exact_f64,
            exact.max_abs_q_diff_vs_float,
            exact.max_abs_beta_diff_vs_float,
            exact.max_abs_capacity_gradient_diff,
        );
    } else {
        println!("  exact best-sigma sidecar: sigma not admissible / solvable in exact mode");
    }

    // Critical point check
    let is_critical = sensitivity.gradient_norm_hn < 1e-6;
    println!(
        "\n  Critical point check: |∇sys| = {:.6e} → {}",
        sensitivity.gradient_norm_hn,
        if is_critical {
            "YES — HKO2024 is a critical point"
        } else {
            "NO — gradient is nonzero"
        }
    );

    if smoke {
        println!("\n  Smoke mode: stopping after sensitivity computation.");
        return;
    }

    // Step bounds
    let t_max_h = if sensitivity.gradient_norm_h > EPS_NUMERICAL_ZERO {
        compute_step_bound(polytope, &sensitivity.d_sys_h)
    } else {
        0.0
    };
    let t_max_hn = if sensitivity.gradient_norm_hn > EPS_NUMERICAL_ZERO {
        compute_step_bound_hn(polytope, &sensitivity.d_sys_h, &sensitivity.d_sys_n)
    } else {
        0.0
    };
    println!("  t_max_h = {t_max_h:.6e}");
    println!("  t_max_hn = {t_max_hn:.6e}");

    // Per-orbit sensitivity (subdifferential)
    println!("\n--- Per-orbit gradients (subdifferential) ---");
    let mut per_orbit_d_sys_h: Vec<Vec<f64>> = Vec::new();
    let mut per_orbit_gradient_norm_h: Vec<f64> = Vec::new();

    for (i, orbit) in near_optimal.iter().enumerate() {
        let orbit_sens = compute_sensitivity(polytope, vol, cap, sys, orbit);
        let norm = orbit_sens.gradient_norm_h;
        println!(
            "  Orbit #{}: |∇sys_h| = {:.6e}, d_sys_h = {:?}",
            i,
            norm,
            orbit_sens
                .d_sys_h
                .iter()
                .map(|x| format!("{:.4e}", x))
                .collect::<Vec<_>>()
        );
        per_orbit_d_sys_h.push(orbit_sens.d_sys_h);
        per_orbit_gradient_norm_h.push(norm);
    }

    // Write sensitivity JSONL
    let sens_path = base_dir.join("gradient-analysis/hko-neighborhood-sensitivity.jsonl");
    let sens_file = File::create(&sens_path).expect("create sensitivity JSONL");
    let mut sens_writer = BufWriter::new(sens_file);

    let duals_raw: Vec<[f64; 4]> = polytope
        .dual_vertices_f64()
        .iter()
        .map(|a| [a[0], a[1], a[2], a[3]])
        .collect();
    let d_sys_n_raw: Vec<[f64; 4]> = sensitivity
        .d_sys_n
        .iter()
        .map(|v| [v[0], v[1], v[2], v[3]])
        .collect();

    let orbit_infos: Vec<OrbitInfo> = near_optimal
        .iter()
        .map(|o| OrbitInfo {
            subset: subset_of_sigma(&o.sigma),
            permutation: o.sigma.clone(),
            action: o.action,
            relative_gap: (o.action - best_action) / best_action,
            beta: o.beta.clone(),
            q_value: o.q,
        })
        .collect();

    let sens_row = SensitivityRow {
        name: "hko_pentagon".to_string(),
        facet_count: f,
        dual_vertices: duals_raw,
        volume: vol,
        capacity: cap,
        sys,
        n_valid_orbits: instrumented.orbits.len(),
        n_near_optimal: near_optimal.len(),
        near_optimal_gap: NEAR_OPTIMAL_GAP,
        orbits: orbit_infos,
        d_sys_h: sensitivity.d_sys_h.clone(),
        gradient_norm_h: sensitivity.gradient_norm_h,
        d_sys_n: d_sys_n_raw,
        gradient_norm_n: sensitivity.gradient_norm_n,
        gradient_norm_hn: sensitivity.gradient_norm_hn,
        t_max_h,
        t_max_hn,
        per_orbit_d_sys_h,
        per_orbit_gradient_norm_h,
        exact_best_sigma,
        time_instrumented_ms,
        time_sensitivity_ms,
    };
    serde_json::to_writer(&mut sens_writer, &sens_row).expect("write sensitivity");
    writeln!(sens_writer).expect("newline");
    sens_writer.flush().expect("flush sensitivity");
    println!("\n  Wrote {}", sens_path.display());

    // =========================================================================
    // Gradient ascent with Armijo backtracking
    // =========================================================================

    println!("\n--- Gradient ascent with Armijo backtracking ---");

    let ascent_path = base_dir.join("gradient-analysis/hko-neighborhood-ascent.jsonl");
    let ascent_file = File::create(&ascent_path).expect("create ascent JSONL");
    let mut ascent_writer = BufWriter::new(ascent_file);

    let mut current =
        Polytope4D::from_f64(polytope.dual_vertices_f64().to_vec()).expect("reconstruct HKO2024");
    let mut current_sys = sys;
    let mut prev_subset = subset_of_sigma(&instrumented.orbits[0].sigma);
    let mut prev_perm = instrumented.orbits[0].sigma.clone();

    for iter in 0..MAX_ASCENT_ITERATIONS {
        let t_iter = Instant::now();

        // Recompute instrumented capacity
        let instr = match ehz_capacity_instrumented(&current) {
            Some(r) => r,
            None => {
                println!("  Iter {iter}: no valid orbits, stopping");
                break;
            }
        };
        let cap = instr.capacity;
        let vol = volume(&current).expect("volume");
        let sys_now = cap * cap / (2.0 * vol);
        let best_orbit = &instr.orbits[0];

        // Orbit switch detection
        let orbit_switched =
            subset_of_sigma(&best_orbit.sigma) != prev_subset || best_orbit.sigma != prev_perm;

        // Sensitivity
        let sens = compute_sensitivity(&current, vol, cap, sys_now, best_orbit);

        // Step bounds
        let t_max_h = if sens.gradient_norm_h > EPS_NUMERICAL_ZERO {
            compute_step_bound(&current, &sens.d_sys_h)
        } else {
            0.0
        };
        let t_max_hn = if sens.gradient_norm_hn > EPS_NUMERICAL_ZERO {
            compute_step_bound_hn(&current, &sens.d_sys_h, &sens.d_sys_n)
        } else {
            0.0
        };

        // Try Armijo for both h-only and h+n, pick better
        let step_h = if t_max_h > 0.0 && sens.gradient_norm_h > EPS_NUMERICAL_ZERO {
            armijo_step_h(&current, &sens.d_sys_h, t_max_h, sys_now)
        } else {
            None
        };
        let step_hn = if t_max_hn > 0.0 && sens.gradient_norm_hn > EPS_NUMERICAL_ZERO {
            armijo_step_hn(&current, &sens.d_sys_h, &sens.d_sys_n, t_max_hn, sys_now)
        } else {
            None
        };

        let (new_poly, new_sys, new_vol, new_cap, t_actual, step_type, t_max_used) =
            match (step_h, step_hn) {
                (Some((p1, s1, v1, c1, t1)), Some((p2, s2, v2, c2, t2))) => {
                    if s1 >= s2 {
                        (p1, s1, v1, c1, t1, "h_only", t_max_h)
                    } else {
                        (p2, s2, v2, c2, t2, "h_n", t_max_hn)
                    }
                }
                (Some((p, s, v, c, t)), None) => (p, s, v, c, t, "h_only", t_max_h),
                (None, Some((p, s, v, c, t))) => (p, s, v, c, t, "h_n", t_max_hn),
                (None, None) => {
                    println!("  Iter {iter}: no improving step found — local maximum");

                    // Near-optimal orbits at this point
                    let best_action = instr.orbits[0].action;
                    let n_near = instr
                        .orbits
                        .iter()
                        .filter(|o| (o.action - best_action) / best_action < NEAR_OPTIMAL_GAP)
                        .count();

                    // Write final state
                    let row = AscentRow {
                        iteration: iter,
                        step_type: "none".to_string(),
                        t_actual: 0.0,
                        t_max: t_max_h,
                        sys_before: sys_now,
                        sys_after: sys_now,
                        delta_sys: 0.0,
                        volume: vol,
                        capacity: cap,
                        best_subset: subset_of_sigma(&instr.orbits[0].sigma),
                        best_permutation: instr.orbits[0].sigma.clone(),
                        orbit_switched,
                        n_near_optimal: n_near,
                        gradient_norm_h: sens.gradient_norm_h,
                        gradient_norm_hn: sens.gradient_norm_hn,
                        dual_vertices: current
                            .dual_vertices_f64()
                            .iter()
                            .map(|a| [a[0], a[1], a[2], a[3]])
                            .collect(),
                        time_ms: t_iter.elapsed().as_secs_f64() * 1000.0,
                    };
                    serde_json::to_writer(&mut ascent_writer, &row).expect("write ascent");
                    writeln!(ascent_writer).expect("newline");
                    break;
                }
            };

        let delta = new_sys - sys_now;
        let time_ms = t_iter.elapsed().as_secs_f64() * 1000.0;

        // Near-optimal orbit count at new point
        let new_instr = ehz_capacity_instrumented(&new_poly);
        let n_near = new_instr
            .as_ref()
            .map(|r| {
                let ba = r.orbits[0].action;
                r.orbits
                    .iter()
                    .filter(|o| (o.action - ba) / ba < NEAR_OPTIMAL_GAP)
                    .count()
            })
            .unwrap_or(0);

        let new_subset = new_instr
            .as_ref()
            .map(|r| subset_of_sigma(&r.orbits[0].sigma))
            .unwrap_or_default();
        let new_perm = new_instr
            .as_ref()
            .map(|r| r.orbits[0].sigma.clone())
            .unwrap_or_default();

        println!(
            "  Iter {iter}: {step_type} t={t_actual:.6e} (t_max={t_max_used:.6e}), \
             sys={sys_now:.10}→{new_sys:.10} (Δ={delta:.6e}), \
             orbit_switch={orbit_switched}, near_optimal={n_near}, {time_ms:.0}ms"
        );

        let row = AscentRow {
            iteration: iter,
            step_type: step_type.to_string(),
            t_actual,
            t_max: t_max_used,
            sys_before: sys_now,
            sys_after: new_sys,
            delta_sys: delta,
            volume: new_vol,
            capacity: new_cap,
            best_subset: new_subset.clone(),
            best_permutation: new_perm.clone(),
            orbit_switched,
            n_near_optimal: n_near,
            gradient_norm_h: sens.gradient_norm_h,
            gradient_norm_hn: sens.gradient_norm_hn,
            dual_vertices: new_poly
                .dual_vertices_f64()
                .iter()
                .map(|a| [a[0], a[1], a[2], a[3]])
                .collect(),
            time_ms,
        };
        serde_json::to_writer(&mut ascent_writer, &row).expect("write ascent");
        writeln!(ascent_writer).expect("newline");

        prev_subset = new_subset;
        prev_perm = new_perm;
        current = new_poly;
        current_sys = new_sys;

        if delta < CONVERGENCE_THRESHOLD {
            println!("  Converged (Δ < {CONVERGENCE_THRESHOLD})");
            break;
        }
    }

    let total_improvement = current_sys - sys;
    println!("\n  Ascent summary: sys {sys:.10} → {current_sys:.10} (Δ={total_improvement:.6e})");
    ascent_writer.flush().expect("flush ascent");
    println!("  Wrote {}", ascent_path.display());
}

// ============================================================================
// Main
// ============================================================================

fn main() {
    let t0 = Instant::now();
    let base_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let smoke = std::env::args().any(|a| a == "--smoke");

    std::fs::create_dir_all(base_dir.join("gradient-analysis"))
        .expect("create gradient-analysis output dir");

    println!("Gradient Analysis: HKO2024 sensitivity + gradient ascent\n");

    run_phase_a(base_dir, smoke);

    let elapsed = t0.elapsed().as_secs_f64();
    println!("\n═══════════════════════════════════════════════════════════");
    println!("Total time: {elapsed:.1}s");
    println!("═══════════════════════════════════════════════════════════");
}

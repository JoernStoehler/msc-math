//! Gradient ascent on sys = c_EHZ² / (2 vol) for F=10 polytopes.
//!
//! Two modes:
//! 1. General random F=10 polytopes — uses instrumented HK2017 (exponential enumeration)
//! 2. F=10 Lagrangian products — uses instrumented billiard with Lagrangian-constrained gradient
//!
//! For Lagrangian products, the gradient step preserves the product structure:
//! q-facet normals stay in the q-plane, p-facet normals stay in the p-plane.
//!
//! Architecture:
//! 1. `cargo run --bin gradient_descent --release` generates dataset
//! 2. Writes to gradient-descent/gradient-descent.jsonl
//! 3. Python script reads JSONL, produces figures
//!
//! Input: generates its own starting polytopes (no external data dependency)
//! Output: gradient-descent/gradient-descent.jsonl

#[path = "kkt_instrumented.rs"]
mod kkt_instrumented;

use kkt_instrumented::*;
use nalgebra::{Matrix4, Vector4};
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use serde::Serialize;
use std::collections::HashSet;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::time::Instant;
use symplectic::geom::polygon::random_polygon_2d;
// TODO: These will be re-exported from top-level `symplectic::` in wave 4 (subagent #16).
use symplectic::geom::lagrangian_product::lagrangian_product;
use symplectic::random::sample_random_polytope;
use symplectic::algorithms::billiard::billiard_capacity;
use symplectic::geom::volume::volume;
use symplectic::geom::polytope::Polytope4D;
use symplectic::geom::skeleton::Skeleton;

// ============================================================================
// Configuration
// ============================================================================

const SEED: u64 = 42;

/// Number of general random F=10 polytopes to generate.
const N_GENERAL: usize = 500;

/// Number of Lagrangian product polytopes per split bucket.
const N_LAGRANGIAN_PER_BUCKET: usize = 167;

/// Lagrangian product splits (q_facets, p_facets) summing to 10.
const LAGRANGIAN_SPLITS: &[(usize, usize)] = &[(3, 7), (4, 6), (5, 5)];

/// Facet count for all polytopes.
const FACET_COUNT: usize = 10;

/// Height range for random generation. Centered around 1.0 with ±20% spread
/// to produce polytopes of moderate eccentricity. Narrower spreads produce nearly
/// spherical polytopes (boring), wider spreads produce highly elongated ones
/// (more degenerate KKT solutions, fewer valid orbits).
const H_MIN: f64 = 0.8;
const H_MAX: f64 = 1.2;

/// Maximum number of gradient ascent iterations. 20 is enough for convergence
/// in >90% of cases (most converge in 5-15 iterations). Increasing beyond 20
/// adds runtime but rarely improves final sys.
const MAX_ITERATIONS: usize = 20;

/// Minimum improvement per iteration to continue. 1e-6 is well above f64 noise
/// (~1e-15) but small enough to capture meaningful gradient steps. At this
/// threshold, convergence means <0.0001% change per iteration.
const CONVERGENCE_THRESHOLD: f64 = 1e-6;

/// Step fractions of t_max to evaluate. Geometric-ish spacing from conservative
/// (0.1) to aggressive (0.95). We always pick the fraction giving highest sys,
/// so more fractions = better line search at cost of more capacity evaluations.
/// 5 fractions is a good tradeoff: ~5x cost per iteration vs exhaustive search.
const STEP_FRACTIONS: &[f64] = &[0.1, 0.25, 0.5, 0.75, 0.95];

/// Maximum step size cap. Prevents pathological steps when t_max is unbounded
/// (e.g., gradient nearly parallel to a constraint). Value of 100.0 is generous
/// — typical useful steps are O(0.01)–O(1.0).
const MAX_STEP_SIZE: f64 = 100.0;

// ============================================================================
// Output schema
// ============================================================================

#[derive(Debug, Serialize)]
struct GradientDescentRow {
    name: String,
    polytope_type: String, // "general" or "lagrangian_3x7" etc.
    facet_count: usize,
    iteration: usize,
    step_type: String, // "h_only" or "h_n"
    t_fraction: f64,
    t_actual: f64,
    sys_before: f64,
    sys_after: f64,
    delta_sys: f64,
    starting_sys: f64,
    cumulative_delta: f64,
    gradient_norm_h: f64,
    gradient_norm_n: f64,
    time_ms: f64,
    // Final state (only on last iteration or converged)
    #[serde(skip_serializing_if = "Option::is_none")]
    final_normals: Option<Vec<[f64; 4]>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    final_heights: Option<Vec<f64>>,
}

/// Summary row written once per polytope (after all iterations).
#[derive(Debug, Serialize)]
struct SummaryRow {
    name: String,
    polytope_type: String,
    facet_count: usize,
    starting_sys: f64,
    final_sys: f64,
    total_delta: f64,
    iterations: usize,
    converged: bool,
    total_time_ms: f64,
}

// ============================================================================
// Sensitivity computation (d(sys)/d(h_k) and d(sys)/d(n_k))
// Adapted from sys_optimization.rs
// ============================================================================

struct SensitivityResult {
    d_sys_h: Vec<f64>,
    gradient_norm_h: f64,
    d_sys_n: Vec<Vector4<f64>>,
    gradient_norm_n: f64,
    gradient_norm_hn: f64,
}

/// d(vol)/d(h_k) = S_k (3D volume of facet k).
fn compute_volume_derivatives_h(polytope: &Polytope4D) -> Vec<f64> {
    let normals = polytope.normals_f64();
    let heights = polytope.heights_f64();
    let vertices = polytope.vertices_f64();
    let f = normals.len();
    (0..f)
        .map(|k| facet_volume_3d(&normals, &heights, &vertices, k, f))
        .collect()
}

/// d(c_EHZ)/d(h_k) via envelope theorem.
/// For orbit (S,σ) with KKT solution (β, Q, ν): dA/dh_k = −ν·β_{i₀}/(2Q²)
fn compute_capacity_derivatives_h(best_orbit: &ValidOrbit, facet_count: usize) -> Vec<f64> {
    let q_sq = best_orbit.q_value * best_orbit.q_value;
    (0..facet_count)
        .map(|k| {
            match best_orbit.permutation.iter().position(|&f| f == k) {
                // Lemma lem:cap-derivative: ∂A/∂h_k = ν·β_{i₀}/(2Q²)
                Some(i0) => best_orbit.nu * best_orbit.beta[i0] / (2.0 * q_sq),
                None => 0.0,
            }
        })
        .collect()
}

/// d(vol)/d(n_k) projected onto T_{n_k}S³.
/// Tangent gradient: −S_k(x̄_k − h_k n_k)
fn compute_volume_derivatives_n(polytope: &Polytope4D) -> Vec<Vector4<f64>> {
    let normals = polytope.normals_f64();
    let heights = polytope.heights_f64();
    let vertices = polytope.vertices_f64();
    let f = normals.len();

    (0..f)
        .map(|k| {
            let (s_k, centroid_k) = facet_volume_and_centroid_3d(&normals, &heights, &vertices, k, f);
            if s_k < 1e-30 {
                return Vector4::zeros();
            }
            let tangent_centroid = centroid_k - heights[k] * normals[k];
            -s_k * tangent_centroid
        })
        .collect()
}

/// d(c_EHZ)/d(n_k) via envelope theorem, projected onto T_{n_k}S³.
fn compute_capacity_derivatives_n(
    best_orbit: &ValidOrbit,
    normals: &[Vector4<f64>],
    facet_count: usize,
) -> Vec<Vector4<f64>> {
    let q_sq = best_orbit.q_value * best_orbit.q_value;
    let perm = &best_orbit.permutation;
    let beta = &best_orbit.beta;
    let lambda = Vector4::new(
        best_orbit.lambda[0],
        best_orbit.lambda[1],
        best_orbit.lambda[2],
        best_orbit.lambda[3],
    );

    (0..facet_count)
        .map(|k| {
            let i0 = match perm.iter().position(|&f| f == k) {
                Some(pos) => pos,
                None => return Vector4::zeros(),
            };

            // P_{i₀} = Σ_{i < i₀} β_i · n_{σ(i)}
            let mut p = Vector4::zeros();
            for i in 0..i0 {
                p += beta[i] * normals[perm[i]];
            }

            // ∂Q*/∂n_k = β_{i₀} · [J₀(2P + β_{i₀} n_k) − λ]
            let inner = 2.0 * p + beta[i0] * normals[k];
            let j0_inner = j0_apply(&inner);
            let dq_dn = beta[i0] * (j0_inner - lambda);

            // Project onto T_{n_k}S³
            let dq_dn_tangent = dq_dn - dq_dn.dot(&normals[k]) * normals[k];

            // ∂A/∂n_k = −∂Q*/∂n_k / (2Q²)
            -dq_dn_tangent / (2.0 * q_sq)
        })
        .collect()
}

/// Full sensitivity: d(sys)/d(h_k) and d(sys)/d(n_k) via chain rule.
fn compute_sensitivity(
    polytope: &Polytope4D,
    vol: f64,
    cap: f64,
    sys: f64,
    instrumented: &InstrumentedResult,
) -> SensitivityResult {
    let normals = polytope.normals_f64();
    let f = normals.len();
    let best_orbit = &instrumented.orbits[0];

    // Height derivatives
    let d_vol_h = compute_volume_derivatives_h(polytope);
    let d_cap_h = compute_capacity_derivatives_h(best_orbit, f);

    // d(sys)/d(h_k) = (1/vol) * [c · dc/dh_k − sys · dvol/dh_k]
    let d_sys_h: Vec<f64> = d_vol_h
        .iter()
        .zip(d_cap_h.iter())
        .map(|(&dv, &dc)| {
            if dv.is_nan() || dc.is_nan() {
                f64::NAN
            } else {
                (cap * dc - sys * dv) / vol
            }
        })
        .collect();

    let gradient_norm_h = d_sys_h
        .iter()
        .filter(|x| !x.is_nan())
        .map(|x| x * x)
        .sum::<f64>()
        .sqrt();

    // Normal derivatives
    let d_vol_n = compute_volume_derivatives_n(polytope);
    let d_cap_n = compute_capacity_derivatives_n(best_orbit, &normals, f);

    let d_sys_n: Vec<Vector4<f64>> = d_vol_n
        .iter()
        .zip(d_cap_n.iter())
        .map(|(dv, dc)| (cap * dc - sys * dv) / vol)
        .collect();

    let gradient_norm_n = d_sys_n
        .iter()
        .map(|v| v.norm_squared())
        .sum::<f64>()
        .sqrt();

    let gradient_norm_hn =
        (gradient_norm_h * gradient_norm_h + gradient_norm_n * gradient_norm_n).sqrt();

    SensitivityResult {
        d_sys_h,
        gradient_norm_h,
        d_sys_n,
        gradient_norm_n,
        gradient_norm_hn,
    }
}

// ============================================================================
// Step bounds (copied from sys_optimization.rs)
// ============================================================================

/// Maximum step t > 0 along height direction before combinatorial type changes.
fn compute_step_bound_h(polytope: &Polytope4D, direction: &[f64]) -> f64 {
    let normals = polytope.normals_f64();
    let heights = polytope.heights_f64();
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
                if rate < -1e-15 {
                    let t_crit = slack / (-rate);
                    if t_crit > 0.0 && t_crit < t_max {
                        t_max = t_crit;
                    }
                }
            }
        } else {
            // Non-simple vertex: conservative bound
            for j in 0..f {
                if vertex_facets.contains(&j) {
                    continue;
                }
                let slack = heights[j] - normals[j].dot(v);
                let max_g = direction.iter().map(|x| x.abs()).fold(0.0f64, f64::max);
                if max_g > 1e-15 {
                    let t_crit = slack / max_g;
                    if t_crit > 0.0 && t_crit < t_max {
                        t_max = t_crit;
                    }
                }
            }
        }
    }

    // Height positivity
    for k in 0..f {
        if direction[k] < -1e-15 {
            let t_crit = heights[k] / (-direction[k]);
            if t_crit > 0.0 && t_crit < t_max {
                t_max = t_crit;
            }
        }
    }

    t_max.min(MAX_STEP_SIZE)
}

/// Maximum step t > 0 along (h, n) direction.
fn compute_step_bound_hn(
    polytope: &Polytope4D,
    g_h: &[f64],
    g_n: &[Vector4<f64>],
) -> f64 {
    let normals = polytope.normals_f64();
    let heights = polytope.heights_f64();
    let vertices = polytope.vertices_f64();
    let f = polytope.facet_count();
    let skeleton = Skeleton::compute(polytope);

    let mut t_max = f64::INFINITY;

    // Vertex-crossing checks
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
                if rate < -1e-15 {
                    let t_crit = slack / (-rate);
                    if t_crit > 0.0 && t_crit < t_max {
                        t_max = t_crit;
                    }
                }
            }
        } else {
            // Non-simple vertex: conservative bound
            for j in 0..f {
                if vertex_facets.contains(&j) {
                    continue;
                }
                let slack = heights[j] - normals[j].dot(v);
                let max_g_h = g_h.iter().map(|x| x.abs()).fold(0.0f64, f64::max);
                let max_g_n = g_n.iter().map(|g| g.norm()).fold(0.0f64, f64::max);
                let max_rate = max_g_h + max_g_n * v.norm();
                if max_rate > 1e-15 {
                    let t_crit = slack / max_rate;
                    if t_crit > 0.0 && t_crit < t_max {
                        t_max = t_crit;
                    }
                }
            }
        }
    }

    // Height positivity
    for k in 0..f {
        if g_h[k] < -1e-15 {
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
        let omega_ij = omega0_local(&normals[i], &normals[j]);
        let d_omega = omega0_local(&g_n[i], &normals[j]) + omega0_local(&normals[i], &g_n[j]);
        if omega_ij.abs() > 1e-15 && d_omega.abs() > 1e-15 {
            let t_flip = -omega_ij / d_omega;
            if t_flip > 0.0 && t_flip < t_max {
                t_max = t_flip;
            }
        }
    }

    t_max.min(MAX_STEP_SIZE)
}

// ============================================================================
// Gradient steps
// ============================================================================

/// Which capacity algorithm to use for evaluation after a step.
///
/// - `Hk2017`: General polytopes (only algorithm available for non-Lagrangian products).
/// - `Billiard`: Lagrangian products (block-structured enumeration with directed ω₀ pruning).
enum CapacityBackend {
    Hk2017,
    Billiard,
}

/// Try a height-only gradient step, returning the new polytope and its sys value.
fn try_step_h(
    normals: &[Vector4<f64>],
    heights: &[f64],
    direction: &[f64],
    t: f64,
    backend: &CapacityBackend,
) -> Option<(Polytope4D, f64)> {
    let f = normals.len();
    let new_heights: Vec<f64> = (0..f).map(|k| heights[k] + t * direction[k]).collect();

    let new_polytope = Polytope4D::from_normals_and_heights(normals.to_vec(), new_heights).ok()?;
    let vol = volume(&new_polytope).ok().filter(|&v| v > 0.0)?;
    let cap = compute_capacity(&new_polytope, backend)?;
    let sys = cap * cap / (2.0 * vol);
    sys.is_finite().then_some((new_polytope, sys))
}

/// Try a (h, n) gradient step, returning the new polytope and its sys value.
/// For Lagrangian products, normal perturbations are projected to preserve structure.
fn try_step_hn(
    normals: &[Vector4<f64>],
    heights: &[f64],
    g_h: &[f64],
    g_n: &[Vector4<f64>],
    t: f64,
    backend: &CapacityBackend,
    lagrangian_class: Option<&FacetClassification>,
) -> Option<(Polytope4D, f64)> {
    let f = normals.len();
    let new_heights: Vec<f64> = (0..f).map(|k| heights[k] + t * g_h[k]).collect();
    let new_normals: Vec<Vector4<f64>> = (0..f)
        .map(|k| {
            let mut n = normals[k] + t * g_n[k];
            // For Lagrangian products, project to preserve subspace constraint
            if let Some(class) = lagrangian_class {
                if class.q_indices.contains(&k) {
                    // q-facet: zero out p-components
                    n[2] = 0.0;
                    n[3] = 0.0;
                } else if class.p_indices.contains(&k) {
                    // p-facet: zero out q-components
                    n[0] = 0.0;
                    n[1] = 0.0;
                }
            }
            let norm = n.norm();
            if norm < 1e-15 {
                normals[k] // fallback: keep original
            } else {
                n / norm
            }
        })
        .collect();

    let new_polytope = Polytope4D::from_normals_and_heights(new_normals, new_heights).ok()?;
    let vol = volume(&new_polytope).ok().filter(|&v| v > 0.0)?;
    let cap = compute_capacity(&new_polytope, backend)?;
    let sys = cap * cap / (2.0 * vol);
    sys.is_finite().then_some((new_polytope, sys))
}

/// Compute capacity using the appropriate backend.
fn compute_capacity(polytope: &Polytope4D, backend: &CapacityBackend) -> Option<f64> {
    match backend {
        CapacityBackend::Hk2017 => {
            // TODO: ehz_capacity will be re-exported from top-level in wave 4
            symplectic::algorithms::hk2017::ehz_capacity(polytope).map(|r| r.result.capacity)
        }
        CapacityBackend::Billiard => {
            billiard_capacity(polytope).ok()?.map(|r| r.result.capacity)
        }
    }
}

// ============================================================================
// Gradient ascent loop
// ============================================================================

struct AscentResult {
    iterations: Vec<GradientDescentRow>,
    summary: SummaryRow,
}

fn run_gradient_ascent(
    name: &str,
    polytope_type: &str,
    start_polytope: &Polytope4D,
    backend: &CapacityBackend,
    lagrangian_class: Option<&FacetClassification>,
) -> AscentResult {
    let f = start_polytope.facet_count();
    let t_start = Instant::now();

    let mut current = match Polytope4D::from_normals_and_heights(
        start_polytope.normals_f64().to_vec(),
        start_polytope.heights_f64().to_vec(),
    ) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("  {name}: reconstruct failed: {e}");
            return AscentResult {
                iterations: Vec::new(),
                summary: SummaryRow {
                    name: name.to_string(),
                    polytope_type: polytope_type.to_string(),
                    facet_count: f,
                    starting_sys: f64::NAN,
                    final_sys: f64::NAN,
                    total_delta: 0.0,
                    iterations: 0,
                    converged: false,
                    total_time_ms: 0.0,
                },
            };
        }
    };

    let mut rows = Vec::new();
    let mut starting_sys = 0.0f64;
    let mut current_sys = 0.0f64;
    let mut n_iter = 0usize;
    let mut converged = false;

    for iter in 0..MAX_ITERATIONS {
        let t_iter = Instant::now();

        // 1. Instrumented capacity
        let instrumented = match backend {
            CapacityBackend::Hk2017 => ehz_capacity_instrumented(&current),
            CapacityBackend::Billiard => billiard_capacity_instrumented(&current),
        };
        let instrumented = match instrumented {
            Some(r) => r,
            None => break,
        };
        let cap = instrumented.capacity;
        let vol = match volume(&current) {
            Ok(v) if v > 0.0 => v,
            _ => break,
        };
        let sys = cap * cap / (2.0 * vol);

        if iter == 0 {
            starting_sys = sys;
            current_sys = sys;
        }

        // 2. Sensitivity
        let sensitivity = compute_sensitivity(&current, vol, cap, sys, &instrumented);

        // 3. Step bounds
        let normals = current.normals_f64();
        let heights = current.heights_f64();

        let t_max_h = if sensitivity.gradient_norm_h > 1e-15 {
            compute_step_bound_h(&current, &sensitivity.d_sys_h)
        } else {
            0.0
        };
        let t_max_hn = if sensitivity.gradient_norm_hn > 1e-15 {
            compute_step_bound_hn(&current, &sensitivity.d_sys_h, &sensitivity.d_sys_n)
        } else {
            0.0
        };

        // 4. Try all step fractions for both types, pick best
        let mut best: Option<(Polytope4D, f64, String, f64, f64)> = None;

        if t_max_h > 0.0 && sensitivity.gradient_norm_h > 1e-15 {
            for &frac in STEP_FRACTIONS {
                let t = frac * t_max_h;
                if let Some((p, new_sys)) =
                    try_step_h(&normals, &heights, &sensitivity.d_sys_h, t, backend)
                {
                    if new_sys > sys && best.as_ref().is_none_or(|b| new_sys > b.1) {
                        best = Some((p, new_sys, "h_only".to_string(), frac, t));
                    }
                }
            }
        }

        if t_max_hn > 0.0 && sensitivity.gradient_norm_hn > 1e-15 {
            for &frac in STEP_FRACTIONS {
                let t = frac * t_max_hn;
                if let Some((p, new_sys)) = try_step_hn(
                    &normals,
                    &heights,
                    &sensitivity.d_sys_h,
                    &sensitivity.d_sys_n,
                    t,
                    backend,
                    lagrangian_class,
                ) {
                    if new_sys > sys && best.as_ref().is_none_or(|b| new_sys > b.1) {
                        best = Some((p, new_sys, "h_n".to_string(), frac, t));
                    }
                }
            }
        }

        let time_ms = t_iter.elapsed().as_secs_f64() * 1000.0;

        // 5. Take best step or stop
        match best {
            Some((new_polytope, new_sys, step_type, frac, t)) => {
                let delta = new_sys - sys;
                let cumulative = new_sys - starting_sys;

                // Check if this is the last iteration (converged or max)
                let is_last = delta < CONVERGENCE_THRESHOLD || iter + 1 >= MAX_ITERATIONS;

                rows.push(GradientDescentRow {
                    name: name.to_string(),
                    polytope_type: polytope_type.to_string(),
                    facet_count: f,
                    iteration: iter,
                    step_type,
                    t_fraction: frac,
                    t_actual: t,
                    sys_before: sys,
                    sys_after: new_sys,
                    delta_sys: delta,
                    starting_sys,
                    cumulative_delta: cumulative,
                    gradient_norm_h: sensitivity.gradient_norm_h,
                    gradient_norm_n: sensitivity.gradient_norm_n,
                    time_ms,
                    final_normals: if is_last {
                        Some(
                            new_polytope
                                .normals_f64()
                                .iter()
                                .map(|n| [n[0], n[1], n[2], n[3]])
                                .collect(),
                        )
                    } else {
                        None
                    },
                    final_heights: if is_last {
                        Some(new_polytope.heights_f64().to_vec())
                    } else {
                        None
                    },
                });

                current = new_polytope;
                current_sys = new_sys;
                n_iter = iter + 1;

                if delta < CONVERGENCE_THRESHOLD {
                    converged = true;
                    break;
                }
            }
            None => break,
        }
    }

    let total_time_ms = t_start.elapsed().as_secs_f64() * 1000.0;

    AscentResult {
        iterations: rows,
        summary: SummaryRow {
            name: name.to_string(),
            polytope_type: polytope_type.to_string(),
            facet_count: f,
            starting_sys,
            final_sys: current_sys,
            total_delta: current_sys - starting_sys,
            iterations: n_iter,
            converged,
            total_time_ms,
        },
    }
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
// Main
// ============================================================================

/// Load completed polytope names from existing JSONL file (for resume after crash).
/// Returns an empty set if the file doesn't exist.
fn load_completed_names(path: &std::path::Path) -> HashSet<String> {
    let mut names = HashSet::new();
    if let Ok(file) = File::open(path) {
        let reader = BufReader::new(file);
        for line in reader.lines().map_while(Result::ok) {
            let line = line.trim().to_string();
            if line.is_empty() {
                continue;
            }
            // Parse just the name field
            if let Ok(v) = serde_json::from_str::<serde_json::Value>(&line) {
                if let Some(name) = v.get("name").and_then(|n| n.as_str()) {
                    names.insert(name.to_string());
                }
            }
        }
    }
    names
}

fn main() {
    let t0 = Instant::now();
    let base_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let output_path = base_dir.join("gradient-descent/gradient-descent.jsonl");

    println!("Gradient ascent experiment: F={FACET_COUNT} polytopes\n");

    // Resume support: load already-completed polytope names from existing JSONL.
    // On a fresh run (no file or --fresh flag), starts from scratch.
    let fresh = std::env::args().any(|a| a == "--fresh");
    let completed = if fresh {
        // Remove existing file for a clean run
        let _ = std::fs::remove_file(&output_path);
        HashSet::new()
    } else {
        load_completed_names(&output_path)
    };

    if completed.is_empty() {
        println!("Starting fresh run.");
    } else {
        println!("Resuming: {} polytopes already completed.", completed.len());
    }

    // Open in append mode for resume, or create for fresh
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&output_path)
        .expect("open output JSONL");
    let mut writer = BufWriter::new(file);

    let mut rng = ChaCha8Rng::seed_from_u64(SEED);

    // =========================================================================
    // Phase 1: General random polytopes
    // =========================================================================

    println!("Generating {N_GENERAL} general random F={FACET_COUNT} polytopes...");
    let general = generate_general_polytopes(&mut rng);
    println!("Generated {} polytopes.\n", general.len());

    let mut max_sys = f64::NEG_INFINITY;
    let mut max_sys_name = String::new();
    let mut n_converged = 0usize;

    let n_skip_general = general.iter().filter(|(n, _)| completed.contains(n)).count();
    println!(
        "Running gradient ascent on general polytopes (HK2017)... ({} to skip)\n",
        n_skip_general
    );
    for (idx, (name, polytope)) in general.iter().enumerate() {
        if completed.contains(name) {
            continue;
        }
        print!(
            "[{}/{}] {}: ",
            idx + 1,
            general.len(),
            name
        );

        let result = run_gradient_ascent(
            name,
            "general",
            polytope,
            &CapacityBackend::Hk2017,
            None,
        );

        for row in &result.iterations {
            serde_json::to_writer(&mut writer, row).expect("write row");
            writeln!(writer).expect("newline");
        }

        let s = &result.summary;
        if s.final_sys > max_sys {
            max_sys = s.final_sys;
            max_sys_name = s.name.clone();
        }
        if s.converged {
            n_converged += 1;
        }

        println!(
            "iter={}, sys: {:.6}→{:.6} (Δ={:.6}){}, {:.1}s",
            s.iterations,
            s.starting_sys,
            s.final_sys,
            s.total_delta,
            if s.converged { ", converged" } else { "" },
            s.total_time_ms / 1000.0,
        );
    }

    println!(
        "\nGeneral summary: best sys={:.6} ({}), {}/{} converged\n",
        max_sys,
        max_sys_name,
        n_converged,
        general.len()
    );

    // =========================================================================
    // Phase 2: Lagrangian products
    // =========================================================================

    println!(
        "Generating Lagrangian products (splits: {:?}, {} per bucket)...",
        LAGRANGIAN_SPLITS, N_LAGRANGIAN_PER_BUCKET
    );
    let lagrangian = generate_lagrangian_polytopes(&mut rng);
    println!("Generated {} Lagrangian products.\n", lagrangian.len());

    // Cross-check: instrumented HK2017 vs library billiard on first 5 products.
    // Both algorithms should agree on capacity for Lagrangian products.
    println!("Cross-checking instrumented HK2017 vs library billiard...");
    for (name, _, polytope) in lagrangian.iter().take(5) {
        let lib_cap = billiard_capacity(polytope)
            .expect("billiard failed")
            .expect("billiard None")
            .result
            .capacity;
        let inst_cap = ehz_capacity_instrumented(polytope)
            .expect("instrumented HK2017 None")
            .capacity;
        let rel_err = ((lib_cap - inst_cap) / lib_cap).abs();
        println!(
            "  {name}: billiard={lib_cap:.10}, hk2017_inst={inst_cap:.10}, rel_err={rel_err:.2e}"
        );
        assert!(
            rel_err < 1e-6,
            "Capacity mismatch: billiard={lib_cap}, hk2017={inst_cap}, rel_err={rel_err}"
        );
    }
    println!("Cross-check passed.\n");

    let mut max_sys_lp = f64::NEG_INFINITY;
    let mut max_sys_name_lp = String::new();
    let mut n_converged_lp = 0usize;

    let n_skip_lp = lagrangian.iter().filter(|(n, _, _)| completed.contains(n)).count();
    println!(
        "Running gradient ascent on Lagrangian products (billiard, Lagrangian-constrained gradient)... ({} to skip)\n",
        n_skip_lp
    );
    for (idx, (name, bucket, polytope)) in lagrangian.iter().enumerate() {
        if completed.contains(name) {
            continue;
        }
        print!(
            "[{}/{}] {}: ",
            idx + 1,
            lagrangian.len(),
            name
        );

        // Classify facets for constrained step
        let class = classify_facets(polytope).expect("Lagrangian product should classify");

        let result = run_gradient_ascent(
            name,
            bucket,
            polytope,
            &CapacityBackend::Billiard,
            Some(&class),
        );

        for row in &result.iterations {
            serde_json::to_writer(&mut writer, row).expect("write row");
            writeln!(writer).expect("newline");
        }

        let s = &result.summary;
        if s.final_sys > max_sys_lp {
            max_sys_lp = s.final_sys;
            max_sys_name_lp = s.name.clone();
        }
        if s.converged {
            n_converged_lp += 1;
        }

        println!(
            "iter={}, sys: {:.6}→{:.6} (Δ={:.6}){}, {:.1}s",
            s.iterations,
            s.starting_sys,
            s.final_sys,
            s.total_delta,
            if s.converged { ", converged" } else { "" },
            s.total_time_ms / 1000.0,
        );
    }

    writer.flush().expect("flush");

    println!(
        "\nLagrangian summary: best sys={:.6} ({}), {}/{} converged",
        max_sys_lp,
        max_sys_name_lp,
        n_converged_lp,
        lagrangian.len()
    );

    println!(
        "\nOverall best sys: {:.6}",
        max_sys.max(max_sys_lp)
    );
    println!("Total time: {:.1}s", t0.elapsed().as_secs_f64());
    println!("Output: {}", output_path.display());
}

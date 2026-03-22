//! Sys-optimization Phase 1–4: sensitivity, gradient steps, iteration, validity testing.
//!
//! Computes d(sys)/d(h_k) and d(sys)/d(n_k) for polytopes from random-sweep and
//! random-product-sweep, then takes finite gradient steps bounded by combinatorial type
//! preservation (Phase 2) and iterates to convergence (Phase 3).
//!
//! Uses library KKT solver, derivative functions, and facet volume helpers.
//! Experiment-specific code: sensitivity analysis, gradient iteration,
//! step bounds, and validity testing.
//!
//! Architecture:
//! 1. `cargo run --bin sys_optimization --release` generates datasets
//! 2. Writes to sys-optimization/sys-optimization-{sensitivity,steps,iterations,validity}.jsonl
//! 3. Python script reads JSONL, produces figures and stats
//!
//! Input: random-sweep/random-sweep.jsonl, random-product-sweep/random-product-sweep.jsonl
//! Filter: F ≤ 10 (HK2017 is exponential in F)

use nalgebra::{Matrix4, Vector4};
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use rand_distr::{Distribution, StandardNormal};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::time::Instant;
use symplectic::algorithms::hk2017::ehz_capacity;
use symplectic::derivatives::{
    capacity_derivatives_h, capacity_derivatives_n,
    volume_derivatives_h, volume_derivatives_h_fd, volume_derivatives_n,
};
use symplectic::geom::polytope::Polytope4D;
use symplectic::geom::skeleton::Skeleton;
use symplectic::geom::symplectic_form::omega0;
use symplectic::geom::volume::volume;
use symplectic::kkt::saddle_point_solver::{solve_kkt_for, KktResult};

/// Maximum facet count to process (HK2017 cost is exponential).
const MAX_FACET_COUNT: usize = 10;

/// Maximum step size cap (prevents infinite steps when no combinatorial bound exists).
/// Heights are O(1), so steps beyond 100 are unreasonable.
const MAX_STEP_SIZE: f64 = 100.0;

/// Step fractions of t_max to evaluate.
const STEP_FRACTIONS: &[f64] = &[0.1, 0.25, 0.5, 0.75, 0.95];

/// Maximum number of gradient ascent iterations in Phase 3.
const MAX_ITERATIONS: usize = 20;

/// Minimum improvement per iteration to continue (convergence threshold).
const CONVERGENCE_THRESHOLD: f64 = 1e-6;

/// Number of random directions to test per polytope in Phase 4.
const N_RANDOM_DIRECTIONS: usize = 10;

/// Step fractions of t_max to test in Phase 4 (includes beyond-t_max values).
const VALIDITY_STEP_FRACTIONS: &[f64] = &[0.01, 0.1, 0.25, 0.5, 1.0, 2.0, 5.0, 10.0];

// ============================================================================
// Output schemas
// ============================================================================

#[derive(Debug, Serialize)]
struct SensitivityRow {
    name: String,
    source_dataset: String,
    facet_count: usize,
    normals: Vec<[f64; 4]>,
    heights: Vec<f64>,
    volume: f64,
    capacity: f64,
    sys: f64,
    /// Number of facets in the best orbit's permutation.
    orbit_length: usize,
    best_action: f64,
    // Height derivatives
    d_vol_h: Vec<f64>,
    d_cap_h: Vec<f64>,
    d_sys_h: Vec<f64>,
    gradient_norm_h: f64,
    // Normal derivatives (tangent vectors, 4 components each)
    d_vol_n: Vec<[f64; 4]>,
    d_cap_n: Vec<[f64; 4]>,
    d_sys_n: Vec<[f64; 4]>,
    gradient_norm_n: f64,
    // Combined
    gradient_norm_hn: f64,
    n_favorable: usize,
    t_max_h: f64,
    t_max_hn: f64,
    time_instrumented_ms: f64,
    time_sensitivity_ms: f64,
}

#[derive(Debug, Serialize)]
struct StepRow {
    name: String,
    source_dataset: String,
    facet_count: usize,
    step_type: String, // "h_only" or "h_n"
    t_fraction: f64,
    t_actual: f64,
    old_sys: f64,
    new_sys: f64,
    delta_sys: f64,
    new_volume: f64,
    new_capacity: f64,
    vertex_count_old: usize,
    vertex_count_new: usize,
    construction_ok: bool,
}

#[derive(Debug, Serialize)]
struct IterationRow {
    name: String,
    source_dataset: String,
    facet_count: usize,
    iteration: usize,
    step_type: String,       // "h_only" or "h_n"
    t_fraction: f64,
    t_actual: f64,
    sys_before: f64,
    sys_after: f64,
    delta_sys: f64,
    starting_sys: f64,
    cumulative_delta: f64,
    gradient_norm_h: f64,
    gradient_norm_n: f64,
    gradient_norm_hn: f64,
    vertex_count: usize,
    time_ms: f64,
}

#[derive(Debug, Serialize)]
struct ValidityRow {
    name: String,
    facet_count: usize,
    starting_sys: f64,
    direction_type: String,  // "gradient_h", "gradient_hn", "random"
    direction_index: usize,  // 0 for gradient, 0..N-1 for random
    t_fraction: f64,         // fraction of t_max
    t_actual: f64,
    t_max: f64,
    predicted_delta_sys: f64,
    actual_delta_sys: f64,
    prediction_error: f64,
    relative_error: f64,
    directional_derivative: f64,
    construction_ok: bool,
    vertex_count_changed: bool,
    beyond_t_max: bool,
}

// ============================================================================
// Input deserialization (reads from random-sweep / random-product-sweep JSONL)
// ============================================================================

#[derive(Debug, Deserialize)]
struct InputRow {
    name: String,
    #[serde(alias = "facet_count")]
    facet_count: usize,
    normals: Vec<[f64; 4]>,
    heights: Vec<f64>,
}

// KKT solver: uses library solve_kkt_for (crates/src/kkt/saddle_point_solver.rs).
// Capacity: uses library ehz_capacity, then solve_kkt_for for the best permutation
// to obtain the full KKT solution (beta, mu, xi) needed for derivative computation.

// ============================================================================
// Sensitivity computation
// ============================================================================

struct SensitivityResult {
    // Height derivatives
    d_vol_h: Vec<f64>,
    d_cap_h: Vec<f64>,
    d_sys_h: Vec<f64>,
    gradient_norm_h: f64,
    // Normal derivatives (tangent vectors in T_{n_k}S³)
    d_vol_n: Vec<Vector4<f64>>,
    d_cap_n: Vec<Vector4<f64>>,
    d_sys_n: Vec<Vector4<f64>>,
    gradient_norm_n: f64,
    // Combined gradient norm
    gradient_norm_hn: f64,
}


// Derivative functions and facet volume helpers: uses library
// (crates/src/derivatives.rs, crates/src/geom/facet_volume.rs).

/// Compute full sensitivity: d(sys)/d(h_k) and d(sys)/d(n_k) via chain rule.
///
/// Height derivatives: library volume_derivatives_h + capacity_derivatives_h (envelope theorem).
/// Normal derivatives: library volume_derivatives_n + capacity_derivatives_n.
///
/// Sign convention: Library uses symmetric KKT (Hβ + Nμ + ηξ = 0).
/// The derivative functions handle the sign convention internally — no adjustment needed.
///
/// # Arguments
/// - `kkt`: KKT solution for the best orbit (from `solve_kkt_for`)
/// - `perm`: cyclic facet permutation of the best orbit (from `EhzResult`)
fn compute_sensitivity(
    polytope: &Polytope4D,
    vol: f64,
    cap: f64,
    sys: f64,
    kkt: &KktResult,
    perm: &[usize],
) -> SensitivityResult {
    let normals = polytope.normals_f64();
    let heights = polytope.heights_f64();
    let f = normals.len();

    // --- Height derivatives ---
    let d_vol_h = volume_derivatives_h(polytope);

    // Cross-check: analytical volume derivatives (h) vs finite differences
    debug_assert!({
        let d_vol_fd = volume_derivatives_h_fd(&normals, &heights, 1e-3, |n, h| {
            let p = Polytope4D::from_normals_and_heights(n.to_vec(), h.to_vec()).ok()?;
            volume(&p).ok()
        });
        let ok = d_vol_h.iter().zip(d_vol_fd.iter()).all(|(a, fd)| {
            if fd.is_nan() { return true; }
            let tol = (0.05 * a.abs()).max(0.1);
            (a - fd).abs() < tol
        });
        if !ok {
            eprintln!("volume h-derivative mismatch: analytical={:?} fd={:?}", d_vol_h, d_vol_fd);
        }
        ok
    }, "volume h-derivative: analytical vs FD mismatch");

    // Library capacity_derivatives_h uses symmetric convention (xi from KktResult).
    let d_cap_h = capacity_derivatives_h(
        &kkt.beta,
        kkt.q_corrected,
        kkt.xi,
        perm,
        f,
    );

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

    // --- Normal derivatives ---
    let d_vol_n = volume_derivatives_n(polytope);

    // Library capacity_derivatives_n uses symmetric convention (mu from KktResult).
    let d_cap_n = capacity_derivatives_n(
        &kkt.beta,
        kkt.q_corrected,
        &kkt.mu,
        perm,
        &normals,
    );

    // Chain rule: d(sys)/d(n_k) = (1/vol) * [c * dc/dn_k - sys * dvol/dn_k]
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

    let gradient_norm_hn = (gradient_norm_h * gradient_norm_h + gradient_norm_n * gradient_norm_n).sqrt();

    SensitivityResult {
        d_vol_h,
        d_cap_h,
        d_sys_h,
        gradient_norm_h,
        d_vol_n,
        d_cap_n,
        d_sys_n,
        gradient_norm_n,
        gradient_norm_hn,
    }
}

// ============================================================================
// Step bounds computation
// ============================================================================

/// Compute maximum step t > 0 along direction g before combinatorial type changes.
///
/// For step direction g = (g_0, ..., g_{F-1}), new heights h'_k = h_k + t * g_k.
/// The combinatorial type changes when a vertex hits a new facet or becomes infeasible.
fn compute_step_bound(
    polytope: &Polytope4D,
    direction: &[f64],
) -> f64 {
    let normals = polytope.normals_f64();
    let heights = polytope.heights_f64();
    let vertices = polytope.vertices_f64();
    let f = polytope.facet_count();
    let skeleton = Skeleton::compute(polytope);

    let mut t_max = f64::INFINITY;

    for (vi, vertex_facets) in skeleton.vertex_facets.iter().enumerate() {
        let v = &vertices[vi];

        if vertex_facets.len() == 4 {
            // Simple vertex: exactly 4 determining facets
            // Build 4×4 normal matrix and compute vertex movement direction
            let det_facets = &vertex_facets;
            let n_mat = Matrix4::from_rows(&[
                normals[det_facets[0]].transpose(),
                normals[det_facets[1]].transpose(),
                normals[det_facets[2]].transpose(),
                normals[det_facets[3]].transpose(),
            ]);

            let n_inv = match n_mat.try_inverse() {
                Some(inv) => inv,
                None => continue, // Degenerate vertex, skip
            };

            // RHS: height changes for the determining facets
            let g_det = Vector4::new(
                direction[det_facets[0]],
                direction[det_facets[1]],
                direction[det_facets[2]],
                direction[det_facets[3]],
            );

            // Vertex movement: dv/dt = N^{-1} * g_det
            let dv_dt = n_inv * g_det;

            // Check each non-determining facet
            for j in 0..f {
                if vertex_facets.contains(&j) {
                    continue;
                }
                // Current slack: h_j - n_j · v > 0
                let slack = heights[j] - normals[j].dot(v);
                // Rate of slack change: d(slack)/dt = g_j - n_j · dv/dt
                let rate = direction[j] - normals[j].dot(&dv_dt);
                // Slack hits zero at t = slack / (-rate) when rate < 0
                if rate < -1e-15 {
                    let t_crit = slack / (-rate);
                    if t_crit > 0.0 && t_crit < t_max {
                        t_max = t_crit;
                    }
                }
            }
        } else {
            // Non-simple vertex (>4 incident facets, e.g. in Lagrangian products).
            // Conservative bound: check slack for all non-incident facets assuming
            // vertex moves at worst rate. Use minimum slack / max possible rate.
            for j in 0..f {
                if vertex_facets.contains(&j) {
                    continue;
                }
                let slack = heights[j] - normals[j].dot(v);
                // Conservative: assume vertex doesn't move but facet j moves
                // This underestimates t_max but is always safe
                if direction[j] < -1e-15 {
                    // Facet j moves inward: doesn't affect this vertex
                    // (constraint h_j - n_j·v gets tighter only if n_j·v increases)
                    continue;
                }
                // For non-incident facets, the vertex might approach due to height changes
                // of the determining facets. Without inverting a >4 system, use a crude bound:
                // assume the slack can decrease at most at rate proportional to max |g_k|
                let max_g = direction.iter().map(|x| x.abs()).fold(0.0f64, f64::max);
                if max_g > 1e-15 {
                    // Very conservative: t_max ≤ slack / max_g
                    let t_crit = slack / max_g;
                    if t_crit > 0.0 && t_crit < t_max {
                        t_max = t_crit;
                    }
                }
            }
        }
    }

    // Also check that all heights stay positive
    for k in 0..f {
        if direction[k] < -1e-15 {
            // h_k + t * g_k > 0 → t < -h_k / g_k = h_k / |g_k|
            let t_crit = heights[k] / (-direction[k]);
            if t_crit > 0.0 && t_crit < t_max {
                t_max = t_crit;
            }
        }
    }

    // Cap at practical maximum
    t_max.min(MAX_STEP_SIZE)
}

/// Compute maximum step t > 0 along combined (g_h, g_n) direction.
///
/// Extends `compute_step_bound` to handle both height and normal perturbations.
/// At step t: h'_k = h_k + t·g_{h,k}, n'_k = normalize(n_k + t·g_{n,k}).
///
/// Additional constraint: ω₀(n_i, n_j) must not change sign for ridge-adjacent pairs.
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

    // --- Vertex-crossing checks ---
    for (vi, vertex_facets) in skeleton.vertex_facets.iter().enumerate() {
        let v = &vertices[vi];

        if vertex_facets.len() == 4 {
            // Simple vertex: v = N_v⁻¹ h_v
            // When both normals and heights change:
            // N_v(t) · v(t) = h_v(t) differentiate:
            // N_v · dv/dt + dN_v/dt · v = dh_v/dt
            // dv/dt = N_v⁻¹ (dh_v/dt - dN_v/dt · v)
            // where dN_v/dt has rows g_{n,det[r]} and dh_v/dt has entries g_{h,det[r]}
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

            // RHS: dh_v/dt - dN_v/dt · v
            let rhs = Vector4::new(
                g_h[det_facets[0]] - g_n[det_facets[0]].dot(v),
                g_h[det_facets[1]] - g_n[det_facets[1]].dot(v),
                g_h[det_facets[2]] - g_n[det_facets[2]].dot(v),
                g_h[det_facets[3]] - g_n[det_facets[3]].dot(v),
            );
            let dv_dt = n_inv * rhs;

            // Check each non-determining facet
            for j in 0..f {
                if vertex_facets.contains(&j) {
                    continue;
                }
                // Slack: s_j(t) = h_j(t) - n_j(t) · v(t)
                // ds_j/dt = g_{h,j} - g_{n,j} · v - n_j · dv/dt
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
                // Conservative: max rate from all sources
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

    // --- Height positivity ---
    for k in 0..f {
        if g_h[k] < -1e-15 {
            let t_crit = heights[k] / (-g_h[k]);
            if t_crit > 0.0 && t_crit < t_max {
                t_max = t_crit;
            }
        }
    }

    // --- ω₀ sign preservation for ridge-adjacent pairs ---
    for ridge in &skeleton.ridges {
        let i = ridge.facets[0];
        let j = ridge.facets[1];
        let omega_ij = omega0(&normals[i], &normals[j]);
        // d(ω₀(n_i(t), n_j(t)))/dt = ω₀(g_{n,i}, n_j) + ω₀(n_i, g_{n,j})
        let d_omega = omega0(&g_n[i], &normals[j]) + omega0(&normals[i], &g_n[j]);
        // Sign flips when omega_ij + t * d_omega = 0 → t = -omega_ij / d_omega
        // Only relevant if the sign would flip (omega_ij and d_omega have opposite signs)
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
// Gradient step evaluation
// ============================================================================

/// Take a gradient step in (h,n) space and evaluate the result.
fn evaluate_gradient_step_hn(
    normals: &[Vector4<f64>],
    heights: &[f64],
    g_h: &[f64],
    g_n: &[Vector4<f64>],
    t: f64,
    old_sys: f64,
    old_vertex_count: usize,
) -> StepRow {
    let f = normals.len();
    let new_heights: Vec<f64> = (0..f).map(|k| heights[k] + t * g_h[k]).collect();
    let new_normals: Vec<Vector4<f64>> = (0..f)
        .map(|k| {
            let n = normals[k] + t * g_n[k];
            n / n.norm() // renormalize to unit length
        })
        .collect();

    match Polytope4D::from_normals_and_heights(new_normals, new_heights) {
        Ok(new_polytope) => {
            let new_vol = volume(&new_polytope).unwrap_or(f64::NAN);
            let new_cap = ehz_capacity(&new_polytope)
                .map(|r| r.result.capacity)
                .unwrap_or(f64::NAN);
            let new_sys = if new_vol > 0.0 && new_cap.is_finite() {
                new_cap * new_cap / (2.0 * new_vol)
            } else {
                f64::NAN
            };

            StepRow {
                name: String::new(),
                source_dataset: String::new(),
                facet_count: f,
                step_type: "h_n".to_string(),
                t_fraction: 0.0,
                t_actual: t,
                old_sys,
                new_sys,
                delta_sys: new_sys - old_sys,
                new_volume: new_vol,
                new_capacity: new_cap,
                vertex_count_old: old_vertex_count,
                vertex_count_new: new_polytope.vertices_f64().len(),
                construction_ok: true,
            }
        }
        Err(e) => {
            eprintln!("    (h,n) step t={t:.6} failed: {e}");
            StepRow {
                name: String::new(),
                source_dataset: String::new(),
                facet_count: f,
                step_type: "h_n".to_string(),
                t_fraction: 0.0,
                t_actual: t,
                old_sys,
                new_sys: f64::NAN,
                delta_sys: f64::NAN,
                new_volume: f64::NAN,
                new_capacity: f64::NAN,
                vertex_count_old: old_vertex_count,
                vertex_count_new: 0,
                construction_ok: false,
            }
        }
    }
}

/// Take a gradient step and evaluate the result.
fn evaluate_gradient_step(
    normals: &[Vector4<f64>],
    heights: &[f64],
    direction: &[f64],
    t: f64,
    old_sys: f64,
    old_vertex_count: usize,
) -> StepRow {
    let f = normals.len();
    let new_heights: Vec<f64> = (0..f).map(|k| heights[k] + t * direction[k]).collect();

    match Polytope4D::from_normals_and_heights(normals.to_vec(), new_heights) {
        Ok(new_polytope) => {
            let new_vol = volume(&new_polytope).unwrap_or(f64::NAN);
            // Use library ehz_capacity for the step evaluation (not instrumented — faster)
            let new_cap = ehz_capacity(&new_polytope)
                .map(|r| r.result.capacity)
                .unwrap_or(f64::NAN);
            let new_sys = if new_vol > 0.0 && new_cap.is_finite() {
                new_cap * new_cap / (2.0 * new_vol)
            } else {
                f64::NAN
            };

            StepRow {
                name: String::new(), // filled in by caller
                source_dataset: String::new(),
                facet_count: f,
                step_type: "h_only".to_string(),
                t_fraction: 0.0, // filled in by caller
                t_actual: t,
                old_sys,
                new_sys,
                delta_sys: new_sys - old_sys,
                new_volume: new_vol,
                new_capacity: new_cap,
                vertex_count_old: old_vertex_count,
                vertex_count_new: new_polytope.vertices_f64().len(),
                construction_ok: true,
            }
        }
        Err(e) => {
            eprintln!("    Step t={t:.6} failed: {e}");
            StepRow {
                name: String::new(),
                source_dataset: String::new(),
                facet_count: f,
                step_type: "h_only".to_string(),
                t_fraction: 0.0,
                t_actual: t,
                old_sys,
                new_sys: f64::NAN,
                delta_sys: f64::NAN,
                new_volume: f64::NAN,
                new_capacity: f64::NAN,
                vertex_count_old: old_vertex_count,
                vertex_count_new: 0,
                construction_ok: false,
            }
        }
    }
}

// ============================================================================
// Phase 3 helpers: gradient steps that return the new polytope
// ============================================================================

/// Try a height-only gradient step, returning the new polytope and its sys value.
fn try_step_h_polytope(
    normals: &[Vector4<f64>],
    heights: &[f64],
    direction: &[f64],
    t: f64,
) -> Option<(Polytope4D, f64)> {
    let f = normals.len();
    let new_heights: Vec<f64> = (0..f).map(|k| heights[k] + t * direction[k]).collect();

    let new_polytope = match Polytope4D::from_normals_and_heights(normals.to_vec(), new_heights) {
        Ok(p) => p,
        Err(_) => return None,
    };
    let vol = volume(&new_polytope).unwrap_or(0.0);
    if vol <= 0.0 {
        return None;
    }
    let cap = ehz_capacity(&new_polytope)
        .map(|r| r.result.capacity)
        .unwrap_or(f64::NAN);
    if !cap.is_finite() {
        return None;
    }
    let sys = cap * cap / (2.0 * vol);
    if sys.is_finite() {
        Some((new_polytope, sys))
    } else {
        None
    }
}

/// Try a (h,n) gradient step, returning the new polytope and its sys value.
fn try_step_hn_polytope(
    normals: &[Vector4<f64>],
    heights: &[f64],
    g_h: &[f64],
    g_n: &[Vector4<f64>],
    t: f64,
) -> Option<(Polytope4D, f64)> {
    let f = normals.len();
    let new_heights: Vec<f64> = (0..f).map(|k| heights[k] + t * g_h[k]).collect();
    let new_normals: Vec<Vector4<f64>> = (0..f)
        .map(|k| {
            let n = normals[k] + t * g_n[k];
            n / n.norm()
        })
        .collect();

    let new_polytope = match Polytope4D::from_normals_and_heights(new_normals, new_heights) {
        Ok(p) => p,
        Err(_) => return None,
    };
    let vol = volume(&new_polytope).unwrap_or(0.0);
    if vol <= 0.0 {
        return None;
    }
    let cap = ehz_capacity(&new_polytope)
        .map(|r| r.result.capacity)
        .unwrap_or(f64::NAN);
    if !cap.is_finite() {
        return None;
    }
    let sys = cap * cap / (2.0 * vol);
    if sys.is_finite() {
        Some((new_polytope, sys))
    } else {
        None
    }
}

// ============================================================================
// Data loading
// ============================================================================

fn load_polytopes_from_jsonl(path: &std::path::Path, source: &str) -> Vec<(String, String, Polytope4D)> {
    let file = match File::open(path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("WARNING: Could not open {}: {e}", path.display());
            eprintln!("  Run the corresponding experiment binary first.");
            return Vec::new();
        }
    };
    let reader = BufReader::new(file);
    let mut polytopes = Vec::new();

    for (line_no, line) in reader.lines().enumerate() {
        let line = match line {
            Ok(l) => l,
            Err(e) => {
                eprintln!("  Line {}: read error: {e}", line_no + 1);
                continue;
            }
        };
        if line.trim().is_empty() {
            continue;
        }

        let row: InputRow = match serde_json::from_str(&line) {
            Ok(r) => r,
            Err(e) => {
                eprintln!("  Line {}: parse error: {e}", line_no + 1);
                continue;
            }
        };

        if row.facet_count > MAX_FACET_COUNT {
            continue;
        }

        let normals: Vec<Vector4<f64>> = row
            .normals
            .iter()
            .map(|n| Vector4::new(n[0], n[1], n[2], n[3]))
            .collect();

        match Polytope4D::from_normals_and_heights(normals, row.heights) {
            Ok(p) => polytopes.push((row.name, source.to_string(), p)),
            Err(e) => {
                eprintln!("  {}: construction failed: {e}", row.name);
            }
        }
    }

    polytopes
}

// ============================================================================
// Main
// ============================================================================

fn main() {
    let t0 = Instant::now();
    let base_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));

    println!("Sys-optimization Phase 1–3: sensitivity + steps + iterative ascent\n");

    // =========================================================================
    // Load starting polytopes
    // =========================================================================

    println!("Loading starting polytopes (F ≤ {MAX_FACET_COUNT})...");

    let random_sweep_path = base_dir.join("random-sweep/random-sweep.jsonl");
    let random_product_path = base_dir.join("random-product-sweep/random-product-sweep.jsonl");

    let mut polytopes: Vec<(String, String, Polytope4D)> = Vec::new();

    let rs = load_polytopes_from_jsonl(&random_sweep_path, "random-sweep");
    println!("  random-sweep: {} polytopes loaded", rs.len());
    polytopes.extend(rs);

    let rp = load_polytopes_from_jsonl(&random_product_path, "random-product-sweep");
    println!("  random-product-sweep: {} polytopes loaded", rp.len());
    polytopes.extend(rp);

    let n_polytopes = polytopes.len();
    println!("  Total: {n_polytopes} polytopes\n");

    if n_polytopes == 0 {
        eprintln!("ERROR: No polytopes loaded. Run random_sweep and random_product_sweep first.");
        std::process::exit(1);
    }

    // =========================================================================
    // Phase 1: Sensitivity analysis
    // =========================================================================

    println!("Phase 1: Computing sensitivities...\n");

    let sensitivity_path = base_dir.join("sys-optimization/sys-optimization-sensitivity.jsonl");
    let steps_path = base_dir.join("sys-optimization/sys-optimization-steps.jsonl");

    let sens_file = File::create(&sensitivity_path).expect("create sensitivity JSONL");
    let mut sens_writer = BufWriter::new(sens_file);

    let steps_file = File::create(&steps_path).expect("create steps JSONL");
    let mut steps_writer = BufWriter::new(steps_file);

    let mut total_favorable = 0usize;
    let mut total_facets = 0usize;
    let mut best_sys_after = f64::NEG_INFINITY;
    let mut best_sys_before = 0.0f64;
    let mut n_improved = 0usize;

    for (idx, (name, source, polytope)) in polytopes.iter().enumerate() {
        let f = polytope.facet_count();
        let normals = polytope.normals_f64();
        let heights = polytope.heights_f64();

        print!("[{}/{}] {} (F={}): ", idx + 1, n_polytopes, name, f);

        // --- Library HK2017 + KKT ---
        let t_instr = Instant::now();
        let ehz = match ehz_capacity(polytope) {
            Some(r) => r,
            None => {
                println!("SKIP (no valid orbits)");
                continue;
            }
        };
        let best_perm = &ehz.result.best_permutation;
        let kkt = match solve_kkt_for(polytope, best_perm) {
            Some(r) => r,
            None => {
                println!("SKIP (KKT solve failed for best permutation)");
                continue;
            }
        };
        let time_instrumented_ms = t_instr.elapsed().as_secs_f64() * 1000.0;

        let cap = ehz.result.capacity;
        let vol = volume(polytope).expect("volume failed");
        let sys = cap * cap / (2.0 * vol);

        // --- Sensitivity ---
        let t_sens = Instant::now();
        let sensitivity = compute_sensitivity(polytope, vol, cap, sys, &kkt, best_perm);
        let time_sensitivity_ms = t_sens.elapsed().as_secs_f64() * 1000.0;

        // Count favorable facets: d_sys_h > 0 means increasing h_k improves sys,
        // d_sys_h < 0 means decreasing h_k improves sys. Either is "favorable".
        let n_favorable = sensitivity
            .d_sys_h
            .iter()
            .filter(|&&ds| !ds.is_nan() && ds.abs() > 1e-10)
            .count();
        total_favorable += n_favorable;
        total_facets += f;

        // --- Step bounds ---
        // h-only direction: steepest ascent = d_sys_h
        let t_max_h = if sensitivity.gradient_norm_h > 1e-15 {
            compute_step_bound(polytope, &sensitivity.d_sys_h)
        } else {
            0.0
        };

        // (h,n) direction: steepest ascent = (d_sys_h, d_sys_n)
        let t_max_hn = if sensitivity.gradient_norm_hn > 1e-15 {
            compute_step_bound_hn(polytope, &sensitivity.d_sys_h, &sensitivity.d_sys_n)
        } else {
            0.0
        };

        println!(
            "sys={:.6}, |∇h|={:.4e}, |∇n|={:.4e}, |∇hn|={:.4e}, t_h={:.4e}, t_hn={:.4e}, {:.0}ms",
            sys,
            sensitivity.gradient_norm_h,
            sensitivity.gradient_norm_n,
            sensitivity.gradient_norm_hn,
            t_max_h,
            t_max_hn,
            time_instrumented_ms + time_sensitivity_ms
        );

        // Write sensitivity row
        let normals_raw: Vec<[f64; 4]> = normals.iter().map(|n| [n[0], n[1], n[2], n[3]]).collect();
        let d_vol_n_raw: Vec<[f64; 4]> = sensitivity.d_vol_n.iter().map(|v| [v[0], v[1], v[2], v[3]]).collect();
        let d_cap_n_raw: Vec<[f64; 4]> = sensitivity.d_cap_n.iter().map(|v| [v[0], v[1], v[2], v[3]]).collect();
        let d_sys_n_raw: Vec<[f64; 4]> = sensitivity.d_sys_n.iter().map(|v| [v[0], v[1], v[2], v[3]]).collect();
        let sens_row = SensitivityRow {
            name: name.clone(),
            source_dataset: source.clone(),
            facet_count: f,
            normals: normals_raw,
            heights: heights.to_vec(),
            volume: vol,
            capacity: cap,
            sys,
            orbit_length: best_perm.len(),
            best_action: cap,
            d_vol_h: sensitivity.d_vol_h,
            d_cap_h: sensitivity.d_cap_h,
            d_sys_h: sensitivity.d_sys_h.clone(),
            gradient_norm_h: sensitivity.gradient_norm_h,
            d_vol_n: d_vol_n_raw,
            d_cap_n: d_cap_n_raw,
            d_sys_n: d_sys_n_raw,
            gradient_norm_n: sensitivity.gradient_norm_n,
            gradient_norm_hn: sensitivity.gradient_norm_hn,
            n_favorable,
            t_max_h,
            t_max_hn,
            time_instrumented_ms,
            time_sensitivity_ms,
        };
        serde_json::to_writer(&mut sens_writer, &sens_row).expect("write sensitivity");
        writeln!(sens_writer).expect("newline");

        // =========================================================================
        // Phase 2: Gradient steps (h-only)
        // =========================================================================

        let vertex_count_old = polytope.vertices_f64().len();

        if t_max_h > 0.0 && sensitivity.gradient_norm_h > 1e-15 {
            for &frac in STEP_FRACTIONS {
                let t = frac * t_max_h;
                let mut step_row = evaluate_gradient_step(
                    &normals,
                    &heights,
                    &sensitivity.d_sys_h,
                    t,
                    sys,
                    vertex_count_old,
                );
                step_row.name = name.clone();
                step_row.source_dataset = source.clone();
                step_row.t_fraction = frac;

                if step_row.construction_ok && step_row.new_sys > best_sys_after {
                    best_sys_after = step_row.new_sys;
                    best_sys_before = sys;
                }
                if step_row.construction_ok && step_row.delta_sys > 1e-10 {
                    n_improved += 1;
                }

                serde_json::to_writer(&mut steps_writer, &step_row).expect("write step");
                writeln!(steps_writer).expect("newline");
            }
        }

        // =========================================================================
        // Phase 2b: Gradient steps (h+n combined)
        // =========================================================================

        if t_max_hn > 0.0 && sensitivity.gradient_norm_hn > 1e-15 {
            for &frac in STEP_FRACTIONS {
                let t = frac * t_max_hn;
                let mut step_row = evaluate_gradient_step_hn(
                    &normals,
                    &heights,
                    &sensitivity.d_sys_h,
                    &sensitivity.d_sys_n,
                    t,
                    sys,
                    vertex_count_old,
                );
                step_row.name = name.clone();
                step_row.source_dataset = source.clone();
                step_row.t_fraction = frac;

                serde_json::to_writer(&mut steps_writer, &step_row).expect("write step");
                writeln!(steps_writer).expect("newline");
            }
        }
    }

    sens_writer.flush().expect("flush sensitivity");
    steps_writer.flush().expect("flush steps");

    // =========================================================================
    // Phase 3: Iterative gradient ascent
    // =========================================================================

    println!("\nPhase 3: Iterative gradient ascent (max {MAX_ITERATIONS} iterations)...\n");

    let iterations_path = base_dir.join("sys-optimization/sys-optimization-iterations.jsonl");
    let iter_file = File::create(&iterations_path).expect("create iterations JSONL");
    let mut iter_writer = BufWriter::new(iter_file);

    let mut total_iterations = 0usize;
    let mut max_sys_achieved = f64::NEG_INFINITY;
    let mut max_sys_name = String::new();
    let mut n_converged = 0usize;

    for (idx, (name, source, start_polytope)) in polytopes.iter().enumerate() {
        let f = start_polytope.facet_count();
        print!("[{}/{}] {} (F={}): ", idx + 1, n_polytopes, name, f);

        let t_poly = Instant::now();

        // Reconstruct to get an owned polytope we can replace each iteration
        let mut current = match Polytope4D::from_normals_and_heights(
            start_polytope.normals_f64().to_vec(),
            start_polytope.heights_f64().to_vec(),
        ) {
            Ok(p) => p,
            Err(e) => {
                println!("SKIP (reconstruct failed: {e})");
                continue;
            }
        };

        let mut starting_sys = 0.0f64;
        let mut current_sys = 0.0f64;
        let mut n_iter = 0usize;
        let mut converged = false;
        let mut n_h_only = 0usize;
        let mut n_h_n = 0usize;

        for iter in 0..MAX_ITERATIONS {
            let t_iter = Instant::now();

            // 1. Library HK2017 + KKT
            let ehz = match ehz_capacity(&current) {
                Some(r) => r,
                None => break,
            };
            let best_perm = &ehz.result.best_permutation;
            let kkt = match solve_kkt_for(&current, best_perm) {
                Some(r) => r,
                None => break,
            };
            let cap = ehz.result.capacity;
            let vol = volume(&current).expect("volume");
            let sys = cap * cap / (2.0 * vol);

            if iter == 0 {
                starting_sys = sys;
                current_sys = sys;
            }

            // 2. Sensitivity
            let sensitivity = compute_sensitivity(&current, vol, cap, sys, &kkt, best_perm);

            // 3. Step bounds
            let normals = current.normals_f64();
            let heights = current.heights_f64();

            let t_max_h = if sensitivity.gradient_norm_h > 1e-15 {
                compute_step_bound(&current, &sensitivity.d_sys_h)
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
                    if let Some((p, new_sys)) = try_step_h_polytope(
                        &normals, &heights, &sensitivity.d_sys_h, t,
                    ) {
                        if new_sys > sys && best.as_ref().is_none_or(|b| new_sys > b.1) {
                            best = Some((p, new_sys, "h_only".to_string(), frac, t));
                        }
                    }
                }
            }

            if t_max_hn > 0.0 && sensitivity.gradient_norm_hn > 1e-15 {
                for &frac in STEP_FRACTIONS {
                    let t = frac * t_max_hn;
                    if let Some((p, new_sys)) = try_step_hn_polytope(
                        &normals, &heights, &sensitivity.d_sys_h, &sensitivity.d_sys_n, t,
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

                    let row = IterationRow {
                        name: name.clone(),
                        source_dataset: source.clone(),
                        facet_count: f,
                        iteration: iter,
                        step_type: step_type.clone(),
                        t_fraction: frac,
                        t_actual: t,
                        sys_before: sys,
                        sys_after: new_sys,
                        delta_sys: delta,
                        starting_sys,
                        cumulative_delta: cumulative,
                        gradient_norm_h: sensitivity.gradient_norm_h,
                        gradient_norm_n: sensitivity.gradient_norm_n,
                        gradient_norm_hn: sensitivity.gradient_norm_hn,
                        vertex_count: new_polytope.vertices_f64().len(),
                        time_ms,
                    };
                    serde_json::to_writer(&mut iter_writer, &row).expect("write iteration");
                    writeln!(iter_writer).expect("newline");

                    if step_type == "h_only" {
                        n_h_only += 1;
                    } else {
                        n_h_n += 1;
                    }

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

        let total_delta = current_sys - starting_sys;
        let poly_time = t_poly.elapsed().as_secs_f64();
        total_iterations += n_iter;

        if current_sys > max_sys_achieved {
            max_sys_achieved = current_sys;
            max_sys_name = name.clone();
        }
        if converged {
            n_converged += 1;
        }

        println!(
            "iter={n_iter} (h:{n_h_only} n:{n_h_n}), sys: {:.6}→{:.6} (Δ={:.6}), {}{:.1}s",
            starting_sys,
            current_sys,
            total_delta,
            if converged { "converged, " } else { "" },
            poly_time,
        );
    }

    iter_writer.flush().expect("flush iterations");

    // =========================================================================
    // Phase 4: Gradient validity testing
    // =========================================================================

    println!("\nPhase 4: Gradient validity testing...\n");

    let validity_path = base_dir.join("sys-optimization/sys-optimization-validity.jsonl");
    let validity_file = File::create(&validity_path).expect("create validity JSONL");
    let mut validity_writer = BufWriter::new(validity_file);

    let mut rng = ChaCha8Rng::seed_from_u64(42);
    let mut total_validity_evals = 0usize;
    let mut total_ok_within = 0usize;
    let mut total_ok_beyond = 0usize;
    let mut total_within = 0usize;
    let mut total_beyond = 0usize;

    for (idx, (name, _source, polytope)) in polytopes.iter().enumerate() {
        let f = polytope.facet_count();
        let normals = polytope.normals_f64();
        let heights = polytope.heights_f64();
        let vertex_count_orig = polytope.vertices_f64().len();

        print!("[{}/{}] {} (F={}): ", idx + 1, n_polytopes, name, f);

        // Recompute sensitivity (same as Phase 1)
        let ehz = match ehz_capacity(polytope) {
            Some(r) => r,
            None => {
                println!("SKIP");
                continue;
            }
        };
        let best_perm = &ehz.result.best_permutation;
        let kkt = match solve_kkt_for(polytope, best_perm) {
            Some(r) => r,
            None => {
                println!("SKIP (KKT failed)");
                continue;
            }
        };
        let cap = ehz.result.capacity;
        let vol = volume(polytope).expect("volume failed");
        let sys = cap * cap / (2.0 * vol);

        let sens = compute_sensitivity(polytope, vol, cap, sys, &kkt, best_perm);

        // --- Build directions to test ---
        struct Direction {
            g_h: Vec<f64>,
            g_n: Vec<Vector4<f64>>,
            dir_type: String,
            dir_index: usize,
            directional_deriv: f64,
        }

        let mut directions: Vec<Direction> = Vec::new();

        // Direction 1: gradient h-only (normalize d_sys_h, zero normal component)
        if sens.gradient_norm_h > 1e-15 {
            let scale = 1.0 / sens.gradient_norm_h;
            let g_h: Vec<f64> = sens.d_sys_h.iter().map(|x| x * scale).collect();
            let g_n: Vec<Vector4<f64>> = vec![Vector4::zeros(); f];
            let dd = sens.gradient_norm_h; // ∇sys · (∇_h sys / |∇_h sys|) = |∇_h sys|
            directions.push(Direction {
                g_h,
                g_n,
                dir_type: "gradient_h".to_string(),
                dir_index: 0,
                directional_deriv: dd,
            });
        }

        // Direction 2: gradient (h,n) (normalize combined)
        if sens.gradient_norm_hn > 1e-15 {
            let scale = 1.0 / sens.gradient_norm_hn;
            let g_h: Vec<f64> = sens.d_sys_h.iter().map(|x| x * scale).collect();
            let g_n: Vec<Vector4<f64>> = sens.d_sys_n.iter().map(|v| v * scale).collect();
            let dd = sens.gradient_norm_hn; // ∇sys · (∇sys / |∇sys|) = |∇sys|
            directions.push(Direction {
                g_h,
                g_n,
                dir_type: "gradient_hn".to_string(),
                dir_index: 0,
                directional_deriv: dd,
            });
        }

        // Directions 3..12: random (h,n) directions
        for dir_idx in 0..N_RANDOM_DIRECTIONS {
            // Random h-component: Gaussian in R^F
            let raw_h: Vec<f64> = (0..f)
                .map(|_| StandardNormal.sample(&mut rng))
                .collect();

            // Random n-component: Gaussian in R^4, projected to T_{n_k}S³
            let raw_n: Vec<Vector4<f64>> = (0..f)
                .map(|k| {
                    let v = Vector4::new(
                        StandardNormal.sample(&mut rng),
                        StandardNormal.sample(&mut rng),
                        StandardNormal.sample(&mut rng),
                        StandardNormal.sample(&mut rng),
                    );
                    // Project to tangent space: v - (v·n_k) n_k
                    v - normals[k] * v.dot(&normals[k])
                })
                .collect();

            // Normalize the combined (h, n) direction to unit norm
            let norm_sq: f64 = raw_h.iter().map(|x| x * x).sum::<f64>()
                + raw_n.iter().map(|v| v.norm_squared()).sum::<f64>();
            let norm = norm_sq.sqrt();
            if norm < 1e-15 {
                continue;
            }
            let scale = 1.0 / norm;
            let g_h: Vec<f64> = raw_h.iter().map(|x| x * scale).collect();
            let g_n: Vec<Vector4<f64>> = raw_n.iter().map(|v| v * scale).collect();

            // Directional derivative: ∇sys · δ
            let dd: f64 = sens
                .d_sys_h
                .iter()
                .zip(g_h.iter())
                .map(|(ds, d)| ds * d)
                .sum::<f64>()
                + sens
                    .d_sys_n
                    .iter()
                    .zip(g_n.iter())
                    .map(|(ds, d)| ds.dot(d))
                    .sum::<f64>();

            directions.push(Direction {
                g_h,
                g_n,
                dir_type: "random".to_string(),
                dir_index: dir_idx,
                directional_deriv: dd,
            });
        }

        let mut n_evals = 0usize;

        // For each direction, compute t_max and test at multiple fractions
        for dir in &directions {
            // Compute step bound for this direction
            let t_max = if dir.g_n.iter().all(|v| v.norm() < 1e-15) {
                // Pure height direction
                compute_step_bound(polytope, &dir.g_h)
            } else {
                compute_step_bound_hn(polytope, &dir.g_h, &dir.g_n)
            };

            if t_max < 1e-15 {
                continue; // Degenerate direction
            }

            for &frac in VALIDITY_STEP_FRACTIONS {
                let t = frac * t_max;
                let beyond = frac > 1.0;

                // Predicted delta_sys from linear approximation
                let predicted_delta = t * dir.directional_deriv;

                // Actual: construct perturbed polytope, compute sys
                let (actual_delta, construction_ok, vertex_count_changed) =
                    if dir.g_n.iter().all(|v| v.norm() < 1e-15) {
                        // h-only step
                        match try_step_h_polytope(&normals, &heights, &dir.g_h, t) {
                            Some((new_poly, new_sys)) => {
                                let vc = new_poly.vertices_f64().len();
                                (new_sys - sys, true, vc != vertex_count_orig)
                            }
                            None => (f64::NAN, false, false),
                        }
                    } else {
                        // (h,n) step
                        match try_step_hn_polytope(&normals, &heights, &dir.g_h, &dir.g_n, t) {
                            Some((new_poly, new_sys)) => {
                                let vc = new_poly.vertices_f64().len();
                                (new_sys - sys, true, vc != vertex_count_orig)
                            }
                            None => (f64::NAN, false, false),
                        }
                    };

                let prediction_error = if actual_delta.is_finite() {
                    (actual_delta - predicted_delta).abs()
                } else {
                    f64::NAN
                };

                let relative_error = if actual_delta.is_finite() {
                    let denom = actual_delta.abs().max(1e-10);
                    (actual_delta - predicted_delta).abs() / denom
                } else {
                    f64::NAN
                };

                if beyond {
                    total_beyond += 1;
                    if construction_ok {
                        total_ok_beyond += 1;
                    }
                } else {
                    total_within += 1;
                    if construction_ok {
                        total_ok_within += 1;
                    }
                }

                let row = ValidityRow {
                    name: name.clone(),
                    facet_count: f,
                    starting_sys: sys,
                    direction_type: dir.dir_type.clone(),
                    direction_index: dir.dir_index,
                    t_fraction: frac,
                    t_actual: t,
                    t_max,
                    predicted_delta_sys: predicted_delta,
                    actual_delta_sys: actual_delta,
                    prediction_error,
                    relative_error,
                    directional_derivative: dir.directional_deriv,
                    construction_ok,
                    vertex_count_changed,
                    beyond_t_max: beyond,
                };

                serde_json::to_writer(&mut validity_writer, &row)
                    .expect("write validity row");
                validity_writer.write_all(b"\n").expect("write newline");
                n_evals += 1;
                total_validity_evals += 1;
            }
        }

        println!("{} directions × {} steps = {} evaluations", directions.len(), VALIDITY_STEP_FRACTIONS.len(), n_evals);
    }

    validity_writer.flush().expect("flush validity");

    // =========================================================================
    // Summary
    // =========================================================================

    let total_time = t0.elapsed().as_secs_f64();
    println!("\n═══════════════════════════════════════════════");
    println!("Summary (Phase 1–2)");
    println!("═══════════════════════════════════════════════");
    println!("Polytopes processed: {n_polytopes}");
    println!(
        "Favorable facets:    {total_favorable}/{total_facets} ({:.1}%)",
        100.0 * total_favorable as f64 / total_facets.max(1) as f64
    );
    println!("Steps that improved sys: {n_improved}");
    if best_sys_after > f64::NEG_INFINITY {
        println!(
            "Best sys (single step): {:.6} (from {:.6}, Δ={:.6})",
            best_sys_after,
            best_sys_before,
            best_sys_after - best_sys_before
        );
    }

    println!("\n═══════════════════════════════════════════════");
    println!("Summary (Phase 3 — iterative)");
    println!("═══════════════════════════════════════════════");
    println!("Total iterations:    {total_iterations}");
    println!(
        "Mean iterations:     {:.1}",
        total_iterations as f64 / n_polytopes.max(1) as f64
    );
    println!("Converged:           {n_converged}/{n_polytopes}");
    if max_sys_achieved > f64::NEG_INFINITY {
        println!(
            "Best sys (iterative): {:.6} ({})",
            max_sys_achieved, max_sys_name
        );
    }

    println!("\n═══════════════════════════════════════════════");
    println!("Summary (Phase 4 — validity testing)");
    println!("═══════════════════════════════════════════════");
    println!("Total evaluations:   {total_validity_evals}");
    println!(
        "Within t_max:        {total_ok_within}/{total_within} OK ({:.0}%)",
        100.0 * total_ok_within as f64 / total_within.max(1) as f64
    );
    println!(
        "Beyond t_max:        {total_ok_beyond}/{total_beyond} OK ({:.0}%)",
        100.0 * total_ok_beyond as f64 / total_beyond.max(1) as f64
    );

    println!("\nTotal time:          {total_time:.1}s");
    println!();
    println!("Output:");
    println!("  {}", sensitivity_path.display());
    println!("  {}", steps_path.display());
    println!("  {}", iterations_path.display());
    println!("  {}", validity_path.display());
}

//! Base point recovery and orbit verification for Reeb orbits on polytopes.
//!
//! Given an EHZ capacity result (sigma, beta, capacity), recovers the primal
//! Reeb orbit gamma on the boundary of K and verifies its geometric validity.
//! Combines the old `recover_base_point` and `verify_orbit` into a single
//! `recover_and_verify` function, since they are always called together.
//!
//! ## Mathematical background
//!
//! A simple orbit with parameters (sigma, tau) has piecewise-constant velocity
//! R_{sigma(k)} on each segment. The position during segment k is:
//!
//!   gamma(t) = b + v_k + (t - t_k) * R_{sigma(k)}
//!
//! where v_k = sum_{j<k} tau_j R_{sigma(j)} is the accumulated displacement.
//!
//! For gamma(t) to lie on facet F_{sigma(k)}, we need <n_{sigma(k)}, gamma(t)> = h_{sigma(k)}.
//! Since R_{sigma(k)} is tangent to F_{sigma(k)} (<n_{sigma(k)}, R_{sigma(k)}> = 0),
//! this reduces to a linear system in b:
//!
//!   <n_{sigma(k)}, b> = h_{sigma(k)} - <n_{sigma(k)}, v_k>   for each active k
//!
//! The system has m equations and 4 unknowns (b in R^4). We solve via SVD
//! (least-norm solution when underdetermined), then optimize in the null space
//! to minimize constraint violations.
//!
//! Dwell time conversion: tau_k = T * h_{sigma(k)} * beta_k, where T = c_EHZ(K).
//!
//! Mathematical correspondence: [lem:base-point-recovery], [rem:beta-to-tau]

use super::EhzResult;
use crate::geom::polytope::Polytope4D;
use crate::geom::symplectic_form::omega0;
use nalgebra::{DMatrix, DVector, Vector4};

/// Result of recovering and verifying a Reeb orbit from a capacity computation.
///
/// Contains the orbit breakpoints (vertices of the piecewise-linear trajectory),
/// dwell times on each facet, the facet visitation sequence, the computed action,
/// and error metrics for geometric validity.
///
/// [lem:base-point-recovery]: recovery of base point b = gamma(0).
#[derive(Clone, Debug)]
pub struct OrbitRecovery {
    /// Points where the trajectory transitions between facets: b + v_k for k = 0..m.
    /// `breakpoints[0]` is the base point b = gamma(0).
    /// For a closed orbit, `breakpoints[m]` should be close to `breakpoints[0]`.
    pub breakpoints: Vec<Vector4<f64>>,

    /// Time spent on each facet: tau_k = T * h_{sigma(k)} * beta_k.
    /// Length equals the permutation length m.
    pub dwell_times: Vec<f64>,

    /// Maximum halfspace violation across all breakpoints:
    /// max_{j,k} (<n_j, breakpoint_k> - h_j).
    /// Should be <= 0 (or numerically near 0) for a valid orbit inside K.
    pub max_violation: f64,

    /// Computed symplectic action via the shoelace formula:
    /// A = (1/2) sum_{i>j} tau_j tau_i omega_0(R_{sigma(j)}, R_{sigma(i)}).
    /// Should match the EHZ capacity.
    ///
    /// [lem:shoelace]: action from dwell times and Reeb vectors.
    pub action: f64,

    /// Closure error: ||breakpoints[m] - breakpoints[0]||.
    /// Near zero for a valid closed orbit.
    pub closure_error: f64,

    /// Facet indices visited in order (copy of the best permutation sigma).
    pub facet_sequence: Vec<usize>,
}

/// Recover a Reeb orbit from an EHZ capacity result and verify its validity.
///
/// Performs three stages:
/// 1. **Beta-to-tau conversion**: dwell times tau_k = capacity * h_{sigma(k)} * beta_k.
/// 2. **Base point recovery**: solve the linear system N_S b = r via SVD, then
///    optimize in the null space to minimize halfspace violations.
/// 3. **Orbit verification**: check closure, on-facet, inside-K, and action consistency.
///
/// Returns `None` if the linear system has no active equations (all dwell times
/// are zero), which should not happen for valid algorithm output.
///
/// [lem:base-point-recovery], [rem:beta-to-tau]
pub fn recover_and_verify(
    polytope: &Polytope4D,
    result: &EhzResult,
) -> Option<OrbitRecovery> {
    let normals = polytope.normals_f64();
    let heights = polytope.heights_f64();
    let sigma = &result.result.best_permutation;
    let beta = &result.result.best_beta;
    let capacity = result.result.capacity;
    let m = sigma.len();

    // ── Stage 1: convert beta to dwell times ──
    //
    // tau_k = T * h_{sigma(k)} * beta_k, where T = capacity.
    // [rem:beta-to-tau]
    let dwell_times: Vec<f64> = (0..m)
        .map(|k| capacity * heights[sigma[k]] * beta[k])
        .collect();

    // ── Stage 2: recover base point ──
    //
    // Reeb vectors: R_i = (2/h_i) J_0 n_i.
    let reeb_vectors: Vec<Vector4<f64>> = (0..m)
        .map(|k| {
            let i = sigma[k];
            let n = &normals[i];
            let h = heights[i];
            // J_0 n = (-n[2], -n[3], n[0], n[1]) in (q1,q2,p1,p2) coordinates
            Vector4::new(-n[2], -n[3], n[0], n[1]) * (2.0 / h)
        })
        .collect();

    // Accumulated displacements: v_k = sum_{j<k} tau_j R_{sigma(j)}.
    let mut displacements: Vec<Vector4<f64>> = Vec::with_capacity(m + 1);
    displacements.push(Vector4::zeros());
    for k in 0..m {
        let v_next = displacements[k] + dwell_times[k] * reeb_vectors[k];
        displacements.push(v_next);
    }

    // Build linear system N_S b = r for active facets (tau_k > 0).
    // For each active k: <n_{sigma(k)}, b> = h_{sigma(k)} - <n_{sigma(k)}, v_k>.
    let active: Vec<usize> = (0..m).filter(|&k| dwell_times[k] > 0.0).collect();
    let n_active = active.len();

    if n_active == 0 {
        return None;
    }

    let mut mat = DMatrix::<f64>::zeros(n_active, 4);
    let mut rhs = DVector::<f64>::zeros(n_active);

    for (row, &k) in active.iter().enumerate() {
        let i = sigma[k];
        let n = &normals[i];
        for col in 0..4 {
            mat[(row, col)] = n[col];
        }
        rhs[row] = heights[i] - n.dot(&displacements[k]);
    }

    // Solve via SVD. Pad to at least 4 rows so SVD yields a 4x4 V^T
    // from which we can extract null space vectors.
    let rows = n_active.max(4);
    let mut mat_padded = DMatrix::<f64>::zeros(rows, 4);
    let mut rhs_padded = DVector::<f64>::zeros(rows);
    for row in 0..n_active {
        for col in 0..4 {
            mat_padded[(row, col)] = mat[(row, col)];
        }
        rhs_padded[row] = rhs[row];
    }

    let svd = mat_padded.svd(true, true);
    let tol = 1e-10 * svd.singular_values[0].max(1.0);
    let rank = svd.singular_values.iter().filter(|&&s| s > tol).count();
    let solution_dim = 4 - rank;

    let b_vec = svd.solve(&rhs_padded, tol).ok()?;
    let mut base_point = Vector4::new(b_vec[0], b_vec[1], b_vec[2], b_vec[3]);

    // If underdetermined, search the null space for a point minimizing
    // halfspace violations. Null space basis: rows of V^T with indices >= rank.
    if solution_dim > 0 {
        if let Some(v_mat) = &svd.v_t {
            let null_vecs: Vec<Vector4<f64>> = (rank..4)
                .map(|i| {
                    Vector4::new(v_mat[(i, 0)], v_mat[(i, 1)], v_mat[(i, 2)], v_mat[(i, 3)])
                })
                .collect();
            base_point = optimize_in_null_space(
                base_point,
                &null_vecs,
                &displacements,
                &normals,
                &heights,
            );
        }
    }

    // ── Stage 3: compute breakpoints and verify ──
    //
    // Breakpoints: b + v_k for k = 0..=m.
    let breakpoints: Vec<Vector4<f64>> = (0..=m)
        .map(|k| base_point + displacements[k])
        .collect();

    // Max violation: max_{j,k} (<n_j, breakpoint_k> - h_j).
    let max_violation = breakpoints
        .iter()
        .flat_map(|p| {
            normals
                .iter()
                .zip(heights.iter())
                .map(move |(n, &h)| n.dot(p) - h)
        })
        .fold(f64::NEG_INFINITY, f64::max);

    // Closure error: ||breakpoints[m] - breakpoints[0]||.
    let closure_error = (breakpoints[m] - breakpoints[0]).norm();

    // Computed action via shoelace formula:
    // A = (1/2) sum_{i>j} tau_j tau_i omega_0(R_{sigma(j)}, R_{sigma(i)}).
    // [lem:shoelace]
    let mut action_sum = 0.0;
    for i in 1..m {
        for j in 0..i {
            let w = omega0(&reeb_vectors[j], &reeb_vectors[i]);
            action_sum += dwell_times[j] * dwell_times[i] * w;
        }
    }
    let action = action_sum / 2.0;

    Some(OrbitRecovery {
        breakpoints,
        dwell_times,
        max_violation,
        action,
        closure_error,
        facet_sequence: sigma.clone(),
    })
}

// ── Internal helpers ──

/// Compute the maximum halfspace violation for a candidate base point b.
///
/// Returns max_{j,k} (<n_j, b + v_k> - h_j).
fn max_violation_for(
    b: &Vector4<f64>,
    displacements: &[Vector4<f64>],
    normals: &[Vector4<f64>],
    heights: &[f64],
) -> f64 {
    displacements
        .iter()
        .flat_map(|v| {
            let p = b + v;
            normals
                .iter()
                .zip(heights.iter())
                .map(move |(n, &h)| n.dot(&p) - h)
        })
        .fold(f64::NEG_INFINITY, f64::max)
}

/// Find the point b = b0 + sum(alpha_i * d_i) in the null space that
/// minimizes the maximum halfspace violation across all breakpoints.
///
/// Uses iterative coordinate-wise ternary search. The max-violation objective
/// is piecewise-linear and convex in each coordinate, so ternary search
/// converges to the minimum.
fn optimize_in_null_space(
    b0: Vector4<f64>,
    null_vecs: &[Vector4<f64>],
    displacements: &[Vector4<f64>],
    normals: &[Vector4<f64>],
    heights: &[f64],
) -> Vector4<f64> {
    if null_vecs.is_empty() {
        return b0;
    }

    let mut alphas = vec![0.0_f64; null_vecs.len()];

    // Iterative coordinate-wise minimization (20 rounds).
    for _iter in 0..20 {
        for dim in 0..null_vecs.len() {
            let candidate = |a: f64| -> Vector4<f64> {
                let mut b = b0;
                for (i, d) in null_vecs.iter().enumerate() {
                    let ai = if i == dim { a } else { alphas[i] };
                    b += ai * d;
                }
                b
            };

            // Ternary search on alpha[dim] in [-100, 100].
            let mut lo = -100.0_f64;
            let mut hi = 100.0_f64;

            for _ in 0..100 {
                let m1 = lo + (hi - lo) / 3.0;
                let m2 = hi - (hi - lo) / 3.0;
                let v1 = max_violation_for(&candidate(m1), displacements, normals, heights);
                let v2 = max_violation_for(&candidate(m2), displacements, normals, heights);
                if v1 < v2 {
                    hi = m2;
                } else {
                    lo = m1;
                }
            }

            alphas[dim] = (lo + hi) / 2.0;
        }
    }

    let mut b = b0;
    for (i, d) in null_vecs.iter().enumerate() {
        b += alphas[i] * d;
    }
    b
}

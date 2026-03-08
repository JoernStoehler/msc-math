//! Base point recovery for simple Reeb orbits.
//!
//! Given the output (σ, β) from the EHZ capacity algorithm, recovers the
//! base point b = γ(0) ∈ ∂K of the corresponding primal Reeb orbit.
//!
//! # Mathematical background (`[lem:base-point-recovery]`)
//!
//! A simple orbit with parameters (σ, τ) has piecewise-constant velocity
//! R_{σ(k)} on each segment. The position during segment k is:
//!
//!   γ(t) = b + v_k + (t − t_k) · R_{σ(k)}
//!
//! where v_k = Σ_{j<k} τ_j R_{σ(j)} is the accumulated displacement.
//!
//! For γ(t) to lie on facet F_{σ(k)}, we need ⟨n_{σ(k)}, γ(t)⟩ = h_{σ(k)}.
//! Since R_{σ(k)} is tangent to F_{σ(k)} (⟨n_{σ(k)}, R_{σ(k)}⟩ = 0),
//! this reduces to a linear system in b:
//!
//!   ⟨n_{σ(k)}, b⟩ = h_{σ(k)} − ⟨n_{σ(k)}, v_k⟩   for each active k
//!
//! The system has m equations (active facets) and 4 unknowns (b ∈ ℝ⁴).
//! We solve via SVD (least-norm solution when underdetermined).
//!
//! # Conversion from algorithm output (`[rem:beta-to-tau]`)
//!
//! Dwell times: τ_k = T · h_{σ(k)} · β_{σ(k)}, where T = c_EHZ(K).

use super::EhzResult;
use crate::geom::polytope::Polytope4D;
use nalgebra::{DMatrix, DVector, Vector4};

/// Compute the maximum inequality violation for a candidate base point b.
/// Returns max_j max_k (⟨n_j, b + v_k⟩ − h_j).
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

/// Given a particular solution b0 and null space basis vectors, find the
/// point b = b0 + Σ α_i d_i that minimizes the maximum inequality violation
/// across all breakpoints.
///
/// For 1D null space: binary search / golden section on the single parameter.
/// For higher dimensions: iterative coordinate-wise optimization.
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

    // Iterative coordinate-wise minimization of max violation.
    // Each coordinate is optimized by ternary search (the objective is
    // piecewise-linear and convex in each coordinate).
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

            // Ternary search on alpha[dim] in a wide range.
            // The range needs to be large enough to contain the valid region.
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

/// Result of base point recovery.
#[derive(Clone, Debug)]
pub struct BasePointRecovery {
    /// Recovered base point b = γ(0).
    pub base_point: Vector4<f64>,

    /// Dimension of the solution space (4 − rank(N_S)).
    /// 0 means b is unique; >0 means there is a family of valid base points.
    pub solution_dim: usize,

    /// Maximum inequality violation across all breakpoints.
    /// For each breakpoint p = b + v_k, checks max_j(⟨n_j, p⟩ − h_j).
    /// Should be ≤ 0 (or numerically ≈ 0) for a valid orbit.
    pub max_violation: f64,

    /// Dwell times τ_k for each position in the permutation σ.
    /// τ[k] is the time spent with velocity R_{σ(k)}.
    pub dwell_times: Vec<f64>,

    /// Breakpoints of the orbit: b + v_k for k = 0, ..., m.
    /// breakpoints[0] = b, breakpoints[m] = b (closure).
    pub breakpoints: Vec<Vector4<f64>>,
}

/// Recover the base point b from the algorithm output (σ, β).
///
/// See `[lem:base-point-recovery]` (thesis) for the mathematical derivation.
///
/// Returns `None` if the linear system has no solution (should not happen
/// for valid algorithm output, but guards against degenerate input).
pub fn recover_base_point(
    polytope: &Polytope4D,
    result: &EhzResult,
) -> Option<BasePointRecovery> {
    let normals = polytope.normals_f64();
    let heights = polytope.heights_f64();
    let sigma = &result.best_permutation;
    let beta = &result.best_beta;
    let capacity = result.capacity;

    // Step 1: Convert β to dwell times τ.
    // τ_k = T · h_{σ(k)} · β_{σ(k)}, where T = capacity.
    // Note: beta[k] corresponds to σ[k] (the k-th entry in the permutation).
    let dwell_times: Vec<f64> = (0..sigma.len())
        .map(|k| capacity * heights[sigma[k]] * beta[k])
        .collect();

    // Step 2: Compute Reeb vectors R_i = (2/h_i) J₀ n_i.
    let reeb_vectors: Vec<Vector4<f64>> = (0..sigma.len())
        .map(|k| {
            let i = sigma[k];
            let n = &normals[i];
            let h = heights[i];
            // J₀ n = (-n[2], -n[3], n[0], n[1])
            Vector4::new(-n[2], -n[3], n[0], n[1]) * (2.0 / h)
        })
        .collect();

    // Step 3: Compute accumulated displacements v_k = Σ_{j<k} τ_j R_{σ(j)}.
    let m = sigma.len();
    let mut displacements: Vec<Vector4<f64>> = Vec::with_capacity(m + 1);
    displacements.push(Vector4::zeros());
    for k in 0..m {
        let v_next = displacements[k] + dwell_times[k] * reeb_vectors[k];
        displacements.push(v_next);
    }

    // Step 4: Build linear system N_S · b = r.
    // For each active k: ⟨n_{σ(k)}, b⟩ = h_{σ(k)} − ⟨n_{σ(k)}, v_k⟩
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

    // Step 5: Solve via SVD.
    // To get the full null space, pad N_S to a 4×4 matrix (extra zero rows).
    // This ensures SVD produces a 4×4 V^T from which we can extract null vectors.
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

    // Step 5b: If underdetermined (solution_dim > 0), search the null space
    // for a point that satisfies the inequality constraints.
    // Null space basis: rows of V^T with indices >= rank (V^T is 4×4).
    if solution_dim > 0 {
        if let Some(v_mat) = &svd.v_t {
            let null_vecs: Vec<Vector4<f64>> = (rank..4)
                .map(|i| Vector4::new(v_mat[(i, 0)], v_mat[(i, 1)], v_mat[(i, 2)], v_mat[(i, 3)]))
                .collect();

            base_point =
                optimize_in_null_space(base_point, &null_vecs, &displacements, normals, heights);
        }
    }

    // Step 6: Compute breakpoints and check inequality constraints.
    let breakpoints: Vec<Vector4<f64>> = (0..=m)
        .map(|k| base_point + displacements[k])
        .collect();

    // Check: for each breakpoint, verify ⟨n_j, p⟩ ≤ h_j for all j.
    let max_violation = breakpoints
        .iter()
        .flat_map(|p| {
            normals
                .iter()
                .zip(heights.iter())
                .map(move |(n, &h)| n.dot(p) - h)
        })
        .fold(f64::NEG_INFINITY, f64::max);

    Some(BasePointRecovery {
        base_point,
        solution_dim,
        max_violation,
        dwell_times,
        breakpoints,
    })
}

/// Verify that a recovered orbit is valid.
///
/// Checks:
/// 1. Closure: the orbit returns to its starting point
/// 2. On-facet: each segment lies on the correct facet
/// 3. Inside K: the orbit stays inside K at all breakpoints
/// 4. Action: the orbit's action matches the capacity
///
/// Returns a struct with all error metrics.
pub fn verify_orbit(
    polytope: &Polytope4D,
    result: &EhzResult,
    recovery: &BasePointRecovery,
) -> OrbitVerification {
    let normals = polytope.normals_f64();
    let heights = polytope.heights_f64();
    let sigma = &result.best_permutation;
    let m = sigma.len();

    // 1. Closure error: ‖breakpoints[m] − breakpoints[0]‖
    let closure_error = (recovery.breakpoints[m] - recovery.breakpoints[0]).norm();

    // 2. On-facet error: max_k |⟨n_{σ(k)}, breakpoint_k⟩ − h_{σ(k)}|
    let on_facet_error = (0..m)
        .filter(|&k| recovery.dwell_times[k] > 0.0)
        .map(|k| {
            let i = sigma[k];
            (normals[i].dot(&recovery.breakpoints[k]) - heights[i]).abs()
        })
        .fold(0.0_f64, f64::max);

    // 3. Inside-K error: max violation across all breakpoints
    let inside_k_error = recovery.max_violation;

    // 4. Action: compute A(γ) = ½ Σ_{j<i} τ_j τ_i ω₀(R_{σ(j)}, R_{σ(i)})
    //    via the shoelace formula [lem:shoelace].
    let reeb_vectors: Vec<Vector4<f64>> = (0..m)
        .map(|k| {
            let i = sigma[k];
            let n = &normals[i];
            let h = heights[i];
            Vector4::new(-n[2], -n[3], n[0], n[1]) * (2.0 / h)
        })
        .collect();

    let mut action_sum = 0.0;
    for i in 1..m {
        for j in 0..i {
            let omega = crate::geom::symplectic::omega0(&reeb_vectors[j], &reeb_vectors[i]);
            action_sum += recovery.dwell_times[j] * recovery.dwell_times[i] * omega;
        }
    }
    let computed_action = action_sum / 2.0;
    let action_error = (computed_action - result.capacity).abs();

    OrbitVerification {
        closure_error,
        on_facet_error,
        inside_k_error,
        computed_action,
        action_error,
    }
}

/// Verification metrics for a recovered orbit.
#[derive(Clone, Debug)]
pub struct OrbitVerification {
    /// ‖breakpoints[m] − breakpoints[0]‖. Should be ≈ 0.
    pub closure_error: f64,
    /// Max |⟨n_{σ(k)}, breakpoint_k⟩ − h_{σ(k)}| over active segments. Should be ≈ 0.
    pub on_facet_error: f64,
    /// Max ⟨n_j, p⟩ − h_j over all breakpoints and facets. Should be ≤ 0 (or ≈ 0).
    pub inside_k_error: f64,
    /// Action computed via shoelace formula. Should equal capacity.
    pub computed_action: f64,
    /// |computed_action − capacity|. Should be ≈ 0.
    pub action_error: f64,
}

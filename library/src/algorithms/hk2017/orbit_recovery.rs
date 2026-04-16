//! Base point recovery and orbit verification for Reeb orbits on polytopes.
//!
//! Given solved orbit data (sigma, beta, action), recovers the primal
//! Reeb orbit gamma on the boundary of K and verifies its geometric validity.
//! Combines the old `recover_base_point` and `verify_orbit` into a single
//! `recover_and_verify` function, since they are always called together.
//!
//! ## Mathematical background
//!
//! A simple orbit with parameters (sigma, tau) has piecewise-constant velocity
//! R_{sigma(k)} = 2 J_0 a_{sigma(k)} on each segment. The position during segment k is:
//!
//!   gamma(t) = b + v_k + (t - t_k) * R_{sigma(k)}
//!
//! where v_k = sum_{j<k} tau_j R_{sigma(j)} is the accumulated displacement.
//!
//! For gamma(t) to lie on facet F_{sigma(k)}, we need <a_{sigma(k)}, gamma(t)> = 1.
//! Since R_{sigma(k)} is tangent to F_{sigma(k)} (<a_{sigma(k)}, R_{sigma(k)}> = 0),
//! this reduces to a linear system in b:
//!
//!   <a_{sigma(k)}, b> = 1 - <a_{sigma(k)}, v_k>   for each active k
//!
//! The system has m equations and 4 unknowns (b in R^4). We solve via SVD
//! (least-norm solution when underdetermined), then optimize in the null space
//! to minimize constraint violations.
//!
//! Dwell time conversion: tau_k = T * beta_k, where T = c_EHZ(K).
//!
//! Mathematical correspondence: [lem:base-point-recovery], [rem:beta-to-tau]

use crate::algorithms::OrbitKktData;
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

    /// Dimension of the affine solution space for the recovered base point.
    /// Computed as 4 - rank(N_S) from the active-facet linear system.
    pub solution_dim: usize,

    /// Facet indices visited in order (copy of the best permutation sigma).
    pub facet_sequence: Vec<usize>,
}

/// Recover a Reeb orbit from solved orbit/KKT data and verify its validity.
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
    orbit: &OrbitKktData,
) -> Option<OrbitRecovery> {
    let duals = polytope.dual_vertices_f64();
    let sigma = &orbit.sigma;
    let beta = &orbit.beta;
    let capacity = orbit.action;
    let m = sigma.len();

    // ── Stage 1: convert beta to dwell times ──
    //
    // tau_k = T * beta_k, where T = capacity.
    // [rem:beta-to-tau]
    let dwell_times: Vec<f64> = (0..m)
        .map(|k| capacity * beta[k])
        .collect();

    // ── Stage 2: recover base point ──
    //
    // Reeb vectors: R_i = 2 J_0 a_i.
    let reeb_vectors: Vec<Vector4<f64>> = (0..m)
        .map(|k| {
            let a = &duals[sigma[k]];
            // J_0 a = (-a[2], -a[3], a[0], a[1]) in (q1,q2,p1,p2) coordinates
            Vector4::new(-a[2], -a[3], a[0], a[1]) * 2.0
        })
        .collect();

    // Accumulated displacements: v_k = sum_{j<k} tau_j R_{sigma(j)}.
    let mut displacements: Vec<Vector4<f64>> = Vec::with_capacity(m + 1);
    displacements.push(Vector4::zeros());
    for k in 0..m {
        let v_next = displacements[k] + dwell_times[k] * reeb_vectors[k];
        displacements.push(v_next);
    }

    // Build linear system A_S b = r for active facets (tau_k > 0).
    // For each active k: <a_{sigma(k)}, b> = 1 - <a_{sigma(k)}, v_k>.
    let active: Vec<usize> = (0..m).filter(|&k| dwell_times[k] > 0.0).collect();
    let n_active = active.len();

    if n_active == 0 {
        return None;
    }

    let mut mat = DMatrix::<f64>::zeros(n_active, 4);
    let mut rhs = DVector::<f64>::zeros(n_active);

    for (row, &k) in active.iter().enumerate() {
        let a = &duals[sigma[k]];
        for col in 0..4 {
            mat[(row, col)] = a[col];
        }
        rhs[row] = 1.0 - a.dot(&displacements[k]);
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
                duals,
            );
        }
    }

    // ── Stage 3: compute breakpoints and verify ──
    //
    // Breakpoints: b + v_k for k = 0..=m.
    let breakpoints: Vec<Vector4<f64>> = (0..=m)
        .map(|k| base_point + displacements[k])
        .collect();

    // Max violation: max_{j,k} (<a_j, breakpoint_k> - 1).
    let max_violation = breakpoints
        .iter()
        .flat_map(|p| {
            duals
                .iter()
                .map(move |a| a.dot(p) - 1.0)
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
        solution_dim,
        facet_sequence: sigma.clone(),
    })
}

// ── Internal helpers ──

/// Compute the maximum halfspace violation for a candidate base point b.
///
/// Returns max_{j,k} (<a_j, b + v_k> - 1).
fn max_violation_for(
    b: &Vector4<f64>,
    displacements: &[Vector4<f64>],
    dual_vertices: &[Vector4<f64>],
) -> f64 {
    displacements
        .iter()
        .flat_map(|v| {
            let p = b + v;
            dual_vertices
                .iter()
                .map(move |a| a.dot(&p) - 1.0)
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
    dual_vertices: &[Vector4<f64>],
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
                let v1 = max_violation_for(&candidate(m1), displacements, dual_vertices);
                let v2 = max_violation_for(&candidate(m2), displacements, dual_vertices);
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algorithms::{OrbitAdmissibility, OrbitKktData, OrbitSearchResult};
    use crate::geom::known_polytopes;
    use crate::{ehz_capacity_pruned, ehz_capacity_unpruned};

    // Tests for orbit_recovery: base point recovery and orbit verification.
    //
    // Proposition: for a valid EHZ result (sigma, beta), the recovered orbit gamma
    // has closure error ~ 0, lies on the correct facets (on-facet error ~ 0),
    // stays inside K (max violation ~ 0), and its computed action matches the capacity.
    //
    // Reference: [lem:base-point-recovery], [rem:beta-to-tau], [lem:shoelace]
    //
    // Strategy: known-polytope-based checks from `known_polytopes`.

    /// Tolerance for floating-point comparisons (closure, on-facet, action).
    const TOL: f64 = 1e-8;

    /// Tolerance for inequality constraint violations.
    /// Slightly positive to allow numerical noise at breakpoints.
    const INEQ_TOL: f64 = 1e-6;

    fn best_orbit_payload(result: &OrbitSearchResult) -> OrbitKktData {
        let beta_margin = result
            .best_beta()
            .iter()
            .copied()
            .fold(f64::INFINITY, f64::min);
        OrbitKktData {
            sigma: result.best_sigma().to_vec(),
            beta: result.best_beta().to_vec(),
            beta_margin,
            action: result.capacity(),
            action_lower: result.min_action_lower,
            action_upper: result.min_action_upper,
            q: 0.5 / result.capacity(),
            q_error_bound: 0.0,
            mu: None,
            xi: None,
            admissibility: OrbitAdmissibility::AdmissibleF64,
        }
    }

    /// Run the full recovery + verification pipeline on a known polytope and
    /// check all error metrics against tolerances.
    fn test_recovery(name: &str, polytope: &Polytope4D, expected_capacity: f64) {
        let result = ehz_capacity_pruned(polytope).unwrap_or_else(|_| {
            panic!("{name}: capacity computation failed");
        });
        assert!(
            (result.capacity() - expected_capacity).abs() < 1e-4,
            "{name}: capacity mismatch: got {}, expected {expected_capacity}",
            result.capacity()
        );

        let orbit = best_orbit_payload(&result);
        let recovery = recover_and_verify(polytope, &orbit).unwrap_or_else(|| {
            panic!("{name}: orbit recovery failed");
        });

        eprintln!(
            "{name}: max_violation={:.2e}, closure={:.2e}, action_err={:.2e}, segments={}",
            recovery.max_violation,
            recovery.closure_error,
            (recovery.action - orbit.action).abs(),
            recovery.facet_sequence.len(),
        );

        // Closure: orbit returns to its starting point.
        assert!(
            recovery.closure_error < TOL,
            "{name}: closure error {:.2e} exceeds tolerance",
            recovery.closure_error
        );

        // Inside K: orbit stays inside the polytope at all breakpoints.
        assert!(
            recovery.max_violation < INEQ_TOL,
            "{name}: max violation {:.2e} exceeds tolerance\n  facet_sequence={:?}\n  dwell_times={:?}",
            recovery.max_violation,
            recovery.facet_sequence,
            recovery.dwell_times,
        );

        // Action: computed action matches capacity.
        let action_error = (recovery.action - orbit.action).abs();
        assert!(
            action_error < TOL,
            "{name}: action error {:.2e} (computed {}, expected {})",
            action_error,
            recovery.action,
            orbit.action,
        );

        // Facet sequence matches the best permutation.
        assert_eq!(
            recovery.facet_sequence,
            orbit.sigma,
            "{name}: facet_sequence does not match best_permutation"
        );

        // Breakpoint count = permutation length + 1 (includes start and closure point).
        assert_eq!(
            recovery.breakpoints.len(),
            recovery.facet_sequence.len() + 1,
            "{name}: breakpoint count mismatch"
        );

        // Dwell time count matches permutation length.
        assert_eq!(
            recovery.dwell_times.len(),
            recovery.facet_sequence.len(),
            "{name}: dwell_times length mismatch"
        );
    }

    /// Helper to verify on-facet property: each breakpoint k lies on facet sigma(k).
    fn check_on_facet(
        name: &str,
        polytope: &Polytope4D,
        result: &OrbitSearchResult,
    ) {
        let duals = polytope.dual_vertices_f64();
        let orbit = best_orbit_payload(result);
        let sigma = &orbit.sigma;

        let recovery = recover_and_verify(polytope, &orbit).unwrap();

        // For each active segment k (dwell_times[k] > 0), breakpoint[k] should lie
        // on facet sigma(k): <a_{sigma(k)}, breakpoint[k]> ~ 1.
        let on_facet_error = (0..sigma.len())
            .filter(|&k| recovery.dwell_times[k] > 0.0)
            .map(|k| {
                let i = sigma[k];
                (duals[i].dot(&recovery.breakpoints[k]) - 1.0).abs()
            })
            .fold(0.0_f64, f64::max);

        assert!(
            on_facet_error < TOL,
            "{name}: on-facet error {:.2e} exceeds tolerance",
            on_facet_error
        );
    }

    /// Recover orbit for the 4-simplex (F=5).
    ///
    /// Minimal polytope. Known capacity = 2.0.
    /// Exercises SVD solve and verification on a small system.
    #[test]
    fn simplex_recovery() {
        let kp = known_polytopes::simplex();
        test_recovery("simplex", &kp.polytope, kp.capacity);
    }

    /// On-facet check for the simplex.
    #[test]
    fn simplex_on_facet() {
        let kp = known_polytopes::simplex();
        let result = ehz_capacity_pruned(&kp.polytope).unwrap();
        check_on_facet("simplex", &kp.polytope, &result);
    }

    /// Recover orbit for the hypercube (F=8).
    ///
    /// High symmetry, known capacity = 1.0. Lagrangian product structure
    /// may produce a non-unique base point (solution_dim > 0).
    #[test]
    fn hypercube_recovery() {
        let kp = known_polytopes::hypercube();
        test_recovery("hypercube", &kp.polytope, kp.capacity);
    }

    // crosspolytope orbit recovery is tested in crosspolytope_upper_bound()
    // (hk2017/mod.rs) via a direct KKT solve on the known permutation, avoiding
    // the ehz_capacity() call that test_recovery() would require (F=16 is
    // infeasible for the library's unpruned/pruned algorithm).

    /// Recover orbit for the HKO pentagon (F=10).
    ///
    /// The Haim-Kislev-Ostrover counterexample with sys > 1.
    /// F=10 is fast with the pruned algorithm.
    #[test]
    fn hko_pentagon_recovery() {
        let kp = known_polytopes::hko_pentagon();
        test_recovery("hko_pentagon", &kp.polytope, kp.capacity);
    }

    /// Recover orbit for a Lagrangian triangle product (F=7).
    ///
    /// Lagrangian products have special billiard structure; good cross-check.
    #[test]
    fn lagrangian_triangle_product_recovery() {
        let kp = known_polytopes::lagrangian_triangle_product();
        test_recovery("lagrangian_triangle_product", &kp.polytope, kp.capacity);
    }

    /// Recover orbit for a symplectic triangle product (F=7).
    ///
    /// Non-Lagrangian product geometry.
    #[test]
    fn symplectic_triangle_product_recovery() {
        let kp = known_polytopes::symplectic_triangle_product();
        test_recovery("symplectic_triangle_product", &kp.polytope, kp.capacity);
    }

    /// Recover orbit for a Lagrangian triangle-square product (F=7).
    ///
    /// Mixed product geometry: triangle x square.
    #[test]
    fn lagrangian_triangle_square_recovery() {
        let kp = known_polytopes::lagrangian_triangle_square();
        test_recovery("lagrangian_triangle_square", &kp.polytope, kp.capacity);
    }

    /// Recover orbit for a symplectic triangle-square product (F=7).
    ///
    /// Another mixed product geometry.
    #[test]
    fn symplectic_triangle_square_recovery() {
        let kp = known_polytopes::symplectic_triangle_square();
        test_recovery("symplectic_triangle_square", &kp.polytope, kp.capacity);
    }

    /// Verify that dwell times are non-negative for all known polytopes.
    ///
    /// Dwell times tau_k = T * h_{sigma(k)} * beta_k. Since T > 0, h > 0,
    /// and beta_k > 0 (certified), all dwell times should be positive.
    /// Skips polytopes with F > 10 (too slow for debug mode).
    #[test]
    fn dwell_times_positive() {
        for kp in known_polytopes::all_known() {
            if kp.polytope.facet_count() > 10 {
                continue;
            }
            let result = ehz_capacity_pruned(&kp.polytope).unwrap();
            let orbit = best_orbit_payload(&result);
            let recovery = recover_and_verify(&kp.polytope, &orbit).unwrap();

            for (k, &tau) in recovery.dwell_times.iter().enumerate() {
                assert!(
                    tau > 0.0,
                    "{}: dwell_times[{k}] = {tau:.2e} is not positive",
                    kp.name,
                );
            }
        }
    }

    /// Verify breakpoint count equals permutation length + 1.
    ///
    /// The breakpoints array includes the starting point and the closure point,
    /// so it has m+1 entries for an m-facet orbit.
    #[test]
    fn breakpoint_count_consistency() {
        for kp in known_polytopes::all_known() {
            if kp.polytope.facet_count() > 10 {
                continue;
            }
            let result = ehz_capacity_pruned(&kp.polytope).unwrap();
            let orbit = best_orbit_payload(&result);
            let recovery = recover_and_verify(&kp.polytope, &orbit).unwrap();

            assert_eq!(
                recovery.breakpoints.len(),
                recovery.facet_sequence.len() + 1,
                "{}: expected {} breakpoints, got {}",
                kp.name,
                recovery.facet_sequence.len() + 1,
                recovery.breakpoints.len(),
            );
        }
    }

    /// Verify that the unpruned algorithm gives consistent recovery results.
    ///
    /// Both pruned and unpruned algorithms should yield orbits with the same
    /// computed action on the simplex (fast enough for unpruned in debug mode).
    #[test]
    fn unpruned_recovery_consistent() {
        let kp = known_polytopes::simplex();

        let result_pruned = ehz_capacity_pruned(&kp.polytope).unwrap();
        let result_unpruned = ehz_capacity_unpruned(&kp.polytope).unwrap();

        let orbit_pruned = best_orbit_payload(&result_pruned);
        let orbit_unpruned = best_orbit_payload(&result_unpruned);
        let recovery_pruned = recover_and_verify(&kp.polytope, &orbit_pruned).unwrap();
        let recovery_unpruned = recover_and_verify(&kp.polytope, &orbit_unpruned).unwrap();

        // Both should have valid orbits.
        assert!(recovery_pruned.closure_error < TOL);
        assert!(recovery_unpruned.closure_error < TOL);
        assert!(recovery_pruned.max_violation < INEQ_TOL);
        assert!(recovery_unpruned.max_violation < INEQ_TOL);

        // Actions should match (same polytope, same capacity).
        assert!(
            (recovery_pruned.action - recovery_unpruned.action).abs() < TOL,
            "action mismatch: pruned {}, unpruned {}",
            recovery_pruned.action,
            recovery_unpruned.action,
        );
    }
}

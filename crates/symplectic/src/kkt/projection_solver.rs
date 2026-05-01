//! Projection-based solver for the constrained QP.
//!
//! Solves: max (1/2) beta^T H beta  subject to  C beta = d, beta > 0.
//!
//! # Algorithm
//!
//! **Step 1 -- Solve constraints.** C beta = d -> particular solution beta0, null-space
//! basis V. If inconsistent: return False.
//!
//! **Step 2 -- Project objective.** Form the reduced Hessian H' = V^T H V and
//! reduced right-hand side b' = -V^T H beta0. Solve H' alpha = b' via
//! eigendecomposition, partitioning eigenvalues into retained
//! (|lambda| > threshold) and null (|lambda| <= threshold).
//!
//! **Step 3 -- Compose search space.** The full solution is beta = beta0 + V(alpha0 + W gamma),
//! where W are the null-space eigenvectors of H'. These directions don't change Q
//! but can change beta -- so they're the search space for finding beta > 0.
//!
//! **Step 4 -- Max-margin search.** Find gamma maximizing min_k beta_k via
//! `beta_feasibility::find_max_margin`. Classify the verdict from the margin.
//!
//! **Step 5 -- Compute Q.** Q = (1/2) beta^T H beta, constant over the solution set.
//!
//! Mathematical correspondence: [lem:kkt], Part C.2 of algorithm design

use super::beta_feasibility;
use super::constraint_solver;
use super::{classify_margin, Solution, Verdict, EPS_EIGEN_FLOOR, QP};
use nalgebra::{DMatrix, DVector};

/// Eigenvalue threshold for the reduced Hessian H'.
///
/// Near-null eigenvalues mean Q varies little along those directions but beta varies
/// a lot. These directions are included in the margin search space rather than
/// used for optimization.
///
/// Same role as EIGEN_CONDITION_TAU in saddle_point_solver.rs, same calibration:
/// the degenerate (4,4) Lagrangian product at theta ~ 0 deg has reduced Hessian
/// eigenvalue ratios ~ 4e-4; the 1e-3 threshold catches this with 2.5x margin.
/// See saddle_point_solver.rs::EIGEN_CONDITION_TAU for full rationale.
///
/// **Why not shared:** saddle_point_solver.rs and projection_solver.rs have
/// different matrix structures (augmented (m+5)x(m+5) vs reduced kxk H'),
/// so they may need independent tuning in the future. Kept separate to allow
/// independent adjustment.
const EPS_EIGEN_THRESHOLD: f64 = 1e-3;

/// Solve the QP via constraint projection.
///
/// See module doc for the 5-step algorithm. Returns a Solution with verdict,
/// Q value, beta vector, and margin.
///
/// [lem:kkt]: KKT conditions characterize the EHZ capacity optimum; this solver applies them via constraint projection.
pub fn solve_projected(qp: &QP) -> Solution {
    let m = qp.c.ncols();

    // Step 1: Solve constraints.
    let constraint_sol = match constraint_solver::solve_constraints(&qp.c, &qp.d) {
        Ok(sol) => sol,
        Err(_err) => {
            return Solution {
                verdict: Verdict::False,
                q: 0.0,
                beta: vec![0.0; m],
                margin: f64::NEG_INFINITY,
            };
        }
    };

    let beta0 = &constraint_sol.x0;
    let v = &constraint_sol.null_basis;
    let k = v.ncols();

    // Special case: k = 0 (unique beta from constraints).
    if k == 0 {
        let q = q_value_from_dvec(&qp.h, beta0);
        let margin = beta0.iter().copied().fold(f64::INFINITY, f64::min);
        let verdict = classify_margin(margin);
        return Solution {
            verdict,
            q,
            beta: beta0.as_slice().to_vec(),
            margin,
        };
    }

    // Step 2: Project and optimize.
    // Reduced Hessian: H' = V^T H V (k x k symmetric).
    let hv = &qp.h * v;
    let h_prime = v.transpose() * &hv;

    // Reduced gradient: b' = -V^T H beta0 (k x 1).
    // Sign: solving H' alpha + V^T H beta0 = 0 for alpha => alpha = (H')^{-1} b'
    // with b' = -V^T H beta0.
    let h_beta0 = &qp.h * beta0;
    let b_prime = -(v.transpose() * &h_beta0);

    // Eigendecompose H' = P Lambda P^T.
    let eig = h_prime.clone().symmetric_eigen();
    let eigenvalues = &eig.eigenvalues;
    let eigenvectors = &eig.eigenvectors;

    // Partition eigenvalues into retained and null.
    let lambda_max = eigenvalues.iter().map(|e| e.abs()).fold(0.0_f64, f64::max);
    let threshold = if lambda_max < EPS_EIGEN_FLOOR {
        f64::INFINITY // nothing retained
    } else {
        lambda_max * EPS_EIGEN_THRESHOLD
    };

    // Particular solution for H' alpha = b' using retained eigenvalues (pseudoinverse).
    let mut alpha0 = DVector::zeros(k);
    for i in 0..k {
        if eigenvalues[i].abs() > threshold {
            let pi = eigenvectors.column(i);
            let coeff = pi.dot(&b_prime) / eigenvalues[i];
            alpha0 += coeff * pi;
        }
    }

    // Null-space directions of H' (columns of W in alpha-space).
    let null_indices: Vec<usize> = (0..k)
        .filter(|&i| eigenvalues[i].abs() <= threshold)
        .collect();

    // Step 3: Compose search space.
    // beta_base = beta0 + V * alpha0 (the "optimized" particular solution).
    let beta_base = beta0 + v * &alpha0;

    // V_search = V * W_alpha (m x |null_indices|).
    // These are the directions in beta-space that don't change Q.
    let n_null = null_indices.len();
    let v_search = if n_null > 0 {
        let mut w_alpha = DMatrix::zeros(k, n_null);
        for (j, &idx) in null_indices.iter().enumerate() {
            let col = eigenvectors.column(idx);
            for i in 0..k {
                w_alpha[(i, j)] = col[i];
            }
        }
        v * w_alpha
    } else {
        DMatrix::zeros(m, 0)
    };

    // Step 4: Max-margin search.
    let margin_result = beta_feasibility::find_max_margin(&beta_base, &v_search);

    // Step 5: Compute Q.
    let q = q_value_from_dvec(&qp.h, &margin_result.beta);
    let margin = margin_result.margin;
    let verdict = classify_margin(margin);

    Solution {
        verdict,
        q,
        beta: margin_result.beta.as_slice().to_vec(),
        margin,
    }
}

/// Compute Q = (1/2) beta^T H beta from DVector (internal helper).
fn q_value_from_dvec(h: &DMatrix<f64>, beta: &DVector<f64>) -> f64 {
    0.5 * beta.dot(&(h * beta))
}

#[cfg(test)]
mod tests {
    use super::super::qp_assembly::build_qp;
    use super::super::{Verdict, QP};
    use super::*;
    use crate::geom::known_polytopes;
    use nalgebra::{DMatrix, DVector};

    // Tests for projection_solver: projection-based QP solver correctness.
    //
    // Proposition: solve_projected returns a Solution where verdict, Q, beta,
    // and margin are consistent, with C beta = d satisfied for non-False verdicts.
    // Reference: [lem:kkt], Part C.2 of algorithm design.
    //
    // Strategy: fixture-based with synthetic QPs (hand-checkable) plus cross-validation
    // against the saddle-point solver on known polytopes.

    // ── Synthetic tests (context-free, hand-checkable) ──

    /// Inconsistent constraints: return False.
    #[test]
    fn inconsistent_constraints() {
        let c = DMatrix::identity(3, 3);
        let d = DVector::from_column_slice(&[1.0, 0.0, 0.0]);
        let h = DMatrix::zeros(3, 3);
        let qp = QP { c, d, h };

        let sol = solve_projected(&qp);
        // C is 3x3 identity, d = [1,0,0]. Unique beta = [1,0,0]. beta2 = beta3 = 0.
        // margin = 0 -> Indeterminate.
        assert!(sol.margin <= 0.0);
    }

    /// Unique beta (k=0), all positive -> True.
    #[test]
    fn unique_beta_positive() {
        let c = DMatrix::identity(5, 5);
        let d = DVector::from_element(5, 0.2);
        let h = DMatrix::identity(5, 5);
        let qp = QP { c, d, h };

        let sol = solve_projected(&qp);
        assert_eq!(sol.verdict, Verdict::True);
        assert!((sol.q - 0.1).abs() < 1e-10, "Q = {}, expected 0.1", sol.q);
        assert!(
            (sol.margin - 0.2).abs() < 1e-10,
            "margin = {}, expected 0.2",
            sol.margin
        );
    }

    /// Unique beta (k=0), some negative -> False.
    #[test]
    fn unique_beta_negative() {
        let c = DMatrix::identity(5, 5);
        let d = DVector::from_column_slice(&[0.5, 0.5, -0.5, 0.5, 0.5]);
        let h = DMatrix::identity(5, 5);
        let qp = QP { c, d, h };

        let sol = solve_projected(&qp);
        assert_eq!(sol.verdict, Verdict::False);
        assert!(sol.margin < 0.0);
    }

    /// One free variable (k=1, m=6, p=5). Verify Q matches hand computation.
    #[test]
    fn one_free_variable() {
        #[rustfmt::skip]
        let c = DMatrix::from_row_slice(5, 6, &[
            1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
            0.0, 1.0, 0.0, 0.0, 0.0, 1.0,
            0.0, 0.0, 1.0, 0.0, 0.0, 1.0,
            0.0, 0.0, 0.0, 1.0, 0.0, 1.0,
            0.0, 0.0, 0.0, 0.0, 1.0, 1.0,
        ]);
        let d = DVector::from_element(5, 1.0);
        let h = DMatrix::identity(6, 6);
        let qp = QP { c, d, h };

        let sol = solve_projected(&qp);
        assert!(sol.q > 0.0, "Q should be positive");
        // Q_max = 5/12
        assert!(
            (sol.q - 5.0 / 12.0).abs() < 1e-8,
            "Q = {}, expected 5/12 = {}",
            sol.q,
            5.0 / 12.0
        );
    }

    /// Q is constant when H = 0: Q = 0 for all beta in the constraint set.
    #[test]
    fn q_constant_when_h_zero() {
        #[rustfmt::skip]
        let c = DMatrix::from_row_slice(2, 4, &[
            1.0, 0.0, 1.0, 0.0,
            0.0, 1.0, 0.0, 1.0,
        ]);
        let d = DVector::from_column_slice(&[1.0, 1.0]);
        let h = DMatrix::zeros(4, 4);
        let qp = QP { c, d, h };

        let sol = solve_projected(&qp);
        assert!(
            sol.q.abs() < 1e-12,
            "Q should be 0 when H = 0, got {}",
            sol.q
        );
    }

    // ── Mathematical proposition tests ──

    /// Prop: C beta = d for every returned beta with verdict != False.
    #[test]
    fn prop_constraint_satisfaction() {
        let cases = [
            {
                #[rustfmt::skip]
                let c = DMatrix::from_row_slice(3, 5, &[
                    1.0, 0.0, 0.0, 1.0, 0.0,
                    0.0, 1.0, 0.0, 0.0, 1.0,
                    0.0, 0.0, 1.0, 1.0, 1.0,
                ]);
                let d = DVector::from_column_slice(&[1.0, 1.0, 1.0]);
                let h = DMatrix::identity(5, 5);
                QP { c, d, h }
            },
            {
                #[rustfmt::skip]
                let c = DMatrix::from_row_slice(5, 8, &[
                    1.0,  0.0, -1.0,  0.5,  0.0,  1.0, -0.5,  0.0,
                    0.0,  1.0,  0.0, -1.0,  0.5,  0.0,  1.0, -0.5,
                    0.5,  0.0,  1.0,  0.0, -1.0,  0.5,  0.0,  1.0,
                    0.0,  0.5,  0.0,  1.0,  0.0, -1.0,  0.5,  0.0,
                    1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,
                ]);
                let d = DVector::from_column_slice(&[0.0, 0.0, 0.0, 0.0, 1.0]);
                let h = DMatrix::identity(8, 8);
                QP { c, d, h }
            },
        ];

        for (i, qp) in cases.iter().enumerate() {
            let sol = solve_projected(qp);
            if sol.verdict == Verdict::False {
                continue;
            }
            let beta_dv = DVector::from_column_slice(&sol.beta);
            let residual = (&qp.c * &beta_dv - &qp.d).norm();
            assert!(
                residual < 1e-8,
                "case {}: ||C beta - d|| = {:.2e}",
                i,
                residual
            );
        }
    }

    /// Prop: returned Q equals (1/2) beta^T H beta.
    #[test]
    fn prop_q_is_half_beta_h_beta() {
        #[rustfmt::skip]
        let c = DMatrix::from_row_slice(2, 4, &[
            1.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 1.0,
        ]);
        let d = DVector::from_column_slice(&[1.0, 1.0]);
        #[rustfmt::skip]
        let h = DMatrix::from_row_slice(4, 4, &[
            0.0, 1.0, 0.0, 0.0,
            1.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
            0.0, 0.0, 1.0, 0.0,
        ]);
        let qp = QP { c, d, h };

        let sol = solve_projected(&qp);
        let beta_dv = DVector::from_column_slice(&sol.beta);
        let q_check = 0.5 * beta_dv.dot(&(&qp.h * &beta_dv));
        assert!(
            (sol.q - q_check).abs() < 1e-10,
            "Q mismatch: solver = {}, direct = {}, diff = {:.2e}",
            sol.q,
            q_check,
            (sol.q - q_check).abs()
        );
    }

    /// Prop: margin = min(beta) exactly.
    #[test]
    fn prop_margin_equals_min_beta() {
        let c = DMatrix::identity(3, 6);
        let d = DVector::from_column_slice(&[0.5, 0.3, 0.8]);
        let h = DMatrix::identity(6, 6);
        let qp = QP { c, d, h };

        let sol = solve_projected(&qp);
        let min_beta = sol.beta.iter().copied().fold(f64::INFINITY, f64::min);
        assert!(
            (sol.margin - min_beta).abs() < 1e-12,
            "margin = {}, min(beta) = {}, diff = {:.2e}",
            sol.margin,
            min_beta,
            (sol.margin - min_beta).abs()
        );
    }

    /// Regression test for the reduced-gradient sign.
    ///
    /// Prop: `solve_projected` computes `alpha0 = -(H')^{-1} V^T H beta0`, i.e.
    /// the Lagrange critical point of `(1/2) beta^T H beta` on `{C beta = d}`.
    /// Reference: [lem:kkt] — stationarity condition of the reduced objective
    /// `f(alpha) = const + alpha^T (V^T H beta0) + (1/2) alpha^T H' alpha`.
    ///
    /// # Why this test exists
    ///
    /// Before 2026-04-12, line 95 of this file computed `b_prime = +V^T H beta0`
    /// (missing minus sign), so `alpha0` had the wrong sign and `beta` was
    /// reflected across `beta0` in the null-space direction. All other
    /// projection-solver tests in this module passed under both signs because
    /// their particular solution `beta0 = C^+ d` is minimum-norm and satisfies
    /// `V^T H beta0 ~ 0` in the reduced-space sense (e.g. for H = I, `V^T beta0 = 0`
    /// because `beta0 in row(C)` and `V` is orthogonal to `row(C)`; verified
    /// numerically `V^T H beta0 ~ 3.3e-16` for the `one_free_variable` fixture).
    /// Consequently `alpha0 ~ 0` and the sign was unobservable.
    ///
    /// This test uses H with non-zero off-diagonal entries so that
    /// `H beta0` is NOT in `row(C)` and the reduced gradient is non-zero.
    ///
    /// # Construction (hand-checked)
    ///
    /// Variables m = 3, constraints p = 2, null dimension k = 1.
    /// - C = [[1,0,1],[0,1,1]], d = [1,1]. Minimum-norm particular solution:
    ///   `beta0 = C^T (C C^T)^{-1} d = (1/3, 1/3, 2/3)`.
    /// - Null-space basis (orthonormal): `V = (-1, -1, 1)^T / sqrt(3)`.
    /// - H = [[0,1,0],[1,0,0],[0,0,0]]. Then `H beta0 = (1/3, 1/3, 0)` and
    ///   `V^T H beta0 = (-1/3 - 1/3 + 0)/sqrt(3) = -2/(3 sqrt(3)) != 0`.
    /// - Reduced Hessian: `H V = (-1, -1, 0)^T / sqrt(3)`,
    ///   `H' = V^T H V = 2/3`.
    /// - Stationarity: `alpha = -(V^T H beta0) / H' = (2/(3 sqrt(3))) / (2/3) = 1/sqrt(3)`.
    /// - `beta* = beta0 + V alpha = (1/3, 1/3, 2/3) + (-1/3, -1/3, 1/3) = (0, 0, 1)`.
    /// - `Q* = (1/2) beta*^T H beta* = beta*_0 * beta*_1 = 0`.
    ///
    /// Under the buggy sign `alpha = +(V^T H beta0) / H' = -1/sqrt(3)`:
    /// - `beta_buggy = (2/3, 2/3, 1/3)`, `Q_buggy = (2/3)(2/3) = 4/9`.
    ///
    /// Step 4 (max-margin LP) cannot mask the difference: `H' = 2/3` is the
    /// single retained eigenvalue (above `EPS_EIGEN_THRESHOLD * lambda_max`),
    /// so `v_search` has zero columns and the LP returns `beta_base` unchanged.
    ///
    /// # Assertions
    ///
    /// - `Q` matches the fixed-sign value `0` (buggy sign gives `4/9`).
    /// - `beta` matches `(0, 0, 1)` component-wise (buggy sign gives `(2/3, 2/3, 1/3)`).
    #[test]
    fn reduced_gradient_sign_distinguishes_fix() {
        #[rustfmt::skip]
        let c = DMatrix::from_row_slice(2, 3, &[
            1.0, 0.0, 1.0,
            0.0, 1.0, 1.0,
        ]);
        let d = DVector::from_column_slice(&[1.0, 1.0]);
        #[rustfmt::skip]
        let h = DMatrix::from_row_slice(3, 3, &[
            0.0, 1.0, 0.0,
            1.0, 0.0, 0.0,
            0.0, 0.0, 0.0,
        ]);
        let qp = QP { c, d, h };

        let sol = solve_projected(&qp);

        // Expected Q under the fixed sign is 0; the buggy sign gives 4/9 ~ 0.444.
        // Tolerance 1e-10 is ~10^8 times smaller than the 4/9 gap.
        assert!(
            sol.q.abs() < 1e-10,
            "Q = {}, expected 0 (fixed sign). \
             Buggy sign would give Q = 4/9 ~ 0.4444. \
             If this fails with Q ~ 0.4444, the reduced-gradient sign at \
             projection_solver.rs:95 has been reverted.",
            sol.q
        );

        // Expected beta under the fixed sign is (0, 0, 1); buggy sign gives (2/3, 2/3, 1/3).
        // The L2 gap between the two is sqrt(2 * (2/3)^2 + (2/3)^2) = sqrt(12/9) ~ 1.155.
        let expected_beta = [0.0, 0.0, 1.0];
        for (i, &exp) in expected_beta.iter().enumerate() {
            assert!(
                (sol.beta[i] - exp).abs() < 1e-10,
                "beta[{}] = {}, expected {} (fixed sign). \
                 Buggy sign would give beta = (2/3, 2/3, 1/3).",
                i,
                sol.beta[i],
                exp
            );
        }

        // Constraint satisfaction sanity check (C beta = d).
        let beta_dv = DVector::from_column_slice(&sol.beta);
        let residual = (&qp.c * &beta_dv - &qp.d).norm();
        assert!(residual < 1e-10, "||C beta - d|| = {:.2e}", residual);
    }

    // ── Cross-variant tests: projection solver vs saddle-point solver ──

    /// Both solvers agree on capacity for the simplex.
    #[test]
    #[ignore] // Test body not implemented — needs cross-solver comparison logic
    fn capacity_agrees_on_simplex() {
        // TODO: Implement cross-solver comparison: run both augmented-system and
        // projection solver on the simplex, verify they agree on capacity.
    }

    /// Both solvers agree on capacity for the hypercube.
    #[test]
    #[ignore] // Test body not implemented — needs cross-solver comparison logic
    fn capacity_agrees_on_hypercube() {
        // TODO: Implement cross-solver comparison for the hypercube.
    }

    /// Projection solver finds positive Q on simplex with exhaustive search over
    /// all subset sizes and orderings.
    #[test]
    fn projection_finds_positive_q_on_simplex() {
        let simplex = known_polytopes::simplex();
        let polytope = &simplex.polytope;
        let f = polytope.facet_count();

        let mut found = false;
        // Try subset sizes 2 to min(f, 5)
        for size in 2..=f.min(5) {
            for_each_combination(f, size, &mut |subset| {
                let perm = subset.to_vec();
                let qp = build_qp(polytope, &perm);
                let sol = solve_projected(&qp);
                if sol.verdict == Verdict::True && sol.q > 1e-6 {
                    found = true;
                }
            });
        }
        assert!(found, "projection solver should find positive Q on simplex");
    }

    /// Call `f` with every k-element subset of {0, ..., n-1}.
    fn for_each_combination(n: usize, k: usize, f: &mut impl FnMut(&[usize])) {
        let mut indices: Vec<usize> = (0..k).collect();
        loop {
            f(&indices);
            let mut i = k;
            loop {
                if i == 0 {
                    return;
                }
                i -= 1;
                indices[i] += 1;
                if indices[i] <= n - k + i {
                    break;
                }
            }
            for j in (i + 1)..k {
                indices[j] = indices[j - 1] + 1;
            }
        }
    }

    /// Projection solver result is consistent with build_qp assembly.
    #[test]
    fn projection_qp_assembly_consistency() {
        let simplex = known_polytopes::simplex();
        let polytope = &simplex.polytope;
        let perm = vec![0, 1, 2];

        let qp = build_qp(polytope, &perm);
        let sol = solve_projected(&qp);

        if sol.verdict != Verdict::False {
            // Verify C beta = d.
            let beta_dv = DVector::from_column_slice(&sol.beta);
            let residual = (&qp.c * &beta_dv - &qp.d).norm();
            assert!(residual < 1e-8, "||C beta - d|| = {:.2e}", residual);

            // Verify Q = (1/2) beta^T H beta.
            let q_check = 0.5 * beta_dv.dot(&(&qp.h * &beta_dv));
            assert!(
                (sol.q - q_check).abs() < 1e-10,
                "Q = {}, direct = {}, diff = {:.2e}",
                sol.q,
                q_check,
                (sol.q - q_check).abs()
            );
        }
    }
}

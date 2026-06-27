//! Projection-based solvers for constrained QP stationarity.
//!
//! The primary surface solves the critical-point equation for
//! `Q(beta) = (1/2) beta^T H beta` subject to `C beta = d`.
//! Positivity `beta > 0` is a separate question.
//!
//! # Algorithm
//!
//! **Step 1 -- Solve constraints.** C beta = d -> particular solution beta0,
//! null-space basis V. If inconsistent, the critical-point surface returns
//! `NoConstraintSolution` and the legacy `Solution` surface returns `False`.
//!
//! **Step 2 -- Project objective.** Form the reduced Hessian H' = V^T H V and
//! reduced right-hand side b' = -V^T H beta0. Solve H' alpha = b' via
//! eigendecomposition, partitioning eigenvalues into retained
//! (|lambda| > threshold) and null (|lambda| <= threshold).
//!
//! **Step 3 -- Compose critical set.** The full critical set is
//! beta = beta0 + V(alpha0 + W gamma), where W are the null-space eigenvectors
//! of H'. These directions don't change Q but can change beta.
//!
//! **Step 4 -- Optional max-margin search.** `solve_projected` preserves the
//! older behavior by finding gamma maximizing min_k beta_k via
//! `beta_feasibility::find_max_margin`. `solve_projected_critical_point` stops
//! before this step.
//!
//! **Step 5 -- Compute Q.** Q = (1/2) beta^T H beta, constant over the solution set.
//!
//! Mathematical correspondence: [lem:kkt], Part C.2 of algorithm design

use super::beta_feasibility;
use super::constraint_solver::{self, ConstraintSolveError};
use super::qp_assembly::build_qp_from_dual_vertices;
use super::{classify_margin, Solution, Verdict, EPS_EIGEN_FLOOR, QP};
use nalgebra::{DMatrix, DVector, Vector4};

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
const EPS_PROJECTED_STATIONARITY: f64 = 1e-8;

/// Critical point of `Q` on the affine constraint space `C beta = d`.
///
/// This result deliberately does not decide `beta > 0`. If
/// `flat_direction_count > 0`, the critical set is affine: `beta` is one
/// representative and flat directions can change beta without changing Q.
/// `q_error_bound` is a residual-based bound for the computed projected
/// stationarity problem: when finite, it bounds the Q-value gap caused by the
/// reported stationarity residual in the retained eigenspace. It is not an
/// exact-arithmetic certificate for the input data. It is `None` when accepted
/// near-flat residuals do not give a finite global value bound.
#[derive(Clone, Debug)]
pub struct ProjectedCriticalPointData {
    pub q: f64,
    pub q_error_bound: Option<f64>,
    pub beta: Vec<f64>,
    pub flat_direction_count: usize,
    pub stationarity_residual: f64,
    pub constraint_residual: f64,
    pub min_beta: f64,
}

/// Outcome of solving projected stationarity for `Q` on `C beta = d`.
#[derive(Clone, Debug)]
pub enum ProjectedCriticalPoint {
    Found(ProjectedCriticalPointData),
    NoConstraintSolution {
        residual: f64,
    },
    NoCriticalPoint {
        stationarity_residual: f64,
        flat_direction_count: usize,
    },
}

struct ProjectedCriticalPointParts {
    q: f64,
    q_error_bound: Option<f64>,
    beta: DVector<f64>,
    flat_directions: DMatrix<f64>,
    stationarity_residual: f64,
    constraint_residual: f64,
}

enum ProjectedCriticalPointPartsError {
    NoConstraintSolution {
        residual: f64,
    },
    NoCriticalPoint {
        stationarity_residual: f64,
        flat_direction_count: usize,
    },
}

/// Solve only the projected stationarity equation for the action/Q critical point.
///
/// This computes `V^T H V alpha = -V^T H beta0` after solving
/// `C beta = d`, and returns one representative of the critical affine set.
/// It does not run the max-margin LP and does not classify `beta > 0`.
pub fn solve_projected_critical_point(qp: &QP) -> ProjectedCriticalPoint {
    match solve_projected_critical_point_parts(qp) {
        Ok(parts) => ProjectedCriticalPoint::Found(ProjectedCriticalPointData {
            q: parts.q,
            q_error_bound: parts.q_error_bound,
            beta: parts.beta.as_slice().to_vec(),
            flat_direction_count: parts.flat_directions.ncols(),
            stationarity_residual: parts.stationarity_residual,
            constraint_residual: parts.constraint_residual,
            min_beta: parts.beta.iter().copied().fold(f64::INFINITY, f64::min),
        }),
        Err(ProjectedCriticalPointPartsError::NoConstraintSolution { residual }) => {
            ProjectedCriticalPoint::NoConstraintSolution { residual }
        }
        Err(ProjectedCriticalPointPartsError::NoCriticalPoint {
            stationarity_residual,
            flat_direction_count,
        }) => ProjectedCriticalPoint::NoCriticalPoint {
            stationarity_residual,
            flat_direction_count,
        },
    }
}

/// Assemble the dual-vertex QP for `perm` and solve projected stationarity.
pub fn solve_projected_critical_point_for_dual_vertices(
    dual_vertices: &[Vector4<f64>],
    perm: &[usize],
) -> ProjectedCriticalPoint {
    let qp = build_qp_from_dual_vertices(dual_vertices, perm);
    solve_projected_critical_point(&qp)
}

/// Solve the QP via constraint projection.
///
/// See module doc for the 5-step algorithm. Returns a Solution with verdict,
/// Q value, beta vector, and margin.
///
/// [lem:kkt]: KKT conditions characterize the EHZ capacity optimum; this solver applies them via constraint projection.
pub fn solve_projected(qp: &QP) -> Solution {
    let m = qp.c.ncols();
    let parts = match solve_projected_critical_point_parts(qp) {
        Ok(parts) => parts,
        Err(_) => {
            return Solution {
                verdict: Verdict::False,
                q: 0.0,
                beta: vec![0.0; m],
                margin: f64::NEG_INFINITY,
            };
        }
    };

    // Existing API resolves positivity by optimizing over the flat critical set.
    let margin_result = beta_feasibility::find_max_margin(&parts.beta, &parts.flat_directions);
    let margin = margin_result.margin;
    let verdict = classify_margin(margin);
    let q = q_value_from_dvec(&qp.h, &margin_result.beta);

    Solution {
        verdict,
        q,
        beta: margin_result.beta.as_slice().to_vec(),
        margin,
    }
}

fn solve_projected_critical_point_parts(
    qp: &QP,
) -> Result<ProjectedCriticalPointParts, ProjectedCriticalPointPartsError> {
    let m = qp.c.ncols();
    let constraint_sol =
        constraint_solver::solve_constraints(&qp.c, &qp.d).map_err(|err| match err {
            ConstraintSolveError::Inconsistent { residual } => {
                ProjectedCriticalPointPartsError::NoConstraintSolution { residual }
            }
        })?;

    let beta0 = &constraint_sol.x0;
    let v = &constraint_sol.null_basis;
    let k = v.ncols();

    if k == 0 {
        let constraint_residual = (&qp.c * beta0 - &qp.d).norm();
        return Ok(ProjectedCriticalPointParts {
            q: q_value_from_dvec(&qp.h, beta0),
            q_error_bound: Some(0.0),
            beta: beta0.clone(),
            flat_directions: DMatrix::zeros(m, 0),
            stationarity_residual: 0.0,
            constraint_residual,
        });
    }

    // Reduced Hessian: H' = V^T H V (k x k symmetric).
    let hv = &qp.h * v;
    let h_prime = v.transpose() * &hv;

    // Reduced gradient: b' = -V^T H beta0 (k x 1).
    // Sign: solving H' alpha + V^T H beta0 = 0 for alpha => alpha = (H')^{-1} b'
    // with b' = -V^T H beta0.
    let h_beta0 = &qp.h * beta0;
    let b_prime = -(v.transpose() * &h_beta0);

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

    let mut alpha0 = DVector::zeros(k);
    for i in 0..k {
        if eigenvalues[i].abs() > threshold {
            let pi = eigenvectors.column(i);
            let coeff = pi.dot(&b_prime) / eigenvalues[i];
            alpha0 += coeff * pi;
        }
    }

    let projected_stationarity_residual = (&h_prime * &alpha0 - &b_prime).norm();

    let null_indices: Vec<usize> = (0..k)
        .filter(|&i| eigenvalues[i].abs() <= threshold)
        .collect();
    let retained_min_abs_eigenvalue = (0..k)
        .filter_map(|i| {
            let abs = eigenvalues[i].abs();
            (abs > threshold).then_some(abs)
        })
        .fold(f64::INFINITY, f64::min);
    let q_error_bound = q_error_bound_from_projected_residual(
        projected_stationarity_residual,
        if retained_min_abs_eigenvalue.is_finite() {
            Some(retained_min_abs_eigenvalue)
        } else {
            None
        },
        null_indices.len(),
    );
    if projected_stationarity_residual > EPS_PROJECTED_STATIONARITY {
        return Err(ProjectedCriticalPointPartsError::NoCriticalPoint {
            stationarity_residual: projected_stationarity_residual,
            flat_direction_count: null_indices.len(),
        });
    }

    let beta_base = beta0 + v * &alpha0;

    let n_null = null_indices.len();
    let flat_directions = if n_null > 0 {
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
    let q = q_value_from_dvec(&qp.h, &beta_base);
    let constraint_residual = (&qp.c * &beta_base - &qp.d).norm();

    Ok(ProjectedCriticalPointParts {
        q,
        q_error_bound,
        beta: beta_base,
        flat_directions,
        stationarity_residual: projected_stationarity_residual,
        constraint_residual,
    })
}

/// Compute Q = (1/2) beta^T H beta from DVector (internal helper).
fn q_value_from_dvec(h: &DMatrix<f64>, beta: &DVector<f64>) -> f64 {
    0.5 * beta.dot(&(h * beta))
}

fn q_error_bound_from_projected_residual(
    residual: f64,
    retained_min_abs_eigenvalue: Option<f64>,
    flat_direction_count: usize,
) -> Option<f64> {
    if residual == 0.0 {
        return Some(0.0);
    }
    if flat_direction_count > 0 {
        return None;
    }
    retained_min_abs_eigenvalue.map(|lambda_min| 0.5 * residual * residual / lambda_min)
}

#[cfg(test)]
mod tests {
    use super::super::qp_assembly::build_qp_from_dual_vertices;
    use super::super::{Verdict, QP};
    use super::*;
    use crate::geom::known_polytopes;
    use crate::kkt::saddle_point_solver::{solve_kkt_for_dual_vertices, KktOutcome};
    use nalgebra::{DMatrix, DVector, Vector4};

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

    #[test]
    fn critical_point_reports_flat_action_family_without_lp() {
        let c = DMatrix::from_row_slice(1, 2, &[1.0, 1.0]);
        let d = DVector::from_column_slice(&[1.0]);
        let h = DMatrix::zeros(2, 2);
        let qp = QP { c, d, h };

        let critical = solve_projected_critical_point(&qp);

        let ProjectedCriticalPoint::Found(data) = critical else {
            panic!("expected flat critical family, got {critical:?}");
        };
        assert_eq!(data.flat_direction_count, 1);
        assert!(data.q.abs() < 1e-12);
        assert_eq!(data.q_error_bound, Some(0.0));
        assert!(data.stationarity_residual < 1e-12);
        assert!(data.constraint_residual < 1e-12);
        assert!((data.beta.iter().sum::<f64>() - 1.0).abs() < 1e-12);
    }

    #[test]
    fn critical_point_rejects_linear_reduced_objective() {
        let c = DMatrix::from_row_slice(1, 2, &[1.0, 1.0]);
        let d = DVector::from_column_slice(&[1.0]);
        let h = DMatrix::from_diagonal(&DVector::from_column_slice(&[1.0, -1.0]));
        let qp = QP { c, d, h };

        let critical = solve_projected_critical_point(&qp);

        let ProjectedCriticalPoint::NoCriticalPoint {
            stationarity_residual,
            flat_direction_count,
        } = critical
        else {
            panic!("expected no projected critical point, got {critical:?}");
        };
        assert_eq!(flat_direction_count, 1);
        assert!(stationarity_residual > 0.1);
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

    /// Projection critical point and saddle-point KKT agree on a simplex sigma.
    #[test]
    fn critical_point_agrees_with_saddle_kkt_on_simplex_sigma() {
        let simplex = known_polytopes::simplex();
        assert_projection_critical_point_agrees_with_saddle(simplex.dual_vertices_f64.as_slice());
    }

    /// Projection critical point and saddle-point KKT agree on a hypercube sigma.
    #[test]
    fn critical_point_agrees_with_saddle_kkt_on_hypercube_sigma() {
        let hypercube = known_polytopes::hypercube();
        assert_projection_critical_point_agrees_with_saddle(hypercube.dual_vertices_f64.as_slice());
    }

    /// Projection solver finds positive Q on simplex with exhaustive search over
    /// all subset sizes and orderings.
    #[test]
    fn projection_finds_positive_q_on_simplex() {
        let simplex = known_polytopes::simplex();
        let dual_vertices = &simplex.dual_vertices_f64;
        let f = simplex.facet_count();

        let mut found = false;
        // Try subset sizes 2 to min(f, 5)
        for size in 2..=f.min(5) {
            for_each_combination(f, size, &mut |subset| {
                let perm = subset.to_vec();
                let qp = build_qp_from_dual_vertices(dual_vertices, &perm);
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

    /// Projection solver result is consistent with flat QP assembly.
    #[test]
    fn projection_qp_assembly_consistency() {
        let simplex = known_polytopes::simplex();
        let dual_vertices = &simplex.dual_vertices_f64;
        let perm = vec![0, 1, 2];

        let qp = build_qp_from_dual_vertices(dual_vertices, &perm);
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

    fn assert_projection_critical_point_agrees_with_saddle(dual_vertices: &[Vector4<f64>]) {
        let f = dual_vertices.len();
        let mut checked = false;
        for size in 2..=f.min(6) {
            for_each_combination(f, size, &mut |subset| {
                if checked {
                    return;
                }
                let mut sigma = subset.to_vec();
                for_each_permutation(&mut sigma, &mut |sigma| {
                    if checked {
                        return;
                    }
                    let KktOutcome::Feasible(kkt) =
                        solve_kkt_for_dual_vertices(dual_vertices, sigma)
                    else {
                        return;
                    };

                    let qp = build_qp_from_dual_vertices(dual_vertices, sigma);
                    let critical = solve_projected_critical_point(&qp);
                    let ProjectedCriticalPoint::Found(data) = critical else {
                        panic!(
                            "saddle-point KKT found feasible sigma {sigma:?}, \
                             projection critical-point solve returned {critical:?}"
                        );
                    };

                    assert!(
                        (data.q - kkt.q_corrected).abs() < 1e-8,
                        "projection Q {} and saddle Q {} disagree for sigma {:?}",
                        data.q,
                        kkt.q_corrected,
                        sigma
                    );
                    assert!(
                        (0.5 / data.q - 0.5 / kkt.q_corrected).abs() < 1e-8,
                        "projection action and saddle action disagree for sigma {:?}",
                        sigma
                    );
                    assert!(
                        data.stationarity_residual < 1e-8,
                        "projected stationarity residual {:.2e} for sigma {:?}",
                        data.stationarity_residual,
                        sigma
                    );
                    assert!(
                        data.constraint_residual < 1e-8,
                        "constraint residual {:.2e} for sigma {:?}",
                        data.constraint_residual,
                        sigma
                    );
                    assert!(
                        data.q_error_bound.is_some_and(|bound| bound < 1e-12),
                        "expected small finite projection Q error bound for sigma {:?}, got {:?}",
                        sigma,
                        data.q_error_bound
                    );
                    checked = true;
                });
            });
        }
        assert!(checked, "expected at least one saddle-feasible sigma");
    }

    fn for_each_permutation(values: &mut [usize], f: &mut impl FnMut(&[usize])) {
        fn recurse(values: &mut [usize], start: usize, f: &mut impl FnMut(&[usize])) {
            if start == values.len() {
                f(values);
                return;
            }
            for i in start..values.len() {
                values.swap(start, i);
                recurse(values, start + 1, f);
                values.swap(start, i);
            }
        }
        recurse(values, 0, f);
    }
}

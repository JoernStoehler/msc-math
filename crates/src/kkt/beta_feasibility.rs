//! Max-margin LP search for beta > 0 in an affine solution set.
//!
//! Given base point beta0 in R^m and direction matrix V in R^{m x k} with
//! orthonormal columns, solves the Chebyshev center problem:
//!
//!   max_alpha  min_j  (beta0 + V * alpha)_j
//!
//! This finds the point in the affine subspace {beta0 + V * alpha : alpha in R^k}
//! with maximum clearance from all positivity constraints beta_j >= 0.
//!
//! - k = 0: trivial (no degrees of freedom).
//! - k >= 1: LP via clarabel interior-point solver (through `good_lp`).
//!
//! Used by `projection_solver` (Step 4) to classify the verdict for a KKT node.
//!
//! Mathematical correspondence: [lem:numerical-transition-feasibility]

use good_lp::{constraint, default_solver, variable, variables, Expression, Solution, SolverModel};
use nalgebra::{DMatrix, DVector};

/// Result of the max-margin feasibility search.
///
/// Always returned (never None). The margin classifies feasibility:
/// - margin > +eps -> certified feasible (all beta_j > 0)
/// - margin < -eps -> certified infeasible (no beta > 0 in the subspace)
/// - |margin| <= eps -> ambiguous (Indeterminate)
#[derive(Clone, Debug)]
pub struct MarginResult {
    /// The maximum margin: max_alpha min_j (beta0 + V * alpha)_j.
    /// Positive -> feasible, negative -> infeasible, near-zero -> ambiguous.
    pub margin: f64,
    /// The optimal alpha achieving the margin (in null-space coordinates).
    /// Length k (= null_basis.ncols()). Empty (length 0) when k = 0.
    pub alpha: DVector<f64>,
    /// The solution point beta = beta0 + V * alpha.
    pub beta: DVector<f64>,
}

/// Find the point in the affine subspace {beta0 + V * alpha} with maximum minimum component.
///
/// This is the Chebyshev center problem for the polytope {alpha : beta0 + V * alpha >= 0}.
///
/// # Cases by null-space dimension k
///
/// - **k = 0**: No degrees of freedom. margin = min(beta0).
/// - **k >= 1**: LP solver (GLPK via `good_lp`). Certified optimal margin.
///
/// # Guarantees
///
/// - Always returns a result (never panics for valid inputs).
/// - The returned margin equals min(beta) exactly.
/// - For all k, the margin is the certified global optimum.
pub fn find_max_margin(beta0: &DVector<f64>, null_basis: &DMatrix<f64>) -> MarginResult {
    let k = null_basis.ncols();

    match k {
        0 => find_max_margin_k0(beta0),
        _ => find_max_margin_kn(beta0, null_basis),
    }
}

/// Convenience wrapper: find feasible beta from a QP's constraint solution.
///
/// Given a constraint solution (x0, null_basis), finds the point with
/// maximum margin and returns it if margin > 0.
///
/// This is the entry point used by the projection solver path.
pub fn find_feasible_beta(
    beta0: &DVector<f64>,
    null_basis: &DMatrix<f64>,
) -> Option<DVector<f64>> {
    let result = find_max_margin(beta0, null_basis);
    if result.margin > 0.0 {
        Some(result.beta)
    } else {
        None
    }
}

/// k = 0: No degrees of freedom. Margin is simply min(beta0).
fn find_max_margin_k0(beta0: &DVector<f64>) -> MarginResult {
    let m = beta0.len();
    let margin = if m == 0 {
        0.0
    } else {
        beta0.iter().copied().fold(f64::INFINITY, f64::min)
    };

    MarginResult {
        margin,
        alpha: DVector::zeros(0),
        beta: beta0.clone(),
    }
}

/// k >= 1: LP solution via clarabel (through `good_lp`).
///
/// Reformulates the Chebyshev center problem as:
///
///   max t
///   s.t.  sum_i V[j,i] * alpha_i - t >= -beta0[j]   for each j = 1..m
///
/// Variables: alpha_1..alpha_k (unbounded) and t (unbounded).
/// Total k+1 variables, m constraints.
///
/// GLPK's simplex implementation uses Bland's anti-cycling rule,
/// so it terminates on degenerate inputs where microlp would cycle.
fn find_max_margin_kn(beta0: &DVector<f64>, null_basis: &DMatrix<f64>) -> MarginResult {
    let m = beta0.len();
    let k = null_basis.ncols();

    let mut vars = variables!();

    // Variables alpha_1..alpha_k: unbounded.
    let alpha_vars: Vec<_> = (0..k)
        .map(|_| vars.add(variable().min(f64::NEG_INFINITY)))
        .collect();

    // Variable t: unbounded (objective: maximize t).
    let t_var = vars.add(variable().min(f64::NEG_INFINITY));

    let objective: Expression = t_var.into();

    let mut model = vars.maximise(objective).using(default_solver);

    // Constraints: for each j, sum_i V[j,i] * alpha_i - t >= -beta0[j]
    for j in 0..m {
        let mut lhs: Expression = Expression::from(0.0);
        for i in 0..k {
            let coeff = null_basis[(j, i)];
            if coeff != 0.0 {
                lhs += coeff * alpha_vars[i];
            }
        }
        lhs -= t_var;
        model = model.with(constraint!(lhs >= -beta0[j]));
    }

    match model.solve() {
        Ok(solution) => {
            let alpha = DVector::from_fn(k, |i, _| solution.value(alpha_vars[i]));
            let beta = beta0 + null_basis * &alpha;
            let margin = beta.iter().copied().fold(f64::INFINITY, f64::min);
            MarginResult {
                margin,
                alpha,
                beta,
            }
        }
        Err(_) => {
            // Solver error. Shouldn't happen for our formulation, but handle gracefully.
            let beta = beta0.clone();
            let margin = beta.iter().copied().fold(f64::INFINITY, f64::min);
            MarginResult {
                margin,
                alpha: DVector::zeros(k),
                beta,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra::{DMatrix, DVector};

    // Tests for beta_feasibility: max-margin LP search for beta > 0.
    //
    // Proposition: find_max_margin returns the certified global optimum of
    // max_alpha min_j (beta0 + V * alpha)_j, with margin = min(beta) exactly.
    // Reference: Chebyshev center LP formulation.
    //
    // Strategy: fixture-based covering k=0 (trivial), k=1 (analytic), k>=2 (LP).

    // ── Helpers ──

    /// Verify the critical invariant: margin = min(beta).
    fn assert_margin_is_tight(result: &MarginResult) {
        let min_beta = result
            .beta
            .iter()
            .copied()
            .fold(f64::INFINITY, f64::min);
        assert!(
            (result.margin - min_beta).abs() < 1e-12,
            "margin ({:.2e}) != min(beta) ({:.2e}), diff = {:.2e}",
            result.margin,
            min_beta,
            (result.margin - min_beta).abs()
        );
    }

    /// Verify beta = beta0 + V * alpha reconstruction.
    fn assert_beta_reconstruction(beta0: &DVector<f64>, null_basis: &DMatrix<f64>, result: &MarginResult) {
        let expected = beta0 + null_basis * &result.alpha;
        let diff = (&result.beta - &expected).norm();
        assert!(
            diff < 1e-12,
            "beta reconstruction error: ||beta - (beta0 + V * alpha)|| = {:.2e}",
            diff
        );
    }

    // ── k = 0: trivial cases ──

    #[test]
    fn trivial_feasible() {
        let beta0 = DVector::from_vec(vec![1.0, 1.0, 1.0]);
        let v = DMatrix::zeros(3, 0);
        let result = find_max_margin(&beta0, &v);

        assert!((result.margin - 1.0).abs() < 1e-14);
        assert_eq!(result.alpha.len(), 0);
        assert_margin_is_tight(&result);
        assert_beta_reconstruction(&beta0, &v, &result);
    }

    #[test]
    fn trivial_infeasible() {
        let beta0 = DVector::from_vec(vec![-1.0, 1.0, 1.0]);
        let v = DMatrix::zeros(3, 0);
        let result = find_max_margin(&beta0, &v);

        assert!((result.margin - (-1.0)).abs() < 1e-14);
        assert_margin_is_tight(&result);
    }

    #[test]
    fn trivial_single_component() {
        let beta0 = DVector::from_vec(vec![3.5]);
        let v = DMatrix::zeros(1, 0);
        let result = find_max_margin(&beta0, &v);

        assert!((result.margin - 3.5).abs() < 1e-14);
        assert_margin_is_tight(&result);
    }

    // ── k = 1: analytic cases ──

    #[test]
    fn one_dim_feasible() {
        let beta0 = DVector::from_vec(vec![-1.0, 2.0]);
        let v = DMatrix::from_vec(2, 1, vec![1.0, 0.0]);
        let result = find_max_margin(&beta0, &v);

        assert!(result.margin > 0.0, "expected feasible, got margin = {}", result.margin);
        assert_margin_is_tight(&result);
        assert_beta_reconstruction(&beta0, &v, &result);
    }

    #[test]
    fn one_dim_infeasible() {
        let beta0 = DVector::from_vec(vec![-1.0, -2.0]);
        let v = DMatrix::from_vec(2, 1, vec![1.0, -1.0]);
        let result = find_max_margin(&beta0, &v);

        assert!(result.margin < 0.0, "expected infeasible, got margin = {}", result.margin);
        assert_margin_is_tight(&result);
        assert_beta_reconstruction(&beta0, &v, &result);
    }

    #[test]
    fn one_dim_midpoint() {
        let beta0 = DVector::from_vec(vec![0.0, 0.0]);
        let v = DMatrix::from_vec(2, 1, vec![1.0, -1.0]);
        let result = find_max_margin(&beta0, &v);

        assert!(result.margin.abs() < 1e-14, "expected margin ~ 0, got {}", result.margin);
        assert_margin_is_tight(&result);
        assert_beta_reconstruction(&beta0, &v, &result);
    }

    #[test]
    fn one_dim_bounded_interval() {
        let beta0 = DVector::from_vec(vec![2.0, 4.0]);
        let v = DMatrix::from_vec(2, 1, vec![1.0, -1.0]);
        let result = find_max_margin(&beta0, &v);

        assert!((result.margin - 3.0).abs() < 1e-12, "expected margin = 3, got {}", result.margin);
        assert!((result.alpha[0] - 1.0).abs() < 1e-12, "expected alpha = 1, got {}", result.alpha[0]);
        assert_margin_is_tight(&result);
        assert_beta_reconstruction(&beta0, &v, &result);
    }

    #[test]
    fn one_dim_all_directions_zero() {
        let beta0 = DVector::from_vec(vec![1.0, 2.0, 3.0]);
        let v = DMatrix::from_vec(3, 1, vec![1e-16, 0.0, 1e-17]);
        let result = find_max_margin(&beta0, &v);

        assert!((result.margin - 1.0).abs() < 1e-10, "expected margin ~ 1, got {}", result.margin);
        assert_margin_is_tight(&result);
    }

    // ── k >= 2: multi-dimensional cases ──

    #[test]
    fn two_dim_feasible() {
        let beta0 = DVector::from_vec(vec![-1.0, -1.0, 3.0]);
        #[rustfmt::skip]
        let v = DMatrix::from_row_slice(3, 2, &[
            1.0, 0.0,
            0.0, 1.0,
            0.0, 0.0,
        ]);
        let result = find_max_margin(&beta0, &v);

        assert!(result.margin > 0.0, "expected feasible, got margin = {}", result.margin);
        for j in 0..3 {
            assert!(result.beta[j] > 0.0, "beta[{}] = {} should be > 0", j, result.beta[j]);
        }
        assert_margin_is_tight(&result);
        assert_beta_reconstruction(&beta0, &v, &result);
    }

    #[test]
    fn two_dim_infeasible() {
        // sum(beta) = -15 constant, so at least one component <= -5.
        let beta0 = DVector::from_vec(vec![-5.0, -5.0, -5.0]);
        #[rustfmt::skip]
        let v = DMatrix::from_row_slice(3, 2, &[
            1.0,  0.0,
            0.0,  1.0,
           -1.0, -1.0,
        ]);
        let result = find_max_margin(&beta0, &v);

        assert!(result.margin < 0.0, "expected infeasible, got margin = {}", result.margin);
        assert_margin_is_tight(&result);
        assert_beta_reconstruction(&beta0, &v, &result);
    }

    #[test]
    fn two_dim_symmetric() {
        // beta = [alpha1, alpha2, -alpha1, 6 - alpha2].
        // Optimal: alpha1 = 0, alpha2 = 3 -> beta = [0, 3, 0, 3], margin = 0.
        let beta0 = DVector::from_vec(vec![0.0, 0.0, 0.0, 6.0]);
        #[rustfmt::skip]
        let v = DMatrix::from_row_slice(4, 2, &[
             1.0,  0.0,
             0.0,  1.0,
            -1.0,  0.0,
             0.0, -1.0,
        ]);
        let result = find_max_margin(&beta0, &v);

        assert!(result.margin.abs() < 0.1, "expected margin ~ 0, got {}", result.margin);
        assert_margin_is_tight(&result);
        assert_beta_reconstruction(&beta0, &v, &result);
    }

    // ── Cross-cutting properties ──

    #[test]
    fn margin_is_tight_k0() {
        let beta0 = DVector::from_vec(vec![3.0, 1.5, 7.2, 0.1]);
        let v = DMatrix::zeros(4, 0);
        let result = find_max_margin(&beta0, &v);
        assert_margin_is_tight(&result);
    }

    #[test]
    fn margin_is_tight_k1() {
        let beta0 = DVector::from_vec(vec![1.0, 3.0, 0.5, 2.0]);
        let v = DMatrix::from_vec(4, 1, vec![0.5, -0.3, 0.8, -0.1]);
        let result = find_max_margin(&beta0, &v);
        assert_margin_is_tight(&result);
        assert_beta_reconstruction(&beta0, &v, &result);
    }

    #[test]
    fn margin_is_tight_k2() {
        let beta0 = DVector::from_vec(vec![1.0, -0.5, 2.0, 0.1, -0.3]);
        #[rustfmt::skip]
        let v = DMatrix::from_row_slice(5, 2, &[
            0.3,  0.1,
           -0.2,  0.5,
            0.1, -0.3,
            0.4,  0.2,
           -0.1,  0.6,
        ]);
        let result = find_max_margin(&beta0, &v);
        assert_margin_is_tight(&result);
        assert_beta_reconstruction(&beta0, &v, &result);
    }

    #[test]
    fn null_basis_empty_equals_k0() {
        let beta0 = DVector::from_vec(vec![2.0, 0.5, 1.0]);
        let v = DMatrix::zeros(3, 0);
        let result = find_max_margin(&beta0, &v);

        assert!((result.margin - 0.5).abs() < 1e-14);
        assert_eq!(result.alpha.len(), 0);
        assert_margin_is_tight(&result);
    }

    #[test]
    fn k1_optimal_is_midpoint() {
        // lo = -1, hi = 5, midpoint = 2. beta = [3, 3], margin = 3.
        let beta0 = DVector::from_vec(vec![1.0, 5.0]);
        let v = DMatrix::from_vec(2, 1, vec![1.0, -1.0]);
        let result = find_max_margin(&beta0, &v);

        assert!((result.margin - 3.0).abs() < 1e-12);
        assert!((result.alpha[0] - 2.0).abs() < 1e-12);
    }

    #[test]
    fn larger_k3_feasible() {
        let beta0 = DVector::from_vec(vec![-1.0, -2.0, -1.0, 10.0, 10.0, 10.0]);
        #[rustfmt::skip]
        let v = DMatrix::from_row_slice(6, 3, &[
            1.0, 0.0, 0.0,
            0.0, 1.0, 0.0,
            0.0, 0.0, 1.0,
            0.0, 0.0, 0.0,
            0.0, 0.0, 0.0,
            0.0, 0.0, 0.0,
        ]);
        let result = find_max_margin(&beta0, &v);

        assert!(result.margin > 0.0, "expected feasible, got margin = {}", result.margin);
        assert_margin_is_tight(&result);
        assert_beta_reconstruction(&beta0, &v, &result);
    }

    #[test]
    fn k1_half_bounded_lower_only() {
        let beta0 = DVector::from_vec(vec![2.0, 0.0]);
        let v = DMatrix::from_vec(2, 1, vec![0.0, 1.0]);
        let result = find_max_margin(&beta0, &v);

        assert!(result.margin > 0.0, "expected positive margin, got {}", result.margin);
        assert_margin_is_tight(&result);
        assert_beta_reconstruction(&beta0, &v, &result);
    }

    // ── Convenience wrapper tests ──

    #[test]
    fn find_feasible_beta_returns_some_when_feasible() {
        let beta0 = DVector::from_vec(vec![-1.0, -1.0, 3.0]);
        #[rustfmt::skip]
        let v = DMatrix::from_row_slice(3, 2, &[
            1.0, 0.0,
            0.0, 1.0,
            0.0, 0.0,
        ]);
        let result = find_feasible_beta(&beta0, &v);
        assert!(result.is_some(), "should find feasible beta");
        let beta = result.unwrap();
        assert!(beta.iter().all(|&b| b > 0.0), "all components should be positive");
    }

    #[test]
    fn find_feasible_beta_returns_none_when_infeasible() {
        let beta0 = DVector::from_vec(vec![-5.0, -5.0, -5.0]);
        #[rustfmt::skip]
        let v = DMatrix::from_row_slice(3, 2, &[
            1.0,  0.0,
            0.0,  1.0,
           -1.0, -1.0,
        ]);
        let result = find_feasible_beta(&beta0, &v);
        assert!(result.is_none(), "should not find feasible beta");
    }
}

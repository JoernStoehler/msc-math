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
//! - k = 1: analytic solution via interval analysis.
//! - k >= 2: exact LP via `microlp`.
//!
//! Used by `projection_solver` (Step 4) to classify the verdict for a KKT node.
//!
//! Mathematical correspondence: [lem:numerical-transition-feasibility]

use microlp::{ComparisonOp, OptimizationDirection, Problem};
use nalgebra::{DMatrix, DVector};

/// Component of null-space vector below this magnitude is treated as zero (k=1 case).
///
/// Tighter than EPS_BETA_POSITIVE (1e-12) because V has orthonormal columns --
/// components below 1e-15 are numerical zeros from the SVD/eigensolver.
///
/// **Why 1e-15:** The null-space basis V has unit-length columns (||v|| = 1).
/// For a unit vector with m components, machine epsilon contributes ~sqrt(m) * 1e-16
/// ~ 4e-16 to each component's noise floor. 1e-15 is 2.5x above this floor.
/// This is tighter than EPS_BETA_POSITIVE (1e-12): a v-component at 1e-13 is not
/// machine noise for a unit eigenvector -- it's a small but real direction.
/// Only true zero components (1e-15 magnitude) should be skipped in interval analysis.
/// Making it 10x larger (1e-14) risks skipping real small components in the k=1
/// analytic solver. Making it 10x smaller (1e-16) risks treating machine-epsilon
/// noise as a real bound.
const EPS_DIRECTION_ZERO: f64 = 1e-15;

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
/// - **k = 1**: Analytic solution via interval analysis.
/// - **k >= 2**: LP solver (`microlp`). Certified optimal margin.
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
        1 => find_max_margin_k1(beta0, &null_basis.column(0).into_owned()),
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

/// k = 1: Analytic solution via interval analysis.
///
/// For each component j, the constraint (beta0 + v * alpha)_j >= 0 gives:
/// - v_j > 0: alpha >= -beta0_j / v_j  (lower bound)
/// - v_j < 0: alpha <= -beta0_j / v_j  (upper bound)
/// - |v_j| ~ 0: no constraint on alpha (beta0_j is fixed)
///
/// The feasible interval is [lo, hi]. The midpoint maximizes the margin.
fn find_max_margin_k1(beta0: &DVector<f64>, v: &DVector<f64>) -> MarginResult {
    let m = beta0.len();
    let mut lo = f64::NEG_INFINITY;
    let mut hi = f64::INFINITY;

    for j in 0..m {
        if v[j].abs() < EPS_DIRECTION_ZERO {
            continue;
        }
        let bound = -beta0[j] / v[j];
        if v[j] > 0.0 {
            lo = lo.max(bound);
        } else {
            hi = hi.min(bound);
        }
    }

    let alpha_scalar = if lo.is_finite() && hi.is_finite() {
        (lo + hi) / 2.0
    } else if lo.is_finite() {
        lo + 1.0
    } else if hi.is_finite() {
        hi - 1.0
    } else {
        0.0
    };

    let alpha = DVector::from_element(1, alpha_scalar);
    let beta = beta0 + v * alpha_scalar;
    let margin = beta.iter().copied().fold(f64::INFINITY, f64::min);

    MarginResult {
        margin,
        alpha,
        beta,
    }
}

/// k >= 2: Exact LP solution via `microlp`.
///
/// Reformulates the Chebyshev center problem as:
///
///   max t
///   s.t.  sum_i V[j,i] * alpha_i - t >= -beta0[j]   for each j = 1..m
///
/// Variables: alpha_1..alpha_k (unbounded) and t (unbounded).
/// Total k+1 variables, m constraints.
///
/// The LP is always feasible (t can go to -infinity) and bounded
/// (the affine subspace is finite-dimensional).
fn find_max_margin_kn(beta0: &DVector<f64>, null_basis: &DMatrix<f64>) -> MarginResult {
    let m = beta0.len();
    let k = null_basis.ncols();

    let mut problem = Problem::new(OptimizationDirection::Maximize);

    // Variables alpha_1..alpha_k: unbounded, zero objective coefficient.
    let alpha_vars: Vec<_> = (0..k)
        .map(|_| problem.add_var(0.0, (f64::NEG_INFINITY, f64::INFINITY)))
        .collect();

    // Variable t: unbounded, objective coefficient 1.0.
    let t_var = problem.add_var(1.0, (f64::NEG_INFINITY, f64::INFINITY));

    // Constraints: for each j, sum_i V[j,i] * alpha_i - t >= -beta0[j]
    for j in 0..m {
        let mut terms: Vec<(microlp::Variable, f64)> = Vec::with_capacity(k + 1);
        for i in 0..k {
            let coeff = null_basis[(j, i)];
            if coeff != 0.0 {
                terms.push((alpha_vars[i], coeff));
            }
        }
        terms.push((t_var, -1.0));
        problem.add_constraint(terms.as_slice(), ComparisonOp::Ge, -beta0[j]);
    }

    match problem.solve() {
        Ok(solution) => {
            let alpha = DVector::from_fn(k, |i, _| solution[alpha_vars[i]]);
            let beta = beta0 + null_basis * &alpha;
            let margin = beta.iter().copied().fold(f64::INFINITY, f64::min);
            MarginResult {
                margin,
                alpha,
                beta,
            }
        }
        Err(_) => {
            // Solver error (Unbounded, Infeasible, or InternalError).
            // These shouldn't happen for our formulation, but handle gracefully.
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

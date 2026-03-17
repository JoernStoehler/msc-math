//! Solve linear constraint systems Cx = d via SVD.
//!
//! Given C in R^{p x m} and d in R^p, finds the affine solution set { x : Cx = d }
//! decomposed as x = x0 + V alpha, where x0 is the minimum-norm particular solution
//! and V has orthonormal columns spanning ker(C).
//!
//! This is a context-free linear algebra sub-component: it knows nothing about
//! symplectic geometry, KKT conditions, or polytopes. The constraint solution
//! feeds into both the projection solver (Step 1) and beta feasibility search.
//!
//! Mathematical correspondence: Step 1 of the projection solver, Part C.2 of the
//! algorithm design.

use nalgebra::{DMatrix, DVector};

/// Relative threshold for SVD rank detection: sigma_i < sigma_max * tau is treated as zero.
///
/// At 1e-10 this is well above machine epsilon (~1e-16) and catches
/// near-rank-deficiency without discarding meaningful singular values.
const EPS_RANK_THRESHOLD: f64 = 1e-10;

/// Maximum residual ||Cx0 - d|| to accept as consistent.
///
/// If the residual exceeds this after projecting d onto the column space,
/// the system Cx = d has no solution.
///
/// **Why 1e-8:** The SVD roundoff noise on our O(1)-scale matrices is ~1e-14
/// to ~1e-13. The tolerance 1e-8 is:
/// - Far above SVD noise: no false negatives (genuine solutions incorrectly
///   rejected) on well-conditioned systems.
/// - Well below scales where a genuine inconsistency would be masked: a true
///   inconsistency (e.g. wrong closure condition) produces residuals O(0.1)--O(1).
/// Making it 10x larger (1e-7) would mask near-inconsistent systems where the
/// closure constraint is violated by a small numerical perturbation. Making it
/// 10x smaller (1e-9) risks false negatives on moderately ill-conditioned
/// constraint matrices.
const EPS_CONSISTENCY: f64 = 1e-8;

/// Solution of the linear constraint system Cx = d.
///
/// The full solution set is the affine subspace { x0 + V alpha : alpha in R^k },
/// where k = m - rank(C).
#[derive(Clone, Debug)]
pub struct ConstraintSolution {
    /// Particular solution x0 (minimum-norm via SVD pseudoinverse).
    pub x0: DVector<f64>,
    /// Orthonormal numerical null-space basis V in R^{m x k}. Columns approximately span ker(C).
    /// k = m - rank. Empty (m x 0 matrix) when C has full column rank.
    pub null_basis: DMatrix<f64>,
    /// Numerical rank of C (number of singular values above threshold).
    pub rank: usize,
}

/// Solve Cx = d via SVD with threshold rank detection.
///
/// Returns `None` if the system is inconsistent (d not in the column space of C).
///
/// # Algorithm
///
/// 1. Compute SVD: C = U Sigma V^T
/// 2. Rank detection: r = |{ i : sigma_i > sigma_max * EPS_RANK_THRESHOLD }|
/// 3. Consistency: ||(I - U_r U_r^T) d|| < EPS_CONSISTENCY
/// 4. Particular solution: x0 = V_r Sigma_r^{-1} U_r^T d (minimum-norm)
/// 5. Null basis: columns of V corresponding to zero singular values
///
/// # Panics
///
/// Panics if `c.nrows() != d.nrows()` (dimension mismatch).
pub fn solve_constraints(
    c: &DMatrix<f64>,
    d: &DVector<f64>,
) -> Option<ConstraintSolution> {
    let p = c.nrows();
    let m = c.ncols();
    assert_eq!(p, d.nrows(), "C has {} rows but d has {} rows", p, d.nrows());

    // Edge case: zero-row constraint matrix (no constraints).
    if p == 0 {
        return Some(ConstraintSolution {
            x0: DVector::zeros(m),
            null_basis: DMatrix::identity(m, m),
            rank: 0,
        });
    }

    // Edge case: zero-column system. Consistent only if d = 0.
    if m == 0 {
        if d.norm() < EPS_CONSISTENCY {
            return Some(ConstraintSolution {
                x0: DVector::zeros(0),
                null_basis: DMatrix::zeros(0, 0),
                rank: 0,
            });
        } else {
            return None;
        }
    }

    // Step 1: Compute SVD of C.
    //
    // nalgebra's thin SVD of a p x m matrix gives V^T with min(p,m) rows.
    // When p < m (underdetermined), this is insufficient to extract the full
    // null space (dimension m - rank). Pad C with zero rows to m x m so that
    // V^T has the full m rows. The zero rows don't change rank, column space,
    // or null space.
    let c_for_svd = if p < m {
        let mut padded = DMatrix::zeros(m, m);
        for i in 0..p {
            for j in 0..m {
                padded[(i, j)] = c[(i, j)];
            }
        }
        padded
    } else {
        c.clone()
    };

    let svd = c_for_svd.svd(true, true);
    let sigma = &svd.singular_values;
    let u = svd.u.as_ref().expect("SVD computed with u=true");
    let vt = svd.v_t.as_ref().expect("SVD computed with v_t=true");

    // Step 2: Rank detection.
    let sigma_max = sigma.iter().cloned().fold(0.0_f64, f64::max);
    let threshold = sigma_max * EPS_RANK_THRESHOLD;
    let rank = sigma.iter().filter(|&&s| s > threshold).count();

    // Step 3: Consistency check.
    // d_perp = d - U_r U_r^T d. When C was padded, U has more rows than d,
    // so use only the first p rows of each left singular vector.
    let mut d_proj = DVector::zeros(p);
    for i in 0..rank {
        let ui_full = u.column(i);
        let coeff: f64 = (0..p).map(|k| ui_full[k] * d[k]).sum();
        d_proj += coeff * DVector::from_fn(p, |k, _| ui_full[k]);
    }
    let residual = (d - &d_proj).norm();
    if residual > EPS_CONSISTENCY {
        return None;
    }

    // Step 4: Particular solution (minimum-norm via pseudoinverse).
    // x0 = sum_i (u_i^T d / sigma_i) v_i for retained singular values.
    let mut x0 = DVector::zeros(m);
    for i in 0..rank {
        let ui_full = u.column(i);
        let vi = vt.row(i).transpose();
        let coeff: f64 = (0..p).map(|k| ui_full[k] * d[k]).sum();
        x0 += (coeff / sigma[i]) * &vi;
    }

    // Step 5: Null basis -- columns of V for zero singular values.
    let null_dim = m - rank;
    let null_basis = if null_dim > 0 {
        let mut basis = DMatrix::zeros(m, null_dim);
        for j in 0..null_dim {
            let row = vt.row(rank + j);
            for i in 0..m {
                basis[(i, j)] = row[i];
            }
        }
        basis
    } else {
        DMatrix::zeros(m, 0)
    };

    Some(ConstraintSolution {
        x0,
        null_basis,
        rank,
    })
}

/// Exact KKT solver over BigRational (rational arithmetic).
///
/// Solves the same constrained optimization as [`crate::kkt`] — max Q(β) subject to
/// N^T β = 0, η^T β = 1 — but using exact arithmetic over Q instead of f64.
///
/// # Purpose
///
/// The f64 solver in [`crate::kkt`] uses eigendecomposition with residual correction
/// and error bounds. This exact solver serves as the ground truth:
///
/// - **Validation**: confirm that the f64 solver's error bound is valid
///   (|Q̃ - Q_exact| ≤ E) on specific (S,σ) nodes.
/// - **Exact capacity**: when needed, compute the exact rational capacity value
///   without any floating-point error.
///
/// # Performance
///
/// Gaussian elimination over BigRational is ~100x slower than f64 eigendecomposition
/// for typical (S,σ) sizes (m ≈ 2–16). This solver is intended for single (S,σ)
/// lookups (e.g. the winning node), not for sweeping all nodes.
///
/// # Mathematical correspondence
///
/// Uses the same symmetric KKT matrix as `[lem:kkt]` (thesis):
/// ```text
/// [ H   |  N   |  η ] [ β ]   [ 0 ]
/// [ N^T |  0   |  0 ] [ μ ] = [ 0 ]
/// [ η^T |  0   |  0 ] [ ξ ]   [ 1 ]
/// ```
/// All entries are exact rationals obtained from f64 via lossless conversion
/// ([`f64_to_rational`](crate::geom::rational::f64_to_rational)).
use nalgebra::Vector4;
use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{ToPrimitive, Zero};

use crate::geom::rational::f64_to_rational;
use crate::geom::symplectic::omega0;

/// Result of an exact KKT solve over BigRational.
#[derive(Clone, Debug)]
pub struct ExactKktResult {
    /// Exact β vector (all components rational).
    pub beta: Vec<BigRational>,
    /// Exact Q(β) = Σ_{i>j} β_i β_j ω₀(n_{σ(j)}, n_{σ(i)}) over Q.
    pub q_exact: BigRational,
    /// Q_exact converted to f64 (for convenient comparison with f64 solver).
    pub q_exact_f64: f64,
}

/// Solve the KKT system exactly for a single (S,σ) combinatorics.
///
/// Given facet normals, heights, and a specific permutation `perm` (the σ in the
/// thesis), builds the KKT matrix over Q and solves via Gaussian elimination.
///
/// Returns `None` if the system is singular (no unique solution).
///
/// # Arguments
///
/// - `normals`: f64 unit normal vectors (reinterpreted as exact rationals)
/// - `heights`: f64 positive heights (reinterpreted as exact rationals)
/// - `perm`: facet index sequence defining the (S,σ) node
pub fn solve_kkt_exact(
    normals: &[Vector4<f64>],
    heights: &[f64],
    perm: &[usize],
) -> Option<ExactKktResult> {
    let (mat, rhs) = build_kkt_rational(normals, heights, perm);
    let x = gauss_solve(&mat, &rhs)?;

    let m = perm.len();
    let beta: Vec<BigRational> = x[..m].to_vec();
    let q_exact = q_from_beta_rational(normals, perm, &beta);
    let q_exact_f64 = rational_to_f64(&q_exact);

    Some(ExactKktResult {
        beta,
        q_exact,
        q_exact_f64,
    })
}

/// Build the KKT matrix and RHS over Q (exact rational arithmetic).
///
/// Mirrors [`crate::kkt::build_kkt_system`] exactly, but all entries are BigRational.
/// The f64 values (normals, heights, ω₀) are converted losslessly via
/// [`f64_to_rational`].
fn build_kkt_rational(
    normals: &[Vector4<f64>],
    heights: &[f64],
    perm: &[usize],
) -> (Vec<Vec<BigRational>>, Vec<BigRational>) {
    let m = perm.len();
    let size = m + 5;
    let zero = BigRational::zero();

    let mut mat = vec![vec![zero.clone(); size]; size];
    let mut rhs = vec![zero.clone(); size];

    // H block: H_{ij} = ω₀(n_i, n_j) for i ≠ j (symmetric)
    for i in 0..m {
        for j in (i + 1)..m {
            let val = f64_to_rational(omega0(&normals[perm[i]], &normals[perm[j]]));
            mat[i][j] = val.clone();
            mat[j][i] = val;
        }
    }

    // N block: N_{i,d} = normals[perm[i]][d]
    for i in 0..m {
        for d in 0..4 {
            let val = f64_to_rational(normals[perm[i]][d]);
            mat[i][m + d] = val.clone();
            mat[m + d][i] = val;
        }
    }

    // η block: η_i = heights[perm[i]]
    for i in 0..m {
        let val = f64_to_rational(heights[perm[i]]);
        mat[i][m + 4] = val.clone();
        mat[m + 4][i] = val;
    }

    // RHS: [0, ..., 0, 1]
    rhs[m + 4] = BigRational::from(BigInt::from(1));

    (mat, rhs)
}

/// Gaussian elimination with partial pivoting over BigRational.
///
/// Solves Ax = b exactly. Returns `None` if the system is singular.
fn gauss_solve(mat: &[Vec<BigRational>], rhs: &[BigRational]) -> Option<Vec<BigRational>> {
    let n = rhs.len();
    // Augmented matrix [A | b]
    let mut aug: Vec<Vec<BigRational>> = mat
        .iter()
        .enumerate()
        .map(|(i, row)| {
            let mut r = row.clone();
            r.push(rhs[i].clone());
            r
        })
        .collect();

    // Forward elimination
    for col in 0..n {
        // Find pivot (first nonzero in column)
        let pivot_row = (col..n).find(|&r| !aug[r][col].is_zero())?;
        aug.swap(col, pivot_row);

        let pivot = aug[col][col].clone();
        for row in (col + 1)..n {
            if !aug[row][col].is_zero() {
                let factor = &aug[row][col] / &pivot;
                // Indexing two rows (col and row) by j — can't use iter_mut
                #[allow(clippy::needless_range_loop)]
                for j in col..=n {
                    let val = &aug[col][j] * &factor;
                    aug[row][j] -= &val;
                }
            }
        }
    }

    // Back substitution
    let mut x = vec![BigRational::zero(); n];
    for i in (0..n).rev() {
        let mut sum = aug[i][n].clone();
        for j in (i + 1)..n {
            sum -= &aug[i][j] * &x[j];
        }
        if aug[i][i].is_zero() {
            return None; // Singular
        }
        x[i] = sum / &aug[i][i];
    }

    Some(x)
}

/// Compute exact Q(β) = Σ_{i>j} β_i β_j ω₀(n_{σ(j)}, n_{σ(i)}) = (1/2) β^T H β over BigRational.
///
/// Mirrors [`crate::kkt::q_from_beta`] exactly, but in exact arithmetic.
/// Q > 0 for permutations in positive Reeb direction.
fn q_from_beta_rational(
    normals: &[Vector4<f64>],
    perm: &[usize],
    beta: &[BigRational],
) -> BigRational {
    let m = beta.len();
    let mut sum = BigRational::zero();
    for i in 1..m {
        for j in 0..i {
            let omega = f64_to_rational(omega0(&normals[perm[j]], &normals[perm[i]]));
            sum += &beta[i] * &beta[j] * omega;
        }
    }
    sum
}

/// Convert a BigRational to f64 (best-effort, may lose precision for large rationals).
fn rational_to_f64(r: &BigRational) -> f64 {
    let n = r.numer().to_f64().unwrap_or(f64::NAN);
    let d = r.denom().to_f64().unwrap_or(f64::NAN);
    n / d
}

#[cfg(test)]
#[path = "kkt_rational_test.rs"]
mod kkt_rational_test;

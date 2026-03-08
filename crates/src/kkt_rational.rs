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
/// # Rank-deficient systems
///
/// When the KKT system is rank-deficient (common for polytopes with axis-aligned
/// normals in symplectic subplanes), Q(β) is constant along the null space
/// (`[lem:well-defined]`). The solver detects rank deficiency via pivot analysis,
/// extracts null-space basis vectors, and searches for β > 0 in the null space.
/// This mirrors the f64 solver's eigendecomposition-based null-space handling.
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
/// Entries are exact rationals — either natively (from `RationalPolytope4D`) or
/// converted losslessly from f64 via [`solve_kkt_exact_f64`].
use nalgebra::Vector4;
use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{One, Signed, ToPrimitive, Zero};

use crate::geom::rational::f64_to_rational;

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
/// Given facet normals (as exact BigRational), heights, and a specific permutation
/// `perm` (the σ in the thesis), builds the KKT matrix over Q and solves via
/// Gaussian elimination with null-space handling for rank-deficient systems.
///
/// Returns `None` if:
/// - the system is inconsistent, or
/// - no β > 0 solution exists (neither unique nor in the null space).
///
/// # Arguments
///
/// - `normals`: exact rational normal vectors (not necessarily unit).
/// - `heights`: exact rational positive heights.
/// - `perm`: facet index sequence defining the (S,σ) node.
pub fn solve_kkt_exact(
    normals: &[[BigRational; 4]],
    heights: &[BigRational],
    perm: &[usize],
) -> Option<ExactKktResult> {
    let m = perm.len();
    let (mat, rhs) = build_kkt_rational(normals, heights, perm);

    match gauss_solve_with_null_space(&mat, &rhs)? {
        GaussResult::FullRank(x) => {
            let beta: Vec<BigRational> = x[..m].to_vec();
            let q_exact = q_from_beta_rational(normals, perm, &beta);
            let q_exact_f64 = rational_to_f64(&q_exact);
            Some(ExactKktResult {
                beta,
                q_exact,
                q_exact_f64,
            })
        }
        GaussResult::RankDeficient {
            particular,
            null_space,
        } => {
            let beta0: Vec<BigRational> = particular[..m].to_vec();
            let null_beta: Vec<Vec<BigRational>> = null_space
                .iter()
                .map(|v| v[..m].to_vec())
                .collect();

            // Search null space for β > 0.
            let beta = if null_beta.len() == 1 {
                find_positive_beta_rational_1d(&beta0, &null_beta[0])
            } else {
                find_positive_beta_rational_nd(&beta0, &null_beta)
            }?;

            // Q is constant along the null space ([lem:well-defined]).
            let q_exact = q_from_beta_rational(normals, perm, &beta);
            let q_exact_f64 = rational_to_f64(&q_exact);
            Some(ExactKktResult {
                beta,
                q_exact,
                q_exact_f64,
            })
        }
    }
}

/// Convenience wrapper: solve KKT exactly from f64 inputs.
///
/// Converts f64 normals and heights to exact BigRational via [`f64_to_rational`],
/// then calls [`solve_kkt_exact`]. Every finite f64 is an exact rational (m·2^e),
/// so the conversion is lossless.
///
/// Note: the resulting "exact" answer is exact arithmetic on the f64 values,
/// which are themselves approximate (e.g. unit normals involve sqrt). For truly
/// exact results, pass native BigRational data from `RationalPolytope4D::new()`.
pub fn solve_kkt_exact_f64(
    normals: &[Vector4<f64>],
    heights: &[f64],
    perm: &[usize],
) -> Option<ExactKktResult> {
    let rat_normals: Vec<[BigRational; 4]> = normals
        .iter()
        .map(|n| std::array::from_fn(|i| f64_to_rational(n[i])))
        .collect();
    let rat_heights: Vec<BigRational> = heights.iter().map(|&h| f64_to_rational(h)).collect();
    solve_kkt_exact(&rat_normals, &rat_heights, perm)
}

// ── KKT matrix construction ──────────────────────────────────────────────

/// Build the KKT matrix and RHS over Q (exact rational arithmetic).
///
/// Mirrors [`crate::kkt::build_kkt_system`] exactly, but all entries are BigRational.
fn build_kkt_rational(
    normals: &[[BigRational; 4]],
    heights: &[BigRational],
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
            let val = omega0_rational(&normals[perm[i]], &normals[perm[j]]);
            mat[i][j] = val.clone();
            mat[j][i] = val;
        }
    }

    // N block: N_{i,d} = normals[perm[i]][d]
    for i in 0..m {
        for d in 0..4 {
            let val = normals[perm[i]][d].clone();
            mat[i][m + d] = val.clone();
            mat[m + d][i] = val;
        }
    }

    // η block: η_i = heights[perm[i]]
    for i in 0..m {
        let val = heights[perm[i]].clone();
        mat[i][m + 4] = val.clone();
        mat[m + 4][i] = val;
    }

    // RHS: [0, ..., 0, 1]
    rhs[m + 4] = BigRational::one();

    (mat, rhs)
}

/// Standard symplectic form ω₀(u, v) = u₀v₂ - u₂v₀ + u₁v₃ - u₃v₁ over Q.
///
/// Same formula as [`crate::geom::symplectic::omega0`] but over BigRational.
fn omega0_rational(u: &[BigRational; 4], v: &[BigRational; 4]) -> BigRational {
    &u[0] * &v[2] - &u[2] * &v[0] + &u[1] * &v[3] - &u[3] * &v[1]
}

// ── Gaussian elimination with null-space extraction ──────────────────────

/// Result of Gaussian elimination: either a unique solution or a particular
/// solution plus null-space basis vectors.
enum GaussResult {
    /// System has full rank — unique solution.
    FullRank(Vec<BigRational>),
    /// System is rank-deficient — particular solution with free variables = 0,
    /// plus basis vectors for the null space.
    RankDeficient {
        particular: Vec<BigRational>,
        null_space: Vec<Vec<BigRational>>,
    },
}

/// Relative threshold for detecting near-zero pivots during elimination.
///
/// A pivot is treated as zero (column is free / null-space direction) if:
///   |pivot| < max_entry_abs * PIVOT_RELATIVE_THRESHOLD
/// where max_entry_abs is the largest absolute entry in the original matrix.
///
/// This catches the case where f64→rational conversion makes a mathematically
/// zero eigenvalue into a tiny nonzero rational (e.g. ~10^-17).
///
/// For truly exact BigRational inputs (integer/fraction coordinates), zero
/// pivots are exactly zero and no threshold is needed. The threshold only
/// matters for f64-derived inputs.
///
/// **Why 1e-12:** f64 has ~15.9 decimal digits. A relative magnitude of 1e-12
/// means ~12 digits lost to cancellation, leaving ~4 digits of signal.
/// Well-conditioned systems (simplex, hypercube) have relative pivots > 1e-6.
/// The hko_pentagon rank-deficient node has relative pivot ~1e-17.
const PIVOT_RELATIVE_THRESHOLD: f64 = 1e-12;

/// Gaussian elimination with partial pivoting and null-space extraction.
///
/// Returns `None` if the system is inconsistent (no solution exists).
/// Returns `GaussResult::FullRank` for unique solutions.
/// Returns `GaussResult::RankDeficient` for rank-deficient systems with
/// a particular solution (free variables = 0) and null-space basis vectors.
///
/// The threshold for "effectively zero" pivots is calibrated from the maximum
/// entry magnitude of the original matrix, computed BEFORE elimination begins.
/// This avoids using near-zero pivots for elimination (which would amplify
/// noise and corrupt subsequent rows).
fn gauss_solve_with_null_space(
    mat: &[Vec<BigRational>],
    rhs: &[BigRational],
) -> Option<GaussResult> {
    let n = rhs.len();

    // Compute max entry magnitude from the original matrix for threshold.
    let max_entry_abs: f64 = mat
        .iter()
        .flat_map(|row| row.iter())
        .map(rational_abs_f64)
        .fold(0.0_f64, f64::max);
    let threshold = max_entry_abs * PIVOT_RELATIVE_THRESHOLD;

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

    // Forward elimination with partial pivoting.
    // Columns with below-threshold pivots are SKIPPED (not used for elimination).
    let mut pivot_positions: Vec<(usize, usize)> = Vec::new(); // (row, col)
    let mut free_cols: Vec<usize> = Vec::new();
    let mut current_row = 0;

    for col in 0..n {
        // Find largest-magnitude nonzero entry in this column, rows current_row..n.
        let best_row = (current_row..n)
            .filter(|&r| !aug[r][col].is_zero())
            .max_by(|&a, &b| {
                let abs_a = rational_abs_f64(&aug[a][col]);
                let abs_b = rational_abs_f64(&aug[b][col]);
                abs_a
                    .partial_cmp(&abs_b)
                    .unwrap_or(std::cmp::Ordering::Equal)
            });

        match best_row {
            None => {
                // All entries zero — column is free.
                free_cols.push(col);
            }
            Some(best) if rational_abs_f64(&aug[best][col]) <= threshold => {
                // Best pivot is below threshold — treat column as free.
                // Do NOT use this pivot for elimination (avoids noise amplification).
                free_cols.push(col);
            }
            Some(best) => {
                // Valid pivot — swap to current_row and eliminate below.
                aug.swap(current_row, best);

                for row in (current_row + 1)..n {
                    if !aug[row][col].is_zero() {
                        let factor = &aug[row][col] / &aug[current_row][col];
                        #[allow(clippy::needless_range_loop)]
                        for j in col..=n {
                            let val = &aug[current_row][j] * &factor;
                            aug[row][j] -= &val;
                        }
                    }
                }

                pivot_positions.push((current_row, col));
                current_row += 1;
            }
        }
    }

    let rank = pivot_positions.len();

    // Consistency check: rows below rank should have near-zero RHS.
    // With exact inputs, inconsistency means nonzero RHS on a zero row.
    // With f64-derived inputs, small residuals from skipped pivots are tolerated.
    for aug_row in aug.iter().take(n).skip(rank) {
        let rhs_abs = rational_abs_f64(&aug_row[n]);
        if rhs_abs > threshold.max(1e-10) {
            return None; // Inconsistent system
        }
    }

    if free_cols.is_empty() {
        // Full rank: unique solution via back substitution.
        let x = back_substitute(&aug, &pivot_positions, n)?;
        return Some(GaussResult::FullRank(x));
    }

    // Rank-deficient: extract particular solution and null space.
    let x_particular = back_substitute(&aug, &pivot_positions, n)?;

    // Null space: for each free column j, set x_j = 1, all other free vars = 0,
    // and back-substitute the pivot variables.
    let null_space: Vec<Vec<BigRational>> = free_cols
        .iter()
        .map(|&free_col| {
            let mut x = vec![BigRational::zero(); n];
            x[free_col] = BigRational::one();
            // Back-substitute pivot variables from bottom to top.
            for &(pivot_row, pivot_col) in pivot_positions.iter().rev() {
                let mut sum = BigRational::zero();
                for j in (pivot_col + 1)..n {
                    sum += &aug[pivot_row][j] * &x[j];
                }
                // RHS is 0 for null-space vectors (Ax = 0).
                x[pivot_col] = -sum / &aug[pivot_row][pivot_col];
            }
            x
        })
        .collect();

    Some(GaussResult::RankDeficient {
        particular: x_particular,
        null_space,
    })
}

/// Back substitution from row echelon form.
///
/// Uses the pivot positions to extract the solution. Free variables (not in
/// pivot_positions) remain at zero.
fn back_substitute(
    aug: &[Vec<BigRational>],
    pivot_positions: &[(usize, usize)],
    n: usize,
) -> Option<Vec<BigRational>> {
    let mut x = vec![BigRational::zero(); n];
    for &(pivot_row, pivot_col) in pivot_positions.iter().rev() {
        let mut sum = aug[pivot_row][n].clone(); // RHS
        for j in (pivot_col + 1)..n {
            sum -= &aug[pivot_row][j] * &x[j];
        }
        if aug[pivot_row][pivot_col].is_zero() {
            return None;
        }
        x[pivot_col] = sum / &aug[pivot_row][pivot_col];
    }
    Some(x)
}

// ── Null-space search for β > 0 ─────────────────────────────────────────

/// Search 1D null space for β > 0 (exact rational arithmetic).
///
/// Given particular solution β₀ and null-space vector v, finds α ∈ Q such that
/// β₀ + α·v has all components strictly positive. Returns the midpoint of the
/// feasible interval for maximum margin.
///
/// Returns `None` if no α makes all β_i > 0.
fn find_positive_beta_rational_1d(
    beta0: &[BigRational],
    v: &[BigRational],
) -> Option<Vec<BigRational>> {
    let m = beta0.len();
    let mut lo: Option<BigRational> = None; // None = -∞
    let mut hi: Option<BigRational> = None; // None = +∞

    for j in 0..m {
        if v[j].is_zero() {
            // Component fixed — must already be positive.
            if !beta0[j].is_positive() {
                return None;
            }
        } else {
            // β₀[j] + α·v[j] > 0  ⟺  α > -β₀[j]/v[j]  (if v[j] > 0)
            //                        ⟺  α < -β₀[j]/v[j]  (if v[j] < 0)
            let bound = -&beta0[j] / &v[j];
            if v[j].is_positive() {
                lo = Some(match lo {
                    Some(l) => l.max(bound),
                    None => bound,
                });
            } else {
                hi = Some(match hi {
                    Some(h) => h.min(bound),
                    None => bound,
                });
            }
        }
    }

    // Feasibility: lo < hi (strict for β > 0).
    match (&lo, &hi) {
        (Some(l), Some(h)) if l >= h => return None,
        _ => {}
    }

    // Pick midpoint of [lo, hi] for maximum margin from both bounds.
    let two = BigRational::from(BigInt::from(2));
    let alpha = match (lo, hi) {
        (Some(l), Some(h)) => (&l + &h) / &two,
        (Some(l), None) => &l + BigRational::one(),
        (None, Some(h)) => &h - BigRational::one(),
        (None, None) => BigRational::zero(),
    };

    let beta: Vec<BigRational> = (0..m)
        .map(|j| &beta0[j] + &alpha * &v[j])
        .collect();

    if beta.iter().all(|b| b.is_positive()) {
        Some(beta)
    } else {
        None
    }
}

/// Search multi-dimensional null space for β > 0 (exact rational arithmetic).
///
/// Uses iterative coordinate ascent on the most-violated constraint.
/// For each iteration: find the worst β_j, pick the null-space direction with
/// the largest component at index j, and step to push β_j positive.
///
/// Returns `None` if no combination of null-space directions makes all β_i > 0.
fn find_positive_beta_rational_nd(
    beta0: &[BigRational],
    null_vecs: &[Vec<BigRational>],
) -> Option<Vec<BigRational>> {
    let m = beta0.len();
    let k = null_vecs.len();
    let mut alpha = vec![BigRational::zero(); k];

    for _iter in 0..100 {
        // Current β = β₀ + Σ αᵢ vᵢ
        let beta: Vec<BigRational> = (0..m)
            .map(|j| {
                let mut val = beta0[j].clone();
                for i in 0..k {
                    val += &alpha[i] * &null_vecs[i][j];
                }
                val
            })
            .collect();

        // Find most-violated component (smallest β_j).
        let (worst_j, worst_val) = beta
            .iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| a.cmp(b))
            .unwrap();

        if worst_val.is_positive() {
            return Some(beta); // All β > 0
        }

        // Find the null-space direction with largest |v_i[worst_j]|.
        let best_dir = (0..k)
            .filter(|&i| !null_vecs[i][worst_j].is_zero())
            .max_by(|&a, &b| {
                null_vecs[a][worst_j]
                    .abs()
                    .cmp(&null_vecs[b][worst_j].abs())
            });

        let Some(dir) = best_dir else {
            return None; // Can't improve worst component
        };

        // Step: push β[worst_j] to a small positive target.
        // target = 1/1000 (arbitrary small positive rational)
        let target =
            BigRational::new(BigInt::from(1), BigInt::from(1000));
        let step = (&target - worst_val) / &null_vecs[dir][worst_j];
        alpha[dir] += step;
    }

    // Final check after iterations.
    let beta: Vec<BigRational> = (0..m)
        .map(|j| {
            let mut val = beta0[j].clone();
            for i in 0..k {
                val += &alpha[i] * &null_vecs[i][j];
            }
            val
        })
        .collect();

    if beta.iter().all(|b| b.is_positive()) {
        Some(beta)
    } else {
        None
    }
}

// ── Q computation ────────────────────────────────────────────────────────

/// Compute exact Q(β) = Σ_{i>j} β_i β_j ω₀(n_{σ(j)}, n_{σ(i)}) over BigRational.
///
/// Mirrors [`crate::kkt::q_from_beta`] exactly, but in exact arithmetic.
/// Q > 0 for permutations in positive Reeb direction.
fn q_from_beta_rational(
    normals: &[[BigRational; 4]],
    perm: &[usize],
    beta: &[BigRational],
) -> BigRational {
    let m = beta.len();
    let mut sum = BigRational::zero();
    for i in 1..m {
        for j in 0..i {
            let omega = omega0_rational(&normals[perm[j]], &normals[perm[i]]);
            sum += &beta[i] * &beta[j] * omega;
        }
    }
    sum
}

// ── Helpers ──────────────────────────────────────────────────────────────

/// Approximate absolute value of a BigRational as f64 (for pivot comparison only).
fn rational_abs_f64(r: &BigRational) -> f64 {
    let n = r.numer().to_f64().unwrap_or(0.0);
    let d = r.denom().to_f64().unwrap_or(1.0);
    (n / d).abs()
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

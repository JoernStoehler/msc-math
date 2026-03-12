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
/// Gaussian elimination over BigRational is substantially slower than f64 eigendecomposition
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
/// Entries are exact rationals from `Polytope4D::dual_vertices()`.
///
/// The η block is all ones because dual vertices y_i = n_i/h_i absorb the heights.
/// This is mathematically equivalent to the f64 system (which uses separate n_i and h_i
/// with η_i = h_i): the change of variable β_rational_i = β_f64_i · h_i preserves Q(β).
use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{One, Signed, ToPrimitive, Zero};

/// Result of an exact KKT solve over BigRational.
#[derive(Clone, Debug)]
pub struct ExactKktResult {
    /// Exact β vector (all components rational).
    pub beta: Vec<BigRational>,
    /// Exact Q(β) = Σ_{i>j} β_i β_j ω₀(y_{σ(j)}, y_{σ(i)}) over Q.
    pub q_exact: BigRational,
    /// Q_exact converted to f64 (for convenient comparison with f64 solver).
    pub q_exact_f64: f64,
}

/// Solve the KKT system exactly for a single (S,σ) combinatorics.
///
/// Given dual vertices y_i = n_i/h_i (from `Polytope4D::dual_vertices()`) and a
/// specific permutation `perm` (the σ in the thesis), builds the KKT matrix over Q
/// and solves via Gaussian elimination with null-space handling for rank-deficient
/// systems.
///
/// The dual vertex representation {y_i · x ≤ 1} has implicit heights h_i = 1,
/// so the η block of the KKT matrix is all ones.
///
/// Returns `None` (certified) if:
/// - the system is inconsistent, or
/// - no β > 0 solution exists (certified via Fourier-Motzkin elimination).
///
/// # Arguments
///
/// - `dual_vertices`: exact rational dual vertices y_i = n_i/h_i ∈ Q^4.
/// - `perm`: facet index sequence defining the (S,σ) node.
pub fn solve_kkt_exact(
    dual_vertices: &[[BigRational; 4]],
    perm: &[usize],
) -> Option<ExactKktResult> {
    let m = perm.len();
    let (mat, rhs) = build_kkt_rational(dual_vertices, perm);

    match gauss_solve_with_null_space(&mat, &rhs)? {
        GaussResult::FullRank(x) => {
            let beta: Vec<BigRational> = x[..m].to_vec();
            let q_exact = q_from_beta_rational(dual_vertices, perm, &beta);
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

            // Search null space for β > 0 (exact via Fourier-Motzkin).
            let beta = find_positive_beta_rational(&beta0, &null_beta)?;

            // Q is constant along the null space ([lem:well-defined]).
            let q_exact = q_from_beta_rational(dual_vertices, perm, &beta);
            let q_exact_f64 = rational_to_f64(&q_exact);
            Some(ExactKktResult {
                beta,
                q_exact,
                q_exact_f64,
            })
        }
    }
}

// ── KKT matrix construction ──────────────────────────────────────────────

/// Build the KKT matrix and RHS over Q (exact rational arithmetic).
///
/// Same block structure as [`crate::kkt::build_kkt_system`], but uses dual vertices
/// y_i = n_i/h_i instead of separate normals and heights. The η block is all ones
/// (heights are absorbed into the dual vertices).
fn build_kkt_rational(
    dual_vertices: &[[BigRational; 4]],
    perm: &[usize],
) -> (Vec<Vec<BigRational>>, Vec<BigRational>) {
    let m = perm.len();
    let size = m + 5;
    let zero = BigRational::zero();

    let mut mat = vec![vec![zero.clone(); size]; size];
    let mut rhs = vec![zero.clone(); size];

    // H block: H_{ij} = ω₀(y_i, y_j) for i ≠ j (symmetric)
    for i in 0..m {
        for j in (i + 1)..m {
            let val = omega0_rational(&dual_vertices[perm[i]], &dual_vertices[perm[j]]);
            mat[i][j] = val.clone();
            mat[j][i] = val;
        }
    }

    // N block: N_{i,d} = y_{perm[i]}[d]
    for i in 0..m {
        for d in 0..4 {
            let val = dual_vertices[perm[i]][d].clone();
            mat[i][m + d] = val.clone();
            mat[m + d][i] = val;
        }
    }

    // η block: all ones (dual vertex representation has h_i = 1)
    let one = BigRational::one();
    #[allow(clippy::needless_range_loop)]
    for i in 0..m {
        mat[i][m + 4] = one.clone();
        mat[m + 4][i] = one.clone();
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
    // Floor of 1e-10: when threshold is tiny (small matrix entries), we still need
    // a reasonable absolute floor to avoid false inconsistency from pivot-skip artifacts.
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

// ── Null-space search for β > 0 (Fourier-Motzkin) ────────────────────────

/// Exact feasibility search for β₀ + V·α > 0 over Q.
///
/// Given particular solution β₀ and null-space basis vectors v₁,...,vₖ,
/// decides whether there exist α₁,...,αₖ ∈ Q such that
///   β₀[j] + α₁·v₁[j] + ... + αₖ·vₖ[j] > 0   for all j.
///
/// Uses Fourier-Motzkin variable elimination: exact and certifying.
/// - `Some(β)`: witness with all β[j] > 0.
/// - `None`: no solution exists (certified).
///
/// Complexity: O(m^{2^k}) constraints worst-case, where m = len(β₀),
/// k = len(null_vecs). For KKT systems (m ≤ 16, k ≤ 3): ≤ ~1000 constraints.
fn find_positive_beta_rational(
    beta0: &[BigRational],
    null_vecs: &[Vec<BigRational>],
) -> Option<Vec<BigRational>> {
    let m = beta0.len();
    let k = null_vecs.len();

    // Each constraint: coeffs · α > rhs  (strict inequality).
    // coeffs.len() shrinks by 1 at each elimination step.
    type Constraint = (Vec<BigRational>, BigRational);

    // Initial system: v[i][j] · αᵢ > -β₀[j] for each component j.
    let mut constraints: Vec<Constraint> = (0..m)
        .map(|j| {
            let coeffs: Vec<BigRational> =
                (0..k).map(|i| null_vecs[i][j].clone()).collect();
            (coeffs, -&beta0[j])
        })
        .collect();

    // A bound records: α[var] ≷ (rhs - remaining · α_remaining) / divisor.
    // divisor > 0 → lower bound; divisor < 0 → upper bound.
    struct Bound {
        remaining_coeffs: Vec<BigRational>,
        rhs: BigRational,
        divisor: BigRational,
    }

    // Forward pass: eliminate variables k-1, k-2, ..., 0 (always the last index
    // in the current coefficient vector, so remove(idx) = pop).
    let mut stages: Vec<Vec<Bound>> = Vec::with_capacity(k);

    for elim_idx in (0..k).rev() {
        let mut bounds = Vec::new();
        let mut positive: Vec<&Constraint> = Vec::new(); // coeff[elim_idx] > 0
        let mut negative: Vec<&Constraint> = Vec::new(); // coeff[elim_idx] < 0
        let mut new_constraints: Vec<Constraint> = Vec::new();

        for c in &constraints {
            let coeff = &c.0[elim_idx];
            if coeff.is_positive() {
                positive.push(c);
            } else if coeff.is_negative() {
                negative.push(c);
            } else {
                // Pass through (remove the zero-coefficient column).
                let mut new_coeffs = c.0.clone();
                new_coeffs.remove(elim_idx);
                new_constraints.push((new_coeffs, c.1.clone()));
            }
        }

        // Record bounds for back-substitution.
        for c in positive.iter().chain(negative.iter()) {
            let mut remaining = c.0.clone();
            let divisor = remaining.remove(elim_idx);
            bounds.push(Bound {
                remaining_coeffs: remaining,
                rhs: c.1.clone(),
                divisor,
            });
        }
        stages.push(bounds);

        // Combine each (positive, negative) pair to eliminate α[elim_idx].
        // Lower: a_l · α + ... > r_l  →  α > (r_l - ...) / a_l
        // Upper: a_u · α + ... > r_u  →  α < (r_u - ...) / a_u  (a_u < 0)
        // Combined: Σ (a_l·c_u[i] - a_u·c_l[i]) α[i] > a_l·r_u - a_u·r_l
        for (c_l, r_l) in &positive {
            for (c_u, r_u) in &negative {
                let a_l = &c_l[elim_idx];
                let a_u = &c_u[elim_idx];
                let mut new_coeffs = Vec::with_capacity(c_l.len() - 1);
                for i in 0..c_l.len() {
                    if i == elim_idx {
                        continue;
                    }
                    new_coeffs.push(a_l * &c_u[i] - a_u * &c_l[i]);
                }
                let new_rhs = a_l * r_u - a_u * r_l;
                new_constraints.push((new_coeffs, new_rhs));
            }
        }

        constraints = new_constraints;
    }

    // After all eliminations: constraints have empty coefficients.
    // Feasibility: 0 > rhs, i.e., rhs must be negative.
    for (coeffs, rhs) in &constraints {
        debug_assert!(coeffs.is_empty());
        if !rhs.is_negative() {
            return None; // Infeasible (certified)
        }
    }

    // Back-substitution: assign α values from last-eliminated to first.
    // stages[s] eliminated variable (k-1-s), with remaining_coeffs for vars [0..k-1-s).
    // Process in reverse: assign var 0 (from stages[k-1]), then var 1, ..., var k-1.
    let two = BigRational::from(BigInt::from(2));
    let mut alpha = vec![BigRational::zero(); k];

    for assign_var in 0..k {
        let stage_idx = k - 1 - assign_var;
        let mut lo: Option<BigRational> = None;
        let mut hi: Option<BigRational> = None;

        for bound in &stages[stage_idx] {
            // Evaluate: (rhs - remaining · α_assigned) / divisor
            let mut numerator = bound.rhs.clone();
            for (i, c) in bound.remaining_coeffs.iter().enumerate() {
                numerator -= c * &alpha[i];
            }
            let value = &numerator / &bound.divisor;

            if bound.divisor.is_positive() {
                lo = Some(match lo {
                    Some(l) => l.max(value),
                    None => value,
                });
            } else {
                hi = Some(match hi {
                    Some(h) => h.min(value),
                    None => value,
                });
            }
        }

        alpha[assign_var] = match (&lo, &hi) {
            (Some(l), Some(h)) => {
                debug_assert!(l < h, "FM back-sub: lo >= hi (should be infeasible)");
                (l + h) / &two
            }
            (Some(l), None) => l + BigRational::one(),
            (None, Some(h)) => h - BigRational::one(),
            (None, None) => BigRational::zero(),
        };
    }

    // Compute β = β₀ + V · α.
    let beta: Vec<BigRational> = (0..m)
        .map(|j| {
            let mut val = beta0[j].clone();
            for i in 0..k {
                val += &alpha[i] * &null_vecs[i][j];
            }
            val
        })
        .collect();

    debug_assert!(
        beta.iter().all(|b| b.is_positive()),
        "FM back-substitution produced non-positive β"
    );
    Some(beta)
}

// ── Q computation ────────────────────────────────────────────────────────

/// Compute exact Q(β) = Σ_{i>j} β_i β_j ω₀(y_{σ(j)}, y_{σ(i)}) over BigRational.
///
/// Same formula as [`crate::kkt::q_from_beta`] but in exact arithmetic over Q,
/// using dual vertices y_i instead of unit normals.
/// Q > 0 for permutations in positive Reeb direction.
fn q_from_beta_rational(
    dual_vertices: &[[BigRational; 4]],
    perm: &[usize],
    beta: &[BigRational],
) -> BigRational {
    let m = beta.len();
    (1..m)
        .flat_map(|i| (0..i).map(move |j| (i, j)))
        .map(|(i, j)| {
            let omega = omega0_rational(&dual_vertices[perm[j]], &dual_vertices[perm[i]]);
            &beta[i] * &beta[j] * omega
        })
        .fold(BigRational::zero(), |acc, x| acc + x)
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

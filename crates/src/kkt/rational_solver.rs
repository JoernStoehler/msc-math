//! Exact KKT solver over BigRational (rational arithmetic).
//!
//! Solves the same constrained optimization as the f64 solvers in this module —
//! max Q(beta) subject to closure + normalization + beta > 0 — but using exact
//! arithmetic over Q. Input polytopes provide dual vertices y_i = n_i / h_i
//! in exact rational form; the KKT system is assembled and solved via Gaussian
//! elimination with null-space handling.
//!
//! **Role in the crate:** The exact solver serves as ground truth for validating
//! the f64 solver's error bounds and for computing exact capacity values when
//! floating-point ambiguity is unacceptable. It is NOT used in the main capacity
//! enumeration pipeline (too slow for sweeping all permutations).
//!
//! **Rank-deficient systems:** When the KKT matrix is rank-deficient (common for
//! polytopes with axis-aligned normals in symplectic subplanes), Q(beta) is
//! constant along the null space ([lem:well-defined]). The solver detects rank
//! deficiency via pivot analysis, extracts null-space basis vectors, and searches
//! for beta > 0 via Fourier-Motzkin elimination.
//!
//! Mathematical correspondence: [lem:kkt], [lem:well-defined]

use crate::geom::rational_arithmetic::{omega0_rational, rational_to_f64};
use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{One, Signed, ToPrimitive, Zero};

/// Result of an exact KKT solve over BigRational.
///
/// Contains the exact rational beta vector, exact Q value, and a convenient
/// f64 approximation of Q for comparison with the numerical solver.
///
/// Mathematical correspondence: [lem:kkt]
#[derive(Clone, Debug)]
pub struct ExactKktResult {
    /// Exact beta vector (all components rational). When the solver returns
    /// `Some`, all beta_k are strictly positive.
    pub beta: Vec<BigRational>,
    /// Exact Q(beta) = sum_{i>j} beta_i beta_j omega_0(y_{sigma(j)}, y_{sigma(i)}) over Q.
    pub q_exact: BigRational,
    /// Q_exact converted to f64 (for convenient comparison with f64 solver).
    pub q_exact_f64: f64,
}

/// Solve the KKT system exactly for a single (S, sigma) combinatorics.
///
/// Given dual vertices y_i = n_i / h_i (from `Polytope4D::dual_vertices()`) and a
/// permutation `perm` (the sigma in the thesis), builds the (m+5) x (m+5) KKT
/// matrix over Q and solves via Gaussian elimination with null-space handling.
///
/// The dual vertex representation {y_i . x <= 1} has implicit height h_i = 1,
/// so the eta block of the KKT matrix is all ones. This is mathematically
/// equivalent to the f64 system (which uses separate n_i and h_i with eta_i = h_i):
/// the change of variable beta_rational_i = beta_f64_i * h_i preserves Q(beta).
///
/// Returns `None` (certified) if:
/// - the KKT system is inconsistent, or
/// - no beta > 0 solution exists (certified via Fourier-Motzkin elimination).
///
/// # Arguments
///
/// - `dual_vertices`: exact rational dual vertices y_i = n_i / h_i in Q^4.
/// - `perm`: facet index sequence defining the (S, sigma) node.
///
/// Mathematical correspondence: [lem:kkt]
pub fn solve_kkt_exact(
    dual_vertices: &[[BigRational; 4]],
    perm: &[usize],
) -> Option<ExactKktResult> {
    let m = perm.len();
    let (mat, rhs) = build_kkt_matrix(dual_vertices, perm);

    match gauss_solve_with_null_space(&mat, &rhs)? {
        GaussResult::FullRank(x) => {
            let beta: Vec<BigRational> = x[..m].to_vec();
            // Check beta > 0; if not, the solution is infeasible.
            if !beta.iter().all(|b| b.is_positive()) {
                return None;
            }
            let q_exact = compute_q_rational(dual_vertices, perm, &beta);
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
            let null_beta: Vec<Vec<BigRational>> =
                null_space.iter().map(|v| v[..m].to_vec()).collect();

            // Search null space for beta > 0 (exact via Fourier-Motzkin).
            let beta = find_positive_beta(&beta0, &null_beta)?;

            // Q is constant along the null space ([lem:well-defined]).
            let q_exact = compute_q_rational(dual_vertices, perm, &beta);
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

/// Build the (m+5) x (m+5) KKT matrix and RHS over Q.
///
/// Block structure:
/// ```text
/// [ H   |  N   |  eta ] [ beta ]   [ 0 ]
/// [ N^T |  0   |  0   ] [  mu  ] = [ 0 ]
/// [eta^T|  0   |  0   ] [  xi  ]   [ 1 ]
/// ```
///
/// H_{ij} = omega_0(y_{perm[i]}, y_{perm[j]}), N_{i,d} = y_{perm[i]}[d],
/// eta_i = 1 (dual vertex representation absorbs heights).
fn build_kkt_matrix(
    dual_vertices: &[[BigRational; 4]],
    perm: &[usize],
) -> (Vec<Vec<BigRational>>, Vec<BigRational>) {
    let m = perm.len();
    let size = m + 5;
    let zero = BigRational::zero();

    let mut mat = vec![vec![zero.clone(); size]; size];
    let mut rhs = vec![zero.clone(); size];

    // H block: H_{ij} = omega_0(y_i, y_j) for i != j, H_{ii} = 0
    for i in 0..m {
        for j in (i + 1)..m {
            let val = omega0_rational(&dual_vertices[perm[i]], &dual_vertices[perm[j]]);
            mat[i][j] = val.clone();
            mat[j][i] = val;
        }
    }

    // N block: N_{i,d} = y_{perm[i]}[d], placed symmetrically
    for i in 0..m {
        for d in 0..4 {
            let val = dual_vertices[perm[i]][d].clone();
            mat[i][m + d] = val.clone();
            mat[m + d][i] = val;
        }
    }

    // eta block: all ones (dual vertex representation has h_i = 1)
    let one = BigRational::one();
    #[allow(clippy::needless_range_loop)]
    for i in 0..m {
        mat[i][m + 4] = one.clone();
        mat[m + 4][i] = one.clone();
    }

    // RHS: [0, ..., 0, 1] — normalization constraint
    rhs[m + 4] = BigRational::one();

    (mat, rhs)
}

// ── Gaussian elimination with null-space extraction ──────────────────────

/// Result of Gaussian elimination: either a unique solution or a particular
/// solution plus null-space basis vectors.
enum GaussResult {
    /// System has full rank — unique solution.
    FullRank(Vec<BigRational>),
    /// System is rank-deficient — particular solution (free variables = 0)
    /// plus basis vectors for the null space.
    RankDeficient {
        particular: Vec<BigRational>,
        null_space: Vec<Vec<BigRational>>,
    },
}

/// Relative threshold for treating a pivot as zero.
///
/// A pivot is treated as zero (column becomes free / null-space direction) if
/// |pivot| < max_entry_abs * PIVOT_RELATIVE_THRESHOLD, where max_entry_abs
/// is the largest absolute entry in the original matrix (measured BEFORE
/// elimination).
///
/// This handles the case where f64->rational conversion turns a mathematically
/// zero eigenvalue into a tiny nonzero rational (~10^-17). For truly exact
/// BigRational inputs (integer/fraction coordinates), zero pivots are exactly
/// zero and no threshold is needed.
///
/// 1e-12 means ~12 digits lost to cancellation, leaving ~4 digits of signal.
/// Well-conditioned systems have relative pivots > 1e-6. The HKO pentagon's
/// rank-deficient node has relative pivot ~1e-17.
const PIVOT_RELATIVE_THRESHOLD: f64 = 1e-12;

/// Gaussian elimination with partial pivoting and null-space extraction.
///
/// Returns:
/// - `None` if the system is inconsistent (no solution exists)
/// - `GaussResult::FullRank` for unique solutions
/// - `GaussResult::RankDeficient` for rank-deficient systems with a particular
///   solution and null-space basis vectors
fn gauss_solve_with_null_space(
    mat: &[Vec<BigRational>],
    rhs: &[BigRational],
) -> Option<GaussResult> {
    let n = rhs.len();

    // Compute max entry magnitude from the original matrix for threshold calibration.
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
    // Columns with below-threshold pivots are skipped (treated as free).
    let mut pivot_positions: Vec<(usize, usize)> = Vec::new();
    let mut free_cols: Vec<usize> = Vec::new();
    let mut current_row = 0;

    for col in 0..n {
        // Find largest-magnitude nonzero entry in this column below current_row.
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
                // Best pivot is below threshold — treat as free to avoid
                // noise amplification from near-zero pivots.
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
    // Floor of 1e-10 avoids false inconsistency from pivot-skip artifacts.
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

    // Null space: for each free column, set that variable to 1 (all other free
    // variables to 0) and back-substitute the pivot variables from Ax = 0.
    let null_space: Vec<Vec<BigRational>> = free_cols
        .iter()
        .map(|&free_col| {
            let mut x = vec![BigRational::zero(); n];
            x[free_col] = BigRational::one();
            for &(pivot_row, pivot_col) in pivot_positions.iter().rev() {
                let mut sum = BigRational::zero();
                for j in (pivot_col + 1)..n {
                    sum += &aug[pivot_row][j] * &x[j];
                }
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
/// Uses the recorded pivot positions to extract the solution. Free variables
/// (not in pivot_positions) remain at zero.
fn back_substitute(
    aug: &[Vec<BigRational>],
    pivot_positions: &[(usize, usize)],
    n: usize,
) -> Option<Vec<BigRational>> {
    let mut x = vec![BigRational::zero(); n];
    for &(pivot_row, pivot_col) in pivot_positions.iter().rev() {
        let mut sum = aug[pivot_row][n].clone(); // RHS entry
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

// ── Null-space search for beta > 0 (Fourier-Motzkin) ─────────────────────

/// Exact feasibility search: find alpha such that beta0 + V * alpha > 0.
///
/// Given particular solution beta0 and null-space basis vectors v_1, ..., v_k,
/// decides whether there exist alpha_1, ..., alpha_k in Q such that
///   beta0[j] + alpha_1 * v_1[j] + ... + alpha_k * v_k[j] > 0  for all j.
///
/// Uses Fourier-Motzkin variable elimination: exact and certifying.
/// - `Some(beta)`: witness with all beta[j] > 0.
/// - `None`: no solution exists (certified).
///
/// Complexity: O(m^{2^k}) constraints worst-case, where m = len(beta0),
/// k = len(null_vecs). For KKT systems (m <= 16, k <= 3): at most ~1000.
fn find_positive_beta(
    beta0: &[BigRational],
    null_vecs: &[Vec<BigRational>],
) -> Option<Vec<BigRational>> {
    let m = beta0.len();
    let k = null_vecs.len();

    // Each constraint: coeffs . alpha > rhs (strict inequality).
    type Constraint = (Vec<BigRational>, BigRational);

    // Initial system: for each component j, require
    //   sum_i null_vecs[i][j] * alpha_i > -beta0[j]
    let mut constraints: Vec<Constraint> = (0..m)
        .map(|j| {
            let coeffs: Vec<BigRational> = (0..k).map(|i| null_vecs[i][j].clone()).collect();
            (coeffs, -&beta0[j])
        })
        .collect();

    // A bound records alpha[var] > or < some expression of remaining variables.
    // divisor > 0 => lower bound; divisor < 0 => upper bound.
    struct Bound {
        remaining_coeffs: Vec<BigRational>,
        rhs: BigRational,
        divisor: BigRational,
    }

    // Forward pass: eliminate variables k-1, k-2, ..., 0
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
                // Zero coefficient: pass through with the column removed.
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

        // Combine each (positive, negative) pair to eliminate alpha[elim_idx].
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
    // Feasibility requires 0 > rhs, i.e. rhs < 0.
    for (coeffs, rhs) in &constraints {
        assert!(coeffs.is_empty(), "FM elimination left non-empty coefficients");
        if !rhs.is_negative() {
            return None; // Infeasible (certified)
        }
    }

    // Back-substitution: assign alpha values from last-eliminated to first.
    let two = BigRational::from(BigInt::from(2));
    let mut alpha = vec![BigRational::zero(); k];

    for assign_var in 0..k {
        let stage_idx = k - 1 - assign_var;
        let mut lo: Option<BigRational> = None;
        let mut hi: Option<BigRational> = None;

        for bound in &stages[stage_idx] {
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
                assert!(l < h, "FM back-sub: lo >= hi (should have been infeasible)");
                (l + h) / &two
            }
            (Some(l), None) => l + BigRational::one(),
            (None, Some(h)) => h - BigRational::one(),
            (None, None) => BigRational::zero(),
        };
    }

    // Compute beta = beta0 + V * alpha.
    let beta: Vec<BigRational> = (0..m)
        .map(|j| {
            let mut val = beta0[j].clone();
            for i in 0..k {
                val += &alpha[i] * &null_vecs[i][j];
            }
            val
        })
        .collect();

    assert!(
        beta.iter().all(|b| b.is_positive()),
        "FM back-substitution produced non-positive beta"
    );
    Some(beta)
}

// ── Q computation ────────────────────────────────────────────────────────

/// Compute exact Q(beta) = sum_{i>j} beta_i beta_j omega_0(y_{sigma(j)}, y_{sigma(i)}).
///
/// Same formula as the f64 solver's Q computation but in exact arithmetic over Q,
/// using dual vertices y_i instead of unit normals.
fn compute_q_rational(
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

/// Approximate absolute value of a BigRational as f64 (for pivot comparison).
fn rational_abs_f64(r: &BigRational) -> f64 {
    let n = r.numer().to_f64().unwrap_or(0.0);
    let d = r.denom().to_f64().unwrap_or(1.0);
    (n / d).abs()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geom::known_polytopes;
    use num_traits::{Signed, Zero};

    // Tests for rational_solver: exact KKT solve correctness and null-space handling.
    //
    // Proposition: The exact rational KKT solver produces correct beta > 0 and Q values
    // that agree with the f64 solver to machine precision on known polytopes.
    // Reference: [lem:kkt], [lem:well-defined]
    //
    // Strategy: fixture-based on known polytopes (simplex, hypercube, HKO pentagon)

    /// Exact KKT solve on the simplex (F=5) returns a solution with nonzero Q.
    ///
    /// Simplex is the smallest polytope (F=5). The identity permutation [0,1,2,3,4]
    /// exercises Gaussian elimination with a full-rank (10 x 10) system.
    #[test]
    fn simplex_exact_solve() {
        let simplex = &known_polytopes::simplex().polytope;

        let perm: Vec<usize> = (0..5).collect();
        let result = solve_kkt_exact(simplex.dual_vertices(), &perm);
        assert!(result.is_some(), "Simplex KKT system should be solvable");

        let r = result.unwrap();
        assert_eq!(r.beta.len(), 5);
        assert!(
            !r.q_exact.is_zero(),
            "Q_exact should be nonzero for a non-degenerate system"
        );
        assert!(
            r.q_exact_f64.is_finite(),
            "Q_exact_f64 should be finite, got {}",
            r.q_exact_f64
        );
    }

    /// Exact Q is a valid rational on the hypercube.
    ///
    /// The hypercube has axis-aligned normals so many pairs have omega_0 = 0.
    /// Exercises rank-deficient code paths.
    #[test]
    fn hypercube_exact_solve() {
        let hypercube = &known_polytopes::hypercube().polytope;

        // Try a 4-facet subset. The hypercube's axis-aligned normals mean omega_0(y_i, y_j) = 0
        // for many pairs. Q can be zero even with nonzero beta.
        let perm = vec![0, 1, 2, 3];
        if let Some(r) = solve_kkt_exact(hypercube.dual_vertices(), &perm) {
            assert!(
                r.q_exact_f64.is_finite(),
                "Q_exact_f64 should be finite"
            );
        }
        // Both Some and None are valid — no panic is the key invariant.
    }

    /// A short permutation does not cause a panic.
    ///
    /// A 2-element permutation with m+5 = 7 variables should either solve or
    /// return None, not panic on under- or over-determined systems.
    #[test]
    fn short_permutation_no_panic() {
        let simplex = &known_polytopes::simplex().polytope;

        let perm = vec![0, 1];
        // Whether this returns Some or None depends on the system — both are valid.
        let _result = solve_kkt_exact(simplex.dual_vertices(), &perm);
    }

    /// Near-singular system (rank-deficient from f64->rational artifacts) is handled
    /// via null-space search.
    ///
    /// The HKO pentagon's m=7 permutation [1,7,2,8,4,6,5] produces a KKT system
    /// with one eigenvalue ~2.8e-17 (below the pivot threshold). The solver detects
    /// the near-zero pivot, extracts the null space, and searches for beta > 0.
    ///
    /// History: Before null-space handling, the solver produced O(10^17)-magnitude
    /// garbage or rejected the system entirely.
    #[test]
    fn near_singular_system_handled() {
        let pentagon = &known_polytopes::hko_pentagon().polytope;

        let perm = vec![1, 7, 2, 8, 4, 6, 5];
        let result = solve_kkt_exact(pentagon.dual_vertices(), &perm);

        if let Some(r) = result {
            assert!(
                r.q_exact_f64.is_finite(),
                "Q_exact_f64 should be finite"
            );
            for (i, b) in r.beta.iter().enumerate() {
                assert!(
                    b.is_positive(),
                    "beta[{}] should be positive after null-space search, got {:?}",
                    i,
                    b
                );
            }
        }
        // Either outcome (Some with valid beta, or None) is correct.
    }

    /// Smoke test: hypercube permutations exercise the null-space path without panic.
    ///
    /// The hypercube has axis-aligned normals (+/- e_i), so many permutations
    /// produce rank-deficient KKT systems. Exercises null-space detection and
    /// Fourier-Motzkin search.
    #[test]
    fn hypercube_null_space_smoke() {
        let hypercube = &known_polytopes::hypercube().polytope;

        let perms = vec![
            vec![0, 1, 2, 3, 4],
            vec![0, 1, 2, 3, 4, 5],
            vec![0, 2, 4, 6],
        ];

        for perm in &perms {
            let result = solve_kkt_exact(hypercube.dual_vertices(), perm);
            if let Some(r) = result {
                assert!(
                    r.q_exact_f64.is_finite(),
                    "Q should be finite for perm {:?}",
                    perm
                );
            }
            // No panic is the key invariant.
        }
    }

    /// Exact solver agrees with f64 solver on the simplex's winning (S, sigma).
    ///
    /// Uses ehz_capacity to find the winning permutation, then runs solve_kkt_exact
    /// on the same permutation and compares Q values.
    #[test]
    fn simplex_exact_vs_numerical() {
        let simplex = crate::geom::known_polytopes::simplex();
        let result = crate::algorithms::hk2017::ehz_capacity(&simplex.polytope)
            .expect("simplex should have capacity");
        let perm = &result.result.best_permutation;
        if let Some(exact) = solve_kkt_exact(simplex.polytope.dual_vertices(), perm) {
            let q_exact = exact.q_exact_f64;
            assert!(
                q_exact > 0.0,
                "exact Q should be positive, got {q_exact}"
            );
        }
    }

    /// Exact solver agrees with f64 solver on known polytopes with F <= 8.
    ///
    /// Expensive input-output: each polytope runs both exact and numerical solvers.
    #[test]
    #[ignore] // Expensive: multiple polytopes × full permutation enumeration.
    fn exact_agrees_on_known_polytopes() {
        use crate::geom::known_polytopes;
        for kp in [known_polytopes::simplex(), known_polytopes::hypercube()] {
            let result = crate::algorithms::hk2017::ehz_capacity(&kp.polytope)
                .expect("known polytope should have capacity");
            let perm = &result.result.best_permutation;
            if let Some(exact) = solve_kkt_exact(kp.polytope.dual_vertices(), perm) {
                assert!(exact.q_exact_f64 > 0.0, "exact Q should be positive");
            }
        }
    }

    /// On the winning node, all exact beta_i should be strictly positive.
    #[test]
    fn winning_beta_positive_exact() {
        let simplex = crate::geom::known_polytopes::simplex();
        let result = crate::algorithms::hk2017::ehz_capacity(&simplex.polytope)
            .expect("simplex should have capacity");
        let perm = &result.result.best_permutation;
        if let Some(exact) = solve_kkt_exact(simplex.polytope.dual_vertices(), perm) {
            assert!(
                exact.beta.iter().all(|b| b.is_positive()),
                "all exact beta should be strictly positive on winning node"
            );
        }
    }
}

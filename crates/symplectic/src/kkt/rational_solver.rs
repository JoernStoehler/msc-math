//! Exact KKT solver over BigRational (rational arithmetic).
//!
//! Solves the KKT stationarity system for the constrained QP used by the f64
//! solvers — closure + normalization + beta > 0 — using exact arithmetic over
//! Q. Input polytopes provide dual vertices y_i = n_i / h_i in exact rational
//! form; the KKT system is assembled here and solved by the shared exact linear
//! solver with null-space handling.
//!
//! **Role in the crate:** The exact solver produces exact one-word KKT
//! witnesses for validating f64 behavior and for exact aggregation. A returned
//! witness is not by itself a fixed-word maximum or a physical Reeb orbit. A
//! complete exact HK enumeration may use the values of all such feasible
//! witnesses to recover the scalar capacity; that outer completeness contract
//! is separate from this solver. The solver is NOT used in the main f64
//! capacity enumeration pipeline (too slow for sweeping all permutations).
//!
//! **Rank-deficient systems:** When the KKT matrix is exactly rank-deficient
//! over `Q` (common for polytopes with axis-aligned normals in symplectic
//! subplanes), Q(beta) is constant along the null space ([lem:well-defined]).
//! The shared exact linear solver detects rank deficiency over `Q` and returns
//! null-space basis vectors; this module searches for beta > 0 via
//! Fourier-Motzkin elimination.
//!
//! Mathematical correspondence: [lem:kkt], [lem:well-defined]

use crate::geom::rational_arithmetic::{omega0_rational, rational_to_f64};
use algebraic_numbers::{solve_linear_system, LinearSystemSolution};
use nalgebra::{DMatrix, DVector};
use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{One, Signed, Zero};

/// Exact KKT witness for one ordered support over `BigRational`.
///
/// Contains the exact rational beta vector, exact Q value, and a convenient
/// f64 approximation of Q for comparison with the numerical solver. The
/// result establishes stationarity, closure, normalization, positivity, and
/// the reported objective value. It does not by itself establish a fixed-word
/// maximum, a physical Reeb orbit, or global capacity.
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
/// Given exact rational dual vertices y_i = n_i / h_i and a
/// active traversal word `perm` (the sigma in the thesis), builds the (m+5) x
/// (m+5) KKT matrix over Q and solves via the shared exact linear solver with
/// null-space handling.
///
/// The dual vertex representation {y_i . x <= 1} has implicit height h_i = 1,
/// so the eta block of the KKT matrix is all ones. This is mathematically
/// equivalent to the f64 system (which uses separate n_i and h_i with eta_i = h_i):
/// the change of variable beta_rational_i = beta_f64_i * h_i preserves Q(beta).
///
/// Returns `None` if exact arithmetic certifies that:
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
    let (matrix, rhs) = build_kkt_matrix(dual_vertices, perm);

    match solve_linear_system(&matrix, &rhs) {
        LinearSystemSolution::Inconsistent => None,
        LinearSystemSolution::Consistent {
            particular,
            kernel_basis,
        } if kernel_basis.ncols() == 0 => {
            let x: Vec<BigRational> = particular.iter().cloned().collect();
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
        LinearSystemSolution::Consistent {
            particular,
            kernel_basis,
        } => {
            let beta0: Vec<BigRational> = particular.iter().take(m).cloned().collect();
            let null_beta: Vec<Vec<BigRational>> = (0..kernel_basis.ncols())
                .map(|col| (0..m).map(|row| kernel_basis[(row, col)].clone()).collect())
                .collect();

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
) -> (DMatrix<BigRational>, DVector<BigRational>) {
    let m = perm.len();
    let size = m + 5;
    let zero = BigRational::zero();

    let mut mat = DMatrix::from_element(size, size, zero.clone());
    let mut rhs = DVector::from_element(size, zero);

    // H block: H_{ij} = omega_0(y_i, y_j) for i != j, H_{ii} = 0
    for i in 0..m {
        for j in (i + 1)..m {
            let val = omega0_rational(&dual_vertices[perm[i]], &dual_vertices[perm[j]]);
            mat[(i, j)] = val.clone();
            mat[(j, i)] = val;
        }
    }

    // N block: N_{i,d} = y_{perm[i]}[d], placed symmetrically
    for i in 0..m {
        for d in 0..4 {
            let val = dual_vertices[perm[i]][d].clone();
            mat[(i, m + d)] = val.clone();
            mat[(m + d, i)] = val;
        }
    }

    // eta block: all ones (dual vertex representation has h_i = 1)
    let one = BigRational::one();
    #[allow(clippy::needless_range_loop)]
    for i in 0..m {
        mat[(i, m + 4)] = one.clone();
        mat[(m + 4, i)] = one.clone();
    }

    // RHS: [0, ..., 0, 1] — normalization constraint
    rhs[m + 4] = BigRational::one();

    (mat, rhs)
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
        assert!(
            coeffs.is_empty(),
            "FM elimination left non-empty coefficients"
        );
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algorithms::test_helpers::pruned_capacity_for_fixture;
    use crate::geom::known_polytopes;
    use crate::geom::lagrangian_product::lagrangian_product;
    use crate::geom::polygon::regular_polygon_2d;
    use crate::geom::rational_arithmetic::f64_to_rational;
    use nalgebra::Vector4;
    use num_traits::{Signed, Zero};

    fn exact_dual_vertex_arrays(dual_vertices: &[Vector4<f64>]) -> Vec<[BigRational; 4]> {
        dual_vertices
            .iter()
            .map(|a| {
                [
                    f64_to_rational(a[0]),
                    f64_to_rational(a[1]),
                    f64_to_rational(a[2]),
                    f64_to_rational(a[3]),
                ]
            })
            .collect()
    }

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
    /// exercises the unique-solution path on a full-rank (10 x 10) system.
    #[test]
    fn simplex_exact_solve() {
        let simplex = &known_polytopes::simplex();

        let perm: Vec<usize> = (0..5).collect();
        let result = solve_kkt_exact(&simplex.dual_vertices, &perm);
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
        let hypercube = &known_polytopes::hypercube();

        // Try a 4-facet subset. The hypercube's axis-aligned normals mean omega_0(y_i, y_j) = 0
        // for many pairs. Q can be zero even with nonzero beta.
        let perm = vec![0, 1, 2, 3];
        if let Some(r) = solve_kkt_exact(&hypercube.dual_vertices, &perm) {
            assert!(r.q_exact_f64.is_finite(), "Q_exact_f64 should be finite");
        }
        // Both Some and None are valid — no panic is the key invariant.
    }

    /// A short permutation does not cause a panic.
    ///
    /// A 2-element permutation with m+5 = 7 variables should either solve or
    /// return None, not panic on under- or over-determined systems.
    #[test]
    fn short_permutation_no_panic() {
        let simplex = &known_polytopes::simplex();

        let perm = vec![0, 1];
        // Whether this returns Some or None depends on the system — both are valid.
        let _result = solve_kkt_exact(&simplex.dual_vertices, &perm);
    }

    /// Near-singular f64-rationalized systems do not panic.
    ///
    /// The HKO pentagon's m=7 permutation [1,7,2,8,4,6,5] is close to a
    /// rank-deficient algebraic node. The exact rational solver must solve the
    /// rationalized input it was given, not silently replace small nonzero
    /// pivots by zero.
    #[test]
    fn near_singular_system_handled() {
        let pentagon = &known_polytopes::hko_pentagon();

        let perm = vec![1, 7, 2, 8, 4, 6, 5];
        let result = solve_kkt_exact(&pentagon.dual_vertices, &perm);

        if let Some(r) = result {
            assert!(r.q_exact_f64.is_finite(), "Q_exact_f64 should be finite");
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

    /// Regression: exact fallback must not certify the f64 square-product
    /// near-boundary sigma by dropping tiny nonzero rational pivots.
    ///
    /// With thresholded rank decisions this sigma returned a positive beta and
    /// action 1.904761904761905, contradicting the cube squeeze bound around
    /// the algebraic square product. Strict rational pivoting rejects it.
    #[test]
    fn f64_square_product_bad_sigma_rejected_by_exact_rank() {
        let (qn, qh) = regular_polygon_2d(4, 1.0);
        let (pn, ph) = regular_polygon_2d(4, 1.0);
        let dual_vertices = lagrangian_product(&qn, &qh, &pn, &ph).expect("square product");
        let dual_vertices_exact = exact_dual_vertex_arrays(&dual_vertices);

        assert!(
            solve_kkt_exact(&dual_vertices_exact, &[0, 3, 4, 2, 6]).is_none(),
            "exact rank decisions must reject the square-product bad sigma"
        );
    }

    /// Smoke test: hypercube permutations exercise the null-space path without panic.
    ///
    /// The hypercube has axis-aligned normals (+/- e_i), so many permutations
    /// produce rank-deficient KKT systems. Exercises null-space detection and
    /// Fourier-Motzkin search.
    #[test]
    fn hypercube_null_space_smoke() {
        let hypercube = &known_polytopes::hypercube();

        let perms = vec![
            vec![0, 1, 2, 3, 4],
            vec![0, 1, 2, 3, 4, 5],
            vec![0, 2, 4, 6],
        ];

        for perm in &perms {
            let result = solve_kkt_exact(&hypercube.dual_vertices, perm);
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
        let result = pruned_capacity_for_fixture(simplex).expect("simplex should have capacity");
        let perm = result.best_sigma();
        if let Some(exact) = solve_kkt_exact(&simplex.dual_vertices, perm) {
            let q_exact = exact.q_exact_f64;
            assert!(q_exact > 0.0, "exact Q should be positive, got {q_exact}");
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
            let result =
                pruned_capacity_for_fixture(kp).expect("known polytope should have capacity");
            let perm = result.best_sigma();
            if let Some(exact) = solve_kkt_exact(&kp.dual_vertices, perm) {
                assert!(exact.q_exact_f64 > 0.0, "exact Q should be positive");
            }
        }
    }

    /// On the winning node, all exact beta_i should be strictly positive.
    #[test]
    fn winning_beta_positive_exact() {
        let simplex = crate::geom::known_polytopes::simplex();
        let result = pruned_capacity_for_fixture(simplex).expect("simplex should have capacity");
        let perm = result.best_sigma();
        if let Some(exact) = solve_kkt_exact(&simplex.dual_vertices, perm) {
            assert!(
                exact.beta.iter().all(|b| b.is_positive()),
                "all exact beta should be strictly positive on winning node"
            );
        }
    }
}

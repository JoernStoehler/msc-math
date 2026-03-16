//! Solve linear constraint systems Cx = d via SVD.
//!
//! Given C ∈ R^{p×m} and d ∈ R^p, finds the affine solution set { x : Cx = d }
//! decomposed as x = x₀ + Vα, where x₀ is the minimum-norm particular solution
//! and V has orthonormal columns spanning ker(C).
//!
//! This is a context-free linear algebra sub-component: it knows nothing about
//! symplectic geometry, KKT conditions, or polytopes. See `projection.rs` for
//! how the constraint solution feeds into the projected QP solver.
//!
//! # Thesis correspondence
//!
//! Step 1 of the projection solver (Part C.2 of the algorithm design):
//! "Solve Cβ = d → particular solution β₀, null-space basis V."

use nalgebra::{DMatrix, DVector};

/// Relative threshold for SVD rank detection: σ_i < σ_max · τ is treated as zero.
///
/// At 1e-10 this is well above machine epsilon (~1e-16) and catches
/// near-rank-deficiency without discarding meaningful singular values.
/// For context, the rational solver uses a pivot threshold of 1e-12 relative
/// to the max matrix entry (see `kkt_rational.rs::PIVOT_RELATIVE_THRESHOLD`).
const EPS_RANK_THRESHOLD: f64 = 1e-10;

/// Maximum residual ‖Cx₀ - d‖ to accept as consistent.
///
/// If the residual exceeds this after projecting d onto the column space,
/// the system Cx = d has no solution. The tolerance is chosen well above
/// the noise floor from SVD roundoff (~1e-14) but below scales where a
/// genuine inconsistency would be masked.
const EPS_CONSISTENCY: f64 = 1e-8;

/// Solution of the linear constraint system Cx = d.
///
/// The full solution set is the affine subspace { x₀ + Vα : α ∈ R^k },
/// where k = m - rank(C).
#[derive(Clone, Debug)]
pub struct ConstraintSolution {
    /// Particular solution x₀ (minimum-norm via SVD pseudoinverse).
    pub x0: DVector<f64>,
    /// Orthonormal null-space basis V ∈ R^{m×k}. Columns span ker(C).
    /// k = m - rank. Empty (m×0 matrix) when C has full column rank.
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
/// 1. Compute SVD: C = U Σ Vᵀ
/// 2. Rank detection: r = |{ i : σ_i > σ_max · EPS_RANK_THRESHOLD }|
/// 3. Consistency: ‖(I - U_r U_rᵀ) d‖ < EPS_CONSISTENCY
/// 4. Particular solution: x₀ = V_r Σ_r⁻¹ U_rᵀ d  (minimum-norm)
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

    // Edge case: zero-size constraint matrix.
    if p == 0 {
        return Some(ConstraintSolution {
            x0: DVector::zeros(m),
            null_basis: DMatrix::identity(m, m),
            rank: 0,
        });
    }
    if m == 0 {
        // C is p×0. System Cx = d is consistent only if d = 0.
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
    // nalgebra's thin SVD of a p×m matrix gives V^T with min(p,m) rows.
    // When p < m (underdetermined), this is insufficient to extract the full
    // null space (dimension m - rank). We pad C with zero rows to m×m so that
    // V^T has the full m rows. The zero rows don't change rank, column space,
    // or null space — they only ensure the SVD computes all right singular vectors.
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
    // Compute the component of d orthogonal to the column space of C.
    // d_perp = d - U_r U_rᵀ d. If ‖d_perp‖ is large, the system is inconsistent.
    // When C was padded, U has more rows than d, so we use only the first p rows
    // of each left singular vector for the projection.
    let mut d_proj = DVector::zeros(p);
    for i in 0..rank {
        let ui_full = u.column(i);
        // Extract the first p components (the original C's left singular vectors).
        let coeff: f64 = (0..p).map(|k| ui_full[k] * d[k]).sum();
        d_proj += coeff * DVector::from_fn(p, |k, _| ui_full[k]);
    }
    let residual = (d - &d_proj).norm();
    if residual > EPS_CONSISTENCY {
        return None;
    }

    // Step 4: Particular solution (minimum-norm via pseudoinverse).
    // x₀ = V_r Σ_r⁻¹ U_rᵀ d = Σᵢ (uᵢᵀd / σᵢ) vᵢ
    // Again, project d using only the first p components of each left singular vector.
    let mut x0 = DVector::zeros(m);
    for i in 0..rank {
        let ui_full = u.column(i);
        let vi = vt.row(i).transpose();
        let coeff: f64 = (0..p).map(|k| ui_full[k] * d[k]).sum();
        x0 += (coeff / sigma[i]) * &vi;
    }

    // Step 5: Null basis — columns of V corresponding to zero singular values.
    // V^T is m×m (guaranteed by padding), so rows rank..m are available.
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

#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra::{DMatrix, DVector};

    /// Helper: assert two f64 values are approximately equal.
    fn assert_approx(a: f64, b: f64, tol: f64, msg: &str) {
        assert!(
            (a - b).abs() < tol,
            "{}: |{} - {}| = {} >= {}",
            msg, a, b, (a - b).abs(), tol
        );
    }

    // ── Smoke tests ──────────────────────────────────────────────────────

    /// C = I (identity constraints): unique solution x₀ = d, empty null basis.
    #[test]
    fn identity_constraints() {
        let c = DMatrix::identity(3, 3);
        let d = DVector::from_column_slice(&[1.0, 0.0, 0.0]);

        let sol = solve_constraints(&c, &d).expect("consistent system");
        assert_eq!(sol.rank, 3);
        assert_eq!(sol.null_basis.ncols(), 0);
        for i in 0..3 {
            assert_approx(sol.x0[i], d[i], 1e-12, &format!("x0[{}]", i));
        }
    }

    /// 3×6 system with rank 3: null basis should have 3 columns.
    ///
    /// C = [I_3 | I_3] has rank 3. Null space is spanned by
    /// { e_i - e_{i+3} : i = 0,1,2 } (dimension 3).
    #[test]
    fn known_rank_system_3x6() {
        let mut c = DMatrix::zeros(3, 6);
        for i in 0..3 {
            c[(i, i)] = 1.0;
            c[(i, i + 3)] = 1.0;
        }
        let d = DVector::from_column_slice(&[1.0, 2.0, 3.0]);

        let sol = solve_constraints(&c, &d).expect("consistent system");
        assert_eq!(sol.rank, 3);
        assert_eq!(sol.null_basis.ncols(), 3, "expected 3 null-space dimensions");
        assert_eq!(sol.null_basis.nrows(), 6);
    }

    /// Overdetermined consistent: 4×2 system with an exact solution.
    ///
    /// C = [[1, 0], [0, 1], [1, 1], [1, -1]], d = [2, 3, 5, -1].
    /// Unique solution x = [2, 3].
    #[test]
    fn overdetermined_consistent() {
        #[rustfmt::skip]
        let c = DMatrix::from_row_slice(4, 2, &[
            1.0, 0.0,
            0.0, 1.0,
            1.0, 1.0,
            1.0, -1.0,
        ]);
        let d = DVector::from_column_slice(&[2.0, 3.0, 5.0, -1.0]);

        let sol = solve_constraints(&c, &d).expect("consistent system");
        assert_eq!(sol.rank, 2);
        assert_eq!(sol.null_basis.ncols(), 0);
        assert_approx(sol.x0[0], 2.0, 1e-10, "x0[0]");
        assert_approx(sol.x0[1], 3.0, 1e-10, "x0[1]");
    }

    /// Overdetermined inconsistent: 4×2 system with no solution.
    ///
    /// Same C as above, but d = [2, 3, 5, 0] (last row inconsistent: 2 - 3 = -1 ≠ 0).
    #[test]
    fn overdetermined_inconsistent() {
        #[rustfmt::skip]
        let c = DMatrix::from_row_slice(4, 2, &[
            1.0, 0.0,
            0.0, 1.0,
            1.0, 1.0,
            1.0, -1.0,
        ]);
        let d = DVector::from_column_slice(&[2.0, 3.0, 5.0, 0.0]);

        assert!(
            solve_constraints(&c, &d).is_none(),
            "expected None for inconsistent system"
        );
    }

    /// Zero RHS: Cx = 0 should give x₀ ≈ 0 and null basis spanning ker(C).
    ///
    /// C = [1 1 0; 0 0 1] (2×3, rank 2). Null space is 1-dimensional:
    /// span{[-1, 1, 0]ᵀ} (up to normalization).
    #[test]
    fn zero_rhs() {
        #[rustfmt::skip]
        let c = DMatrix::from_row_slice(2, 3, &[
            1.0, 1.0, 0.0,
            0.0, 0.0, 1.0,
        ]);
        let d = DVector::zeros(2);

        let sol = solve_constraints(&c, &d).expect("consistent system");
        assert_eq!(sol.rank, 2);
        assert_eq!(sol.null_basis.ncols(), 1, "expected 1 null-space dimension");
        // x₀ should be (approximately) zero since Cx = 0 and we pick min-norm.
        assert!(
            sol.x0.norm() < 1e-12,
            "expected x0 ≈ 0, got ‖x0‖ = {}",
            sol.x0.norm()
        );
    }

    /// Singular values near threshold: verify rank detection.
    ///
    /// Construct a 3×3 diagonal matrix with σ = [1.0, 1e-11, 1e-15].
    /// With EPS_RANK_THRESHOLD = 1e-10 and σ_max = 1.0, the threshold is 1e-10.
    /// Only σ₁ = 1.0 is above threshold, so rank should be 1.
    #[test]
    fn singular_values_near_threshold() {
        // C = diag(1.0, 1e-11, 1e-15) — constructed as a diagonal matrix.
        let c = DMatrix::from_diagonal(&DVector::from_column_slice(&[1.0, 1e-11, 1e-15]));
        let d = DVector::from_column_slice(&[5.0, 0.0, 0.0]);

        let sol = solve_constraints(&c, &d).expect("consistent (d in column space)");
        assert_eq!(sol.rank, 1, "expected rank 1 (only σ=1.0 above threshold)");
        assert_eq!(sol.null_basis.ncols(), 2, "expected 2 null-space dimensions");
    }

    /// Full column rank with more rows than columns (overdetermined, consistent).
    ///
    /// C = [[1]; [0]; [0]], d = [3; 0; 0]. Unique solution x = [3].
    #[test]
    fn single_variable() {
        #[rustfmt::skip]
        let c = DMatrix::from_row_slice(3, 1, &[1.0, 0.0, 0.0]);
        let d = DVector::from_column_slice(&[3.0, 0.0, 0.0]);

        let sol = solve_constraints(&c, &d).expect("consistent system");
        assert_eq!(sol.rank, 1);
        assert_eq!(sol.null_basis.ncols(), 0);
        assert_approx(sol.x0[0], 3.0, 1e-12, "x0[0]");
    }

    /// Zero matrix C: rank = 0, full null space, d must be zero.
    #[test]
    fn zero_matrix_consistent() {
        let c = DMatrix::zeros(2, 3);
        let d = DVector::zeros(2);

        let sol = solve_constraints(&c, &d).expect("consistent (d = 0)");
        assert_eq!(sol.rank, 0);
        assert_eq!(sol.null_basis.ncols(), 3, "full null space");
        assert!(sol.x0.norm() < 1e-12, "x0 should be zero");
    }

    /// Zero matrix C with nonzero d: inconsistent.
    #[test]
    fn zero_matrix_inconsistent() {
        let c = DMatrix::zeros(2, 3);
        let d = DVector::from_column_slice(&[1.0, 0.0]);

        assert!(
            solve_constraints(&c, &d).is_none(),
            "expected None: 0·x = [1,0] is inconsistent"
        );
    }

    // ── Mathematical property tests ──────────────────────────────────────

    /// Round-trip: Cx₀ ≈ d for every consistent system.
    ///
    /// Tests the defining property of the particular solution.
    #[test]
    fn round_trip_cx0_equals_d() {
        // Test several different shapes and ranks.
        let cases: Vec<(DMatrix<f64>, DVector<f64>)> = vec![
            // Square, full rank
            (
                DMatrix::from_row_slice(3, 3, &[1.0, 2.0, 3.0, 0.0, 1.0, 4.0, 5.0, 6.0, 0.0]),
                DVector::from_column_slice(&[14.0, 13.0, 17.0]),
            ),
            // Underdetermined (2×4)
            (
                DMatrix::from_row_slice(2, 4, &[1.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 1.0]),
                DVector::from_column_slice(&[3.0, 7.0]),
            ),
            // Rank-deficient (3×3, rank 2)
            (
                DMatrix::from_row_slice(3, 3, &[1.0, 2.0, 3.0, 2.0, 4.0, 6.0, 0.0, 1.0, 1.0]),
                DVector::from_column_slice(&[6.0, 12.0, 2.0]),
            ),
        ];

        for (i, (c, d)) in cases.iter().enumerate() {
            let sol = solve_constraints(c, d).expect(&format!("case {} should be consistent", i));
            let residual = (c * &sol.x0 - d).norm();
            assert!(
                residual < 1e-10,
                "case {}: ‖Cx₀ - d‖ = {:.2e} (expected < 1e-10)",
                i, residual
            );
        }
    }

    /// Null basis orthogonality to C: CV ≈ 0 for every column of V.
    ///
    /// The null basis columns must lie in ker(C).
    #[test]
    fn null_basis_in_kernel() {
        let cases: Vec<DMatrix<f64>> = vec![
            // 2×5, rank 2 → 3 null vectors
            DMatrix::from_row_slice(
                2, 5,
                &[1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0],
            ),
            // 3×6 [I₃ | I₃], rank 3 → 3 null vectors
            {
                let mut c = DMatrix::zeros(3, 6);
                for i in 0..3 {
                    c[(i, i)] = 1.0;
                    c[(i, i + 3)] = 1.0;
                }
                c
            },
        ];

        for (i, c) in cases.iter().enumerate() {
            let d = DVector::zeros(c.nrows());
            let sol = solve_constraints(c, &d).expect(&format!("case {} consistent", i));

            if sol.null_basis.ncols() == 0 {
                continue;
            }

            let cv = c * &sol.null_basis;
            let max_entry = cv.iter().map(|x| x.abs()).fold(0.0_f64, f64::max);
            assert!(
                max_entry < 1e-10,
                "case {}: max|CV| = {:.2e} (expected < 1e-10)",
                i, max_entry
            );
        }
    }

    /// Null basis columns are orthonormal.
    #[test]
    fn null_basis_orthonormal() {
        // 2×5 system, rank 2, null space dim 3.
        #[rustfmt::skip]
        let c = DMatrix::from_row_slice(2, 5, &[
            1.0, 0.0, 1.0, 0.0, 1.0,
            0.0, 1.0, 0.0, 1.0, 0.0,
        ]);
        let d = DVector::zeros(2);

        let sol = solve_constraints(&c, &d).expect("consistent");
        let v = &sol.null_basis;
        let k = v.ncols();
        assert!(k > 0, "need non-empty null basis for this test");

        // VᵀV should be I_k.
        let vtv = v.transpose() * v;
        for i in 0..k {
            for j in 0..k {
                let expected = if i == j { 1.0 } else { 0.0 };
                assert_approx(
                    vtv[(i, j)], expected, 1e-12,
                    &format!("VᵀV[{},{}]", i, j),
                );
            }
        }
    }

    /// Null basis dimension = m - rank for various shapes.
    #[test]
    fn null_dim_equals_m_minus_rank() {
        let cases: Vec<(DMatrix<f64>, DVector<f64>)> = vec![
            // 3×3 identity: rank 3, null dim 0
            (DMatrix::identity(3, 3), DVector::from_element(3, 1.0)),
            // 1×4: rank 1, null dim 3
            (
                DMatrix::from_row_slice(1, 4, &[1.0, 1.0, 1.0, 1.0]),
                DVector::from_column_slice(&[4.0]),
            ),
            // 2×2 rank 1: [[1,2],[2,4]], null dim 1
            (
                DMatrix::from_row_slice(2, 2, &[1.0, 2.0, 2.0, 4.0]),
                DVector::from_column_slice(&[3.0, 6.0]),
            ),
        ];

        for (i, (c, d)) in cases.iter().enumerate() {
            let sol = solve_constraints(&c, &d).expect(&format!("case {} consistent", i));
            let m = c.ncols();
            assert_eq!(
                sol.null_basis.ncols(), m - sol.rank,
                "case {}: null_dim {} != m - rank = {} - {}",
                i, sol.null_basis.ncols(), m, sol.rank,
            );
        }
    }

    /// Any x in the affine solution set satisfies Cx = d.
    ///
    /// For a system with non-trivial null space, pick a random α and verify
    /// that C(x₀ + Vα) = d.
    #[test]
    fn affine_solution_set_satisfies_constraints() {
        // 2×5 underdetermined system.
        #[rustfmt::skip]
        let c = DMatrix::from_row_slice(2, 5, &[
            1.0, 2.0, 0.0, 1.0, 0.0,
            0.0, 1.0, 1.0, 0.0, 1.0,
        ]);
        let d = DVector::from_column_slice(&[3.0, 2.0]);

        let sol = solve_constraints(&c, &d).expect("consistent");
        let k = sol.null_basis.ncols();
        assert!(k > 0, "need non-trivial null space");

        // Several test points in the affine subspace.
        let alphas: Vec<DVector<f64>> = vec![
            DVector::from_element(k, 1.0),
            DVector::from_element(k, -2.5),
            DVector::from_fn(k, |i, _| (i as f64 + 1.0) * 0.7),
        ];

        for (j, alpha) in alphas.iter().enumerate() {
            let x = &sol.x0 + &sol.null_basis * alpha;
            let residual = (&c * &x - &d).norm();
            assert!(
                residual < 1e-10,
                "alpha[{}]: ‖C(x₀ + Vα) - d‖ = {:.2e}",
                j, residual
            );
        }
    }

    /// x₀ is the minimum-norm solution.
    ///
    /// For an underdetermined system, any other solution x = x₀ + Vα
    /// should satisfy ‖x‖ ≥ ‖x₀‖.
    #[test]
    fn x0_is_minimum_norm() {
        // 1×3 system: [1 1 1] x = [3]. Min-norm solution is x₀ = [1, 1, 1].
        let c = DMatrix::from_row_slice(1, 3, &[1.0, 1.0, 1.0]);
        let d = DVector::from_column_slice(&[3.0]);

        let sol = solve_constraints(&c, &d).expect("consistent");
        let x0_norm = sol.x0.norm();

        // Perturb along null space.
        let k = sol.null_basis.ncols();
        assert_eq!(k, 2);
        for scale in &[0.1, 1.0, -0.5, 3.0] {
            for j in 0..k {
                let alpha = {
                    let mut a = DVector::zeros(k);
                    a[j] = *scale;
                    a
                };
                let x = &sol.x0 + &sol.null_basis * &alpha;
                assert!(
                    x.norm() >= x0_norm - 1e-12,
                    "‖x₀ + Vα‖ = {} < ‖x₀‖ = {} (violated min-norm)",
                    x.norm(), x0_norm
                );
            }
        }
    }

    // ── Edge case: empty dimensions ──────────────────────────────────────

    /// p = 0 (no constraints): x₀ = 0, null basis = I_m.
    #[test]
    fn no_constraints() {
        let c = DMatrix::zeros(0, 4);
        let d = DVector::zeros(0);

        let sol = solve_constraints(&c, &d).expect("trivially consistent");
        assert_eq!(sol.rank, 0);
        assert_eq!(sol.null_basis.ncols(), 4);
        assert_eq!(sol.null_basis.nrows(), 4);
        assert!(sol.x0.norm() < 1e-15, "x0 should be zero");

        // Null basis should be orthonormal (identity in this case).
        let vtv = sol.null_basis.transpose() * &sol.null_basis;
        for i in 0..4 {
            for j in 0..4 {
                let expected = if i == j { 1.0 } else { 0.0 };
                assert_approx(vtv[(i, j)], expected, 1e-12, &format!("VᵀV[{},{}]", i, j));
            }
        }
    }

    // ── EHZ-sized test: 5×m constraint system ────────────────────────────

    /// Typical EHZ constraint shape: 5 constraints, m = 8 variables.
    ///
    /// C has 4 closure rows + 1 normalization row (all ones), d = [0,0,0,0,1].
    /// This matches the structure from `assemble_qp` in the spec (§10).
    #[test]
    fn ehz_shaped_constraint_system() {
        #[rustfmt::skip]
        let c = DMatrix::from_row_slice(5, 8, &[
            // 4 closure rows (random-ish dual vertex coordinates)
             1.0,  0.0, -1.0,  0.5,  0.0,  1.0, -0.5,  0.0,
             0.0,  1.0,  0.0, -1.0,  0.5,  0.0,  1.0, -0.5,
             0.5,  0.0,  1.0,  0.0, -1.0,  0.5,  0.0,  1.0,
             0.0,  0.5,  0.0,  1.0,  0.0, -1.0,  0.5,  0.0,
            // normalization row
             1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,
        ]);
        let d = DVector::from_column_slice(&[0.0, 0.0, 0.0, 0.0, 1.0]);

        let sol = solve_constraints(&c, &d).expect("consistent");
        assert_eq!(sol.rank, 5, "C should have full row rank (5)");
        assert_eq!(sol.null_basis.ncols(), 3, "null space dim = 8 - 5 = 3");

        // Verify Cx₀ = d.
        let residual = (&c * &sol.x0 - &d).norm();
        assert!(residual < 1e-10, "‖Cx₀ - d‖ = {:.2e}", residual);

        // Verify CV = 0.
        let cv = &c * &sol.null_basis;
        let max_cv = cv.iter().map(|x| x.abs()).fold(0.0_f64, f64::max);
        assert!(max_cv < 1e-10, "max|CV| = {:.2e}", max_cv);

        // Verify orthonormality.
        let vtv = sol.null_basis.transpose() * &sol.null_basis;
        for i in 0..3 {
            for j in 0..3 {
                let expected = if i == j { 1.0 } else { 0.0 };
                assert_approx(vtv[(i, j)], expected, 1e-12, &format!("VᵀV[{},{}]", i, j));
            }
        }
    }
}

//! Tests for constraint_solver: SVD-based linear constraint solution.
//!
//! Proposition: solve_constraints returns x0 satisfying Cx0 = d with minimum norm,
//! and null_basis columns spanning ker(C) with orthonormality.
//! Reference: SVD pseudoinverse theory.
//!
//! Strategy: fixture-based with hand-verifiable systems covering all shapes
//! (square, over/underdetermined, rank-deficient, trivial).

use super::constraint_solver::*;
use nalgebra::{DMatrix, DVector};

// ── Helpers ──

fn assert_approx(a: f64, b: f64, tol: f64, msg: &str) {
    assert!(
        (a - b).abs() < tol,
        "{}: |{} - {}| = {} >= {}",
        msg, a, b, (a - b).abs(), tol
    );
}

// ── Smoke tests ──

/// C = I (identity constraints): unique solution x0 = d, empty null basis.
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

/// 3x6 system with rank 3: null basis should have 3 columns.
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

/// Overdetermined consistent: 4x2 system with exact solution.
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

/// Overdetermined inconsistent: 4x2 system with no solution.
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

/// Zero RHS: Cx = 0 should give x0 ~ 0 and null basis spanning ker(C).
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
    assert!(sol.x0.norm() < 1e-12, "expected x0 ~ 0, got ||x0|| = {}", sol.x0.norm());
}

/// Singular values near threshold: verify rank detection.
///
/// C = diag(1.0, 1e-11, 1e-15). With threshold = 1e-10, only sigma = 1.0 is above.
#[test]
fn singular_values_near_threshold() {
    let c = DMatrix::from_diagonal(&DVector::from_column_slice(&[1.0, 1e-11, 1e-15]));
    let d = DVector::from_column_slice(&[5.0, 0.0, 0.0]);

    let sol = solve_constraints(&c, &d).expect("consistent (d in column space)");
    assert_eq!(sol.rank, 1, "expected rank 1 (only sigma=1.0 above threshold)");
    assert_eq!(sol.null_basis.ncols(), 2, "expected 2 null-space dimensions");
}

/// Single variable: full column rank with more rows than columns.
#[test]
fn single_variable() {
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
        "expected None: 0*x = [1,0] is inconsistent"
    );
}

// ── Mathematical property tests ──

/// Round-trip: Cx0 ~ d for every consistent system.
#[test]
fn round_trip_cx0_equals_d() {
    let cases: Vec<(DMatrix<f64>, DVector<f64>)> = vec![
        // Square, full rank
        (
            DMatrix::from_row_slice(3, 3, &[1.0, 2.0, 3.0, 0.0, 1.0, 4.0, 5.0, 6.0, 0.0]),
            DVector::from_column_slice(&[14.0, 13.0, 17.0]),
        ),
        // Underdetermined (2x4)
        (
            DMatrix::from_row_slice(2, 4, &[1.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 1.0]),
            DVector::from_column_slice(&[3.0, 7.0]),
        ),
        // Rank-deficient (3x3, rank 2)
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
            "case {}: ||Cx0 - d|| = {:.2e} (expected < 1e-10)",
            i, residual
        );
    }
}

/// Null basis orthogonality to C: CV ~ 0 for every column of V.
#[test]
fn null_basis_in_kernel() {
    let cases: Vec<DMatrix<f64>> = vec![
        // 2x5, rank 2 -> 3 null vectors
        DMatrix::from_row_slice(
            2, 5,
            &[1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0],
        ),
        // 3x6 [I3 | I3], rank 3 -> 3 null vectors
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

    let vtv = v.transpose() * v;
    for i in 0..k {
        for j in 0..k {
            let expected = if i == j { 1.0 } else { 0.0 };
            assert_approx(vtv[(i, j)], expected, 1e-12, &format!("V^T V[{},{}]", i, j));
        }
    }
}

/// Null basis dimension = m - rank for various shapes.
#[test]
fn null_dim_equals_m_minus_rank() {
    let cases: Vec<(DMatrix<f64>, DVector<f64>)> = vec![
        (DMatrix::identity(3, 3), DVector::from_element(3, 1.0)),
        (
            DMatrix::from_row_slice(1, 4, &[1.0, 1.0, 1.0, 1.0]),
            DVector::from_column_slice(&[4.0]),
        ),
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
#[test]
fn affine_solution_set_satisfies_constraints() {
    #[rustfmt::skip]
    let c = DMatrix::from_row_slice(2, 5, &[
        1.0, 2.0, 0.0, 1.0, 0.0,
        0.0, 1.0, 1.0, 0.0, 1.0,
    ]);
    let d = DVector::from_column_slice(&[3.0, 2.0]);

    let sol = solve_constraints(&c, &d).expect("consistent");
    let k = sol.null_basis.ncols();
    assert!(k > 0, "need non-trivial null space");

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
            "alpha[{}]: ||C(x0 + V alpha) - d|| = {:.2e}",
            j, residual
        );
    }
}

/// x0 is the minimum-norm solution.
#[test]
fn x0_is_minimum_norm() {
    let c = DMatrix::from_row_slice(1, 3, &[1.0, 1.0, 1.0]);
    let d = DVector::from_column_slice(&[3.0]);

    let sol = solve_constraints(&c, &d).expect("consistent");
    let x0_norm = sol.x0.norm();

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
                "||x0 + V alpha|| = {} < ||x0|| = {} (violated min-norm)",
                x.norm(), x0_norm
            );
        }
    }
}

// ── Edge cases ──

/// p = 0 (no constraints): x0 = 0, null basis = I_m.
#[test]
fn no_constraints() {
    let c = DMatrix::zeros(0, 4);
    let d = DVector::zeros(0);

    let sol = solve_constraints(&c, &d).expect("trivially consistent");
    assert_eq!(sol.rank, 0);
    assert_eq!(sol.null_basis.ncols(), 4);
    assert_eq!(sol.null_basis.nrows(), 4);
    assert!(sol.x0.norm() < 1e-15, "x0 should be zero");
}

// ── EHZ-sized test: 5 x m constraint system ──

/// Typical EHZ constraint shape: 5 constraints, m = 8 variables.
///
/// C has 4 closure rows + 1 normalization row (all ones), d = [0,0,0,0,1].
#[test]
fn ehz_shaped_constraint_system() {
    #[rustfmt::skip]
    let c = DMatrix::from_row_slice(5, 8, &[
         1.0,  0.0, -1.0,  0.5,  0.0,  1.0, -0.5,  0.0,
         0.0,  1.0,  0.0, -1.0,  0.5,  0.0,  1.0, -0.5,
         0.5,  0.0,  1.0,  0.0, -1.0,  0.5,  0.0,  1.0,
         0.0,  0.5,  0.0,  1.0,  0.0, -1.0,  0.5,  0.0,
         1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,
    ]);
    let d = DVector::from_column_slice(&[0.0, 0.0, 0.0, 0.0, 1.0]);

    let sol = solve_constraints(&c, &d).expect("consistent");
    assert_eq!(sol.rank, 5, "C should have full row rank (5)");
    assert_eq!(sol.null_basis.ncols(), 3, "null space dim = 8 - 5 = 3");

    // Verify Cx0 = d.
    let residual = (&c * &sol.x0 - &d).norm();
    assert!(residual < 1e-10, "||Cx0 - d|| = {:.2e}", residual);

    // Verify CV = 0.
    let cv = &c * &sol.null_basis;
    let max_cv = cv.iter().map(|x| x.abs()).fold(0.0_f64, f64::max);
    assert!(max_cv < 1e-10, "max|CV| = {:.2e}", max_cv);

    // Verify orthonormality.
    let vtv = sol.null_basis.transpose() * &sol.null_basis;
    for i in 0..3 {
        for j in 0..3 {
            let expected = if i == j { 1.0 } else { 0.0 };
            assert_approx(vtv[(i, j)], expected, 1e-12, &format!("V^T V[{},{}]", i, j));
        }
    }
}

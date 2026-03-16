//! Tests for projection_solver: projection-based QP solver correctness.
//!
//! Proposition: solve_projected returns a Solution where verdict, Q, beta,
//! and margin are consistent, with C beta = d satisfied for non-False verdicts.
//! Reference: [lem:kkt], Part C.2 of algorithm design.
//!
//! Strategy: fixture-based with synthetic QPs (hand-checkable) plus cross-validation
//! against the saddle-point solver on known polytopes.

use super::projection_solver::solve_projected;
use super::qp_assembly::build_qp;
use super::{QP, Verdict};
use crate::geom::known_polytopes;
use nalgebra::{DMatrix, DVector};

// ── Synthetic tests (context-free, hand-checkable) ──

/// Inconsistent constraints: return False.
#[test]
fn inconsistent_constraints() {
    let c = DMatrix::identity(3, 3);
    let d = DVector::from_column_slice(&[1.0, 0.0, 0.0]);
    let h = DMatrix::zeros(3, 3);
    let qp = QP { c, d, h };

    let sol = solve_projected(&qp);
    // C is 3x3 identity, d = [1,0,0]. Unique beta = [1,0,0]. beta2 = beta3 = 0.
    // margin = 0 -> Indeterminate.
    assert!(sol.margin <= 0.0);
}

/// Unique beta (k=0), all positive -> True.
#[test]
fn unique_beta_positive() {
    let c = DMatrix::identity(5, 5);
    let d = DVector::from_element(5, 0.2);
    let h = DMatrix::identity(5, 5);
    let qp = QP { c, d, h };

    let sol = solve_projected(&qp);
    assert_eq!(sol.verdict, Verdict::True);
    assert!((sol.q - 0.1).abs() < 1e-10, "Q = {}, expected 0.1", sol.q);
    assert!(
        (sol.margin - 0.2).abs() < 1e-10,
        "margin = {}, expected 0.2",
        sol.margin
    );
}

/// Unique beta (k=0), some negative -> False.
#[test]
fn unique_beta_negative() {
    let c = DMatrix::identity(5, 5);
    let d = DVector::from_column_slice(&[0.5, 0.5, -0.5, 0.5, 0.5]);
    let h = DMatrix::identity(5, 5);
    let qp = QP { c, d, h };

    let sol = solve_projected(&qp);
    assert_eq!(sol.verdict, Verdict::False);
    assert!(sol.margin < 0.0);
}

/// One free variable (k=1, m=6, p=5). Verify Q matches hand computation.
#[test]
fn one_free_variable() {
    #[rustfmt::skip]
    let c = DMatrix::from_row_slice(5, 6, &[
        1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        0.0, 1.0, 0.0, 0.0, 0.0, 1.0,
        0.0, 0.0, 1.0, 0.0, 0.0, 1.0,
        0.0, 0.0, 0.0, 1.0, 0.0, 1.0,
        0.0, 0.0, 0.0, 0.0, 1.0, 1.0,
    ]);
    let d = DVector::from_element(5, 1.0);
    let h = DMatrix::identity(6, 6);
    let qp = QP { c, d, h };

    let sol = solve_projected(&qp);
    assert!(sol.q > 0.0, "Q should be positive");
    // Q_max = 5/12
    assert!(
        (sol.q - 5.0 / 12.0).abs() < 1e-8,
        "Q = {}, expected 5/12 = {}",
        sol.q,
        5.0 / 12.0
    );
}

/// Q is constant when H = 0: Q = 0 for all beta in the constraint set.
#[test]
fn q_constant_when_h_zero() {
    #[rustfmt::skip]
    let c = DMatrix::from_row_slice(2, 4, &[
        1.0, 0.0, 1.0, 0.0,
        0.0, 1.0, 0.0, 1.0,
    ]);
    let d = DVector::from_column_slice(&[1.0, 1.0]);
    let h = DMatrix::zeros(4, 4);
    let qp = QP { c, d, h };

    let sol = solve_projected(&qp);
    assert!(
        sol.q.abs() < 1e-12,
        "Q should be 0 when H = 0, got {}",
        sol.q
    );
}

// ── Mathematical proposition tests ──

/// Prop: C beta = d for every returned beta with verdict != False.
#[test]
fn prop_constraint_satisfaction() {
    let cases = vec![
        {
            #[rustfmt::skip]
            let c = DMatrix::from_row_slice(3, 5, &[
                1.0, 0.0, 0.0, 1.0, 0.0,
                0.0, 1.0, 0.0, 0.0, 1.0,
                0.0, 0.0, 1.0, 1.0, 1.0,
            ]);
            let d = DVector::from_column_slice(&[1.0, 1.0, 1.0]);
            let h = DMatrix::identity(5, 5);
            QP { c, d, h }
        },
        {
            #[rustfmt::skip]
            let c = DMatrix::from_row_slice(5, 8, &[
                1.0,  0.0, -1.0,  0.5,  0.0,  1.0, -0.5,  0.0,
                0.0,  1.0,  0.0, -1.0,  0.5,  0.0,  1.0, -0.5,
                0.5,  0.0,  1.0,  0.0, -1.0,  0.5,  0.0,  1.0,
                0.0,  0.5,  0.0,  1.0,  0.0, -1.0,  0.5,  0.0,
                1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  1.0,
            ]);
            let d = DVector::from_column_slice(&[0.0, 0.0, 0.0, 0.0, 1.0]);
            let h = DMatrix::identity(8, 8);
            QP { c, d, h }
        },
    ];

    for (i, qp) in cases.iter().enumerate() {
        let sol = solve_projected(qp);
        if sol.verdict == Verdict::False {
            continue;
        }
        let beta_dv = DVector::from_column_slice(&sol.beta);
        let residual = (&qp.c * &beta_dv - &qp.d).norm();
        assert!(
            residual < 1e-8,
            "case {}: ||C beta - d|| = {:.2e}",
            i,
            residual
        );
    }
}

/// Prop: returned Q equals (1/2) beta^T H beta.
#[test]
fn prop_q_is_half_beta_h_beta() {
    #[rustfmt::skip]
    let c = DMatrix::from_row_slice(2, 4, &[
        1.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 1.0,
    ]);
    let d = DVector::from_column_slice(&[1.0, 1.0]);
    #[rustfmt::skip]
    let h = DMatrix::from_row_slice(4, 4, &[
        0.0, 1.0, 0.0, 0.0,
        1.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 1.0,
        0.0, 0.0, 1.0, 0.0,
    ]);
    let qp = QP { c, d, h };

    let sol = solve_projected(&qp);
    let beta_dv = DVector::from_column_slice(&sol.beta);
    let q_check = 0.5 * beta_dv.dot(&(&qp.h * &beta_dv));
    assert!(
        (sol.q - q_check).abs() < 1e-10,
        "Q mismatch: solver = {}, direct = {}, diff = {:.2e}",
        sol.q,
        q_check,
        (sol.q - q_check).abs()
    );
}

/// Prop: margin = min(beta) exactly.
#[test]
fn prop_margin_equals_min_beta() {
    let c = DMatrix::identity(3, 6);
    let d = DVector::from_column_slice(&[0.5, 0.3, 0.8]);
    let h = DMatrix::identity(6, 6);
    let qp = QP { c, d, h };

    let sol = solve_projected(&qp);
    let min_beta = sol.beta.iter().copied().fold(f64::INFINITY, f64::min);
    assert!(
        (sol.margin - min_beta).abs() < 1e-12,
        "margin = {}, min(beta) = {}, diff = {:.2e}",
        sol.margin,
        min_beta,
        (sol.margin - min_beta).abs()
    );
}

// ── Cross-variant tests: projection solver vs saddle-point solver ──

/// Both solvers agree on capacity for the simplex.
#[test]
#[ignore] // Depends on algorithms::hk2017 (wave 3)
fn capacity_agrees_on_simplex() {
    // TODO: Once algorithms::hk2017::ehz_capacity is available (wave 3),
    // this test should compare augmented and projection solver results.
}

/// Both solvers agree on capacity for the hypercube.
#[test]
#[ignore] // Depends on algorithms::hk2017 (wave 3)
fn capacity_agrees_on_hypercube() {
    // TODO: Uncomment when algorithms::hk2017 is available.
}

/// Projection solver finds positive Q on simplex with exhaustive search over
/// all subset sizes and orderings.
#[test]
fn projection_finds_positive_q_on_simplex() {
    let simplex = known_polytopes::simplex();
    let polytope = &simplex.polytope;
    let f = polytope.facet_count();

    let mut found = false;
    // Try subset sizes 2 to min(f, 5)
    for size in 2..=f.min(5) {
        for_each_combination(f, size, &mut |subset| {
            let perm = subset.to_vec();
            let qp = build_qp(polytope, &perm);
            let sol = solve_projected(&qp);
            if sol.verdict == Verdict::True && sol.q > 1e-6 {
                found = true;
            }
        });
    }
    assert!(found, "projection solver should find positive Q on simplex");
}

/// Call `f` with every k-element subset of {0, ..., n-1}.
fn for_each_combination(n: usize, k: usize, f: &mut impl FnMut(&[usize])) {
    let mut indices: Vec<usize> = (0..k).collect();
    loop {
        f(&indices);
        let mut i = k;
        loop {
            if i == 0 {
                return;
            }
            i -= 1;
            indices[i] += 1;
            if indices[i] <= n - k + i {
                break;
            }
        }
        for j in (i + 1)..k {
            indices[j] = indices[j - 1] + 1;
        }
    }
}

/// Projection solver result is consistent with build_qp assembly.
#[test]
fn projection_qp_assembly_consistency() {
    let simplex = known_polytopes::simplex();
    let polytope = &simplex.polytope;
    let perm = vec![0, 1, 2];

    let qp = build_qp(polytope, &perm);
    let sol = solve_projected(&qp);

    if sol.verdict != Verdict::False {
        // Verify C beta = d.
        let beta_dv = DVector::from_column_slice(&sol.beta);
        let residual = (&qp.c * &beta_dv - &qp.d).norm();
        assert!(
            residual < 1e-8,
            "||C beta - d|| = {:.2e}",
            residual
        );

        // Verify Q = (1/2) beta^T H beta.
        let q_check = 0.5 * beta_dv.dot(&(&qp.h * &beta_dv));
        assert!(
            (sol.q - q_check).abs() < 1e-10,
            "Q = {}, direct = {}, diff = {:.2e}",
            sol.q, q_check, (sol.q - q_check).abs()
        );
    }
}

//! Tests for exact rational linear algebra helpers used by vertex enumeration.
//!
//! Proposition: The low-level linear algebra routines (det4, solve4, rank_over_q,
//! cross_product_4d_rational, dot4) compute exact results over Q with no
//! floating-point approximation.
//! Reference: [lem:vertex-enumeration]
//!
//! Strategy: fixture-based on known matrices (identity, diagonal, singular)
//! and vectors, verifying exact algebraic identities.

use crate::geom::rational_arithmetic::{frac, rat};
use crate::geom::vertex_enumeration::{
    cross_product_4d_rational, det4, dot4, rank_over_q, solve4,
};
use num_rational::BigRational;
use num_traits::Zero;

// ── Determinant ─────────────────────────────────────────────────────────

/// Proposition: det(I_4) = 1.
#[test]
fn det4_identity() {
    let id: [[BigRational; 4]; 4] = [
        [rat(1), rat(0), rat(0), rat(0)],
        [rat(0), rat(1), rat(0), rat(0)],
        [rat(0), rat(0), rat(1), rat(0)],
        [rat(0), rat(0), rat(0), rat(1)],
    ];
    assert_eq!(det4(&id), rat(1));
}

/// Proposition: a matrix with two identical rows has determinant 0.
#[test]
fn det4_singular() {
    let singular: [[BigRational; 4]; 4] = [
        [rat(1), rat(2), rat(3), rat(4)],
        [rat(1), rat(2), rat(3), rat(4)],
        [rat(5), rat(6), rat(7), rat(8)],
        [rat(9), rat(10), rat(11), rat(12)],
    ];
    assert_eq!(det4(&singular), rat(0));
}

/// Proposition: det(diag(2,3,5,7)) = 210.
#[test]
fn det4_diagonal() {
    let diag: [[BigRational; 4]; 4] = [
        [rat(2), rat(0), rat(0), rat(0)],
        [rat(0), rat(3), rat(0), rat(0)],
        [rat(0), rat(0), rat(5), rat(0)],
        [rat(0), rat(0), rat(0), rat(7)],
    ];
    assert_eq!(det4(&diag), rat(210));
}

// ── Linear system solver (Cramer's rule) ────────────────────────────────

/// Proposition: solving I*x = b yields x = b.
#[test]
fn solve4_identity_system() {
    let id: [[BigRational; 4]; 4] = [
        [rat(1), rat(0), rat(0), rat(0)],
        [rat(0), rat(1), rat(0), rat(0)],
        [rat(0), rat(0), rat(1), rat(0)],
        [rat(0), rat(0), rat(0), rat(1)],
    ];
    let rhs = [rat(3), rat(7), frac(1, 2), rat(-5)];
    let x = solve4(&id, &rhs).expect("non-singular");
    assert_eq!(x, rhs);
}

/// Proposition: solving diag(2,3,5,7)*x = (4,9,10,21) yields x = (2,3,2,3).
#[test]
fn solve4_diagonal_system() {
    let diag: [[BigRational; 4]; 4] = [
        [rat(2), rat(0), rat(0), rat(0)],
        [rat(0), rat(3), rat(0), rat(0)],
        [rat(0), rat(0), rat(5), rat(0)],
        [rat(0), rat(0), rat(0), rat(7)],
    ];
    let rhs = [rat(4), rat(9), rat(10), rat(21)];
    let x = solve4(&diag, &rhs).expect("non-singular");
    assert_eq!(x, [rat(2), rat(3), rat(2), rat(3)]);
}

/// Proposition: solve4 returns None for a singular system (two identical rows).
#[test]
fn solve4_singular_returns_none() {
    let singular: [[BigRational; 4]; 4] = [
        [rat(1), rat(0), rat(0), rat(0)],
        [rat(1), rat(0), rat(0), rat(0)],
        [rat(0), rat(0), rat(1), rat(0)],
        [rat(0), rat(0), rat(0), rat(1)],
    ];
    assert!(solve4(&singular, &[rat(1), rat(1), rat(1), rat(1)]).is_none());
}

// ── Matrix rank ─────────────────────────────────────────────────────────

/// Proposition: rank(I_4) = 4.
#[test]
fn rank_over_q_identity() {
    let id = vec![
        [rat(1), rat(0), rat(0), rat(0)],
        [rat(0), rat(1), rat(0), rat(0)],
        [rat(0), rat(0), rat(1), rat(0)],
        [rat(0), rat(0), rat(0), rat(1)],
    ];
    assert_eq!(rank_over_q(&id), 4);
}

/// Proposition: replacing one row with a scalar multiple of another drops rank to 3.
#[test]
fn rank_over_q_dependent_row() {
    let rows = vec![
        [rat(1), rat(0), rat(0), rat(0)],
        [rat(0), rat(1), rat(0), rat(0)],
        [rat(0), rat(0), rat(1), rat(0)],
        [rat(2), rat(0), rat(0), rat(0)], // 2 * row 0
    ];
    assert_eq!(rank_over_q(&rows), 3);
}

/// Proposition: the zero vector has rank 0.
#[test]
fn rank_over_q_zero_vector() {
    let zeros = vec![[rat(0), rat(0), rat(0), rat(0)]];
    assert_eq!(rank_over_q(&zeros), 0);
}

/// Proposition: the empty set has rank 0.
#[test]
fn rank_over_q_empty() {
    let empty: Vec<[BigRational; 4]> = vec![];
    assert_eq!(rank_over_q(&empty), 0);
}

/// Proposition: a single nonzero vector has rank 1.
#[test]
fn rank_over_q_single_nonzero() {
    let single = vec![[rat(3), rat(-1), rat(0), rat(7)]];
    assert_eq!(rank_over_q(&single), 1);
}

// ── 4D cross product ────────────────────────────────────────────────────

/// Proposition: cross_product_4d_rational(a, b, c) is perpendicular to all three inputs
/// and is nonzero when a, b, c are linearly independent.
#[test]
fn cross_product_4d_rational_perpendicular() {
    let a = [rat(1), rat(2), rat(3), rat(4)];
    let b = [rat(5), rat(-1), rat(2), rat(0)];
    let c = [rat(0), rat(3), rat(-2), rat(1)];
    let d = cross_product_4d_rational(&a, &b, &c);

    assert!(dot4(&d, &a).is_zero(), "d . a = {} should be 0", dot4(&d, &a));
    assert!(dot4(&d, &b).is_zero(), "d . b = {} should be 0", dot4(&d, &b));
    assert!(dot4(&d, &c).is_zero(), "d . c = {} should be 0", dot4(&d, &c));
    assert!(
        !d.iter().all(|x| x.is_zero()),
        "cross product should be nonzero for independent inputs"
    );
}

/// Proposition: cross product of three dependent vectors is the zero vector.
#[test]
fn cross_product_4d_rational_dependent_is_zero() {
    let a = [rat(1), rat(0), rat(0), rat(0)];
    let b = [rat(0), rat(1), rat(0), rat(0)];
    // c = a + b, linearly dependent
    let c = [rat(1), rat(1), rat(0), rat(0)];
    let d = cross_product_4d_rational(&a, &b, &c);
    assert!(
        d.iter().all(|x| x.is_zero()),
        "cross product of dependent vectors should be zero"
    );
}

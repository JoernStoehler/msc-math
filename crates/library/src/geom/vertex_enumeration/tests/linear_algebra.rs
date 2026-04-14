//! Dedicated tests for reusable low-level linear algebra routines.

use super::super::linear_algebra::{cross_product_4d_rational, det4, dot4, rank_over_q, solve4};
use crate::geom::rational_arithmetic::{frac, rat};
use num_rational::BigRational;
use num_traits::Zero;

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

#[test]
fn solve4_identity_system() {
    let id: [[BigRational; 4]; 4] = [
        [rat(1), rat(0), rat(0), rat(0)],
        [rat(0), rat(1), rat(0), rat(0)],
        [rat(0), rat(0), rat(1), rat(0)],
        [rat(0), rat(0), rat(0), rat(1)],
    ];
    let rhs = [rat(3), rat(7), frac(1, 2), rat(-5)];
    assert_eq!(solve4(&id, &rhs), Some(rhs));
}

#[test]
fn rank_over_q_dependent_row() {
    let rows = vec![
        [rat(1), rat(0), rat(0), rat(0)],
        [rat(0), rat(1), rat(0), rat(0)],
        [rat(0), rat(0), rat(1), rat(0)],
        [rat(2), rat(0), rat(0), rat(0)],
    ];
    assert_eq!(rank_over_q(&rows), 3);
}

#[test]
fn cross_product_is_perpendicular() {
    let a = [rat(1), rat(2), rat(3), rat(4)];
    let b = [rat(5), rat(-1), rat(2), rat(0)];
    let c = [rat(0), rat(3), rat(-2), rat(1)];
    let d = cross_product_4d_rational(&a, &b, &c);
    assert!(dot4(&d, &a).is_zero());
    assert!(dot4(&d, &b).is_zero());
    assert!(dot4(&d, &c).is_zero());
}

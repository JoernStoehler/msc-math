mod common;

use common::{a, q, sample_coeffs, samples, Qsqrt5};
use num_rational::BigRational;
use std::cmp::Ordering;

#[test]
fn ordering_is_exact_for_q_sqrt5_examples() {
    assert!(Qsqrt5::root() - Qsqrt5::from(2) > Qsqrt5::from(0));
    assert!(Qsqrt5::root() - Qsqrt5::from(3) < Qsqrt5::from(0));
    assert!(Qsqrt5::root() > a(2, 0));
    assert!(Qsqrt5::root() < a(3, 0));
}

#[test]
fn sampled_ordering_against_zero_matches_rational_interval_witnesses() {
    for (rational, root_coeff) in sample_coeffs() {
        let expected = interval_ordering_witness(rational, root_coeff);
        assert_eq!(a(rational, root_coeff).cmp(&Qsqrt5::from(0)), expected);
    }
}

#[test]
fn sampled_order_matches_ordering_of_difference() {
    let values = samples();

    for left in &values {
        for right in &values {
            let difference_ordering = (left.clone() - right.clone()).cmp(&Qsqrt5::from(0));
            assert_eq!(left == right, difference_ordering == Ordering::Equal);
            assert_eq!(left < right, difference_ordering == Ordering::Less);
            assert_eq!(left > right, difference_ordering == Ordering::Greater);
        }
    }
}

fn interval_ordering_witness(rational: i64, root_coeff: i64) -> Ordering {
    // 2.23 < sqrt(5) < 2.24, and no nonzero sampled a + b*sqrt(5)
    // changes sign across this interval for -2 <= a,b <= 2.
    let lower = evaluate_at(rational, root_coeff, q(223) / q(100));
    let upper = evaluate_at(rational, root_coeff, q(224) / q(100));

    if lower > BigRational::from_integer(0.into()) && upper > BigRational::from_integer(0.into()) {
        Ordering::Greater
    } else if lower < BigRational::from_integer(0.into())
        && upper < BigRational::from_integer(0.into())
    {
        Ordering::Less
    } else {
        assert_eq!((rational, root_coeff), (0, 0));
        Ordering::Equal
    }
}

fn evaluate_at(rational: i64, root_coeff: i64, root: BigRational) -> BigRational {
    q(rational) + q(root_coeff) * root
}

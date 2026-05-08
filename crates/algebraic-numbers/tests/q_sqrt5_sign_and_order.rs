mod common;

use algebraic_numbers::Sign;
use common::{a, q, sample_coeffs, samples, Qsqrt5};
use num_rational::BigRational;

#[test]
fn sign_and_order_are_exact_for_q_sqrt5_examples() {
    assert_eq!((Qsqrt5::root() - Qsqrt5::from(2)).sign(), Sign::Positive);
    assert_eq!((Qsqrt5::root() - Qsqrt5::from(3)).sign(), Sign::Negative);
    assert!(Qsqrt5::root() > a(2, 0));
    assert!(Qsqrt5::root() < a(3, 0));
}

#[test]
fn sampled_signs_match_rational_interval_witnesses() {
    for (rational, root_coeff) in sample_coeffs() {
        let expected = interval_sign_witness(rational, root_coeff);
        assert_eq!(a(rational, root_coeff).sign(), expected);
    }
}

#[test]
fn sampled_order_matches_sign_of_difference() {
    let values = samples();

    for left in &values {
        for right in &values {
            let difference_sign = (left.clone() - right.clone()).sign();
            assert_eq!(left == right, difference_sign == Sign::Zero);
            assert_eq!(left < right, difference_sign == Sign::Negative);
            assert_eq!(left > right, difference_sign == Sign::Positive);
        }
    }
}

fn interval_sign_witness(rational: i64, root_coeff: i64) -> Sign {
    // 2.23 < sqrt(5) < 2.24, and no nonzero sampled a + b*sqrt(5)
    // changes sign across this interval for -2 <= a,b <= 2.
    let lower = evaluate_at(rational, root_coeff, q(223) / q(100));
    let upper = evaluate_at(rational, root_coeff, q(224) / q(100));

    if lower > BigRational::from_integer(0.into()) && upper > BigRational::from_integer(0.into()) {
        Sign::Positive
    } else if lower < BigRational::from_integer(0.into())
        && upper < BigRational::from_integer(0.into())
    {
        Sign::Negative
    } else {
        assert_eq!((rational, root_coeff), (0, 0));
        Sign::Zero
    }
}

fn evaluate_at(rational: i64, root_coeff: i64, root: BigRational) -> BigRational {
    q(rational) + q(root_coeff) * root
}

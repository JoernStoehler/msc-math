mod common;

use algebraic_numbers::ExactScalar;
use common::{a, q, Qsqrt5};
use num_rational::BigRational;

#[test]
fn big_rational_and_algebraic_are_exact_scalars() {
    accepts_exact_scalar::<BigRational>();
    accepts_exact_scalar::<Qsqrt5>();
}

fn accepts_exact_scalar<T: ExactScalar>() {}

#[test]
fn exact_scalars_round_to_f64() {
    assert_eq!(BigRational::new(1.into(), 4.into()).round_to_f64(), 0.25);

    let root = Qsqrt5::root().round_to_f64();
    assert!((root - 5.0_f64.sqrt()).abs() < 1e-12);

    let value = a(1, 2).round_to_f64();
    assert!((value - (1.0 + 2.0 * 5.0_f64.sqrt())).abs() < 1e-12);
    assert_eq!(Qsqrt5::from(q(7)).round_to_f64(), 7.0);
}

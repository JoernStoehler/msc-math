mod common;

use algebraic_numbers::ExactScalar;
use common::Qsqrt5;
use num_rational::BigRational;

#[test]
fn big_rational_and_algebraic_are_exact_scalars() {
    accepts_exact_scalar::<BigRational>();
    accepts_exact_scalar::<Qsqrt5>();
}

fn accepts_exact_scalar<T: ExactScalar>() {}

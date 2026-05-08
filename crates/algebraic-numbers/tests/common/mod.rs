#![allow(dead_code)]

use algebraic_numbers::{Algebraic, RealAlgebraicField};
use num_rational::BigRational;

pub enum Sqrt5 {}

impl RealAlgebraicField for Sqrt5 {
    const DEGREE: usize = 2;

    fn polynomial() -> Vec<BigRational> {
        // Low-to-high coefficients for t^2 - 5.
        vec![q(-5), q(0), q(1)]
    }

    fn isolating_interval() -> (BigRational, BigRational) {
        // Select the positive root sqrt(5).
        (q(2), q(3))
    }
}

pub type Qsqrt5 = Algebraic<Sqrt5>;

pub fn q(n: i64) -> BigRational {
    BigRational::from_integer(n.into())
}

pub fn a(rational: i64, sqrt5_coeff: i64) -> Qsqrt5 {
    Qsqrt5::new([q(rational), q(sqrt5_coeff)])
}

pub fn ar(rational: i64, sqrt5_numer: i64, sqrt5_denom: i64) -> Qsqrt5 {
    Qsqrt5::new([
        q(rational),
        BigRational::new(sqrt5_numer.into(), sqrt5_denom.into()),
    ])
}

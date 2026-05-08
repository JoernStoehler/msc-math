#![allow(dead_code)]

use algebraic_numbers::{Algebraic, RationalInterval, RealAlgebraicField};
use num_rational::BigRational;
use num_traits::{One, Zero};

pub enum Sqrt5 {}

impl RealAlgebraicField for Sqrt5 {
    const DEGREE: usize = 2;

    fn polynomial() -> Vec<BigRational> {
        vec![
            BigRational::from_integer((-5).into()),
            BigRational::zero(),
            BigRational::one(),
        ]
    }

    fn isolating_interval() -> RationalInterval {
        RationalInterval::new(q(2), q(3))
    }
}

pub type Qsqrt5 = Algebraic<Sqrt5>;

pub fn q(n: i64) -> BigRational {
    BigRational::from_integer(n.into())
}

pub fn a(rational: i64, sqrt5_coeff: i64) -> Qsqrt5 {
    Qsqrt5::new(vec![q(rational), q(sqrt5_coeff)]).unwrap()
}

pub fn ar(rational: i64, sqrt5_numer: i64, sqrt5_denom: i64) -> Qsqrt5 {
    Qsqrt5::new(vec![
        q(rational),
        BigRational::new(sqrt5_numer.into(), sqrt5_denom.into()),
    ])
    .unwrap()
}

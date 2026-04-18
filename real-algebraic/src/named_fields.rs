//! Purpose: named field specifications shipped by the crate.
//! Context: these are common examples that clients can use directly without
//! rewriting the defining polynomial and root interval.

use crate::spec::StaticFieldSpec;
use num_bigint::BigInt;
use num_rational::BigRational;

/// Field specification for `Q[tan(pi/5)]`.
#[derive(Clone, Copy, Debug)]
pub struct TanPiFifth;

impl StaticFieldSpec for TanPiFifth {
    fn name() -> &'static str {
        "Q(tan(pi/5))"
    }

    fn generator_name() -> &'static str {
        "t"
    }

    fn minimal_polynomial() -> Vec<BigRational> {
        vec![
            BigRational::from_integer(BigInt::from(5)),
            BigRational::from_integer(BigInt::from(0)),
            BigRational::from_integer(BigInt::from(-10)),
            BigRational::from_integer(BigInt::from(0)),
            BigRational::from_integer(BigInt::from(1)),
        ]
    }

    fn isolating_interval() -> (BigRational, BigRational) {
        (
            BigRational::new(BigInt::from(1), BigInt::from(2)),
            BigRational::from_integer(BigInt::from(1)),
        )
    }
}

use num_rational::BigRational;

/// Static specification of one real algebraic field `Q[alpha]`.
///
/// `polynomial()` must return the coefficients of a monic degree-`DEGREE`
/// polynomial in low-to-high order. For example, `x^2 - 5` is represented as
/// `[-5, 0, 1]`.
///
/// `isolating_interval()` selects the real root used as `alpha`. This crate
/// does not try to create or discover new fields at runtime.
pub trait RealAlgebraicField: 'static {
    const DEGREE: usize;

    fn polynomial() -> Vec<BigRational>;
    fn isolating_interval() -> RationalInterval;
}

/// Rational open interval known to contain exactly the chosen real root.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RationalInterval {
    pub lower: BigRational,
    pub upper: BigRational,
}

impl RationalInterval {
    pub fn new(lower: BigRational, upper: BigRational) -> Self {
        assert!(lower < upper);
        Self { lower, upper }
    }

    pub(crate) fn midpoint(&self) -> BigRational {
        (self.lower.clone() + self.upper.clone()) / BigRational::from_integer(2.into())
    }
}

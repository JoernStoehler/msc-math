use num_rational::BigRational;

/// Static specification of one real algebraic field `Q[alpha]`.
///
/// `polynomial()` must return the coefficients of the monic minimal polynomial
/// of `alpha`, in low-to-high order. For example, `x^2 - 5` is represented as
/// `[-5, 0, 1]`. Equality and inversion rely on this polynomial being minimal,
/// not just any polynomial that has `alpha` as a root.
///
/// `isolating_interval()` selects the real root used as `alpha`. The endpoints
/// must be rational, must not be roots of the polynomial, and the interval must
/// contain exactly one real root.
///
/// This crate trusts these contracts. It does not try to create, discover, or
/// validate fields at runtime.
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

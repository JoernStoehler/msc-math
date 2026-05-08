use num_rational::BigRational;

/// Static specification of one real algebraic field `Q[alpha]`.
///
/// `polynomial()` must return the coefficients of the monic minimal polynomial
/// of `alpha`, in low-to-high order. For example, `x^2 - 5` is represented as
/// `[-5, 0, 1]`. Equality and inversion rely on this polynomial being minimal,
/// not just any polynomial that has `alpha` as a root.
///
/// `isolating_interval()` selects the real root used as `alpha`. It returns
/// `(lower, upper)`. The endpoints must be rational, must not be roots of the
/// polynomial, and the interval must contain exactly one real root.
///
/// This crate trusts these contracts. It does not try to create, discover, or
/// validate fields at runtime.
pub trait RealAlgebraicField: 'static {
    fn polynomial() -> Vec<BigRational>;
    fn isolating_interval() -> (BigRational, BigRational);
}

pub(crate) fn field_degree<F: RealAlgebraicField>() -> usize {
    let polynomial_len = F::polynomial().len();
    assert!(
        polynomial_len >= 2,
        "field polynomial must have positive degree"
    );
    polynomial_len - 1
}

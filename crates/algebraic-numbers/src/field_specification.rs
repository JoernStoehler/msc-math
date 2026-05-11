use num_rational::BigRational;
use num_traits::{ToPrimitive, Zero};

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

    fn root_f64() -> f64 {
        let polynomial = Self::polynomial()
            .into_iter()
            .map(|coefficient| rational_to_f64(&coefficient))
            .collect::<Vec<_>>();
        let (lower, upper) = Self::isolating_interval();
        let mut lower = rational_to_f64(&lower);
        let mut upper = rational_to_f64(&upper);

        assert!(lower.is_finite() && upper.is_finite());
        assert!(lower < upper);

        let mut lower_value = polynomial_eval_f64(&polynomial, lower);
        assert!(lower_value.is_finite());
        assert!(lower_value != 0.0, "isolating interval endpoint is a root");

        for _ in 0..80 {
            let midpoint = 0.5 * (lower + upper);
            if midpoint == lower || midpoint == upper {
                return midpoint;
            }

            let midpoint_value = polynomial_eval_f64(&polynomial, midpoint);
            if midpoint_value == 0.0 {
                return midpoint;
            }
            assert!(midpoint_value.is_finite());

            if lower_value.signum() == midpoint_value.signum() {
                lower = midpoint;
                lower_value = midpoint_value;
            } else {
                upper = midpoint;
            }
        }

        0.5 * (lower + upper)
    }
}

pub(crate) fn field_degree<F: RealAlgebraicField>() -> usize {
    let polynomial_len = F::polynomial().len();
    assert!(
        polynomial_len >= 2,
        "field polynomial must have positive degree"
    );
    polynomial_len - 1
}

fn rational_to_f64(value: &BigRational) -> f64 {
    value.to_f64().unwrap_or_else(|| {
        if value < &BigRational::zero() {
            f64::NEG_INFINITY
        } else {
            f64::INFINITY
        }
    })
}

fn polynomial_eval_f64(coeffs: &[f64], x: f64) -> f64 {
    coeffs
        .iter()
        .rev()
        .fold(0.0, |value, coeff| value.mul_add(x, *coeff))
}

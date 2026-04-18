//! Purpose: compile-time field specifications for named algebraic fields.
//! Context: Rust lacks dependent types, so named fields are modeled by marker
//! types that provide the defining polynomial and the chosen real root.

use num_rational::BigRational;
use num_traits::{Signed, Zero};
use std::any::TypeId;
use std::collections::HashMap;
use std::sync::{OnceLock, RwLock};

/// Validation errors for one field specification.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FieldSpecError {
    ZeroPolynomial,
    ConstantPolynomial,
    InvalidIntervalOrder,
    LowerEndpointIsRoot,
    UpperEndpointIsRoot,
    IntervalDoesNotIsolateUniqueRoot { root_count: usize },
}

/// Compile-time specification of one algebraic field `Q[t] / (p(t))` together
/// with an isolating interval for the chosen real root.
pub trait StaticFieldSpec: 'static {
    /// Human-readable field name used in diagnostics and serialization.
    fn name() -> &'static str;

    /// Symbol used for the generator in basis labels.
    fn generator_name() -> &'static str;

    /// Irreducible minimal polynomial in ascending coefficient order.
    ///
    /// Example: `x^2 - 2` is `[-2, 0, 1]`.
    fn minimal_polynomial() -> Vec<BigRational>;

    /// Rational interval `(lo, hi)` isolating the chosen real root.
    fn isolating_interval() -> (BigRational, BigRational);
}

/// Validate that a field specification presents a single chosen real root of a
/// nonconstant polynomial.
///
/// This checks the real-root isolation data. It does not prove that
/// [`StaticFieldSpec::minimal_polynomial`] is irreducible over `Q`; callers
/// must still supply the actual minimal polynomial.
pub fn validate_field_spec<S: StaticFieldSpec>() -> Result<(), FieldSpecError> {
    validate_spec_data(&S::minimal_polynomial(), &S::isolating_interval())
}

pub(crate) fn assert_valid_field_spec<S: StaticFieldSpec>() {
    let type_id = TypeId::of::<S>();
    let cache = field_spec_cache();

    if let Some(result) = cache
        .read()
        .expect("field-spec validation cache poisoned")
        .get(&type_id)
        .cloned()
    {
        result.unwrap_or_else(|err| panic!("invalid field specification {}: {:?}", S::name(), err));
        return;
    }

    let result = validate_field_spec::<S>();
    cache
        .write()
        .expect("field-spec validation cache poisoned")
        .insert(type_id, result.clone());
    result.unwrap_or_else(|err| panic!("invalid field specification {}: {:?}", S::name(), err));
}

fn field_spec_cache() -> &'static RwLock<HashMap<TypeId, Result<(), FieldSpecError>>> {
    static CACHE: OnceLock<RwLock<HashMap<TypeId, Result<(), FieldSpecError>>>> = OnceLock::new();
    CACHE.get_or_init(|| RwLock::new(HashMap::new()))
}

fn validate_spec_data(
    minimal_polynomial: &[BigRational],
    interval: &(BigRational, BigRational),
) -> Result<(), FieldSpecError> {
    let poly = trim(minimal_polynomial.to_vec());
    if poly.iter().all(Zero::is_zero) {
        return Err(FieldSpecError::ZeroPolynomial);
    }
    if poly.len() < 2 {
        return Err(FieldSpecError::ConstantPolynomial);
    }

    let (lo, hi) = interval;
    if lo >= hi {
        return Err(FieldSpecError::InvalidIntervalOrder);
    }

    if eval_poly(&poly, lo).is_zero() {
        return Err(FieldSpecError::LowerEndpointIsRoot);
    }
    if eval_poly(&poly, hi).is_zero() {
        return Err(FieldSpecError::UpperEndpointIsRoot);
    }

    let root_count = sturm_root_count_between(&poly, lo, hi);
    if root_count != 1 {
        return Err(FieldSpecError::IntervalDoesNotIsolateUniqueRoot { root_count });
    }

    Ok(())
}

fn trim(mut coeffs: Vec<BigRational>) -> Vec<BigRational> {
    while coeffs.last().is_some_and(Zero::is_zero) {
        coeffs.pop();
    }
    if coeffs.is_empty() {
        vec![BigRational::zero()]
    } else {
        coeffs
    }
}

fn poly_degree(poly: &[BigRational]) -> usize {
    poly.iter().rposition(|coeff| !coeff.is_zero()).unwrap_or(0)
}

fn eval_poly(coeffs: &[BigRational], x: &BigRational) -> BigRational {
    coeffs.iter().rev().fold(BigRational::zero(), |acc, coeff| {
        acc * x.clone() + coeff.clone()
    })
}

fn derivative(poly: &[BigRational]) -> Vec<BigRational> {
    if poly.len() <= 1 {
        return vec![BigRational::zero()];
    }
    trim(
        poly.iter()
            .enumerate()
            .skip(1)
            .map(|(idx, coeff)| BigRational::from_integer((idx as i64).into()) * coeff.clone())
            .collect(),
    )
}

fn poly_neg(poly: &[BigRational]) -> Vec<BigRational> {
    trim(poly.iter().map(|coeff| -coeff.clone()).collect())
}

fn poly_div_rem(
    dividend: Vec<BigRational>,
    divisor: Vec<BigRational>,
) -> (Vec<BigRational>, Vec<BigRational>) {
    let dividend = trim(dividend);
    let divisor = trim(divisor);
    assert!(
        !divisor.iter().all(Zero::is_zero),
        "polynomial division by zero"
    );

    if poly_degree(&dividend) < poly_degree(&divisor) {
        return (vec![BigRational::zero()], dividend);
    }

    let mut remainder = dividend;
    let divisor_degree = poly_degree(&divisor);
    let divisor_leading = divisor[divisor_degree].clone();
    let mut quotient = vec![BigRational::zero(); poly_degree(&remainder) - divisor_degree + 1];

    while !remainder.iter().all(Zero::is_zero) && poly_degree(&remainder) >= divisor_degree {
        let remainder_degree = poly_degree(&remainder);
        let shift = remainder_degree - divisor_degree;
        let factor = remainder[remainder_degree].clone() / divisor_leading.clone();
        quotient[shift] += factor.clone();
        for (idx, coeff) in divisor.iter().enumerate().take(divisor_degree + 1) {
            remainder[idx + shift] -= factor.clone() * coeff.clone();
        }
        remainder = trim(remainder);
    }

    (trim(quotient), trim(remainder))
}

fn sturm_sequence(poly: &[BigRational]) -> Vec<Vec<BigRational>> {
    let mut seq = vec![trim(poly.to_vec()), derivative(poly)];
    while !seq
        .last()
        .expect("nonempty sturm sequence")
        .iter()
        .all(Zero::is_zero)
    {
        let prev = seq[seq.len() - 2].clone();
        let curr = seq[seq.len() - 1].clone();
        let (_, remainder) = poly_div_rem(prev, curr);
        if remainder.iter().all(Zero::is_zero) {
            break;
        }
        seq.push(poly_neg(&remainder));
    }
    seq
}

fn sign_variations_at(seq: &[Vec<BigRational>], x: &BigRational) -> usize {
    let signs: Vec<i8> = seq
        .iter()
        .filter_map(|poly| {
            let value = eval_poly(poly, x);
            if value.is_zero() {
                None
            } else if value.is_positive() {
                Some(1)
            } else {
                Some(-1)
            }
        })
        .collect();

    signs.windows(2).filter(|pair| pair[0] != pair[1]).count()
}

fn sturm_root_count_between(poly: &[BigRational], lo: &BigRational, hi: &BigRational) -> usize {
    let seq = sturm_sequence(poly);
    sign_variations_at(&seq, lo) - sign_variations_at(&seq, hi)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct ValidSqrtTwo;

    impl StaticFieldSpec for ValidSqrtTwo {
        fn name() -> &'static str {
            "Q(sqrt(2))"
        }

        fn generator_name() -> &'static str {
            "s"
        }

        fn minimal_polynomial() -> Vec<BigRational> {
            vec![
                BigRational::from_integer((-2).into()),
                BigRational::from_integer(0.into()),
                BigRational::from_integer(1.into()),
            ]
        }

        fn isolating_interval() -> (BigRational, BigRational) {
            (
                BigRational::from_integer(1.into()),
                BigRational::from_integer(2.into()),
            )
        }
    }

    struct ConstantSpec;

    impl StaticFieldSpec for ConstantSpec {
        fn name() -> &'static str {
            "constant"
        }

        fn generator_name() -> &'static str {
            "c"
        }

        fn minimal_polynomial() -> Vec<BigRational> {
            vec![BigRational::from_integer(1.into())]
        }

        fn isolating_interval() -> (BigRational, BigRational) {
            (
                BigRational::from_integer(0.into()),
                BigRational::from_integer(1.into()),
            )
        }
    }

    struct EndpointRootSpec;

    impl StaticFieldSpec for EndpointRootSpec {
        fn name() -> &'static str {
            "endpoint_root"
        }

        fn generator_name() -> &'static str {
            "x"
        }

        fn minimal_polynomial() -> Vec<BigRational> {
            vec![
                BigRational::from_integer((-1).into()),
                BigRational::from_integer(0.into()),
                BigRational::from_integer(1.into()),
            ]
        }

        fn isolating_interval() -> (BigRational, BigRational) {
            (
                BigRational::from_integer(0.into()),
                BigRational::from_integer(1.into()),
            )
        }
    }

    struct MultipleRootsInIntervalSpec;

    impl StaticFieldSpec for MultipleRootsInIntervalSpec {
        fn name() -> &'static str {
            "multiple_roots"
        }

        fn generator_name() -> &'static str {
            "x"
        }

        fn minimal_polynomial() -> Vec<BigRational> {
            vec![
                BigRational::from_integer(0.into()),
                BigRational::from_integer((-1).into()),
                BigRational::from_integer(0.into()),
                BigRational::from_integer(1.into()),
            ]
        }

        fn isolating_interval() -> (BigRational, BigRational) {
            (
                BigRational::from_integer((-2).into()),
                BigRational::from_integer(2.into()),
            )
        }
    }

    #[test]
    fn valid_field_spec_is_accepted() {
        assert_eq!(validate_field_spec::<ValidSqrtTwo>(), Ok(()));
    }

    #[test]
    fn constant_polynomial_is_rejected() {
        assert_eq!(
            validate_field_spec::<ConstantSpec>(),
            Err(FieldSpecError::ConstantPolynomial)
        );
    }

    #[test]
    fn endpoint_root_is_rejected() {
        assert_eq!(
            validate_field_spec::<EndpointRootSpec>(),
            Err(FieldSpecError::UpperEndpointIsRoot)
        );
    }

    #[test]
    fn multiple_roots_interval_is_rejected() {
        assert_eq!(
            validate_field_spec::<MultipleRootsInIntervalSpec>(),
            Err(FieldSpecError::IntervalDoesNotIsolateUniqueRoot { root_count: 3 })
        );
    }
}

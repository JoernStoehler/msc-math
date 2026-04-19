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

use num_rational::BigRational;
use num_traits::Zero;

use crate::field_specification::RealAlgebraicField;
use crate::polynomial_arithmetic::polynomial_eval;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Sign {
    Negative,
    Zero,
    Positive,
}

pub(crate) fn sign_at_field_root<F: RealAlgebraicField>(coeffs: &[BigRational]) -> Sign {
    if coeffs.iter().all(BigRational::is_zero) {
        return Sign::Zero;
    }

    let mut interval = Interval::from_pair(F::isolating_interval());

    loop {
        // Two tempting shortcuts are wrong for this crate:
        // - evaluating at an f64 approximation of alpha loses exactness;
        // - evaluating once on the initial interval may be inconclusive.
        // We instead refine the isolating interval until interval evaluation
        // proves a strict sign, or until alpha is found as a rational midpoint.
        let value_interval = polynomial_interval_eval(coeffs, &interval);
        if value_interval.lower > BigRational::zero() {
            return Sign::Positive;
        }
        if value_interval.upper < BigRational::zero() {
            return Sign::Negative;
        }

        match refine_root_interval::<F>(&interval) {
            RefinedRoot::Exact(root) => return rational_sign(&polynomial_eval(coeffs, &root)),
            RefinedRoot::Interval(next) => interval = next,
        }
    }
}

enum RefinedRoot {
    Exact(BigRational),
    Interval(Interval),
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Interval {
    lower: BigRational,
    upper: BigRational,
}

impl Interval {
    fn from_pair((lower, upper): (BigRational, BigRational)) -> Self {
        assert!(lower < upper);
        Self { lower, upper }
    }

    fn with_bounds(lower: BigRational, upper: BigRational) -> Self {
        assert!(lower <= upper);
        Self { lower, upper }
    }

    fn point(value: BigRational) -> Self {
        Self::with_bounds(value.clone(), value)
    }

    fn midpoint(&self) -> BigRational {
        (self.lower.clone() + self.upper.clone()) / BigRational::from_integer(2.into())
    }
}

fn refine_root_interval<F: RealAlgebraicField>(interval: &Interval) -> RefinedRoot {
    let polynomial = F::polynomial();
    let midpoint = interval.midpoint();
    let lower_sign = rational_sign(&polynomial_eval(&polynomial, &interval.lower));
    let middle_sign = rational_sign(&polynomial_eval(&polynomial, &midpoint));
    let upper_sign = rational_sign(&polynomial_eval(&polynomial, &interval.upper));

    assert_ne!(
        lower_sign,
        Sign::Zero,
        "isolating interval endpoint is a root"
    );
    assert_ne!(
        upper_sign,
        Sign::Zero,
        "isolating interval endpoint is a root"
    );

    if middle_sign == Sign::Zero {
        return RefinedRoot::Exact(midpoint);
    }

    if lower_sign != middle_sign {
        RefinedRoot::Interval(Interval::from_pair((interval.lower.clone(), midpoint)))
    } else {
        assert_ne!(
            middle_sign, upper_sign,
            "interval does not isolate a sign-changing root"
        );
        RefinedRoot::Interval(Interval::from_pair((midpoint, interval.upper.clone())))
    }
}

fn polynomial_interval_eval(coeffs: &[BigRational], interval: &Interval) -> Interval {
    let mut result = Interval::point(BigRational::zero());

    for coeff in coeffs.iter().rev() {
        result = interval_mul(&result, interval);
        result.lower += coeff.clone();
        result.upper += coeff.clone();
    }

    result
}

fn interval_mul(left: &Interval, right: &Interval) -> Interval {
    let values = [
        left.lower.clone() * right.lower.clone(),
        left.lower.clone() * right.upper.clone(),
        left.upper.clone() * right.lower.clone(),
        left.upper.clone() * right.upper.clone(),
    ];
    let lower = values.iter().min().expect("array is nonempty").clone();
    let upper = values.iter().max().expect("array is nonempty").clone();
    Interval::with_bounds(lower, upper)
}

fn rational_sign(value: &BigRational) -> Sign {
    if value < &BigRational::zero() {
        Sign::Negative
    } else if value > &BigRational::zero() {
        Sign::Positive
    } else {
        Sign::Zero
    }
}

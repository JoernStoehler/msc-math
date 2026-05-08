use std::cmp::Ordering;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use num_rational::BigRational;
use num_traits::{One, Zero};

pub trait ExactScalar:
    Clone
    + Debug
    + Eq
    + Ord
    + Zero
    + One
    + Neg<Output = Self>
    + Add<Output = Self>
    + AddAssign
    + Sub<Output = Self>
    + SubAssign
    + Mul<Output = Self>
    + MulAssign
    + Div<Output = Self>
    + DivAssign
{
}

impl ExactScalar for BigRational {}

pub trait RealAlgebraicField: 'static {
    const DEGREE: usize;

    fn polynomial() -> Vec<BigRational>;
    fn isolating_interval() -> RationalInterval;
}

pub struct Algebraic<F: RealAlgebraicField> {
    coeffs: Vec<BigRational>,
    field: PhantomData<F>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BadDegree {
    pub expected: usize,
    pub actual: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RationalInterval {
    pub lower: BigRational,
    pub upper: BigRational,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Sign {
    Negative,
    Zero,
    Positive,
}

impl<F: RealAlgebraicField> Algebraic<F> {
    pub fn new(coeffs: Vec<BigRational>) -> Result<Self, BadDegree> {
        if coeffs.len() != F::DEGREE {
            return Err(BadDegree {
                expected: F::DEGREE,
                actual: coeffs.len(),
            });
        }

        Ok(Self::from_coeffs_unchecked(coeffs))
    }

    pub fn coeffs(&self) -> &[BigRational] {
        &self.coeffs
    }

    pub fn from_rational(rational: BigRational) -> Self {
        let mut coeffs = vec![BigRational::zero(); F::DEGREE];
        coeffs[0] = rational;
        Self::from_coeffs_unchecked(coeffs)
    }

    pub fn alpha() -> Self {
        assert!(F::DEGREE > 1);
        let mut coeffs = vec![BigRational::zero(); F::DEGREE];
        coeffs[1] = BigRational::one();
        Self::from_coeffs_unchecked(coeffs)
    }

    pub fn sign(&self) -> Sign {
        sign_at_field_root::<F>(&self.coeffs)
    }

    pub fn inverse(&self) -> Self {
        assert!(!self.is_zero(), "cannot invert zero");
        inverse_mod_monic::<F>(&self.coeffs)
    }

    fn from_coeffs_unchecked(coeffs: Vec<BigRational>) -> Self {
        debug_assert_eq!(coeffs.len(), F::DEGREE);
        Self {
            coeffs,
            field: PhantomData,
        }
    }
}

impl<F: RealAlgebraicField> Clone for Algebraic<F> {
    fn clone(&self) -> Self {
        Self::from_coeffs_unchecked(self.coeffs.clone())
    }
}

impl<F: RealAlgebraicField> Debug for Algebraic<F> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Algebraic")
            .field("coeffs", &self.coeffs)
            .finish()
    }
}

impl<F: RealAlgebraicField> PartialEq for Algebraic<F> {
    fn eq(&self, other: &Self) -> bool {
        self.coeffs == other.coeffs
    }
}

impl<F: RealAlgebraicField> Eq for Algebraic<F> {}

impl<F: RealAlgebraicField> PartialOrd for Algebraic<F> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<F: RealAlgebraicField> Ord for Algebraic<F> {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self.clone() - other.clone()).sign() {
            Sign::Negative => Ordering::Less,
            Sign::Zero => Ordering::Equal,
            Sign::Positive => Ordering::Greater,
        }
    }
}

impl RationalInterval {
    pub fn new(lower: BigRational, upper: BigRational) -> Self {
        assert!(lower < upper);
        Self { lower, upper }
    }

    fn midpoint(&self) -> BigRational {
        (self.lower.clone() + self.upper.clone()) / BigRational::from_integer(2.into())
    }
}

impl<F: RealAlgebraicField> Zero for Algebraic<F> {
    fn zero() -> Self {
        Self::from_coeffs_unchecked(vec![BigRational::zero(); F::DEGREE])
    }

    fn is_zero(&self) -> bool {
        self.coeffs.iter().all(BigRational::is_zero)
    }
}

impl<F: RealAlgebraicField> One for Algebraic<F> {
    fn one() -> Self {
        let mut coeffs = vec![BigRational::zero(); F::DEGREE];
        coeffs[0] = BigRational::one();
        Self::from_coeffs_unchecked(coeffs)
    }
}

impl<F: RealAlgebraicField> Neg for Algebraic<F> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::from_coeffs_unchecked(self.coeffs.into_iter().map(Neg::neg).collect())
    }
}

impl<F: RealAlgebraicField> Add for Algebraic<F> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let coeffs = self
            .coeffs
            .into_iter()
            .zip(rhs.coeffs)
            .map(|(left, right)| left + right)
            .collect();
        Self::from_coeffs_unchecked(coeffs)
    }
}

impl<F: RealAlgebraicField> AddAssign for Algebraic<F> {
    fn add_assign(&mut self, rhs: Self) {
        for (left, right) in self.coeffs.iter_mut().zip(rhs.coeffs) {
            *left += right;
        }
    }
}

impl<F: RealAlgebraicField> Sub for Algebraic<F> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let coeffs = self
            .coeffs
            .into_iter()
            .zip(rhs.coeffs)
            .map(|(left, right)| left - right)
            .collect();
        Self::from_coeffs_unchecked(coeffs)
    }
}

impl<F: RealAlgebraicField> SubAssign for Algebraic<F> {
    fn sub_assign(&mut self, rhs: Self) {
        for (left, right) in self.coeffs.iter_mut().zip(rhs.coeffs) {
            *left -= right;
        }
    }
}

impl<F: RealAlgebraicField> Mul for Algebraic<F> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        multiply(self, rhs)
    }
}

impl<F: RealAlgebraicField> MulAssign for Algebraic<F> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.clone() * rhs;
    }
}

impl<F: RealAlgebraicField> Div for Algebraic<F> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        multiply(self, rhs.inverse())
    }
}

impl<F: RealAlgebraicField> DivAssign for Algebraic<F> {
    fn div_assign(&mut self, rhs: Self) {
        *self = self.clone() / rhs;
    }
}

impl<F: RealAlgebraicField> From<BigRational> for Algebraic<F> {
    fn from(value: BigRational) -> Self {
        Self::from_rational(value)
    }
}

impl<F: RealAlgebraicField> From<i64> for Algebraic<F> {
    fn from(value: i64) -> Self {
        Self::from_rational(BigRational::from_integer(value.into()))
    }
}

macro_rules! impl_scalar_ops {
    ($scalar:ty) => {
        impl<F: RealAlgebraicField> Add<$scalar> for Algebraic<F> {
            type Output = Self;

            fn add(self, rhs: $scalar) -> Self::Output {
                self + Self::from(rhs)
            }
        }

        impl<F: RealAlgebraicField> AddAssign<$scalar> for Algebraic<F> {
            fn add_assign(&mut self, rhs: $scalar) {
                *self += Self::from(rhs);
            }
        }

        impl<F: RealAlgebraicField> Add<Algebraic<F>> for $scalar {
            type Output = Algebraic<F>;

            fn add(self, rhs: Algebraic<F>) -> Self::Output {
                Algebraic::from(self) + rhs
            }
        }

        impl<F: RealAlgebraicField> Sub<$scalar> for Algebraic<F> {
            type Output = Self;

            fn sub(self, rhs: $scalar) -> Self::Output {
                self - Self::from(rhs)
            }
        }

        impl<F: RealAlgebraicField> SubAssign<$scalar> for Algebraic<F> {
            fn sub_assign(&mut self, rhs: $scalar) {
                *self -= Self::from(rhs);
            }
        }

        impl<F: RealAlgebraicField> Sub<Algebraic<F>> for $scalar {
            type Output = Algebraic<F>;

            fn sub(self, rhs: Algebraic<F>) -> Self::Output {
                Algebraic::from(self) - rhs
            }
        }

        impl<F: RealAlgebraicField> Mul<$scalar> for Algebraic<F> {
            type Output = Self;

            fn mul(self, rhs: $scalar) -> Self::Output {
                self * Self::from(rhs)
            }
        }

        impl<F: RealAlgebraicField> MulAssign<$scalar> for Algebraic<F> {
            fn mul_assign(&mut self, rhs: $scalar) {
                *self *= Self::from(rhs);
            }
        }

        impl<F: RealAlgebraicField> Mul<Algebraic<F>> for $scalar {
            type Output = Algebraic<F>;

            fn mul(self, rhs: Algebraic<F>) -> Self::Output {
                Algebraic::from(self) * rhs
            }
        }

        impl<F: RealAlgebraicField> Div<$scalar> for Algebraic<F> {
            type Output = Self;

            fn div(self, rhs: $scalar) -> Self::Output {
                self / Self::from(rhs)
            }
        }

        impl<F: RealAlgebraicField> DivAssign<$scalar> for Algebraic<F> {
            fn div_assign(&mut self, rhs: $scalar) {
                *self /= Self::from(rhs);
            }
        }

        impl<F: RealAlgebraicField> Div<Algebraic<F>> for $scalar {
            type Output = Algebraic<F>;

            fn div(self, rhs: Algebraic<F>) -> Self::Output {
                Algebraic::from(self) / rhs
            }
        }
    };
}

impl_scalar_ops!(BigRational);
impl_scalar_ops!(i64);

impl<F: RealAlgebraicField> ExactScalar for Algebraic<F> {}

fn multiply<F: RealAlgebraicField>(left: Algebraic<F>, right: Algebraic<F>) -> Algebraic<F> {
    let degree = F::DEGREE;
    let mut product = vec![BigRational::zero(); 2 * degree - 1];

    for (i, left_coeff) in left.coeffs.into_iter().enumerate() {
        for (j, right_coeff) in right.coeffs.iter().enumerate() {
            product[i + j] += left_coeff.clone() * right_coeff.clone();
        }
    }

    reduce_monic::<F>(product)
}

fn reduce_monic<F: RealAlgebraicField>(mut coeffs: Vec<BigRational>) -> Algebraic<F> {
    let degree = F::DEGREE;
    let polynomial = F::polynomial();

    assert_eq!(polynomial.len(), degree + 1);
    assert_eq!(polynomial[degree], BigRational::one());

    while coeffs.len() > degree {
        let leading = coeffs.pop().expect("length checked above");
        if leading.is_zero() {
            continue;
        }

        let offset = coeffs.len() - degree;
        for i in 0..degree {
            coeffs[offset + i] -= leading.clone() * polynomial[i].clone();
        }
    }

    coeffs.resize(degree, BigRational::zero());
    Algebraic::from_coeffs_unchecked(coeffs)
}

fn inverse_mod_monic<F: RealAlgebraicField>(coeffs: &[BigRational]) -> Algebraic<F> {
    let mut old_r = F::polynomial();
    let mut r = coeffs.to_vec();
    trim(&mut old_r);
    trim(&mut r);

    let mut old_t = Vec::new();
    let mut t = vec![BigRational::one()];

    while !r.is_empty() {
        let (quotient, remainder) = polynomial_div_rem(&old_r, &r);
        old_r = r;
        r = remainder;

        let next_t = polynomial_sub(&old_t, &polynomial_mul(&quotient, &t));
        old_t = t;
        t = next_t;
    }

    assert_eq!(
        old_r.len(),
        1,
        "element is not invertible modulo field polynomial"
    );
    let gcd_constant = old_r.pop().expect("length checked above");
    let inverse = old_t
        .into_iter()
        .map(|coeff| coeff / gcd_constant.clone())
        .collect();
    reduce_monic::<F>(inverse)
}

fn sign_at_field_root<F: RealAlgebraicField>(coeffs: &[BigRational]) -> Sign {
    if coeffs.iter().all(BigRational::is_zero) {
        return Sign::Zero;
    }

    let mut interval = F::isolating_interval();

    loop {
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
    Interval(RationalInterval),
}

fn refine_root_interval<F: RealAlgebraicField>(interval: &RationalInterval) -> RefinedRoot {
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
        RefinedRoot::Interval(RationalInterval::new(interval.lower.clone(), midpoint))
    } else {
        assert_ne!(
            middle_sign, upper_sign,
            "interval does not isolate a sign-changing root"
        );
        RefinedRoot::Interval(RationalInterval::new(midpoint, interval.upper.clone()))
    }
}

fn polynomial_interval_eval(
    coeffs: &[BigRational],
    interval: &RationalInterval,
) -> RationalInterval {
    let mut result = RationalInterval {
        lower: BigRational::zero(),
        upper: BigRational::zero(),
    };

    for coeff in coeffs.iter().rev() {
        result = interval_mul(&result, interval);
        result.lower += coeff.clone();
        result.upper += coeff.clone();
    }

    result
}

fn interval_mul(left: &RationalInterval, right: &RationalInterval) -> RationalInterval {
    let values = [
        left.lower.clone() * right.lower.clone(),
        left.lower.clone() * right.upper.clone(),
        left.upper.clone() * right.lower.clone(),
        left.upper.clone() * right.upper.clone(),
    ];
    let lower = values.iter().min().expect("array is nonempty").clone();
    let upper = values.iter().max().expect("array is nonempty").clone();
    RationalInterval { lower, upper }
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

fn polynomial_eval(coeffs: &[BigRational], x: &BigRational) -> BigRational {
    let mut result = BigRational::zero();
    for coeff in coeffs.iter().rev() {
        result *= x.clone();
        result += coeff.clone();
    }
    result
}

fn polynomial_sub(left: &[BigRational], right: &[BigRational]) -> Vec<BigRational> {
    let len = left.len().max(right.len());
    let mut result = vec![BigRational::zero(); len];
    for i in 0..len {
        if i < left.len() {
            result[i] += left[i].clone();
        }
        if i < right.len() {
            result[i] -= right[i].clone();
        }
    }
    trim(&mut result);
    result
}

fn polynomial_mul(left: &[BigRational], right: &[BigRational]) -> Vec<BigRational> {
    if left.is_empty() || right.is_empty() {
        return Vec::new();
    }

    let mut result = vec![BigRational::zero(); left.len() + right.len() - 1];
    for (i, left_coeff) in left.iter().enumerate() {
        for (j, right_coeff) in right.iter().enumerate() {
            result[i + j] += left_coeff.clone() * right_coeff.clone();
        }
    }
    trim(&mut result);
    result
}

fn polynomial_div_rem(
    numerator: &[BigRational],
    denominator: &[BigRational],
) -> (Vec<BigRational>, Vec<BigRational>) {
    assert!(!denominator.is_empty(), "division by zero polynomial");

    let mut remainder = numerator.to_vec();
    trim(&mut remainder);
    if remainder.len() < denominator.len() {
        return (Vec::new(), remainder);
    }

    let mut quotient = vec![BigRational::zero(); remainder.len() - denominator.len() + 1];
    let denominator_leading = denominator.last().expect("denominator is nonempty").clone();

    while !remainder.is_empty() && remainder.len() >= denominator.len() {
        let offset = remainder.len() - denominator.len();
        let quotient_coeff =
            remainder.last().expect("remainder is nonempty").clone() / denominator_leading.clone();
        quotient[offset] = quotient_coeff.clone();

        for (i, denominator_coeff) in denominator.iter().enumerate() {
            remainder[offset + i] -= quotient_coeff.clone() * denominator_coeff.clone();
        }
        trim(&mut remainder);
    }

    trim(&mut quotient);
    (quotient, remainder)
}

fn trim(poly: &mut Vec<BigRational>) {
    while poly.last().is_some_and(BigRational::is_zero) {
        poly.pop();
    }
}

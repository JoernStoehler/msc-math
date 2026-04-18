//! Exact ordered-field abstraction for the experiment-owned algebraic spike.
//!
//! The first concrete backends are `BigRational` and the pentagon field. The
//! geometry and exact-KKT modules only rely on this trait so the experiment can
//! stay generic while keeping the implementation surface small.

use super::named_field::NamedFieldTag;
use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{One, Signed, ToPrimitive, Zero};

/// Exact sign of a field element.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExactSign {
    Negative,
    Zero,
    Positive,
}

/// Exact ordered field interface used by the algebraic exactness experiment.
pub trait ExactOrderedField:
    Clone
    + std::fmt::Debug
    + PartialEq
    + Eq
    + std::ops::Add<Output = Self>
    + std::ops::Sub<Output = Self>
    + std::ops::Mul<Output = Self>
    + std::ops::Div<Output = Self>
    + std::ops::Neg<Output = Self>
{
    /// Zero element.
    fn zero() -> Self;

    /// Multiplicative identity.
    fn one() -> Self;

    /// Convert a rational scalar into the field.
    fn from_big_rational(value: BigRational) -> Self;

    /// Exact sign classification.
    fn sign(&self) -> ExactSign;

    /// Best-effort `f64` approximation for reporting and cross-checks.
    fn to_f64(&self) -> f64;

    /// Canonical basis coefficients used by the experiment-owned exact catalog.
    fn canonical_coeffs(&self) -> Vec<BigRational>;

    /// Row-level exact field tag used by the experiment-owned exact catalog.
    fn field_tag() -> NamedFieldTag;

    /// Convenience integer embedding.
    fn from_i64(value: i64) -> Self {
        Self::from_big_rational(BigRational::from_integer(BigInt::from(value)))
    }

    /// Convenience fraction embedding.
    fn from_frac(numer: i64, denom: i64) -> Self {
        Self::from_big_rational(BigRational::new(BigInt::from(numer), BigInt::from(denom)))
    }

    /// Exact zero check.
    fn is_zero(&self) -> bool {
        self.sign() == ExactSign::Zero
    }

    /// Exact positivity check.
    fn is_positive(&self) -> bool {
        self.sign() == ExactSign::Positive
    }

    /// Exact negativity check.
    fn is_negative(&self) -> bool {
        self.sign() == ExactSign::Negative
    }
}

impl ExactOrderedField for BigRational {
    fn zero() -> Self {
        <BigRational as Zero>::zero()
    }

    fn one() -> Self {
        <BigRational as One>::one()
    }

    fn from_big_rational(value: BigRational) -> Self {
        value
    }

    fn sign(&self) -> ExactSign {
        if <BigRational as Zero>::is_zero(self) {
            ExactSign::Zero
        } else if <BigRational as Signed>::is_positive(self) {
            ExactSign::Positive
        } else {
            ExactSign::Negative
        }
    }

    fn to_f64(&self) -> f64 {
        let numer = self.numer().to_f64().unwrap_or(f64::NAN);
        let denom = self.denom().to_f64().unwrap_or(1.0);
        numer / denom
    }

    fn canonical_coeffs(&self) -> Vec<BigRational> {
        vec![self.clone()]
    }

    fn field_tag() -> NamedFieldTag {
        NamedFieldTag::Rational
    }
}

/// Convenience rational integer.
pub fn rat(n: i64) -> BigRational {
    BigRational::from_integer(BigInt::from(n))
}

/// Convenience rational fraction.
pub fn frac(numer: i64, denom: i64) -> BigRational {
    BigRational::new(BigInt::from(numer), BigInt::from(denom))
}

/// Exact ordering helper on any supported field.
pub fn cmp_field<F: ExactOrderedField>(left: &F, right: &F) -> std::cmp::Ordering {
    match (left.clone() - right.clone()).sign() {
        ExactSign::Negative => std::cmp::Ordering::Less,
        ExactSign::Zero => std::cmp::Ordering::Equal,
        ExactSign::Positive => std::cmp::Ordering::Greater,
    }
}

/// Exact `min` helper on any supported field.
pub fn min_field<F: ExactOrderedField>(left: F, right: F) -> F {
    if cmp_field(&left, &right).is_gt() {
        right
    } else {
        left
    }
}

/// Exact `max` helper on any supported field.
pub fn max_field<F: ExactOrderedField>(left: F, right: F) -> F {
    if cmp_field(&left, &right).is_lt() {
        right
    } else {
        left
    }
}

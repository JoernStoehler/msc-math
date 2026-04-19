//! Purpose: common ordered-field trait and scalar helper functions.
//! Context: crate clients write arithmetic-heavy code against this trait so the
//! same algorithms can run over `BigRational` and algebraic field elements.

use crate::sign::Sign;
use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{One, Signed, ToPrimitive, Zero};
use std::cmp::Ordering;

/// Workspace-wide rational scalar type.
pub type Rational = BigRational;

/// Ordered scalar API used by the crate and downstream algorithms.
pub trait OrderedField:
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
    /// Additive identity.
    fn zero() -> Self;

    /// Multiplicative identity.
    fn one() -> Self;

    /// Rational embedding.
    fn from_rational(value: BigRational) -> Self;

    /// Human-readable field name.
    fn field_name() -> &'static str;

    /// Basis labels in canonical coefficient order.
    fn basis_labels() -> Vec<String>;

    /// Exact sign classification.
    fn sign(&self) -> Sign;

    /// Total-order comparison in the chosen real embedding.
    fn cmp_real(&self, other: &Self) -> Ordering {
        match (self.clone() - other.clone()).sign() {
            Sign::Negative => Ordering::Less,
            Sign::Zero => Ordering::Equal,
            Sign::Positive => Ordering::Greater,
        }
    }

    /// Best-effort `f64` approximation for diagnostics and benchmarks.
    fn to_f64(&self) -> f64;

    /// Canonical coefficient vector for stable serialization.
    fn canonical_coeffs(&self) -> Vec<BigRational>;

    /// Small convenience embedding from `i64`.
    fn from_i64(value: i64) -> Self {
        Self::from_rational(BigRational::from_integer(BigInt::from(value)))
    }

    /// Small convenience embedding from a rational pair.
    fn from_frac(numer: i64, denom: i64) -> Self {
        Self::from_rational(BigRational::new(BigInt::from(numer), BigInt::from(denom)))
    }

    /// Exact zero predicate.
    fn is_zero(&self) -> bool {
        self.sign() == Sign::Zero
    }

    /// Exact positivity predicate.
    fn is_positive(&self) -> bool {
        self.sign() == Sign::Positive
    }

    /// Exact negativity predicate.
    fn is_negative(&self) -> bool {
        self.sign() == Sign::Negative
    }
}

impl OrderedField for BigRational {
    fn zero() -> Self {
        <BigRational as Zero>::zero()
    }

    fn one() -> Self {
        <BigRational as One>::one()
    }

    fn from_rational(value: BigRational) -> Self {
        value
    }

    fn field_name() -> &'static str {
        "Q"
    }

    fn basis_labels() -> Vec<String> {
        vec!["1".to_string()]
    }

    fn sign(&self) -> Sign {
        if <BigRational as Zero>::is_zero(self) {
            Sign::Zero
        } else if <BigRational as Signed>::is_positive(self) {
            Sign::Positive
        } else {
            Sign::Negative
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
}

/// Exact comparison helper on any supported scalar type.
pub fn cmp_field<F: OrderedField>(left: &F, right: &F) -> Ordering {
    left.cmp_real(right)
}

/// Exact `min` helper on any supported scalar type.
pub fn min_field<F: OrderedField>(left: F, right: F) -> F {
    if cmp_field(&left, &right).is_gt() {
        right
    } else {
        left
    }
}

/// Exact `max` helper on any supported scalar type.
pub fn max_field<F: OrderedField>(left: F, right: F) -> F {
    if cmp_field(&left, &right).is_lt() {
        right
    } else {
        left
    }
}

/// Dot product in fixed dimension.
pub fn dot<const N: usize, F: OrderedField>(left: &[F; N], right: &[F; N]) -> F {
    let mut sum = F::zero();
    for idx in 0..N {
        sum = sum + left[idx].clone() * right[idx].clone();
    }
    sum
}

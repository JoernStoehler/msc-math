//! Thin scalar bridge from the algebraic-exactness experiment to
//! `algebraic-numbers`.
//!
//! The experiment keeps its exact geometry/KKT code local, but arithmetic and
//! sign-sensitive comparisons now come from the shared companion crate instead
//! of a second hand-written field implementation.

use super::named_field::NamedFieldTag;
use super::pentagon::PentagonField;
use algebraic_numbers::{Algebraic, ExactScalar, RealAlgebraicField};
use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{ToPrimitive, Zero};
use std::cmp::Ordering;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExactSign {
    Negative,
    Zero,
    Positive,
}

/// Experiment-local scalar conveniences kept out of the durable crate API.
pub trait ExactOrderedField: ExactScalar {
    fn from_i64(value: i64) -> Self;
    fn from_frac(numer: i64, denom: i64) -> Self;
    fn generator() -> Self {
        panic!("this scalar backend has no distinguished generator")
    }
    fn canonical_coeffs(&self) -> Vec<BigRational>;
    fn to_f64(&self) -> f64;

    fn sign(&self) -> ExactSign {
        match self.cmp(&Self::zero()) {
            Ordering::Less => ExactSign::Negative,
            Ordering::Equal => ExactSign::Zero,
            Ordering::Greater => ExactSign::Positive,
        }
    }

    fn is_positive(&self) -> bool {
        self > &Self::zero()
    }

    fn is_negative(&self) -> bool {
        self < &Self::zero()
    }
}

impl ExactOrderedField for BigRational {
    fn from_i64(value: i64) -> Self {
        rat(value)
    }

    fn from_frac(numer: i64, denom: i64) -> Self {
        frac(numer, denom)
    }

    fn canonical_coeffs(&self) -> Vec<BigRational> {
        vec![self.clone()]
    }

    fn to_f64(&self) -> f64 {
        ToPrimitive::to_f64(self).expect("experiment rational should fit in f64")
    }
}

impl<F: RealAlgebraicField> ExactOrderedField for Algebraic<F> {
    fn from_i64(value: i64) -> Self {
        Self::from(value)
    }

    fn from_frac(numer: i64, denom: i64) -> Self {
        Self::from(frac(numer, denom))
    }

    fn generator() -> Self {
        Self::root()
    }

    fn canonical_coeffs(&self) -> Vec<BigRational> {
        self.coefficients().to_vec()
    }

    fn to_f64(&self) -> f64 {
        algebraic_to_f64(self)
    }
}

pub fn cmp_field<F: ExactOrderedField>(left: &F, right: &F) -> Ordering {
    left.cmp(right)
}

pub fn max_field<F: ExactOrderedField>(left: F, right: F) -> F {
    left.max(right)
}

pub fn min_field<F: ExactOrderedField>(left: F, right: F) -> F {
    left.min(right)
}

/// Experiment-owned catalog metadata attached to supported scalar backends.
pub trait CatalogField: ExactOrderedField {
    /// Row-level exact field tag used by the experiment-owned exact catalog.
    fn field_tag() -> NamedFieldTag;
}

impl CatalogField for BigRational {
    fn field_tag() -> NamedFieldTag {
        NamedFieldTag::Rational
    }
}

impl CatalogField for PentagonField {
    fn field_tag() -> NamedFieldTag {
        NamedFieldTag::PentagonTanPiFifth
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

fn algebraic_to_f64<F: RealAlgebraicField>(value: &Algebraic<F>) -> f64 {
    if value.is_zero() {
        return 0.0;
    }

    let mut lower = BigRational::from_integer((-1).into());
    let mut upper = BigRational::from_integer(1.into());
    while Algebraic::<F>::from(lower.clone()) > value.clone() {
        lower *= BigRational::from_integer(2.into());
    }
    while Algebraic::<F>::from(upper.clone()) < value.clone() {
        upper *= BigRational::from_integer(2.into());
    }

    for _ in 0..80 {
        let midpoint = (lower.clone() + upper.clone()) / BigRational::from_integer(2.into());
        if Algebraic::<F>::from(midpoint.clone()) <= value.clone() {
            lower = midpoint;
        } else {
            upper = midpoint;
        }
    }

    ToPrimitive::to_f64(&((lower + upper) / BigRational::from_integer(2.into())))
        .expect("bounded algebraic approximation should fit in f64")
}

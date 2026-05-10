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

/// Experiment scalar extras kept out of the durable crate API.
///
/// `algebraic-numbers` owns exact arithmetic and ordering. This trait only adds
/// reporting operations that this experiment needs for JSONL records and
/// f64-side diagnostics.
pub trait ExperimentScalar: ExactScalar {
    fn canonical_coefficients(&self) -> Vec<BigRational>;
    fn to_f64(&self) -> f64;
}

impl ExperimentScalar for BigRational {
    fn canonical_coefficients(&self) -> Vec<BigRational> {
        vec![self.clone()]
    }

    fn to_f64(&self) -> f64 {
        ToPrimitive::to_f64(self).expect("experiment rational should fit in f64")
    }
}

impl<F: RealAlgebraicField> ExperimentScalar for Algebraic<F> {
    fn canonical_coefficients(&self) -> Vec<BigRational> {
        self.coefficients().to_vec()
    }

    fn to_f64(&self) -> f64 {
        algebraic_to_f64(self)
    }
}

pub fn sign_of<F: ExactScalar>(value: &F) -> ExactSign {
    match value.cmp(&F::zero()) {
        Ordering::Less => ExactSign::Negative,
        Ordering::Equal => ExactSign::Zero,
        Ordering::Greater => ExactSign::Positive,
    }
}

pub fn is_strictly_positive<F: ExactScalar>(value: &F) -> bool {
    value > &F::zero()
}

pub fn is_strictly_negative<F: ExactScalar>(value: &F) -> bool {
    value < &F::zero()
}

/// Experiment-owned catalog metadata attached to supported scalar backends.
pub trait CatalogField: ExperimentScalar {
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

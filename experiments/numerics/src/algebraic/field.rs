//! Thin scalar bridge from the algebraic-exactness experiment to
//! `algebraic-numbers`.
//!
//! The experiment keeps its exact geometry/KKT code local, but arithmetic and
//! sign-sensitive comparisons now come from the shared companion crate instead
//! of a second hand-written field implementation.

use super::named_field::NamedFieldTag;
use super::pentagon::PentagonField;
use num_bigint::BigInt;
use num_rational::BigRational;

pub use algebraic_numbers::cmp_field;
pub use algebraic_numbers::max_field;
pub use algebraic_numbers::min_field;
pub use algebraic_numbers::OrderedField as ExactOrderedField;
pub use algebraic_numbers::Sign as ExactSign;

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

//! Purpose: public API for ordered arithmetic over real algebraic extensions of
//! `Q`.
//! Context: this companion crate provides scalar arithmetic, serialization
//! helpers, and a few small linear-algebra routines for experiment and library
//! clients that need trustworthy non-floating arithmetic.
//!
//! Typical usage defines a compile-time field specification and then uses the
//! generic [`Algebraic`] element container:
//!
//! ```rust
//! use num_rational::BigRational;
//! use real_algebraic::{Algebraic, OrderedField, StaticFieldSpec};
//!
//! struct SqrtTwo;
//!
//! impl StaticFieldSpec for SqrtTwo {
//!     fn name() -> &'static str { "Q(sqrt(2))" }
//!     fn generator_name() -> &'static str { "s" }
//!     fn minimal_polynomial() -> Vec<BigRational> {
//!         vec![
//!             BigRational::from_integer((-2).into()),
//!             BigRational::from_integer(0.into()),
//!             BigRational::from_integer(1.into()),
//!         ]
//!     }
//!     fn isolating_interval() -> (BigRational, BigRational) {
//!         (
//!             BigRational::from_integer(1.into()),
//!             BigRational::from_integer(2.into()),
//!         )
//!     }
//! }
//!
//! type SqrtTwoField = Algebraic<SqrtTwo>;
//!
//! let s = SqrtTwoField::generator();
//! let value = (SqrtTwoField::one() + s.clone()) * (SqrtTwoField::one() + s.clone());
//! let expected = SqrtTwoField::from_i64(3) + SqrtTwoField::from_i64(2) * s;
//! assert_eq!(value, expected);
//! ```

mod algebraic;
mod field;
mod linear;
mod named_fields;
mod serialize;
mod sign;
mod spec;

pub use algebraic::Algebraic;
pub use field::{cmp_field, dot, max_field, min_field, OrderedField, Rational};
pub use linear::{rank_rows, solve_square, SolveResult};
pub use named_fields::TanPiFifth;
pub use serialize::{canonical_element, CanonicalElement};
pub use sign::Sign;
pub use spec::{validate_field_spec, FieldSpecError, StaticFieldSpec};

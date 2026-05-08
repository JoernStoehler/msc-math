//! Minimal exact real algebraic scalars.
//!
//! The crate models elements of one statically chosen real algebraic field
//! `Q[alpha]` at a time. An [`Algebraic<F>`] value stores rational coefficients
//! in the basis
//!
//! ```text
//! 1, alpha, alpha^2, ..., alpha^(degree - 1).
//! ```
//!
//! The field marker `F` supplies the monic polynomial for `alpha` and a rational
//! isolating interval selecting the intended real root. This keeps field choices
//! explicit in Rust types and avoids runtime "parent ring" objects.
//!
//! Deliberate non-goals for this small API slice:
//! - no dynamic construction of new fields such as `Q[sqrt(2), sqrt(3)]`;
//! - no `f64` implementation of [`ExactScalar`];
//! - no attempt to implement nalgebra's `RealField`/`ComplexField`, whose API is
//!   shaped around approximate floating-point algorithms;
//! - no matrix solve/diagonalization layer until an actual caller needs it.

mod algebraic_element;
mod arithmetic_ops;
mod exact_scalar;
mod field_specification;
mod polynomial_arithmetic;
mod sign_ordering;

pub use algebraic_element::Algebraic;
pub use exact_scalar::ExactScalar;
pub use field_specification::RealAlgebraicField;

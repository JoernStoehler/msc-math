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
//! - no diagonalization layer; exact callers should start with rank, solve,
//!   kernel, and definiteness checks.

mod algebraic_element;
mod arithmetic_ops;
mod definiteness;
mod exact_scalar;
mod field_specification;
mod linear_solve;
mod polynomial_arithmetic;
mod row_reduction;
mod sign_ordering;

pub use algebraic_element::Algebraic;
pub use definiteness::is_negative_definite;
pub use exact_scalar::ExactScalar;
pub use field_specification::RealAlgebraicField;
pub use linear_solve::{kernel_basis, solve_linear_system, LinearSystemSolution};
pub use row_reduction::{rank, row_reduction, RowReduction};

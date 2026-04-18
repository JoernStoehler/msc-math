//! Purpose: compile-time field specifications for named algebraic fields.
//! Context: Rust lacks dependent types, so named fields are modeled by marker
//! types that provide the defining polynomial and the chosen real root.

use num_rational::BigRational;

/// Compile-time specification of one algebraic field `Q[t] / (p(t))` together
/// with an isolating interval for the chosen real root.
pub trait StaticFieldSpec: 'static {
    /// Human-readable field name used in diagnostics and serialization.
    fn name() -> &'static str;

    /// Symbol used for the generator in basis labels.
    fn generator_name() -> &'static str;

    /// Minimal polynomial in ascending coefficient order.
    ///
    /// Example: `x^2 - 2` is `[-2, 0, 1]`.
    fn minimal_polynomial() -> Vec<BigRational>;

    /// Rational interval `(lo, hi)` isolating the chosen real root.
    fn isolating_interval() -> (BigRational, BigRational);
}

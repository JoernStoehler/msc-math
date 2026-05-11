use std::fmt::Debug;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use num_rational::BigRational;
use num_traits::{One, ToPrimitive, Zero};

/// Exact scalar coefficients for algebra and exact linear-algebra code.
///
/// Implementations are explicit. This avoids accidentally classifying a type as
/// exact just because it happens to expose arithmetic operators. In particular,
/// `f64` is intentionally not an `ExactScalar`.
///
/// ```compile_fail
/// use algebraic_numbers::ExactScalar;
///
/// fn accepts_exact_scalar<T: ExactScalar>() {}
///
/// accepts_exact_scalar::<f64>();
/// ```
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
    /// Lossy IEEE-754 rounding for exact algorithms that can use f64
    /// arithmetic to dismiss impossible candidates before exact fallback.
    fn round_to_f64(&self) -> f64;
}

impl ExactScalar for BigRational {
    fn round_to_f64(&self) -> f64 {
        round_rational_to_f64(self)
    }
}

pub(crate) fn round_rational_to_f64(value: &BigRational) -> f64 {
    value.to_f64().unwrap_or_else(|| {
        if value < &BigRational::zero() {
            f64::NEG_INFINITY
        } else {
            f64::INFINITY
        }
    })
}

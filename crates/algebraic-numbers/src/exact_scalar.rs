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
    /// Optional lossy f64 approximation for exact algorithms that can use
    /// floating-point arithmetic to dismiss impossible candidates before exact
    /// fallback. Returning `None` keeps the caller on the exact-only path.
    fn to_f64_approx(&self) -> Option<f64> {
        None
    }
}

impl ExactScalar for BigRational {
    fn to_f64_approx(&self) -> Option<f64> {
        self.to_f64()
    }
}

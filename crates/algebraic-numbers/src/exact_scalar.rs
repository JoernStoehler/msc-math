use std::fmt::Debug;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use num_rational::BigRational;
use num_traits::{One, Zero};

/// Exact scalar coefficients for algebra and exact linear-algebra code.
///
/// Implementations are explicit. This avoids accidentally classifying a type as
/// exact just because it happens to expose arithmetic operators. In particular,
/// `f64` is intentionally not an `ExactScalar`.
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
}

impl ExactScalar for BigRational {}

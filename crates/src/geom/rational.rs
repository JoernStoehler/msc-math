//! Rational arithmetic utilities for symplectic geometry.
//!
//! Provides:
//! - [`Sign`] enum for exact sign classification
//! - [`omega0_rational`] — standard symplectic form over Q
//! - Scalar conversion between [`f64`] and [`BigRational`]
//! - Small helper constructors ([`rat`], [`frac`])
//! - [`random_small_rational`] for perturbation

use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{Signed, Zero};

/// Sign of an exact rational value.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Sign {
    Plus,
    Minus,
    Zero,
}

impl Sign {
    /// Compute the sign of a rational number.
    pub fn of(r: &BigRational) -> Self {
        if r.is_zero() {
            Sign::Zero
        } else if r.is_positive() {
            Sign::Plus
        } else {
            Sign::Minus
        }
    }
}

/// Standard symplectic form ω₀(u, v) = u₀v₂ - u₂v₀ + u₁v₃ - u₃v₁.
///
/// Same formula as [`super::symplectic::omega0`] but over Q.
/// For dual vertices y_i = n_i/h_i: sign(ω₀(y_i, y_k)) = sign(ω₀(n_i, n_k))
/// since h_i, h_k > 0.
pub(super) fn omega0_rational(u: &[BigRational; 4], v: &[BigRational; 4]) -> BigRational {
    &u[0] * &v[2] - &u[2] * &v[0] + &u[1] * &v[3] - &u[3] * &v[1]
}

// ── Scalar conversion utilities ──────────────────────────────────────────

/// Convert a BigRational to f64 (best approximation).
///
/// For rationals with power-of-2 denominators (the common case from
/// [`f64_to_rational`]), the division is exact in f64 arithmetic and
/// recovers the original value. For general BigRationals with large
/// numerators/denominators, this produces the nearest f64 approximation.
pub(super) fn rational_to_f64(r: &BigRational) -> f64 {
    use num_traits::ToPrimitive;
    let numer: f64 = r.numer().to_f64().unwrap_or(f64::NAN);
    let denom: f64 = r.denom().to_f64().unwrap_or(f64::NAN);
    numer / denom
}

/// Lossless conversion from f64 to exact BigRational.
///
/// Every finite f64 is exactly m · 2^e for some integer mantissa m and
/// exponent e. This function extracts (m, e) from the IEEE-754 bit
/// representation and constructs the exact rational m / 2^(-e) or m · 2^e.
///
/// Panics on NaN or infinity.
pub fn f64_to_rational(x: f64) -> BigRational {
    assert!(
        x.is_finite(),
        "f64_to_rational: input must be finite, got {x}"
    );
    if x == 0.0 {
        return BigRational::zero();
    }
    let bits = x.to_bits();
    let sign = if bits >> 63 == 0 { 1i64 } else { -1i64 };
    let exponent = ((bits >> 52) & 0x7FF) as i64;
    let mantissa = if exponent == 0 {
        // Subnormal: mantissa without implicit 1
        (bits & 0x000F_FFFF_FFFF_FFFF) as i64
    } else {
        // Normal: mantissa with implicit 1
        ((bits & 0x000F_FFFF_FFFF_FFFF) | 0x0010_0000_0000_0000) as i64
    };
    let e = if exponent == 0 {
        1 - 1023 - 52 // Subnormal exponent
    } else {
        exponent - 1023 - 52 // Normal exponent
    };

    let numer = BigInt::from(sign * mantissa);
    if e >= 0 {
        let scale = BigInt::from(1u64) << (e as u64);
        BigRational::new(numer * scale, BigInt::from(1))
    } else {
        let scale = BigInt::from(1u64) << ((-e) as u64);
        BigRational::new(numer, scale)
    }
}

/// Generate a random rational number with magnitude < 2^{-bits}.
///
/// Uses uniform random numerator in [-2^32, 2^32) and denominator 2^{bits+32}.
/// This gives numbers like k / 2^{bits+32} for random k, which are exact
/// rationals with bounded denominator size.
pub(super) fn random_small_rational(rng: &mut impl rand::Rng, bits: u32) -> BigRational {
    let numer: i64 = rng.gen_range(-(1i64 << 32)..(1i64 << 32));
    let denom = BigInt::from(1u64) << (bits as u64 + 32);
    BigRational::new(BigInt::from(numer), denom)
}

/// Helper: create a BigRational from an integer.
pub fn rat(n: i64) -> BigRational {
    BigRational::from(BigInt::from(n))
}

/// Helper: create a BigRational from a fraction.
pub fn frac(numer: i64, denom: i64) -> BigRational {
    BigRational::new(BigInt::from(numer), BigInt::from(denom))
}

#[cfg(test)]
#[path = "rational_test.rs"]
mod rational_test;

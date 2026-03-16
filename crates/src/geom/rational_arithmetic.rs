//! Exact rational number type and arithmetic operations for symplectic geometry.
//!
//! Provides exact rational arithmetic utilities used by the vertex enumeration
//! pipeline and perturbation system. All discrete/combinatorial decisions in the
//! crate (adjacency, omega signs, irredundancy) use exact rational arithmetic
//! to avoid floating-point ambiguity.
//!
//! Key functions:
//! - [`f64_to_rational`]: lossless IEEE-754 to BigRational conversion
//! - [`omega0_rational`]: standard symplectic form over Q
//! - [`Sign`]: exact sign classification for rational values
//!
//! Mathematical correspondence: [def:symplectic-form], [lem:rational-pipeline]

use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{Signed, Zero};

/// Exact sign of a rational value.
///
/// Used to classify symplectic form values ω₀(y_i, y_k) without
/// floating-point ambiguity.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Sign {
    /// Strictly positive.
    Plus,
    /// Strictly negative.
    Minus,
    /// Exactly zero.
    Zero,
}

impl Sign {
    /// Classify the sign of a rational number.
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

/// Standard symplectic form ω₀(u, v) over Q.
///
/// In coordinates (q₁, q₂, p₁, p₂):
///   ω₀(u, v) = u₀v₂ - u₂v₀ + u₁v₃ - u₃v₁
///
/// Same formula as `symplectic_form::omega0` but exact over Q.
/// For dual vertices y_i = n_i/h_i: sign(ω₀(y_i, y_k)) = sign(ω₀(n_i, n_k))
/// since h_i, h_k > 0.
///
/// Mathematical correspondence: [def:symplectic-form]
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
/// Every finite f64 is exactly m * 2^e for integer mantissa m and exponent e.
/// This function extracts (m, e) from the IEEE-754 bit representation and
/// constructs the exact rational m / 2^(-e) or m * 2^e.
///
/// # Panics
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
        // Subnormal: mantissa without implicit leading 1
        (bits & 0x000F_FFFF_FFFF_FFFF) as i64
    } else {
        // Normal: mantissa with implicit leading 1
        ((bits & 0x000F_FFFF_FFFF_FFFF) | 0x0010_0000_0000_0000) as i64
    };
    let e = if exponent == 0 {
        1 - 1023 - 52 // Subnormal exponent bias
    } else {
        exponent - 1023 - 52 // Normal exponent bias
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
/// This produces exact rationals k / 2^{bits+32} for random k, with bounded
/// denominator size. Used for perturbation of degenerate polytopes.
pub(super) fn random_small_rational(rng: &mut impl rand::Rng, bits: u32) -> BigRational {
    let numer: i64 = rng.gen_range(-(1i64 << 32)..(1i64 << 32));
    let denom = BigInt::from(1u64) << (bits as u64 + 32);
    BigRational::new(BigInt::from(numer), denom)
}

/// Create a BigRational from an integer.
pub fn rat(n: i64) -> BigRational {
    BigRational::from(BigInt::from(n))
}

/// Create a BigRational from a fraction n/d.
pub fn frac(numer: i64, denom: i64) -> BigRational {
    BigRational::new(BigInt::from(numer), BigInt::from(denom))
}

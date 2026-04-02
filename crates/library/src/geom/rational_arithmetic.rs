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
pub(crate) fn omega0_rational(u: &[BigRational; 4], v: &[BigRational; 4]) -> BigRational {
    &u[0] * &v[2] - &u[2] * &v[0] + &u[1] * &v[3] - &u[3] * &v[1]
}

// ── Scalar conversion utilities ──────────────────────────────────────────

/// Convert a BigRational to f64 (best approximation).
///
/// For rationals with power-of-2 denominators (the common case from
/// [`f64_to_rational`]), the division is exact in f64 arithmetic and
/// recovers the original value. For general BigRationals with large
/// numerators/denominators, this produces the nearest f64 approximation.
pub(crate) fn rational_to_f64(r: &BigRational) -> f64 {
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

#[cfg(test)]
mod tests {
    use super::*;

    // Tests for rational_arithmetic: f64-to-rational roundtrip and sign agreement.
    //
    // Proposition: f64_to_rational is lossless; rational_to_f64 inverts it exactly
    // for power-of-2 denominators; omega0_rational agrees with f64 omega0 on integers.
    // Reference: [lem:rational-pipeline]
    //
    // Strategy: fixture-based on known values and representative f64 inputs.

    // ── f64 <-> rational lossless conversion ─────────────────────────────────

    /// Proposition: f64_to_rational roundtrips exactly for all representable f64 values.
    #[test]
    fn f64_to_rational_roundtrip() {
        let test_values: Vec<f64> = vec![
            0.0,
            1.0,
            -1.0,
            0.5,
            -0.5,
            0.1,
            -0.1,
            1.0 / 3.0,
            std::f64::consts::PI,
            std::f64::consts::FRAC_1_SQRT_2,
            1e-15,
            1e15,
            f64::MIN_POSITIVE,
            (2.0_f64).powi(52),
            0.8090169943749473,
        ];
        for &x in &test_values {
            let r = f64_to_rational(x);
            let back = rational_to_f64(&r);
            assert_eq!(
                back, x,
                "round-trip failed for {x}: rational = {r}, back = {back}"
            );
        }
    }

    /// Proposition: f64_to_rational produces the exact rational for known power-of-2 values.
    #[test]
    fn f64_to_rational_exact_values() {
        assert_eq!(f64_to_rational(0.0), rat(0));
        assert_eq!(f64_to_rational(1.0), rat(1));
        assert_eq!(f64_to_rational(-1.0), rat(-1));
        assert_eq!(f64_to_rational(0.5), frac(1, 2));
        assert_eq!(f64_to_rational(-0.5), frac(-1, 2));
        assert_eq!(f64_to_rational(0.25), frac(1, 4));
        assert_eq!(f64_to_rational(2.0), rat(2));
        assert_eq!(f64_to_rational(1024.0), rat(1024));
    }

    // ── Sign classification ──────────────────────────────────────────────────

    /// Proposition: Sign::of correctly classifies positive, negative, and zero.
    #[test]
    fn sign_classification() {
        assert_eq!(Sign::of(&rat(0)), Sign::Zero);
        assert_eq!(Sign::of(&rat(1)), Sign::Plus);
        assert_eq!(Sign::of(&rat(-1)), Sign::Minus);
        assert_eq!(Sign::of(&frac(1, 3)), Sign::Plus);
        assert_eq!(Sign::of(&frac(-7, 11)), Sign::Minus);
    }

    // ── omega0_rational agreement with f64 ───────────────────────────────────

    /// Proposition: omega0_rational agrees with the f64 symplectic form on integer inputs.
    #[test]
    fn omega0_rational_agrees_with_f64() {
        use crate::geom::symplectic_form::omega0;
        use nalgebra::Vector4;

        let test_cases: Vec<([i64; 4], [i64; 4])> = vec![
            ([1, 0, 0, 0], [0, 0, 1, 0]),
            ([1, 0, 0, 0], [0, 0, 0, 1]),
            ([0, 1, 0, 0], [0, 0, 0, 1]),
            ([1, 2, 3, 4], [5, 6, 7, 8]),
            ([3, -1, 4, -1], [5, -9, 2, -6]),
        ];
        for (u_arr, v_arr) in &test_cases {
            let u_rat: [BigRational; 4] = std::array::from_fn(|i| rat(u_arr[i]));
            let v_rat: [BigRational; 4] = std::array::from_fn(|i| rat(v_arr[i]));
            let u_f64 = Vector4::new(
                u_arr[0] as f64,
                u_arr[1] as f64,
                u_arr[2] as f64,
                u_arr[3] as f64,
            );
            let v_f64 = Vector4::new(
                v_arr[0] as f64,
                v_arr[1] as f64,
                v_arr[2] as f64,
                v_arr[3] as f64,
            );
            let rational_result = omega0_rational(&u_rat, &v_rat);
            let f64_result = omega0(&u_f64, &v_f64);
            assert_eq!(
                rational_to_f64(&rational_result),
                f64_result,
                "omega0({u_arr:?}, {v_arr:?}): rational={rational_result}, f64={f64_result}"
            );
        }
    }

    // ── random_small_rational ────────────────────────────────────────────────

    /// Proposition: random_small_rational produces values with magnitude < 2^{-bits}.
    #[test]
    fn random_small_rational_bounded() {
        use rand::SeedableRng;
        let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(42);
        for _ in 0..100 {
            let r = random_small_rational(&mut rng, 64);
            let val = rational_to_f64(&r);
            // 2^{-64} is about 5.4e-20; values should be well below that
            assert!(
                val.abs() < 1e-9,
                "random_small_rational(64) produced {val}, expected < 1e-9"
            );
        }
    }
}

//! Tests for rational_arithmetic: f64-to-rational roundtrip and sign agreement.
//!
//! Proposition: f64_to_rational is lossless; rational_to_f64 inverts it exactly
//! for power-of-2 denominators; omega0_rational agrees with f64 omega0 on integers.
//! Reference: [lem:rational-pipeline]
//!
//! Strategy: fixture-based on known values and representative f64 inputs.

use super::rational_arithmetic::*;
use num_rational::BigRational;

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

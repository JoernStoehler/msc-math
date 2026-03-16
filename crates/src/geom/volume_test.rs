//! Tests for volume: computation vs known values for standard polytopes.
//!
//! Proposition: volume(K) agrees with known exact values:
//!   simplex = 1/24, hypercube = 16, crosspolytope = 32/3.
//! Reference: [def:volume]
//!
//! Strategy: fixture-based (simplex, hypercube, crosspolytope) + qhull triangulation

use crate::geom::known_polytopes;
use crate::geom::test_utils::{crosspolytope, scaled_hypercube};
use crate::geom::volume::{simplex_volume_5, volume};
use nalgebra::Vector4;

/// Verify that the 4-simplex has volume 1/24 via direct vertex computation.
#[test]
fn simplex_4d_volume_from_vertices() {
    // Standard 4-simplex: conv{0, e1, e2, e3, e4}
    // Volume = 1/24
    let v0 = Vector4::zeros();
    let v1 = Vector4::x();
    let v2 = Vector4::y();
    let v3 = Vector4::z();
    let v4 = Vector4::w();

    let vol = simplex_volume_5(v0, v1, v2, v3, v4);
    assert!(
        (vol - 1.0 / 24.0).abs() < 1e-10,
        "simplex volume: got {vol}, expected {}",
        1.0 / 24.0
    );
}

/// Verify that the hypercube [-1,1]^4 has volume 2^4 = 16.
#[test]
fn hypercube_volume() {
    // [-1, 1]^4 has volume 2^4 = 16
    let polytope = known_polytopes::hypercube().polytope;
    let vol = volume(&polytope).expect("volume computation failed");
    assert!(
        (vol - 16.0).abs() < 1e-6,
        "hypercube volume: got {vol}, expected 16"
    );
}

/// Verify that the simplex polytope has volume 1/24.
#[test]
fn simplex_polytope_volume() {
    // Standard simplex, volume = 1/24
    let polytope = known_polytopes::simplex().polytope;
    let vol = volume(&polytope).expect("volume computation failed");
    assert!(
        (vol - 1.0 / 24.0).abs() < 1e-6,
        "simplex polytope volume: got {vol}, expected {}",
        1.0 / 24.0
    );
}

/// Verify that the 4D crosspolytope has volume 32/3.
#[test]
fn crosspolytope_volume() {
    // 4D crosspolytope: conv{+/-e1, +/-e2, +/-e3, +/-e4} (after vertex enumeration).
    // With our normalization (normals (+/-1,+/-1,+/-1,+/-1)/2, heights 1.0),
    // the vertices are at +/-2*e_i. Vol = 2^n / n! * (2)^n = 32/3 for edge half-length 2.
    let polytope = crosspolytope();
    let vol = volume(&polytope).expect("volume computation failed");
    let expected = 32.0 / 3.0;
    assert!(
        (vol - expected).abs() < 1e-6,
        "crosspolytope volume: got {vol}, expected {expected}"
    );
}

/// Verify vol(s*K) = s^4 * vol(K) for the hypercube at several scales.
#[test]
fn scaling_property() {
    // vol(s*K) = s^4 * vol(K) for the hypercube [-s,s]^4.
    let base_vol = volume(&scaled_hypercube(1.0)).expect("volume computation failed");
    for &s in &[0.5, 2.0, 3.0, 0.1] {
        let scaled_vol = volume(&scaled_hypercube(s)).expect("volume computation failed");
        let expected = base_vol * s.powi(4);
        assert!(
            (scaled_vol - expected).abs() < 1e-4,
            "scaling: vol({s}*cube) = {scaled_vol}, expected {expected}"
        );
    }
}

/// Verify that volume is positive for all known polytope fixtures.
#[test]
fn volume_positive_for_known_polytopes() {
    for kp in known_polytopes::all_known() {
        let vol = volume(&kp.polytope).expect("volume computation failed");
        assert!(
            vol > 0.0,
            "{}: volume should be positive, got {vol}",
            kp.name
        );
    }
}

#[cfg(test)]
mod proptests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        /// Property: volume scaling vol(s*K) = s^4 * vol(K).
        #[test]
        fn volume_scales_with_fourth_power(scale in 0.1f64..10.0) {
            let unit_cube = scaled_hypercube(1.0);
            let scaled_cube = scaled_hypercube(scale);

            let vol_unit = volume(&unit_cube).expect("volume computation failed");
            let vol_scaled = volume(&scaled_cube).expect("volume computation failed");

            let expected_scaled = vol_unit * scale.powi(4);
            let relative_error = ((vol_scaled - expected_scaled) / expected_scaled).abs();

            prop_assert!(
                relative_error < 1e-4,
                "volume scaling failed: scale={}, vol_unit={}, vol_scaled={}, expected={}, rel_error={}",
                scale, vol_unit, vol_scaled, expected_scaled, relative_error
            );
        }
    }
}

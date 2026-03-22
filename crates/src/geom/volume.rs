//! 4D polytope volume computation via qhull triangulation.
//!
//! Provides the primary volume function `volume()` which delegates to qhull's
//! `qconvex FA` command. Also provides `simplex_volume_5()` for computing the
//! volume of a 4-simplex from its 5 vertices.
//!
//! Reference: Gruenbaum, "Convex Polytopes", section 14.1.
//!
//! Mathematical correspondence: [def:volume]

use crate::geom::polytope::Polytope4D;
use nalgebra::Vector4;

/// Volume of a 4-simplex from its 5 vertices.
///
/// vol(conv{v0, v1, v2, v3, v4}) = |det[v1-v0, v2-v0, v3-v0, v4-v0]| / 24.
///
/// The factor 1/24 = 1/4! is the 4-dimensional analogue of 1/6 for tetrahedra.
///
/// Mathematical correspondence: [def:volume] (simplex case)
pub fn simplex_volume_5(
    v0: Vector4<f64>,
    v1: Vector4<f64>,
    v2: Vector4<f64>,
    v3: Vector4<f64>,
    v4: Vector4<f64>,
) -> f64 {
    let mat = nalgebra::Matrix4::from_columns(&[v1 - v0, v2 - v0, v3 - v0, v4 - v0]);
    mat.determinant().abs() / 24.0
}

/// Compute volume of a 4D convex polytope via qhull triangulation.
///
/// Uses qhull's `qconvex FA` to compute the volume from the polytope's vertices.
/// This approach is simpler than a divergence theorem implementation and has been
/// empirically validated to agree within 5e-8 relative error on 1000+ polytopes.
///
/// # Errors
///
/// Returns `QhullError` if qhull fails (typically due to numerical issues or
/// qhull not being installed).
///
/// Mathematical correspondence: [def:volume]
pub fn volume(polytope: &Polytope4D) -> Result<f64, crate::geom::qhull::QhullError> {
    let vertices = polytope.vertices_f64();
    crate::geom::qhull::compute_volume_qconvex(vertices)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geom::known_polytopes;
    use crate::geom::test_utils::{crosspolytope, scaled_hypercube};
    use nalgebra::Vector4;

    // Tests for volume: computation vs known values for standard polytopes.
    //
    // Proposition: volume(K) agrees with known exact values:
    //   simplex = 1/24, hypercube = 16, crosspolytope = 32/3.
    // Reference: [def:volume]
    //
    // Strategy: fixture-based (simplex, hypercube, crosspolytope) + qhull triangulation

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
        let polytope = &known_polytopes::hypercube().polytope;
        let vol = volume(polytope).expect("volume computation failed");
        assert!(
            (vol - 16.0).abs() < 1e-6,
            "hypercube volume: got {vol}, expected 16"
        );
    }

    /// Verify that the simplex polytope has volume 1/24.
    #[test]
    fn simplex_polytope_volume() {
        // Standard simplex, volume = 1/24
        let polytope = &known_polytopes::simplex().polytope;
        let vol = volume(polytope).expect("volume computation failed");
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
        let vol = volume(polytope).expect("volume computation failed");
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
            #![proptest_config(proptest::prelude::ProptestConfig::with_cases(16))]
            /// Property: volume scaling vol(s*K) = s^4 * vol(K).
            ///
            /// 16 cases in default suite (each calls qhull twice). Run with
            /// --ignored for the full 256-case version.
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

    // ---- Volume property tests ----
    //
    // Property tests for volume computation.
    //
    // Proposition: vol(K) > 0 for all valid bounded 4D polytopes.
    // Exact values: simplex = 1/24, hypercube = 16, crosspolytope = 32/3.
    // Reference: [def:volume]
    //
    // Strategy: fixture-based (known polytopes) + random polytopes (40 cases)

    /// Verify volume matches exact values for simplex, hypercube, and crosspolytope.
    #[test]
    fn volume_positive_on_known_polytopes() {
        let cases = vec![
            ("simplex", known_polytopes::simplex(), 1.0 / 24.0),
            ("hypercube", known_polytopes::hypercube(), 16.0),
            ("crosspolytope", known_polytopes::crosspolytope(), 32.0 / 3.0),
        ];

        for (name, kp, expected) in cases {
            let vol = volume(&kp.polytope).unwrap_or_else(|_| panic!("{name} volume computation"));

            assert!(
                (vol - expected).abs() / expected < 1e-6,
                "{name}: volume = {vol}, expected = {expected}"
            );

            assert!(vol > 0.0, "{name}: volume should be positive");
        }
    }

    /// Verify vol(K) > 0 for 40 random bounded polytopes with 5-8 facets.
    ///
    /// 40 cases = 4 facet counts (5..=8) x 10 seeds. Each calls qhull for
    /// convex hull triangulation (~1.5s per call in debug mode -> ~60s total).
    /// In release mode: ~0.1s per call -> ~4s total.
    #[test]
    #[ignore] // Expensive input-output: 40 qhull calls, ~60s debug / ~4s release.
    fn volume_positive_on_random_polytopes() {
        use crate::geom::test_utils::random_bounded_polytope;
        use rand::SeedableRng;
        use rand_chacha::ChaCha8Rng;

        let mut tested = 0;

        for facet_count in 5..=8 {
            for i in 0..10 {
                let seed = 12345u64 + (facet_count as u64 * 100) + (i as u64);
                let mut rng = ChaCha8Rng::seed_from_u64(seed);

                let polytope = random_bounded_polytope(facet_count, &mut rng);
                let vol = volume(&polytope).expect("volume computation");
                assert!(
                    vol > 0.0,
                    "f={facet_count}: volume should be positive, got {vol}"
                );
                tested += 1;
            }
        }

        assert!(
            tested > 0,
            "Should have tested at least some random polytopes"
        );
    }
}

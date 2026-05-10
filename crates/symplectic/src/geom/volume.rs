//! 4D polytope volume computation via origin-star triangulation.
//!
//! The canonical `volume()` path is pure Rust. It uses the exact vertex-facet
//! incidence stored on [`Polytope4D`] to triangulate each 3-facet from an
//! interior facet point, then cones those tetrahedra to the origin. The qhull
//! subprocess wrapper remains available as `volume_qhull()` for verification
//! and benchmarking only.
//!
//! Mathematical correspondence: [def:volume], [lem:volume-star-triangulation]

use crate::geom::polytope::Polytope4D;
use crate::geom::qhull::QhullError;
use euclidean_polytopes::volume_from_incidence_f64;
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

/// Compute volume of a 4D convex polytope by coning a facet triangulation to `0`.
///
/// Since [`Polytope4D`] is stored in normalized H-representation
/// `K = {x : a_i^T x <= 1}`, the origin lies strictly in the interior of every
/// valid polytope. For each 3-facet `F_i`, we triangulate its boundary ridges
/// from the arithmetic mean of its vertices, producing tetrahedra that fill
/// `F_i`. Coning those tetrahedra to `0` gives a 4-simplex decomposition of `K`.
///
/// Mathematical correspondence: [def:volume], [lem:volume-star-triangulation]
pub fn volume(polytope: &Polytope4D) -> f64 {
    volume_from_incidence_f64(polytope.vertices_f64(), polytope.incidence())
        .expect("valid Polytope4D has finite f64 vertices and matching incidence")
}

/// Compute volume of a 4D convex polytope via qhull triangulation.
///
/// This is retained for validation and performance comparison with the pure-Rust
/// canonical implementation. Dataset producers and the public `volume()` API do
/// not depend on qhull.
pub fn volume_qhull(polytope: &Polytope4D) -> Result<f64, QhullError> {
    let vertices = polytope.vertices_f64();
    crate::geom::qhull::compute_volume_qconvex(vertices)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geom::known_polytopes;
    use crate::geom::test_utils::{crosspolytope, scaled_hypercube};
    use euclidean_polytopes::volume_from_incidence_f64;
    use nalgebra::Vector4;

    // Tests for volume: computation vs known values for standard polytopes.
    //
    // Proposition: volume(K) agrees with known exact values:
    //   simplex = 1/24, hypercube = 16, crosspolytope = 32/3.
    // Reference: [def:volume]
    //
    // Strategy: fixture-based (simplex, hypercube, crosspolytope) + qhull cross-check

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
        let vol = volume(polytope);
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
        let vol = volume(polytope);
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
        let vol = volume(polytope);
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
        let base_vol = volume(&scaled_hypercube(1.0));
        for &s in &[0.5, 2.0, 3.0, 0.1] {
            let scaled_vol = volume(&scaled_hypercube(s));
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
            let vol = volume(&kp.polytope);
            assert!(
                vol > 0.0,
                "{}: volume should be positive, got {vol}",
                kp.name
            );
        }
    }

    /// Wiring regression: the compatibility wrapper
    /// `symplectic::geom::volume::volume` delegates to the Euclidean
    /// known-incidence helper on valid `Polytope4D` fixtures.
    ///
    /// Mathematical correctness is covered by the exact-value and qhull tests
    /// above and below; this test protects the cross-crate migration boundary.
    ///
    /// Operationalization: compare all known fixtures, using exact
    /// `Polytope4D` incidence and f64 vertex copies. Tolerance:
    /// `max(1e-10, 1e-10 * |volume|)`.
    #[test]
    fn volume_wrapper_matches_euclidean_known_incidence_helper() {
        for kp in known_polytopes::all_known() {
            let wrapper_volume = volume(&kp.polytope);
            let euclidean_volume =
                volume_from_incidence_f64(kp.polytope.vertices_f64(), kp.polytope.incidence())
                    .expect("valid Polytope4D fixture");
            let allowed_error = 1.0e-10_f64.max(1.0e-10 * euclidean_volume.abs());

            assert!(
                (wrapper_volume - euclidean_volume).abs() <= allowed_error,
                "{}: wrapper volume = {wrapper_volume}, euclidean volume = {euclidean_volume}",
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
            /// 16 cases in default suite (each evaluates `volume()` twice). Run with
            /// --ignored for the full 256-case version.
            #[test]
            fn volume_scales_with_fourth_power(scale in 0.1f64..10.0) {
                let unit_cube = scaled_hypercube(1.0);
                let scaled_cube = scaled_hypercube(scale);

                let vol_unit = volume(&unit_cube);
                let vol_scaled = volume(&scaled_cube);

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
            (
                "crosspolytope",
                known_polytopes::crosspolytope(),
                32.0 / 3.0,
            ),
        ];

        for (name, kp, expected) in cases {
            let vol = volume(&kp.polytope);

            assert!(
                (vol - expected).abs() / expected < 1e-6,
                "{name}: volume = {vol}, expected = {expected}"
            );

            assert!(vol > 0.0, "{name}: volume should be positive");
        }
    }

    /// Verify vol(K) > 0 for 40 random bounded polytopes with 5-8 facets.
    ///
    /// 40 cases = 4 facet counts (5..=8) x 10 seeds. This keeps a broader
    /// random positivity check out of the default fast suite.
    #[test]
    #[ignore] // Broader random sweep; keep ignored so default library tests stay fast.
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
                let vol = volume(&polytope);
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

    /// Cross-check the pure-Rust canonical path against qhull when available.
    #[test]
    fn volume_matches_qhull_on_known_polytopes() {
        for kp in known_polytopes::all_known() {
            let rust_vol = volume(&kp.polytope);
            let qhull_vol = match volume_qhull(&kp.polytope) {
                Ok(vol) => vol,
                Err(QhullError::QhullNotInstalled) => return,
                Err(err) => panic!("{} qhull cross-check failed: {err}", kp.name),
            };

            let rel_err = ((rust_vol - qhull_vol) / qhull_vol).abs();
            assert!(
                rel_err < 1e-6,
                "{}: rust volume = {rust_vol}, qhull volume = {qhull_vol}, rel_err = {rel_err}",
                kp.name
            );
        }
    }
}

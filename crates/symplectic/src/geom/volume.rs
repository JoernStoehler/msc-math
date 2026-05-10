//! 4D polytope volume computation via exact known-incidence triangulation.
//!
//! The canonical source of truth is [`volume_exact`]. It converts the exact
//! vertices stored on [`Polytope4D`] to Euclidean `Vector4<BigRational>` values
//! and delegates to `euclidean_polytopes::volume_from_incidence_exact` with the
//! stored exact vertex-facet incidence. [`volume_f64`] is the explicit f64
//! projection of that exact result. The qhull subprocess helper remains
//! available as `volume_qhull()` for verification and benchmarking only.
//!
//! Mathematical correspondence: [def:volume], [lem:volume-star-triangulation]

use crate::geom::polytope::Polytope4D;
use crate::geom::qhull::QhullError;
use crate::geom::rational_arithmetic::rational_to_f64;
use euclidean_polytopes::volume_from_incidence_exact;
use nalgebra::Vector4;
use num_rational::BigRational;

/// Compute exact volume of a 4D convex polytope from stored exact incidence.
///
/// Since [`Polytope4D`] is stored in normalized H-representation
/// `K = {x : a_i^T x <= 1}`, the origin lies strictly in the interior of every
/// valid polytope. This function does not recompute incidence, does not use
/// dual vertices, and does not convert vertices to f64. It delegates the exact
/// origin-star triangulation to `euclidean-polytopes`.
///
/// Mathematical correspondence: [def:volume], [lem:volume-star-triangulation]
pub fn volume_exact(polytope: &Polytope4D) -> BigRational {
    let vertices = vertices_as_vector4_exact(polytope.vertices());
    volume_from_incidence_exact(&vertices, polytope.incidence())
}

/// Compute f64 volume of a 4D convex polytope by converting the exact volume.
///
/// Use [`volume_exact`] when exact rational output is needed. This function is
/// for callers whose numeric workflow is f64 and converts the exact
/// source-of-truth value with `rational_to_f64`.
///
/// Mathematical correspondence: [def:volume], [lem:volume-star-triangulation]
pub fn volume_f64(polytope: &Polytope4D) -> f64 {
    rational_to_f64(&volume_exact(polytope))
}

/// Compute volume of a 4D convex polytope via qhull triangulation.
///
/// This is retained for validation and performance comparison with the pure-Rust
/// canonical implementation. Dataset producers and the public `volume_f64()` API do
/// not depend on qhull.
pub fn volume_qhull(polytope: &Polytope4D) -> Result<f64, QhullError> {
    let vertices = polytope.vertices_f64();
    crate::geom::qhull::compute_volume_qconvex(vertices)
}

fn vertices_as_vector4_exact(vertices: &[[BigRational; 4]]) -> Vec<Vector4<BigRational>> {
    vertices
        .iter()
        .map(|vertex| {
            Vector4::new(
                vertex[0].clone(),
                vertex[1].clone(),
                vertex[2].clone(),
                vertex[3].clone(),
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geom::known_polytopes;
    use crate::geom::rational_arithmetic::{frac, rat};
    use crate::geom::test_utils::{crosspolytope, scaled_hypercube};

    // Tests for volume: computation vs known values for standard polytopes.
    //
    // Proposition: volume_f64(K) agrees with known exact values:
    //   simplex = 1/24, hypercube = 16, crosspolytope = 32/3.
    // Reference: [def:volume]
    //
    // Strategy: fixture-based (simplex, hypercube, crosspolytope) + qhull cross-check

    /// Verify that the hypercube [-1,1]^4 has volume 2^4 = 16.
    #[test]
    fn hypercube_volume() {
        // [-1, 1]^4 has volume 2^4 = 16
        let polytope = &known_polytopes::hypercube().polytope;
        let vol = volume_f64(polytope);
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
        let vol = volume_f64(polytope);
        assert!(
            (vol - 1.0 / 24.0).abs() < 1e-6,
            "simplex polytope volume: got {vol}, expected {}",
            1.0 / 24.0
        );
    }

    /// Verify exact fixture volumes for the standard rational fixtures.
    #[test]
    fn exact_fixture_volumes_match_known_values() {
        let cases = vec![
            ("simplex", known_polytopes::simplex(), frac(1, 24)),
            ("hypercube", known_polytopes::hypercube(), rat(16)),
            (
                "crosspolytope",
                known_polytopes::crosspolytope(),
                frac(32, 3),
            ),
        ];

        for (name, kp, expected) in cases {
            assert_eq!(
                volume_exact(&kp.polytope),
                expected,
                "{name}: exact volume mismatch"
            );
        }
    }

    /// Verify that the 4D crosspolytope has volume 32/3.
    #[test]
    fn crosspolytope_volume() {
        // 4D crosspolytope: conv{+/-e1, +/-e2, +/-e3, +/-e4} (after vertex enumeration).
        // With our normalization (normals (+/-1,+/-1,+/-1,+/-1)/2, heights 1.0),
        // the vertices are at +/-2*e_i. Vol = 2^n / n! * (2)^n = 32/3 for edge half-length 2.
        let polytope = crosspolytope();
        let vol = volume_f64(polytope);
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
        let base_vol = volume_f64(&scaled_hypercube(1.0));
        for &s in &[0.5, 2.0, 3.0, 0.1] {
            let scaled_vol = volume_f64(&scaled_hypercube(s));
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
            let vol = volume_f64(&kp.polytope);
            assert!(
                vol > 0.0,
                "{}: volume should be positive, got {vol}",
                kp.name
            );
        }
    }

    /// Wiring regression: the symplectic exact volume API
    /// `symplectic::geom::volume::volume_exact` delegates to the Euclidean
    /// exact known-incidence helper on valid `Polytope4D` fixtures, and
    /// `volume_f64` converts that exact value to f64.
    ///
    /// Mathematical correctness is covered by the exact-value and qhull tests
    /// above and below; this test protects the cross-crate migration boundary.
    ///
    /// Operationalization: compare all known fixtures, using exact
    /// `Polytope4D` vertices and incidence. Tolerance for the f64 projection is
    /// `max(1e-10, 1e-10 * |volume|)`.
    #[test]
    fn volume_api_matches_euclidean_known_incidence_helper() {
        for kp in known_polytopes::all_known() {
            let symplectic_exact = volume_exact(&kp.polytope);
            let euclidean_exact = volume_from_incidence_exact(
                &vertices_as_vector4_exact(kp.polytope.vertices()),
                kp.polytope.incidence(),
            );
            assert_eq!(
                symplectic_exact, euclidean_exact,
                "{}: symplectic exact volume differs from Euclidean exact helper",
                kp.name
            );

            let symplectic_volume = volume_f64(&kp.polytope);
            let euclidean_volume = rational_to_f64(&euclidean_exact);
            let allowed_error = 1.0e-10_f64.max(1.0e-10 * euclidean_volume.abs());

            assert!(
                (symplectic_volume - euclidean_volume).abs() <= allowed_error,
                "{}: symplectic volume = {symplectic_volume}, euclidean volume = {euclidean_volume}",
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
            /// 16 cases in default suite (each evaluates `volume_f64()` twice). Run with
            /// --ignored for the full 256-case version.
            #[test]
            fn volume_scales_with_fourth_power(scale in 0.1f64..10.0) {
                let unit_cube = scaled_hypercube(1.0);
                let scaled_cube = scaled_hypercube(scale);

                let vol_unit = volume_f64(&unit_cube);
                let vol_scaled = volume_f64(&scaled_cube);

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
            let vol = volume_f64(&kp.polytope);

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
                let vol = volume_f64(&polytope);
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
            let rust_vol = volume_f64(&kp.polytope);
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

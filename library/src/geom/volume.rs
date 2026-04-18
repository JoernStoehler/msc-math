//! 4D polytope volume computation via origin-star triangulation.
//!
//! The canonical `volume()` path is pure Rust. It uses the exact vertex-facet
//! incidence stored on [`Polytope4D`] to triangulate each 3-facet from an
//! interior facet point, then cones those tetrahedra to the origin. The qhull
//! subprocess wrapper remains available as `volume_qhull()` for verification
//! and benchmarking only.
//!
//! Mathematical correspondence: [def:volume], [lem:volume-star-triangulation]

use crate::geom::polygon_order::sort_polygon_order;
use crate::geom::polytope::Polytope4D;
use crate::geom::qhull::QhullError;
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
    let vertices = polytope.vertices_f64();
    if vertices.len() < 5 {
        return 0.0;
    }

    let facet_vertices = facet_vertex_indices(polytope);
    let facet_centroids: Vec<Vector4<f64>> = facet_vertices
        .iter()
        .map(|indices| mean_vertex(vertices, indices))
        .collect();
    let adjacency = polytope.vertex_adjacency();

    let mut total = 0.0;
    for fi in 0..polytope.facet_count() {
        for fj in 0..polytope.facet_count() {
            if fi == fj || !adjacency[(fi, fj)] {
                continue;
            }

            let ridge = intersect_sorted(&facet_vertices[fi], &facet_vertices[fj]);
            if ridge.len() < 3 {
                continue;
            }

            let ordered = order_polygon_vertex_indices(vertices, &ridge);
            let facet_center = facet_centroids[fi];
            for k in 1..ordered.len() - 1 {
                total += simplex_volume_5(
                    Vector4::zeros(),
                    facet_center,
                    vertices[ordered[0]],
                    vertices[ordered[k]],
                    vertices[ordered[k + 1]],
                );
            }
        }
    }

    total
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

fn facet_vertex_indices(polytope: &Polytope4D) -> Vec<Vec<usize>> {
    let incidence = polytope.incidence();
    let vertex_count = incidence.nrows();

    (0..polytope.facet_count())
        .map(|fi| {
            (0..vertex_count)
                .filter(|&vi| incidence[(vi, fi)])
                .collect::<Vec<_>>()
        })
        .collect()
}

fn mean_vertex(vertices: &[Vector4<f64>], indices: &[usize]) -> Vector4<f64> {
    debug_assert!(
        !indices.is_empty(),
        "valid Polytope4D facets should have at least one incident vertex"
    );
    indices.iter().map(|&vi| vertices[vi]).sum::<Vector4<f64>>() / indices.len() as f64
}

fn intersect_sorted(lhs: &[usize], rhs: &[usize]) -> Vec<usize> {
    let mut out = Vec::new();
    let (mut i, mut j) = (0, 0);
    while i < lhs.len() && j < rhs.len() {
        match lhs[i].cmp(&rhs[j]) {
            std::cmp::Ordering::Less => i += 1,
            std::cmp::Ordering::Greater => j += 1,
            std::cmp::Ordering::Equal => {
                out.push(lhs[i]);
                i += 1;
                j += 1;
            }
        }
    }
    out
}

fn order_polygon_vertex_indices(all_vertices: &[Vector4<f64>], indices: &[usize]) -> Vec<usize> {
    if indices.len() <= 2 {
        return indices.to_vec();
    }

    let ridge_vertices: Vec<Vector4<f64>> = indices.iter().map(|&i| all_vertices[i]).collect();
    match sort_polygon_order(&ridge_vertices) {
        Some(order) => order.into_iter().map(|pos| indices[pos]).collect(),
        None => indices.to_vec(),
    }
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

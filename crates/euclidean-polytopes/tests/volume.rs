use euclidean_polytopes::{
    facet_volume_and_centroid_from_incidence_f64, facet_volume_from_incidence_f64, volume_f64,
    volume_from_incidence_exact, volume_from_incidence_f64, F64GeometryError, VolumeF64,
};
use nalgebra::{DMatrix, Vector4};
use num_rational::BigRational;
use num_traits::ToPrimitive;
use proptest::prelude::*;

type Q = BigRational;

fn vf(entries: [f64; 4]) -> Vector4<f64> {
    Vector4::new(entries[0], entries[1], entries[2], entries[3])
}

fn q(n: i64) -> Q {
    Q::from_integer(n.into())
}

fn qr(numerator: i64, denominator: i64) -> Q {
    Q::new(numerator.into(), denominator.into())
}

fn vq(entries: [Q; 4]) -> Vector4<Q> {
    Vector4::new(
        entries[0].clone(),
        entries[1].clone(),
        entries[2].clone(),
        entries[3].clone(),
    )
}

fn exact_points_to_f64(points: &[Vector4<Q>]) -> Vec<Vector4<f64>> {
    points
        .iter()
        .map(|point| {
            vf([
                point[0].to_f64().unwrap(),
                point[1].to_f64().unwrap(),
                point[2].to_f64().unwrap(),
                point[3].to_f64().unwrap(),
            ])
        })
        .collect()
}

fn centered_simplex() -> (Vec<Vector4<f64>>, Vec<Vector4<f64>>) {
    let vertices = vec![
        vf([-0.2, -0.2, -0.2, -0.2]),
        vf([0.8, -0.2, -0.2, -0.2]),
        vf([-0.2, 0.8, -0.2, -0.2]),
        vf([-0.2, -0.2, 0.8, -0.2]),
        vf([-0.2, -0.2, -0.2, 0.8]),
    ];
    let dual_vertices = vec![
        vf([5.0, 5.0, 5.0, 5.0]),
        vf([-5.0, 0.0, 0.0, 0.0]),
        vf([0.0, -5.0, 0.0, 0.0]),
        vf([0.0, 0.0, -5.0, 0.0]),
        vf([0.0, 0.0, 0.0, -5.0]),
    ];

    (dual_vertices, vertices)
}

fn centered_simplex_exact() -> Vec<Vector4<Q>> {
    vec![
        vq([qr(-1, 5), qr(-1, 5), qr(-1, 5), qr(-1, 5)]),
        vq([qr(4, 5), qr(-1, 5), qr(-1, 5), qr(-1, 5)]),
        vq([qr(-1, 5), qr(4, 5), qr(-1, 5), qr(-1, 5)]),
        vq([qr(-1, 5), qr(-1, 5), qr(4, 5), qr(-1, 5)]),
        vq([qr(-1, 5), qr(-1, 5), qr(-1, 5), qr(4, 5)]),
    ]
}

fn centered_simplex_incidence() -> DMatrix<bool> {
    DMatrix::from_row_slice(
        5,
        5,
        &[
            false, true, true, true, true, //
            true, false, true, true, true, //
            true, true, false, true, true, //
            true, true, true, false, true, //
            true, true, true, true, false,
        ],
    )
}

fn hypercube(scale: f64) -> (Vec<Vector4<f64>>, Vec<Vector4<f64>>) {
    let dual_vertices = vec![
        vf([1.0 / scale, 0.0, 0.0, 0.0]),
        vf([-1.0 / scale, 0.0, 0.0, 0.0]),
        vf([0.0, 1.0 / scale, 0.0, 0.0]),
        vf([0.0, -1.0 / scale, 0.0, 0.0]),
        vf([0.0, 0.0, 1.0 / scale, 0.0]),
        vf([0.0, 0.0, -1.0 / scale, 0.0]),
        vf([0.0, 0.0, 0.0, 1.0 / scale]),
        vf([0.0, 0.0, 0.0, -1.0 / scale]),
    ];

    let mut vertices = Vec::new();
    for x0 in [-scale, scale] {
        for x1 in [-scale, scale] {
            for x2 in [-scale, scale] {
                for x3 in [-scale, scale] {
                    vertices.push(vf([x0, x1, x2, x3]));
                }
            }
        }
    }

    (dual_vertices, vertices)
}

fn rational_box(scales: [Q; 4]) -> Vec<Vector4<Q>> {
    let mut vertices = Vec::new();
    for x0 in [-scales[0].clone(), scales[0].clone()] {
        for x1 in [-scales[1].clone(), scales[1].clone()] {
            for x2 in [-scales[2].clone(), scales[2].clone()] {
                for x3 in [-scales[3].clone(), scales[3].clone()] {
                    vertices.push(vq([x0.clone(), x1.clone(), x2.clone(), x3.clone()]));
                }
            }
        }
    }
    vertices
}

fn rational_box_incidence(vertices: &[Vector4<Q>], scales: &[Q; 4]) -> DMatrix<bool> {
    DMatrix::from_fn(vertices.len(), 8, |vertex_index, facet_index| {
        let coordinate_index = facet_index / 2;
        let positive_facet = facet_index % 2 == 0;
        let coordinate = &vertices[vertex_index][coordinate_index];
        if positive_facet {
            coordinate == &scales[coordinate_index]
        } else {
            coordinate == &-scales[coordinate_index].clone()
        }
    })
}

fn hypercube_incidence(vertices: &[Vector4<f64>], scale: f64) -> DMatrix<bool> {
    DMatrix::from_fn(vertices.len(), 8, |vertex_index, facet_index| {
        let coordinate_index = facet_index / 2;
        let positive_facet = facet_index % 2 == 0;
        let coordinate = vertices[vertex_index][coordinate_index];
        if positive_facet {
            coordinate == scale
        } else {
            coordinate == -scale
        }
    })
}

fn crosspolytope_radius_2() -> (Vec<Vector4<f64>>, Vec<Vector4<f64>>) {
    let mut dual_vertices = Vec::new();
    for s0 in [-0.5, 0.5] {
        for s1 in [-0.5, 0.5] {
            for s2 in [-0.5, 0.5] {
                for s3 in [-0.5, 0.5] {
                    dual_vertices.push(vf([s0, s1, s2, s3]));
                }
            }
        }
    }

    let vertices = vec![
        vf([2.0, 0.0, 0.0, 0.0]),
        vf([-2.0, 0.0, 0.0, 0.0]),
        vf([0.0, 2.0, 0.0, 0.0]),
        vf([0.0, -2.0, 0.0, 0.0]),
        vf([0.0, 0.0, 2.0, 0.0]),
        vf([0.0, 0.0, -2.0, 0.0]),
        vf([0.0, 0.0, 0.0, 2.0]),
        vf([0.0, 0.0, 0.0, -2.0]),
    ];

    (dual_vertices, vertices)
}

fn crosspolytope_radius_2_exact() -> Vec<Vector4<Q>> {
    vec![
        vq([q(2), q(0), q(0), q(0)]),
        vq([q(-2), q(0), q(0), q(0)]),
        vq([q(0), q(2), q(0), q(0)]),
        vq([q(0), q(-2), q(0), q(0)]),
        vq([q(0), q(0), q(2), q(0)]),
        vq([q(0), q(0), q(-2), q(0)]),
        vq([q(0), q(0), q(0), q(2)]),
        vq([q(0), q(0), q(0), q(-2)]),
    ]
}

fn crosspolytope_radius_2_incidence() -> DMatrix<bool> {
    DMatrix::from_fn(8, 16, |vertex_index, facet_index| {
        let coordinate_index = vertex_index / 2;
        let positive_vertex = vertex_index % 2 == 0;
        let facet_sign_bit = (facet_index >> (3 - coordinate_index)) & 1;
        positive_vertex == (facet_sign_bit == 1)
    })
}

fn decided_volume(dual_vertices: &[Vector4<f64>], vertices: &[Vector4<f64>]) -> f64 {
    match volume_f64(dual_vertices, vertices).expect("finite input") {
        VolumeF64::Decided { volume } => volume,
        VolumeF64::Indeterminate {
            indeterminate_incidence,
        } => panic!("expected decided volume, got {indeterminate_incidence:?}"),
    }
}

fn known_incidence_volume(vertices: &[Vector4<f64>], incidence: &DMatrix<bool>) -> f64 {
    volume_from_incidence_f64(vertices, incidence).expect("finite input")
}

fn exact_known_incidence_volume(vertices: &[Vector4<Q>], incidence: &DMatrix<bool>) -> Q {
    volume_from_incidence_exact(vertices, incidence)
}

fn assert_close(actual: f64, expected: f64, tolerance: f64) {
    assert!(
        (actual - expected).abs() <= tolerance,
        "got {actual}, expected {expected}"
    );
}

fn assert_vector_close(actual: Vector4<f64>, expected: Vector4<f64>, tolerance: f64) {
    assert!(
        (actual - expected).norm() <= tolerance,
        "got {actual:?}, expected {expected:?}"
    );
}

fn permute_by_stride<T: Clone>(values: &[T], stride: usize) -> Vec<T> {
    assert_eq!(
        gcd(values.len(), stride),
        1,
        "stride must define a full permutation"
    );
    (0..values.len())
        .map(|index| values[(index * stride) % values.len()].clone())
        .collect()
}

fn gcd(mut left: usize, mut right: usize) -> usize {
    while right != 0 {
        let remainder = left % right;
        left = right;
        right = remainder;
    }
    left
}

#[test]
fn simplex_volume_is_one_over_24() {
    let (dual_vertices, vertices) = centered_simplex();

    assert_close(
        decided_volume(&dual_vertices, &vertices),
        1.0 / 24.0,
        1.0e-12,
    );
}

#[test]
fn hypercube_volume_is_16() {
    let (dual_vertices, vertices) = hypercube(1.0);

    assert_close(decided_volume(&dual_vertices, &vertices), 16.0, 1.0e-10);
}

#[test]
fn crosspolytope_radius_2_volume_is_32_over_3() {
    let (dual_vertices, vertices) = crosspolytope_radius_2();

    assert_close(
        decided_volume(&dual_vertices, &vertices),
        32.0 / 3.0,
        1.0e-10,
    );
}

/// Proposition: if exact vertex-facet incidence is already known for a
/// normalized full-dimensional 4-polytope containing `0`, the known-incidence
/// helper computes the ordinary Euclidean volume without f64 incidence
/// recovery.
///
/// Operationalization: use explicit incidence matrices for the centered
/// simplex, the hypercube `[-1,1]^4`, and the radius-2 crosspolytope.
/// Tolerances match the existing f64 determinant-sum checks.
#[test]
fn known_incidence_volume_matches_fixture_values() {
    let (_, simplex_vertices) = centered_simplex();
    assert_close(
        known_incidence_volume(&simplex_vertices, &centered_simplex_incidence()),
        1.0 / 24.0,
        1.0e-12,
    );

    let (_, hypercube_vertices) = hypercube(1.0);
    assert_close(
        known_incidence_volume(
            &hypercube_vertices,
            &hypercube_incidence(&hypercube_vertices, 1.0),
        ),
        16.0,
        1.0e-10,
    );

    let (_, crosspolytope_vertices) = crosspolytope_radius_2();
    assert_close(
        known_incidence_volume(&crosspolytope_vertices, &crosspolytope_radius_2_incidence()),
        32.0 / 3.0,
        1.0e-10,
    );
}

/// Proposition: exact known vertex-facet incidence determines the ordinary
/// full-dimensional Euclidean volume by determinant triangulation over the
/// exact scalar field.
///
/// Operationalization: use explicit BigRational incidence fixtures for the
/// centered simplex, `[-1,1]^4`, and the radius-2 crosspolytope.
#[test]
fn exact_known_incidence_volume_matches_fixture_values() {
    assert_eq!(
        exact_known_incidence_volume(&centered_simplex_exact(), &centered_simplex_incidence()),
        qr(1, 24),
    );

    let scales = [q(1), q(1), q(1), q(1)];
    let box_vertices = rational_box(scales.clone());
    assert_eq!(
        exact_known_incidence_volume(
            &box_vertices,
            &rational_box_incidence(&box_vertices, &scales)
        ),
        q(16),
    );

    assert_eq!(
        exact_known_incidence_volume(
            &crosspolytope_radius_2_exact(),
            &crosspolytope_radius_2_incidence()
        ),
        qr(32, 3),
    );
}

/// Proposition: on rational fixtures whose f64 coordinates are exactly
/// representable or well-conditioned, the f64 known-incidence volume agrees
/// with the exact known-incidence determinant sum after conversion to f64.
///
/// Operationalization: compare the centered simplex, `[-1,1]^4`, and the
/// radius-2 crosspolytope. Tolerance is scale-aware and matches the existing
/// f64 fixture checks.
#[test]
fn known_incidence_f64_agrees_with_exact_on_rational_fixtures() {
    for (exact_vertices, incidence) in [
        (centered_simplex_exact(), centered_simplex_incidence()),
        {
            let scales = [q(1), q(1), q(1), q(1)];
            let vertices = rational_box(scales.clone());
            let incidence = rational_box_incidence(&vertices, &scales);
            (vertices, incidence)
        },
        (
            crosspolytope_radius_2_exact(),
            crosspolytope_radius_2_incidence(),
        ),
    ] {
        let exact_volume = exact_known_incidence_volume(&exact_vertices, &incidence)
            .to_f64()
            .unwrap();
        let f64_vertices = exact_points_to_f64(&exact_vertices);
        let f64_volume = known_incidence_volume(&f64_vertices, &incidence);
        let allowed_error = 1.0e-10_f64.max(1.0e-10 * exact_volume.abs());

        assert_close(f64_volume, exact_volume, allowed_error);
    }
}

/// Proposition: exact vertex-facet incidence is sufficient to compute the
/// ordinary 3D volume of a facet without recovering incidence from f64 signed
/// gaps.
///
/// Operationalization: every facet of `[-1,1]^4` is a 3-cube with volume `8`.
#[test]
fn known_incidence_hypercube_facet_volume_is_8() {
    let (_, vertices) = hypercube(1.0);
    let incidence = hypercube_incidence(&vertices, 1.0);

    for facet_index in 0..incidence.ncols() {
        let volume = facet_volume_from_incidence_f64(&vertices, &incidence, facet_index)
            .expect("finite input");
        assert_close(volume, 8.0, 1.0e-10);
    }
}

#[test]
fn known_incidence_hypercube_facet_centroids_are_face_centers() {
    let (_, vertices) = hypercube(1.0);
    let incidence = hypercube_incidence(&vertices, 1.0);

    for facet_index in 0..incidence.ncols() {
        let (volume, centroid) =
            facet_volume_and_centroid_from_incidence_f64(&vertices, &incidence, facet_index)
                .expect("finite input");
        let coordinate_index = facet_index / 2;
        let expected_sign = if facet_index % 2 == 0 { 1.0 } else { -1.0 };
        let mut expected = Vector4::zeros();
        expected[coordinate_index] = expected_sign;

        assert_close(volume, 8.0, 1.0e-10);
        assert_vector_close(centroid, expected, 1.0e-12);
    }
}

#[test]
fn known_incidence_volume_only_does_not_apply_centroid_floor() {
    let epsilon = 1.0e-31;
    let vertices = vec![
        vf([0.0, 0.0, 0.0, 0.0]),
        vf([1.0, 0.0, 0.0, 0.0]),
        vf([0.0, 1.0, 0.0, 0.0]),
        vf([1.0, 1.0, epsilon, 0.0]),
        vf([0.0, 0.0, 0.0, 1.0]),
    ];
    let incidence = DMatrix::from_fn(5, 5, |vertex_index, facet_index| {
        if facet_index == 0 {
            vertex_index < 4
        } else if vertex_index < 4 {
            vertex_index != facet_index - 1
        } else {
            true
        }
    });

    let volume = facet_volume_from_incidence_f64(&vertices, &incidence, 0).expect("finite input");
    let (centroid_volume, centroid) =
        facet_volume_and_centroid_from_incidence_f64(&vertices, &incidence, 0)
            .expect("finite input");

    assert_close(volume, epsilon / 6.0, 1.0e-40);
    assert_eq!(centroid_volume, 0.0);
    assert_eq!(centroid, Vector4::zeros());
}

#[test]
fn known_incidence_crosspolytope_facet_centroids_lie_on_facets() {
    let (dual_vertices, vertices) = crosspolytope_radius_2();
    let incidence = crosspolytope_radius_2_incidence();

    for (facet_index, dual) in dual_vertices.iter().enumerate() {
        let (volume, centroid) =
            facet_volume_and_centroid_from_incidence_f64(&vertices, &incidence, facet_index)
                .expect("finite input");

        assert!(
            volume > 0.0,
            "facet {facet_index} should have positive volume"
        );
        assert_close(dual.dot(&centroid), 1.0, 1.0e-10);
    }
}

/// Proposition: facet 3-volumes and known-incidence full 4-volume satisfy the
/// normalized support-function divergence theorem
/// `vol(K) = (1/4) sum_i S_i / ||a_i||`.
#[test]
fn known_incidence_facet_volumes_reconstruct_full_volume() {
    for (dual_vertices, vertices, incidence) in [
        {
            let (dual_vertices, vertices) = hypercube(1.0);
            let incidence = hypercube_incidence(&vertices, 1.0);
            (dual_vertices, vertices, incidence)
        },
        {
            let (dual_vertices, vertices) = crosspolytope_radius_2();
            (dual_vertices, vertices, crosspolytope_radius_2_incidence())
        },
    ] {
        let volume =
            volume_from_incidence_f64(&vertices, &incidence).expect("known-incidence volume");
        let volume_from_facets = (0..incidence.ncols())
            .map(|facet_index| {
                let facet_volume =
                    facet_volume_from_incidence_f64(&vertices, &incidence, facet_index)
                        .expect("known-incidence facet volume");
                facet_volume / dual_vertices[facet_index].norm()
            })
            .sum::<f64>()
            / 4.0;

        let allowed_error = 1.0e-10_f64.max(1.0e-10 * volume.abs());
        assert_close(volume_from_facets, volume, allowed_error);
    }
}

#[test]
fn recovered_and_known_incidence_volume_paths_agree() {
    for (dual_vertices, vertices, incidence) in [
        {
            let (dual_vertices, vertices) = centered_simplex();
            (dual_vertices, vertices, centered_simplex_incidence())
        },
        {
            let (dual_vertices, vertices) = hypercube(1.0);
            let incidence = hypercube_incidence(&vertices, 1.0);
            (dual_vertices, vertices, incidence)
        },
        {
            let (dual_vertices, vertices) = crosspolytope_radius_2();
            (dual_vertices, vertices, crosspolytope_radius_2_incidence())
        },
    ] {
        let recovered_volume = decided_volume(&dual_vertices, &vertices);
        let known_volume = known_incidence_volume(&vertices, &incidence);
        let allowed_error = 1.0e-10_f64.max(1.0e-10 * known_volume.abs());
        assert_close(recovered_volume, known_volume, allowed_error);
    }
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(16))]

    /// Proposition: for every axis-aligned rational box
    /// `[-s_0,s_0] x ... x [-s_3,s_3]` with positive scales, exact
    /// known-incidence volume is `16 * s_0 * s_1 * s_2 * s_3`.
    ///
    /// Operationalization: generate four scales
    /// `s_i = numerator_i / denominator_i` with terms in `1..=8`. The
    /// incidence matrix is constructed directly from the generated box
    /// vertices. Cases: 16 generated examples.
    #[test]
    fn exact_known_incidence_rational_box_volume_is_product_of_side_lengths(
        n0 in 1_i64..=8, d0 in 1_i64..=8,
        n1 in 1_i64..=8, d1 in 1_i64..=8,
        n2 in 1_i64..=8, d2 in 1_i64..=8,
        n3 in 1_i64..=8, d3 in 1_i64..=8,
    ) {
        let scales = [qr(n0, d0), qr(n1, d1), qr(n2, d2), qr(n3, d3)];
        let vertices = rational_box(scales.clone());
        let incidence = rational_box_incidence(&vertices, &scales);
        let expected = q(16)
            * scales[0].clone()
            * scales[1].clone()
            * scales[2].clone()
            * scales[3].clone();

        prop_assert_eq!(
            volume_from_incidence_exact(&vertices, &incidence),
            expected,
        );
    }

    /// Proposition: for every full-dimensional normalized polar pair
    /// `(K^circ, K)` with stable f64 incidence, `volume_f64` scales by `s^4`
    /// under `vertices -> s vertices` and
    /// `dual_vertices -> dual_vertices / s`.
    ///
    /// Operationalization: generate hypercubes `[-s,s]^4` with rational
    /// `s = numerator / denominator`, where both terms lie in `1..=16`.
    /// Discard generated scales whose active products are not stable enough
    /// for `volume_f64` to decide incidence. Cases: 16 accepted examples.
    /// Tolerance: `max(1e-10, 1e-10 * |16*s^4|)`.
    #[test]
    fn hypercube_volume_scales_by_fourth_power_for_rational_scales(
        numerator in 1_u32..=16,
        denominator in 1_u32..=16,
    ) {
        let scale = f64::from(numerator) / f64::from(denominator);
        let (dual_vertices, vertices) = hypercube(scale);

        let volume = match volume_f64(&dual_vertices, &vertices).expect("finite input") {
            VolumeF64::Decided { volume } => volume,
            VolumeF64::Indeterminate { .. } => {
                prop_assume!(false);
                unreachable!("prop_assume stops this generated case")
            }
        };
        let expected = 16.0 * scale.powi(4);
        let allowed_error = 1.0e-10_f64.max(1.0e-10 * expected.abs());
        prop_assert!(
            (volume - expected).abs() <= allowed_error,
            "scale={scale}, volume={volume}, expected={expected}, allowed_error={allowed_error}"
        );
    }

    /// Proposition: for every full-dimensional normalized polar pair
    /// `(K^circ, K)` with stable f64 incidence, `volume_f64` scales by `s^4`
    /// under `vertices -> s vertices` and
    /// `dual_vertices -> dual_vertices / s`.
    ///
    /// Operationalization: generate hypercubes `[-2^k,2^k]^4` for
    /// `k in {-4,...,4}`. No discard rule. Cases: 16 generated examples
    /// sampled from 9 scales. Tolerance:
    /// `max(1e-12, 1e-12 * |16*2^(4k)|)`.
    #[test]
    fn hypercube_volume_scales_by_fourth_power(exponent in -4_i32..=4) {
        let scale = 2.0_f64.powi(exponent);
        let (dual_vertices, vertices) = hypercube(scale);

        let volume = decided_volume(&dual_vertices, &vertices);
        let expected = 16.0 * scale.powi(4);
        let allowed_error = 1.0e-12_f64.max(1.0e-12 * expected.abs());
        prop_assert!(
            (volume - expected).abs() <= allowed_error,
            "scale={scale}, volume={volume}, expected={expected}, allowed_error={allowed_error}"
        );
    }
}

/// Proposition: for every full-dimensional normalized polar pair with stable
/// f64 incidence, `volume_f64` is invariant under permutation of the facet
/// list and under permutation of the vertex list.
///
/// Operationalization: check the hypercube and crosspolytope fixtures, using
/// fixed coprime-stride permutations of facets and vertices. Cases: 2
/// deterministic fixtures. Tolerance: `max(1e-10, 1e-10 * |volume|)`.
#[test]
fn volume_is_invariant_under_vertex_and_facet_permutation() {
    for (dual_vertices, vertices) in [hypercube(1.0), crosspolytope_radius_2()] {
        let volume = decided_volume(&dual_vertices, &vertices);
        let permuted_dual_vertices = permute_by_stride(&dual_vertices, 3);
        let permuted_vertices = permute_by_stride(&vertices, 5);
        let allowed_error = 1.0e-10_f64.max(1.0e-10 * volume.abs());

        let facet_permuted_volume = decided_volume(&permuted_dual_vertices, &vertices);
        assert_close(facet_permuted_volume, volume, allowed_error);

        let vertex_permuted_volume = decided_volume(&dual_vertices, &permuted_vertices);
        assert_close(vertex_permuted_volume, volume, allowed_error);

        let combined_permuted_volume = decided_volume(&permuted_dual_vertices, &permuted_vertices);
        assert_close(combined_permuted_volume, volume, allowed_error);
    }
}

#[test]
fn non_finite_input_returns_geometry_error() {
    let (dual_vertices, mut vertices) = hypercube(1.0);
    vertices[3][2] = f64::NAN;

    let error = volume_f64(&dual_vertices, &vertices).expect_err("non-finite coordinate");

    assert!(
        matches!(
            error,
            F64GeometryError::NonFiniteCoordinate {
                vector_role: "vertices",
                vector_index: 3,
                coordinate_index: 2,
                value,
            } if value.is_nan()
        ),
        "unexpected error: {error:?}"
    );
}

#[test]
fn known_incidence_non_finite_input_returns_geometry_error() {
    let (_, mut vertices) = hypercube(1.0);
    vertices[3][2] = f64::NAN;
    let incidence = hypercube_incidence(&vertices, 1.0);

    let error =
        volume_from_incidence_f64(&vertices, &incidence).expect_err("non-finite coordinate");

    assert!(
        matches!(
            error,
            F64GeometryError::NonFiniteCoordinate {
                vector_role: "vertices",
                vector_index: 3,
                coordinate_index: 2,
                value,
            } if value.is_nan()
        ),
        "unexpected error: {error:?}"
    );
}

#[test]
fn known_incidence_facet_non_finite_input_returns_geometry_error() {
    let (_, mut vertices) = hypercube(1.0);
    vertices[3][2] = f64::NAN;
    let incidence = hypercube_incidence(&vertices, 1.0);

    let error = facet_volume_from_incidence_f64(&vertices, &incidence, 0)
        .expect_err("non-finite coordinate");

    assert!(
        matches!(
            error,
            F64GeometryError::NonFiniteCoordinate {
                vector_role: "vertices",
                vector_index: 3,
                coordinate_index: 2,
                value,
            } if value.is_nan()
        ),
        "unexpected error: {error:?}"
    );
}

#[test]
#[should_panic(
    expected = "volume_from_incidence_f64 requires incidence rows to match vertices length"
)]
fn known_incidence_shape_mismatch_panics() {
    let (_, vertices) = hypercube(1.0);
    let incidence = DMatrix::from_element(vertices.len() + 1, 8, false);

    let _ = volume_from_incidence_f64(&vertices, &incidence);
}

#[test]
#[should_panic(
    expected = "volume_from_incidence_exact requires incidence rows to match vertices length"
)]
fn exact_known_incidence_shape_mismatch_panics() {
    let scales = [q(1), q(1), q(1), q(1)];
    let vertices = rational_box(scales);
    let incidence = DMatrix::from_element(vertices.len() + 1, 8, false);

    let _ = volume_from_incidence_exact(&vertices, &incidence);
}

#[test]
#[should_panic(
    expected = "known-incidence facet helpers require incidence rows to match vertices length"
)]
fn known_incidence_facet_shape_mismatch_panics() {
    let (_, vertices) = hypercube(1.0);
    let incidence = DMatrix::from_element(vertices.len() + 1, 8, false);

    let _ = facet_volume_from_incidence_f64(&vertices, &incidence, 0);
}

#[test]
#[should_panic(
    expected = "known-incidence facet helpers require facet_index to be a valid incidence column"
)]
fn known_incidence_facet_out_of_range_panics() {
    let (_, vertices) = hypercube(1.0);
    let incidence = hypercube_incidence(&vertices, 1.0);

    let _ = facet_volume_from_incidence_f64(&vertices, &incidence, incidence.ncols());
}

#[test]
fn near_incidence_returns_indeterminate_instead_of_guessing() {
    let (dual_vertices, mut vertices) = hypercube(1.0);
    vertices[0][0] += 1.0e-14;

    let result = volume_f64(&dual_vertices, &vertices).expect("finite input");

    let VolumeF64::Indeterminate {
        indeterminate_incidence,
    } = result
    else {
        panic!("near-boundary incidence should not be decided");
    };

    assert!(
        indeterminate_incidence
            .iter()
            .any(|relation| relation.vertex_index == 0
                && relation.facet_index == 1
                && relation.signed_gap > 0.0
                && relation.signed_gap <= relation.signed_gap_abs_error_bound),
        "missing expected ambiguous relation: {indeterminate_incidence:?}"
    );
}

use euclidean_polytopes::{volume_f64, F64GeometryError, VolumeF64};
use nalgebra::Vector4;
use proptest::prelude::*;

fn vf(entries: [f64; 4]) -> Vector4<f64> {
    Vector4::new(entries[0], entries[1], entries[2], entries[3])
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

fn decided_volume(dual_vertices: &[Vector4<f64>], vertices: &[Vector4<f64>]) -> f64 {
    match volume_f64(dual_vertices, vertices).expect("finite input") {
        VolumeF64::Decided { volume } => volume,
        VolumeF64::Indeterminate {
            indeterminate_incidence,
        } => panic!("expected decided volume, got {indeterminate_incidence:?}"),
    }
}

fn assert_close(actual: f64, expected: f64, tolerance: f64) {
    assert!(
        (actual - expected).abs() <= tolerance,
        "got {actual}, expected {expected}"
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

proptest! {
    #![proptest_config(ProptestConfig::with_cases(16))]

    /// Proposition: for every full-dimensional normalized polar pair
    /// `(K^circ, K)` with stable f64 incidence, `volume_f64` scales by `s^4`
    /// under `vertices -> s vertices` and
    /// `dual_vertices -> dual_vertices / s`.
    ///
    /// Operationalization: generate hypercubes `[-s,s]^4` with rational
    /// `s = numerator / denominator`, where both terms lie in `1..=16`.
    /// No discard rule. Cases: 16. Tolerance:
    /// `max(1e-10, 1e-10 * |16*s^4|)`.
    #[test]
    fn hypercube_volume_scales_by_fourth_power_for_rational_scales(
        numerator in 1_u32..=16,
        denominator in 1_u32..=16,
    ) {
        let scale = f64::from(numerator) / f64::from(denominator);
        let (dual_vertices, vertices) = hypercube(scale);

        let volume = decided_volume(&dual_vertices, &vertices);
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
        let permuted_volume = decided_volume(&permuted_dual_vertices, &permuted_vertices);
        let allowed_error = 1.0e-10_f64.max(1.0e-10 * volume.abs());

        assert_close(permuted_volume, volume, allowed_error);
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

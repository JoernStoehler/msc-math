use euclidean_polytopes::{
    origin_in_interior_of_conv_exact, polar_vertices_exact, PolarVerticesExact,
};
use nalgebra::Vector4;
use num_rational::BigRational;
use proptest::prelude::*;

type Q = BigRational;

fn q(n: i64) -> Q {
    Q::from_integer(n.into())
}

fn vq(entries: [i64; 4]) -> Vector4<Q> {
    Vector4::new(q(entries[0]), q(entries[1]), q(entries[2]), q(entries[3]))
}

fn vq_frac(entries: [(i64, i64); 4]) -> Vector4<Q> {
    Vector4::new(
        Q::new(entries[0].0.into(), entries[0].1.into()),
        Q::new(entries[1].0.into(), entries[1].1.into()),
        Q::new(entries[2].0.into(), entries[2].1.into()),
        Q::new(entries[3].0.into(), entries[3].1.into()),
    )
}

fn simplex_vertices_exact() -> Vec<Vector4<Q>> {
    vec![
        vq([1, 0, 0, 0]),
        vq([0, 1, 0, 0]),
        vq([0, 0, 1, 0]),
        vq([0, 0, 0, 1]),
        vq([-1, -1, -1, -1]),
    ]
}

fn assert_exact_set_eq(mut actual: Vec<Vector4<Q>>, mut expected: Vec<Vector4<Q>>) {
    actual.sort_by(vector_cmp);
    expected.sort_by(vector_cmp);
    assert_eq!(actual, expected);
}

fn vector_cmp(left: &Vector4<Q>, right: &Vector4<Q>) -> std::cmp::Ordering {
    for coordinate in 0..4 {
        match left[coordinate].cmp(&right[coordinate]) {
            std::cmp::Ordering::Equal => {}
            other => return other,
        }
    }
    std::cmp::Ordering::Equal
}

fn crosspolytope_vertices_exact() -> Vec<Vector4<Q>> {
    vec![
        vq([1, 0, 0, 0]),
        vq([-1, 0, 0, 0]),
        vq([0, 1, 0, 0]),
        vq([0, -1, 0, 0]),
        vq([0, 0, 1, 0]),
        vq([0, 0, -1, 0]),
        vq([0, 0, 0, 1]),
        vq([0, 0, 0, -1]),
    ]
}

fn cube_vertices_exact() -> Vec<Vector4<Q>> {
    let mut vertices = Vec::new();
    for x0 in [-1, 1] {
        for x1 in [-1, 1] {
            for x2 in [-1, 1] {
                for x3 in [-1, 1] {
                    vertices.push(vq([x0, x1, x2, x3]));
                }
            }
        }
    }
    vertices
}

fn scaled_crosspolytope_vertices_exact(scales: [i64; 4]) -> Vec<Vector4<Q>> {
    let mut vertices = Vec::new();
    for axis in 0..4 {
        for sign in [-1, 1] {
            let mut point = Vector4::new(q(0), q(0), q(0), q(0));
            point[axis] = q(sign * scales[axis]);
            vertices.push(point);
        }
    }
    vertices
}

fn positive_spanning_points_with_redundant_edge_points(
    scales: [i64; 4],
    extra_codes: &[u8],
) -> (Vec<Vector4<Q>>, Vec<Vector4<Q>>) {
    let extreme_points = scaled_crosspolytope_vertices_exact(scales);
    let mut points = extreme_points.clone();
    for &code in extra_codes {
        points.push(redundant_crosspolytope_edge_point(scales, code));
    }

    (points, extreme_points)
}

fn redundant_crosspolytope_edge_point(scales: [i64; 4], code: u8) -> Vector4<Q> {
    let first_axis = (code & 0b0000_0011) as usize;
    let mut second_axis = ((code >> 2) & 0b0000_0011) as usize;
    if second_axis == first_axis {
        second_axis = (second_axis + 1) % 4;
    }

    let first_sign = if code & 0b0001_0000 == 0 { 1 } else { -1 };
    let second_sign = if code & 0b0010_0000 == 0 { 1 } else { -1 };
    let two = q(2);
    let mut point = Vector4::new(q(0), q(0), q(0), q(0));
    point[first_axis] = q(first_sign * scales[first_axis]) / two.clone();
    point[second_axis] = q(second_sign * scales[second_axis]) / two;
    point
}

fn dot_q(left: &Vector4<Q>, right: &Vector4<Q>) -> Q {
    left[0].clone() * right[0].clone()
        + left[1].clone() * right[1].clone()
        + left[2].clone() * right[2].clone()
        + left[3].clone() * right[3].clone()
}

fn assert_exact_polar_soundness(points: &[Vector4<Q>]) {
    let PolarVerticesExact {
        vertices,
        vertex_facet_incidence,
    } = polar_vertices_exact(points);
    assert_eq!(vertex_facet_incidence.nrows(), vertices.len());
    assert_eq!(vertex_facet_incidence.ncols(), points.len());

    let one = q(1);
    for (vertex_index, polar_vertex) in vertices.iter().enumerate() {
        for (facet_index, point) in points.iter().enumerate() {
            let dot = dot_q(point, polar_vertex);
            assert!(
                dot <= one,
                "polar vertex {vertex_index} violates inequality {facet_index}: {dot}"
            );
            assert_eq!(
                vertex_facet_incidence[(vertex_index, facet_index)],
                dot == one,
                "wrong exact incidence at ({vertex_index}, {facet_index})"
            );
        }
    }
}

fn assert_exact_polarity_roundtrip(
    points: &[Vector4<Q>],
    expected_extreme_points: Vec<Vector4<Q>>,
) {
    let PolarVerticesExact { vertices, .. } = polar_vertices_exact(points);
    let PolarVerticesExact {
        vertices: double_polar_vertices,
        ..
    } = polar_vertices_exact(&vertices);

    assert_exact_set_eq(double_polar_vertices, expected_extreme_points);
}

#[test]
fn origin_in_interior_exact_detects_full_dimensional_positive_span() {
    assert!(origin_in_interior_of_conv_exact(
        &crosspolytope_vertices_exact()
    ));
    assert!(!origin_in_interior_of_conv_exact(&[
        vq([1, 0, 0, 0]),
        vq([0, 1, 0, 0]),
        vq([0, 0, 1, 0]),
        vq([0, 0, 0, 1]),
        vq([1, 1, 1, 1]),
    ]));
}

#[test]
fn simplex_polar_vertices_are_exact_set() {
    let primal = simplex_vertices_exact();

    let PolarVerticesExact {
        vertices,
        vertex_facet_incidence,
    } = polar_vertices_exact(&primal);

    assert_eq!(vertex_facet_incidence.nrows(), 5);
    assert_eq!(vertex_facet_incidence.ncols(), 5);
    assert_exact_set_eq(
        vertices,
        vec![
            vq([1, 1, 1, 1]),
            vq([-4, 1, 1, 1]),
            vq([1, -4, 1, 1]),
            vq([1, 1, -4, 1]),
            vq([1, 1, 1, -4]),
        ],
    );
}

#[test]
fn cube_polar_vertices_are_crosspolytope() {
    let PolarVerticesExact { vertices, .. } = polar_vertices_exact(&cube_vertices_exact());

    assert_exact_set_eq(vertices, crosspolytope_vertices_exact());
}

#[test]
fn crosspolytope_polar_vertices_are_cube() {
    let PolarVerticesExact { vertices, .. } = polar_vertices_exact(&crosspolytope_vertices_exact());

    assert_exact_set_eq(vertices, cube_vertices_exact());
}

#[test]
fn redundant_input_point_does_not_change_exact_polar_vertices() {
    let base = crosspolytope_vertices_exact();
    let mut redundant = base.clone();
    redundant.push(vq_frac([(1, 2), (0, 1), (0, 1), (0, 1)]));

    let PolarVerticesExact {
        vertices: base_vertices,
        ..
    } = polar_vertices_exact(&base);
    let PolarVerticesExact {
        vertices: redundant_vertices,
        ..
    } = polar_vertices_exact(&redundant);

    assert_exact_set_eq(redundant_vertices, base_vertices);
}

#[test]
#[should_panic(expected = "polar_vertices_exact requires 0 in int conv(vertices)")]
fn polar_vertices_exact_panics_when_origin_is_not_interior() {
    let points = vec![
        vq([1, 0, 0, 0]),
        vq([0, 1, 0, 0]),
        vq([0, 0, 1, 0]),
        vq([0, 0, 0, 1]),
    ];

    let _ = polar_vertices_exact(&points);
}

#[test]
fn polar_vertices_exact_deduplicates_non_simple_vertices() {
    let PolarVerticesExact { vertices, .. } = polar_vertices_exact(&cube_vertices_exact());

    assert_eq!(vertices.len(), 8);
}

/// Proposition: for every finite `P subset Q^4`, if `0 in int conv(P)`, then
/// every `y` returned by `polar_vertices_exact(P)` satisfies `<p, y> <= 1`
/// for all `p in P`, and the returned incidence matrix is exactly
/// `(<p_j, y_i> == 1)`.
///
/// Operationalization: check the named simplex, cube, and crosspolytope exact
/// fixtures. Cases: 3 deterministic fixtures. Tolerance: none, exact `Q`.
#[test]
fn exact_polar_vertices_are_feasible_and_incidence_is_exact_on_named_fixtures() {
    for points in [
        simplex_vertices_exact(),
        cube_vertices_exact(),
        crosspolytope_vertices_exact(),
    ] {
        assert_exact_polar_soundness(&points);
    }
}

/// Proposition: for every finite `P subset Q^4`, if `0 in int conv(P)`, then
/// the double polar computed from `polar_vertices_exact(P)` is the set of
/// extreme points of `conv(P)`.
///
/// Operationalization: check the named simplex, cube, and crosspolytope exact
/// fixtures, whose listed points are all known extreme points. Cases: 3
/// deterministic fixtures. Tolerance: none, exact `Q`.
#[test]
fn exact_polarity_roundtrip_returns_named_fixture_vertices() {
    for points in [
        simplex_vertices_exact(),
        cube_vertices_exact(),
        crosspolytope_vertices_exact(),
    ] {
        assert_exact_polarity_roundtrip(&points, points.clone());
    }
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(32))]

    /// Proposition: for every finite `P subset Q^4`, if `0 in int conv(P)`, then
    /// every `y` returned by `polar_vertices_exact(P)` satisfies `<p, y> <= 1`
    /// for all `p in P`, and the returned incidence matrix is exactly
    /// `(<p_j, y_i> == 1)`.
    ///
    /// Operationalization: generate positive-spanning scaled crosspolytopes
    /// `+-s_i e_i` with `s_i in {1,2,3}`, then append up to four exact edge
    /// midpoints selected by `u8` codes. No discard rule: every generated sample
    /// has `0 in int conv(P)`. Cases: 32. Tolerance: none, exact `Q`.
    #[test]
    fn generated_positive_spanning_exact_polar_is_sound(
        scales in [1_i64..=3, 1_i64..=3, 1_i64..=3, 1_i64..=3],
        extra_codes in proptest::collection::vec(0_u8..64, 0..=4),
    ) {
        let (points, _) = positive_spanning_points_with_redundant_edge_points(scales, &extra_codes);

        prop_assert!(origin_in_interior_of_conv_exact(&points));
        assert_exact_polar_soundness(&points);
    }
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(12))]

    /// Proposition: for every finite `P subset Q^4`, if `0 in int conv(P)`, then
    /// the double polar computed from `polar_vertices_exact(P)` is the set of
    /// extreme points of `conv(P)`.
    ///
    /// Operationalization: generate positive-spanning scaled crosspolytopes
    /// `+-s_i e_i` with `s_i in {1,2,3}`, then append up to four exact edge
    /// midpoints selected by `u8` codes. The expected extreme set is the original
    /// `+-s_i e_i` vertex set. No discard rule: every generated sample has
    /// `0 in int conv(P)`. Cases: 12, because exact double-polar enumeration
    /// is the expensive property in this crate test suite. Tolerance: none,
    /// exact `Q`.
    #[test]
    fn generated_exact_polarity_roundtrip_discards_redundant_edge_points(
        scales in [1_i64..=3, 1_i64..=3, 1_i64..=3, 1_i64..=3],
        extra_codes in proptest::collection::vec(0_u8..64, 0..=4),
    ) {
        let (points, expected_extreme_points) =
            positive_spanning_points_with_redundant_edge_points(scales, &extra_codes);

        prop_assert!(origin_in_interior_of_conv_exact(&points));
        assert_exact_polarity_roundtrip(&points, expected_extreme_points);
    }
}

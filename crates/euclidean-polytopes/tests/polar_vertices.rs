use euclidean_polytopes::{
    origin_in_interior_of_conv_exact, polar_vertices_exact, polar_vertices_f64, F64GeometryError,
    PolarVerticesF64,
};
use nalgebra::Vector4;
use num_rational::BigRational;
use num_traits::ToPrimitive;
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

fn vf(entries: [f64; 4]) -> Vector4<f64> {
    Vector4::new(entries[0], entries[1], entries[2], entries[3])
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
    let (vertices, vertex_facet_incidence) = polar_vertices_exact(points);
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
    let (vertices, _) = polar_vertices_exact(points);
    let (double_polar_vertices, _) = polar_vertices_exact(&vertices);

    assert_exact_set_eq(double_polar_vertices, expected_extreme_points);
}

fn exact_points_to_f64(points: &[Vector4<Q>]) -> Vec<Vector4<f64>> {
    points
        .iter()
        .map(|point| {
            Vector4::new(
                point[0].to_f64().unwrap(),
                point[1].to_f64().unwrap(),
                point[2].to_f64().unwrap(),
                point[3].to_f64().unwrap(),
            )
        })
        .collect()
}

fn assert_f64_set_close(actual: &[Vector4<f64>], expected: &[Vector4<f64>], tolerance: f64) {
    assert_eq!(
        actual.len(),
        expected.len(),
        "actual vertices: {actual:?}; expected vertices: {expected:?}"
    );

    let mut matched = vec![false; actual.len()];
    for expected_vertex in expected {
        let Some(actual_index) = actual
            .iter()
            .enumerate()
            .position(|(index, actual_vertex)| {
                !matched[index]
                    && max_abs_coordinate_difference(actual_vertex, expected_vertex) <= tolerance
            })
        else {
            panic!(
                "no f64 vertex within {tolerance} of {expected_vertex:?}; actual vertices: {actual:?}"
            );
        };
        matched[actual_index] = true;
    }
}

fn max_abs_coordinate_difference(left: &Vector4<f64>, right: &Vector4<f64>) -> f64 {
    (0..4)
        .map(|coordinate| (left[coordinate] - right[coordinate]).abs())
        .fold(0.0, f64::max)
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

    let (vertices, vertex_facet_incidence) = polar_vertices_exact(&primal);

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
    let (vertices, _) = polar_vertices_exact(&cube_vertices_exact());

    assert_exact_set_eq(vertices, crosspolytope_vertices_exact());
}

#[test]
fn crosspolytope_polar_vertices_are_cube() {
    let (vertices, _) = polar_vertices_exact(&crosspolytope_vertices_exact());

    assert_exact_set_eq(vertices, cube_vertices_exact());
}

#[test]
fn redundant_input_point_does_not_change_exact_polar_vertices() {
    let base = crosspolytope_vertices_exact();
    let mut redundant = base.clone();
    redundant.push(vq_frac([(1, 2), (0, 1), (0, 1), (0, 1)]));

    let (base_vertices, _) = polar_vertices_exact(&base);
    let (redundant_vertices, _) = polar_vertices_exact(&redundant);

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
    let (vertices, _) = polar_vertices_exact(&cube_vertices_exact());

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

/// Proposition: for exact fixtures whose f64 coordinates are well-conditioned
/// and whose active/incidence gaps are stable in `f64`, `polar_vertices_f64`
/// returns no indeterminate candidates and agrees with `polar_vertices_exact`
/// after exact-to-f64 conversion.
///
/// Operationalization: check the centered simplex fixture, where each polar
/// vertex is simple and every inactive signed gap is integral. Cases: 1
/// deterministic fixture. Tolerance: `1e-10` max coordinate error.
#[test]
fn polar_vertices_f64_agrees_with_exact_simplex_without_indeterminate_candidates() {
    let primal = simplex_vertices_exact();
    let (exact_vertices, _) = polar_vertices_exact(&primal);
    let expected = exact_points_to_f64(&exact_vertices);

    let PolarVerticesF64 {
        vertices,
        indeterminate_candidates,
        ..
    } = polar_vertices_f64(&exact_points_to_f64(&primal)).expect("finite f64 input");

    assert!(
        indeterminate_candidates.is_empty(),
        "simplex fixture should be decided: {indeterminate_candidates:?}"
    );
    assert_f64_set_close(&vertices, &expected, 1.0e-10);
}

/// Proposition: for exact fixtures whose f64 coordinates are well-conditioned
/// and whose accepted candidate gaps are stable in `f64`, every decided
/// `polar_vertices_f64` vertex agrees with `polar_vertices_exact` after
/// conversion.
///
/// Operationalization: check the crosspolytope fixture. This fixture has
/// stable simple polar vertices but also singular 4-tuples; the current
/// diagnostic API reports those singular tuples as indeterminate instead of
/// silently skipping them. Cases: 1 deterministic fixture. Tolerance: `1e-10`
/// max coordinate error.
#[test]
fn polar_vertices_f64_decided_crosspolytope_vertices_agree_with_exact() {
    let primal = crosspolytope_vertices_exact();
    let (exact_vertices, _) = polar_vertices_exact(&primal);
    let expected = exact_points_to_f64(&exact_vertices);

    let PolarVerticesF64 { vertices, .. } =
        polar_vertices_f64(&exact_points_to_f64(&primal)).expect("finite f64 input");

    assert_f64_set_close(&vertices, &expected, 1.0e-10);
}

#[test]
fn polar_vertices_f64_rejects_non_finite_input() {
    let points = vec![vf([1.0, 0.0, 0.0, 0.0]), vf([f64::NAN, 1.0, 0.0, 0.0])];

    let error = polar_vertices_f64(&points).expect_err("non-finite coordinate");

    assert!(
        matches!(
            error,
            F64GeometryError::NonFiniteCoordinate {
                vector_role: "vertices",
                vector_index: 1,
                coordinate_index: 0,
                value,
            } if value.is_nan()
        ),
        "unexpected error: {error:?}"
    );
}

#[test]
fn polar_vertices_f64_reports_near_boundary_tuple_as_indeterminate() {
    let mut points: Vec<Vector4<f64>> = crosspolytope_vertices_exact()
        .into_iter()
        .map(|point| {
            Vector4::new(
                point[0].to_f64().unwrap(),
                point[1].to_f64().unwrap(),
                point[2].to_f64().unwrap(),
                point[3].to_f64().unwrap(),
            )
        })
        .collect();
    points.push(vf([1.0 - 1.0e-14, 0.0, 0.0, 0.0]));

    let PolarVerticesF64 {
        indeterminate_candidates,
        ..
    } = polar_vertices_f64(&points).expect("finite f64 input");

    assert!(
        indeterminate_candidates
            .iter()
            .any(|candidate| candidate.vertex.is_some()),
        "near-boundary halfspace membership must return the approximate candidate"
    );
}

#[test]
fn polar_vertices_f64_incidence_reports_local_signed_gap_diagnostics() {
    let simplex = vec![
        vq([1, 0, 0, 0]),
        vq([0, 1, 0, 0]),
        vq([0, 0, 1, 0]),
        vq([0, 0, 0, 1]),
        vq([-1, -1, -1, -1]),
    ];
    let points: Vec<Vector4<f64>> = simplex
        .into_iter()
        .map(|point| {
            Vector4::new(
                point[0].to_f64().unwrap(),
                point[1].to_f64().unwrap(),
                point[2].to_f64().unwrap(),
                point[3].to_f64().unwrap(),
            )
        })
        .collect();

    let PolarVerticesF64 {
        vertices,
        incidence,
        ..
    } = polar_vertices_f64(&points).expect("finite f64 input");

    assert!(
        !incidence.is_empty(),
        "simplex polar should have accepted incidences"
    );
    for relation in incidence {
        let facet = &points[relation.facet_index];
        let vertex = &vertices[relation.vertex_index];

        assert_eq!(relation.signed_gap, 1.0 - facet.dot(vertex));
        assert_eq!(
            relation.signed_gap_abs_error_bound,
            expected_signed_gap_abs_error_bound(facet, vertex)
        );
    }
}

#[test]
fn polar_vertices_f64_reports_singular_tuple_without_candidate_vertex() {
    let points = vec![
        vf([1.0, 0.0, 0.0, 0.0]),
        vf([0.0, 1.0, 0.0, 0.0]),
        vf([0.0, 0.0, 1.0, 0.0]),
        vf([0.0, 0.0, 2.0, 0.0]),
    ];

    let PolarVerticesF64 {
        indeterminate_candidates,
        ..
    } = polar_vertices_f64(&points).expect("finite f64 input");

    assert!(
        indeterminate_candidates
            .iter()
            .any(|candidate| candidate.vertex.is_none()),
        "singular tuple must not invent an approximate candidate"
    );
}

fn expected_signed_gap_abs_error_bound(facet: &Vector4<f64>, candidate: &Vector4<f64>) -> f64 {
    const EPS_MACH: f64 = f64::EPSILON / 2.0;
    const ERROR_SCALE: f64 = 1.0e4;

    ERROR_SCALE * EPS_MACH * (facet.norm() * candidate.norm() + facet.dot(candidate).abs() + 1.0)
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

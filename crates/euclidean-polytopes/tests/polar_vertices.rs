use euclidean_polytopes::{
    origin_in_interior_of_conv_exact, polar_vertices_exact, polar_vertices_f64, F64GeometryError,
    PolarVerticesF64,
};
use nalgebra::Vector4;
use num_rational::BigRational;
use num_traits::ToPrimitive;

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
    let primal = vec![
        vq([1, 0, 0, 0]),
        vq([0, 1, 0, 0]),
        vq([0, 0, 1, 0]),
        vq([0, 0, 0, 1]),
        vq([-1, -1, -1, -1]),
    ];

    let polar = polar_vertices_exact(&primal);

    assert_eq!(polar.incidence.nrows(), 5);
    assert_eq!(polar.incidence.ncols(), 5);
    assert_exact_set_eq(
        polar.vertices,
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
    let polar = polar_vertices_exact(&cube_vertices_exact());

    assert_exact_set_eq(polar.vertices, crosspolytope_vertices_exact());
}

#[test]
fn crosspolytope_polar_vertices_are_cube() {
    let polar = polar_vertices_exact(&crosspolytope_vertices_exact());

    assert_exact_set_eq(polar.vertices, cube_vertices_exact());
}

#[test]
fn redundant_input_point_does_not_change_exact_polar_vertices() {
    let base = crosspolytope_vertices_exact();
    let mut redundant = base.clone();
    redundant.push(vq_frac([(1, 2), (0, 1), (0, 1), (0, 1)]));

    let base_polar = polar_vertices_exact(&base);
    let redundant_polar = polar_vertices_exact(&redundant);

    assert_exact_set_eq(redundant_polar.vertices, base_polar.vertices);
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
    let polar = polar_vertices_exact(&cube_vertices_exact());

    assert_eq!(polar.vertices.len(), 8);
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

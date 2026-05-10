use euclidean_polytopes::all_points_are_extreme_exact;
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

fn simplex_vertices_exact() -> Vec<Vector4<Q>> {
    vec![
        vq([1, 0, 0, 0]),
        vq([0, 1, 0, 0]),
        vq([0, 0, 1, 0]),
        vq([0, 0, 0, 1]),
        vq([-1, -1, -1, -1]),
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

fn axis_aligned_simplex_vertices(scales: [i64; 4]) -> Vec<Vector4<Q>> {
    let mut vertices = vec![vq([0, 0, 0, 0])];
    for axis in 0..4 {
        let mut point = Vector4::new(q(0), q(0), q(0), q(0));
        point[axis] = q(scales[axis]);
        vertices.push(point);
    }
    vertices
}

fn axis_aligned_box_vertices(scales: [i64; 4]) -> Vec<Vector4<Q>> {
    let mut vertices = Vec::new();
    for x0 in [-scales[0], scales[0]] {
        for x1 in [-scales[1], scales[1]] {
            for x2 in [-scales[2], scales[2]] {
                for x3 in [-scales[3], scales[3]] {
                    vertices.push(vq([x0, x1, x2, x3]));
                }
            }
        }
    }
    vertices
}

#[test]
fn simplex_and_cube_vertices_are_all_extreme() {
    assert!(all_points_are_extreme_exact(&simplex_vertices_exact()));
    assert!(all_points_are_extreme_exact(&cube_vertices_exact()));
}

#[test]
fn exact_duplicate_points_are_not_all_extreme() {
    let mut points = simplex_vertices_exact();
    points.push(vq([1, 0, 0, 0]));

    assert!(!all_points_are_extreme_exact(&points));
}

#[test]
fn interior_point_in_simplex_is_not_extreme() {
    let mut points = simplex_vertices_exact();
    points.push(vq([0, 0, 0, 0]));

    assert!(!all_points_are_extreme_exact(&points));
}

#[test]
fn lower_dimensional_polygon_vertices_in_r4_are_all_extreme() {
    let points = vec![
        vq([0, 0, 0, 0]),
        vq([1, 0, 0, 0]),
        vq([1, 1, 0, 0]),
        vq([0, 1, 0, 0]),
    ];

    assert!(all_points_are_extreme_exact(&points));
}

#[test]
fn collinear_midpoint_in_r4_is_not_extreme() {
    let points = vec![vq([0, 0, 0, 0]), vq([1, 0, 0, 0]), vq([2, 0, 0, 0])];

    assert!(!all_points_are_extreme_exact(&points));
}

#[test]
fn planar_square_with_center_or_edge_midpoint_is_not_all_extreme() {
    let square = vec![
        vq([0, 0, 0, 0]),
        vq([2, 0, 0, 0]),
        vq([2, 2, 0, 0]),
        vq([0, 2, 0, 0]),
    ];

    let mut with_center = square.clone();
    with_center.push(vq([1, 1, 0, 0]));
    assert!(!all_points_are_extreme_exact(&with_center));

    let mut with_edge_midpoint = square;
    with_edge_midpoint.push(vq([1, 0, 0, 0]));
    assert!(!all_points_are_extreme_exact(&with_edge_midpoint));
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(8))]

    /// Proposition: for every affinely independent 4-simplex vertex set and
    /// every centrally symmetric full-dimensional box vertex set in `Q^4`,
    /// every listed point is an extreme point of its convex hull.
    ///
    /// Operationalization: generate axis-aligned simplex vertices
    /// `{0, s_i e_i}` and axis-aligned boxes `prod_i [-s_i, s_i]` with
    /// `s_i in {1,2,3,4}`. No discard rule. Cases: 8, because exact
    /// non-redundancy over 16 box vertices is the expensive branch here.
    /// Tolerance: none,
    /// exact `Q`.
    #[test]
    fn generated_axis_aligned_simplices_and_boxes_are_nonredundant(
        scales in [1_i64..=4, 1_i64..=4, 1_i64..=4, 1_i64..=4],
    ) {
        prop_assert!(all_points_are_extreme_exact(&axis_aligned_simplex_vertices(scales)));
        prop_assert!(all_points_are_extreme_exact(&axis_aligned_box_vertices(scales)));
    }
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(32))]

    /// Proposition: for every finite input list `P` in `Q^4` and every
    /// `x in conv(P)`, the list obtained by appending `x` to `P` is not all
    /// extreme.
    ///
    /// Operationalization: generate five points in `[-3,3]^4` and append their
    /// fixed positive rational convex combination with weights
    /// `(1,2,3,4,5)/15`. No discard rule; degenerate inputs only make the
    /// non-extremality conclusion easier. Cases: 32. Tolerance: none, exact
    /// `Q`.
    #[test]
    fn generated_convex_combination_is_not_extreme(entries in proptest::collection::vec(-3_i64..=3, 20)) {
        let points = entries
            .chunks_exact(4)
            .map(|chunk| vq([chunk[0], chunk[1], chunk[2], chunk[3]]))
            .collect::<Vec<_>>();

        let numerator_sum = q(15);
        let weights = [q(1), q(2), q(3), q(4), q(5)];
        let mut convex_combination = Vector4::new(q(0), q(0), q(0), q(0));
        for (point, weight) in points.iter().zip(weights) {
            for coordinate in 0..4 {
                convex_combination[coordinate] +=
                    point[coordinate].clone() * weight.clone() / numerator_sum.clone();
            }
        }

        let mut with_combination = points;
        with_combination.push(convex_combination);

        prop_assert!(!all_points_are_extreme_exact(&with_combination));
    }

    /// Proposition: for every non-empty segment in `Q^4`, the midpoint is not
    /// an extreme point of the three-point set consisting of the two endpoints
    /// and the midpoint.
    ///
    /// Operationalization: generate endpoints as `a` and `a + 2d` with
    /// `a,d in [-3,3]^4`, append `a + d`, and discard only `d = 0` so the
    /// fixture is a genuine segment rather than a duplicate-point case.
    /// Cases: 32. Tolerance: none, exact `Q`.
    #[test]
    fn generated_segment_midpoint_is_not_extreme(
        start_entries in [-3_i64..=3, -3_i64..=3, -3_i64..=3, -3_i64..=3],
        direction_entries in [-3_i64..=3, -3_i64..=3, -3_i64..=3, -3_i64..=3],
    ) {
        prop_assume!(direction_entries != [0, 0, 0, 0]);

        let start = vq(start_entries);
        let direction = vq(direction_entries);
        let midpoint = start.clone() + direction.clone();
        let endpoint = start + direction.clone() + direction;

        let points = vec![vq(start_entries), endpoint, midpoint];

        prop_assert!(!all_points_are_extreme_exact(&points));
    }
}

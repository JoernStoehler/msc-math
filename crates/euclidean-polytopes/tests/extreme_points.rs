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

proptest! {
    #![proptest_config(ProptestConfig::with_cases(32))]

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
}

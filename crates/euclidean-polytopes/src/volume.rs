//! Full-dimensional `R^4` volume and facet 3-volume from known incidence.
//!
//! This module keeps the operation flat. Callers pass primal vertices and a
//! reliable vertex-facet incidence matrix directly. The f64 and exact
//! full-volume helpers share the same incidence convention, and the f64
//! facet-volume helpers use that incidence for facet and 2-face combinatorics.
//! These APIs do not recover incidence from dual facet normals or approximate
//! signed gaps.

use algebraic_numbers::ExactScalar;
use nalgebra::{DMatrix, Matrix4, Vector4};

use crate::f64_geometry::{validate_finite_vectors4, F64GeometryError};

/// Floor for meaningful facet 3-volume in centroid division.
///
/// This matches the legacy symplectic facet-volume behavior. A facet whose
/// accumulated tetrahedron volume is below this floor is reported as zero with
/// the zero centroid instead of dividing by a numerically meaningless total.
const EPS_FACET_VOLUME_FLOOR: f64 = 1e-30;

/// Compute full-dimensional Euclidean volume from known vertex-facet incidence.
///
/// Use this helper when a caller already has reliable combinatorial incidence,
/// for example from an exact construction. This function does not recover
/// incidence from floating-point signed gaps.
///
/// Validated here: every vertex coordinate is finite. Contract assumptions:
/// `incidence[(v, f)]` is true exactly when `vertices[v]` lies on facet `f` of
/// a normalized full-dimensional `R^4` polytope containing the origin. The
/// incidence matrix must have one row per vertex and one column per facet.
///
/// Shape mismatches and violated full-dimensional decomposition assumptions
/// panic as programmer errors.
pub fn volume_from_incidence_f64(
    vertices: &[Vector4<f64>],
    incidence: &DMatrix<bool>,
) -> Result<f64, F64GeometryError> {
    assert_eq!(
        incidence.nrows(),
        vertices.len(),
        "volume_from_incidence_f64 requires incidence rows to match vertices length"
    );
    validate_finite_vectors4("vertices", vertices)?;

    assert!(
        vertices.len() >= 5,
        "volume_from_incidence_f64 requires at least five vertices for a full-dimensional R^4 polytope"
    );
    assert!(
        incidence.ncols() >= 5,
        "volume_from_incidence_f64 requires at least five facets for a full-dimensional bounded R^4 polytope"
    );

    Ok(origin_star_volume_and_centroid(vertices, incidence, "volume_from_incidence_f64").0)
}

/// Compute full-dimensional Euclidean volume and body centroid from known incidence.
///
/// This uses the same decomposition and assumptions as
/// [`volume_from_incidence_f64`]. The centroid is the volume-weighted average
/// of the 4-simplex centroids in the origin-star decomposition. It is a body
/// centroid, not an arithmetic average of vertices.
pub fn volume_and_centroid_from_incidence_f64(
    vertices: &[Vector4<f64>],
    incidence: &DMatrix<bool>,
) -> Result<(f64, Vector4<f64>), F64GeometryError> {
    assert_eq!(
        incidence.nrows(),
        vertices.len(),
        "volume_and_centroid_from_incidence_f64 requires incidence rows to match vertices length"
    );
    validate_finite_vectors4("vertices", vertices)?;

    assert!(
        vertices.len() >= 5,
        "volume_and_centroid_from_incidence_f64 requires at least five vertices for a full-dimensional R^4 polytope"
    );
    assert!(
        incidence.ncols() >= 5,
        "volume_and_centroid_from_incidence_f64 requires at least five facets for a full-dimensional bounded R^4 polytope"
    );

    let (volume, weighted_centroid) = origin_star_volume_and_centroid(
        vertices,
        incidence,
        "volume_and_centroid_from_incidence_f64",
    );
    assert!(
        volume > 0.0,
        "volume_and_centroid_from_incidence_f64 requires positive decomposed volume"
    );
    Ok((volume, weighted_centroid / volume))
}

/// Compute exact full-dimensional Euclidean volume from known vertex-facet incidence.
///
/// Contract assumptions: `incidence[(v, f)]` is true exactly when
/// `vertices[v]` lies on facet `f` of a normalized full-dimensional `R^4`
/// polytope containing the origin. The incidence matrix must have one row per
/// vertex and one column per facet.
///
/// This helper does not take dual vertices and does not recover incidence. It
/// triangulates each facet from the exact arithmetic mean of its incident
/// vertices, orders every polygonal 2-face from incidence only, cones the
/// resulting tetrahedra to the origin, and sums exact determinant volumes.
///
/// Shape mismatches and violated full-dimensional decomposition assumptions
/// panic as programmer errors.
pub fn volume_from_incidence_exact<T: ExactScalar + 'static>(
    vertices: &[Vector4<T>],
    incidence: &DMatrix<bool>,
) -> T {
    assert_eq!(
        incidence.nrows(),
        vertices.len(),
        "volume_from_incidence_exact requires incidence rows to match vertices length"
    );
    assert!(
        vertices.len() >= 5,
        "volume_from_incidence_exact requires at least five vertices for a full-dimensional R^4 polytope"
    );
    assert!(
        incidence.ncols() >= 5,
        "volume_from_incidence_exact requires at least five facets for a full-dimensional bounded R^4 polytope"
    );

    origin_star_volume_exact(vertices, incidence, "volume_from_incidence_exact")
}

/// Compute the ordinary 3D Euclidean volume of one facet from known incidence.
///
/// Use this helper when a caller already has reliable combinatorial incidence,
/// for example from an exact construction. Unlike legacy raw symplectic facet
/// helpers, this function does not recover facet or 2-face incidence from
/// floating-point signed gaps.
///
/// Validated here: every vertex coordinate is finite. Contract assumptions:
/// `incidence[(v, f)]` is true exactly when `vertices[v]` lies on facet `f` of
/// a normalized full-dimensional `R^4` polytope containing the origin. The
/// incidence matrix must have one row per vertex. `facet_index` must be a valid
/// incidence column.
///
/// Shape mismatches, out-of-range facet indices, and violated decomposition
/// assumptions panic as programmer errors. A facet with fewer than four incident
/// vertices has zero reported volume.
pub fn facet_volume_from_incidence_f64(
    vertices: &[Vector4<f64>],
    incidence: &DMatrix<bool>,
    facet_index: usize,
) -> Result<f64, F64GeometryError> {
    let (total_volume, _) =
        facet_volume_centroid_sum_from_incidence(vertices, incidence, facet_index)?;
    Ok(total_volume)
}

/// Compute one facet's ordinary 3D volume and volume-weighted centroid.
///
/// The facet is decomposed by taking the arithmetic mean of its incident
/// vertices as apex, ordering each 2-face `facet_index ∩ neighbor_index` from
/// incidence only, and summing tetrahedron 3-volumes via the ordinary `R^4`
/// cross-product norm divided by `6`.
///
/// The centroid is the volume-weighted average of tetrahedron centroids. If the
/// accumulated volume is below the local facet-volume floor, this returns
/// `(0.0, Vector4::zeros())`.
pub fn facet_volume_and_centroid_from_incidence_f64(
    vertices: &[Vector4<f64>],
    incidence: &DMatrix<bool>,
    facet_index: usize,
) -> Result<(f64, Vector4<f64>), F64GeometryError> {
    let (total_volume, weighted_centroid) =
        facet_volume_centroid_sum_from_incidence(vertices, incidence, facet_index)?;

    if total_volume > EPS_FACET_VOLUME_FLOOR {
        Ok((total_volume, weighted_centroid / total_volume))
    } else {
        Ok((0.0, Vector4::zeros()))
    }
}

fn facet_volume_centroid_sum_from_incidence(
    vertices: &[Vector4<f64>],
    incidence: &DMatrix<bool>,
    facet_index: usize,
) -> Result<(f64, Vector4<f64>), F64GeometryError> {
    assert_eq!(
        incidence.nrows(),
        vertices.len(),
        "known-incidence facet helpers require incidence rows to match vertices length"
    );
    assert!(
        facet_index < incidence.ncols(),
        "known-incidence facet helpers require facet_index to be a valid incidence column"
    );
    validate_finite_vectors4("vertices", vertices)?;

    let facet_vertices = facet_vertices_from_incidence(incidence);
    let target_vertices = &facet_vertices[facet_index];
    if target_vertices.len() < 4 {
        return Ok((0.0, Vector4::zeros()));
    }

    let apex = mean_vertex(vertices, target_vertices);
    let mut total_volume = 0.0;
    let mut weighted_centroid = Vector4::zeros();

    for neighbor_index in 0..incidence.ncols() {
        if neighbor_index == facet_index {
            continue;
        }

        let ordered = order_2face_vertices_from_incidence(incidence, facet_index, neighbor_index);
        if ordered.len() < 3 {
            continue;
        }

        for k in 1..ordered.len() - 1 {
            let a = vertices[ordered[0]];
            let b = vertices[ordered[k]];
            let c = vertices[ordered[k + 1]];
            let tetrahedron_volume = cross_product_4d(a - apex, b - apex, c - apex).norm() / 6.0;
            let tetrahedron_centroid = (apex + a + b + c) / 4.0;

            total_volume += tetrahedron_volume;
            weighted_centroid += tetrahedron_volume * tetrahedron_centroid;
        }
    }

    Ok((total_volume, weighted_centroid))
}

fn facet_vertices_from_incidence(incidence: &DMatrix<bool>) -> Vec<Vec<usize>> {
    (0..incidence.ncols())
        .map(|facet_index| {
            (0..incidence.nrows())
                .filter(|&vertex_index| incidence[(vertex_index, facet_index)])
                .collect()
        })
        .collect()
}

fn origin_star_volume_and_centroid(
    vertices: &[Vector4<f64>],
    incidence: &DMatrix<bool>,
    caller: &str,
) -> (f64, Vector4<f64>) {
    let facet_vertices = facet_vertices_from_incidence(incidence);

    for (facet_index, indices) in facet_vertices.iter().enumerate() {
        assert!(
            indices.len() >= 4,
            "{caller} full-dimensional facet {facet_index} has fewer than four vertices"
        );
    }

    let facet_centroids: Vec<Vector4<f64>> = facet_vertices
        .iter()
        .map(|indices| mean_vertex(vertices, indices))
        .collect();

    let mut total_volume = 0.0;
    let mut weighted_centroid = Vector4::zeros();
    for (facet_index, facet_centroid) in facet_centroids.iter().enumerate() {
        for neighbor_index in 0..incidence.ncols() {
            if facet_index == neighbor_index {
                continue;
            }

            let ordered =
                order_2face_vertices_from_incidence(incidence, facet_index, neighbor_index);
            if ordered.len() < 3 {
                continue;
            }

            for k in 1..ordered.len() - 1 {
                let simplex_volume = simplex_volume_5(
                    Vector4::zeros(),
                    *facet_centroid,
                    vertices[ordered[0]],
                    vertices[ordered[k]],
                    vertices[ordered[k + 1]],
                );
                let simplex_centroid = (*facet_centroid
                    + vertices[ordered[0]]
                    + vertices[ordered[k]]
                    + vertices[ordered[k + 1]])
                    / 5.0;
                total_volume += simplex_volume;
                weighted_centroid += simplex_volume * simplex_centroid;
            }
        }
    }

    (total_volume, weighted_centroid)
}

fn origin_star_volume_exact<T: ExactScalar + 'static>(
    vertices: &[Vector4<T>],
    incidence: &DMatrix<bool>,
    caller: &str,
) -> T {
    let facet_vertices = facet_vertices_from_incidence(incidence);

    for (facet_index, indices) in facet_vertices.iter().enumerate() {
        assert!(
            indices.len() >= 4,
            "{caller} full-dimensional facet {facet_index} has fewer than four vertices"
        );
    }

    let facet_centroids: Vec<Vector4<T>> = facet_vertices
        .iter()
        .map(|indices| mean_vertex_exact(vertices, indices))
        .collect();

    let mut total = T::zero();
    for (facet_index, facet_centroid) in facet_centroids.iter().enumerate() {
        for neighbor_index in 0..incidence.ncols() {
            if facet_index == neighbor_index {
                continue;
            }

            let ordered =
                order_2face_vertices_from_incidence(incidence, facet_index, neighbor_index);
            if ordered.len() < 3 {
                continue;
            }

            for k in 1..ordered.len() - 1 {
                total += simplex_volume_5_exact(
                    &Vector4::zeros(),
                    facet_centroid,
                    &vertices[ordered[0]],
                    &vertices[ordered[k]],
                    &vertices[ordered[k + 1]],
                );
            }
        }
    }

    total
}

fn simplex_volume_5(
    v0: Vector4<f64>,
    v1: Vector4<f64>,
    v2: Vector4<f64>,
    v3: Vector4<f64>,
    v4: Vector4<f64>,
) -> f64 {
    Matrix4::from_columns(&[v1 - v0, v2 - v0, v3 - v0, v4 - v0])
        .determinant()
        .abs()
        / 24.0
}

fn simplex_volume_5_exact<T: ExactScalar>(
    v0: &Vector4<T>,
    v1: &Vector4<T>,
    v2: &Vector4<T>,
    v3: &Vector4<T>,
    v4: &Vector4<T>,
) -> T {
    abs_exact(det4_exact(&[
        vector_sub_exact(v1, v0),
        vector_sub_exact(v2, v0),
        vector_sub_exact(v3, v0),
        vector_sub_exact(v4, v0),
    ])) / exact_from_usize(24)
}

fn det4_exact<T: ExactScalar>(columns: &[Vector4<T>; 4]) -> T {
    let m = |row: usize, col: usize| columns[col][row].clone();

    let minor0 = det3_exact([
        [m(1, 1), m(1, 2), m(1, 3)],
        [m(2, 1), m(2, 2), m(2, 3)],
        [m(3, 1), m(3, 2), m(3, 3)],
    ]);
    let minor1 = det3_exact([
        [m(1, 0), m(1, 2), m(1, 3)],
        [m(2, 0), m(2, 2), m(2, 3)],
        [m(3, 0), m(3, 2), m(3, 3)],
    ]);
    let minor2 = det3_exact([
        [m(1, 0), m(1, 1), m(1, 3)],
        [m(2, 0), m(2, 1), m(2, 3)],
        [m(3, 0), m(3, 1), m(3, 3)],
    ]);
    let minor3 = det3_exact([
        [m(1, 0), m(1, 1), m(1, 2)],
        [m(2, 0), m(2, 1), m(2, 2)],
        [m(3, 0), m(3, 1), m(3, 2)],
    ]);

    m(0, 0) * minor0 - m(0, 1) * minor1 + m(0, 2) * minor2 - m(0, 3) * minor3
}

fn det3_exact<T: ExactScalar>(m: [[T; 3]; 3]) -> T {
    m[0][0].clone() * (m[1][1].clone() * m[2][2].clone() - m[1][2].clone() * m[2][1].clone())
        - m[0][1].clone() * (m[1][0].clone() * m[2][2].clone() - m[1][2].clone() * m[2][0].clone())
        + m[0][2].clone() * (m[1][0].clone() * m[2][1].clone() - m[1][1].clone() * m[2][0].clone())
}

fn vector_sub_exact<T: ExactScalar>(left: &Vector4<T>, right: &Vector4<T>) -> Vector4<T> {
    Vector4::new(
        left[0].clone() - right[0].clone(),
        left[1].clone() - right[1].clone(),
        left[2].clone() - right[2].clone(),
        left[3].clone() - right[3].clone(),
    )
}

fn abs_exact<T: ExactScalar>(value: T) -> T {
    if value < T::zero() {
        -value
    } else {
        value
    }
}

fn cross_product_4d(a: Vector4<f64>, b: Vector4<f64>, c: Vector4<f64>) -> Vector4<f64> {
    let bc_01 = b[0] * c[1] - b[1] * c[0];
    let bc_02 = b[0] * c[2] - b[2] * c[0];
    let bc_03 = b[0] * c[3] - b[3] * c[0];
    let bc_12 = b[1] * c[2] - b[2] * c[1];
    let bc_13 = b[1] * c[3] - b[3] * c[1];
    let bc_23 = b[2] * c[3] - b[3] * c[2];

    Vector4::new(
        a[1] * bc_23 - a[2] * bc_13 + a[3] * bc_12,
        -(a[0] * bc_23 - a[2] * bc_03 + a[3] * bc_02),
        a[0] * bc_13 - a[1] * bc_03 + a[3] * bc_01,
        -(a[0] * bc_12 - a[1] * bc_02 + a[2] * bc_01),
    )
}

fn mean_vertex(vertices: &[Vector4<f64>], indices: &[usize]) -> Vector4<f64> {
    indices
        .iter()
        .map(|&idx| vertices[idx])
        .sum::<Vector4<f64>>()
        / indices.len() as f64
}

fn mean_vertex_exact<T: ExactScalar + 'static>(
    vertices: &[Vector4<T>],
    indices: &[usize],
) -> Vector4<T> {
    let mut sum = Vector4::zeros();
    for &idx in indices {
        sum += vertices[idx].clone();
    }
    sum / exact_from_usize::<T>(indices.len())
}

fn exact_from_usize<T: ExactScalar>(value: usize) -> T {
    let mut result = T::zero();
    for _ in 0..value {
        result += T::one();
    }
    result
}

fn order_2face_vertices_from_incidence(
    incidence: &DMatrix<bool>,
    facet_i: usize,
    facet_j: usize,
) -> Vec<usize> {
    assert_ne!(
        facet_i, facet_j,
        "order_2face_vertices_from_incidence requires two distinct facets"
    );
    assert!(
        facet_i < incidence.ncols(),
        "order_2face_vertices_from_incidence requires facet_i to be a valid incidence column"
    );
    assert!(
        facet_j < incidence.ncols(),
        "order_2face_vertices_from_incidence requires facet_j to be a valid incidence column"
    );

    let shared_vertices: Vec<usize> = (0..incidence.nrows())
        .filter(|&vertex_index| {
            incidence[(vertex_index, facet_i)] && incidence[(vertex_index, facet_j)]
        })
        .collect();
    if shared_vertices.len() <= 2 {
        return shared_vertices;
    }

    let mut neighbors = vec![Vec::new(); shared_vertices.len()];
    for left_position in 0..shared_vertices.len() {
        for right_position in left_position + 1..shared_vertices.len() {
            let left_vertex = shared_vertices[left_position];
            let right_vertex = shared_vertices[right_position];
            if share_third_facet(incidence, left_vertex, right_vertex, facet_i, facet_j) {
                neighbors[left_position].push(right_position);
                neighbors[right_position].push(left_position);
            }
        }
    }

    for (position, vertex_neighbors) in neighbors.iter().enumerate() {
        assert_eq!(
            vertex_neighbors.len(),
            2,
            "order_2face_vertices_from_incidence requires every polygonal 2-face vertex to have degree 2; vertex {} has degree {}",
            shared_vertices[position],
            vertex_neighbors.len()
        );
    }

    let mut order_positions = vec![0];
    let mut previous_position = 0;
    let mut current_position = neighbors[0][0];
    while current_position != 0 {
        assert!(
            !order_positions.contains(&current_position),
            "order_2face_vertices_from_incidence requires the induced graph to be one cycle"
        );
        order_positions.push(current_position);

        let current_neighbors = &neighbors[current_position];
        let next_position = if current_neighbors[0] == previous_position {
            current_neighbors[1]
        } else {
            current_neighbors[0]
        };
        previous_position = current_position;
        current_position = next_position;
    }

    assert_eq!(
        order_positions.len(),
        shared_vertices.len(),
        "order_2face_vertices_from_incidence requires the induced graph to be one cycle"
    );

    order_positions
        .into_iter()
        .map(|position| shared_vertices[position])
        .collect()
}

fn share_third_facet(
    incidence: &DMatrix<bool>,
    left_vertex: usize,
    right_vertex: usize,
    facet_i: usize,
    facet_j: usize,
) -> bool {
    (0..incidence.ncols()).any(|facet_index| {
        facet_index != facet_i
            && facet_index != facet_j
            && incidence[(left_vertex, facet_index)]
            && incidence[(right_vertex, facet_index)]
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn incidence_from_rows(row_facets: &[&[usize]], facet_count: usize) -> DMatrix<bool> {
        DMatrix::from_fn(
            row_facets.len(),
            facet_count,
            |vertex_index, facet_index| row_facets[vertex_index].contains(&facet_index),
        )
    }

    #[test]
    fn order_2face_vertices_returns_empty_intersection() {
        let incidence = incidence_from_rows(&[&[0], &[1], &[2]], 4);

        assert_eq!(order_2face_vertices_from_incidence(&incidence, 0, 1), []);
    }

    #[test]
    fn order_2face_vertices_returns_one_point_intersection() {
        let incidence = incidence_from_rows(&[&[0, 1], &[0], &[1]], 3);

        assert_eq!(order_2face_vertices_from_incidence(&incidence, 0, 1), [0]);
    }

    #[test]
    fn order_2face_vertices_returns_two_point_intersection_in_sorted_order() {
        let incidence = incidence_from_rows(&[&[0, 1, 3], &[0], &[0, 1, 2], &[1]], 4);

        assert_eq!(
            order_2face_vertices_from_incidence(&incidence, 0, 1),
            [0, 2]
        );
    }

    #[test]
    fn order_2face_vertices_orders_triangle_from_incidence() {
        let incidence = incidence_from_rows(&[&[0, 1, 2, 4], &[0, 1, 2, 3], &[0, 1, 3, 4]], 5);

        assert_eq!(
            order_2face_vertices_from_incidence(&incidence, 0, 1),
            [0, 1, 2]
        );
    }

    #[test]
    fn order_2face_vertices_orders_quadrilateral_from_incidence() {
        let incidence = incidence_from_rows(
            &[&[0, 1, 2, 5], &[0, 1, 2, 3], &[0, 1, 3, 4], &[0, 1, 4, 5]],
            6,
        );

        assert_eq!(
            order_2face_vertices_from_incidence(&incidence, 0, 1),
            [0, 1, 2, 3]
        );
    }

    #[test]
    fn order_2face_vertices_orders_higher_vertex_polygon_from_incidence() {
        let incidence = incidence_from_rows(
            &[
                &[0, 1, 2, 6],
                &[0, 1, 2, 3],
                &[0, 1, 3, 4],
                &[0, 1, 4, 5],
                &[0, 1, 5, 6],
            ],
            7,
        );

        assert_eq!(
            order_2face_vertices_from_incidence(&incidence, 0, 1),
            [0, 1, 2, 3, 4]
        );
    }

    #[test]
    #[should_panic(expected = "requires two distinct facets")]
    fn order_2face_vertices_panics_for_same_facet() {
        let incidence = incidence_from_rows(&[&[0, 1]], 2);

        let _ = order_2face_vertices_from_incidence(&incidence, 0, 0);
    }

    #[test]
    #[should_panic(expected = "requires facet_j to be a valid incidence column")]
    fn order_2face_vertices_panics_for_out_of_range_facet() {
        let incidence = incidence_from_rows(&[&[0, 1]], 2);

        let _ = order_2face_vertices_from_incidence(&incidence, 0, 2);
    }

    #[test]
    #[should_panic(expected = "requires the induced graph to be one cycle")]
    fn order_2face_vertices_panics_when_degree_two_graph_is_disconnected() {
        let incidence = incidence_from_rows(
            &[
                &[0, 1, 2, 4],
                &[0, 1, 2, 3],
                &[0, 1, 3, 4],
                &[0, 1, 5, 7],
                &[0, 1, 5, 6],
                &[0, 1, 6, 7],
            ],
            8,
        );

        let _ = order_2face_vertices_from_incidence(&incidence, 0, 1);
    }

    #[test]
    #[should_panic(expected = "to have degree 2")]
    fn order_2face_vertices_panics_when_polygonal_graph_has_wrong_degree() {
        let incidence = incidence_from_rows(&[&[0, 1, 2], &[0, 1, 2, 3], &[0, 1, 3]], 4);

        let _ = order_2face_vertices_from_incidence(&incidence, 0, 1);
    }

    #[test]
    fn known_fixture_polygonal_2face_intersections_are_orderable() {
        let fixtures = [
            centered_simplex_incidence(),
            hypercube_incidence(),
            crosspolytope_incidence(),
            diagonal_cut_hypercube_incidence(),
        ];

        for incidence in fixtures {
            assert_polygonal_2face_intersections_are_orderable(&incidence);
        }
    }

    fn assert_polygonal_2face_intersections_are_orderable(incidence: &DMatrix<bool>) {
        for facet_i in 0..incidence.ncols() {
            for facet_j in facet_i + 1..incidence.ncols() {
                let shared_vertices: Vec<usize> = (0..incidence.nrows())
                    .filter(|&vertex_index| {
                        incidence[(vertex_index, facet_i)] && incidence[(vertex_index, facet_j)]
                    })
                    .collect();
                if shared_vertices.len() < 3 {
                    continue;
                }

                let ordered = order_2face_vertices_from_incidence(incidence, facet_i, facet_j);
                let mut sorted_ordered = ordered.clone();
                sorted_ordered.sort_unstable();
                assert_eq!(sorted_ordered, shared_vertices);
            }
        }
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

    fn hypercube_incidence() -> DMatrix<bool> {
        DMatrix::from_fn(16, 8, |vertex_index, facet_index| {
            let coordinate_index = facet_index / 2;
            let positive_facet = facet_index % 2 == 0;
            let positive_vertex = ((vertex_index >> coordinate_index) & 1) == 1;
            positive_vertex == positive_facet
        })
    }

    fn crosspolytope_incidence() -> DMatrix<bool> {
        DMatrix::from_fn(8, 16, |vertex_index, facet_index| {
            let coordinate_index = vertex_index / 2;
            let positive_vertex = vertex_index % 2 == 0;
            let facet_sign_bit = (facet_index >> (3 - coordinate_index)) & 1;
            positive_vertex == (facet_sign_bit == 1)
        })
    }

    fn diagonal_cut_hypercube_incidence() -> DMatrix<bool> {
        let vertices: Vec<[i32; 4]> = [-1, 1]
            .into_iter()
            .flat_map(|x0| {
                [-1, 1].into_iter().flat_map(move |x1| {
                    [-1, 1]
                        .into_iter()
                        .flat_map(move |x2| [-1, 1].into_iter().map(move |x3| [x0, x1, x2, x3]))
                })
            })
            .filter(|vertex| vertex.iter().sum::<i32>() <= 2)
            .collect();

        DMatrix::from_fn(vertices.len(), 9, |vertex_index, facet_index| {
            let vertex = vertices[vertex_index];
            if facet_index < 8 {
                let coordinate_index = facet_index / 2;
                let positive_facet = facet_index % 2 == 0;
                vertex[coordinate_index] == if positive_facet { 1 } else { -1 }
            } else {
                vertex.iter().sum::<i32>() == 2
            }
        })
    }
}

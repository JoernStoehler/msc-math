//! Two-face symplectic area feature columns.

use euclidean_polytopes::TwoFace;
use nalgebra::{DMatrix, Vector4};
use symplectic::geom::symplectic_form::omega0;

use super::features_helpers::{
    fraction_at_most, max_share, quantile_or_zero, stats_or_zero, top_k_share,
};

pub struct FaceSymplecticFields {
    pub ridge_symp_area_ordered_face_count: usize,
    pub ridge_symp_area_ordering_failure_count: usize,
    pub ridge_symp_area_ordered_fraction: f64,
    pub ridge_symp_area_mean: f64,
    pub ridge_symp_area_std: f64,
    pub ridge_symp_area_min: f64,
    pub ridge_symp_area_max: f64,
    pub ridge_symp_area_q25: f64,
    pub ridge_symp_area_median: f64,
    pub ridge_symp_area_q75: f64,
    pub ridge_symp_area_q90: f64,
    pub ridge_symp_area_q95: f64,
    pub ridge_symp_area_sum: f64,
    pub ridge_symp_area_max_share: f64,
    pub ridge_symp_area_top3_share: f64,
    pub ridge_symp_area_le_1em3_over_volume_sqrt_fraction: f64,
    pub ridge_symp_area_le_1em2_over_volume_sqrt_fraction: f64,
    pub ridge_symp_area_le_1em1_over_volume_sqrt_fraction: f64,
    pub ridge_symp_area_entropy: f64,
    pub ridge_symp_area_effective_face_count: f64,
    pub ridge_symp_area_normalized_entropy: f64,
}

#[derive(Clone, Debug)]
pub struct OrderedTwoFaceSymplecticArea {
    pub facets: [usize; 2],
    pub vertices: Vec<usize>,
    pub area: f64,
}

fn two_face_symplectic_area(vertices: &[Vector4<f64>]) -> f64 {
    if vertices.len() < 3 {
        return 0.0;
    }
    let doubled_area = (0..vertices.len())
        .map(|idx| {
            let next = (idx + 1) % vertices.len();
            omega0(&vertices[idx], &vertices[next])
        })
        .sum::<f64>();
    0.5 * doubled_area.abs()
}

fn share_third_facet(
    incidence: &DMatrix<bool>,
    left_vertex: usize,
    right_vertex: usize,
    face_facets: [usize; 2],
) -> bool {
    (0..incidence.ncols()).any(|facet_index| {
        !face_facets.contains(&facet_index)
            && incidence[(left_vertex, facet_index)]
            && incidence[(right_vertex, facet_index)]
    })
}

fn order_two_face_vertices_from_incidence(
    incidence: &DMatrix<bool>,
    two_face: &TwoFace,
) -> Option<Vec<usize>> {
    if two_face.vertices.len() < 3 {
        return None;
    }

    let mut neighbors = vec![Vec::new(); two_face.vertices.len()];
    for left_position in 0..two_face.vertices.len() {
        for right_position in left_position + 1..two_face.vertices.len() {
            let left_vertex = two_face.vertices[left_position];
            let right_vertex = two_face.vertices[right_position];
            if share_third_facet(incidence, left_vertex, right_vertex, two_face.facets) {
                neighbors[left_position].push(right_position);
                neighbors[right_position].push(left_position);
            }
        }
    }

    if neighbors
        .iter()
        .any(|vertex_neighbors| vertex_neighbors.len() != 2)
    {
        return None;
    }

    let mut order_positions = vec![0];
    let mut previous_position = 0;
    let mut current_position = neighbors[0][0];
    while current_position != 0 {
        if order_positions.contains(&current_position) {
            return None;
        }
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

    (order_positions.len() == two_face.vertices.len()).then(|| {
        order_positions
            .into_iter()
            .map(|position| two_face.vertices[position])
            .collect()
    })
}

/// Return the successfully ordered two-faces and their unsigned symplectic
/// areas. The face identity is the unordered containing-facet pair; the vertex
/// order is cyclic but its starting point and orientation are not canonical.
/// The unsigned area is invariant under either choice.
pub fn ordered_two_face_symplectic_areas(
    two_faces: &[TwoFace],
    vertices: &[Vector4<f64>],
    incidence: &DMatrix<bool>,
) -> (Vec<OrderedTwoFaceSymplecticArea>, usize) {
    let mut ordered = Vec::new();
    let mut ordering_failure_count = 0usize;
    for two_face in two_faces {
        let Some(ordered_vertices) = order_two_face_vertices_from_incidence(incidence, two_face)
        else {
            ordering_failure_count += 1;
            continue;
        };
        let two_face_vertices = ordered_vertices
            .iter()
            .map(|&vertex| vertices[vertex])
            .collect::<Vec<_>>();
        ordered.push(OrderedTwoFaceSymplecticArea {
            facets: two_face.facets,
            vertices: ordered_vertices,
            area: two_face_symplectic_area(&two_face_vertices),
        });
    }
    (ordered, ordering_failure_count)
}

pub fn compute_face_symplectic_fields(
    two_faces: &[TwoFace],
    vertices: &[Vector4<f64>],
    incidence: &DMatrix<bool>,
    volume_sqrt: f64,
) -> FaceSymplecticFields {
    let (ordered_faces, ordering_failure_count) =
        ordered_two_face_symplectic_areas(two_faces, vertices, incidence);
    let ridge_symp_areas = ordered_faces
        .iter()
        .map(|face| face.area)
        .collect::<Vec<_>>();
    let (ridge_symp_area_mean, ridge_symp_area_std, ridge_symp_area_min, ridge_symp_area_max) =
        stats_or_zero(&ridge_symp_areas);
    let normalized_areas = ridge_symp_areas
        .iter()
        .map(|area| area / volume_sqrt)
        .collect::<Vec<_>>();
    let total_area = ridge_symp_areas.iter().sum::<f64>();
    let ridge_symp_area_entropy = if total_area <= 0.0 {
        0.0
    } else {
        ridge_symp_areas
            .iter()
            .filter_map(|area| {
                let probability = area / total_area;
                (probability > 0.0).then_some(-probability * probability.ln())
            })
            .sum::<f64>()
    };
    let ridge_symp_area_effective_face_count = if total_area <= 0.0 {
        0.0
    } else {
        ridge_symp_area_entropy.exp()
    };
    let ridge_symp_area_normalized_entropy = if ridge_symp_areas.len() <= 1 {
        0.0
    } else {
        ridge_symp_area_entropy / (ridge_symp_areas.len() as f64).ln()
    };

    FaceSymplecticFields {
        ridge_symp_area_ordered_face_count: ridge_symp_areas.len(),
        ridge_symp_area_ordering_failure_count: ordering_failure_count,
        ridge_symp_area_ordered_fraction: if two_faces.is_empty() {
            0.0
        } else {
            ridge_symp_areas.len() as f64 / two_faces.len() as f64
        },
        ridge_symp_area_mean,
        ridge_symp_area_std,
        ridge_symp_area_min,
        ridge_symp_area_max,
        ridge_symp_area_q25: quantile_or_zero(&ridge_symp_areas, 0.25),
        ridge_symp_area_median: quantile_or_zero(&ridge_symp_areas, 0.50),
        ridge_symp_area_q75: quantile_or_zero(&ridge_symp_areas, 0.75),
        ridge_symp_area_q90: quantile_or_zero(&ridge_symp_areas, 0.90),
        ridge_symp_area_q95: quantile_or_zero(&ridge_symp_areas, 0.95),
        ridge_symp_area_sum: ridge_symp_areas.iter().sum::<f64>(),
        ridge_symp_area_max_share: max_share(&ridge_symp_areas),
        ridge_symp_area_top3_share: top_k_share(&ridge_symp_areas, 3),
        ridge_symp_area_le_1em3_over_volume_sqrt_fraction: fraction_at_most(
            &normalized_areas,
            1e-3,
        ),
        ridge_symp_area_le_1em2_over_volume_sqrt_fraction: fraction_at_most(
            &normalized_areas,
            1e-2,
        ),
        ridge_symp_area_le_1em1_over_volume_sqrt_fraction: fraction_at_most(
            &normalized_areas,
            1e-1,
        ),
        ridge_symp_area_entropy,
        ridge_symp_area_effective_face_count,
        ridge_symp_area_normalized_entropy,
    }
}

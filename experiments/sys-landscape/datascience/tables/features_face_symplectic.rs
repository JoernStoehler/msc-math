//! Two-face symplectic area feature columns.

use euclidean_polytopes::TwoFace;
use nalgebra::Vector4;
use symplectic::geom::symplectic_form::omega0;

use super::features_helpers::{fraction_at_most, max_share, stats_or_zero};

const EPS_BASIS_DEGENERATE: f64 = 1e-12;
const EPS_COLLINEAR: f64 = 1e-10;

pub struct FaceSymplecticFields {
    pub ridge_symp_area_volnorm_mean: f64,
    pub ridge_symp_area_volnorm_std: f64,
    pub ridge_symp_area_volnorm_min: f64,
    pub ridge_symp_area_volnorm_max: f64,
    pub ridge_symp_area_volnorm_sum: f64,
    pub ridge_symp_area_volnorm_max_share: f64,
    pub ridge_symp_area_volnorm_zero_fraction: f64,
    pub ridge_symp_area_volnorm_le_1em3_fraction: f64,
    pub ridge_symp_area_volnorm_le_1em2_fraction: f64,
    pub ridge_symp_area_volnorm_le_1em1_fraction: f64,
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

fn sort_two_face_vertices(all_vertices: &[Vector4<f64>], indices: &[usize]) -> Vec<usize> {
    if indices.len() < 3 {
        return indices.to_vec();
    }

    let coords: Vec<Vector4<f64>> = indices.iter().map(|&index| all_vertices[index]).collect();
    let centroid = coords.iter().copied().sum::<Vector4<f64>>() / coords.len() as f64;
    let d1_raw = coords[0] - centroid;
    let d1_norm = d1_raw.norm();
    if d1_norm < EPS_BASIS_DEGENERATE {
        return indices.to_vec();
    }
    let d1 = d1_raw / d1_norm;

    let Some(d2) = coords.iter().skip(1).find_map(|vertex| {
        let rel = *vertex - centroid;
        let proj = rel - d1 * rel.dot(&d1);
        (proj.norm() > EPS_COLLINEAR).then(|| proj.normalize())
    }) else {
        return indices.to_vec();
    };

    let mut indexed_angles: Vec<(f64, usize)> = coords
        .iter()
        .enumerate()
        .map(|(position, vertex)| {
            let rel = *vertex - centroid;
            (rel.dot(&d2).atan2(rel.dot(&d1)), position)
        })
        .collect();
    indexed_angles.sort_by(|left, right| left.0.partial_cmp(&right.0).unwrap());
    indexed_angles
        .into_iter()
        .map(|(_, position)| indices[position])
        .collect()
}

pub fn compute_face_symplectic_fields(
    two_faces: &[TwoFace],
    vertices: &[Vector4<f64>],
    volume_scale: f64,
) -> FaceSymplecticFields {
    let ridge_symp_areas = two_faces
        .iter()
        .map(|two_face| {
            let ordered_vertices = sort_two_face_vertices(vertices, &two_face.vertices);
            let two_face_vertices = ordered_vertices
                .iter()
                .map(|&vertex| vertices[vertex])
                .collect::<Vec<_>>();
            two_face_symplectic_area(&two_face_vertices) / volume_scale
        })
        .collect::<Vec<_>>();
    let (
        ridge_symp_area_volnorm_mean,
        ridge_symp_area_volnorm_std,
        ridge_symp_area_volnorm_min,
        ridge_symp_area_volnorm_max,
    ) = stats_or_zero(&ridge_symp_areas);

    FaceSymplecticFields {
        ridge_symp_area_volnorm_mean,
        ridge_symp_area_volnorm_std,
        ridge_symp_area_volnorm_min,
        ridge_symp_area_volnorm_max,
        ridge_symp_area_volnorm_sum: ridge_symp_areas.iter().sum::<f64>(),
        ridge_symp_area_volnorm_max_share: max_share(&ridge_symp_areas),
        ridge_symp_area_volnorm_zero_fraction: fraction_at_most(&ridge_symp_areas, 1e-12),
        ridge_symp_area_volnorm_le_1em3_fraction: fraction_at_most(&ridge_symp_areas, 1e-3),
        ridge_symp_area_volnorm_le_1em2_fraction: fraction_at_most(&ridge_symp_areas, 1e-2),
        ridge_symp_area_volnorm_le_1em1_fraction: fraction_at_most(&ridge_symp_areas, 1e-1),
    }
}

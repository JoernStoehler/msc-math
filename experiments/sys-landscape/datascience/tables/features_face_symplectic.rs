//! Ridge-level symplectic area feature columns.

use nalgebra::Vector4;
use symplectic::geom::skeleton::Skeleton;
use symplectic::geom::symplectic_form::omega0;

use super::features_helpers::{fraction_at_most, max_share, stats_or_zero};

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

fn ridge_symplectic_area(vertices: &[Vector4<f64>]) -> f64 {
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

pub fn compute_face_symplectic_fields(
    skeleton: &Skeleton,
    vertices: &[Vector4<f64>],
    volume_scale: f64,
) -> FaceSymplecticFields {
    let ridge_symp_areas = skeleton
        .ridges
        .iter()
        .map(|ridge| {
            let ridge_vertices = ridge
                .vertices
                .iter()
                .map(|&vertex| vertices[vertex])
                .collect::<Vec<_>>();
            ridge_symplectic_area(&ridge_vertices) / volume_scale
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

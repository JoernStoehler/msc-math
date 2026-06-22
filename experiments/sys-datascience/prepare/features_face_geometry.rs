//! Edge-length and facet-volume feature columns.

use exp_sys_landscape::SysLandscapePolytopeCache;
use nalgebra::Vector4;
use symplectic::geom::facet_volume::facet_volume_from_incidence_f64;

use super::features_helpers::{max_share, stats_or_zero};

pub struct FaceGeometryFields {
    pub edge_length_vol1_mean: f64,
    pub edge_length_vol1_std: f64,
    pub edge_length_vol1_min: f64,
    pub edge_length_vol1_max: f64,
    pub edge_length_vol1_max_share: f64,
    pub facet_volume_vol1_mean: f64,
    pub facet_volume_vol1_std: f64,
    pub facet_volume_vol1_min: f64,
    pub facet_volume_vol1_max: f64,
    pub facet_volume_vol1_sum: f64,
    pub facet_volume_vol1_max_share: f64,
}

pub fn compute_face_geometry_fields(
    polytope: &SysLandscapePolytopeCache,
    edges: &[[usize; 2]],
    vertices: &[Vector4<f64>],
    facet_count: usize,
    linear_scale: f64,
    facet_scale: f64,
) -> FaceGeometryFields {
    let edge_lengths = edges
        .iter()
        .map(|edge| (vertices[edge[0]] - vertices[edge[1]]).norm() / linear_scale)
        .collect::<Vec<_>>();
    let facet_volumes = (0..facet_count)
        .map(|facet| {
            facet_volume_from_incidence_f64(
                &polytope.vertices_f64,
                &polytope.vertex_facet_incidence,
                facet,
            )
            .expect("dataset polytope has valid finite geometry")
                / facet_scale
        })
        .collect::<Vec<_>>();
    let (edge_length_vol1_mean, edge_length_vol1_std, edge_length_vol1_min, edge_length_vol1_max) =
        stats_or_zero(&edge_lengths);
    let (
        facet_volume_vol1_mean,
        facet_volume_vol1_std,
        facet_volume_vol1_min,
        facet_volume_vol1_max,
    ) = stats_or_zero(&facet_volumes);

    FaceGeometryFields {
        edge_length_vol1_mean,
        edge_length_vol1_std,
        edge_length_vol1_min,
        edge_length_vol1_max,
        edge_length_vol1_max_share: max_share(&edge_lengths),
        facet_volume_vol1_mean,
        facet_volume_vol1_std,
        facet_volume_vol1_min,
        facet_volume_vol1_max,
        facet_volume_vol1_sum: facet_volumes.iter().sum::<f64>(),
        facet_volume_vol1_max_share: max_share(&facet_volumes),
    }
}

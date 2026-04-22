use nalgebra::Vector4;
use symplectic::geom::facet_volume::facet_volume_3d;
use symplectic::geom::polytope::Polytope4D;
use symplectic::geom::skeleton::Skeleton;

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

fn stats_or_zero(values: &[f64]) -> (f64, f64, f64, f64) {
    if values.is_empty() {
        return (0.0, 0.0, 0.0, 0.0);
    }
    let mean = values.iter().sum::<f64>() / values.len() as f64;
    let var = values
        .iter()
        .map(|value| {
            let delta = value - mean;
            delta * delta
        })
        .sum::<f64>()
        / values.len() as f64;
    let min = values.iter().copied().fold(f64::INFINITY, f64::min);
    let max = values.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    (mean, var.sqrt(), min, max)
}

fn max_share(values: &[f64]) -> f64 {
    let total = values.iter().sum::<f64>();
    if total <= 0.0 {
        return 0.0;
    }
    values.iter().copied().fold(f64::NEG_INFINITY, f64::max) / total
}

pub fn compute(
    polytope: &Polytope4D,
    skeleton: &Skeleton,
    vertices: &[Vector4<f64>],
    facet_count: usize,
    linear_scale: f64,
    facet_scale: f64,
) -> FaceGeometryFields {
    let edge_lengths = skeleton
        .edges
        .iter()
        .map(|edge| (vertices[edge[0]] - vertices[edge[1]]).norm() / linear_scale)
        .collect::<Vec<_>>();
    let facet_volumes = (0..facet_count)
        .map(|facet| facet_volume_3d(polytope, facet) / facet_scale)
        .collect::<Vec<_>>();
    let (edge_length_vol1_mean, edge_length_vol1_std, edge_length_vol1_min, edge_length_vol1_max) =
        stats_or_zero(&edge_lengths);
    let (facet_volume_vol1_mean, facet_volume_vol1_std, facet_volume_vol1_min, facet_volume_vol1_max) =
        stats_or_zero(&facet_volumes);

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

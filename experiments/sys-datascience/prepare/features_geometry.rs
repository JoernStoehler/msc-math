//! Basic geometry summary feature columns from dual-vertex coordinates.

use nalgebra::DMatrix;

use super::features_helpers::stats_or_zero;

pub struct GeometryFields {
    pub geom_norm_mean: f64,
    pub geom_norm_std: f64,
    pub geom_norm_min: f64,
    pub geom_norm_max: f64,
    pub geom_centroid_norm: f64,
    pub geom_coord_std_x: f64,
    pub geom_coord_std_y: f64,
    pub geom_coord_std_z: f64,
    pub geom_coord_std_w: f64,
    pub geom_cosine_mean: f64,
    pub geom_cosine_std: f64,
    pub geom_cosine_min: f64,
    pub geom_cosine_max: f64,
    pub geom_pairwise_dist_mean: f64,
    pub geom_pairwise_dist_std: f64,
    pub geom_pairwise_dist_min: f64,
    pub geom_pairwise_dist_max: f64,
    pub geom_sval_1: f64,
    pub geom_sval_2: f64,
    pub geom_sval_3: f64,
    pub geom_sval_4: f64,
}

fn centered_singular_values(vertices: &[[f64; 4]]) -> [f64; 4] {
    let arr = DMatrix::from_fn(vertices.len(), 4, |r, c| vertices[r][c]);
    let mut centered = arr.clone();
    for c in 0..4 {
        let mean = (0..vertices.len()).map(|r| arr[(r, c)]).sum::<f64>() / vertices.len() as f64;
        for r in 0..vertices.len() {
            centered[(r, c)] -= mean;
        }
    }
    let svd = centered.svd(false, false);
    let mut out = [0.0; 4];
    for (idx, value) in svd.singular_values.iter().take(4).enumerate() {
        out[idx] = *value;
    }
    out
}

pub fn compute_geometry_fields(dual_vertices: &[[f64; 4]]) -> GeometryFields {
    let norms = dual_vertices
        .iter()
        .map(|vertex| vertex.iter().map(|coord| coord * coord).sum::<f64>().sqrt())
        .collect::<Vec<_>>();
    let centroid: [f64; 4] = std::array::from_fn(|i| {
        dual_vertices.iter().map(|vertex| vertex[i]).sum::<f64>() / dual_vertices.len() as f64
    });
    let centroid_norm = centroid
        .iter()
        .map(|coord| coord * coord)
        .sum::<f64>()
        .sqrt();
    let coord_std: [f64; 4] = std::array::from_fn(|i| {
        let mean = centroid[i];
        let var = dual_vertices
            .iter()
            .map(|vertex| {
                let delta = vertex[i] - mean;
                delta * delta
            })
            .sum::<f64>()
            / dual_vertices.len() as f64;
        var.sqrt()
    });
    let mut cosines = Vec::new();
    let mut pairwise_distances = Vec::new();
    for i in 0..dual_vertices.len() {
        for j in (i + 1)..dual_vertices.len() {
            let dot = (0..4)
                .map(|k| dual_vertices[i][k] * dual_vertices[j][k])
                .sum::<f64>();
            let denom = norms[i] * norms[j];
            if denom > 0.0 {
                cosines.push(dot / denom);
            }
            pairwise_distances.push(
                (0..4)
                    .map(|k| {
                        let delta = dual_vertices[i][k] - dual_vertices[j][k];
                        delta * delta
                    })
                    .sum::<f64>()
                    .sqrt(),
            );
        }
    }
    let (geom_norm_mean, geom_norm_std, geom_norm_min, geom_norm_max) = stats_or_zero(&norms);
    let (geom_cosine_mean, geom_cosine_std, geom_cosine_min, geom_cosine_max) =
        stats_or_zero(&cosines);
    let (
        geom_pairwise_dist_mean,
        geom_pairwise_dist_std,
        geom_pairwise_dist_min,
        geom_pairwise_dist_max,
    ) = stats_or_zero(&pairwise_distances);
    let singular_values = centered_singular_values(dual_vertices);

    GeometryFields {
        geom_norm_mean,
        geom_norm_std,
        geom_norm_min,
        geom_norm_max,
        geom_centroid_norm: centroid_norm,
        geom_coord_std_x: coord_std[0],
        geom_coord_std_y: coord_std[1],
        geom_coord_std_z: coord_std[2],
        geom_coord_std_w: coord_std[3],
        geom_cosine_mean,
        geom_cosine_std,
        geom_cosine_min,
        geom_cosine_max,
        geom_pairwise_dist_mean,
        geom_pairwise_dist_std,
        geom_pairwise_dist_min,
        geom_pairwise_dist_max,
        geom_sval_1: singular_values[0],
        geom_sval_2: singular_values[1],
        geom_sval_3: singular_values[2],
        geom_sval_4: singular_values[3],
    }
}

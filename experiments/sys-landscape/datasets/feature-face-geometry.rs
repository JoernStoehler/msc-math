//! Compute a bounded face-level Euclidean feature table keyed by `poly_id`.
//!
//! Goal: enrich the hostile-landscape normalized dataset with scalar summaries
//! of edge lengths and facet 3-volumes derived from exact polytope geometry
//! after rescaling each polytope to the `vol(K)=1` convention.
//! TODO: add formal labels in `formal/sys-landscape/*.tex` for the `vol(K)=1`
//! face-geometry normalization rule and cite them here as `[def:...]` / `[lem:...]`.
//! Input Artifacts:
//!   - experiments/sys-landscape/normalized-dataset outputs under `--normalized-dir`
//!     (`polytopes.jsonl` required)
//! Output Artifacts: None by default (writes to an untracked temp file unless `--out` is set)

use exp_sys_landscape::features::{
    deserialize_vec4_rational, parse_standard_feature_args, read_jsonl, write_jsonl,
};
use num_rational::BigRational;
use serde::{Deserialize, Serialize};
use symplectic::geom::facet_volume::facet_volume_3d;
use symplectic::geom::polytope::Polytope4D;
use symplectic::geom::skeleton::Skeleton;
use symplectic::geom::volume::volume;

#[derive(Debug, Deserialize)]
struct PolytopeInputRow {
    poly_id: String,
    #[serde(deserialize_with = "deserialize_vec4_rational")]
    dual_vertices_rational: Vec<[BigRational; 4]>,
    #[serde(deserialize_with = "deserialize_vec4_rational")]
    vertices_rational: Vec<[BigRational; 4]>,
    facet_count: usize,
}

#[derive(Debug, Serialize)]
struct FaceGeometryFeatureRow {
    poly_id: String,
    facet_count: usize,
    vertex_count: usize,
    edge_count: usize,
    edge_length_vol1_mean: f64,
    edge_length_vol1_std: f64,
    edge_length_vol1_min: f64,
    edge_length_vol1_max: f64,
    edge_length_vol1_max_share: f64,
    facet_volume_vol1_mean: f64,
    facet_volume_vol1_std: f64,
    facet_volume_vol1_min: f64,
    facet_volume_vol1_max: f64,
    facet_volume_vol1_sum: f64,
    facet_volume_vol1_max_share: f64,
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

fn build_row(poly: &PolytopeInputRow) -> FaceGeometryFeatureRow {
    let polytope = Polytope4D::from_rational_parts(
        poly.dual_vertices_rational.clone(),
        poly.vertices_rational.clone(),
    )
    .unwrap_or_else(|e| panic!("reconstruct {}: {e}", poly.poly_id));
    let polytope_volume = volume(&polytope);
    let linear_scale = polytope_volume.powf(0.25);
    let facet_scale = polytope_volume.powf(0.75);
    assert!(
        linear_scale > 0.0 && facet_scale > 0.0,
        "volume-normalization scale must be positive for {}",
        poly.poly_id
    );
    let skeleton = Skeleton::compute(&polytope);
    let vertices = polytope.vertices_f64();

    let edge_lengths = skeleton
        .edges
        .iter()
        .map(|edge| (vertices[edge[0]] - vertices[edge[1]]).norm() / linear_scale)
        .collect::<Vec<_>>();
    let facet_volumes = (0..poly.facet_count)
        .map(|facet| facet_volume_3d(&polytope, facet) / facet_scale)
        .collect::<Vec<_>>();

    let (
        edge_length_vol1_mean,
        edge_length_vol1_std,
        edge_length_vol1_min,
        edge_length_vol1_max,
    ) = stats_or_zero(&edge_lengths);
    let (
        facet_volume_vol1_mean,
        facet_volume_vol1_std,
        facet_volume_vol1_min,
        facet_volume_vol1_max,
    ) = stats_or_zero(&facet_volumes);

    FaceGeometryFeatureRow {
        poly_id: poly.poly_id.clone(),
        facet_count: poly.facet_count,
        vertex_count: vertices.len(),
        edge_count: skeleton.edges.len(),
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

fn main() {
    let args = parse_standard_feature_args("face-geometry");
    let polytopes = read_jsonl::<PolytopeInputRow>(&args.normalized_dir.join("polytopes.jsonl"));
    let mut rows = polytopes.iter().map(build_row).collect::<Vec<_>>();
    rows.sort_by(|a, b| a.poly_id.cmp(&b.poly_id));
    write_jsonl(&args.out, &rows);
    println!("Wrote {} face-geometry rows", rows.len());
    println!("Output path: {}", args.out.display());
}

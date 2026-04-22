//! Compute a bounded face-level symplectic feature table keyed by `poly_id`.
//!
//! Goal: enrich the hostile-landscape normalized dataset with volume-normalized
//! ridge-local symplectic area summaries derived from ordered ridge polygons in
//! exact 4D polytope geometry.
//! TODO: add formal labels in `formal/sys-landscape/*.tex` for the ridge
//! symplectic-area definition and the `vol(K)^(1/2)` normalization rule, then
//! cite them here as `[def:...]` / `[lem:...]`.
//! Input Artifacts:
//!   - experiments/sys-landscape/normalized-dataset outputs under `--normalized-dir`
//!     (`polytopes.jsonl` required)
//! Output Artifacts: None by default (writes to an untracked temp file unless `--out` is set)

use exp_sys_landscape::features::{
    deserialize_vec4_rational, parse_standard_feature_args, read_jsonl, write_jsonl,
};
use num_rational::BigRational;
use serde::{Deserialize, Serialize};
use symplectic::geom::polytope::Polytope4D;
use symplectic::geom::skeleton::Skeleton;
use symplectic::geom::symplectic_form::omega0;
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
struct FaceSymplecticFeatureRow {
    poly_id: String,
    facet_count: usize,
    ridge_count: usize,
    ridge_symp_area_volnorm_mean: f64,
    ridge_symp_area_volnorm_std: f64,
    ridge_symp_area_volnorm_min: f64,
    ridge_symp_area_volnorm_max: f64,
    ridge_symp_area_volnorm_sum: f64,
    ridge_symp_area_volnorm_max_share: f64,
    ridge_symp_area_volnorm_zero_fraction: f64,
    ridge_symp_area_volnorm_le_1em3_fraction: f64,
    ridge_symp_area_volnorm_le_1em2_fraction: f64,
    ridge_symp_area_volnorm_le_1em1_fraction: f64,
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

fn fraction_at_most(values: &[f64], threshold: f64) -> f64 {
    if values.is_empty() {
        return 0.0;
    }
    values.iter().filter(|&&value| value <= threshold).count() as f64 / values.len() as f64
}

fn ridge_symplectic_area(vertices: &[nalgebra::Vector4<f64>]) -> f64 {
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

fn build_row(poly: &PolytopeInputRow) -> FaceSymplecticFeatureRow {
    let polytope = Polytope4D::from_rational_parts(
        poly.dual_vertices_rational.clone(),
        poly.vertices_rational.clone(),
    )
    .unwrap_or_else(|e| panic!("reconstruct {}: {e}", poly.poly_id));
    let polytope_volume = volume(&polytope);
    let volume_scale = polytope_volume.sqrt();
    assert!(
        volume_scale > 0.0,
        "volume-normalization scale must be positive for {}",
        poly.poly_id
    );
    let skeleton = Skeleton::compute(&polytope);
    let vertices = polytope.vertices_f64();

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

    FaceSymplecticFeatureRow {
        poly_id: poly.poly_id.clone(),
        facet_count: poly.facet_count,
        ridge_count: skeleton.ridges.len(),
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

fn main() {
    let args = parse_standard_feature_args("face-symplectic");
    let polytopes = read_jsonl::<PolytopeInputRow>(&args.normalized_dir.join("polytopes.jsonl"));
    let mut rows = polytopes.iter().map(build_row).collect::<Vec<_>>();
    rows.sort_by(|a, b| a.poly_id.cmp(&b.poly_id));
    write_jsonl(&args.out, &rows);
    println!("Wrote {} face-symplectic rows", rows.len());
    println!("Output path: {}", args.out.display());
}

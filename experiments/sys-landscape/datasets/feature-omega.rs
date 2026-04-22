//! Compute a bounded symplectic/transition feature table keyed by `poly_id`.
//!
//! Goal: enrich the hostile-landscape normalized dataset with cheap symplectic
//! summaries from exact facet adjacency, omega signs, and ridge-local `omega_0`
//! magnitudes, without orbit recomputation, after rescaling each polytope to
//! the `vol(K)=1` convention.
//! TODO: add formal labels in `formal/sys-landscape/*.tex` for the
//! volume-normalized dual-side `omega_0` summaries and cite them here as
//! `[def:...]` / `[lem:...]`.
//! Input Artifacts:
//!   - experiments/sys-landscape/normalized-dataset outputs under `--normalized-dir`
//!     (`polytopes.jsonl` required)
//! Output Artifacts: None by default (writes to an untracked temp file unless `--out` is set)

use exp_sys_landscape::features::{
    deserialize_vec4_rational, parse_standard_feature_args, read_jsonl, write_jsonl,
};
use num_rational::BigRational;
use serde::{Deserialize, Serialize};
use symplectic::algorithms::facet_adjacency::build_transition_matrix;
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
struct OmegaFeatureRow {
    poly_id: String,
    facet_count: usize,
    allpair_abs_omega_vol1_mean: f64,
    allpair_abs_omega_vol1_std: f64,
    allpair_abs_omega_vol1_min: f64,
    allpair_abs_omega_vol1_max: f64,
    allpair_zero_fraction: f64,
    ridge_abs_omega_vol1_mean: f64,
    ridge_abs_omega_vol1_std: f64,
    ridge_abs_omega_vol1_min: f64,
    ridge_abs_omega_vol1_max: f64,
    ridge_zero_fraction: f64,
    ridge_abs_omega_vol1_le_1em3_fraction: f64,
    ridge_abs_omega_vol1_le_1em2_fraction: f64,
    ridge_abs_omega_vol1_le_1em1_fraction: f64,
    transition_density: f64,
    transition_bidirectional_fraction: f64,
    transition_out_degree_mean: f64,
    transition_out_degree_std: f64,
    transition_out_degree_min: f64,
    transition_out_degree_max: f64,
}

fn stats(values: &[f64]) -> (f64, f64, f64, f64) {
    assert!(!values.is_empty(), "stats requires non-empty slice");
    let mean = values.iter().sum::<f64>() / values.len() as f64;
    let var = values
        .iter()
        .map(|v| {
            let d = v - mean;
            d * d
        })
        .sum::<f64>()
        / values.len() as f64;
    let min = values.iter().copied().fold(f64::INFINITY, f64::min);
    let max = values.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    (mean, var.sqrt(), min, max)
}

fn fraction_at_most(values: &[f64], threshold: f64) -> f64 {
    values.iter().filter(|&&value| value <= threshold).count() as f64 / values.len() as f64
}

fn build_row(poly: &PolytopeInputRow) -> OmegaFeatureRow {
    let polytope = Polytope4D::from_rational_parts(
        poly.dual_vertices_rational.clone(),
        poly.vertices_rational.clone(),
    )
    .unwrap_or_else(|e| panic!("reconstruct {}: {e}", poly.poly_id));
    let polytope_volume = volume(&polytope);
    let omega_scale = polytope_volume.sqrt();
    assert!(
        omega_scale > 0.0,
        "volume-normalization scale must be positive for {}",
        poly.poly_id
    );
    let skeleton = Skeleton::compute(&polytope);
    let duals = polytope.dual_vertices_f64();
    let f = poly.facet_count;

    let mut allpair_abs_omegas = Vec::new();
    let mut allpair_zero_count = 0usize;
    for i in 0..f {
        for j in (i + 1)..f {
            let value = omega0(&duals[i], &duals[j]).abs() * omega_scale;
            if polytope.omega_signs()[(i, j)] == 0 {
                allpair_zero_count += 1;
            }
            allpair_abs_omegas.push(value);
        }
    }

    let ridge_abs_omegas = skeleton
        .ridges
        .iter()
        .map(|ridge| omega0(&duals[ridge.facets[0]], &duals[ridge.facets[1]]).abs() * omega_scale)
        .collect::<Vec<_>>();
    let ridge_zero_count = skeleton
        .ridges
        .iter()
        .filter(|ridge| polytope.omega_signs()[(ridge.facets[0], ridge.facets[1])] == 0)
        .count();

    let transition = build_transition_matrix(&polytope);
    let mut transition_true_count = 0usize;
    let mut adjacent_pair_count = 0usize;
    let mut bidirectional_pair_count = 0usize;
    let mut out_degrees = Vec::new();
    for i in 0..f {
        let mut out = 0usize;
        for j in 0..f {
            if transition[(i, j)] {
                transition_true_count += 1;
                out += 1;
            }
        }
        out_degrees.push(out as f64);
    }
    for i in 0..f {
        for j in (i + 1)..f {
            if polytope.vertex_adjacency()[(i, j)] {
                adjacent_pair_count += 1;
                if transition[(i, j)] && transition[(j, i)] {
                    bidirectional_pair_count += 1;
                }
            }
        }
    }

    let (allpair_abs_omega_vol1_mean, allpair_abs_omega_vol1_std, allpair_abs_omega_vol1_min, allpair_abs_omega_vol1_max) =
        stats(&allpair_abs_omegas);
    let (ridge_abs_omega_vol1_mean, ridge_abs_omega_vol1_std, ridge_abs_omega_vol1_min, ridge_abs_omega_vol1_max) =
        stats(&ridge_abs_omegas);
    let (transition_out_degree_mean, transition_out_degree_std, transition_out_degree_min, transition_out_degree_max) =
        stats(&out_degrees);

    let total_pairs = (f * (f - 1) / 2) as f64;
    let transition_density = transition_true_count as f64 / (f * (f - 1)) as f64;
    let transition_bidirectional_fraction = if adjacent_pair_count > 0 {
        bidirectional_pair_count as f64 / adjacent_pair_count as f64
    } else {
        0.0
    };

    OmegaFeatureRow {
        poly_id: poly.poly_id.clone(),
        facet_count: f,
        allpair_abs_omega_vol1_mean,
        allpair_abs_omega_vol1_std,
        allpair_abs_omega_vol1_min,
        allpair_abs_omega_vol1_max,
        allpair_zero_fraction: allpair_zero_count as f64 / total_pairs,
        ridge_abs_omega_vol1_mean,
        ridge_abs_omega_vol1_std,
        ridge_abs_omega_vol1_min,
        ridge_abs_omega_vol1_max,
        ridge_zero_fraction: ridge_zero_count as f64 / skeleton.ridges.len() as f64,
        ridge_abs_omega_vol1_le_1em3_fraction: fraction_at_most(&ridge_abs_omegas, 1e-3),
        ridge_abs_omega_vol1_le_1em2_fraction: fraction_at_most(&ridge_abs_omegas, 1e-2),
        ridge_abs_omega_vol1_le_1em1_fraction: fraction_at_most(&ridge_abs_omegas, 1e-1),
        transition_density,
        transition_bidirectional_fraction,
        transition_out_degree_mean,
        transition_out_degree_std,
        transition_out_degree_min,
        transition_out_degree_max,
    }
}

fn main() {
    let args = parse_standard_feature_args("omega");
    let polytopes = read_jsonl::<PolytopeInputRow>(&args.normalized_dir.join("polytopes.jsonl"));
    let mut rows = polytopes.iter().map(build_row).collect::<Vec<_>>();
    rows.sort_by(|a, b| a.poly_id.cmp(&b.poly_id));
    write_jsonl(&args.out, &rows);
    println!("Wrote {} omega rows", rows.len());
    println!("Output path: {}", args.out.display());
}

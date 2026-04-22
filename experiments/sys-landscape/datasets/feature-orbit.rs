//! Compute a bounded sigma-local orbit feature table keyed by `poly_id`.
//!
//! Goal: enrich the hostile-landscape normalized dataset with cheap
//! orbit-sensitive summaries derived from cached `best_sigma` permutations plus
//! exact polytope geometry and bounded best-orbit KKT scalars. When older
//! cache rows lack the scalar payload, this binary falls back to one
//! best-sigma KKT solve instead of a full capacity rerun.
//! Input Artifacts:
//!   - experiments/sys-landscape/normalized-dataset outputs under `--normalized-dir`
//!     (`polytopes.jsonl` required)
//!   - experiments/sys-landscape/cache.jsonl
//!   - experiments/combinatorial-cells/polytopes.jsonl
//!   - experiments/sys-landscape/raw/continuation-cache.jsonl (or `--continuation-cache`)
//! Output Artifacts: None by default (writes to an untracked temp file unless `--out` is set)

use exp_sys_landscape::{package_root, raw_dataset_cache_path};
use exp_sys_landscape::features::{
    default_feature_output_path, deserialize_vec4_rational, read_jsonl, write_jsonl,
};
use num_rational::BigRational;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use symplectic::algorithms::facet_adjacency::build_transition_matrix;
use symplectic::algorithms::solve_orbit_sigma;
use symplectic::database::{load_many, DualVerticesKey, OrbitScalars, PolytopeRecord};
use symplectic::geom::polytope::Polytope4D;
use symplectic::geom::symplectic_form::omega0;
use symplectic::{OrbitAdmissibility, OrbitSolveBackend};

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
struct OrbitFeatureRow {
    poly_id: String,
    facet_count: usize,
    orbit_sigma_available: f64,
    orbit_sigma_count: f64,
    orbit_sigma_gap_cutoff: f64,
    orbit_sigma_len: f64,
    orbit_sigma_fraction: f64,
    orbit_selected_norm_mean: f64,
    orbit_selected_norm_std: f64,
    orbit_selected_norm_min: f64,
    orbit_selected_norm_max: f64,
    orbit_cycle_abs_omega_mean: f64,
    orbit_cycle_abs_omega_std: f64,
    orbit_cycle_abs_omega_min: f64,
    orbit_cycle_abs_omega_max: f64,
    orbit_cycle_abs_omega_le_1e3_fraction: f64,
    orbit_cycle_abs_omega_le_1e2_fraction: f64,
    orbit_cycle_abs_omega_le_1e1_fraction: f64,
    orbit_cycle_zero_fraction: f64,
    orbit_cycle_transition_fraction: f64,
    orbit_cycle_bidirectional_fraction: f64,
    orbit_cycle_adjacent_fraction: f64,
    orbit_selected_out_degree_mean: f64,
    orbit_selected_out_degree_std: f64,
    orbit_selected_out_degree_min: f64,
    orbit_selected_out_degree_max: f64,
    orbit_kkt_available: f64,
    orbit_search_scalar_available: f64,
    orbit_result_iterations_log1p: f64,
    orbit_result_returned_orbit_count: f64,
    orbit_best_beta_margin: f64,
    orbit_best_q_error_bound: f64,
    orbit_best_has_mu: f64,
    orbit_best_has_xi: f64,
    orbit_best_is_admissible_exact: f64,
    orbit_best_is_indeterminate_f64: f64,
}

fn parse_args() -> (PathBuf, PathBuf, PathBuf) {
    let args: Vec<String> = std::env::args().collect();
    let mut normalized_dir: Option<PathBuf> = None;
    let mut out: Option<PathBuf> = None;
    let mut continuation_cache: Option<PathBuf> = None;
    let mut i = 1usize;
    while i < args.len() {
        match args[i].as_str() {
            "--normalized-dir" => {
                let value = args.get(i + 1).expect("--normalized-dir requires a value");
                normalized_dir = Some(PathBuf::from(value));
                i += 2;
            }
            "--out" => {
                let value = args.get(i + 1).expect("--out requires a value");
                out = Some(PathBuf::from(value));
                i += 2;
            }
            "--continuation-cache" => {
                let value = args
                    .get(i + 1)
                    .expect("--continuation-cache requires a value");
                continuation_cache = Some(PathBuf::from(value));
                i += 2;
            }
            other => panic!("unknown argument: {other}"),
        }
    }
    let normalized_dir = normalized_dir.expect("--normalized-dir is required");
    let out = out.unwrap_or_else(|| default_feature_output_path("orbit"));
    let continuation_cache =
        continuation_cache.unwrap_or_else(|| raw_dataset_cache_path("continuation"));
    (normalized_dir, out, continuation_cache)
}

fn stats_or_zero(values: &[f64]) -> (f64, f64, f64, f64) {
    if values.is_empty() {
        return (0.0, 0.0, 0.0, 0.0);
    }
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
    if values.is_empty() {
        return 0.0;
    }
    values.iter().filter(|&&value| value <= threshold).count() as f64 / values.len() as f64
}

fn empty_row(poly_id: &str, facet_count: usize) -> OrbitFeatureRow {
    OrbitFeatureRow {
        poly_id: poly_id.to_string(),
        facet_count,
        orbit_sigma_available: 0.0,
        orbit_sigma_count: 0.0,
        orbit_sigma_gap_cutoff: 0.0,
        orbit_sigma_len: 0.0,
        orbit_sigma_fraction: 0.0,
        orbit_selected_norm_mean: 0.0,
        orbit_selected_norm_std: 0.0,
        orbit_selected_norm_min: 0.0,
        orbit_selected_norm_max: 0.0,
        orbit_cycle_abs_omega_mean: 0.0,
        orbit_cycle_abs_omega_std: 0.0,
        orbit_cycle_abs_omega_min: 0.0,
        orbit_cycle_abs_omega_max: 0.0,
        orbit_cycle_abs_omega_le_1e3_fraction: 0.0,
        orbit_cycle_abs_omega_le_1e2_fraction: 0.0,
        orbit_cycle_abs_omega_le_1e1_fraction: 0.0,
        orbit_cycle_zero_fraction: 0.0,
        orbit_cycle_transition_fraction: 0.0,
        orbit_cycle_bidirectional_fraction: 0.0,
        orbit_cycle_adjacent_fraction: 0.0,
        orbit_selected_out_degree_mean: 0.0,
        orbit_selected_out_degree_std: 0.0,
        orbit_selected_out_degree_min: 0.0,
        orbit_selected_out_degree_max: 0.0,
        orbit_kkt_available: 0.0,
        orbit_search_scalar_available: 0.0,
        orbit_result_iterations_log1p: 0.0,
        orbit_result_returned_orbit_count: 0.0,
        orbit_best_beta_margin: 0.0,
        orbit_best_q_error_bound: 0.0,
        orbit_best_has_mu: 0.0,
        orbit_best_has_xi: 0.0,
        orbit_best_is_admissible_exact: 0.0,
        orbit_best_is_indeterminate_f64: 0.0,
    }
}

fn fallback_orbit_scalars(polytope: &Polytope4D, record: &PolytopeRecord) -> Option<OrbitScalars> {
    let best_sigma = record.sigmas.as_ref()?.first()?;
    let orbit =
        solve_orbit_sigma(polytope, &best_sigma.perm, OrbitSolveBackend::SaddlePoint).ok()?;
    Some(OrbitScalars {
        iterations: 0,
        returned_orbit_count: 0,
        best_beta_margin: orbit.beta_margin,
        best_q_error_bound: orbit.q_error_bound,
        best_has_mu: orbit.mu.is_some(),
        best_has_xi: orbit.xi.is_some(),
        best_is_admissible_exact: matches!(
            orbit.admissibility,
            OrbitAdmissibility::AdmissibleExact
        ),
        best_is_indeterminate_f64: matches!(
            orbit.admissibility,
            OrbitAdmissibility::IndeterminateF64
        ),
    })
}

fn build_cache_index(
    package_root: &Path,
    continuation_cache: &Path,
) -> HashMap<DualVerticesKey, PolytopeRecord> {
    let repo_root = package_root
        .parent()
        .and_then(Path::parent)
        .and_then(Path::parent)
        .expect("package root should be experiments/sys-landscape");
    let paths = [
        package_root.join("cache.jsonl"),
        repo_root.join("experiments/combinatorial-cells/polytopes.jsonl"),
        continuation_cache.to_path_buf(),
    ];
    let refs = paths.iter().map(PathBuf::as_path).collect::<Vec<_>>();
    load_many(&refs).unwrap_or_else(|e| panic!("load orbit caches: {e}"))
}

fn build_row(
    poly: &PolytopeInputRow,
    cache: &HashMap<DualVerticesKey, PolytopeRecord>,
) -> OrbitFeatureRow {
    let dual_vertices = poly.dual_vertices_rational.clone();
    let Some(record) = cache.get(&dual_vertices) else {
        return empty_row(&poly.poly_id, poly.facet_count);
    };
    let Some(sigmas) = record.sigmas.as_ref() else {
        return empty_row(&poly.poly_id, poly.facet_count);
    };
    let Some(best_sigma) = sigmas.first() else {
        return empty_row(&poly.poly_id, poly.facet_count);
    };

    let polytope =
        Polytope4D::from_rational_parts(dual_vertices, poly.vertices_rational.clone())
            .unwrap_or_else(|e| panic!("reconstruct {}: {e}", poly.poly_id));
    let duals = polytope.dual_vertices_f64();
    let transition = build_transition_matrix(&polytope);
    let perm = &best_sigma.perm;

    let selected_norms = perm
        .iter()
        .map(|&facet| duals[facet].norm())
        .collect::<Vec<_>>();
    let selected_out_degrees = perm
        .iter()
        .map(|&facet| {
            (0..poly.facet_count)
                .filter(|&other| transition[(facet, other)])
                .count() as f64
        })
        .collect::<Vec<_>>();

    let mut cycle_abs_omegas = Vec::new();
    let mut cycle_zero_count = 0usize;
    let mut cycle_transition_count = 0usize;
    let mut cycle_bidirectional_count = 0usize;
    let mut cycle_adjacent_count = 0usize;
    if perm.len() >= 2 {
        for idx in 0..perm.len() {
            let i = perm[idx];
            let j = perm[(idx + 1) % perm.len()];
            let abs_omega = omega0(&duals[i], &duals[j]).abs();
            cycle_abs_omegas.push(abs_omega);
            if polytope.omega_signs()[(i, j)] == 0 {
                cycle_zero_count += 1;
            }
            if transition[(i, j)] {
                cycle_transition_count += 1;
            }
            if transition[(i, j)] && transition[(j, i)] {
                cycle_bidirectional_count += 1;
            }
            if polytope.vertex_adjacency()[(i, j)] {
                cycle_adjacent_count += 1;
            }
        }
    }

    let (
        orbit_selected_norm_mean,
        orbit_selected_norm_std,
        orbit_selected_norm_min,
        orbit_selected_norm_max,
    ) = stats_or_zero(&selected_norms);
    let (
        orbit_cycle_abs_omega_mean,
        orbit_cycle_abs_omega_std,
        orbit_cycle_abs_omega_min,
        orbit_cycle_abs_omega_max,
    ) = stats_or_zero(&cycle_abs_omegas);
    let (
        orbit_selected_out_degree_mean,
        orbit_selected_out_degree_std,
        orbit_selected_out_degree_min,
        orbit_selected_out_degree_max,
    ) = stats_or_zero(&selected_out_degrees);
    let orbit_scalars = record
        .orbit_scalars
        .clone()
        .or_else(|| fallback_orbit_scalars(&polytope, record));
    let orbit_search_scalar_available = orbit_scalars
        .as_ref()
        .is_some_and(|scalars| scalars.returned_orbit_count > 0 || scalars.iterations > 0);

    let cycle_len = cycle_abs_omegas.len() as f64;
    OrbitFeatureRow {
        poly_id: poly.poly_id.clone(),
        facet_count: poly.facet_count,
        orbit_sigma_available: 1.0,
        orbit_sigma_count: sigmas.len() as f64,
        orbit_sigma_gap_cutoff: record.sigma_gap_cutoff.unwrap_or(0.0),
        orbit_sigma_len: perm.len() as f64,
        orbit_sigma_fraction: perm.len() as f64 / poly.facet_count as f64,
        orbit_selected_norm_mean,
        orbit_selected_norm_std,
        orbit_selected_norm_min,
        orbit_selected_norm_max,
        orbit_cycle_abs_omega_mean,
        orbit_cycle_abs_omega_std,
        orbit_cycle_abs_omega_min,
        orbit_cycle_abs_omega_max,
        orbit_cycle_abs_omega_le_1e3_fraction: fraction_at_most(&cycle_abs_omegas, 1e-3),
        orbit_cycle_abs_omega_le_1e2_fraction: fraction_at_most(&cycle_abs_omegas, 1e-2),
        orbit_cycle_abs_omega_le_1e1_fraction: fraction_at_most(&cycle_abs_omegas, 1e-1),
        orbit_cycle_zero_fraction: if cycle_len > 0.0 {
            cycle_zero_count as f64 / cycle_len
        } else {
            0.0
        },
        orbit_cycle_transition_fraction: if cycle_len > 0.0 {
            cycle_transition_count as f64 / cycle_len
        } else {
            0.0
        },
        orbit_cycle_bidirectional_fraction: if cycle_len > 0.0 {
            cycle_bidirectional_count as f64 / cycle_len
        } else {
            0.0
        },
        orbit_cycle_adjacent_fraction: if cycle_len > 0.0 {
            cycle_adjacent_count as f64 / cycle_len
        } else {
            0.0
        },
        orbit_selected_out_degree_mean,
        orbit_selected_out_degree_std,
        orbit_selected_out_degree_min,
        orbit_selected_out_degree_max,
        orbit_kkt_available: orbit_scalars.is_some() as u8 as f64,
        orbit_search_scalar_available: orbit_search_scalar_available as u8 as f64,
        orbit_result_iterations_log1p: orbit_scalars
            .as_ref()
            .map(|scalars| (scalars.iterations as f64).ln_1p())
            .unwrap_or(0.0),
        orbit_result_returned_orbit_count: orbit_scalars
            .as_ref()
            .map(|scalars| scalars.returned_orbit_count as f64)
            .unwrap_or(0.0),
        orbit_best_beta_margin: orbit_scalars
            .as_ref()
            .map(|scalars| scalars.best_beta_margin)
            .unwrap_or(0.0),
        orbit_best_q_error_bound: orbit_scalars
            .as_ref()
            .map(|scalars| scalars.best_q_error_bound)
            .unwrap_or(0.0),
        orbit_best_has_mu: orbit_scalars
            .as_ref()
            .map(|scalars| scalars.best_has_mu as u8 as f64)
            .unwrap_or(0.0),
        orbit_best_has_xi: orbit_scalars
            .as_ref()
            .map(|scalars| scalars.best_has_xi as u8 as f64)
            .unwrap_or(0.0),
        orbit_best_is_admissible_exact: orbit_scalars
            .as_ref()
            .map(|scalars| scalars.best_is_admissible_exact as u8 as f64)
            .unwrap_or(0.0),
        orbit_best_is_indeterminate_f64: orbit_scalars
            .as_ref()
            .map(|scalars| scalars.best_is_indeterminate_f64 as u8 as f64)
            .unwrap_or(0.0),
    }
}

fn main() {
    let (normalized_dir, out, continuation_cache) = parse_args();
    let package_root = package_root();
    let cache = build_cache_index(&package_root, &continuation_cache);
    let polytopes = read_jsonl::<PolytopeInputRow>(&normalized_dir.join("polytopes.jsonl"));
    let mut rows = polytopes
        .iter()
        .map(|poly| build_row(poly, &cache))
        .collect::<Vec<_>>();
    rows.sort_by(|a, b| a.poly_id.cmp(&b.poly_id));
    let available = rows
        .iter()
        .filter(|row| row.orbit_sigma_available > 0.5)
        .count();
    write_jsonl(&out, &rows);
    println!(
        "Wrote {} orbit rows ({} with cached sigma payload)",
        rows.len(),
        available
    );
    println!("Output path: {}", out.display());
}

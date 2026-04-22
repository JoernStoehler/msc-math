//! Combined polytope-level feature assembly for the datascience dataset stage.
//!
//! This module owns the `load -> enrich -> save` core for polytope-level
//! features. The executable shell should stay thin and only parse paths, load
//! rows, map `enrich_row`, and write the result.

use crate::features::{deserialize_vec4_rational, read_jsonl};
use num_rational::BigRational;
use num_traits::ToPrimitive;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use symplectic::algorithms::facet_adjacency::build_transition_matrix;
use symplectic::algorithms::solve_orbit_sigma;
use symplectic::database::{load_many, DualVerticesKey, OrbitScalars, PolytopeRecord};
use symplectic::geom::facet_volume::facet_volume_3d;
use symplectic::geom::polytope::Polytope4D;
use symplectic::geom::skeleton::Skeleton;
use symplectic::geom::symplectic_form::omega0;
use symplectic::geom::volume::volume;
use symplectic::{OrbitAdmissibility, OrbitSolveBackend};

#[derive(Debug, Deserialize)]
struct NormalizedPolytopeRow {
    poly_id: String,
    #[serde(deserialize_with = "deserialize_vec4_rational")]
    dual_vertices_rational: Vec<[BigRational; 4]>,
    #[serde(deserialize_with = "deserialize_vec4_rational")]
    vertices_rational: Vec<[BigRational; 4]>,
    facet_count: usize,
}

#[derive(Debug, Deserialize)]
struct CapacityResultRow {
    poly_id: String,
    capacity: f64,
    volume: f64,
    sys: f64,
    #[serde(default)]
    iterations: Option<u64>,
    search_result_source: String,
}

#[derive(Debug, Clone)]
pub struct PolytopeFeatureInputRow {
    pub poly_id: String,
    pub dual_vertices_rational: Vec<[BigRational; 4]>,
    pub vertices_rational: Vec<[BigRational; 4]>,
    pub facet_count: usize,
    pub capacity: f64,
    pub volume: f64,
    pub sys: f64,
    pub capacity_iterations: Option<u64>,
    pub capacity_source: String,
}

#[derive(Debug, Serialize)]
pub struct PolytopeFeatureRow {
    pub poly_id: String,
    pub facet_count: usize,
    pub capacity: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_iterations: Option<u64>,
    pub capacity_source: String,
    pub volume: f64,
    pub sys: f64,
    pub dual_vertex_count: usize,
    pub dual_vertices_f64: Vec<[f64; 4]>,
    pub dual_vertices_flat_f64: Vec<f64>,
    pub vertex_count: usize,
    pub edge_count: usize,
    pub ridge_count: usize,
    pub is_simple: bool,
    pub simple_vertex_fraction: f64,
    pub edge_density: f64,
    pub vertex_incident_facets_mean: f64,
    pub vertex_incident_facets_std: f64,
    pub vertex_incident_facets_min: f64,
    pub vertex_incident_facets_max: f64,
    pub vertex_degree_mean: f64,
    pub vertex_degree_std: f64,
    pub vertex_degree_min: f64,
    pub vertex_degree_max: f64,
    pub ridge_size_mean: f64,
    pub ridge_size_std: f64,
    pub ridge_size_min: f64,
    pub ridge_size_max: f64,
    pub facet_vertex_count_mean: f64,
    pub facet_vertex_count_std: f64,
    pub facet_vertex_count_min: f64,
    pub facet_vertex_count_max: f64,
    pub facet_neighbor_count_mean: f64,
    pub facet_neighbor_count_std: f64,
    pub facet_neighbor_count_min: f64,
    pub facet_neighbor_count_max: f64,
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
    pub allpair_abs_omega_vol1_mean: f64,
    pub allpair_abs_omega_vol1_std: f64,
    pub allpair_abs_omega_vol1_min: f64,
    pub allpair_abs_omega_vol1_max: f64,
    pub allpair_zero_fraction: f64,
    pub ridge_abs_omega_vol1_mean: f64,
    pub ridge_abs_omega_vol1_std: f64,
    pub ridge_abs_omega_vol1_min: f64,
    pub ridge_abs_omega_vol1_max: f64,
    pub ridge_zero_fraction: f64,
    pub ridge_abs_omega_vol1_le_1em3_fraction: f64,
    pub ridge_abs_omega_vol1_le_1em2_fraction: f64,
    pub ridge_abs_omega_vol1_le_1em1_fraction: f64,
    pub transition_density: f64,
    pub transition_bidirectional_fraction: f64,
    pub transition_out_degree_mean: f64,
    pub transition_out_degree_std: f64,
    pub transition_out_degree_min: f64,
    pub transition_out_degree_max: f64,
    pub orbit_sigma_available: f64,
    pub orbit_sigma_count: f64,
    pub orbit_sigma_gap_cutoff: f64,
    pub orbit_sigma_len: f64,
    pub orbit_sigma_fraction: f64,
    pub orbit_selected_norm_mean: f64,
    pub orbit_selected_norm_std: f64,
    pub orbit_selected_norm_min: f64,
    pub orbit_selected_norm_max: f64,
    pub orbit_cycle_abs_omega_mean: f64,
    pub orbit_cycle_abs_omega_std: f64,
    pub orbit_cycle_abs_omega_min: f64,
    pub orbit_cycle_abs_omega_max: f64,
    pub orbit_cycle_abs_omega_le_1e3_fraction: f64,
    pub orbit_cycle_abs_omega_le_1e2_fraction: f64,
    pub orbit_cycle_abs_omega_le_1e1_fraction: f64,
    pub orbit_cycle_zero_fraction: f64,
    pub orbit_cycle_transition_fraction: f64,
    pub orbit_cycle_bidirectional_fraction: f64,
    pub orbit_cycle_adjacent_fraction: f64,
    pub orbit_selected_out_degree_mean: f64,
    pub orbit_selected_out_degree_std: f64,
    pub orbit_selected_out_degree_min: f64,
    pub orbit_selected_out_degree_max: f64,
    pub orbit_kkt_available: f64,
    pub orbit_search_scalar_available: f64,
    pub orbit_result_iterations_log1p: f64,
    pub orbit_result_returned_orbit_count: f64,
    pub orbit_best_beta_margin: f64,
    pub orbit_best_q_error_bound: f64,
    pub orbit_best_has_mu: f64,
    pub orbit_best_has_xi: f64,
    pub orbit_best_is_admissible_exact: f64,
    pub orbit_best_is_indeterminate_f64: f64,
}

pub type OrbitCacheIndex = HashMap<DualVerticesKey, PolytopeRecord>;

pub fn load_inputs(normalized_dir: &Path) -> Vec<PolytopeFeatureInputRow> {
    let polytopes = read_jsonl::<NormalizedPolytopeRow>(&normalized_dir.join("polytopes.jsonl"));
    let mut capacities = read_jsonl::<CapacityResultRow>(&normalized_dir.join("capacity_results.jsonl"))
        .into_iter()
        .map(|row| (row.poly_id.clone(), row))
        .collect::<HashMap<_, _>>();
    let mut rows = polytopes
        .into_iter()
        .map(|poly| {
            let cap = capacities
                .remove(&poly.poly_id)
                .unwrap_or_else(|| panic!("missing capacity_results row for {}", poly.poly_id));
            PolytopeFeatureInputRow {
                poly_id: poly.poly_id,
                dual_vertices_rational: poly.dual_vertices_rational,
                vertices_rational: poly.vertices_rational,
                facet_count: poly.facet_count,
                capacity: cap.capacity,
                volume: cap.volume,
                sys: cap.sys,
                capacity_iterations: cap.iterations,
                capacity_source: cap.search_result_source,
            }
        })
        .collect::<Vec<_>>();
    rows.sort_by(|a, b| a.poly_id.cmp(&b.poly_id));
    rows
}

pub fn build_cache_index(
    package_root: &Path,
    continuation_cache: &Path,
) -> OrbitCacheIndex {
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

fn rational_to_f64(value: &BigRational) -> f64 {
    value
        .to_f64()
        .unwrap_or_else(|| panic!("cannot convert rational {value} to f64"))
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

pub fn enrich_row(
    row: &PolytopeFeatureInputRow,
    cache: &OrbitCacheIndex,
) -> PolytopeFeatureRow {
    let polytope = Polytope4D::from_rational_parts(
        row.dual_vertices_rational.clone(),
        row.vertices_rational.clone(),
    )
    .unwrap_or_else(|e| panic!("reconstruct {}: {e}", row.poly_id));
    let polytope_volume = volume(&polytope);
    let linear_scale = polytope_volume.powf(0.25);
    let facet_scale = polytope_volume.powf(0.75);
    let omega_scale = polytope_volume.sqrt();
    let volume_scale = polytope_volume.sqrt();
    let skeleton = Skeleton::compute(&polytope);
    let vertices = polytope.vertices_f64();
    let dual_vertices_f64 = row
        .dual_vertices_rational
        .iter()
        .map(|vertex| std::array::from_fn(|i| rational_to_f64(&vertex[i])))
        .collect::<Vec<_>>();
    let dual_vertices_flat_f64 = dual_vertices_f64
        .iter()
        .flat_map(|vertex| vertex.iter().copied())
        .collect::<Vec<_>>();
    let duals = polytope.dual_vertices_f64();
    let facet_count = row.facet_count;

    let edge_lengths = skeleton
        .edges
        .iter()
        .map(|edge| (vertices[edge[0]] - vertices[edge[1]]).norm() / linear_scale)
        .collect::<Vec<_>>();
    let facet_volumes = (0..facet_count)
        .map(|facet| facet_volume_3d(&polytope, facet) / facet_scale)
        .collect::<Vec<_>>();
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

    let vertex_count = polytope.vertices().len();
    let edge_count = skeleton.edges.len();
    let ridge_count = skeleton.ridges.len();
    let vertex_incident_facets = skeleton
        .vertex_facets
        .iter()
        .map(|facets| facets.len() as f64)
        .collect::<Vec<_>>();
    let simple_vertices = vertex_incident_facets
        .iter()
        .filter(|&&count| (count - 4.0).abs() < f64::EPSILON)
        .count();
    let simple_vertex_fraction = if vertex_count > 0 {
        simple_vertices as f64 / vertex_count as f64
    } else {
        0.0
    };
    let is_simple = simple_vertices == vertex_count;
    let mut vertex_degrees = vec![0usize; vertex_count];
    for edge in &skeleton.edges {
        vertex_degrees[edge[0]] += 1;
        vertex_degrees[edge[1]] += 1;
    }
    let vertex_degrees = vertex_degrees
        .into_iter()
        .map(|count| count as f64)
        .collect::<Vec<_>>();
    let ridge_sizes = skeleton
        .ridges
        .iter()
        .map(|ridge| ridge.vertices.len() as f64)
        .collect::<Vec<_>>();
    let mut facet_vertex_counts = vec![0usize; facet_count];
    for facets in &skeleton.vertex_facets {
        for &facet in facets {
            facet_vertex_counts[facet] += 1;
        }
    }
    let facet_vertex_counts = facet_vertex_counts
        .into_iter()
        .map(|count| count as f64)
        .collect::<Vec<_>>();
    let facet_neighbor_counts = (0..facet_count)
        .map(|facet| {
            (0..facet_count)
                .filter(|&other| polytope.vertex_adjacency()[(facet, other)])
                .count() as f64
        })
        .collect::<Vec<_>>();
    let edge_density = if vertex_count >= 2 {
        (2.0 * edge_count as f64) / ((vertex_count * (vertex_count - 1)) as f64)
    } else {
        0.0
    };

    let mut allpair_abs_omegas = Vec::new();
    let mut allpair_zero_count = 0usize;
    for i in 0..facet_count {
        for j in (i + 1)..facet_count {
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
    for i in 0..facet_count {
        let mut out = 0usize;
        for j in 0..facet_count {
            if transition[(i, j)] {
                transition_true_count += 1;
                out += 1;
            }
        }
        out_degrees.push(out as f64);
    }
    for i in 0..facet_count {
        for j in (i + 1)..facet_count {
            if polytope.vertex_adjacency()[(i, j)] {
                adjacent_pair_count += 1;
                if transition[(i, j)] && transition[(j, i)] {
                    bidirectional_pair_count += 1;
                }
            }
        }
    }

    let (vertex_incident_facets_mean, vertex_incident_facets_std, vertex_incident_facets_min, vertex_incident_facets_max) =
        stats_or_zero(&vertex_incident_facets);
    let (vertex_degree_mean, vertex_degree_std, vertex_degree_min, vertex_degree_max) =
        stats_or_zero(&vertex_degrees);
    let (ridge_size_mean, ridge_size_std, ridge_size_min, ridge_size_max) =
        stats_or_zero(&ridge_sizes);
    let (facet_vertex_count_mean, facet_vertex_count_std, facet_vertex_count_min, facet_vertex_count_max) =
        stats_or_zero(&facet_vertex_counts);
    let (facet_neighbor_count_mean, facet_neighbor_count_std, facet_neighbor_count_min, facet_neighbor_count_max) =
        stats_or_zero(&facet_neighbor_counts);
    let (edge_length_vol1_mean, edge_length_vol1_std, edge_length_vol1_min, edge_length_vol1_max) =
        stats_or_zero(&edge_lengths);
    let (facet_volume_vol1_mean, facet_volume_vol1_std, facet_volume_vol1_min, facet_volume_vol1_max) =
        stats_or_zero(&facet_volumes);
    let (ridge_symp_area_volnorm_mean, ridge_symp_area_volnorm_std, ridge_symp_area_volnorm_min, ridge_symp_area_volnorm_max) =
        stats_or_zero(&ridge_symp_areas);
    let (allpair_abs_omega_vol1_mean, allpair_abs_omega_vol1_std, allpair_abs_omega_vol1_min, allpair_abs_omega_vol1_max) =
        stats_or_zero(&allpair_abs_omegas);
    let (ridge_abs_omega_vol1_mean, ridge_abs_omega_vol1_std, ridge_abs_omega_vol1_min, ridge_abs_omega_vol1_max) =
        stats_or_zero(&ridge_abs_omegas);
    let (transition_out_degree_mean, transition_out_degree_std, transition_out_degree_min, transition_out_degree_max) =
        stats_or_zero(&out_degrees);

    let dual_key = row.dual_vertices_rational.clone();
    let orbit_metrics = if let Some(record) = cache.get(&dual_key) {
        if let Some(sigmas) = record.sigmas.as_ref() {
            if let Some(best_sigma) = sigmas.first() {
                let perm = &best_sigma.perm;
                let selected_norms = perm
                    .iter()
                    .map(|&facet| duals[facet].norm())
                    .collect::<Vec<_>>();
                let selected_out_degrees = perm
                    .iter()
                    .map(|&facet| {
                        (0..facet_count)
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
                let (orbit_selected_norm_mean, orbit_selected_norm_std, orbit_selected_norm_min, orbit_selected_norm_max) =
                    stats_or_zero(&selected_norms);
                let (orbit_cycle_abs_omega_mean, orbit_cycle_abs_omega_std, orbit_cycle_abs_omega_min, orbit_cycle_abs_omega_max) =
                    stats_or_zero(&cycle_abs_omegas);
                let (orbit_selected_out_degree_mean, orbit_selected_out_degree_std, orbit_selected_out_degree_min, orbit_selected_out_degree_max) =
                    stats_or_zero(&selected_out_degrees);
                let orbit_scalars = record
                    .orbit_scalars
                    .clone()
                    .or_else(|| fallback_orbit_scalars(&polytope, record));
                let orbit_search_scalar_available = orbit_scalars
                    .as_ref()
                    .is_some_and(|scalars| scalars.returned_orbit_count > 0 || scalars.iterations > 0);
                let cycle_len = cycle_abs_omegas.len() as f64;
                (
                    1.0,
                    sigmas.len() as f64,
                    record.sigma_gap_cutoff.unwrap_or(0.0),
                    perm.len() as f64,
                    perm.len() as f64 / facet_count as f64,
                    orbit_selected_norm_mean,
                    orbit_selected_norm_std,
                    orbit_selected_norm_min,
                    orbit_selected_norm_max,
                    orbit_cycle_abs_omega_mean,
                    orbit_cycle_abs_omega_std,
                    orbit_cycle_abs_omega_min,
                    orbit_cycle_abs_omega_max,
                    fraction_at_most(&cycle_abs_omegas, 1e-3),
                    fraction_at_most(&cycle_abs_omegas, 1e-2),
                    fraction_at_most(&cycle_abs_omegas, 1e-1),
                    if cycle_len > 0.0 { cycle_zero_count as f64 / cycle_len } else { 0.0 },
                    if cycle_len > 0.0 { cycle_transition_count as f64 / cycle_len } else { 0.0 },
                    if cycle_len > 0.0 { cycle_bidirectional_count as f64 / cycle_len } else { 0.0 },
                    if cycle_len > 0.0 { cycle_adjacent_count as f64 / cycle_len } else { 0.0 },
                    orbit_selected_out_degree_mean,
                    orbit_selected_out_degree_std,
                    orbit_selected_out_degree_min,
                    orbit_selected_out_degree_max,
                    if orbit_scalars.is_some() { 1.0 } else { 0.0 },
                    if orbit_search_scalar_available { 1.0 } else { 0.0 },
                    orbit_scalars
                        .as_ref()
                        .map(|scalars| (scalars.iterations as f64).ln_1p())
                        .unwrap_or(0.0),
                    orbit_scalars
                        .as_ref()
                        .map(|scalars| scalars.returned_orbit_count as f64)
                        .unwrap_or(0.0),
                    orbit_scalars
                        .as_ref()
                        .map(|scalars| scalars.best_beta_margin)
                        .unwrap_or(0.0),
                    orbit_scalars
                        .as_ref()
                        .map(|scalars| scalars.best_q_error_bound)
                        .unwrap_or(0.0),
                    orbit_scalars
                        .as_ref()
                        .map(|scalars| if scalars.best_has_mu { 1.0 } else { 0.0 })
                        .unwrap_or(0.0),
                    orbit_scalars
                        .as_ref()
                        .map(|scalars| if scalars.best_has_xi { 1.0 } else { 0.0 })
                        .unwrap_or(0.0),
                    orbit_scalars
                        .as_ref()
                        .map(|scalars| if scalars.best_is_admissible_exact { 1.0 } else { 0.0 })
                        .unwrap_or(0.0),
                    orbit_scalars
                        .as_ref()
                        .map(|scalars| if scalars.best_is_indeterminate_f64 { 1.0 } else { 0.0 })
                        .unwrap_or(0.0),
                )
            } else {
                zero_orbit_metrics(facet_count)
            }
        } else {
            zero_orbit_metrics(facet_count)
        }
    } else {
        zero_orbit_metrics(facet_count)
    };

    let total_pairs = (facet_count * (facet_count - 1) / 2) as f64;
    let transition_density = if facet_count >= 2 {
        transition_true_count as f64 / (facet_count * (facet_count - 1)) as f64
    } else {
        0.0
    };
    let transition_bidirectional_fraction = if adjacent_pair_count > 0 {
        bidirectional_pair_count as f64 / adjacent_pair_count as f64
    } else {
        0.0
    };

    PolytopeFeatureRow {
        poly_id: row.poly_id.clone(),
        facet_count,
        capacity: row.capacity,
        capacity_iterations: row.capacity_iterations,
        capacity_source: row.capacity_source.clone(),
        volume: row.volume,
        sys: row.sys,
        dual_vertex_count: dual_vertices_f64.len(),
        dual_vertices_f64,
        dual_vertices_flat_f64,
        vertex_count,
        edge_count,
        ridge_count,
        is_simple,
        simple_vertex_fraction,
        edge_density,
        vertex_incident_facets_mean,
        vertex_incident_facets_std,
        vertex_incident_facets_min,
        vertex_incident_facets_max,
        vertex_degree_mean,
        vertex_degree_std,
        vertex_degree_min,
        vertex_degree_max,
        ridge_size_mean,
        ridge_size_std,
        ridge_size_min,
        ridge_size_max,
        facet_vertex_count_mean,
        facet_vertex_count_std,
        facet_vertex_count_min,
        facet_vertex_count_max,
        facet_neighbor_count_mean,
        facet_neighbor_count_std,
        facet_neighbor_count_min,
        facet_neighbor_count_max,
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
        allpair_abs_omega_vol1_mean,
        allpair_abs_omega_vol1_std,
        allpair_abs_omega_vol1_min,
        allpair_abs_omega_vol1_max,
        allpair_zero_fraction: if total_pairs > 0.0 {
            allpair_zero_count as f64 / total_pairs
        } else {
            0.0
        },
        ridge_abs_omega_vol1_mean,
        ridge_abs_omega_vol1_std,
        ridge_abs_omega_vol1_min,
        ridge_abs_omega_vol1_max,
        ridge_zero_fraction: if ridge_count > 0 {
            ridge_zero_count as f64 / ridge_count as f64
        } else {
            0.0
        },
        ridge_abs_omega_vol1_le_1em3_fraction: fraction_at_most(&ridge_abs_omegas, 1e-3),
        ridge_abs_omega_vol1_le_1em2_fraction: fraction_at_most(&ridge_abs_omegas, 1e-2),
        ridge_abs_omega_vol1_le_1em1_fraction: fraction_at_most(&ridge_abs_omegas, 1e-1),
        transition_density,
        transition_bidirectional_fraction,
        transition_out_degree_mean,
        transition_out_degree_std,
        transition_out_degree_min,
        transition_out_degree_max,
        orbit_sigma_available: orbit_metrics.0,
        orbit_sigma_count: orbit_metrics.1,
        orbit_sigma_gap_cutoff: orbit_metrics.2,
        orbit_sigma_len: orbit_metrics.3,
        orbit_sigma_fraction: orbit_metrics.4,
        orbit_selected_norm_mean: orbit_metrics.5,
        orbit_selected_norm_std: orbit_metrics.6,
        orbit_selected_norm_min: orbit_metrics.7,
        orbit_selected_norm_max: orbit_metrics.8,
        orbit_cycle_abs_omega_mean: orbit_metrics.9,
        orbit_cycle_abs_omega_std: orbit_metrics.10,
        orbit_cycle_abs_omega_min: orbit_metrics.11,
        orbit_cycle_abs_omega_max: orbit_metrics.12,
        orbit_cycle_abs_omega_le_1e3_fraction: orbit_metrics.13,
        orbit_cycle_abs_omega_le_1e2_fraction: orbit_metrics.14,
        orbit_cycle_abs_omega_le_1e1_fraction: orbit_metrics.15,
        orbit_cycle_zero_fraction: orbit_metrics.16,
        orbit_cycle_transition_fraction: orbit_metrics.17,
        orbit_cycle_bidirectional_fraction: orbit_metrics.18,
        orbit_cycle_adjacent_fraction: orbit_metrics.19,
        orbit_selected_out_degree_mean: orbit_metrics.20,
        orbit_selected_out_degree_std: orbit_metrics.21,
        orbit_selected_out_degree_min: orbit_metrics.22,
        orbit_selected_out_degree_max: orbit_metrics.23,
        orbit_kkt_available: orbit_metrics.24,
        orbit_search_scalar_available: orbit_metrics.25,
        orbit_result_iterations_log1p: orbit_metrics.26,
        orbit_result_returned_orbit_count: orbit_metrics.27,
        orbit_best_beta_margin: orbit_metrics.28,
        orbit_best_q_error_bound: orbit_metrics.29,
        orbit_best_has_mu: orbit_metrics.30,
        orbit_best_has_xi: orbit_metrics.31,
        orbit_best_is_admissible_exact: orbit_metrics.32,
        orbit_best_is_indeterminate_f64: orbit_metrics.33,
    }
}

#[allow(clippy::type_complexity)]
fn zero_orbit_metrics(
    _facet_count: usize,
) -> (
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
) {
    (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0)
}

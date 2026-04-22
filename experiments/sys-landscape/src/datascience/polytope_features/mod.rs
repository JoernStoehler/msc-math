//! Combined polytope-level feature assembly for the datascience dataset stage.
//!
//! This module owns the `load -> enrich -> save` core for polytope-level
//! features. The executable shell should stay thin and only parse paths, load
//! rows, map `enrich_row`, and write the result.

mod feature_capacity;
mod feature_dual_vertices;
mod feature_face_geometry;
mod feature_face_symplectic;
mod feature_omega;
mod feature_orbit;
mod feature_skeleton;
mod feature_sys;
mod feature_volume;

use crate::datascience::io::{deserialize_vec4_rational, read_jsonl};
use feature_capacity::CapacityFields;
use feature_dual_vertices::DualVertexFields;
use feature_face_geometry::FaceGeometryFields;
use feature_face_symplectic::FaceSymplecticFields;
use feature_omega::OmegaFields;
pub use feature_orbit::{build_cache_index, OrbitCacheIndex, OrbitFields};
use feature_skeleton::SkeletonFields;
use feature_sys::SysFields;
use feature_volume::VolumeFields;
use num_rational::BigRational;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use symplectic::geom::polytope::Polytope4D;
use symplectic::geom::skeleton::Skeleton;
use symplectic::geom::volume::volume;

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

pub fn load_inputs(core_tables_dir: &Path) -> Vec<PolytopeFeatureInputRow> {
    let polytopes = read_jsonl::<NormalizedPolytopeRow>(&core_tables_dir.join("polytopes.jsonl"));
    let mut capacities = read_jsonl::<CapacityResultRow>(&core_tables_dir.join("capacity_results.jsonl"))
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

pub fn enrich_row(row: &PolytopeFeatureInputRow, cache: &OrbitCacheIndex) -> PolytopeFeatureRow {
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
    let duals = polytope.dual_vertices_f64();
    let facet_count = row.facet_count;

    let capacity_fields: CapacityFields = feature_capacity::compute(row);
    let volume_fields: VolumeFields = feature_volume::compute(row);
    let sys_fields: SysFields = feature_sys::compute(row);
    let dual_vertex_fields: DualVertexFields =
        feature_dual_vertices::compute(&row.dual_vertices_rational);
    let skeleton_fields: SkeletonFields = feature_skeleton::compute(&polytope, &skeleton, facet_count);
    let face_geometry_fields: FaceGeometryFields = feature_face_geometry::compute(
        &polytope,
        &skeleton,
        &vertices,
        facet_count,
        linear_scale,
        facet_scale,
    );
    let face_symplectic_fields: FaceSymplecticFields =
        feature_face_symplectic::compute(&skeleton, &vertices, volume_scale);
    let omega_fields: OmegaFields =
        feature_omega::compute(&polytope, &skeleton, &duals, facet_count, omega_scale);
    let orbit_fields: OrbitFields = feature_orbit::compute(
        row,
        &polytope,
        &duals,
        facet_count,
        &omega_fields.transition,
        cache,
    );

    PolytopeFeatureRow {
        poly_id: row.poly_id.clone(),
        facet_count,
        capacity: capacity_fields.capacity,
        capacity_iterations: capacity_fields.capacity_iterations,
        capacity_source: capacity_fields.capacity_source,
        volume: volume_fields.volume,
        sys: sys_fields.sys,
        dual_vertex_count: dual_vertex_fields.dual_vertex_count,
        dual_vertices_f64: dual_vertex_fields.dual_vertices_f64,
        dual_vertices_flat_f64: dual_vertex_fields.dual_vertices_flat_f64,
        vertex_count: skeleton_fields.vertex_count,
        edge_count: skeleton_fields.edge_count,
        ridge_count: skeleton_fields.ridge_count,
        is_simple: skeleton_fields.is_simple,
        simple_vertex_fraction: skeleton_fields.simple_vertex_fraction,
        edge_density: skeleton_fields.edge_density,
        vertex_incident_facets_mean: skeleton_fields.vertex_incident_facets_mean,
        vertex_incident_facets_std: skeleton_fields.vertex_incident_facets_std,
        vertex_incident_facets_min: skeleton_fields.vertex_incident_facets_min,
        vertex_incident_facets_max: skeleton_fields.vertex_incident_facets_max,
        vertex_degree_mean: skeleton_fields.vertex_degree_mean,
        vertex_degree_std: skeleton_fields.vertex_degree_std,
        vertex_degree_min: skeleton_fields.vertex_degree_min,
        vertex_degree_max: skeleton_fields.vertex_degree_max,
        ridge_size_mean: skeleton_fields.ridge_size_mean,
        ridge_size_std: skeleton_fields.ridge_size_std,
        ridge_size_min: skeleton_fields.ridge_size_min,
        ridge_size_max: skeleton_fields.ridge_size_max,
        facet_vertex_count_mean: skeleton_fields.facet_vertex_count_mean,
        facet_vertex_count_std: skeleton_fields.facet_vertex_count_std,
        facet_vertex_count_min: skeleton_fields.facet_vertex_count_min,
        facet_vertex_count_max: skeleton_fields.facet_vertex_count_max,
        facet_neighbor_count_mean: skeleton_fields.facet_neighbor_count_mean,
        facet_neighbor_count_std: skeleton_fields.facet_neighbor_count_std,
        facet_neighbor_count_min: skeleton_fields.facet_neighbor_count_min,
        facet_neighbor_count_max: skeleton_fields.facet_neighbor_count_max,
        edge_length_vol1_mean: face_geometry_fields.edge_length_vol1_mean,
        edge_length_vol1_std: face_geometry_fields.edge_length_vol1_std,
        edge_length_vol1_min: face_geometry_fields.edge_length_vol1_min,
        edge_length_vol1_max: face_geometry_fields.edge_length_vol1_max,
        edge_length_vol1_max_share: face_geometry_fields.edge_length_vol1_max_share,
        facet_volume_vol1_mean: face_geometry_fields.facet_volume_vol1_mean,
        facet_volume_vol1_std: face_geometry_fields.facet_volume_vol1_std,
        facet_volume_vol1_min: face_geometry_fields.facet_volume_vol1_min,
        facet_volume_vol1_max: face_geometry_fields.facet_volume_vol1_max,
        facet_volume_vol1_sum: face_geometry_fields.facet_volume_vol1_sum,
        facet_volume_vol1_max_share: face_geometry_fields.facet_volume_vol1_max_share,
        ridge_symp_area_volnorm_mean: face_symplectic_fields.ridge_symp_area_volnorm_mean,
        ridge_symp_area_volnorm_std: face_symplectic_fields.ridge_symp_area_volnorm_std,
        ridge_symp_area_volnorm_min: face_symplectic_fields.ridge_symp_area_volnorm_min,
        ridge_symp_area_volnorm_max: face_symplectic_fields.ridge_symp_area_volnorm_max,
        ridge_symp_area_volnorm_sum: face_symplectic_fields.ridge_symp_area_volnorm_sum,
        ridge_symp_area_volnorm_max_share: face_symplectic_fields.ridge_symp_area_volnorm_max_share,
        ridge_symp_area_volnorm_zero_fraction: face_symplectic_fields.ridge_symp_area_volnorm_zero_fraction,
        ridge_symp_area_volnorm_le_1em3_fraction: face_symplectic_fields.ridge_symp_area_volnorm_le_1em3_fraction,
        ridge_symp_area_volnorm_le_1em2_fraction: face_symplectic_fields.ridge_symp_area_volnorm_le_1em2_fraction,
        ridge_symp_area_volnorm_le_1em1_fraction: face_symplectic_fields.ridge_symp_area_volnorm_le_1em1_fraction,
        allpair_abs_omega_vol1_mean: omega_fields.allpair_abs_omega_vol1_mean,
        allpair_abs_omega_vol1_std: omega_fields.allpair_abs_omega_vol1_std,
        allpair_abs_omega_vol1_min: omega_fields.allpair_abs_omega_vol1_min,
        allpair_abs_omega_vol1_max: omega_fields.allpair_abs_omega_vol1_max,
        allpair_zero_fraction: omega_fields.allpair_zero_fraction,
        ridge_abs_omega_vol1_mean: omega_fields.ridge_abs_omega_vol1_mean,
        ridge_abs_omega_vol1_std: omega_fields.ridge_abs_omega_vol1_std,
        ridge_abs_omega_vol1_min: omega_fields.ridge_abs_omega_vol1_min,
        ridge_abs_omega_vol1_max: omega_fields.ridge_abs_omega_vol1_max,
        ridge_zero_fraction: omega_fields.ridge_zero_fraction,
        ridge_abs_omega_vol1_le_1em3_fraction: omega_fields.ridge_abs_omega_vol1_le_1em3_fraction,
        ridge_abs_omega_vol1_le_1em2_fraction: omega_fields.ridge_abs_omega_vol1_le_1em2_fraction,
        ridge_abs_omega_vol1_le_1em1_fraction: omega_fields.ridge_abs_omega_vol1_le_1em1_fraction,
        transition_density: omega_fields.transition_density,
        transition_bidirectional_fraction: omega_fields.transition_bidirectional_fraction,
        transition_out_degree_mean: omega_fields.transition_out_degree_mean,
        transition_out_degree_std: omega_fields.transition_out_degree_std,
        transition_out_degree_min: omega_fields.transition_out_degree_min,
        transition_out_degree_max: omega_fields.transition_out_degree_max,
        orbit_sigma_available: orbit_fields.orbit_sigma_available,
        orbit_sigma_count: orbit_fields.orbit_sigma_count,
        orbit_sigma_gap_cutoff: orbit_fields.orbit_sigma_gap_cutoff,
        orbit_sigma_len: orbit_fields.orbit_sigma_len,
        orbit_sigma_fraction: orbit_fields.orbit_sigma_fraction,
        orbit_selected_norm_mean: orbit_fields.orbit_selected_norm_mean,
        orbit_selected_norm_std: orbit_fields.orbit_selected_norm_std,
        orbit_selected_norm_min: orbit_fields.orbit_selected_norm_min,
        orbit_selected_norm_max: orbit_fields.orbit_selected_norm_max,
        orbit_cycle_abs_omega_mean: orbit_fields.orbit_cycle_abs_omega_mean,
        orbit_cycle_abs_omega_std: orbit_fields.orbit_cycle_abs_omega_std,
        orbit_cycle_abs_omega_min: orbit_fields.orbit_cycle_abs_omega_min,
        orbit_cycle_abs_omega_max: orbit_fields.orbit_cycle_abs_omega_max,
        orbit_cycle_abs_omega_le_1e3_fraction: orbit_fields.orbit_cycle_abs_omega_le_1e3_fraction,
        orbit_cycle_abs_omega_le_1e2_fraction: orbit_fields.orbit_cycle_abs_omega_le_1e2_fraction,
        orbit_cycle_abs_omega_le_1e1_fraction: orbit_fields.orbit_cycle_abs_omega_le_1e1_fraction,
        orbit_cycle_zero_fraction: orbit_fields.orbit_cycle_zero_fraction,
        orbit_cycle_transition_fraction: orbit_fields.orbit_cycle_transition_fraction,
        orbit_cycle_bidirectional_fraction: orbit_fields.orbit_cycle_bidirectional_fraction,
        orbit_cycle_adjacent_fraction: orbit_fields.orbit_cycle_adjacent_fraction,
        orbit_selected_out_degree_mean: orbit_fields.orbit_selected_out_degree_mean,
        orbit_selected_out_degree_std: orbit_fields.orbit_selected_out_degree_std,
        orbit_selected_out_degree_min: orbit_fields.orbit_selected_out_degree_min,
        orbit_selected_out_degree_max: orbit_fields.orbit_selected_out_degree_max,
        orbit_kkt_available: orbit_fields.orbit_kkt_available,
        orbit_search_scalar_available: orbit_fields.orbit_search_scalar_available,
        orbit_result_iterations_log1p: orbit_fields.orbit_result_iterations_log1p,
        orbit_result_returned_orbit_count: orbit_fields.orbit_result_returned_orbit_count,
        orbit_best_beta_margin: orbit_fields.orbit_best_beta_margin,
        orbit_best_q_error_bound: orbit_fields.orbit_best_q_error_bound,
        orbit_best_has_mu: orbit_fields.orbit_best_has_mu,
        orbit_best_has_xi: orbit_fields.orbit_best_has_xi,
        orbit_best_is_admissible_exact: orbit_fields.orbit_best_is_admissible_exact,
        orbit_best_is_indeterminate_f64: orbit_fields.orbit_best_is_indeterminate_f64,
    }
}

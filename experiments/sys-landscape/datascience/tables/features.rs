//! Build the enriched polytope table from unified producer rows.

#[path = "features_dual_vertices.rs"]
mod features_dual_vertices;
#[path = "features_face_geometry.rs"]
mod features_face_geometry;
#[path = "features_face_symplectic.rs"]
mod features_face_symplectic;
#[path = "features_geometry.rs"]
mod features_geometry;
#[path = "features_helpers.rs"]
mod features_helpers;
#[path = "features_omega.rs"]
mod features_omega;
#[path = "features_orbit.rs"]
mod features_orbit;
#[path = "features_skeleton.rs"]
mod features_skeleton;

use crate::load_caches::LoadedPolytopeRow;
use crate::rows::PolytopeTableRow;
use euclidean_polytopes::{
    edges_from_vertex_facet_incidence, two_faces_from_vertex_facet_incidence,
    vertex_facets_from_vertex_facet_incidence,
};
use exp_sys_landscape::capacity_auto;
use exp_sys_landscape::euclidean_volume_f64;
use exp_sys_landscape::SysLandscapePolytopeCache;

fn enrich_row(row: &LoadedPolytopeRow) -> PolytopeTableRow {
    let (dual_vectors, dual_vertex_fields) = features_dual_vertices::dual_vertices_f64(row);
    let polytope: SysLandscapePolytopeCache =
        SysLandscapePolytopeCache::from_f64_dual_vertices(dual_vectors.clone())
            .unwrap_or_else(|| panic!("reconstruct {}", row.poly_id));
    let polytope_volume =
        euclidean_volume_f64(&polytope.vertices, &polytope.vertex_facet_incidence);
    let actual_capacity = if row.capacity > 0.0 {
        row.capacity
    } else {
        capacity_auto(
            &polytope.dual_vertices_f64,
            &polytope.dual_vertices,
            &polytope.facet_intersection_is_nonempty,
            &polytope.omega_signs,
        )
        .unwrap_or_else(|e| panic!("capacity {}: {:?}", row.poly_id, e))
        .capacity()
    };
    let sys_value = actual_capacity * actual_capacity / (2.0 * polytope_volume);
    let facet_count = polytope.facet_count();
    let geometry_fields = features_geometry::compute_geometry_fields(
        &dual_vertex_fields.dual_vertices_f64,
        polytope_volume,
    );
    let linear_scale = polytope_volume.powf(0.25);
    let facet_scale = polytope_volume.powf(0.75);
    let omega_scale = polytope_volume.sqrt();
    let volume_scale = polytope_volume.sqrt();
    let incidence = &polytope.vertex_facet_incidence;
    let vertex_facets = vertex_facets_from_vertex_facet_incidence(incidence);
    let edges = edges_from_vertex_facet_incidence(incidence);
    let two_faces = two_faces_from_vertex_facet_incidence(incidence);
    let vertices = &polytope.vertices_f64;
    let duals = &polytope.dual_vertices_f64;
    let skeleton_fields =
        features_skeleton::compute_skeleton_fields(&polytope, &vertex_facets, &edges, &two_faces);
    let face_geometry_fields = features_face_geometry::compute_face_geometry_fields(
        &polytope,
        &edges,
        &vertices,
        facet_count,
        linear_scale,
        facet_scale,
    );
    let face_symplectic_fields = features_face_symplectic::compute_face_symplectic_fields(
        &two_faces,
        &vertices,
        volume_scale,
    );
    let omega_fields = features_omega::compute_omega_fields(
        &polytope,
        &two_faces,
        &duals,
        facet_count,
        omega_scale,
    );
    let orbit_fields = features_orbit::compute_orbit_fields(
        row,
        &polytope,
        &duals,
        facet_count,
        &omega_fields.transition,
    );

    PolytopeTableRow {
        poly_id: row.poly_id.clone(),
        dual_vertices_rational: row.dual_vertices_rational.clone(),
        facet_count,
        capacity: actual_capacity,
        capacity_iterations: row.capacity_iterations,
        capacity_source: row.capacity_source.clone(),
        volume: polytope_volume,
        sys: sys_value,
        sigma_gap_cutoff: row.sigma_gap_cutoff,
        sigmas: row.sigmas.clone(),
        raw_orbit_scalars: row.orbit_scalars.clone(),
        geom_vol1_norm_mean: geometry_fields.geom_vol1_norm_mean,
        geom_vol1_norm_std: geometry_fields.geom_vol1_norm_std,
        geom_vol1_norm_min: geometry_fields.geom_vol1_norm_min,
        geom_vol1_norm_max: geometry_fields.geom_vol1_norm_max,
        geom_vol1_centroid_norm: geometry_fields.geom_vol1_centroid_norm,
        geom_vol1_coord_std_x: geometry_fields.geom_vol1_coord_std_x,
        geom_vol1_coord_std_y: geometry_fields.geom_vol1_coord_std_y,
        geom_vol1_coord_std_z: geometry_fields.geom_vol1_coord_std_z,
        geom_vol1_coord_std_w: geometry_fields.geom_vol1_coord_std_w,
        geom_cosine_mean: geometry_fields.geom_cosine_mean,
        geom_cosine_std: geometry_fields.geom_cosine_std,
        geom_cosine_min: geometry_fields.geom_cosine_min,
        geom_cosine_max: geometry_fields.geom_cosine_max,
        geom_vol1_pairwise_dist_mean: geometry_fields.geom_vol1_pairwise_dist_mean,
        geom_vol1_pairwise_dist_std: geometry_fields.geom_vol1_pairwise_dist_std,
        geom_vol1_pairwise_dist_min: geometry_fields.geom_vol1_pairwise_dist_min,
        geom_vol1_pairwise_dist_max: geometry_fields.geom_vol1_pairwise_dist_max,
        geom_vol1_sval_1: geometry_fields.geom_vol1_sval_1,
        geom_vol1_sval_2: geometry_fields.geom_vol1_sval_2,
        geom_vol1_sval_3: geometry_fields.geom_vol1_sval_3,
        geom_vol1_sval_4: geometry_fields.geom_vol1_sval_4,
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
        ridge_symp_area_volnorm_zero_fraction: face_symplectic_fields
            .ridge_symp_area_volnorm_zero_fraction,
        ridge_symp_area_volnorm_le_1em3_fraction: face_symplectic_fields
            .ridge_symp_area_volnorm_le_1em3_fraction,
        ridge_symp_area_volnorm_le_1em2_fraction: face_symplectic_fields
            .ridge_symp_area_volnorm_le_1em2_fraction,
        ridge_symp_area_volnorm_le_1em1_fraction: face_symplectic_fields
            .ridge_symp_area_volnorm_le_1em1_fraction,
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
        transition_bidirectional_given_facet_intersection_fraction: omega_fields
            .transition_bidirectional_given_facet_intersection_fraction,
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
        orbit_cycle_facet_intersection_fraction: orbit_fields
            .orbit_cycle_facet_intersection_fraction,
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

pub fn build_polytope_table(rows: &[LoadedPolytopeRow]) -> Vec<PolytopeTableRow> {
    rows.iter().map(enrich_row).collect()
}

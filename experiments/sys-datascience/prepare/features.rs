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
#[path = "features_skeleton.rs"]
mod features_skeleton;

use crate::canonize::VolumeOneTransform;
use crate::load_caches::LoadedPolytopeRow;
use crate::rows::PolytopeTableRow;
use euclidean_polytopes::{
    edges_from_vertex_facet_incidence, two_faces_from_vertex_facet_incidence,
    vertex_facets_from_vertex_facet_incidence,
};
use exp_sys_landscape::SysLandscapePolytopeCache;
use rayon::prelude::*;

fn enrich_row(row: &LoadedPolytopeRow) -> PolytopeTableRow {
    let producer_volume = row.volume;
    assert!(
        producer_volume.is_finite() && producer_volume > 0.0,
        "polytope {} has invalid producer volume {}",
        row.poly_id,
        producer_volume
    );
    let producer_capacity = if row.capacity > 0.0 {
        row.capacity
    } else {
        panic!(
            "polytope {} lacks producer capacity; normal table builds do not repair capacity",
            row.poly_id
        )
    };
    let transform = VolumeOneTransform::from_volume(producer_volume);
    let raw_dual_vectors = features_dual_vertices::raw_dual_vertices_f64(row);
    let volume_one_dual_vectors = transform.apply_dual_vertices(&raw_dual_vectors);
    let polytope: SysLandscapePolytopeCache =
        SysLandscapePolytopeCache::from_f64_dual_vertices(volume_one_dual_vectors)
            .unwrap_or_else(|| panic!("reconstruct volume-one {}", row.poly_id));
    let capacity_prepared = transform.apply_action_value(producer_capacity);
    let sys_value = capacity_prepared * capacity_prepared / 2.0;
    let facet_count = polytope.facet_count();
    let dual_vertex_fields =
        features_dual_vertices::dual_vertex_fields(&polytope.dual_vertices_f64);
    let geometry_fields =
        features_geometry::compute_geometry_fields(&dual_vertex_fields.dual_vertices_f64);
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
        vertices,
        facet_count,
    );
    let face_symplectic_fields =
        features_face_symplectic::compute_face_symplectic_fields(&two_faces, vertices, incidence);
    let omega_fields =
        features_omega::compute_omega_fields(&polytope, &two_faces, duals, facet_count);

    PolytopeTableRow {
        poly_id: row.poly_id.clone(),
        facet_count,
        capacity: capacity_prepared,
        capacity_source: row.capacity_source.clone(),
        volume: 1.0,
        sys: sys_value,
        geom_norm_mean: geometry_fields.geom_norm_mean,
        geom_norm_std: geometry_fields.geom_norm_std,
        geom_norm_min: geometry_fields.geom_norm_min,
        geom_norm_max: geometry_fields.geom_norm_max,
        geom_centroid_norm: geometry_fields.geom_centroid_norm,
        geom_coord_std_x: geometry_fields.geom_coord_std_x,
        geom_coord_std_y: geometry_fields.geom_coord_std_y,
        geom_coord_std_z: geometry_fields.geom_coord_std_z,
        geom_coord_std_w: geometry_fields.geom_coord_std_w,
        geom_cosine_mean: geometry_fields.geom_cosine_mean,
        geom_cosine_std: geometry_fields.geom_cosine_std,
        geom_cosine_min: geometry_fields.geom_cosine_min,
        geom_cosine_max: geometry_fields.geom_cosine_max,
        geom_pairwise_dist_mean: geometry_fields.geom_pairwise_dist_mean,
        geom_pairwise_dist_std: geometry_fields.geom_pairwise_dist_std,
        geom_pairwise_dist_min: geometry_fields.geom_pairwise_dist_min,
        geom_pairwise_dist_max: geometry_fields.geom_pairwise_dist_max,
        geom_sval_1: geometry_fields.geom_sval_1,
        geom_sval_2: geometry_fields.geom_sval_2,
        geom_sval_3: geometry_fields.geom_sval_3,
        geom_sval_4: geometry_fields.geom_sval_4,
        dual_vertex_count: dual_vertex_fields.dual_vertex_count,
        dual_vertices_f64: dual_vertex_fields.dual_vertices_f64,
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
        edge_length_mean: face_geometry_fields.edge_length_mean,
        edge_length_std: face_geometry_fields.edge_length_std,
        edge_length_min: face_geometry_fields.edge_length_min,
        edge_length_max: face_geometry_fields.edge_length_max,
        edge_length_max_share: face_geometry_fields.edge_length_max_share,
        facet_volume_mean: face_geometry_fields.facet_volume_mean,
        facet_volume_std: face_geometry_fields.facet_volume_std,
        facet_volume_min: face_geometry_fields.facet_volume_min,
        facet_volume_max: face_geometry_fields.facet_volume_max,
        facet_volume_sum: face_geometry_fields.facet_volume_sum,
        facet_volume_max_share: face_geometry_fields.facet_volume_max_share,
        ridge_symp_area_mean: face_symplectic_fields.ridge_symp_area_mean,
        ridge_symp_area_std: face_symplectic_fields.ridge_symp_area_std,
        ridge_symp_area_min: face_symplectic_fields.ridge_symp_area_min,
        ridge_symp_area_max: face_symplectic_fields.ridge_symp_area_max,
        ridge_symp_area_q25: face_symplectic_fields.ridge_symp_area_q25,
        ridge_symp_area_median: face_symplectic_fields.ridge_symp_area_median,
        ridge_symp_area_q75: face_symplectic_fields.ridge_symp_area_q75,
        ridge_symp_area_q90: face_symplectic_fields.ridge_symp_area_q90,
        ridge_symp_area_q95: face_symplectic_fields.ridge_symp_area_q95,
        ridge_symp_area_sum: face_symplectic_fields.ridge_symp_area_sum,
        ridge_symp_area_max_share: face_symplectic_fields.ridge_symp_area_max_share,
        ridge_symp_area_top3_share: face_symplectic_fields.ridge_symp_area_top3_share,
        ridge_symp_area_zero_fraction: face_symplectic_fields.ridge_symp_area_zero_fraction,
        ridge_symp_area_le_1em3_fraction: face_symplectic_fields.ridge_symp_area_le_1em3_fraction,
        ridge_symp_area_le_1em2_fraction: face_symplectic_fields.ridge_symp_area_le_1em2_fraction,
        ridge_symp_area_le_1em1_fraction: face_symplectic_fields.ridge_symp_area_le_1em1_fraction,
        ridge_euclidean_area_mean: face_symplectic_fields.ridge_euclidean_area_mean,
        ridge_euclidean_area_std: face_symplectic_fields.ridge_euclidean_area_std,
        ridge_euclidean_area_min: face_symplectic_fields.ridge_euclidean_area_min,
        ridge_euclidean_area_max: face_symplectic_fields.ridge_euclidean_area_max,
        ridge_euclidean_area_q25: face_symplectic_fields.ridge_euclidean_area_q25,
        ridge_euclidean_area_median: face_symplectic_fields.ridge_euclidean_area_median,
        ridge_euclidean_area_q75: face_symplectic_fields.ridge_euclidean_area_q75,
        ridge_euclidean_area_q90: face_symplectic_fields.ridge_euclidean_area_q90,
        ridge_euclidean_area_q95: face_symplectic_fields.ridge_euclidean_area_q95,
        ridge_euclidean_area_sum: face_symplectic_fields.ridge_euclidean_area_sum,
        ridge_euclidean_area_max_share: face_symplectic_fields.ridge_euclidean_area_max_share,
        ridge_euclidean_area_top3_share: face_symplectic_fields.ridge_euclidean_area_top3_share,
        ridge_euclidean_area_zero_fraction: face_symplectic_fields
            .ridge_euclidean_area_zero_fraction,
        ridge_euclidean_area_le_1em3_fraction: face_symplectic_fields
            .ridge_euclidean_area_le_1em3_fraction,
        ridge_euclidean_area_le_1em2_fraction: face_symplectic_fields
            .ridge_euclidean_area_le_1em2_fraction,
        ridge_euclidean_area_le_1em1_fraction: face_symplectic_fields
            .ridge_euclidean_area_le_1em1_fraction,
        ridge_symp_over_euclidean_area_mean: face_symplectic_fields
            .ridge_symp_over_euclidean_area_mean,
        ridge_symp_over_euclidean_area_std: face_symplectic_fields
            .ridge_symp_over_euclidean_area_std,
        ridge_symp_over_euclidean_area_min: face_symplectic_fields
            .ridge_symp_over_euclidean_area_min,
        ridge_symp_over_euclidean_area_max: face_symplectic_fields
            .ridge_symp_over_euclidean_area_max,
        ridge_symp_over_euclidean_area_q25: face_symplectic_fields
            .ridge_symp_over_euclidean_area_q25,
        ridge_symp_over_euclidean_area_median: face_symplectic_fields
            .ridge_symp_over_euclidean_area_median,
        ridge_symp_over_euclidean_area_q75: face_symplectic_fields
            .ridge_symp_over_euclidean_area_q75,
        ridge_symp_over_euclidean_area_q90: face_symplectic_fields
            .ridge_symp_over_euclidean_area_q90,
        ridge_symp_over_euclidean_area_q95: face_symplectic_fields
            .ridge_symp_over_euclidean_area_q95,
        ridge_symp_area_ordered_face_count: face_symplectic_fields
            .ridge_symp_area_ordered_face_count,
        ridge_symp_area_ordering_failure_count: face_symplectic_fields
            .ridge_symp_area_ordering_failure_count,
        ridge_symp_area_ordered_fraction: face_symplectic_fields.ridge_symp_area_ordered_fraction,
        allpair_abs_omega_mean: omega_fields.allpair_abs_omega_mean,
        allpair_abs_omega_std: omega_fields.allpair_abs_omega_std,
        allpair_abs_omega_min: omega_fields.allpair_abs_omega_min,
        allpair_abs_omega_max: omega_fields.allpair_abs_omega_max,
        allpair_abs_omega_q25: omega_fields.allpair_abs_omega_q25,
        allpair_abs_omega_median: omega_fields.allpair_abs_omega_median,
        allpair_abs_omega_q75: omega_fields.allpair_abs_omega_q75,
        allpair_abs_omega_q90: omega_fields.allpair_abs_omega_q90,
        allpair_abs_omega_top3_share: omega_fields.allpair_abs_omega_top3_share,
        allpair_zero_fraction: omega_fields.allpair_zero_fraction,
        omega_matrix_frobenius_norm: omega_fields.omega_matrix_frobenius_norm,
        omega_matrix_spectral_norm: omega_fields.omega_matrix_spectral_norm,
        omega_matrix_stable_rank: omega_fields.omega_matrix_stable_rank,
        omega_matrix_rank_1em10: omega_fields.omega_matrix_rank_1em10,
        omega_matrix_nullity_1em10: omega_fields.omega_matrix_nullity_1em10,
        omega_sign_out_degree_mean: omega_fields.omega_sign_out_degree_mean,
        omega_sign_out_degree_std: omega_fields.omega_sign_out_degree_std,
        omega_sign_out_degree_min: omega_fields.omega_sign_out_degree_min,
        omega_sign_out_degree_max: omega_fields.omega_sign_out_degree_max,
        allpair_abs_normalized_omega_mean: omega_fields.allpair_abs_normalized_omega_mean,
        allpair_abs_normalized_omega_std: omega_fields.allpair_abs_normalized_omega_std,
        allpair_abs_normalized_omega_min: omega_fields.allpair_abs_normalized_omega_min,
        allpair_abs_normalized_omega_max: omega_fields.allpair_abs_normalized_omega_max,
        ridge_abs_omega_mean: omega_fields.ridge_abs_omega_mean,
        ridge_abs_omega_std: omega_fields.ridge_abs_omega_std,
        ridge_abs_omega_min: omega_fields.ridge_abs_omega_min,
        ridge_abs_omega_max: omega_fields.ridge_abs_omega_max,
        ridge_abs_omega_q25: omega_fields.ridge_abs_omega_q25,
        ridge_abs_omega_median: omega_fields.ridge_abs_omega_median,
        ridge_abs_omega_q75: omega_fields.ridge_abs_omega_q75,
        ridge_abs_omega_q90: omega_fields.ridge_abs_omega_q90,
        ridge_abs_omega_top3_share: omega_fields.ridge_abs_omega_top3_share,
        ridge_zero_fraction: omega_fields.ridge_zero_fraction,
        ridge_abs_normalized_omega_mean: omega_fields.ridge_abs_normalized_omega_mean,
        ridge_abs_normalized_omega_std: omega_fields.ridge_abs_normalized_omega_std,
        ridge_abs_normalized_omega_min: omega_fields.ridge_abs_normalized_omega_min,
        ridge_abs_normalized_omega_max: omega_fields.ridge_abs_normalized_omega_max,
        ridge_abs_omega_le_1em3_fraction: omega_fields.ridge_abs_omega_le_1em3_fraction,
        ridge_abs_omega_le_1em2_fraction: omega_fields.ridge_abs_omega_le_1em2_fraction,
        ridge_abs_omega_le_1em1_fraction: omega_fields.ridge_abs_omega_le_1em1_fraction,
        transition_density: omega_fields.transition_density,
        transition_bidirectional_given_facet_intersection_fraction: omega_fields
            .transition_bidirectional_given_facet_intersection_fraction,
        transition_out_degree_mean: omega_fields.transition_out_degree_mean,
        transition_out_degree_std: omega_fields.transition_out_degree_std,
        transition_out_degree_min: omega_fields.transition_out_degree_min,
        transition_out_degree_max: omega_fields.transition_out_degree_max,
    }
}

pub fn build_polytope_table(rows: &[LoadedPolytopeRow]) -> Vec<PolytopeTableRow> {
    rows.par_iter().map(enrich_row).collect()
}

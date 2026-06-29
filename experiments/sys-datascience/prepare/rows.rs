//! Output row schemas for the sys-landscape datascience tables.

use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct PolytopeTableRow {
    pub poly_id: String,
    pub facet_count: usize,
    pub capacity_source: String,
    pub sys: f64,
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
    pub ridge_symp_area_ordered_face_count: usize,
    pub ridge_symp_area_ordering_failure_count: usize,
    pub ridge_symp_area_ordered_fraction: f64,
    pub ridge_symp_area_mean_over_volume_sqrt: f64,
    pub ridge_symp_area_std_over_volume_sqrt: f64,
    pub ridge_symp_area_min_over_volume_sqrt: f64,
    pub ridge_symp_area_max_over_volume_sqrt: f64,
    pub ridge_symp_area_q25_over_volume_sqrt: f64,
    pub ridge_symp_area_median_over_volume_sqrt: f64,
    pub ridge_symp_area_q75_over_volume_sqrt: f64,
    pub ridge_symp_area_q90_over_volume_sqrt: f64,
    pub ridge_symp_area_q95_over_volume_sqrt: f64,
    pub ridge_symp_area_sum_over_volume_sqrt: f64,
    pub ridge_symp_area_max_share: f64,
    pub ridge_symp_area_top3_share: f64,
}

#[derive(Debug, Serialize)]
pub struct ProvenanceRunRow {
    pub provenance_id: String,
    pub poly_id: String,
    pub dataset: String,
    pub family: String,
    pub role: String,
    pub search_space: String,
    pub optimizer: String,
    pub backend: String,
    pub source_name: String,
    pub root_group_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_seed: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_attempt: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_h_min: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_h_max: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_k: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_m: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_bounces: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed_index: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lineage_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_time_ms: Option<f64>,
}

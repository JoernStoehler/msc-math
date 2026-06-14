use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ScanRow {
    pub family: String,
    pub source_id: String,
    pub input_source: String,
    pub generated_attempt: Option<u64>,
    pub generator_seed: Option<u64>,
    pub requested_facet_count: Option<usize>,
    #[serde(default)]
    pub original_facet_count: Option<usize>,
    pub facet_count: usize,
    #[serde(default = "default_product_rounding_status")]
    pub product_rounding_status: String,
    #[serde(default)]
    pub product_rounding_max_minor_over_major: Option<f64>,
    #[serde(default)]
    pub product_rounding_max_abs_change: Option<f64>,
    #[serde(default)]
    pub product_q_facet_count: Option<usize>,
    #[serde(default)]
    pub product_p_facet_count: Option<usize>,
    #[serde(default = "default_facet_simplification_policy")]
    pub facet_simplification_policy: String,
    #[serde(default = "default_facet_simplification_status")]
    pub facet_simplification_status: String,
    #[serde(default = "default_product_simplification_status")]
    pub product_simplification_status: String,
    #[serde(default)]
    pub simplified_facet_count: Option<usize>,
    #[serde(default)]
    pub removed_facet_count: usize,
    #[serde(default)]
    pub removed_original_facets: Vec<usize>,
    #[serde(default)]
    pub facet_simplification_delta_bound: Option<f64>,
    #[serde(default)]
    pub product_simplification_delta_bound: Option<f64>,
    #[serde(default)]
    pub capacity_ratio_upper_bound: Option<f64>,
    #[serde(default)]
    pub volume_ratio_upper_bound: Option<f64>,
    #[serde(default)]
    pub sys_ratio_lower_bound: Option<f64>,
    #[serde(default)]
    pub sys_ratio_upper_bound: Option<f64>,
    #[serde(default = "default_validation_policy")]
    pub validation_policy: String,
    #[serde(default = "default_capacity_method")]
    pub capacity_method: String,
    pub validation_status: String,
    pub validation_reasons: Vec<String>,
    pub validation_time_ms: f64,
    pub origin_status: String,
    pub origin_lp_status: String,
    pub origin_lp_max_min_lambda: Option<f64>,
    pub origin_lp_max_abs_residual: Option<f64>,
    pub facet_extremality_status: String,
    pub facets_with_definite_vertex_count: usize,
    pub facets_with_possible_vertex_count: usize,
    pub facets_without_definite_vertex_count: usize,
    pub facets_without_possible_vertex_count: usize,
    pub outcome: String,
    pub failure_reason: Option<String>,
    pub f64_capacity: Option<f64>,
    #[serde(default)]
    pub simplified_f64_capacity: Option<f64>,
    pub audit_capacity_label: Option<f64>,
    #[serde(default)]
    pub original_artifact_capacity_label: Option<f64>,
    #[serde(default)]
    pub simplified_audit_capacity_label: Option<f64>,
    pub artifact_capacity_label: Option<f64>,
    pub exact_audit_status: String,
    pub exact_audit_time_ms: f64,
    pub exact_audit_reasons: Vec<String>,
    pub abs_action_error: Option<f64>,
    pub rel_action_error: Option<f64>,
    #[serde(default)]
    pub simplified_f64_vs_simplified_audit_abs_error: Option<f64>,
    #[serde(default)]
    pub simplified_f64_vs_simplified_audit_rel_error: Option<f64>,
    #[serde(default)]
    pub simplified_f64_vs_original_artifact_abs_error: Option<f64>,
    #[serde(default)]
    pub simplified_f64_vs_original_artifact_rel_error: Option<f64>,
    #[serde(default)]
    pub simplified_f64_vs_original_artifact_within_bound: Option<bool>,
    #[serde(default)]
    pub simplified_audit_vs_original_artifact_abs_error: Option<f64>,
    #[serde(default)]
    pub simplified_audit_vs_original_artifact_rel_error: Option<f64>,
    #[serde(default)]
    pub simplified_audit_vs_original_artifact_within_bound: Option<bool>,
    pub f64_time_ms: f64,
    pub agreement_status: String,
    pub trust_class: String,
    pub trust_reasons: Vec<String>,
    pub f64_sigma: Option<Vec<usize>>,
    pub audit_sigma_label: Option<Vec<usize>>,
    #[serde(alias = "iterations")]
    pub sigma_count: u64,
    pub admissible_f64_count: usize,
    pub indeterminate_f64_count: usize,
    pub inadmissible_count: usize,
    pub numerical_failure_count: usize,
    pub vertex_count: usize,
    pub vertex_indeterminate_count: usize,
    pub near_singular_vertex_count: usize,
    pub bounded_near_singular_vertex_count: usize,
    pub ambiguous_vertex_incidence_count: usize,
    pub facet_intersection_true_count: usize,
    pub facet_intersection_false_count: usize,
    pub facet_intersection_indeterminate_count: usize,
    pub omega_indeterminate_count: usize,
    pub min_action_gap: Option<f64>,
    pub indeterminate_overlaps_best_interval: bool,
}

fn default_validation_policy() -> String {
    "strict".to_string()
}

fn default_capacity_method() -> String {
    "transition_pruned_hk".to_string()
}

fn default_product_rounding_status() -> String {
    "unknown_legacy_row".to_string()
}

fn default_product_simplification_status() -> String {
    "unknown_legacy_row".to_string()
}

fn default_facet_simplification_policy() -> String {
    "unknown_legacy_row".to_string()
}

fn default_facet_simplification_status() -> String {
    "unknown_legacy_row".to_string()
}

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum AlgorithmStateRow {
    EvaluatedPoint { evaluation_id: String },
    EvaluatedPopulation { evaluation_ids: Vec<String> },
    UnevaluatedModelOrDistribution,
    NoSingleCurrentState,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SourcePoint {
    pub name: String,
    pub facet_count: usize,
    pub dual_flat: Vec<f64>,
    pub source_sys: Option<f64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RunProvenance {
    pub schema_version: u32,
    pub manifest_path: String,
    pub manifest_blake3: String,
    pub resolved_plan_hash: String,
    pub git_commit: String,
    pub git_dirty: bool,
    pub executable: String,
    pub executable_blake3: String,
    pub started_unix_ms: u128,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EvaluationRow {
    pub schema_version: u32,
    pub run_id: String,
    pub evaluation_id: String,
    pub proposal_id: Option<String>,
    pub role: String,
    pub logical_call: usize,
    pub charged: bool,
    pub point_key: String,
    pub cache_status: String,
    pub status: String,
    pub geometry_route: String,
    pub fallback_reason: Option<String>,
    pub usable_by_optimizer: bool,
    pub error: Option<String>,
    pub facet_count: usize,
    pub dual_flat: Vec<f64>,
    pub sys: Option<f64>,
    pub capacity: Option<f64>,
    pub volume: Option<f64>,
    pub winning_sigma: Option<Vec<usize>>,
    pub winning_beta_margin: Option<f64>,
    pub orbit_count: Option<usize>,
    pub sigma_iterations: Option<u64>,
    pub geometry_indeterminate_count: usize,
    pub vertex_indeterminate_count: usize,
    pub bounded_near_singular_vertex_count: usize,
    pub ambiguous_vertex_incidence_count: usize,
    pub facet_intersection_indeterminate_count: usize,
    pub omega_indeterminate_count: usize,
    pub geometry_ms: f64,
    pub volume_ms: f64,
    pub capacity_ms: f64,
    pub total_ms: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProposalRow {
    pub schema_version: u32,
    pub run_id: String,
    pub round_id: String,
    pub proposal_id: String,
    pub evaluation_id: String,
    pub proposal_index: usize,
    pub baseline_evaluation_id: Option<String>,
    pub displacement_l2: Option<f64>,
    pub normalized_displacement_l2: Option<f64>,
    pub algorithm_fields: serde_json::Value,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RoundRow {
    pub schema_version: u32,
    pub run_id: String,
    pub round_id: String,
    pub round_index: usize,
    pub charged_calls_before: usize,
    pub charged_calls_after: usize,
    #[serde(default)]
    pub charged_compute_ms_before: f64,
    #[serde(default)]
    pub charged_compute_ms_after: f64,
    pub best_evaluation_id_before: String,
    pub best_evaluation_id_after: String,
    pub best_sys_before: f64,
    pub best_sys_after: f64,
    // State recording was added after the first retained schema-1 rounds. A
    // missing field means that no state was recorded, not that the best
    // evaluation was the algorithm's exact current state.
    #[serde(default = "default_no_single_current_state")]
    pub algorithm_state_before: AlgorithmStateRow,
    #[serde(default = "default_no_single_current_state")]
    pub algorithm_state_after: AlgorithmStateRow,
    pub geometric_reference_kind: Option<String>,
    pub geometric_reference_dual_flat: Option<Vec<f64>>,
    pub ask_ms: f64,
    pub tell_ms: f64,
    pub proposal_ids: Vec<String>,
    pub selected: Vec<SelectedProposal>,
    pub stop_reason: Option<String>,
    pub algorithm_fields: serde_json::Value,
}

fn default_no_single_current_state() -> AlgorithmStateRow {
    AlgorithmStateRow::NoSingleCurrentState
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SelectedProposal {
    pub proposal_id: String,
    pub weight: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RunRow {
    pub schema_version: u32,
    pub run_id: String,
    pub start_id: String,
    pub algorithm_id: String,
    pub algorithm_kind: String,
    pub seed: u64,
    pub budget: usize,
    #[serde(default)]
    pub compute_budget_ms: Option<f64>,
    #[serde(default)]
    pub stop_sys_threshold: Option<f64>,
    pub charge_initial: bool,
    pub initial_evaluation_id: String,
    pub initial_sys: f64,
    pub best_evaluation_id: String,
    pub best_sys: f64,
    pub final_algorithm_state: AlgorithmStateRow,
    pub charged_calls: usize,
    #[serde(default)]
    pub evaluator_compute_ms: f64,
    #[serde(default)]
    pub optimizer_compute_ms: f64,
    #[serde(default)]
    pub charged_compute_ms: f64,
    #[serde(default)]
    pub compute_budget_overshoot_ms: f64,
    pub physical_evaluations: usize,
    pub invalid_evaluations: usize,
    pub indeterminate_evaluations: usize,
    pub exact_fallback_evaluations: usize,
    pub rounds: usize,
    pub stop_reason: String,
    pub wall_ms: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProbeSelection {
    pub schema_version: u32,
    pub source_artifact_dir: String,
    pub radii: Vec<f64>,
    pub random_direction_count: usize,
    pub selection_strategy: String,
    pub population_start_count: usize,
    pub selected_start_ids: Vec<String>,
    pub checkpoints: Vec<ProbeCheckpoint>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProbeCheckpoint {
    pub checkpoint_id: String,
    pub run_id: String,
    pub algorithm_id: String,
    pub checkpoint_call: usize,
    pub evaluation_id: String,
    pub base_sys: f64,
    pub dual_flat: Vec<f64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProbeRow {
    pub schema_version: u32,
    pub checkpoint_id: String,
    pub run_id: String,
    pub algorithm_id: String,
    pub checkpoint_call: usize,
    pub base_evaluation_id: String,
    pub base_sys: f64,
    pub direction_family: String,
    pub direction_index: usize,
    pub sign: i8,
    pub radius: f64,
    pub target_status: String,
    pub target_usable: bool,
    pub target_sys: Option<f64>,
    pub delta_sys: Option<f64>,
    pub slope: Option<f64>,
    pub normalized_displacement_l2: f64,
    pub target_evaluation_id: String,
}

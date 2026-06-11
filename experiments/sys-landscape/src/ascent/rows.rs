use crate::SysLandscapePolytopeCache;
use serde::{Deserialize, Serialize};
use symplectic::database::{OrbitScalars, PolytopeRecord, SigmaAction};

/// One row per seed — the main analysis dataset.
///
/// Schema is byte-identical between `gradient-ascent-general` and
/// `gradient-ascent-products`. `polytope_type` is set by the experiment:
/// general passes the literal `"general"` (see `gradient-ascent-general/main.rs`
/// line 507); products passes `lagrangian_{q_f}x{p_f}` where `q_f` and `p_f`
/// are the facet counts of the two Lagrangian factors (see
/// `gradient-ascent-products/main.rs` line 443, `bucket_name`).
///
/// The row stores both exact rational endpoint geometry and the legacy `f64`
/// endpoint dual vertices. The exact fields are the durable join surface for
/// later normalized datasets; the `f64` field remains for backwards-compatible
/// plotting and quick inspection.
#[derive(Debug, Serialize, Deserialize)]
pub struct SummaryRow {
    pub name: String,
    pub seed_index: usize,
    #[serde(default)]
    pub source_name: String,
    #[serde(default)]
    pub lineage_id: String,
    pub polytope_type: String,
    pub facet_count: usize,
    pub starting_sys: f64,
    #[serde(default)]
    pub final_capacity: f64,
    #[serde(default)]
    pub final_volume: f64,
    pub final_sys: f64,
    pub total_delta: f64,
    pub n_ascent_phases: usize,
    pub n_gradient_iters_total: usize,
    pub n_escape_overshoot: usize,
    pub n_escape_wiggle: usize,
    pub best_strategy: String,
    pub total_time_ms: f64,
    #[serde(default)]
    pub starting_dual_vertices_rational: Vec<[String; 4]>,
    #[serde(default)]
    pub final_dual_vertices_rational: Vec<[String; 4]>,
    pub final_dual_vertices: Vec<[f64; 4]>,
}

/// One row per iteration per ascent phase — diagnostic trace.
#[derive(Debug, Serialize, Deserialize)]
pub struct TraceRow {
    pub name: String,
    pub phase: usize,
    pub iteration: usize,
    pub step_type: String,
    pub t_fraction: f64,
    pub t_actual: f64,
    pub sys_before: f64,
    pub sys_after: f64,
    pub delta_sys: f64,
    pub gradient_norm: f64,
}

/// One row per successful computed polytope during an ascent run.
///
/// This is a producer output row, not a deduped table row. Multiple rows may
/// refer to the same exact polytope if the runtime computed its capacity more
/// than once in different run contexts.
#[derive(Clone, Serialize, Deserialize)]
pub struct ComputedPolytopeRow {
    pub result_id: String,
    pub dataset: String,
    pub run_id: String,
    pub seed_index: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phase: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iteration: Option<usize>,
    pub role: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t_fraction: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t_actual: Option<f64>,
    pub accepted_in_iteration: bool,
    pub became_run_final: bool,
    pub dual_vertices_rational: Vec<[String; 4]>,
    pub facet_count: usize,
    pub capacity: f64,
    pub volume: f64,
    pub sys: f64,
    pub sigmas: Vec<SigmaAction>,
    pub orbit_scalars: OrbitScalars,
}

/// One row per ascent occurrence of a polytope.
///
/// This is run metadata. It records where a polytope appeared in an ascent run
/// and points at the expensive-computation cache by `polytope_key`.
#[derive(Clone, Serialize, Deserialize)]
pub struct AscentEventRow {
    pub event_id: String,
    pub dataset: String,
    pub run_id: String,
    pub seed_index: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phase: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iteration: Option<usize>,
    pub role: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t_fraction: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t_actual: Option<f64>,
    pub accepted_in_iteration: bool,
    pub became_run_final: bool,
    pub polytope_key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lineage_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub polytope_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub facet_count: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_sys: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub final_capacity: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub final_volume: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub final_sys: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_delta: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n_ascent_phases: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n_gradient_iters_total: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n_escape_overshoot: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n_escape_wiggle: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub best_strategy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_time_ms: Option<f64>,
}

/// Result of processing one seed: the summary row plus its trace rows.
pub struct SeedResult {
    pub summary: SummaryRow,
    pub trace: Vec<TraceRow>,
    pub computed_polytopes: Vec<ComputedPolytopeRow>,
    pub ascent_events: Vec<AscentEventRow>,
    pub final_polytope: SysLandscapePolytopeCache,
    pub final_record: PolytopeRecord,
}

use crate::SysLandscapePolytopeCache;
use serde::{Deserialize, Serialize};
use symplectic::database::PolytopeRecord;

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

/// Result of processing one seed: the summary row plus its trace rows.
pub struct SeedResult {
    pub summary: SummaryRow,
    pub trace: Vec<TraceRow>,
    pub final_polytope: SysLandscapePolytopeCache,
    pub final_record: PolytopeRecord,
}

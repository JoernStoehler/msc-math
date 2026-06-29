//! Shared row schema for local `sys(a0 + t u)` prediction experiments.
//!
//! These rows describe experiment identity. Computation caches such as active
//! `sys` payloads are acceleration artifacts keyed by states, not replacements
//! for these rows.

use serde::Serialize;

#[derive(Serialize)]
pub struct BasepointRow {
    pub basepoint_id: String,
    pub basepoint_kind: String,
    pub basepoint_state_id: String,
    pub basepoint_poly_id: String,
    pub selection_label: String,
    pub selection_rank_within_label: usize,
    pub branch_threshold_relative: f64,
    pub input_facet_count: usize,
    pub input_sys: f64,
}

#[derive(Clone, Debug, Serialize)]
pub struct StateRow {
    pub state_id: String,
    pub basepoint_id: String,
    pub role: String,
    pub parent_state_id: Option<String>,
    pub poly_id: Option<String>,
    pub direction_label: Option<String>,
    pub step: Option<f64>,
    pub status: String,
    pub dual_vertices_f64: Vec<[f64; 4]>,
}

#[derive(Serialize)]
pub struct PerturbationEventRow {
    pub event_id: String,
    pub basepoint_id: String,
    pub event_kind: String,
    pub base_state_id: String,
    pub target_state_id: Option<String>,
    pub direction_label: String,
    pub step: f64,
    pub status: String,
}

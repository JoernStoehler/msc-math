use serde::Serialize;
use symplectic::algorithms::OrbitSearchError;

#[derive(Debug, Serialize)]
pub(super) struct PredictionRow {
    pub(super) basepoint_name: String,
    pub(super) facet_count: usize,
    pub(super) direction_label: String,
    pub(super) step: f64,
    pub(super) status: String,
    pub(super) sys0: f64,
    pub(super) predicted_sys: f64,
    pub(super) recomputed_sys: Option<f64>,
    pub(super) abs_prediction_error: Option<f64>,
    pub(super) rel_prediction_error: Option<f64>,
    pub(super) base_best_sigma: Vec<usize>,
    pub(super) target_best_sigma: Option<Vec<usize>>,
    pub(super) target_best_sigma_in_base_active_set: Option<bool>,
    pub(super) target_best_sigma_in_base_candidate_window: Option<bool>,
    pub(super) target_best_sigma_base_transition_allowed: Option<bool>,
    pub(super) target_best_sigma_base_solve_status: Option<String>,
    pub(super) target_best_sigma_base_action_gap: Option<f64>,
    pub(super) target_best_sigma_transitions_opened: Option<Vec<[usize; 2]>>,
    pub(super) active_orbit_count: usize,
    pub(super) base_candidate_orbit_count: usize,
    pub(super) base_candidate_action_gap: f64,
    pub(super) active_action_spread: f64,
    pub(super) active_min_beta_margin: f64,
    pub(super) active_max_q_error_bound: f64,
}

#[derive(Debug)]
pub(crate) enum PredictionError {
    Geometry(String),
    Capacity(OrbitSearchError),
    Derivative(String),
    Io(std::io::Error),
    Json(serde_json::Error),
}

impl From<std::io::Error> for PredictionError {
    fn from(err: std::io::Error) -> Self {
        PredictionError::Io(err)
    }
}

impl From<serde_json::Error> for PredictionError {
    fn from(err: serde_json::Error) -> Self {
        PredictionError::Json(err)
    }
}

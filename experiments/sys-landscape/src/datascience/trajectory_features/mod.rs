//! Trajectory-feature assembly helpers for the datascience dataset stage.

mod feature_trajectory;

use crate::datascience::io::read_jsonl;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Deserialize)]
struct StateInputRow {
    state_id: String,
}

#[derive(Debug, Deserialize, Clone)]
struct StepEventInputRow {
    state_id: String,
    phase: usize,
    iteration: usize,
    step_type: String,
    t_fraction: f64,
    t_actual: f64,
    sys_before: f64,
    sys_after: f64,
    delta_sys: f64,
    gradient_norm: f64,
}

#[derive(Debug)]
pub struct TrajectoryFeatureInputRow {
    pub state_id: String,
    pub events: Vec<TrajectoryEvent>,
}

#[derive(Debug, Clone)]
pub struct TrajectoryEvent {
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

#[derive(Debug, Serialize)]
pub struct TrajectoryFeatureRow {
    pub state_id: String,
    pub trajectory_trace_available: f64,
    pub trajectory_event_count: usize,
    pub trajectory_phase_count: usize,
    pub trajectory_mean_iters_per_phase: f64,
    pub trajectory_overshoot_fraction: f64,
    pub trajectory_overshoot_15_fraction: f64,
    pub trajectory_overshoot_2_fraction: f64,
    pub trajectory_overshoot_3_fraction: f64,
    pub trajectory_t_fraction_mean: f64,
    pub trajectory_t_fraction_std: f64,
    pub trajectory_t_fraction_max: f64,
    pub trajectory_t_actual_mean: f64,
    pub trajectory_t_actual_std: f64,
    pub trajectory_t_actual_max: f64,
    pub trajectory_gradient_norm_mean: f64,
    pub trajectory_gradient_norm_std: f64,
    pub trajectory_gradient_norm_max: f64,
    pub trajectory_delta_share_top1: f64,
    pub trajectory_delta_share_top3: f64,
    pub trajectory_restart_drop_mean: f64,
    pub trajectory_restart_drop_max: f64,
    pub trajectory_restart_drop_fraction: f64,
    pub trajectory_efficiency_mean: f64,
    pub trajectory_efficiency_std: f64,
    pub trajectory_efficiency_max: f64,
}

pub fn load_inputs(core_tables_dir: &Path) -> Vec<TrajectoryFeatureInputRow> {
    let states: Vec<StateInputRow> = read_jsonl(&core_tables_dir.join("states.jsonl"));
    let step_events: Vec<StepEventInputRow> = read_jsonl(&core_tables_dir.join("step_events.jsonl"));

    let mut events_by_state = HashMap::<String, Vec<TrajectoryEvent>>::new();
    for event in step_events {
        events_by_state
            .entry(event.state_id.clone())
            .or_default()
            .push(TrajectoryEvent {
                phase: event.phase,
                iteration: event.iteration,
                step_type: event.step_type,
                t_fraction: event.t_fraction,
                t_actual: event.t_actual,
                sys_before: event.sys_before,
                sys_after: event.sys_after,
                delta_sys: event.delta_sys,
                gradient_norm: event.gradient_norm,
            });
    }

    let mut rows = states
        .into_iter()
        .map(|state| TrajectoryFeatureInputRow {
            events: events_by_state.remove(&state.state_id).unwrap_or_default(),
            state_id: state.state_id,
        })
        .collect::<Vec<_>>();
    rows.sort_by(|a, b| a.state_id.cmp(&b.state_id));
    rows
}

pub fn enrich_row(input: &TrajectoryFeatureInputRow) -> TrajectoryFeatureRow {
    feature_trajectory::compute(input)
}

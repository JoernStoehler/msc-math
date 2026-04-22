//! Build the observation table from provisioning rows and trace events.

use crate::load_caches::{LoadedObservationRow, TraceEvent};
use crate::rows::ObservationTableRow;

fn stats_or_zero(values: &[f64]) -> (f64, f64, f64) {
    if values.is_empty() {
        return (0.0, 0.0, 0.0);
    }
    let mean = values.iter().sum::<f64>() / values.len() as f64;
    let var = values
        .iter()
        .map(|value| {
            let delta = value - mean;
            delta * delta
        })
        .sum::<f64>()
        / values.len() as f64;
    let max = values.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    (mean, var.sqrt(), max)
}

fn zero_row(row: &LoadedObservationRow) -> ObservationTableRow {
    ObservationTableRow {
        observation_id: row.observation_id.clone(),
        poly_id: row.poly_id.clone(),
        dataset: row.dataset.clone(),
        family: row.family.clone(),
        role: row.role.clone(),
        search_space: row.search_space.clone(),
        optimizer: row.optimizer.clone(),
        backend: row.backend.clone(),
        source_name: row.source_name.clone(),
        root_group_id: row.root_group_id.clone(),
        seed_index: row.seed_index,
        lineage_id: row.lineage_id.clone(),
        parent_observation_id: row.parent_observation_id.clone(),
        rq: row.rq.clone(),
        path: row.path.clone(),
        starting_f: row.starting_f,
        starting_sys: row.starting_sys,
        reported_final_sys: row.reported_final_sys,
        reported_delta: row.reported_delta,
        sys_after_addition: row.sys_after_addition,
        n_iterations: row.n_iterations,
        n_phases: row.n_phases,
        best_strategy: row.best_strategy.clone(),
        n_escape_overshoot: row.n_escape_overshoot,
        n_escape_wiggle: row.n_escape_wiggle,
        placement_direction: row.placement_direction,
        facet_remained_active: row.facet_remained_active,
        total_time_ms: row.total_time_ms,
        trajectory_trace_available: 0.0,
        trajectory_event_count: 0,
        trajectory_phase_count: 0,
        trajectory_mean_iters_per_phase: 0.0,
        trajectory_overshoot_fraction: 0.0,
        trajectory_overshoot_15_fraction: 0.0,
        trajectory_overshoot_2_fraction: 0.0,
        trajectory_overshoot_3_fraction: 0.0,
        trajectory_t_fraction_mean: 0.0,
        trajectory_t_fraction_std: 0.0,
        trajectory_t_fraction_max: 0.0,
        trajectory_t_actual_mean: 0.0,
        trajectory_t_actual_std: 0.0,
        trajectory_t_actual_max: 0.0,
        trajectory_gradient_norm_mean: 0.0,
        trajectory_gradient_norm_std: 0.0,
        trajectory_gradient_norm_max: 0.0,
        trajectory_delta_share_top1: 0.0,
        trajectory_delta_share_top3: 0.0,
        trajectory_restart_drop_mean: 0.0,
        trajectory_restart_drop_max: 0.0,
        trajectory_restart_drop_fraction: 0.0,
        trajectory_efficiency_mean: 0.0,
        trajectory_efficiency_std: 0.0,
        trajectory_efficiency_max: 0.0,
    }
}

fn enrich_trace(row: &LoadedObservationRow, events: &[TraceEvent]) -> ObservationTableRow {
    if events.is_empty() {
        return zero_row(row);
    }

    let mut sorted = events.to_vec();
    sorted.sort_by(|a, b| a.phase.cmp(&b.phase).then(a.iteration.cmp(&b.iteration)));

    let event_count = sorted.len();
    let phase_count = sorted.last().map(|event| event.phase + 1).unwrap_or(0);
    let mean_iters_per_phase = if phase_count == 0 {
        0.0
    } else {
        event_count as f64 / phase_count as f64
    };

    let overshoot_15_count = sorted
        .iter()
        .filter(|event| event.step_type == "overshoot_1.5x")
        .count();
    let overshoot_2_count = sorted
        .iter()
        .filter(|event| event.step_type == "overshoot_2x")
        .count();
    let overshoot_3_count = sorted
        .iter()
        .filter(|event| event.step_type == "overshoot_3x")
        .count();
    let overshoot_count = overshoot_15_count + overshoot_2_count + overshoot_3_count;

    let t_fractions = sorted.iter().map(|event| event.t_fraction).collect::<Vec<_>>();
    let t_actuals = sorted.iter().map(|event| event.t_actual).collect::<Vec<_>>();
    let gradient_norms = sorted
        .iter()
        .map(|event| event.gradient_norm)
        .collect::<Vec<_>>();
    let deltas = sorted.iter().map(|event| event.delta_sys).collect::<Vec<_>>();
    let efficiencies = sorted
        .iter()
        .map(|event| event.delta_sys / (event.t_actual * event.gradient_norm).max(1e-12))
        .collect::<Vec<_>>();

    let (trajectory_t_fraction_mean, trajectory_t_fraction_std, trajectory_t_fraction_max) =
        stats_or_zero(&t_fractions);
    let (trajectory_t_actual_mean, trajectory_t_actual_std, trajectory_t_actual_max) =
        stats_or_zero(&t_actuals);
    let (trajectory_gradient_norm_mean, trajectory_gradient_norm_std, trajectory_gradient_norm_max) =
        stats_or_zero(&gradient_norms);
    let (trajectory_efficiency_mean, trajectory_efficiency_std, trajectory_efficiency_max) =
        stats_or_zero(&efficiencies);

    let total_delta = deltas.iter().sum::<f64>();
    let mut sorted_deltas = deltas.clone();
    sorted_deltas.sort_by(|a, b| b.total_cmp(a));
    let trajectory_delta_share_top1 = if total_delta > 0.0 {
        sorted_deltas.first().copied().unwrap_or(0.0) / total_delta
    } else {
        0.0
    };
    let trajectory_delta_share_top3 = if total_delta > 0.0 {
        sorted_deltas.iter().take(3).sum::<f64>() / total_delta
    } else {
        0.0
    };

    let mut restart_drops = Vec::new();
    let mut previous_phase = None;
    let mut previous_after = 0.0;
    for event in &sorted {
        if let Some(phase) = previous_phase {
            if event.phase != phase {
                restart_drops.push((previous_after - event.sys_before).max(0.0));
            }
        }
        previous_phase = Some(event.phase);
        previous_after = event.sys_after;
    }
    let restart_drop_sum = restart_drops.iter().sum::<f64>();
    let (trajectory_restart_drop_mean, _, trajectory_restart_drop_max) =
        stats_or_zero(&restart_drops);
    let trajectory_restart_drop_fraction = if total_delta > 0.0 {
        restart_drop_sum / total_delta
    } else {
        0.0
    };

    ObservationTableRow {
        observation_id: row.observation_id.clone(),
        poly_id: row.poly_id.clone(),
        dataset: row.dataset.clone(),
        family: row.family.clone(),
        role: row.role.clone(),
        search_space: row.search_space.clone(),
        optimizer: row.optimizer.clone(),
        backend: row.backend.clone(),
        source_name: row.source_name.clone(),
        root_group_id: row.root_group_id.clone(),
        seed_index: row.seed_index,
        lineage_id: row.lineage_id.clone(),
        parent_observation_id: row.parent_observation_id.clone(),
        rq: row.rq.clone(),
        path: row.path.clone(),
        starting_f: row.starting_f,
        starting_sys: row.starting_sys,
        reported_final_sys: row.reported_final_sys,
        reported_delta: row.reported_delta,
        sys_after_addition: row.sys_after_addition,
        n_iterations: row.n_iterations,
        n_phases: row.n_phases,
        best_strategy: row.best_strategy.clone(),
        n_escape_overshoot: row.n_escape_overshoot,
        n_escape_wiggle: row.n_escape_wiggle,
        placement_direction: row.placement_direction,
        facet_remained_active: row.facet_remained_active,
        total_time_ms: row.total_time_ms,
        trajectory_trace_available: 1.0,
        trajectory_event_count: event_count,
        trajectory_phase_count: phase_count,
        trajectory_mean_iters_per_phase: mean_iters_per_phase,
        trajectory_overshoot_fraction: overshoot_count as f64 / event_count as f64,
        trajectory_overshoot_15_fraction: overshoot_15_count as f64 / event_count as f64,
        trajectory_overshoot_2_fraction: overshoot_2_count as f64 / event_count as f64,
        trajectory_overshoot_3_fraction: overshoot_3_count as f64 / event_count as f64,
        trajectory_t_fraction_mean,
        trajectory_t_fraction_std,
        trajectory_t_fraction_max,
        trajectory_t_actual_mean,
        trajectory_t_actual_std,
        trajectory_t_actual_max,
        trajectory_gradient_norm_mean,
        trajectory_gradient_norm_std,
        trajectory_gradient_norm_max,
        trajectory_delta_share_top1,
        trajectory_delta_share_top3,
        trajectory_restart_drop_mean,
        trajectory_restart_drop_max,
        trajectory_restart_drop_fraction,
        trajectory_efficiency_mean,
        trajectory_efficiency_std,
        trajectory_efficiency_max,
    }
}

pub fn build_observation_table(rows: &[LoadedObservationRow]) -> Vec<ObservationTableRow> {
    rows.iter()
        .map(|row| enrich_trace(row, &row.trace_events))
        .collect()
}

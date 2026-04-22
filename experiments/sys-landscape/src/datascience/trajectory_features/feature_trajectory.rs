//! Cheap trajectory-summary features for datascience consumers.

use super::{TrajectoryFeatureInputRow, TrajectoryFeatureRow};

fn stats_or_zero(values: &[f64]) -> (f64, f64, f64) {
    if values.is_empty() {
        return (0.0, 0.0, 0.0);
    }
    let mean = values.iter().sum::<f64>() / values.len() as f64;
    let var = values
        .iter()
        .map(|v| {
            let d = v - mean;
            d * d
        })
        .sum::<f64>()
        / values.len() as f64;
    let max = values.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    (mean, var.sqrt(), max)
}

fn zero_row(state_id: &str) -> TrajectoryFeatureRow {
    TrajectoryFeatureRow {
        state_id: state_id.to_string(),
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

pub fn compute(input: &TrajectoryFeatureInputRow) -> TrajectoryFeatureRow {
    if input.events.is_empty() {
        return zero_row(&input.state_id);
    }

    let mut sorted = input.events.clone();
    sorted.sort_by(|a, b| a.phase.cmp(&b.phase).then(a.iteration.cmp(&b.iteration)));

    let event_count = sorted.len();
    let phase_count = sorted
        .last()
        .map(|row| row.phase + 1)
        .expect("non-empty trajectory events");
    let mean_iters_per_phase = event_count as f64 / phase_count as f64;

    let overshoot_15_count = sorted
        .iter()
        .filter(|row| row.step_type == "overshoot_1.5x")
        .count();
    let overshoot_2_count = sorted
        .iter()
        .filter(|row| row.step_type == "overshoot_2x")
        .count();
    let overshoot_3_count = sorted
        .iter()
        .filter(|row| row.step_type == "overshoot_3x")
        .count();
    let overshoot_count = overshoot_15_count + overshoot_2_count + overshoot_3_count;

    let t_fractions = sorted.iter().map(|row| row.t_fraction).collect::<Vec<_>>();
    let t_actuals = sorted.iter().map(|row| row.t_actual).collect::<Vec<_>>();
    let gradient_norms = sorted
        .iter()
        .map(|row| row.gradient_norm)
        .collect::<Vec<_>>();
    let deltas = sorted.iter().map(|row| row.delta_sys).collect::<Vec<_>>();
    let efficiencies = sorted
        .iter()
        .map(|row| row.delta_sys / (row.t_actual * row.gradient_norm).max(1e-12))
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
    let mut prev_phase: Option<usize> = None;
    let mut prev_after = 0.0;
    for row in &sorted {
        if let Some(prev) = prev_phase {
            if row.phase != prev {
                let drop = (prev_after - row.sys_before).max(0.0);
                restart_drops.push(drop);
            }
        }
        prev_phase = Some(row.phase);
        prev_after = row.sys_after;
    }
    let restart_drop_sum = restart_drops.iter().sum::<f64>();
    let (trajectory_restart_drop_mean, _trajectory_restart_drop_std, trajectory_restart_drop_max) =
        stats_or_zero(&restart_drops);
    let trajectory_restart_drop_fraction = if total_delta > 0.0 {
        restart_drop_sum / total_delta
    } else {
        0.0
    };

    TrajectoryFeatureRow {
        state_id: input.state_id.clone(),
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

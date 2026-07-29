use crate::algorithm::{EvaluatedProposal, Optimizer, Proposal, TellOutcome};
use crate::branch_model::{winning_gradient, SliceMode};
use crate::evaluator::Evaluation;
use crate::quotient::{add_flat_direction, flatten, l2_norm, quotient_basis};
use crate::schedule::{DistanceSchedule, DistanceScheduleSpec};
use nalgebra::DVector;
use serde_json::json;
use std::time::Instant;

pub struct SafeguardedGradient {
    current: Evaluation,
    schedule: DistanceSchedule,
    slice_mode: SliceMode,
    pending_predicted_delta: Option<f64>,
    done: Option<String>,
}

impl SafeguardedGradient {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        initial: &Evaluation,
        schedule: DistanceScheduleSpec,
        slice_mode: SliceMode,
    ) -> Result<Self, String> {
        Ok(Self {
            current: initial.clone(),
            schedule: DistanceSchedule::new(schedule),
            slice_mode,
            pending_predicted_delta: None,
            done: None,
        })
    }
}

impl Optimizer for SafeguardedGradient {
    fn ask(&mut self, _remaining_budget: usize) -> Result<Vec<Proposal>, String> {
        if self.done.is_some() {
            return Ok(Vec::new());
        }
        let started = Instant::now();
        let gradient = winning_gradient(&self.current)?;
        let gradient_flat = DVector::from_vec(flatten(&gradient));
        let mut direction = gradient_flat.clone();
        let quotient_ms;
        if self.slice_mode == SliceMode::SymmetryTransverse {
            let quotient_started = Instant::now();
            for axis in quotient_basis(&self.current.duals)?.orbit_basis {
                direction -= &axis * axis.dot(&direction);
            }
            quotient_ms = quotient_started.elapsed().as_secs_f64() * 1000.0;
        } else {
            quotient_ms = 0.0;
        }
        let norm = direction.norm();
        if !norm.is_finite() || norm <= 1.0e-14 {
            self.done = Some("zero_projected_gradient".to_string());
            return Ok(Vec::new());
        }
        direction /= norm;
        let normalized_distance = self.schedule.current();
        let absolute_distance = normalized_distance * l2_norm(&self.current.duals);
        let predicted_delta = absolute_distance * gradient_flat.dot(&direction);
        self.pending_predicted_delta = Some(predicted_delta);
        let target = add_flat_direction(&self.current.duals, &direction, absolute_distance);
        let total_ms = started.elapsed().as_secs_f64() * 1000.0;
        Ok(vec![Proposal {
            duals: target,
            baseline_evaluation_id: Some(self.current.row.evaluation_id.clone()),
            geometric_reference_kind: Some("current_state".to_string()),
            geometric_reference_duals: Some(self.current.duals.clone()),
            fields: json!({
                "scheduled_normalized_distance": normalized_distance,
                "schedule_kind": self.schedule.kind(),
                "slice_mode": self.slice_mode,
                "direction_kind": "normalized_winning_branch_gradient",
                "predicted_delta": predicted_delta,
                "base_sigma": self.current.row.winning_sigma,
                "phase_ms": {
                    "quotient_slice": quotient_ms,
                    "branch_derivative_and_direction": total_ms - quotient_ms,
                },
            }),
        }])
    }

    fn tell(&mut self, observations: &[EvaluatedProposal]) -> Result<TellOutcome, String> {
        let observation = observations
            .first()
            .ok_or("safeguarded gradient expected one observation")?;
        if observations.len() != 1 {
            return Err("safeguarded gradient received multiple observations".to_string());
        }
        let before = self.current.row.sys.ok_or("current evaluation lacks sys")?;
        let after = observation.evaluation.row.sys;
        let observed_delta = after.map(|value| value - before);
        let accepted = observation.evaluation.row.usable_by_optimizer
            && observed_delta.is_some_and(|delta| delta > 0.0);
        let distance_before = self.schedule.current();
        if accepted {
            self.current = observation.evaluation.clone();
        }
        self.schedule.observe(accepted);
        if self.schedule.is_done() {
            self.done = Some("distance_schedule_finished".to_string());
        }
        Ok(TellOutcome {
            selected: accepted.then_some((0, 1.0)).into_iter().collect(),
            stop_reason: self.done.clone(),
            fields: json!({
                "accepted": accepted,
                "observed_delta": observed_delta,
                "predicted_delta": self.pending_predicted_delta,
                "prediction_error": observed_delta.zip(self.pending_predicted_delta).map(|(observed, predicted)| observed - predicted),
                "scheduled_distance_before": distance_before,
                "scheduled_distance_after": self.schedule.current(),
            }),
        })
    }

    fn is_done(&self) -> Option<String> {
        self.done.clone()
    }
}

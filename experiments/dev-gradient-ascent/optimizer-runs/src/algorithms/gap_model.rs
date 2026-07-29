use crate::algorithm::{EvaluatedProposal, Optimizer, Proposal, TellOutcome};
use crate::branch_model::{
    BranchExtensionMode, BranchModel, BranchModelConfig, DirectionSolution, NormMode, SliceMode,
};
use crate::evaluator::Evaluation;
use crate::quotient::{add_flat_direction, l2_norm};
use crate::schedule::{DistanceSchedule, DistanceScheduleSpec};
use serde_json::json;

pub struct GapModel {
    current: Evaluation,
    candidate_window_relative: f64,
    extension_mode: BranchExtensionMode,
    extension_reachability_scale: f64,
    schedule: DistanceSchedule,
    slice_mode: SliceMode,
    norm_mode: NormMode,
    require_positive_prediction: bool,
    pending: Option<DirectionSolution>,
    done: Option<String>,
}

impl GapModel {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        initial: &Evaluation,
        candidate_window_relative: f64,
        extension_mode: BranchExtensionMode,
        extension_reachability_scale: f64,
        schedule: DistanceScheduleSpec,
        slice_mode: SliceMode,
        norm_mode: NormMode,
        require_positive_prediction: bool,
    ) -> Result<Self, String> {
        if !candidate_window_relative.is_finite() || candidate_window_relative < 0.0 {
            return Err("candidate window must be nonnegative and finite".to_string());
        }
        if !extension_reachability_scale.is_finite() || extension_reachability_scale < 0.0 {
            return Err("extension reachability scale must be nonnegative and finite".to_string());
        }
        Ok(Self {
            current: initial.clone(),
            candidate_window_relative,
            extension_mode,
            extension_reachability_scale,
            schedule: DistanceSchedule::new(schedule),
            slice_mode,
            norm_mode,
            require_positive_prediction,
            pending: None,
            done: None,
        })
    }

    fn solve_current_model(&mut self) -> Result<(BranchModel, DirectionSolution), String> {
        let model = BranchModel::build(
            &self.current,
            &BranchModelConfig {
                candidate_window_relative: self.candidate_window_relative,
                extension_mode: self.extension_mode,
            },
        )?;
        let solution = match self.norm_mode {
            NormMode::BoxLinf => {
                let dimension = self.current.duals.len() * 4;
                model.solve_box(
                    &self.current.duals,
                    self.schedule.current() * l2_norm(&self.current.duals)
                        / (dimension as f64).sqrt(),
                    self.slice_mode,
                    self.extension_reachability_scale,
                )?
            }
            NormMode::EuclideanL2 => model.solve_euclidean(
                &self.current.duals,
                self.schedule.current(),
                self.slice_mode,
                self.extension_reachability_scale,
            )?,
        };
        Ok((model, solution))
    }
}

impl Optimizer for GapModel {
    fn ask(&mut self, _remaining_budget: usize) -> Result<Vec<Proposal>, String> {
        if self.done.is_some() {
            return Ok(Vec::new());
        }
        let mut internal_resolves = 0usize;
        let (model, solution) = loop {
            let (model, solution) = self.solve_current_model()?;
            if !self.require_positive_prediction || solution.predicted_delta > 0.0 {
                break (model, solution);
            }
            let continued = self.schedule.contract_without_evaluation();
            internal_resolves += 1;
            if !continued {
                self.done = Some(
                    if self.schedule.kind() == "adaptive" {
                        "no_positive_model_move_above_minimum_distance"
                    } else {
                        "nonpositive_model_prediction_under_fixed_schedule"
                    }
                    .to_string(),
                );
                return Ok(Vec::new());
            }
        };
        let target = add_flat_direction(
            &self.current.duals,
            &nalgebra::DVector::from_vec(solution.displacement_flat.clone()),
            1.0,
        );
        let fields = json!({
            "scheduled_normalized_distance": self.schedule.current(),
            "schedule_kind": self.schedule.kind(),
            "candidate_window_relative": self.candidate_window_relative,
            "branch_extension_mode": self.extension_mode,
            "extension_reachability_scale": self.extension_reachability_scale,
            "slice_mode": self.slice_mode,
            "norm_mode": self.norm_mode,
            "predicted_delta": solution.predicted_delta,
            "predicted_winning_sigma": solution.predicted_winning_sigma,
            "candidate_branch_count": solution.candidate_branch_count,
            "extended_branch_count": model.extended.len(),
            "negative_beta_extended_branch_count": model.extended
                .iter()
                .filter(|branch| branch.beta_margin < 0.0)
                .count(),
            "minimum_extended_beta_margin": model.extended
                .iter()
                .map(|branch| branch.beta_margin)
                .min_by(f64::total_cmp),
            "reachable_extended_branch_count": solution.reachable_extended_branch_count,
            "represented_branch_count": solution.represented_branch_count,
            "internal_radius_resolves": internal_resolves,
            "displacement_flat": solution.displacement_flat,
            "displacement_inf_norm": solution.displacement_flat
                .iter()
                .map(|entry| entry.abs())
                .fold(0.0_f64, f64::max),
            "phase_ms": {
                "candidate_window_search": model.timing.candidate_search_ms,
                "branch_derivative": model.timing.derivative_ms,
                "branch_extension_enumeration": model.timing.extension_enumeration_ms,
                "model_solve": solution.solve_ms,
            },
        });
        self.pending = Some(solution);
        Ok(vec![Proposal {
            duals: target,
            baseline_evaluation_id: Some(self.current.row.evaluation_id.clone()),
            geometric_reference_kind: Some("current_state".to_string()),
            geometric_reference_duals: Some(self.current.duals.clone()),
            fields,
        }])
    }

    fn tell(&mut self, observations: &[EvaluatedProposal]) -> Result<TellOutcome, String> {
        let observation = observations
            .first()
            .ok_or("gap model expected one observation")?;
        if observations.len() != 1 {
            return Err("gap model received multiple observations".to_string());
        }
        let pending = self
            .pending
            .take()
            .ok_or("gap model tell called without a pending proposal")?;
        let before = self.current.row.sys.ok_or("current evaluation lacks sys")?;
        let after = observation.evaluation.row.sys;
        let observed_delta = after.map(|value| value - before);
        let accepted = observation.evaluation.row.usable_by_optimizer
            && observed_delta.is_some_and(|delta| delta > 0.0);
        let radius_before = self.schedule.current();
        if accepted {
            self.current = observation.evaluation.clone();
        }
        self.schedule.observe(accepted);
        if self.schedule.is_done() {
            self.done = Some("distance_schedule_finished".to_string());
        }
        let target_sigma = observation.evaluation.row.winning_sigma.clone();
        Ok(TellOutcome {
            selected: accepted.then_some((0, 1.0)).into_iter().collect(),
            stop_reason: self.done.clone(),
            fields: json!({
                "accepted": accepted,
                "observed_delta": observed_delta,
                "predicted_delta": pending.predicted_delta,
                "prediction_error": observed_delta.map(|observed| observed - pending.predicted_delta),
                "predicted_winning_sigma": pending.predicted_winning_sigma,
                "target_winning_sigma": target_sigma,
                "winner_prediction_correct": target_sigma
                    .as_ref()
                    .is_some_and(|sigma| sigma == &pending.predicted_winning_sigma),
                "scheduled_distance_before": radius_before,
                "scheduled_distance_after": self.schedule.current(),
            }),
        })
    }

    fn is_done(&self) -> Option<String> {
        self.done.clone()
    }
}

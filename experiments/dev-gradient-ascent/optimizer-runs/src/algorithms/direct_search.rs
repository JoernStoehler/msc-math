use crate::algorithm::{EvaluatedProposal, Optimizer, Proposal, TellOutcome};
use crate::evaluator::Evaluation;
use crate::quotient::{add_flat_direction, l2_norm, quotient_basis};
use crate::schema::AlgorithmStateRow;
use serde_json::json;

pub struct DirectSearch {
    incumbent: Evaluation,
    radius: f64,
    expansion: f64,
    contraction: f64,
    minimum_radius: f64,
    done: Option<String>,
    pending_complete_poll: bool,
}

impl DirectSearch {
    pub fn new(
        initial: &Evaluation,
        radius: f64,
        expansion: f64,
        contraction: f64,
        minimum_radius: f64,
    ) -> Result<Self, String> {
        if !initial.row.usable_by_optimizer {
            return Err("direct search requires a usable initial point".to_string());
        }
        Ok(Self {
            incumbent: initial.clone(),
            radius,
            expansion,
            contraction,
            minimum_radius,
            done: None,
            pending_complete_poll: false,
        })
    }
}

impl Optimizer for DirectSearch {
    fn ask(&mut self, remaining_budget: usize) -> Result<Vec<Proposal>, String> {
        if self.done.is_some() || remaining_budget == 0 {
            return Ok(Vec::new());
        }
        let quotient = quotient_basis(&self.incumbent.duals)?;
        let full_count = quotient.slice_basis.len() * 2;
        let count = full_count.min(remaining_budget);
        self.pending_complete_poll = count == full_count;
        let scale = self.radius * l2_norm(&self.incumbent.duals);
        let mut proposals = Vec::with_capacity(count);
        for proposal_index in 0..count {
            let axis_index = proposal_index / 2;
            let sign = if proposal_index % 2 == 0 { 1.0 } else { -1.0 };
            proposals.push(Proposal {
                duals: add_flat_direction(
                    &self.incumbent.duals,
                    &quotient.slice_basis[axis_index],
                    sign * scale,
                ),
                baseline_evaluation_id: Some(self.incumbent.row.evaluation_id.clone()),
                geometric_reference_kind: Some("incumbent".to_string()),
                geometric_reference_duals: Some(self.incumbent.duals.clone()),
                fields: json!({
                    "axis_index": axis_index,
                    "sign": sign as i8,
                    "radius": self.radius,
                    "complete_poll": self.pending_complete_poll,
                    "slice_dimension": quotient.slice_basis.len(),
                }),
            });
        }
        Ok(proposals)
    }

    fn tell(&mut self, observations: &[EvaluatedProposal]) -> Result<TellOutcome, String> {
        let radius_before = self.radius;
        let incumbent_sys = self
            .incumbent
            .row
            .sys
            .ok_or_else(|| "incumbent lacks sys".to_string())?;
        let best = observations
            .iter()
            .enumerate()
            .filter_map(|(index, observation)| {
                observation
                    .evaluation
                    .row
                    .usable_by_optimizer
                    .then_some((index, observation.evaluation.row.sys?))
            })
            .max_by(|left, right| left.1.total_cmp(&right.1));
        let mut selected = Vec::new();
        let accepted = best.is_some_and(|(_, sys)| sys > incumbent_sys);
        if let Some((index, _)) = best.filter(|(_, sys)| *sys > incumbent_sys) {
            self.incumbent = observations[index].evaluation.clone();
            self.radius *= self.expansion;
            selected.push((index, 1.0));
        } else {
            self.radius *= self.contraction;
        }
        if self.radius < self.minimum_radius {
            self.done = Some("minimum_radius".to_string());
        }
        Ok(TellOutcome {
            selected,
            stop_reason: self.done.clone(),
            fields: json!({
                "accepted": accepted,
                "complete_poll": self.pending_complete_poll,
                "radius_before": radius_before,
                "radius_after": self.radius,
                "incumbent_sys_before": incumbent_sys,
                "incumbent_sys_after": self.incumbent.row.sys,
            }),
        })
    }

    fn is_done(&self) -> Option<String> {
        self.done.clone()
    }

    fn algorithm_state(&self) -> AlgorithmStateRow {
        AlgorithmStateRow::EvaluatedPoint {
            evaluation_id: self.incumbent.row.evaluation_id.clone(),
        }
    }
}

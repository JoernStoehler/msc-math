use crate::algorithm::{EvaluatedProposal, Optimizer, Proposal, TellOutcome};
use crate::branch_model::winning_gradient;
use crate::evaluator::Evaluation;
use crate::schema::AlgorithmStateRow;
use serde_json::json;
use std::time::Instant;

pub struct LiteralGradient {
    current: Evaluation,
    rate: f64,
    done: Option<String>,
}

impl LiteralGradient {
    pub fn new(initial: &Evaluation, rate: f64) -> Result<Self, String> {
        if !rate.is_finite() || rate <= 0.0 {
            return Err("literal gradient rate must be positive and finite".to_string());
        }
        Ok(Self {
            current: initial.clone(),
            rate,
            done: None,
        })
    }
}

impl Optimizer for LiteralGradient {
    fn ask(&mut self, _remaining_budget: usize) -> Result<Vec<Proposal>, String> {
        if self.done.is_some() {
            return Ok(Vec::new());
        }
        let started = Instant::now();
        let gradient = winning_gradient(&self.current)?;
        let target = self
            .current
            .duals
            .iter()
            .zip(&gradient)
            .map(|(base, derivative)| base + self.rate * derivative)
            .collect::<Vec<_>>();
        let derivative_ms = started.elapsed().as_secs_f64() * 1000.0;
        Ok(vec![Proposal {
            duals: target,
            baseline_evaluation_id: Some(self.current.row.evaluation_id.clone()),
            geometric_reference_kind: Some("current_state".to_string()),
            geometric_reference_duals: Some(self.current.duals.clone()),
            fields: json!({
                "rate": self.rate,
                "direction_kind": "raw_winning_branch_gradient",
                "base_sigma": self.current.row.winning_sigma,
                "phase_ms": {"branch_derivative": derivative_ms},
            }),
        }])
    }

    fn tell(&mut self, observations: &[EvaluatedProposal]) -> Result<TellOutcome, String> {
        let observation = observations
            .first()
            .ok_or("literal gradient expected one observation")?;
        if observations.len() != 1 {
            return Err("literal gradient received multiple observations".to_string());
        }
        let before = self.current.row.sys.ok_or("current evaluation lacks sys")?;
        let after = observation.evaluation.row.sys;
        if !observation.evaluation.row.usable_by_optimizer || after.is_none() {
            self.done = Some("invalid_target".to_string());
            return Ok(TellOutcome {
                selected: Vec::new(),
                stop_reason: self.done.clone(),
                fields: json!({
                    "accepted": false,
                    "observed_delta": null,
                    "reason": "invalid_target",
                }),
            });
        }
        let observed_delta = after.expect("checked") - before;
        self.current = observation.evaluation.clone();
        Ok(TellOutcome {
            selected: vec![(0, 1.0)],
            stop_reason: None,
            fields: json!({
                "accepted": true,
                "observed_delta": observed_delta,
                "accepted_even_if_decreasing": true,
            }),
        })
    }

    fn is_done(&self) -> Option<String> {
        self.done.clone()
    }

    fn algorithm_state(&self) -> AlgorithmStateRow {
        AlgorithmStateRow::EvaluatedPoint {
            evaluation_id: self.current.row.evaluation_id.clone(),
        }
    }
}

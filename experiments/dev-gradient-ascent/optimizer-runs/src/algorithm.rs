use crate::evaluator::Evaluation;
use crate::schema::AlgorithmStateRow;
use nalgebra::Vector4;
use serde_json::Value;

#[derive(Clone, Debug)]
pub struct Proposal {
    pub duals: Vec<Vector4<f64>>,
    pub baseline_evaluation_id: Option<String>,
    pub geometric_reference_kind: Option<String>,
    pub geometric_reference_duals: Option<Vec<Vector4<f64>>>,
    pub fields: Value,
}

#[derive(Clone, Debug)]
pub struct EvaluatedProposal {
    pub proposal_id: String,
    pub evaluation: Evaluation,
}

#[derive(Clone, Debug, Default)]
pub struct TellOutcome {
    pub selected: Vec<(usize, f64)>,
    pub stop_reason: Option<String>,
    pub fields: Value,
}

pub trait Optimizer {
    fn ask(&mut self, remaining_budget: usize) -> Result<Vec<Proposal>, String>;
    fn tell(&mut self, observations: &[EvaluatedProposal]) -> Result<TellOutcome, String>;
    fn is_done(&self) -> Option<String>;

    fn algorithm_state(&self) -> AlgorithmStateRow {
        AlgorithmStateRow::NoSingleCurrentState
    }
}

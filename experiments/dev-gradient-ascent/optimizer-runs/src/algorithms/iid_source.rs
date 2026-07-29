use crate::algorithm::{EvaluatedProposal, Optimizer, Proposal, TellOutcome};
use crate::evaluator::Evaluation;
use crate::quotient::unflatten;
use crate::schema::SourcePoint;
use rand::seq::SliceRandom;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use serde_json::json;

pub struct IidSource {
    candidates: Vec<SourcePoint>,
    cursor: usize,
    batch_size: usize,
    exhausted: bool,
}

impl IidSource {
    pub fn new(
        seed: u64,
        batch_size: usize,
        initial: &Evaluation,
        source_pool: &[SourcePoint],
    ) -> Result<Self, String> {
        let mut candidates = source_pool
            .iter()
            .filter(|point| {
                point.facet_count == initial.duals.len() && point.dual_flat != initial.row.dual_flat
            })
            .cloned()
            .collect::<Vec<_>>();
        let mut rng = ChaCha8Rng::seed_from_u64(seed);
        candidates.shuffle(&mut rng);
        if candidates.is_empty() {
            return Err("iid source pool has no candidate distinct from start".to_string());
        }
        Ok(Self {
            candidates,
            cursor: 0,
            batch_size,
            exhausted: false,
        })
    }
}

impl Optimizer for IidSource {
    fn ask(&mut self, remaining_budget: usize) -> Result<Vec<Proposal>, String> {
        let count = self
            .batch_size
            .min(remaining_budget)
            .min(self.candidates.len().saturating_sub(self.cursor));
        if count == 0 {
            self.exhausted = true;
            return Ok(Vec::new());
        }
        let proposals = self.candidates[self.cursor..self.cursor + count]
            .iter()
            .map(|point| {
                Ok(Proposal {
                    duals: unflatten(&point.dual_flat)?,
                    baseline_evaluation_id: None,
                    geometric_reference_kind: None,
                    geometric_reference_duals: None,
                    fields: json!({"source_point_id": point.name}),
                })
            })
            .collect::<Result<Vec<_>, String>>()?;
        self.cursor += count;
        self.exhausted = self.cursor == self.candidates.len();
        Ok(proposals)
    }

    fn tell(&mut self, observations: &[EvaluatedProposal]) -> Result<TellOutcome, String> {
        let selected = observations
            .iter()
            .enumerate()
            .filter_map(|(index, observation)| {
                observation
                    .evaluation
                    .row
                    .usable_by_optimizer
                    .then_some((index, observation.evaluation.row.sys?))
            })
            .max_by(|left, right| left.1.total_cmp(&right.1))
            .map(|(index, _)| vec![(index, 1.0)])
            .unwrap_or_default();
        Ok(TellOutcome {
            selected,
            stop_reason: self.exhausted.then(|| "source_pool_exhausted".to_string()),
            fields: json!({"source_cursor": self.cursor}),
        })
    }

    fn is_done(&self) -> Option<String> {
        self.exhausted.then(|| "source_pool_exhausted".to_string())
    }
}

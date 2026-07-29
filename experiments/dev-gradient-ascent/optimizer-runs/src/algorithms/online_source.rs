use crate::algorithm::{EvaluatedProposal, Optimizer, Proposal, TellOutcome};
use crate::evaluator::Evaluation;
use euclidean_polytopes::sample_random_dual_vertices_f64;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use serde_json::json;
use std::time::Instant;

/// Independent raw source attempts generated online.
///
/// The evaluator, rather than this optimizer, decides whether each raw
/// candidate is a valid polytope. Thus invalid generation attempts remain
/// visible and charged.
pub struct OnlineSource {
    rng: ChaCha8Rng,
    batch_size: usize,
    facet_count: usize,
    height_min: f64,
    height_max: f64,
    incumbent_sys: f64,
    attempt: usize,
    pending_generation_ms: f64,
}

impl OnlineSource {
    pub fn new(
        seed: u64,
        batch_size: usize,
        facet_count: usize,
        height_min: f64,
        height_max: f64,
        initial: &Evaluation,
    ) -> Result<Self, String> {
        let incumbent_sys = initial.row.sys.ok_or("initial evaluation lacks sys")?;
        Ok(Self {
            rng: ChaCha8Rng::seed_from_u64(seed),
            batch_size,
            facet_count,
            height_min,
            height_max,
            incumbent_sys,
            attempt: 0,
            pending_generation_ms: 0.0,
        })
    }
}

impl Optimizer for OnlineSource {
    fn ask(&mut self, remaining_budget: usize) -> Result<Vec<Proposal>, String> {
        let count = self.batch_size.min(remaining_budget);
        let started = Instant::now();
        let proposals = (0..count)
            .map(|_| {
                let attempt = self.attempt;
                self.attempt += 1;
                Proposal {
                    duals: sample_random_dual_vertices_f64(
                        self.facet_count,
                        self.height_min,
                        self.height_max,
                        &mut self.rng,
                    ),
                    baseline_evaluation_id: None,
                    geometric_reference_kind: None,
                    geometric_reference_duals: None,
                    fields: json!({
                        "source_attempt": attempt,
                        "facet_count": self.facet_count,
                        "height_min": self.height_min,
                        "height_max": self.height_max,
                    }),
                }
            })
            .collect();
        self.pending_generation_ms = started.elapsed().as_secs_f64() * 1000.0;
        Ok(proposals)
    }

    fn tell(&mut self, observations: &[EvaluatedProposal]) -> Result<TellOutcome, String> {
        let mut selected = Vec::new();
        let before = self.incumbent_sys;
        for (index, observation) in observations.iter().enumerate() {
            if observation.evaluation.row.usable_by_optimizer {
                if let Some(value) = observation.evaluation.row.sys {
                    if value > self.incumbent_sys {
                        self.incumbent_sys = value;
                        selected.clear();
                        selected.push((index, 1.0));
                    }
                }
            }
        }
        Ok(TellOutcome {
            selected,
            stop_reason: None,
            fields: json!({
                "incumbent_before": before,
                "incumbent_after": self.incumbent_sys,
                "valid_candidates": observations
                    .iter()
                    .filter(|observation| observation.evaluation.row.usable_by_optimizer)
                    .count(),
                "phase_ms": {"candidate_generation": self.pending_generation_ms},
            }),
        })
    }

    fn is_done(&self) -> Option<String> {
        None
    }
}

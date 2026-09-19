//! Fixed-policy multistart local search for the S0 product-search packet.
//!
//! This module deliberately does not construct polytopes or compute `sys`.
//! The packet evaluator owns those operations, including candidate identities,
//! construction rejection accounting, cache rows, and the hard target-attempt
//! gate.  The driver owns the policy frozen in `resolved-config.json`: proposal
//! order, strict-best selection, and the rule that a truncated line-search grid
//! may not change the local path.

use crate::model::ProposalRole;

pub const WITHIN_STEP_FRACTIONS: [f64; 5] = [0.1, 0.25, 0.5, 0.75, 0.95];
pub const OVERSHOOT_MULTIPLIERS: [f64; 3] = [1.5, 2.0, 3.0];
pub const OVERSHOOT_STEP_BOUND_CUTOFF: f64 = 100.0;
pub const IMPROVEMENT_THRESHOLD: f64 = 1e-6;

/// A successful target evaluation together with the opaque state that can be
/// used as the parent of a later local proposal.
#[derive(Clone, Debug, PartialEq)]
pub struct ScoredState<S> {
    pub state: S,
    pub sys: f64,
}

/// Metadata supplied to the concrete chart/evaluator for one charged request.
/// `proposal_index` is the position in the fixed grid (or the trajectory index
/// for a local start), so target rows can reproduce the policy order.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LocalProposal {
    pub trajectory: usize,
    pub iteration: Option<usize>,
    pub proposal_index: usize,
    pub role: ProposalRole,
    pub step_scale: Option<f64>,
    pub step_size: Option<f64>,
}

impl LocalProposal {
    fn start(trajectory: usize) -> Self {
        Self {
            trajectory,
            iteration: None,
            proposal_index: trajectory,
            role: ProposalRole::LocalStart,
            step_scale: None,
            step_size: None,
        }
    }
}

/// A completed charged target request.  A failed full computation is a normal
/// result here: it already consumed the evaluator's target attempt and must be
/// retained in the output artifact.
#[derive(Clone, Debug, PartialEq)]
pub enum ChargedEvaluation<S, O> {
    Success {
        scored: ScoredState<S>,
        observation: O,
    },
    Failure {
        observation: O,
    },
    /// Geometry construction failed before the target boundary.  This is
    /// counted by the concrete engine but produces no target observation.
    UnchargedConstructionRejection,
}

impl<S, O> ChargedEvaluation<S, O> {
    fn observation(self) -> Option<O> {
        match self {
            Self::Success { observation, .. } | Self::Failure { observation } => Some(observation),
            Self::UnchargedConstructionRejection => None,
        }
    }
}

/// Returned only before a target request.  It is distinct from
/// [`ChargedEvaluation::Failure`], because a failure is charged whereas this
/// condition means no request was made and the search must stop immediately.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TargetBudgetExhausted;

pub type EvaluationResult<S, O> = Result<ChargedEvaluation<S, O>, TargetBudgetExhausted>;

/// Adapter boundary between this policy and the concrete S0 chart/evaluator.
///
/// Implementations must make every `Ok` result correspond to one charged full
/// target attempt, including cache hits, duplicates, and full-computation
/// failures.  After its hard gate is reached, an implementation returns
/// `Err(TargetBudgetExhausted)` without creating another target row.
pub trait LocalSearchEngine {
    type State: Clone;
    type Direction;
    type Observation;

    /// Draw and evaluate the next IID local start.
    fn evaluate_start(
        &mut self,
        proposal: LocalProposal,
    ) -> EvaluationResult<Self::State, Self::Observation>;

    /// Return the branch-aware product-ascent direction at `current`.
    fn ascent_direction(&mut self, current: &ScoredState<Self::State>) -> Option<Self::Direction>;

    /// Return the finite product-cell step bound for `direction`.
    fn step_bound(
        &mut self,
        current: &ScoredState<Self::State>,
        direction: &Self::Direction,
    ) -> f64;

    /// Construct and evaluate the specified local proposal.  Invalid geometric
    /// construction is owned by the adapter; it must preserve the packet's
    /// construction-rejection accounting and only return `Ok` for a charged
    /// target evaluation.
    fn evaluate_step(
        &mut self,
        current: &ScoredState<Self::State>,
        direction: &Self::Direction,
        proposal: LocalProposal,
    ) -> EvaluationResult<Self::State, Self::Observation>;

    /// Mark the already-recorded winning target row as the next local state.
    /// The driver invokes this exactly once for each complete grid that has a
    /// strict improvement, and never for a truncated grid.
    fn accept_next_state(&mut self, next: &ScoredState<Self::State>);
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TrajectoryStop {
    NoDirection,
    InvalidStepBound,
    NoImprovement,
    ImprovementBelowThreshold,
    IncompleteGrid,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TrajectoryResult<S> {
    pub trajectory: usize,
    pub start: ScoredState<S>,
    pub final_state: ScoredState<S>,
    pub accepted_iterations: usize,
    pub stop: TrajectoryStop,
}

#[derive(Clone, Debug, PartialEq)]
pub struct LocalObservation<O> {
    pub proposal: LocalProposal,
    pub observation: O,
}

#[derive(Clone, Debug, PartialEq)]
pub struct LocalSearchResult<S, O> {
    /// Every target row produced by this arm, including failed requests and
    /// the evaluated prefix of a truncated line-search grid.
    pub observations: Vec<LocalObservation<O>>,
    pub trajectories: Vec<TrajectoryResult<S>>,
    pub exhausted: bool,
}

impl<S, O> Default for LocalSearchResult<S, O> {
    fn default() -> Self {
        Self {
            observations: Vec::new(),
            trajectories: Vec::new(),
            exhausted: false,
        }
    }
}

/// Execute the frozen `multistart_branch_local_phase0` policy until the
/// evaluator's hard target-attempt gate is reached.
///
/// A line search advances only if every planned proposal was charged.  Thus a
/// later, better observation in a partial grid cannot depend on the accidental
/// amount of budget remaining when the trajectory began.
pub fn multistart_branch_local_phase0<E>(
    engine: &mut E,
) -> LocalSearchResult<E::State, E::Observation>
where
    E: LocalSearchEngine,
{
    let mut result = LocalSearchResult::default();
    let mut trajectory = 0usize;

    loop {
        let start_proposal = LocalProposal::start(trajectory);
        let start = match engine.evaluate_start(start_proposal) {
            Ok(ChargedEvaluation::Success {
                scored,
                observation,
            }) => {
                result.observations.push(LocalObservation {
                    proposal: start_proposal,
                    observation,
                });
                scored
            }
            Ok(evaluation) => {
                if let Some(observation) = evaluation.observation() {
                    result.observations.push(LocalObservation {
                        proposal: start_proposal,
                        observation,
                    });
                }
                trajectory += 1;
                continue;
            }
            Err(TargetBudgetExhausted) => {
                result.exhausted = true;
                return result;
            }
        };

        let (outcome, exhausted) =
            run_trajectory(engine, trajectory, start, &mut result.observations);
        result.trajectories.push(outcome);
        if exhausted {
            result.exhausted = true;
            return result;
        }
        trajectory += 1;
    }
}

fn run_trajectory<E>(
    engine: &mut E,
    trajectory: usize,
    start: ScoredState<E::State>,
    observations: &mut Vec<LocalObservation<E::Observation>>,
) -> (TrajectoryResult<E::State>, bool)
where
    E: LocalSearchEngine,
{
    let mut current = start.clone();

    for iteration in 0usize.. {
        let Some(direction) = engine.ascent_direction(&current) else {
            return (
                TrajectoryResult {
                    trajectory,
                    start,
                    final_state: current,
                    accepted_iterations: iteration,
                    stop: TrajectoryStop::NoDirection,
                },
                false,
            );
        };
        let step_bound = engine.step_bound(&current, &direction);
        if !step_bound.is_finite() || step_bound <= 0.0 {
            return (
                TrajectoryResult {
                    trajectory,
                    start,
                    final_state: current,
                    accepted_iterations: iteration,
                    stop: TrajectoryStop::InvalidStepBound,
                },
                false,
            );
        }

        let proposals = line_search_grid(trajectory, iteration, step_bound);
        let mut best: Option<ScoredState<E::State>> = None;
        for proposal in proposals {
            match engine.evaluate_step(&current, &direction, proposal) {
                Ok(ChargedEvaluation::Success {
                    scored,
                    observation,
                }) => {
                    observations.push(LocalObservation {
                        proposal,
                        observation,
                    });
                    if scored.sys.is_finite()
                        && scored.sys > current.sys
                        && best
                            .as_ref()
                            .is_none_or(|previous| scored.sys > previous.sys)
                    {
                        best = Some(scored);
                    }
                }
                Ok(evaluation) => {
                    if let Some(observation) = evaluation.observation() {
                        observations.push(LocalObservation {
                            proposal,
                            observation,
                        });
                    }
                }
                Err(TargetBudgetExhausted) => {
                    return (
                        TrajectoryResult {
                            trajectory,
                            start,
                            final_state: current,
                            accepted_iterations: iteration,
                            stop: TrajectoryStop::IncompleteGrid,
                        },
                        true,
                    );
                }
            }
        }

        let Some(next) = best else {
            return (
                TrajectoryResult {
                    trajectory,
                    start,
                    final_state: current,
                    accepted_iterations: iteration,
                    stop: TrajectoryStop::NoImprovement,
                },
                false,
            );
        };
        let delta = next.sys - current.sys;
        engine.accept_next_state(&next);
        current = next;
        if delta < IMPROVEMENT_THRESHOLD {
            return (
                TrajectoryResult {
                    trajectory,
                    start,
                    final_state: current,
                    accepted_iterations: iteration + 1,
                    stop: TrajectoryStop::ImprovementBelowThreshold,
                },
                false,
            );
        }
    }

    unreachable!("unbounded iterator returns only through a local stop")
}

fn line_search_grid(trajectory: usize, iteration: usize, step_bound: f64) -> Vec<LocalProposal> {
    let mut proposals =
        Vec::with_capacity(WITHIN_STEP_FRACTIONS.len() + OVERSHOOT_MULTIPLIERS.len());
    proposals.extend(WITHIN_STEP_FRACTIONS.into_iter().enumerate().map(
        |(proposal_index, step_scale)| LocalProposal {
            trajectory,
            iteration: Some(iteration),
            proposal_index,
            role: ProposalRole::WithinStep,
            step_scale: Some(step_scale),
            step_size: Some(step_scale * step_bound),
        },
    ));
    if step_bound < OVERSHOOT_STEP_BOUND_CUTOFF {
        proposals.extend(OVERSHOOT_MULTIPLIERS.into_iter().enumerate().map(
            |(offset, step_scale)| LocalProposal {
                trajectory,
                iteration: Some(iteration),
                proposal_index: WITHIN_STEP_FRACTIONS.len() + offset,
                role: ProposalRole::Overshoot,
                step_scale: Some(step_scale),
                step_size: Some(step_scale * step_bound),
            },
        ));
    }
    proposals
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;

    #[derive(Clone, Debug, PartialEq)]
    enum Planned {
        Success {
            id: usize,
            sys: f64,
            note: &'static str,
        },
        Failure {
            note: &'static str,
        },
    }

    #[derive(Default)]
    struct SyntheticEvaluator {
        budget: usize,
        charged: usize,
        starts: VecDeque<Planned>,
        steps: VecDeque<Planned>,
        directions_left: usize,
        calls: Vec<LocalProposal>,
        accepted: Vec<usize>,
    }

    impl SyntheticEvaluator {
        fn charge(
            &mut self,
            proposal: LocalProposal,
            planned: Planned,
        ) -> EvaluationResult<usize, &'static str> {
            if self.charged == self.budget {
                return Err(TargetBudgetExhausted);
            }
            self.charged += 1;
            self.calls.push(proposal);
            match planned {
                Planned::Success { id, sys, note } => Ok(ChargedEvaluation::Success {
                    scored: ScoredState { state: id, sys },
                    observation: note,
                }),
                Planned::Failure { note } => Ok(ChargedEvaluation::Failure { observation: note }),
            }
        }
    }

    impl LocalSearchEngine for SyntheticEvaluator {
        type State = usize;
        type Direction = ();
        type Observation = &'static str;

        fn evaluate_start(
            &mut self,
            proposal: LocalProposal,
        ) -> EvaluationResult<Self::State, Self::Observation> {
            let planned = self.starts.pop_front().unwrap_or(Planned::Failure {
                note: "spare-start",
            });
            self.charge(proposal, planned)
        }

        fn ascent_direction(
            &mut self,
            _current: &ScoredState<Self::State>,
        ) -> Option<Self::Direction> {
            (self.directions_left > 0).then(|| {
                self.directions_left -= 1;
            })
        }

        fn step_bound(
            &mut self,
            _current: &ScoredState<Self::State>,
            _direction: &Self::Direction,
        ) -> f64 {
            100.0
        }

        fn evaluate_step(
            &mut self,
            _current: &ScoredState<Self::State>,
            _direction: &Self::Direction,
            proposal: LocalProposal,
        ) -> EvaluationResult<Self::State, Self::Observation> {
            let planned = self
                .steps
                .pop_front()
                .unwrap_or(Planned::Failure { note: "spare-step" });
            self.charge(proposal, planned)
        }

        fn accept_next_state(&mut self, next: &ScoredState<Self::State>) {
            self.accepted.push(next.state);
        }
    }

    fn evaluator(
        budget: usize,
        starts: Vec<Planned>,
        steps: Vec<Planned>,
        directions_left: usize,
    ) -> SyntheticEvaluator {
        SyntheticEvaluator {
            budget,
            starts: starts.into(),
            steps: steps.into(),
            directions_left,
            ..SyntheticEvaluator::default()
        }
    }

    #[test]
    fn complete_grid_advances_to_the_strict_best_improvement() {
        let mut engine = evaluator(
            6,
            vec![Planned::Success {
                id: 0,
                sys: 1.0,
                note: "start",
            }],
            vec![
                Planned::Success {
                    id: 1,
                    sys: 1.2,
                    note: "0.1",
                },
                Planned::Success {
                    id: 2,
                    sys: 1.6,
                    note: "0.25",
                },
                Planned::Success {
                    id: 3,
                    sys: 1.6,
                    note: "0.5-tie",
                },
                Planned::Success {
                    id: 4,
                    sys: 1.4,
                    note: "0.75",
                },
                Planned::Success {
                    id: 5,
                    sys: 1.1,
                    note: "0.95",
                },
            ],
            1,
        );

        let result = multistart_branch_local_phase0(&mut engine);

        assert!(result.exhausted);
        assert_eq!(engine.charged, 6);
        assert_eq!(result.observations.len(), 6);
        let trajectory = &result.trajectories[0];
        assert_eq!(
            trajectory.final_state.state, 2,
            "ties retain the earlier fixed-order proposal"
        );
        assert_eq!(trajectory.accepted_iterations, 1);
        assert_eq!(trajectory.stop, TrajectoryStop::NoDirection);
        assert_eq!(engine.accepted, vec![2]);
        assert_eq!(
            engine
                .calls
                .iter()
                .skip(1)
                .map(|proposal| proposal.step_scale)
                .collect::<Vec<_>>(),
            vec![Some(0.1), Some(0.25), Some(0.5), Some(0.75), Some(0.95)]
        );
    }

    #[test]
    fn incomplete_grid_retains_prefix_but_never_advances_path() {
        let mut engine = evaluator(
            4,
            vec![Planned::Success {
                id: 0,
                sys: 1.0,
                note: "start",
            }],
            vec![
                Planned::Success {
                    id: 1,
                    sys: 9.0,
                    note: "best-but-partial",
                },
                Planned::Success {
                    id: 2,
                    sys: 8.0,
                    note: "partial",
                },
                Planned::Success {
                    id: 3,
                    sys: 7.0,
                    note: "partial",
                },
            ],
            1,
        );

        let result = multistart_branch_local_phase0(&mut engine);

        assert!(result.exhausted);
        assert_eq!(engine.charged, 4);
        assert_eq!(result.observations.len(), 4);
        let trajectory = &result.trajectories[0];
        assert_eq!(trajectory.final_state.state, 0);
        assert_eq!(trajectory.accepted_iterations, 0);
        assert_eq!(trajectory.stop, TrajectoryStop::IncompleteGrid);
        assert!(engine.accepted.is_empty());
    }

    #[test]
    fn failures_and_duplicate_hits_are_charged_and_can_share_a_grid() {
        let mut engine = evaluator(
            6,
            vec![Planned::Success {
                id: 0,
                sys: 1.0,
                note: "start",
            }],
            vec![
                Planned::Failure {
                    note: "failed-miss",
                },
                Planned::Success {
                    id: 0,
                    sys: 1.0,
                    note: "duplicate-hit",
                },
                Planned::Success {
                    id: 2,
                    sys: 1.4,
                    note: "improvement",
                },
                Planned::Failure {
                    note: "another-failure",
                },
                Planned::Success {
                    id: 4,
                    sys: 1.2,
                    note: "lower",
                },
            ],
            1,
        );

        let result = multistart_branch_local_phase0(&mut engine);

        assert_eq!(engine.charged, 6);
        assert_eq!(result.observations.len(), 6);
        assert_eq!(result.trajectories[0].final_state.state, 2);
        assert_eq!(result.observations[1].observation, "failed-miss");
        assert_eq!(result.observations[2].observation, "duplicate-hit");
    }

    #[test]
    fn hard_budget_stop_makes_no_request_after_exhaustion() {
        let mut engine = evaluator(
            0,
            vec![Planned::Success {
                id: 0,
                sys: 1.0,
                note: "unreached",
            }],
            vec![],
            0,
        );

        let result = multistart_branch_local_phase0(&mut engine);

        assert!(result.exhausted);
        assert_eq!(engine.charged, 0);
        assert!(engine.calls.is_empty());
        assert!(result.observations.is_empty());
        assert!(result.trajectories.is_empty());
    }
}

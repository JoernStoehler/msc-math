use super::cma_es::CmaEs;
use crate::algorithm::{EvaluatedProposal, Optimizer, Proposal, TellOutcome};
use crate::branch_model::{solve_raw_sysext_kkt, BranchModel, SliceMode};
use crate::evaluator::{build_geometry, compute_volume, Evaluation, EvaluatorConfig};
use crate::manifest::{CandidateAcceptancePolicy, CmaScaleMode, DirectionalTransitionPolicy};
use crate::quotient::flatten;
use crate::schema::AlgorithmStateRow;
use nalgebra::{DMatrix, Vector4};
use serde_json::json;
use std::collections::{HashSet, VecDeque};
use std::time::Instant;
use symplectic::algorithms::billiard::for_each_sigma_from_facets;
use symplectic::algorithms::facet_adjacency::{
    build_transition_matrix_from_facet_intersections_and_omega, is_feasible_cycle,
};
use symplectic::algorithms::hk2017::{
    for_each_sigma_pruned_by_transition, SimpleDirectedCyclesCanonical,
};
use symplectic::geom::symplectic_form::omega0;
use symplectic::{
    aggregate_orbits_with_dual_vertices_exact, classify_facets_from_dual_vertices,
    solve_orbit_sigma_saddle_point, systolic_ratio, OrbitAdmissibility, OrbitGuaranteeMode,
};

#[derive(Clone, Debug)]
/// Named branch set selected from one [`CandidateUniverse`].
///
/// Beta sign is descriptive and is not an exclusion criterion. The sigmas are
/// still only those transition-feasible during anchor enumeration.
pub struct CandidatePool {
    pub sigmas: Vec<Vec<usize>>,
    pub beta_nonpositive_at_anchor: usize,
    pub raw_solve_failures: usize,
    pub enumerated_sigmas: usize,
    pub discovery_ms: f64,
}

#[derive(Clone, Debug)]
/// Unrestricted f64 KKT branch germ at a validated anchor.
///
/// This record does not claim transition or beta admissibility away from the
/// anchor.
pub struct CandidateGerm {
    pub sigma: Vec<usize>,
    pub action: f64,
    pub beta_margin: f64,
    pub beta_scale: f64,
}

#[derive(Clone, Debug)]
/// Transition-feasible anchor enumeration with unrestricted f64 KKT solves.
///
/// Failed raw solves are counted rather than silently represented. The full
/// evaluator's winning branch is inserted as a semantic witness if the raw
/// solve omitted it.
pub struct CandidateUniverse {
    pub germs: Vec<CandidateGerm>,
    pub min_action: f64,
    pub raw_solve_failures: usize,
    pub enumerated_sigmas: usize,
    pub discovery_ms: f64,
}

impl CandidateUniverse {
    /// Selects germs with `action <= min_action * (1 + relative_window)`.
    ///
    /// Callers should provide a finite nonnegative relative window. No beta
    /// sign cutoff is applied.
    pub fn pool(&self, candidate_window_relative: f64) -> CandidatePool {
        self.pool_with_beta_allowance(candidate_window_relative, None)
    }

    /// Selects germs by action and, when supplied, normalized beta margin.
    ///
    /// `beta_allowance = Some(b)` retains a germ when
    /// `min(beta) / max(abs(beta)) >= -b`. `None` preserves the unfiltered
    /// fixed-candidate behavior.
    pub fn pool_with_beta_allowance(
        &self,
        candidate_window_relative: f64,
        beta_allowance: Option<f64>,
    ) -> CandidatePool {
        let cutoff = self.min_action * (1.0 + candidate_window_relative);
        let selected = self
            .germs
            .iter()
            .filter(|germ| {
                germ.action <= cutoff
                    && beta_allowance.is_none_or(|allowance| {
                        germ.beta_margin / germ.beta_scale.max(f64::EPSILON) >= -allowance
                    })
            })
            .collect::<Vec<_>>();
        let mut sigmas = selected
            .iter()
            .map(|germ| germ.sigma.clone())
            .collect::<Vec<_>>();
        sigmas.sort();
        CandidatePool {
            sigmas,
            beta_nonpositive_at_anchor: selected
                .iter()
                .filter(|germ| germ.beta_margin <= 0.0)
                .count(),
            raw_solve_failures: self.raw_solve_failures,
            enumerated_sigmas: self.enumerated_sigmas,
            discovery_ms: self.discovery_ms,
        }
    }
}

#[derive(Clone, Debug)]
/// Heuristic f64 nonlinear envelope evaluation for a named sigma set.
///
/// Transition-blocked and beta-inadmissible branches do not contribute.
/// Indeterminate beta decisions use the exact singleton fallback.
pub struct SurrogateOutcome {
    pub sys: Option<f64>,
    pub winning_sigma: Option<Vec<usize>>,
    pub admissible_branches: usize,
    pub transition_blocked_branches: usize,
    pub indeterminate_branches: usize,
    pub branch_solve_failures: usize,
    pub geometry_ms: f64,
    pub volume_ms: f64,
    pub branch_ms: f64,
    pub total_ms: f64,
}

#[derive(Clone, Debug)]
/// Transition and unrestricted f64 KKT diagnostics for one named sigma.
///
/// This does not assert physical orbit admissibility. `raw_status` is `ok` or
/// the diagnostic solver's failure text.
pub struct NamedRawDiagnostic {
    pub transition_feasible: bool,
    pub raw_status: String,
    pub action: Option<f64>,
    pub beta_margin: Option<f64>,
    pub beta_scale: Option<f64>,
}

pub struct NonlinearCandidateCma {
    current: Evaluation,
    evaluator_config: EvaluatorConfig,
    candidate_window_relative: f64,
    inner_generations: usize,
    population_size: usize,
    sigma: f64,
    minimum_sigma: f64,
    maximum_sigma: f64,
    seed: u64,
    epoch: usize,
    pool: Option<CandidatePool>,
    pending_surrogate_sys: Option<f64>,
    pending_surrogate_anchor_sys: Option<f64>,
    pending_pool_size: usize,
    done: Option<String>,
}

pub struct NonlinearCandidateRelinearized {
    current: Evaluation,
    incumbent: Evaluation,
    evaluator_config: EvaluatorConfig,
    candidate_window_relative: f64,
    beta_allowance: Option<f64>,
    history_depth: usize,
    acceptance: CandidateAcceptancePolicy,
    directional_transition: DirectionalTransitionPolicy,
    remember_validated_winner: bool,
    candidate_history: VecDeque<Vec<Vec<usize>>>,
    inner_steps: usize,
    distance: f64,
    expansion: f64,
    contraction: f64,
    minimum_distance: f64,
    epoch: usize,
    pool: Option<CandidatePool>,
    pending_surrogate_sys: Option<f64>,
    pending_surrogate_anchor_sys: Option<f64>,
    pending_pool_size: usize,
    pending_candidate_sigmas: Vec<Vec<usize>>,
    done: Option<String>,
}

impl NonlinearCandidateCma {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        initial: &Evaluation,
        candidate_window_relative: f64,
        inner_generations: usize,
        population_size: usize,
        initial_sigma: f64,
        minimum_sigma: f64,
        maximum_sigma: f64,
        seed: u64,
        evaluator_config: EvaluatorConfig,
    ) -> Result<Self, String> {
        if !initial.row.usable_by_optimizer {
            return Err("nonlinear candidate CMA requires a usable initial point".to_string());
        }
        if !candidate_window_relative.is_finite() || candidate_window_relative < 0.0 {
            return Err("candidate window must be nonnegative and finite".to_string());
        }
        if inner_generations == 0 {
            return Err("inner_generations must be positive".to_string());
        }
        if population_size < 4 {
            return Err("population_size must be at least four".to_string());
        }
        if !initial_sigma.is_finite()
            || !minimum_sigma.is_finite()
            || !maximum_sigma.is_finite()
            || minimum_sigma <= 0.0
            || minimum_sigma >= initial_sigma
            || maximum_sigma <= initial_sigma
        {
            return Err("invalid nonlinear candidate CMA sigma bounds".to_string());
        }
        Ok(Self {
            current: initial.clone(),
            evaluator_config,
            candidate_window_relative,
            inner_generations,
            population_size,
            sigma: initial_sigma,
            minimum_sigma,
            maximum_sigma,
            seed,
            epoch: 0,
            pool: None,
            pending_surrogate_sys: None,
            pending_surrogate_anchor_sys: None,
            pending_pool_size: 0,
            done: None,
        })
    }

    fn ensure_pool(&mut self) -> Result<&CandidatePool, String> {
        if self.pool.is_none() {
            self.pool = Some(discover_candidates(
                &self.current,
                self.candidate_window_relative,
            )?);
        }
        self.pool
            .as_ref()
            .ok_or("candidate pool was not built".to_string())
    }
}

impl Optimizer for NonlinearCandidateCma {
    fn ask(&mut self, _remaining_budget: usize) -> Result<Vec<Proposal>, String> {
        if self.done.is_some() {
            return Ok(Vec::new());
        }
        let pool = self.ensure_pool()?.clone();
        let anchor_surrogate =
            evaluate_surrogate(&self.current.duals, &pool.sigmas, &self.evaluator_config);
        let anchor_surrogate_sys = anchor_surrogate
            .sys
            .ok_or("candidate surrogate is invalid at its discovery anchor")?;
        let mut inner = CmaEs::new(
            self.seed ^ (self.epoch as u64).wrapping_mul(0x9e37_79b9_7f4a_7c15),
            &self.current,
            self.sigma,
            self.population_size,
            self.minimum_sigma,
            self.maximum_sigma,
            CmaScaleMode::NormalizedRmsDistance,
        )?;
        let inner_started = Instant::now();
        let mut best: Option<(Vec<Vector4<f64>>, f64)> = None;
        let mut surrogate_queries = 0usize;
        let mut valid_surrogate_queries = 0usize;
        let mut total_admissible = 0usize;
        let mut total_transition_blocked = 0usize;
        let mut total_indeterminate = 0usize;
        let mut total_branch_failures = 0usize;
        let mut total_geometry_ms = anchor_surrogate.geometry_ms;
        let mut total_volume_ms = anchor_surrogate.volume_ms;
        let mut total_branch_ms = anchor_surrogate.branch_ms;
        let mut total_surrogate_ms = anchor_surrogate.total_ms;

        for generation in 0..self.inner_generations {
            let proposals = inner.ask(self.population_size)?;
            if proposals.is_empty() {
                break;
            }
            let observations = proposals
                .into_iter()
                .enumerate()
                .map(|(index, proposal)| {
                    let outcome =
                        evaluate_surrogate(&proposal.duals, &pool.sigmas, &self.evaluator_config);
                    surrogate_queries += 1;
                    valid_surrogate_queries += usize::from(outcome.sys.is_some());
                    total_admissible += outcome.admissible_branches;
                    total_transition_blocked += outcome.transition_blocked_branches;
                    total_indeterminate += outcome.indeterminate_branches;
                    total_branch_failures += outcome.branch_solve_failures;
                    total_geometry_ms += outcome.geometry_ms;
                    total_volume_ms += outcome.volume_ms;
                    total_branch_ms += outcome.branch_ms;
                    total_surrogate_ms += outcome.total_ms;
                    if let Some(value) = outcome.sys {
                        if best
                            .as_ref()
                            .is_none_or(|(_, best_value)| value > *best_value)
                        {
                            best = Some((proposal.duals.clone(), value));
                        }
                    }
                    EvaluatedProposal {
                        proposal_id: format!("inner-g{generation}-p{index}"),
                        evaluation: surrogate_evaluation(
                            &self.current,
                            proposal.duals,
                            outcome.sys,
                        ),
                    }
                })
                .collect::<Vec<_>>();
            inner.tell(&observations)?;
        }
        let inner_ms = inner_started.elapsed().as_secs_f64() * 1000.0;
        let Some((target, predicted_sys)) = best else {
            self.sigma *= 0.5;
            if self.sigma < self.minimum_sigma {
                self.done = Some("no_valid_surrogate_target_above_minimum_sigma".to_string());
                return Ok(Vec::new());
            }
            return self.ask(_remaining_budget);
        };
        self.pending_surrogate_sys = Some(predicted_sys);
        self.pending_surrogate_anchor_sys = Some(anchor_surrogate_sys);
        self.pending_pool_size = pool.sigmas.len();
        Ok(vec![Proposal {
            duals: target,
            baseline_evaluation_id: Some(self.current.row.evaluation_id.clone()),
            geometric_reference_kind: Some("surrogate_epoch_anchor".to_string()),
            geometric_reference_duals: Some(self.current.duals.clone()),
            fields: json!({
                "candidate_window_relative": self.candidate_window_relative,
                "candidate_set_policy": "replace_at_each_validated_epoch",
                "candidate_count": pool.sigmas.len(),
                "beta_nonpositive_candidate_count_at_anchor": pool.beta_nonpositive_at_anchor,
                "raw_candidate_solve_failures": pool.raw_solve_failures,
                "enumerated_sigma_count": pool.enumerated_sigmas,
                "inner_optimizer": "cma_es",
                "inner_generations": self.inner_generations,
                "inner_population_size": self.population_size,
                "surrogate_queries": surrogate_queries,
                "valid_surrogate_queries": valid_surrogate_queries,
                "predicted_anchor_sys": anchor_surrogate_sys,
                "predicted_target_sys": predicted_sys,
                "predicted_delta": predicted_sys - anchor_surrogate_sys,
                "mean_admissible_branch_count": total_admissible as f64
                    / (surrogate_queries + 1) as f64,
                "mean_transition_blocked_branch_count": total_transition_blocked as f64
                    / (surrogate_queries + 1) as f64,
                "indeterminate_branch_count": total_indeterminate,
                "branch_solve_failure_count": total_branch_failures,
                "sigma": self.sigma,
                "epoch": self.epoch,
                "phase_ms": {
                    "candidate_discovery": pool.discovery_ms,
                    "surrogate_geometry": total_geometry_ms,
                    "surrogate_volume": total_volume_ms,
                    "surrogate_named_branches": total_branch_ms,
                    "surrogate_total": total_surrogate_ms,
                    "inner_optimizer_total": inner_ms,
                },
            }),
        }])
    }

    fn tell(&mut self, observations: &[EvaluatedProposal]) -> Result<TellOutcome, String> {
        if observations.len() != 1 {
            return Err("nonlinear candidate CMA expects one validated epoch target".to_string());
        }
        let observation = &observations[0];
        let before = self.current.row.sys.ok_or("current evaluation lacks sys")?;
        let after = observation.evaluation.row.sys;
        let predicted_target = self
            .pending_surrogate_sys
            .take()
            .ok_or("missing pending surrogate target")?;
        let predicted_anchor = self
            .pending_surrogate_anchor_sys
            .take()
            .ok_or("missing pending surrogate anchor")?;
        let target_winner_in_candidate_set = observation
            .evaluation
            .row
            .winning_sigma
            .as_ref()
            .is_some_and(|winner| {
                self.pool
                    .as_ref()
                    .is_some_and(|pool| pool.sigmas.contains(winner))
            });
        let target_usable = observation.evaluation.row.usable_by_optimizer && after.is_some();
        let accepted = validated_improvement(before, after, target_usable);
        if accepted {
            self.current = observation.evaluation.clone();
            self.pool = None;
        } else {
            self.sigma *= 0.5;
            if self.sigma < self.minimum_sigma {
                self.done = Some("minimum_sigma_after_invalid_validation".to_string());
            }
        }
        self.epoch += 1;
        Ok(TellOutcome {
            selected: accepted.then_some((0, 1.0)).into_iter().collect(),
            stop_reason: self.done.clone(),
            fields: json!({
                "validated_target_usable": target_usable,
                "moved_epoch_anchor": accepted,
                "observed_delta": after.map(|value| value - before),
                "predicted_anchor_sys": predicted_anchor,
                "predicted_target_sys": predicted_target,
                "prediction_error": after.map(|value| value - predicted_target),
                "target_winner_in_candidate_set": target_winner_in_candidate_set,
                "candidate_count": self.pending_pool_size,
                "sigma_after": self.sigma,
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

impl NonlinearCandidateRelinearized {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        initial: &Evaluation,
        candidate_window_relative: f64,
        beta_allowance: Option<f64>,
        history_depth: usize,
        acceptance: CandidateAcceptancePolicy,
        directional_transition: DirectionalTransitionPolicy,
        remember_validated_winner: bool,
        inner_steps: usize,
        initial_distance: f64,
        expansion: f64,
        contraction: f64,
        minimum_distance: f64,
        evaluator_config: EvaluatorConfig,
    ) -> Result<Self, String> {
        if !initial.row.usable_by_optimizer {
            return Err(
                "nonlinear candidate relinearization requires a usable initial point".to_string(),
            );
        }
        if !candidate_window_relative.is_finite() || candidate_window_relative < 0.0 {
            return Err("candidate window must be nonnegative and finite".to_string());
        }
        if beta_allowance.is_some_and(|value| !value.is_finite() || value < 0.0) {
            return Err("beta allowance must be nonnegative and finite".to_string());
        }
        if history_depth == 0 {
            return Err("history depth must be positive".to_string());
        }
        if inner_steps == 0 {
            return Err("inner_steps must be positive".to_string());
        }
        if !initial_distance.is_finite()
            || !minimum_distance.is_finite()
            || initial_distance <= minimum_distance
            || minimum_distance <= 0.0
            || !expansion.is_finite()
            || expansion <= 1.0
            || !contraction.is_finite()
            || !(0.0..1.0).contains(&contraction)
        {
            return Err("invalid nonlinear relinearization distance policy".to_string());
        }
        Ok(Self {
            current: initial.clone(),
            incumbent: initial.clone(),
            evaluator_config,
            candidate_window_relative,
            beta_allowance,
            history_depth,
            acceptance,
            directional_transition,
            remember_validated_winner,
            candidate_history: VecDeque::new(),
            inner_steps,
            distance: initial_distance,
            expansion,
            contraction,
            minimum_distance,
            epoch: 0,
            pool: None,
            pending_surrogate_sys: None,
            pending_surrogate_anchor_sys: None,
            pending_pool_size: 0,
            pending_candidate_sigmas: Vec::new(),
            done: None,
        })
    }

    fn ensure_pool(&mut self) -> Result<&CandidatePool, String> {
        if self.pool.is_none() {
            self.pool = Some(
                discover_candidate_universe(&self.current)?
                    .pool_with_beta_allowance(self.candidate_window_relative, self.beta_allowance),
            );
        }
        self.pool
            .as_ref()
            .ok_or("candidate pool was not built".to_string())
    }
}

impl Optimizer for NonlinearCandidateRelinearized {
    fn ask(&mut self, _remaining_budget: usize) -> Result<Vec<Proposal>, String> {
        if self.done.is_some() {
            return Ok(Vec::new());
        }
        let pool = self.ensure_pool()?.clone();
        let mut candidate_set = pool.sigmas.iter().cloned().collect::<HashSet<_>>();
        for historical in &self.candidate_history {
            candidate_set.extend(historical.iter().cloned());
        }
        let mut sigmas = candidate_set.into_iter().collect::<Vec<_>>();
        sigmas.sort();
        let anchor = evaluate_surrogate(&self.current.duals, &sigmas, &self.evaluator_config);
        let anchor_sys = anchor
            .sys
            .ok_or("candidate surrogate is invalid at its discovery anchor")?;
        let started = Instant::now();
        let mut point = self.current.duals.clone();
        let mut point_sys = anchor_sys;
        let mut accepted_steps = 0usize;
        let mut model_solves = 0usize;
        let mut target_queries = 0usize;
        let mut total_model_build_ms = 0.0;
        let mut total_model_solve_ms = 0.0;
        let mut total_target_ms = 0.0;
        let mut total_directional_graph_ms = 0.0;
        let mut directional_graph_cycles = 0usize;
        let mut directional_candidates_added = 0usize;
        let mut directional_raw_solve_failures = 0usize;
        let distance_before = self.distance;

        for _ in 0..self.inner_steps {
            let geometry_started = Instant::now();
            let Ok((polytope, _)) = build_geometry(&point, self.evaluator_config.geometry_mode)
            else {
                self.distance *= self.contraction;
                continue;
            };
            let Ok(volume) = compute_volume(&polytope, self.evaluator_config.volume_mode) else {
                self.distance *= self.contraction;
                continue;
            };
            let model = BranchModel::build_from_named_candidates(&polytope, volume, &sigmas)?;
            total_model_build_ms += geometry_started.elapsed().as_secs_f64() * 1000.0;
            let solution =
                model.solve_euclidean(&point, self.distance, SliceMode::SymmetryTransverse, 1.0)?;
            model_solves += 1;
            total_model_solve_ms += solution.solve_ms;
            let target = crate::quotient::add_flat_direction(
                &point,
                &nalgebra::DVector::from_vec(solution.displacement_flat),
                1.0,
            );
            let directional = directional_transition_candidates(
                &point,
                &target,
                &polytope,
                volume,
                point_sys,
                &self.directional_transition,
            );
            total_directional_graph_ms += directional.elapsed_ms;
            directional_graph_cycles += directional.graph_cycles;
            directional_raw_solve_failures += directional.raw_solve_failures;
            let old_count = sigmas.len();
            sigmas.extend(directional.sigmas);
            sigmas.sort();
            sigmas.dedup();
            directional_candidates_added += sigmas.len() - old_count;
            let target_outcome = evaluate_surrogate(&target, &sigmas, &self.evaluator_config);
            target_queries += 1;
            total_target_ms += target_outcome.total_ms;
            if target_outcome
                .sys
                .is_some_and(|target_sys| target_sys > point_sys)
            {
                point = target;
                point_sys = target_outcome.sys.expect("checked value");
                accepted_steps += 1;
                self.distance *= self.expansion;
            } else {
                self.distance *= self.contraction;
            }
            if self.distance < self.minimum_distance {
                self.done = Some("minimum_inner_distance".to_string());
                break;
            }
        }
        if accepted_steps == 0 {
            if self.done.is_some() {
                return Ok(Vec::new());
            }
            return self.ask(_remaining_budget);
        }
        self.pending_surrogate_sys = Some(point_sys);
        self.pending_surrogate_anchor_sys = Some(anchor_sys);
        self.pending_pool_size = sigmas.len();
        self.pending_candidate_sigmas = sigmas.clone();
        Ok(vec![Proposal {
            duals: point,
            baseline_evaluation_id: Some(self.current.row.evaluation_id.clone()),
            geometric_reference_kind: Some("surrogate_epoch_anchor".to_string()),
            geometric_reference_duals: Some(self.current.duals.clone()),
            fields: json!({
                "candidate_window_relative": self.candidate_window_relative,
                "beta_allowance": self.beta_allowance,
                "candidate_history_depth": self.history_depth,
                "directional_transition_policy": self.directional_transition,
                "remember_validated_winner": self.remember_validated_winner,
                "candidate_set_policy": "current_ranked_set_union_recent_ranked_sets",
                "candidate_count": sigmas.len(),
                "fresh_candidate_count": pool.sigmas.len(),
                "historical_set_count": self.candidate_history.len(),
                "beta_nonpositive_candidate_count_at_anchor": pool.beta_nonpositive_at_anchor,
                "raw_candidate_solve_failures": pool.raw_solve_failures,
                "enumerated_sigma_count": pool.enumerated_sigmas,
                "inner_optimizer": "repeated_relinearization",
                "inner_steps": self.inner_steps,
                "accepted_inner_steps": accepted_steps,
                "model_solves": model_solves,
                "surrogate_target_queries": target_queries,
                "directional_graph_cycles": directional_graph_cycles,
                "directional_candidates_added": directional_candidates_added,
                "directional_raw_solve_failures": directional_raw_solve_failures,
                "predicted_anchor_sys": anchor_sys,
                "predicted_target_sys": point_sys,
                "predicted_delta": point_sys - anchor_sys,
                "distance_before": distance_before,
                "distance_after": self.distance,
                "epoch": self.epoch,
                "phase_ms": {
                    "candidate_discovery": pool.discovery_ms,
                    "anchor_surrogate": anchor.total_ms,
                    "inner_model_build": total_model_build_ms,
                    "inner_model_solve": total_model_solve_ms,
                    "inner_target_surrogate": total_target_ms,
                    "directional_transition": total_directional_graph_ms,
                    "inner_optimizer_total": started.elapsed().as_secs_f64() * 1000.0,
                },
            }),
        }])
    }

    fn tell(&mut self, observations: &[EvaluatedProposal]) -> Result<TellOutcome, String> {
        if observations.len() != 1 {
            return Err(
                "nonlinear candidate relinearization expects one validated epoch target"
                    .to_string(),
            );
        }
        let observation = &observations[0];
        let before = self.current.row.sys.ok_or("current evaluation lacks sys")?;
        let after = observation.evaluation.row.sys;
        let predicted_target = self
            .pending_surrogate_sys
            .take()
            .ok_or("missing pending surrogate target")?;
        let predicted_anchor = self
            .pending_surrogate_anchor_sys
            .take()
            .ok_or("missing pending surrogate anchor")?;
        let target_winner_in_candidate_set = observation
            .evaluation
            .row
            .winning_sigma
            .as_ref()
            .is_some_and(|winner| self.pending_candidate_sigmas.contains(winner));
        let validated_winner = observation.evaluation.row.winning_sigma.clone();
        let learned_validated_winner = self.remember_validated_winner
            && validated_winner
                .as_ref()
                .is_some_and(|winner| !self.pending_candidate_sigmas.contains(winner));
        let target_usable = observation.evaluation.row.usable_by_optimizer && after.is_some();
        let incumbent_sys = self
            .incumbent
            .row
            .sys
            .ok_or("incumbent evaluation lacks sys")?;
        let accepted = candidate_move_is_accepted(
            &self.acceptance,
            before,
            incumbent_sys,
            after,
            target_usable,
        );
        let mut returned_to_incumbent = false;
        if accepted {
            if self.history_depth > 1 {
                let mut fresh = self
                    .pool
                    .as_ref()
                    .ok_or("missing fresh candidate pool")?
                    .sigmas
                    .clone();
                if self.remember_validated_winner {
                    fresh.extend(validated_winner.iter().cloned());
                    fresh.sort();
                    fresh.dedup();
                }
                self.candidate_history.push_front(fresh);
                self.candidate_history.truncate(self.history_depth - 1);
            }
            self.current = observation.evaluation.clone();
            if after.is_some_and(|value| value > incumbent_sys) {
                self.incumbent = observation.evaluation.clone();
            }
            self.pool = None;
        } else {
            if learned_validated_winner {
                let pool = self
                    .pool
                    .as_mut()
                    .ok_or("missing candidate pool while learning validated winner")?;
                pool.sigmas.extend(validated_winner.iter().cloned());
                pool.sigmas.sort();
                pool.sigmas.dedup();
            }
            if matches!(
                self.acceptance,
                CandidateAcceptancePolicy::BoundedIncumbentDrawdown {
                    return_to_incumbent_on_rejection: true,
                    ..
                }
            ) && self.current.row.evaluation_id != self.incumbent.row.evaluation_id
            {
                self.current = self.incumbent.clone();
                self.pool = None;
                returned_to_incumbent = true;
            }
            self.distance *= self.contraction;
            if self.distance < self.minimum_distance {
                self.done = Some("minimum_distance_after_invalid_validation".to_string());
            }
        }
        self.epoch += 1;
        self.pending_candidate_sigmas.clear();
        Ok(TellOutcome {
            selected: accepted.then_some((0, 1.0)).into_iter().collect(),
            stop_reason: self.done.clone(),
            fields: json!({
                "validated_target_usable": target_usable,
                "moved_epoch_anchor": accepted,
                "returned_to_incumbent": returned_to_incumbent,
                "incumbent_sys": incumbent_sys,
                "acceptance_policy": self.acceptance,
                "observed_delta": after.map(|value| value - before),
                "predicted_anchor_sys": predicted_anchor,
                "predicted_target_sys": predicted_target,
                "prediction_error": after.map(|value| value - predicted_target),
                "target_winner_in_candidate_set": target_winner_in_candidate_set,
                "learned_validated_winner": learned_validated_winner,
                "candidate_count": self.pending_pool_size,
                "distance_after": self.distance,
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

struct DirectionalCandidates {
    sigmas: Vec<Vec<usize>>,
    graph_cycles: usize,
    raw_solve_failures: usize,
    elapsed_ms: f64,
}

fn directional_transition_candidates(
    point: &[Vector4<f64>],
    target: &[Vector4<f64>],
    polytope: &exp_sys_landscape::SysLandscapePolytopeCache,
    volume: f64,
    point_sys: f64,
    policy: &DirectionalTransitionPolicy,
) -> DirectionalCandidates {
    let started = Instant::now();
    if matches!(policy, DirectionalTransitionPolicy::None) {
        return DirectionalCandidates {
            sigmas: Vec::new(),
            graph_cycles: 0,
            raw_solve_failures: 0,
            elapsed_ms: started.elapsed().as_secs_f64() * 1000.0,
        };
    }
    let displacement = point
        .iter()
        .zip(target)
        .map(|(left, right)| right - left)
        .collect::<Vec<_>>();
    if let DirectionalTransitionPolicy::UnfilteredAboveDistance {
        minimum_normalized_distance,
    } = policy
    {
        let point_norm = flatten(point)
            .iter()
            .map(|value| value * value)
            .sum::<f64>()
            .sqrt();
        let displacement_norm = flatten(&displacement)
            .iter()
            .map(|value| value * value)
            .sum::<f64>()
            .sqrt();
        if point_norm > 0.0 && displacement_norm / point_norm < *minimum_normalized_distance {
            return DirectionalCandidates {
                sigmas: Vec::new(),
                graph_cycles: 0,
                raw_solve_failures: 0,
                elapsed_ms: started.elapsed().as_secs_f64() * 1000.0,
            };
        }
    }
    let affine_omega = DMatrix::from_fn(point.len(), point.len(), |i, j| {
        if i == j {
            return 0;
        }
        let base = omega0(&point[i], &point[j]);
        let left = omega0(&displacement[i], &point[j]);
        let right = omega0(&point[i], &displacement[j]);
        ternary_sign(base + left + right, base.abs() + left.abs() + right.abs())
    });
    let transition = build_transition_matrix_from_facet_intersections_and_omega(
        &polytope.facet_intersection_is_nonempty,
        &affine_omega,
    );
    let graph = SimpleDirectedCyclesCanonical::new(&transition).collect::<Vec<_>>();
    let graph_cycles = graph.len();
    let mut raw_solve_failures = 0usize;
    let sigmas = match policy {
        DirectionalTransitionPolicy::None => unreachable!(),
        DirectionalTransitionPolicy::Unfiltered
        | DirectionalTransitionPolicy::UnfilteredAboveDistance { .. } => graph,
        DirectionalTransitionPolicy::AnchorActionWindow { relative_window } => {
            let minimum_action = (2.0 * volume * point_sys).sqrt();
            let cutoff = minimum_action * (1.0 + relative_window);
            graph
                .into_iter()
                .filter(
                    |sigma| match solve_raw_sysext_kkt(&polytope.dual_vertices_f64, sigma) {
                        Ok(raw) => raw.action <= cutoff,
                        Err(_) => {
                            raw_solve_failures += 1;
                            false
                        }
                    },
                )
                .collect()
        }
    };
    DirectionalCandidates {
        sigmas,
        graph_cycles,
        raw_solve_failures,
        elapsed_ms: started.elapsed().as_secs_f64() * 1000.0,
    }
}

fn ternary_sign(value: f64, scale: f64) -> i8 {
    let tolerance = 64.0 * f64::EPSILON * scale.max(1.0);
    if value > tolerance {
        1
    } else if value < -tolerance {
        -1
    } else {
        0
    }
}

/// Discovers the raw branch universe used by fixed-candidate optimizers.
///
/// The anchor must be a validated evaluator result with reusable geometry
/// context. Enumeration is transition-feasible at the anchor but does not
/// impose a beta sign or action-window cutoff.
pub fn discover_candidate_universe(anchor: &Evaluation) -> Result<CandidateUniverse, String> {
    let started = Instant::now();
    let context = anchor
        .context
        .as_ref()
        .ok_or("candidate discovery requires evaluator geometry context")?;
    let transition = build_transition_matrix_from_facet_intersections_and_omega(
        &context.polytope.facet_intersection_is_nonempty,
        &context.polytope.omega_signs,
    );
    let mut enumerated = Vec::new();
    if let Ok(classification) =
        classify_facets_from_dual_vertices(&context.polytope.dual_vertices_f64)
    {
        for_each_sigma_from_facets(
            &classification.q_indices,
            &classification.p_indices,
            &context.polytope.facet_intersection_is_nonempty,
            &transition,
            |sigma| enumerated.push(sigma.to_vec()),
        );
    } else {
        for_each_sigma_pruned_by_transition(&transition, |sigma| enumerated.push(sigma.to_vec()));
    }
    let enumerated_sigmas = enumerated.len();
    let mut raw_solve_failures = 0usize;
    let mut germs = HashSet::new();
    let mut solved = Vec::new();
    for sigma in enumerated {
        match solve_raw_sysext_kkt(&context.polytope.dual_vertices_f64, &sigma) {
            Ok(raw) => {
                germs.insert(sigma.clone());
                solved.push(CandidateGerm {
                    sigma,
                    action: raw.action,
                    beta_margin: raw.beta_margin,
                    beta_scale: raw
                        .beta
                        .iter()
                        .map(|value| value.abs())
                        .fold(0.0_f64, f64::max),
                });
            }
            Err(_) => raw_solve_failures += 1,
        }
    }
    if germs.insert(context.winning_orbit.sigma.clone()) {
        solved.push(CandidateGerm {
            sigma: context.winning_orbit.sigma.clone(),
            action: context.winning_orbit.action,
            beta_margin: context.winning_orbit.beta_margin,
            beta_scale: context.winning_orbit.beta_margin.abs(),
        });
    }
    solved.sort_by(|left, right| {
        left.action
            .total_cmp(&right.action)
            .then_with(|| left.sigma.cmp(&right.sigma))
    });
    Ok(CandidateUniverse {
        germs: solved,
        min_action: context.min_action,
        raw_solve_failures,
        enumerated_sigmas,
        discovery_ms: started.elapsed().as_secs_f64() * 1000.0,
    })
}

pub fn diagnose_named_raw_branch(
    evaluation: &Evaluation,
    sigma: &[usize],
) -> Result<NamedRawDiagnostic, String> {
    let context = evaluation
        .context
        .as_ref()
        .ok_or("named branch diagnosis requires evaluator geometry context")?;
    let transition = build_transition_matrix_from_facet_intersections_and_omega(
        &context.polytope.facet_intersection_is_nonempty,
        &context.polytope.omega_signs,
    );
    let transition_feasible = is_feasible_cycle(sigma, &transition);
    Ok(
        match solve_raw_sysext_kkt(&context.polytope.dual_vertices_f64, sigma) {
            Ok(raw) => NamedRawDiagnostic {
                transition_feasible,
                raw_status: "ok".to_string(),
                action: Some(raw.action),
                beta_margin: Some(raw.beta_margin),
                beta_scale: Some(
                    raw.beta
                        .iter()
                        .map(|value| value.abs())
                        .fold(0.0_f64, f64::max),
                ),
            },
            Err(error) => NamedRawDiagnostic {
                transition_feasible,
                raw_status: error,
                action: None,
                beta_margin: None,
                beta_scale: None,
            },
        },
    )
}

fn discover_candidates(
    anchor: &Evaluation,
    candidate_window_relative: f64,
) -> Result<CandidatePool, String> {
    Ok(discover_candidate_universe(anchor)?.pool(candidate_window_relative))
}

fn validated_improvement(before: f64, after: Option<f64>, target_usable: bool) -> bool {
    target_usable && after.is_some_and(|value| value > before)
}

fn candidate_move_is_accepted(
    policy: &CandidateAcceptancePolicy,
    before: f64,
    incumbent: f64,
    after: Option<f64>,
    target_usable: bool,
) -> bool {
    if !target_usable {
        return false;
    }
    let Some(after) = after else {
        return false;
    };
    match policy {
        CandidateAcceptancePolicy::AnyUsable => true,
        CandidateAcceptancePolicy::ImprovingOnly => after > before,
        CandidateAcceptancePolicy::BoundedIncumbentDrawdown {
            max_relative_drawdown,
            ..
        } => after >= incumbent * (1.0 - max_relative_drawdown),
    }
}

/// Evaluates the nonlinear minimum over named sigmas at `duals`.
///
/// This is an f64 optimizer heuristic with exact fallback only for
/// indeterminate beta, not a replacement for full `sys` validation.
pub fn evaluate_surrogate(
    duals: &[Vector4<f64>],
    sigmas: &[Vec<usize>],
    config: &EvaluatorConfig,
) -> SurrogateOutcome {
    let total_started = Instant::now();
    let geometry_started = Instant::now();
    let Ok((polytope, _)) = build_geometry(duals, config.geometry_mode) else {
        return failed_surrogate(total_started, geometry_started);
    };
    let geometry_ms = geometry_started.elapsed().as_secs_f64() * 1000.0;
    let volume_started = Instant::now();
    let Ok(volume) = compute_volume(&polytope, config.volume_mode) else {
        return SurrogateOutcome {
            sys: None,
            winning_sigma: None,
            admissible_branches: 0,
            transition_blocked_branches: 0,
            indeterminate_branches: 0,
            branch_solve_failures: 0,
            geometry_ms,
            volume_ms: volume_started.elapsed().as_secs_f64() * 1000.0,
            branch_ms: 0.0,
            total_ms: total_started.elapsed().as_secs_f64() * 1000.0,
        };
    };
    let volume_ms = volume_started.elapsed().as_secs_f64() * 1000.0;
    let transition = build_transition_matrix_from_facet_intersections_and_omega(
        &polytope.facet_intersection_is_nonempty,
        &polytope.omega_signs,
    );
    let branch_started = Instant::now();
    let mut min_action = f64::INFINITY;
    let mut winning_sigma = None;
    let mut admissible_branches = 0usize;
    let mut transition_blocked_branches = 0usize;
    let mut indeterminate_branches = 0usize;
    let mut branch_solve_failures = 0usize;
    for sigma in sigmas {
        if !is_feasible_cycle(sigma, &transition) {
            transition_blocked_branches += 1;
            continue;
        }
        match solve_orbit_sigma_saddle_point(&polytope.dual_vertices_f64, sigma) {
            Ok(orbit) if orbit.admissibility == OrbitAdmissibility::AdmissibleF64 => {
                admissible_branches += 1;
                if orbit.action < min_action {
                    min_action = orbit.action;
                    winning_sigma = Some(orbit.sigma);
                }
            }
            Ok(orbit) => {
                indeterminate_branches += 1;
                match aggregate_orbits_with_dual_vertices_exact(
                    &polytope.dual_vertices,
                    vec![orbit],
                    1,
                    0.0,
                    OrbitGuaranteeMode::AllSafe,
                ) {
                    Ok(exact) => {
                        admissible_branches += 1;
                        if exact.min_action < min_action {
                            min_action = exact.min_action;
                            winning_sigma = Some(exact.best_orbit().sigma.clone());
                        }
                    }
                    Err(_) => branch_solve_failures += 1,
                }
            }
            Err(_) => {}
        }
    }
    let branch_ms = branch_started.elapsed().as_secs_f64() * 1000.0;
    SurrogateOutcome {
        sys: min_action
            .is_finite()
            .then(|| systolic_ratio(min_action, volume)),
        winning_sigma,
        admissible_branches,
        transition_blocked_branches,
        indeterminate_branches,
        branch_solve_failures,
        geometry_ms,
        volume_ms,
        branch_ms,
        total_ms: total_started.elapsed().as_secs_f64() * 1000.0,
    }
}

fn failed_surrogate(total_started: Instant, geometry_started: Instant) -> SurrogateOutcome {
    SurrogateOutcome {
        sys: None,
        winning_sigma: None,
        admissible_branches: 0,
        transition_blocked_branches: 0,
        indeterminate_branches: 0,
        branch_solve_failures: 0,
        geometry_ms: geometry_started.elapsed().as_secs_f64() * 1000.0,
        volume_ms: 0.0,
        branch_ms: 0.0,
        total_ms: total_started.elapsed().as_secs_f64() * 1000.0,
    }
}

fn surrogate_evaluation(
    template: &Evaluation,
    duals: Vec<Vector4<f64>>,
    sys: Option<f64>,
) -> Evaluation {
    let mut evaluation = template.clone();
    evaluation.duals = duals.clone();
    evaluation.row.dual_flat = flatten(&duals);
    evaluation.row.sys = sys;
    evaluation.row.usable_by_optimizer = sys.is_some();
    evaluation.row.status = if sys.is_some() {
        "surrogate".to_string()
    } else {
        "invalid_surrogate".to_string()
    };
    evaluation.context = None;
    evaluation.physical_evaluation = false;
    evaluation
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::evaluator::Evaluator;

    #[test]
    fn candidate_pool_can_filter_by_normalized_beta_margin() {
        let universe = CandidateUniverse {
            germs: vec![
                CandidateGerm {
                    sigma: vec![0],
                    action: 1.0,
                    beta_margin: -0.2,
                    beta_scale: 1.0,
                },
                CandidateGerm {
                    sigma: vec![1],
                    action: 1.1,
                    beta_margin: -0.4,
                    beta_scale: 1.0,
                },
                CandidateGerm {
                    sigma: vec![2],
                    action: 1.4,
                    beta_margin: 0.1,
                    beta_scale: 1.0,
                },
            ],
            min_action: 1.0,
            raw_solve_failures: 0,
            enumerated_sigmas: 3,
            discovery_ms: 0.0,
        };
        assert_eq!(universe.pool(0.3).sigmas, vec![vec![0], vec![1]]);
        assert_eq!(
            universe.pool_with_beta_allowance(0.3, Some(0.3)).sigmas,
            vec![vec![0]]
        );
    }

    #[test]
    fn directional_omega_sign_is_ternary_near_roundoff() {
        assert_eq!(ternary_sign(1.0, 1.0), 1);
        assert_eq!(ternary_sign(-1.0, 1.0), -1);
        assert_eq!(ternary_sign(f64::EPSILON, 1.0), 0);
    }

    #[test]
    fn validated_candidate_move_must_improve_full_sys() {
        assert!(validated_improvement(0.8, Some(0.9), true));
        assert!(!validated_improvement(0.8, Some(0.8), true));
        assert!(!validated_improvement(0.8, Some(0.7), true));
        assert!(!validated_improvement(0.8, Some(0.9), false));
    }

    #[test]
    fn bounded_acceptance_uses_drawdown_from_incumbent() {
        let policy = CandidateAcceptancePolicy::BoundedIncumbentDrawdown {
            max_relative_drawdown: 0.03,
            return_to_incumbent_on_rejection: false,
        };
        assert!(candidate_move_is_accepted(
            &policy,
            0.98,
            1.0,
            Some(0.975),
            true
        ));
        assert!(!candidate_move_is_accepted(
            &policy,
            0.98,
            1.0,
            Some(0.969),
            true
        ));
        assert!(!candidate_move_is_accepted(
            &policy,
            0.98,
            1.0,
            Some(0.99),
            false
        ));
    }

    #[test]
    fn nonlinear_candidate_envelope_matches_full_sys_at_discovery_anchor() {
        let duals = vec![
            Vector4::new(1.0, 0.0, 0.0, 0.0),
            Vector4::new(-1.0, 0.0, 0.0, 0.0),
            Vector4::new(0.0, 1.0, 0.0, 0.0),
            Vector4::new(0.0, -1.0, 0.0, 0.0),
            Vector4::new(0.0, 0.0, 1.0, 0.0),
            Vector4::new(0.0, 0.0, -1.0, 0.0),
            Vector4::new(0.0, 0.0, 0.0, 1.0),
            Vector4::new(0.0, 0.0, 0.0, -1.0),
        ];
        let config = EvaluatorConfig::default();
        let mut evaluator = Evaluator::new(config.clone());
        let evaluation =
            evaluator.evaluate("test", "initial".into(), None, "initial", 0, false, duals);
        let pool = discover_candidates(&evaluation, 1.0).unwrap();
        let envelope = evaluate_surrogate(&evaluation.duals, &pool.sigmas, &config);
        assert!(pool.sigmas.contains(
            evaluation
                .row
                .winning_sigma
                .as_ref()
                .expect("full evaluator winner")
        ));
        assert!(
            (envelope.sys.expect("surrogate value") - evaluation.row.sys.unwrap()).abs() < 1.0e-10
        );
        assert_eq!(envelope.winning_sigma, evaluation.row.winning_sigma);
    }
}

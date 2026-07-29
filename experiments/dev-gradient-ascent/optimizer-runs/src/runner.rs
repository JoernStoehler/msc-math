use crate::algorithms;
use crate::evaluator::Evaluator;
use crate::manifest::{AlgorithmSpec, ResolvedPlan, ResolvedRun};
use crate::output::write_jsonl;
use crate::quotient::{displacement_l2, l2_norm, unflatten};
use crate::schema::{EvaluationRow, ProposalRow, RoundRow, RunRow, SelectedProposal, SourcePoint};
use rayon::prelude::*;
use std::collections::HashMap;
use std::path::Path;
use std::time::Instant;

const SCHEMA_VERSION: u32 = 1;

#[derive(Default)]
struct RunArtifacts {
    evaluations: Vec<EvaluationRow>,
    proposals: Vec<ProposalRow>,
    rounds: Vec<RoundRow>,
    run: Option<RunRow>,
}

pub fn run_plan(
    plan: &ResolvedPlan,
    source_pool: &[SourcePoint],
    out_dir: &Path,
) -> Result<(), String> {
    let starts = plan
        .starts
        .iter()
        .map(|point| (point.name.as_str(), point))
        .collect::<HashMap<_, _>>();
    let algorithms = plan
        .algorithms
        .iter()
        .map(|spec| (spec.id(), spec))
        .collect::<HashMap<_, _>>();
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(plan.parallelism)
        .build()
        .map_err(|error| format!("build Rayon pool: {error}"))?;
    let results = pool.install(|| {
        plan.runs
            .par_iter()
            .map(|run| {
                let start = starts.get(run.start_id.as_str()).ok_or_else(|| {
                    format!("resolved run references absent start {}", run.start_id)
                })?;
                let algorithm = algorithms.get(run.algorithm_id.as_str()).ok_or_else(|| {
                    format!(
                        "resolved run references absent algorithm {}",
                        run.algorithm_id
                    )
                })?;
                run_one(plan, run, algorithm, start, source_pool)
            })
            .collect::<Vec<_>>()
    });
    let mut artifacts = results.into_iter().collect::<Result<Vec<_>, String>>()?;
    artifacts.sort_by(|left, right| {
        left.run
            .as_ref()
            .expect("completed run")
            .run_id
            .cmp(&right.run.as_ref().expect("completed run").run_id)
    });
    let evaluations = artifacts
        .iter()
        .flat_map(|artifact| artifact.evaluations.clone())
        .collect::<Vec<_>>();
    let proposals = artifacts
        .iter()
        .flat_map(|artifact| artifact.proposals.clone())
        .collect::<Vec<_>>();
    let rounds = artifacts
        .iter()
        .flat_map(|artifact| artifact.rounds.clone())
        .collect::<Vec<_>>();
    let runs = artifacts
        .into_iter()
        .map(|artifact| artifact.run.expect("completed run"))
        .collect::<Vec<_>>();
    write_jsonl(&out_dir.join("evaluations.jsonl"), &evaluations)?;
    write_jsonl(&out_dir.join("proposals.jsonl"), &proposals)?;
    write_jsonl(&out_dir.join("rounds.jsonl"), &rounds)?;
    // Completion rows are written after every detail table.
    write_jsonl(&out_dir.join("runs.jsonl"), &runs)?;
    Ok(())
}

fn run_one(
    plan: &ResolvedPlan,
    run: &ResolvedRun,
    algorithm_spec: &AlgorithmSpec,
    start: &SourcePoint,
    source_pool: &[SourcePoint],
) -> Result<RunArtifacts, String> {
    let wall_started = Instant::now();
    let mut artifacts = RunArtifacts::default();
    let mut evaluator = Evaluator::new(plan.evaluator.clone());
    let start_duals = unflatten(&start.dual_flat)?;
    let initial_charged = plan.charge_initial;
    let mut charged_calls = usize::from(initial_charged);
    let initial_id = format!("{}--e{:06}", run.run_id, 0);
    let initial = evaluator.evaluate(
        &run.run_id,
        initial_id.clone(),
        None,
        "initial",
        charged_calls,
        initial_charged,
        start_duals,
    );
    if !initial.row.usable_by_optimizer {
        return Err(format!(
            "{} initial evaluation failed: {:?}",
            run.run_id, initial.row.error
        ));
    }
    let initial_sys = initial
        .row
        .sys
        .ok_or_else(|| format!("{} initial evaluation lacks sys", run.run_id))?;
    artifacts.evaluations.push(initial.row.clone());
    let mut evaluator_compute_ms = if initial_charged {
        initial.row.total_ms
    } else {
        0.0
    };
    let mut optimizer_compute_ms = 0.0;
    let mut optimizer = algorithms::construct(
        algorithm_spec,
        run.seed,
        &initial,
        source_pool,
        &plan.evaluator,
    )?;
    let mut best = initial.clone();
    let mut physical_evaluations = usize::from(initial.physical_evaluation);
    let mut invalid_evaluations = usize::from(initial.row.status == "invalid");
    let mut indeterminate_evaluations = usize::from(initial.row.status == "indeterminate_geometry");
    let mut exact_fallback_evaluations = usize::from(initial.row.status == "exact_fallback");
    let mut round_index = 0usize;
    let mut next_evaluation_serial = 1usize;
    let mut stop_reason = None;
    while charged_calls < plan.budget {
        if plan
            .compute_budget_ms
            .is_some_and(|budget| evaluator_compute_ms + optimizer_compute_ms >= budget)
        {
            stop_reason = Some("compute_budget_exhausted".to_string());
            break;
        }
        if plan
            .stop_sys_threshold
            .is_some_and(|threshold| best.row.sys.is_some_and(|value| value >= threshold))
        {
            stop_reason = Some("sys_threshold_reached".to_string());
            break;
        }
        if let Some(reason) = optimizer.is_done() {
            stop_reason = Some(reason);
            break;
        }
        let remaining = plan.budget - charged_calls;
        let ask_started = Instant::now();
        let proposals = optimizer.ask(remaining)?;
        let ask_ms = ask_started.elapsed().as_secs_f64() * 1000.0;
        optimizer_compute_ms += ask_ms;
        if proposals.is_empty() {
            stop_reason = Some("optimizer_returned_no_proposals".to_string());
            break;
        }
        if proposals.len() > remaining {
            return Err(format!(
                "{} optimizer proposed {} points with budget {}",
                run.run_id,
                proposals.len(),
                remaining
            ));
        }
        let round_id = format!("{}--r{:05}", run.run_id, round_index);
        let algorithm_state_before = optimizer.algorithm_state();
        let best_before = best.clone();
        let calls_before = charged_calls;
        let compute_before = evaluator_compute_ms + optimizer_compute_ms - ask_ms;
        let geometric_reference_kind = proposals
            .first()
            .and_then(|proposal| proposal.geometric_reference_kind.clone());
        let geometric_reference_duals = proposals
            .first()
            .and_then(|proposal| proposal.geometric_reference_duals.clone());
        if proposals.iter().any(|proposal| {
            proposal.geometric_reference_kind != geometric_reference_kind
                || proposal.geometric_reference_duals != geometric_reference_duals
        }) {
            return Err(format!(
                "{} round {} has inconsistent geometric references",
                run.run_id, round_index
            ));
        }
        let mut observations = Vec::with_capacity(proposals.len());
        let mut proposal_ids = Vec::with_capacity(proposals.len());
        for (proposal_index, proposal) in proposals.into_iter().enumerate() {
            if proposal_index > 0
                && plan
                    .compute_budget_ms
                    .is_some_and(|budget| evaluator_compute_ms + optimizer_compute_ms >= budget)
            {
                break;
            }
            let proposal_id = format!("{round_id}--p{proposal_index:03}");
            let evaluation_id = format!("{}--e{:06}", run.run_id, next_evaluation_serial);
            next_evaluation_serial += 1;
            charged_calls += 1;
            let reference = proposal.geometric_reference_duals.as_deref();
            let displacement = reference.map(|value| displacement_l2(value, &proposal.duals));
            let normalized_displacement = reference.and_then(|value| {
                let norm = l2_norm(value);
                (norm > 0.0).then(|| displacement_l2(value, &proposal.duals) / norm)
            });
            artifacts.proposals.push(ProposalRow {
                schema_version: SCHEMA_VERSION,
                run_id: run.run_id.clone(),
                round_id: round_id.clone(),
                proposal_id: proposal_id.clone(),
                evaluation_id: evaluation_id.clone(),
                proposal_index,
                baseline_evaluation_id: proposal.baseline_evaluation_id,
                displacement_l2: displacement,
                normalized_displacement_l2: normalized_displacement,
                algorithm_fields: proposal.fields,
            });
            let evaluation = evaluator.evaluate(
                &run.run_id,
                evaluation_id,
                Some(proposal_id.clone()),
                "proposal",
                charged_calls,
                true,
                proposal.duals,
            );
            evaluator_compute_ms += evaluation.row.total_ms;
            physical_evaluations += usize::from(evaluation.physical_evaluation);
            invalid_evaluations += usize::from(evaluation.row.status == "invalid");
            indeterminate_evaluations +=
                usize::from(evaluation.row.status == "indeterminate_geometry");
            exact_fallback_evaluations += usize::from(evaluation.row.status == "exact_fallback");
            if evaluation.row.usable_by_optimizer
                && evaluation
                    .row
                    .sys
                    .is_some_and(|sys| sys > best.row.sys.expect("best evaluation always has sys"))
            {
                best = evaluation.clone();
            }
            artifacts.evaluations.push(evaluation.row.clone());
            proposal_ids.push(proposal_id.clone());
            observations.push(crate::algorithm::EvaluatedProposal {
                proposal_id,
                evaluation,
            });
        }
        if observations.is_empty() {
            stop_reason = Some("compute_budget_exhausted_during_proposal_generation".to_string());
            break;
        }
        let tell_started = Instant::now();
        let outcome = optimizer.tell(&observations)?;
        let algorithm_state_after = optimizer.algorithm_state();
        let tell_ms = tell_started.elapsed().as_secs_f64() * 1000.0;
        optimizer_compute_ms += tell_ms;
        let selected = outcome
            .selected
            .iter()
            .map(|(index, weight)| {
                let observation = observations.get(*index).ok_or_else(|| {
                    format!(
                        "{} optimizer selected absent proposal index {}",
                        run.run_id, index
                    )
                })?;
                if !weight.is_finite() || *weight < 0.0 {
                    return Err(format!(
                        "{} optimizer returned invalid selection weight {}",
                        run.run_id, weight
                    ));
                }
                Ok(SelectedProposal {
                    proposal_id: observation.proposal_id.clone(),
                    weight: *weight,
                })
            })
            .collect::<Result<Vec<_>, String>>()?;
        artifacts.rounds.push(RoundRow {
            schema_version: SCHEMA_VERSION,
            run_id: run.run_id.clone(),
            round_id,
            round_index,
            charged_calls_before: calls_before,
            charged_calls_after: charged_calls,
            charged_compute_ms_before: compute_before,
            charged_compute_ms_after: evaluator_compute_ms + optimizer_compute_ms,
            best_evaluation_id_before: best_before.row.evaluation_id.clone(),
            best_evaluation_id_after: best.row.evaluation_id.clone(),
            best_sys_before: best_before.row.sys.expect("best has sys"),
            best_sys_after: best.row.sys.expect("best has sys"),
            algorithm_state_before,
            algorithm_state_after,
            geometric_reference_kind,
            geometric_reference_dual_flat: geometric_reference_duals
                .as_deref()
                .map(crate::quotient::flatten),
            ask_ms,
            tell_ms,
            proposal_ids,
            selected,
            stop_reason: outcome.stop_reason.clone(),
            algorithm_fields: outcome.fields,
        });
        round_index += 1;
        if let Some(reason) = outcome.stop_reason {
            stop_reason = Some(reason);
            break;
        }
        if plan
            .stop_sys_threshold
            .is_some_and(|threshold| best.row.sys.is_some_and(|value| value >= threshold))
        {
            stop_reason = Some("sys_threshold_reached".to_string());
            break;
        }
    }
    let stop_reason = stop_reason.unwrap_or_else(|| {
        if charged_calls >= plan.budget {
            "budget_exhausted".to_string()
        } else {
            "unknown".to_string()
        }
    });
    artifacts.run = Some(RunRow {
        schema_version: SCHEMA_VERSION,
        run_id: run.run_id.clone(),
        start_id: run.start_id.clone(),
        algorithm_id: run.algorithm_id.clone(),
        algorithm_kind: run.algorithm_kind.clone(),
        seed: run.seed,
        budget: plan.budget,
        compute_budget_ms: plan.compute_budget_ms,
        stop_sys_threshold: plan.stop_sys_threshold,
        charge_initial: plan.charge_initial,
        initial_evaluation_id: initial_id,
        initial_sys,
        best_evaluation_id: best.row.evaluation_id,
        best_sys: best.row.sys.expect("best has sys"),
        final_algorithm_state: optimizer.algorithm_state(),
        charged_calls,
        evaluator_compute_ms,
        optimizer_compute_ms,
        charged_compute_ms: evaluator_compute_ms + optimizer_compute_ms,
        compute_budget_overshoot_ms: plan
            .compute_budget_ms
            .map(|budget| (evaluator_compute_ms + optimizer_compute_ms - budget).max(0.0))
            .unwrap_or(0.0),
        physical_evaluations,
        invalid_evaluations,
        indeterminate_evaluations,
        exact_fallback_evaluations,
        rounds: round_index,
        stop_reason,
        wall_ms: wall_started.elapsed().as_secs_f64() * 1000.0,
    });
    Ok(artifacts)
}

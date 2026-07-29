use nalgebra::{DVector, Vector4};
use optimizer_runs::algorithm::Proposal;
use optimizer_runs::algorithms;
use optimizer_runs::branch_model::{
    BranchExtensionMode, BranchModel, BranchModelConfig, NormMode, SliceMode,
};
use optimizer_runs::evaluator::{Evaluation, Evaluator, EvaluatorConfig, GeometryMode, VolumeMode};
use optimizer_runs::manifest::AlgorithmSpec;
use optimizer_runs::quotient::{add_flat_direction, displacement_l2, l2_norm, quotient_basis};
use optimizer_runs::schedule::DistanceScheduleSpec;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::{HashMap, HashSet};
use std::fs::{create_dir_all, File};
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Instant;
use symplectic::known_polytopes::hko_pentagon;

const RADII: [f64; 5] = [1.0e-3, 3.0e-4, 1.0e-4, 3.0e-5, 1.0e-5];
const MAX_ACCEPTED_STEPS: usize = 10;
const MIN_GAIN: f64 = 1.0e-12;

#[derive(Clone, Copy)]
enum RunMode {
    Debug,
    Full,
}

impl RunMode {
    fn name(self) -> &'static str {
        match self {
            Self::Debug => "debug",
            Self::Full => "full",
        }
    }

    fn radii(self) -> &'static [f64] {
        match self {
            Self::Debug => &[1.0e-3, 1.0e-4],
            Self::Full => &RADII,
        }
    }

    fn accepted_step_cap(self) -> usize {
        match self {
            Self::Debug => 1,
            Self::Full => MAX_ACCEPTED_STEPS,
        }
    }
}

#[derive(Debug, Deserialize)]
struct CheckpointPacket {
    checkpoints: Vec<Checkpoint>,
}

#[derive(Debug, Deserialize)]
struct Checkpoint {
    checkpoint_id: String,
    base_sys: f64,
    dual_flat: Vec<f64>,
}

#[derive(Clone)]
struct State {
    state_id: String,
    role: String,
    recorded_sys: Option<f64>,
    duals: Vec<Vector4<f64>>,
    direction_class: Option<String>,
    source_distance: Option<f64>,
    model_radii: Option<Vec<f64>>,
    accepted_step_cap: Option<usize>,
    reference: Option<ReferenceState>,
}

#[derive(Clone)]
struct ReferenceState {
    reference_id: String,
    sys: f64,
    duals: Vec<Vector4<f64>>,
}

#[derive(Debug, Deserialize)]
struct ExternalStatePacket {
    reference: ExternalReference,
    states: Vec<ExternalState>,
}

#[derive(Debug, Deserialize)]
struct ExternalReference {
    reference_id: String,
    sys: f64,
    dual_flat: Vec<f64>,
}

#[derive(Debug, Deserialize)]
struct ExternalState {
    state_id: String,
    role: String,
    recorded_sys: f64,
    dual_flat: Vec<f64>,
    direction_class: String,
    source_distance: f64,
    model_radii: Vec<f64>,
    accepted_step_cap: usize,
}

#[derive(Serialize)]
struct CandidateRow {
    state_id: String,
    accepted_step: usize,
    candidate_id: String,
    family: String,
    normalized_radius: f64,
    fallback_scan: bool,
    usable: bool,
    base_sys: f64,
    candidate_sys: Option<f64>,
    delta_sys: Option<f64>,
    absolute_distance: f64,
    normalized_distance: f64,
    slope: Option<f64>,
    base_winning_sigma: Option<Vec<usize>>,
    candidate_winning_sigma: Option<Vec<usize>>,
    candidate_winner_in_base_branch_set: Option<bool>,
    candidate_winner_in_extension_pool: Option<bool>,
    candidate_winner_base_branch_predicted_delta: Option<f64>,
    candidate_winner_extension_predicted_delta: Option<f64>,
    incidence_changed: Option<bool>,
    base_geometry_indeterminate_count: usize,
    candidate_geometry_indeterminate_count: usize,
    base_vertex_indeterminate_count: usize,
    candidate_vertex_indeterminate_count: usize,
    sigma_iterations: Option<u64>,
    geometry_ms: f64,
    volume_ms: f64,
    capacity_ms: f64,
    total_ms: f64,
    proposal_fields: Value,
    selected: bool,
}

#[derive(Serialize)]
struct StepRow {
    state_id: String,
    role: String,
    accepted_step: usize,
    candidate_id: String,
    family: String,
    normalized_radius: f64,
    sys_before: f64,
    sys_after: f64,
    delta_sys: f64,
    absolute_distance: f64,
    normalized_distance: f64,
    slope: f64,
    cumulative_path_length: f64,
    cumulative_normalized_path_length: f64,
    cumulative_gain: f64,
    turning_angle_radians: Option<f64>,
    winning_sigma_changed: bool,
    full_sys_evaluations_so_far: usize,
    reference_id: Option<String>,
    distance_to_reference: Option<f64>,
    recovered_gap_fraction: Option<f64>,
}

#[derive(Serialize)]
struct StateSummary {
    state_id: String,
    role: String,
    recorded_sys: Option<f64>,
    recomputed_initial_sys: Option<f64>,
    initial_minus_recorded: Option<f64>,
    final_sys: Option<f64>,
    accepted_steps: usize,
    cumulative_path_length: f64,
    cumulative_normalized_path_length: f64,
    cumulative_gain: f64,
    final_step_slope: Option<f64>,
    full_sys_evaluations: usize,
    stop_reason: String,
    direction_class: Option<String>,
    source_distance: Option<f64>,
    reference_id: Option<String>,
    reference_sys: Option<f64>,
    initial_reference_gap: Option<f64>,
    initial_reference_distance: Option<f64>,
    final_reference_distance: Option<f64>,
    recovered_gap_fraction: Option<f64>,
    reference_distance_contraction: Option<f64>,
}

fn main() -> Result<(), String> {
    let (out_dir, mode, states_json, mirror_model_directions) = parse_cli()?;
    create_dir_all(&out_dir).map_err(|error| error.to_string())?;
    let started = Instant::now();
    let external_states = states_json.is_some();
    let mut states = load_states(states_json.as_deref())?;
    if matches!(mode, RunMode::Debug) && !external_states {
        states.truncate(1);
    }
    let evaluator_config = EvaluatorConfig {
        geometry_mode: GeometryMode::F64,
        volume_mode: VolumeMode::F64,
        accept_indeterminate_geometry: true,
        exact_geometry_fallback: false,
        cache_within_run: false,
    };
    let mut evaluator = Evaluator::new(evaluator_config.clone());
    let mut candidate_writer = jsonl_writer(out_dir.join("candidates.jsonl"))?;
    let mut step_writer = jsonl_writer(out_dir.join("steps.jsonl"))?;
    let mut state_summary_writer = jsonl_writer(out_dir.join("state-summaries.jsonl"))?;
    let mut summaries = Vec::new();

    for state in states {
        let model_radii = state
            .model_radii
            .clone()
            .unwrap_or_else(|| mode.radii().to_vec());
        let accepted_step_cap = state
            .accepted_step_cap
            .unwrap_or_else(|| mode.accepted_step_cap());
        let reference_id = state
            .reference
            .as_ref()
            .map(|reference| reference.reference_id.clone());
        let reference_sys = state.reference.as_ref().map(|reference| reference.sys);
        let initial_reference_distance = state
            .reference
            .as_ref()
            .map(|reference| displacement_l2(&state.duals, &reference.duals));
        let run_id = format!("continuation--{}", state.state_id);
        let mut logical_call = 0usize;
        let mut current = evaluator.evaluate(
            &run_id,
            format!("{run_id}--initial"),
            None,
            "initial",
            logical_call,
            true,
            state.duals,
        );
        logical_call += usize::from(current.physical_evaluation);
        let initial_sys = current.row.sys;
        if !current.row.usable_by_optimizer || initial_sys.is_none() {
            let summary = StateSummary {
                state_id: state.state_id,
                role: state.role,
                recorded_sys: state.recorded_sys,
                recomputed_initial_sys: initial_sys,
                initial_minus_recorded: initial_sys.zip(state.recorded_sys).map(|(a, b)| a - b),
                final_sys: initial_sys,
                accepted_steps: 0,
                cumulative_path_length: 0.0,
                cumulative_normalized_path_length: 0.0,
                cumulative_gain: 0.0,
                final_step_slope: None,
                full_sys_evaluations: logical_call,
                stop_reason: format!("initial_evaluation_failed:{}", current.row.status),
                direction_class: state.direction_class,
                source_distance: state.source_distance,
                reference_id,
                reference_sys,
                initial_reference_gap: initial_sys
                    .zip(reference_sys)
                    .map(|(initial, reference)| reference - initial),
                initial_reference_distance,
                final_reference_distance: initial_reference_distance,
                recovered_gap_fraction: None,
                reference_distance_contraction: None,
            };
            write_jsonl(&mut state_summary_writer, &summary)?;
            state_summary_writer
                .flush()
                .map_err(|error| error.to_string())?;
            summaries.push(summary);
            continue;
        }

        let initial_sys = initial_sys.unwrap();
        let initial_reference_gap = reference_sys.map(|reference| reference - initial_sys);
        let mut cumulative_path = 0.0;
        let mut cumulative_normalized_path = 0.0;
        let mut previous_direction: Option<DVector<f64>> = None;
        let mut final_slope = None;
        let mut accepted_steps = 0usize;
        let mut stop_reason = "accepted_step_cap".to_string();

        while accepted_steps < accepted_step_cap {
            let mut proposals = model_proposals(
                &current,
                &evaluator_config,
                &model_radii,
                mirror_model_directions,
            )?;
            let mut fallback_scan = false;
            let mut evaluated = evaluate_proposals(
                &mut evaluator,
                &run_id,
                accepted_steps,
                &current,
                &mut logical_call,
                proposals.drain(..),
            );
            if best_positive_index(&evaluated).is_none() {
                fallback_scan = true;
                evaluated.extend(evaluate_proposals(
                    &mut evaluator,
                    &run_id,
                    accepted_steps,
                    &current,
                    &mut logical_call,
                    basis_proposals(&current)?,
                ));
            }

            let selected = best_positive_index(&evaluated);
            for (index, candidate) in evaluated.iter().enumerate() {
                let row = CandidateRow {
                    state_id: state.state_id.clone(),
                    accepted_step: accepted_steps,
                    candidate_id: candidate.id.clone(),
                    family: candidate.family.clone(),
                    normalized_radius: candidate.radius,
                    fallback_scan: candidate.fallback,
                    usable: candidate.evaluation.row.usable_by_optimizer,
                    base_sys: current.row.sys.unwrap(),
                    candidate_sys: candidate.evaluation.row.sys,
                    delta_sys: candidate.delta,
                    absolute_distance: candidate.absolute_distance,
                    normalized_distance: candidate.normalized_distance,
                    slope: candidate.slope,
                    base_winning_sigma: current.row.winning_sigma.clone(),
                    candidate_winning_sigma: candidate.evaluation.row.winning_sigma.clone(),
                    candidate_winner_in_base_branch_set: candidate
                        .evaluation
                        .row
                        .winning_sigma
                        .as_ref()
                        .zip(candidate.base_candidate_sigmas.as_ref())
                        .map(|(sigma, candidates)| candidates.contains(sigma)),
                    candidate_winner_in_extension_pool: candidate
                        .evaluation
                        .row
                        .winning_sigma
                        .as_ref()
                        .zip(candidate.extended_sigmas.as_ref())
                        .map(|(sigma, candidates)| candidates.contains(sigma)),
                    candidate_winner_base_branch_predicted_delta: candidate
                        .evaluation
                        .row
                        .winning_sigma
                        .as_ref()
                        .and_then(|sigma| {
                            candidate
                                .base_branch_predicted_deltas
                                .as_ref()?
                                .get(sigma)
                                .copied()
                        }),
                    candidate_winner_extension_predicted_delta: candidate
                        .evaluation
                        .row
                        .winning_sigma
                        .as_ref()
                        .and_then(|sigma| {
                            candidate
                                .extension_predicted_deltas
                                .as_ref()?
                                .get(sigma)
                                .copied()
                        }),
                    incidence_changed: current
                        .context
                        .as_ref()
                        .zip(candidate.evaluation.context.as_ref())
                        .map(|(base, target)| {
                            base.polytope.vertex_facet_incidence
                                != target.polytope.vertex_facet_incidence
                        }),
                    base_geometry_indeterminate_count: current.row.geometry_indeterminate_count,
                    candidate_geometry_indeterminate_count: candidate
                        .evaluation
                        .row
                        .geometry_indeterminate_count,
                    base_vertex_indeterminate_count: current.row.vertex_indeterminate_count,
                    candidate_vertex_indeterminate_count: candidate
                        .evaluation
                        .row
                        .vertex_indeterminate_count,
                    sigma_iterations: candidate.evaluation.row.sigma_iterations,
                    geometry_ms: candidate.evaluation.row.geometry_ms,
                    volume_ms: candidate.evaluation.row.volume_ms,
                    capacity_ms: candidate.evaluation.row.capacity_ms,
                    total_ms: candidate.evaluation.row.total_ms,
                    proposal_fields: candidate.fields.clone(),
                    selected: selected == Some(index),
                };
                write_jsonl(&mut candidate_writer, &row)?;
            }

            let Some(selected) = selected else {
                stop_reason = if fallback_scan {
                    "no_validated_improvement_in_models_or_signed_basis"
                } else {
                    "no_validated_improvement_in_models"
                }
                .to_string();
                break;
            };
            let chosen = evaluated.swap_remove(selected);
            let before_sys = current.row.sys.unwrap();
            let after_sys = chosen.evaluation.row.sys.unwrap();
            let direction = displacement_vector(&current.duals, &chosen.evaluation.duals);
            let turning_angle = previous_direction
                .as_ref()
                .map(|previous| previous.dot(&direction).clamp(-1.0, 1.0).acos());
            previous_direction = Some(direction);
            cumulative_path += chosen.absolute_distance;
            cumulative_normalized_path += chosen.normalized_distance;
            final_slope = chosen.slope;
            accepted_steps += 1;
            let distance_to_reference = state
                .reference
                .as_ref()
                .map(|reference| displacement_l2(&chosen.evaluation.duals, &reference.duals));
            let recovered_gap_fraction = initial_reference_gap
                .filter(|gap| *gap > MIN_GAIN)
                .map(|gap| (after_sys - initial_sys) / gap);
            let step = StepRow {
                state_id: state.state_id.clone(),
                role: state.role.clone(),
                accepted_step: accepted_steps,
                candidate_id: chosen.id.clone(),
                family: chosen.family.clone(),
                normalized_radius: chosen.radius,
                sys_before: before_sys,
                sys_after: after_sys,
                delta_sys: after_sys - before_sys,
                absolute_distance: chosen.absolute_distance,
                normalized_distance: chosen.normalized_distance,
                slope: chosen.slope.unwrap(),
                cumulative_path_length: cumulative_path,
                cumulative_normalized_path_length: cumulative_normalized_path,
                cumulative_gain: after_sys - initial_sys,
                turning_angle_radians: turning_angle,
                winning_sigma_changed: current.row.winning_sigma
                    != chosen.evaluation.row.winning_sigma,
                full_sys_evaluations_so_far: logical_call,
                reference_id: reference_id.clone(),
                distance_to_reference,
                recovered_gap_fraction,
            };
            write_jsonl(&mut step_writer, &step)?;
            current = chosen.evaluation;
        }

        let final_reference_distance = state
            .reference
            .as_ref()
            .map(|reference| displacement_l2(&current.duals, &reference.duals));
        let recovered_gap_fraction = initial_reference_gap
            .filter(|gap| *gap > MIN_GAIN)
            .map(|gap| (current.row.sys.unwrap() - initial_sys) / gap);
        let reference_distance_contraction = initial_reference_distance
            .zip(final_reference_distance)
            .filter(|(initial, _)| *initial > 0.0)
            .map(|(initial, final_distance)| 1.0 - final_distance / initial);
        let summary = StateSummary {
            state_id: state.state_id,
            role: state.role,
            recorded_sys: state.recorded_sys,
            recomputed_initial_sys: Some(initial_sys),
            initial_minus_recorded: state.recorded_sys.map(|value| initial_sys - value),
            final_sys: current.row.sys,
            accepted_steps,
            cumulative_path_length: cumulative_path,
            cumulative_normalized_path_length: cumulative_normalized_path,
            cumulative_gain: current.row.sys.unwrap() - initial_sys,
            final_step_slope: final_slope,
            full_sys_evaluations: logical_call,
            stop_reason,
            direction_class: state.direction_class,
            source_distance: state.source_distance,
            reference_id,
            reference_sys,
            initial_reference_gap,
            initial_reference_distance,
            final_reference_distance,
            recovered_gap_fraction,
            reference_distance_contraction,
        };
        write_jsonl(&mut state_summary_writer, &summary)?;
        candidate_writer
            .flush()
            .map_err(|error| error.to_string())?;
        step_writer.flush().map_err(|error| error.to_string())?;
        state_summary_writer
            .flush()
            .map_err(|error| error.to_string())?;
        summaries.push(summary);
    }

    candidate_writer
        .flush()
        .map_err(|error| error.to_string())?;
    step_writer.flush().map_err(|error| error.to_string())?;
    state_summary_writer
        .flush()
        .map_err(|error| error.to_string())?;
    let summary = json!({
        "schema_version": 1,
        "diagnostic": "repeated validated ascent continuation",
        "run_mode": mode.name(),
        "default_model_radii": mode.radii(),
        "default_maximum_accepted_steps": mode.accepted_step_cap(),
        "external_state_packet": external_states,
        "minimum_accepted_gain": MIN_GAIN,
        "fallback": "signed symmetry-transverse basis at relative radius 1e-5, used only when all model proposals fail",
        "evaluator": evaluator_config,
        "wall_seconds": started.elapsed().as_secs_f64(),
        "states": summaries,
        "claim_boundary": "Stopping means that the tested finite-step models and signed basis scan found no improvement. It does not establish local maximality."
    });
    serde_json::to_writer_pretty(
        File::create(out_dir.join("summary.json")).map_err(|error| error.to_string())?,
        &summary,
    )
    .map_err(|error| error.to_string())?;
    println!("{}", serde_json::to_string_pretty(&summary).unwrap());
    Ok(())
}

struct EvaluatedCandidate {
    id: String,
    family: String,
    radius: f64,
    fallback: bool,
    fields: Value,
    evaluation: Evaluation,
    delta: Option<f64>,
    absolute_distance: f64,
    normalized_distance: f64,
    slope: Option<f64>,
    base_candidate_sigmas: Option<Arc<HashSet<Vec<usize>>>>,
    extended_sigmas: Option<Arc<HashSet<Vec<usize>>>>,
    base_branch_predicted_deltas: Option<Arc<HashMap<Vec<usize>, f64>>>,
    extension_predicted_deltas: Option<Arc<HashMap<Vec<usize>, f64>>>,
}

fn evaluate_proposals(
    evaluator: &mut Evaluator,
    run_id: &str,
    accepted_step: usize,
    current: &Evaluation,
    logical_call: &mut usize,
    proposals: impl Iterator<Item = NamedProposal>,
) -> Vec<EvaluatedCandidate> {
    proposals
        .enumerate()
        .map(|(index, proposal)| {
            let id = format!(
                "{run_id}--s{accepted_step:02}--{}--{index:03}",
                proposal.family
            );
            let absolute_distance = displacement_l2(&current.duals, &proposal.proposal.duals);
            let normalized_distance = absolute_distance / l2_norm(&current.duals);
            let evaluation = evaluator.evaluate(
                run_id,
                format!("{id}--evaluation"),
                Some(id.clone()),
                "continuation_candidate",
                *logical_call,
                true,
                proposal.proposal.duals,
            );
            *logical_call += usize::from(evaluation.physical_evaluation);
            let delta = evaluation
                .row
                .sys
                .zip(current.row.sys)
                .map(|(after, before)| after - before);
            let slope = delta
                .and_then(|gain| (absolute_distance > 0.0).then_some(gain / absolute_distance));
            EvaluatedCandidate {
                id,
                family: proposal.family,
                radius: proposal.radius,
                fallback: proposal.fallback,
                fields: proposal.proposal.fields,
                evaluation,
                delta,
                absolute_distance,
                normalized_distance,
                slope,
                base_candidate_sigmas: proposal.base_candidate_sigmas,
                extended_sigmas: proposal.extended_sigmas,
                base_branch_predicted_deltas: proposal.base_branch_predicted_deltas,
                extension_predicted_deltas: proposal.extension_predicted_deltas,
            }
        })
        .collect()
}

fn best_positive_index(candidates: &[EvaluatedCandidate]) -> Option<usize> {
    candidates
        .iter()
        .enumerate()
        .filter(|(_, candidate)| {
            candidate.evaluation.row.usable_by_optimizer
                && candidate.delta.is_some_and(|gain| gain > MIN_GAIN)
        })
        .max_by(|(_, left), (_, right)| left.delta.unwrap().total_cmp(&right.delta.unwrap()))
        .map(|(index, _)| index)
}

struct NamedProposal {
    family: String,
    radius: f64,
    fallback: bool,
    proposal: Proposal,
    base_candidate_sigmas: Option<Arc<HashSet<Vec<usize>>>>,
    extended_sigmas: Option<Arc<HashSet<Vec<usize>>>>,
    base_branch_predicted_deltas: Option<Arc<HashMap<Vec<usize>, f64>>>,
    extension_predicted_deltas: Option<Arc<HashMap<Vec<usize>, f64>>>,
}

fn model_proposals(
    current: &Evaluation,
    evaluator_config: &EvaluatorConfig,
    radii: &[f64],
    mirror_model_directions: bool,
) -> Result<Vec<NamedProposal>, String> {
    let mut result = Vec::new();
    for candidate_window_relative in [0.1, 1.0] {
        let family = format!("gap-window-{candidate_window_relative:.1}");
        let model = BranchModel::build(
            current,
            &BranchModelConfig {
                candidate_window_relative,
                extension_mode: BranchExtensionMode::TransitionBlockedAdmissible,
            },
        )?;
        let base_candidate_sigmas = Arc::new(
            model
                .candidates
                .iter()
                .map(|branch| branch.sigma.clone())
                .collect(),
        );
        let extended_sigmas = Arc::new(
            model
                .extended
                .iter()
                .map(|branch| branch.sigma.clone())
                .collect(),
        );
        for (radius_index, &radius) in radii.iter().enumerate() {
            let solution = model.solve_euclidean(
                &current.duals,
                radius,
                SliceMode::SymmetryTransverse,
                1.0,
            )?;
            if solution.predicted_delta <= 0.0 {
                continue;
            }
            let target = add_flat_direction(
                &current.duals,
                &DVector::from_vec(solution.displacement_flat.clone()),
                1.0,
            );
            let branch_predicted_delta = |branch: &optimizer_runs::branch_model::LinearBranch| {
                branch.gap
                    + branch
                        .gradient
                        .iter()
                        .flat_map(|entry| entry.iter())
                        .zip(solution.displacement_flat.iter())
                        .map(|(gradient, displacement)| gradient * displacement)
                        .sum::<f64>()
            };
            let base_branch_predicted_deltas = Arc::new(
                model
                    .candidates
                    .iter()
                    .map(|branch| (branch.sigma.clone(), branch_predicted_delta(branch)))
                    .collect(),
            );
            let extension_predicted_deltas = Arc::new(
                model
                    .extended
                    .iter()
                    .map(|branch| (branch.sigma.clone(), branch_predicted_delta(branch)))
                    .collect(),
            );
            result.push(NamedProposal {
                family: family.clone(),
                radius,
                fallback: false,
                proposal: Proposal {
                    duals: target,
                    baseline_evaluation_id: Some(current.row.evaluation_id.clone()),
                    geometric_reference_kind: Some("current_state".to_string()),
                    geometric_reference_duals: Some(current.duals.clone()),
                    fields: json!({
                        "scheduled_normalized_distance": radius,
                        "candidate_window_relative": candidate_window_relative,
                        "branch_extension_mode": BranchExtensionMode::TransitionBlockedAdmissible,
                        "extension_reachability_scale": 1.0,
                        "slice_mode": SliceMode::SymmetryTransverse,
                        "norm_mode": NormMode::EuclideanL2,
                        "predicted_delta": solution.predicted_delta,
                        "predicted_winning_sigma": solution.predicted_winning_sigma,
                        "candidate_branch_count": solution.candidate_branch_count,
                        "extended_branch_count": model.extended.len(),
                        "negative_beta_extended_branch_count": model.extended
                            .iter()
                            .filter(|branch| branch.beta_margin < 0.0)
                            .count(),
                        "reachable_extended_branch_count": solution.reachable_extended_branch_count,
                        "represented_branch_count": solution.represented_branch_count,
                        "displacement_flat": solution.displacement_flat.clone(),
                        "phase_ms": {
                            "shared_model_build": (radius_index == 0).then_some(json!({
                                "candidate_window_search": model.timing.candidate_search_ms,
                                "branch_derivative": model.timing.derivative_ms,
                                "branch_extension_enumeration": model.timing.extension_enumeration_ms,
                                "total": model.timing.total_ms,
                            })),
                            "model_solve": solution.solve_ms,
                        },
                    }),
                },
                base_candidate_sigmas: Some(Arc::clone(&base_candidate_sigmas)),
                extended_sigmas: Some(Arc::clone(&extended_sigmas)),
                base_branch_predicted_deltas: Some(base_branch_predicted_deltas),
                extension_predicted_deltas: Some(extension_predicted_deltas),
            });
            if mirror_model_directions {
                let mirrored_displacement: Vec<f64> = solution
                    .displacement_flat
                    .iter()
                    .map(|value| -*value)
                    .collect();
                let mirrored_target = add_flat_direction(
                    &current.duals,
                    &DVector::from_vec(mirrored_displacement.clone()),
                    1.0,
                );
                result.push(NamedProposal {
                    family: format!("mirrored-{family}"),
                    radius,
                    fallback: false,
                    proposal: Proposal {
                        duals: mirrored_target,
                        baseline_evaluation_id: Some(current.row.evaluation_id.clone()),
                        geometric_reference_kind: Some("current_state".to_string()),
                        geometric_reference_duals: Some(current.duals.clone()),
                        fields: json!({
                            "direction_kind": "mirror_of_positive_model_solution",
                            "source_predicted_delta": solution.predicted_delta,
                            "scheduled_normalized_distance": radius,
                            "candidate_window_relative": candidate_window_relative,
                            "displacement_flat": mirrored_displacement,
                        }),
                    },
                    base_candidate_sigmas: Some(Arc::clone(&base_candidate_sigmas)),
                    extended_sigmas: Some(Arc::clone(&extended_sigmas)),
                    base_branch_predicted_deltas: None,
                    extension_predicted_deltas: None,
                });
            }
        }
    }
    for &radius in radii {
        let schedule = DistanceScheduleSpec::FixedSequence {
            distances: vec![radius],
            repeat_last: false,
        };
        let spec = AlgorithmSpec::SafeguardedGradient {
            id: "winning-branch-gradient".to_string(),
            schedule,
            slice_mode: SliceMode::SymmetryTransverse,
        };
        let family = spec.id().to_string();
        let mut optimizer = algorithms::construct(&spec, 0, current, &[], evaluator_config)?;
        for proposal in optimizer.ask(1)? {
            result.push(NamedProposal {
                family: family.clone(),
                radius,
                fallback: false,
                proposal,
                base_candidate_sigmas: None,
                extended_sigmas: None,
                base_branch_predicted_deltas: None,
                extension_predicted_deltas: None,
            });
        }
    }
    Ok(result)
}

fn basis_proposals(current: &Evaluation) -> Result<std::vec::IntoIter<NamedProposal>, String> {
    let radius = 1.0e-5;
    let quotient = quotient_basis(&current.duals)?;
    let absolute_distance = radius * l2_norm(&current.duals);
    let mut proposals = Vec::with_capacity(2 * quotient.slice_basis.len());
    for (basis_index, axis) in quotient.slice_basis.into_iter().enumerate() {
        for sign in [-1.0, 1.0] {
            proposals.push(NamedProposal {
                family: format!("signed-basis-{basis_index:02}-{sign:+.0}"),
                radius,
                fallback: true,
                proposal: Proposal {
                    duals: add_flat_direction(&current.duals, &axis, sign * absolute_distance),
                    baseline_evaluation_id: Some(current.row.evaluation_id.clone()),
                    geometric_reference_kind: Some("current_state".to_string()),
                    geometric_reference_duals: Some(current.duals.clone()),
                    fields: json!({
                        "direction_kind": "signed_symmetry_transverse_basis",
                        "basis_index": basis_index,
                        "sign": sign,
                        "scheduled_normalized_distance": radius,
                    }),
                },
                base_candidate_sigmas: None,
                extended_sigmas: None,
                base_branch_predicted_deltas: None,
                extension_predicted_deltas: None,
            });
        }
    }
    Ok(proposals.into_iter())
}

fn displacement_vector(from: &[Vector4<f64>], to: &[Vector4<f64>]) -> DVector<f64> {
    let mut flat = Vec::with_capacity(from.len() * 4);
    for (left, right) in from.iter().zip(to) {
        flat.extend((right - left).iter().copied());
    }
    let mut direction = DVector::from_vec(flat);
    let norm = direction.norm();
    if norm > 0.0 {
        direction /= norm;
    }
    direction
}

fn load_states(states_json: Option<&Path>) -> Result<Vec<State>, String> {
    if let Some(path) = states_json {
        return load_external_states(path);
    }
    let shallow_path = Path::new("experiments/dev-gradient-ascent/optimizer-comparison/artifacts/heldout-f10-64-finalists-19a8b4dfd-analysis/checkpoint-selection.json");
    let deep_path = Path::new("experiments/dev-gradient-ascent/optimizer-comparison/artifacts/history-f10-16-compute-depth-426ec7a7c-analysis/checkpoint-selection.json");
    let mut states = vec![
        load_checkpoint(
            shallow_path,
            "random_F10_s0_34--history-baseline--c000128",
            "known_improvable_one_second_endpoint",
        )?,
        load_checkpoint(
            deep_path,
            "random_F10_s0_34--history-baseline--c000640",
            "residual_positive_basis_slope_endpoint",
        )?,
        load_checkpoint(
            deep_path,
            "random_F10_s0_44--history-baseline--c000640",
            "stable_negative_basis_slope_endpoint",
        )?,
    ];
    let hko = hko_pentagon();
    states.push(State {
        state_id: "positive_control_hko2024".to_string(),
        role: "theorem_local_maximum_control".to_string(),
        recorded_sys: None,
        duals: hko.dual_vertices_f64.clone(),
        direction_class: None,
        source_distance: None,
        model_radii: None,
        accepted_step_cap: None,
        reference: None,
    });
    Ok(states)
}

fn load_external_states(path: &Path) -> Result<Vec<State>, String> {
    let packet: ExternalStatePacket =
        serde_json::from_reader(File::open(path).map_err(|error| error.to_string())?)
            .map_err(|error| error.to_string())?;
    let reference = ReferenceState {
        reference_id: packet.reference.reference_id,
        sys: packet.reference.sys,
        duals: duals_from_flat(
            &packet.reference.dual_flat,
            &format!("{} reference", path.display()),
        )?,
    };
    packet
        .states
        .into_iter()
        .map(|state| {
            Ok(State {
                state_id: state.state_id,
                role: state.role,
                recorded_sys: Some(state.recorded_sys),
                duals: duals_from_flat(&state.dual_flat, &state.direction_class)?,
                direction_class: Some(state.direction_class),
                source_distance: Some(state.source_distance),
                model_radii: Some(state.model_radii),
                accepted_step_cap: Some(state.accepted_step_cap),
                reference: Some(reference.clone()),
            })
        })
        .collect()
}

fn duals_from_flat(flat: &[f64], label: &str) -> Result<Vec<Vector4<f64>>, String> {
    if flat.len() % 4 != 0 {
        return Err(format!(
            "{label}: dual coordinate count is not divisible by four"
        ));
    }
    Ok(flat
        .chunks_exact(4)
        .map(|chunk| Vector4::new(chunk[0], chunk[1], chunk[2], chunk[3]))
        .collect())
}

fn load_checkpoint(path: &Path, id: &str, role: &str) -> Result<State, String> {
    let packet: CheckpointPacket =
        serde_json::from_reader(File::open(path).map_err(|error| error.to_string())?)
            .map_err(|error| error.to_string())?;
    let checkpoint = packet
        .checkpoints
        .into_iter()
        .find(|row| row.checkpoint_id == id)
        .ok_or_else(|| format!("checkpoint {id} not found in {}", path.display()))?;
    if checkpoint.dual_flat.len() % 4 != 0 {
        return Err(format!(
            "{id}: dual coordinate count is not divisible by four"
        ));
    }
    Ok(State {
        state_id: checkpoint.checkpoint_id,
        role: role.to_string(),
        recorded_sys: Some(checkpoint.base_sys),
        duals: duals_from_flat(&checkpoint.dual_flat, id)?,
        direction_class: None,
        source_distance: None,
        model_radii: None,
        accepted_step_cap: None,
        reference: None,
    })
}

fn parse_cli() -> Result<(PathBuf, RunMode, Option<PathBuf>, bool), String> {
    let mut args = std::env::args().skip(1);
    let mut out_dir = None;
    let mut mode = None;
    let mut states_json = None;
    let mut mirror_model_directions = false;
    while let Some(flag) = args.next() {
        match flag.as_str() {
            "--out-dir" => out_dir = args.next().map(PathBuf::from),
            "--mode" => {
                mode = Some(match args.next().as_deref() {
                    Some("debug") => RunMode::Debug,
                    Some("full") => RunMode::Full,
                    _ => return Err("--mode must be debug or full".to_string()),
                })
            }
            "--states-json" => states_json = args.next().map(PathBuf::from),
            "--mirror-model-directions" => mirror_model_directions = true,
            _ => return Err(format!("unknown argument {flag}")),
        }
    }
    Ok((
        out_dir.ok_or("--out-dir is required")?,
        mode.ok_or("--mode debug|full is required")?,
        states_json,
        mirror_model_directions,
    ))
}

fn jsonl_writer(path: PathBuf) -> Result<BufWriter<File>, String> {
    File::create(path)
        .map(BufWriter::new)
        .map_err(|error| error.to_string())
}

fn write_jsonl(writer: &mut BufWriter<File>, row: &impl Serialize) -> Result<(), String> {
    serde_json::to_writer(&mut *writer, row).map_err(|error| error.to_string())?;
    writeln!(writer).map_err(|error| error.to_string())
}

//! Replay predictor atoms at saved optimizer proposals and controlled distances.

use nalgebra::{DVector, Vector4};
use optimizer_runs::algorithms::nonlinear_candidate_cma::{
    diagnose_named_raw_branch, discover_candidate_universe, evaluate_surrogate, CandidateUniverse,
};
use optimizer_runs::branch_model::BranchModel;
use optimizer_runs::dataset::Dataset;
use optimizer_runs::evaluator::{Evaluation, Evaluator, EvaluatorConfig};
use optimizer_runs::output::{prepare_empty_directory, write_json, write_jsonl};
use optimizer_runs::quotient::{flatten, quotient_basis, unflatten};
use optimizer_runs::schema::EvaluationRow;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs::File;
use std::path::{Path, PathBuf};
use std::time::Instant;

const CONFIG_SCHEMA_VERSION: u32 = 1;
const OUTPUT_SCHEMA_VERSION: u32 = 2;

#[derive(Clone, Debug, Deserialize, Serialize)]
struct Config {
    schema_version: u32,
    source_dataset: PathBuf,
    #[serde(default)]
    algorithm_ids: Vec<String>,
    #[serde(default = "default_pairs_per_run")]
    pairs_per_run: usize,
    #[serde(default = "default_action_windows")]
    action_windows: Vec<f64>,
    #[serde(default = "default_distance_scales")]
    distance_scales: Vec<f64>,
    #[serde(default)]
    max_runs: Option<usize>,
}

fn default_pairs_per_run() -> usize {
    3
}

fn default_action_windows() -> Vec<f64> {
    vec![0.01, 0.10, 0.30, 1.0]
}

fn default_distance_scales() -> Vec<f64> {
    vec![0.5, 1.0, 2.0]
}

#[derive(Clone, Debug)]
struct SelectedPair {
    pair_id: String,
    run_id: String,
    start_id: String,
    algorithm_id: String,
    round_index: usize,
    trajectory_phase: String,
    accepted_by_optimizer: bool,
    anchor: EvaluationRow,
    recorded_target: EvaluationRow,
}

#[derive(Clone, Debug, Serialize)]
struct PairRow {
    schema_version: u32,
    pair_id: String,
    run_id: String,
    start_id: String,
    algorithm_id: String,
    round_index: usize,
    trajectory_phase: String,
    accepted_by_optimizer: bool,
    distance_scale: f64,
    normalized_distance: f64,
    symmetry_transverse_normalized_distance: Option<f64>,
    anchor_sys: f64,
    target_sys: Option<f64>,
    target_status: String,
    target_error: Option<String>,
    target_geometry_route: String,
    target_fallback_reason: Option<String>,
    target_winning_sigma: Option<Vec<usize>>,
    full_evaluation_ms: f64,
}

#[derive(Clone, Debug, Serialize)]
struct AtomRow {
    schema_version: u32,
    pair_id: String,
    run_id: String,
    start_id: String,
    algorithm_id: String,
    round_index: usize,
    trajectory_phase: String,
    accepted_by_optimizer: bool,
    distance_scale: f64,
    normalized_distance: f64,
    symmetry_transverse_normalized_distance: Option<f64>,
    selector: String,
    value_model: String,
    domain_model: String,
    candidate_count: usize,
    represented_branch_count: Option<usize>,
    target_winner_covered: Option<bool>,
    anchor_sys: f64,
    target_sys: Option<f64>,
    predicted_target_sys: Option<f64>,
    predicted_winning_sigma: Option<Vec<usize>>,
    predicted_winner_matches_target: Option<bool>,
    predicted_winner_target_sys: Option<f64>,
    predicted_winner_target_status: Option<String>,
    predicted_winner_target_transition_feasible: Option<bool>,
    predicted_winner_target_raw_status: Option<String>,
    predicted_winner_target_raw_beta_margin: Option<f64>,
    predicted_winner_target_raw_normalized_beta_margin: Option<f64>,
    selected_branch_prediction_error: Option<f64>,
    predicted_delta: Option<f64>,
    actual_delta: Option<f64>,
    prediction_error: Option<f64>,
    sign_correct: Option<bool>,
    geometry_ms: f64,
    volume_ms: f64,
    named_branch_ms: f64,
    model_ms: f64,
}

#[derive(Clone, Debug, Serialize)]
struct WinnerMechanismRow {
    schema_version: u32,
    pair_id: String,
    run_id: String,
    start_id: String,
    algorithm_id: String,
    round_index: usize,
    trajectory_phase: String,
    distance_scale: f64,
    normalized_distance: f64,
    symmetry_transverse_normalized_distance: Option<f64>,
    target_winning_sigma: Vec<usize>,
    same_as_anchor_winner: bool,
    in_anchor_universe: bool,
    in_target_universe: bool,
    anchor_transition_feasible: bool,
    target_transition_feasible: bool,
    anchor_raw_status: String,
    target_raw_status: String,
    anchor_raw_action: Option<f64>,
    target_raw_action: Option<f64>,
    anchor_raw_beta_margin: Option<f64>,
    target_raw_beta_margin: Option<f64>,
    anchor_raw_normalized_beta_margin: Option<f64>,
    target_raw_normalized_beta_margin: Option<f64>,
    anchor_action_window_needed: Option<f64>,
    omission_class: String,
}

#[derive(Clone, Debug, Serialize)]
struct CandidateLifetimeRow {
    schema_version: u32,
    anchor_pair_id: String,
    run_id: String,
    start_id: String,
    algorithm_id: String,
    anchor_round_index: usize,
    target_round_index: usize,
    accepted_steps_after_anchor: usize,
    rounds_after_anchor: usize,
    selector: String,
    candidate_count: usize,
    normalized_distance: f64,
    symmetry_transverse_normalized_distance: Option<f64>,
    target_sys: f64,
    predicted_target_sys: Option<f64>,
    prediction_error: Option<f64>,
    target_winner_covered: bool,
}

#[derive(Clone, Debug, Serialize)]
struct RollbackRow {
    schema_version: u32,
    anchor_pair_id: String,
    run_id: String,
    start_id: String,
    algorithm_id: String,
    anchor_round_index: usize,
    target_round_index: usize,
    target_step: usize,
    target_winning_sigma: Vec<usize>,
    earliest_admissible_step: Option<usize>,
    earliest_within_1e_2_step: Option<usize>,
    earliest_within_1e_3_step: Option<usize>,
    earliest_winner_identity_step: Option<usize>,
    admissible_lead_steps: Option<usize>,
    within_1e_2_lead_steps: Option<usize>,
    within_1e_3_lead_steps: Option<usize>,
    winner_identity_lead_steps: Option<usize>,
    previous_step_admissible: Option<bool>,
    previous_step_gap: Option<f64>,
    target_step_gap: Option<f64>,
}

#[derive(Clone, Debug, Serialize)]
struct Provenance {
    schema_version: u32,
    config_path: String,
    config_blake3: String,
    source_dataset: String,
    source_hashes: BTreeMap<String, String>,
}

#[derive(Clone, Debug, Serialize)]
struct Summary {
    schema_version: u32,
    selected_pairs: usize,
    realized_pair_targets: usize,
    atom_rows: usize,
    winner_mechanism_rows: usize,
    candidate_lifetime_rows: usize,
    rollback_rows: usize,
    skipped_targets: usize,
    elapsed_ms: f64,
    claim_boundary: &'static str,
}

fn main() {
    if let Err(error) = run() {
        eprintln!("optimizer atom replay failed: {error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let started = Instant::now();
    let (config_path, out_dir) = parse_args()?;
    let config: Config = serde_json::from_reader(
        File::open(&config_path)
            .map_err(|error| format!("open {}: {error}", config_path.display()))?,
    )
    .map_err(|error| format!("parse {}: {error}", config_path.display()))?;
    validate_config(&config)?;
    let source_dataset = resolve_path(&config_path, &config.source_dataset);
    let dataset = Dataset::load(&source_dataset)?;
    let selected = select_pairs(&dataset, &config)?;
    prepare_empty_directory(&out_dir)?;
    write_json(&out_dir.join("config.json"), &config)?;
    write_json(
        &out_dir.join("provenance.json"),
        &Provenance {
            schema_version: OUTPUT_SCHEMA_VERSION,
            config_path: config_path.display().to_string(),
            config_blake3: file_blake3(&config_path)?,
            source_dataset: source_dataset.display().to_string(),
            source_hashes: [
                "runs.jsonl",
                "rounds.jsonl",
                "proposals.jsonl",
                "evaluations.jsonl",
            ]
            .into_iter()
            .map(|name| Ok((name.to_string(), file_blake3(&source_dataset.join(name))?)))
            .collect::<Result<_, String>>()?,
        },
    )?;

    let evaluator_config = EvaluatorConfig {
        exact_geometry_fallback: false,
        cache_within_run: true,
        ..EvaluatorConfig::default()
    };
    let mut evaluator = Evaluator::new(evaluator_config.clone());
    let mut pair_rows = Vec::new();
    let mut atom_rows = Vec::new();
    let mut winner_mechanism_rows = Vec::new();
    let mut candidate_lifetime_rows = Vec::new();
    let mut rollback_rows = Vec::new();
    let accepted_states = accepted_trajectory_states(&dataset)?;
    let mut evaluation_serial = 0usize;
    let mut skipped_targets = 0usize;
    for pair in &selected {
        let anchor_duals = unflatten(&pair.anchor.dual_flat)?;
        let recorded_target_duals = unflatten(&pair.recorded_target.dual_flat)?;
        let anchor = evaluate(
            &mut evaluator,
            &pair.pair_id,
            &mut evaluation_serial,
            "anchor",
            anchor_duals.clone(),
        );
        if !anchor.row.usable_by_optimizer {
            skipped_targets += config.distance_scales.len();
            continue;
        }
        let universe = discover_candidate_universe(&anchor)?;
        for &distance_scale in &config.distance_scales {
            let target_duals = interpolate(&anchor_duals, &recorded_target_duals, distance_scale);
            let target = evaluate(
                &mut evaluator,
                &pair.pair_id,
                &mut evaluation_serial,
                "target",
                target_duals,
            );
            let normalized_distance =
                normalized_distance(&anchor.duals, &target.duals).unwrap_or(f64::NAN);
            let symmetry_transverse_normalized_distance =
                symmetry_transverse_normalized_distance(&anchor.duals, &target.duals);
            pair_rows.push(PairRow {
                schema_version: OUTPUT_SCHEMA_VERSION,
                pair_id: pair.pair_id.clone(),
                run_id: pair.run_id.clone(),
                start_id: pair.start_id.clone(),
                algorithm_id: pair.algorithm_id.clone(),
                round_index: pair.round_index,
                trajectory_phase: pair.trajectory_phase.clone(),
                accepted_by_optimizer: pair.accepted_by_optimizer,
                distance_scale,
                normalized_distance,
                symmetry_transverse_normalized_distance,
                anchor_sys: anchor.row.sys.expect("usable anchor has sys"),
                target_sys: target.row.sys,
                target_status: target.row.status.clone(),
                target_error: target.row.error.clone(),
                target_geometry_route: target.row.geometry_route.clone(),
                target_fallback_reason: target.row.fallback_reason.clone(),
                target_winning_sigma: target.row.winning_sigma.clone(),
                full_evaluation_ms: target.row.total_ms,
            });
            if !target.row.usable_by_optimizer {
                skipped_targets += 1;
                continue;
            }
            let target_universe = discover_candidate_universe(&target)?;
            winner_mechanism_rows.push(winner_mechanism_row(
                pair,
                distance_scale,
                normalized_distance,
                symmetry_transverse_normalized_distance,
                &anchor,
                &target,
                &universe,
                &target_universe,
            )?);
            let selectors = selectors(&anchor, &target, &universe, &target_universe, &config);
            for (selector, sigmas) in selectors {
                atom_rows.push(nonlinear_atom(
                    pair,
                    distance_scale,
                    normalized_distance,
                    symmetry_transverse_normalized_distance,
                    &anchor,
                    &target,
                    &selector,
                    &sigmas,
                    &evaluator_config,
                ));
                atom_rows.push(affine_atom(
                    pair,
                    distance_scale,
                    normalized_distance,
                    symmetry_transverse_normalized_distance,
                    &anchor,
                    &target,
                    &selector,
                    &sigmas,
                    &evaluator_config,
                ));
            }
        }
        candidate_lifetime_rows.extend(candidate_lifetime_rows_for_anchor(
            pair,
            &anchor,
            &universe,
            accepted_states
                .get(&pair.run_id)
                .map(Vec::as_slice)
                .unwrap_or_default(),
            &config,
            &evaluator_config,
        )?);
        rollback_rows.extend(rollback_rows_for_anchor(
            pair,
            &anchor,
            accepted_states
                .get(&pair.run_id)
                .map(Vec::as_slice)
                .unwrap_or_default(),
            &evaluator_config,
        )?);
    }
    write_jsonl(&out_dir.join("pairs.jsonl"), &pair_rows)?;
    write_jsonl(&out_dir.join("atoms.jsonl"), &atom_rows)?;
    write_jsonl(
        &out_dir.join("winner-mechanisms.jsonl"),
        &winner_mechanism_rows,
    )?;
    write_jsonl(
        &out_dir.join("candidate-lifetimes.jsonl"),
        &candidate_lifetime_rows,
    )?;
    write_jsonl(&out_dir.join("rollbacks.jsonl"), &rollback_rows)?;
    write_json(
        &out_dir.join("summary.json"),
        &Summary {
            schema_version: OUTPUT_SCHEMA_VERSION,
            selected_pairs: selected.len(),
            realized_pair_targets: pair_rows.len(),
            atom_rows: atom_rows.len(),
            winner_mechanism_rows: winner_mechanism_rows.len(),
            candidate_lifetime_rows: candidate_lifetime_rows.len(),
            rollback_rows: rollback_rows.len(),
            skipped_targets,
            elapsed_ms: started.elapsed().as_secs_f64() * 1000.0,
            claim_boundary: "development replay on selected recorded proposals; not a complete optimizer comparison",
        },
    )?;
    write_json(
        &out_dir.join("completion.json"),
        &serde_json::json!({"schema_version": OUTPUT_SCHEMA_VERSION, "status": "complete"}),
    )?;
    println!(
        "replayed {} selected pairs into {} atom rows",
        selected.len(),
        atom_rows.len()
    );
    Ok(())
}

fn validate_config(config: &Config) -> Result<(), String> {
    if config.schema_version != CONFIG_SCHEMA_VERSION {
        return Err(format!(
            "unsupported config schema version {}",
            config.schema_version
        ));
    }
    if config.pairs_per_run == 0 {
        return Err("pairs_per_run must be positive".to_string());
    }
    if config.action_windows.is_empty()
        || config
            .action_windows
            .iter()
            .any(|value| !value.is_finite() || *value < 0.0)
    {
        return Err("action_windows must be finite and nonnegative".to_string());
    }
    if config.distance_scales.is_empty()
        || config
            .distance_scales
            .iter()
            .any(|value| !value.is_finite() || *value <= 0.0)
    {
        return Err("distance_scales must be positive and finite".to_string());
    }
    if config.max_runs.is_some_and(|value| value == 0) {
        return Err("max_runs must be positive when supplied".to_string());
    }
    Ok(())
}

fn select_pairs(dataset: &Dataset, config: &Config) -> Result<Vec<SelectedPair>, String> {
    let algorithms = config.algorithm_ids.iter().collect::<HashSet<_>>();
    let runs = dataset
        .runs
        .iter()
        .filter(|run| algorithms.is_empty() || algorithms.contains(&run.algorithm_id))
        .take(config.max_runs.unwrap_or(usize::MAX))
        .collect::<Vec<_>>();
    if runs.is_empty() {
        return Err("no runs match the atom replay config".to_string());
    }
    let evaluations = dataset.evaluations_by_id();
    let proposals = dataset
        .proposals
        .iter()
        .map(|row| (row.proposal_id.as_str(), row))
        .collect::<HashMap<_, _>>();
    let rounds_by_run =
        dataset
            .rounds
            .iter()
            .fold(HashMap::<&str, Vec<_>>::new(), |mut grouped, round| {
                grouped.entry(&round.run_id).or_default().push(round);
                grouped
            });
    let mut selected = Vec::new();
    for run in runs {
        let mut rounds = rounds_by_run
            .get(run.run_id.as_str())
            .cloned()
            .unwrap_or_default();
        rounds.sort_by_key(|round| round.round_index);
        for round in evenly_spaced(&rounds, config.pairs_per_run) {
            let Some(proposal_id) = round.proposal_ids.first() else {
                continue;
            };
            let proposal = proposals
                .get(proposal_id.as_str())
                .ok_or_else(|| format!("absent proposal {proposal_id}"))?;
            let anchor_id = proposal
                .baseline_evaluation_id
                .as_ref()
                .unwrap_or(&round.best_evaluation_id_before);
            let anchor = evaluations
                .get(anchor_id.as_str())
                .ok_or_else(|| format!("absent anchor evaluation {anchor_id}"))?;
            let target = evaluations
                .get(proposal.evaluation_id.as_str())
                .ok_or_else(|| format!("absent target evaluation {}", proposal.evaluation_id))?;
            selected.push(SelectedPair {
                pair_id: format!("{}--round-{:05}", run.run_id, round.round_index),
                run_id: run.run_id.clone(),
                start_id: run.start_id.clone(),
                algorithm_id: run.algorithm_id.clone(),
                round_index: round.round_index,
                trajectory_phase: trajectory_phase(
                    round.round_index,
                    rounds.last().map_or(0, |last| last.round_index),
                ),
                accepted_by_optimizer: round
                    .selected
                    .iter()
                    .any(|selected| selected.proposal_id == *proposal_id),
                anchor: (*anchor).clone(),
                recorded_target: (*target).clone(),
            });
        }
    }
    Ok(selected)
}

fn trajectory_phase(round_index: usize, final_round_index: usize) -> String {
    let fraction = if final_round_index == 0 {
        0.0
    } else {
        round_index as f64 / final_round_index as f64
    };
    if fraction < 1.0 / 3.0 {
        "early"
    } else if fraction < 2.0 / 3.0 {
        "middle"
    } else {
        "late"
    }
    .to_string()
}

fn evenly_spaced<'a, T>(values: &'a [&'a T], count: usize) -> Vec<&'a T> {
    if values.len() <= count {
        return values.to_vec();
    }
    let indices = (0..count)
        .map(|index| index * (values.len() - 1) / (count - 1).max(1))
        .collect::<HashSet<_>>();
    values
        .iter()
        .enumerate()
        .filter(|(index, _)| indices.contains(index))
        .map(|(_, value)| *value)
        .collect()
}

fn selectors(
    anchor: &Evaluation,
    target: &Evaluation,
    anchor_universe: &optimizer_runs::algorithms::nonlinear_candidate_cma::CandidateUniverse,
    target_universe: &optimizer_runs::algorithms::nonlinear_candidate_cma::CandidateUniverse,
    config: &Config,
) -> Vec<(String, Vec<Vec<usize>>)> {
    let mut result = vec![(
        "anchor_winner".to_string(),
        anchor.row.winning_sigma.clone().into_iter().collect(),
    )];
    for &window in &config.action_windows {
        result.push((
            format!("anchor_action_window_{window:.6}"),
            anchor_universe.pool(window).sigmas,
        ));
    }
    result.push((
        "anchor_transition_feasible_all".to_string(),
        anchor_universe
            .germs
            .iter()
            .map(|germ| germ.sigma.clone())
            .collect(),
    ));
    result.push((
        "target_winner_oracle".to_string(),
        target.row.winning_sigma.clone().into_iter().collect(),
    ));
    result.push((
        "target_effective_all_oracle".to_string(),
        target_universe
            .germs
            .iter()
            .map(|germ| germ.sigma.clone())
            .collect(),
    ));
    for (_, sigmas) in &mut result {
        sigmas.sort();
        sigmas.dedup();
    }
    result
}

fn winner_mechanism_row(
    pair: &SelectedPair,
    distance_scale: f64,
    normalized_distance: f64,
    symmetry_transverse_normalized_distance: Option<f64>,
    anchor: &Evaluation,
    target: &Evaluation,
    anchor_universe: &CandidateUniverse,
    target_universe: &CandidateUniverse,
) -> Result<WinnerMechanismRow, String> {
    let winner = target
        .row
        .winning_sigma
        .clone()
        .ok_or_else(|| format!("usable target {} has no winning sigma", pair.pair_id))?;
    let anchor_diagnostic = diagnose_named_raw_branch(anchor, &winner)?;
    let target_diagnostic = diagnose_named_raw_branch(target, &winner)?;
    let in_anchor_universe = anchor_universe
        .germs
        .iter()
        .any(|germ| germ.sigma == winner);
    let in_target_universe = target_universe
        .germs
        .iter()
        .any(|germ| germ.sigma == winner);
    let omission_class = if in_anchor_universe {
        "covered_by_anchor_universe"
    } else if !anchor_diagnostic.transition_feasible {
        "anchor_transition_blocked"
    } else if anchor_diagnostic.raw_status != "ok" {
        "anchor_raw_kkt_failed"
    } else {
        "anchor_enumeration_omission"
    };
    Ok(WinnerMechanismRow {
        schema_version: OUTPUT_SCHEMA_VERSION,
        pair_id: pair.pair_id.clone(),
        run_id: pair.run_id.clone(),
        start_id: pair.start_id.clone(),
        algorithm_id: pair.algorithm_id.clone(),
        round_index: pair.round_index,
        trajectory_phase: pair.trajectory_phase.clone(),
        distance_scale,
        normalized_distance,
        symmetry_transverse_normalized_distance,
        target_winning_sigma: winner.clone(),
        same_as_anchor_winner: anchor.row.winning_sigma.as_ref() == Some(&winner),
        in_anchor_universe,
        in_target_universe,
        anchor_transition_feasible: anchor_diagnostic.transition_feasible,
        target_transition_feasible: target_diagnostic.transition_feasible,
        anchor_raw_status: anchor_diagnostic.raw_status,
        target_raw_status: target_diagnostic.raw_status,
        anchor_raw_action: anchor_diagnostic.action,
        target_raw_action: target_diagnostic.action,
        anchor_raw_beta_margin: anchor_diagnostic.beta_margin,
        target_raw_beta_margin: target_diagnostic.beta_margin,
        anchor_raw_normalized_beta_margin: normalized_beta_margin(
            anchor_diagnostic.beta_margin,
            anchor_diagnostic.beta_scale,
        ),
        target_raw_normalized_beta_margin: normalized_beta_margin(
            target_diagnostic.beta_margin,
            target_diagnostic.beta_scale,
        ),
        anchor_action_window_needed: anchor_diagnostic
            .action
            .map(|action| (action / anchor_universe.min_action - 1.0).max(0.0)),
        omission_class: omission_class.to_string(),
    })
}

fn normalized_beta_margin(margin: Option<f64>, scale: Option<f64>) -> Option<f64> {
    margin
        .zip(scale)
        .map(|(margin, scale)| margin / scale.max(f64::EPSILON))
}

fn accepted_trajectory_states(
    dataset: &Dataset,
) -> Result<HashMap<String, Vec<(usize, EvaluationRow)>>, String> {
    let evaluations = dataset.evaluations_by_id();
    let proposals = dataset
        .proposals
        .iter()
        .map(|row| (row.proposal_id.as_str(), row))
        .collect::<HashMap<_, _>>();
    let mut states = HashMap::<String, Vec<(usize, EvaluationRow)>>::new();
    for round in &dataset.rounds {
        let Some(selected) = round.selected.first() else {
            continue;
        };
        let proposal = proposals
            .get(selected.proposal_id.as_str())
            .ok_or_else(|| format!("absent selected proposal {}", selected.proposal_id))?;
        let evaluation = evaluations
            .get(proposal.evaluation_id.as_str())
            .ok_or_else(|| format!("absent selected evaluation {}", proposal.evaluation_id))?;
        if evaluation.usable_by_optimizer && evaluation.sys.is_some() {
            states
                .entry(round.run_id.clone())
                .or_default()
                .push((round.round_index, (*evaluation).clone()));
        }
    }
    for values in states.values_mut() {
        values.sort_by_key(|(round_index, _)| *round_index);
    }
    Ok(states)
}

fn candidate_lifetime_rows_for_anchor(
    pair: &SelectedPair,
    anchor: &Evaluation,
    universe: &CandidateUniverse,
    accepted_states: &[(usize, EvaluationRow)],
    config: &Config,
    evaluator_config: &EvaluatorConfig,
) -> Result<Vec<CandidateLifetimeRow>, String> {
    let selectors = config
        .action_windows
        .iter()
        .map(|window| {
            (
                format!("anchor_action_window_{window:.6}"),
                universe.pool(*window).sigmas,
            )
        })
        .chain(std::iter::once((
            "anchor_transition_feasible_all".to_string(),
            universe
                .germs
                .iter()
                .map(|germ| germ.sigma.clone())
                .collect(),
        )))
        .collect::<Vec<(String, Vec<Vec<usize>>)>>();
    let mut result = Vec::new();
    for (accepted_steps_after_anchor, (target_round_index, target)) in accepted_states
        .iter()
        .filter(|(round_index, _)| *round_index >= pair.round_index)
        .enumerate()
    {
        let target_sys = target
            .sys
            .ok_or_else(|| format!("usable accepted state {} has no sys", target.evaluation_id))?;
        let target_winner = target.winning_sigma.as_ref().ok_or_else(|| {
            format!(
                "usable accepted state {} has no winning sigma",
                target.evaluation_id
            )
        })?;
        let target_duals = unflatten(&target.dual_flat)?;
        let normalized_distance =
            normalized_distance(&anchor.duals, &target_duals).unwrap_or(f64::NAN);
        let symmetry_transverse_normalized_distance =
            symmetry_transverse_normalized_distance(&anchor.duals, &target_duals);
        for (selector, sigmas) in &selectors {
            let outcome = evaluate_surrogate(&target_duals, sigmas, evaluator_config);
            result.push(CandidateLifetimeRow {
                schema_version: OUTPUT_SCHEMA_VERSION,
                anchor_pair_id: pair.pair_id.clone(),
                run_id: pair.run_id.clone(),
                start_id: pair.start_id.clone(),
                algorithm_id: pair.algorithm_id.clone(),
                anchor_round_index: pair.round_index,
                target_round_index: *target_round_index,
                accepted_steps_after_anchor: accepted_steps_after_anchor + 1,
                rounds_after_anchor: target_round_index.saturating_sub(pair.round_index),
                selector: selector.clone(),
                candidate_count: sigmas.len(),
                normalized_distance,
                symmetry_transverse_normalized_distance,
                target_sys,
                predicted_target_sys: outcome.sys,
                prediction_error: outcome.sys.map(|predicted| target_sys - predicted),
                target_winner_covered: sigmas.contains(target_winner),
            });
        }
    }
    Ok(result)
}

fn rollback_rows_for_anchor(
    pair: &SelectedPair,
    anchor: &Evaluation,
    accepted_states: &[(usize, EvaluationRow)],
    evaluator_config: &EvaluatorConfig,
) -> Result<Vec<RollbackRow>, String> {
    let anchor_sys = anchor.row.sys.expect("usable anchor has sys");
    let anchor_winner = anchor
        .row
        .winning_sigma
        .clone()
        .ok_or_else(|| format!("usable anchor {} has no winning sigma", pair.pair_id))?;
    let mut states = vec![(
        pair.round_index,
        anchor.duals.clone(),
        anchor_sys,
        anchor_winner,
    )];
    for (round_index, row) in accepted_states
        .iter()
        .filter(|(round_index, _)| *round_index >= pair.round_index)
    {
        states.push((
            *round_index,
            unflatten(&row.dual_flat)?,
            row.sys
                .ok_or_else(|| format!("usable state {} has no sys", row.evaluation_id))?,
            row.winning_sigma.clone().ok_or_else(|| {
                format!("usable state {} has no winning sigma", row.evaluation_id)
            })?,
        ));
    }
    let mut rows = Vec::new();
    for target_step in 1..states.len() {
        let target_winner = states[target_step].3.clone();
        let mut gaps = Vec::with_capacity(target_step + 1);
        for (_, duals, sys, _) in states.iter().take(target_step + 1) {
            let predicted = evaluate_surrogate(
                duals,
                std::slice::from_ref(&target_winner),
                evaluator_config,
            );
            gaps.push(predicted.sys.map(|branch_sys| branch_sys - sys));
        }
        let earliest = |predicate: &dyn Fn(usize, Option<f64>) -> bool| {
            gaps.iter()
                .enumerate()
                .find_map(|(step, gap)| predicate(step, *gap).then_some(step))
        };
        let earliest_admissible = earliest(&|_, gap| gap.is_some());
        let earliest_within_1e_2 =
            earliest(&|_, gap| gap.is_some_and(|value| value.abs() <= 1.0e-2));
        let earliest_within_1e_3 =
            earliest(&|_, gap| gap.is_some_and(|value| value.abs() <= 1.0e-3));
        let earliest_winner_identity = earliest(&|step, _| states[step].3 == target_winner);
        rows.push(RollbackRow {
            schema_version: OUTPUT_SCHEMA_VERSION,
            anchor_pair_id: pair.pair_id.clone(),
            run_id: pair.run_id.clone(),
            start_id: pair.start_id.clone(),
            algorithm_id: pair.algorithm_id.clone(),
            anchor_round_index: pair.round_index,
            target_round_index: states[target_step].0,
            target_step,
            target_winning_sigma: target_winner,
            earliest_admissible_step: earliest_admissible,
            earliest_within_1e_2_step: earliest_within_1e_2,
            earliest_within_1e_3_step: earliest_within_1e_3,
            earliest_winner_identity_step: earliest_winner_identity,
            admissible_lead_steps: earliest_admissible.map(|step| target_step - step),
            within_1e_2_lead_steps: earliest_within_1e_2.map(|step| target_step - step),
            within_1e_3_lead_steps: earliest_within_1e_3.map(|step| target_step - step),
            winner_identity_lead_steps: earliest_winner_identity.map(|step| target_step - step),
            previous_step_admissible: (target_step > 0).then(|| gaps[target_step - 1].is_some()),
            previous_step_gap: (target_step > 0).then(|| gaps[target_step - 1]).flatten(),
            target_step_gap: gaps[target_step],
        });
    }
    Ok(rows)
}

fn surrogate_status(
    outcome: &optimizer_runs::algorithms::nonlinear_candidate_cma::SurrogateOutcome,
) -> String {
    if outcome.sys.is_some() {
        "admissible".to_string()
    } else if outcome.transition_blocked_branches > 0 {
        "transition_blocked".to_string()
    } else if outcome.indeterminate_branches > 0 {
        "indeterminate_resolution_failed".to_string()
    } else if outcome.branch_solve_failures > 0 {
        "branch_solve_failed".to_string()
    } else {
        "no_admissible_branch".to_string()
    }
}

#[allow(clippy::too_many_arguments)]
fn nonlinear_atom(
    pair: &SelectedPair,
    distance_scale: f64,
    normalized_distance: f64,
    symmetry_transverse_normalized_distance: Option<f64>,
    anchor: &Evaluation,
    target: &Evaluation,
    selector: &str,
    sigmas: &[Vec<usize>],
    config: &EvaluatorConfig,
) -> AtomRow {
    let outcome = evaluate_surrogate(&target.duals, sigmas, config);
    atom_row(
        pair,
        distance_scale,
        normalized_distance,
        symmetry_transverse_normalized_distance,
        anchor,
        target,
        selector,
        "named_branch_kkt_at_target",
        "target_transition_and_beta",
        sigmas,
        None,
        outcome.sys,
        outcome.winning_sigma,
        config,
        outcome.geometry_ms,
        outcome.volume_ms,
        outcome.branch_ms,
        0.0,
    )
}

#[allow(clippy::too_many_arguments)]
fn affine_atom(
    pair: &SelectedPair,
    distance_scale: f64,
    normalized_distance: f64,
    symmetry_transverse_normalized_distance: Option<f64>,
    anchor: &Evaluation,
    target: &Evaluation,
    selector: &str,
    sigmas: &[Vec<usize>],
    config: &EvaluatorConfig,
) -> AtomRow {
    let started = Instant::now();
    let displacement = DVector::from_vec(
        flatten(&target.duals)
            .iter()
            .zip(flatten(&anchor.duals))
            .map(|(target, anchor)| target - anchor)
            .collect(),
    );
    let model = anchor.context.as_ref().and_then(|context| {
        BranchModel::build_from_named_candidates(&context.polytope, context.volume, sigmas).ok()
    });
    let represented = model.as_ref().map(|model| model.candidates.len());
    let prediction = model.and_then(|model| {
        let base_sys = model.base_sys;
        model
            .predict_displacement(&anchor.duals, &displacement, 1.0)
            .ok()
            .map(|(delta, sigma, _)| (base_sys + delta, sigma))
    });
    atom_row(
        pair,
        distance_scale,
        normalized_distance,
        symmetry_transverse_normalized_distance,
        anchor,
        target,
        selector,
        "affine_named_branches_at_anchor",
        "anchor_admissibility_constant",
        sigmas,
        represented,
        prediction.as_ref().map(|(value, _)| *value),
        prediction.map(|(_, sigma)| sigma),
        config,
        0.0,
        0.0,
        0.0,
        started.elapsed().as_secs_f64() * 1000.0,
    )
}

#[allow(clippy::too_many_arguments)]
fn atom_row(
    pair: &SelectedPair,
    distance_scale: f64,
    normalized_distance: f64,
    symmetry_transverse_normalized_distance: Option<f64>,
    anchor: &Evaluation,
    target: &Evaluation,
    selector: &str,
    value_model: &str,
    domain_model: &str,
    sigmas: &[Vec<usize>],
    represented_branch_count: Option<usize>,
    predicted_target_sys: Option<f64>,
    predicted_winning_sigma: Option<Vec<usize>>,
    config: &EvaluatorConfig,
    geometry_ms: f64,
    volume_ms: f64,
    named_branch_ms: f64,
    model_ms: f64,
) -> AtomRow {
    let anchor_sys = anchor.row.sys.expect("usable anchor has sys");
    let target_sys = target.row.sys;
    let actual_delta = target_sys.map(|value| value - anchor_sys);
    let predicted_delta = predicted_target_sys.map(|value| value - anchor_sys);
    let predicted_branch_target = predicted_winning_sigma
        .as_ref()
        .map(|sigma| evaluate_surrogate(&target.duals, std::slice::from_ref(sigma), config));
    let predicted_winner_target_sys = predicted_branch_target
        .as_ref()
        .and_then(|outcome| outcome.sys);
    let predicted_winner_target_status = predicted_branch_target.as_ref().map(surrogate_status);
    let predicted_winner_target_raw = predicted_winning_sigma
        .as_ref()
        .and_then(|sigma| diagnose_named_raw_branch(target, sigma).ok());
    AtomRow {
        schema_version: OUTPUT_SCHEMA_VERSION,
        pair_id: pair.pair_id.clone(),
        run_id: pair.run_id.clone(),
        start_id: pair.start_id.clone(),
        algorithm_id: pair.algorithm_id.clone(),
        round_index: pair.round_index,
        trajectory_phase: pair.trajectory_phase.clone(),
        accepted_by_optimizer: pair.accepted_by_optimizer,
        distance_scale,
        normalized_distance,
        symmetry_transverse_normalized_distance,
        selector: selector.to_string(),
        value_model: value_model.to_string(),
        domain_model: domain_model.to_string(),
        candidate_count: sigmas.len(),
        represented_branch_count,
        target_winner_covered: target
            .row
            .winning_sigma
            .as_ref()
            .map(|winner| sigmas.contains(winner)),
        anchor_sys,
        target_sys,
        predicted_target_sys,
        predicted_winner_matches_target: predicted_winning_sigma
            .as_ref()
            .zip(target.row.winning_sigma.as_ref())
            .map(|(predicted, target)| predicted == target),
        predicted_winning_sigma,
        predicted_winner_target_sys,
        predicted_winner_target_status,
        predicted_winner_target_transition_feasible: predicted_winner_target_raw
            .as_ref()
            .map(|diagnostic| diagnostic.transition_feasible),
        predicted_winner_target_raw_status: predicted_winner_target_raw
            .as_ref()
            .map(|diagnostic| diagnostic.raw_status.clone()),
        predicted_winner_target_raw_beta_margin: predicted_winner_target_raw
            .as_ref()
            .and_then(|diagnostic| diagnostic.beta_margin),
        predicted_winner_target_raw_normalized_beta_margin: predicted_winner_target_raw
            .as_ref()
            .and_then(|diagnostic| {
                normalized_beta_margin(diagnostic.beta_margin, diagnostic.beta_scale)
            }),
        selected_branch_prediction_error: predicted_winner_target_sys
            .zip(predicted_target_sys)
            .map(|(actual_branch, predicted_branch)| actual_branch - predicted_branch),
        predicted_delta,
        actual_delta,
        prediction_error: predicted_target_sys
            .zip(target_sys)
            .map(|(predicted, actual)| actual - predicted),
        sign_correct: predicted_delta
            .zip(actual_delta)
            .map(|(predicted, actual)| (predicted > 0.0) == (actual > 0.0)),
        geometry_ms,
        volume_ms,
        named_branch_ms,
        model_ms,
    }
}

fn evaluate(
    evaluator: &mut Evaluator,
    run_id: &str,
    serial: &mut usize,
    role: &str,
    duals: Vec<Vector4<f64>>,
) -> Evaluation {
    let evaluation = evaluator.evaluate(
        run_id,
        format!("{run_id}--atom-e{:06}", *serial),
        None,
        role,
        *serial,
        false,
        duals,
    );
    *serial += 1;
    evaluation
}

fn interpolate(anchor: &[Vector4<f64>], target: &[Vector4<f64>], scale: f64) -> Vec<Vector4<f64>> {
    anchor
        .iter()
        .zip(target)
        .map(|(anchor, target)| anchor + scale * (target - anchor))
        .collect()
}

fn normalized_distance(left: &[Vector4<f64>], right: &[Vector4<f64>]) -> Option<f64> {
    let norm = left.iter().map(Vector4::norm_squared).sum::<f64>().sqrt();
    (norm > 0.0).then(|| {
        left.iter()
            .zip(right)
            .map(|(left, right)| (left - right).norm_squared())
            .sum::<f64>()
            .sqrt()
            / norm
    })
}

fn symmetry_transverse_normalized_distance(
    left: &[Vector4<f64>],
    right: &[Vector4<f64>],
) -> Option<f64> {
    let norm = flatten(left)
        .iter()
        .map(|value| value * value)
        .sum::<f64>()
        .sqrt();
    if norm <= 0.0 {
        return None;
    }
    let basis = quotient_basis(left).ok()?;
    let mut displacement = DVector::from_vec(
        flatten(right)
            .iter()
            .zip(flatten(left))
            .map(|(right, left)| right - left)
            .collect(),
    );
    for axis in &basis.orbit_basis {
        displacement -= axis * axis.dot(&displacement);
    }
    Some(displacement.norm() / norm)
}

fn resolve_path(config_path: &Path, configured: &Path) -> PathBuf {
    if configured.is_absolute() || configured.exists() {
        configured.to_path_buf()
    } else {
        config_path
            .parent()
            .unwrap_or_else(|| Path::new("."))
            .join(configured)
    }
}

fn parse_args() -> Result<(PathBuf, PathBuf), String> {
    let mut config = None;
    let mut out = None;
    let mut args = std::env::args().skip(1);
    while let Some(argument) = args.next() {
        match argument.as_str() {
            "--config" => config = Some(PathBuf::from(args.next().ok_or("--config needs a path")?)),
            "--out" => out = Some(PathBuf::from(args.next().ok_or("--out needs a path")?)),
            _ => return Err(format!("unknown argument {argument}")),
        }
    }
    Ok((
        config.ok_or("missing --config")?,
        out.ok_or("missing --out")?,
    ))
}

fn file_blake3(path: &Path) -> Result<String, String> {
    let bytes = std::fs::read(path).map_err(|error| format!("read {}: {error}", path.display()))?;
    Ok(blake3::hash(&bytes).to_hex().to_string())
}

#[cfg(test)]
mod tests {
    use super::{evenly_spaced, interpolate};
    use nalgebra::Vector4;

    #[test]
    fn pair_selection_includes_first_and_last() {
        let values = [0, 1, 2, 3, 4];
        let references = values.iter().collect::<Vec<_>>();
        assert_eq!(evenly_spaced(&references, 3), vec![&0, &2, &4]);
    }

    #[test]
    fn interpolation_scales_recorded_displacement() {
        let anchor = [Vector4::zeros()];
        let target = [Vector4::repeat(2.0)];
        assert_eq!(
            interpolate(&anchor, &target, 0.5),
            vec![Vector4::repeat(1.0)]
        );
    }
}

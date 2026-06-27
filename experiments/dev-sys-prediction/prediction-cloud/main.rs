//! Prediction-cloud producer tied to the branch degeneracy diagnostic.
//!
//! This command consumes a `dev-gradient-ascent-branch-diagnostic` output
//! directory, selects representative classified basepoints, and compares
//! single-anchor branch predictions with recomputed `sys(a0 + t d)` over a
//! finite direction/radius cloud.

use exp_sys_landscape::{
    compute_active_sys_state, exact_volume_from_incidence_as_f64, SysLandscapePolytopeCache,
};
use good_lp::{constraint, default_solver, variable, variables, Expression, Solution, SolverModel};
use nalgebra::Vector4;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fs::{self, File};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};
use std::time::{Instant, SystemTime, UNIX_EPOCH};
use symplectic::derivatives::{
    capacity_subgradients_a, clarke_directional_derivative_a, systolic_ratio_gradient_a,
    volume_derivatives_a,
};
use symplectic::{
    aggregate_orbits_with_dual_vertices_exact, classify_facets_from_dual_vertices,
    solve_billiard_candidates, solve_pruned_hk2017_candidates, OrbitAdmissibility,
    OrbitGuaranteeMode, OrbitKktData, OrbitSearchError, OrbitSearchResult,
};

const DEFAULT_SELECTION_THRESHOLD_RELATIVE: f64 = 1.0e-3;
const DEFAULT_ACTION_WINDOW_RELATIVE: f64 = 1.0e-2;
const DEFAULT_STEPS: &[f64] = &[1.0e-4, 1.0e-3];
const DEFAULT_MAX_FIXTURES_PER_LABEL: usize = 1;
const DEFAULT_TRACE_ITERATIONS: usize = 1;
const DEFAULT_MIN_OBSERVED_DELTA: f64 = 0.0;
const DEFAULT_MIN_OBSERVED_RELATIVE_DELTA: f64 = 0.0;

#[derive(Debug)]
struct Cli {
    diagnostic_dir: PathBuf,
    polytope_table: PathBuf,
    out_dir: PathBuf,
    selection_threshold_relative: f64,
    action_window_relative: f64,
    direction_model: DirectionModel,
    include_candidate_window_directions: bool,
    write_step_ranking_audit: bool,
    steps: Vec<f64>,
    endpoint_steps: Option<Vec<f64>>,
    max_fixtures_per_label: usize,
    skip_fixtures_per_label: usize,
    trace_iterations: usize,
    degeneracy_labels: Vec<String>,
    min_observed_delta: f64,
    min_observed_relative_delta: f64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DirectionModel {
    NearActive,
    CandidateWindow,
}

impl DirectionModel {
    fn as_str(self) -> &'static str {
        match self {
            DirectionModel::NearActive => "near_active",
            DirectionModel::CandidateWindow => "candidate_window",
        }
    }

    fn method_variant(self) -> &'static str {
        match self {
            DirectionModel::NearActive => "iterative_observed_multi_direction_probe",
            DirectionModel::CandidateWindow => {
                "iterative_candidate_window_scored_multi_direction_probe"
            }
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
struct DiagnosticRow {
    poly_id: String,
    selection_buckets: Vec<String>,
    datasets: Vec<String>,
    input_facet_count: usize,
    input_sys: f64,
    threshold_relative: f64,
    near_active_count: Option<usize>,
    degeneracy_label: String,
    failure: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
struct PolytopeRow {
    poly_id: String,
    capacity: f64,
    sys: f64,
    dual_vertices_f64: Vec<[f64; 4]>,
}

#[derive(Clone, Debug)]
struct Fixture {
    diagnostic: DiagnosticRow,
    polytope: PolytopeRow,
    selection_rank_within_label: usize,
}

#[derive(Serialize)]
struct FixtureRow {
    poly_id: String,
    degeneracy_label: String,
    selection_rank_within_label: usize,
    threshold_relative: f64,
    selection_buckets: Vec<String>,
    datasets: Vec<String>,
    input_facet_count: usize,
    input_sys: f64,
    near_active_count: usize,
}

#[derive(Serialize)]
struct LocalGeometryProbeRow {
    poly_id: String,
    degeneracy_label: String,
    direction_label: String,
    step: f64,
    status: String,
    base_sys: f64,
    predicted_delta_per_step: Option<f64>,
    predicted_delta_sys: Option<f64>,
    recomputed_sys: Option<f64>,
    observed_delta_sys: Option<f64>,
    target_near_active_count: Option<usize>,
    target_best_sigma_in_base_near_active_set: Option<bool>,
    target_best_sigma_in_base_candidate_window: Option<bool>,
    base_near_active_count: usize,
    base_returned_orbit_count: usize,
    base_orbit_iterations: u64,
    target_orbit_iterations: Option<u64>,
}

#[derive(Serialize)]
struct RunTraceRow {
    poly_id: String,
    degeneracy_label: String,
    iteration: usize,
    method_variant: String,
    branch_threshold_relative: f64,
    action_window_relative: f64,
    direction_model: String,
    base_near_active_count: usize,
    target_near_active_count: Option<usize>,
    chosen_direction_label: Option<String>,
    chosen_step: Option<f64>,
    attempted_direction_labels: Vec<String>,
    rejected_direction_labels: Vec<String>,
    line_search_attempts: usize,
    rejected_steps: Vec<f64>,
    line_search_status: String,
    min_observed_delta: f64,
    min_observed_relative_delta: f64,
    effective_min_observed_delta: f64,
    predicted_delta_sys: Option<f64>,
    observed_delta_sys: Option<f64>,
    base_sys: f64,
    target_sys: Option<f64>,
    base_orbit_iterations: u64,
    target_orbit_iterations: Option<u64>,
    accepted: bool,
    stop_reason: String,
}

#[derive(Serialize)]
struct StepRankingAuditRow {
    poly_id: String,
    degeneracy_label: String,
    iteration: usize,
    direction_label: String,
    step: f64,
    status: String,
    base_sys: f64,
    effective_min_observed_delta: f64,
    near_active_predicted_delta_sys: Option<f64>,
    candidate_window_predicted_delta_sys: Option<f64>,
    candidate_window_witness_orbit_index: Option<usize>,
    candidate_window_witness_sigma: Option<Vec<usize>>,
    candidate_window_witness_action: Option<f64>,
    candidate_window_witness_relative_action_gap: Option<f64>,
    candidate_window_witness_base_gap: Option<f64>,
    candidate_window_witness_derivative: Option<f64>,
    decomposition_predicted_sys: Option<f64>,
    decomposition_actual_sys: Option<f64>,
    decomposition_total_prediction_error: Option<f64>,
    decomposition_base_window_exact_sys: Option<f64>,
    decomposition_linearization_error: Option<f64>,
    decomposition_sigma_set_error: Option<f64>,
    decomposition_sum_error: Option<f64>,
    decomposition_sum_residual: Option<f64>,
    fixed_winner_actual_action: Option<f64>,
    fixed_winner_predicted_action: Option<f64>,
    fixed_winner_action_error: Option<f64>,
    fixed_winner_actual_volume: Option<f64>,
    fixed_winner_predicted_volume: Option<f64>,
    fixed_winner_volume_error: Option<f64>,
    fixed_winner_actual_sys: Option<f64>,
    fixed_winner_predicted_sys_full: Option<f64>,
    fixed_winner_predicted_sys_actual_action_linear_volume: Option<f64>,
    fixed_winner_predicted_sys_linear_action_actual_volume: Option<f64>,
    fixed_winner_sys_error_full: Option<f64>,
    fixed_winner_sys_error_action_part: Option<f64>,
    fixed_winner_sys_error_volume_part: Option<f64>,
    fixed_winner_sys_error_interaction_residual: Option<f64>,
    observed_delta_sys: Option<f64>,
    target_sys: Option<f64>,
    above_threshold_observed: Option<bool>,
    positive_observed: Option<bool>,
    near_active_prediction_positive: Option<bool>,
    candidate_window_prediction_positive: Option<bool>,
    observed_rank_desc: Option<usize>,
    near_active_rank_desc: Option<usize>,
    candidate_window_rank_desc: Option<usize>,
    base_near_active_count: usize,
    base_candidate_window_count: usize,
    base_orbit_iterations: u64,
    target_orbit_iterations: Option<u64>,
}

#[derive(Serialize)]
struct EndpointDiagnosticRow {
    poly_id: String,
    degeneracy_label: String,
    trace_stop_reason: String,
    final_sys: Option<f64>,
    final_near_active_count: Option<usize>,
    post_stop_direction_label: Option<String>,
    post_stop_step: Option<f64>,
    post_stop_line_search_attempts: Option<usize>,
    post_stop_rejected_steps: Vec<f64>,
    post_stop_line_search_status: Option<String>,
    min_observed_delta: f64,
    min_observed_relative_delta: f64,
    effective_min_observed_delta: Option<f64>,
    post_stop_prediction_selected_step_found: Option<bool>,
    post_stop_predicted_delta_sys: Option<f64>,
    post_stop_observed_delta_sys: Option<f64>,
    post_stop_improvement_found: Option<bool>,
    post_stop_threshold_improvement_found: Option<bool>,
    diagnostic_status: String,
    base_orbit_iterations: Option<u64>,
    target_orbit_iterations: Option<u64>,
    caveat: String,
}

#[derive(Serialize)]
struct ComputeBudgetReport {
    command: String,
    diagnostic_dir: String,
    polytope_table: String,
    selection_threshold_relative: f64,
    direction_model: String,
    include_candidate_window_directions: bool,
    max_fixtures_per_label: usize,
    skip_fixtures_per_label: usize,
    degeneracy_labels: Vec<String>,
    selected_fixtures: usize,
    eligible_diagnostic_counts_by_label: BTreeMap<String, usize>,
    selected_counts_by_label: BTreeMap<String, usize>,
    missing_polytope_counts_by_label: BTreeMap<String, usize>,
    requested_labels_with_no_selected_fixture: Vec<String>,
    probe_rows: usize,
    run_trace_rows: usize,
    endpoint_diagnostic_rows: usize,
    endpoint_direction_scan_rows: usize,
    base_orbit_iterations: u64,
    target_orbit_iterations: u64,
    trace_base_orbit_iterations: u64,
    trace_target_orbit_iterations: u64,
    endpoint_base_orbit_iterations: u64,
    endpoint_target_orbit_iterations: u64,
    endpoint_scan_base_orbit_iterations: u64,
    endpoint_scan_target_orbit_iterations: u64,
    failed_probe_rows: usize,
    failed_endpoint_direction_scan_rows: usize,
    elapsed_ms: f64,
}

#[derive(Serialize)]
struct Summary {
    method: String,
    direction_model: String,
    include_candidate_window_directions: bool,
    selection_threshold_relative: f64,
    max_fixtures_per_label: usize,
    skip_fixtures_per_label: usize,
    degeneracy_labels: Vec<String>,
    selected_fixtures: usize,
    eligible_diagnostic_counts_by_label: BTreeMap<String, usize>,
    selected_counts_by_label: BTreeMap<String, usize>,
    missing_polytope_counts_by_label: BTreeMap<String, usize>,
    requested_labels_with_no_selected_fixture: Vec<String>,
    probe_rows: usize,
    run_trace_rows: usize,
    endpoint_diagnostic_rows: usize,
    endpoint_direction_scan_rows: usize,
    failed_probe_rows: usize,
    failed_endpoint_direction_scan_rows: usize,
    degeneracy_counts: BTreeMap<String, usize>,
    status_counts: BTreeMap<String, usize>,
    trace_stop_reason_counts: BTreeMap<String, usize>,
    trace_line_search_status_counts: BTreeMap<String, usize>,
    endpoint_status_counts: BTreeMap<String, usize>,
    endpoint_line_search_status_counts: BTreeMap<String, usize>,
    endpoint_direction_scan_status_counts: BTreeMap<String, usize>,
    endpoint_direction_scan_threshold_counts: BTreeMap<String, usize>,
    write_step_ranking_audit: bool,
    out_dir: String,
    caveat: String,
}

#[derive(Clone, Debug)]
struct BaseState {
    polytope: SysLandscapePolytopeCache,
    capacity: OrbitSearchResult,
    volume: f64,
    volume_gradient: Vec<Vector4<f64>>,
    sys: f64,
    near_active_orbits: Vec<OrbitKktData>,
    sys_gradients: Vec<Vec<Vector4<f64>>>,
    candidate_orbits: Vec<OrbitKktData>,
    candidate_sys_gradients: Vec<Vec<Vector4<f64>>>,
}

#[derive(Clone, Copy, Debug)]
struct StopThreshold {
    absolute_delta: f64,
    relative_delta: f64,
}

#[derive(Clone, Debug)]
struct SelectionDiagnostics {
    eligible_counts_by_label: BTreeMap<String, usize>,
    selected_counts_by_label: BTreeMap<String, usize>,
    missing_polytope_counts_by_label: BTreeMap<String, usize>,
    requested_labels_with_no_selected_fixture: Vec<String>,
}

#[derive(Clone, Debug)]
struct ProbeDirection {
    label: String,
    vector: Vec<Vector4<f64>>,
    only_step: Option<f64>,
}

impl ProbeDirection {
    fn allows_step(&self, step: f64) -> bool {
        self.only_step
            .is_none_or(|allowed| steps_match(allowed, step))
    }
}

impl StopThreshold {
    fn effective_delta(self, base_sys: f64) -> f64 {
        self.absolute_delta
            .max(self.relative_delta * base_sys.abs())
    }
}

fn main() {
    let cli = parse_args();
    fs::create_dir_all(&cli.out_dir).expect("failed to create output directory");
    let t0 = Instant::now();

    let diagnostic_rows: Vec<DiagnosticRow> =
        load_jsonl(&cli.diagnostic_dir.join("branch-set-diagnostic.jsonl"));
    let polytope_rows: Vec<PolytopeRow> = load_jsonl(&cli.polytope_table);
    let polytope_by_id: HashMap<String, PolytopeRow> = polytope_rows
        .into_iter()
        .map(|row| (row.poly_id.clone(), row))
        .collect();
    let fixtures = select_fixtures(
        &diagnostic_rows,
        &polytope_by_id,
        cli.selection_threshold_relative,
        cli.max_fixtures_per_label,
        cli.skip_fixtures_per_label,
        &cli.degeneracy_labels,
    );
    let selection_diagnostics = selection_diagnostics(
        &diagnostic_rows,
        &polytope_by_id,
        cli.selection_threshold_relative,
        &cli.degeneracy_labels,
        &fixtures,
    );

    let fixture_rows: Vec<FixtureRow> = fixtures.iter().map(fixture_row).collect();
    let mut probe_rows = Vec::new();
    let mut base_orbit_iterations = 0u64;
    let mut target_orbit_iterations = 0u64;

    for fixture in &fixtures {
        match compute_base_state_from_row(
            &fixture.polytope,
            cli.action_window_relative,
            cli.selection_threshold_relative,
        ) {
            Ok(base) => {
                base_orbit_iterations += base.capacity.iterations;
                let directions =
                    probe_directions(&base, &cli.steps, cli.include_candidate_window_directions);
                for direction in directions {
                    for &step in &cli.steps {
                        if !direction.allows_step(step) {
                            continue;
                        }
                        let row = local_probe_row(
                            fixture,
                            &base,
                            &direction.label,
                            &direction.vector,
                            step,
                            cli.direction_model,
                            cli.action_window_relative,
                            cli.selection_threshold_relative,
                        );
                        if let Some(iterations) = row.target_orbit_iterations {
                            target_orbit_iterations += iterations;
                        }
                        probe_rows.push(row);
                    }
                }
            }
            Err(err) => {
                probe_rows.push(LocalGeometryProbeRow {
                    poly_id: fixture.polytope.poly_id.clone(),
                    degeneracy_label: fixture.diagnostic.degeneracy_label.clone(),
                    direction_label: "base_state".to_string(),
                    step: 0.0,
                    status: err,
                    base_sys: fixture.polytope.sys,
                    predicted_delta_per_step: None,
                    predicted_delta_sys: None,
                    recomputed_sys: None,
                    observed_delta_sys: None,
                    target_near_active_count: None,
                    target_best_sigma_in_base_near_active_set: None,
                    target_best_sigma_in_base_candidate_window: None,
                    base_near_active_count: 0,
                    base_returned_orbit_count: 0,
                    base_orbit_iterations: 0,
                    target_orbit_iterations: None,
                });
            }
        }
    }

    write_jsonl(cli.out_dir.join("fixture-selection.jsonl"), &fixture_rows)
        .expect("failed to write fixture-selection.jsonl");
    write_jsonl(cli.out_dir.join("local-geometry-probe.jsonl"), &probe_rows)
        .expect("failed to write local-geometry-probe.jsonl");
    let trace_artifacts = run_trace_and_endpoint_rows(
        &fixtures,
        &probe_rows,
        cli.selection_threshold_relative,
        cli.action_window_relative,
        cli.direction_model,
        cli.include_candidate_window_directions,
        &cli.steps,
        cli.endpoint_steps.as_deref().unwrap_or(&cli.steps),
        cli.trace_iterations,
        StopThreshold {
            absolute_delta: cli.min_observed_delta,
            relative_delta: cli.min_observed_relative_delta,
        },
    );
    let trace_rows = trace_artifacts.trace_rows;
    let endpoint_rows = trace_artifacts.endpoint_rows;
    let endpoint_direction_scan_rows = trace_artifacts.endpoint_direction_scan_rows;
    let step_ranking_audit_rows = trace_artifacts.step_ranking_audit_rows;
    write_jsonl(cli.out_dir.join("run-trace.jsonl"), &trace_rows)
        .expect("failed to write run-trace.jsonl");
    write_jsonl(
        cli.out_dir.join("endpoint-diagnostic.jsonl"),
        &endpoint_rows,
    )
    .expect("failed to write endpoint-diagnostic.jsonl");
    write_jsonl(
        cli.out_dir.join("endpoint-direction-scan.jsonl"),
        &endpoint_direction_scan_rows,
    )
    .expect("failed to write endpoint-direction-scan.jsonl");
    write_jsonl(
        cli.out_dir.join("prediction-cloud.jsonl"),
        &step_ranking_audit_rows,
    )
    .expect("failed to write prediction-cloud.jsonl");
    if cli.write_step_ranking_audit {
        write_jsonl(
            cli.out_dir.join("step-ranking-audit.jsonl"),
            &step_ranking_audit_rows,
        )
        .expect("failed to write step-ranking-audit.jsonl");
    }

    let failed_probe_rows = probe_rows
        .iter()
        .filter(|row| row.status.as_str() != "ok")
        .count();
    let failed_endpoint_direction_scan_rows = endpoint_direction_scan_rows
        .iter()
        .filter(|row| row.status.as_str() != "ok")
        .count();
    let trace_base_orbit_iterations = trace_rows
        .iter()
        .map(|row| row.base_orbit_iterations)
        .sum::<u64>();
    let trace_target_orbit_iterations = trace_rows
        .iter()
        .filter_map(|row| row.target_orbit_iterations)
        .sum::<u64>();
    let endpoint_base_orbit_iterations = endpoint_rows
        .iter()
        .filter_map(|row| row.base_orbit_iterations)
        .sum::<u64>();
    let endpoint_target_orbit_iterations = endpoint_rows
        .iter()
        .filter_map(|row| row.target_orbit_iterations)
        .sum::<u64>();
    let endpoint_scan_base_orbit_iterations = endpoint_direction_scan_rows
        .iter()
        .map(|row| row.base_orbit_iterations)
        .sum::<u64>();
    let endpoint_scan_target_orbit_iterations = endpoint_direction_scan_rows
        .iter()
        .filter_map(|row| row.target_orbit_iterations)
        .sum::<u64>();
    let report = ComputeBudgetReport {
        command: "dev-sys-prediction-cloud".to_string(),
        diagnostic_dir: cli.diagnostic_dir.display().to_string(),
        polytope_table: cli.polytope_table.display().to_string(),
        selection_threshold_relative: cli.selection_threshold_relative,
        direction_model: cli.direction_model.as_str().to_string(),
        include_candidate_window_directions: cli.include_candidate_window_directions,
        max_fixtures_per_label: cli.max_fixtures_per_label,
        skip_fixtures_per_label: cli.skip_fixtures_per_label,
        degeneracy_labels: cli.degeneracy_labels.clone(),
        selected_fixtures: fixtures.len(),
        eligible_diagnostic_counts_by_label: selection_diagnostics.eligible_counts_by_label.clone(),
        selected_counts_by_label: selection_diagnostics.selected_counts_by_label.clone(),
        missing_polytope_counts_by_label: selection_diagnostics
            .missing_polytope_counts_by_label
            .clone(),
        requested_labels_with_no_selected_fixture: selection_diagnostics
            .requested_labels_with_no_selected_fixture
            .clone(),
        probe_rows: probe_rows.len(),
        run_trace_rows: trace_rows.len(),
        endpoint_diagnostic_rows: endpoint_rows.len(),
        endpoint_direction_scan_rows: endpoint_direction_scan_rows.len(),
        base_orbit_iterations,
        target_orbit_iterations,
        trace_base_orbit_iterations,
        trace_target_orbit_iterations,
        endpoint_base_orbit_iterations,
        endpoint_target_orbit_iterations,
        endpoint_scan_base_orbit_iterations,
        endpoint_scan_target_orbit_iterations,
        failed_probe_rows,
        failed_endpoint_direction_scan_rows,
        elapsed_ms: t0.elapsed().as_secs_f64() * 1000.0,
    };
    write_json(cli.out_dir.join("compute-budget-report.json"), &report)
        .expect("failed to write compute-budget-report.json");

    let summary = Summary {
        method: "dev-sys-prediction-cloud".to_string(),
        direction_model: cli.direction_model.as_str().to_string(),
        include_candidate_window_directions: cli.include_candidate_window_directions,
        selection_threshold_relative: cli.selection_threshold_relative,
        max_fixtures_per_label: cli.max_fixtures_per_label,
        skip_fixtures_per_label: cli.skip_fixtures_per_label,
        degeneracy_labels: cli.degeneracy_labels.clone(),
        selected_fixtures: fixtures.len(),
        eligible_diagnostic_counts_by_label: selection_diagnostics.eligible_counts_by_label,
        selected_counts_by_label: selection_diagnostics.selected_counts_by_label,
        missing_polytope_counts_by_label: selection_diagnostics.missing_polytope_counts_by_label,
        requested_labels_with_no_selected_fixture: selection_diagnostics
            .requested_labels_with_no_selected_fixture,
        probe_rows: probe_rows.len(),
        run_trace_rows: trace_rows.len(),
        endpoint_diagnostic_rows: endpoint_rows.len(),
        endpoint_direction_scan_rows: endpoint_direction_scan_rows.len(),
        failed_probe_rows,
        failed_endpoint_direction_scan_rows,
        degeneracy_counts: count_fixture_degeneracy(&fixtures),
        status_counts: count_probe_statuses(&probe_rows),
        trace_stop_reason_counts: count_trace_stop_reasons(&trace_rows),
        trace_line_search_status_counts: count_trace_line_search_statuses(&trace_rows),
        endpoint_status_counts: count_endpoint_statuses(&endpoint_rows),
        endpoint_line_search_status_counts: count_endpoint_line_search_statuses(&endpoint_rows),
        endpoint_direction_scan_status_counts: count_probe_statuses(&endpoint_direction_scan_rows),
        endpoint_direction_scan_threshold_counts: count_probe_threshold_outcomes(
            &endpoint_direction_scan_rows,
            StopThreshold {
                absolute_delta: cli.min_observed_delta,
                relative_delta: cli.min_observed_relative_delta,
            },
        ),
        write_step_ranking_audit: cli.write_step_ranking_audit,
        out_dir: cli.out_dir.display().to_string(),
        caveat: "finite single-anchor prediction cloud only; this does not certify endpoint local maximality"
            .to_string(),
    };
    write_json(cli.out_dir.join("summary.json"), &summary).expect("failed to write summary.json");

    println!("{}", cli.out_dir.display());
}

fn select_fixtures(
    diagnostic_rows: &[DiagnosticRow],
    polytopes: &HashMap<String, PolytopeRow>,
    threshold: f64,
    max_per_label: usize,
    skip_per_label: usize,
    degeneracy_labels: &[String],
) -> Vec<Fixture> {
    let mut selected = Vec::new();
    let mut eligible_seen_by_label: BTreeMap<String, usize> = BTreeMap::new();
    let mut selected_by_label: BTreeMap<String, usize> = BTreeMap::new();
    let wanted: BTreeSet<&str> = degeneracy_labels.iter().map(String::as_str).collect();

    let mut rows: Vec<&DiagnosticRow> = diagnostic_rows
        .iter()
        .filter(|row| row.failure.is_none())
        .filter(|row| wanted.contains(row.degeneracy_label.as_str()))
        .filter(|row| (row.threshold_relative - threshold).abs() <= 1.0e-15)
        .collect();
    rows.sort_by(|a, b| {
        a.degeneracy_label
            .cmp(&b.degeneracy_label)
            .then_with(|| b.input_sys.total_cmp(&a.input_sys))
            .then_with(|| a.poly_id.cmp(&b.poly_id))
    });

    for row in rows {
        let eligible_seen = eligible_seen_by_label
            .entry(row.degeneracy_label.clone())
            .or_insert(0);
        let selection_rank_within_label = *eligible_seen;
        *eligible_seen += 1;

        if selection_rank_within_label < skip_per_label {
            continue;
        }

        let selected_count = selected_by_label
            .entry(row.degeneracy_label.clone())
            .or_insert(0);
        if *selected_count >= max_per_label {
            continue;
        }
        let Some(polytope) = polytopes.get(&row.poly_id) else {
            continue;
        };
        selected.push(Fixture {
            diagnostic: row.clone(),
            polytope: polytope.clone(),
            selection_rank_within_label,
        });
        *selected_count += 1;
    }

    selected
}

fn selection_diagnostics(
    diagnostic_rows: &[DiagnosticRow],
    polytopes: &HashMap<String, PolytopeRow>,
    threshold: f64,
    degeneracy_labels: &[String],
    fixtures: &[Fixture],
) -> SelectionDiagnostics {
    let wanted: BTreeSet<&str> = degeneracy_labels.iter().map(String::as_str).collect();
    let mut eligible_counts_by_label = BTreeMap::new();
    let mut missing_polytope_counts_by_label = BTreeMap::new();
    for row in diagnostic_rows
        .iter()
        .filter(|row| row.failure.is_none())
        .filter(|row| wanted.contains(row.degeneracy_label.as_str()))
        .filter(|row| (row.threshold_relative - threshold).abs() <= 1.0e-15)
    {
        *eligible_counts_by_label
            .entry(row.degeneracy_label.clone())
            .or_insert(0) += 1;
        if !polytopes.contains_key(&row.poly_id) {
            *missing_polytope_counts_by_label
                .entry(row.degeneracy_label.clone())
                .or_insert(0) += 1;
        }
    }

    let selected_counts_by_label = count_fixture_degeneracy(fixtures);
    let requested_labels_with_no_selected_fixture = degeneracy_labels
        .iter()
        .filter(|label| selected_counts_by_label.get(*label).copied().unwrap_or(0) == 0)
        .cloned()
        .collect();

    SelectionDiagnostics {
        eligible_counts_by_label,
        selected_counts_by_label,
        missing_polytope_counts_by_label,
        requested_labels_with_no_selected_fixture,
    }
}

struct TraceArtifacts {
    trace_rows: Vec<RunTraceRow>,
    endpoint_rows: Vec<EndpointDiagnosticRow>,
    endpoint_direction_scan_rows: Vec<LocalGeometryProbeRow>,
    step_ranking_audit_rows: Vec<StepRankingAuditRow>,
}

fn run_trace_and_endpoint_rows(
    fixtures: &[Fixture],
    probe_rows: &[LocalGeometryProbeRow],
    branch_threshold_relative: f64,
    action_window_relative: f64,
    direction_model: DirectionModel,
    include_candidate_window_directions: bool,
    steps: &[f64],
    endpoint_steps: &[f64],
    trace_iterations: usize,
    stop_threshold: StopThreshold,
) -> TraceArtifacts {
    let mut rows = Vec::new();
    let mut endpoint_rows = Vec::new();
    let mut endpoint_direction_scan_rows = Vec::new();
    let mut step_ranking_audit_rows = Vec::new();
    for fixture in fixtures {
        let mut current = match polytope_from_row(&fixture.polytope) {
            Ok(polytope) => polytope,
            Err(err) => {
                rows.push(trace_failure_row(
                    fixture,
                    0,
                    direction_model,
                    branch_threshold_relative,
                    action_window_relative,
                    stop_threshold,
                    0.0,
                    0,
                    "initial_polytope_failed",
                    Some(err.clone()),
                ));
                endpoint_rows.push(endpoint_failure_row(
                    fixture,
                    "initial_polytope_failed",
                    "endpoint_not_run_initial_polytope_failed",
                    None,
                    None,
                    stop_threshold,
                    None,
                ));
                endpoint_direction_scan_rows.push(endpoint_direction_scan_failure_row(
                    fixture,
                    "initial_polytope_failed",
                    0.0,
                    err,
                ));
                continue;
            }
        };
        let mut trace_stop_reason = "trace_iteration_limit".to_string();

        for iteration in 0..trace_iterations {
            let action_gap = if iteration == 0 {
                fixture.polytope.capacity * action_window_relative
            } else {
                match compute_active_sys_state(&current) {
                    Some(state) => state.capacity.min_action * action_window_relative,
                    None => {
                        rows.push(trace_failure_row(
                            fixture,
                            iteration,
                            direction_model,
                            branch_threshold_relative,
                            action_window_relative,
                            stop_threshold,
                            0.0,
                            0,
                            "current_sys_failed",
                            None,
                        ));
                        trace_stop_reason = "current_sys_failed".to_string();
                        break;
                    }
                }
            };
            let base = match compute_base_state_from_polytope(
                current.clone(),
                action_gap,
                branch_threshold_relative,
            ) {
                Ok(base) => base,
                Err(err) => {
                    rows.push(trace_failure_row(
                        fixture,
                        iteration,
                        direction_model,
                        branch_threshold_relative,
                        action_window_relative,
                        stop_threshold,
                        0.0,
                        0,
                        "base_state_failed",
                        Some(err),
                    ));
                    trace_stop_reason = "base_state_failed".to_string();
                    break;
                }
            };
            step_ranking_audit_rows.extend(step_ranking_audit_rows_for_base(
                fixture,
                iteration,
                &base,
                steps,
                include_candidate_window_directions,
                action_window_relative,
                branch_threshold_relative,
                stop_threshold,
            ));
            let Some(candidate) = best_line_search_step(
                fixture,
                iteration,
                direction_model,
                include_candidate_window_directions,
                &base,
                steps,
                action_window_relative,
                branch_threshold_relative,
                stop_threshold,
            ) else {
                rows.push(trace_failure_row(
                    fixture,
                    iteration,
                    direction_model,
                    branch_threshold_relative,
                    action_window_relative,
                    stop_threshold,
                    base.sys,
                    base.near_active_orbits.len(),
                    "no_prediction_selected_step",
                    None,
                ));
                trace_stop_reason = "no_prediction_selected_step".to_string();
                break;
            };

            let accepted = candidate.accepted;
            rows.push(candidate.row);
            if accepted {
                current = candidate.target_polytope;
            } else {
                trace_stop_reason = "line_search_all_steps_below_min_observed_delta".to_string();
                break;
            }
        }

        endpoint_rows.push(endpoint_diagnostic_row(
            fixture,
            &current,
            &trace_stop_reason,
            trace_iterations,
            direction_model,
            include_candidate_window_directions,
            branch_threshold_relative,
            action_window_relative,
            steps,
            stop_threshold,
        ));
        endpoint_direction_scan_rows.extend(endpoint_direction_scan_rows_for_final_state(
            fixture,
            &current,
            branch_threshold_relative,
            action_window_relative,
            direction_model,
            include_candidate_window_directions,
            endpoint_steps,
        ));
    }

    if trace_iterations == 1 {
        align_first_trace_rows_with_probe_rows(&mut rows, probe_rows);
    }
    TraceArtifacts {
        trace_rows: rows,
        endpoint_rows,
        endpoint_direction_scan_rows,
        step_ranking_audit_rows,
    }
}

struct TraceCandidate {
    row: RunTraceRow,
    target_polytope: SysLandscapePolytopeCache,
    accepted: bool,
}

fn step_ranking_audit_rows_for_base(
    fixture: &Fixture,
    iteration: usize,
    base: &BaseState,
    steps: &[f64],
    include_candidate_window_directions: bool,
    action_window_relative: f64,
    branch_threshold_relative: f64,
    stop_threshold: StopThreshold,
) -> Vec<StepRankingAuditRow> {
    let effective_min_observed_delta = stop_threshold.effective_delta(base.sys);
    let mut rows = Vec::new();
    for direction in probe_directions(base, steps, include_candidate_window_directions) {
        for &step in steps {
            if !direction.allows_step(step) {
                continue;
            }
            rows.push(step_ranking_audit_row(
                fixture,
                iteration,
                base,
                &direction,
                step,
                action_window_relative,
                branch_threshold_relative,
                effective_min_observed_delta,
            ));
        }
    }
    assign_descending_ranks(&mut rows);
    rows
}

fn step_ranking_audit_row(
    fixture: &Fixture,
    iteration: usize,
    base: &BaseState,
    direction: &ProbeDirection,
    step: f64,
    action_window_relative: f64,
    branch_threshold_relative: f64,
    effective_min_observed_delta: f64,
) -> StepRankingAuditRow {
    let near_active_predicted_delta_sys =
        branch_model_predicted_delta(base, &direction.vector, step, DirectionModel::NearActive);
    let candidate_window_prediction =
        candidate_window_prediction_witness(base, &direction.vector, step);
    let candidate_window_predicted_delta_sys = candidate_window_prediction
        .as_ref()
        .map(|witness| witness.predicted_delta);
    let target_duals: Vec<Vector4<f64>> = base
        .polytope
        .dual_vertices_f64
        .iter()
        .zip(&direction.vector)
        .map(|(dual, delta)| dual + step * delta)
        .collect();

    let mut status = "ok".to_string();
    let mut target_sys = None;
    let mut observed_delta_sys = None;
    let mut target_orbit_iterations = None;
    let mut decomposition = None;

    match SysLandscapePolytopeCache::from_f64_dual_vertices(target_duals.clone()) {
        Some(target_polytope) => {
            match capacity_auto_with_gap(
                &target_polytope,
                base.capacity.min_action * action_window_relative,
            ) {
                Ok(target_capacity) => {
                    target_orbit_iterations = Some(target_capacity.iterations);
                    match compute_active_sys_state(&target_polytope) {
                        Some(target_state) => {
                            let _target_near_active =
                                near_active_orbits(&target_capacity, branch_threshold_relative);
                            target_sys = Some(target_state.sys);
                            observed_delta_sys = Some(target_state.sys - base.sys);
                            decomposition = candidate_window_decomposition(
                                base,
                                &target_duals,
                                target_state.vol,
                                target_state.sys,
                                &direction.vector,
                                step,
                                candidate_window_prediction.as_ref(),
                            );
                        }
                        None => {
                            status = "target_sys_failed".to_string();
                        }
                    }
                }
                Err(err) => {
                    status = format!("target_capacity_failed:{err:?}");
                }
            }
        }
        None => {
            status = "target_polytope_construction_failed".to_string();
        }
    }

    StepRankingAuditRow {
        poly_id: fixture.polytope.poly_id.clone(),
        degeneracy_label: fixture.diagnostic.degeneracy_label.clone(),
        iteration,
        direction_label: direction.label.clone(),
        step,
        status,
        base_sys: base.sys,
        effective_min_observed_delta,
        near_active_predicted_delta_sys,
        candidate_window_predicted_delta_sys,
        candidate_window_witness_orbit_index: candidate_window_prediction
            .as_ref()
            .map(|witness| witness.orbit_index),
        candidate_window_witness_sigma: candidate_window_prediction
            .as_ref()
            .map(|witness| witness.sigma.clone()),
        candidate_window_witness_action: candidate_window_prediction
            .as_ref()
            .map(|witness| witness.action),
        candidate_window_witness_relative_action_gap: candidate_window_prediction
            .as_ref()
            .map(|witness| witness.relative_action_gap),
        candidate_window_witness_base_gap: candidate_window_prediction
            .as_ref()
            .map(|witness| witness.base_gap),
        candidate_window_witness_derivative: candidate_window_prediction
            .as_ref()
            .map(|witness| witness.derivative),
        decomposition_predicted_sys: decomposition.as_ref().map(|d| d.predicted_sys),
        decomposition_actual_sys: decomposition.as_ref().map(|d| d.actual_sys),
        decomposition_total_prediction_error: decomposition
            .as_ref()
            .map(|d| d.total_prediction_error),
        decomposition_base_window_exact_sys: decomposition
            .as_ref()
            .and_then(|d| d.base_window_exact_sys),
        decomposition_linearization_error: decomposition
            .as_ref()
            .and_then(|d| d.linearization_error),
        decomposition_sigma_set_error: decomposition.as_ref().and_then(|d| d.sigma_set_error),
        decomposition_sum_error: decomposition.as_ref().and_then(|d| d.sum_error),
        decomposition_sum_residual: decomposition.as_ref().and_then(|d| d.sum_residual),
        fixed_winner_actual_action: decomposition
            .as_ref()
            .and_then(|d| d.fixed_winner.as_ref())
            .map(|w| w.actual_action),
        fixed_winner_predicted_action: decomposition
            .as_ref()
            .and_then(|d| d.fixed_winner.as_ref())
            .map(|w| w.predicted_action),
        fixed_winner_action_error: decomposition
            .as_ref()
            .and_then(|d| d.fixed_winner.as_ref())
            .map(|w| w.action_error),
        fixed_winner_actual_volume: decomposition
            .as_ref()
            .and_then(|d| d.fixed_winner.as_ref())
            .map(|w| w.actual_volume),
        fixed_winner_predicted_volume: decomposition
            .as_ref()
            .and_then(|d| d.fixed_winner.as_ref())
            .map(|w| w.predicted_volume),
        fixed_winner_volume_error: decomposition
            .as_ref()
            .and_then(|d| d.fixed_winner.as_ref())
            .map(|w| w.volume_error),
        fixed_winner_actual_sys: decomposition
            .as_ref()
            .and_then(|d| d.fixed_winner.as_ref())
            .map(|w| w.actual_sys),
        fixed_winner_predicted_sys_full: decomposition
            .as_ref()
            .and_then(|d| d.fixed_winner.as_ref())
            .map(|w| w.predicted_sys_full),
        fixed_winner_predicted_sys_actual_action_linear_volume: decomposition
            .as_ref()
            .and_then(|d| d.fixed_winner.as_ref())
            .map(|w| w.predicted_sys_actual_action_linear_volume),
        fixed_winner_predicted_sys_linear_action_actual_volume: decomposition
            .as_ref()
            .and_then(|d| d.fixed_winner.as_ref())
            .map(|w| w.predicted_sys_linear_action_actual_volume),
        fixed_winner_sys_error_full: decomposition
            .as_ref()
            .and_then(|d| d.fixed_winner.as_ref())
            .map(|w| w.sys_error_full),
        fixed_winner_sys_error_action_part: decomposition
            .as_ref()
            .and_then(|d| d.fixed_winner.as_ref())
            .map(|w| w.sys_error_action_part),
        fixed_winner_sys_error_volume_part: decomposition
            .as_ref()
            .and_then(|d| d.fixed_winner.as_ref())
            .map(|w| w.sys_error_volume_part),
        fixed_winner_sys_error_interaction_residual: decomposition
            .as_ref()
            .and_then(|d| d.fixed_winner.as_ref())
            .map(|w| w.sys_error_interaction_residual),
        observed_delta_sys,
        target_sys,
        above_threshold_observed: observed_delta_sys
            .map(|delta| delta > effective_min_observed_delta),
        positive_observed: observed_delta_sys.map(|delta| delta > 0.0),
        near_active_prediction_positive: near_active_predicted_delta_sys.map(|delta| delta > 0.0),
        candidate_window_prediction_positive: candidate_window_predicted_delta_sys
            .map(|delta| delta > 0.0),
        observed_rank_desc: None,
        near_active_rank_desc: None,
        candidate_window_rank_desc: None,
        base_near_active_count: base.near_active_orbits.len(),
        base_candidate_window_count: base.candidate_orbits.len(),
        base_orbit_iterations: base.capacity.iterations,
        target_orbit_iterations,
    }
}

#[derive(Clone, Debug)]
struct PredictionDecomposition {
    predicted_sys: f64,
    actual_sys: f64,
    total_prediction_error: f64,
    base_window_exact_sys: Option<f64>,
    linearization_error: Option<f64>,
    sigma_set_error: Option<f64>,
    sum_error: Option<f64>,
    sum_residual: Option<f64>,
    fixed_winner: Option<FixedWinnerDecomposition>,
}

#[derive(Clone, Debug)]
struct FixedWinnerDecomposition {
    actual_action: f64,
    predicted_action: f64,
    action_error: f64,
    actual_volume: f64,
    predicted_volume: f64,
    volume_error: f64,
    actual_sys: f64,
    predicted_sys_full: f64,
    predicted_sys_actual_action_linear_volume: f64,
    predicted_sys_linear_action_actual_volume: f64,
    sys_error_full: f64,
    sys_error_action_part: f64,
    sys_error_volume_part: f64,
    sys_error_interaction_residual: f64,
}

fn candidate_window_decomposition(
    base: &BaseState,
    target_duals: &[Vector4<f64>],
    target_volume: f64,
    actual_sys: f64,
    direction: &[Vector4<f64>],
    step: f64,
    witness: Option<&CandidateWindowPredictionWitness>,
) -> Option<PredictionDecomposition> {
    let witness = witness?;
    let predicted_sys = base.sys + witness.predicted_delta;
    let total_prediction_error = predicted_sys - actual_sys;

    let base_window_exact_sys = exact_base_window_sys_at_target(base, target_duals, target_volume);
    let linearization_error = base_window_exact_sys.map(|exact| predicted_sys - exact);
    let sigma_set_error = base_window_exact_sys.map(|exact| exact - actual_sys);
    let sum_error = linearization_error
        .zip(sigma_set_error)
        .map(|(left, right)| left + right);
    let sum_residual = sum_error.map(|sum| total_prediction_error - sum);
    let fixed_winner =
        fixed_winner_decomposition(base, target_duals, target_volume, direction, step, witness);

    Some(PredictionDecomposition {
        predicted_sys,
        actual_sys,
        total_prediction_error,
        base_window_exact_sys,
        linearization_error,
        sigma_set_error,
        sum_error,
        sum_residual,
        fixed_winner,
    })
}

fn exact_base_window_sys_at_target(
    base: &BaseState,
    target_duals: &[Vector4<f64>],
    target_volume: f64,
) -> Option<f64> {
    base.candidate_orbits
        .iter()
        .filter_map(|orbit| fixed_sigma_action(target_duals, &orbit.sigma))
        .map(|action| symplectic::systolic_ratio(action, target_volume))
        .filter(|sys| sys.is_finite())
        .min_by(|a, b| a.total_cmp(b))
}

fn fixed_winner_decomposition(
    base: &BaseState,
    target_duals: &[Vector4<f64>],
    target_volume: f64,
    direction: &[Vector4<f64>],
    step: f64,
    witness: &CandidateWindowPredictionWitness,
) -> Option<FixedWinnerDecomposition> {
    let base_orbit = base.candidate_orbits.get(witness.orbit_index)?;
    let actual_action = fixed_sigma_action(target_duals, &base_orbit.sigma)?;
    let base_capacity_gradient =
        capacity_subgradients_a(&base.polytope.dual_vertices_f64, &[base_orbit.clone()])
            .ok()?
            .into_iter()
            .next()?;
    let action_derivative = gradient_direction_dot(&base_capacity_gradient, direction)?;
    let predicted_action = base_orbit.action + step * action_derivative;
    let action_error = predicted_action - actual_action;

    let volume_derivative = gradient_direction_dot(&base.volume_gradient, direction)?;
    let predicted_volume = base.volume + step * volume_derivative;
    if predicted_volume <= 0.0 || !predicted_volume.is_finite() {
        return None;
    }
    let volume_error = predicted_volume - target_volume;

    let actual_sys = symplectic::systolic_ratio(actual_action, target_volume);
    let predicted_sys_full = base.sys + witness.predicted_delta;
    let predicted_sys_actual_action_linear_volume =
        symplectic::systolic_ratio(actual_action, predicted_volume);
    let predicted_sys_linear_action_actual_volume =
        symplectic::systolic_ratio(predicted_action, target_volume);

    let sys_error_full = predicted_sys_full - actual_sys;
    let sys_error_action_part = predicted_sys_linear_action_actual_volume - actual_sys;
    let sys_error_volume_part = predicted_sys_actual_action_linear_volume - actual_sys;
    let sys_error_interaction_residual =
        sys_error_full - sys_error_action_part - sys_error_volume_part;

    Some(FixedWinnerDecomposition {
        actual_action,
        predicted_action,
        action_error,
        actual_volume: target_volume,
        predicted_volume,
        volume_error,
        actual_sys,
        predicted_sys_full,
        predicted_sys_actual_action_linear_volume,
        predicted_sys_linear_action_actual_volume,
        sys_error_full,
        sys_error_action_part,
        sys_error_volume_part,
        sys_error_interaction_residual,
    })
}

fn fixed_sigma_action(dual_vertices: &[Vector4<f64>], sigma: &[usize]) -> Option<f64> {
    let outcome =
        symplectic::kkt::saddle_point_solver::solve_kkt_for_dual_vertices(dual_vertices, sigma);
    let result = outcome.feasible()?;
    let action = 0.5 / result.q_corrected;
    action.is_finite().then_some(action)
}

fn assign_descending_ranks(rows: &mut [StepRankingAuditRow]) {
    let observed = ranked_indices(
        rows.iter()
            .enumerate()
            .filter_map(|(index, row)| row.observed_delta_sys.map(|value| (index, value)))
            .collect(),
    );
    for (rank, index) in observed {
        rows[index].observed_rank_desc = Some(rank);
    }

    let near_active = ranked_indices(
        rows.iter()
            .enumerate()
            .filter_map(|(index, row)| {
                row.near_active_predicted_delta_sys
                    .map(|value| (index, value))
            })
            .collect(),
    );
    for (rank, index) in near_active {
        rows[index].near_active_rank_desc = Some(rank);
    }

    let candidate_window = ranked_indices(
        rows.iter()
            .enumerate()
            .filter_map(|(index, row)| {
                row.candidate_window_predicted_delta_sys
                    .map(|value| (index, value))
            })
            .collect(),
    );
    for (rank, index) in candidate_window {
        rows[index].candidate_window_rank_desc = Some(rank);
    }
}

fn ranked_indices(mut scored: Vec<(usize, f64)>) -> Vec<(usize, usize)> {
    scored.retain(|(_, score)| score.is_finite());
    scored.sort_by(|(_, left), (_, right)| right.total_cmp(left));
    scored
        .into_iter()
        .enumerate()
        .map(|(position, (index, _))| (position + 1, index))
        .collect()
}

fn best_line_search_step(
    fixture: &Fixture,
    iteration: usize,
    direction_model: DirectionModel,
    include_candidate_window_directions: bool,
    base: &BaseState,
    steps: &[f64],
    action_window_relative: f64,
    branch_threshold_relative: f64,
    stop_threshold: StopThreshold,
) -> Option<TraceCandidate> {
    let effective_min_observed_delta = stop_threshold.effective_delta(base.sys);
    let mut candidates = Vec::new();
    for direction in probe_directions(base, steps, include_candidate_window_directions) {
        let best_predicted_delta = steps
            .iter()
            .filter(|step| direction.allows_step(**step))
            .filter_map(|step| {
                branch_model_predicted_delta(base, &direction.vector, *step, direction_model)
            })
            .filter(|delta| delta.is_finite())
            .max_by(|a, b| a.total_cmp(b));
        let Some(best_predicted_delta) = best_predicted_delta else {
            continue;
        };
        if best_predicted_delta == 0.0 {
            continue;
        }
        candidates.push((direction, best_predicted_delta));
    }
    candidates.sort_by(|(_, a), (_, b)| b.total_cmp(a));

    let mut attempts = 0usize;
    let mut rejected_steps = Vec::new();
    let mut attempted_direction_labels = Vec::new();
    let mut rejected_direction_labels = Vec::new();
    let mut last_direction_label = None;
    let mut last_rejected_predicted_delta = None;
    let mut last_rejected_observed_delta = None;

    for (direction, _) in candidates {
        attempted_direction_labels.push(direction.label.clone());
        last_direction_label = Some(direction.label.clone());
        let rejected_count_before_direction = rejected_steps.len();

        for (step, predicted_delta) in
            predicted_steps_for_direction(base, &direction, steps, direction_model)
        {
            attempts += 1;
            last_rejected_predicted_delta = Some(predicted_delta);
            let target_duals: Vec<Vector4<f64>> = base
                .polytope
                .dual_vertices_f64
                .iter()
                .zip(&direction.vector)
                .map(|(dual, delta)| dual + step * delta)
                .collect();
            let Some(target_polytope) =
                SysLandscapePolytopeCache::from_f64_dual_vertices(target_duals)
            else {
                rejected_steps.push(step);
                continue;
            };
            let Ok(target_capacity) = capacity_auto_with_gap(
                &target_polytope,
                base.capacity.min_action * action_window_relative,
            ) else {
                rejected_steps.push(step);
                continue;
            };
            let Some(target_state) = compute_active_sys_state(&target_polytope) else {
                rejected_steps.push(step);
                continue;
            };
            let target_near_active =
                near_active_orbits(&target_capacity, branch_threshold_relative);
            let observed_delta = target_state.sys - base.sys;
            if observed_delta <= effective_min_observed_delta {
                rejected_steps.push(step);
                last_rejected_observed_delta = Some(observed_delta);
                continue;
            }

            return Some(TraceCandidate {
                row: RunTraceRow {
                    poly_id: fixture.polytope.poly_id.clone(),
                    degeneracy_label: fixture.diagnostic.degeneracy_label.clone(),
                    iteration,
                    method_variant: direction_model.method_variant().to_string(),
                    branch_threshold_relative,
                    action_window_relative,
                    direction_model: direction_model.as_str().to_string(),
                    base_near_active_count: base.near_active_orbits.len(),
                    target_near_active_count: Some(target_near_active.len()),
                    chosen_direction_label: Some(direction.label),
                    chosen_step: Some(step),
                    attempted_direction_labels,
                    rejected_direction_labels,
                    line_search_attempts: attempts,
                    rejected_steps,
                    line_search_status: "accepted".to_string(),
                    min_observed_delta: stop_threshold.absolute_delta,
                    min_observed_relative_delta: stop_threshold.relative_delta,
                    effective_min_observed_delta,
                    predicted_delta_sys: Some(predicted_delta),
                    observed_delta_sys: Some(observed_delta),
                    base_sys: base.sys,
                    target_sys: Some(target_state.sys),
                    base_orbit_iterations: base.capacity.iterations,
                    target_orbit_iterations: Some(target_capacity.iterations),
                    accepted: true,
                    stop_reason: "accepted_observed_delta_above_threshold".to_string(),
                },
                target_polytope,
                accepted: true,
            });
        }

        if rejected_steps.len() > rejected_count_before_direction {
            rejected_direction_labels.push(direction.label);
        }
    }

    Some(TraceCandidate {
        row: RunTraceRow {
            poly_id: fixture.polytope.poly_id.clone(),
            degeneracy_label: fixture.diagnostic.degeneracy_label.clone(),
            iteration,
            method_variant: direction_model.method_variant().to_string(),
            branch_threshold_relative,
            action_window_relative,
            direction_model: direction_model.as_str().to_string(),
            base_near_active_count: base.near_active_orbits.len(),
            target_near_active_count: None,
            chosen_direction_label: last_direction_label,
            chosen_step: None,
            attempted_direction_labels,
            rejected_direction_labels,
            line_search_attempts: attempts,
            rejected_steps,
            line_search_status: "all_steps_below_min_observed_delta".to_string(),
            min_observed_delta: stop_threshold.absolute_delta,
            min_observed_relative_delta: stop_threshold.relative_delta,
            effective_min_observed_delta,
            predicted_delta_sys: last_rejected_predicted_delta,
            observed_delta_sys: last_rejected_observed_delta,
            base_sys: base.sys,
            target_sys: None,
            base_orbit_iterations: base.capacity.iterations,
            target_orbit_iterations: None,
            accepted: false,
            stop_reason: "line_search_all_steps_below_min_observed_delta".to_string(),
        },
        target_polytope: base.polytope.clone(),
        accepted: false,
    })
}

fn predicted_steps_for_direction(
    base: &BaseState,
    direction: &ProbeDirection,
    steps: &[f64],
    direction_model: DirectionModel,
) -> Vec<(f64, f64)> {
    let mut scored_steps: Vec<(f64, f64)> = steps
        .iter()
        .copied()
        .filter(|step| direction.allows_step(*step))
        .filter_map(|step| {
            branch_model_predicted_delta(base, &direction.vector, step, direction_model)
                .filter(|delta| delta.is_finite())
                .map(|delta| (step, delta))
        })
        .collect();
    scored_steps.sort_by(|(_, left), (_, right)| right.total_cmp(left));
    scored_steps
}

fn trace_failure_row(
    fixture: &Fixture,
    iteration: usize,
    direction_model: DirectionModel,
    branch_threshold_relative: f64,
    action_window_relative: f64,
    stop_threshold: StopThreshold,
    base_sys: f64,
    base_near_active_count: usize,
    stop_reason: &str,
    detail: Option<String>,
) -> RunTraceRow {
    RunTraceRow {
        poly_id: fixture.polytope.poly_id.clone(),
        degeneracy_label: fixture.diagnostic.degeneracy_label.clone(),
        iteration,
        method_variant: direction_model.method_variant().to_string(),
        branch_threshold_relative,
        action_window_relative,
        direction_model: direction_model.as_str().to_string(),
        base_near_active_count,
        target_near_active_count: None,
        chosen_direction_label: None,
        chosen_step: None,
        attempted_direction_labels: Vec::new(),
        rejected_direction_labels: Vec::new(),
        line_search_attempts: 0,
        rejected_steps: Vec::new(),
        line_search_status: "not_run".to_string(),
        min_observed_delta: stop_threshold.absolute_delta,
        min_observed_relative_delta: stop_threshold.relative_delta,
        effective_min_observed_delta: stop_threshold.effective_delta(base_sys),
        predicted_delta_sys: None,
        observed_delta_sys: None,
        base_sys,
        target_sys: None,
        base_orbit_iterations: 0,
        target_orbit_iterations: None,
        accepted: false,
        stop_reason: match detail {
            Some(detail) => format!("{stop_reason}:{detail}"),
            None => stop_reason.to_string(),
        },
    }
}

fn endpoint_diagnostic_row(
    fixture: &Fixture,
    final_polytope: &SysLandscapePolytopeCache,
    trace_stop_reason: &str,
    trace_iterations: usize,
    direction_model: DirectionModel,
    include_candidate_window_directions: bool,
    branch_threshold_relative: f64,
    action_window_relative: f64,
    steps: &[f64],
    stop_threshold: StopThreshold,
) -> EndpointDiagnosticRow {
    let active_state = match compute_active_sys_state(final_polytope) {
        Some(state) => state,
        None => {
            return endpoint_failure_row(
                fixture,
                trace_stop_reason,
                "final_sys_failed",
                None,
                None,
                stop_threshold,
                None,
            );
        }
    };
    let base = match compute_base_state_from_polytope(
        final_polytope.clone(),
        active_state.capacity.min_action * action_window_relative,
        branch_threshold_relative,
    ) {
        Ok(base) => base,
        Err(err) => {
            return endpoint_failure_row(
                fixture,
                trace_stop_reason,
                "final_base_state_failed",
                Some(active_state.sys),
                None,
                stop_threshold,
                Some(err),
            );
        }
    };
    let effective_min_observed_delta = stop_threshold.effective_delta(base.sys);
    let candidate = best_line_search_step(
        fixture,
        trace_iterations,
        direction_model,
        include_candidate_window_directions,
        &base,
        steps,
        action_window_relative,
        branch_threshold_relative,
        stop_threshold,
    );

    match candidate {
        Some(candidate) => {
            let row = candidate.row;
            let improvement_found = row.observed_delta_sys.is_some_and(|delta| delta > 0.0);
            let threshold_improvement_found = row
                .observed_delta_sys
                .is_some_and(|delta| delta > effective_min_observed_delta);
            EndpointDiagnosticRow {
                poly_id: fixture.polytope.poly_id.clone(),
                degeneracy_label: fixture.diagnostic.degeneracy_label.clone(),
                trace_stop_reason: trace_stop_reason.to_string(),
                final_sys: Some(base.sys),
                final_near_active_count: Some(base.near_active_orbits.len()),
                post_stop_direction_label: row.chosen_direction_label,
                post_stop_step: row.chosen_step,
                post_stop_line_search_attempts: Some(row.line_search_attempts),
                post_stop_rejected_steps: row.rejected_steps,
                post_stop_line_search_status: Some(row.line_search_status),
                min_observed_delta: stop_threshold.absolute_delta,
                min_observed_relative_delta: stop_threshold.relative_delta,
                effective_min_observed_delta: Some(effective_min_observed_delta),
                post_stop_prediction_selected_step_found: Some(true),
                post_stop_predicted_delta_sys: row.predicted_delta_sys,
                post_stop_observed_delta_sys: row.observed_delta_sys,
                post_stop_improvement_found: Some(improvement_found),
                post_stop_threshold_improvement_found: Some(threshold_improvement_found),
                diagnostic_status: if threshold_improvement_found {
                    "post_stop_prediction_selected_above_threshold_improvement_found"
                } else if improvement_found {
                    "post_stop_prediction_selected_positive_below_threshold"
                } else {
                    "post_stop_prediction_selected_no_positive_observed_improvement"
                }
                .to_string(),
                base_orbit_iterations: Some(row.base_orbit_iterations),
                target_orbit_iterations: row.target_orbit_iterations,
                caveat: "prediction-selected post-stop finite probe; use endpoint-direction-scan rows for all generated post-stop directions; not a local-maximum certificate"
                    .to_string(),
            }
        }
        None => EndpointDiagnosticRow {
            poly_id: fixture.polytope.poly_id.clone(),
            degeneracy_label: fixture.diagnostic.degeneracy_label.clone(),
            trace_stop_reason: trace_stop_reason.to_string(),
            final_sys: Some(base.sys),
            final_near_active_count: Some(base.near_active_orbits.len()),
            post_stop_direction_label: None,
            post_stop_step: None,
            post_stop_line_search_attempts: None,
            post_stop_rejected_steps: Vec::new(),
            post_stop_line_search_status: None,
            min_observed_delta: stop_threshold.absolute_delta,
            min_observed_relative_delta: stop_threshold.relative_delta,
            effective_min_observed_delta: Some(effective_min_observed_delta),
            post_stop_prediction_selected_step_found: Some(false),
            post_stop_predicted_delta_sys: None,
            post_stop_observed_delta_sys: None,
            post_stop_improvement_found: Some(false),
            post_stop_threshold_improvement_found: Some(false),
            diagnostic_status: "no_prediction_selected_post_stop_step".to_string(),
            base_orbit_iterations: Some(base.capacity.iterations),
            target_orbit_iterations: None,
            caveat: "finite direction set only; absence of a prediction-selected step is not a local-maximum certificate"
                .to_string(),
        },
    }
}

fn endpoint_failure_row(
    fixture: &Fixture,
    trace_stop_reason: &str,
    diagnostic_status: &str,
    final_sys: Option<f64>,
    final_near_active_count: Option<usize>,
    stop_threshold: StopThreshold,
    detail: Option<String>,
) -> EndpointDiagnosticRow {
    EndpointDiagnosticRow {
        poly_id: fixture.polytope.poly_id.clone(),
        degeneracy_label: fixture.diagnostic.degeneracy_label.clone(),
        trace_stop_reason: trace_stop_reason.to_string(),
        final_sys,
        final_near_active_count,
        post_stop_direction_label: None,
        post_stop_step: None,
        post_stop_line_search_attempts: None,
        post_stop_rejected_steps: Vec::new(),
        post_stop_line_search_status: None,
        min_observed_delta: stop_threshold.absolute_delta,
        min_observed_relative_delta: stop_threshold.relative_delta,
        effective_min_observed_delta: final_sys.map(|sys| stop_threshold.effective_delta(sys)),
        post_stop_prediction_selected_step_found: None,
        post_stop_predicted_delta_sys: None,
        post_stop_observed_delta_sys: None,
        post_stop_improvement_found: None,
        post_stop_threshold_improvement_found: None,
        diagnostic_status: match detail {
            Some(detail) => format!("{diagnostic_status}:{detail}"),
            None => diagnostic_status.to_string(),
        },
        base_orbit_iterations: None,
        target_orbit_iterations: None,
        caveat: "endpoint diagnostic failed before a post-stop probe could run".to_string(),
    }
}

fn endpoint_direction_scan_rows_for_final_state(
    fixture: &Fixture,
    final_polytope: &SysLandscapePolytopeCache,
    branch_threshold_relative: f64,
    action_window_relative: f64,
    direction_model: DirectionModel,
    include_candidate_window_directions: bool,
    steps: &[f64],
) -> Vec<LocalGeometryProbeRow> {
    let active_state = match compute_active_sys_state(final_polytope) {
        Some(state) => state,
        None => {
            return vec![endpoint_direction_scan_failure_row(
                fixture,
                "final_sys_failed",
                0.0,
                "compute_active_sys_state returned None".to_string(),
            )];
        }
    };
    let base = match compute_base_state_from_polytope(
        final_polytope.clone(),
        active_state.capacity.min_action * action_window_relative,
        branch_threshold_relative,
    ) {
        Ok(base) => base,
        Err(err) => {
            return vec![endpoint_direction_scan_failure_row(
                fixture,
                "final_base_state_failed",
                active_state.sys,
                err,
            )];
        }
    };

    let mut rows = Vec::new();
    for direction in probe_directions(&base, steps, include_candidate_window_directions) {
        let scan_direction_label = format!("post_stop_{}", direction.label);
        for &step in steps {
            if !direction.allows_step(step) {
                continue;
            }
            rows.push(local_probe_row(
                fixture,
                &base,
                &scan_direction_label,
                &direction.vector,
                step,
                direction_model,
                action_window_relative,
                branch_threshold_relative,
            ));
        }
    }
    rows
}

fn endpoint_direction_scan_failure_row(
    fixture: &Fixture,
    status: &str,
    base_sys: f64,
    detail: String,
) -> LocalGeometryProbeRow {
    LocalGeometryProbeRow {
        poly_id: fixture.polytope.poly_id.clone(),
        degeneracy_label: fixture.diagnostic.degeneracy_label.clone(),
        direction_label: "post_stop_direction_scan".to_string(),
        step: 0.0,
        status: format!("{status}:{detail}"),
        base_sys,
        predicted_delta_per_step: None,
        predicted_delta_sys: None,
        recomputed_sys: None,
        observed_delta_sys: None,
        target_near_active_count: None,
        target_best_sigma_in_base_near_active_set: None,
        target_best_sigma_in_base_candidate_window: None,
        base_near_active_count: 0,
        base_returned_orbit_count: 0,
        base_orbit_iterations: 0,
        target_orbit_iterations: None,
    }
}

fn align_first_trace_rows_with_probe_rows(
    rows: &mut [RunTraceRow],
    probe_rows: &[LocalGeometryProbeRow],
) {
    for row in rows {
        if row.iteration != 0 || row.chosen_direction_label.is_none() || row.chosen_step.is_none() {
            continue;
        }
        let direction = row.chosen_direction_label.as_ref().unwrap();
        let step = row.chosen_step.unwrap();
        if let Some(probe) = probe_rows.iter().find(|probe| {
            probe.poly_id == row.poly_id
                && probe.direction_label == *direction
                && (probe.step - step).abs() <= 1.0e-15
        }) {
            row.target_near_active_count = probe.target_near_active_count;
            row.predicted_delta_sys = probe.predicted_delta_sys;
            row.observed_delta_sys = probe.observed_delta_sys;
            row.target_sys = probe.recomputed_sys;
            row.accepted = probe
                .observed_delta_sys
                .is_some_and(|delta| delta > row.effective_min_observed_delta);
            row.stop_reason = if row.accepted {
                "accepted_observed_delta_above_threshold"
            } else {
                "rejected_observed_delta_below_threshold"
            }
            .to_string();
        }
    }
}

fn compute_base_state_from_row(
    row: &PolytopeRow,
    action_window_relative: f64,
    branch_threshold_relative: f64,
) -> Result<BaseState, String> {
    let polytope = polytope_from_row(row)?;
    compute_base_state_from_polytope(
        polytope,
        row.capacity * action_window_relative,
        branch_threshold_relative,
    )
}

fn compute_base_state_from_polytope(
    polytope: SysLandscapePolytopeCache,
    action_gap: f64,
    branch_threshold_relative: f64,
) -> Result<BaseState, String> {
    let capacity = capacity_auto_with_gap(&polytope, action_gap)
        .map_err(|err| format!("base_capacity_failed:{err:?}"))?;
    let vol =
        exact_volume_from_incidence_as_f64(&polytope.vertices, &polytope.vertex_facet_incidence);
    if !vol.is_finite() || vol <= 0.0 {
        return Err("base_volume_failed".to_string());
    }
    let sys = symplectic::systolic_ratio(capacity.min_action, vol);
    if !sys.is_finite() {
        return Err("base_sys_failed".to_string());
    }
    let near_active_orbits = near_active_orbits(&capacity, branch_threshold_relative);
    let d_volume_da = volume_derivatives_a(
        &polytope.dual_vertices_f64,
        &polytope.vertices_f64,
        &polytope.vertex_facet_incidence,
    )
    .map_err(|err| format!("volume_derivative_failed:{err:?}"))?;
    let d_capacity_da = capacity_subgradients_a(&polytope.dual_vertices_f64, &near_active_orbits)
        .map_err(|err| format!("capacity_derivative_failed:{err:?}"))?;
    let sys_gradients: Vec<Vec<Vector4<f64>>> = d_capacity_da
        .iter()
        .map(|capacity_gradient| {
            systolic_ratio_gradient_a(capacity.min_action, vol, capacity_gradient, &d_volume_da)
        })
        .collect();
    let candidate_capacity_gradients =
        capacity_subgradients_a(&polytope.dual_vertices_f64, &capacity.orbits)
            .map_err(|err| format!("candidate_capacity_derivative_failed:{err:?}"))?;
    let candidate_sys_gradients: Vec<Vec<Vector4<f64>>> = candidate_capacity_gradients
        .iter()
        .zip(capacity.orbits.iter())
        .map(|(capacity_gradient, orbit)| {
            systolic_ratio_gradient_a(orbit.action, vol, capacity_gradient, &d_volume_da)
        })
        .collect();

    Ok(BaseState {
        polytope,
        candidate_orbits: capacity.orbits.clone(),
        capacity,
        volume: vol,
        volume_gradient: d_volume_da,
        sys,
        near_active_orbits,
        sys_gradients,
        candidate_sys_gradients,
    })
}

fn local_probe_row(
    fixture: &Fixture,
    base: &BaseState,
    direction_label: &str,
    direction: &[Vector4<f64>],
    step: f64,
    direction_model: DirectionModel,
    action_window_relative: f64,
    branch_threshold_relative: f64,
) -> LocalGeometryProbeRow {
    let Some(predicted_delta) =
        branch_model_predicted_delta(base, direction, step, direction_model)
    else {
        return failed_probe_row(
            fixture,
            base,
            direction_label,
            step,
            "branch_model_prediction_failed".to_string(),
        );
    };
    let predicted_delta_per_step = if step == 0.0 {
        0.0
    } else {
        predicted_delta / step
    };
    let target_duals: Vec<Vector4<f64>> = base
        .polytope
        .dual_vertices_f64
        .iter()
        .zip(direction)
        .map(|(dual, delta)| dual + step * delta)
        .collect();
    let Some(target_polytope) = SysLandscapePolytopeCache::from_f64_dual_vertices(target_duals)
    else {
        return failed_probe_row(
            fixture,
            base,
            direction_label,
            step,
            "target_polytope_construction_failed".to_string(),
        );
    };
    let target_capacity = match capacity_auto_with_gap(
        &target_polytope,
        base.capacity.min_action * action_window_relative,
    ) {
        Ok(capacity) => capacity,
        Err(err) => {
            return failed_probe_row(
                fixture,
                base,
                direction_label,
                step,
                format!("target_capacity_failed:{err:?}"),
            );
        }
    };
    let target_state = match compute_active_sys_state(&target_polytope) {
        Some(state) => state,
        None => {
            return failed_probe_row(
                fixture,
                base,
                direction_label,
                step,
                "target_sys_failed".to_string(),
            );
        }
    };
    let target_near_active = near_active_orbits(&target_capacity, branch_threshold_relative);
    let target_best_sigma = target_capacity.best_sigma().to_vec();
    let target_best_sigma_in_base_near_active_set = base
        .near_active_orbits
        .iter()
        .any(|orbit| orbit.sigma == target_best_sigma);
    let target_best_sigma_in_base_candidate_window = base
        .candidate_orbits
        .iter()
        .any(|orbit| orbit.sigma == target_best_sigma);

    LocalGeometryProbeRow {
        poly_id: fixture.polytope.poly_id.clone(),
        degeneracy_label: fixture.diagnostic.degeneracy_label.clone(),
        direction_label: direction_label.to_string(),
        step,
        status: "ok".to_string(),
        base_sys: base.sys,
        predicted_delta_per_step: Some(predicted_delta_per_step),
        predicted_delta_sys: Some(predicted_delta),
        recomputed_sys: Some(target_state.sys),
        observed_delta_sys: Some(target_state.sys - base.sys),
        target_near_active_count: Some(target_near_active.len()),
        target_best_sigma_in_base_near_active_set: Some(target_best_sigma_in_base_near_active_set),
        target_best_sigma_in_base_candidate_window: Some(
            target_best_sigma_in_base_candidate_window,
        ),
        base_near_active_count: base.near_active_orbits.len(),
        base_returned_orbit_count: base.capacity.orbits.len(),
        base_orbit_iterations: base.capacity.iterations,
        target_orbit_iterations: Some(target_capacity.iterations),
    }
}

fn failed_probe_row(
    fixture: &Fixture,
    base: &BaseState,
    direction_label: &str,
    step: f64,
    status: String,
) -> LocalGeometryProbeRow {
    LocalGeometryProbeRow {
        poly_id: fixture.polytope.poly_id.clone(),
        degeneracy_label: fixture.diagnostic.degeneracy_label.clone(),
        direction_label: direction_label.to_string(),
        step,
        status,
        base_sys: base.sys,
        predicted_delta_per_step: None,
        predicted_delta_sys: None,
        recomputed_sys: None,
        observed_delta_sys: None,
        target_near_active_count: None,
        target_best_sigma_in_base_near_active_set: None,
        target_best_sigma_in_base_candidate_window: None,
        base_near_active_count: base.near_active_orbits.len(),
        base_returned_orbit_count: base.capacity.orbits.len(),
        base_orbit_iterations: base.capacity.iterations,
        target_orbit_iterations: None,
    }
}

fn branch_model_predicted_delta(
    base: &BaseState,
    direction: &[Vector4<f64>],
    step: f64,
    direction_model: DirectionModel,
) -> Option<f64> {
    match direction_model {
        DirectionModel::NearActive => {
            clarke_directional_derivative_a(&base.sys_gradients, direction)
                .ok()
                .map(|derivative| step * derivative)
        }
        DirectionModel::CandidateWindow => {
            candidate_window_prediction_witness(base, direction, step)
                .map(|witness| witness.predicted_delta)
        }
    }
}

#[derive(Clone, Debug)]
struct CandidateWindowPredictionWitness {
    orbit_index: usize,
    sigma: Vec<usize>,
    action: f64,
    relative_action_gap: f64,
    base_gap: f64,
    derivative: f64,
    predicted_delta: f64,
}

fn candidate_window_prediction_witness(
    base: &BaseState,
    direction: &[Vector4<f64>],
    step: f64,
) -> Option<CandidateWindowPredictionWitness> {
    if base.candidate_orbits.len() != base.candidate_sys_gradients.len() {
        return None;
    }
    let min_action = base.capacity.min_action;
    base.candidate_orbits
        .iter()
        .zip(base.candidate_sys_gradients.iter())
        .enumerate()
        .filter_map(|(orbit_index, (orbit, gradient))| {
            let action_ratio = orbit.action / min_action;
            let base_gap = base.sys * (action_ratio * action_ratio - 1.0);
            let derivative = gradient_direction_dot(gradient, direction)?;
            let predicted_delta = base_gap + step * derivative;
            predicted_delta
                .is_finite()
                .then(|| CandidateWindowPredictionWitness {
                    orbit_index,
                    sigma: orbit.sigma.clone(),
                    action: orbit.action,
                    relative_action_gap: action_ratio - 1.0,
                    base_gap,
                    derivative,
                    predicted_delta,
                })
        })
        .min_by(|a, b| a.predicted_delta.total_cmp(&b.predicted_delta))
}

fn gradient_direction_dot(gradient: &[Vector4<f64>], direction: &[Vector4<f64>]) -> Option<f64> {
    if gradient.len() != direction.len() {
        return None;
    }
    Some(
        gradient
            .iter()
            .zip(direction)
            .map(|(grad, delta)| grad.dot(delta))
            .sum(),
    )
}

fn probe_directions(
    base: &BaseState,
    steps: &[f64],
    include_candidate_window_directions: bool,
) -> Vec<ProbeDirection> {
    let mut directions = Vec::new();
    if let Some(first_gradient) = base.sys_gradients.first() {
        if let Some(direction) = normalize_direction(first_gradient) {
            directions.push(ProbeDirection {
                label: "single_near_active_gradient".to_string(),
                vector: direction.clone(),
                only_step: None,
            });
            directions.push(ProbeDirection {
                label: "negative_single_near_active_gradient".to_string(),
                vector: direction.iter().map(|v| -*v).collect(),
                only_step: None,
            });
        }
    }
    if base.sys_gradients.len() > 1 {
        if let Some(direction) = box_lp_normalized_direction(&base.sys_gradients) {
            directions.push(ProbeDirection {
                label: "near_active_box_lp_normalized_direction".to_string(),
                vector: direction,
                only_step: None,
            });
        }
    }
    if include_candidate_window_directions {
        for &step in steps {
            if let Some(direction) = candidate_window_box_lp_normalized_direction(base, step) {
                directions.push(ProbeDirection {
                    label: format!(
                        "candidate_window_box_lp_normalized_step_{}",
                        format_step_label(step)
                    ),
                    vector: direction,
                    only_step: Some(step),
                });
            }
        }
    }
    let facet_count = base.polytope.facet_count();
    for seed in 0..1 {
        if let Some(vector) = deterministic_random_direction(facet_count, seed) {
            directions.push(ProbeDirection {
                label: format!("random_direction_{seed}"),
                vector,
                only_step: None,
            });
        }
    }
    let originals = directions.clone();
    for (idx, direction) in originals.iter().take(1).enumerate() {
        for seed in 0..1 {
            let Some(noise) = deterministic_random_direction(facet_count, 100 + 10 * idx + seed)
            else {
                continue;
            };
            let mixed: Vec<Vector4<f64>> = direction
                .vector
                .iter()
                .zip(noise)
                .map(|(base, perturb)| base + 0.25 * perturb)
                .collect();
            if let Some(vector) = normalize_direction(&mixed) {
                directions.push(ProbeDirection {
                    label: format!("angled_{}_{}", direction.label, seed),
                    vector,
                    only_step: direction.only_step,
                });
            }
        }
    }
    directions
}

fn deterministic_random_direction(facet_count: usize, seed: usize) -> Option<Vec<Vector4<f64>>> {
    let mut direction = Vec::with_capacity(facet_count);
    for facet in 0..facet_count {
        let coords = [0, 1, 2, 3].map(|coord| {
            let x = ((seed + 1) * 1009 + (facet + 1) * 9176 + (coord + 3) * 7919) as f64;
            (x.sin() * 12_989.0).fract() * 2.0 - 1.0
        });
        direction.push(Vector4::new(coords[0], coords[1], coords[2], coords[3]));
    }
    normalize_direction(&direction)
}

fn candidate_window_box_lp_normalized_direction(
    base: &BaseState,
    step: f64,
) -> Option<Vec<Vector4<f64>>> {
    if step <= 0.0 || base.candidate_orbits.len() != base.candidate_sys_gradients.len() {
        return None;
    }
    let facet_count = base.candidate_sys_gradients.first()?.len();
    let dim = facet_count * 4;
    let min_action = base.capacity.min_action;
    let mut vars = variables!();
    let direction_vars: Vec<_> = (0..dim)
        .map(|_| vars.add(variable().min(-1.0).max(1.0)))
        .collect();
    let t_var = vars.add(variable().min(f64::NEG_INFINITY));

    let mut model = vars.maximise(Expression::from(t_var)).using(default_solver);
    for (orbit, gradient) in base
        .candidate_orbits
        .iter()
        .zip(base.candidate_sys_gradients.iter())
    {
        if gradient.len() != facet_count {
            return None;
        }
        let action_ratio = orbit.action / min_action;
        let base_gap = base.sys * (action_ratio * action_ratio - 1.0);
        if !base_gap.is_finite() {
            continue;
        }
        let flat = flatten_gradient(gradient);
        let mut lhs = Expression::from(base_gap);
        for (coeff, var) in flat.iter().zip(&direction_vars) {
            if *coeff != 0.0 {
                lhs += step * *coeff * *var;
            }
        }
        model = model.with(constraint!(lhs >= t_var));
    }

    let solution = model.solve().ok()?;
    let flat_direction: Vec<f64> = direction_vars
        .iter()
        .map(|var| solution.value(*var))
        .collect();
    normalize_direction(&unflatten_direction(&flat_direction))
}

fn format_step_label(step: f64) -> String {
    format!("{step:.0e}").replace('+', "")
}

fn steps_match(left: f64, right: f64) -> bool {
    (left - right).abs() <= 1.0e-15
}

fn box_lp_normalized_direction(sys_gradients: &[Vec<Vector4<f64>>]) -> Option<Vec<Vector4<f64>>> {
    let facet_count = sys_gradients.first()?.len();
    let dim = facet_count * 4;
    let mut vars = variables!();
    let direction_vars: Vec<_> = (0..dim)
        .map(|_| vars.add(variable().min(-1.0).max(1.0)))
        .collect();
    let t_var = vars.add(variable().min(f64::NEG_INFINITY));

    let mut model = vars.maximise(Expression::from(t_var)).using(default_solver);
    for gradient in sys_gradients {
        let flat = flatten_gradient(gradient);
        let mut lhs = Expression::from(0.0);
        for (coeff, var) in flat.iter().zip(&direction_vars) {
            if *coeff != 0.0 {
                lhs += *coeff * *var;
            }
        }
        model = model.with(constraint!(lhs >= t_var));
    }

    let solution = model.solve().ok()?;
    let flat_direction: Vec<f64> = direction_vars
        .iter()
        .map(|var| solution.value(*var))
        .collect();
    normalize_direction(&unflatten_direction(&flat_direction))
}

fn near_active_orbits(result: &OrbitSearchResult, threshold_relative: f64) -> Vec<OrbitKktData> {
    let cutoff = result.min_action * (1.0 + threshold_relative.max(0.0));
    let mut active: Vec<OrbitKktData> = result
        .orbits
        .iter()
        .filter(|orbit| {
            matches!(
                orbit.admissibility,
                OrbitAdmissibility::AdmissibleF64 | OrbitAdmissibility::AdmissibleExact
            )
        })
        .filter(|orbit| orbit.action <= cutoff)
        .cloned()
        .collect();
    if active.is_empty() {
        active.push(result.best_orbit().clone());
    }
    active
}

fn capacity_auto_with_gap(
    polytope: &SysLandscapePolytopeCache,
    action_gap: f64,
) -> Result<OrbitSearchResult, OrbitSearchError> {
    if let Ok(classification) = classify_facets_from_dual_vertices(&polytope.dual_vertices_f64) {
        let transition_is_allowed =
            symplectic::algorithms::facet_adjacency::build_transition_matrix_from_facet_intersections_and_omega(
                &polytope.facet_intersection_is_nonempty,
                &polytope.omega_signs,
            );
        let (orbits, iterations) = solve_billiard_candidates(
            &polytope.dual_vertices_f64,
            &classification.q_indices,
            &classification.p_indices,
            &polytope.facet_intersection_is_nonempty,
            &transition_is_allowed,
        )?;
        return aggregate_orbits_with_dual_vertices_exact(
            &polytope.dual_vertices,
            orbits,
            iterations,
            action_gap.max(0.0),
            OrbitGuaranteeMode::AllSafe,
        );
    }

    let transition_is_allowed =
        symplectic::algorithms::facet_adjacency::build_transition_matrix_from_facet_intersections_and_omega(
            &polytope.facet_intersection_is_nonempty,
            &polytope.omega_signs,
        );
    let (orbits, iterations) =
        solve_pruned_hk2017_candidates(&polytope.dual_vertices_f64, &transition_is_allowed)?;
    aggregate_orbits_with_dual_vertices_exact(
        &polytope.dual_vertices,
        orbits,
        iterations,
        action_gap.max(0.0),
        OrbitGuaranteeMode::AllSafe,
    )
}

fn polytope_from_row(row: &PolytopeRow) -> Result<SysLandscapePolytopeCache, String> {
    let dual_vertices: Vec<Vector4<f64>> = row
        .dual_vertices_f64
        .iter()
        .map(|v| Vector4::new(v[0], v[1], v[2], v[3]))
        .collect();
    SysLandscapePolytopeCache::from_f64_dual_vertices(dual_vertices)
        .ok_or_else(|| "failed_to_construct_polytope".to_string())
}

fn fixture_row(fixture: &Fixture) -> FixtureRow {
    FixtureRow {
        poly_id: fixture.polytope.poly_id.clone(),
        degeneracy_label: fixture.diagnostic.degeneracy_label.clone(),
        selection_rank_within_label: fixture.selection_rank_within_label,
        threshold_relative: fixture.diagnostic.threshold_relative,
        selection_buckets: fixture.diagnostic.selection_buckets.clone(),
        datasets: fixture.diagnostic.datasets.clone(),
        input_facet_count: fixture.diagnostic.input_facet_count,
        input_sys: fixture.diagnostic.input_sys,
        near_active_count: fixture.diagnostic.near_active_count.unwrap_or(0),
    }
}

fn flatten_gradient(gradient: &[Vector4<f64>]) -> Vec<f64> {
    gradient
        .iter()
        .flat_map(|v| [v[0], v[1], v[2], v[3]])
        .collect()
}

fn unflatten_direction(flat: &[f64]) -> Vec<Vector4<f64>> {
    flat.chunks_exact(4)
        .map(|chunk| Vector4::new(chunk[0], chunk[1], chunk[2], chunk[3]))
        .collect()
}

fn normalize_direction(direction: &[Vector4<f64>]) -> Option<Vec<Vector4<f64>>> {
    let norm = direction
        .iter()
        .flat_map(|v| [v[0], v[1], v[2], v[3]])
        .map(|x| x * x)
        .sum::<f64>()
        .sqrt();
    (norm > 0.0 && norm.is_finite()).then(|| direction.iter().map(|v| v / norm).collect())
}

fn count_fixture_degeneracy(fixtures: &[Fixture]) -> BTreeMap<String, usize> {
    let mut counts = BTreeMap::new();
    for fixture in fixtures {
        *counts
            .entry(fixture.diagnostic.degeneracy_label.clone())
            .or_insert(0) += 1;
    }
    counts
}

fn count_probe_statuses(rows: &[LocalGeometryProbeRow]) -> BTreeMap<String, usize> {
    let mut counts = BTreeMap::new();
    for row in rows {
        *counts.entry(row.status.clone()).or_insert(0) += 1;
    }
    counts
}

fn count_trace_stop_reasons(rows: &[RunTraceRow]) -> BTreeMap<String, usize> {
    let mut counts = BTreeMap::new();
    for row in rows {
        *counts.entry(row.stop_reason.clone()).or_insert(0) += 1;
    }
    counts
}

fn count_trace_line_search_statuses(rows: &[RunTraceRow]) -> BTreeMap<String, usize> {
    let mut counts = BTreeMap::new();
    for row in rows {
        *counts.entry(row.line_search_status.clone()).or_insert(0) += 1;
    }
    counts
}

fn count_endpoint_statuses(rows: &[EndpointDiagnosticRow]) -> BTreeMap<String, usize> {
    let mut counts = BTreeMap::new();
    for row in rows {
        *counts.entry(row.diagnostic_status.clone()).or_insert(0) += 1;
    }
    counts
}

fn count_endpoint_line_search_statuses(rows: &[EndpointDiagnosticRow]) -> BTreeMap<String, usize> {
    let mut counts = BTreeMap::new();
    for row in rows {
        let status = row
            .post_stop_line_search_status
            .clone()
            .unwrap_or_else(|| "not_run".to_string());
        *counts.entry(status).or_insert(0) += 1;
    }
    counts
}

fn count_probe_threshold_outcomes(
    rows: &[LocalGeometryProbeRow],
    stop_threshold: StopThreshold,
) -> BTreeMap<String, usize> {
    let mut counts = BTreeMap::new();
    for row in rows {
        let outcome = if row.status.as_str() != "ok" {
            "failed"
        } else {
            match row.observed_delta_sys {
                Some(delta) if delta > stop_threshold.effective_delta(row.base_sys) => {
                    "above_threshold"
                }
                Some(delta) if delta > 0.0 => "positive_below_threshold",
                Some(_) => "nonpositive",
                None => "missing_observed_delta",
            }
        };
        *counts.entry(outcome.to_string()).or_insert(0) += 1;
    }
    counts
}

fn parse_args() -> Cli {
    let mut cli = Cli {
        diagnostic_dir: PathBuf::new(),
        polytope_table: default_tables_dir().join("polytope-table.jsonl"),
        out_dir: default_output_dir(),
        selection_threshold_relative: DEFAULT_SELECTION_THRESHOLD_RELATIVE,
        action_window_relative: DEFAULT_ACTION_WINDOW_RELATIVE,
        direction_model: DirectionModel::NearActive,
        include_candidate_window_directions: false,
        write_step_ranking_audit: false,
        steps: DEFAULT_STEPS.to_vec(),
        endpoint_steps: None,
        max_fixtures_per_label: DEFAULT_MAX_FIXTURES_PER_LABEL,
        skip_fixtures_per_label: 0,
        trace_iterations: DEFAULT_TRACE_ITERATIONS,
        degeneracy_labels: vec![
            "large_gap".to_string(),
            "narrow_gap".to_string(),
            "high_degeneracy".to_string(),
        ],
        min_observed_delta: DEFAULT_MIN_OBSERVED_DELTA,
        min_observed_relative_delta: DEFAULT_MIN_OBSERVED_RELATIVE_DELTA,
    };

    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--diagnostic-dir" => {
                cli.diagnostic_dir =
                    PathBuf::from(args.next().expect("--diagnostic-dir requires a path"));
            }
            "--polytope-table" => {
                cli.polytope_table =
                    PathBuf::from(args.next().expect("--polytope-table requires a path"));
            }
            "--out-dir" => {
                cli.out_dir = PathBuf::from(args.next().expect("--out-dir requires a path"));
            }
            "--selection-threshold-relative" => {
                cli.selection_threshold_relative = args
                    .next()
                    .expect("--selection-threshold-relative requires an f64")
                    .parse()
                    .expect("--selection-threshold-relative must be an f64");
            }
            "--action-window-relative" => {
                cli.action_window_relative = args
                    .next()
                    .expect("--action-window-relative requires an f64")
                    .parse()
                    .expect("--action-window-relative must be an f64");
            }
            "--direction-model" => {
                cli.direction_model = parse_direction_model(
                    &args
                        .next()
                        .expect("--direction-model requires near-active or candidate-window"),
                );
            }
            "--include-candidate-window-directions" => {
                cli.include_candidate_window_directions = true;
            }
            "--write-step-ranking-audit" => {
                cli.write_step_ranking_audit = true;
            }
            "--steps" => {
                cli.steps = args
                    .next()
                    .expect("--steps requires comma-separated f64 values")
                    .split(',')
                    .map(|value| value.parse().expect("--steps entries must be f64"))
                    .collect();
            }
            "--endpoint-steps" => {
                cli.endpoint_steps = Some(
                    args.next()
                        .expect("--endpoint-steps requires comma-separated f64 values")
                        .split(',')
                        .map(|value| value.parse().expect("--endpoint-steps entries must be f64"))
                        .collect(),
                );
            }
            "--max-fixtures-per-label" => {
                cli.max_fixtures_per_label = args
                    .next()
                    .expect("--max-fixtures-per-label requires an integer")
                    .parse()
                    .expect("--max-fixtures-per-label must be an integer");
            }
            "--skip-fixtures-per-label" => {
                cli.skip_fixtures_per_label = args
                    .next()
                    .expect("--skip-fixtures-per-label requires an integer")
                    .parse()
                    .expect("--skip-fixtures-per-label must be an integer");
            }
            "--trace-iterations" => {
                cli.trace_iterations = args
                    .next()
                    .expect("--trace-iterations requires an integer")
                    .parse()
                    .expect("--trace-iterations must be an integer");
            }
            "--degeneracy-labels" => {
                cli.degeneracy_labels = args
                    .next()
                    .expect("--degeneracy-labels requires comma-separated labels")
                    .split(',')
                    .filter(|value| !value.is_empty())
                    .map(ToString::to_string)
                    .collect();
            }
            "--min-observed-delta" => {
                cli.min_observed_delta = args
                    .next()
                    .expect("--min-observed-delta requires an f64")
                    .parse()
                    .expect("--min-observed-delta must be an f64");
            }
            "--min-observed-relative-delta" => {
                cli.min_observed_relative_delta = args
                    .next()
                    .expect("--min-observed-relative-delta requires an f64")
                    .parse()
                    .expect("--min-observed-relative-delta must be an f64");
            }
            "--help" | "-h" => {
                print_usage();
                std::process::exit(0);
            }
            other => panic!("unsupported argument: {other}"),
        }
    }

    if cli.diagnostic_dir.as_os_str().is_empty() {
        print_usage();
        panic!("--diagnostic-dir is required");
    }
    cli
}

fn print_usage() {
    eprintln!(
        "Usage: dev-sys-prediction-cloud --diagnostic-dir PATH \
         [--polytope-table PATH] [--out-dir PATH] \
         [--selection-threshold-relative F64] [--action-window-relative F64] \
         [--direction-model near-active|candidate-window] \
         [--include-candidate-window-directions] \
         [--write-step-ranking-audit] \
         [--steps CSV] [--endpoint-steps CSV] \
         [--max-fixtures-per-label N] [--skip-fixtures-per-label N] \
         [--trace-iterations N] \
         [--degeneracy-labels CSV] [--min-observed-delta F64] \
         [--min-observed-relative-delta F64]"
    );
}

fn parse_direction_model(raw: &str) -> DirectionModel {
    match raw {
        "near-active" | "near_active" => DirectionModel::NearActive,
        "candidate-window" | "candidate_window" => DirectionModel::CandidateWindow,
        other => panic!("unsupported --direction-model value: {other}"),
    }
}

fn default_tables_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../sys-datascience/prepare")
}

fn default_output_dir() -> PathBuf {
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock before UNIX_EPOCH")
        .as_millis();
    std::env::temp_dir().join(format!(
        "dev-sys-prediction-cloud-{}-{stamp}",
        std::process::id()
    ))
}

fn load_jsonl<T: for<'de> Deserialize<'de>>(path: &Path) -> Vec<T> {
    let file =
        File::open(path).unwrap_or_else(|err| panic!("failed to open {}: {err}", path.display()));
    let reader = BufReader::new(file);
    reader
        .lines()
        .enumerate()
        .filter_map(|(idx, line)| {
            let line = line.unwrap_or_else(|err| {
                panic!("failed to read {}:{}: {err}", path.display(), idx + 1)
            });
            (!line.trim().is_empty()).then(|| {
                serde_json::from_str(&line).unwrap_or_else(|err| {
                    panic!("failed to parse {}:{}: {err}", path.display(), idx + 1)
                })
            })
        })
        .collect()
}

fn write_jsonl<P: AsRef<Path>, T: Serialize>(path: P, rows: &[T]) -> std::io::Result<()> {
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);
    for row in rows {
        serde_json::to_writer(&mut writer, row)?;
        writer.write_all(b"\n")?;
    }
    Ok(())
}

fn write_json<P: AsRef<Path>, T: Serialize>(path: P, value: &T) -> std::io::Result<()> {
    let file = File::create(path)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, value)?;
    Ok(())
}

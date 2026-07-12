//! Local geometry probe tied to the branch degeneracy diagnostic.
//!
//! This command consumes a `dev-gradient-ascent-branch-diagnostic` output
//! directory, selects representative classified basepoints, and evaluates
//! finite steps of `sys(a0 + t d)` along branch-derived directions.

use exp_sys_landscape::{
    compute_active_sys_state, compute_step_bound_detailed, compute_sys_from_capacity,
    dual_vertices_rational_strings, exact_volume_from_incidence_as_f64, poly_id_from_dual_vertices,
    EventType, SysLandscapePolytopeCache,
};
use good_lp::{constraint, default_solver, variable, variables, Expression, Solution, SolverModel};
use nalgebra::Vector4;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fs::{self, File};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
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
const DEFAULT_TRACE_ITERATIONS: usize = 2;
const DEFAULT_MIN_OBSERVED_DELTA: f64 = 0.0;
const DEFAULT_MIN_OBSERVED_RELATIVE_DELTA: f64 = 0.0;
const GEOMETRIC_STEP_FACTOR: f64 = 2.0;
const GEOMETRIC_MAX_EXPANSIONS: usize = 8;
const GEOMETRIC_MAX_BACKTRACKS: usize = 8;

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
    audit_iterations: Option<BTreeSet<usize>>,
    audit_step_policies: Vec<AuditStepPolicy>,
    audit_direction_limit: Option<usize>,
    audit_policy_proposal_limit: Option<usize>,
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum AuditStepPolicy {
    Fixed,
    Geometric,
    BoundaryScaled,
}

impl AuditStepPolicy {
    fn as_str(self) -> &'static str {
        match self {
            Self::Fixed => "fixed",
            Self::Geometric => "geometric",
            Self::BoundaryScaled => "boundary_scaled",
        }
    }
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
struct AuditedStateRow {
    state_id: String,
    source_poly_id: String,
    source_degeneracy_label: String,
    source_selection_rank_within_label: usize,
    iteration: usize,
    role: String,
    /// Immediate predecessor geometry hash. In audit mode every reached trace
    /// base is emitted, so a non-null predecessor resolves to another row in
    /// this file.
    predecessor_state_id: Option<String>,
    dual_vertices_f64: Vec<[f64; 4]>,
    dual_vertices_rational: Vec<[String; 4]>,
    base_sys: f64,
    min_action: f64,
    branch_threshold_relative: f64,
    action_window_relative: f64,
    action_window_absolute: f64,
    min_observed_delta: f64,
    min_observed_relative_delta: f64,
}

#[derive(Serialize)]
struct AuditStateStatusRow {
    poly_id: String,
    degeneracy_label: String,
    selection_rank_within_label: usize,
    requested_iteration: usize,
    status: String,
    state_id: Option<String>,
    trace_stop_reason: String,
}

#[derive(Serialize)]
struct RunProvenance {
    full_cli_args: Vec<String>,
    parameters: RunParameters,
    inputs: Vec<InputIdentity>,
    source: SourceIdentity,
}

#[derive(Serialize)]
struct RunParameters {
    selection_threshold_relative: f64,
    action_window_relative: f64,
    steps: Vec<f64>,
    endpoint_steps: Option<Vec<f64>>,
    trace_iterations: usize,
    audit_iterations: Option<Vec<usize>>,
    audit_step_policies: Vec<String>,
    audit_direction_limit: Option<usize>,
    audit_policy_proposal_limit: Option<usize>,
    min_observed_delta: f64,
    min_observed_relative_delta: f64,
}

#[derive(Serialize)]
struct InputIdentity {
    role: String,
    observed_path: String,
    portable_path: String,
    blake3: String,
}

#[derive(Serialize)]
struct SourceIdentity {
    repo_head: Option<String>,
    worktree_diff_blake3: String,
    source_file: String,
    source_file_blake3: String,
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
    move_key: String,
    audit_step_policy: String,
    policy_proposal_order: usize,
    policy_evaluation_order: usize,
    exact_evaluation_order: usize,
    exact_evaluation_reused: bool,
    boundary_t_max: Option<f64>,
    boundary_event: Option<String>,
    status: String,
    base_sys: f64,
    effective_min_observed_delta: f64,
    near_active_predicted_delta_sys: Option<f64>,
    candidate_window_predicted_delta_sys: Option<f64>,
    /// Index in the nominal-action-filtered analytic candidate list.
    candidate_window_witness_filtered_orbit_index: Option<usize>,
    candidate_window_witness_sigma: Option<Vec<usize>>,
    candidate_window_witness_admissibility: Option<OrbitAdmissibility>,
    candidate_window_witness_action: Option<f64>,
    candidate_window_witness_action_lower: Option<f64>,
    candidate_window_witness_action_upper: Option<f64>,
    candidate_window_witness_relative_action_gap: Option<f64>,
    candidate_window_witness_q: Option<f64>,
    candidate_window_witness_q_error_bound: Option<f64>,
    candidate_window_witness_beta_margin: Option<f64>,
    candidate_window_witness_base_gap: Option<f64>,
    candidate_window_witness_derivative: Option<f64>,
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
    run_provenance_path: String,
    run_provenance_blake3: String,
    diagnostic_dir: String,
    polytope_table: String,
    selection_threshold_relative: f64,
    direction_model: String,
    include_candidate_window_directions: bool,
    audit_iterations: Option<Vec<usize>>,
    audit_step_policies: Vec<String>,
    audit_direction_limit: Option<usize>,
    audit_policy_proposal_limit: Option<usize>,
    max_fixtures_per_label: usize,
    skip_fixtures_per_label: usize,
    degeneracy_labels: Vec<String>,
    selected_fixtures: usize,
    probe_rows: usize,
    run_trace_rows: usize,
    endpoint_diagnostic_rows: usize,
    endpoint_direction_scan_rows: usize,
    audited_state_rows: usize,
    selected_audited_state_rows: usize,
    audit_state_status_rows: usize,
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
    run_provenance_path: String,
    run_provenance_blake3: String,
    direction_model: String,
    include_candidate_window_directions: bool,
    audit_iterations: Option<Vec<usize>>,
    audit_step_policies: Vec<String>,
    audit_direction_limit: Option<usize>,
    audit_policy_proposal_limit: Option<usize>,
    selection_threshold_relative: f64,
    max_fixtures_per_label: usize,
    skip_fixtures_per_label: usize,
    degeneracy_labels: Vec<String>,
    selected_fixtures: usize,
    probe_rows: usize,
    run_trace_rows: usize,
    endpoint_diagnostic_rows: usize,
    endpoint_direction_scan_rows: usize,
    audited_state_rows: usize,
    selected_audited_state_rows: usize,
    audit_state_status_counts: BTreeMap<String, usize>,
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
    out_dir: String,
    caveat: String,
}

#[derive(Clone, Debug)]
struct BaseState {
    polytope: SysLandscapePolytopeCache,
    capacity: OrbitSearchResult,
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
    reset_owned_output_files(&cli.out_dir);
    let run_provenance = build_run_provenance(&cli);
    let run_provenance_blake3 = blake3::hash(
        &serde_json::to_vec(&run_provenance).expect("failed to serialize run provenance"),
    )
    .to_hex()
    .to_string();
    write_json(cli.out_dir.join("run-provenance.json"), &run_provenance)
        .expect("failed to write run-provenance.json");
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

    let fixture_rows: Vec<FixtureRow> = fixtures.iter().map(fixture_row).collect();
    let mut probe_rows = Vec::new();
    let mut base_orbit_iterations = 0u64;
    let mut target_orbit_iterations = 0u64;

    if cli.audit_iterations.is_none() {
        for fixture in &fixtures {
            match compute_base_state_from_row(
                &fixture.polytope,
                cli.action_window_relative,
                cli.selection_threshold_relative,
            ) {
                Ok(base) => {
                    base_orbit_iterations += base.capacity.iterations;
                    let directions = probe_directions(
                        &base,
                        &cli.steps,
                        cli.include_candidate_window_directions,
                    );
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
                        base_near_active_count: 0,
                        base_returned_orbit_count: 0,
                        base_orbit_iterations: 0,
                        target_orbit_iterations: None,
                    });
                }
            }
        }
    }

    write_jsonl(cli.out_dir.join("fixture-selection.jsonl"), &fixture_rows)
        .expect("failed to write fixture-selection.jsonl");
    if cli.audit_iterations.is_none() {
        write_jsonl(cli.out_dir.join("local-geometry-probe.jsonl"), &probe_rows)
            .expect("failed to write local-geometry-probe.jsonl");
    }
    let trace_artifacts = run_trace_and_endpoint_rows(
        &fixtures,
        &probe_rows,
        cli.selection_threshold_relative,
        cli.action_window_relative,
        cli.direction_model,
        cli.include_candidate_window_directions,
        cli.write_step_ranking_audit,
        &cli.steps,
        cli.endpoint_steps.as_deref().unwrap_or(&cli.steps),
        cli.trace_iterations,
        cli.audit_iterations.as_ref(),
        &cli.audit_step_policies,
        cli.audit_direction_limit,
        cli.audit_policy_proposal_limit,
        StopThreshold {
            absolute_delta: cli.min_observed_delta,
            relative_delta: cli.min_observed_relative_delta,
        },
    );
    let trace_rows = trace_artifacts.trace_rows;
    let endpoint_rows = trace_artifacts.endpoint_rows;
    let endpoint_direction_scan_rows = trace_artifacts.endpoint_direction_scan_rows;
    let step_ranking_audit_rows = trace_artifacts.step_ranking_audit_rows;
    let audited_state_rows = trace_artifacts.audited_state_rows;
    let audit_state_status_rows = trace_artifacts.audit_state_status_rows;
    write_jsonl(cli.out_dir.join("run-trace.jsonl"), &trace_rows)
        .expect("failed to write run-trace.jsonl");
    if cli.audit_iterations.is_none() {
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
    }
    if cli.write_step_ranking_audit || cli.audit_iterations.is_some() {
        write_jsonl(
            cli.out_dir.join("step-ranking-audit.jsonl"),
            &step_ranking_audit_rows,
        )
        .expect("failed to write step-ranking-audit.jsonl");
    }
    if cli.audit_iterations.is_some() {
        write_jsonl(cli.out_dir.join("states.jsonl"), &audited_state_rows)
            .expect("failed to write states.jsonl");
        write_jsonl(
            cli.out_dir.join("audit-state-status.jsonl"),
            &audit_state_status_rows,
        )
        .expect("failed to write audit-state-status.jsonl");
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
        command: "dev-gradient-ascent-local-geometry-probe".to_string(),
        run_provenance_path: "run-provenance.json".to_string(),
        run_provenance_blake3: run_provenance_blake3.clone(),
        diagnostic_dir: cli.diagnostic_dir.display().to_string(),
        polytope_table: cli.polytope_table.display().to_string(),
        selection_threshold_relative: cli.selection_threshold_relative,
        direction_model: cli.direction_model.as_str().to_string(),
        include_candidate_window_directions: cli.include_candidate_window_directions,
        audit_iterations: cli
            .audit_iterations
            .as_ref()
            .map(|iterations| iterations.iter().copied().collect()),
        audit_step_policies: cli
            .audit_step_policies
            .iter()
            .map(|policy| policy.as_str().to_string())
            .collect(),
        audit_direction_limit: cli.audit_direction_limit,
        audit_policy_proposal_limit: cli.audit_policy_proposal_limit,
        max_fixtures_per_label: cli.max_fixtures_per_label,
        skip_fixtures_per_label: cli.skip_fixtures_per_label,
        degeneracy_labels: cli.degeneracy_labels.clone(),
        selected_fixtures: fixtures.len(),
        probe_rows: probe_rows.len(),
        run_trace_rows: trace_rows.len(),
        endpoint_diagnostic_rows: endpoint_rows.len(),
        endpoint_direction_scan_rows: endpoint_direction_scan_rows.len(),
        audited_state_rows: audited_state_rows.len(),
        selected_audited_state_rows: audited_state_rows
            .iter()
            .filter(|row| row.role == "selected_audit_state")
            .count(),
        audit_state_status_rows: audit_state_status_rows.len(),
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
        method: "dev-gradient-ascent-local-geometry-probe".to_string(),
        run_provenance_path: "run-provenance.json".to_string(),
        run_provenance_blake3,
        direction_model: cli.direction_model.as_str().to_string(),
        include_candidate_window_directions: cli.include_candidate_window_directions,
        audit_iterations: cli
            .audit_iterations
            .as_ref()
            .map(|iterations| iterations.iter().copied().collect()),
        audit_step_policies: cli
            .audit_step_policies
            .iter()
            .map(|policy| policy.as_str().to_string())
            .collect(),
        audit_direction_limit: cli.audit_direction_limit,
        audit_policy_proposal_limit: cli.audit_policy_proposal_limit,
        selection_threshold_relative: cli.selection_threshold_relative,
        max_fixtures_per_label: cli.max_fixtures_per_label,
        skip_fixtures_per_label: cli.skip_fixtures_per_label,
        degeneracy_labels: cli.degeneracy_labels.clone(),
        selected_fixtures: fixtures.len(),
        probe_rows: probe_rows.len(),
        run_trace_rows: trace_rows.len(),
        endpoint_diagnostic_rows: endpoint_rows.len(),
        endpoint_direction_scan_rows: endpoint_direction_scan_rows.len(),
        audited_state_rows: audited_state_rows.len(),
        selected_audited_state_rows: audited_state_rows
            .iter()
            .filter(|row| row.role == "selected_audit_state")
            .count(),
        audit_state_status_counts: count_audit_state_statuses(&audit_state_status_rows),
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
        out_dir: cli.out_dir.display().to_string(),
        caveat: "finite local probe only; this does not certify endpoint local maximality"
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

struct TraceArtifacts {
    trace_rows: Vec<RunTraceRow>,
    endpoint_rows: Vec<EndpointDiagnosticRow>,
    endpoint_direction_scan_rows: Vec<LocalGeometryProbeRow>,
    step_ranking_audit_rows: Vec<StepRankingAuditRow>,
    audited_state_rows: Vec<AuditedStateRow>,
    audit_state_status_rows: Vec<AuditStateStatusRow>,
}

fn run_trace_and_endpoint_rows(
    fixtures: &[Fixture],
    probe_rows: &[LocalGeometryProbeRow],
    branch_threshold_relative: f64,
    action_window_relative: f64,
    direction_model: DirectionModel,
    include_candidate_window_directions: bool,
    write_step_ranking_audit: bool,
    steps: &[f64],
    endpoint_steps: &[f64],
    trace_iterations: usize,
    audit_iterations: Option<&BTreeSet<usize>>,
    audit_step_policies: &[AuditStepPolicy],
    audit_direction_limit: Option<usize>,
    audit_policy_proposal_limit: Option<usize>,
    stop_threshold: StopThreshold,
) -> TraceArtifacts {
    let mut rows = Vec::new();
    let mut endpoint_rows = Vec::new();
    let mut endpoint_direction_scan_rows = Vec::new();
    let mut step_ranking_audit_rows = Vec::new();
    let mut audited_state_rows = Vec::new();
    let mut audit_state_status_rows = Vec::new();
    let iteration_limit = trace_iteration_limit(trace_iterations, audit_iterations);
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
                if audit_iterations.is_none() {
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
                } else if let Some(iterations) = audit_iterations {
                    audit_state_status_rows.extend(audit_state_status_rows_for_fixture(
                        fixture,
                        iterations,
                        &BTreeMap::new(),
                        "initial_polytope_failed",
                    ));
                }
                continue;
            }
        };
        let mut trace_stop_reason = "trace_iteration_limit".to_string();
        let mut current_state_id = state_id_for_polytope(&current);
        let mut predecessor_state_id = None;
        let mut reached_audit_state_ids = BTreeMap::new();

        for iteration in 0..iteration_limit {
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
            let audit_this_iteration =
                audit_iterations.is_some_and(|iterations| iterations.contains(&iteration));
            if audit_iterations.is_some() {
                let role = if audit_this_iteration {
                    "selected_audit_state"
                } else {
                    "trace_lineage_state"
                };
                audited_state_rows.push(audited_state_row(
                    fixture,
                    iteration,
                    &base,
                    current_state_id.clone(),
                    predecessor_state_id.clone(),
                    role,
                    branch_threshold_relative,
                    action_window_relative,
                    action_gap,
                    stop_threshold,
                ));
            }
            if audit_this_iteration {
                reached_audit_state_ids.insert(iteration, current_state_id.clone());
            }
            if write_step_ranking_audit || audit_this_iteration {
                step_ranking_audit_rows.extend(step_ranking_audit_rows_for_base(
                    fixture,
                    iteration,
                    &base,
                    steps,
                    audit_step_policies,
                    audit_direction_limit,
                    audit_policy_proposal_limit,
                    direction_model,
                    include_candidate_window_directions,
                    action_window_relative,
                    branch_threshold_relative,
                    stop_threshold,
                ));
            }
            if is_last_selected_audit(iteration, audit_iterations) {
                trace_stop_reason = "last_selected_audit_complete".to_string();
                break;
            }
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
                predecessor_state_id = Some(current_state_id);
                current = candidate.target_polytope;
                current_state_id = state_id_for_polytope(&current);
            } else {
                trace_stop_reason = "line_search_all_steps_below_min_observed_delta".to_string();
                break;
            }
        }

        if audit_iterations.is_none() {
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
        } else if let Some(iterations) = audit_iterations {
            audit_state_status_rows.extend(audit_state_status_rows_for_fixture(
                fixture,
                iterations,
                &reached_audit_state_ids,
                &trace_stop_reason,
            ));
        }
    }

    if audit_iterations.is_none() && trace_iterations == 1 {
        align_first_trace_rows_with_probe_rows(&mut rows, probe_rows);
    }
    TraceArtifacts {
        trace_rows: rows,
        endpoint_rows,
        endpoint_direction_scan_rows,
        step_ranking_audit_rows,
        audited_state_rows,
        audit_state_status_rows,
    }
}

fn trace_iteration_limit(
    ordinary_limit: usize,
    audit_iterations: Option<&BTreeSet<usize>>,
) -> usize {
    audit_iterations
        .and_then(|iterations| iterations.last().copied())
        .map_or(ordinary_limit, |last| last + 1)
}

fn is_last_selected_audit(iteration: usize, audit_iterations: Option<&BTreeSet<usize>>) -> bool {
    audit_iterations
        .and_then(|iterations| iterations.last())
        .is_some_and(|last| iteration == *last)
}

fn state_id_for_polytope(polytope: &SysLandscapePolytopeCache) -> String {
    format!(
        "state:{}",
        poly_id_from_dual_vertices(&polytope.dual_vertices_f64)
    )
}

#[allow(clippy::too_many_arguments)]
fn audited_state_row(
    fixture: &Fixture,
    iteration: usize,
    base: &BaseState,
    state_id: String,
    predecessor_state_id: Option<String>,
    role: &str,
    branch_threshold_relative: f64,
    action_window_relative: f64,
    action_window_absolute: f64,
    stop_threshold: StopThreshold,
) -> AuditedStateRow {
    AuditedStateRow {
        state_id,
        source_poly_id: fixture.polytope.poly_id.clone(),
        source_degeneracy_label: fixture.diagnostic.degeneracy_label.clone(),
        source_selection_rank_within_label: fixture.selection_rank_within_label,
        iteration,
        role: role.to_string(),
        predecessor_state_id,
        dual_vertices_f64: base
            .polytope
            .dual_vertices_f64
            .iter()
            .map(|dual| [dual[0], dual[1], dual[2], dual[3]])
            .collect(),
        dual_vertices_rational: dual_vertices_rational_strings(&base.polytope),
        base_sys: base.sys,
        min_action: base.capacity.min_action,
        branch_threshold_relative,
        action_window_relative,
        action_window_absolute,
        min_observed_delta: stop_threshold.absolute_delta,
        min_observed_relative_delta: stop_threshold.relative_delta,
    }
}

fn audit_state_status_rows_for_fixture(
    fixture: &Fixture,
    requested_iterations: &BTreeSet<usize>,
    reached_state_ids: &BTreeMap<usize, String>,
    trace_stop_reason: &str,
) -> Vec<AuditStateStatusRow> {
    requested_iterations
        .iter()
        .map(|&requested_iteration| {
            let state_id = reached_state_ids.get(&requested_iteration).cloned();
            AuditStateStatusRow {
                poly_id: fixture.polytope.poly_id.clone(),
                degeneracy_label: fixture.diagnostic.degeneracy_label.clone(),
                selection_rank_within_label: fixture.selection_rank_within_label,
                requested_iteration,
                status: if state_id.is_some() {
                    "selected".to_string()
                } else {
                    "unreached_trace_stopped".to_string()
                },
                state_id,
                trace_stop_reason: trace_stop_reason.to_string(),
            }
        })
        .collect()
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
    audit_step_policies: &[AuditStepPolicy],
    audit_direction_limit: Option<usize>,
    audit_policy_proposal_limit: Option<usize>,
    direction_model: DirectionModel,
    include_candidate_window_directions: bool,
    action_window_relative: f64,
    branch_threshold_relative: f64,
    stop_threshold: StopThreshold,
) -> Vec<StepRankingAuditRow> {
    let effective_min_observed_delta = stop_threshold.effective_delta(base.sys);
    let mut rows = Vec::new();
    let directions = audit_ordered_directions(
        base,
        steps,
        direction_model,
        include_candidate_window_directions,
    );
    let directions = directions
        .into_iter()
        .take(audit_direction_limit.unwrap_or(usize::MAX));
    let policies = if audit_step_policies.is_empty() {
        &[AuditStepPolicy::Fixed][..]
    } else {
        audit_step_policies
    };
    let mut exact_cache: HashMap<MoveCacheKey, CachedExactStep> = HashMap::new();
    let mut next_exact_evaluation_order = 1usize;

    for &policy in policies {
        let mut policy_order = 0usize;
        for direction in directions.clone() {
            if audit_policy_proposal_limit.is_some_and(|limit| policy_order >= limit) {
                break;
            }
            let boundary = compute_step_bound_detailed(&base.polytope, &direction.vector);
            match policy {
                AuditStepPolicy::Fixed => {
                    for &step in steps {
                        if audit_policy_proposal_limit.is_some_and(|limit| policy_order >= limit) {
                            break;
                        }
                        if direction.allows_step(step) {
                            push_audit_step(
                                &mut rows,
                                &mut exact_cache,
                                &mut next_exact_evaluation_order,
                                &mut policy_order,
                                fixture,
                                iteration,
                                base,
                                &direction,
                                step,
                                policy,
                                &boundary,
                                action_window_relative,
                                branch_threshold_relative,
                                effective_min_observed_delta,
                            );
                        }
                    }
                }
                AuditStepPolicy::BoundaryScaled => {
                    if boundary.t_max.is_finite() && boundary.t_max > 0.0 {
                        for factor in [0.1, 0.25, 0.5, 0.75, 0.95, 1.5, 2.0, 3.0] {
                            if audit_policy_proposal_limit
                                .is_some_and(|limit| policy_order >= limit)
                            {
                                break;
                            }
                            let step = factor * boundary.t_max;
                            if step.is_finite() && direction.allows_step(step) {
                                push_audit_step(
                                    &mut rows,
                                    &mut exact_cache,
                                    &mut next_exact_evaluation_order,
                                    &mut policy_order,
                                    fixture,
                                    iteration,
                                    base,
                                    &direction,
                                    step,
                                    policy,
                                    &boundary,
                                    action_window_relative,
                                    branch_threshold_relative,
                                    effective_min_observed_delta,
                                );
                            }
                        }
                    }
                }
                AuditStepPolicy::Geometric => {
                    if audit_policy_proposal_limit.is_some_and(|limit| policy_order >= limit) {
                        continue;
                    }
                    let Some(&initial_step) = steps.first() else {
                        continue;
                    };
                    if !direction.allows_step(initial_step) {
                        continue;
                    }
                    let initial_delta = push_audit_step(
                        &mut rows,
                        &mut exact_cache,
                        &mut next_exact_evaluation_order,
                        &mut policy_order,
                        fixture,
                        iteration,
                        base,
                        &direction,
                        initial_step,
                        policy,
                        &boundary,
                        action_window_relative,
                        branch_threshold_relative,
                        effective_min_observed_delta,
                    );
                    if initial_delta.is_some_and(|delta| delta > 0.0) {
                        let mut previous_delta = initial_delta.unwrap();
                        let mut step = initial_step;
                        for _ in 0..GEOMETRIC_MAX_EXPANSIONS {
                            if audit_policy_proposal_limit
                                .is_some_and(|limit| policy_order >= limit)
                            {
                                break;
                            }
                            step *= GEOMETRIC_STEP_FACTOR;
                            if !step.is_finite() || !direction.allows_step(step) {
                                break;
                            }
                            let delta = push_audit_step(
                                &mut rows,
                                &mut exact_cache,
                                &mut next_exact_evaluation_order,
                                &mut policy_order,
                                fixture,
                                iteration,
                                base,
                                &direction,
                                step,
                                policy,
                                &boundary,
                                action_window_relative,
                                branch_threshold_relative,
                                effective_min_observed_delta,
                            );
                            let Some(delta) = delta else { break };
                            if delta <= 0.0 || delta <= previous_delta {
                                break;
                            }
                            previous_delta = delta;
                        }
                    } else {
                        let mut step = initial_step;
                        for _ in 0..GEOMETRIC_MAX_BACKTRACKS {
                            if audit_policy_proposal_limit
                                .is_some_and(|limit| policy_order >= limit)
                            {
                                break;
                            }
                            step /= GEOMETRIC_STEP_FACTOR;
                            if step <= 0.0 || !direction.allows_step(step) {
                                break;
                            }
                            let delta = push_audit_step(
                                &mut rows,
                                &mut exact_cache,
                                &mut next_exact_evaluation_order,
                                &mut policy_order,
                                fixture,
                                iteration,
                                base,
                                &direction,
                                step,
                                policy,
                                &boundary,
                                action_window_relative,
                                branch_threshold_relative,
                                effective_min_observed_delta,
                            );
                            if delta.is_some_and(|delta| delta > 0.0) {
                                break;
                            }
                        }
                    }
                }
            }
        }
    }
    assign_descending_ranks(&mut rows);
    rows
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct MoveCacheKey {
    step_bits: u64,
    direction_bits: Vec<u64>,
}

#[derive(Clone)]
struct ExactStepEvaluation {
    status: String,
    observed_delta_sys: Option<f64>,
    target_sys: Option<f64>,
    target_orbit_iterations: Option<u64>,
}

#[derive(Clone)]
struct CachedExactStep {
    exact_evaluation_order: usize,
    evaluation: ExactStepEvaluation,
}

fn audit_ordered_directions(
    base: &BaseState,
    steps: &[f64],
    direction_model: DirectionModel,
    include_candidate_window_directions: bool,
) -> Vec<ProbeDirection> {
    let ordering_step = steps.first().copied().unwrap_or(0.0);
    let mut indexed: Vec<_> = probe_directions(base, steps, include_candidate_window_directions)
        .into_iter()
        .enumerate()
        .collect();
    indexed.sort_by(|(left_index, left), (right_index, right)| {
        let left_score =
            branch_model_predicted_delta(base, &left.vector, ordering_step, direction_model)
                .filter(|score| score.is_finite());
        let right_score =
            branch_model_predicted_delta(base, &right.vector, ordering_step, direction_model)
                .filter(|score| score.is_finite());
        right_score
            .partial_cmp(&left_score)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| left_index.cmp(right_index))
    });
    indexed
        .into_iter()
        .map(|(_, direction)| direction)
        .collect()
}

#[allow(clippy::too_many_arguments)]
fn push_audit_step(
    rows: &mut Vec<StepRankingAuditRow>,
    exact_cache: &mut HashMap<MoveCacheKey, CachedExactStep>,
    next_exact_evaluation_order: &mut usize,
    policy_order: &mut usize,
    fixture: &Fixture,
    iteration: usize,
    base: &BaseState,
    direction: &ProbeDirection,
    step: f64,
    policy: AuditStepPolicy,
    boundary: &exp_sys_landscape::BoundaryEvent,
    action_window_relative: f64,
    branch_threshold_relative: f64,
    effective_min_observed_delta: f64,
) -> Option<f64> {
    *policy_order += 1;
    let key = move_cache_key(direction, step);
    let (cached, reused) = cached_exact_step(exact_cache, next_exact_evaluation_order, key, || {
        evaluate_exact_step(
            base,
            direction,
            step,
            action_window_relative,
            branch_threshold_relative,
        )
    });
    let observed_delta = cached.evaluation.observed_delta_sys;
    rows.push(step_ranking_audit_row(
        fixture,
        iteration,
        base,
        direction,
        step,
        policy,
        *policy_order,
        cached.exact_evaluation_order,
        reused,
        boundary,
        action_window_relative,
        effective_min_observed_delta,
        cached.evaluation,
    ));
    observed_delta
}

fn cached_exact_step<F>(
    exact_cache: &mut HashMap<MoveCacheKey, CachedExactStep>,
    next_exact_evaluation_order: &mut usize,
    key: MoveCacheKey,
    evaluate: F,
) -> (CachedExactStep, bool)
where
    F: FnOnce() -> ExactStepEvaluation,
{
    match exact_cache.get(&key) {
        Some(cached) => (cached.clone(), true),
        None => {
            let cached = CachedExactStep {
                exact_evaluation_order: *next_exact_evaluation_order,
                evaluation: evaluate(),
            };
            *next_exact_evaluation_order += 1;
            exact_cache.insert(key, cached.clone());
            (cached, false)
        }
    }
}

fn move_cache_key(direction: &ProbeDirection, step: f64) -> MoveCacheKey {
    MoveCacheKey {
        step_bits: step.to_bits(),
        direction_bits: direction
            .vector
            .iter()
            .flat_map(|vector| vector.iter().map(|coordinate| coordinate.to_bits()))
            .collect(),
    }
}

fn move_key(direction: &ProbeDirection, step: f64) -> String {
    format!("{}:{:016x}", direction.label, step.to_bits())
}

fn boundary_event_label(event: &EventType) -> String {
    match event {
        EventType::IncidenceFlip {
            vertex_index,
            new_facet,
        } => format!("incidence_flip:vertex={vertex_index}:facet={new_facet}"),
        EventType::OmegaFlip { facet_i, facet_j } => {
            format!("omega_flip:facet_i={facet_i}:facet_j={facet_j}")
        }
        EventType::DualVertexDegen { facet } => {
            format!("dual_vertex_degeneracy:facet={facet}")
        }
        EventType::Unbounded => "unbounded".to_string(),
    }
}

fn step_ranking_audit_row(
    fixture: &Fixture,
    iteration: usize,
    base: &BaseState,
    direction: &ProbeDirection,
    step: f64,
    policy: AuditStepPolicy,
    policy_order: usize,
    exact_evaluation_order: usize,
    exact_evaluation_reused: bool,
    boundary: &exp_sys_landscape::BoundaryEvent,
    action_window_relative: f64,
    effective_min_observed_delta: f64,
    exact: ExactStepEvaluation,
) -> StepRankingAuditRow {
    let near_active_predicted_delta_sys =
        branch_model_predicted_delta(base, &direction.vector, step, DirectionModel::NearActive);
    let candidate_window_prediction =
        candidate_window_prediction_witness(base, &direction.vector, step);
    let candidate_window_predicted_delta_sys = candidate_window_prediction
        .as_ref()
        .map(|witness| witness.predicted_delta);
    let ExactStepEvaluation {
        status,
        observed_delta_sys,
        target_sys,
        target_orbit_iterations,
    } = exact;

    StepRankingAuditRow {
        poly_id: fixture.polytope.poly_id.clone(),
        degeneracy_label: fixture.diagnostic.degeneracy_label.clone(),
        iteration,
        direction_label: direction.label.clone(),
        step,
        move_key: move_key(direction, step),
        audit_step_policy: policy.as_str().to_string(),
        policy_proposal_order: policy_order,
        policy_evaluation_order: policy_order,
        exact_evaluation_order,
        exact_evaluation_reused,
        boundary_t_max: boundary.t_max.is_finite().then_some(boundary.t_max),
        boundary_event: Some(boundary_event_label(&boundary.event)),
        status,
        base_sys: base.sys,
        effective_min_observed_delta,
        near_active_predicted_delta_sys,
        candidate_window_predicted_delta_sys,
        candidate_window_witness_filtered_orbit_index: candidate_window_prediction
            .as_ref()
            .map(|witness| witness.orbit_index),
        candidate_window_witness_sigma: candidate_window_prediction
            .as_ref()
            .map(|witness| witness.sigma.clone()),
        candidate_window_witness_admissibility: candidate_window_prediction
            .as_ref()
            .map(|witness| witness.admissibility),
        candidate_window_witness_action: candidate_window_prediction
            .as_ref()
            .map(|witness| witness.action),
        candidate_window_witness_action_lower: candidate_window_prediction
            .as_ref()
            .map(|witness| witness.action_lower),
        candidate_window_witness_action_upper: candidate_window_prediction
            .as_ref()
            .map(|witness| witness.action_upper),
        candidate_window_witness_relative_action_gap: candidate_window_prediction
            .as_ref()
            .map(|witness| witness.relative_action_gap),
        candidate_window_witness_q: candidate_window_prediction
            .as_ref()
            .map(|witness| witness.q),
        candidate_window_witness_q_error_bound: candidate_window_prediction
            .as_ref()
            .map(|witness| witness.q_error_bound),
        candidate_window_witness_beta_margin: candidate_window_prediction
            .as_ref()
            .map(|witness| witness.beta_margin),
        candidate_window_witness_base_gap: candidate_window_prediction
            .as_ref()
            .map(|witness| witness.base_gap),
        candidate_window_witness_derivative: candidate_window_prediction
            .as_ref()
            .map(|witness| witness.derivative),
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

fn evaluate_exact_step(
    base: &BaseState,
    direction: &ProbeDirection,
    step: f64,
    action_window_relative: f64,
    _branch_threshold_relative: f64,
) -> ExactStepEvaluation {
    let target_duals: Vec<Vector4<f64>> = base
        .polytope
        .dual_vertices_f64
        .iter()
        .zip(&direction.vector)
        .map(|(dual, delta)| dual + step * delta)
        .collect();
    let Some(target_polytope) = SysLandscapePolytopeCache::from_f64_dual_vertices(target_duals)
    else {
        return ExactStepEvaluation {
            status: "target_polytope_construction_failed".to_string(),
            observed_delta_sys: None,
            target_sys: None,
            target_orbit_iterations: None,
        };
    };
    let target_capacity = match capacity_auto_with_gap(
        &target_polytope,
        base.capacity.min_action * action_window_relative,
    ) {
        Ok(capacity) => capacity,
        Err(err) => {
            return ExactStepEvaluation {
                status: format!("target_capacity_failed:{err:?}"),
                observed_delta_sys: None,
                target_sys: None,
                target_orbit_iterations: None,
            };
        }
    };
    let target_orbit_iterations = Some(target_capacity.iterations);
    let Some(target_sys) = compute_sys_from_capacity(&target_polytope, &target_capacity) else {
        return ExactStepEvaluation {
            status: "target_sys_failed".to_string(),
            observed_delta_sys: None,
            target_sys: None,
            target_orbit_iterations,
        };
    };
    ExactStepEvaluation {
        status: "ok".to_string(),
        observed_delta_sys: Some(target_sys - base.sys),
        target_sys: Some(target_sys),
        target_orbit_iterations,
    }
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

        for &step in steps {
            if !direction.allows_step(step) {
                continue;
            }
            let Some(predicted_delta) =
                branch_model_predicted_delta(base, &direction.vector, step, direction_model)
            else {
                rejected_steps.push(step);
                continue;
            };
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
            let Some(target_sys) = compute_sys_from_capacity(&target_polytope, &target_capacity)
            else {
                rejected_steps.push(step);
                continue;
            };
            let target_near_active =
                near_active_orbits(&target_capacity, branch_threshold_relative);
            let observed_delta = target_sys - base.sys;
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
                    target_sys: Some(target_sys),
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
    // Aggregation retains candidates whose action interval intersects its gap.
    // The analytic model is defined by the explicitly requested window in the
    // producer's nominal action, so filter before differentiating.
    let candidate_orbits = candidate_window_orbits(&capacity, action_gap);
    let candidate_capacity_gradients =
        capacity_subgradients_a(&polytope.dual_vertices_f64, &candidate_orbits)
            .map_err(|err| format!("candidate_capacity_derivative_failed:{err:?}"))?;
    let candidate_sys_gradients: Vec<Vec<Vector4<f64>>> = candidate_capacity_gradients
        .iter()
        .zip(candidate_orbits.iter())
        .map(|(capacity_gradient, orbit)| {
            systolic_ratio_gradient_a(orbit.action, vol, capacity_gradient, &d_volume_da)
        })
        .collect();
    debug_assert_eq!(candidate_orbits.len(), candidate_sys_gradients.len());

    Ok(BaseState {
        polytope,
        candidate_orbits,
        capacity,
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
    admissibility: OrbitAdmissibility,
    action: f64,
    action_lower: f64,
    action_upper: f64,
    relative_action_gap: f64,
    q: f64,
    q_error_bound: f64,
    beta_margin: f64,
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
                    admissibility: orbit.admissibility,
                    action: orbit.action,
                    action_lower: orbit.action_lower,
                    action_upper: orbit.action_upper,
                    relative_action_gap: action_ratio - 1.0,
                    q: orbit.q,
                    q_error_bound: orbit.q_error_bound,
                    beta_margin: orbit.beta_margin,
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
        if let Some(direction) = maximin_direction(&base.sys_gradients) {
            directions.push(ProbeDirection {
                label: "near_active_maximin_direction".to_string(),
                vector: direction,
                only_step: None,
            });
        }
    }
    if include_candidate_window_directions {
        for &step in steps {
            if let Some(direction) = candidate_window_maximin_direction(base, step) {
                directions.push(ProbeDirection {
                    label: format!("candidate_window_maximin_step_{}", format_step_label(step)),
                    vector: direction,
                    only_step: Some(step),
                });
            }
        }
    }
    directions
}

fn candidate_window_maximin_direction(base: &BaseState, step: f64) -> Option<Vec<Vector4<f64>>> {
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

fn maximin_direction(sys_gradients: &[Vec<Vector4<f64>>]) -> Option<Vec<Vector4<f64>>> {
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

fn candidate_window_orbits(result: &OrbitSearchResult, action_gap: f64) -> Vec<OrbitKktData> {
    let cutoff = result.min_action + action_gap.max(0.0);
    // This only absorbs ordinary f64 rounding at the explicitly requested
    // boundary; it is deliberately much smaller than experiment action gaps.
    let tolerance = 64.0 * f64::EPSILON * cutoff.abs().max(1.0);
    result
        .orbits
        .iter()
        .filter(|orbit| {
            matches!(
                orbit.admissibility,
                OrbitAdmissibility::AdmissibleF64 | OrbitAdmissibility::AdmissibleExact
            )
        })
        .filter(|orbit| orbit.action.is_finite() && orbit.action <= cutoff + tolerance)
        .cloned()
        .collect()
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

fn count_audit_state_statuses(rows: &[AuditStateStatusRow]) -> BTreeMap<String, usize> {
    let mut counts = BTreeMap::new();
    for row in rows {
        *counts.entry(row.status.clone()).or_insert(0) += 1;
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
        audit_iterations: None,
        audit_step_policies: vec![AuditStepPolicy::Fixed],
        audit_direction_limit: None,
        audit_policy_proposal_limit: None,
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
            "--audit-iterations" => {
                cli.audit_iterations =
                    Some(parse_audit_iterations(&args.next().expect(
                        "--audit-iterations requires comma-separated integers",
                    )));
            }
            "--audit-step-policies" => {
                cli.audit_step_policies = parse_audit_step_policies(
                    &args
                        .next()
                        .expect("--audit-step-policies requires comma-separated policies"),
                );
            }
            "--audit-direction-limit" => {
                let limit: usize = args
                    .next()
                    .expect("--audit-direction-limit requires a positive integer")
                    .parse()
                    .expect("--audit-direction-limit must be a positive integer");
                assert!(limit > 0, "--audit-direction-limit must be positive");
                cli.audit_direction_limit = Some(limit);
            }
            "--audit-policy-proposal-limit" => {
                let limit: usize = args
                    .next()
                    .expect("--audit-policy-proposal-limit requires a positive integer")
                    .parse()
                    .expect("--audit-policy-proposal-limit must be a positive integer");
                assert!(limit > 0, "--audit-policy-proposal-limit must be positive");
                cli.audit_policy_proposal_limit = Some(limit);
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
        "Usage: dev-gradient-ascent-local-geometry-probe --diagnostic-dir PATH \
         [--polytope-table PATH] [--out-dir PATH] \
         [--selection-threshold-relative F64] [--action-window-relative F64] \
         [--direction-model near-active|candidate-window] \
         [--include-candidate-window-directions] \
         [--write-step-ranking-audit] \
         [--audit-iterations CSV] \
         [--audit-step-policies fixed,geometric,boundary-scaled] \
         [--audit-direction-limit N] \
         [--audit-policy-proposal-limit N] \
         [--steps CSV] [--endpoint-steps CSV] \
         [--max-fixtures-per-label N] [--skip-fixtures-per-label N] \
         [--trace-iterations N] \
         [--degeneracy-labels CSV] [--min-observed-delta F64] \
         [--min-observed-relative-delta F64]"
    );
}

fn parse_audit_iterations(raw: &str) -> BTreeSet<usize> {
    let iterations: BTreeSet<usize> = raw
        .split(',')
        .filter(|value| !value.is_empty())
        .map(|value| {
            value
                .parse()
                .expect("--audit-iterations entries must be nonnegative integers")
        })
        .collect();
    assert!(
        !iterations.is_empty(),
        "--audit-iterations requires at least one iteration"
    );
    iterations
}

fn parse_audit_step_policies(raw: &str) -> Vec<AuditStepPolicy> {
    let mut policies = Vec::new();
    for value in raw.split(',').filter(|value| !value.is_empty()) {
        let policy = match value {
            "fixed" => AuditStepPolicy::Fixed,
            "geometric" => AuditStepPolicy::Geometric,
            "boundary-scaled" | "boundary_scaled" => AuditStepPolicy::BoundaryScaled,
            other => panic!("unsupported --audit-step-policies value: {other}"),
        };
        if !policies.contains(&policy) {
            policies.push(policy);
        }
    }
    assert!(
        !policies.is_empty(),
        "--audit-step-policies requires at least one policy"
    );
    policies
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
        "dev-gradient-ascent-local-geometry-probe-{}-{stamp}",
        std::process::id()
    ))
}

fn build_run_provenance(cli: &Cli) -> RunProvenance {
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("experiment crate must be nested under the repository root")
        .to_path_buf();
    let diagnostic_input = cli.diagnostic_dir.join("branch-set-diagnostic.jsonl");
    RunProvenance {
        full_cli_args: std::env::args().collect(),
        parameters: RunParameters {
            selection_threshold_relative: cli.selection_threshold_relative,
            action_window_relative: cli.action_window_relative,
            steps: cli.steps.clone(),
            endpoint_steps: cli.endpoint_steps.clone(),
            trace_iterations: cli.trace_iterations,
            audit_iterations: cli
                .audit_iterations
                .as_ref()
                .map(|iterations| iterations.iter().copied().collect()),
            audit_step_policies: cli
                .audit_step_policies
                .iter()
                .map(|policy| policy.as_str().to_string())
                .collect(),
            audit_direction_limit: cli.audit_direction_limit,
            audit_policy_proposal_limit: cli.audit_policy_proposal_limit,
            min_observed_delta: cli.min_observed_delta,
            min_observed_relative_delta: cli.min_observed_relative_delta,
        },
        inputs: vec![
            input_identity("branch_set_diagnostic", &diagnostic_input, &repo_root),
            input_identity("polytope_table", &cli.polytope_table, &repo_root),
        ],
        source: source_identity(&repo_root),
    }
}

fn input_identity(role: &str, path: &Path, repo_root: &Path) -> InputIdentity {
    let bytes = fs::read(path)
        .unwrap_or_else(|error| panic!("failed to hash input {}: {error}", path.display()));
    InputIdentity {
        role: role.to_string(),
        observed_path: path.display().to_string(),
        portable_path: portable_path(path, repo_root),
        blake3: blake3::hash(&bytes).to_hex().to_string(),
    }
}

fn portable_path(path: &Path, repo_root: &Path) -> String {
    match path.strip_prefix(repo_root) {
        Ok(relative) => format!("repo:{}", relative.display()),
        Err(_) => format!(
            "external-blake3-input:{}",
            path.file_name().unwrap_or_default().to_string_lossy()
        ),
    }
}

fn source_identity(repo_root: &Path) -> SourceIdentity {
    let source_file =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("local-geometry-probe/main.rs");
    let source_bytes = fs::read(&source_file)
        .unwrap_or_else(|error| panic!("failed to hash source {}: {error}", source_file.display()));
    SourceIdentity {
        repo_head: git_stdout(repo_root, ["rev-parse", "HEAD"]),
        worktree_diff_blake3: blake3::hash(&git_stdout_bytes(
            repo_root,
            ["diff", "--binary", "--no-ext-diff"],
        ))
        .to_hex()
        .to_string(),
        source_file: portable_path(&source_file, repo_root),
        source_file_blake3: blake3::hash(&source_bytes).to_hex().to_string(),
    }
}

fn git_stdout<const N: usize>(repo_root: &Path, args: [&str; N]) -> Option<String> {
    let output = Command::new("git")
        .current_dir(repo_root)
        .args(args)
        .output()
        .ok()?;
    output
        .status
        .success()
        .then(|| String::from_utf8_lossy(&output.stdout).trim().to_string())
}

fn git_stdout_bytes<const N: usize>(repo_root: &Path, args: [&str; N]) -> Vec<u8> {
    let output = Command::new("git")
        .current_dir(repo_root)
        .args(args)
        .output()
        .expect("failed to identify the worktree diff");
    assert!(
        output.status.success(),
        "failed to identify the worktree diff"
    );
    output.stdout
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

/// Remove only files owned by this command so a reused output directory cannot
/// mix normal-mode and audit-mode artifacts from different runs.
fn reset_owned_output_files(out_dir: &Path) {
    for name in [
        "fixture-selection.jsonl",
        "local-geometry-probe.jsonl",
        "run-trace.jsonl",
        "endpoint-diagnostic.jsonl",
        "endpoint-direction-scan.jsonl",
        "step-ranking-audit.jsonl",
        "states.jsonl",
        "audit-state-status.jsonl",
        "run-provenance.json",
        "compute-budget-report.json",
        "summary.json",
    ] {
        let path = out_dir.join(name);
        if let Err(error) = fs::remove_file(&path) {
            assert_eq!(
                error.kind(),
                std::io::ErrorKind::NotFound,
                "failed to remove stale owned output {}: {error}",
                path.display()
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;

    fn test_fixture() -> Fixture {
        Fixture {
            diagnostic: DiagnosticRow {
                poly_id: "fixture-poly".to_string(),
                selection_buckets: Vec::new(),
                datasets: Vec::new(),
                input_facet_count: 0,
                input_sys: 0.0,
                threshold_relative: 0.001,
                near_active_count: None,
                degeneracy_label: "narrow_gap".to_string(),
                failure: None,
            },
            polytope: PolytopeRow {
                poly_id: "fixture-poly".to_string(),
                capacity: 1.0,
                sys: 1.0,
                dual_vertices_f64: Vec::new(),
            },
            selection_rank_within_label: 0,
        }
    }

    fn orbit(
        sigma: usize,
        action: f64,
        action_lower: f64,
        admissibility: OrbitAdmissibility,
    ) -> OrbitKktData {
        OrbitKktData {
            sigma: vec![sigma],
            beta: vec![1.0],
            beta_margin: 1.0,
            action,
            action_lower,
            action_upper: action,
            q: 1.0,
            q_error_bound: 0.0,
            mu: Some([0.0; 4]),
            xi: Some(0.0),
            admissibility,
        }
    }

    #[test]
    fn candidate_window_uses_nominal_action_and_admissibility() {
        let result = OrbitSearchResult {
            orbits: vec![
                orbit(0, 1.0, 1.0, OrbitAdmissibility::AdmissibleF64),
                orbit(1, 1.01, 1.01, OrbitAdmissibility::AdmissibleExact),
                // Its interval intersects the requested window, but its
                // nominal action is far outside it.
                orbit(2, 10.0, 1.005, OrbitAdmissibility::AdmissibleF64),
                orbit(3, 1.005, 1.005, OrbitAdmissibility::IndeterminateF64),
            ],
            min_action: 1.0,
            min_action_lower: 1.0,
            min_action_upper: 1.0,
            iterations: 4,
        };

        let retained = candidate_window_orbits(&result, 0.01);
        let retained_sigma: Vec<_> = retained.iter().map(|orbit| orbit.sigma[0]).collect();
        assert_eq!(retained_sigma, vec![0, 1]);
    }

    #[test]
    fn candidate_window_tolerates_roundoff_at_requested_boundary() {
        let cutoff = 1.01;
        let result = OrbitSearchResult {
            orbits: vec![orbit(
                0,
                cutoff + 8.0 * f64::EPSILON,
                cutoff,
                OrbitAdmissibility::AdmissibleF64,
            )],
            min_action: 1.0,
            min_action_lower: 1.0,
            min_action_upper: 1.0,
            iterations: 1,
        };

        assert_eq!(candidate_window_orbits(&result, 0.01).len(), 1);
    }

    #[test]
    fn audit_iterations_are_sorted_and_deduplicated() {
        assert_eq!(parse_audit_iterations("4,0,2,2"), BTreeSet::from([0, 2, 4]));
    }

    #[test]
    fn selective_audit_runs_through_last_selected_base_then_stops() {
        let selected = BTreeSet::from([0, 4, 8]);
        assert_eq!(trace_iteration_limit(2, Some(&selected)), 9);
        assert!(!is_last_selected_audit(7, Some(&selected)));
        assert!(is_last_selected_audit(8, Some(&selected)));
        assert_eq!(trace_iteration_limit(2, None), 2);
    }

    #[test]
    fn audit_step_policies_preserve_order_and_deduplicate() {
        assert_eq!(
            parse_audit_step_policies("geometric,fixed,geometric,boundary-scaled"),
            vec![
                AuditStepPolicy::Geometric,
                AuditStepPolicy::Fixed,
                AuditStepPolicy::BoundaryScaled,
            ]
        );
    }

    #[test]
    fn rejected_audit_has_deterministic_nonempty_status_rows() {
        let requested = BTreeSet::from([1, 3]);
        let reached = BTreeMap::from([(1, "state:one".to_string())]);
        let rows = audit_state_status_rows_for_fixture(
            &test_fixture(),
            &requested,
            &reached,
            "line_search_all_steps_below_min_observed_delta",
        );

        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].requested_iteration, 1);
        assert_eq!(rows[0].status, "selected");
        assert_eq!(rows[0].state_id.as_deref(), Some("state:one"));
        assert_eq!(rows[1].requested_iteration, 3);
        assert_eq!(rows[1].status, "unreached_trace_stopped");
        assert!(rows[1].state_id.is_none());
        assert_eq!(
            rows[1].trace_stop_reason,
            "line_search_all_steps_below_min_observed_delta"
        );
    }

    #[test]
    fn audit_exact_step_cache_reuses_capacity_evaluation() {
        let mut cache = HashMap::new();
        let mut next_order = 1;
        let key = MoveCacheKey {
            step_bits: 1,
            direction_bits: vec![2, 3],
        };
        let evaluations = Cell::new(0usize);
        let first = cached_exact_step(&mut cache, &mut next_order, key.clone(), || {
            evaluations.set(evaluations.get() + 1);
            ExactStepEvaluation {
                status: "ok".to_string(),
                observed_delta_sys: Some(0.25),
                target_sys: Some(1.25),
                target_orbit_iterations: Some(7),
            }
        });
        let second = cached_exact_step(&mut cache, &mut next_order, key, || {
            panic!("cached move must not recompute its capacity")
        });

        assert!(!first.1);
        assert!(second.1);
        assert_eq!(evaluations.get(), 1);
        assert_eq!(
            first.0.exact_evaluation_order,
            second.0.exact_evaluation_order
        );
        assert_eq!(next_order, 2);
    }

    #[test]
    fn reset_owned_outputs_removes_stale_mode_specific_files_only() {
        let dir = std::env::temp_dir().join(format!(
            "dev-gradient-ascent-reset-owned-{}",
            std::process::id()
        ));
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join("endpoint-diagnostic.jsonl"), b"stale\n").unwrap();
        fs::write(dir.join("states.jsonl"), b"stale\n").unwrap();
        fs::write(dir.join("run-provenance.json"), b"stale\n").unwrap();
        fs::write(dir.join("unrelated.txt"), b"keep\n").unwrap();

        reset_owned_output_files(&dir);

        assert!(!dir.join("endpoint-diagnostic.jsonl").exists());
        assert!(!dir.join("states.jsonl").exists());
        assert!(!dir.join("run-provenance.json").exists());
        assert!(dir.join("unrelated.txt").exists());
        fs::remove_dir_all(dir).unwrap();
    }
}

//! Branch cartography tied to the branch degeneracy diagnostic.
//!
//! This command consumes a `dev-gradient-ascent-branch-diagnostic` output
//! directory, selects representative classified basepoints, and records
//! `(a0, data(a0), [(a, data(a), relation_to_a0)])` samples. The important
//! extra fields classify whether a nearby target best sigma was already visible
//! at `a0`, merely inside the wider candidate window, or apparently created by
//! a branch-domain/transition change.

use exp_sys_landscape::{reference::exact_volume_as_f64, SysLandscapePolytopeCache};
use good_lp::{constraint, default_solver, variable, variables, Expression, Solution, SolverModel};
use nalgebra::{DMatrix, Vector4};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fs::{self, File};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};
use std::time::{Instant, SystemTime, UNIX_EPOCH};
use symplectic::algorithms::facet_adjacency::{
    build_transition_matrix_from_facet_intersections_and_omega, is_feasible_cycle,
};
use symplectic::derivatives::{
    capacity_subgradients_a, clarke_directional_derivative_a, systolic_ratio_gradient_a,
    volume_derivatives_a,
};
use symplectic::{
    aggregate_orbits_with_dual_vertices_exact, classify_facets_from_dual_vertices,
    solve_billiard_candidates, solve_orbit_sigma_saddle_point, solve_pruned_hk2017_candidates,
    OrbitAdmissibility, OrbitGuaranteeMode, OrbitKktData, OrbitSearchError, OrbitSearchResult,
};

const DEFAULT_SELECTION_THRESHOLD_RELATIVE: f64 = 1.0e-3;
const DEFAULT_ACTION_WINDOW_RELATIVE: f64 = 1.0e-2;
const DEFAULT_STEPS: &[f64] = &[1.0e-4, 1.0e-3];
const DEFAULT_MAX_FIXTURES_PER_LABEL: usize = 1;
const DEFAULT_RANDOM_DIRECTIONS: usize = 2;
const DEFAULT_RANDOM_SEED: u64 = 0x5a51_2026_0612;
const DEFAULT_LAYERS: usize = 1;

#[derive(Debug)]
struct Cli {
    diagnostic_dir: PathBuf,
    polytope_table: PathBuf,
    out_dir: PathBuf,
    selection_threshold_relative: f64,
    action_window_relative: f64,
    steps: Vec<f64>,
    layers: usize,
    random_directions: usize,
    seed: u64,
    max_fixtures_per_label: usize,
    skip_fixtures_per_label: usize,
    degeneracy_labels: Vec<String>,
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
struct DiagnosticFixtureRow {
    poly_id: String,
    roles: Vec<String>,
    source_names: Vec<String>,
    seed_indices: Vec<usize>,
    best_strategies: Vec<String>,
    input_capacity: f64,
    input_volume: f64,
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
    provenance: Option<DiagnosticFixtureRow>,
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
    roles: Vec<String>,
    source_names: Vec<String>,
    seed_indices: Vec<usize>,
    best_strategies: Vec<String>,
    input_facet_count: usize,
    input_capacity: Option<f64>,
    input_volume: Option<f64>,
    input_sys: f64,
    near_active_count: usize,
}

#[derive(Clone, Debug, Serialize)]
struct PointRecord {
    point_key: String,
    source_state: String,
    parent_point_key: Option<String>,
    poly_id: String,
    degeneracy_label: String,
    selection_rank_within_label: usize,
    status: String,
    sys: Option<f64>,
    capacity: Option<f64>,
    volume: Option<f64>,
    min_action: Option<f64>,
    best_sigma: Option<Vec<usize>>,
    near_active_count: Option<usize>,
    candidate_orbit_count: Option<usize>,
    orbit_iterations: Option<u64>,
    active_min_beta_margin: Option<f64>,
    active_max_q_error_bound: Option<f64>,
}

#[derive(Clone, Debug, Serialize)]
struct BranchCartographySampleRow {
    base_point_key: String,
    target_point_key: String,
    poly_id: String,
    degeneracy_label: String,
    base_source_state: String,
    target_source_state: String,
    direction_label: String,
    step: f64,
    status: String,
    base_sys: f64,
    target_sys: Option<f64>,
    predicted_directional_derivative: Option<f64>,
    predicted_delta_sys: Option<f64>,
    observed_delta_sys: Option<f64>,
    base_best_sigma: Vec<usize>,
    target_best_sigma: Option<Vec<usize>>,
    base_near_active_count: usize,
    target_near_active_count: Option<usize>,
    base_candidate_orbit_count: usize,
    target_candidate_orbit_count: Option<usize>,
    target_best_sigma_in_base_near_active_set: Option<bool>,
    target_best_sigma_in_base_candidate_window: Option<bool>,
    target_best_sigma_base_transition_allowed: Option<bool>,
    target_best_sigma_base_solve_status: Option<String>,
    target_best_sigma_base_action_gap: Option<f64>,
    target_best_sigma_transitions_opened: Option<Vec<[usize; 2]>>,
    classification: String,
    base_orbit_iterations: u64,
    target_orbit_iterations: Option<u64>,
}

#[derive(Serialize)]
struct ComputeBudgetReport {
    command: String,
    diagnostic_dir: String,
    diagnostic_branch_set_path: String,
    diagnostic_fixture_selection_path: String,
    polytope_table: String,
    input_file_metadata: BTreeMap<String, FileMetadata>,
    selection_threshold_relative: f64,
    action_window_relative: f64,
    steps: Vec<f64>,
    layers: usize,
    random_directions: usize,
    seed: u64,
    max_fixtures_per_label: usize,
    skip_fixtures_per_label: usize,
    degeneracy_labels: Vec<String>,
    selected_fixtures: usize,
    point_records: usize,
    sample_rows: usize,
    base_orbit_iterations: u64,
    target_orbit_iterations: u64,
    failed_sample_rows: usize,
    elapsed_ms: f64,
}

#[derive(Serialize)]
struct Summary {
    method: String,
    diagnostic_dir: String,
    diagnostic_branch_set_path: String,
    diagnostic_fixture_selection_path: String,
    polytope_table: String,
    selection_threshold_relative: f64,
    action_window_relative: f64,
    steps: Vec<f64>,
    layers: usize,
    random_directions: usize,
    seed: u64,
    max_fixtures_per_label: usize,
    skip_fixtures_per_label: usize,
    degeneracy_labels: Vec<String>,
    selected_fixtures: usize,
    point_records: usize,
    sample_rows: usize,
    failed_sample_rows: usize,
    degeneracy_counts: BTreeMap<String, usize>,
    source_state_counts: BTreeMap<String, usize>,
    point_status_counts: BTreeMap<String, usize>,
    sample_status_counts: BTreeMap<String, usize>,
    sample_classification_counts: BTreeMap<String, usize>,
    out_dir: String,
    caveat: String,
}

#[derive(Clone, Debug, Serialize)]
struct FileMetadata {
    bytes: Option<u64>,
    modified_unix_seconds: Option<u64>,
    status: String,
}

#[derive(Clone, Debug)]
struct BaseState {
    polytope: SysLandscapePolytopeCache,
    capacity: OrbitSearchResult,
    sys: f64,
    near_active_orbits: Vec<OrbitKktData>,
    sys_gradients: Vec<Vec<Vector4<f64>>>,
}

struct FrontierPoint {
    point_key: String,
    source_state: String,
    base: BaseState,
}

fn main() {
    let cli = parse_args();
    fs::create_dir_all(&cli.out_dir).expect("failed to create output directory");
    let t0 = Instant::now();

    let branch_set_path = cli.diagnostic_dir.join("branch-set-diagnostic.jsonl");
    let fixture_selection_path = cli.diagnostic_dir.join("fixture-selection.jsonl");
    let diagnostic_rows: Vec<DiagnosticRow> = load_jsonl(&branch_set_path);
    let diagnostic_fixture_rows: HashMap<String, DiagnosticFixtureRow> =
        load_optional_jsonl(&fixture_selection_path)
            .into_iter()
            .map(|row: DiagnosticFixtureRow| (row.poly_id.clone(), row))
            .collect();
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
        &diagnostic_fixture_rows,
    );
    assert!(
        !fixtures.is_empty(),
        "branch cartography selected no fixtures; check --selection-threshold-relative, \
         --degeneracy-labels, --skip-fixtures-per-label, and that the polytope table \
         contains the selected diagnostic poly_id rows"
    );

    let fixture_rows: Vec<FixtureRow> = fixtures.iter().map(fixture_row).collect();
    let mut point_records = Vec::new();
    let mut sample_rows = Vec::new();
    let mut base_orbit_iterations = 0u64;
    let mut target_orbit_iterations = 0u64;

    for fixture in &fixtures {
        let base_point_key = selected_point_key(fixture);
        match compute_base_state_from_row(
            &fixture.polytope,
            cli.action_window_relative,
            cli.selection_threshold_relative,
        ) {
            Ok(base) => {
                base_orbit_iterations += base.capacity.iterations;
                point_records.push(point_record(
                    fixture,
                    &base_point_key,
                    "selected_fixture",
                    None,
                    &base,
                    "ok",
                ));
                let mut frontier = vec![FrontierPoint {
                    point_key: base_point_key,
                    source_state: "selected_fixture".to_string(),
                    base,
                }];
                for layer in 0..cli.layers {
                    let mut next_frontier = Vec::new();
                    for frontier_point in frontier {
                        let directions = cartography_directions(
                            &frontier_point.base,
                            cli.random_directions,
                            cli.seed ^ stable_fixture_seed(fixture) ^ layer as u64,
                        );
                        for (direction_label, direction) in directions {
                            for &step in &cli.steps {
                                let target_source_state =
                                    format!("sample_target_layer_{}", layer + 1);
                                let outcome = cartography_sample_row(
                                    fixture,
                                    &frontier_point.base,
                                    &frontier_point.point_key,
                                    &frontier_point.source_state,
                                    &target_source_state,
                                    &direction_label,
                                    &direction,
                                    step,
                                    cli.action_window_relative,
                                    cli.selection_threshold_relative,
                                );
                                if let Some(iterations) = outcome.row.target_orbit_iterations {
                                    target_orbit_iterations += iterations;
                                }
                                if let Some(target_point) = outcome.target_point {
                                    point_records.push(target_point);
                                }
                                if layer + 1 < cli.layers
                                    && outcome
                                        .row
                                        .observed_delta_sys
                                        .is_some_and(|delta| delta > 0.0)
                                {
                                    if let Some(target_base) = outcome.target_base {
                                        next_frontier.push(FrontierPoint {
                                            point_key: outcome.row.target_point_key.clone(),
                                            source_state: target_source_state,
                                            base: target_base,
                                        });
                                    }
                                }
                                sample_rows.push(outcome.row);
                            }
                        }
                    }
                    frontier = next_frontier;
                    if frontier.is_empty() {
                        break;
                    }
                }
            }
            Err(err) => {
                point_records.push(failed_point_record(
                    fixture,
                    &base_point_key,
                    "selected_fixture",
                    None,
                    err,
                ));
            }
        }
    }

    write_jsonl(cli.out_dir.join("fixture-selection.jsonl"), &fixture_rows)
        .expect("failed to write fixture-selection.jsonl");
    write_jsonl(
        cli.out_dir.join("branch-cartography-points.jsonl"),
        &point_records,
    )
    .expect("failed to write branch-cartography-points.jsonl");
    write_jsonl(
        cli.out_dir.join("branch-cartography-samples.jsonl"),
        &sample_rows,
    )
    .expect("failed to write branch-cartography-samples.jsonl");

    let failed_sample_rows = sample_rows
        .iter()
        .filter(|row| row.status.as_str() != "ok")
        .count();
    let report = ComputeBudgetReport {
        command: "dev-gradient-ascent-branch-cartography".to_string(),
        diagnostic_dir: cli.diagnostic_dir.display().to_string(),
        diagnostic_branch_set_path: branch_set_path.display().to_string(),
        diagnostic_fixture_selection_path: fixture_selection_path.display().to_string(),
        polytope_table: cli.polytope_table.display().to_string(),
        input_file_metadata: input_file_metadata(&[
            ("branch-set-diagnostic", branch_set_path.as_path()),
            ("fixture-selection", fixture_selection_path.as_path()),
            ("polytope-table", cli.polytope_table.as_path()),
        ]),
        selection_threshold_relative: cli.selection_threshold_relative,
        action_window_relative: cli.action_window_relative,
        steps: cli.steps.clone(),
        layers: cli.layers,
        random_directions: cli.random_directions,
        seed: cli.seed,
        max_fixtures_per_label: cli.max_fixtures_per_label,
        skip_fixtures_per_label: cli.skip_fixtures_per_label,
        degeneracy_labels: cli.degeneracy_labels.clone(),
        selected_fixtures: fixtures.len(),
        point_records: point_records.len(),
        sample_rows: sample_rows.len(),
        base_orbit_iterations,
        target_orbit_iterations,
        failed_sample_rows,
        elapsed_ms: t0.elapsed().as_secs_f64() * 1000.0,
    };
    write_json(cli.out_dir.join("compute-budget-report.json"), &report)
        .expect("failed to write compute-budget-report.json");

    let summary = Summary {
        method: "dev-gradient-ascent-branch-cartography".to_string(),
        diagnostic_dir: cli.diagnostic_dir.display().to_string(),
        diagnostic_branch_set_path: branch_set_path.display().to_string(),
        diagnostic_fixture_selection_path: fixture_selection_path.display().to_string(),
        polytope_table: cli.polytope_table.display().to_string(),
        selection_threshold_relative: cli.selection_threshold_relative,
        action_window_relative: cli.action_window_relative,
        steps: cli.steps.clone(),
        layers: cli.layers,
        random_directions: cli.random_directions,
        seed: cli.seed,
        max_fixtures_per_label: cli.max_fixtures_per_label,
        skip_fixtures_per_label: cli.skip_fixtures_per_label,
        degeneracy_labels: cli.degeneracy_labels.clone(),
        selected_fixtures: fixtures.len(),
        point_records: point_records.len(),
        sample_rows: sample_rows.len(),
        failed_sample_rows,
        degeneracy_counts: count_fixture_degeneracy(&fixtures),
        source_state_counts: count_point_source_states(&point_records),
        point_status_counts: count_point_statuses(&point_records),
        sample_status_counts: count_sample_statuses(&sample_rows),
        sample_classification_counts: count_sample_classifications(&sample_rows),
        out_dir: cli.out_dir.display().to_string(),
        caveat: "finite branch-cartography samples only; this does not certify endpoint local maximality"
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
    diagnostic_fixture_rows: &HashMap<String, DiagnosticFixtureRow>,
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
            provenance: diagnostic_fixture_rows.get(&row.poly_id).cloned(),
            selection_rank_within_label,
        });
        *selected_count += 1;
    }

    selected
}

struct CartographySampleOutcome {
    row: BranchCartographySampleRow,
    target_point: Option<PointRecord>,
    target_base: Option<BaseState>,
}

fn cartography_sample_row(
    fixture: &Fixture,
    base: &BaseState,
    base_point_key: &str,
    base_source_state: &str,
    target_source_state: &str,
    direction_label: &str,
    direction: &[Vector4<f64>],
    step: f64,
    action_window_relative: f64,
    branch_threshold_relative: f64,
) -> CartographySampleOutcome {
    let target_point_key = sample_point_key(base_point_key, direction_label, step);
    let base_best_sigma = base.capacity.best_sigma().to_vec();
    let predicted_directional_derivative =
        match clarke_directional_derivative_a(&base.sys_gradients, direction) {
            Ok(value) => value,
            Err(err) => {
                return failed_cartography_sample(
                    fixture,
                    base,
                    base_point_key,
                    base_source_state,
                    target_source_state,
                    &target_point_key,
                    direction_label,
                    step,
                    format!("directional_derivative_failed:{err:?}"),
                );
            }
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
        return failed_cartography_sample(
            fixture,
            base,
            base_point_key,
            base_source_state,
            target_source_state,
            &target_point_key,
            direction_label,
            step,
            "target_polytope_construction_failed".to_string(),
        );
    };
    let target_base = match compute_base_state_from_polytope(
        target_polytope,
        base.capacity.min_action * action_window_relative,
        branch_threshold_relative,
    ) {
        Ok(target_base) => target_base,
        Err(err) => {
            return failed_cartography_sample(
                fixture,
                base,
                base_point_key,
                base_source_state,
                target_source_state,
                &target_point_key,
                direction_label,
                step,
                err,
            );
        }
    };
    let target_best_sigma = target_base.capacity.best_sigma().to_vec();
    let relation = target_sigma_relation(base, &target_base, &target_best_sigma);
    let target_sys = target_base.sys;
    let classification = classify_sample(
        target_sys - base.sys,
        relation.in_base_near_active_set,
        relation.in_base_candidate_window,
        relation.base_transition_allowed,
        &relation.base_solve_status,
        relation.base_action_gap,
        &relation.transitions_opened,
    );
    let row = BranchCartographySampleRow {
        base_point_key: base_point_key.to_string(),
        target_point_key: target_point_key.clone(),
        poly_id: fixture.polytope.poly_id.clone(),
        degeneracy_label: fixture.diagnostic.degeneracy_label.clone(),
        base_source_state: base_source_state.to_string(),
        target_source_state: target_source_state.to_string(),
        direction_label: direction_label.to_string(),
        step,
        status: "ok".to_string(),
        base_sys: base.sys,
        target_sys: Some(target_sys),
        predicted_directional_derivative: Some(predicted_directional_derivative),
        predicted_delta_sys: Some(step * predicted_directional_derivative),
        observed_delta_sys: Some(target_sys - base.sys),
        base_best_sigma,
        target_best_sigma: Some(target_best_sigma),
        base_near_active_count: base.near_active_orbits.len(),
        target_near_active_count: Some(target_base.near_active_orbits.len()),
        base_candidate_orbit_count: base.capacity.orbits.len(),
        target_candidate_orbit_count: Some(target_base.capacity.orbits.len()),
        target_best_sigma_in_base_near_active_set: Some(relation.in_base_near_active_set),
        target_best_sigma_in_base_candidate_window: Some(relation.in_base_candidate_window),
        target_best_sigma_base_transition_allowed: Some(relation.base_transition_allowed),
        target_best_sigma_base_solve_status: Some(relation.base_solve_status),
        target_best_sigma_base_action_gap: relation.base_action_gap,
        target_best_sigma_transitions_opened: Some(relation.transitions_opened),
        classification,
        base_orbit_iterations: base.capacity.iterations,
        target_orbit_iterations: Some(target_base.capacity.iterations),
    };
    let target_point = point_record(
        fixture,
        &target_point_key,
        target_source_state,
        Some(base_point_key.to_string()),
        &target_base,
        "ok",
    );
    CartographySampleOutcome {
        row,
        target_point: Some(target_point),
        target_base: Some(target_base),
    }
}

struct TargetSigmaRelation {
    in_base_near_active_set: bool,
    in_base_candidate_window: bool,
    base_transition_allowed: bool,
    base_solve_status: String,
    base_action_gap: Option<f64>,
    transitions_opened: Vec<[usize; 2]>,
}

fn target_sigma_relation(
    base: &BaseState,
    target: &BaseState,
    target_best_sigma: &[usize],
) -> TargetSigmaRelation {
    let in_base_near_active_set = base
        .near_active_orbits
        .iter()
        .any(|orbit| orbit.sigma == target_best_sigma);
    let in_base_candidate_window = base
        .capacity
        .orbits
        .iter()
        .any(|orbit| orbit.sigma == target_best_sigma);
    let base_transition_is_allowed = build_transition_matrix_from_facet_intersections_and_omega(
        &base.polytope.facet_intersection_is_nonempty,
        &base.polytope.omega_signs,
    );
    let target_transition_is_allowed = build_transition_matrix_from_facet_intersections_and_omega(
        &target.polytope.facet_intersection_is_nonempty,
        &target.polytope.omega_signs,
    );
    let base_transition_allowed = is_feasible_cycle(target_best_sigma, &base_transition_is_allowed);
    let transitions_opened = transitions_rejected_then_allowed(
        target_best_sigma,
        &base_transition_is_allowed,
        &target_transition_is_allowed,
    );
    let (base_solve_status, base_action_gap) =
        match solve_orbit_sigma_saddle_point(&base.polytope.dual_vertices_f64, target_best_sigma) {
            Ok(base_target_orbit) => (
                format!("ok:{:?}", base_target_orbit.admissibility),
                Some(base_target_orbit.action - base.capacity.min_action),
            ),
            Err(err) => (format!("{err:?}"), None),
        };
    TargetSigmaRelation {
        in_base_near_active_set,
        in_base_candidate_window,
        base_transition_allowed,
        base_solve_status,
        base_action_gap,
        transitions_opened,
    }
}

fn failed_cartography_sample(
    fixture: &Fixture,
    base: &BaseState,
    base_point_key: &str,
    base_source_state: &str,
    target_source_state: &str,
    target_point_key: &str,
    direction_label: &str,
    step: f64,
    status: String,
) -> CartographySampleOutcome {
    CartographySampleOutcome {
        row: BranchCartographySampleRow {
            base_point_key: base_point_key.to_string(),
            target_point_key: target_point_key.to_string(),
            poly_id: fixture.polytope.poly_id.clone(),
            degeneracy_label: fixture.diagnostic.degeneracy_label.clone(),
            base_source_state: base_source_state.to_string(),
            target_source_state: target_source_state.to_string(),
            direction_label: direction_label.to_string(),
            step,
            status,
            base_sys: base.sys,
            target_sys: None,
            predicted_directional_derivative: None,
            predicted_delta_sys: None,
            observed_delta_sys: None,
            base_best_sigma: base.capacity.best_sigma().to_vec(),
            target_best_sigma: None,
            base_near_active_count: base.near_active_orbits.len(),
            target_near_active_count: None,
            base_candidate_orbit_count: base.capacity.orbits.len(),
            target_candidate_orbit_count: None,
            target_best_sigma_in_base_near_active_set: None,
            target_best_sigma_in_base_candidate_window: None,
            target_best_sigma_base_transition_allowed: None,
            target_best_sigma_base_solve_status: None,
            target_best_sigma_base_action_gap: None,
            target_best_sigma_transitions_opened: None,
            classification: "sample_failed".to_string(),
            base_orbit_iterations: base.capacity.iterations,
            target_orbit_iterations: None,
        },
        target_point: None,
        target_base: None,
    }
}

fn classify_sample(
    observed_delta_sys: f64,
    in_base_near_active_set: bool,
    in_base_candidate_window: bool,
    base_transition_allowed: bool,
    base_solve_status: &str,
    base_action_gap: Option<f64>,
    transitions_opened: &[[usize; 2]],
) -> String {
    let sign = if observed_delta_sys > 0.0 {
        "improving"
    } else {
        "non_improving"
    };
    let relation = if in_base_near_active_set {
        "visible_near_active_branch"
    } else if in_base_candidate_window {
        "seen_in_candidate_window"
    } else if !transitions_opened.is_empty() {
        "transition_opened"
    } else if !base_transition_allowed {
        "base_transition_blocked"
    } else if !base_solve_status.contains("AdmissibleF64")
        && !base_solve_status.contains("AdmissibleExact")
    {
        "kkt_or_admissibility_change"
    } else if base_action_gap.is_some() {
        "outside_base_candidate_window"
    } else {
        "unclassified_target_sigma"
    };
    format!("{sign}_{relation}")
}

fn point_record(
    fixture: &Fixture,
    point_key: &str,
    source_state: &str,
    parent_point_key: Option<String>,
    base: &BaseState,
    status: &str,
) -> PointRecord {
    let volume = exact_volume_as_f64(
        &base.polytope.vertices,
        &base.polytope.vertex_facet_incidence,
    );
    let active_min_beta_margin = base
        .near_active_orbits
        .iter()
        .map(|orbit| orbit.beta_margin)
        .fold(f64::INFINITY, f64::min);
    let active_max_q_error_bound = base
        .near_active_orbits
        .iter()
        .map(|orbit| orbit.q_error_bound)
        .fold(0.0, f64::max);
    PointRecord {
        point_key: point_key.to_string(),
        source_state: source_state.to_string(),
        parent_point_key,
        poly_id: fixture.polytope.poly_id.clone(),
        degeneracy_label: fixture.diagnostic.degeneracy_label.clone(),
        selection_rank_within_label: fixture.selection_rank_within_label,
        status: status.to_string(),
        sys: Some(base.sys),
        capacity: Some(base.capacity.min_action),
        volume: Some(volume),
        min_action: Some(base.capacity.min_action),
        best_sigma: Some(base.capacity.best_sigma().to_vec()),
        near_active_count: Some(base.near_active_orbits.len()),
        candidate_orbit_count: Some(base.capacity.orbits.len()),
        orbit_iterations: Some(base.capacity.iterations),
        active_min_beta_margin: active_min_beta_margin
            .is_finite()
            .then_some(active_min_beta_margin),
        active_max_q_error_bound: Some(active_max_q_error_bound),
    }
}

fn failed_point_record(
    fixture: &Fixture,
    point_key: &str,
    source_state: &str,
    parent_point_key: Option<String>,
    status: String,
) -> PointRecord {
    PointRecord {
        point_key: point_key.to_string(),
        source_state: source_state.to_string(),
        parent_point_key,
        poly_id: fixture.polytope.poly_id.clone(),
        degeneracy_label: fixture.diagnostic.degeneracy_label.clone(),
        selection_rank_within_label: fixture.selection_rank_within_label,
        status,
        sys: None,
        capacity: None,
        volume: None,
        min_action: None,
        best_sigma: None,
        near_active_count: None,
        candidate_orbit_count: None,
        orbit_iterations: None,
        active_min_beta_margin: None,
        active_max_q_error_bound: None,
    }
}

fn cartography_directions(
    base: &BaseState,
    random_directions: usize,
    seed: u64,
) -> Vec<(String, Vec<Vector4<f64>>)> {
    let mut directions = probe_directions(base);
    let mut rng = ChaCha8Rng::seed_from_u64(seed);
    for idx in 0..random_directions {
        if let Some(direction) =
            random_unit_direction(base.polytope.dual_vertices_f64.len(), &mut rng)
        {
            directions.push((format!("random_unit_{idx}"), direction));
        }
    }
    directions
}

fn random_unit_direction(facet_count: usize, rng: &mut ChaCha8Rng) -> Option<Vec<Vector4<f64>>> {
    let direction: Vec<Vector4<f64>> = (0..facet_count)
        .map(|_| {
            Vector4::new(
                rng.gen_range(-1.0..=1.0),
                rng.gen_range(-1.0..=1.0),
                rng.gen_range(-1.0..=1.0),
                rng.gen_range(-1.0..=1.0),
            )
        })
        .collect();
    normalize_direction(&direction)
}

fn selected_point_key(fixture: &Fixture) -> String {
    format!(
        "selected:{}:{}:{}",
        fixture.diagnostic.degeneracy_label,
        fixture.selection_rank_within_label,
        fixture.polytope.poly_id
    )
}

fn sample_point_key(base_point_key: &str, direction_label: &str, step: f64) -> String {
    format!("{base_point_key}:sample:{direction_label}:{step:.3e}")
}

fn stable_fixture_seed(fixture: &Fixture) -> u64 {
    fixture
        .polytope
        .poly_id
        .bytes()
        .fold(fixture.selection_rank_within_label as u64, |acc, byte| {
            acc.wrapping_mul(0x100_0000_01b3).wrapping_add(byte as u64)
        })
}

fn transitions_rejected_then_allowed(
    sigma: &[usize],
    base_transition_is_allowed: &DMatrix<bool>,
    target_transition_is_allowed: &DMatrix<bool>,
) -> Vec<[usize; 2]> {
    sigma
        .iter()
        .copied()
        .zip(sigma.iter().copied().cycle().skip(1))
        .take(sigma.len())
        .filter(|&(from, to)| {
            !base_transition_is_allowed[(from, to)] && target_transition_is_allowed[(from, to)]
        })
        .map(|(from, to)| [from, to])
        .collect()
}

fn count_point_statuses(rows: &[PointRecord]) -> BTreeMap<String, usize> {
    let mut counts = BTreeMap::new();
    for row in rows {
        *counts.entry(row.status.clone()).or_insert(0) += 1;
    }
    counts
}

fn count_point_source_states(rows: &[PointRecord]) -> BTreeMap<String, usize> {
    let mut counts = BTreeMap::new();
    for row in rows {
        *counts.entry(row.source_state.clone()).or_insert(0) += 1;
    }
    counts
}

fn count_sample_statuses(rows: &[BranchCartographySampleRow]) -> BTreeMap<String, usize> {
    let mut counts = BTreeMap::new();
    for row in rows {
        *counts.entry(row.status.clone()).or_insert(0) += 1;
    }
    counts
}

fn count_sample_classifications(rows: &[BranchCartographySampleRow]) -> BTreeMap<String, usize> {
    let mut counts = BTreeMap::new();
    for row in rows {
        *counts.entry(row.classification.clone()).or_insert(0) += 1;
    }
    counts
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
    let vol = exact_volume_as_f64(&polytope.vertices, &polytope.vertex_facet_incidence);
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

    Ok(BaseState {
        polytope,
        capacity,
        sys,
        near_active_orbits,
        sys_gradients,
    })
}

fn probe_directions(base: &BaseState) -> Vec<(String, Vec<Vector4<f64>>)> {
    let mut directions = Vec::new();
    if let Some(first_gradient) = base.sys_gradients.first() {
        if let Some(direction) = normalize_direction(first_gradient) {
            directions.push(("single_near_active_gradient".to_string(), direction.clone()));
            directions.push((
                "negative_single_near_active_gradient".to_string(),
                direction.iter().map(|v| -*v).collect(),
            ));
        }
    }
    if base.sys_gradients.len() > 1 {
        if let Some(direction) = maximin_direction(&base.sys_gradients) {
            directions.push(("near_active_maximin_direction".to_string(), direction));
        }
    }
    directions
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
    let provenance = fixture.provenance.as_ref();
    FixtureRow {
        poly_id: fixture.polytope.poly_id.clone(),
        degeneracy_label: fixture.diagnostic.degeneracy_label.clone(),
        selection_rank_within_label: fixture.selection_rank_within_label,
        threshold_relative: fixture.diagnostic.threshold_relative,
        selection_buckets: fixture.diagnostic.selection_buckets.clone(),
        datasets: fixture.diagnostic.datasets.clone(),
        roles: provenance.map(|row| row.roles.clone()).unwrap_or_default(),
        source_names: provenance
            .map(|row| row.source_names.clone())
            .unwrap_or_default(),
        seed_indices: provenance
            .map(|row| row.seed_indices.clone())
            .unwrap_or_default(),
        best_strategies: provenance
            .map(|row| row.best_strategies.clone())
            .unwrap_or_default(),
        input_facet_count: fixture.diagnostic.input_facet_count,
        input_capacity: provenance.map(|row| row.input_capacity),
        input_volume: provenance.map(|row| row.input_volume),
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

fn parse_args() -> Cli {
    let mut cli = Cli {
        diagnostic_dir: PathBuf::new(),
        polytope_table: default_tables_dir().join("polytope-table.jsonl"),
        out_dir: default_output_dir(),
        selection_threshold_relative: DEFAULT_SELECTION_THRESHOLD_RELATIVE,
        action_window_relative: DEFAULT_ACTION_WINDOW_RELATIVE,
        steps: DEFAULT_STEPS.to_vec(),
        layers: DEFAULT_LAYERS,
        random_directions: DEFAULT_RANDOM_DIRECTIONS,
        seed: DEFAULT_RANDOM_SEED,
        max_fixtures_per_label: DEFAULT_MAX_FIXTURES_PER_LABEL,
        skip_fixtures_per_label: 0,
        degeneracy_labels: vec![
            "large_gap".to_string(),
            "narrow_gap".to_string(),
            "high_degeneracy".to_string(),
        ],
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
            "--steps" => {
                cli.steps = args
                    .next()
                    .expect("--steps requires comma-separated f64 values")
                    .split(',')
                    .map(|value| value.parse().expect("--steps entries must be f64"))
                    .collect();
            }
            "--layers" => {
                cli.layers = args
                    .next()
                    .expect("--layers requires an integer")
                    .parse()
                    .expect("--layers must be an integer");
            }
            "--random-directions" => {
                cli.random_directions = args
                    .next()
                    .expect("--random-directions requires an integer")
                    .parse()
                    .expect("--random-directions must be an integer");
            }
            "--seed" => {
                cli.seed = args
                    .next()
                    .expect("--seed requires an integer")
                    .parse()
                    .expect("--seed must be an integer");
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
            "--degeneracy-labels" => {
                cli.degeneracy_labels = args
                    .next()
                    .expect("--degeneracy-labels requires comma-separated labels")
                    .split(',')
                    .filter(|value| !value.is_empty())
                    .map(ToString::to_string)
                    .collect();
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
        "Usage: dev-gradient-ascent-branch-cartography --diagnostic-dir PATH \
         [--polytope-table PATH] [--out-dir PATH] \
         [--selection-threshold-relative F64] [--action-window-relative F64] \
         [--steps CSV] [--layers N] \
         [--random-directions N] [--seed U64] \
         [--max-fixtures-per-label N] [--skip-fixtures-per-label N] \
         [--degeneracy-labels CSV]"
    );
}

fn default_tables_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../polytope-invariant-table")
}

fn default_output_dir() -> PathBuf {
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock before UNIX_EPOCH")
        .as_millis();
    std::env::temp_dir().join(format!(
        "dev-gradient-ascent-branch-cartography-{}-{stamp}",
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

fn load_optional_jsonl<T: for<'de> Deserialize<'de>>(path: &Path) -> Vec<T> {
    if path.exists() {
        load_jsonl(path)
    } else {
        Vec::new()
    }
}

fn input_file_metadata(paths: &[(&str, &Path)]) -> BTreeMap<String, FileMetadata> {
    paths
        .iter()
        .map(|(label, path)| ((*label).to_string(), file_metadata(path)))
        .collect()
}

fn file_metadata(path: &Path) -> FileMetadata {
    match fs::metadata(path) {
        Ok(metadata) => {
            let modified_unix_seconds = metadata
                .modified()
                .ok()
                .and_then(|modified| modified.duration_since(UNIX_EPOCH).ok())
                .map(|duration| duration.as_secs());
            FileMetadata {
                bytes: Some(metadata.len()),
                modified_unix_seconds,
                status: "ok".to_string(),
            }
        }
        Err(err) => FileMetadata {
            bytes: None,
            modified_unix_seconds: None,
            status: format!("metadata_failed:{err}"),
        },
    }
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

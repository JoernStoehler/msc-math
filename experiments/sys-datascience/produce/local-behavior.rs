//! Scratch producer for local and semi-local `sys(a)` branch-behavior probes.
//!
//! This command is intentionally producer-shaped: it writes reusable computed
//! polytope payloads plus metadata for generated `(a0, a0 + t d)` samples under
//! an explicit output directory. It is not a canonical retained producer yet.

use exp_sys_landscape::{
    exact_volume_from_incidence_as_f64, f64_dual_vertices, orbit_scalars_from_result, poly_id,
    rational_vec4_to_strings, ComputedPolytopePayloadRow, SysLandscapePolytopeCache,
};
use good_lp::{constraint, default_solver, variable, variables, Expression, Solution, SolverModel};
use nalgebra::Vector4;
use num_rational::BigRational;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use rand_distr::StandardNormal;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::time::Instant;
use symplectic::algorithms::facet_adjacency::build_transition_matrix_from_facet_intersections_and_omega;
use symplectic::database::SigmaAction;
use symplectic::derivatives::{
    capacity_subgradients_a, clarke_directional_derivative_a, systolic_ratio_gradient_a,
    volume_derivatives_a,
};
use symplectic::{
    aggregate_certified_orbits_with_dual_vertices_exact, aggregate_orbits_with_dual_vertices_exact,
    classify_facets_from_dual_vertices, solve_billiard_candidates, solve_pruned_hk2017_candidates,
    CertifiedOrbitSearchResult, CertifiedOrbitSetMode, OrbitAdmissibility, OrbitGuaranteeMode,
    OrbitKktData, OrbitSearchError, OrbitSearchResult,
};

const DEFAULT_RADII: &[f64] = &[1.0e-6, 1.0e-5, 1.0e-4, 1.0e-3, 1.0e-2];
const DEFAULT_BRANCH_THRESHOLD_RELATIVE: f64 = 1.0e-3;
const DEFAULT_ACTION_WINDOW_RELATIVE: f64 = 1.0e-2;
const DEFAULT_MAX_TOP_BASEPOINTS: usize = 2;
const DEFAULT_MAX_HASH_BASEPOINTS: usize = 2;
const DEFAULT_RANDOM_DIRECTIONS: usize = 2;
const DEFAULT_SEED: u64 = 0x5159_2026_0616;

#[derive(Debug)]
struct Args {
    polytope_table: PathBuf,
    output_dir: PathBuf,
    max_top_basepoints: usize,
    max_hash_basepoints: usize,
    radii: Vec<f64>,
    branch_threshold_relative: f64,
    action_window_relative: f64,
    random_directions: usize,
    seed: u64,
}

#[derive(Clone, Debug, Deserialize)]
struct PolytopeRow {
    poly_id: String,
    facet_count: usize,
    capacity: f64,
    sys: f64,
    dual_vertices_f64: Vec<[f64; 4]>,
}

#[derive(Clone, Debug)]
struct SelectedBasepoint {
    row: PolytopeRow,
    selection_buckets: BTreeSet<String>,
}

#[derive(Debug)]
struct LocalState {
    polytope: SysLandscapePolytopeCache,
    capacity: OrbitSearchResult,
    certified_capacity: CertifiedOrbitSearchResult,
    backend: String,
    action_gap: f64,
    volume: f64,
    sys: f64,
    near_active_orbits: Vec<OrbitKktData>,
    capacity_gradients: Vec<Vec<Vector4<f64>>>,
    sys_gradients: Vec<Vec<Vector4<f64>>>,
    time_volume_ms: f64,
    time_capacity_ms: f64,
}

#[derive(Clone, Debug)]
struct DirectionSpec {
    label: String,
    direction: Vec<Vector4<f64>>,
    predicted_derivative: Option<f64>,
}

#[derive(Debug, Serialize)]
struct BasepointStateRow {
    basepoint_id: String,
    input_poly_id: String,
    base_poly_id: Option<String>,
    selection_buckets: Vec<String>,
    facet_count: usize,
    input_capacity: f64,
    input_sys: f64,
    recomputed_capacity: Option<f64>,
    recomputed_volume: Option<f64>,
    recomputed_sys: Option<f64>,
    branch_threshold_relative: f64,
    action_window_relative: f64,
    returned_orbit_count: Option<usize>,
    admissible_orbit_count: Option<usize>,
    near_active_count: Option<usize>,
    min_action: Option<f64>,
    action_gap_to_second: Option<f64>,
    best_sigma: Option<Vec<usize>>,
    strict_min_branch_count: Option<usize>,
    strict_min_branch_sigmas: Vec<Vec<usize>>,
    certified_action_window_branch_count: Option<usize>,
    certified_exact_resolutions: Option<usize>,
    near_active_sigmas: Vec<Vec<usize>>,
    candidate_window_sigmas: Vec<Vec<usize>>,
    orbit_iterations: Option<u64>,
    time_volume_ms: Option<f64>,
    time_capacity_ms: Option<f64>,
    gradient_count: Option<usize>,
    generated_direction_count: Option<usize>,
    failure: Option<String>,
}

#[derive(Debug, Serialize)]
struct LocalBehaviorSampleRow {
    sample_id: String,
    basepoint_id: String,
    input_poly_id: String,
    base_poly_id: String,
    target_poly_id: Option<String>,
    selection_buckets: Vec<String>,
    direction_label: String,
    direction_vector: Vec<[f64; 4]>,
    radius: f64,
    status: String,
    base_sys: f64,
    target_sys: Option<f64>,
    observed_delta_sys: Option<f64>,
    predicted_derivative: Option<f64>,
    predicted_delta_sys: Option<f64>,
    prediction_error: Option<f64>,
    prediction_error_abs: Option<f64>,
    prediction_ratio_observed_over_predicted: Option<f64>,
    target_orbit_iterations: Option<u64>,
    time_volume_ms: Option<f64>,
    time_capacity_ms: Option<f64>,
    failure: Option<String>,
}

#[derive(Debug)]
struct SampleOutcome {
    row: LocalBehaviorSampleRow,
}

#[derive(Debug, Serialize)]
struct BranchGradientRow {
    basepoint_id: String,
    input_poly_id: String,
    base_poly_id: String,
    orbit_index: usize,
    sigma: Vec<usize>,
    action: f64,
    relative_action_gap_from_min: f64,
    capacity_gradient: Vec<[f64; 4]>,
    sys_sigma_gradient: Vec<[f64; 4]>,
    gradient_norm: f64,
}

#[derive(Debug, Serialize)]
struct ProduceStatsRow {
    polytope_table: String,
    output_dir: String,
    max_top_basepoints: usize,
    max_hash_basepoints: usize,
    selected_basepoints: usize,
    radii: Vec<f64>,
    branch_threshold_relative: f64,
    action_window_relative: f64,
    random_directions: usize,
    seed: u64,
    basepoint_rows: usize,
    sample_rows: usize,
    branch_gradient_rows: usize,
    computed_payload_rows: usize,
    failures: usize,
    sample_status_counts: BTreeMap<String, usize>,
    max_base_sys: Option<f64>,
    max_target_sys: Option<f64>,
    max_observed_delta_sys: Option<f64>,
    local_state_volume_ms: f64,
    local_state_capacity_ms: f64,
    wall_time_ms: f64,
}

fn main() {
    let started = Instant::now();
    let args = parse_args();
    std::fs::create_dir_all(&args.output_dir).expect("create output dir");

    let computed_payload_path = args.output_dir.join("computed-polytopes.jsonl");
    let input_rows = read_jsonl::<PolytopeRow>(&args.polytope_table);
    let selected = select_basepoints(
        input_rows,
        args.max_top_basepoints,
        args.max_hash_basepoints,
    );

    println!(
        "local-behavior produce: basepoints={} radii={:?} random_directions={} output_dir={}",
        selected.len(),
        args.radii,
        args.random_directions,
        args.output_dir.display()
    );
    flush_stdout();

    let failures = Mutex::new(0usize);
    let mut basepoint_rows = Vec::new();
    let mut sample_rows = Vec::new();
    let mut branch_gradient_rows = Vec::new();
    let mut payload_rows = BTreeMap::new();

    for (basepoint_index, basepoint) in selected.iter().enumerate() {
        let basepoint_id = format!("base_{basepoint_index:04}");
        let base_state = compute_local_state_from_row(
            &basepoint.row,
            args.action_window_relative,
            args.branch_threshold_relative,
        );
        match base_state {
            Ok(base_state) => {
                insert_payload(&mut payload_rows, payload_from_state(&base_state));
                let directions = generate_directions(
                    &base_state,
                    &basepoint.row.poly_id,
                    args.random_directions,
                    args.seed,
                );
                basepoint_rows.push(basepoint_state_row(
                    &basepoint_id,
                    basepoint,
                    &base_state,
                    &directions,
                    &args,
                ));
                branch_gradient_rows.extend(basepoint_branch_gradient_rows(
                    &basepoint_id,
                    basepoint,
                    &base_state,
                ));
                for direction in directions {
                    for (radius_index, &radius) in args.radii.iter().enumerate() {
                        let outcome = sample_row(
                            &basepoint_id,
                            basepoint,
                            &base_state,
                            &direction,
                            radius_index,
                            radius,
                            &args,
                            &mut payload_rows,
                        );
                        sample_rows.push(outcome.row);
                    }
                }
            }
            Err(err) => {
                *failures.lock().expect("failure mutex poisoned") += 1;
                basepoint_rows.push(failed_basepoint_state_row(
                    &basepoint_id,
                    basepoint,
                    &args,
                    err,
                ));
            }
        }
    }

    let payload_rows: Vec<ComputedPolytopePayloadRow> = payload_rows.into_values().collect();
    write_jsonl(
        args.output_dir.join("local-behavior-basepoints.jsonl"),
        &basepoint_rows,
    );
    write_jsonl(
        args.output_dir.join("local-behavior-samples.jsonl"),
        &sample_rows,
    );
    write_jsonl(
        args.output_dir
            .join("local-behavior-branch-gradients.jsonl"),
        &branch_gradient_rows,
    );
    write_jsonl(computed_payload_path, &payload_rows);

    let failure_count = *failures.lock().expect("failure mutex poisoned")
        + sample_rows
            .iter()
            .filter(|row| row.failure.is_some())
            .count();
    let stats = ProduceStatsRow {
        polytope_table: args.polytope_table.display().to_string(),
        output_dir: args.output_dir.display().to_string(),
        max_top_basepoints: args.max_top_basepoints,
        max_hash_basepoints: args.max_hash_basepoints,
        selected_basepoints: selected.len(),
        radii: args.radii.clone(),
        branch_threshold_relative: args.branch_threshold_relative,
        action_window_relative: args.action_window_relative,
        random_directions: args.random_directions,
        seed: args.seed,
        basepoint_rows: basepoint_rows.len(),
        sample_rows: sample_rows.len(),
        branch_gradient_rows: branch_gradient_rows.len(),
        computed_payload_rows: payload_rows.len(),
        failures: failure_count,
        sample_status_counts: count_by(&sample_rows, |row| row.status.as_str()),
        max_base_sys: basepoint_rows
            .iter()
            .filter_map(|row| row.recomputed_sys)
            .reduce(f64::max),
        max_target_sys: sample_rows
            .iter()
            .filter_map(|row| row.target_sys)
            .reduce(f64::max),
        max_observed_delta_sys: sample_rows
            .iter()
            .filter_map(|row| row.observed_delta_sys)
            .reduce(f64::max),
        local_state_volume_ms: basepoint_rows
            .iter()
            .filter_map(|row| row.time_volume_ms)
            .sum::<f64>()
            + sample_rows
                .iter()
                .filter_map(|row| row.time_volume_ms)
                .sum::<f64>(),
        local_state_capacity_ms: basepoint_rows
            .iter()
            .filter_map(|row| row.time_capacity_ms)
            .sum::<f64>()
            + sample_rows
                .iter()
                .filter_map(|row| row.time_capacity_ms)
                .sum::<f64>(),
        wall_time_ms: started.elapsed().as_secs_f64() * 1000.0,
    };
    write_json(args.output_dir.join("produce-stats.json"), &stats);
    println!(
        "wrote basepoints={} samples={} branch_gradients={} computed_payloads={} failures={}",
        stats.basepoint_rows,
        stats.sample_rows,
        stats.branch_gradient_rows,
        stats.computed_payload_rows,
        stats.failures
    );
}

fn parse_args() -> Args {
    let mut polytope_table =
        PathBuf::from("experiments/sys-datascience/prepare/polytope-table.jsonl");
    let mut output_dir = None;
    let mut max_top_basepoints = DEFAULT_MAX_TOP_BASEPOINTS;
    let mut max_hash_basepoints = DEFAULT_MAX_HASH_BASEPOINTS;
    let mut radii = DEFAULT_RADII.to_vec();
    let mut branch_threshold_relative = DEFAULT_BRANCH_THRESHOLD_RELATIVE;
    let mut action_window_relative = DEFAULT_ACTION_WINDOW_RELATIVE;
    let mut random_directions = DEFAULT_RANDOM_DIRECTIONS;
    let mut seed = DEFAULT_SEED;

    let argv: Vec<String> = std::env::args().collect();
    let mut i = 1usize;
    while i < argv.len() {
        let flag = argv[i].as_str();
        let value = || -> &str {
            argv.get(i + 1)
                .map(String::as_str)
                .unwrap_or_else(|| panic!("{flag} requires a value"))
        };
        match flag {
            "--polytope-table" => {
                polytope_table = PathBuf::from(value());
                i += 2;
            }
            "--out-dir" => {
                output_dir = Some(PathBuf::from(value()));
                i += 2;
            }
            "--max-top-basepoints" => {
                max_top_basepoints = parse_usize(value(), "--max-top-basepoints");
                i += 2;
            }
            "--max-hash-basepoints" => {
                max_hash_basepoints = parse_usize(value(), "--max-hash-basepoints");
                i += 2;
            }
            "--radii" => {
                radii = parse_f64_list(value(), "--radii");
                i += 2;
            }
            "--branch-threshold-relative" => {
                branch_threshold_relative =
                    parse_nonnegative_f64(value(), "--branch-threshold-relative");
                i += 2;
            }
            "--action-window-relative" => {
                action_window_relative = parse_nonnegative_f64(value(), "--action-window-relative");
                i += 2;
            }
            "--random-directions" => {
                random_directions = parse_usize(value(), "--random-directions");
                i += 2;
            }
            "--seed" => {
                seed = value().parse().expect("--seed must be a u64");
                i += 2;
            }
            "--help" | "-h" => {
                print_help(
                    argv.first()
                        .map(String::as_str)
                        .unwrap_or("sys-local-behavior-produce"),
                );
                std::process::exit(0);
            }
            other => panic!("unknown argument: {other}"),
        }
    }

    Args {
        polytope_table,
        output_dir: output_dir.expect("--out-dir is required"),
        max_top_basepoints,
        max_hash_basepoints,
        radii,
        branch_threshold_relative,
        action_window_relative,
        random_directions,
        seed,
    }
}

fn print_help(program: &str) {
    println!(
        "\
Scratch producer for local `sys(a)` branch-behavior probes.

Usage:
  {program} --out-dir <dir> [options]

Inputs:
  --polytope-table <path>             default: experiments/sys-datascience/prepare/polytope-table.jsonl

Selection:
  --max-top-basepoints <n>            default: {DEFAULT_MAX_TOP_BASEPOINTS}
  --max-hash-basepoints <n>           default: {DEFAULT_MAX_HASH_BASEPOINTS}

Probe parameters:
  --radii <comma-list>                default: 1e-6,1e-5,1e-4,1e-3,1e-2
  --branch-threshold-relative <x>     default: {DEFAULT_BRANCH_THRESHOLD_RELATIVE}
  --action-window-relative <x>        default: {DEFAULT_ACTION_WINDOW_RELATIVE}
  --random-directions <n>             default: {DEFAULT_RANDOM_DIRECTIONS}
  --seed <u64>                        default: {DEFAULT_SEED}

Outputs:
  computed-polytopes.jsonl
  local-behavior-basepoints.jsonl
  local-behavior-samples.jsonl
  local-behavior-branch-gradients.jsonl
  produce-stats.json
"
    );
}

fn parse_usize(raw: &str, flag: &str) -> usize {
    raw.parse()
        .unwrap_or_else(|_| panic!("{flag} must be a non-negative integer"))
}

fn parse_nonnegative_f64(raw: &str, flag: &str) -> f64 {
    let value: f64 = raw.parse().unwrap_or_else(|_| panic!("{flag} must be f64"));
    assert!(value >= 0.0, "{flag} must be non-negative");
    value
}

fn parse_f64_list(raw: &str, flag: &str) -> Vec<f64> {
    let values: Vec<f64> = raw
        .split(',')
        .filter(|item| !item.trim().is_empty())
        .map(|item| {
            let value: f64 = item
                .trim()
                .parse()
                .unwrap_or_else(|_| panic!("{flag} contains invalid f64 {item:?}"));
            assert!(value > 0.0, "{flag} values must be positive");
            value
        })
        .collect();
    assert!(!values.is_empty(), "{flag} must not be empty");
    values
}

fn read_jsonl<T: for<'de> Deserialize<'de>>(path: &Path) -> Vec<T> {
    let file = File::open(path).unwrap_or_else(|err| panic!("open {}: {err}", path.display()));
    let reader = BufReader::new(file);
    reader
        .lines()
        .enumerate()
        .filter_map(|(line_idx, line)| {
            let line = line.unwrap_or_else(|err| {
                panic!("read {} line {}: {err}", path.display(), line_idx + 1)
            });
            if line.trim().is_empty() {
                None
            } else {
                Some(serde_json::from_str(&line).unwrap_or_else(|err| {
                    panic!("parse {} line {}: {err}", path.display(), line_idx + 1)
                }))
            }
        })
        .collect()
}

fn select_basepoints(
    mut rows: Vec<PolytopeRow>,
    max_top_basepoints: usize,
    max_hash_basepoints: usize,
) -> Vec<SelectedBasepoint> {
    let mut selected: BTreeMap<String, SelectedBasepoint> = BTreeMap::new();

    rows.sort_by(|a, b| {
        b.sys
            .total_cmp(&a.sys)
            .then_with(|| a.poly_id.cmp(&b.poly_id))
    });
    for row in rows.iter().take(max_top_basepoints) {
        add_selection(&mut selected, row.clone(), "top_sys");
    }

    rows.sort_by(|a, b| stable_hash_key(&a.poly_id).cmp(&stable_hash_key(&b.poly_id)));
    for row in rows.iter().take(max_hash_basepoints) {
        add_selection(&mut selected, row.clone(), "hash_control");
    }

    selected.into_values().collect()
}

fn add_selection(
    selected: &mut BTreeMap<String, SelectedBasepoint>,
    row: PolytopeRow,
    bucket: &str,
) {
    selected
        .entry(row.poly_id.clone())
        .and_modify(|entry| {
            entry.selection_buckets.insert(bucket.to_string());
        })
        .or_insert_with(|| {
            let mut buckets = BTreeSet::new();
            buckets.insert(bucket.to_string());
            SelectedBasepoint {
                row,
                selection_buckets: buckets,
            }
        });
}

fn stable_hash_key(raw: &str) -> [u8; 32] {
    *blake3::hash(raw.as_bytes()).as_bytes()
}

fn compute_local_state_from_row(
    row: &PolytopeRow,
    action_window_relative: f64,
    branch_threshold_relative: f64,
) -> Result<LocalState, String> {
    let polytope = polytope_from_f64_rows(&row.dual_vertices_f64)
        .ok_or_else(|| "construct_base_polytope_failed".to_string())?;
    compute_local_state_from_polytope(
        polytope,
        row.capacity * action_window_relative,
        branch_threshold_relative,
    )
}

fn compute_local_state_from_polytope(
    polytope: SysLandscapePolytopeCache,
    action_gap: f64,
    branch_threshold_relative: f64,
) -> Result<LocalState, String> {
    let start_capacity = Instant::now();
    let (capacity, certified_capacity, backend) = capacity_auto_with_gap(&polytope, action_gap)
        .map_err(|err| format!("capacity_with_gap_failed:{err:?}"))?;
    let time_capacity_ms = start_capacity.elapsed().as_secs_f64() * 1000.0;
    let start_volume = Instant::now();
    let volume =
        exact_volume_from_incidence_as_f64(&polytope.vertices, &polytope.vertex_facet_incidence);
    let time_volume_ms = start_volume.elapsed().as_secs_f64() * 1000.0;
    if !volume.is_finite() || volume <= 0.0 {
        return Err("volume_failed".to_string());
    }
    let sys = symplectic::systolic_ratio(capacity.capacity(), volume);
    if !sys.is_finite() {
        return Err("sys_failed".to_string());
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
            systolic_ratio_gradient_a(capacity.capacity(), volume, capacity_gradient, &d_volume_da)
        })
        .collect();
    Ok(LocalState {
        polytope,
        capacity,
        certified_capacity,
        backend: backend.to_string(),
        action_gap,
        volume,
        sys,
        near_active_orbits,
        capacity_gradients: d_capacity_da,
        sys_gradients,
        time_volume_ms,
        time_capacity_ms,
    })
}

fn capacity_auto_with_gap(
    polytope: &SysLandscapePolytopeCache,
    action_gap: f64,
) -> Result<(OrbitSearchResult, CertifiedOrbitSearchResult, &'static str), OrbitSearchError> {
    let action_gap_exact = exact_action_gap_from_f64(action_gap)?;
    if let Ok(classification) = classify_facets_from_dual_vertices(&polytope.dual_vertices_f64) {
        let transition_is_allowed = build_transition_matrix_from_facet_intersections_and_omega(
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
        let certified = aggregate_certified_orbits_with_dual_vertices_exact(
            &polytope.dual_vertices,
            orbits.clone(),
            iterations,
            action_gap_exact,
            CertifiedOrbitSetMode::GapWindow,
        )?;
        let result = aggregate_orbits_with_dual_vertices_exact(
            &polytope.dual_vertices,
            orbits,
            iterations,
            action_gap.max(0.0),
            OrbitGuaranteeMode::AllSafe,
        )?;
        return Ok((result, certified, "billiard"));
    }

    let transition_is_allowed = build_transition_matrix_from_facet_intersections_and_omega(
        &polytope.facet_intersection_is_nonempty,
        &polytope.omega_signs,
    );
    let (orbits, iterations) =
        solve_pruned_hk2017_candidates(&polytope.dual_vertices_f64, &transition_is_allowed)?;
    let certified = aggregate_certified_orbits_with_dual_vertices_exact(
        &polytope.dual_vertices,
        orbits.clone(),
        iterations,
        action_gap_exact,
        CertifiedOrbitSetMode::GapWindow,
    )?;
    let result = aggregate_orbits_with_dual_vertices_exact(
        &polytope.dual_vertices,
        orbits,
        iterations,
        action_gap.max(0.0),
        OrbitGuaranteeMode::AllSafe,
    )?;
    Ok((result, certified, "auto"))
}

fn exact_action_gap_from_f64(action_gap: f64) -> Result<BigRational, OrbitSearchError> {
    if !action_gap.is_finite() || action_gap < 0.0 {
        return Err(OrbitSearchError::InvalidGap);
    }
    BigRational::from_float(action_gap).ok_or(OrbitSearchError::InvalidGap)
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

fn strict_min_branch_sigmas(state: &LocalState) -> Vec<Vec<usize>> {
    state
        .certified_capacity
        .minimizers
        .iter()
        .map(|orbit| canonical_cycle(&orbit.sigma))
        .collect()
}

fn certified_window_branch_sigmas(state: &LocalState) -> Vec<Vec<usize>> {
    state
        .certified_capacity
        .orbits
        .iter()
        .map(|orbit| canonical_cycle(&orbit.sigma))
        .collect()
}

fn generate_directions(
    base: &LocalState,
    poly_id: &str,
    random_directions: usize,
    seed: u64,
) -> Vec<DirectionSpec> {
    let mut directions = Vec::new();
    if let Some(first_gradient) = base.sys_gradients.first() {
        if let Some(direction) = normalize_direction(first_gradient) {
            directions.push(direction_spec(
                "single_near_active_gradient",
                direction.clone(),
                base,
            ));
            directions.push(direction_spec(
                "negative_single_near_active_gradient",
                direction.iter().map(|v| -*v).collect(),
                base,
            ));
        }
    }
    if base.sys_gradients.len() > 1 {
        if let Some(direction) = maximin_direction(&base.sys_gradients) {
            directions.push(direction_spec(
                "near_active_maximin_direction",
                direction,
                base,
            ));
        }
    }

    let mut rng = ChaCha8Rng::from_seed(direction_seed(seed, poly_id));
    for idx in 0..random_directions {
        if let Some(direction) = random_unit_direction(base.polytope.facet_count(), &mut rng) {
            directions.push(direction_spec(
                &format!("random_unit_direction_{idx}"),
                direction,
                base,
            ));
        }
    }
    directions
}

fn direction_spec(label: &str, direction: Vec<Vector4<f64>>, base: &LocalState) -> DirectionSpec {
    DirectionSpec {
        label: label.to_string(),
        predicted_derivative: clarke_directional_derivative_a(&base.sys_gradients, &direction).ok(),
        direction,
    }
}

fn direction_seed(seed: u64, poly_id: &str) -> [u8; 32] {
    let mut material = Vec::new();
    material.extend_from_slice(&seed.to_le_bytes());
    material.extend_from_slice(poly_id.as_bytes());
    blake3::derive_key("sys-local-behavior-random-directions", &material)
}

fn random_unit_direction(facet_count: usize, rng: &mut ChaCha8Rng) -> Option<Vec<Vector4<f64>>> {
    let direction: Vec<Vector4<f64>> = (0..facet_count)
        .map(|_| {
            Vector4::new(
                rng.sample(StandardNormal),
                rng.sample(StandardNormal),
                rng.sample(StandardNormal),
                rng.sample(StandardNormal),
            )
        })
        .collect();
    normalize_direction(&direction)
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

fn sample_row(
    basepoint_id: &str,
    basepoint: &SelectedBasepoint,
    base: &LocalState,
    direction: &DirectionSpec,
    radius_index: usize,
    radius: f64,
    args: &Args,
    payload_rows: &mut BTreeMap<String, ComputedPolytopePayloadRow>,
) -> SampleOutcome {
    let sample_id = format!(
        "{basepoint_id}:{}:r{radius_index:03}:{radius:.17e}",
        direction.label
    );
    let new_duals: Vec<Vector4<f64>> = base
        .polytope
        .dual_vertices_f64
        .iter()
        .zip(&direction.direction)
        .map(|(a, d)| a + radius * d)
        .collect();
    let predicted_delta_sys = direction.predicted_derivative.map(|value| radius * value);
    let Some(target_polytope) = SysLandscapePolytopeCache::from_f64_dual_vertices(new_duals) else {
        return SampleOutcome {
            row: failed_sample_row(
                sample_id,
                basepoint_id,
                basepoint,
                base,
                direction,
                radius,
                predicted_delta_sys,
                "construct_target_polytope_failed",
            ),
        };
    };
    let target_poly_id = poly_id(&target_polytope);
    let target_state = compute_local_state_from_polytope(
        target_polytope,
        base.capacity.capacity() * args.action_window_relative,
        args.branch_threshold_relative,
    );
    let Ok(target) = target_state else {
        return SampleOutcome {
            row: failed_sample_row(
                sample_id,
                basepoint_id,
                basepoint,
                base,
                direction,
                radius,
                predicted_delta_sys,
                "target_state_failed",
            ),
        };
    };
    insert_payload(payload_rows, payload_from_state(&target));

    let observed_delta_sys = target.sys - base.sys;
    let prediction_error = predicted_delta_sys.map(|prediction| observed_delta_sys - prediction);
    let prediction_error_abs = prediction_error.map(f64::abs);
    let prediction_ratio_observed_over_predicted = predicted_delta_sys
        .filter(|value| value.abs() > 1.0e-15)
        .map(|prediction| observed_delta_sys / prediction);

    SampleOutcome {
        row: LocalBehaviorSampleRow {
            sample_id: sample_id.clone(),
            basepoint_id: basepoint_id.to_string(),
            input_poly_id: basepoint.row.poly_id.clone(),
            base_poly_id: poly_id(&base.polytope),
            target_poly_id: Some(target_poly_id),
            selection_buckets: sorted_buckets(&basepoint.selection_buckets),
            direction_label: direction.label.clone(),
            direction_vector: vector_rows(&direction.direction),
            radius,
            status: "ok".to_string(),
            base_sys: base.sys,
            target_sys: Some(target.sys),
            observed_delta_sys: Some(observed_delta_sys),
            predicted_derivative: direction.predicted_derivative,
            predicted_delta_sys,
            prediction_error,
            prediction_error_abs,
            prediction_ratio_observed_over_predicted,
            target_orbit_iterations: Some(target.capacity.iterations),
            time_volume_ms: Some(target.time_volume_ms),
            time_capacity_ms: Some(target.time_capacity_ms),
            failure: None,
        },
    }
}

fn canonical_cycle(sigma: &[usize]) -> Vec<usize> {
    let Some((min_idx, _)) = sigma.iter().enumerate().min_by_key(|&(_, value)| value) else {
        return Vec::new();
    };
    sigma[min_idx..]
        .iter()
        .chain(&sigma[..min_idx])
        .copied()
        .collect()
}

fn failed_sample_row(
    sample_id: String,
    basepoint_id: &str,
    basepoint: &SelectedBasepoint,
    base: &LocalState,
    direction: &DirectionSpec,
    radius: f64,
    predicted_delta_sys: Option<f64>,
    failure: &str,
) -> LocalBehaviorSampleRow {
    LocalBehaviorSampleRow {
        sample_id,
        basepoint_id: basepoint_id.to_string(),
        input_poly_id: basepoint.row.poly_id.clone(),
        base_poly_id: poly_id(&base.polytope),
        target_poly_id: None,
        selection_buckets: sorted_buckets(&basepoint.selection_buckets),
        direction_label: direction.label.clone(),
        direction_vector: vector_rows(&direction.direction),
        radius,
        status: "failed".to_string(),
        base_sys: base.sys,
        target_sys: None,
        observed_delta_sys: None,
        predicted_derivative: direction.predicted_derivative,
        predicted_delta_sys,
        prediction_error: None,
        prediction_error_abs: None,
        prediction_ratio_observed_over_predicted: None,
        target_orbit_iterations: None,
        time_volume_ms: None,
        time_capacity_ms: None,
        failure: Some(failure.to_string()),
    }
}

fn basepoint_state_row(
    basepoint_id: &str,
    basepoint: &SelectedBasepoint,
    base: &LocalState,
    directions: &[DirectionSpec],
    args: &Args,
) -> BasepointStateRow {
    let admissible_actions = admissible_actions(&base.capacity);
    let strict_min_branch_sigmas = strict_min_branch_sigmas(base);
    BasepointStateRow {
        basepoint_id: basepoint_id.to_string(),
        input_poly_id: basepoint.row.poly_id.clone(),
        base_poly_id: Some(poly_id(&base.polytope)),
        selection_buckets: sorted_buckets(&basepoint.selection_buckets),
        facet_count: basepoint.row.facet_count,
        input_capacity: basepoint.row.capacity,
        input_sys: basepoint.row.sys,
        recomputed_capacity: Some(base.capacity.capacity()),
        recomputed_volume: Some(base.volume),
        recomputed_sys: Some(base.sys),
        branch_threshold_relative: args.branch_threshold_relative,
        action_window_relative: args.action_window_relative,
        returned_orbit_count: Some(base.capacity.orbits.len()),
        admissible_orbit_count: Some(admissible_actions.len()),
        near_active_count: Some(base.near_active_orbits.len()),
        min_action: Some(base.capacity.min_action),
        action_gap_to_second: admissible_actions
            .get(1)
            .map(|action| action - base.capacity.min_action),
        best_sigma: Some(base.capacity.best_sigma().to_vec()),
        strict_min_branch_count: Some(strict_min_branch_sigmas.len()),
        strict_min_branch_sigmas,
        certified_action_window_branch_count: Some(base.certified_capacity.orbits.len()),
        certified_exact_resolutions: Some(base.certified_capacity.exact_resolutions),
        near_active_sigmas: base
            .near_active_orbits
            .iter()
            .map(|orbit| orbit.sigma.clone())
            .collect(),
        candidate_window_sigmas: base
            .capacity
            .orbits
            .iter()
            .map(|orbit| orbit.sigma.clone())
            .collect(),
        orbit_iterations: Some(base.capacity.iterations),
        time_volume_ms: Some(base.time_volume_ms),
        time_capacity_ms: Some(base.time_capacity_ms),
        gradient_count: Some(base.sys_gradients.len()),
        generated_direction_count: Some(directions.len()),
        failure: None,
    }
}

fn failed_basepoint_state_row(
    basepoint_id: &str,
    basepoint: &SelectedBasepoint,
    args: &Args,
    failure: String,
) -> BasepointStateRow {
    BasepointStateRow {
        basepoint_id: basepoint_id.to_string(),
        input_poly_id: basepoint.row.poly_id.clone(),
        base_poly_id: None,
        selection_buckets: sorted_buckets(&basepoint.selection_buckets),
        facet_count: basepoint.row.facet_count,
        input_capacity: basepoint.row.capacity,
        input_sys: basepoint.row.sys,
        recomputed_capacity: None,
        recomputed_volume: None,
        recomputed_sys: None,
        branch_threshold_relative: args.branch_threshold_relative,
        action_window_relative: args.action_window_relative,
        returned_orbit_count: None,
        admissible_orbit_count: None,
        near_active_count: None,
        min_action: None,
        action_gap_to_second: None,
        best_sigma: None,
        strict_min_branch_count: None,
        strict_min_branch_sigmas: Vec::new(),
        certified_action_window_branch_count: None,
        certified_exact_resolutions: None,
        near_active_sigmas: Vec::new(),
        candidate_window_sigmas: Vec::new(),
        orbit_iterations: None,
        time_volume_ms: None,
        time_capacity_ms: None,
        gradient_count: None,
        generated_direction_count: None,
        failure: Some(failure),
    }
}

fn basepoint_branch_gradient_rows(
    basepoint_id: &str,
    basepoint: &SelectedBasepoint,
    base: &LocalState,
) -> Vec<BranchGradientRow> {
    let base_poly_id = poly_id(&base.polytope);
    base.near_active_orbits
        .iter()
        .zip(base.capacity_gradients.iter())
        .zip(base.sys_gradients.iter())
        .enumerate()
        .map(
            |(orbit_index, ((orbit, capacity_gradient), sys_sigma_gradient))| BranchGradientRow {
                basepoint_id: basepoint_id.to_string(),
                input_poly_id: basepoint.row.poly_id.clone(),
                base_poly_id: base_poly_id.clone(),
                orbit_index,
                sigma: canonical_cycle(&orbit.sigma),
                action: orbit.action,
                relative_action_gap_from_min: orbit.action / base.capacity.min_action - 1.0,
                capacity_gradient: vector_rows(capacity_gradient),
                sys_sigma_gradient: vector_rows(sys_sigma_gradient),
                gradient_norm: gradient_norm(sys_sigma_gradient),
            },
        )
        .collect()
}

fn insert_payload(
    payload_rows: &mut BTreeMap<String, ComputedPolytopePayloadRow>,
    payload: ComputedPolytopePayloadRow,
) {
    payload_rows
        .entry(payload.poly_id.clone())
        .or_insert(payload);
}

fn payload_from_state(state: &LocalState) -> ComputedPolytopePayloadRow {
    ComputedPolytopePayloadRow {
        poly_id: poly_id(&state.polytope),
        dual_vertices: f64_dual_vertices(&state.polytope),
        dual_vertices_rational: rational_vec4_to_strings(&state.polytope.dual_vertices),
        vertices_rational: rational_vec4_to_strings(&state.polytope.vertices),
        facet_count: state.polytope.facet_count(),
        backend: state.backend.clone(),
        volume: state.volume,
        capacity: state.capacity.capacity(),
        sys: state.sys,
        sigma_gap_cutoff: state.action_gap,
        sigmas: admissible_sigma_actions(&state.capacity),
        orbit_scalars: orbit_scalars_from_result(&state.capacity),
        time_volume_ms: state.time_volume_ms,
        time_capacity_ms: state.time_capacity_ms,
    }
}

fn admissible_sigma_actions(result: &OrbitSearchResult) -> Vec<SigmaAction> {
    result
        .orbits
        .iter()
        .filter(|orbit| {
            matches!(
                orbit.admissibility,
                OrbitAdmissibility::AdmissibleF64 | OrbitAdmissibility::AdmissibleExact
            )
        })
        .map(|orbit| SigmaAction {
            perm: orbit.sigma.clone(),
            action: orbit.action,
        })
        .collect()
}

fn admissible_actions(result: &OrbitSearchResult) -> Vec<f64> {
    let mut actions: Vec<f64> = result
        .orbits
        .iter()
        .filter(|orbit| {
            matches!(
                orbit.admissibility,
                OrbitAdmissibility::AdmissibleF64 | OrbitAdmissibility::AdmissibleExact
            )
        })
        .map(|orbit| orbit.action)
        .collect();
    actions.sort_by(|a, b| a.total_cmp(b));
    actions
}

fn polytope_from_f64_rows(rows: &[[f64; 4]]) -> Option<SysLandscapePolytopeCache> {
    let dual_vertices: Vec<Vector4<f64>> = rows
        .iter()
        .map(|row| Vector4::new(row[0], row[1], row[2], row[3]))
        .collect();
    SysLandscapePolytopeCache::from_f64_dual_vertices(dual_vertices)
}

fn normalize_direction(direction: &[Vector4<f64>]) -> Option<Vec<Vector4<f64>>> {
    let norm = direction
        .iter()
        .map(|v| v.norm_squared())
        .sum::<f64>()
        .sqrt();
    (norm > 0.0 && norm.is_finite()).then(|| direction.iter().map(|v| v / norm).collect())
}

fn flatten_gradient(grad: &[Vector4<f64>]) -> Vec<f64> {
    grad.iter()
        .flat_map(|vk| [vk[0], vk[1], vk[2], vk[3]])
        .collect()
}

fn gradient_norm(grad: &[Vector4<f64>]) -> f64 {
    grad.iter()
        .map(|vector| vector.norm_squared())
        .sum::<f64>()
        .sqrt()
}

fn vector_rows(vectors: &[Vector4<f64>]) -> Vec<[f64; 4]> {
    vectors
        .iter()
        .map(|vector| [vector[0], vector[1], vector[2], vector[3]])
        .collect()
}

fn unflatten_direction(flat: &[f64]) -> Vec<Vector4<f64>> {
    flat.chunks_exact(4)
        .map(|chunk| Vector4::new(chunk[0], chunk[1], chunk[2], chunk[3]))
        .collect()
}

fn sorted_buckets(buckets: &BTreeSet<String>) -> Vec<String> {
    buckets.iter().cloned().collect()
}

fn count_by(
    rows: &[LocalBehaviorSampleRow],
    key: impl Fn(&LocalBehaviorSampleRow) -> &str,
) -> BTreeMap<String, usize> {
    let mut counts = BTreeMap::new();
    for row in rows {
        *counts.entry(key(row).to_string()).or_insert(0) += 1;
    }
    counts
}

fn write_jsonl<T: Serialize>(path: PathBuf, rows: &[T]) {
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent).expect("create output parent");
        }
    }
    let file = File::create(&path).unwrap_or_else(|err| panic!("create {}: {err}", path.display()));
    let mut writer = BufWriter::new(file);
    for row in rows {
        serde_json::to_writer(&mut writer, row)
            .unwrap_or_else(|err| panic!("serialize {}: {err}", path.display()));
        writeln!(writer).unwrap_or_else(|err| panic!("write {}: {err}", path.display()));
    }
    writer
        .flush()
        .unwrap_or_else(|err| panic!("flush {}: {err}", path.display()));
}

fn write_json<T: Serialize>(path: PathBuf, value: &T) {
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent).expect("create output parent");
        }
    }
    let file = File::create(&path).unwrap_or_else(|err| panic!("create {}: {err}", path.display()));
    serde_json::to_writer_pretty(BufWriter::new(file), value)
        .unwrap_or_else(|err| panic!("write {}: {err}", path.display()));
}

fn flush_stdout() {
    std::io::stdout().flush().expect("flush stdout");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_direction_rejects_bad_inputs() {
        assert!(normalize_direction(&[]).is_none());
        assert!(normalize_direction(&[Vector4::zeros()]).is_none());
        assert!(normalize_direction(&[Vector4::new(f64::NAN, 0.0, 0.0, 0.0)]).is_none());
    }

    #[test]
    fn normalize_direction_returns_unit_direction() {
        let direction = vec![
            Vector4::new(3.0, 4.0, 0.0, 0.0),
            Vector4::new(0.0, 0.0, 12.0, 0.0),
        ];
        let normalized = normalize_direction(&direction).expect("nonzero finite direction");
        let norm = normalized
            .iter()
            .map(|v| v.norm_squared())
            .sum::<f64>()
            .sqrt();
        assert!((norm - 1.0).abs() < 1.0e-12);
    }
}

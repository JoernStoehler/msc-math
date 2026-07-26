//! Quotient-aware, derivative-free endpoint diagnostic for generic `sys(a)` states.
//!
//! The diagnostic constructs the Euclidean orthogonal complement of the
//! translation, scaling, and linear-symplectic orbit tangent at each base state,
//! then polls the signed vectors of an orthonormal quotient basis.  It evaluates
//! full `sys` at every perturbed state; no active-gradient list is assumed complete.

use exp_sys_landscape::{
    compute_sys_computation, reference::exact_volume_as_f64, SysComputation,
    SysLandscapePolytopeCache,
};
use nalgebra::{DMatrix, DVector, Matrix4, Vector4};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Instant;
use symplectic::known_polytopes::hko_pentagon;

const BASE_COMMIT: &str = "e1fbe217ed6a6b181eb80c3bc9afe97b7052632e";
const FULL_RADII: &[f64] = &[1.0e-3, 1.0e-4, 1.0e-5];
const ORTHONORMAL_TOLERANCE: f64 = 2.0e-10;

#[derive(Debug)]
struct Cli {
    out_dir: PathBuf,
    trajectory_root: PathBuf,
    smoke: bool,
    threads: usize,
}

#[derive(Clone, Debug, Deserialize)]
struct TrajectoryRow {
    eta: f64,
    iteration: usize,
    state_valid: bool,
    sys: Option<f64>,
    full_sys_delta: Option<f64>,
    best_sys: f64,
    best_iteration: usize,
    dual_vertices_after: Vec<[f64; 4]>,
}

#[derive(Clone, Debug)]
struct LocatedRow {
    path: PathBuf,
    row: TrajectoryRow,
}

#[derive(Clone, Debug)]
struct DiagnosticState {
    state_id: String,
    control_role: String,
    selection_rule: String,
    source_path: String,
    source_iteration: Option<usize>,
    source_eta: Option<f64>,
    recorded_sys: Option<f64>,
    recorded_next_full_sys_delta: Option<f64>,
    dual_vertices: Vec<Vector4<f64>>,
}

#[derive(Clone, Debug)]
struct QuotientBasis {
    orbit_basis: Vec<DVector<f64>>,
    slice_basis: Vec<DVector<f64>>,
    orbit_generator_count: usize,
    max_orbit_orthonormal_error: f64,
    max_slice_orthonormal_error: f64,
    max_cross_inner_product: f64,
}

#[derive(Clone, Debug, Serialize)]
struct StateRow {
    state_id: String,
    control_role: String,
    selection_rule: String,
    source_path: String,
    source_iteration: Option<usize>,
    source_eta: Option<f64>,
    recorded_sys: Option<f64>,
    recomputed_sys: f64,
    recomputed_minus_recorded: Option<f64>,
    recorded_next_full_sys_delta: Option<f64>,
    facet_count: usize,
    vertex_count: usize,
    base_incidence_signature: Vec<String>,
    dual_norm: f64,
    base_volume: f64,
    base_min_action: f64,
    base_min_action_lower: f64,
    base_min_action_upper: f64,
    base_orbit_count: usize,
    base_orbit_iterations: u64,
    base_best_sigma: Vec<usize>,
    ambient_dimension: usize,
    orbit_generator_count: usize,
    orbit_rank: usize,
    quotient_dimension: usize,
    max_orbit_orthonormal_error: f64,
    max_slice_orthonormal_error: f64,
    max_orbit_slice_inner_product: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct PollRow {
    state_id: String,
    control_role: String,
    relative_radius: f64,
    absolute_radius: f64,
    basis_index: usize,
    sign: i8,
    direction: Vec<[f64; 4]>,
    direction_norm: f64,
    orbit_projection_norm: f64,
    step_norm: f64,
    base_sys: f64,
    perturbed_sys: Option<f64>,
    delta_sys: Option<f64>,
    delta_sys_per_step: Option<f64>,
    state_valid: bool,
    failure: Option<String>,
    facet_count: Option<usize>,
    vertex_count: Option<usize>,
    all_facets_defining: bool,
    same_incidence_signature: bool,
    volume: Option<f64>,
    min_action: Option<f64>,
    min_action_lower: Option<f64>,
    min_action_upper: Option<f64>,
    returned_orbit_count: Option<usize>,
    orbit_iterations: Option<u64>,
    best_sigma: Option<Vec<usize>>,
    wall_seconds: f64,
}

#[derive(Clone, Debug, Serialize)]
struct RadiusSummary {
    state_id: String,
    control_role: String,
    relative_radius: f64,
    expected_direction_count: usize,
    valid_direction_count: usize,
    invalid_direction_count: usize,
    improving_direction_count: usize,
    combinatorial_change_count: usize,
    max_delta_sys: Option<f64>,
    min_delta_sys: Option<f64>,
    max_delta_sys_per_step: Option<f64>,
    best_basis_index: Option<usize>,
    best_sign: Option<i8>,
    finite_poll_status: String,
}

#[derive(Debug, Serialize)]
struct RunSummary {
    diagnostic: String,
    smoke: bool,
    radii: Vec<f64>,
    state_count: usize,
    poll_row_count: usize,
    wall_seconds: f64,
    finite_claim_boundary: String,
    states: Vec<StateRow>,
    radius_summaries: Vec<RadiusSummary>,
}

#[derive(Debug, Serialize)]
struct InputIdentity {
    path: String,
    blake3: String,
}

#[derive(Debug, Serialize)]
struct RunProvenance {
    command: Vec<String>,
    source_repo_head: Option<String>,
    required_base_commit: String,
    implementation_path: String,
    implementation_blake3: String,
    analyzer_path: String,
    analyzer_blake3: String,
    manifest_path: String,
    manifest_blake3: String,
    input_identities: Vec<InputIdentity>,
    selection_contract: Vec<String>,
    quotient_contract: String,
    radius_contract: String,
    sys_contract: String,
    threads: usize,
    smoke: bool,
}

fn main() {
    let cli = parse_args(std::env::args().skip(1));
    fs::create_dir_all(&cli.out_dir).expect("create output directory");
    let all_rows = load_all_trajectory_rows(&cli.trajectory_root);
    let states = select_states(&cli, &all_rows);
    let radii = if cli.smoke {
        vec![1.0e-4]
    } else {
        FULL_RADII.to_vec()
    };
    let selection_inputs = trajectory_input_paths(&all_rows);
    write_provenance(&cli, &selection_inputs);

    let started = Instant::now();
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(cli.threads)
        .build()
        .expect("build diagnostic thread pool");
    let mut state_rows = Vec::new();
    let mut all_poll_rows = Vec::new();

    for state in &states {
        let (base_polytope, base_computation) = compute_state(&state.dual_vertices)
            .unwrap_or_else(|failure| panic!("base state {} failed: {failure}", state.state_id));
        let quotient = quotient_basis(&state.dual_vertices);
        validate_quotient_basis(&quotient, state.dual_vertices.len());
        let state_row = make_state_row(state, &base_polytope, &base_computation, &quotient);
        let base_signature = state_row.base_incidence_signature.clone();
        let dual_norm = state_row.dual_norm;
        let mut directions: Vec<(usize, i8, DVector<f64>)> = quotient
            .slice_basis
            .iter()
            .enumerate()
            .flat_map(|(basis_index, direction)| {
                [
                    (basis_index, 1, direction.clone()),
                    (basis_index, -1, -direction),
                ]
            })
            .collect();
        if cli.smoke {
            directions.truncate(2);
        }
        let jobs: Vec<(f64, usize, i8, DVector<f64>)> = radii
            .iter()
            .copied()
            .flat_map(|radius| {
                directions
                    .iter()
                    .cloned()
                    .map(move |(index, sign, direction)| (radius, index, sign, direction))
            })
            .collect();
        let orbit_basis = quotient.orbit_basis.clone();
        let mut rows = pool.install(|| {
            jobs.into_par_iter()
                .map(|(radius, basis_index, sign, direction)| {
                    poll_direction(
                        state,
                        state_row.recomputed_sys,
                        &base_signature,
                        dual_norm,
                        radius,
                        basis_index,
                        sign,
                        &direction,
                        &orbit_basis,
                    )
                })
                .collect::<Vec<_>>()
        });
        rows.sort_by(|a, b| {
            a.relative_radius
                .total_cmp(&b.relative_radius)
                .reverse()
                .then_with(|| a.basis_index.cmp(&b.basis_index))
                .then_with(|| b.sign.cmp(&a.sign))
        });
        state_rows.push(state_row);
        all_poll_rows.extend(rows);
    }

    let radius_summaries = summarize_poll_rows(&state_rows, &all_poll_rows, &radii);
    write_jsonl(cli.out_dir.join("states.jsonl"), &state_rows);
    write_jsonl(cli.out_dir.join("poll-directions.jsonl"), &all_poll_rows);
    write_jsonl(
        cli.out_dir.join("radius-summaries.jsonl"),
        &radius_summaries,
    );
    let summary = RunSummary {
        diagnostic: "signed orthonormal-basis poll on the Euclidean complement of the 15-dimensional sys-symmetry tangent space".to_string(),
        smoke: cli.smoke,
        radii,
        state_count: state_rows.len(),
        poll_row_count: all_poll_rows.len(),
        wall_seconds: started.elapsed().as_secs_f64(),
        finite_claim_boundary: "A no-positive result means only that no signed basis direction improved full recomputed sys at the retained radii. It is not a proof, a complete branch-germ test, or coverage of arbitrary quotient directions.".to_string(),
        states: state_rows,
        radius_summaries,
    };
    write_json(cli.out_dir.join("summary.json"), &summary);
}

fn select_states(cli: &Cli, all_rows: &[LocatedRow]) -> Vec<DiagnosticState> {
    let negative_initial_path = cli
        .trajectory_root
        .join("random_F6_s0_0/trajectory-eta-1e-4.jsonl");
    let negative_mid_path = cli
        .trajectory_root
        .join("random_F6_s0_2/trajectory-eta-1e-3.jsonl");
    let negative_initial = row_at(&negative_initial_path, 0);
    let negative_initial_next = row_at(&negative_initial_path, 1);
    let negative_mid = row_at(&negative_mid_path, 20);
    let negative_mid_next = row_at(&negative_mid_path, 21);
    assert_positive_update_witness(&negative_initial, &negative_initial_next);
    assert_positive_update_witness(&negative_mid, &negative_mid_next);

    let mut states = vec![
        trajectory_state(
            "negative_control_initial",
            "negative_control_known_later_literal_improvement",
            "fixed ordinary initial state random_F6_s0_0 at eta=1e-4; its retained next literal update is positive",
            negative_initial,
            Some(negative_initial_next.row.full_sys_delta.unwrap()),
        ),
        trajectory_state(
            "negative_control_midtrajectory",
            "negative_control_known_later_literal_improvement",
            "fixed ordinary state random_F6_s0_2 at eta=1e-3 iteration 20; its retained next literal update is positive",
            negative_mid,
            Some(negative_mid_next.row.full_sys_delta.unwrap()),
        ),
    ];

    if cli.smoke {
        states.truncate(1);
    } else {
        let global_best = all_rows
            .iter()
            .filter(|located| located.row.state_valid && located.row.sys.is_some())
            .max_by(|a, b| a.row.sys.unwrap().total_cmp(&b.row.sys.unwrap()))
            .expect("at least one valid trajectory row")
            .clone();
        let terminal_best = all_rows
            .iter()
            .filter(|located| {
                located.row.state_valid
                    && located.row.iteration == 100
                    && located.row.best_iteration == 100
                    && located.row.sys.is_some()
            })
            .max_by(|a, b| a.row.sys.unwrap().total_cmp(&b.row.sys.unwrap()))
            .expect("at least one terminal-best trajectory row")
            .clone();
        states.push(trajectory_state(
            "unknown_global_best_so_far",
            "unknown_frozen_high_best_so_far",
            "highest valid full-sys row across all frozen six-start/six-rate trajectories; selected before endpoint-poll outcomes",
            global_best,
            None,
        ));
        states.push(trajectory_state(
            "unknown_terminal_best_so_far",
            "unknown_frozen_terminal_best_so_far",
            "highest iteration-100 row among complete frozen trajectories whose retained best occurs at iteration 100; selected before endpoint-poll outcomes",
            terminal_best,
            None,
        ));
    }

    let hko = hko_pentagon();
    let hko_volume = exact_volume_as_f64(&hko.vertices, &hko.vertex_facet_incidence);
    states.push(DiagnosticState {
        state_id: "positive_control_hko2024".to_string(),
        control_role: "positive_control_exact_theorem_local_maximum".to_string(),
        selection_rule: "fixed HKO2024 ten-facet theorem control; theorem authority is experiments/hko-local-maximum/theorem, not this diagnostic".to_string(),
        source_path: "symplectic::known_polytopes::hko_pentagon".to_string(),
        source_iteration: None,
        source_eta: None,
        recorded_sys: Some(symplectic::systolic_ratio(hko.capacity, hko_volume)),
        recorded_next_full_sys_delta: None,
        dual_vertices: hko.dual_vertices_f64.clone(),
    });
    states
}

fn trajectory_state(
    state_id: &str,
    role: &str,
    selection_rule: &str,
    located: LocatedRow,
    next_delta: Option<f64>,
) -> DiagnosticState {
    assert!(located.row.state_valid, "selected row must be valid");
    let dual_vertices = located
        .row
        .dual_vertices_after
        .iter()
        .map(array_to_vector)
        .collect();
    DiagnosticState {
        state_id: state_id.to_string(),
        control_role: role.to_string(),
        selection_rule: selection_rule.to_string(),
        source_path: located.path.display().to_string(),
        source_iteration: Some(located.row.iteration),
        source_eta: Some(located.row.eta),
        recorded_sys: located.row.sys,
        recorded_next_full_sys_delta: next_delta,
        dual_vertices,
    }
}

fn assert_positive_update_witness(base: &LocatedRow, next: &LocatedRow) {
    assert_eq!(next.row.iteration, base.row.iteration + 1);
    assert!(next.row.state_valid);
    let delta = next
        .row
        .full_sys_delta
        .expect("negative-control next row must record delta");
    assert!(
        delta > 0.0,
        "negative control must have positive later update"
    );
    let recomputed_delta = next.row.sys.unwrap() - base.row.sys.unwrap();
    assert!((delta - recomputed_delta).abs() <= 2.0e-14_f64.max(2.0e-12 * delta.abs()));
}

fn quotient_basis(duals: &[Vector4<f64>]) -> QuotientBasis {
    let generators = symmetry_generators(duals);
    let orbit_basis = orthonormalize(&generators, 1.0e-11);
    let ambient_dimension = duals.len() * 4;
    let mut slice_basis = Vec::with_capacity(ambient_dimension - orbit_basis.len());
    for coordinate in 0..ambient_dimension {
        let mut candidate = DVector::zeros(ambient_dimension);
        candidate[coordinate] = 1.0;
        project_away(&mut candidate, &orbit_basis);
        project_away(&mut candidate, &slice_basis);
        // A second pass suppresses modified-Gram-Schmidt loss at nearly aligned axes.
        project_away(&mut candidate, &orbit_basis);
        project_away(&mut candidate, &slice_basis);
        let norm = candidate.norm();
        if norm > 1.0e-10 {
            slice_basis.push(candidate / norm);
        }
    }
    QuotientBasis {
        max_orbit_orthonormal_error: max_orthonormal_error(&orbit_basis),
        max_slice_orthonormal_error: max_orthonormal_error(&slice_basis),
        max_cross_inner_product: max_cross_inner_product(&orbit_basis, &slice_basis),
        orbit_basis,
        slice_basis,
        orbit_generator_count: generators.len(),
    }
}

fn symmetry_generators(duals: &[Vector4<f64>]) -> Vec<DVector<f64>> {
    let mut generators = Vec::with_capacity(15);
    // Translation y: a_i -> a_i/(1 + <a_i,y>), so da_i = -<a_i,y>a_i.
    for coordinate in 0..4 {
        generators.push(flatten_vectors(
            &duals.iter().map(|a| -a[coordinate] * a).collect::<Vec<_>>(),
        ));
    }
    // Positive scaling rho: a_i -> rho^{-1} a_i.
    generators.push(flatten_vectors(
        &duals.iter().map(|a| -a).collect::<Vec<_>>(),
    ));
    // X in sp(4): X = [[A,B],[C,-A^T]], with B and C symmetric.
    for x in sp4_basis() {
        generators.push(flatten_vectors(
            &duals.iter().map(|a| -x.transpose() * a).collect::<Vec<_>>(),
        ));
    }
    generators
}

fn sp4_basis() -> Vec<Matrix4<f64>> {
    let mut result = Vec::with_capacity(10);
    for row in 0..2 {
        for col in 0..2 {
            let mut x = Matrix4::zeros();
            x[(row, col)] = 1.0;
            x[(2 + col, 2 + row)] = -1.0;
            result.push(x);
        }
    }
    for &(row, col) in &[(0, 0), (0, 1), (1, 1)] {
        let mut x = Matrix4::zeros();
        x[(row, 2 + col)] = 1.0;
        x[(col, 2 + row)] = 1.0;
        if row == col {
            x[(row, 2 + col)] = 1.0;
        }
        result.push(x);
    }
    for &(row, col) in &[(0, 0), (0, 1), (1, 1)] {
        let mut x = Matrix4::zeros();
        x[(2 + row, col)] = 1.0;
        x[(2 + col, row)] = 1.0;
        if row == col {
            x[(2 + row, col)] = 1.0;
        }
        result.push(x);
    }
    result
}

fn orthonormalize(vectors: &[DVector<f64>], tolerance: f64) -> Vec<DVector<f64>> {
    let mut basis = Vec::new();
    for source in vectors {
        let mut candidate = source.clone();
        project_away(&mut candidate, &basis);
        project_away(&mut candidate, &basis);
        let norm = candidate.norm();
        if norm > tolerance * source.norm().max(1.0) {
            basis.push(candidate / norm);
        }
    }
    basis
}

fn project_away(vector: &mut DVector<f64>, basis: &[DVector<f64>]) {
    for axis in basis {
        *vector -= axis * axis.dot(vector);
    }
}

fn validate_quotient_basis(quotient: &QuotientBasis, facet_count: usize) {
    assert_eq!(quotient.orbit_generator_count, 15);
    assert_eq!(
        quotient.orbit_basis.len() + quotient.slice_basis.len(),
        4 * facet_count
    );
    assert!(quotient.max_orbit_orthonormal_error <= ORTHONORMAL_TOLERANCE);
    assert!(quotient.max_slice_orthonormal_error <= ORTHONORMAL_TOLERANCE);
    assert!(quotient.max_cross_inner_product <= ORTHONORMAL_TOLERANCE);
}

fn make_state_row(
    state: &DiagnosticState,
    polytope: &SysLandscapePolytopeCache,
    computation: &SysComputation,
    quotient: &QuotientBasis,
) -> StateRow {
    let recorded_sys = state.recorded_sys;
    StateRow {
        state_id: state.state_id.clone(),
        control_role: state.control_role.clone(),
        selection_rule: state.selection_rule.clone(),
        source_path: state.source_path.clone(),
        source_iteration: state.source_iteration,
        source_eta: state.source_eta,
        recorded_sys,
        recomputed_sys: computation.sys,
        recomputed_minus_recorded: recorded_sys.map(|value| computation.sys - value),
        recorded_next_full_sys_delta: state.recorded_next_full_sys_delta,
        facet_count: polytope.facet_count(),
        vertex_count: polytope.vertices.len(),
        base_incidence_signature: incidence_signature(&polytope.vertex_facet_incidence),
        dual_norm: l2_norm(&state.dual_vertices),
        base_volume: computation.vol,
        base_min_action: computation.capacity.min_action,
        base_min_action_lower: computation.capacity.min_action_lower,
        base_min_action_upper: computation.capacity.min_action_upper,
        base_orbit_count: computation.capacity.orbits.len(),
        base_orbit_iterations: computation.capacity.iterations,
        base_best_sigma: computation.capacity.best_orbit().sigma.clone(),
        ambient_dimension: state.dual_vertices.len() * 4,
        orbit_generator_count: quotient.orbit_generator_count,
        orbit_rank: quotient.orbit_basis.len(),
        quotient_dimension: quotient.slice_basis.len(),
        max_orbit_orthonormal_error: quotient.max_orbit_orthonormal_error,
        max_slice_orthonormal_error: quotient.max_slice_orthonormal_error,
        max_orbit_slice_inner_product: quotient.max_cross_inner_product,
    }
}

#[allow(clippy::too_many_arguments)]
fn poll_direction(
    state: &DiagnosticState,
    base_sys: f64,
    base_signature: &[String],
    dual_norm: f64,
    relative_radius: f64,
    basis_index: usize,
    sign: i8,
    direction: &DVector<f64>,
    orbit_basis: &[DVector<f64>],
) -> PollRow {
    let started = Instant::now();
    let absolute_radius = relative_radius * dual_norm;
    let direction_vectors = unflatten_vector(direction);
    let perturbed: Vec<Vector4<f64>> = state
        .dual_vertices
        .iter()
        .zip(&direction_vectors)
        .map(|(a, d)| a + absolute_radius * d)
        .collect();
    let direction_norm = direction.norm();
    let orbit_projection_norm = projection_norm(direction, orbit_basis);
    match compute_state(&perturbed) {
        Ok((polytope, computation)) => {
            let signature = incidence_signature(&polytope.vertex_facet_incidence);
            let delta = computation.sys - base_sys;
            PollRow {
                state_id: state.state_id.clone(),
                control_role: state.control_role.clone(),
                relative_radius,
                absolute_radius,
                basis_index,
                sign,
                direction: vectors_to_arrays(&direction_vectors),
                direction_norm,
                orbit_projection_norm,
                step_norm: absolute_radius * direction_norm,
                base_sys,
                perturbed_sys: Some(computation.sys),
                delta_sys: Some(delta),
                delta_sys_per_step: Some(delta / (absolute_radius * direction_norm)),
                state_valid: true,
                failure: None,
                facet_count: Some(polytope.facet_count()),
                vertex_count: Some(polytope.vertices.len()),
                all_facets_defining: polytope.facet_count() == state.dual_vertices.len(),
                same_incidence_signature: signature == base_signature,
                volume: Some(computation.vol),
                min_action: Some(computation.capacity.min_action),
                min_action_lower: Some(computation.capacity.min_action_lower),
                min_action_upper: Some(computation.capacity.min_action_upper),
                returned_orbit_count: Some(computation.capacity.orbits.len()),
                orbit_iterations: Some(computation.capacity.iterations),
                best_sigma: Some(computation.capacity.best_orbit().sigma.clone()),
                wall_seconds: started.elapsed().as_secs_f64(),
            }
        }
        Err(failure) => PollRow {
            state_id: state.state_id.clone(),
            control_role: state.control_role.clone(),
            relative_radius,
            absolute_radius,
            basis_index,
            sign,
            direction: vectors_to_arrays(&direction_vectors),
            direction_norm,
            orbit_projection_norm,
            step_norm: absolute_radius * direction_norm,
            base_sys,
            perturbed_sys: None,
            delta_sys: None,
            delta_sys_per_step: None,
            state_valid: false,
            failure: Some(failure),
            facet_count: None,
            vertex_count: None,
            all_facets_defining: false,
            same_incidence_signature: false,
            volume: None,
            min_action: None,
            min_action_lower: None,
            min_action_upper: None,
            returned_orbit_count: None,
            orbit_iterations: None,
            best_sigma: None,
            wall_seconds: started.elapsed().as_secs_f64(),
        },
    }
}

fn compute_state(
    dual_vertices: &[Vector4<f64>],
) -> Result<(SysLandscapePolytopeCache, SysComputation), String> {
    let polytope = SysLandscapePolytopeCache::from_f64_dual_vertices(dual_vertices.to_vec())
        .ok_or_else(|| "invalid_geometry_or_redundant_facet".to_string())?;
    let computation = compute_sys_computation(&polytope)
        .ok_or_else(|| "full_sys_computation_failed".to_string())?;
    Ok((polytope, computation))
}

fn summarize_poll_rows(states: &[StateRow], rows: &[PollRow], radii: &[f64]) -> Vec<RadiusSummary> {
    let mut result = Vec::new();
    for state in states {
        for &radius in radii {
            let selected: Vec<&PollRow> = rows
                .iter()
                .filter(|row| row.state_id == state.state_id && row.relative_radius == radius)
                .collect();
            let valid: Vec<&PollRow> = selected
                .iter()
                .copied()
                .filter(|row| row.state_valid)
                .collect();
            let best = valid.iter().copied().max_by(|a, b| {
                a.delta_sys
                    .unwrap()
                    .total_cmp(&b.delta_sys.unwrap())
                    .then_with(|| b.basis_index.cmp(&a.basis_index))
            });
            let max_delta = best.and_then(|row| row.delta_sys);
            let status = if valid.len() != selected.len() {
                "incomplete_invalid_poll"
            } else if valid.iter().any(|row| !row.same_incidence_signature) {
                "incomplete_combinatorial_change"
            } else if max_delta.is_some_and(|delta| delta > 0.0) {
                "positive_observed"
            } else {
                "no_positive_observed"
            };
            result.push(RadiusSummary {
                state_id: state.state_id.clone(),
                control_role: state.control_role.clone(),
                relative_radius: radius,
                expected_direction_count: selected.len(),
                valid_direction_count: valid.len(),
                invalid_direction_count: selected.len() - valid.len(),
                improving_direction_count: valid
                    .iter()
                    .filter(|row| row.delta_sys.unwrap() > 0.0)
                    .count(),
                combinatorial_change_count: valid
                    .iter()
                    .filter(|row| !row.same_incidence_signature)
                    .count(),
                max_delta_sys: max_delta,
                min_delta_sys: valid
                    .iter()
                    .map(|row| row.delta_sys.unwrap())
                    .min_by(f64::total_cmp),
                max_delta_sys_per_step: valid
                    .iter()
                    .map(|row| row.delta_sys_per_step.unwrap())
                    .max_by(f64::total_cmp),
                best_basis_index: best.map(|row| row.basis_index),
                best_sign: best.map(|row| row.sign),
                finite_poll_status: status.to_string(),
            });
        }
    }
    result
}

fn load_all_trajectory_rows(root: &Path) -> Vec<LocatedRow> {
    let mut paths = Vec::new();
    for directory in fs::read_dir(root).expect("read trajectory root") {
        let directory = directory.expect("read trajectory directory entry").path();
        if !directory.is_dir() {
            continue;
        }
        for entry in fs::read_dir(&directory).expect("read trajectory start directory") {
            let path = entry.expect("read trajectory file entry").path();
            if path.extension().and_then(|value| value.to_str()) == Some("jsonl") {
                paths.push(path);
            }
        }
    }
    paths.sort();
    let mut result = Vec::new();
    for path in paths {
        for row in load_jsonl::<TrajectoryRow>(&path) {
            result.push(LocatedRow {
                path: path.clone(),
                row,
            });
        }
    }
    assert_eq!(result.len(), 3142, "frozen trajectory row count changed");
    result
}

fn row_at(path: &Path, iteration: usize) -> LocatedRow {
    let rows = load_jsonl::<TrajectoryRow>(path);
    let row = rows
        .into_iter()
        .find(|row| row.iteration == iteration)
        .unwrap_or_else(|| panic!("missing iteration {iteration} in {}", path.display()));
    LocatedRow {
        path: path.to_path_buf(),
        row,
    }
}

fn trajectory_input_paths(rows: &[LocatedRow]) -> Vec<PathBuf> {
    let mut paths: Vec<PathBuf> = rows.iter().map(|located| located.path.clone()).collect();
    paths.sort();
    paths.dedup();
    paths
}

fn write_provenance(cli: &Cli, input_paths: &[PathBuf]) {
    let implementation_path =
        PathBuf::from("experiments/dev-gradient-ascent/quotient-endpoint-diagnostic/main.rs");
    let analyzer_path =
        PathBuf::from("experiments/dev-gradient-ascent/quotient-endpoint-diagnostic/analyze.py");
    let manifest_path = PathBuf::from("experiments/dev-gradient-ascent/Cargo.toml");
    let provenance = RunProvenance {
        command: std::env::args().collect(),
        source_repo_head: git_output(&["rev-parse", "HEAD"]),
        required_base_commit: BASE_COMMIT.to_string(),
        implementation_path: implementation_path.display().to_string(),
        implementation_blake3: hash_file(&implementation_path),
        analyzer_path: analyzer_path.display().to_string(),
        analyzer_blake3: hash_file(&analyzer_path),
        manifest_path: manifest_path.display().to_string(),
        manifest_blake3: hash_file(&manifest_path),
        input_identities: input_paths
            .iter()
            .map(|path| InputIdentity {
                path: path.display().to_string(),
                blake3: hash_file(path),
            })
            .collect(),
        selection_contract: vec![
            "negative controls are two fixed retained states with a verified positive next literal update".to_string(),
            "unknown_global_best_so_far is the highest valid sys row among all 3142 frozen trajectory rows".to_string(),
            "unknown_terminal_best_so_far is the highest valid iteration-100 row whose best_iteration is 100".to_string(),
            "HKO2024 is a fixed theorem-authorized positive control and is not selected from generic outcomes".to_string(),
        ],
        quotient_contract: "at each base a, use the Euclidean orthogonal complement of the rank-15 tangent span of translations, positive scaling, and sp(4,R); poll every signed orthonormal basis vector".to_string(),
        radius_contract: if cli.smoke {
            "smoke: relative radius 1e-4; absolute step norm = relative radius * ||a||_2".to_string()
        } else {
            "full: relative radii 1e-3,1e-4,1e-5; absolute step norm = relative radius * ||a||_2".to_string()
        },
        sys_contract: "reconstruct every f64 state as exact rationals, require all dual points extreme, recompute exact incidence/volume, and use the current minimum-safe full capacity route; compare incidence signatures to the base".to_string(),
        threads: cli.threads,
        smoke: cli.smoke,
    };
    write_json(cli.out_dir.join("run-provenance.json"), &provenance);
}

fn incidence_signature(matrix: &DMatrix<bool>) -> Vec<String> {
    let mut rows = (0..matrix.nrows())
        .map(|row| {
            (0..matrix.ncols())
                .filter(|&column| matrix[(row, column)])
                .map(|column| column.to_string())
                .collect::<Vec<_>>()
                .join(",")
        })
        .collect::<Vec<_>>();
    rows.sort();
    rows
}

fn max_orthonormal_error(basis: &[DVector<f64>]) -> f64 {
    let mut max_error: f64 = 0.0;
    for (i, left) in basis.iter().enumerate() {
        for (j, right) in basis.iter().enumerate() {
            let expected = if i == j { 1.0 } else { 0.0 };
            max_error = max_error.max((left.dot(right) - expected).abs());
        }
    }
    max_error
}

fn max_cross_inner_product(left: &[DVector<f64>], right: &[DVector<f64>]) -> f64 {
    left.iter()
        .flat_map(|a| right.iter().map(move |b| a.dot(b).abs()))
        .fold(0.0, f64::max)
}

fn projection_norm(vector: &DVector<f64>, basis: &[DVector<f64>]) -> f64 {
    basis
        .iter()
        .map(|axis| axis.dot(vector).powi(2))
        .sum::<f64>()
        .sqrt()
}

fn flatten_vectors(vectors: &[Vector4<f64>]) -> DVector<f64> {
    DVector::from_iterator(
        vectors.len() * 4,
        vectors.iter().flat_map(|vector| vector.iter().copied()),
    )
}

fn unflatten_vector(vector: &DVector<f64>) -> Vec<Vector4<f64>> {
    vector
        .as_slice()
        .chunks_exact(4)
        .map(|chunk| Vector4::new(chunk[0], chunk[1], chunk[2], chunk[3]))
        .collect()
}

fn vectors_to_arrays(vectors: &[Vector4<f64>]) -> Vec<[f64; 4]> {
    vectors
        .iter()
        .map(|vector| [vector[0], vector[1], vector[2], vector[3]])
        .collect()
}

fn array_to_vector(values: &[f64; 4]) -> Vector4<f64> {
    Vector4::new(values[0], values[1], values[2], values[3])
}

fn l2_norm(vectors: &[Vector4<f64>]) -> f64 {
    vectors
        .iter()
        .flat_map(|vector| vector.iter())
        .map(|value| value * value)
        .sum::<f64>()
        .sqrt()
}

fn load_jsonl<T: for<'de> Deserialize<'de>>(path: &Path) -> Vec<T> {
    BufReader::new(File::open(path).unwrap_or_else(|err| {
        panic!(
            "open {}: {err}; canonical raw trajectories must be checked out from Git LFS",
            path.display()
        )
    }))
    .lines()
    .map(|line| serde_json::from_str(&line.expect("read JSONL line")).expect("parse JSONL row"))
    .collect()
}

fn write_jsonl<T: Serialize>(path: PathBuf, rows: &[T]) {
    let mut writer = BufWriter::new(File::create(&path).expect("create JSONL artifact"));
    for row in rows {
        serde_json::to_writer(&mut writer, row).expect("serialize JSONL row");
        writeln!(writer).expect("write JSONL newline");
    }
    writer.flush().expect("flush JSONL artifact");
}

fn write_json<T: Serialize>(path: PathBuf, value: &T) {
    let mut writer = BufWriter::new(File::create(&path).expect("create JSON artifact"));
    serde_json::to_writer_pretty(&mut writer, value).expect("serialize JSON artifact");
    writeln!(writer).expect("write JSON newline");
    writer.flush().expect("flush JSON artifact");
}

fn hash_file(path: &Path) -> String {
    let bytes = fs::read(path).unwrap_or_else(|err| panic!("read {}: {err}", path.display()));
    blake3::hash(&bytes).to_hex().to_string()
}

fn git_output(args: &[&str]) -> Option<String> {
    let output = Command::new("git").args(args).output().ok()?;
    output
        .status
        .success()
        .then(|| String::from_utf8_lossy(&output.stdout).trim().to_string())
}

fn parse_args(args: impl Iterator<Item = String>) -> Cli {
    let mut out_dir = None;
    let mut trajectory_root = None;
    let mut smoke = false;
    let mut threads = 8usize;
    let mut args = args.peekable();
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--out-dir" => out_dir = Some(PathBuf::from(args.next().expect("--out-dir value"))),
            "--trajectory-root" => {
                trajectory_root = Some(PathBuf::from(args.next().expect("--trajectory-root value")))
            }
            "--smoke" => smoke = true,
            "--threads" => {
                threads = args
                    .next()
                    .expect("--threads value")
                    .parse()
                    .expect("--threads integer")
            }
            "-h" | "--help" => usage_and_exit(),
            _ => panic!("unknown argument: {arg}"),
        }
    }
    assert!(threads > 0, "--threads must be positive");
    Cli {
        out_dir: out_dir.expect("--out-dir is required"),
        trajectory_root: trajectory_root.unwrap_or_else(|| {
            PathBuf::from(
                "experiments/dev-gradient-ascent/literal-naive-gradient/artifacts/evaluation/trajectories",
            )
        }),
        smoke,
        threads,
    }
}

fn usage_and_exit() -> ! {
    eprintln!(
        "Usage: dev-gradient-ascent-quotient-endpoint-diagnostic --out-dir PATH [--trajectory-root PATH] [--smoke] [--threads N]"
    );
    std::process::exit(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sp4_basis_satisfies_lie_algebra_equation() {
        let mut j = Matrix4::zeros();
        j[(0, 2)] = 1.0;
        j[(1, 3)] = 1.0;
        j[(2, 0)] = -1.0;
        j[(3, 1)] = -1.0;
        let basis = sp4_basis();
        assert_eq!(basis.len(), 10);
        for x in basis {
            assert!((x.transpose() * j + j * x).norm() <= 1.0e-14);
        }
    }

    #[test]
    fn generic_six_facet_fixture_has_expected_quotient_dimension() {
        let duals = symplectic::known_polytopes::lagrangian_triangle_product()
            .dual_vertices_f64
            .clone();
        let quotient = quotient_basis(&duals);
        validate_quotient_basis(&quotient, duals.len());
        assert_eq!(quotient.slice_basis.len(), 9);
    }

    #[test]
    fn hko_has_source_backed_quotient_dimension() {
        let duals = hko_pentagon().dual_vertices_f64.clone();
        let quotient = quotient_basis(&duals);
        validate_quotient_basis(&quotient, duals.len());
        assert_eq!(quotient.slice_basis.len(), 25);
    }
}

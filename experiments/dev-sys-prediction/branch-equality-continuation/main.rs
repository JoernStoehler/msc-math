//! Local projection sampler for equality sets of two fixed KKT branch actions.
//!
//! The first control uses the twenty tied minimizing words of
//! `P_5 x_L R(9 degrees) P_5` in a five-dimensional local product slice.

use exp_regular_products::{exact_volume_reference_as_f64, ProductPolytopeCache};
use nalgebra::{Vector2, Vector4};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use rand_distr::StandardNormal;
use serde::Serialize;
use serde_json::json;
use std::env;
use std::fs::{create_dir_all, File};
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use std::time::Instant;
use symplectic::derivatives::capacity_derivatives_a_from_kkt_result;
use symplectic::geom::polygon::{regular_polygon_2d, rotate_polygon_2d};
use symplectic::kkt::saddle_point_solver::{solve_kkt_for_dual_vertices, KktOutcome};
use symplectic::{
    aggregate_orbits_with_dual_vertices_exact, classify_facets_from_dual_vertices,
    solve_billiard_candidates, OrbitGuaranteeMode, OrbitSearchError, OrbitSearchResult,
};

const PARAM_DIM: usize = 11;
const EXPECTED_NUISANCE_DIM: usize = 6;
const EXPECTED_SLICE_DIM: usize = 5;
const BASE_THETA_DEG: f64 = 9.0;
const BASIS_TOL: f64 = 1.0e-11;
const TRANSVERSALITY_TOL: f64 = 1.0e-9;
const BASE_GRADIENT_GROUP_TOL: f64 = 1.0e-9;
const EXPOSED_MARGIN_TOL: f64 = 1.0e-8;
const NEWTON_TOL: f64 = 1.0e-12;
const MAX_NEWTON_ITERATIONS: usize = 12;
const RETURNED_ACTION_GAP: f64 = 1.0e-6;
const JOINT_MIN_RELATIVE_TOL: f64 = 1.0e-8;
const EQUALITY_RELATIVE_TOL: f64 = 1.0e-10;
const DEFAULT_SEED: u64 = 0x6272_616e_6368_6571;
const PAIR_SEARCH_DIRECTIONS: usize = 8192;

// The exact twenty raw words recorded as tied minimizers in the retained
// pentagon branch landscape at 9 degrees. They are a fixture, not a claim that
// this is the complete active catalog at perturbed points.
const TIED_BASE_SIGMAS: [[usize; 6]; 20] = [
    [0, 5, 2, 3, 7, 8],
    [0, 5, 2, 3, 8, 7],
    [0, 5, 3, 2, 7, 8],
    [0, 5, 3, 2, 8, 7],
    [1, 6, 3, 4, 8, 9],
    [1, 6, 3, 4, 9, 8],
    [1, 6, 4, 3, 8, 9],
    [1, 6, 4, 3, 9, 8],
    [2, 7, 0, 4, 5, 9],
    [2, 7, 0, 4, 9, 5],
    [2, 7, 4, 0, 5, 9],
    [2, 7, 4, 0, 9, 5],
    [3, 8, 0, 1, 5, 6],
    [3, 8, 0, 1, 6, 5],
    [3, 8, 1, 0, 5, 6],
    [3, 8, 1, 0, 6, 5],
    [4, 9, 1, 2, 6, 7],
    [4, 9, 1, 2, 7, 6],
    [4, 9, 2, 1, 6, 7],
    [4, 9, 2, 1, 7, 6],
];

#[derive(Debug)]
struct Cli {
    output: PathBuf,
    samples_per_radius: usize,
    radii: Vec<f64>,
    seed: u64,
}

#[derive(Clone)]
struct PentagonChart {
    q_normals: Vec<Vector2<f64>>,
    p_normals_base: Vec<Vector2<f64>>,
    q_heights_base: Vec<f64>,
    p_heights_base: Vec<f64>,
    theta_base: f64,
}

#[derive(Clone, Debug)]
struct BranchEvaluation {
    action: f64,
    beta_margin: f64,
    kkt_n_zero: usize,
    gradient_slice: Vec<f64>,
}

#[derive(Clone, Debug)]
struct PairChoice {
    first_index: usize,
    second_index: usize,
    first_group_members: Vec<usize>,
    second_group_members: Vec<usize>,
    base_gradient_group_count: usize,
    equality_normal: Vec<f64>,
    witness_direction: Vec<f64>,
    exposed_margin_per_unit_step: f64,
}

#[derive(Debug)]
struct CorrectedPoint {
    y: Vec<f64>,
    iterations: usize,
    correction_norm: f64,
    equality_log_residual: f64,
    first: BranchEvaluation,
    second: BranchEvaluation,
    polytope: ProductPolytopeCache,
}

#[derive(Serialize)]
struct SampleRow {
    record_type: &'static str,
    schema_version: &'static str,
    sample_kind: String,
    sample_index: usize,
    requested_radius: f64,
    proposal_direction_slice: Vec<f64>,
    corrected_coordinate_slice: Vec<f64>,
    corrected_radius: f64,
    correction_norm: f64,
    correction_iterations: usize,
    equality_log_residual: f64,
    equality_relative_residual: f64,
    first_action: f64,
    second_action: f64,
    first_beta_margin: f64,
    second_beta_margin: f64,
    first_kkt_n_zero: usize,
    second_kkt_n_zero: usize,
    full_capacity: f64,
    pair_relative_gap_above_capacity: f64,
    pair_joint_minimizer_nominal: bool,
    first_sigma_returned: bool,
    second_sigma_returned: bool,
    first_group_any_sigma_returned: bool,
    second_group_any_sigma_returned: bool,
    nearest_other_relative_gap: Option<f64>,
    best_sigma: Vec<usize>,
    returned_orbit_count: usize,
    capacity_iterations: u64,
    volume: f64,
    sys: f64,
    capacity_runtime_ms: f64,
}

fn main() {
    let cli = parse_cli();
    if let Some(parent) = cli.output.parent() {
        create_dir_all(parent).expect("failed to create output parent directory");
    }

    let chart = PentagonChart::new();
    let (nuisance_basis, slice_basis) = chart.local_slice_basis();
    assert_eq!(nuisance_basis.len(), EXPECTED_NUISANCE_DIM);
    assert_eq!(slice_basis.len(), EXPECTED_SLICE_DIM);

    let zero = vec![0.0; EXPECTED_SLICE_DIM];
    let base = chart
        .polytope_at(&zero, &slice_basis)
        .expect("base pentagon product must construct");
    let base_branches = evaluate_fixture_branches(&base, &slice_basis)
        .expect("all twenty base fixture words must be admissible");
    let base_action_min = base_branches
        .iter()
        .map(|branch| branch.action)
        .fold(f64::INFINITY, f64::min);
    let base_action_max = base_branches
        .iter()
        .map(|branch| branch.action)
        .fold(f64::NEG_INFINITY, f64::max);
    let base_relative_spread = base_action_max / base_action_min - 1.0;
    assert!(
        base_relative_spread <= 1.0e-12,
        "the twenty-word base fixture no longer ties: relative spread={base_relative_spread:e}"
    );

    let mut pair_rng = ChaCha8Rng::seed_from_u64(cli.seed ^ 0x7061_6972);
    let pair = select_exposed_pair(&base_branches, &mut pair_rng)
        .expect("no transverse exposed pair found among the twenty tied base words");
    assert!(
        norm(&pair.equality_normal) > TRANSVERSALITY_TOL,
        "selected pair is not transverse in the local product slice"
    );

    let first_sigma = TIED_BASE_SIGMAS[pair.first_index].to_vec();
    let second_sigma = TIED_BASE_SIGMAS[pair.second_index].to_vec();
    let output = File::create(&cli.output).expect("failed to create output JSONL");
    let mut writer = BufWriter::new(output);

    write_jsonl(
        &mut writer,
        &json!({
            "record_type": "metadata",
            "schema_version": "branch-equality-continuation-v1",
            "epistemic_role": "local numerical method control; fixed-branch equality is not a capacity claim",
            "command_args": env::args().collect::<Vec<_>>(),
            "producer_source": "experiments/dev-sys-prediction/branch-equality-continuation/main.rs",
            "base_family": "P5_x_Rtheta_P5",
            "base_theta_deg": BASE_THETA_DEG,
            "parameter_coordinates": "ten log supports followed by relative rotation in radians",
            "parameter_dimension": PARAM_DIM,
            "removed_local_equivalence_directions": [
                "q_translation_x", "q_translation_y", "p_translation_x", "p_translation_y",
                "common_dilation", "reciprocal_factor_scaling"
            ],
            "nuisance_rank": nuisance_basis.len(),
            "slice_dimension": slice_basis.len(),
            "slice_basis_columns": slice_basis,
            "seed": cli.seed,
            "samples_per_radius": cli.samples_per_radius,
            "radii": cli.radii,
            "newton_tolerance_log_action": NEWTON_TOL,
            "max_newton_iterations": MAX_NEWTON_ITERATIONS,
            "returned_action_gap_absolute": RETURNED_ACTION_GAP,
            "joint_min_relative_tolerance": JOINT_MIN_RELATIVE_TOL,
            "base_tied_word_count": TIED_BASE_SIGMAS.len(),
            "base_relative_action_spread": base_relative_spread,
            "selected_first_sigma": first_sigma,
            "selected_second_sigma": second_sigma,
            "base_log_gradient_group_count": pair.base_gradient_group_count,
            "selected_first_group_sigmas": pair.first_group_members.iter().map(|&index| TIED_BASE_SIGMAS[index].to_vec()).collect::<Vec<_>>(),
            "selected_second_group_sigmas": pair.second_group_members.iter().map(|&index| TIED_BASE_SIGMAS[index].to_vec()).collect::<Vec<_>>(),
            "selected_equality_normal_slice": pair.equality_normal,
            "selected_exposed_witness_direction_slice": pair.witness_direction,
            "selected_exposed_margin_per_unit_step": pair.exposed_margin_per_unit_step,
            "pair_search_directions": PAIR_SEARCH_DIRECTIONS,
        }),
    );

    let mut sample_rng = ChaCha8Rng::seed_from_u64(cli.seed ^ 0x7361_6d70_6c65);
    let mut completed = 0usize;
    let mut correction_failures = 0usize;
    let mut joint_minimizers = 0usize;
    let mut rows = Vec::new();

    for &radius in &cli.radii {
        let mut directions = Vec::with_capacity(cli.samples_per_radius + 1);
        directions.push((
            "exposed_witness".to_string(),
            pair.witness_direction.clone(),
        ));
        for index in 0..cli.samples_per_radius {
            let direction = random_tangent_direction(
                &mut sample_rng,
                EXPECTED_SLICE_DIM,
                &pair.equality_normal,
            );
            directions.push((format!("random_tangent_{index}"), direction));
        }

        for (sample_index, (sample_kind, direction)) in directions.into_iter().enumerate() {
            match run_sample(
                &chart,
                &slice_basis,
                &pair,
                radius,
                sample_index,
                &sample_kind,
                direction,
            ) {
                Ok(row) => {
                    completed += 1;
                    joint_minimizers += usize::from(row.pair_joint_minimizer_nominal);
                    rows.push(row);
                }
                Err(error) => {
                    correction_failures += 1;
                    write_jsonl(
                        &mut writer,
                        &json!({
                            "record_type": "failure",
                            "schema_version": "branch-equality-continuation-v1",
                            "requested_radius": radius,
                            "sample_kind": sample_kind,
                            "sample_index": sample_index,
                            "error": error,
                        }),
                    );
                }
            }
        }
    }

    for row in &rows {
        write_jsonl(&mut writer, row);
    }
    write_jsonl(
        &mut writer,
        &json!({
            "record_type": "run_summary",
            "schema_version": "branch-equality-continuation-v1",
            "completed_samples": completed,
            "correction_or_evaluation_failures": correction_failures,
            "nominal_joint_minimizer_samples": joint_minimizers,
            "max_equality_relative_residual": rows.iter().map(|row| row.equality_relative_residual).fold(0.0_f64, f64::max),
            "max_correction_norm": rows.iter().map(|row| row.correction_norm).fold(0.0_f64, f64::max),
            "max_pair_relative_gap_above_capacity": rows.iter().map(|row| row.pair_relative_gap_above_capacity).fold(0.0_f64, f64::max),
        }),
    );
    writer.flush().expect("failed to flush output JSONL");

    println!(
        "wrote {} completed samples ({} failures, {} nominal joint minima) to {}",
        completed,
        correction_failures,
        joint_minimizers,
        cli.output.display()
    );
}

fn parse_cli() -> Cli {
    let mut output = PathBuf::from("/tmp/branch-equality-continuation.jsonl");
    let mut samples_per_radius = 16usize;
    let mut radii = vec![1.0e-5, 1.0e-4, 1.0e-3, 1.0e-2];
    let mut seed = DEFAULT_SEED;
    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--output" => output = PathBuf::from(args.next().expect("--output needs a path")),
            "--samples-per-radius" => {
                samples_per_radius = args
                    .next()
                    .expect("--samples-per-radius needs a value")
                    .parse()
                    .expect("invalid --samples-per-radius")
            }
            "--radii" => {
                radii = args
                    .next()
                    .expect("--radii needs comma-separated values")
                    .split(',')
                    .map(|value| value.parse::<f64>().expect("invalid radius"))
                    .collect()
            }
            "--seed" => {
                seed = args
                    .next()
                    .expect("--seed needs a value")
                    .parse()
                    .expect("invalid --seed")
            }
            "--help" | "-h" => {
                println!(
                    "Usage: dev-branch-equality-continuation [--output PATH] [--samples-per-radius N] [--radii R1,R2,...] [--seed U64]"
                );
                std::process::exit(0);
            }
            _ => panic!("unknown argument: {arg}"),
        }
    }
    assert!(!radii.is_empty(), "at least one radius is required");
    assert!(
        radii
            .iter()
            .all(|radius| radius.is_finite() && *radius > 0.0),
        "all radii must be finite and positive"
    );
    Cli {
        output,
        samples_per_radius,
        radii,
        seed,
    }
}

impl PentagonChart {
    fn new() -> Self {
        let (q_normals, q_heights_base) = regular_polygon_2d(5, 1.0);
        let (p_normals_base, p_heights_base) = regular_polygon_2d(5, 1.0);
        Self {
            q_normals,
            p_normals_base,
            q_heights_base,
            p_heights_base,
            theta_base: BASE_THETA_DEG.to_radians(),
        }
    }

    fn local_slice_basis(&self) -> (Vec<Vec<f64>>, Vec<Vec<f64>>) {
        let (p_normals_rotated, _) =
            rotate_polygon_2d(&self.p_normals_base, &self.p_heights_base, self.theta_base);
        let mut nuisance = Vec::new();

        let mut q_tx = vec![0.0; PARAM_DIM];
        let mut q_ty = vec![0.0; PARAM_DIM];
        for index in 0..5 {
            q_tx[index] = self.q_normals[index][0] / self.q_heights_base[index];
            q_ty[index] = self.q_normals[index][1] / self.q_heights_base[index];
        }
        nuisance.push(q_tx);
        nuisance.push(q_ty);

        let mut p_tx = vec![0.0; PARAM_DIM];
        let mut p_ty = vec![0.0; PARAM_DIM];
        for index in 0..5 {
            p_tx[5 + index] = p_normals_rotated[index][0] / self.p_heights_base[index];
            p_ty[5 + index] = p_normals_rotated[index][1] / self.p_heights_base[index];
        }
        nuisance.push(p_tx);
        nuisance.push(p_ty);

        let mut common_dilation = vec![0.0; PARAM_DIM];
        common_dilation[..10].fill(1.0);
        nuisance.push(common_dilation);

        let mut reciprocal_scaling = vec![0.0; PARAM_DIM];
        reciprocal_scaling[..5].fill(1.0);
        reciprocal_scaling[5..10].fill(-1.0);
        nuisance.push(reciprocal_scaling);

        let nuisance_basis = orthonormalize(nuisance, BASIS_TOL);
        let mut complement_candidates = Vec::new();
        for coordinate in 0..PARAM_DIM {
            let mut vector = vec![0.0; PARAM_DIM];
            vector[coordinate] = 1.0;
            subtract_projection(&mut vector, &nuisance_basis);
            subtract_projection(&mut vector, &complement_candidates);
            let length = norm(&vector);
            if length > BASIS_TOL {
                scale(&mut vector, 1.0 / length);
                complement_candidates.push(vector);
            }
        }
        (nuisance_basis, complement_candidates)
    }

    fn polytope_at(&self, y: &[f64], slice_basis: &[Vec<f64>]) -> Option<ProductPolytopeCache> {
        let parameters = lift_from_slice(y, slice_basis);
        let q_heights: Vec<f64> = self
            .q_heights_base
            .iter()
            .zip(&parameters[..5])
            .map(|(height, delta)| height * delta.exp())
            .collect();
        let p_heights: Vec<f64> = self
            .p_heights_base
            .iter()
            .zip(&parameters[5..10])
            .map(|(height, delta)| height * delta.exp())
            .collect();
        let theta = self.theta_base + parameters[10];
        let (p_normals, _) = rotate_polygon_2d(&self.p_normals_base, &self.p_heights_base, theta);
        ProductPolytopeCache::from_lagrangian_product(
            &self.q_normals,
            &q_heights,
            &p_normals,
            &p_heights,
        )
    }
}

fn evaluate_fixture_branches(
    polytope: &ProductPolytopeCache,
    slice_basis: &[Vec<f64>],
) -> Option<Vec<BranchEvaluation>> {
    TIED_BASE_SIGMAS
        .iter()
        .map(|sigma| evaluate_branch(polytope, sigma, slice_basis))
        .collect()
}

fn evaluate_branch(
    polytope: &ProductPolytopeCache,
    sigma: &[usize],
    slice_basis: &[Vec<f64>],
) -> Option<BranchEvaluation> {
    let KktOutcome::Feasible(kkt) = solve_kkt_for_dual_vertices(&polytope.dual_vertices_f64, sigma)
    else {
        return None;
    };
    if !kkt.q_corrected.is_finite() || kkt.q_corrected <= 0.0 {
        return None;
    }
    let action = 0.5 / kkt.q_corrected;
    let beta_margin = kkt.beta.iter().copied().fold(f64::INFINITY, f64::min);
    let gradients_a =
        capacity_derivatives_a_from_kkt_result(&polytope.dual_vertices_f64, sigma, &kkt);

    let mut gradient_parameters = vec![0.0; PARAM_DIM];
    for facet in 0..10 {
        gradient_parameters[facet] = gradients_a[facet].dot(&(-polytope.dual_vertices_f64[facet]));
    }
    for facet in 5..10 {
        let a = polytope.dual_vertices_f64[facet];
        let derivative_theta = Vector4::new(0.0, 0.0, -a[3], a[2]);
        gradient_parameters[10] += gradients_a[facet].dot(&derivative_theta);
    }
    let gradient_slice = slice_basis
        .iter()
        .map(|basis_vector| dot(&gradient_parameters, basis_vector))
        .collect();
    Some(BranchEvaluation {
        action,
        beta_margin,
        kkt_n_zero: kkt.n_zero,
        gradient_slice,
    })
}

fn select_exposed_pair(branches: &[BranchEvaluation], rng: &mut ChaCha8Rng) -> Option<PairChoice> {
    let log_gradients: Vec<Vec<f64>> = branches
        .iter()
        .map(|branch| {
            branch
                .gradient_slice
                .iter()
                .map(|derivative| derivative / branch.action)
                .collect()
        })
        .collect();
    let mut groups: Vec<Vec<usize>> = Vec::new();
    for (index, gradient) in log_gradients.iter().enumerate() {
        if let Some(group) = groups
            .iter_mut()
            .find(|group| distance(gradient, &log_gradients[group[0]]) <= BASE_GRADIENT_GROUP_TOL)
        {
            group.push(index);
        } else {
            groups.push(vec![index]);
        }
    }

    let mut best: Option<PairChoice> = None;
    for first_group_index in 0..groups.len() {
        for second_group_index in first_group_index + 1..groups.len() {
            let first = groups[first_group_index][0];
            let second = groups[second_group_index][0];
            let normal: Vec<f64> = log_gradients[first]
                .iter()
                .zip(&log_gradients[second])
                .map(|(left, right)| left - right)
                .collect();
            if norm(&normal) <= TRANSVERSALITY_TOL {
                continue;
            }
            for _ in 0..PAIR_SEARCH_DIRECTIONS {
                let direction = random_tangent_direction(rng, EXPECTED_SLICE_DIM, &normal);
                let first_slope = dot(&log_gradients[first], &direction);
                let second_slope = dot(&log_gradients[second], &direction);
                let common_slope = 0.5 * (first_slope + second_slope);
                let margin = groups
                    .iter()
                    .enumerate()
                    .filter(|(index, _)| {
                        *index != first_group_index && *index != second_group_index
                    })
                    .map(|(_, group)| dot(&log_gradients[group[0]], &direction) - common_slope)
                    .fold(f64::INFINITY, f64::min);
                if best
                    .as_ref()
                    .is_none_or(|candidate| margin > candidate.exposed_margin_per_unit_step)
                {
                    best = Some(PairChoice {
                        first_index: first,
                        second_index: second,
                        first_group_members: groups[first_group_index].clone(),
                        second_group_members: groups[second_group_index].clone(),
                        base_gradient_group_count: groups.len(),
                        equality_normal: normal.clone(),
                        witness_direction: direction,
                        exposed_margin_per_unit_step: margin,
                    });
                }
            }
        }
    }
    best.filter(|choice| choice.exposed_margin_per_unit_step > EXPOSED_MARGIN_TOL)
}

fn run_sample(
    chart: &PentagonChart,
    slice_basis: &[Vec<f64>],
    pair: &PairChoice,
    radius: f64,
    sample_index: usize,
    sample_kind: &str,
    direction: Vec<f64>,
) -> Result<SampleRow, String> {
    let proposal: Vec<f64> = direction.iter().map(|value| radius * value).collect();
    let corrected = correct_to_equality(chart, slice_basis, pair, &proposal)?;
    let pair_action = 0.5 * (corrected.first.action + corrected.second.action);
    let equality_relative_residual = (corrected.first.action / corrected.second.action - 1.0).abs();

    let capacity_started = Instant::now();
    let capacity = collect_billiard_result(&corrected.polytope, RETURNED_ACTION_GAP)
        .map_err(|error| format!("capacity recomputation failed: {error:?}"))?;
    let capacity_runtime_ms = capacity_started.elapsed().as_secs_f64() * 1000.0;
    let pair_relative_gap_above_capacity = pair_action / capacity.min_action - 1.0;
    let first_sigma = &TIED_BASE_SIGMAS[pair.first_index];
    let second_sigma = &TIED_BASE_SIGMAS[pair.second_index];
    let first_sigma_returned = capacity
        .orbits
        .iter()
        .any(|orbit| orbit.sigma.as_slice() == first_sigma);
    let second_sigma_returned = capacity
        .orbits
        .iter()
        .any(|orbit| orbit.sigma.as_slice() == second_sigma);
    let first_group_any_sigma_returned = capacity.orbits.iter().any(|orbit| {
        pair.first_group_members
            .iter()
            .any(|&index| orbit.sigma.as_slice() == TIED_BASE_SIGMAS[index])
    });
    let second_group_any_sigma_returned = capacity.orbits.iter().any(|orbit| {
        pair.second_group_members
            .iter()
            .any(|&index| orbit.sigma.as_slice() == TIED_BASE_SIGMAS[index])
    });
    let nearest_other_relative_gap = capacity
        .orbits
        .iter()
        .filter(|orbit| {
            !pair
                .first_group_members
                .iter()
                .chain(&pair.second_group_members)
                .any(|&index| orbit.sigma.as_slice() == TIED_BASE_SIGMAS[index])
        })
        .map(|orbit| orbit.action / pair_action - 1.0)
        .min_by(|left, right| left.total_cmp(right));
    let pair_joint_minimizer_nominal = equality_relative_residual <= EQUALITY_RELATIVE_TOL
        && pair_relative_gap_above_capacity.abs() <= JOINT_MIN_RELATIVE_TOL;
    let volume = exact_volume_reference_as_f64(
        &corrected.polytope.vertices,
        &corrected.polytope.vertex_facet_incidence,
    );
    let sys = symplectic::systolic_ratio(capacity.min_action, volume);

    Ok(SampleRow {
        record_type: "sample",
        schema_version: "branch-equality-continuation-v1",
        sample_kind: sample_kind.to_string(),
        sample_index,
        requested_radius: radius,
        proposal_direction_slice: direction,
        corrected_coordinate_slice: corrected.y.clone(),
        corrected_radius: norm(&corrected.y),
        correction_norm: corrected.correction_norm,
        correction_iterations: corrected.iterations,
        equality_log_residual: corrected.equality_log_residual,
        equality_relative_residual,
        first_action: corrected.first.action,
        second_action: corrected.second.action,
        first_beta_margin: corrected.first.beta_margin,
        second_beta_margin: corrected.second.beta_margin,
        first_kkt_n_zero: corrected.first.kkt_n_zero,
        second_kkt_n_zero: corrected.second.kkt_n_zero,
        full_capacity: capacity.min_action,
        pair_relative_gap_above_capacity,
        pair_joint_minimizer_nominal,
        first_sigma_returned,
        second_sigma_returned,
        first_group_any_sigma_returned,
        second_group_any_sigma_returned,
        nearest_other_relative_gap,
        best_sigma: capacity.best_sigma().to_vec(),
        returned_orbit_count: capacity.orbits.len(),
        capacity_iterations: capacity.iterations,
        volume,
        sys,
        capacity_runtime_ms,
    })
}

fn correct_to_equality(
    chart: &PentagonChart,
    slice_basis: &[Vec<f64>],
    pair: &PairChoice,
    proposal: &[f64],
) -> Result<CorrectedPoint, String> {
    let mut y = proposal.to_vec();
    for iteration in 0..=MAX_NEWTON_ITERATIONS {
        let polytope = chart
            .polytope_at(&y, slice_basis)
            .ok_or_else(|| "product geometry construction failed".to_string())?;
        let first = evaluate_branch(&polytope, &TIED_BASE_SIGMAS[pair.first_index], slice_basis)
            .ok_or_else(|| "first branch became invalid or unsolved".to_string())?;
        let second = evaluate_branch(&polytope, &TIED_BASE_SIGMAS[pair.second_index], slice_basis)
            .ok_or_else(|| "second branch became invalid or unsolved".to_string())?;
        let residual = first.action.ln() - second.action.ln();
        if residual.abs() <= NEWTON_TOL {
            let correction: Vec<f64> = y
                .iter()
                .zip(proposal)
                .map(|(corrected, proposed)| corrected - proposed)
                .collect();
            return Ok(CorrectedPoint {
                y,
                iterations: iteration,
                correction_norm: norm(&correction),
                equality_log_residual: residual,
                first,
                second,
                polytope,
            });
        }
        if iteration == MAX_NEWTON_ITERATIONS {
            return Err(format!(
                "Newton correction did not converge: |log action ratio|={:e}",
                residual.abs()
            ));
        }
        let gradient: Vec<f64> = first
            .gradient_slice
            .iter()
            .zip(&second.gradient_slice)
            .map(|(left, right)| left / first.action - right / second.action)
            .collect();
        let squared_norm = dot(&gradient, &gradient);
        if squared_norm <= TRANSVERSALITY_TOL * TRANSVERSALITY_TOL {
            return Err(format!(
                "equality normal lost rank: norm={:e}",
                squared_norm.sqrt()
            ));
        }
        let multiplier = residual / squared_norm;
        for (coordinate, derivative) in y.iter_mut().zip(gradient) {
            *coordinate -= multiplier * derivative;
        }
    }
    unreachable!("bounded Newton loop returns on every exit")
}

fn collect_billiard_result(
    polytope: &ProductPolytopeCache,
    action_gap: f64,
) -> Result<OrbitSearchResult, OrbitSearchError> {
    let classification = classify_facets_from_dual_vertices(&polytope.dual_vertices_f64)
        .expect("constructed product must classify as a Lagrangian product");
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
    aggregate_orbits_with_dual_vertices_exact(
        &polytope.dual_vertices,
        orbits,
        iterations,
        action_gap,
        OrbitGuaranteeMode::MinimaSafe,
    )
}

fn random_tangent_direction(rng: &mut ChaCha8Rng, dimension: usize, normal: &[f64]) -> Vec<f64> {
    loop {
        let mut direction: Vec<f64> = (0..dimension).map(|_| rng.sample(StandardNormal)).collect();
        let normal_squared = dot(normal, normal);
        assert!(normal_squared > 0.0);
        let projection = dot(&direction, normal) / normal_squared;
        for (coordinate, normal_coordinate) in direction.iter_mut().zip(normal) {
            *coordinate -= projection * normal_coordinate;
        }
        let length = norm(&direction);
        if length > 1.0e-12 {
            scale(&mut direction, 1.0 / length);
            return direction;
        }
    }
}

fn orthonormalize(vectors: Vec<Vec<f64>>, tolerance: f64) -> Vec<Vec<f64>> {
    let mut basis = Vec::new();
    for mut vector in vectors {
        subtract_projection(&mut vector, &basis);
        let length = norm(&vector);
        if length > tolerance {
            scale(&mut vector, 1.0 / length);
            basis.push(vector);
        }
    }
    basis
}

fn subtract_projection(vector: &mut [f64], basis: &[Vec<f64>]) {
    for basis_vector in basis {
        let coefficient = dot(vector, basis_vector);
        for (value, basis_value) in vector.iter_mut().zip(basis_vector) {
            *value -= coefficient * basis_value;
        }
    }
}

fn lift_from_slice(y: &[f64], basis: &[Vec<f64>]) -> Vec<f64> {
    assert_eq!(y.len(), basis.len());
    let mut parameters = vec![0.0; PARAM_DIM];
    for (coefficient, basis_vector) in y.iter().zip(basis) {
        for (parameter, basis_value) in parameters.iter_mut().zip(basis_vector) {
            *parameter += coefficient * basis_value;
        }
    }
    parameters
}

fn dot(left: &[f64], right: &[f64]) -> f64 {
    assert_eq!(left.len(), right.len());
    left.iter().zip(right).map(|(a, b)| a * b).sum()
}

fn norm(vector: &[f64]) -> f64 {
    dot(vector, vector).sqrt()
}

fn distance(left: &[f64], right: &[f64]) -> f64 {
    assert_eq!(left.len(), right.len());
    left.iter()
        .zip(right)
        .map(|(a, b)| (a - b) * (a - b))
        .sum::<f64>()
        .sqrt()
}

fn scale(vector: &mut [f64], coefficient: f64) {
    for value in vector {
        *value *= coefficient;
    }
}

fn write_jsonl(writer: &mut impl Write, value: &impl Serialize) {
    serde_json::to_writer(&mut *writer, value).expect("failed to serialize JSONL row");
    writer
        .write_all(b"\n")
        .expect("failed to write JSONL newline");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn local_slice_has_expected_rank_and_is_orthogonal() {
        let chart = PentagonChart::new();
        let (nuisance, slice) = chart.local_slice_basis();
        assert_eq!(nuisance.len(), EXPECTED_NUISANCE_DIM);
        assert_eq!(slice.len(), EXPECTED_SLICE_DIM);
        for left in &nuisance {
            for right in &slice {
                assert!(dot(left, right).abs() <= 1.0e-12);
            }
        }
        for (index, left) in slice.iter().enumerate() {
            for (other_index, right) in slice.iter().enumerate() {
                let expected = if index == other_index { 1.0 } else { 0.0 };
                assert!((dot(left, right) - expected).abs() <= 1.0e-12);
            }
        }
    }

    #[test]
    fn base_fixture_has_a_transverse_exposed_pair() {
        let chart = PentagonChart::new();
        let (_, slice) = chart.local_slice_basis();
        let base = chart.polytope_at(&vec![0.0; slice.len()], &slice).unwrap();
        let branches = evaluate_fixture_branches(&base, &slice).unwrap();
        assert!(branches.iter().all(|branch| branch.kkt_n_zero == 0));
        let mut rng = ChaCha8Rng::seed_from_u64(DEFAULT_SEED ^ 0x7465_7374);
        let pair = select_exposed_pair(&branches, &mut rng).unwrap();
        assert!(pair.exposed_margin_per_unit_step > 0.0);
        assert!(norm(&pair.equality_normal) > TRANSVERSALITY_TOL);
    }

    #[test]
    fn exposed_witness_corrects_at_smoke_radius() {
        let chart = PentagonChart::new();
        let (_, slice) = chart.local_slice_basis();
        let base = chart.polytope_at(&vec![0.0; slice.len()], &slice).unwrap();
        let branches = evaluate_fixture_branches(&base, &slice).unwrap();
        let mut rng = ChaCha8Rng::seed_from_u64(DEFAULT_SEED ^ 0x6e65_7774);
        let pair = select_exposed_pair(&branches, &mut rng).unwrap();
        let proposal: Vec<f64> = pair
            .witness_direction
            .iter()
            .map(|value| 1.0e-4 * value)
            .collect();
        let corrected = correct_to_equality(&chart, &slice, &pair, &proposal).unwrap();
        assert!(corrected.equality_log_residual.abs() <= NEWTON_TOL);
        assert!(corrected.first.beta_margin > 0.0);
        assert!(corrected.second.beta_margin > 0.0);
    }

    #[test]
    fn pair_log_action_gradient_matches_finite_difference() {
        let chart = PentagonChart::new();
        let (_, slice) = chart.local_slice_basis();
        let base = chart.polytope_at(&vec![0.0; slice.len()], &slice).unwrap();
        let branches = evaluate_fixture_branches(&base, &slice).unwrap();
        let mut rng = ChaCha8Rng::seed_from_u64(DEFAULT_SEED ^ 0x6772_6164);
        let pair = select_exposed_pair(&branches, &mut rng).unwrap();
        let y: Vec<f64> = pair
            .witness_direction
            .iter()
            .map(|coordinate| 3.0e-4 * coordinate)
            .collect();
        let point = chart.polytope_at(&y, &slice).unwrap();
        let first = evaluate_branch(&point, &TIED_BASE_SIGMAS[pair.first_index], &slice).unwrap();
        let second = evaluate_branch(&point, &TIED_BASE_SIGMAS[pair.second_index], &slice).unwrap();
        let analytic: Vec<f64> = first
            .gradient_slice
            .iter()
            .zip(&second.gradient_slice)
            .map(|(left, right)| left / first.action - right / second.action)
            .collect();
        let epsilon = 1.0e-6;
        for coordinate in 0..slice.len() {
            let mut plus = y.clone();
            let mut minus = y.clone();
            plus[coordinate] += epsilon;
            minus[coordinate] -= epsilon;
            let plus_polytope = chart.polytope_at(&plus, &slice).unwrap();
            let minus_polytope = chart.polytope_at(&minus, &slice).unwrap();
            let plus_first =
                evaluate_branch(&plus_polytope, &TIED_BASE_SIGMAS[pair.first_index], &slice)
                    .unwrap();
            let plus_second =
                evaluate_branch(&plus_polytope, &TIED_BASE_SIGMAS[pair.second_index], &slice)
                    .unwrap();
            let minus_first =
                evaluate_branch(&minus_polytope, &TIED_BASE_SIGMAS[pair.first_index], &slice)
                    .unwrap();
            let minus_second = evaluate_branch(
                &minus_polytope,
                &TIED_BASE_SIGMAS[pair.second_index],
                &slice,
            )
            .unwrap();
            let plus_residual = plus_first.action.ln() - plus_second.action.ln();
            let minus_residual = minus_first.action.ln() - minus_second.action.ln();
            let numerical = (plus_residual - minus_residual) / (2.0 * epsilon);
            assert!(
                (analytic[coordinate] - numerical).abs() <= 2.0e-7,
                "coordinate {coordinate}: analytic={} numerical={}",
                analytic[coordinate],
                numerical
            );
        }
    }
}

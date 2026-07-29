use exp_dev_quadratic_program::f64_geometry_payload;
use nalgebra::{DMatrix, DVector, Vector4};
use optimizer_runs::evaluator::{
    reconstruct_geometry_and_volume, EvaluatorConfig, GeometryMode, VolumeMode,
};
use optimizer_runs::quotient::{add_flat_direction, l2_norm, unflatten};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fs::{create_dir_all, File};
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::time::Instant;
use symplectic::derivatives::{
    capacity_derivatives_a_from_orbit, directional_derivative_a, systolic_ratio_gradient_a,
    volume_derivatives_a,
};
use symplectic::kkt::qp_assembly::build_augmented_system_from_dual_vertices;
use symplectic::kkt::saddle_point_solver::{solve_kkt_for_dual_vertices, KktOutcome};
use symplectic::{solve_orbit_sigma_saddle_point, systolic_ratio};

const SOURCE_RADIUS: f64 = 1.0e-5;
const AUDIT_RADII: [f64; 6] = [1.0e-5, 3.0e-6, 1.0e-6, 1.0e-7, 3.0e-8, 1.0e-8];
const CASES: [(&str, &str); 3] = [
    ("top_failure", "rank-001--random_F10_s0_12"),
    ("positive_control", "rank-002--random_F10_s1_4"),
    ("clean_failure", "rank-004--random_F10_s1_6"),
];

#[derive(Debug)]
struct Cli {
    input: PathBuf,
    candidates: PathBuf,
    out: PathBuf,
}

#[derive(Deserialize)]
struct InputPacket {
    states: Vec<InputState>,
}

#[derive(Deserialize)]
struct InputState {
    state_id: String,
    dual_flat: Vec<f64>,
}

#[derive(Deserialize)]
struct CandidateRow {
    state_id: String,
    family: String,
    normalized_radius: f64,
    delta_sys: Option<f64>,
    candidate_winning_sigma: Option<Vec<usize>>,
    proposal_fields: Value,
}

#[derive(Serialize)]
struct AuditOutput {
    schema_version: u32,
    question: &'static str,
    input: String,
    candidates: String,
    source_family: &'static str,
    source_normalized_radius: f64,
    audit_normalized_radii: Vec<f64>,
    elapsed_ms: f64,
    cases: Vec<CaseAudit>,
    claim_boundary: &'static str,
}

#[derive(Serialize)]
struct CaseAudit {
    role: String,
    state_id: String,
    source_delta_sys: f64,
    sigma: Vec<usize>,
    base_norm: f64,
    source_absolute_distance: f64,
    base_action: f64,
    base_f64_volume: f64,
    base_exact_volume: f64,
    base_branch_ratio_f64_volume: f64,
    analytic_action_directional: f64,
    analytic_f64_volume_directional: f64,
    analytic_branch_ratio_directional: f64,
    base: PointAudit,
    radii: Vec<RadiusAudit>,
}

#[derive(Serialize)]
struct RadiusAudit {
    normalized_radius: f64,
    absolute_radius: f64,
    kkt_frobenius_perturbation_over_base_eigen_gap: f64,
    plus: PointAudit,
    minus: PointAudit,
    action: DerivativeComparison,
    f64_volume: DerivativeComparison,
    exact_volume: DerivativeComparison,
    branch_ratio_f64_volume: DerivativeComparison,
}

#[derive(Serialize)]
struct DerivativeComparison {
    analytic: f64,
    forward: Option<f64>,
    backward: Option<f64>,
    central: Option<f64>,
    central_relative_error: Option<f64>,
}

#[derive(Serialize)]
struct PointAudit {
    sign: i8,
    normalized_radius: f64,
    action: Option<f64>,
    action_lower: Option<f64>,
    action_upper: Option<f64>,
    beta: Option<Vec<f64>>,
    beta_margin: Option<f64>,
    admissibility: Option<String>,
    branch_ratio_f64_volume: Option<f64>,
    f64_volume: f64,
    exact_volume: f64,
    f64_vs_exact_volume_relative_error: f64,
    f64_exact_incidence_agree: bool,
    f64_exact_facet_intersections_agree: bool,
    f64_exact_omega_signs_agree: bool,
    f64_vertex_count: usize,
    exact_vertex_count: usize,
    vertex_indeterminate_count: usize,
    bounded_near_singular_vertex_count: usize,
    ambiguous_vertex_incidence_count: usize,
    facet_intersection_indeterminate_count: usize,
    omega_indeterminate_count: usize,
    kkt: KktAudit,
}

#[derive(Serialize)]
struct KktAudit {
    outcome: String,
    matrix_size: usize,
    maximum_abs_eigenvalue: f64,
    minimum_abs_eigenvalue: f64,
    closest_to_zero_eigenvalue: f64,
    raw_negative_eigenvalue_count: usize,
    permissive_rank_1e_12: usize,
    strict_rank_relative_1e_3: usize,
    retained_condition_number_1e_12: Option<f64>,
    residual_l2: Option<f64>,
    solver_n_positive: Option<usize>,
    solver_n_negative: Option<usize>,
    solver_n_zero: Option<usize>,
}

fn main() -> Result<(), String> {
    let cli = parse_cli()?;
    let started = Instant::now();
    let packet: InputPacket = serde_json::from_reader(
        File::open(&cli.input).map_err(|error| format!("open input: {error}"))?,
    )
    .map_err(|error| format!("parse input: {error}"))?;
    let candidates: Vec<CandidateRow> = read_jsonl(&cli.candidates)?;
    let states = packet
        .states
        .into_iter()
        .map(|state| (state.state_id.clone(), state))
        .collect::<HashMap<_, _>>();

    let mut cases = Vec::new();
    for (role, state_id) in CASES {
        let state = states
            .get(state_id)
            .ok_or_else(|| format!("missing state {state_id}"))?;
        let row = candidates
            .iter()
            .find(|row| {
                row.state_id == state_id
                    && row.family == "gap-window-0.1"
                    && (row.normalized_radius - SOURCE_RADIUS).abs() <= 1.0e-15
            })
            .ok_or_else(|| format!("missing source proposal for {state_id}"))?;
        cases.push(audit_case(role, state, row)?);
    }

    create_dir_all(&cli.out).map_err(|error| format!("create output: {error}"))?;
    let output = AuditOutput {
        schema_version: 1,
        question: "Which derivative component explains represented-winner affine failures?",
        input: cli.input.display().to_string(),
        candidates: cli.candidates.display().to_string(),
        source_family: "gap-window-0.1",
        source_normalized_radius: SOURCE_RADIUS,
        audit_normalized_radii: AUDIT_RADII.to_vec(),
        elapsed_ms: started.elapsed().as_secs_f64() * 1000.0,
        cases,
        claim_boundary: "Named-branch directional audit of three outcome-selected proposals. It does not certify mathematical capacity, candidate completeness, or endpoint stationarity.",
    };
    serde_json::to_writer_pretty(
        File::create(cli.out.join("audit.json"))
            .map_err(|error| format!("create audit.json: {error}"))?,
        &output,
    )
    .map_err(|error| format!("write audit.json: {error}"))?;
    println!(
        "{}",
        serde_json::to_string_pretty(&output).map_err(|error| error.to_string())?
    );
    Ok(())
}

fn audit_case(role: &str, state: &InputState, source: &CandidateRow) -> Result<CaseAudit, String> {
    let base = unflatten(&state.dual_flat)?;
    let displacement = source
        .proposal_fields
        .get("displacement_flat")
        .and_then(Value::as_array)
        .ok_or_else(|| format!("{} lacks displacement_flat", state.state_id))?
        .iter()
        .map(|value| value.as_f64().ok_or("non-f64 displacement"))
        .collect::<Result<Vec<_>, _>>()
        .map_err(str::to_string)?;
    let source_distance = displacement.iter().map(|x| x * x).sum::<f64>().sqrt();
    if !source_distance.is_finite() || source_distance <= 0.0 {
        return Err(format!("invalid source distance for {}", state.state_id));
    }
    let direction_flat = DVector::from_vec(displacement) / source_distance;
    let direction = unflatten(direction_flat.as_slice())?;
    let sigma = source
        .candidate_winning_sigma
        .clone()
        .ok_or_else(|| format!("{} lacks target winner", state.state_id))?;
    let source_delta_sys = source
        .delta_sys
        .ok_or_else(|| format!("{} lacks source delta", state.state_id))?;

    let f64_config = config(GeometryMode::F64, VolumeMode::F64);
    let exact_config = config(GeometryMode::Exact, VolumeMode::Exact);
    let (base_polytope, base_f64_volume) = reconstruct_geometry_and_volume(&base, &f64_config)?;
    let (_, base_exact_volume) = reconstruct_geometry_and_volume(&base, &exact_config)?;
    let base_orbit = solve_orbit_sigma_saddle_point(&base, &sigma)
        .map_err(|error| format!("base named orbit failed for {}: {error:?}", state.state_id))?;
    let d_action = capacity_derivatives_a_from_orbit(&base, &base_orbit)
        .map_err(|error| format!("base action derivative failed: {error:?}"))?;
    let d_volume = volume_derivatives_a(
        &base_polytope.dual_vertices_f64,
        &base_polytope.vertices_f64,
        &base_polytope.vertex_facet_incidence,
    )
    .map_err(|error| format!("base volume derivative failed: {error:?}"))?;
    let d_ratio =
        systolic_ratio_gradient_a(base_orbit.action, base_f64_volume, &d_action, &d_volume);
    let analytic_action = directional_derivative_a(&d_action, &direction);
    let analytic_volume = directional_derivative_a(&d_volume, &direction);
    let analytic_ratio = directional_derivative_a(&d_ratio, &direction);
    let base_ratio = systolic_ratio(base_orbit.action, base_f64_volume);
    let base_point = audit_point(&base, &sigma, 0, 0.0)?;
    let (base_kkt_matrix, _) = build_augmented_system_from_dual_vertices(&base, &sigma);
    let base_norm = l2_norm(&base);

    let mut radii = Vec::new();
    for normalized_radius in AUDIT_RADII {
        let absolute_radius = normalized_radius * base_norm;
        let plus_duals = add_flat_direction(&base, &direction_flat, absolute_radius);
        let minus_duals = add_flat_direction(&base, &direction_flat, -absolute_radius);
        let plus = audit_point(&plus_duals, &sigma, 1, normalized_radius)?;
        let minus = audit_point(&minus_duals, &sigma, -1, normalized_radius)?;
        let (plus_kkt_matrix, _) = build_augmented_system_from_dual_vertices(&plus_duals, &sigma);
        let (minus_kkt_matrix, _) = build_augmented_system_from_dual_vertices(&minus_duals, &sigma);
        let kkt_perturbation = (&plus_kkt_matrix - &base_kkt_matrix)
            .norm()
            .max((&minus_kkt_matrix - &base_kkt_matrix).norm());
        radii.push(RadiusAudit {
            normalized_radius,
            absolute_radius,
            kkt_frobenius_perturbation_over_base_eigen_gap: kkt_perturbation
                / base_point.kkt.minimum_abs_eigenvalue,
            action: comparison(
                analytic_action,
                base_orbit.action,
                plus.action,
                minus.action,
                absolute_radius,
            ),
            f64_volume: comparison(
                analytic_volume,
                base_f64_volume,
                Some(plus.f64_volume),
                Some(minus.f64_volume),
                absolute_radius,
            ),
            exact_volume: comparison(
                analytic_volume,
                base_exact_volume,
                Some(plus.exact_volume),
                Some(minus.exact_volume),
                absolute_radius,
            ),
            branch_ratio_f64_volume: comparison(
                analytic_ratio,
                base_ratio,
                plus.branch_ratio_f64_volume,
                minus.branch_ratio_f64_volume,
                absolute_radius,
            ),
            plus,
            minus,
        });
    }

    Ok(CaseAudit {
        role: role.to_string(),
        state_id: state.state_id.clone(),
        source_delta_sys,
        sigma,
        base_norm,
        source_absolute_distance: source_distance,
        base_action: base_orbit.action,
        base_f64_volume,
        base_exact_volume,
        base_branch_ratio_f64_volume: base_ratio,
        analytic_action_directional: analytic_action,
        analytic_f64_volume_directional: analytic_volume,
        analytic_branch_ratio_directional: analytic_ratio,
        base: base_point,
        radii,
    })
}

fn audit_point(
    duals: &[Vector4<f64>],
    sigma: &[usize],
    sign: i8,
    normalized_radius: f64,
) -> Result<PointAudit, String> {
    let f64_config = config(GeometryMode::F64, VolumeMode::F64);
    let exact_config = config(GeometryMode::Exact, VolumeMode::Exact);
    let payload =
        f64_geometry_payload(duals).map_err(|_| "f64 geometry payload failed".to_string())?;
    let (f64_polytope, f64_volume) = reconstruct_geometry_and_volume(duals, &f64_config)?;
    let (exact_polytope, exact_volume) = reconstruct_geometry_and_volume(duals, &exact_config)?;
    let orbit = solve_orbit_sigma_saddle_point(duals, sigma).ok();
    let branch_ratio = orbit
        .as_ref()
        .map(|orbit| systolic_ratio(orbit.action, f64_volume));
    Ok(PointAudit {
        sign,
        normalized_radius,
        action: orbit.as_ref().map(|orbit| orbit.action),
        action_lower: orbit.as_ref().map(|orbit| orbit.action_lower),
        action_upper: orbit.as_ref().map(|orbit| orbit.action_upper),
        beta: orbit.as_ref().map(|orbit| orbit.beta.clone()),
        beta_margin: orbit.as_ref().map(|orbit| orbit.beta_margin),
        admissibility: orbit
            .as_ref()
            .map(|orbit| format!("{:?}", orbit.admissibility)),
        branch_ratio_f64_volume: branch_ratio,
        f64_volume,
        exact_volume,
        f64_vs_exact_volume_relative_error: relative_error(f64_volume, exact_volume),
        f64_exact_incidence_agree: canonical_incidence(&f64_polytope.vertex_facet_incidence)
            == canonical_incidence(&exact_polytope.vertex_facet_incidence),
        f64_exact_facet_intersections_agree: f64_polytope.facet_intersection_is_nonempty
            == exact_polytope.facet_intersection_is_nonempty,
        f64_exact_omega_signs_agree: f64_polytope.omega_signs == exact_polytope.omega_signs,
        f64_vertex_count: f64_polytope.vertices_f64.len(),
        exact_vertex_count: exact_polytope.vertices_f64.len(),
        vertex_indeterminate_count: payload.vertex_indeterminate_count,
        bounded_near_singular_vertex_count: payload.bounded_near_singular_vertex_count,
        ambiguous_vertex_incidence_count: payload.ambiguous_vertex_incidence_count,
        facet_intersection_indeterminate_count: payload.facet_intersection_indeterminate_count,
        omega_indeterminate_count: payload.omega_indeterminate_count,
        kkt: kkt_audit(duals, sigma),
    })
}

fn kkt_audit(duals: &[Vector4<f64>], sigma: &[usize]) -> KktAudit {
    let (matrix, rhs) = build_augmented_system_from_dual_vertices(duals, sigma);
    let eigen = matrix.clone().symmetric_eigen();
    let max_abs = eigen
        .eigenvalues
        .iter()
        .map(|value| value.abs())
        .fold(0.0_f64, f64::max);
    let min_abs = eigen
        .eigenvalues
        .iter()
        .map(|value| value.abs())
        .fold(f64::INFINITY, f64::min);
    let closest_to_zero = eigen
        .eigenvalues
        .iter()
        .copied()
        .min_by(|left, right| left.abs().total_cmp(&right.abs()))
        .unwrap_or(f64::NAN);
    let raw_negative_eigenvalue_count =
        eigen.eigenvalues.iter().filter(|value| **value < 0.0).count();
    let retained = eigen
        .eigenvalues
        .iter()
        .map(|value| value.abs())
        .filter(|value| *value > 1.0e-12)
        .collect::<Vec<_>>();
    let min_retained = retained.iter().copied().fold(f64::INFINITY, f64::min);
    let permissive_rank = retained.len();
    let strict_rank = eigen
        .eigenvalues
        .iter()
        .filter(|value| value.abs() > max_abs * 1.0e-3)
        .count();

    let outcome = solve_kkt_for_dual_vertices(duals, sigma);
    let (label, residual, n_positive, n_negative, n_zero) = match outcome {
        KktOutcome::Feasible(result) => {
            let mut solution = result.beta.clone();
            solution.extend(result.mu.iter().copied());
            solution.push(result.xi);
            let residual = (&matrix * DVector::from_vec(solution) - &rhs).norm();
            (
                "feasible",
                Some(residual),
                Some(result.n_positive),
                Some(result.n_negative),
                Some(result.n_zero),
            )
        }
        KktOutcome::Infeasible => ("infeasible", None, None, None, None),
        KktOutcome::SingularMatrix => ("singular_matrix", None, None, None, None),
        KktOutcome::TypeCViolation => ("type_c_violation", None, None, None, None),
        KktOutcome::ConstraintViolation => ("constraint_violation", None, None, None, None),
    };
    KktAudit {
        outcome: label.to_string(),
        matrix_size: matrix.nrows(),
        maximum_abs_eigenvalue: max_abs,
        minimum_abs_eigenvalue: min_abs,
        closest_to_zero_eigenvalue: closest_to_zero,
        raw_negative_eigenvalue_count,
        permissive_rank_1e_12: permissive_rank,
        strict_rank_relative_1e_3: strict_rank,
        retained_condition_number_1e_12: min_retained.is_finite().then_some(max_abs / min_retained),
        residual_l2: residual,
        solver_n_positive: n_positive,
        solver_n_negative: n_negative,
        solver_n_zero: n_zero,
    }
}

fn comparison(
    analytic: f64,
    base: f64,
    plus: Option<f64>,
    minus: Option<f64>,
    h: f64,
) -> DerivativeComparison {
    let forward = plus.map(|value| (value - base) / h);
    let backward = minus.map(|value| (base - value) / h);
    let central = plus
        .zip(minus)
        .map(|(plus, minus)| (plus - minus) / (2.0 * h));
    DerivativeComparison {
        analytic,
        forward,
        backward,
        central,
        central_relative_error: central.map(|value| relative_error(analytic, value)),
    }
}

fn config(geometry_mode: GeometryMode, volume_mode: VolumeMode) -> EvaluatorConfig {
    EvaluatorConfig {
        geometry_mode,
        volume_mode,
        accept_indeterminate_geometry: true,
        exact_geometry_fallback: false,
        cache_within_run: false,
    }
}

fn canonical_incidence(incidence: &DMatrix<bool>) -> Vec<Vec<usize>> {
    let mut rows = (0..incidence.nrows())
        .map(|row| {
            (0..incidence.ncols())
                .filter(|&col| incidence[(row, col)])
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    rows.sort();
    rows
}

fn relative_error(left: f64, right: f64) -> f64 {
    (left - right).abs() / left.abs().max(right.abs()).max(1.0e-15)
}

fn read_jsonl<T: for<'de> Deserialize<'de>>(path: &Path) -> Result<Vec<T>, String> {
    BufReader::new(File::open(path).map_err(|error| format!("open jsonl: {error}"))?)
        .lines()
        .filter_map(|line| match line {
            Ok(line) if line.trim().is_empty() => None,
            other => Some(other),
        })
        .map(|line| {
            serde_json::from_str(&line.map_err(|error| error.to_string())?)
                .map_err(|error| error.to_string())
        })
        .collect()
}

fn parse_cli() -> Result<Cli, String> {
    let mut input = None;
    let mut candidates = None;
    let mut out = None;
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--input" => input = args.next().map(PathBuf::from),
            "--candidates" => candidates = args.next().map(PathBuf::from),
            "--out" => out = args.next().map(PathBuf::from),
            other => return Err(format!("unknown argument: {other}")),
        }
    }
    Ok(Cli {
        input: input.ok_or("missing --input")?,
        candidates: candidates.ok_or("missing --candidates")?,
        out: out.ok_or("missing --out")?,
    })
}

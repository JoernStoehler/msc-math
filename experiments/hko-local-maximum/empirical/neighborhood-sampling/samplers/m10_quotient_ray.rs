//! Event-labelled affine rays in the full-dimensional Euclidean HKO local slice.
//!
//! This is a finite shell screen in one fixed labelled-coordinate gauge. It does
//! not compute a global quotient distance, certify segmentwise chart validity,
//! assume monotonicity, or exclude thin tubes between sampled rays.

use crate::flat_polytope::HkoPolytopeCache;
use euclidean_polytopes::{all_points_are_extreme_exact, origin_in_interior_of_conv_exact};
use exp_hko_local_maximum::{capacity_auto, capacity_pruned_hk2017, euclidean_volume_f64};
use nalgebra::{DMatrix, DVector, Matrix4, Vector4};
use num_rational::BigRational;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use rand_distr::{Distribution, StandardNormal};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Instant;
use symplectic::geom::rational_arithmetic::f64_to_rational;
use symplectic::{classify_facets_from_dual_vertices, geom::known_polytopes, OrbitSearchResult};

const DEFAULT_SEED: u64 = 44;
const DEFAULT_DIRECTIONS: usize = 32;
const DEFAULT_R_MAX: f64 = 0.5;
const DEFAULT_BISECT_TOL: f64 = 1.0e-4;
const SMOKE_R_MAX: f64 = 3.0e-4;
const SHELLS: &[f64] = &[1e-4, 3e-4, 1e-3, 3e-3, 1e-2, 3e-2, 1e-1, 3e-1, 5e-1];
const POST_FACTORS: &[f64] = &[1.25, 1.5, 2.0, 3.0];
const ROTATED_CONTROL_THETAS: &[f64] = &[
    0.0,
    std::f64::consts::PI / 20.0,
    std::f64::consts::PI / 10.0,
];
const GS_TOL: f64 = 1.0e-11;
const SIGN_TOL: f64 = 1.0e-14;
const RESIDUAL_TOL: f64 = 2.0e-10;
const GAUGE_WARNING: f64 = 1.0e-6;
const GAUGE_SENSITIVITY: &[f64] = &[1.0e-4, 1.0e-6, 1.0e-8];
const CONTROL_TOL: f64 = 2.0e-9;
const MAX_BISECT_ITERS: usize = 64;
const COMPILED_SAMPLER_SOURCE: &[u8] = include_bytes!("m10_quotient_ray.rs");

#[derive(Clone, Debug)]
struct Cli {
    out_dir: PathBuf,
    seed: u64,
    directions: usize,
    r_max: f64,
    bisect_tol: f64,
    smoke: bool,
    frozen_panel: bool,
    launch_packet: Option<PathBuf>,
}

#[derive(Clone, Debug)]
struct QuotientBasis {
    orbit: Vec<DVector<f64>>,
    slice: Vec<DVector<f64>>,
    orbit_residual: f64,
    slice_residual: f64,
    cross_residual: f64,
}

#[derive(Clone, Debug, Serialize)]
struct BasisPayload {
    format: &'static str,
    coordinate_order: &'static str,
    facet_order: &'static str,
    generator_order: Vec<String>,
    gram_schmidt_tolerance: f64,
    sign_convention: &'static str,
    sign_threshold: f64,
    ambient_dimension: usize,
    orbit_rank: usize,
    slice_rank: usize,
    orbit_orthonormal_residual_max: f64,
    slice_orthonormal_residual_max: f64,
    orbit_slice_residual_max: f64,
    residual_gate: f64,
    hko_dual_vertices: Vec<[f64; 4]>,
    orbit_basis: Vec<Vec<[f64; 4]>>,
    slice_basis: Vec<Vec<[f64; 4]>>,
    producer_blake3: String,
    dependency_blake3: Vec<FileIdentity>,
}

#[derive(Debug, Serialize)]
struct BasisArtifact {
    basis_content_blake3: String,
    basis: BasisPayload,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
struct FileIdentity {
    path: String,
    blake3: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
struct TreeIdentity {
    path: String,
    blake3: String,
    file_count: usize,
}

#[derive(Clone, Debug, Serialize)]
struct GitIdentity {
    commit: String,
    tree: String,
    clean: bool,
}

#[derive(Clone, Debug, Serialize)]
struct ToolchainIdentity {
    rustc_verbose: String,
    cargo_version: String,
    build_profile: &'static str,
}

#[derive(Debug, Serialize)]
struct ArtifactBundle {
    format: &'static str,
    artifacts: Vec<FileIdentity>,
    bundle_content_blake3: String,
}

#[derive(Debug, Serialize)]
struct ArtifactBundleRoot<'a> {
    format: &'static str,
    artifacts: &'a [FileIdentity],
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct LaunchSettings {
    seed: u64,
    directions: usize,
    r_max: f64,
    bisect_tol: f64,
    shells: Vec<f64>,
    post_transition_factors: Vec<f64>,
    nonlinear_rotated_control_thetas: Vec<f64>,
    gram_schmidt_tolerance: f64,
    sign_threshold: f64,
    residual_gate: f64,
    gauge_warning_threshold: f64,
    gauge_sensitivity_thresholds: Vec<f64>,
    control_tolerance: f64,
    maximum_bisection_iterations: usize,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct LaunchPacket {
    format: String,
    settings: LaunchSettings,
    expected_git_commit: String,
    expected_git_tree: String,
    expected_git_clean: bool,
    expected_compiled_sampler_source_blake3: String,
    expected_source_and_dependency_hashes: Vec<FileIdentity>,
    expected_local_source_tree_hashes: Vec<TreeIdentity>,
}

#[derive(Clone, Debug)]
struct LaunchPacketVerification {
    path: String,
    blake3: String,
    format: String,
}

#[derive(Debug, Serialize)]
struct Manifest {
    format: &'static str,
    claim_boundary: Vec<&'static str>,
    basis_content_blake3: String,
    source_and_dependency_hashes: Vec<FileIdentity>,
    local_source_tree_hashes: Vec<TreeIdentity>,
    compiled_sampler_source_blake3: String,
    executable_blake3: String,
    executable_path: String,
    git: GitIdentity,
    toolchain: ToolchainIdentity,
    exact_invocation: Vec<String>,
    working_directory: String,
    coordinate_order: &'static str,
    generator_order: Vec<String>,
    seed: u64,
    random_direction_count: usize,
    deterministic_sentinels: Vec<&'static str>,
    nonlinear_rotated_control_thetas: &'static [f64],
    shells: Vec<f64>,
    r_max: f64,
    transition_tolerance: f64,
    maximum_bisection_iterations: usize,
    gauge_rank_tolerance: f64,
    gauge_warning_threshold: f64,
    gauge_sensitivity_thresholds: &'static [f64],
    post_transition_factors: &'static [f64],
    control_tolerance: f64,
    basis_residual_gate: f64,
    basis_sign_threshold: f64,
    float_serialization: &'static str,
    event_stop_precedence: Vec<&'static str>,
    sampling_continuation_rule: &'static str,
    capacity_routes: Vec<&'static str>,
    smoke: bool,
    frozen_panel_requested: bool,
    launch_packet_path: Option<String>,
    launch_packet_blake3: Option<String>,
    launch_packet_format: Option<String>,
    launch_packet_verified: bool,
    canonical_target_predicate: bool,
    build_binding_residual: &'static str,
}

#[derive(Clone, Debug, Serialize)]
struct GaugeRecord {
    numerical_rank: usize,
    rank_tolerance: f64,
    orbit_orthonormal_residual_max: f64,
    base_orbit_overlap_max: f64,
    principal_angle_cosines: Vec<f64>,
    sigma_gauge: Option<f64>,
    sensitivity_warnings: Vec<f64>,
    label: String,
}

#[derive(Clone, Debug, Serialize)]
struct ProbeBracketRef {
    transition_index: usize,
    inside_radius: f64,
    outside_radius: f64,
    factor: f64,
}

#[derive(Clone, Debug, Serialize)]
struct EvaluationRow {
    evaluation_index: usize,
    ray_id: String,
    ray_kind: String,
    direction_index: Option<usize>,
    phase: String,
    post_probe_source_brackets: Vec<ProbeBracketRef>,
    radius: f64,
    coefficient_direction_s24: Vec<f64>,
    expanded_direction_40d: Vec<f64>,
    serialized_dual_vertices: Vec<[f64; 4]>,
    rationalized_dual_vertices: Option<Vec<[String; 4]>>,
    pointwise_exact_rational_geometry: bool,
    chart_label: String,
    chart_validity_reasons: Vec<String>,
    evaluator_reasons: Vec<String>,
    facet_count: Option<usize>,
    vertex_count: Option<usize>,
    incidence_signature: Option<Vec<String>>,
    all_facets_defining: bool,
    volume: Option<f64>,
    gauge: GaugeRecord,
    evaluator_label: String,
    resolved_backend: Option<String>,
    nominal_action: Option<f64>,
    action_lower: Option<f64>,
    action_upper: Option<f64>,
    sys_nominal: Option<f64>,
    sys_route_lower: Option<f64>,
    sys_route_upper: Option<f64>,
    route_side: String,
    nominal_side: String,
    returned_orbit_count: Option<usize>,
    orbit_iterations: Option<u64>,
    best_sigma: Option<Vec<usize>>,
    all_labels: Vec<String>,
    primary_stop_label: String,
    nominal_reentry_observed: bool,
    wall_seconds: f64,
}

#[derive(Clone, Debug, Serialize)]
struct TransitionRow {
    transition_index: usize,
    ray_id: String,
    transition_kind: String,
    label_inside: String,
    label_outside: String,
    inside_radius: f64,
    outside_radius: f64,
    tolerance: f64,
    midpoint_evaluations: usize,
    classification: &'static str,
}

#[derive(Clone, Debug, Serialize)]
struct EventLogEntry {
    event_log_order: usize,
    coarse_interval_index: Option<usize>,
    event_kind: String,
    radius_left: f64,
    radius_right: f64,
    label_left: String,
    label_right: String,
    ordering_qualification: &'static str,
}

#[derive(Debug, Serialize)]
struct ControlRow {
    control_id: String,
    expected: String,
    observed: String,
    passed: bool,
}

#[derive(Clone, Debug, Serialize)]
struct RayOutcome {
    ray_id: String,
    ray_kind: String,
    direction_index: Option<usize>,
    competing_risk_or_censor_label: String,
    last_shell_radius: f64,
    nominal_transition_observed: bool,
    route_side_indeterminate_observed: bool,
    post_transition_probe_count: usize,
    nominal_reentry_observed: bool,
    event_log: Vec<EventLogEntry>,
    terminal_observation_after_nominal_transition: bool,
    terminal_observation_label: Option<String>,
    no_reentry_claim: &'static str,
}

#[derive(Debug, Serialize)]
struct Summary {
    controls_passed: bool,
    evaluation_count: usize,
    capacity_evaluation_count: usize,
    transition_count: usize,
    random_directions_completed: usize,
    deterministic_sentinels_completed: usize,
    nominal_reentry_ray_count: usize,
    output_files: Vec<&'static str>,
    elapsed_seconds: f64,
    target_panel_executed: bool,
    canonical_target_predicate: bool,
}

#[derive(Clone, Copy)]
enum EvaluatorRoute {
    Auto,
    DirectPruned,
}

struct Output {
    evaluations: BufWriter<File>,
    transitions: BufWriter<File>,
    controls: BufWriter<File>,
    ray_outcomes: BufWriter<File>,
    next_evaluation: usize,
    capacity_evaluations: usize,
    transition_count: usize,
    nominal_reentry_rays: usize,
}

fn print_usage() {
    eprintln!(
        r#"Usage: hko-neighborhood-sampling m10-quotient-ray --out-dir PATH [options]

Required:
  --out-dir PATH       New or empty output directory.

Options:
  --seed N             ChaCha8 seed (default: 44).
  --directions N       Random S^24 directions (default: 32; smoke: 1).
  --r-max X            Maximum radius (default: 0.5; smoke capped at 3e-4).
  --bisect-tol X       Absolute transition tolerance (default: 1e-4).
  --smoke              Minimal controls/sentinels plus one short random ray.
  --frozen-panel       Require a reviewed external launch packet for the 32-ray panel.
  --launch-packet PATH Reviewed packet required by --frozen-panel.
  --help, -h           Show this help."#
    );
}

fn parse_args(raw: &[String]) -> Result<Cli, String> {
    let mut out_dir = None;
    let mut seed = DEFAULT_SEED;
    let mut directions = None;
    let mut r_max = DEFAULT_R_MAX;
    let mut bisect_tol = DEFAULT_BISECT_TOL;
    let mut smoke = false;
    let mut frozen_panel = false;
    let mut launch_packet = None;
    let mut i = 0;
    while i < raw.len() {
        let value = raw.get(i + 1);
        match raw[i].as_str() {
            "--help" | "-h" => {
                print_usage();
                std::process::exit(0);
            }
            "--smoke" => smoke = true,
            "--frozen-panel" => frozen_panel = true,
            "--launch-packet" => {
                launch_packet = Some(PathBuf::from(value.ok_or("--launch-packet needs PATH")?));
                i += 1;
            }
            "--out-dir" => {
                out_dir = Some(PathBuf::from(value.ok_or("--out-dir needs PATH")?));
                i += 1;
            }
            "--seed" => {
                seed = value
                    .ok_or("--seed needs N")?
                    .parse()
                    .map_err(|_| "invalid seed")?;
                i += 1;
            }
            "--directions" => {
                directions = Some(
                    value
                        .ok_or("--directions needs N")?
                        .parse()
                        .map_err(|_| "invalid direction count")?,
                );
                i += 1;
            }
            "--r-max" => {
                r_max = value
                    .ok_or("--r-max needs X")?
                    .parse()
                    .map_err(|_| "invalid r-max")?;
                i += 1;
            }
            "--bisect-tol" => {
                bisect_tol = value
                    .ok_or("--bisect-tol needs X")?
                    .parse()
                    .map_err(|_| "invalid bisection tolerance")?;
                i += 1;
            }
            other => return Err(format!("unknown argument: {other}")),
        }
        i += 1;
    }
    if !(r_max.is_finite() && r_max >= 1e-4) {
        return Err("r-max must be finite and at least 1e-4".into());
    }
    if !(bisect_tol.is_finite() && bisect_tol > 0.0) {
        return Err("bisect-tol must be finite and positive".into());
    }
    let directions = directions.unwrap_or(if smoke { 1 } else { DEFAULT_DIRECTIONS });
    if directions == 0 {
        return Err("directions must be positive".into());
    }
    if smoke {
        r_max = r_max.min(SMOKE_R_MAX);
    }
    if frozen_panel && smoke {
        return Err("--frozen-panel cannot be combined with --smoke".into());
    }
    if frozen_panel && launch_packet.is_none() {
        return Err("--frozen-panel requires --launch-packet PATH".into());
    }
    if !frozen_panel && launch_packet.is_some() {
        return Err("--launch-packet is only valid with --frozen-panel".into());
    }
    Ok(Cli {
        out_dir: out_dir.ok_or("--out-dir is required")?,
        seed,
        directions,
        r_max,
        bisect_tol,
        smoke,
        frozen_panel,
        launch_packet,
    })
}

fn generator_order() -> Vec<String> {
    let mut result = (0..4)
        .map(|k| format!("translation_{k}"))
        .collect::<Vec<_>>();
    result.push("positive_scaling".into());
    result.extend((0..4).map(|k| format!("sp4_A_{k}")));
    result.extend((0..3).map(|k| format!("sp4_B_symmetric_{k}")));
    result.extend((0..3).map(|k| format!("sp4_C_symmetric_{k}")));
    result
}

fn quotient_basis(duals: &[Vector4<f64>]) -> QuotientBasis {
    let orbit = orthonormalize(&symmetry_generators(duals), GS_TOL, true);
    let ambient = duals.len() * 4;
    let mut slice = Vec::with_capacity(ambient.saturating_sub(orbit.len()));
    for coordinate in 0..ambient {
        let mut candidate = DVector::zeros(ambient);
        candidate[coordinate] = 1.0;
        project_away(&mut candidate, &orbit);
        project_away(&mut candidate, &slice);
        project_away(&mut candidate, &orbit);
        project_away(&mut candidate, &slice);
        if candidate.norm() > 1.0e-10 {
            let mut axis = candidate.normalize();
            canonicalize_sign(&mut axis);
            slice.push(axis);
        }
    }
    QuotientBasis {
        orbit_residual: max_orthonormal_error(&orbit),
        slice_residual: max_orthonormal_error(&slice),
        cross_residual: max_cross_inner_product(&orbit, &slice),
        orbit,
        slice,
    }
}

fn symmetry_generators(duals: &[Vector4<f64>]) -> Vec<DVector<f64>> {
    let mut result = Vec::with_capacity(15);
    for coordinate in 0..4 {
        result.push(flatten(
            &duals.iter().map(|a| -a[coordinate] * a).collect::<Vec<_>>(),
        ));
    }
    result.push(flatten(&duals.iter().map(|a| -a).collect::<Vec<_>>()));
    for x in sp4_basis() {
        result.push(flatten(
            &duals.iter().map(|a| -x.transpose() * a).collect::<Vec<_>>(),
        ));
    }
    result
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

fn orthonormalize(vectors: &[DVector<f64>], tolerance: f64, fix_sign: bool) -> Vec<DVector<f64>> {
    let mut basis = Vec::new();
    for source in vectors {
        let mut candidate = source.clone();
        project_away(&mut candidate, &basis);
        project_away(&mut candidate, &basis);
        let norm = candidate.norm();
        if norm > tolerance * source.norm().max(1.0) {
            candidate /= norm;
            if fix_sign {
                canonicalize_sign(&mut candidate);
            }
            basis.push(candidate);
        }
    }
    basis
}

fn canonicalize_sign(vector: &mut DVector<f64>) {
    if vector
        .iter()
        .copied()
        .find(|x| x.abs() > SIGN_TOL)
        .is_some_and(|x| x < 0.0)
    {
        *vector *= -1.0;
    }
}

fn project_away(vector: &mut DVector<f64>, basis: &[DVector<f64>]) {
    for axis in basis {
        *vector -= axis * axis.dot(vector);
    }
}

fn max_orthonormal_error(basis: &[DVector<f64>]) -> f64 {
    let mut result: f64 = 0.0;
    for (i, left) in basis.iter().enumerate() {
        for (j, right) in basis.iter().enumerate() {
            result = result.max((left.dot(right) - usize::from(i == j) as f64).abs());
        }
    }
    result
}

fn max_cross_inner_product(left: &[DVector<f64>], right: &[DVector<f64>]) -> f64 {
    left.iter()
        .flat_map(|a| right.iter().map(move |b| a.dot(b).abs()))
        .fold(0.0, f64::max)
}

fn flatten(vectors: &[Vector4<f64>]) -> DVector<f64> {
    DVector::from_iterator(
        vectors.len() * 4,
        vectors.iter().flat_map(|v| v.iter().copied()),
    )
}

fn unflatten(vector: &DVector<f64>) -> Vec<Vector4<f64>> {
    vector
        .as_slice()
        .chunks_exact(4)
        .map(|x| Vector4::new(x[0], x[1], x[2], x[3]))
        .collect()
}

fn nested_vectors(basis: &[DVector<f64>]) -> Vec<Vec<[f64; 4]>> {
    basis
        .iter()
        .map(|axis| vectors_to_arrays(&unflatten(axis)))
        .collect()
}

fn vectors_to_arrays(vectors: &[Vector4<f64>]) -> Vec<[f64; 4]> {
    vectors.iter().map(|v| [v[0], v[1], v[2], v[3]]).collect()
}

fn rational_string(value: &BigRational) -> String {
    format!("{}/{}", value.numer(), value.denom())
}

fn rationalized_arrays(duals: &[Vector4<BigRational>]) -> Vec<[String; 4]> {
    duals
        .iter()
        .map(|a| std::array::from_fn(|i| rational_string(&a[i])))
        .collect()
}

fn incidence_signature(matrix: &DMatrix<bool>) -> Vec<String> {
    let mut rows = (0..matrix.nrows())
        .map(|row| {
            (0..matrix.ncols())
                .filter(|&col| matrix[(row, col)])
                .map(|col| col.to_string())
                .collect::<Vec<_>>()
                .join(",")
        })
        .collect::<Vec<_>>();
    rows.sort();
    rows
}

fn gauge_record(duals: &[Vector4<f64>], base_orbit: &[DVector<f64>]) -> GaugeRecord {
    let orbit = orthonormalize(&symmetry_generators(duals), GS_TOL, false);
    let rank = orbit.len();
    let residual = max_orthonormal_error(&orbit);
    let cross = max_cross_inner_product(base_orbit, &orbit);
    let singular_values = if rank == 15 {
        let matrix = DMatrix::from_fn(15, 15, |i, j| base_orbit[i].dot(&orbit[j]));
        let mut values = matrix.svd(false, false).singular_values.as_slice().to_vec();
        values.sort_by(|a, b| b.total_cmp(a));
        values
    } else {
        Vec::new()
    };
    let sigma = singular_values.last().copied();
    let warnings = sigma
        .map(|s| {
            GAUGE_SENSITIVITY
                .iter()
                .copied()
                .filter(|&x| s <= x)
                .collect()
        })
        .unwrap_or_default();
    let label = if rank != 15 {
        "gauge_numeric_rank_event"
    } else if sigma.is_some_and(|s| s <= GAUGE_WARNING) {
        "gauge_near_tangency_warning"
    } else {
        "gauge_nominal"
    };
    GaugeRecord {
        numerical_rank: rank,
        rank_tolerance: GS_TOL,
        orbit_orthonormal_residual_max: residual,
        base_orbit_overlap_max: cross,
        principal_angle_cosines: singular_values,
        sigma_gauge: sigma,
        sensitivity_warnings: warnings,
        label: label.into(),
    }
}

fn exact_rational_geometry(
    duals: &[Vector4<f64>],
) -> Result<(HkoPolytopeCache, Vec<Vector4<BigRational>>), String> {
    if duals.len() != 10 {
        return Err(format!("facet_count_{}_not_10", duals.len()));
    }
    if duals
        .iter()
        .any(|a| !a.iter().all(|x| x.is_finite()) || a.norm() < 1e-15)
    {
        return Err("nonfinite_or_near_zero_dual".into());
    }
    let exact = duals
        .iter()
        .map(|a| {
            Vector4::new(
                f64_to_rational(a[0]),
                f64_to_rational(a[1]),
                f64_to_rational(a[2]),
                f64_to_rational(a[3]),
            )
        })
        .collect::<Vec<_>>();
    if !origin_in_interior_of_conv_exact(&exact) {
        return Err("origin_not_in_exact_rationalized_interior".into());
    }
    if !all_points_are_extreme_exact(&exact) {
        return Err("not_all_exact_rationalized_duals_extreme".into());
    }
    let arrays = exact
        .iter()
        .map(|a| std::array::from_fn(|i| a[i].clone()))
        .collect();
    let cache = HkoPolytopeCache::new(arrays, Some(duals.to_vec()))
        .ok_or_else(|| "exact_rational_cache_reconstruction_failed".to_string())?;
    Ok((cache, exact))
}

fn primary_label(chart: &str, gauge: &str, evaluator: &str, route: &str, nominal: &str) -> String {
    if chart != "chart_nominal" {
        chart.into()
    } else if gauge != "gauge_nominal" {
        gauge.into()
    } else if evaluator != "evaluator_available" {
        evaluator.into()
    } else if route == "route_side_indeterminate" {
        route.into()
    } else {
        nominal.into()
    }
}

fn authoritative_nominal_side(
    chart: &str,
    gauge: &str,
    evaluator: &str,
    sys_nominal: Option<f64>,
) -> &'static str {
    if chart != "chart_nominal" || gauge != "gauge_nominal" || evaluator != "evaluator_available" {
        return "nominal_side_unavailable";
    }
    match sys_nominal {
        Some(value) if value.is_finite() && value > 1.0 => "nominal_above_one",
        Some(value) if value.is_finite() => "nominal_below_or_equal_one",
        _ => "nominal_side_unavailable",
    }
}

fn action_data_issue(
    nominal_action: Option<f64>,
    action_lower: Option<f64>,
    action_upper: Option<f64>,
) -> Option<&'static str> {
    let (Some(action), Some(lower), Some(upper)) = (nominal_action, action_lower, action_upper)
    else {
        return Some("missing_action_data");
    };
    if !(action.is_finite() && action > 0.0) {
        return Some("nonpositive_or_nonfinite_nominal_action");
    }
    if !(lower.is_finite() && upper.is_finite() && lower >= 0.0 && upper >= lower) {
        return Some("unusable_action_interval");
    }
    if action < lower || action > upper {
        return Some("nominal_action_outside_action_interval");
    }
    None
}

fn authoritative_route_side(
    chart: &str,
    gauge: &str,
    evaluator: &str,
    nominal_action: Option<f64>,
    action_lower: Option<f64>,
    action_upper: Option<f64>,
    route_interval: Option<(f64, f64)>,
) -> &'static str {
    if chart != "chart_nominal"
        || gauge != "gauge_nominal"
        || evaluator != "evaluator_available"
        || action_data_issue(nominal_action, action_lower, action_upper).is_some()
    {
        return "route_side_unavailable";
    }
    match route_interval {
        Some((lower, upper))
            if lower.is_finite() && upper.is_finite() && lower <= upper && lower > 1.0 =>
        {
            "route_above_one"
        }
        Some((lower, upper))
            if lower.is_finite() && upper.is_finite() && lower <= upper && upper <= 1.0 =>
        {
            "route_below_or_equal_one"
        }
        Some((lower, upper)) if lower.is_finite() && upper.is_finite() && lower <= upper => {
            "route_side_indeterminate"
        }
        _ => "route_side_unavailable",
    }
}

fn is_authoritative_nominal_label(label: &str) -> bool {
    matches!(label, "nominal_above_one" | "nominal_below_or_equal_one")
}

#[allow(clippy::too_many_arguments)]
fn evaluate(
    output: &mut Output,
    ray_id: &str,
    ray_kind: &str,
    direction_index: Option<usize>,
    phase: &str,
    post_probe_source_brackets: &[ProbeBracketRef],
    radius: f64,
    coeffs: &[f64],
    direction: &DVector<f64>,
    duals: Vec<Vector4<f64>>,
    base_orbit: &[DVector<f64>],
    base_incidence: &[String],
    route: EvaluatorRoute,
) -> EvaluationRow {
    let started = Instant::now();
    let gauge = gauge_record(&duals, base_orbit);
    let mut chart_label = "chart_nominal".to_string();
    let mut chart_reasons = Vec::new();
    let mut evaluator_reasons = Vec::new();
    let mut rationalized = None;
    let mut facet_count = None;
    let mut vertex_count = None;
    let mut signature = None;
    let mut all_facets_defining = false;
    let mut volume = None;
    let mut evaluator_label = "evaluator_unavailable".to_string();
    let mut backend = None;
    let mut capacity: Option<OrbitSearchResult> = None;

    match exact_rational_geometry(&duals) {
        Err(reason) => {
            chart_label = "chart_invalid".into();
            chart_reasons.push(reason);
        }
        Ok((polytope, exact)) => {
            rationalized = Some(rationalized_arrays(&exact));
            facet_count = Some(polytope.facet_count());
            vertex_count = Some(polytope.vertices.len());
            let observed_signature = incidence_signature(&polytope.vertex_facet_incidence);
            all_facets_defining = (0..polytope.facet_count()).all(|col| {
                (0..polytope.vertex_facet_incidence.nrows())
                    .any(|row| polytope.vertex_facet_incidence[(row, col)])
            });
            if observed_signature != base_incidence {
                chart_label = "chart_invalid".into();
                chart_reasons.push("labelled_incidence_signature_changed".into());
            }
            if !all_facets_defining {
                chart_label = "chart_invalid".into();
                chart_reasons.push("not_all_facets_defining".into());
            }
            let vol = euclidean_volume_f64(&polytope.vertices, &polytope.vertex_facet_incidence);
            volume = Some(vol);
            if !(vol.is_finite() && vol > 0.0) {
                chart_label = "chart_invalid".into();
                chart_reasons.push("nonpositive_or_nonfinite_volume".into());
            }
            signature = Some(observed_signature);
            if chart_label == "chart_nominal" {
                let result = match route {
                    EvaluatorRoute::Auto => {
                        let selected =
                            if classify_facets_from_dual_vertices(&polytope.dual_vertices_f64)
                                .is_ok()
                            {
                                "auto:billiard"
                            } else {
                                "auto:pruned_hk2017"
                            };
                        backend = Some(selected.into());
                        capacity_auto(
                            &polytope.dual_vertices,
                            &polytope.dual_vertices_f64,
                            &polytope.facet_intersection_is_nonempty,
                            &polytope.omega_signs,
                        )
                    }
                    EvaluatorRoute::DirectPruned => {
                        backend = Some("direct:pruned_hk2017".into());
                        capacity_pruned_hk2017(
                            &polytope.dual_vertices,
                            &polytope.dual_vertices_f64,
                            &polytope.facet_intersection_is_nonempty,
                            &polytope.omega_signs,
                        )
                    }
                };
                output.capacity_evaluations += 1;
                match result {
                    Ok(value) => {
                        evaluator_label = "evaluator_available".into();
                        capacity = Some(value);
                    }
                    Err(error) => evaluator_reasons.push(format!("capacity_error:{error:?}")),
                }
            }
        }
    }

    let (nominal_action, action_lower, action_upper, orbit_count, iterations, best_sigma) =
        capacity
            .as_ref()
            .map(|c| {
                (
                    Some(c.min_action),
                    Some(c.min_action_lower),
                    Some(c.min_action_upper),
                    Some(c.orbits.len()),
                    Some(c.iterations),
                    Some(c.best_sigma().to_vec()),
                )
            })
            .unwrap_or((None, None, None, None, None, None));
    let raw_sys_nominal = nominal_action.zip(volume).map(|(a, v)| a * a / (2.0 * v));
    let route_interval = action_lower
        .zip(action_upper)
        .zip(volume)
        .and_then(|((lo, hi), v)| {
            if !(lo.is_finite() && hi.is_finite() && lo >= 0.0 && hi >= lo && v > 0.0) {
                return None;
            }
            let lower = lo * lo / (2.0 * v);
            let upper = hi * hi / (2.0 * v);
            (lower.is_finite() && upper.is_finite() && lower <= upper).then_some((lower, upper))
        });
    if capacity.is_some() {
        if let Some(reason) = action_data_issue(nominal_action, action_lower, action_upper) {
            evaluator_label = "evaluator_indeterminate".into();
            evaluator_reasons.push(reason.into());
        } else if route_interval.is_none() {
            evaluator_label = "evaluator_indeterminate".into();
            evaluator_reasons.push("unusable_action_interval".into());
        }
    }
    if raw_sys_nominal.is_some_and(|value| !value.is_finite()) {
        evaluator_label = "evaluator_indeterminate".into();
        evaluator_reasons.push("nonfinite_nominal_sys".into());
    }
    let sys_nominal = raw_sys_nominal.filter(|value| value.is_finite());
    let route_side = authoritative_route_side(
        &chart_label,
        &gauge.label,
        &evaluator_label,
        nominal_action,
        action_lower,
        action_upper,
        route_interval,
    )
    .to_string();
    let nominal_side =
        authoritative_nominal_side(&chart_label, &gauge.label, &evaluator_label, sys_nominal)
            .to_string();
    let mut labels = vec![
        chart_label.clone(),
        gauge.label.clone(),
        evaluator_label.clone(),
        route_side.clone(),
        nominal_side.clone(),
    ];
    labels.extend(chart_reasons.iter().map(|x| format!("chart_reason:{x}")));
    labels.extend(
        evaluator_reasons
            .iter()
            .map(|x| format!("evaluator_reason:{x}")),
    );
    let primary = primary_label(
        &chart_label,
        &gauge.label,
        &evaluator_label,
        &route_side,
        &nominal_side,
    );
    let nominal_reentry_observed = (phase.starts_with("post_transition_probe")
        || phase == "shell_after_observed_nominal_below")
        && chart_label == "chart_nominal"
        && gauge.label == "gauge_nominal"
        && evaluator_label == "evaluator_available"
        && nominal_side == "nominal_above_one";
    let row = EvaluationRow {
        evaluation_index: output.next_evaluation,
        ray_id: ray_id.into(),
        ray_kind: ray_kind.into(),
        direction_index,
        phase: phase.into(),
        post_probe_source_brackets: post_probe_source_brackets.to_vec(),
        radius,
        coefficient_direction_s24: coeffs.to_vec(),
        expanded_direction_40d: direction.as_slice().to_vec(),
        serialized_dual_vertices: vectors_to_arrays(&duals),
        rationalized_dual_vertices: rationalized,
        pointwise_exact_rational_geometry: chart_label == "chart_nominal",
        chart_label,
        chart_validity_reasons: chart_reasons,
        evaluator_reasons,
        facet_count,
        vertex_count,
        incidence_signature: signature,
        all_facets_defining,
        volume,
        gauge,
        evaluator_label,
        resolved_backend: backend,
        nominal_action: nominal_action.filter(|value| value.is_finite()),
        action_lower: action_lower.filter(|value| value.is_finite()),
        action_upper: action_upper.filter(|value| value.is_finite()),
        sys_nominal,
        sys_route_lower: route_interval.map(|x| x.0),
        sys_route_upper: route_interval.map(|x| x.1),
        route_side,
        nominal_side,
        returned_orbit_count: orbit_count,
        orbit_iterations: iterations,
        best_sigma,
        all_labels: labels,
        primary_stop_label: primary,
        nominal_reentry_observed,
        wall_seconds: started.elapsed().as_secs_f64(),
    };
    write_jsonl(&mut output.evaluations, &row);
    output.next_evaluation += 1;
    row
}

fn state_on_ray(base: &[Vector4<f64>], direction: &DVector<f64>, radius: f64) -> Vec<Vector4<f64>> {
    base.iter()
        .zip(unflatten(direction))
        .map(|(a, u)| a + radius * u)
        .collect()
}

fn classification(row: &EvaluationRow, kind: &str) -> String {
    match kind {
        "chart" => row.chart_label.clone(),
        "gauge" => row.gauge.label.clone(),
        "evaluator" => row.evaluator_label.clone(),
        "route_side" => row.route_side.clone(),
        "nominal_sys" => row.nominal_side.clone(),
        _ => unreachable!(),
    }
}

fn changed_kinds(left: &EvaluationRow, right: &EvaluationRow) -> Vec<&'static str> {
    ["chart", "gauge", "evaluator", "route_side", "nominal_sys"]
        .into_iter()
        .filter(|kind| classification(left, kind) != classification(right, kind))
        .collect()
}

#[allow(clippy::too_many_arguments)]
fn bisect_transition(
    output: &mut Output,
    ray_id: &str,
    ray_kind: &str,
    direction_index: Option<usize>,
    coeffs: &[f64],
    direction: &DVector<f64>,
    base: &[Vector4<f64>],
    base_orbit: &[DVector<f64>],
    base_incidence: &[String],
    kind: &str,
    left: &EvaluationRow,
    right: &EvaluationRow,
    tolerance: f64,
    all_rows: &mut Vec<EvaluationRow>,
) -> TransitionRow {
    let mut inside = left.radius;
    let mut outside = right.radius;
    let inside_label = classification(left, kind);
    let mut outside_label = classification(right, kind);
    let mut midpoint_evaluations = 0;
    while outside - inside > tolerance && midpoint_evaluations < MAX_BISECT_ITERS {
        let radius = (inside + outside) / 2.0;
        let midpoint = evaluate(
            output,
            ray_id,
            ray_kind,
            direction_index,
            &format!("bisection:{kind}"),
            &[],
            radius,
            coeffs,
            direction,
            state_on_ray(base, direction, radius),
            base_orbit,
            base_incidence,
            EvaluatorRoute::Auto,
        );
        midpoint_evaluations += 1;
        all_rows.push(midpoint.clone());
        let label = classification(&midpoint, kind);
        if label == inside_label {
            inside = radius;
        } else {
            outside = radius;
            outside_label = label;
        }
    }
    TransitionRow {
        transition_index: output.transition_count,
        ray_id: ray_id.into(),
        transition_kind: kind.into(),
        label_inside: inside_label,
        label_outside: outside_label,
        inside_radius: inside,
        outside_radius: outside,
        tolerance,
        midpoint_evaluations,
        classification: "shell_bracketed_transition",
    }
}

fn is_qualifying_nominal_transition(transition: &TransitionRow) -> bool {
    transition.transition_kind == "nominal_sys"
        && is_authoritative_nominal_label(&transition.label_inside)
        && is_authoritative_nominal_label(&transition.label_outside)
        && transition.label_inside == "nominal_above_one"
        && transition.label_outside == "nominal_below_or_equal_one"
}

fn nominal_reentry_witness_from_observations<'a>(
    observations: impl IntoIterator<Item = (f64, &'a str)>,
) -> Option<(f64, f64)> {
    let mut observations = observations
        .into_iter()
        .filter(|(radius, label)| radius.is_finite() && is_authoritative_nominal_label(label))
        .collect::<Vec<_>>();
    observations.sort_by(|left, right| left.0.total_cmp(&right.0));
    let mut below_radius: Option<f64> = None;
    for (radius, label) in observations {
        if label == "nominal_below_or_equal_one" {
            below_radius = Some(below_radius.map_or(radius, |old| old.min(radius)));
        } else if label == "nominal_above_one" && below_radius.is_some_and(|below| radius > below) {
            return Some((below_radius.expect("checked below radius"), radius));
        }
    }
    None
}

fn nominal_reentry_witness_from_rows(rows: &[EvaluationRow]) -> Option<(f64, f64)> {
    nominal_reentry_witness_from_observations(
        rows.iter()
            .map(|row| (row.radius, row.nominal_side.as_str())),
    )
}

fn route_side_indeterminate_from_labels<'a>(labels: impl IntoIterator<Item = &'a str>) -> bool {
    labels
        .into_iter()
        .any(|label| label == "route_side_indeterminate")
}

fn competing_risk_or_censor_label(
    nominal_transition_observed: bool,
    chart: &str,
    gauge: &str,
    evaluator: &str,
    last_radius: f64,
    r_max: f64,
) -> &'static str {
    if nominal_transition_observed {
        "nominal_shell_transition_observed"
    } else if chart != "chart_nominal" {
        "chart_competing_observation_limit"
    } else if gauge != "gauge_nominal" {
        "gauge_competing_observation_limit"
    } else if evaluator != "evaluator_available" {
        "evaluator_competing_observation_limit"
    } else if last_radius == r_max {
        "nominal_trace_right_censored_at_r_max"
    } else {
        "sampling_ended_without_declared_outcome"
    }
}

fn plan_post_transition_probes(
    transitions: &[TransitionRow],
    r_max: f64,
) -> BTreeMap<u64, Vec<ProbeBracketRef>> {
    let mut radii: BTreeMap<u64, Vec<ProbeBracketRef>> = BTreeMap::new();
    for transition in transitions {
        let midpoint = (transition.inside_radius + transition.outside_radius) / 2.0;
        for &factor in POST_FACTORS {
            let candidate = (factor * midpoint).min(r_max);
            if candidate > transition.outside_radius {
                radii
                    .entry(candidate.to_bits())
                    .or_default()
                    .push(ProbeBracketRef {
                        transition_index: transition.transition_index,
                        inside_radius: transition.inside_radius,
                        outside_radius: transition.outside_radius,
                        factor,
                    });
            }
        }
    }
    radii
}

fn is_terminal(row: &EvaluationRow) -> bool {
    row.chart_label != "chart_nominal"
        || row.gauge.label != "gauge_nominal"
        || row.evaluator_label != "evaluator_available"
}

#[allow(clippy::too_many_arguments)]
fn run_ray(
    output: &mut Output,
    ray_id: &str,
    ray_kind: &str,
    direction_index: Option<usize>,
    coeffs: &[f64],
    direction: &DVector<f64>,
    base: &[Vector4<f64>],
    basis: &QuotientBasis,
    base_incidence: &[String],
    shells: &[f64],
    r_max: f64,
    tolerance: f64,
) -> RayOutcome {
    let mut rows = Vec::new();
    rows.push(evaluate(
        output,
        ray_id,
        ray_kind,
        direction_index,
        "origin",
        &[],
        0.0,
        coeffs,
        direction,
        base.to_vec(),
        &basis.orbit,
        base_incidence,
        EvaluatorRoute::Auto,
    ));
    let mut all_rows = rows.clone();
    let mut nominal_below_seen = false;
    for &radius in shells {
        let row = evaluate(
            output,
            ray_id,
            ray_kind,
            direction_index,
            if nominal_below_seen {
                "shell_after_observed_nominal_below"
            } else {
                "shell"
            },
            &[],
            radius,
            coeffs,
            direction,
            state_on_ray(base, direction, radius),
            &basis.orbit,
            base_incidence,
            EvaluatorRoute::Auto,
        );
        let terminal = is_terminal(&row);
        nominal_below_seen |= row.nominal_side == "nominal_below_or_equal_one";
        all_rows.push(row.clone());
        rows.push(row);
        if terminal {
            break;
        }
    }

    let mut nominal_transitions = Vec::new();
    let mut chronology = Vec::new();
    for (coarse_interval_index, pair) in rows.windows(2).enumerate() {
        for kind in changed_kinds(&pair[0], &pair[1]) {
            let transition = bisect_transition(
                output,
                ray_id,
                ray_kind,
                direction_index,
                coeffs,
                direction,
                base,
                &basis.orbit,
                base_incidence,
                kind,
                &pair[0],
                &pair[1],
                tolerance,
                &mut all_rows,
            );
            if is_qualifying_nominal_transition(&transition) {
                nominal_transitions.push(transition.clone());
            }
            chronology.push(EventLogEntry {
                event_log_order: chronology.len(),
                coarse_interval_index: Some(coarse_interval_index),
                event_kind: format!("shell_bracketed_{}", transition.transition_kind),
                radius_left: transition.inside_radius,
                radius_right: transition.outside_radius,
                label_left: transition.label_inside.clone(),
                label_right: transition.label_outside.clone(),
                ordering_qualification: "labels changed on this adjacent evaluated pair; events sharing an interval are simultaneous observations and have no inferred within-interval order",
            });
            write_jsonl(&mut output.transitions, &transition);
            output.transition_count += 1;
        }
    }

    let mut post_transition_probe_count = 0;
    let radii = plan_post_transition_probes(&nominal_transitions, r_max);
    for (bits, provenance) in radii {
        let radius = f64::from_bits(bits);
        let row = evaluate(
            output,
            ray_id,
            ray_kind,
            direction_index,
            "post_transition_probe",
            &provenance,
            radius,
            coeffs,
            direction,
            state_on_ray(base, direction, radius),
            &basis.orbit,
            base_incidence,
            EvaluatorRoute::Auto,
        );
        post_transition_probe_count += 1;
        all_rows.push(row);
    }

    let last = rows.last().expect("ray has origin evaluation");
    let competing_risk_or_censor_label = competing_risk_or_censor_label(
        !nominal_transitions.is_empty(),
        &last.chart_label,
        &last.gauge.label,
        &last.evaluator_label,
        last.radius,
        r_max,
    );
    let route_side_indeterminate_observed =
        route_side_indeterminate_from_labels(all_rows.iter().map(|row| row.route_side.as_str()));
    let nominal_reentry_witness = nominal_reentry_witness_from_rows(&all_rows);
    let nominal_reentry_observed = nominal_reentry_witness.is_some();
    let terminal = is_terminal(last);
    if let Some((below_radius, above_radius)) = nominal_reentry_witness {
        chronology.push(EventLogEntry {
            event_log_order: chronology.len(),
            coarse_interval_index: None,
            event_kind: "nominal_reentry_observed".into(),
            radius_left: below_radius,
            radius_right: above_radius,
            label_left: "nominal_below_or_equal_one".into(),
            label_right: "nominal_above_one".into(),
            ordering_qualification:
                "derived only from radial ordering of retained authoritative evaluations",
        });
    }
    if terminal {
        chronology.push(EventLogEntry {
            event_log_order: chronology.len(),
            coarse_interval_index: None,
            event_kind: "terminal_observation_limit".into(),
            radius_left: last.radius,
            radius_right: last.radius,
            label_left: last.primary_stop_label.clone(),
            label_right: last.primary_stop_label.clone(),
            ordering_qualification: "direct terminal coarse-shell observation; earlier retained transition records remain valid observations",
        });
    }
    let outcome = RayOutcome {
        ray_id: ray_id.into(),
        ray_kind: ray_kind.into(),
        direction_index,
        competing_risk_or_censor_label: competing_risk_or_censor_label.into(),
        last_shell_radius: last.radius,
        nominal_transition_observed: !nominal_transitions.is_empty(),
        route_side_indeterminate_observed,
        post_transition_probe_count,
        nominal_reentry_observed,
        event_log: chronology,
        terminal_observation_after_nominal_transition: terminal && !nominal_transitions.is_empty(),
        terminal_observation_label: terminal.then(|| last.primary_stop_label.clone()),
        no_reentry_claim:
            "false means only that no re-entry was directly seen among retained shell, bisection, and declared post-probe evaluations",
    };
    write_jsonl(&mut output.ray_outcomes, &outcome);
    if outcome.nominal_reentry_observed {
        output.nominal_reentry_rays += 1;
    }
    outcome
}

fn random_coefficients(rng: &mut ChaCha8Rng, dimension: usize) -> Vec<f64> {
    let mut values = (0..dimension)
        .map(|_| StandardNormal.sample(rng))
        .collect::<Vec<f64>>();
    let norm = values.iter().map(|x| x * x).sum::<f64>().sqrt();
    for value in &mut values {
        *value /= norm;
    }
    values
}

fn expand_coefficients(coeffs: &[f64], basis: &[DVector<f64>]) -> DVector<f64> {
    let mut result = DVector::zeros(basis[0].len());
    for (coefficient, axis) in coeffs.iter().zip(basis) {
        result += axis * *coefficient;
    }
    result
}

fn rotated_pentagon(theta: f64) -> Vec<Vector4<f64>> {
    let h = (std::f64::consts::PI / 5.0).cos();
    (0..5)
        .map(|i| {
            let angle = std::f64::consts::FRAC_PI_2 + 2.0 * std::f64::consts::PI * i as f64 / 5.0;
            Vector4::new(angle.cos() / h, angle.sin() / h, 0.0, 0.0)
        })
        .chain((0..5).map(|i| {
            let angle =
                std::f64::consts::FRAC_PI_2 + 2.0 * std::f64::consts::PI * i as f64 / 5.0 + theta;
            Vector4::new(0.0, 0.0, angle.cos() / h, angle.sin() / h)
        }))
        .collect()
}

fn pentagon_profile(theta: f64) -> f64 {
    let period = std::f64::consts::PI / 5.0;
    let nearest = (theta / period).round() * period;
    let distance = (theta - nearest).abs();
    (5.0 + 2.0 * 5.0_f64.sqrt()) / 10.0 / distance.cos().powi(2)
}

fn rotated_tangent_in_slice(base: &[Vector4<f64>], basis: &QuotientBasis) -> DVector<f64> {
    let tangent = base
        .iter()
        .enumerate()
        .map(|(i, a)| {
            if i < 5 {
                Vector4::zeros()
            } else {
                // Derivative through the serialized HKO state when only its
                // p-factor follows the actual nonlinear rotation family.
                Vector4::new(0.0, 0.0, -a[3], a[2])
            }
        })
        .collect::<Vec<_>>();
    let mut flat = flatten(&tangent);
    project_away(&mut flat, &basis.orbit);
    flat.normalize()
}

fn write_control(writer: &mut BufWriter<File>, control: ControlRow) {
    write_jsonl(writer, &control);
    assert!(
        control.passed,
        "control failed: {} ({})",
        control.control_id, control.observed
    );
}

fn approximately_equal(left: f64, right: f64, tolerance: f64) -> bool {
    left.is_finite()
        && right.is_finite()
        && (left - right).abs() <= tolerance.max(tolerance * left.abs().max(right.abs()))
}

fn run_controls(
    output: &mut Output,
    base: &[Vector4<f64>],
    basis: &QuotientBasis,
    base_incidence: &[String],
) {
    let zero = DVector::zeros(40);
    let expected_hko = (3.0 + 5.0_f64.sqrt()) / 5.0;
    let auto = evaluate(
        output,
        "control_hko_auto",
        "control",
        None,
        "control",
        &[],
        0.0,
        &[],
        &zero,
        base.to_vec(),
        &basis.orbit,
        base_incidence,
        EvaluatorRoute::Auto,
    );
    let direct = evaluate(
        output,
        "control_hko_direct_pruned",
        "control",
        None,
        "control",
        &[],
        0.0,
        &[],
        &zero,
        base.to_vec(),
        &basis.orbit,
        base_incidence,
        EvaluatorRoute::DirectPruned,
    );
    let auto_sys = auto.sys_nominal.expect("base auto sys");
    let direct_sys = direct.sys_nominal.expect("base direct sys");
    write_control(
        &mut output.controls,
        ControlRow {
            control_id: "serialized_rationalized_hko_geometry_and_known_value".into(),
            expected: format!("serialized rationalized fixture chart nominal and sys={expected_hko:.16e} within {CONTROL_TOL}"),
            observed: format!("chart={}, sys={auto_sys:.16e}", auto.chart_label),
            passed: auto.chart_label == "chart_nominal"
                && (auto_sys - expected_hko).abs() <= CONTROL_TOL,
        },
    );
    write_control(
        &mut output.controls,
        ControlRow {
            control_id: "auto_vs_direct_pruned_at_hko".into(),
            expected: format!("backends, nominal sys, action/sys interval endpoints, and route-side label agree within {CONTROL_TOL}"),
            observed: format!(
                "auto_backend={:?}, direct_backend={:?}, auto_sys={auto_sys:.16e}, direct_sys={direct_sys:.16e}, auto_action_interval={:?}..{:?}, direct_action_interval={:?}..{:?}, auto_sys_interval={:?}..{:?}, direct_sys_interval={:?}..{:?}, route_sides={}..{}",
                auto.resolved_backend,
                direct.resolved_backend,
                auto.action_lower,
                auto.action_upper,
                direct.action_lower,
                direct.action_upper,
                auto.sys_route_lower,
                auto.sys_route_upper,
                direct.sys_route_lower,
                direct.sys_route_upper,
                auto.route_side,
                direct.route_side,
            ),
            passed: auto.resolved_backend.as_deref() == Some("auto:billiard")
                && direct.resolved_backend.as_deref() == Some("direct:pruned_hk2017")
                && approximately_equal(auto_sys, direct_sys, CONTROL_TOL)
                && auto
                    .action_lower
                    .zip(direct.action_lower)
                    .is_some_and(|(left, right)| approximately_equal(left, right, CONTROL_TOL))
                && auto
                    .action_upper
                    .zip(direct.action_upper)
                    .is_some_and(|(left, right)| approximately_equal(left, right, CONTROL_TOL))
                && auto
                    .sys_route_lower
                    .zip(direct.sys_route_lower)
                    .is_some_and(|(left, right)| approximately_equal(left, right, CONTROL_TOL))
                && auto
                    .sys_route_upper
                    .zip(direct.sys_route_upper)
                    .is_some_and(|(left, right)| approximately_equal(left, right, CONTROL_TOL))
                && auto.route_side == direct.route_side,
        },
    );

    let mut quarter_turn = Matrix4::zeros();
    quarter_turn[(0, 1)] = -1.0;
    quarter_turn[(1, 0)] = 1.0;
    quarter_turn[(2, 3)] = -1.0;
    quarter_turn[(3, 2)] = 1.0;
    for (name, matrix) in [
        ("central_inversion", -Matrix4::identity()),
        ("simultaneous_quarter_turn", quarter_turn),
    ] {
        let transformed = base.iter().map(|a| matrix * a).collect::<Vec<_>>();
        let row = evaluate(
            output,
            &format!("control_{name}"),
            "control",
            None,
            "control",
            &[],
            0.0,
            &[],
            &zero,
            transformed,
            &basis.orbit,
            base_incidence,
            EvaluatorRoute::Auto,
        );
        let observed = row.sys_nominal.expect("symmetry sys");
        write_control(
            &mut output.controls,
            ControlRow {
                control_id: format!("exact_finite_symmetry_{name}"),
                expected: format!("nominal sys={auto_sys:.16e} within {CONTROL_TOL}"),
                observed: format!("sys={observed:.16e}"),
                passed: (observed - auto_sys).abs() <= CONTROL_TOL,
            },
        );
    }

    for &theta in ROTATED_CONTROL_THETAS {
        let state = rotated_pentagon(theta);
        let row = evaluate(
            output,
            &format!("control_rotated_pentagon_{theta:.8}"),
            "control",
            None,
            "control",
            &[],
            theta,
            &[],
            &zero,
            state,
            &basis.orbit,
            base_incidence,
            EvaluatorRoute::Auto,
        );
        let expected = pentagon_profile(theta);
        let observed = row.sys_nominal.expect("rotated control sys");
        write_control(
            &mut output.controls,
            ControlRow {
                control_id: format!("nonlinear_rotated_pentagon_theta_{theta:.8}"),
                expected: format!("profile sys={expected:.16e} within {CONTROL_TOL}"),
                observed: format!("sys={observed:.16e}"),
                passed: (observed - expected).abs() <= CONTROL_TOL,
            },
        );
    }
}

fn file_identities() -> Vec<FileIdentity> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let candidates = [
        "empirical/neighborhood-sampling/samplers/m10_quotient_ray.rs",
        "empirical/neighborhood-sampling/samplers/mod.rs",
        "empirical/neighborhood-sampling/main.rs",
        "empirical/neighborhood-sampling/README.md",
        "src/lib.rs",
        "src/flat_polytope.rs",
        "Cargo.toml",
        "../../Cargo.toml",
        "../../Cargo.lock",
    ];
    candidates
        .iter()
        .map(|relative| FileIdentity {
            path: (*relative).to_string(),
            blake3: hash_file(&root.join(relative)),
        })
        .collect()
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("canonical repository root")
}

fn command_output(program: &str, args: &[&str], current_dir: &Path) -> String {
    let output = Command::new(program)
        .args(args)
        .current_dir(current_dir)
        .output()
        .unwrap_or_else(|error| panic!("run {program}: {error}"));
    assert!(
        output.status.success(),
        "{program} {} failed: {}",
        args.join(" "),
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout)
        .expect("command output UTF-8")
        .trim()
        .to_string()
}

fn git_identity(root: &Path) -> GitIdentity {
    GitIdentity {
        commit: command_output("git", &["rev-parse", "HEAD"], root),
        tree: command_output("git", &["rev-parse", "HEAD^{tree}"], root),
        clean: command_output("git", &["status", "--porcelain"], root).is_empty(),
    }
}

fn toolchain_identity(root: &Path) -> ToolchainIdentity {
    ToolchainIdentity {
        rustc_verbose: command_output("rustc", &["-Vv"], root),
        cargo_version: command_output("cargo", &["-V"], root),
        build_profile: if cfg!(debug_assertions) {
            "debug"
        } else {
            "release"
        },
    }
}

fn collect_tree_files(current: &Path, files: &mut Vec<PathBuf>) {
    let mut entries = fs::read_dir(current)
        .unwrap_or_else(|error| panic!("read source tree {}: {error}", current.display()))
        .map(|entry| entry.expect("source tree entry").path())
        .collect::<Vec<_>>();
    entries.sort();
    for path in entries {
        if path.is_dir() {
            collect_tree_files(&path, files);
        } else if path.is_file() {
            files.push(path);
        }
    }
}

fn hash_tree(root: &Path, relative: &str) -> TreeIdentity {
    let tree_root = root.join(relative);
    let mut files = Vec::new();
    collect_tree_files(&tree_root, &mut files);
    let mut hasher = blake3::Hasher::new();
    for path in &files {
        let local = path.strip_prefix(&tree_root).expect("tree-relative path");
        let local_bytes = local.to_string_lossy();
        let bytes =
            fs::read(path).unwrap_or_else(|error| panic!("read {}: {error}", path.display()));
        hasher.update(&(local_bytes.len() as u64).to_le_bytes());
        hasher.update(local_bytes.as_bytes());
        hasher.update(&(bytes.len() as u64).to_le_bytes());
        hasher.update(&bytes);
    }
    TreeIdentity {
        path: relative.into(),
        blake3: hasher.finalize().to_hex().to_string(),
        file_count: files.len(),
    }
}

fn local_source_tree_hashes() -> Vec<TreeIdentity> {
    let root = repo_root();
    [
        "experiments/hko-local-maximum/src",
        "crates/symplectic/src",
        "crates/euclidean-polytopes/src",
        "crates/algebraic-numbers/src",
    ]
    .iter()
    .map(|relative| hash_tree(&root, relative))
    .collect()
}

fn launch_settings_match_literal(settings: &LaunchSettings, cli: &Cli, shells: &[f64]) -> bool {
    settings.seed == 44
        && settings.directions == 32
        && settings.r_max == 0.5
        && settings.bisect_tol == 1.0e-4
        && settings.shells == [1e-4, 3e-4, 1e-3, 3e-3, 1e-2, 3e-2, 1e-1, 3e-1, 5e-1]
        && settings.post_transition_factors == [1.25, 1.5, 2.0, 3.0]
        && settings.nonlinear_rotated_control_thetas
            == [
                0.0,
                std::f64::consts::PI / 20.0,
                std::f64::consts::PI / 10.0,
            ]
        && settings.gram_schmidt_tolerance == 1.0e-11
        && settings.sign_threshold == 1.0e-14
        && settings.residual_gate == 2.0e-10
        && settings.gauge_warning_threshold == 1.0e-6
        && settings.gauge_sensitivity_thresholds == [1.0e-4, 1.0e-6, 1.0e-8]
        && settings.control_tolerance == 2.0e-9
        && settings.maximum_bisection_iterations == 64
        && cli.seed == settings.seed
        && cli.directions == settings.directions
        && cli.r_max == settings.r_max
        && cli.bisect_tol == settings.bisect_tol
        && shells == settings.shells
        && POST_FACTORS == settings.post_transition_factors
        && ROTATED_CONTROL_THETAS == settings.nonlinear_rotated_control_thetas
        && GS_TOL == settings.gram_schmidt_tolerance
        && SIGN_TOL == settings.sign_threshold
        && RESIDUAL_TOL == settings.residual_gate
        && GAUGE_WARNING == settings.gauge_warning_threshold
        && GAUGE_SENSITIVITY == settings.gauge_sensitivity_thresholds
        && CONTROL_TOL == settings.control_tolerance
        && MAX_BISECT_ITERS == settings.maximum_bisection_iterations
}

fn verify_launch_packet(
    cli: &Cli,
    shells: &[f64],
    git: &GitIdentity,
    toolchain: &ToolchainIdentity,
    compiled_sampler_source_blake3: &str,
    runtime_sampler_source_blake3: &str,
    source_identities: &[FileIdentity],
    source_tree_identities: &[TreeIdentity],
) -> Result<LaunchPacketVerification, String> {
    let path = cli
        .launch_packet
        .as_ref()
        .ok_or("frozen panel requires a launch packet")?;
    let bytes = fs::read(path)
        .map_err(|error| format!("read launch packet {}: {error}", path.display()))?;
    let packet: LaunchPacket = serde_json::from_slice(&bytes)
        .map_err(|error| format!("parse launch packet {}: {error}", path.display()))?;
    if packet.format != "hko_e2_quotient_ray_launch_v1" {
        return Err("launch packet format is not hko_e2_quotient_ray_launch_v1".into());
    }
    if !launch_settings_match_literal(&packet.settings, cli, shells) {
        return Err(
            "launch packet settings do not match the literal reviewed protocol and CLI".into(),
        );
    }
    // Checkout/source identity is advisory provenance. The literal protocol,
    // release profile, dimensions, and numerical checks remain blocking.
    if !packet.expected_git_clean || !git.clean {
        eprintln!(
            "warning: launch packet or current worktree is dirty; continuing. \
             Correlate the working directory and run timestamp with Git history \
             before reusing retained interpretation."
        );
    }
    if packet.expected_git_commit != git.commit || packet.expected_git_tree != git.tree {
        eprintln!(
            "warning: checkout differs from the launch packet revision/tree; \
             continuing with protocol checks. Reassess retained interpretation."
        );
    }
    if toolchain.build_profile != "release" {
        return Err("frozen panel requires a release build".into());
    }
    if compiled_sampler_source_blake3 != runtime_sampler_source_blake3 {
        eprintln!(
            "warning: compiled sampler source differs from checked-out source; \
             continuing with protocol checks. Reassess retained interpretation."
        );
    }
    if packet.expected_compiled_sampler_source_blake3 != compiled_sampler_source_blake3 {
        eprintln!(
            "warning: compiled sampler differs from the launch packet bytes; \
             continuing with protocol checks. Reassess retained interpretation."
        );
    }
    if packet.expected_source_and_dependency_hashes != source_identities {
        eprintln!(
            "warning: source/dependency bytes differ from the launch packet; \
             continuing with protocol checks. Reassess retained interpretation."
        );
    }
    if packet.expected_local_source_tree_hashes != source_tree_identities {
        eprintln!(
            "warning: local source-tree bytes differ from the launch packet; \
             continuing with protocol checks. Reassess retained interpretation."
        );
    }
    Ok(LaunchPacketVerification {
        path: path.display().to_string(),
        blake3: blake3::hash(&bytes).to_hex().to_string(),
        format: packet.format,
    })
}

fn write_artifact_bundle(out_dir: &Path) -> ArtifactBundle {
    let names = [
        "basis.json",
        "manifest.json",
        "controls.jsonl",
        "evaluations.jsonl",
        "transitions.jsonl",
        "ray-outcomes.jsonl",
        "summary.json",
    ];
    let artifacts = names
        .iter()
        .map(|name| FileIdentity {
            path: (*name).into(),
            blake3: hash_file(&out_dir.join(name)),
        })
        .collect::<Vec<_>>();
    let root = ArtifactBundleRoot {
        format: "hko_e2_artifact_bundle_v1",
        artifacts: &artifacts,
    };
    let bundle_content_blake3 =
        blake3::hash(&serde_json::to_vec(&root).expect("serialize artifact bundle root"))
            .to_hex()
            .to_string();
    let bundle = ArtifactBundle {
        format: "hko_e2_artifact_bundle_v1",
        artifacts,
        bundle_content_blake3,
    };
    write_json(out_dir.join("artifact-bundle.json"), &bundle);
    bundle
}

fn basis_artifact(base: &[Vector4<f64>], basis: &QuotientBasis) -> BasisArtifact {
    assert_eq!(basis.orbit.len(), 15, "orbit numerical rank");
    assert_eq!(basis.slice.len(), 25, "slice numerical rank");
    assert!(basis.orbit_residual <= RESIDUAL_TOL, "orbit residual");
    assert!(basis.slice_residual <= RESIDUAL_TOL, "slice residual");
    assert!(basis.cross_residual <= RESIDUAL_TOL, "cross residual");
    for axis in basis.orbit.iter().chain(&basis.slice) {
        assert_eq!(
            flatten(&unflatten(axis)).as_slice(),
            axis.as_slice(),
            "basis tuple reconstruction"
        );
    }
    let identities = file_identities();
    let payload = BasisPayload {
        format: "hko_e2_canonical_basis_v1",
        coordinate_order: "(q1,q2,p1,p2), facet-major",
        facet_order: "known_polytopes::hko_pentagon().dual_vertices order",
        generator_order: generator_order(),
        gram_schmidt_tolerance: GS_TOL,
        sign_convention: "first component with abs > threshold is positive",
        sign_threshold: SIGN_TOL,
        ambient_dimension: 40,
        orbit_rank: basis.orbit.len(),
        slice_rank: basis.slice.len(),
        orbit_orthonormal_residual_max: basis.orbit_residual,
        slice_orthonormal_residual_max: basis.slice_residual,
        orbit_slice_residual_max: basis.cross_residual,
        residual_gate: RESIDUAL_TOL,
        hko_dual_vertices: vectors_to_arrays(base),
        orbit_basis: nested_vectors(&basis.orbit),
        slice_basis: nested_vectors(&basis.slice),
        producer_blake3: identities[0].blake3.clone(),
        dependency_blake3: identities[1..].to_vec(),
    };
    let bytes = serde_json::to_vec(&payload).expect("serialize basis payload");
    BasisArtifact {
        basis_content_blake3: blake3::hash(&bytes).to_hex().to_string(),
        basis: payload,
    }
}

fn prepare_output(cli: &Cli) -> Output {
    if cli.out_dir.exists() {
        assert!(
            cli.out_dir
                .read_dir()
                .expect("read output directory")
                .next()
                .is_none(),
            "output directory must be empty: {}",
            cli.out_dir.display()
        );
    } else {
        fs::create_dir_all(&cli.out_dir).expect("create output directory");
    }
    Output {
        evaluations: BufWriter::new(
            File::create(cli.out_dir.join("evaluations.jsonl")).expect("evaluations"),
        ),
        transitions: BufWriter::new(
            File::create(cli.out_dir.join("transitions.jsonl")).expect("transitions"),
        ),
        controls: BufWriter::new(
            File::create(cli.out_dir.join("controls.jsonl")).expect("controls"),
        ),
        ray_outcomes: BufWriter::new(
            File::create(cli.out_dir.join("ray-outcomes.jsonl")).expect("ray outcomes"),
        ),
        next_evaluation: 0,
        capacity_evaluations: 0,
        transition_count: 0,
        nominal_reentry_rays: 0,
    }
}

fn write_json<T: Serialize>(path: PathBuf, value: &T) {
    let bytes = serde_json::to_vec_pretty(value).expect("serialize JSON");
    fs::write(path, bytes).expect("write JSON");
}

fn write_jsonl<T: Serialize>(writer: &mut BufWriter<File>, value: &T) {
    serde_json::to_writer(&mut *writer, value).expect("serialize JSONL");
    writer.write_all(b"\n").expect("write JSONL newline");
}

fn hash_file(path: &Path) -> String {
    let bytes = fs::read(path).unwrap_or_else(|error| panic!("read {}: {error}", path.display()));
    blake3::hash(&bytes).to_hex().to_string()
}

pub fn run(raw_args: &[String]) {
    let cli = parse_args(raw_args).unwrap_or_else(|message| {
        eprintln!("error: {message}\n");
        print_usage();
        std::process::exit(2);
    });
    let started = Instant::now();
    let root = repo_root();
    let git = git_identity(&root);
    let toolchain = toolchain_identity(&root);
    let source_identities = file_identities();
    let source_tree_identities = local_source_tree_hashes();
    let compiled_sampler_source_blake3 = blake3::hash(COMPILED_SAMPLER_SOURCE).to_hex().to_string();
    let runtime_sampler_source_blake3 = &source_identities[0].blake3;
    let executable = std::env::current_exe().expect("current executable path");
    let executable_blake3 = hash_file(&executable);
    let mut shells = SHELLS
        .iter()
        .copied()
        .filter(|&radius| radius <= cli.r_max)
        .collect::<Vec<_>>();
    if shells.last().copied() != Some(cli.r_max) {
        shells.push(cli.r_max);
    }
    let launch_verification = cli.frozen_panel.then(|| {
        verify_launch_packet(
            &cli,
            &shells,
            &git,
            &toolchain,
            &compiled_sampler_source_blake3,
            runtime_sampler_source_blake3,
            &source_identities,
            &source_tree_identities,
        )
        .unwrap_or_else(|message| panic!("frozen launch-packet verification failed: {message}"))
    });
    let canonical_target = cli.frozen_panel && launch_verification.is_some();

    // Launch-packet verification deliberately precedes all target geometry and capacity calls.
    let known = known_polytopes::hko_pentagon();
    let base_cache =
        HkoPolytopeCache::from_rational_parts(known.dual_vertices.clone(), known.vertices.clone())
            .expect("exact HKO base cache");
    let base = base_cache.dual_vertices_f64.clone();
    let base_incidence = incidence_signature(&base_cache.vertex_facet_incidence);
    let basis = quotient_basis(&base);
    let artifact = basis_artifact(&base, &basis);
    let replay = basis_artifact(&base, &quotient_basis(&base));
    let artifact_bytes = serde_json::to_vec_pretty(&artifact).expect("serialize basis");
    let replay_bytes = serde_json::to_vec_pretty(&replay).expect("serialize replay basis");
    assert_eq!(artifact_bytes, replay_bytes, "basis byte replay");

    let mut output = prepare_output(&cli);
    fs::write(cli.out_dir.join("basis.json"), artifact_bytes).expect("write basis artifact");
    let manifest = Manifest {
        format: "hko_e2_quotient_ray_manifest_v1",
        claim_boundary: vec![
            "finite shell screen in a fixed Euclidean local slice",
            "pointwise chart checks do not certify segments",
            "shell-bracketed transitions are not first mathematical exits",
            "no global quotient, monotonicity, trapping, star-shapedness, or thin-tube exclusion",
            "radii are ambient 40D Euclidean displacements in a fixed labelled-coordinate gauge and are metric dependent",
            "32 rays are only a mechanism/readiness screen, not a stable radius distribution or population probability",
            "the screen establishes neither a positivity inradius nor coverage of rare or lower-dimensional directions",
        ],
        basis_content_blake3: artifact.basis_content_blake3.clone(),
        source_and_dependency_hashes: source_identities,
        local_source_tree_hashes: source_tree_identities,
        compiled_sampler_source_blake3,
        executable_blake3,
        executable_path: executable.display().to_string(),
        git,
        toolchain,
        exact_invocation: std::env::args().collect(),
        working_directory: std::env::current_dir()
            .expect("current working directory")
            .display()
            .to_string(),
        coordinate_order: "(q1,q2,p1,p2), facet-major",
        generator_order: generator_order(),
        seed: cli.seed,
        random_direction_count: cli.directions,
        deterministic_sentinels: vec!["slice_basis_column_0", "projected_rotated_pentagon_tangent"],
        nonlinear_rotated_control_thetas: ROTATED_CONTROL_THETAS,
        shells: shells.clone(),
        r_max: cli.r_max,
        transition_tolerance: cli.bisect_tol,
        maximum_bisection_iterations: MAX_BISECT_ITERS,
        gauge_rank_tolerance: GS_TOL,
        gauge_warning_threshold: GAUGE_WARNING,
        gauge_sensitivity_thresholds: GAUGE_SENSITIVITY,
        post_transition_factors: POST_FACTORS,
        control_tolerance: CONTROL_TOL,
        basis_residual_gate: RESIDUAL_TOL,
        basis_sign_threshold: SIGN_TOL,
        float_serialization: "serde_json shortest-roundtrip f64 numbers; exact rationalized coordinates are numerator/denominator strings",
        event_stop_precedence: vec![
            "chart_invalid",
            "gauge_numeric_rank_event_or_warning",
            "evaluator_unavailable_or_interval_indeterminate",
            "route_side_indeterminate",
            "nominal_sys_side",
        ],
        sampling_continuation_rule: "a broad but usable action interval blocks capacity-grounded interpretation but does not truncate the nominal shell trace; chart, gauge, or evaluator unavailability is terminal",
        capacity_routes: vec![
            "Auto (resolved route recorded)",
            "direct pruned HK2017 base control",
        ],
        smoke: cli.smoke,
        frozen_panel_requested: cli.frozen_panel,
        launch_packet_path: launch_verification.as_ref().map(|value| value.path.clone()),
        launch_packet_blake3: launch_verification
            .as_ref()
            .map(|value| value.blake3.clone()),
        launch_packet_format: launch_verification
            .as_ref()
            .map(|value| value.format.clone()),
        launch_packet_verified: launch_verification.is_some(),
        canonical_target_predicate: canonical_target,
        build_binding_residual: "the executable hash, compiled sampler-source hash, clean Git commit/tree, local source-tree hashes, toolchain, profile, and exact invocation are retained; Rust does not natively attest that every linked path-dependency object was compiled from those trees",
    };
    write_json(cli.out_dir.join("manifest.json"), &manifest);

    run_controls(&mut output, &base, &basis, &base_incidence);

    let sentinel_directions = [
        ("sentinel_slice_basis_column_0", basis.slice[0].clone()),
        (
            "sentinel_projected_rotated_pentagon_tangent",
            rotated_tangent_in_slice(&base, &basis),
        ),
    ];
    for (id, direction) in sentinel_directions {
        run_ray(
            &mut output,
            id,
            "deterministic_sentinel",
            None,
            &[],
            &direction,
            &base,
            &basis,
            &base_incidence,
            &shells,
            cli.r_max,
            cli.bisect_tol,
        );
    }

    let mut rng = ChaCha8Rng::seed_from_u64(cli.seed);
    for index in 0..cli.directions {
        let coeffs = random_coefficients(&mut rng, basis.slice.len());
        let direction = expand_coefficients(&coeffs, &basis.slice);
        assert!((direction.norm() - 1.0).abs() <= RESIDUAL_TOL);
        run_ray(
            &mut output,
            &format!("random_{index:03}"),
            "random_s24",
            Some(index),
            &coeffs,
            &direction,
            &base,
            &basis,
            &base_incidence,
            &shells,
            cli.r_max,
            cli.bisect_tol,
        );
    }
    output.evaluations.flush().expect("flush evaluations");
    output.transitions.flush().expect("flush transitions");
    output.controls.flush().expect("flush controls");
    output.ray_outcomes.flush().expect("flush ray outcomes");
    let evaluation_count = output.next_evaluation;
    let capacity_evaluation_count = output.capacity_evaluations;
    let transition_count = output.transition_count;
    let nominal_reentry_ray_count = output.nominal_reentry_rays;
    drop(output);
    let summary = Summary {
        controls_passed: true,
        evaluation_count,
        capacity_evaluation_count,
        transition_count,
        random_directions_completed: cli.directions,
        deterministic_sentinels_completed: 2,
        nominal_reentry_ray_count,
        output_files: vec![
            "basis.json",
            "manifest.json",
            "controls.jsonl",
            "evaluations.jsonl",
            "transitions.jsonl",
            "ray-outcomes.jsonl",
            "summary.json",
            "artifact-bundle.json",
        ],
        elapsed_seconds: started.elapsed().as_secs_f64(),
        target_panel_executed: canonical_target,
        canonical_target_predicate: canonical_target,
    };
    write_json(cli.out_dir.join("summary.json"), &summary);
    let bundle = write_artifact_bundle(&cli.out_dir);
    println!(
        "wrote {} evaluations ({} capacity calls), {} transitions to {} in {:.3}s; artifact bundle {}",
        summary.evaluation_count,
        summary.capacity_evaluation_count,
        summary.transition_count,
        cli.out_dir.display(),
        summary.elapsed_seconds,
        bundle.bundle_content_blake3,
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_traits::ToPrimitive;

    fn base() -> Vec<Vector4<f64>> {
        known_polytopes::hko_pentagon()
            .dual_vertices
            .iter()
            .map(|a| {
                Vector4::new(
                    a[0].to_f64().unwrap(),
                    a[1].to_f64().unwrap(),
                    a[2].to_f64().unwrap(),
                    a[3].to_f64().unwrap(),
                )
            })
            .collect()
    }

    #[test]
    fn canonical_basis_has_expected_ranks_and_residuals() {
        let basis = quotient_basis(&base());
        assert_eq!(basis.orbit.len(), 15);
        assert_eq!(basis.slice.len(), 25);
        assert!(basis.orbit_residual <= RESIDUAL_TOL);
        assert!(basis.slice_residual <= RESIDUAL_TOL);
        assert!(basis.cross_residual <= RESIDUAL_TOL);
        assert!(basis.orbit.iter().chain(&basis.slice).all(|v| v
            .iter()
            .find(|x| x.abs() > SIGN_TOL)
            .is_none_or(|x| *x > 0.0)));
    }

    #[test]
    fn basis_generation_is_byte_replayable() {
        let base = base();
        let first = basis_artifact(&base, &quotient_basis(&base));
        let second = basis_artifact(&base, &quotient_basis(&base));
        assert_eq!(
            serde_json::to_vec_pretty(&first).unwrap(),
            serde_json::to_vec_pretty(&second).unwrap()
        );
        assert_eq!(first.basis_content_blake3, second.basis_content_blake3);
    }

    #[test]
    fn local_source_tree_hashes_are_deterministic_and_nonempty() {
        let first = local_source_tree_hashes();
        let second = local_source_tree_hashes();
        assert_eq!(first.len(), 4);
        assert!(first.iter().all(|tree| tree.file_count > 0));
        assert!(first
            .iter()
            .zip(second)
            .all(|(left, right)| left.path == right.path && left.blake3 == right.blake3));
    }

    #[test]
    fn profile_matches_declared_hko_maximum() {
        let theta = std::f64::consts::PI / 10.0;
        assert!((pentagon_profile(theta) - (3.0 + 5.0_f64.sqrt()) / 5.0).abs() < 1e-14);
    }

    #[test]
    fn nonlinear_family_tangent_sentinel_is_unit_and_in_slice() {
        let base = base();
        let basis = quotient_basis(&base);
        let tangent = rotated_tangent_in_slice(&base, &basis);
        assert!((tangent.norm() - 1.0).abs() <= RESIDUAL_TOL);
        assert!(basis
            .orbit
            .iter()
            .all(|axis| axis.dot(&tangent).abs() <= RESIDUAL_TOL));
    }

    #[test]
    fn smoke_cli_caps_radius_and_defaults_to_one_direction() {
        let cli =
            parse_args(&["--out-dir".into(), "/tmp/example".into(), "--smoke".into()]).unwrap();
        assert_eq!(cli.directions, 1);
        assert_eq!(cli.r_max, SMOKE_R_MAX);
    }

    #[test]
    fn nominal_side_requires_authoritative_finite_state() {
        assert_eq!(
            authoritative_nominal_side(
                "chart_nominal",
                "gauge_nominal",
                "evaluator_available",
                Some(0.9),
            ),
            "nominal_below_or_equal_one"
        );
        assert_eq!(
            authoritative_nominal_side(
                "chart_nominal",
                "gauge_nominal",
                "evaluator_available",
                Some(1.1),
            ),
            "nominal_above_one"
        );
        for (chart, gauge, evaluator, value) in [
            (
                "chart_invalid",
                "gauge_nominal",
                "evaluator_available",
                Some(0.9),
            ),
            (
                "chart_nominal",
                "gauge_near_tangency_warning",
                "evaluator_available",
                Some(0.9),
            ),
            (
                "chart_nominal",
                "gauge_numeric_rank_event",
                "evaluator_available",
                Some(0.9),
            ),
            (
                "chart_nominal",
                "gauge_nominal",
                "evaluator_indeterminate",
                Some(0.9),
            ),
            (
                "chart_nominal",
                "gauge_nominal",
                "evaluator_unavailable",
                Some(0.9),
            ),
            (
                "chart_nominal",
                "gauge_nominal",
                "evaluator_available",
                Some(f64::NAN),
            ),
        ] {
            assert_eq!(
                authoritative_nominal_side(chart, gauge, evaluator, value),
                "nominal_side_unavailable"
            );
        }
    }

    #[test]
    fn route_side_requires_every_authority_gate() {
        let valid = (
            "chart_nominal",
            "gauge_nominal",
            "evaluator_available",
            Some(2.0),
            Some(1.9),
            Some(2.1),
            Some((1.2, 1.4)),
        );
        assert_eq!(
            authoritative_route_side(valid.0, valid.1, valid.2, valid.3, valid.4, valid.5, valid.6),
            "route_above_one"
        );
        for (chart, gauge, evaluator) in [
            ("chart_invalid", "gauge_nominal", "evaluator_available"),
            (
                "chart_nominal",
                "gauge_near_tangency_warning",
                "evaluator_available",
            ),
            (
                "chart_nominal",
                "gauge_numeric_rank_event",
                "evaluator_available",
            ),
            ("chart_nominal", "gauge_nominal", "evaluator_indeterminate"),
            ("chart_nominal", "gauge_nominal", "evaluator_unavailable"),
        ] {
            assert_eq!(
                authoritative_route_side(
                    chart, gauge, evaluator, valid.3, valid.4, valid.5, valid.6,
                ),
                "route_side_unavailable"
            );
        }
    }

    #[test]
    fn nonpositive_or_interval_inconsistent_actions_are_not_authoritative() {
        assert_eq!(
            action_data_issue(Some(-1.0), Some(0.9), Some(1.1)),
            Some("nonpositive_or_nonfinite_nominal_action")
        );
        assert_eq!(
            action_data_issue(Some(1.2), Some(1.3), Some(1.4)),
            Some("nominal_action_outside_action_interval")
        );
        assert_eq!(
            action_data_issue(Some(1.2), Some(1.4), Some(1.3)),
            Some("unusable_action_interval")
        );
        assert_eq!(
            action_data_issue(Some(f64::NAN), Some(0.9), Some(1.1)),
            Some("nonpositive_or_nonfinite_nominal_action")
        );
        assert_eq!(
            authoritative_route_side(
                "chart_nominal",
                "gauge_nominal",
                "evaluator_available",
                Some(-1.0),
                Some(0.9),
                Some(1.1),
                Some((0.8, 1.2)),
            ),
            "route_side_unavailable"
        );
    }

    #[test]
    fn nominal_transition_wins_over_later_terminal_observation() {
        assert_eq!(
            competing_risk_or_censor_label(
                true,
                "chart_invalid",
                "gauge_numeric_rank_event",
                "evaluator_unavailable",
                0.3,
                0.5,
            ),
            "nominal_shell_transition_observed"
        );
    }

    fn transition(index: usize, inside: f64, outside: f64) -> TransitionRow {
        TransitionRow {
            transition_index: index,
            ray_id: "synthetic".into(),
            transition_kind: "nominal_sys".into(),
            label_inside: "nominal_above_one".into(),
            label_outside: "nominal_below_or_equal_one".into(),
            inside_radius: inside,
            outside_radius: outside,
            tolerance: DEFAULT_BISECT_TOL,
            midpoint_evaluations: 0,
            classification: "shell_bracketed_transition",
        }
    }

    #[test]
    fn only_authoritative_nominal_endpoints_qualify_for_probes() {
        let valid = transition(0, 0.1, 0.11);
        assert!(is_qualifying_nominal_transition(&valid));
        for unavailable in [
            "nominal_side_unavailable",
            "gauge_near_tangency_warning",
            "evaluator_indeterminate",
        ] {
            let mut invalid = valid.clone();
            invalid.label_outside = unavailable.into();
            assert!(!is_qualifying_nominal_transition(&invalid));
        }
    }

    #[test]
    fn shell_bisection_and_probe_observations_preserve_reentry_and_indeterminacy() {
        let witness = nominal_reentry_witness_from_observations([
            (0.0, "nominal_above_one"),
            (0.08, "nominal_below_or_equal_one"),
            (0.09, "nominal_side_unavailable"),
            (0.14, "nominal_above_one"),
        ]);
        assert_eq!(witness, Some((0.08, 0.14)));
        assert!(route_side_indeterminate_from_labels([
            "route_above_one",
            "route_side_indeterminate",
            "route_below_or_equal_one",
        ]));
        assert!(nominal_reentry_witness_from_observations([
            (0.0, "nominal_above_one"),
            (0.08, "nominal_side_unavailable"),
            (0.14, "nominal_above_one"),
        ])
        .is_none());
    }

    #[test]
    fn every_qualifying_bracket_contributes_strict_post_probe_provenance() {
        let transitions = [transition(4, 0.08, 0.09), transition(9, 0.18, 0.19)];
        let plan = plan_post_transition_probes(&transitions, DEFAULT_R_MAX);
        let refs = plan
            .iter()
            .flat_map(|(bits, refs)| {
                let radius = f64::from_bits(*bits);
                refs.iter().map(move |reference| (radius, reference))
            })
            .collect::<Vec<_>>();
        for transition in &transitions {
            assert!(refs
                .iter()
                .any(|(_, reference)| reference.transition_index == transition.transition_index));
        }
        assert!(refs
            .iter()
            .all(|(radius, reference)| *radius > reference.outside_radius));
    }

    #[test]
    fn literal_launch_settings_bind_cli_and_runtime_protocol() {
        let cli = Cli {
            out_dir: PathBuf::from("/tmp/canonical"),
            seed: 44,
            directions: 32,
            r_max: 0.5,
            bisect_tol: 1.0e-4,
            smoke: false,
            frozen_panel: true,
            launch_packet: Some(PathBuf::from("/tmp/reviewed.json")),
        };
        let settings = LaunchSettings {
            seed: 44,
            directions: 32,
            r_max: 0.5,
            bisect_tol: 1.0e-4,
            shells: vec![1e-4, 3e-4, 1e-3, 3e-3, 1e-2, 3e-2, 1e-1, 3e-1, 5e-1],
            post_transition_factors: vec![1.25, 1.5, 2.0, 3.0],
            nonlinear_rotated_control_thetas: vec![
                0.0,
                std::f64::consts::PI / 20.0,
                std::f64::consts::PI / 10.0,
            ],
            gram_schmidt_tolerance: 1.0e-11,
            sign_threshold: 1.0e-14,
            residual_gate: 2.0e-10,
            gauge_warning_threshold: 1.0e-6,
            gauge_sensitivity_thresholds: vec![1.0e-4, 1.0e-6, 1.0e-8],
            control_tolerance: 2.0e-9,
            maximum_bisection_iterations: 64,
        };
        assert!(launch_settings_match_literal(&settings, &cli, SHELLS));
        let mut wrong_seed = cli.clone();
        wrong_seed.seed += 1;
        assert!(!launch_settings_match_literal(
            &settings,
            &wrong_seed,
            SHELLS
        ));
        let mut wrong_settings = settings.clone();
        wrong_settings.maximum_bisection_iterations += 1;
        assert!(!launch_settings_match_literal(
            &wrong_settings,
            &cli,
            SHELLS
        ));

        assert!(parse_args(&[
            "--out-dir".into(),
            "/tmp/canonical".into(),
            "--frozen-panel".into(),
            "--launch-packet".into(),
            "/tmp/reviewed.json".into(),
        ])
        .is_ok());
        assert!(parse_args(&[
            "--out-dir".into(),
            "/tmp/not-canonical".into(),
            "--frozen-panel".into(),
        ])
        .is_err());
    }

    #[test]
    fn artifact_bundle_root_hash_includes_format() {
        let artifacts = vec![FileIdentity {
            path: "one.json".into(),
            blake3: "abc".into(),
        }];
        let first = serde_json::to_vec(&ArtifactBundleRoot {
            format: "hko_e2_artifact_bundle_v1",
            artifacts: &artifacts,
        })
        .unwrap();
        let second = serde_json::to_vec(&ArtifactBundleRoot {
            format: "hko_e2_artifact_bundle_v2",
            artifacts: &artifacts,
        })
        .unwrap();
        assert_ne!(blake3::hash(&first), blake3::hash(&second));
    }
}

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
use serde::Serialize;
use std::collections::BTreeMap;
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
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
const GS_TOL: f64 = 1.0e-11;
const SIGN_TOL: f64 = 1.0e-14;
const RESIDUAL_TOL: f64 = 2.0e-10;
const GAUGE_WARNING: f64 = 1.0e-6;
const GAUGE_SENSITIVITY: &[f64] = &[1.0e-4, 1.0e-6, 1.0e-8];
const CONTROL_TOL: f64 = 2.0e-9;
const MAX_BISECT_ITERS: usize = 64;

#[derive(Debug)]
struct Cli {
    out_dir: PathBuf,
    seed: u64,
    directions: usize,
    r_max: f64,
    bisect_tol: f64,
    smoke: bool,
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

#[derive(Clone, Debug, Serialize)]
struct FileIdentity {
    path: String,
    blake3: String,
}

#[derive(Debug, Serialize)]
struct Manifest {
    format: &'static str,
    claim_boundary: Vec<&'static str>,
    basis_content_blake3: String,
    source_and_dependency_hashes: Vec<FileIdentity>,
    coordinate_order: &'static str,
    generator_order: Vec<String>,
    seed: u64,
    random_direction_count: usize,
    deterministic_sentinels: Vec<&'static str>,
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
struct EvaluationRow {
    evaluation_index: usize,
    ray_id: String,
    ray_kind: String,
    direction_index: Option<usize>,
    phase: String,
    radius: f64,
    coefficient_direction_s24: Vec<f64>,
    expanded_direction_40d: Vec<f64>,
    serialized_dual_vertices: Vec<[f64; 4]>,
    rationalized_dual_vertices: Option<Vec<[String; 4]>>,
    pointwise_exact_rational_geometry: bool,
    chart_label: String,
    chart_validity_reasons: Vec<String>,
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

#[derive(Debug, Serialize)]
struct ControlRow {
    control_id: String,
    expected: String,
    observed: String,
    passed: bool,
}

#[derive(Debug, Serialize)]
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
    output_files: Vec<&'static str>,
    elapsed_seconds: f64,
    target_panel_executed: bool,
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
    let mut i = 0;
    while i < raw.len() {
        let value = raw.get(i + 1);
        match raw[i].as_str() {
            "--help" | "-h" => {
                print_usage();
                std::process::exit(0);
            }
            "--smoke" => smoke = true,
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
    Ok(Cli {
        out_dir: out_dir.ok_or("--out-dir is required")?,
        seed,
        directions,
        r_max,
        bisect_tol,
        smoke,
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

#[allow(clippy::too_many_arguments)]
fn evaluate(
    output: &mut Output,
    ray_id: &str,
    ray_kind: &str,
    direction_index: Option<usize>,
    phase: &str,
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
    let mut reasons = Vec::new();
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
            reasons.push(reason);
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
                reasons.push("labelled_incidence_signature_changed".into());
            }
            if !all_facets_defining {
                chart_label = "chart_invalid".into();
                reasons.push("not_all_facets_defining".into());
            }
            let vol = euclidean_volume_f64(&polytope.vertices, &polytope.vertex_facet_incidence);
            volume = Some(vol);
            if !(vol.is_finite() && vol > 0.0) {
                chart_label = "chart_invalid".into();
                reasons.push("nonpositive_or_nonfinite_volume".into());
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
                    Err(error) => reasons.push(format!("capacity_error:{error:?}")),
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
    let sys_nominal = nominal_action.zip(volume).map(|(a, v)| a * a / (2.0 * v));
    let route_interval = action_lower
        .zip(action_upper)
        .zip(volume)
        .and_then(|((lo, hi), v)| {
            (lo.is_finite() && hi.is_finite() && lo >= 0.0 && hi >= lo && v > 0.0)
                .then_some((lo * lo / (2.0 * v), hi * hi / (2.0 * v)))
        });
    if capacity.is_some() && route_interval.is_none() {
        evaluator_label = "evaluator_interval_indeterminate".into();
        reasons.push("unusable_action_interval".into());
    }
    let route_side = match route_interval {
        Some((lo, _)) if lo > 1.0 => "route_above_one",
        Some((_, hi)) if hi <= 1.0 => "route_below_or_equal_one",
        Some(_) => "route_side_indeterminate",
        None => "route_side_unavailable",
    }
    .to_string();
    let nominal_side = match sys_nominal {
        Some(value) if value > 1.0 => "nominal_above_one",
        Some(_) => "nominal_below_or_equal_one",
        None => "nominal_side_unavailable",
    }
    .to_string();
    let mut labels = vec![
        chart_label.clone(),
        gauge.label.clone(),
        evaluator_label.clone(),
        route_side.clone(),
        nominal_side.clone(),
    ];
    labels.extend(reasons.iter().map(|x| format!("reason:{x}")));
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
        radius,
        coefficient_direction_s24: coeffs.to_vec(),
        expanded_direction_40d: direction.as_slice().to_vec(),
        serialized_dual_vertices: vectors_to_arrays(&duals),
        rationalized_dual_vertices: rationalized,
        pointwise_exact_rational_geometry: chart_label == "chart_nominal",
        chart_label,
        chart_validity_reasons: reasons,
        facet_count,
        vertex_count,
        incidence_signature: signature,
        all_facets_defining,
        volume,
        gauge,
        evaluator_label,
        resolved_backend: backend,
        nominal_action,
        action_lower,
        action_upper,
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
            radius,
            coeffs,
            direction,
            state_on_ray(base, direction, radius),
            base_orbit,
            base_incidence,
            EvaluatorRoute::Auto,
        );
        midpoint_evaluations += 1;
        let label = classification(&midpoint, kind);
        if label == inside_label {
            inside = radius;
        } else {
            outside = radius;
            outside_label = label;
        }
    }
    TransitionRow {
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
) {
    let mut rows = Vec::new();
    rows.push(evaluate(
        output,
        ray_id,
        ray_kind,
        direction_index,
        "origin",
        0.0,
        coeffs,
        direction,
        base.to_vec(),
        &basis.orbit,
        base_incidence,
        EvaluatorRoute::Auto,
    ));
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
        rows.push(row);
        if terminal {
            break;
        }
    }

    let mut nominal_transition = None;
    for pair in rows.windows(2) {
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
            );
            if kind == "nominal_sys"
                && transition.label_inside == "nominal_above_one"
                && transition.label_outside == "nominal_below_or_equal_one"
                && nominal_transition.is_none()
            {
                nominal_transition = Some((transition.inside_radius, transition.outside_radius));
            }
            write_jsonl(&mut output.transitions, &transition);
            output.transition_count += 1;
        }
    }

    let mut post_transition_probe_count = 0;
    let mut nominal_reentry_observed = false;
    if let Some((inside, outside)) = nominal_transition {
        let midpoint = (inside + outside) / 2.0;
        let mut radii: BTreeMap<u64, Vec<f64>> = BTreeMap::new();
        for &factor in POST_FACTORS {
            let candidate = (factor * midpoint).min(r_max);
            if candidate > outside {
                radii.entry(candidate.to_bits()).or_default().push(factor);
            }
        }
        for (bits, factors) in radii {
            let radius = f64::from_bits(bits);
            let row = evaluate(
                output,
                ray_id,
                ray_kind,
                direction_index,
                &format!(
                    "post_transition_probe:factors={}",
                    factors
                        .iter()
                        .map(|factor| factor.to_string())
                        .collect::<Vec<_>>()
                        .join(",")
                ),
                radius,
                coeffs,
                direction,
                state_on_ray(base, direction, radius),
                &basis.orbit,
                base_incidence,
                EvaluatorRoute::Auto,
            );
            post_transition_probe_count += 1;
            nominal_reentry_observed |= row.nominal_reentry_observed;
        }
    }

    let last = rows.last().expect("ray has origin evaluation");
    let competing_risk_or_censor_label = if last.chart_label != "chart_nominal" {
        "chart_competing_observation_limit"
    } else if last.gauge.label != "gauge_nominal" {
        "gauge_competing_observation_limit"
    } else if last.evaluator_label != "evaluator_available" {
        "evaluator_competing_observation_limit"
    } else if nominal_transition.is_some() {
        "nominal_shell_transition_observed"
    } else if last.radius == r_max {
        "nominal_trace_right_censored_at_r_max"
    } else {
        "sampling_ended_without_declared_outcome"
    };
    let route_side_indeterminate_observed = rows
        .iter()
        .any(|row| row.route_side == "route_side_indeterminate");
    write_jsonl(
        &mut output.ray_outcomes,
        &RayOutcome {
            ray_id: ray_id.into(),
            ray_kind: ray_kind.into(),
            direction_index,
            competing_risk_or_censor_label: competing_risk_or_censor_label.into(),
            last_shell_radius: last.radius,
            nominal_transition_observed: nominal_transition.is_some(),
            route_side_indeterminate_observed,
            post_transition_probe_count,
            nominal_reentry_observed,
            no_reentry_claim:
                "false means only that no re-entry was directly seen at declared finite probes",
        },
    );
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
            control_id: "hko_exact_geometry_and_known_value".into(),
            expected: format!("chart nominal and sys={expected_hko:.16e} within {CONTROL_TOL}"),
            observed: format!("chart={}, sys={auto_sys:.16e}", auto.chart_label),
            passed: auto.chart_label == "chart_nominal"
                && (auto_sys - expected_hko).abs() <= CONTROL_TOL,
        },
    );
    write_control(
        &mut output.controls,
        ControlRow {
            control_id: "auto_vs_direct_pruned_at_hko".into(),
            expected: format!("nominal sys agreement within {CONTROL_TOL}"),
            observed: format!("auto={auto_sys:.16e}, direct={direct_sys:.16e}"),
            passed: (auto_sys - direct_sys).abs() <= CONTROL_TOL,
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

    for theta in [
        0.0,
        std::f64::consts::PI / 20.0,
        std::f64::consts::PI / 10.0,
    ] {
        let state = rotated_pentagon(theta);
        let row = evaluate(
            output,
            &format!("control_rotated_pentagon_{theta:.8}"),
            "control",
            None,
            "control",
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
        "src/lib.rs",
        "src/flat_polytope.rs",
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
    let mut shells = SHELLS
        .iter()
        .copied()
        .filter(|&radius| radius <= cli.r_max)
        .collect::<Vec<_>>();
    if shells.last().copied() != Some(cli.r_max) {
        shells.push(cli.r_max);
    }
    let manifest = Manifest {
        format: "hko_e2_quotient_ray_manifest_v1",
        claim_boundary: vec![
            "finite shell screen in a fixed Euclidean local slice",
            "pointwise chart checks do not certify segments",
            "shell-bracketed transitions are not first mathematical exits",
            "no global quotient, monotonicity, trapping, star-shapedness, or thin-tube exclusion",
        ],
        basis_content_blake3: artifact.basis_content_blake3.clone(),
        source_and_dependency_hashes: file_identities(),
        coordinate_order: "(q1,q2,p1,p2), facet-major",
        generator_order: generator_order(),
        seed: cli.seed,
        random_direction_count: cli.directions,
        deterministic_sentinels: vec!["slice_basis_column_0", "projected_rotated_pentagon_tangent"],
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
    let summary = Summary {
        controls_passed: true,
        evaluation_count: output.next_evaluation,
        capacity_evaluation_count: output.capacity_evaluations,
        transition_count: output.transition_count,
        random_directions_completed: cli.directions,
        deterministic_sentinels_completed: 2,
        output_files: vec![
            "basis.json",
            "manifest.json",
            "controls.jsonl",
            "evaluations.jsonl",
            "transitions.jsonl",
            "ray-outcomes.jsonl",
            "summary.json",
        ],
        elapsed_seconds: started.elapsed().as_secs_f64(),
        target_panel_executed: !cli.smoke && cli.directions == 32 && cli.r_max == 0.5,
    };
    write_json(cli.out_dir.join("summary.json"), &summary);
    println!(
        "wrote {} evaluations ({} capacity calls), {} transitions to {} in {:.3}s",
        summary.evaluation_count,
        summary.capacity_evaluation_count,
        summary.transition_count,
        cli.out_dir.display(),
        summary.elapsed_seconds
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
}

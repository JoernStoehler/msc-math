use exp_dev_quadratic_program::{
    capacity_f64_only_with_policy_and_method_profiled, exact_binary64_dual_vertex_arrays,
    exact_binary64_transition_matrix_assuming_origin_interior,
    generated_f64_cases_with_source_filter, solve_exact_capacity_for_transition_pruned_sigmas,
    ExactCapacityReport, F64CapacityMethod, F64CapacityOutcome, F64CapacityReport,
    F64ValidationPolicy, ScanCase,
};
use num_rational::BigRational;
use num_traits::{ToPrimitive, Zero};
use symplectic::{
    capacity_4d::{
        capacity_from_dual_vertices, capacity_value, check_dual_vertex_norm_bounds,
        check_facet_count, check_finite_dual_vertices, check_primal_vertex_norm_bounds,
        exact_binary64_polytope_geometry, general_qp_action_window,
        qp_minimizers_from_dual_vertices, QpCandidateFamily4d,
    },
    solve_orbit_sigma_saddle_point, CertifiedOrbitSetMode, OrbitAdmissibility, OrbitGuaranteeMode,
};

fn main() {
    let case = small_generated_case();

    let certified = production_scalar_consumer(&case);
    let minimizers = production_minimizer_consumer(&case);
    let near_minimizers = near_minimizer_window_consumer(&case);
    let fallback = retained_candidate_fallback_consumer(&case);
    let heuristic = heuristic_scan_consumer(&case);
    let timing = timing_only_consumer(&case);

    println!(
        "production_scalar route={} lower={:.12} upper={:.12} maximum_relative_error={:.1e}",
        certified.route, certified.lower, certified.upper, certified.maximum_relative_error
    );
    println!(
        "production_minimizers family={} capacity={:.12} count={} first_sigma={:?}",
        minimizers.family, minimizers.capacity, minimizers.count, minimizers.first_sigma
    );
    println!(
        "near_minimizer_window capacity={:.12} window_candidates={} exact_reference_agrees={}",
        near_minimizers.capacity,
        near_minimizers.window_candidate_count,
        near_minimizers.exact_reference_agrees
    );
    println!(
        "retained_candidate_fallback capacity={:.12} exact_resolutions={} retained_orbits={}",
        fallback.capacity, fallback.exact_resolutions, fallback.retained_orbit_count
    );
    println!(
        "heuristic_scan status={} capacity={:?} sigma={:?} indet={} failures={} route={}",
        heuristic.status,
        heuristic.capacity,
        heuristic.sigma,
        heuristic.indeterminate_count,
        heuristic.numerical_failure_count,
        heuristic.route_label
    );
    println!(
        "timing_only route={} sigmas={} total_measured_ms={:.3}",
        timing.route_label, timing.sigma_count, timing.total_measured_ms
    );
}

/// Ordinary scalar consumer: accept the outward interval only when it is
/// narrow enough for the downstream calculation.
fn production_scalar_consumer(case: &ScanCase) -> ProductionScalar {
    let capacity = capacity_from_dual_vertices(&case.dual_vertices)
        .expect("owned consumer fixture must satisfy the production input contract");
    let bounds = capacity.bounds();
    let maximum_relative_error = 1e-10;
    let _value = capacity_value(&capacity, maximum_relative_error)
        .expect("owned consumer requires capacity relative error at most 1e-10");
    ProductionScalar {
        route: capacity.route_name(),
        lower: bounds.lower(),
        upper: bounds.upper(),
        maximum_relative_error,
    }
}

/// Ordinary minimizer consumer: use exact actions and preserve the candidate
/// family name rather than treating a word as an unqualified physical orbit.
fn production_minimizer_consumer(case: &ScanCase) -> ProductionMinimizers {
    let minimizers = qp_minimizers_from_dual_vertices(&case.dual_vertices)
        .expect("owned consumer fixture must have production minimizers");
    let first = minimizers
        .candidates()
        .first()
        .expect("successful minimizer search returns a word");
    ProductionMinimizers {
        family: match minimizers.family() {
            QpCandidateFamily4d::GeneralHk => "general_hk",
            QpCandidateFamily4d::ProductClosureVertex => "product_closure_vertex",
        },
        capacity: first
            .action_exact()
            .to_f64()
            .expect("owned consumer capacity fits binary64"),
        count: minimizers.candidates().len(),
        first_sigma: first.sigma().to_vec(),
    }
}

/// Ordinary consumer shape for exact near-minimizing `(sigma, action)` rows.
/// The exact-all route below is a correspondence control, not the consumer.
fn near_minimizer_window_consumer(case: &ScanCase) -> NearMinimizerWindow {
    check_facet_count(case.dual_vertices.len()).expect("capacity facet-count bound");
    check_finite_dual_vertices(&case.dual_vertices).expect("finite dual vertices");
    check_dual_vertex_norm_bounds(&case.dual_vertices).expect("dual-vertex norm bounds");
    let geometry =
        exact_binary64_polytope_geometry(&case.dual_vertices).expect("exact binary64 geometry");
    check_primal_vertex_norm_bounds(&geometry).expect("primal-vertex norm bounds");
    let maximum_action_multiple = BigRational::new(101.into(), 100.into());
    let window = general_qp_action_window(&geometry, maximum_action_multiple)
        .expect("owned consumer fixture must have an exact general action window");

    let exact = exact_report(
        case,
        window.capacity_exact().clone() * BigRational::new(1.into(), 100.into()),
    );
    let observed = window
        .witnesses()
        .iter()
        .map(|witness| (witness.sigma.clone(), witness.action()))
        .collect::<Vec<_>>();
    let expected = exact
        .orbits
        .iter()
        .map(|orbit| (orbit.sigma.clone(), orbit.action_exact.clone()))
        .collect::<Vec<_>>();
    NearMinimizerWindow {
        capacity: window
            .capacity_exact()
            .to_f64()
            .expect("owned consumer capacity fits binary64"),
        window_candidate_count: window.witnesses().len(),
        exact_reference_agrees: observed == expected,
    }
}

/// Consumer shape for callers that accept a retained f64 candidate set only
/// after local exact fallback certifies that retained set.
fn retained_candidate_fallback_consumer(case: &ScanCase) -> RetainedCandidateFallback {
    let exact = exact_report(case, BigRational::zero());
    let mut retained =
        solve_orbit_sigma_saddle_point(&case.dual_vertices, &exact.minimizers[0].sigma)
            .expect("owned consumer fixture should solve the retained sigma");
    retained.admissibility = OrbitAdmissibility::IndeterminateF64;

    let dual_vertices_exact = exact_binary64_dual_vertex_arrays(&case.dual_vertices);
    let _interval_result =
        exp_dev_quadratic_program::fallback_route::aggregate_orbits_with_local_exact_fallback(
            &dual_vertices_exact,
            vec![retained.clone()],
            1,
            0.0,
            OrbitGuaranteeMode::BoundSafe,
        )
        .expect("local exact fallback should certify the retained candidate");
    let exact_set =
        exp_dev_quadratic_program::fallback_route::aggregate_certified_orbits_with_local_exact_fallback(
            &dual_vertices_exact,
            vec![retained],
            1,
            BigRational::zero(),
            CertifiedOrbitSetMode::MinimizersOnly,
        )
        .expect("local certified fallback should certify the retained candidate set");

    RetainedCandidateFallback {
        capacity: exact_set.capacity,
        exact_resolutions: exact_set.exact_resolutions,
        retained_orbit_count: exact_set.orbits.len(),
    }
}

/// Consumer shape for large exploratory scans. It accepts heuristic f64 output
/// only because the status and uncertainty counters stay visible.
fn heuristic_scan_consumer(case: &ScanCase) -> HeuristicScanRow {
    let (report, _) = capacity_f64_only_with_policy_and_method_profiled(
        &case.dual_vertices,
        F64ValidationPolicy::LpOriginVertex,
        F64CapacityMethod::ProductBilliardOrHk,
    );
    HeuristicScanRow {
        route_label: F64CapacityMethod::ProductBilliardOrHk.label(),
        status: f64_status(&report),
        capacity: f64_capacity(&report),
        sigma: f64_sigma(&report),
        indeterminate_count: report.indeterminate_f64_count,
        numerical_failure_count: report.numerical_failure_count,
    }
}

/// Consumer shape for profiling. It records route identity and cost, but does
/// not interpret the resulting scalar as correctness evidence.
fn timing_only_consumer(case: &ScanCase) -> TimingOnlyRow {
    let (report, timing) = capacity_f64_only_with_policy_and_method_profiled(
        &case.dual_vertices,
        F64ValidationPolicy::LpOriginVertex,
        F64CapacityMethod::TransitionPrunedHk,
    );
    TimingOnlyRow {
        route_label: F64CapacityMethod::TransitionPrunedHk.label(),
        sigma_count: report.sigma_count,
        total_measured_ms: timing.combinatorics_ms
            + timing.transition_matrix_ms
            + timing.candidate_solve_ms
            + timing.report_ms,
    }
}

fn exact_report(case: &ScanCase, action_gap_exact: BigRational) -> ExactCapacityReport {
    let dual_vertices_exact = exact_binary64_dual_vertex_arrays(&case.dual_vertices);
    let transition =
        exact_binary64_transition_matrix_assuming_origin_interior(&dual_vertices_exact);
    solve_exact_capacity_for_transition_pruned_sigmas(
        &dual_vertices_exact,
        &transition,
        action_gap_exact,
    )
    .expect("owned consumer fixture should have an exact capacity")
}

fn f64_status(report: &F64CapacityReport) -> &'static str {
    match report.outcome {
        F64CapacityOutcome::Success { .. } => "heuristic_success",
        F64CapacityOutcome::Failure { .. } => "heuristic_failure",
    }
}

fn f64_capacity(report: &F64CapacityReport) -> Option<f64> {
    match report.outcome {
        F64CapacityOutcome::Success { capacity, .. } => Some(capacity),
        F64CapacityOutcome::Failure { .. } => None,
    }
}

fn f64_sigma(report: &F64CapacityReport) -> Option<Vec<usize>> {
    match &report.outcome {
        F64CapacityOutcome::Success { sigma, .. } => Some(sigma.clone()),
        F64CapacityOutcome::Failure { .. } => None,
    }
}

fn small_generated_case() -> ScanCase {
    generated_f64_cases_with_source_filter(
        1,
        99540836,
        &["seed99540836:F5:sample0:attempt5000000008".to_string()],
    )
    .pop()
    .expect("known generated case")
}

#[derive(Debug)]
struct ProductionScalar {
    route: &'static str,
    lower: f64,
    upper: f64,
    maximum_relative_error: f64,
}

#[derive(Debug)]
struct ProductionMinimizers {
    family: &'static str,
    capacity: f64,
    count: usize,
    first_sigma: Vec<usize>,
}

#[derive(Debug)]
struct NearMinimizerWindow {
    capacity: f64,
    window_candidate_count: usize,
    exact_reference_agrees: bool,
}

#[derive(Debug)]
struct RetainedCandidateFallback {
    capacity: f64,
    exact_resolutions: usize,
    retained_orbit_count: usize,
}

#[derive(Debug)]
struct HeuristicScanRow {
    route_label: &'static str,
    status: &'static str,
    capacity: Option<f64>,
    sigma: Option<Vec<usize>>,
    indeterminate_count: usize,
    numerical_failure_count: usize,
}

#[derive(Debug)]
struct TimingOnlyRow {
    route_label: &'static str,
    sigma_count: u64,
    total_measured_ms: f64,
}

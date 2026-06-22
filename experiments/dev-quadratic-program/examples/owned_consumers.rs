use exp_dev_quadratic_program::{
    capacity_f64_only_with_policy_and_method_profiled, exact_binary64_dual_vertex_arrays,
    exact_binary64_transition_matrix_assuming_origin_interior,
    generated_f64_cases_with_source_filter, solve_exact_capacity_for_transition_pruned_sigmas,
    ExactCapacityReport, F64CapacityMethod, F64CapacityOutcome, F64CapacityReport,
    F64ValidationPolicy, ScanCase,
};
use num_rational::BigRational;
use num_traits::Zero;

fn main() {
    let case = small_generated_case();

    let certified = certified_scalar_consumer(&case);
    let near_minimizers = near_minimizer_window_consumer(&case);
    let heuristic = heuristic_scan_consumer(&case);
    let timing = timing_only_consumer(&case);

    println!(
        "certified_scalar capacity={:.12} minimizers={} exact_admissible={} iterations={}",
        certified.capacity,
        certified.minimizer_count,
        certified.exact_admissible_count,
        certified.iterations
    );
    println!(
        "near_minimizer_window capacity={:.12} window_orbits={} exact_admissible={}",
        near_minimizers.capacity,
        near_minimizers.window_orbit_count,
        near_minimizers.exact_admissible_count
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

/// Consumer shape for callers that need a scalar capacity whose uncertainty is
/// resolved before use. This deliberately uses the exact route, not f64 labels.
fn certified_scalar_consumer(case: &ScanCase) -> CertifiedScalar {
    let exact = exact_report(case, BigRational::zero());
    assert!(
        !exact.minimizers.is_empty(),
        "certified scalar consumer needs an exact minimizer"
    );
    CertifiedScalar {
        capacity: exact.capacity,
        minimizer_count: exact.minimizers.len(),
        exact_admissible_count: exact.exact_admissible_count,
        iterations: exact.iterations,
    }
}

/// Consumer shape for callers that need near-minimizing `(sigma, action)` rows,
/// not only the scalar value. The exact route owns the gap-window semantics here.
fn near_minimizer_window_consumer(case: &ScanCase) -> NearMinimizerWindow {
    let exact = exact_report(case, BigRational::new(1.into(), 100.into()));
    NearMinimizerWindow {
        capacity: exact.capacity,
        window_orbit_count: exact.orbits.len(),
        exact_admissible_count: exact.exact_admissible_count,
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
struct CertifiedScalar {
    capacity: f64,
    minimizer_count: usize,
    exact_admissible_count: usize,
    iterations: u64,
}

#[derive(Debug)]
struct NearMinimizerWindow {
    capacity: f64,
    window_orbit_count: usize,
    exact_admissible_count: usize,
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

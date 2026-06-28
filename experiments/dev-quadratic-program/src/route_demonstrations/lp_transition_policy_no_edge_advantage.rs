use crate::{
    edge_fixture_cases, scan_case_with_options, F64CapacityMethod, F64ValidationPolicy,
    NearRedundantFacetRemovalPolicy, ScanCase, ScanOptions, ScanRow,
};

/// Demonstrates why the LP transition policy is not currently promoted.
///
/// The route is a plausible repair attempt: instead of deriving facet-pair
/// intersections from a f64 vertex scan, ask each pair-intersection question by
/// a small f64 LP. On the current edge fixtures this buys no route outcome: the
/// same rows reject, request fallback, or return the same f64 value. On the
/// near-redundant product fixture it is strictly less decisive by the counters,
/// adding facet-intersection ambiguity.
#[test]
fn lp_transition_policy_has_no_known_edge_fixture_advantage() {
    for case in edge_fixture_cases() {
        let baseline = scan_with_policy(case.clone(), F64ValidationPolicy::LpOriginVertex);
        let lp = scan_with_policy(case.clone(), F64ValidationPolicy::Lp);

        assert_same_route_outcome(&case, &baseline, &lp);
    }

    let case = edge_fixture_cases()
        .into_iter()
        .find(|case| case.source_id == "edge:near_redundant_product")
        .expect("near-redundant product edge fixture");
    let baseline = scan_with_policy(case.clone(), F64ValidationPolicy::LpOriginVertex);
    let lp = scan_with_policy(case, F64ValidationPolicy::Lp);

    assert_eq!(baseline.facet_intersection_indeterminate_count, 0);
    assert!(
        lp.facet_intersection_indeterminate_count > baseline.facet_intersection_indeterminate_count,
        "LP transition policy should expose why this attempted route is not a known improvement: baseline={baseline:?}, lp={lp:?}"
    );
    assert!(
        lp.omega_indeterminate_count > baseline.omega_indeterminate_count,
        "LP transition policy should not hide that it made this fixture less decisive: baseline={baseline:?}, lp={lp:?}"
    );
}

fn scan_with_policy(case: ScanCase, validation_policy: F64ValidationPolicy) -> ScanRow {
    scan_case_with_options(
        case,
        &ScanOptions {
            validation_policy,
            capacity_method: F64CapacityMethod::TransitionPrunedHk,
            near_redundant_facet_removal: NearRedundantFacetRemovalPolicy::None,
            ..ScanOptions::default()
        },
    )
}

fn assert_same_route_outcome(case: &ScanCase, baseline: &ScanRow, lp: &ScanRow) {
    assert_eq!(
        baseline.validation_status, lp.validation_status,
        "{}: LP transition policy changed validation status without a known advantage",
        case.source_id
    );
    assert_eq!(
        baseline.outcome, lp.outcome,
        "{}: LP transition policy changed capacity outcome without a known advantage",
        case.source_id
    );
    assert_eq!(
        baseline.failure_reason, lp.failure_reason,
        "{}: LP transition policy changed failure reason without a known advantage",
        case.source_id
    );
    assert_eq!(
        baseline.f64_capacity, lp.f64_capacity,
        "{}: LP transition policy changed the f64 scalar without a known advantage",
        case.source_id
    );
    assert_eq!(
        baseline.f64_sigma, lp.f64_sigma,
        "{}: LP transition policy changed the reported sigma without a known advantage",
        case.source_id
    );
}

use crate::{edge_fixture_cases, scan_case, TrustClass};

/// Demonstrates the desired failure mode for a guarded route.
///
/// The guarded scan path is allowed to reject or request fallback. The important
/// behavior is that it does not manufacture a scalar capacity when validation
/// has already found a blocking input problem.
#[test]
fn guarded_route_rejects_invalid_input_instead_of_returning_pseudo_capacity() {
    let case = edge_fixture_cases()
        .into_iter()
        .find(|case| case.source_id == "edge:duplicate_dual_vertices")
        .expect("duplicate dual vertices edge fixture");
    let row = scan_case(case);

    assert_eq!(row.validation_status, "rejected");
    assert_eq!(row.outcome, "not_run");
    assert_eq!(row.failure_reason.as_deref(), Some("validation_rejected"));
    assert_eq!(row.f64_capacity, None);
    assert_eq!(row.trust_class, TrustClass::FallbackRequired.label());
    assert!(
        row.validation_reasons
            .iter()
            .any(|reason| reason.starts_with("near_duplicate_dual_vertices")),
        "safe refusal should carry the concrete validation reason: {row:?}"
    );
}

/// Demonstrates that guarded non-success is also used for ambiguous validation,
/// not only obviously invalid duplicate facets.
#[test]
fn guarded_route_requests_fallback_for_ambiguous_origin_and_omega_signs() {
    let case = edge_fixture_cases()
        .into_iter()
        .find(|case| case.source_id == "edge:missing_origin_interior")
        .expect("missing origin interior edge fixture");
    let row = scan_case(case);

    assert_eq!(row.validation_status, "fallback_required");
    assert_eq!(row.outcome, "not_run");
    assert_eq!(
        row.failure_reason.as_deref(),
        Some("validation_fallback_required")
    );
    assert_eq!(row.f64_capacity, None);
    assert_eq!(row.trust_class, TrustClass::FallbackRequired.label());
    assert!(
        row.validation_reasons
            .iter()
            .any(|reason| reason == "origin_interior_indeterminate"),
        "safe refusal should expose the ambiguous predicate: {row:?}"
    );
    assert!(
        row.validation_reasons
            .iter()
            .any(|reason| reason == "omega_indeterminate"),
        "safe refusal should expose the ambiguous transition predicate: {row:?}"
    );
}

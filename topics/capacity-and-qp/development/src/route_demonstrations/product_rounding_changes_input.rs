use crate::{edge_fixture_cases, scan_case};

/// Demonstrates that product rounding is a preprocessing route, not a
/// same-input capacity certificate.
///
/// The fixture is an exact product with tiny off-block f64 drift. Rounding
/// makes the intended product route available and the scalar agrees with the
/// artifact label, but the row deliberately does not apply the
/// near-redundant-facet distortion bound to this rounding step.
#[test]
fn product_rounding_solves_a_f64_product_recognition_problem_by_changing_input() {
    let case = edge_fixture_cases()
        .into_iter()
        .find(|case| case.source_id == "edge:drifted_product_rounding")
        .expect("drifted product edge fixture");
    let row = scan_case(case);

    assert_eq!(row.product_rounding_status, "rounded");
    assert_eq!(row.original_facet_count, Some(row.facet_count));
    assert_eq!(row.outcome, "success");
    assert_eq!(row.agreement_status, "agrees");
    assert!(
        row.product_rounding_max_abs_change.unwrap_or(0.0) > 0.0,
        "this demo should show a real f64 input edit: {row:?}"
    );
    assert!(
        row.product_rounding_max_minor_over_major
            .unwrap_or(f64::INFINITY)
            < 1e-9,
        "the route should only round tiny off-block drift: {row:?}"
    );

    assert!(
        row.preprocessed_f64_vs_original_artifact_abs_error
            .unwrap_or(f64::INFINITY)
            < 1e-12,
        "the rounded-input f64 scalar should agree empirically with the original artifact label: {row:?}"
    );
    assert_eq!(
        row.preprocessed_f64_vs_original_artifact_within_bound, None,
        "product rounding has no stored capacity-distortion bound; consumers must treat it as changed-input evidence"
    );
}

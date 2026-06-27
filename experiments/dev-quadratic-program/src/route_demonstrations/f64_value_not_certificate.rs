use crate::{edge_fixture_cases, scan_case, OutputDecisionStatus, OutputEpistemics, TrustClass};

/// Demonstrates that a correct-looking f64 scalar is not the same as a trusted
/// capacity result.
///
/// This fixture returns the expected scalar value, but the route still reports
/// structural ambiguity and multiple near-minimizing sigmas. The scalar can be
/// useful for exploration; it is not a complete certificate of the minimizing
/// orbit set.
#[test]
fn f64_value_can_agree_while_minimizer_set_is_undecided() {
    let case = edge_fixture_cases()
        .into_iter()
        .find(|case| case.source_id == "edge:drifted_product_rounding")
        .expect("drifted product edge fixture");
    let row = scan_case(case);

    assert_eq!(row.outcome, "success");
    assert_eq!(row.agreement_status, "agrees");
    assert_eq!(row.trust_class, TrustClass::DegenerateValueAgrees.label());
    assert!(
        row.abs_action_error.unwrap_or(f64::INFINITY) < 1e-12,
        "the point of this demo is not a wrong scalar value: {row:?}"
    );

    assert_eq!(
        row.output_epistemics.minimizing_sigma_set_status,
        OutputDecisionStatus::Undecided
    );
    assert!(
        row.near_minimizing_sigma_count > 1,
        "near-tied minimizing sigmas are why this f64 output is not a complete certificate: {row:?}"
    );
    assert!(
        has_reason(
            &row.output_epistemics,
            "output_minimizing_sigma_set_undecided:multiple_near_minimizing_sigmas"
        ),
        "output epistemics should state the non-certificate reason: {row:?}"
    );
    assert!(
        has_reason(
            &row.output_epistemics,
            "benign_structural:omega_indeterminate"
        ),
        "the route also exposes structural f64 ambiguity instead of hiding it: {row:?}"
    );
}

fn has_reason(epistemics: &OutputEpistemics, reason: &str) -> bool {
    epistemics.reasons.iter().any(|item| item == reason)
}

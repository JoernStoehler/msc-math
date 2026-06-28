use crate::{
    scan_case_with_options, F64CapacityMethod, F64ValidationPolicy,
    NearRedundantFacetRemovalPolicy, ScanCase, ScanOptions,
};
use nalgebra::Vector4;

/// Demonstrates what near-redundant facet removal buys and what it does not
/// claim.
///
/// Without near-redundant facet removal this fixture produces an ambiguous f64
/// output that recommends fallback. Product near-redundant removal deletes one
/// facet and then computes the simpler preprocessed polytope. The row records
/// multiplicative bounds for the removal step.
#[test]
fn near_redundant_removal_turns_ambiguous_direct_output_into_a_bounded_surrogate() {
    let case = skew_product_with_near_redundant_facet_case();

    let baseline = scan_case_with_options(
        case.clone(),
        &ScanOptions {
            validation_policy: F64ValidationPolicy::LpOriginVertex,
            capacity_method: F64CapacityMethod::ProductBilliardOrHk,
            near_redundant_facet_removal: NearRedundantFacetRemovalPolicy::None,
            ..ScanOptions::default()
        },
    );
    assert_eq!(
        baseline.validation_status, "accepted_ambiguous",
        "{baseline:?}"
    );
    assert_eq!(baseline.outcome, "success", "{baseline:?}");
    assert_eq!(baseline.trust_class, "fallback_required");
    assert!(
        baseline.indeterminate_overlaps_best_interval,
        "direct route should expose an unresolved near-minimum ambiguity: {baseline:?}"
    );
    assert!(
        baseline.near_minimizing_sigma_count > 1,
        "direct route should expose multiple near-minimizing sigmas: {baseline:?}"
    );
    assert_eq!(
        baseline.near_redundant_facet_removal_status,
        "not_attempted"
    );

    let surrogate = scan_case_with_options(
        case,
        &ScanOptions {
            validation_policy: F64ValidationPolicy::LpOriginVertex,
            capacity_method: F64CapacityMethod::ProductBilliardOrHk,
            near_redundant_facet_removal: NearRedundantFacetRemovalPolicy::Product,
            near_redundant_facet_removal_delta: 2e-8,
            ..ScanOptions::default()
        },
    );

    assert_eq!(
        surrogate.near_redundant_facet_removal_policy,
        "product_remove_near_redundant_facets"
    );
    assert_eq!(surrogate.near_redundant_facet_removal_status, "removed");
    assert_eq!(surrogate.original_facet_count, Some(7));
    assert_eq!(surrogate.facet_count, 6);
    assert_eq!(surrogate.removed_facet_count, 1);
    assert_eq!(surrogate.outcome, "success", "{surrogate:?}");
    assert!(
        surrogate.sigma_count < baseline.sigma_count,
        "removing the near-redundant facet should simplify the sigma stream: baseline {}, surrogate {}",
        baseline.sigma_count,
        surrogate.sigma_count
    );
    assert_eq!(surrogate.original_artifact_capacity_label, None);
    assert_eq!(surrogate.artifact_capacity_label, None);

    let capacity_ratio_upper = surrogate
        .capacity_ratio_upper_bound
        .expect("facet-removal capacity bound");
    assert!(
        capacity_ratio_upper > 1.0,
        "facet removal should expose a distortion bound, not a same-input claim: {surrogate:?}"
    );
    assert!(
        surrogate.f64_capacity.is_some(),
        "the preprocessed surrogate should be computable by the direct f64 route: {surrogate:?}"
    );
    assert_eq!(
        surrogate.preprocessed_f64_vs_original_artifact_within_bound, None,
        "this local fixture has no trusted original capacity label to compare against"
    );
}

fn skew_product_with_near_redundant_facet_case() -> ScanCase {
    let eps = 1e-8;
    let q0 = Vector4::new(-1.2214036892748639, -0.128410235348687, 0.0, 0.0);
    let q1 = Vector4::new(0.8038785777125631, -1.0394029100481912, 0.0, 0.0);
    let q2 = Vector4::new(1.106104528257497, 0.23181164091432865, 0.0, 0.0);
    ScanCase {
        family: "route_demo".to_string(),
        source_id: "route_demo:skew_product_near_redundant_facet".to_string(),
        input_source: "route_demo".to_string(),
        generated_attempt: None,
        generator_seed: None,
        requested_facet_count: Some(7),
        dual_vertices: vec![
            q0,
            q1,
            q2,
            q0 + eps * q1,
            Vector4::new(0.0, 0.0, -0.7037463173639409, -1.0848793918667465),
            Vector4::new(0.0, 0.0, 1.4619222451670222, 0.41685665805008276),
            Vector4::new(0.0, 0.0, -0.1238504743562827, 1.5876355795695363),
        ],
        audit_capacity_label: None,
        artifact_capacity_label: None,
        audit_sigma_label: None,
    }
}

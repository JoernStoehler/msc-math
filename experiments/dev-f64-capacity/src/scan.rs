use crate::{
    audit_generated_case_exact, capacity_f64_only_with_policy_and_method_profiled, classify_report,
    exact_audit_not_requested, generic, product, validate_f64_polytope_input_with_policy_profiled,
    ExactAuditReport, F64CapacityMethod, F64CapacityReport, F64CapacityTimingBreakdown,
    F64ValidationPolicy, F64ValidationReport, F64ValidationStatus, F64ValidationTimingBreakdown,
    NearRedundantFacetRemovalPolicy, NearRedundantFacetRemovalReport,
    NearRedundantFacetRemovalStatus, ProductRoundingReport, ScanCase, ScanRow,
};
use std::time::Instant;

const BOUND_COMPARISON_TOLERANCE: f64 = 1e-10;

#[derive(Clone, Debug)]
pub struct ScanOptions {
    pub audit_generated: bool,
    pub audit_preprocessed: bool,
    pub validation_policy: F64ValidationPolicy,
    pub capacity_method: F64CapacityMethod,
    pub near_redundant_facet_removal: NearRedundantFacetRemovalPolicy,
    pub near_redundant_facet_removal_delta: f64,
}

#[derive(Clone, Debug, Default)]
pub struct ScanTimingBreakdown {
    pub validation: F64ValidationTimingBreakdown,
    pub capacity: Option<F64CapacityTimingBreakdown>,
}

impl Default for ScanOptions {
    fn default() -> Self {
        Self {
            audit_generated: false,
            audit_preprocessed: false,
            validation_policy: F64ValidationPolicy::LpOriginVertex,
            capacity_method: F64CapacityMethod::ProductBilliardOrHk,
            near_redundant_facet_removal: NearRedundantFacetRemovalPolicy::None,
            near_redundant_facet_removal_delta: 1e-8,
        }
    }
}

pub fn scan_case(case: ScanCase) -> ScanRow {
    scan_case_with_options(case, &ScanOptions::default())
}

pub fn scan_case_with_options(case: ScanCase, options: &ScanOptions) -> ScanRow {
    scan_case_with_options_profiled(case, options).0
}

pub fn scan_case_with_options_profiled(
    mut case: ScanCase,
    options: &ScanOptions,
) -> (ScanRow, ScanTimingBreakdown) {
    let original_facet_count = case.dual_vertices.len();
    let product_rounding = product_preprocess_for_options(&case, options);
    if product_rounding.should_use_rounded_vertices() {
        case.dual_vertices = product_rounding.rounded_dual_vertices.clone();
    }
    let near_redundant_facet_removal = near_redundant_facet_removal_for_options(&case, options);
    let input_had_facet_removal =
        near_redundant_facet_removal.status == NearRedundantFacetRemovalStatus::Removed;
    if input_had_facet_removal {
        case.dual_vertices = near_redundant_facet_removal.vertices_after_removal.clone();
        case.audit_capacity_label = None;
        case.audit_sigma_label = None;
    }

    let validation_started = Instant::now();
    let (validation, validation_timing) = validate_f64_polytope_input_with_policy_profiled(
        &case.dual_vertices,
        options.validation_policy,
    );
    let validation_time_ms = validation_started.elapsed().as_secs_f64() * 1000.0;
    let mut timing = ScanTimingBreakdown {
        validation: validation_timing,
        capacity: None,
    };
    let capacity_report = if validation.status.capacity_may_run() {
        let capacity_started = Instant::now();
        let (report, capacity_timing) = capacity_f64_only_with_policy_and_method_profiled(
            &case.dual_vertices,
            options.validation_policy,
            options.capacity_method,
        );
        timing.capacity = Some(capacity_timing);
        Some((report, capacity_started.elapsed().as_secs_f64() * 1000.0))
    } else {
        None
    };
    let exact_audit = exact_audit_for_case(&case, options, input_had_facet_removal);
    let audit_capacity_label = exact_audit.capacity_label.or(case.audit_capacity_label);
    let audit_sigma_label = exact_audit
        .sigma_label
        .clone()
        .or_else(|| case.audit_sigma_label.clone());

    match capacity_report {
        Some((report, f64_time_ms)) => (
            capacity_row(
                case,
                validation,
                validation_time_ms,
                options.validation_policy,
                options.capacity_method,
                report,
                f64_time_ms,
                original_facet_count,
                product_rounding,
                near_redundant_facet_removal,
                exact_audit,
                audit_capacity_label,
                audit_sigma_label,
            ),
            timing,
        ),
        None => (
            validation_only_row(
                case,
                validation,
                validation_time_ms,
                options.validation_policy,
                options.capacity_method,
                original_facet_count,
                product_rounding,
                near_redundant_facet_removal,
                exact_audit,
                audit_capacity_label,
                audit_sigma_label,
            ),
            timing,
        ),
    }
}

fn capacity_row(
    case: ScanCase,
    validation: F64ValidationReport,
    validation_time_ms: f64,
    validation_policy: F64ValidationPolicy,
    capacity_method: F64CapacityMethod,
    report: F64CapacityReport,
    f64_time_ms: f64,
    original_facet_count: usize,
    product_rounding: ProductRoundingReport,
    near_redundant_facet_removal: NearRedundantFacetRemovalReport,
    exact_audit: ExactAuditReport,
    audit_capacity_label: Option<f64>,
    audit_sigma_label: Option<Vec<usize>>,
) -> ScanRow {
    if !validation.status.capacity_may_run() {
        unreachable!("capacity_row requires accepted validation");
    }
    let f64_capacity = report.outcome.capacity();
    let original_artifact_capacity_label = case.artifact_capacity_label;
    let preprocessed_f64_capacity = f64_capacity;
    let preprocessed_audit_capacity_label = exact_audit.capacity_label;
    let (abs_action_error, rel_action_error) = match (f64_capacity, audit_capacity_label) {
        (Some(actual), Some(audit_label)) => {
            let abs = (actual - audit_label).abs();
            let rel = abs / audit_label.abs().max(1.0);
            (Some(abs), Some(rel))
        }
        _ => (None, None),
    };
    let classification = classify_report(
        &report,
        audit_capacity_label,
        abs_action_error,
        rel_action_error,
    );
    let preprocessed_f64_vs_preprocessed_audit =
        comparison_error(preprocessed_f64_capacity, preprocessed_audit_capacity_label);
    let preprocessed_f64_vs_original_artifact =
        comparison_error(preprocessed_f64_capacity, original_artifact_capacity_label);
    let preprocessed_audit_vs_original_artifact = comparison_error(
        preprocessed_audit_capacity_label,
        original_artifact_capacity_label,
    );
    let capacity_ratio_upper_bound = near_redundant_facet_removal.capacity_ratio_upper;
    let original_artifact_bound_applies = original_artifact_bound_applies(&product_rounding);
    let preprocessed_f64_vs_original_artifact_within_bound = original_artifact_bound_applies
        .then(|| {
            within_capacity_distortion_bound(
                preprocessed_f64_capacity,
                original_artifact_capacity_label,
                capacity_ratio_upper_bound,
            )
        })
        .flatten();
    let preprocessed_audit_vs_original_artifact_within_bound = original_artifact_bound_applies
        .then(|| {
            within_capacity_distortion_bound(
                preprocessed_audit_capacity_label,
                original_artifact_capacity_label,
                capacity_ratio_upper_bound,
            )
        })
        .flatten();
    let (trust_class, trust_reasons) = validation_adjusted_trust(
        validation.status.clone(),
        classification.trust_class.label(),
        classification.trust_reasons,
        &validation.reasons,
    );

    ScanRow {
        family: case.family,
        source_id: case.source_id,
        input_source: case.input_source,
        generated_attempt: case.generated_attempt,
        generator_seed: case.generator_seed,
        requested_facet_count: case.requested_facet_count,
        original_facet_count: Some(original_facet_count),
        facet_count: case.dual_vertices.len(),
        product_rounding_status: product_rounding.status.label().to_string(),
        product_rounding_max_minor_over_major: product_rounding.max_minor_over_major,
        product_rounding_max_abs_change: product_rounding.max_abs_change,
        product_q_facet_count: Some(product_rounding.q_facet_count),
        product_p_facet_count: Some(product_rounding.p_facet_count),
        near_redundant_facet_removal_policy: near_redundant_facet_removal
            .policy
            .label()
            .to_string(),
        near_redundant_facet_removal_status: near_redundant_facet_removal
            .status
            .label()
            .to_string(),
        preprocessed_facet_count: Some(near_redundant_facet_removal.vertices_after_removal.len()),
        removed_facet_count: near_redundant_facet_removal.removed_facets.len(),
        removed_original_facets: near_redundant_facet_removal
            .removed_facets
            .iter()
            .map(|facet| facet.original_index())
            .collect(),
        near_redundant_facet_removal_delta_bound: Some(near_redundant_facet_removal.delta_bound),
        capacity_ratio_upper_bound: Some(near_redundant_facet_removal.capacity_ratio_upper),
        volume_ratio_upper_bound: Some(near_redundant_facet_removal.volume_ratio_upper),
        sys_ratio_lower_bound: Some(near_redundant_facet_removal.sys_ratio_lower),
        sys_ratio_upper_bound: Some(near_redundant_facet_removal.sys_ratio_upper),
        validation_policy: validation_policy.label().to_string(),
        capacity_method: capacity_method.label().to_string(),
        validation_status: validation.status.label().to_string(),
        validation_reasons: validation.reasons.clone(),
        validation_time_ms,
        origin_status: validation.origin_status.label().to_string(),
        origin_lp_status: validation.origin_lp_status,
        origin_lp_max_min_lambda: validation.origin_lp_max_min_lambda,
        origin_lp_max_abs_residual: validation.origin_lp_max_abs_residual,
        facet_extremality_status: validation.facet_extremality_status.label().to_string(),
        facets_with_definite_vertex_count: report.facets_with_definite_vertex_count,
        facets_with_possible_vertex_count: report.facets_with_possible_vertex_count,
        facets_without_definite_vertex_count: validation.facets_without_definite_vertex_count,
        facets_without_possible_vertex_count: validation.facets_without_possible_vertex_count,
        outcome: report.outcome.outcome_label().to_string(),
        failure_reason: report.outcome.failure_reason(),
        f64_capacity,
        preprocessed_f64_capacity,
        audit_capacity_label,
        original_artifact_capacity_label,
        preprocessed_audit_capacity_label,
        artifact_capacity_label: case.artifact_capacity_label,
        exact_audit_status: exact_audit.status.label().to_string(),
        exact_audit_time_ms: exact_audit.time_ms,
        exact_audit_reasons: exact_audit.reasons,
        abs_action_error,
        rel_action_error,
        preprocessed_f64_vs_preprocessed_audit_abs_error: preprocessed_f64_vs_preprocessed_audit
            .abs,
        preprocessed_f64_vs_preprocessed_audit_rel_error: preprocessed_f64_vs_preprocessed_audit
            .rel,
        preprocessed_f64_vs_original_artifact_abs_error: preprocessed_f64_vs_original_artifact.abs,
        preprocessed_f64_vs_original_artifact_rel_error: preprocessed_f64_vs_original_artifact.rel,
        preprocessed_f64_vs_original_artifact_within_bound,
        preprocessed_audit_vs_original_artifact_abs_error: preprocessed_audit_vs_original_artifact
            .abs,
        preprocessed_audit_vs_original_artifact_rel_error: preprocessed_audit_vs_original_artifact
            .rel,
        preprocessed_audit_vs_original_artifact_within_bound,
        f64_time_ms,
        agreement_status: classification.agreement_status.label().to_string(),
        trust_class,
        trust_reasons,
        f64_sigma: report.outcome.sigma(),
        audit_sigma_label,
        sigma_count: report.sigma_count,
        admissible_f64_count: report.admissible_f64_count,
        indeterminate_f64_count: report.indeterminate_f64_count,
        inadmissible_count: report.inadmissible_count,
        numerical_failure_count: report.numerical_failure_count,
        vertex_count: report.vertex_count,
        vertex_indeterminate_count: report.vertex_indeterminate_count,
        near_singular_vertex_count: report.near_singular_vertex_count,
        bounded_near_singular_vertex_count: report.bounded_near_singular_vertex_count,
        ambiguous_vertex_incidence_count: report.ambiguous_vertex_incidence_count,
        facet_intersection_true_count: report.facet_intersection_true_count,
        facet_intersection_false_count: report.facet_intersection_false_count,
        facet_intersection_indeterminate_count: report.facet_intersection_indeterminate_count,
        omega_indeterminate_count: report.omega_indeterminate_count,
        min_action_gap: report.min_action_gap,
        indeterminate_overlaps_best_interval: report.indeterminate_overlaps_best_interval,
    }
}

fn validation_only_row(
    case: ScanCase,
    validation: F64ValidationReport,
    validation_time_ms: f64,
    validation_policy: F64ValidationPolicy,
    capacity_method: F64CapacityMethod,
    original_facet_count: usize,
    product_rounding: ProductRoundingReport,
    near_redundant_facet_removal: NearRedundantFacetRemovalReport,
    exact_audit: ExactAuditReport,
    audit_capacity_label: Option<f64>,
    audit_sigma_label: Option<Vec<usize>>,
) -> ScanRow {
    let original_artifact_capacity_label = case.artifact_capacity_label;
    let preprocessed_audit_capacity_label = exact_audit.capacity_label;
    let preprocessed_audit_vs_original_artifact = comparison_error(
        preprocessed_audit_capacity_label,
        original_artifact_capacity_label,
    );
    let capacity_ratio_upper_bound = near_redundant_facet_removal.capacity_ratio_upper;
    let preprocessed_audit_vs_original_artifact_within_bound =
        original_artifact_bound_applies(&product_rounding)
            .then(|| {
                within_capacity_distortion_bound(
                    preprocessed_audit_capacity_label,
                    original_artifact_capacity_label,
                    capacity_ratio_upper_bound,
                )
            })
            .flatten();
    ScanRow {
        family: case.family,
        source_id: case.source_id,
        input_source: case.input_source,
        generated_attempt: case.generated_attempt,
        generator_seed: case.generator_seed,
        requested_facet_count: case.requested_facet_count,
        original_facet_count: Some(original_facet_count),
        facet_count: case.dual_vertices.len(),
        product_rounding_status: product_rounding.status.label().to_string(),
        product_rounding_max_minor_over_major: product_rounding.max_minor_over_major,
        product_rounding_max_abs_change: product_rounding.max_abs_change,
        product_q_facet_count: Some(product_rounding.q_facet_count),
        product_p_facet_count: Some(product_rounding.p_facet_count),
        near_redundant_facet_removal_policy: near_redundant_facet_removal
            .policy
            .label()
            .to_string(),
        near_redundant_facet_removal_status: near_redundant_facet_removal
            .status
            .label()
            .to_string(),
        preprocessed_facet_count: Some(near_redundant_facet_removal.vertices_after_removal.len()),
        removed_facet_count: near_redundant_facet_removal.removed_facets.len(),
        removed_original_facets: near_redundant_facet_removal
            .removed_facets
            .iter()
            .map(|facet| facet.original_index())
            .collect(),
        near_redundant_facet_removal_delta_bound: Some(near_redundant_facet_removal.delta_bound),
        capacity_ratio_upper_bound: Some(near_redundant_facet_removal.capacity_ratio_upper),
        volume_ratio_upper_bound: Some(near_redundant_facet_removal.volume_ratio_upper),
        sys_ratio_lower_bound: Some(near_redundant_facet_removal.sys_ratio_lower),
        sys_ratio_upper_bound: Some(near_redundant_facet_removal.sys_ratio_upper),
        validation_policy: validation_policy.label().to_string(),
        capacity_method: capacity_method.label().to_string(),
        validation_status: validation.status.label().to_string(),
        validation_reasons: validation.reasons.clone(),
        validation_time_ms,
        origin_status: validation.origin_status.label().to_string(),
        origin_lp_status: validation.origin_lp_status,
        origin_lp_max_min_lambda: validation.origin_lp_max_min_lambda,
        origin_lp_max_abs_residual: validation.origin_lp_max_abs_residual,
        facet_extremality_status: validation.facet_extremality_status.label().to_string(),
        facets_with_definite_vertex_count: validation.facets_with_definite_vertex_count,
        facets_with_possible_vertex_count: validation.facets_with_possible_vertex_count,
        facets_without_definite_vertex_count: validation.facets_without_definite_vertex_count,
        facets_without_possible_vertex_count: validation.facets_without_possible_vertex_count,
        outcome: "not_run".to_string(),
        failure_reason: Some(format!("validation_{}", validation.status.label())),
        f64_capacity: None,
        preprocessed_f64_capacity: None,
        audit_capacity_label,
        original_artifact_capacity_label,
        preprocessed_audit_capacity_label,
        artifact_capacity_label: case.artifact_capacity_label,
        exact_audit_status: exact_audit.status.label().to_string(),
        exact_audit_time_ms: exact_audit.time_ms,
        exact_audit_reasons: exact_audit.reasons,
        abs_action_error: None,
        rel_action_error: None,
        preprocessed_f64_vs_preprocessed_audit_abs_error: None,
        preprocessed_f64_vs_preprocessed_audit_rel_error: None,
        preprocessed_f64_vs_original_artifact_abs_error: None,
        preprocessed_f64_vs_original_artifact_rel_error: None,
        preprocessed_f64_vs_original_artifact_within_bound: None,
        preprocessed_audit_vs_original_artifact_abs_error: preprocessed_audit_vs_original_artifact
            .abs,
        preprocessed_audit_vs_original_artifact_rel_error: preprocessed_audit_vs_original_artifact
            .rel,
        preprocessed_audit_vs_original_artifact_within_bound,
        f64_time_ms: 0.0,
        agreement_status: "no_f64_capacity".to_string(),
        trust_class: "fallback_required".to_string(),
        trust_reasons: validation
            .reasons
            .into_iter()
            .map(|reason| format!("validation:{reason}"))
            .collect(),
        f64_sigma: None,
        audit_sigma_label,
        sigma_count: 0,
        admissible_f64_count: 0,
        indeterminate_f64_count: 0,
        inadmissible_count: 0,
        numerical_failure_count: 0,
        vertex_count: validation.vertex_count,
        vertex_indeterminate_count: validation.vertex_indeterminate_count,
        near_singular_vertex_count: validation.near_singular_vertex_count,
        bounded_near_singular_vertex_count: validation.bounded_near_singular_vertex_count,
        ambiguous_vertex_incidence_count: validation.ambiguous_vertex_incidence_count,
        facet_intersection_true_count: 0,
        facet_intersection_false_count: 0,
        facet_intersection_indeterminate_count: validation.facet_intersection_indeterminate_count,
        omega_indeterminate_count: validation.omega_indeterminate_count,
        min_action_gap: None,
        indeterminate_overlaps_best_interval: false,
    }
}

fn product_preprocess_for_options(case: &ScanCase, options: &ScanOptions) -> ProductRoundingReport {
    if options.capacity_method == F64CapacityMethod::ProductBilliardOrHk
        || options.near_redundant_facet_removal == NearRedundantFacetRemovalPolicy::Product
    {
        product::round_blocks(&case.dual_vertices)
    } else {
        ProductRoundingReport::not_attempted(&case.dual_vertices)
    }
}

fn near_redundant_facet_removal_for_options(
    case: &ScanCase,
    options: &ScanOptions,
) -> NearRedundantFacetRemovalReport {
    match options.near_redundant_facet_removal {
        NearRedundantFacetRemovalPolicy::None => {
            NearRedundantFacetRemovalReport::not_attempted(&case.dual_vertices)
        }
        NearRedundantFacetRemovalPolicy::Product => {
            NearRedundantFacetRemovalReport::from_product(product::remove_near_redundant_facets(
                &case.dual_vertices,
                options.near_redundant_facet_removal_delta,
            ))
        }
        NearRedundantFacetRemovalPolicy::Generic => generic::remove_near_redundant_facets(
            &case.dual_vertices,
            options.near_redundant_facet_removal_delta,
        ),
    }
}

fn exact_audit_for_case(
    case: &ScanCase,
    options: &ScanOptions,
    input_had_facet_removal: bool,
) -> ExactAuditReport {
    if options.audit_generated && case.input_source == "generated_f64" {
        audit_generated_case_exact(&case.dual_vertices)
    } else if options.audit_preprocessed && input_had_facet_removal {
        audit_generated_case_exact(&case.dual_vertices)
    } else {
        exact_audit_not_requested()
    }
}

fn validation_adjusted_trust(
    validation_status: F64ValidationStatus,
    capacity_trust_class: &str,
    mut trust_reasons: Vec<String>,
    validation_reasons: &[String],
) -> (String, Vec<String>) {
    if validation_status != F64ValidationStatus::AcceptedDecisive {
        trust_reasons.push(format!("validation_status:{}", validation_status.label()));
        for reason in validation_reasons {
            trust_reasons.push(format!("validation:{reason}"));
        }
    }
    let trust_class = match validation_status {
        F64ValidationStatus::AcceptedDecisive => capacity_trust_class.to_string(),
        F64ValidationStatus::AcceptedAmbiguous if capacity_trust_class == "clean" => {
            "degenerate_value_agrees".to_string()
        }
        F64ValidationStatus::AcceptedAmbiguous => capacity_trust_class.to_string(),
        F64ValidationStatus::Rejected | F64ValidationStatus::FallbackRequired => {
            "fallback_required".to_string()
        }
    };
    (trust_class, trust_reasons)
}

#[derive(Clone, Copy, Debug)]
struct CapacityComparison {
    abs: Option<f64>,
    rel: Option<f64>,
}

fn comparison_error(left: Option<f64>, right: Option<f64>) -> CapacityComparison {
    match (left, right) {
        (Some(left), Some(right)) => {
            let abs = (left - right).abs();
            let rel = abs / right.abs().max(1.0);
            CapacityComparison {
                abs: Some(abs),
                rel: Some(rel),
            }
        }
        _ => CapacityComparison {
            abs: None,
            rel: None,
        },
    }
}

fn within_capacity_distortion_bound(
    value: Option<f64>,
    original: Option<f64>,
    capacity_ratio_upper_bound: f64,
) -> Option<bool> {
    let (Some(value), Some(original)) = (value, original) else {
        return None;
    };
    let lower = original * (1.0 - BOUND_COMPARISON_TOLERANCE);
    let upper = original * capacity_ratio_upper_bound * (1.0 + BOUND_COMPARISON_TOLERANCE);
    Some(value >= lower && value <= upper)
}

fn original_artifact_bound_applies(product_rounding: &ProductRoundingReport) -> bool {
    if !product_rounding.should_use_rounded_vertices() {
        return true;
    }
    product_rounding.max_abs_change == Some(0.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra::Vector4;
    use symplectic::known_polytopes;

    #[test]
    fn scan_case_does_not_audit_by_default() {
        let row = scan_case(simplex_case());
        assert_eq!(row.exact_audit_status, "not_requested");
        assert!(row.audit_capacity_label.is_none());
        assert_eq!(row.capacity_method, "product_billiard_or_hk");
    }

    #[test]
    fn scan_case_can_audit_generated_success() {
        let row = scan_case_with_options(
            simplex_case(),
            &ScanOptions {
                audit_generated: true,
                audit_preprocessed: false,
                validation_policy: F64ValidationPolicy::Strict,
                capacity_method: F64CapacityMethod::TransitionPrunedHk,
                near_redundant_facet_removal: NearRedundantFacetRemovalPolicy::None,
                near_redundant_facet_removal_delta: 1e-8,
            },
        );
        assert_eq!(row.exact_audit_status, "exact_valid_capacity_success");
        assert!(row.audit_capacity_label.is_some());
        assert!(row.audit_sigma_label.is_some());
    }

    #[test]
    fn scan_case_can_audit_generated_exact_rejection_after_f64_rejection() {
        let row = scan_case_with_options(
            duplicate_case(),
            &ScanOptions {
                audit_generated: true,
                audit_preprocessed: false,
                validation_policy: F64ValidationPolicy::Strict,
                capacity_method: F64CapacityMethod::TransitionPrunedHk,
                near_redundant_facet_removal: NearRedundantFacetRemovalPolicy::None,
                near_redundant_facet_removal_delta: 1e-8,
            },
        );
        assert_eq!(row.validation_status, "rejected");
        assert_eq!(row.exact_audit_status, "exact_validation_rejected");
        assert_eq!(row.outcome, "not_run");
    }

    #[test]
    fn scan_records_explicit_product_rounding() {
        let fixture = known_polytopes::lagrangian_triangle_product();
        let mut dual_vertices = fixture.dual_vertices_f64.clone();
        for vertex in &mut dual_vertices {
            if vertex[2] == 0.0 && vertex[3] == 0.0 {
                vertex[2] = 1e-14;
            } else {
                vertex[0] = -1e-14;
            }
        }
        let row = scan_case(ScanCase {
            family: "test_product".to_string(),
            source_id: "drifted_product".to_string(),
            input_source: "generated_f64".to_string(),
            generated_attempt: Some(0),
            generator_seed: Some(0),
            requested_facet_count: Some(dual_vertices.len()),
            dual_vertices,
            audit_capacity_label: None,
            artifact_capacity_label: None,
            audit_sigma_label: None,
        });
        assert_eq!(row.product_rounding_status, "rounded");
        assert_eq!(row.original_facet_count, Some(row.facet_count));
        assert!(row.product_rounding_max_abs_change.unwrap() > 0.0);
        assert!(row.product_rounding_max_minor_over_major.unwrap() < 1e-9);
        assert_eq!(row.near_redundant_facet_removal_policy, "none");
        assert_eq!(row.near_redundant_facet_removal_status, "not_attempted");
        assert_eq!(row.removed_facet_count, 0);
    }

    #[test]
    fn scan_does_not_apply_facet_removal_bound_to_product_rounding_change() {
        let fixture = known_polytopes::lagrangian_triangle_product();
        let mut dual_vertices = fixture.dual_vertices_f64.clone();
        for vertex in &mut dual_vertices {
            if vertex[2] == 0.0 && vertex[3] == 0.0 {
                vertex[2] = 1e-14;
            } else {
                vertex[0] = -1e-14;
            }
        }
        let row = scan_case(ScanCase {
            family: "test_product".to_string(),
            source_id: "drifted_product_with_label".to_string(),
            input_source: "generated_f64".to_string(),
            generated_attempt: Some(0),
            generator_seed: Some(0),
            requested_facet_count: Some(dual_vertices.len()),
            dual_vertices,
            audit_capacity_label: Some(1.0),
            artifact_capacity_label: Some(1.0),
            audit_sigma_label: None,
        });

        assert_eq!(row.product_rounding_status, "rounded");
        assert!(row.product_rounding_max_abs_change.unwrap() > 0.0);
        assert!(row
            .preprocessed_f64_vs_original_artifact_abs_error
            .is_some());
        assert_eq!(row.preprocessed_f64_vs_original_artifact_within_bound, None);
    }

    #[test]
    fn scan_product_facet_removal_removes_near_redundant_facets_explicitly() {
        let mut case = near_redundant_product_case();
        case.audit_capacity_label = Some(1.0);
        case.artifact_capacity_label = Some(1.0);
        let row = scan_case_with_options(
            case,
            &ScanOptions {
                audit_generated: false,
                audit_preprocessed: true,
                validation_policy: F64ValidationPolicy::LpOriginVertex,
                capacity_method: F64CapacityMethod::ProductBilliardOrHk,
                near_redundant_facet_removal: NearRedundantFacetRemovalPolicy::Product,
                near_redundant_facet_removal_delta: 2e-8,
            },
        );
        assert_eq!(
            row.near_redundant_facet_removal_policy,
            "product_remove_near_redundant_facets"
        );
        assert_eq!(row.near_redundant_facet_removal_status, "removed");
        assert_eq!(row.original_facet_count, Some(8));
        assert_eq!(row.facet_count, 7);
        assert_eq!(row.preprocessed_facet_count, Some(7));
        assert_eq!(row.removed_facet_count, 1);
        let delta_bound = row.near_redundant_facet_removal_delta_bound.unwrap();
        let scale = 1.0 + delta_bound;
        assert!(delta_bound <= 2e-8);
        assert_eq!(row.capacity_ratio_upper_bound.unwrap(), scale.powi(2));
        assert_eq!(row.volume_ratio_upper_bound.unwrap(), scale.powi(4));
        assert_eq!(row.sys_ratio_lower_bound.unwrap(), scale.powi(-4));
        assert_eq!(row.sys_ratio_upper_bound.unwrap(), scale.powi(4));
        assert_eq!(row.exact_audit_status, "exact_valid_capacity_success");
        assert!(row.audit_capacity_label.is_some());
        assert_eq!(
            row.audit_capacity_label,
            row.preprocessed_audit_capacity_label
        );
        assert_eq!(row.original_artifact_capacity_label, Some(1.0));
        assert_eq!(row.artifact_capacity_label, Some(1.0));
        assert_eq!(
            row.preprocessed_audit_vs_original_artifact_within_bound,
            Some(false)
        );
    }

    #[test]
    fn scan_facet_removal_keeps_original_label_without_using_it_as_same_polytope_audit() {
        let mut case = near_redundant_product_case();
        case.audit_capacity_label = Some(1.0);
        case.artifact_capacity_label = Some(1.0);
        let row = scan_case_with_options(
            case,
            &ScanOptions {
                audit_generated: false,
                audit_preprocessed: false,
                validation_policy: F64ValidationPolicy::LpOriginVertex,
                capacity_method: F64CapacityMethod::ProductBilliardOrHk,
                near_redundant_facet_removal: NearRedundantFacetRemovalPolicy::Product,
                near_redundant_facet_removal_delta: 2e-8,
            },
        );
        assert_eq!(
            row.near_redundant_facet_removal_policy,
            "product_remove_near_redundant_facets"
        );
        assert_eq!(row.near_redundant_facet_removal_status, "removed");
        assert!(row.audit_capacity_label.is_none());
        assert!(row.preprocessed_audit_capacity_label.is_none());
        assert_eq!(row.original_artifact_capacity_label, Some(1.0));
        assert_eq!(row.artifact_capacity_label, Some(1.0));
    }

    #[test]
    fn scan_product_facet_removal_reports_non_products_without_changing_input() {
        let row = scan_case_with_options(
            simplex_case(),
            &ScanOptions {
                audit_generated: false,
                audit_preprocessed: false,
                validation_policy: F64ValidationPolicy::LpOriginVertex,
                capacity_method: F64CapacityMethod::TransitionPrunedHk,
                near_redundant_facet_removal: NearRedundantFacetRemovalPolicy::Product,
                near_redundant_facet_removal_delta: 1e-8,
            },
        );
        assert_eq!(
            row.near_redundant_facet_removal_policy,
            "product_remove_near_redundant_facets"
        );
        assert_eq!(row.near_redundant_facet_removal_status, "not_block_product");
        assert_eq!(row.original_facet_count, Some(row.facet_count));
        assert_eq!(row.preprocessed_facet_count, Some(row.facet_count));
        assert_eq!(row.removed_facet_count, 0);
    }

    #[test]
    fn scan_generic_facet_removal_uses_generic_fields() {
        let mut case = generic_near_redundant_case();
        case.audit_capacity_label = Some(1.0);
        case.artifact_capacity_label = Some(1.0);
        let row = scan_case_with_options(
            case,
            &ScanOptions {
                audit_generated: false,
                audit_preprocessed: false,
                validation_policy: F64ValidationPolicy::LpOriginVertex,
                capacity_method: F64CapacityMethod::TransitionPrunedHk,
                near_redundant_facet_removal: NearRedundantFacetRemovalPolicy::Generic,
                near_redundant_facet_removal_delta: 2e-8,
            },
        );
        assert_eq!(
            row.near_redundant_facet_removal_policy,
            "generic_remove_near_redundant_facets"
        );
        assert_eq!(row.near_redundant_facet_removal_status, "removed");
        assert_eq!(row.original_facet_count, Some(9));
        assert_eq!(row.facet_count, 8);
        assert_eq!(row.removed_original_facets.len(), 1);
        assert!([0, 8].contains(&row.removed_original_facets[0]));
        assert!(row.near_redundant_facet_removal_delta_bound.unwrap() <= 2e-8);
    }

    fn simplex_case() -> ScanCase {
        let first = Vector4::new(1.0, 0.2, 0.3, 0.4);
        let second = Vector4::new(0.1, 1.0, 0.5, -0.2);
        let third = Vector4::new(-0.3, 0.4, 1.0, 0.6);
        let fourth = Vector4::new(0.2, -0.5, 0.4, 1.0);
        ScanCase {
            family: "test_generated".to_string(),
            source_id: "simplex".to_string(),
            input_source: "generated_f64".to_string(),
            generated_attempt: Some(0),
            generator_seed: Some(0),
            requested_facet_count: Some(5),
            dual_vertices: vec![
                first,
                second,
                third,
                fourth,
                -(first + second + third + fourth),
            ],
            audit_capacity_label: None,
            artifact_capacity_label: None,
            audit_sigma_label: None,
        }
    }

    fn duplicate_case() -> ScanCase {
        ScanCase {
            family: "test_generated".to_string(),
            source_id: "duplicate".to_string(),
            input_source: "generated_f64".to_string(),
            generated_attempt: Some(1),
            generator_seed: Some(0),
            requested_facet_count: Some(5),
            dual_vertices: vec![
                Vector4::new(1.0, 0.0, 0.0, 0.0),
                Vector4::new(1.0, 0.0, 0.0, 0.0),
                Vector4::new(0.0, 1.0, 0.0, 0.0),
                Vector4::new(0.0, 0.0, 1.0, 0.0),
                Vector4::new(0.0, 0.0, 0.0, 1.0),
            ],
            audit_capacity_label: None,
            artifact_capacity_label: None,
            audit_sigma_label: None,
        }
    }

    fn near_redundant_product_case() -> ScanCase {
        let eps = 1e-8;
        ScanCase {
            family: "test_product".to_string(),
            source_id: "near_redundant_product".to_string(),
            input_source: "generated_f64".to_string(),
            generated_attempt: Some(2),
            generator_seed: Some(0),
            requested_facet_count: Some(8),
            dual_vertices: vec![
                Vector4::new(1.0, 0.0, 0.0, 0.0),
                Vector4::new(0.0, 1.0, 0.0, 0.0),
                Vector4::new(-1.0, 0.0, 0.0, 0.0),
                Vector4::new(0.0, -1.0, 0.0, 0.0),
                Vector4::new(1.0, eps, 0.0, 0.0),
                Vector4::new(0.0, 0.0, 1.0, 0.0),
                Vector4::new(0.0, 0.0, 0.0, 1.0),
                Vector4::new(0.0, 0.0, -1.0, -1.0),
            ],
            audit_capacity_label: None,
            artifact_capacity_label: None,
            audit_sigma_label: None,
        }
    }

    fn generic_near_redundant_case() -> ScanCase {
        let eps = 1e-8;
        ScanCase {
            family: "test_generic".to_string(),
            source_id: "near_redundant_generic".to_string(),
            input_source: "generated_f64".to_string(),
            generated_attempt: Some(3),
            generator_seed: Some(0),
            requested_facet_count: Some(9),
            dual_vertices: vec![
                Vector4::new(1.0, 0.0, 0.0, 0.0),
                Vector4::new(-1.0, 0.0, 0.0, 0.0),
                Vector4::new(0.0, 1.0, 0.0, 0.0),
                Vector4::new(0.0, -1.0, 0.0, 0.0),
                Vector4::new(0.0, 0.0, 1.0, 0.0),
                Vector4::new(0.0, 0.0, -1.0, 0.0),
                Vector4::new(0.0, 0.0, 0.0, 1.0),
                Vector4::new(0.0, 0.0, 0.0, -1.0),
                Vector4::new(1.0, eps, 0.0, 0.0),
            ],
            audit_capacity_label: None,
            artifact_capacity_label: None,
            audit_sigma_label: None,
        }
    }
}

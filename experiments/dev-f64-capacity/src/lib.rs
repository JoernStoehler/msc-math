mod artifact_cases;
mod audit;
mod capacity;
mod cases;
mod classify;
mod decision_compare;
mod facet_simplify;
mod generated_cases;
mod geometry;
mod product_preprocess;
mod product_simplify;
mod rows;
mod scan;
mod validation;

pub use artifact_cases::{hko_case, load_retained_artifact_cases};
pub use audit::{
    audit_generated_case_exact, exact_audit_not_requested, ExactAuditReport, ExactAuditStatus,
};
pub use capacity::{
    capacity_f64_only, capacity_f64_only_with_policy,
    capacity_f64_only_with_policy_and_method_profiled, capacity_f64_only_with_policy_profiled,
    F64CapacityMethod, F64CapacityOutcome, F64CapacityReport, F64CapacityTimingBreakdown,
    F64FailureReason,
};
pub use cases::{array_vertices_to_vectors, ScanCase};
pub use classify::{
    classify_report, AgreementStatus, Classification, TrustClass, ABS_ACTION_TOLERANCE,
    REL_ACTION_TOLERANCE, TINY_GAP_THRESHOLD,
};
pub use decision_compare::{
    compare_f64_decisions, DecisionComparisonReport, DecisionComparisonRow, SingleMethodDecisionRow,
};
pub use facet_simplify::{
    remove_nearly_redundant_facets_single_band, FacetSimplificationPolicy,
    FacetSimplificationRemoval, FacetSimplificationReport, FacetSimplificationStatus,
};
pub use generated_cases::generated_f64_cases;
pub use geometry::F64CombinatoricsTiming;
pub use product_preprocess::{
    round_product_blocks, ProductBlock, ProductRoundingReport, ProductRoundingStatus,
    PRODUCT_ROUNDING_MAX_MINOR_OVER_MAJOR,
};
pub use product_simplify::{
    remove_nearly_redundant_product_facets, ProductFacetRedundancy, ProductSimplificationReport,
    ProductSimplificationStatus,
};
pub use rows::ScanRow;
pub use scan::{
    scan_case, scan_case_with_options, scan_case_with_options_profiled, ScanOptions,
    ScanTimingBreakdown,
};
pub use validation::{
    validate_f64_polytope_input, validate_f64_polytope_input_with_policy,
    validate_f64_polytope_input_with_policy_profiled, F64PredicateStatus, F64ValidationPolicy,
    F64ValidationReport, F64ValidationStatus, F64ValidationTimingBreakdown,
};

mod artifact_cases;
mod audit;
mod capacity;
mod cases;
mod classify;
mod decision_compare;
mod edge_cases;
mod generated_cases;
mod geometry;
mod near_redundant_facet_removal;
mod rows;
mod scan;
mod validation;

pub mod generic;
pub mod product;

pub use artifact_cases::{hko_case, load_retained_artifact_cases};
pub use audit::{
    audit_generated_case_exact, exact_audit_not_requested, ExactAuditReport, ExactAuditStatus,
};
pub use capacity::{
    capacity_f64_only, capacity_f64_only_with_policy,
    capacity_f64_only_with_policy_and_method_profiled, capacity_f64_only_with_policy_profiled,
    F64CapacityMethod, F64CapacityOutcome, F64CapacityReport, F64CapacityTimingBreakdown,
    F64FailureReason, MINIMIZING_SIGMA_SET_ACTION_TOLERANCE,
};
pub use cases::{array_vertices_to_vectors, ScanCase};
pub use classify::{
    classify_report, output_epistemics_not_computed, AgreementStatus, CapacityLabelStatus,
    Classification, LowActionItemsStatus, LowActionListCompleteness, OutputDecisionStatus,
    OutputEpistemics, TrustClass, ABS_ACTION_TOLERANCE, REL_ACTION_TOLERANCE, TINY_GAP_THRESHOLD,
};
pub use decision_compare::{
    compare_f64_decisions, DecisionComparisonReport, DecisionComparisonRow, SingleMethodDecisionRow,
};
pub use edge_cases::edge_fixture_cases;
pub use generated_cases::{generated_f64_cases, generated_f64_cases_with_source_filter};
pub use geometry::F64CombinatoricsTiming;
pub use near_redundant_facet_removal::{
    NearRedundantFacetRemoval, NearRedundantFacetRemovalPolicy, NearRedundantFacetRemovalReport,
    NearRedundantFacetRemovalStatus,
};
pub use product::{
    ProductBlock, ProductFacetRemoval, ProductFacetRemovalReport, ProductFacetRemovalStatus,
    ProductRoundingReport, ProductRoundingStatus, PRODUCT_ROUNDING_MAX_MINOR_OVER_MAJOR,
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

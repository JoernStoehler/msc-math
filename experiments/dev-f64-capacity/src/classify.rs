use serde::{Deserialize, Serialize};

use crate::{F64CapacityOutcome, F64CapacityReport};

pub const ABS_ACTION_TOLERANCE: f64 = 1e-10;
pub const REL_ACTION_TOLERANCE: f64 = 1e-10;
pub const TINY_GAP_THRESHOLD: f64 = 1e-8;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AgreementStatus {
    Agrees,
    Disagrees,
    NoAuditLabel,
    NoF64Capacity,
}

impl AgreementStatus {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Agrees => "agrees",
            Self::Disagrees => "disagrees",
            Self::NoAuditLabel => "no_audit_label",
            Self::NoF64Capacity => "no_f64_capacity",
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TrustClass {
    Clean,
    DegenerateValueAgrees,
    FallbackRequired,
}

impl TrustClass {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Clean => "clean",
            Self::DegenerateValueAgrees => "degenerate_value_agrees",
            Self::FallbackRequired => "fallback_required",
        }
    }
}

#[derive(Clone, Debug)]
pub struct Classification {
    pub agreement_status: AgreementStatus,
    pub trust_class: TrustClass,
    pub trust_reasons: Vec<String>,
}

pub fn classify_report(
    report: &F64CapacityReport,
    audit_capacity_label: Option<f64>,
    abs_action_error: Option<f64>,
    rel_action_error: Option<f64>,
) -> Classification {
    let agreement_status = agreement_status(
        report,
        audit_capacity_label,
        abs_action_error,
        rel_action_error,
    );
    let mut reasons = Vec::new();

    if !matches!(report.outcome, F64CapacityOutcome::Success { .. }) {
        reasons.push("f64_outcome_failure".to_string());
    }
    match agreement_status {
        AgreementStatus::Disagrees => reasons.push("audit_label_disagreement".to_string()),
        AgreementStatus::NoF64Capacity => reasons.push("no_f64_capacity".to_string()),
        AgreementStatus::NoAuditLabel => reasons.push("no_audit_label".to_string()),
        AgreementStatus::Agrees => {}
    }

    let ambiguity_reasons = ambiguity_reasons(report);
    let has_ambiguity = !ambiguity_reasons.is_empty();
    reasons.extend(ambiguity_reasons);

    let trust_class = if reasons.iter().any(|reason| {
        matches!(
            reason.as_str(),
            "f64_outcome_failure"
                | "audit_label_disagreement"
                | "no_f64_capacity"
                | "bounded_near_singular_vertex"
        )
    }) || (matches!(agreement_status, AgreementStatus::NoAuditLabel)
        && report.indeterminate_overlaps_best_interval)
    {
        TrustClass::FallbackRequired
    } else if matches!(agreement_status, AgreementStatus::Agrees) && has_ambiguity {
        TrustClass::DegenerateValueAgrees
    } else if matches!(
        agreement_status,
        AgreementStatus::Agrees | AgreementStatus::NoAuditLabel
    ) {
        TrustClass::Clean
    } else {
        TrustClass::FallbackRequired
    };

    Classification {
        agreement_status,
        trust_class,
        trust_reasons: reasons,
    }
}

fn agreement_status(
    report: &F64CapacityReport,
    audit_capacity_label: Option<f64>,
    abs_action_error: Option<f64>,
    rel_action_error: Option<f64>,
) -> AgreementStatus {
    if !matches!(report.outcome, F64CapacityOutcome::Success { .. }) {
        return AgreementStatus::NoF64Capacity;
    }
    if audit_capacity_label.is_none() {
        return AgreementStatus::NoAuditLabel;
    }
    match (abs_action_error, rel_action_error) {
        (Some(abs), Some(rel)) if abs <= ABS_ACTION_TOLERANCE || rel <= REL_ACTION_TOLERANCE => {
            AgreementStatus::Agrees
        }
        (Some(_), Some(_)) => AgreementStatus::Disagrees,
        _ => AgreementStatus::NoAuditLabel,
    }
}

fn ambiguity_reasons(report: &F64CapacityReport) -> Vec<String> {
    let mut reasons = Vec::new();
    if report.vertex_indeterminate_count > 0 {
        reasons.push("vertex_indeterminate".to_string());
    }
    if report.near_singular_vertex_count > 0 {
        reasons.push("near_singular_vertex".to_string());
    }
    if report.bounded_near_singular_vertex_count > 0 {
        reasons.push("bounded_near_singular_vertex".to_string());
    }
    if report.ambiguous_vertex_incidence_count > 0 {
        reasons.push("ambiguous_vertex_incidence".to_string());
    }
    if report.facet_intersection_indeterminate_count > 0 {
        reasons.push("facet_intersection_indeterminate".to_string());
    }
    if report.omega_indeterminate_count > 0 {
        reasons.push("omega_indeterminate".to_string());
    }
    if report.indeterminate_f64_count > 0 {
        reasons.push("kkt_indeterminate".to_string());
    }
    if report.indeterminate_overlaps_best_interval {
        reasons.push("indeterminate_overlaps_best_interval".to_string());
    }
    if report.numerical_failure_count > 0 {
        reasons.push("numerical_failure".to_string());
    }
    if report
        .min_action_gap
        .is_some_and(|gap| gap <= TINY_GAP_THRESHOLD)
    {
        reasons.push("tiny_action_gap".to_string());
    }
    reasons
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::F64FailureReason;

    #[test]
    fn clean_requires_agreement_and_no_ambiguity() {
        let report = base_success_report();
        let classification = classify_report(&report, Some(2.0), Some(0.0), Some(0.0));
        assert_eq!(classification.agreement_status.label(), "agrees");
        assert_eq!(classification.trust_class.label(), "clean");
        assert!(classification.trust_reasons.is_empty());
    }

    #[test]
    fn agreeing_ambiguous_row_is_degenerate() {
        let mut report = base_success_report();
        report.omega_indeterminate_count = 2;
        let classification = classify_report(&report, Some(2.0), Some(0.0), Some(0.0));
        assert_eq!(
            classification.trust_class.label(),
            "degenerate_value_agrees"
        );
        assert_eq!(classification.trust_reasons, vec!["omega_indeterminate"]);
    }

    #[test]
    fn bounded_near_singular_requires_fallback_even_when_value_agrees() {
        let mut report = base_success_report();
        report.bounded_near_singular_vertex_count = 1;
        let classification = classify_report(&report, Some(2.0), Some(0.0), Some(0.0));
        assert_eq!(classification.trust_class.label(), "fallback_required");
        assert_eq!(
            classification.trust_reasons,
            vec!["bounded_near_singular_vertex"]
        );
    }

    #[test]
    fn failure_requires_fallback() {
        let mut report = base_success_report();
        report.outcome = F64CapacityOutcome::Failure {
            reason: F64FailureReason::NoAdmissibleF64Orbit,
        };
        let classification = classify_report(&report, Some(2.0), None, None);
        assert_eq!(classification.agreement_status.label(), "no_f64_capacity");
        assert_eq!(classification.trust_class.label(), "fallback_required");
    }

    fn base_success_report() -> F64CapacityReport {
        F64CapacityReport {
            outcome: F64CapacityOutcome::Success {
                capacity: 2.0,
                sigma: vec![0, 1],
            },
            iterations: 1,
            admissible_f64_count: 1,
            indeterminate_f64_count: 0,
            inadmissible_count: 0,
            numerical_failure_count: 0,
            vertex_count: 5,
            facets_with_definite_vertex_count: 5,
            facets_with_possible_vertex_count: 5,
            vertex_indeterminate_count: 0,
            near_singular_vertex_count: 0,
            bounded_near_singular_vertex_count: 0,
            ambiguous_vertex_incidence_count: 0,
            facet_intersection_true_count: 0,
            facet_intersection_false_count: 0,
            facet_intersection_indeterminate_count: 0,
            omega_indeterminate_count: 0,
            min_action_gap: Some(1.0),
            indeterminate_overlaps_best_interval: false,
        }
    }
}

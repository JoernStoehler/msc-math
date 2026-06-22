use serde::{Deserialize, Serialize};

use crate::{F64CapacityOutcome, F64CapacityReport, MINIMIZING_SIGMA_SET_ACTION_TOLERANCE};

pub const ABS_ACTION_TOLERANCE: f64 = 1e-10;
pub const REL_ACTION_TOLERANCE: f64 = 1e-10;
pub const TINY_GAP_THRESHOLD: f64 = MINIMIZING_SIGMA_SET_ACTION_TOLERANCE;

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
    pub output_epistemics: OutputEpistemics,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum OutputDecisionStatus {
    Decided,
    Undecided,
    NotComputed,
    NotRecorded,
}

impl OutputDecisionStatus {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Decided => "decided",
            Self::Undecided => "undecided",
            Self::NotComputed => "not_computed",
            Self::NotRecorded => "not_recorded",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CapacityLabelStatus {
    LabelAgrees,
    LabelDisagrees,
    NoLabel,
    NotComputed,
    NotRecorded,
}

impl CapacityLabelStatus {
    pub fn label(&self) -> &'static str {
        match self {
            Self::LabelAgrees => "label_agrees",
            Self::LabelDisagrees => "label_disagrees",
            Self::NoLabel => "no_label",
            Self::NotComputed => "not_computed",
            Self::NotRecorded => "not_recorded",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum LowActionListCompleteness {
    Complete,
    Ambiguous,
    NotComputed,
    NotRecorded,
}

impl LowActionListCompleteness {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Complete => "complete",
            Self::Ambiguous => "ambiguous",
            Self::NotComputed => "not_computed",
            Self::NotRecorded => "not_recorded",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum LowActionItemsStatus {
    Determinate,
    Indeterminate,
    NotComputed,
    NotRecorded,
}

impl LowActionItemsStatus {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Determinate => "determinate",
            Self::Indeterminate => "indeterminate",
            Self::NotComputed => "not_computed",
            Self::NotRecorded => "not_recorded",
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OutputEpistemics {
    pub capacity_value_status: OutputDecisionStatus,
    pub capacity_label_status: CapacityLabelStatus,
    pub minimizing_sigma_set_status: OutputDecisionStatus,
    pub low_action_list_completeness: LowActionListCompleteness,
    pub low_action_items_status: LowActionItemsStatus,
    pub fallback_recommended: bool,
    pub reasons: Vec<String>,
}

impl Default for OutputEpistemics {
    fn default() -> Self {
        Self {
            capacity_value_status: OutputDecisionStatus::NotRecorded,
            capacity_label_status: CapacityLabelStatus::NotRecorded,
            minimizing_sigma_set_status: OutputDecisionStatus::NotRecorded,
            low_action_list_completeness: LowActionListCompleteness::NotRecorded,
            low_action_items_status: LowActionItemsStatus::NotRecorded,
            fallback_recommended: true,
            reasons: vec!["output_epistemics:not_recorded".to_string()],
        }
    }
}

#[derive(Clone, Debug, Default)]
struct OutputSignals {
    capacity_undecided: bool,
    minimizing_sigma_set_undecided: bool,
    low_action_list_ambiguous: bool,
    low_action_items_indeterminate: bool,
    blocking_reasons: Vec<String>,
    degenerate_reasons: Vec<String>,
    benign_reasons: Vec<String>,
}

impl OutputSignals {
    fn from_report(report: &F64CapacityReport) -> Self {
        let mut signals = Self::default();
        if report.bounded_near_singular_vertex_count > 0 {
            signals.capacity_undecided = true;
            signals
                .blocking_reasons
                .push("output_capacity_undecided:bounded_near_singular_vertex".to_string());
        }
        if report.indeterminate_overlaps_best_interval {
            signals.capacity_undecided = true;
            signals.low_action_list_ambiguous = true;
            signals
                .blocking_reasons
                .push("output_capacity_undecided:indeterminate_overlaps_best_interval".to_string());
        }
        if report.numerical_failure_count > 0 {
            signals.capacity_undecided = true;
            signals.low_action_items_indeterminate = true;
            signals
                .blocking_reasons
                .push("output_capacity_undecided:numerical_failure".to_string());
        }
        if report.indeterminate_f64_count > 0 {
            signals.low_action_items_indeterminate = true;
            signals
                .degenerate_reasons
                .push("low_action_list_item_indeterminate:kkt_indeterminate".to_string());
        }
        if report.ambiguous_vertex_incidence_count > 0 {
            signals.low_action_list_ambiguous = true;
            signals
                .degenerate_reasons
                .push("low_action_list_completeness_ambiguous:vertex_incidence".to_string());
        }
        if report.facet_intersection_indeterminate_count > 0 {
            signals.low_action_list_ambiguous = true;
            signals
                .degenerate_reasons
                .push("low_action_list_completeness_ambiguous:facet_intersection".to_string());
        }
        if report.near_minimizing_sigma_count > 1 {
            signals.minimizing_sigma_set_undecided = true;
            signals.degenerate_reasons.push(
                "output_minimizing_sigma_set_undecided:multiple_near_minimizing_sigmas".to_string(),
            );
        }
        if report.vertex_indeterminate_count > 0 {
            signals
                .benign_reasons
                .push("benign_structural:vertex_indeterminate".to_string());
        }
        if report.near_singular_vertex_count > 0 {
            signals
                .benign_reasons
                .push("benign_structural:near_singular_vertex".to_string());
        }
        if report.omega_indeterminate_count > 0 {
            signals
                .benign_reasons
                .push("benign_structural:omega_indeterminate".to_string());
        }
        signals
    }

    fn has_blocking_reasons(&self) -> bool {
        !self.blocking_reasons.is_empty()
    }

    fn has_degenerate_reasons(&self) -> bool {
        !self.degenerate_reasons.is_empty()
    }

    fn reasons(&self) -> Vec<String> {
        let mut reasons = Vec::new();
        reasons.extend(self.blocking_reasons.clone());
        reasons.extend(self.degenerate_reasons.clone());
        reasons.extend(self.benign_reasons.clone());
        reasons
    }
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

    let output_signals = OutputSignals::from_report(report);
    reasons.extend(output_signals.reasons());

    let trust_class = if !matches!(report.outcome, F64CapacityOutcome::Success { .. })
        || matches!(
            agreement_status,
            AgreementStatus::Disagrees | AgreementStatus::NoF64Capacity
        )
        || output_signals.has_blocking_reasons()
    {
        TrustClass::FallbackRequired
    } else if matches!(
        agreement_status,
        AgreementStatus::Agrees | AgreementStatus::NoAuditLabel
    ) && output_signals.has_degenerate_reasons()
    {
        TrustClass::DegenerateValueAgrees
    } else if matches!(
        agreement_status,
        AgreementStatus::Agrees | AgreementStatus::NoAuditLabel
    ) {
        TrustClass::Clean
    } else {
        TrustClass::FallbackRequired
    };

    let output_epistemics =
        output_epistemics(report, &agreement_status, &output_signals, &trust_class);

    Classification {
        agreement_status,
        trust_class,
        trust_reasons: reasons,
        output_epistemics,
    }
}

pub fn output_epistemics_not_computed() -> OutputEpistemics {
    OutputEpistemics {
        capacity_value_status: OutputDecisionStatus::NotComputed,
        capacity_label_status: CapacityLabelStatus::NotComputed,
        minimizing_sigma_set_status: OutputDecisionStatus::NotComputed,
        low_action_list_completeness: LowActionListCompleteness::NotComputed,
        low_action_items_status: LowActionItemsStatus::NotComputed,
        fallback_recommended: true,
        reasons: vec!["capacity_value:not_computed".to_string()],
    }
}

fn output_epistemics(
    report: &F64CapacityReport,
    agreement_status: &AgreementStatus,
    output_signals: &OutputSignals,
    trust_class: &TrustClass,
) -> OutputEpistemics {
    let has_capacity = matches!(report.outcome, F64CapacityOutcome::Success { .. });
    let mut reasons = output_signals.reasons();
    match agreement_status {
        AgreementStatus::Disagrees => {
            reasons.push("capacity_label:audit_label_disagreement".to_string())
        }
        AgreementStatus::NoAuditLabel => reasons.push("capacity_label:no_label".to_string()),
        AgreementStatus::NoF64Capacity => reasons.push("capacity_value:not_computed".to_string()),
        AgreementStatus::Agrees => {}
    }

    OutputEpistemics {
        capacity_value_status: if !has_capacity {
            OutputDecisionStatus::NotComputed
        } else if output_signals.capacity_undecided {
            OutputDecisionStatus::Undecided
        } else {
            OutputDecisionStatus::Decided
        },
        capacity_label_status: match agreement_status {
            AgreementStatus::Agrees => CapacityLabelStatus::LabelAgrees,
            AgreementStatus::Disagrees => CapacityLabelStatus::LabelDisagrees,
            AgreementStatus::NoAuditLabel => CapacityLabelStatus::NoLabel,
            AgreementStatus::NoF64Capacity => CapacityLabelStatus::NotComputed,
        },
        minimizing_sigma_set_status: if !has_capacity {
            OutputDecisionStatus::NotComputed
        } else if output_signals.minimizing_sigma_set_undecided {
            OutputDecisionStatus::Undecided
        } else {
            OutputDecisionStatus::Decided
        },
        low_action_list_completeness: if !has_capacity {
            LowActionListCompleteness::NotComputed
        } else if output_signals.low_action_list_ambiguous {
            LowActionListCompleteness::Ambiguous
        } else {
            LowActionListCompleteness::Complete
        },
        low_action_items_status: if !has_capacity {
            LowActionItemsStatus::NotComputed
        } else if output_signals.low_action_items_indeterminate {
            LowActionItemsStatus::Indeterminate
        } else {
            LowActionItemsStatus::Determinate
        },
        fallback_recommended: matches!(trust_class, TrustClass::FallbackRequired),
        reasons,
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
        assert_eq!(
            classification.output_epistemics.capacity_value_status,
            OutputDecisionStatus::Decided
        );
        assert_eq!(
            classification.output_epistemics.capacity_label_status,
            CapacityLabelStatus::LabelAgrees
        );
        assert_eq!(
            classification.output_epistemics.minimizing_sigma_set_status,
            OutputDecisionStatus::Decided
        );
        assert_eq!(
            classification
                .output_epistemics
                .low_action_list_completeness,
            LowActionListCompleteness::Complete
        );
        assert_eq!(
            classification.output_epistemics.low_action_items_status,
            LowActionItemsStatus::Determinate
        );
        assert!(!classification.output_epistemics.fallback_recommended);
    }

    #[test]
    fn missing_output_epistemics_defaults_to_conservative_not_recorded() {
        let epistemics = OutputEpistemics::default();
        assert_eq!(
            epistemics.capacity_value_status,
            OutputDecisionStatus::NotRecorded
        );
        assert_eq!(
            epistemics.capacity_label_status,
            CapacityLabelStatus::NotRecorded
        );
        assert!(epistemics.fallback_recommended);
        assert_eq!(epistemics.reasons, vec!["output_epistemics:not_recorded"]);
    }

    #[test]
    fn structural_ambiguity_stays_clean_when_value_agrees() {
        let mut report = base_success_report();
        report.vertex_indeterminate_count = 3;
        report.near_singular_vertex_count = 1;
        report.omega_indeterminate_count = 2;
        let classification = classify_report(&report, Some(2.0), Some(0.0), Some(0.0));
        assert_eq!(classification.trust_class.label(), "clean");
        assert_eq!(
            classification.output_epistemics.capacity_value_status,
            OutputDecisionStatus::Decided
        );
        assert_eq!(
            classification
                .output_epistemics
                .low_action_list_completeness,
            LowActionListCompleteness::Complete
        );
        assert_eq!(
            classification.trust_reasons,
            vec![
                "benign_structural:vertex_indeterminate",
                "benign_structural:near_singular_vertex",
                "benign_structural:omega_indeterminate"
            ]
        );
    }

    #[test]
    fn kkt_indeterminate_without_best_overlap_is_degenerate() {
        let mut report = base_success_report();
        report.indeterminate_f64_count = 2;
        let classification = classify_report(&report, Some(2.0), Some(0.0), Some(0.0));
        assert_eq!(
            classification.trust_class.label(),
            "degenerate_value_agrees"
        );
        assert_eq!(
            classification.output_epistemics.capacity_value_status,
            OutputDecisionStatus::Decided
        );
        assert_eq!(
            classification.output_epistemics.low_action_items_status,
            LowActionItemsStatus::Indeterminate
        );
        assert_eq!(
            classification.trust_reasons,
            vec!["low_action_list_item_indeterminate:kkt_indeterminate"]
        );
    }

    #[test]
    fn indeterminate_overlap_requires_fallback_even_when_value_agrees() {
        let mut report = base_success_report();
        report.indeterminate_f64_count = 2;
        report.indeterminate_overlaps_best_interval = true;
        let classification = classify_report(&report, Some(2.0), Some(0.0), Some(0.0));
        assert_eq!(classification.trust_class.label(), "fallback_required");
        assert_eq!(
            classification.output_epistemics.capacity_value_status,
            OutputDecisionStatus::Undecided
        );
        assert_eq!(
            classification
                .output_epistemics
                .low_action_list_completeness,
            LowActionListCompleteness::Ambiguous
        );
        assert!(classification.output_epistemics.fallback_recommended);
        assert_eq!(
            classification.trust_reasons,
            vec![
                "output_capacity_undecided:indeterminate_overlaps_best_interval",
                "low_action_list_item_indeterminate:kkt_indeterminate"
            ]
        );
    }

    #[test]
    fn multiple_near_minimizers_mean_minimizing_sigma_set_not_decided() {
        let mut report = base_success_report();
        report.near_minimizing_sigma_count = 2;
        report.min_action_gap = Some(TINY_GAP_THRESHOLD);
        let classification = classify_report(&report, Some(2.0), Some(0.0), Some(0.0));
        assert_eq!(
            classification.trust_class.label(),
            "degenerate_value_agrees"
        );
        assert_eq!(
            classification.output_epistemics.minimizing_sigma_set_status,
            OutputDecisionStatus::Undecided
        );
        assert_eq!(
            classification.trust_reasons,
            vec!["output_minimizing_sigma_set_undecided:multiple_near_minimizing_sigmas"]
        );
    }

    #[test]
    fn unaudited_multiple_near_minimizers_are_not_clean() {
        let mut report = base_success_report();
        report.near_minimizing_sigma_count = 2;
        let classification = classify_report(&report, None, None, None);
        assert_eq!(classification.agreement_status.label(), "no_audit_label");
        assert_eq!(
            classification.trust_class.label(),
            "degenerate_value_agrees"
        );
        assert_eq!(
            classification.output_epistemics.capacity_label_status,
            CapacityLabelStatus::NoLabel
        );
        assert_eq!(
            classification.output_epistemics.minimizing_sigma_set_status,
            OutputDecisionStatus::Undecided
        );
    }

    #[test]
    fn bounded_near_singular_requires_fallback_even_when_value_agrees() {
        let mut report = base_success_report();
        report.bounded_near_singular_vertex_count = 1;
        let classification = classify_report(&report, Some(2.0), Some(0.0), Some(0.0));
        assert_eq!(classification.trust_class.label(), "fallback_required");
        assert_eq!(
            classification.trust_reasons,
            vec!["output_capacity_undecided:bounded_near_singular_vertex"]
        );
    }

    #[test]
    fn numerical_failure_requires_fallback_even_when_a_value_agrees() {
        let mut report = base_success_report();
        report.numerical_failure_count = 1;
        let classification = classify_report(&report, Some(2.0), Some(0.0), Some(0.0));
        assert_eq!(classification.trust_class.label(), "fallback_required");
        assert_eq!(
            classification.trust_reasons,
            vec!["output_capacity_undecided:numerical_failure"]
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
            sigma_count: 1,
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
            near_minimizing_sigma_count: 1,
            min_action_gap: Some(1.0),
            indeterminate_overlaps_best_interval: false,
        }
    }
}

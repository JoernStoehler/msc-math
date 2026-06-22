use exp_sys_landscape::{capacity_auto, SysLandscapePolytopeCache};
use nalgebra::Vector4;
use serde::{Deserialize, Serialize};
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::time::Instant;

#[derive(Clone, Debug)]
pub struct ExactAuditReport {
    pub status: ExactAuditStatus,
    pub time_ms: f64,
    pub reasons: Vec<String>,
    pub capacity_label: Option<f64>,
    pub sigma_label: Option<Vec<usize>>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ExactAuditStatus {
    NotRequested,
    ReferenceRouteCapacitySuccess,
    ReferenceRouteCapacityFailure,
    ExactValidationRejected,
    ExactAuditError,
}

impl ExactAuditStatus {
    pub fn label(&self) -> &'static str {
        match self {
            Self::NotRequested => "not_requested",
            Self::ReferenceRouteCapacitySuccess => "reference_route_capacity_success",
            Self::ReferenceRouteCapacityFailure => "reference_route_capacity_failure",
            Self::ExactValidationRejected => "exact_validation_rejected",
            Self::ExactAuditError => "exact_audit_error",
        }
    }
}

pub fn exact_audit_not_requested() -> ExactAuditReport {
    ExactAuditReport {
        status: ExactAuditStatus::NotRequested,
        time_ms: 0.0,
        reasons: Vec::new(),
        capacity_label: None,
        sigma_label: None,
    }
}

pub fn audit_generated_case_exact(dual_vertices: &[Vector4<f64>]) -> ExactAuditReport {
    let started = Instant::now();
    match catch_unwind(AssertUnwindSafe(|| {
        audit_generated_case_exact_impl(dual_vertices)
    })) {
        Ok(mut report) => {
            report.time_ms = started.elapsed().as_secs_f64() * 1000.0;
            report
        }
        Err(_) => ExactAuditReport {
            status: ExactAuditStatus::ExactAuditError,
            time_ms: started.elapsed().as_secs_f64() * 1000.0,
            reasons: vec!["panic".to_string()],
            capacity_label: None,
            sigma_label: None,
        },
    }
}

fn audit_generated_case_exact_impl(dual_vertices: &[Vector4<f64>]) -> ExactAuditReport {
    let Some(cache) = SysLandscapePolytopeCache::from_f64_dual_vertices(dual_vertices.to_vec())
    else {
        return ExactAuditReport {
            status: ExactAuditStatus::ExactValidationRejected,
            time_ms: 0.0,
            reasons: vec!["exact_validation_rejected".to_string()],
            capacity_label: None,
            sigma_label: None,
        };
    };

    match capacity_auto(
        &cache.dual_vertices_f64,
        &cache.dual_vertices,
        &cache.facet_intersection_is_nonempty,
        &cache.omega_signs,
    ) {
        Ok(result) => ExactAuditReport {
            status: ExactAuditStatus::ReferenceRouteCapacitySuccess,
            time_ms: 0.0,
            reasons: Vec::new(),
            capacity_label: Some(result.min_action),
            sigma_label: Some(result.best_sigma().to_vec()),
        },
        Err(err) => ExactAuditReport {
            status: ExactAuditStatus::ReferenceRouteCapacityFailure,
            time_ms: 0.0,
            reasons: vec![format!("{err:?}")],
            capacity_label: None,
            sigma_label: None,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn audit_success_on_simplex_like_input() {
        let first = Vector4::new(1.0, 0.2, 0.3, 0.4);
        let second = Vector4::new(0.1, 1.0, 0.5, -0.2);
        let third = Vector4::new(-0.3, 0.4, 1.0, 0.6);
        let fourth = Vector4::new(0.2, -0.5, 0.4, 1.0);
        let dual_vertices = vec![
            first,
            second,
            third,
            fourth,
            -(first + second + third + fourth),
        ];
        let report = audit_generated_case_exact(&dual_vertices);
        assert_eq!(
            report.status,
            ExactAuditStatus::ReferenceRouteCapacitySuccess
        );
        assert!(report.capacity_label.is_some());
        assert!(report.sigma_label.is_some());
    }

    #[test]
    fn audit_reports_exact_validation_rejection() {
        let dual_vertices = vec![
            Vector4::new(1.0, 0.0, 0.0, 0.0),
            Vector4::new(1.0, 0.0, 0.0, 0.0),
            Vector4::new(0.0, 1.0, 0.0, 0.0),
            Vector4::new(0.0, 0.0, 1.0, 0.0),
            Vector4::new(0.0, 0.0, 0.0, 1.0),
        ];
        let report = audit_generated_case_exact(&dual_vertices);
        assert_eq!(report.status, ExactAuditStatus::ExactValidationRejected);
        assert!(report.capacity_label.is_none());
    }

    #[test]
    fn not_requested_has_no_labels() {
        let report = exact_audit_not_requested();
        assert_eq!(report.status, ExactAuditStatus::NotRequested);
        assert_eq!(report.time_ms, 0.0);
        assert!(report.capacity_label.is_none());
    }
}

use crate::geometry::{
    f64_combinatorics_profiled, f64_combinatorics_with_lp_transitions_profiled,
    F64CombinatoricsTiming,
};
use euclidean_polytopes::{origin_in_interior_of_conv_f64, OriginInteriorF64};
use good_lp::{constraint, default_solver, variable, variables, Expression, Solution, SolverModel};
use nalgebra::{DMatrix, Vector4};
use serde::{Deserialize, Serialize};
use std::time::Instant;

const EPS_ZERO_NORM: f64 = 1e-12;
const EPS_DUPLICATE_RELATIVE: f64 = 1e-10;
const EPS_LP_ORIGIN_MARGIN: f64 = 1e-10;
const EPS_LP_RESIDUAL: f64 = 1e-10;
const EPS_RANK: f64 = 1e-10;

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum F64ValidationPolicy {
    Strict,
    LpOriginVertex,
    Lp,
}

impl F64ValidationPolicy {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Strict => "strict",
            Self::LpOriginVertex => "lp_origin_vertex",
            Self::Lp => "lp",
        }
    }
}

#[derive(Clone, Debug)]
pub struct F64ValidationReport {
    pub status: F64ValidationStatus,
    pub reasons: Vec<String>,
    pub origin_status: F64PredicateStatus,
    pub origin_lp_status: String,
    pub origin_lp_max_min_lambda: Option<f64>,
    pub origin_lp_max_abs_residual: Option<f64>,
    pub facet_extremality_status: F64PredicateStatus,
    pub facet_count: usize,
    pub vertex_count: usize,
    pub facets_with_definite_vertex_count: usize,
    pub facets_with_possible_vertex_count: usize,
    pub facets_without_definite_vertex_count: usize,
    pub facets_without_possible_vertex_count: usize,
    pub vertex_indeterminate_count: usize,
    pub near_singular_vertex_count: usize,
    pub bounded_near_singular_vertex_count: usize,
    pub ambiguous_vertex_incidence_count: usize,
    pub facet_intersection_indeterminate_count: usize,
    pub omega_indeterminate_count: usize,
}

#[derive(Clone, Debug, Default)]
pub struct F64ValidationTimingBreakdown {
    pub sanity_ms: f64,
    pub origin_lp_diagnostic_ms: f64,
    pub origin_policy_predicate_ms: f64,
    pub combinatorics_ms: f64,
    pub classification_ms: f64,
    pub geometry: F64CombinatoricsTiming,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum F64ValidationStatus {
    AcceptedDecisive,
    AcceptedAmbiguous,
    Rejected,
    FallbackRequired,
}

impl F64ValidationStatus {
    pub fn label(&self) -> &'static str {
        match self {
            Self::AcceptedDecisive => "accepted_decisive",
            Self::AcceptedAmbiguous => "accepted_ambiguous",
            Self::Rejected => "rejected",
            Self::FallbackRequired => "fallback_required",
        }
    }

    pub fn capacity_may_run(&self) -> bool {
        matches!(self, Self::AcceptedDecisive | Self::AcceptedAmbiguous)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum F64PredicateStatus {
    True,
    False,
    Indeterminate,
}

impl F64PredicateStatus {
    pub fn label(&self) -> &'static str {
        match self {
            Self::True => "true",
            Self::False => "false",
            Self::Indeterminate => "indeterminate",
        }
    }
}

pub fn validate_f64_polytope_input(dual_vertices: &[Vector4<f64>]) -> F64ValidationReport {
    validate_f64_polytope_input_with_policy(dual_vertices, F64ValidationPolicy::LpOriginVertex)
}

pub fn validate_f64_polytope_input_with_policy(
    dual_vertices: &[Vector4<f64>],
    policy: F64ValidationPolicy,
) -> F64ValidationReport {
    validate_f64_polytope_input_with_policy_profiled(dual_vertices, policy).0
}

pub fn validate_f64_polytope_input_with_policy_profiled(
    dual_vertices: &[Vector4<f64>],
    policy: F64ValidationPolicy,
) -> (F64ValidationReport, F64ValidationTimingBreakdown) {
    let mut timing = F64ValidationTimingBreakdown::default();
    let started = Instant::now();
    let mut reasons = input_sanity_reasons(dual_vertices);
    timing.sanity_ms = started.elapsed().as_secs_f64() * 1000.0;
    if !reasons.is_empty() {
        return (
            empty_report(F64ValidationStatus::Rejected, reasons, dual_vertices.len()),
            timing,
        );
    }

    let started = Instant::now();
    let origin_lp = origin_lp_diagnostic(dual_vertices);
    timing.origin_lp_diagnostic_ms = started.elapsed().as_secs_f64() * 1000.0;
    let started = Instant::now();
    let origin_status = match policy {
        F64ValidationPolicy::Strict => match origin_in_interior_of_conv_f64(dual_vertices) {
            OriginInteriorF64::True => F64PredicateStatus::True,
            OriginInteriorF64::False => F64PredicateStatus::False,
            OriginInteriorF64::Indeterminate => F64PredicateStatus::Indeterminate,
        },
        F64ValidationPolicy::LpOriginVertex | F64ValidationPolicy::Lp => {
            origin_status_from_lp(dual_vertices, &origin_lp)
        }
    };
    timing.origin_policy_predicate_ms = started.elapsed().as_secs_f64() * 1000.0;
    match origin_status {
        F64PredicateStatus::False => reasons.push("origin_not_in_interior".to_string()),
        F64PredicateStatus::Indeterminate => {
            reasons.push("origin_interior_indeterminate".to_string())
        }
        F64PredicateStatus::True => {}
    }

    let started = Instant::now();
    let combinatorics_result = match policy {
        F64ValidationPolicy::Strict | F64ValidationPolicy::LpOriginVertex => {
            f64_combinatorics_profiled(dual_vertices)
        }
        F64ValidationPolicy::Lp => f64_combinatorics_with_lp_transitions_profiled(dual_vertices),
    };
    timing.combinatorics_ms = started.elapsed().as_secs_f64() * 1000.0;
    let Ok((combinatorics, geometry_timing)) = combinatorics_result else {
        reasons.push("combinatorics_failed".to_string());
        return (
            empty_report(
                F64ValidationStatus::FallbackRequired,
                reasons,
                dual_vertices.len(),
            ),
            timing,
        );
    };
    timing.geometry = geometry_timing;

    let started = Instant::now();
    let facet_extremality_status = facet_extremality_status(dual_vertices.len(), &combinatorics);
    match facet_extremality_status {
        F64PredicateStatus::False => reasons.push("facet_without_possible_vertex".to_string()),
        F64PredicateStatus::Indeterminate => {
            reasons.push("facet_extremality_indeterminate".to_string())
        }
        F64PredicateStatus::True => {}
    }
    if combinatorics.vertex_count == 0 {
        reasons.push("no_primal_vertices".to_string());
    }
    if combinatorics.vertex_indeterminate_count > 0 {
        reasons.push("vertex_indeterminate".to_string());
    }
    if combinatorics.bounded_near_singular_vertex_count > 0 {
        reasons.push("bounded_near_singular_vertex".to_string());
    }
    if combinatorics.facet_intersection_indeterminate_count > 0 {
        reasons.push("facet_intersection_indeterminate".to_string());
    }
    if combinatorics.omega_indeterminate_count > 0 {
        reasons.push("omega_indeterminate".to_string());
    }

    let status = validation_status(
        policy,
        &origin_status,
        &facet_extremality_status,
        combinatorics.vertex_count,
        &combinatorics,
    );
    timing.classification_ms = started.elapsed().as_secs_f64() * 1000.0;

    (
        F64ValidationReport {
            status,
            reasons,
            origin_status,
            origin_lp_status: origin_lp.status,
            origin_lp_max_min_lambda: origin_lp.max_min_lambda,
            origin_lp_max_abs_residual: origin_lp.max_abs_residual,
            facet_extremality_status,
            facet_count: dual_vertices.len(),
            vertex_count: combinatorics.vertex_count,
            facets_with_definite_vertex_count: combinatorics.facets_with_definite_vertex_count,
            facets_with_possible_vertex_count: combinatorics.facets_with_possible_vertex_count,
            facets_without_definite_vertex_count: dual_vertices
                .len()
                .saturating_sub(combinatorics.facets_with_definite_vertex_count),
            facets_without_possible_vertex_count: dual_vertices
                .len()
                .saturating_sub(combinatorics.facets_with_possible_vertex_count),
            vertex_indeterminate_count: combinatorics.vertex_indeterminate_count,
            near_singular_vertex_count: combinatorics.near_singular_vertex_count,
            bounded_near_singular_vertex_count: combinatorics.bounded_near_singular_vertex_count,
            ambiguous_vertex_incidence_count: combinatorics.ambiguous_vertex_incidence_count,
            facet_intersection_indeterminate_count: combinatorics
                .facet_intersection_indeterminate_count,
            omega_indeterminate_count: combinatorics.omega_indeterminate_count,
        },
        timing,
    )
}

#[derive(Clone, Debug)]
struct OriginLpDiagnostic {
    status: String,
    max_min_lambda: Option<f64>,
    max_abs_residual: Option<f64>,
}

fn origin_lp_diagnostic(dual_vertices: &[Vector4<f64>]) -> OriginLpDiagnostic {
    if dual_vertices.len() < 5
        || dual_vertices
            .iter()
            .any(|v| !v.iter().all(|value| value.is_finite()))
    {
        return OriginLpDiagnostic {
            status: "not_run_invalid_input".to_string(),
            max_min_lambda: None,
            max_abs_residual: None,
        };
    }

    let mut vars = variables!();
    let lambdas = (0..dual_vertices.len())
        .map(|_| vars.add(variable().min(0.0)))
        .collect::<Vec<_>>();
    let tau = vars.add(variable());
    let mut model = vars.maximise(tau).using(default_solver);

    let sum_lambda = lambdas
        .iter()
        .fold(Expression::from(0.0), |acc, lambda| acc + *lambda);
    model = model.with(constraint!(sum_lambda == 1.0));

    for lambda in &lambdas {
        model = model.with(constraint!(*lambda >= tau));
    }
    for coordinate in 0..4 {
        let expr = lambdas
            .iter()
            .enumerate()
            .fold(Expression::from(0.0), |acc, (idx, lambda)| {
                acc + dual_vertices[idx][coordinate] * *lambda
            });
        model = model.with(constraint!(expr == 0.0));
    }

    let Ok(solution) = model.solve() else {
        return OriginLpDiagnostic {
            status: "lp_failed_or_infeasible".to_string(),
            max_min_lambda: None,
            max_abs_residual: None,
        };
    };

    let lambda_values = lambdas
        .iter()
        .map(|lambda| solution.value(*lambda))
        .collect::<Vec<_>>();
    let mut residual = Vector4::zeros();
    for (lambda, dual_vertex) in lambda_values.iter().zip(dual_vertices) {
        residual += dual_vertex * *lambda;
    }
    OriginLpDiagnostic {
        status: "solved".to_string(),
        max_min_lambda: Some(solution.value(tau)),
        max_abs_residual: Some(residual.iter().map(|value| value.abs()).fold(0.0, f64::max)),
    }
}

fn origin_status_from_lp(
    dual_vertices: &[Vector4<f64>],
    origin_lp: &OriginLpDiagnostic,
) -> F64PredicateStatus {
    let full_rank = full_linear_span_status(dual_vertices);
    if full_rank == F64PredicateStatus::False {
        return F64PredicateStatus::False;
    }
    if origin_lp.status != "solved" {
        return F64PredicateStatus::Indeterminate;
    }
    let Some(tau) = origin_lp.max_min_lambda else {
        return F64PredicateStatus::Indeterminate;
    };
    let Some(residual) = origin_lp.max_abs_residual else {
        return F64PredicateStatus::Indeterminate;
    };
    if tau > EPS_LP_ORIGIN_MARGIN
        && residual <= EPS_LP_RESIDUAL
        && full_rank == F64PredicateStatus::True
    {
        F64PredicateStatus::True
    } else if tau < -EPS_LP_ORIGIN_MARGIN || residual > EPS_LP_RESIDUAL {
        F64PredicateStatus::False
    } else {
        F64PredicateStatus::Indeterminate
    }
}

fn full_linear_span_status(dual_vertices: &[Vector4<f64>]) -> F64PredicateStatus {
    let matrix = DMatrix::from_fn(4, dual_vertices.len(), |row, col| dual_vertices[col][row]);
    let singular_values = matrix.svd(false, false).singular_values;
    let small = singular_values
        .iter()
        .filter(|value| value.abs() <= EPS_RANK)
        .count();
    if singular_values.len() < 4 || small > 0 {
        F64PredicateStatus::False
    } else if singular_values
        .iter()
        .any(|value| value.abs() <= 10.0 * EPS_RANK)
    {
        F64PredicateStatus::Indeterminate
    } else {
        F64PredicateStatus::True
    }
}

fn input_sanity_reasons(dual_vertices: &[Vector4<f64>]) -> Vec<String> {
    let mut reasons = Vec::new();
    if dual_vertices.len() < 5 {
        reasons.push("too_few_dual_vertices".to_string());
    }
    for (idx, vertex) in dual_vertices.iter().enumerate() {
        if !vertex.iter().all(|value| value.is_finite()) {
            reasons.push(format!("nonfinite_coordinate:{idx}"));
        }
        if vertex.norm() < EPS_ZERO_NORM {
            reasons.push(format!("near_zero_dual_vertex:{idx}"));
        }
    }
    for i in 0..dual_vertices.len() {
        for j in i + 1..dual_vertices.len() {
            let max_norm = dual_vertices[i].norm().max(dual_vertices[j].norm());
            if (dual_vertices[i] - dual_vertices[j]).norm() < EPS_DUPLICATE_RELATIVE * max_norm {
                reasons.push(format!("near_duplicate_dual_vertices:{i}:{j}"));
            }
        }
    }
    reasons
}

fn facet_extremality_status(
    facet_count: usize,
    combinatorics: &crate::geometry::F64Combinatorics,
) -> F64PredicateStatus {
    if combinatorics.facets_with_definite_vertex_count == facet_count {
        F64PredicateStatus::True
    } else if combinatorics.facets_with_possible_vertex_count == facet_count {
        F64PredicateStatus::Indeterminate
    } else {
        F64PredicateStatus::False
    }
}

fn validation_status(
    policy: F64ValidationPolicy,
    origin_status: &F64PredicateStatus,
    facet_extremality_status: &F64PredicateStatus,
    vertex_count: usize,
    combinatorics: &crate::geometry::F64Combinatorics,
) -> F64ValidationStatus {
    if matches!(origin_status, F64PredicateStatus::False)
        || matches!(facet_extremality_status, F64PredicateStatus::False)
        || vertex_count == 0
    {
        return F64ValidationStatus::Rejected;
    }
    if matches!(origin_status, F64PredicateStatus::Indeterminate)
        || (policy == F64ValidationPolicy::Strict
            && combinatorics.bounded_near_singular_vertex_count > 0)
    {
        return F64ValidationStatus::FallbackRequired;
    }
    if matches!(facet_extremality_status, F64PredicateStatus::Indeterminate)
        || combinatorics.vertex_indeterminate_count > 0
        || combinatorics.facet_intersection_indeterminate_count > 0
        || combinatorics.omega_indeterminate_count > 0
    {
        return F64ValidationStatus::AcceptedAmbiguous;
    }
    F64ValidationStatus::AcceptedDecisive
}

fn empty_report(
    status: F64ValidationStatus,
    reasons: Vec<String>,
    facet_count: usize,
) -> F64ValidationReport {
    F64ValidationReport {
        status,
        reasons,
        origin_status: F64PredicateStatus::Indeterminate,
        origin_lp_status: "not_run".to_string(),
        origin_lp_max_min_lambda: None,
        origin_lp_max_abs_residual: None,
        facet_extremality_status: F64PredicateStatus::Indeterminate,
        facet_count,
        vertex_count: 0,
        facets_with_definite_vertex_count: 0,
        facets_with_possible_vertex_count: 0,
        facets_without_definite_vertex_count: facet_count,
        facets_without_possible_vertex_count: facet_count,
        vertex_indeterminate_count: 0,
        near_singular_vertex_count: 0,
        bounded_near_singular_vertex_count: 0,
        ambiguous_vertex_incidence_count: 0,
        facet_intersection_indeterminate_count: 0,
        omega_indeterminate_count: 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simplex_like_input_is_decisively_accepted() {
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
        let report = validate_f64_polytope_input(&dual_vertices);
        assert_eq!(report.status, F64ValidationStatus::AcceptedDecisive);
        assert_eq!(report.origin_status, F64PredicateStatus::True);
        assert_eq!(report.facet_extremality_status, F64PredicateStatus::True);
    }

    #[test]
    fn lp_policy_accepts_structural_product_origin_case() {
        let dual_vertices = vec![
            Vector4::new(-1.2214036892748639, -0.128410235348687, 0.0, 0.0),
            Vector4::new(0.8038785777125631, -1.0394029100481912, 0.0, 0.0),
            Vector4::new(1.106104528257497, 0.23181164091432865, 0.0, 0.0),
            Vector4::new(0.0, 0.0, -0.7037463173639409, -1.0848793918667465),
            Vector4::new(0.0, 0.0, 1.4619222451670222, 0.41685665805008276),
            Vector4::new(0.0, 0.0, -0.1238504743562827, 1.5876355795695363),
        ];

        let strict =
            validate_f64_polytope_input_with_policy(&dual_vertices, F64ValidationPolicy::Strict);
        assert_eq!(strict.status, F64ValidationStatus::FallbackRequired);
        assert!(strict
            .reasons
            .iter()
            .any(|reason| reason == "origin_interior_indeterminate"));

        let lp = validate_f64_polytope_input_with_policy(&dual_vertices, F64ValidationPolicy::Lp);
        assert_eq!(lp.origin_status, F64PredicateStatus::True);
        assert_eq!(lp.facet_extremality_status, F64PredicateStatus::True);
        assert_eq!(lp.status, F64ValidationStatus::AcceptedAmbiguous);
        assert!(lp.origin_lp_max_min_lambda.unwrap() > 0.04);
    }

    #[test]
    fn default_policy_uses_lp_origin_and_vertex_geometry() {
        let dual_vertices = vec![
            Vector4::new(-1.2214036892748639, -0.128410235348687, 0.0, 0.0),
            Vector4::new(0.8038785777125631, -1.0394029100481912, 0.0, 0.0),
            Vector4::new(1.106104528257497, 0.23181164091432865, 0.0, 0.0),
            Vector4::new(0.0, 0.0, -0.7037463173639409, -1.0848793918667465),
            Vector4::new(0.0, 0.0, 1.4619222451670222, 0.41685665805008276),
            Vector4::new(0.0, 0.0, -0.1238504743562827, 1.5876355795695363),
        ];

        let report = validate_f64_polytope_input(&dual_vertices);
        let explicit = validate_f64_polytope_input_with_policy(
            &dual_vertices,
            F64ValidationPolicy::LpOriginVertex,
        );
        assert_eq!(report.origin_status, explicit.origin_status);
        assert_eq!(
            report.facet_extremality_status,
            explicit.facet_extremality_status
        );
        assert_eq!(report.status, explicit.status);
        assert_eq!(report.status, F64ValidationStatus::AcceptedAmbiguous);
    }

    #[test]
    fn duplicate_dual_vertices_are_rejected() {
        let dual_vertices = vec![
            Vector4::new(1.0, 0.0, 0.0, 0.0),
            Vector4::new(1.0, 0.0, 0.0, 0.0),
            Vector4::new(0.0, 1.0, 0.0, 0.0),
            Vector4::new(0.0, 0.0, 1.0, 0.0),
            Vector4::new(0.0, 0.0, 0.0, 1.0),
        ];
        let report = validate_f64_polytope_input(&dual_vertices);
        assert_eq!(report.status, F64ValidationStatus::Rejected);
        assert!(report
            .reasons
            .iter()
            .any(|reason| reason.starts_with("near_duplicate_dual_vertices")));
    }

    #[test]
    fn missing_origin_interior_is_rejected() {
        let dual_vertices = vec![
            Vector4::new(1.0, 0.0, 0.0, 0.0),
            Vector4::new(0.0, 1.0, 0.0, 0.0),
            Vector4::new(0.0, 0.0, 1.0, 0.0),
            Vector4::new(0.0, 0.0, 0.0, 1.0),
            Vector4::new(1.0, 1.0, 1.0, 1.0),
        ];
        let strict =
            validate_f64_polytope_input_with_policy(&dual_vertices, F64ValidationPolicy::Strict);
        assert_eq!(strict.status, F64ValidationStatus::Rejected);
        assert!(strict
            .reasons
            .iter()
            .any(|reason| reason == "origin_not_in_interior"));

        let default = validate_f64_polytope_input(&dual_vertices);
        assert_eq!(default.status, F64ValidationStatus::FallbackRequired);
        assert!(default
            .reasons
            .iter()
            .any(|reason| reason == "origin_interior_indeterminate"));
    }
}

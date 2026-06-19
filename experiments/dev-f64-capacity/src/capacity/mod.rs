mod candidates;
mod reports;
mod types;

use crate::{
    geometry::{
        f64_combinatorics_profiled, f64_combinatorics_with_lp_transitions_profiled,
        F64Combinatorics, F64CombinatoricsTiming, F64Predicate,
    },
    F64ValidationPolicy,
};
use candidates::{solve_billiard_candidates_summary, solve_transition_pruned_candidates};
use nalgebra::{DMatrix, Vector4};
use reports::{empty_report, no_vertices_report};
use std::time::Instant;
use symplectic::classify_facets_from_dual_vertices;

pub use types::{F64CapacityMethod, F64CapacityOutcome, F64CapacityReport, F64FailureReason};

pub const MINIMIZING_SIGMA_SET_ACTION_TOLERANCE: f64 = 1e-8;

#[derive(Clone, Debug, Default)]
pub struct F64CapacityTimingBreakdown {
    pub combinatorics_ms: f64,
    pub transition_matrix_ms: f64,
    pub candidate_solve_ms: f64,
    pub candidate_kkt_solve_ms: f64,
    pub candidate_non_kkt_ms: f64,
    pub report_ms: f64,
    pub geometry: F64CombinatoricsTiming,
}

/// Measured pure-f64 capacity path.
///
/// This function accepts only rounded dual vertices. It builds f64
/// combinatorics, enumerates transition-pruned HK2017 candidates, and keeps
/// only `AdmissibleF64` candidates. It does not call exact arithmetic.
pub fn capacity_f64_only(dual_vertices: &[Vector4<f64>]) -> F64CapacityReport {
    capacity_f64_only_with_policy(dual_vertices, F64ValidationPolicy::LpOriginVertex)
}

pub fn capacity_f64_only_with_policy(
    dual_vertices: &[Vector4<f64>],
    policy: F64ValidationPolicy,
) -> F64CapacityReport {
    capacity_f64_only_with_policy_and_method_profiled(
        dual_vertices,
        policy,
        F64CapacityMethod::TransitionPrunedHk,
    )
    .0
}

pub fn capacity_f64_only_with_policy_profiled(
    dual_vertices: &[Vector4<f64>],
    policy: F64ValidationPolicy,
) -> (F64CapacityReport, F64CapacityTimingBreakdown) {
    capacity_f64_only_with_policy_and_method_profiled(
        dual_vertices,
        policy,
        F64CapacityMethod::TransitionPrunedHk,
    )
}

pub fn capacity_f64_only_with_policy_and_method_profiled(
    dual_vertices: &[Vector4<f64>],
    policy: F64ValidationPolicy,
    method: F64CapacityMethod,
) -> (F64CapacityReport, F64CapacityTimingBreakdown) {
    let mut timing = F64CapacityTimingBreakdown::default();
    let started = Instant::now();
    let combinatorics_result = match policy {
        F64ValidationPolicy::Strict | F64ValidationPolicy::LpOriginVertex => {
            f64_combinatorics_profiled(dual_vertices)
        }
        F64ValidationPolicy::Lp => f64_combinatorics_with_lp_transitions_profiled(dual_vertices),
    };
    timing.combinatorics_ms = started.elapsed().as_secs_f64() * 1000.0;
    let Ok((combinatorics, geometry_timing)) = combinatorics_result else {
        return (
            empty_report(F64CapacityOutcome::Failure {
                reason: F64FailureReason::InvalidInput,
            }),
            timing,
        );
    };
    timing.geometry = geometry_timing;
    if matches!(
        policy,
        F64ValidationPolicy::Strict | F64ValidationPolicy::LpOriginVertex
    ) && combinatorics.vertex_count == 0
    {
        return (no_vertices_report(combinatorics), timing);
    }

    let started = Instant::now();
    let transition_is_allowed = transition_matrix(&combinatorics, dual_vertices.len());
    timing.transition_matrix_ms = started.elapsed().as_secs_f64() * 1000.0;
    let started = Instant::now();
    let solved = match method {
        F64CapacityMethod::TransitionPrunedHk => {
            solve_transition_pruned_candidates(dual_vertices, &transition_is_allowed)
        }
        F64CapacityMethod::ProductBilliardOrHk => {
            solve_product_billiard_or_hk(dual_vertices, &combinatorics, &transition_is_allowed)
        }
    };
    timing.candidate_solve_ms = started.elapsed().as_secs_f64() * 1000.0;
    timing.candidate_kkt_solve_ms = solved.kkt_solve_ms;
    timing.candidate_non_kkt_ms =
        (timing.candidate_solve_ms - timing.candidate_kkt_solve_ms).max(0.0);
    let started = Instant::now();
    let outcome = match solved.best_admissible() {
        Some(best) => F64CapacityOutcome::Success {
            capacity: best.action,
            sigma: best.sigma.clone(),
        },
        None => F64CapacityOutcome::Failure {
            reason: F64FailureReason::NoAdmissibleF64Orbit,
        },
    };

    let report = F64CapacityReport {
        outcome,
        sigma_count: solved.sigma_count,
        admissible_f64_count: solved.admissible_count(),
        indeterminate_f64_count: solved.indeterminate_count(),
        inadmissible_count: solved.inadmissible_count,
        numerical_failure_count: solved.numerical_failure_count,
        vertex_count: combinatorics.vertex_count,
        facets_with_definite_vertex_count: combinatorics.facets_with_definite_vertex_count,
        facets_with_possible_vertex_count: combinatorics.facets_with_possible_vertex_count,
        vertex_indeterminate_count: combinatorics.vertex_indeterminate_count,
        near_singular_vertex_count: combinatorics.near_singular_vertex_count,
        bounded_near_singular_vertex_count: combinatorics.bounded_near_singular_vertex_count,
        ambiguous_vertex_incidence_count: combinatorics.ambiguous_vertex_incidence_count,
        facet_intersection_true_count: combinatorics.facet_intersection_true_count,
        facet_intersection_false_count: combinatorics.facet_intersection_false_count,
        facet_intersection_indeterminate_count: combinatorics
            .facet_intersection_indeterminate_count,
        omega_indeterminate_count: combinatorics.omega_indeterminate_count,
        near_minimizing_sigma_count: solved
            .near_minimizing_admissible_count(MINIMIZING_SIGMA_SET_ACTION_TOLERANCE),
        min_action_gap: solved.action_gap(),
        indeterminate_overlaps_best_interval: solved.indeterminate_overlaps_best_interval(),
    };
    timing.report_ms = started.elapsed().as_secs_f64() * 1000.0;
    (report, timing)
}

fn transition_matrix(combinatorics: &F64Combinatorics, facet_count: usize) -> DMatrix<bool> {
    DMatrix::from_fn(facet_count, facet_count, |i, j| {
        combinatorics.facet_intersections[(i, j)] != F64Predicate::False
            && combinatorics.omega_signs[(i, j)] >= 0
    })
}

fn solve_product_billiard_or_hk(
    dual_vertices: &[Vector4<f64>],
    combinatorics: &F64Combinatorics,
    transition_is_allowed: &DMatrix<bool>,
) -> candidates::CandidateSolveSummary {
    let Ok(classification) = classify_facets_from_dual_vertices(dual_vertices) else {
        return solve_transition_pruned_candidates(dual_vertices, transition_is_allowed);
    };
    let facet_intersection_is_nonempty =
        DMatrix::from_fn(dual_vertices.len(), dual_vertices.len(), |i, j| {
            combinatorics.facet_intersections[(i, j)] != F64Predicate::False
        });
    let billiard = solve_billiard_candidates_summary(
        dual_vertices,
        &classification.q_indices,
        &classification.p_indices,
        &facet_intersection_is_nonempty,
        transition_is_allowed,
    );
    if billiard.admissible_count() > 0 || billiard.indeterminate_count() > 0 {
        billiard
    } else {
        solve_transition_pruned_candidates(dual_vertices, transition_is_allowed)
    }
}

#[cfg(test)]
mod tests;

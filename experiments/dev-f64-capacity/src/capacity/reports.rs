use crate::geometry::F64Combinatorics;

use super::{F64CapacityOutcome, F64CapacityReport, F64FailureReason};

pub(crate) fn no_vertices_report(combinatorics: F64Combinatorics) -> F64CapacityReport {
    F64CapacityReport {
        outcome: F64CapacityOutcome::Failure {
            reason: F64FailureReason::NoVertices,
        },
        sigma_count: 0,
        admissible_f64_count: 0,
        indeterminate_f64_count: 0,
        inadmissible_count: 0,
        numerical_failure_count: 0,
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
        near_minimizing_sigma_count: 0,
        min_action_gap: None,
        indeterminate_overlaps_best_interval: false,
    }
}

pub(crate) fn empty_report(outcome: F64CapacityOutcome) -> F64CapacityReport {
    F64CapacityReport {
        outcome,
        sigma_count: 0,
        admissible_f64_count: 0,
        indeterminate_f64_count: 0,
        inadmissible_count: 0,
        numerical_failure_count: 0,
        vertex_count: 0,
        facets_with_definite_vertex_count: 0,
        facets_with_possible_vertex_count: 0,
        vertex_indeterminate_count: 0,
        near_singular_vertex_count: 0,
        bounded_near_singular_vertex_count: 0,
        ambiguous_vertex_incidence_count: 0,
        facet_intersection_true_count: 0,
        facet_intersection_false_count: 0,
        facet_intersection_indeterminate_count: 0,
        omega_indeterminate_count: 0,
        near_minimizing_sigma_count: 0,
        min_action_gap: None,
        indeterminate_overlaps_best_interval: false,
    }
}

//! Shared helpers for sys-landscape experiments.
//!
//! Experiments studying the systolic ratio as a global function on polytope
//! space: random-sample, random-product-sample, gradient-ascent-general,
//! gradient-ascent-products, and rejection-calibration.

use nalgebra::{DMatrix, Vector4};
use num_rational::BigRational;
use symplectic::{
    aggregate_orbits_with_dual_vertices_exact, classify_facets_from_dual_vertices,
    solve_billiard_candidates, solve_pruned_hk2017_candidates, BilliardError, OrbitGuaranteeMode,
    OrbitSearchError, OrbitSearchResult,
};

pub mod ascent;
pub mod datascience_cache;
pub mod datasets;
pub mod reference;
pub mod step_bound;
pub mod sys_landscape_cache;

pub use ascent::{
    apply_dual_step, apply_dual_step_with_cached_computation, apply_dual_step_with_computation,
    ascent_direction, ascent_events_path_for, cache_path_for, compute_active_sys_state,
    compute_active_sys_state_cached, compute_capacity_result, compute_sys, compute_sys_computation,
    compute_sys_computation_cached, compute_sys_from_capacity, computed_polytopes_path_for,
    dual_vertices_rational_strings, expensive_computations_cache_path_for, finalize_ascent_output,
    load_completed_names, open_ascent_writers, orbit_scalars_from_result, parse_ascent_args,
    polytope_key, rational_vec4_to_strings, run_parallel_seeds, smoke_output_path, trace_path_for,
    write_expensive_computation_cache_rows, write_seed_result, ActiveSysState, AscentArgs,
    AscentEventRow, AscentMode, AscentOutputPaths, AscentWriters, ComputedPolytopeMeta,
    ComputedPolytopeRecorder, ComputedPolytopeRow, ExpensiveComputationCache,
    ExpensiveComputationCacheRow, ExpensiveComputationCacheStats, SeedResult, SummaryRow,
    SysComputation, TraceRow,
};
pub use datascience_cache::{
    f64_dual_vertices, poly_id, poly_id_from_dual_vertices, CapacityBackend, ComputeCacheStats,
    ComputedPolytopeCache, ComputedPolytopePayloadRow,
};
pub use datasets::{
    continuation_cache_path, experiment_path, package_root, raw_dataset_cache_path,
    raw_dataset_path, raw_dataset_trace_path, raw_root, shared_family_cache_path,
    CONTINUATION_EXPERIMENT_DIR, GRADIENT_ASCENT_GENERAL_DIR,
};
pub use step_bound::{
    compute_step_bound, compute_step_bound_detailed, BoundaryEvent, EventType, MAX_STEP_SIZE,
};
pub use sys_landscape_cache::{exact_binary64_geometry_from_cache, SysLandscapePolytopeCache};

// Compatibility for frozen source snapshots; live callers use `reference`.
#[doc(hidden)]
#[deprecated(
    note = "reference-only helper; use reference::exact_volume_as_f64, or volume_from_incidence_f64 for ordinary computation"
)]
pub fn exact_volume_from_incidence_as_f64(
    vertices: &[[BigRational; 4]],
    incidence: &DMatrix<bool>,
) -> f64 {
    reference::exact_volume_as_f64(vertices, incidence)
}

pub fn capacity_pruned_hk2017(
    dual_vertices_f64: &[Vector4<f64>],
    dual_vertices: &[[BigRational; 4]],
    facet_intersection_is_nonempty: &DMatrix<bool>,
    omega_signs: &DMatrix<i8>,
) -> Result<OrbitSearchResult, OrbitSearchError> {
    let transition_is_allowed =
        symplectic::algorithms::facet_adjacency::build_transition_matrix_from_facet_intersections_and_omega(
            facet_intersection_is_nonempty,
            omega_signs,
        );
    let (orbits, iterations) =
        solve_pruned_hk2017_candidates(dual_vertices_f64, &transition_is_allowed)?;
    aggregate_orbits_with_dual_vertices_exact(
        dual_vertices,
        orbits,
        iterations,
        0.0,
        OrbitGuaranteeMode::MinimaSafe,
    )
}

pub fn capacity_billiard(
    dual_vertices_f64: &[Vector4<f64>],
    dual_vertices: &[[BigRational; 4]],
    facet_intersection_is_nonempty: &DMatrix<bool>,
    omega_signs: &DMatrix<i8>,
) -> Result<OrbitSearchResult, BilliardError> {
    let classification = classify_facets_from_dual_vertices(dual_vertices_f64)?;
    let transition_is_allowed =
        symplectic::algorithms::facet_adjacency::build_transition_matrix_from_facet_intersections_and_omega(
            facet_intersection_is_nonempty,
            omega_signs,
        );
    let (orbits, iterations) = solve_billiard_candidates(
        dual_vertices_f64,
        &classification.q_indices,
        &classification.p_indices,
        facet_intersection_is_nonempty,
        &transition_is_allowed,
    )
    .map_err(BilliardError::OrbitSearch)?;
    aggregate_orbits_with_dual_vertices_exact(
        dual_vertices,
        orbits,
        iterations,
        0.0,
        OrbitGuaranteeMode::MinimaSafe,
    )
    .map_err(BilliardError::OrbitSearch)
}

pub fn capacity_auto(
    dual_vertices_f64: &[Vector4<f64>],
    dual_vertices: &[[BigRational; 4]],
    facet_intersection_is_nonempty: &DMatrix<bool>,
    omega_signs: &DMatrix<i8>,
) -> Result<OrbitSearchResult, OrbitSearchError> {
    if classify_facets_from_dual_vertices(dual_vertices_f64).is_ok() {
        return capacity_billiard(
            dual_vertices_f64,
            dual_vertices,
            facet_intersection_is_nonempty,
            omega_signs,
        )
        .map_err(|err| match err {
            BilliardError::OrbitSearch(err) => err,
            BilliardError::NotLagrangianProduct { .. } | BilliardError::TooFewFacets { .. } => {
                unreachable!("classification was checked immediately before billiard routing")
            }
        });
    }

    capacity_pruned_hk2017(
        dual_vertices_f64,
        dual_vertices,
        facet_intersection_is_nonempty,
        omega_signs,
    )
}

/// Retained orbit-producing frontend for branch-sensitive experiments.
///
/// Product classification selects billiard candidates; classification failure
/// selects pruned HK candidates. The supplied action window is clamped to zero
/// for negative and NaN values before `AllSafe` aggregation. This legacy
/// experiment contract is distinct from the production scalar `capacity_4d`
/// API and does not certify candidate completeness.
pub fn capacity_auto_with_gap(
    polytope: &SysLandscapePolytopeCache,
    action_gap: f64,
) -> Result<OrbitSearchResult, OrbitSearchError> {
    let action_gap = action_gap.max(0.0);
    let transition_is_allowed =
        symplectic::algorithms::facet_adjacency::build_transition_matrix_from_facet_intersections_and_omega(
            &polytope.facet_intersection_is_nonempty,
            &polytope.omega_signs,
        );

    let (orbits, iterations) = if let Ok(classification) =
        classify_facets_from_dual_vertices(&polytope.dual_vertices_f64)
    {
        solve_billiard_candidates(
            &polytope.dual_vertices_f64,
            &classification.q_indices,
            &classification.p_indices,
            &polytope.facet_intersection_is_nonempty,
            &transition_is_allowed,
        )?
    } else {
        solve_pruned_hk2017_candidates(&polytope.dual_vertices_f64, &transition_is_allowed)?
    };

    aggregate_orbits_with_dual_vertices_exact(
        &polytope.dual_vertices,
        orbits,
        iterations,
        action_gap,
        OrbitGuaranteeMode::AllSafe,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use symplectic::known_polytopes::{self, KnownPolytope};

    fn cache(p: &KnownPolytope) -> SysLandscapePolytopeCache {
        SysLandscapePolytopeCache {
            dual_vertices: p.dual_vertices.clone(),
            vertices: p.vertices.clone(),
            vertex_facet_incidence: p.vertex_facet_incidence.clone(),
            facet_intersection_is_nonempty: p.facet_intersection_is_nonempty.clone(),
            omega_signs: p.omega_signs.clone(),
            dual_vertices_f64: p.dual_vertices_f64.clone(),
            vertices_f64: p.vertices_f64.clone(),
        }
    }

    fn explicit_allsafe_composition(
        polytope: &SysLandscapePolytopeCache,
        action_gap: f64,
    ) -> Result<OrbitSearchResult, OrbitSearchError> {
        let transitions =
            symplectic::algorithms::facet_adjacency::build_transition_matrix_from_facet_intersections_and_omega(
                &polytope.facet_intersection_is_nonempty,
                &polytope.omega_signs,
            );
        let (orbits, iterations) = if let Ok(classification) =
            classify_facets_from_dual_vertices(&polytope.dual_vertices_f64)
        {
            solve_billiard_candidates(
                &polytope.dual_vertices_f64,
                &classification.q_indices,
                &classification.p_indices,
                &polytope.facet_intersection_is_nonempty,
                &transitions,
            )?
        } else {
            solve_pruned_hk2017_candidates(&polytope.dual_vertices_f64, &transitions)?
        };
        aggregate_orbits_with_dual_vertices_exact(
            &polytope.dual_vertices,
            orbits,
            iterations,
            action_gap,
            OrbitGuaranteeMode::AllSafe,
        )
    }

    #[test]
    fn action_window_nonproduct_uses_pruned_hk_and_allsafe_aggregation() {
        let polytope = cache(known_polytopes::simplex());
        assert!(classify_facets_from_dual_vertices(&polytope.dual_vertices_f64).is_err());
        let expected = explicit_allsafe_composition(&polytope, 0.25).unwrap();
        assert_eq!(capacity_auto_with_gap(&polytope, 0.25).unwrap(), expected);
    }

    #[test]
    fn action_window_product_uses_billiards_and_allsafe_aggregation() {
        let polytope = cache(known_polytopes::lagrangian_triangle_product());
        assert!(classify_facets_from_dual_vertices(&polytope.dual_vertices_f64).is_ok());
        let expected = explicit_allsafe_composition(&polytope, 0.25).unwrap();
        assert_eq!(capacity_auto_with_gap(&polytope, 0.25).unwrap(), expected);
    }

    #[test]
    fn action_window_clamps_negative_infinity_and_nan_to_zero() {
        for fixture in [
            known_polytopes::simplex(),
            known_polytopes::lagrangian_triangle_product(),
        ] {
            let polytope = cache(fixture);
            let zero = capacity_auto_with_gap(&polytope, 0.0).unwrap();
            for gap in [-1.0, f64::NEG_INFINITY, f64::NAN] {
                assert_eq!(capacity_auto_with_gap(&polytope, gap).unwrap(), zero);
            }
        }
    }

    #[test]
    fn action_window_propagates_empty_candidate_errors_on_both_routes() {
        for fixture in [
            known_polytopes::simplex(),
            known_polytopes::lagrangian_triangle_product(),
        ] {
            let mut polytope = cache(fixture);
            polytope.facet_intersection_is_nonempty = DMatrix::from_element(
                polytope.dual_vertices.len(),
                polytope.dual_vertices.len(),
                false,
            );
            assert!(matches!(
                capacity_auto_with_gap(&polytope, 0.25),
                Err(OrbitSearchError::NoAdmissibleOrbit)
            ));
        }
    }
}

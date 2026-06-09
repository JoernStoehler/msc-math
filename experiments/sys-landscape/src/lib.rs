//! Shared helpers for sys-landscape experiments.
//!
//! Experiments studying the systolic ratio as a global function on polytope
//! space: random-sample, random-product-sample, gradient-ascent-general,
//! gradient-ascent-products, and rejection-calibration.

use euclidean_polytopes::volume_from_incidence_exact;
use nalgebra::{DMatrix, Vector4};
use num_rational::BigRational;
use num_traits::ToPrimitive;
use symplectic::{
    aggregate_orbits_with_dual_vertices_exact, classify_facets_from_dual_vertices,
    solve_billiard_candidates, solve_pruned_hk2017_candidates, BilliardError, OrbitGuaranteeMode,
    OrbitSearchError, OrbitSearchResult,
};

pub mod ascent;
pub mod datasets;
pub mod step_bound;
pub mod sys_landscape_cache;

pub use ascent::{
    apply_dual_step, apply_dual_step_with_cached_computation, apply_dual_step_with_computation,
    ascent_direction, ascent_events_path_for, cache_path_for, compute_active_sys_state,
    compute_active_sys_state_cached, compute_capacity_result, compute_sys, compute_sys_computation,
    compute_sys_computation_cached, compute_sys_from_capacity, computed_polytopes_path_for,
    dual_vertices_rational_strings, expensive_computations_cache_path_for, finalize_ascent_output,
    load_completed_names, open_ascent_writers, orbit_scalars_from_result, parse_ascent_args,
    rational_vec4_to_strings, run_parallel_seeds, smoke_output_path, trace_path_for,
    write_expensive_computation_cache_rows, write_seed_result, ActiveSysState, AscentArgs,
    AscentEventRow, AscentMode, AscentOutputPaths, AscentWriters, ComputedPolytopeMeta,
    ComputedPolytopeRecorder, ComputedPolytopeRow, ExpensiveComputationCache,
    ExpensiveComputationCacheRow, ExpensiveComputationCacheStats, SeedResult, SummaryRow,
    SysComputation, TraceRow,
};
pub use datasets::{
    continuation_cache_path, experiment_path, package_root, raw_dataset_cache_path,
    raw_dataset_path, raw_dataset_trace_path, raw_root, shared_family_cache_path,
    CONTINUATION_EXPERIMENT_DIR, GRADIENT_ASCENT_GENERAL_DIR,
};
pub use step_bound::{
    compute_step_bound, compute_step_bound_detailed, BoundaryEvent, EventType, MAX_STEP_SIZE,
};
pub use sys_landscape_cache::SysLandscapePolytopeCache;

pub fn exact_volume_from_incidence_as_f64(
    vertices: &[[BigRational; 4]],
    incidence: &DMatrix<bool>,
) -> f64 {
    let vertices: Vec<Vector4<BigRational>> = vertices
        .iter()
        .map(|v| Vector4::new(v[0].clone(), v[1].clone(), v[2].clone(), v[3].clone()))
        .collect();
    ToPrimitive::to_f64(&volume_from_incidence_exact(&vertices, incidence)).unwrap_or(f64::NAN)
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

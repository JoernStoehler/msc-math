//! Shared helpers for verification experiments.
//!
//! Purpose: keep target-pool selection and shared run plumbing consistent
//! across the minimum-set and orbit-recovery validation binaries.

pub mod io;
pub mod target_pool;
pub mod verification_cache;

use nalgebra::{DMatrix, Vector4};
use num_rational::BigRational;
use symplectic::{
    aggregate_orbits_with_dual_vertices_exact, classify_facets_from_dual_vertices,
    solve_billiard_candidates, solve_pruned_hk2017_candidates, solve_unpruned_hk2017_candidates,
    BilliardError, OrbitGuaranteeMode, OrbitSearchError, OrbitSearchResult,
};

pub use io::{
    create_jsonl_writer, mode_output_path, parse_run_mode, run_mode_label, write_json_line,
    RunMode, RunModeArgError,
};
pub use target_pool::{
    build_target_pool, target_map, Target, ACTION_TOL, EXCLUDED_KNOWN_NAMES, GEOMETRY_TOL,
    MINIMUM_ACTION_GAP_TOL, SCALAR_TOL, SMOKE_TARGET_NAMES,
};
pub use verification_cache::VerificationPolytopeCache;

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

pub fn capacity_unpruned_hk2017(
    dual_vertices_f64: &[Vector4<f64>],
    dual_vertices: &[[BigRational; 4]],
) -> Result<OrbitSearchResult, OrbitSearchError> {
    let (orbits, iterations) = solve_unpruned_hk2017_candidates(dual_vertices_f64)?;
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

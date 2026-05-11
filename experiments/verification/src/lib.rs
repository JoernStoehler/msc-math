//! Shared helpers for verification experiments.
//!
//! Purpose: keep target-pool selection and shared run plumbing consistent
//! across the minimum-set and orbit-recovery validation binaries.

pub mod io;
pub mod target_pool;

use symplectic::{
    aggregate_orbits_with_dual_vertices_exact, classify_facets_from_dual_vertices,
    solve_billiard_candidates, solve_pruned_hk2017_candidates, solve_unpruned_hk2017_candidates,
    BilliardError, OrbitGuaranteeMode, OrbitSearchError, OrbitSearchResult, Polytope4D,
};

pub use io::{
    create_jsonl_writer, mode_output_path, parse_run_mode, run_mode_label, write_json_line,
    RunMode, RunModeArgError,
};
pub use target_pool::{
    build_target_pool, target_map, Target, ACTION_TOL, EXCLUDED_KNOWN_NAMES, GEOMETRY_TOL,
    MINIMUM_ACTION_GAP_TOL, SCALAR_TOL, SMOKE_TARGET_NAMES,
};

pub fn capacity_pruned_hk2017(
    polytope: &Polytope4D,
) -> Result<OrbitSearchResult, OrbitSearchError> {
    let transition_is_allowed =
        symplectic::algorithms::facet_adjacency::build_transition_matrix_from_facet_intersections_and_omega(
            polytope.facet_intersection_is_nonempty(),
            polytope.omega_signs(),
        );
    let (orbits, iterations) =
        solve_pruned_hk2017_candidates(polytope.dual_vertices_f64(), &transition_is_allowed)?;
    aggregate_orbits_with_dual_vertices_exact(
        polytope.dual_vertices(),
        orbits,
        iterations,
        0.0,
        OrbitGuaranteeMode::MinimaSafe,
    )
}

pub fn capacity_unpruned_hk2017(
    polytope: &Polytope4D,
) -> Result<OrbitSearchResult, OrbitSearchError> {
    let (orbits, iterations) = solve_unpruned_hk2017_candidates(polytope.dual_vertices_f64())?;
    aggregate_orbits_with_dual_vertices_exact(
        polytope.dual_vertices(),
        orbits,
        iterations,
        0.0,
        OrbitGuaranteeMode::MinimaSafe,
    )
}

pub fn capacity_billiard(polytope: &Polytope4D) -> Result<OrbitSearchResult, BilliardError> {
    let classification = classify_facets_from_dual_vertices(polytope.dual_vertices_f64())?;
    let transition_is_allowed =
        symplectic::algorithms::facet_adjacency::build_transition_matrix_from_facet_intersections_and_omega(
            polytope.facet_intersection_is_nonempty(),
            polytope.omega_signs(),
        );
    let (orbits, iterations) = solve_billiard_candidates(
        polytope.dual_vertices_f64(),
        &classification.q_indices,
        &classification.p_indices,
        polytope.facet_intersection_is_nonempty(),
        &transition_is_allowed,
    )
    .map_err(BilliardError::OrbitSearch)?;
    aggregate_orbits_with_dual_vertices_exact(
        polytope.dual_vertices(),
        orbits,
        iterations,
        0.0,
        OrbitGuaranteeMode::MinimaSafe,
    )
    .map_err(BilliardError::OrbitSearch)
}

pub fn capacity_auto(polytope: &Polytope4D) -> Result<OrbitSearchResult, OrbitSearchError> {
    if classify_facets_from_dual_vertices(polytope.dual_vertices_f64()).is_ok() {
        return capacity_billiard(polytope).map_err(|err| match err {
            BilliardError::OrbitSearch(err) => err,
            BilliardError::NotLagrangianProduct { .. } | BilliardError::TooFewFacets { .. } => {
                unreachable!("classification was checked immediately before billiard routing")
            }
        });
    }

    capacity_pruned_hk2017(polytope)
}

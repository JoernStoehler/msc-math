//! Capacity wrappers for regular Lagrangian product sweeps.

use nalgebra::{DMatrix, Vector4};
use num_rational::BigRational;
use symplectic::{
    aggregate_orbits_with_dual_vertices_exact, classify_facets_from_dual_vertices,
    solve_billiard_candidates, BilliardError, OrbitGuaranteeMode, OrbitSearchResult,
};

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

//! Shared helpers for dev-numerical-analysis experiments.
//!
//! The algebraic exactness spike lives here so multiple numerics binaries and
//! tests can share the same experimental field, geometry, KKT, and catalog
//! helpers without touching the library core.

use euclidean_polytopes::volume_from_incidence_exact;
use nalgebra::{DMatrix, Vector4};
use num_rational::BigRational;
use num_traits::ToPrimitive;
use symplectic::{
    aggregate_orbits_with_dual_vertices_exact, classify_facets_from_dual_vertices,
    solve_billiard_candidates, solve_pruned_hk2017_candidates, solve_unpruned_hk2017_candidates,
    BilliardError, OrbitGuaranteeMode, OrbitSearchError, OrbitSearchResult, Polytope4D,
};

pub mod algebraic;

pub fn euclidean_volume_f64(vertices: &[[BigRational; 4]], incidence: &DMatrix<bool>) -> f64 {
    let vertices: Vec<Vector4<BigRational>> = vertices
        .iter()
        .map(|v| Vector4::new(v[0].clone(), v[1].clone(), v[2].clone(), v[3].clone()))
        .collect();
    ToPrimitive::to_f64(&volume_from_incidence_exact(&vertices, incidence)).unwrap_or(f64::NAN)
}

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

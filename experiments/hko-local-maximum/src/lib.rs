//! Shared helpers for exp-hko-local-maximum experiments.

use euclidean_polytopes::volume_from_incidence_exact;
use nalgebra::{DMatrix, Vector4};
use num_rational::BigRational;
use num_traits::ToPrimitive;
use symplectic::{
    aggregate_orbits_with_dual_vertices_exact, classify_facets_from_dual_vertices,
    solve_billiard_candidates, solve_pruned_hk2017_candidates, BilliardError, OrbitGuaranteeMode,
    OrbitSearchError, OrbitSearchResult,
};

pub mod exact_bank;
pub mod instrumented_search;

pub use exact_bank::{
    exact_hko_dual_vertices, exact_simplex_dual_vertices, ExactBankEntry, ExactBankTarget,
    HkoExactScalar, EXACT_BANK_ENTRIES, HKO_FLOAT_WINNING_SIGMA, HKO_NEAR_OPTIMAL_SIGMA_A,
    HKO_NEAR_OPTIMAL_SIGMA_B, HKO_RANK_DEFICIENT_SIGMA, HKO_WINNING_SIGMA, SIMPLEX_CONTROL_SIGMA,
};
pub use instrumented_search::{ehz_capacity_instrumented, InstrumentedOrbitSearch};

pub fn euclidean_volume_f64(vertices: &[[BigRational; 4]], incidence: &DMatrix<bool>) -> f64 {
    let vertices: Vec<Vector4<BigRational>> = vertices
        .iter()
        .map(|v| Vector4::new(v[0].clone(), v[1].clone(), v[2].clone(), v[3].clone()))
        .collect();
    ToPrimitive::to_f64(&volume_from_incidence_exact(&vertices, incidence)).unwrap_or(f64::NAN)
}

pub fn capacity_pruned_hk2017(
    dual_vertices: &[[BigRational; 4]],
    dual_vertices_f64: &[Vector4<f64>],
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
    dual_vertices: &[[BigRational; 4]],
    dual_vertices_f64: &[Vector4<f64>],
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
    dual_vertices: &[[BigRational; 4]],
    dual_vertices_f64: &[Vector4<f64>],
    facet_intersection_is_nonempty: &DMatrix<bool>,
    omega_signs: &DMatrix<i8>,
) -> Result<OrbitSearchResult, OrbitSearchError> {
    if classify_facets_from_dual_vertices(dual_vertices_f64).is_ok() {
        return capacity_billiard(
            dual_vertices,
            dual_vertices_f64,
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
        dual_vertices,
        dual_vertices_f64,
        facet_intersection_is_nonempty,
        omega_signs,
    )
}

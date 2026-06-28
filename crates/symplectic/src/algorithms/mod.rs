//! EHZ capacity algorithms for 4D convex polytopes.
//!
//! Active algorithms:
//! - `hk2017` — general capacity (exponential in #facets).
//! - `billiard` — Lagrangian product capacity (fast).
//!
//! The flow-graph algorithm work surface lives under `flow_graph/`. Its local
//! README is the current algorithm contract/status surface; the old tube notes
//! under `flow_graph/` are legacy/imported source material.
//!
//! # Correctness invariant
//!
//! Where domains overlap (notably hypercube and Lagrangian products, which
//! both `hk2017` and `billiard` accept), the algorithms must agree on the
//! computed capacity within numerical tolerance. Cross-algorithm agreement
//! tests live in `billiard::tests::agrees_with_hk2017_*`. This is the
//! primary external correctness check and the reason multiple algorithms
//! coexist rather than being consolidated.
//!
//! Shared utilities:
//! - `facet_adjacency` — facet-intersection and directed (omega_0-aware)
//!   transition matrices for permutation pruning.
//! - `orbit_search` — shared result-layer types for HK2017-family frontends.

pub mod billiard;
pub mod facet_adjacency;
pub mod flow_graph;
pub mod hk2017;
pub mod orbit_search;

pub use orbit_search::{
    aggregate_certified_orbits_with_dual_vertices_exact, aggregate_orbits_with_dual_vertices_exact,
    solve_orbit_sigma_saddle_point, CertifiedOrbitKktData, CertifiedOrbitSearchResult,
    CertifiedOrbitSetMode, GeometricOrbitError, OrbitAdmissibility, OrbitGuaranteeMode,
    OrbitKktData, OrbitSearchError, OrbitSearchResult, OrbitSolveError,
};

#[cfg(test)]
pub(crate) mod test_helpers {
    use super::{
        aggregate_certified_orbits_with_dual_vertices_exact,
        aggregate_orbits_with_dual_vertices_exact,
        billiard::facet_classification::classify_facets_from_dual_vertices,
        billiard::solve_billiard_candidates,
        facet_adjacency::build_transition_matrix_from_facet_intersections_and_omega,
        hk2017::{solve_pruned_hk2017_candidates, solve_unpruned_hk2017_candidates},
        CertifiedOrbitSearchResult, CertifiedOrbitSetMode, OrbitGuaranteeMode, OrbitSearchError,
        OrbitSearchResult,
    };
    use crate::exact::{
        exact_vertices_with_incidence, facet_intersection_is_nonempty_exact, omega_signs_exact,
    };
    use crate::geom::{known_polytopes::KnownPolytope, rational_arithmetic::f64_to_rational};
    use nalgebra::{DMatrix, Vector4};
    use num_rational::BigRational;

    pub(crate) fn exact_dual_vertex_arrays(
        dual_vertices: &[Vector4<f64>],
    ) -> Vec<[BigRational; 4]> {
        dual_vertices
            .iter()
            .map(|a| {
                [
                    f64_to_rational(a[0]),
                    f64_to_rational(a[1]),
                    f64_to_rational(a[2]),
                    f64_to_rational(a[3]),
                ]
            })
            .collect()
    }

    fn exact_dual_vertex_vectors(dual_vertices: &[Vector4<f64>]) -> Vec<Vector4<BigRational>> {
        dual_vertices
            .iter()
            .map(|a| {
                Vector4::new(
                    f64_to_rational(a[0]),
                    f64_to_rational(a[1]),
                    f64_to_rational(a[2]),
                    f64_to_rational(a[3]),
                )
            })
            .collect()
    }

    pub(crate) fn flat_facet_data_from_dual_vertices(
        dual_vertices: &[Vector4<f64>],
    ) -> (Vec<[BigRational; 4]>, DMatrix<bool>, DMatrix<i8>) {
        let dual_vertices_exact = exact_dual_vertex_arrays(dual_vertices);
        let dual_vertices_exact_vectors = exact_dual_vertex_vectors(dual_vertices);
        let vertices_with_incidence = exact_vertices_with_incidence(&dual_vertices_exact_vectors)
            .expect("dual vertices must define a valid 4D polytope");
        let facet_intersection_is_nonempty =
            facet_intersection_is_nonempty_exact(&vertices_with_incidence.vertex_facet_incidence);
        let omega_signs = omega_signs_exact(&dual_vertices_exact_vectors);
        (
            dual_vertices_exact,
            facet_intersection_is_nonempty,
            omega_signs,
        )
    }

    pub(crate) fn pruned_capacity(
        dual_vertices: &[Vector4<f64>],
        dual_vertices_exact: &[[BigRational; 4]],
        facet_intersection_is_nonempty: &DMatrix<bool>,
        omega_signs: &DMatrix<i8>,
    ) -> Result<OrbitSearchResult, OrbitSearchError> {
        let transition_is_allowed = build_transition_matrix_from_facet_intersections_and_omega(
            facet_intersection_is_nonempty,
            omega_signs,
        );
        let (orbits, iterations) =
            solve_pruned_hk2017_candidates(dual_vertices, &transition_is_allowed)?;
        aggregate_orbits_with_dual_vertices_exact(
            dual_vertices_exact,
            orbits,
            iterations,
            0.0,
            OrbitGuaranteeMode::MinimaSafe,
        )
    }

    pub(crate) fn pruned_capacity_for_fixture(
        fixture: &KnownPolytope,
    ) -> Result<OrbitSearchResult, OrbitSearchError> {
        pruned_capacity(
            &fixture.dual_vertices_f64,
            &fixture.dual_vertices,
            &fixture.facet_intersection_is_nonempty,
            &fixture.omega_signs,
        )
    }

    pub(crate) fn pruned_capacity_for_dual_vertices(
        dual_vertices: &[Vector4<f64>],
    ) -> Result<OrbitSearchResult, OrbitSearchError> {
        let (dual_vertices_exact, facet_intersection_is_nonempty, omega_signs) =
            flat_facet_data_from_dual_vertices(dual_vertices);
        pruned_capacity(
            dual_vertices,
            &dual_vertices_exact,
            &facet_intersection_is_nonempty,
            &omega_signs,
        )
    }

    pub(crate) fn certified_pruned_capacity_for_fixture(
        fixture: &KnownPolytope,
        action_gap_exact: BigRational,
        mode: CertifiedOrbitSetMode,
    ) -> Result<CertifiedOrbitSearchResult, OrbitSearchError> {
        let transition_is_allowed = build_transition_matrix_from_facet_intersections_and_omega(
            &fixture.facet_intersection_is_nonempty,
            &fixture.omega_signs,
        );
        let (orbits, iterations) =
            solve_pruned_hk2017_candidates(&fixture.dual_vertices_f64, &transition_is_allowed)?;
        aggregate_certified_orbits_with_dual_vertices_exact(
            &fixture.dual_vertices,
            orbits,
            iterations,
            action_gap_exact,
            mode,
        )
    }

    pub(crate) fn unpruned_capacity(
        dual_vertices: &[Vector4<f64>],
        dual_vertices_exact: &[[BigRational; 4]],
    ) -> Result<OrbitSearchResult, OrbitSearchError> {
        let (orbits, iterations) = solve_unpruned_hk2017_candidates(dual_vertices)?;
        aggregate_orbits_with_dual_vertices_exact(
            dual_vertices_exact,
            orbits,
            iterations,
            0.0,
            OrbitGuaranteeMode::MinimaSafe,
        )
    }

    pub(crate) fn unpruned_capacity_for_dual_vertices(
        dual_vertices: &[Vector4<f64>],
    ) -> Result<OrbitSearchResult, OrbitSearchError> {
        let dual_vertices_exact = exact_dual_vertex_arrays(dual_vertices);
        unpruned_capacity(dual_vertices, &dual_vertices_exact)
    }

    pub(crate) fn unpruned_capacity_for_fixture(
        fixture: &KnownPolytope,
    ) -> Result<OrbitSearchResult, OrbitSearchError> {
        unpruned_capacity(&fixture.dual_vertices_f64, &fixture.dual_vertices)
    }

    pub(crate) fn billiard_capacity_for_fixture(fixture: &KnownPolytope) -> OrbitSearchResult {
        let classification = classify_facets_from_dual_vertices(&fixture.dual_vertices_f64)
            .expect("fixture must be a Lagrangian product");
        let transition_is_allowed = build_transition_matrix_from_facet_intersections_and_omega(
            &fixture.facet_intersection_is_nonempty,
            &fixture.omega_signs,
        );
        let (orbits, iterations) = solve_billiard_candidates(
            &fixture.dual_vertices_f64,
            &classification.q_indices,
            &classification.p_indices,
            &fixture.facet_intersection_is_nonempty,
            &transition_is_allowed,
        )
        .expect("billiard fixture candidate solve");
        aggregate_orbits_with_dual_vertices_exact(
            &fixture.dual_vertices,
            orbits,
            iterations,
            0.0,
            OrbitGuaranteeMode::MinimaSafe,
        )
        .expect("billiard fixture aggregation")
    }
}

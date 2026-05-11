//! Symplectic geometry library for convex-polytopal data in R^4.
//!
//! Computes the Ekeland-Hofer-Zehnder capacity c_EHZ(K) via exhaustive
//! enumeration of closed Reeb orbits.
//!
//! # Submodules
//!
//! - `geom` — symplectic form, exact rational vertex enumeration,
//!   polygon/Lagrangian-product constructors, and named flat polytope fixtures.
//! - `kkt` — context-free constrained QP solvers (saddle-point and
//!   projection variants) + exact rational fallback.
//! - `algorithms` — EHZ capacity algorithms: `hk2017` (general, exponential)
//!   and `billiard` (Lagrangian products, fast). The tube algorithm is being
//!   re-imported from the current mathematical source before it becomes an
//!   active implementation.
//! - `constants` — cross-module numerical tolerance constants.
//! - `dataset` — JSONL row schemas (`PolytopeRow`, `AcceptanceRow`) for
//!   dataset generation and acceptance sweeps.
//! - `derivatives` — analytical ∂c/∂a and ∂vol/∂a w.r.t. dual vertices,
//!   for gradient-based experiments.
//! - `random` — seeded rejection sampling of accepted random dual vertices
//!   (Haar on S^3).
//!
//! # Module dependency graph
//!
//! ```text
//!     geom ──┐
//!            ├──► algorithms ──► (dataset, derivatives, random, binaries)
//!     kkt  ──┘
//! ```
//!
//! `kkt` is deliberately context-free: it operates on abstract matrices
//! (C, d, H) without knowing they come from symplectic geometry. Assembly
//! of QP inputs from flat dual-vertex data lives in `kkt::qp_assembly`, which
//! is the one place that crosses the `geom` ↔ `kkt` boundary.
//!
//! Mathematical proofs live in per-module `.tex` files under `formal/`.

pub mod algorithms;
pub mod constants;
pub mod database;
pub mod dataset;
pub mod derivatives;
pub mod exact;
pub mod geom;
pub mod kkt;
pub mod random;
#[cfg(test)]
mod test_lib;

// ── Re-exports: public API surface ──

// Types
pub use geom::polytope::ConstructionError;

// Capacity algorithms
pub use algorithms::billiard::{
    facet_classification::classify_facets_from_dual_vertices, solve_billiard_candidates,
    BilliardError,
};
pub use algorithms::hk2017::{solve_pruned_hk2017_candidates, solve_unpruned_hk2017_candidates};
pub use algorithms::{
    aggregate_certified_orbits_with_dual_vertices_exact, aggregate_orbits_with_dual_vertices_exact,
    solve_orbit_sigma_saddle_point, CertifiedOrbitKktData, CertifiedOrbitSearchResult,
    CertifiedOrbitSetMode, GeometricOrbitError, OrbitAdmissibility, OrbitGuaranteeMode,
    OrbitKktData, OrbitSearchError, OrbitSearchResult, OrbitSolveError,
};

// Geometry utility functions
pub use geom::lagrangian_product::lagrangian_product;
pub use geom::polygon::{regular_polygon_2d, rotate_polygon_2d};
pub use geom::symplectic_form::omega0;

// Geometry utility submodules
pub use geom::known_polytopes;
pub use geom::test_utils;

#[cfg(test)]
use geom::polytope::Polytope4D;

/// Compute the systolic ratio `sys = capacity^2 / (2 * volume)`.
///
/// Mathematical correspondence: [def:systolic-ratio]
pub fn systolic_ratio(capacity: f64, volume: f64) -> f64 {
    capacity * capacity / (2.0 * volume)
}

#[cfg(test)]
fn transition_matrix_for_polytope(polytope: &Polytope4D) -> nalgebra::DMatrix<bool> {
    algorithms::facet_adjacency::build_transition_matrix_from_facet_intersections_and_omega(
        polytope.facet_intersection_is_nonempty(),
        polytope.omega_signs(),
    )
}

/// Temporary test-only pruned HK2017 helper on the shared orbit/result surface.
///
/// This is intentionally not public API. It keeps legacy in-crate regression
/// tests focused while experiment and crate consumers migrate to flat
/// candidate frontends plus explicit aggregation.
#[cfg(test)]
pub(crate) fn ehz_capacity_pruned(
    polytope: &Polytope4D,
) -> Result<OrbitSearchResult, OrbitSearchError> {
    let dual_vertices = polytope.dual_vertices_f64();
    let dual_vertices_exact = polytope.dual_vertices();
    let transition_is_allowed = transition_matrix_for_polytope(polytope);
    let (orbits, iterations) =
        algorithms::hk2017::solve_pruned_hk2017_candidates(dual_vertices, &transition_is_allowed)?;
    algorithms::orbit_search::aggregate_orbits_with_dual_vertices_exact(
        dual_vertices_exact,
        orbits,
        iterations,
        0.0,
        OrbitGuaranteeMode::MinimaSafe,
    )
}

/// Temporary test-only unpruned HK2017 helper on the shared orbit/result surface.
///
/// This is intentionally not public API. It keeps legacy in-crate regression
/// tests focused while experiment and crate consumers migrate to flat
/// candidate frontends plus explicit aggregation.
#[cfg(test)]
pub(crate) fn ehz_capacity_unpruned(
    polytope: &Polytope4D,
) -> Result<OrbitSearchResult, OrbitSearchError> {
    let dual_vertices = polytope.dual_vertices_f64();
    let dual_vertices_exact = polytope.dual_vertices();

    let (orbits, iterations) = algorithms::hk2017::solve_unpruned_hk2017_candidates(dual_vertices)?;
    algorithms::orbit_search::aggregate_orbits_with_dual_vertices_exact(
        dual_vertices_exact,
        orbits,
        iterations,
        0.0,
        OrbitGuaranteeMode::MinimaSafe,
    )
}

/// Temporary test-only pruned HK2017 helper with exact rational certified output.
///
/// This is intentionally not public API. It keeps legacy in-crate regression
/// tests focused while experiment and crate consumers migrate to flat
/// candidate frontends plus explicit aggregation.
#[cfg(test)]
pub(crate) fn ehz_capacity_pruned_certified(
    polytope: &Polytope4D,
    action_gap_exact: num_rational::BigRational,
    mode: CertifiedOrbitSetMode,
) -> Result<CertifiedOrbitSearchResult, OrbitSearchError> {
    let dual_vertices = polytope.dual_vertices_f64();
    let dual_vertices_exact = polytope.dual_vertices();
    let transition_is_allowed = transition_matrix_for_polytope(polytope);
    let (orbits, iterations) =
        algorithms::hk2017::solve_pruned_hk2017_candidates(dual_vertices, &transition_is_allowed)?;
    algorithms::orbit_search::aggregate_certified_orbits_with_dual_vertices_exact(
        dual_vertices_exact,
        orbits,
        iterations,
        action_gap_exact,
        mode,
    )
}

/// Temporary test-only billiard helper on the shared orbit/result surface.
///
/// This is intentionally not public API. It keeps legacy in-crate regression
/// tests focused while experiment and crate consumers migrate to flat
/// candidate frontends plus explicit aggregation.
#[cfg(test)]
pub(crate) fn ehz_capacity_billiard(
    polytope: &Polytope4D,
) -> Result<OrbitSearchResult, BilliardError> {
    let classification =
        algorithms::billiard::facet_classification::classify_facets(polytope.dual_vertices_f64())?;
    let dual_vertices = polytope.dual_vertices_f64();
    let dual_vertices_exact = polytope.dual_vertices();
    let transition_is_allowed = transition_matrix_for_polytope(polytope);

    let (orbits, iterations) = algorithms::billiard::solve_billiard_candidates(
        dual_vertices,
        &classification.q_indices,
        &classification.p_indices,
        polytope.facet_intersection_is_nonempty(),
        &transition_is_allowed,
    )
    .map_err(|err| match err {
        OrbitSearchError::NoAdmissibleOrbit => {
            unreachable!("f64-only aggregation should return a result")
        }
        OrbitSearchError::NumericalFailure => {
            unreachable!("solve_sigma_stream_with_dual_vertices does not produce NumericalFailure")
        }
        OrbitSearchError::ExactFallbackFailure => {
            unreachable!("solve_sigma_stream_with_dual_vertices never exact-resolves")
        }
        OrbitSearchError::InvalidGap => {
            unreachable!("solve_sigma_stream_with_dual_vertices does not receive an action gap")
        }
    })?;
    algorithms::orbit_search::aggregate_orbits_with_dual_vertices_exact(
        dual_vertices_exact,
        orbits,
        iterations,
        0.0,
        OrbitGuaranteeMode::MinimaSafe,
    )
    .map_err(BilliardError::OrbitSearch)
}

/// Temporary test-only auto-routing capacity helper.
///
/// This is intentionally not public API. It keeps legacy in-crate regression
/// tests focused while experiment and crate consumers migrate to flat
/// candidate frontends plus explicit aggregation.
#[cfg(test)]
pub(crate) fn ehz_capacity(polytope: &Polytope4D) -> Result<OrbitSearchResult, OrbitSearchError> {
    if algorithms::billiard::facet_classification::classify_facets(polytope.dual_vertices_f64())
        .is_ok()
    {
        return ehz_capacity_billiard(polytope).map_err(|err| match err {
            BilliardError::OrbitSearch(err) => err,
            BilliardError::NotLagrangianProduct { .. } | BilliardError::TooFewFacets { .. } => {
                unreachable!("ehz_capacity only routes to billiard after successful classification")
            }
        });
    }
    ehz_capacity_pruned(polytope)
}

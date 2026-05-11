//! HK2017 algorithm for EHZ capacity of general 4D polytopes.
//!
//! Public building blocks live in [`enumeration`] and [`orbit_recovery`], while
//! exhaustive traversal and KKT-bridge logic stay in focused internal modules.
//!
//! ## Architecture
//!
//! - Add HK2017 algorithm behavior in:
//!   - [`enumeration`] for subset/permutation traversal,
//!   - [`combinatorics`] for combination generation.
//! - Add regression or property tests in dedicated `tests_*.rs` files in this
//!   directory, not in this router module.

pub mod orbit_recovery;
pub mod permutations;

mod combinatorics;
mod enumeration;

use crate::algorithms::orbit_search::solve_sigma_stream_with_dual_vertices;
use crate::algorithms::{OrbitKktData, OrbitSearchError};
use nalgebra::{DMatrix, Vector4};

pub use combinatorics::combinations;
pub use enumeration::{for_each_sigma_pruned_by_transition, for_each_sigma_unpruned_facet_count};

/// Solve every HK2017 sigma candidate without transition pruning.
///
/// Input contract: `dual_vertices` is the ordered facet-dual list for a
/// bounded four-dimensional polytope. Candidate sigmas are generated over
/// those facet indices and solved with the saddle-point KKT path.
pub fn solve_unpruned_hk2017_candidates(
    dual_vertices: &[Vector4<f64>],
) -> Result<(Vec<OrbitKktData>, u64), OrbitSearchError> {
    solve_sigma_stream_with_dual_vertices(dual_vertices, |visit| {
        for_each_sigma_unpruned_facet_count(dual_vertices.len(), visit)
    })
}

/// Solve HK2017 sigma candidates allowed by a precomputed transition matrix.
///
/// Input contract: `transition_is_allowed[(i, j)]` describes the directed
/// feasible transition relation for the same ordered facets as
/// `dual_vertices`.
pub fn solve_pruned_hk2017_candidates(
    dual_vertices: &[Vector4<f64>],
    transition_is_allowed: &DMatrix<bool>,
) -> Result<(Vec<OrbitKktData>, u64), OrbitSearchError> {
    assert_eq!(
        transition_is_allowed.shape(),
        (dual_vertices.len(), dual_vertices.len()),
        "transition_is_allowed must be square with one row/column per dual vertex"
    );

    solve_sigma_stream_with_dual_vertices(dual_vertices, |visit| {
        for_each_sigma_pruned_by_transition(transition_is_allowed, visit)
    })
}

#[cfg(test)]
mod tests_capacity_derivative;
#[cfg(test)]
mod tests_conformality;
#[cfg(test)]
mod tests_kkt_edge_cases;
#[cfg(test)]
mod tests_literature;
#[cfg(test)]
mod tests_pruning;
#[cfg(test)]
mod tests_regression;
#[cfg(test)]
mod tests_symplectic_invariance;

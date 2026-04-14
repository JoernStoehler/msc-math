//! HK2017 algorithm for EHZ capacity of general 4D polytopes.
//!
//! Implements the Haim-Kislev 2017 exhaustive enumeration algorithm: for each
//! subset S of facets and each cyclic permutation sigma of S, solve the KKT
//! system for the constrained maximum of Q(beta) and track the minimum action
//! across all certified solutions.
//!
//! Module architecture:
//! - `search` keeps the two public search entry points and the shared exhaustive-search loop.
//! - `selection` isolates combinatorics over facet subsets and cyclic orderings.
//! - `invariants` isolates KKT->solution conversion rules and feasibility classification.
//! - `permutations`, `orbit_recovery`, and `generate_capacity_fixtures` keep their existing focused roles.
//!
//! # Complexity
//!
//! sum_{m=2}^{F} C(F,m) * (m-1)! — exponential in F.
//!
//! Mathematical correspondence: [alg:ehz]

pub mod generate_capacity_fixtures;
mod invariants;
pub mod orbit_recovery;
pub mod permutations;
mod search;
mod selection;

#[cfg(test)]
mod tests;

pub use search::{ehz_capacity, ehz_capacity_unpruned};
pub use selection::combinations;

use crate::algorithms::capacity_accumulator::CapacityResult;

/// Result of the EHZ capacity computation.
///
/// Wraps [`CapacityResult`] (shared accumulator output) plus the algorithm-specific
/// `best_subset` field identifying which facet indices participate in the optimal orbit.
///
/// Access capacity fields via `.result.capacity` (no Deref — explicit field access).
///
/// [alg:ehz]: result of exhaustive (S, sigma) enumeration.
#[derive(Clone, Debug)]
pub struct EhzResult {
    /// Core capacity result from the accumulator: capacity, uncertainty, best
    /// permutation, beta vector, and iteration count.
    pub result: CapacityResult,
    /// Facet indices S participating in the optimal orbit (unordered).
    pub best_subset: Vec<usize>,
}

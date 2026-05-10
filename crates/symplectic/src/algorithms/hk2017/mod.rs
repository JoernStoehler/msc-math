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

pub use combinatorics::combinations;
pub use enumeration::{
    for_each_sigma_pruned, for_each_sigma_pruned_by_transition, for_each_sigma_unpruned,
    for_each_sigma_unpruned_facet_count,
};

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

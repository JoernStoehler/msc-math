//! HK2017 algorithm for EHZ capacity of general 4D polytopes.
//!
//! Public API is defined in [`api`], while exhaustive traversal and KKT-bridge
//! logic live in focused internal modules.
//!
//! ## Architecture
//!
//! - Add HK2017 algorithm behavior in:
//!   - [`api`] for public orbit-collector entry points,
//!   - [`enumeration`] for subset/permutation traversal,
//!   - [`combinatorics`] for combination generation.
//! - Add regression or property tests in dedicated `tests_*.rs` files in this
//!   directory, not in this router module.

pub mod orbit_recovery;
pub mod permutations;

mod api;
mod combinatorics;
mod enumeration;

pub use api::{
    hk2017_minimum_orbits,
    hk2017_minimum_orbits_unpruned,
};
pub use combinatorics::combinations;

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

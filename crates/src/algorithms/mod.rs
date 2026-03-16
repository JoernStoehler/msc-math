//! EHZ capacity algorithms for 4D convex polytopes.
//!
//! Three algorithms:
//! - `hk2017` — general capacity (exponential in #facets)
//! - `billiard` — Lagrangian product capacity (fast)
//! - `tube` — tube algorithm (placeholder)
//!
//! Shared utilities:
//! - `capacity_accumulator` — certified/uncertain candidate tracking
//! - `facet_adjacency` — undirected and directed facet adjacency matrices

pub mod capacity_accumulator;
pub mod facet_adjacency;
pub mod hk2017;
pub mod billiard;
pub mod tube;

#[cfg(test)]
#[path = "capacity_accumulator_test.rs"]
mod capacity_accumulator_test;

#[cfg(test)]
#[path = "facet_adjacency_test.rs"]
mod facet_adjacency_test;

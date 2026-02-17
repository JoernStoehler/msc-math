//! Capacity computation algorithms.
//!
//! Three algorithms with different applicability and cost:
//! - `hk2017`: All polytopes (exponential in #facets)
//! - `billiard`: Lagrangian products only (polynomial)
//! - `tube`: No Lagrangian 2-faces (placeholder)

pub mod hk2017;
pub mod billiard;
pub mod tube;

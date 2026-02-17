//! Symplectic geometry computations on convex polytopes in R^4.
//!
//! Computes the EHZ capacity (minimum action of generalized Reeb orbits) for
//! convex polytopes. Used to probe Viterbo's conjecture: sys(K) ≤ 1.

pub mod geom;
pub mod algorithms;
pub mod constants;
pub(crate) mod kkt;
pub mod random;
pub mod dataset;

// Re-exports for convenient access
pub use algorithms::hk2017::{ehz_capacity, ehz_capacity_pruned, EhzResult};
pub use algorithms::billiard::{billiard_capacity, BilliardError, BilliardResult};
pub use geom::polytope::{ConstructionError, Polytope4D};
pub use geom::QhullError;
pub use geom::volume::volume;
pub use geom::symplectic::omega0;
pub use geom::lagrangian_product::lagrangian_product;
pub use geom::polygon::{regular_polygon_2d, rotate_polygon_2d};
pub use geom::known_polytopes;
pub use geom::test_utils;
pub use geom::skeleton::Skeleton;

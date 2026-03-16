//! Symplectic geometry library for convex polytopes in R^4.
//!
//! Computes the Ekeland-Hofer-Zehnder capacity c_EHZ(K) via exhaustive
//! enumeration of closed Reeb orbits. Provides exact rational geometry,
//! KKT solvers, and named polytope constructors for experiment pipelines.

pub mod geom;
pub mod kkt;
pub mod algorithms;
pub mod constants;
pub mod dataset;
pub mod random;

// ── Re-exports: public API surface ──

// Types
pub use geom::polytope::{ConstructionError, Polytope4D};
pub use geom::skeleton::Skeleton;
pub use geom::QhullError;

// Algorithms
pub use algorithms::hk2017::{ehz_capacity, ehz_capacity_unpruned, EhzResult};
pub use algorithms::billiard::{billiard_capacity, BilliardError, BilliardResult};

// Geometry utilities
pub use geom::volume::volume;
pub use geom::symplectic_form::omega0;
pub use geom::lagrangian_product::lagrangian_product;
pub use geom::polygon::{regular_polygon_2d, rotate_polygon_2d};
pub use geom::known_polytopes;
pub use geom::test_utils;

#[cfg(test)]
#[path = "dataset_test.rs"]
mod dataset_test;

#[cfg(test)]
#[path = "random_test.rs"]
mod random_test;

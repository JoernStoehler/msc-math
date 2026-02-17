//! 2D and 4D symplectic geometry primitives.
//!
//! Provides the foundational types and computations for symplectic geometry
//! on convex polytopes in R^4. All capacity algorithms depend on this module.

pub mod cross_product;
pub mod lagrangian_product;
pub mod polygon;
pub mod polytope;
mod qhull;
pub mod reeb_trajectory;
pub mod skeleton;
pub mod symplectic;
pub mod validation;
pub mod vertices;
pub mod volume;

pub use qhull::QhullError;

pub mod known_polytopes;
pub mod test_utils;

#[cfg(test)]
mod volume_properties_test;

#[cfg(test)]
mod lib_test;

#[cfg(test)]
#[path = "qhull_boundedness_test.rs"]
mod qhull_boundedness_test;

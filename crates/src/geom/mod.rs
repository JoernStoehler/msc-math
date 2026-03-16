//! Geometry primitives and convex polytope types for R^4.
//!
//! Submodules:
//! - `polytope` — `Polytope4D`, the central convex body type
//! - `skeleton` — face lattice (edges, ridges) of a polytope
//! - `symplectic_form` — standard symplectic form omega_0 and J_0
//! - `volume` — volume computation via qhull triangulation
//! - `polygon` — 2D convex polygon constructors
//! - `lagrangian_product` — Lagrangian products from two 2D polygons
//! - `cross_product_4d` — 4D cross product (vector perpendicular to three vectors)
//! - `validation` — boundedness check (f64 fast-fail pre-filter)
//! - `rational_arithmetic` — exact rational arithmetic utilities
//! - `vertex_enumeration` — exact vertex enumeration over Q
//! - `qhull` — qhull subprocess wrapper for volume computation
//! - `reeb_trajectory` — piecewise-linear Reeb flow simulation
//! - `known_polytopes` — named polytope constructors with known EHZ capacity values
//! - `test_utils` — test-only polytope constructors

pub mod polytope;
pub mod skeleton;
pub mod symplectic_form;
pub mod volume;
pub mod polygon;
pub mod lagrangian_product;
pub mod cross_product_4d;
pub mod validation;
pub mod rational_arithmetic;
pub mod vertex_enumeration;
pub mod qhull;
pub mod reeb_trajectory;
pub mod known_polytopes;
pub mod test_utils;

pub use qhull::QhullError;

#[cfg(test)]
#[path = "polytope_test.rs"]
mod polytope_test;

#[cfg(test)]
#[path = "construction_validation_test.rs"]
mod construction_validation_test;

#[cfg(test)]
#[path = "skeleton_test.rs"]
mod skeleton_test;

#[cfg(test)]
#[path = "symplectic_form_test.rs"]
mod symplectic_form_test;

#[cfg(test)]
#[path = "volume_test.rs"]
mod volume_test;

#[cfg(test)]
#[path = "volume_properties_test.rs"]
mod volume_properties_test;

#[cfg(test)]
#[path = "polygon_test.rs"]
mod polygon_test;

#[cfg(test)]
#[path = "lagrangian_product_test.rs"]
mod lagrangian_product_test;

#[cfg(test)]
#[path = "cross_product_4d_test.rs"]
mod cross_product_4d_test;

#[cfg(test)]
#[path = "validation_test.rs"]
mod validation_test;

#[cfg(test)]
#[path = "rational_arithmetic_test.rs"]
mod rational_arithmetic_test;

#[cfg(test)]
#[path = "vertex_enumeration_test.rs"]
mod vertex_enumeration_test;

#[cfg(test)]
#[path = "vertex_enumeration_linalg_test.rs"]
mod vertex_enumeration_linalg_test;

#[cfg(test)]
#[path = "reeb_trajectory_test.rs"]
mod reeb_trajectory_test;

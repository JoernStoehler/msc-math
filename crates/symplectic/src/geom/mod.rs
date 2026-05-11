//! Geometry primitives and flat convex-polytope data helpers for R^4.
//!
//! Submodules:
//! - `symplectic_form` — standard symplectic form omega_0 and J_0
//! - `polygon` — 2D convex polygon constructors
//! - `lagrangian_product` — Lagrangian products from two 2D polygons
//! - `cross_product_4d` — 4D cross product (vector perpendicular to three vectors)
//! - `facet_volume` — per-facet 3D volumes and centroids
//! - `validation` — boundedness check (f64 fast-fail pre-filter)
//! - `rational_arithmetic` — exact rational arithmetic utilities
//! - `vertex_enumeration` — exact vertex enumeration over Q
//! - `qhull` — test-only qhull subprocess wrapper for volume cross-checks
//! - `reeb_trajectory` — piecewise-linear Reeb flow simulation
//! - `known_polytopes` — named flat fixtures with known EHZ capacity values
//! - `test_utils` — test-only flat fixture helpers

pub mod cross_product_4d;
pub mod facet_volume;
pub mod known_polytopes;
pub mod lagrangian_product;
pub mod polygon;
#[cfg(test)]
mod qhull;
pub mod rational_arithmetic;
pub mod reeb_trajectory;
pub mod symplectic_form;
pub mod test_utils;
pub mod validation;
pub mod vertex_enumeration;

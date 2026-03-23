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
//! - `facet_volume` — per-facet 3D volumes and centroids
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
pub mod facet_volume;
pub mod validation;
pub mod rational_arithmetic;
pub mod vertex_enumeration;
pub mod qhull;
pub mod reeb_trajectory;
pub mod known_polytopes;
pub mod test_utils;

pub use qhull::QhullError;

// TEMPORARY: re-export profiling harness. Remove when profiling is complete.
pub use vertex_enumeration::{profile_construction_phases, ConstructionProfile};

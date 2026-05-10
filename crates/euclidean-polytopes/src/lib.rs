//! Euclidean convex-polytope helpers for thesis Rust code.
//!
//! The current public surface starts with exact point-set predicates and exact
//! polar vertex enumeration. Approximate `f64` polar enumeration is diagnostic:
//! it returns partial vertices and approximate indeterminate candidates instead
//! of deciding near a floating-point boundary.
//!
//! Scope boundary: this crate is for ordinary Euclidean convex geometry in
//! ambient `R^4`, including lower-dimensional polytopes in affine subspaces of
//! `R^4`. Symplectic forms, Reeb orbits, capacity algorithms, and KKT assembly
//! remain in the `symplectic` crate or experiment-owned code.

mod f64_geometry;
mod linalg;
mod polar;
mod predicates;

pub use f64_geometry::F64GeometryError;
pub use polar::{
    polar_vertices_exact, polar_vertices_f64, IncidenceF64, IndeterminatePolarCandidateF64,
    PolarVertexData, PolarVerticesF64,
};
pub use predicates::{all_points_are_extreme_exact, origin_in_interior_of_conv_exact};

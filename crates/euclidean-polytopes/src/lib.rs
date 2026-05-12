//! Euclidean convex-polytope helpers for thesis Rust code.
//!
//! The current public surface starts with exact point-set predicates and exact
//! polar vertex enumeration. Approximate `f64` helpers are diagnostic filters
//! with explicit indeterminate outcomes, or known-incidence volume and
//! facet-volume computations used by operational callers.
//!
//! Scope boundary: this crate is for ordinary Euclidean convex geometry in
//! ambient `R^4`, including lower-dimensional polytopes in affine subspaces of
//! `R^4`. Symplectic forms, Reeb orbits, capacity algorithms, and KKT assembly
//! remain in the `symplectic` crate or experiment-owned code.

mod f64_geometry;
mod faces;
mod linalg;
mod polar;
mod predicates;
mod random;
mod volume;

pub use f64_geometry::F64GeometryError;
pub use faces::{
    edges_from_vertex_facet_incidence, facet_intersection_is_nonempty_from_vertex_facet_incidence,
    facet_vertices_from_vertex_facet_incidence, two_faces_from_vertex_facet_incidence,
    vertex_facets_from_vertex_facet_incidence, TwoFace,
};
pub use polar::{
    polar_vertices_exact, polar_vertices_exact_assuming_origin_interior,
    polar_vertices_exact_rational, polar_vertices_exact_rational_assuming_origin_interior,
    PolarVerticesExact,
};
pub use predicates::{
    all_points_are_extreme_exact, orient4_sign_f64, origin_in_interior_of_conv_exact,
    origin_in_interior_of_conv_f64, CertifiedSign, OriginInteriorF64,
};
pub use random::sample_random_dual_vertices_f64;
pub use volume::{
    facet_volume_and_centroid_from_incidence_f64, facet_volume_from_incidence_f64,
    volume_from_incidence_exact, volume_from_incidence_f64,
};

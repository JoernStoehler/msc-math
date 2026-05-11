//! Exact single-orbit kernels over ordered scalar fields.
//!
//! This module is deliberately expert-facing. It exposes:
//! - flat exact 4D dual-vertex helpers over ordered fields,
//! - exact one-sigma KKT solves,
//! - exact dual-vertex capacity gradients from that KKT payload.
//!
//! It does not expose exhaustive exact sigma search, pruning policy, dataset
//! schemas, or exact-vs-approximate comparison/reporting helpers.

pub mod derivatives;
pub mod orbit;
pub mod polytope;

pub use derivatives::{capacity_derivatives_a_exact, capacity_derivatives_a_exact_from_orbit};
pub use orbit::{solve_orbit_sigma_exact, ExactOrbitKktData};
pub use polytope::{
    dot4, exact_vertices_with_incidence, facet_intersection_is_nonempty_exact, omega0,
    omega_signs_exact, ExactPolytopeError, ExactVerticesWithIncidence,
};

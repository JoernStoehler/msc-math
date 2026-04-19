//! Exact single-orbit kernels over ordered scalar fields.
//!
//! This module is deliberately expert-facing. It exposes:
//! - exact 4D polytopes over ordered fields,
//! - exact one-sigma KKT solves,
//! - exact dual-vertex capacity gradients from that KKT payload.
//!
//! It does not expose exhaustive exact sigma search, pruning policy, dataset
//! schemas, or exact-vs-approximate comparison/reporting helpers.

pub mod derivatives;
pub mod orbit;
pub mod polytope;

pub use derivatives::capacity_derivatives_a_exact;
pub use orbit::{solve_orbit_sigma_exact, ExactOrbitKktData};
pub use polytope::{dot4, omega0, ExactPolytope4D, ExactPolytopeError};

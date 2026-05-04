//! Experimental algebraic exactness helpers for `dev-numerical-analysis`.
//!
//! Scope:
//! - named exact ordered fields used by experiment-only exact arithmetic
//! - exact 4D polytope construction over those fields
//! - exact KKT solves for selected sigma checks
//! - experiment-owned exact catalog serialization
//!
//! This module is intentionally experiment-scoped. Durable arithmetic now lives
//! in `crates/algebraic-numbers`, and reusable exact geometry/orbit pieces live
//! in `crates/symplectic/src/exact`. Keep new migration work pointed at those
//! crate surfaces, not at a separate experiment helper API.

pub mod catalog;
pub mod field;
pub mod fixtures;
pub mod geom;
pub mod kkt;
pub mod named_field;
pub mod pentagon;

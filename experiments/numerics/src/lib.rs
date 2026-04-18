//! Shared helpers for dev-numerical-analysis experiments.
//!
//! The algebraic exactness spike lives here so multiple numerics binaries and
//! tests can share the same experimental field, geometry, KKT, and catalog
//! helpers without touching the library core.

pub mod algebraic;

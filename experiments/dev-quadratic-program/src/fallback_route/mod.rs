//! Bridge to the existing crate fallback route.
//!
//! Exact fallback is already implemented in `crates/symplectic`. This module
//! keeps that route visible from the QP development packet without pretending
//! that the local packet owns an instrumented copy yet. If fallback semantics or
//! diagnostics become part of active method development, copy/edit the relevant
//! route here instead of changing the crate first.

pub use symplectic::{
    aggregate_certified_orbits_with_dual_vertices_exact, aggregate_orbits_with_dual_vertices_exact,
    CertifiedOrbitSearchResult, CertifiedOrbitSetMode, OrbitGuaranteeMode, OrbitSearchError,
    OrbitSearchResult,
};

pub use symplectic::exact::{solve_orbit_sigma_exact, ExactOrbitKktData};

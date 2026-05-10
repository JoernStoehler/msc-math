//! EHZ capacity algorithms for 4D convex polytopes.
//!
//! Active algorithms:
//! - `hk2017` — general capacity (exponential in #facets).
//! - `billiard` — Lagrangian product capacity (fast).
//!
//! The tube algorithm is being re-imported from the current mathematical source
//! under `research/`; the old private test module was deleted because it was
//! not a trusted implementation.
//!
//! # Correctness invariant
//!
//! Where domains overlap (notably hypercube and Lagrangian products, which
//! both `hk2017` and `billiard` accept), the algorithms must agree on the
//! computed capacity within numerical tolerance. Cross-algorithm agreement
//! tests live in `billiard::tests::agrees_with_hk2017_*`. This is the
//! primary external correctness check and the reason multiple algorithms
//! coexist rather than being consolidated.
//!
//! Shared utilities:
//! - `facet_adjacency` — facet-intersection and directed (omega_0-aware)
//!   transition matrices for permutation pruning.
//! - `orbit_search` — shared result-layer types for HK2017-family frontends.

pub mod billiard;
pub mod facet_adjacency;
pub mod hk2017;
pub mod orbit_search;

pub use orbit_search::{
    aggregate_certified_orbits, aggregate_orbits, solve_orbit_sigma, CertifiedOrbitKktData,
    CertifiedOrbitSearchResult, CertifiedOrbitSetMode, GeometricOrbitError, OrbitAdmissibility,
    OrbitGuaranteeMode, OrbitKktData, OrbitSearchError, OrbitSearchResult, OrbitSolveBackend,
    OrbitSolveError,
};

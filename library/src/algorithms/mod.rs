//! EHZ capacity algorithms for 4D convex polytopes.
//!
//! Three algorithms:
//! - `hk2017` — general capacity (exponential in #facets).
//! - `billiard` — Lagrangian product capacity (fast).
//! - `tube` — symplectic polytope capacity. Implementation exists
//!   (~1364 LOC) but the rotation-increment formula is incorrect;
//!   blocked on the `tube-algorithm.tex` writeup. Not re-exported from
//!   `lib.rs`. **Do not use.**
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
//! - `capacity_accumulator` — certified/uncertain candidate tracking with
//!   gap invariant.
//! - `facet_adjacency` — undirected and directed (ω₀-aware) facet
//!   adjacency matrices for permutation pruning.
//! - `orbit_search` — shared result-layer types for HK2017-family frontends.

pub mod capacity_accumulator;
pub mod facet_adjacency;
pub mod orbit_search;
pub mod hk2017;
pub mod billiard;
pub mod tube;

pub use orbit_search::{
    GeometricOrbitError,
    OrbitAdmissibility,
    OrbitGuaranteeMode,
    OrbitKktData,
    OrbitSearchError,
    OrbitSearchResult,
    OrbitSolveError,
    OrbitSolveBackend,
    solve_orbit_sigma,
};

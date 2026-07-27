//! Symplectic geometry library for convex-polytopal data in R^4.
//!
//! Provides finite-word candidate enumeration, KKT solvers, exact fallback
//! kernels, and geometric diagnostics used in Ekeland--Hofer--Zehnder capacity
//! computations. Global-capacity guarantees depend on the selected frontend,
//! candidate-family coverage, and certification policy.
//!
//! # Submodules
//!
//! - `geom` — symplectic form, exact rational vertex enumeration,
//!   polygon/Lagrangian-product constructors, and named flat polytope fixtures.
//! - `kkt` — context-free constrained QP solvers (saddle-point and
//!   projection variants) + exact rational fallback.
//! - `algorithms` — the production `capacity_4d` API, retained
//!   orbit-producing HK/billiard controls, and the conditional exact
//!   flow-graph work surface. Each module documents its own input and
//!   guarantee boundary.
//! - `constants` — cross-module numerical tolerance constants.
//! - `dataset` — JSONL row schemas (`PolytopeRow`, `AcceptanceRow`) for
//!   dataset generation and acceptance sweeps.
//! - `derivatives` — analytical ∂c/∂a and ∂vol/∂a w.r.t. dual vertices,
//!   for gradient-based experiments.
//! - `random` — seeded rejection sampling of accepted random dual vertices
//!   (Haar on S^3).
//!
//! # Module dependency graph
//!
//! ```text
//!     geom ──┐
//!            ├──► algorithms ──► (dataset, derivatives, random, binaries)
//!     kkt  ──┘
//! ```
//!
//! `kkt` is deliberately context-free: it operates on abstract matrices
//! (C, d, H) without knowing they come from symplectic geometry. Assembly
//! of QP inputs from flat dual-vertex data lives in `kkt::qp_assembly`, which
//! is the one place that crosses the `geom` ↔ `kkt` boundary.
//!
//! Mathematical proofs live in per-module `.tex` files under `formal/`.

pub mod algorithms;
pub mod constants;
pub mod database;
pub mod dataset;
pub mod derivatives;
pub mod exact;
pub mod geom;
pub mod kkt;
pub mod random;
#[cfg(test)]
mod test_lib;

// ── Re-exports: public API surface ──

// Types
pub use geom::vertex_enumeration::ConstructionError;

// Production capacity API
pub use algorithms::capacity_4d;

// Legacy orbit-producing algorithms retained for controls and branch-sensitive
// experiments.
pub use algorithms::billiard::{
    facet_classification::classify_facets_from_dual_vertices, solve_billiard_candidates,
    BilliardError,
};
pub use algorithms::hk2017::{solve_pruned_hk2017_candidates, solve_unpruned_hk2017_candidates};
pub use algorithms::{
    aggregate_certified_orbits_with_dual_vertices_exact, aggregate_orbits_with_dual_vertices_exact,
    solve_orbit_sigma_saddle_point, CertifiedOrbitKktData, CertifiedOrbitSearchResult,
    CertifiedOrbitSetMode, GeometricOrbitError, OrbitAdmissibility, OrbitGuaranteeMode,
    OrbitKktData, OrbitSearchError, OrbitSearchResult, OrbitSolveError,
};

// Geometry utility functions
pub use geom::lagrangian_product::{lagrangian_product, LagrangianProductError};
pub use geom::polygon::{regular_polygon_2d, rotate_polygon_2d};
pub use geom::symplectic_form::omega0;

// Geometry utility submodules
pub use geom::known_polytopes;
pub use geom::test_utils;

/// Compute the systolic ratio `sys = capacity^2 / (2 * volume)`.
///
/// Mathematical correspondence: [def:systolic-ratio]
pub fn systolic_ratio(capacity: f64, volume: f64) -> f64 {
    capacity * capacity / (2.0 * volume)
}

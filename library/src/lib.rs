//! Symplectic geometry library for convex polytopes in R^4.
//!
//! Computes the Ekeland-Hofer-Zehnder capacity c_EHZ(K) via exhaustive
//! enumeration of closed Reeb orbits.
//!
//! # Submodules
//!
//! - `geom` — `Polytope4D` central type, symplectic form, exact rational
//!   vertex enumeration, volume via qhull, polygon/Lagrangian-product
//!   constructors, named polytopes.
//! - `kkt` — context-free constrained QP solvers (saddle-point and
//!   projection variants) + exact rational fallback.
//! - `algorithms` — EHZ capacity algorithms: `hk2017` (general, exponential),
//!   `billiard` (Lagrangian products, fast), `tube` (symplectic polytopes,
//!   blocked — see algorithms/mod.rs).
//! - `constants` — cross-module numerical tolerance constants.
//! - `dataset` — JSONL row schemas (`PolytopeRow`, `AcceptanceRow`) for
//!   dataset generation and acceptance sweeps.
//! - `derivatives` — analytical ∂c/∂a and ∂vol/∂a w.r.t. dual vertices,
//!   for gradient-based experiments.
//! - `random` — seeded rejection sampling of random polytopes (Haar on S^3).
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
//! of QP inputs from `Polytope4D` lives in `kkt::qp_assembly`, which is
//! the one place that crosses the `geom` ↔ `kkt` boundary.
//!
//! Mathematical proofs live in per-module `math.tex` files under `formal/`.
//! During migration, the canonical developer-math root is being repaired
//! from the old `crates/` build layout to `formal/`.

pub mod geom;
pub mod kkt;
pub mod algorithms;
pub mod constants;
pub mod database;
pub mod dataset;
pub mod derivatives;
pub mod random;

// ── Re-exports: public API surface ──

// Types
pub use geom::polytope::{ConstructionError, Polytope4D};
pub use geom::skeleton::Skeleton;
pub use geom::QhullError;

// Capacity algorithms
pub use algorithms::hk2017::{ehz_capacity, ehz_capacity_unpruned, EhzResult};
pub use algorithms::billiard::{billiard_capacity, BilliardError, BilliardResult};

// Geometry utility functions
pub use geom::volume::volume;
pub use geom::symplectic_form::omega0;
pub use geom::lagrangian_product::lagrangian_product;
pub use geom::polygon::{regular_polygon_2d, rotate_polygon_2d};

// Geometry utility submodules
pub use geom::known_polytopes;
pub use geom::test_utils;

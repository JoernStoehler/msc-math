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
//! Mathematical proofs live in per-module `.tex` files under `formal/`.

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
pub use algorithms::hk2017::{
    hk2017_minimum_orbits,
    hk2017_minimum_orbits_unpruned,
    EhzResult,
};
pub use algorithms::hk2017::ehz_capacity as ehz_capacity_pruned;
pub use algorithms::hk2017::ehz_capacity_unpruned;
pub use algorithms::billiard::{
    billiard_capacity,
    billiard_minimum_orbits,
    BilliardError,
    BilliardOrbitSearchError,
    BilliardResult,
};
pub use algorithms::billiard::billiard_capacity as ehz_capacity_billiard;
pub use algorithms::{
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

// Geometry utility functions
pub use geom::volume::volume;
pub use geom::symplectic_form::omega0;
pub use geom::lagrangian_product::lagrangian_product;
pub use geom::polygon::{regular_polygon_2d, rotate_polygon_2d};

// Geometry utility submodules
pub use geom::known_polytopes;
pub use geom::test_utils;

/// Default capacity wrapper.
///
/// Uses the billiard algorithm on inputs that pass the Lagrangian-product
/// structure test, and otherwise uses the pruned HK2017 path.
pub fn ehz_capacity(polytope: &Polytope4D) -> Option<EhzResult> {
    ehz_capacity_auto(polytope)
}

/// Convenience wrapper that chooses the fast specialized algorithm on
/// Lagrangian products and falls back to the pruned HK2017 path otherwise.
pub fn ehz_capacity_auto(polytope: &Polytope4D) -> Option<EhzResult> {
    if algorithms::billiard::facet_classification::classify_facets(polytope).is_ok() {
        if let Ok(Some(result)) = ehz_capacity_billiard(polytope) {
            return Some(ehz_result_from_billiard(result));
        }
    }
    ehz_capacity_pruned(polytope)
}

fn ehz_result_from_billiard(result: BilliardResult) -> EhzResult {
    let mut best_subset = result.result.best_permutation.clone();
    best_subset.sort_unstable();
    EhzResult {
        result: result.result,
        best_subset,
    }
}

#[cfg(test)]
mod auto_dispatch_tests {
    use super::*;

    #[test]
    fn top_level_capacity_matches_billiard_on_lagrangian_products() {
        let kp = known_polytopes::lagrangian_triangle_product();
        let auto = ehz_capacity(&kp.polytope).expect("auto capacity");
        let billiard = ehz_capacity_billiard(&kp.polytope)
            .expect("billiard should accept Lagrangian product")
            .expect("billiard capacity");

        assert!(
            (auto.result.capacity - billiard.result.capacity).abs() < 1e-10,
            "auto wrapper should agree with billiard on product inputs"
        );
        assert_eq!(
            auto.result.best_permutation,
            billiard.result.best_permutation,
            "auto wrapper should preserve the chosen billiard minimizer"
        );
    }

    #[test]
    fn top_level_capacity_matches_pruned_hk2017_on_non_products() {
        let kp = known_polytopes::simplex();
        let auto = ehz_capacity(&kp.polytope).expect("auto capacity");
        let hk = ehz_capacity_pruned(&kp.polytope).expect("hk2017 capacity");

        assert!(
            (auto.result.capacity - hk.result.capacity).abs() < 1e-10,
            "auto wrapper should fall back to pruned HK2017 on non-products"
        );
        assert_eq!(
            auto.result.best_permutation,
            hk.result.best_permutation,
            "auto wrapper should preserve the pruned HK2017 minimizer on non-products"
        );
    }
}

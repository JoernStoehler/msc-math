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
pub use algorithms::billiard::{
    BilliardError,
};
pub use algorithms::{
    GeometricOrbitError,
    OrbitAdmissibility,
    OrbitGuaranteeMode,
    OrbitKktData,
    OrbitSearchError,
    OrbitSearchResult,
    OrbitSolveError,
    OrbitSolveBackend,
};

// Geometry utility functions
pub use geom::volume::volume;
pub use geom::symplectic_form::omega0;
pub use geom::lagrangian_product::lagrangian_product;
pub use geom::polygon::{regular_polygon_2d, rotate_polygon_2d};

// Geometry utility submodules
pub use geom::known_polytopes;
pub use geom::test_utils;

/// Explicit pruned HK2017 frontend on the shared orbit/result surface.
pub fn ehz_capacity_pruned(polytope: &Polytope4D) -> Result<OrbitSearchResult, OrbitSearchError> {
    let (orbits, iterations) = algorithms::orbit_search::solve_sigma_stream(
        polytope,
        OrbitSolveBackend::SaddlePoint,
        |visit| algorithms::hk2017::for_each_sigma_pruned(polytope, visit),
    )?;
    algorithms::orbit_search::aggregate_orbits_f64_only(0.0, orbits, iterations)
}

/// Explicit unpruned HK2017 frontend on the shared orbit/result surface.
pub fn ehz_capacity_unpruned(
    polytope: &Polytope4D,
) -> Result<OrbitSearchResult, OrbitSearchError> {
    let (orbits, iterations) = algorithms::orbit_search::solve_sigma_stream(
        polytope,
        OrbitSolveBackend::SaddlePoint,
        |visit| algorithms::hk2017::for_each_sigma_unpruned(polytope, visit),
    )?;
    algorithms::orbit_search::aggregate_orbits_f64_only(0.0, orbits, iterations)
}

/// Explicit billiard frontend on the shared orbit/result surface.
pub fn ehz_capacity_billiard(
    polytope: &Polytope4D,
) -> Result<OrbitSearchResult, BilliardError> {
    algorithms::billiard::facet_classification::classify_facets(polytope)?;

    let (orbits, iterations) = algorithms::orbit_search::solve_sigma_stream(
        polytope,
        OrbitSolveBackend::SaddlePoint,
        |visit| algorithms::billiard::for_each_sigma(polytope, visit)
            .expect("classify_facets already succeeded"),
    )
    .map_err(|err| match err {
        OrbitSearchError::UnsupportedBackend => unreachable!("router hardcodes saddle-point backend"),
        OrbitSearchError::NoAdmissibleOrbit => unreachable!("f64-only aggregation should return a result"),
        OrbitSearchError::NumericalFailure => unreachable!("solve_sigma_stream does not produce NumericalFailure"),
        OrbitSearchError::ExactFallbackFailure => unreachable!("f64-only billiard router never exact-resolves"),
    })?;
    algorithms::orbit_search::aggregate_orbits_f64_only(0.0, orbits, iterations).map_err(|err| match err {
        OrbitSearchError::UnsupportedBackend => unreachable!("aggregation does not use backend selection"),
        OrbitSearchError::NoAdmissibleOrbit => unreachable!("f64-only aggregation should return a result"),
        OrbitSearchError::NumericalFailure => unreachable!("f64-only aggregation does not emit NumericalFailure"),
        OrbitSearchError::ExactFallbackFailure => unreachable!("f64-only aggregation never exact-resolves"),
    })
}

/// Default capacity wrapper on the shared orbit/result surface.
///
/// Uses the billiard algorithm on inputs that pass the Lagrangian-product
/// structure test, and otherwise uses the pruned HK2017 path.
pub fn ehz_capacity(polytope: &Polytope4D) -> Result<OrbitSearchResult, OrbitSearchError> {
    if algorithms::billiard::facet_classification::classify_facets(polytope).is_ok() {
        return ehz_capacity_billiard(polytope).map_err(|_| {
            unreachable!("ehz_capacity only routes to billiard after successful classification")
        });
    }
    ehz_capacity_pruned(polytope)
}

#[cfg(test)]
mod auto_dispatch_tests {
    use super::*;

    #[test]
    fn top_level_capacity_matches_billiard_on_lagrangian_products() {
        let kp = known_polytopes::lagrangian_triangle_product();
        let auto = ehz_capacity(&kp.polytope).expect("auto capacity");
        let billiard = ehz_capacity_billiard(&kp.polytope)
            .expect("billiard should accept Lagrangian product");

        assert!(
            (auto.capacity() - billiard.capacity()).abs() < 1e-10,
            "auto wrapper should agree with billiard on product inputs"
        );
        assert_eq!(
            auto.best_sigma(),
            billiard.best_sigma(),
            "auto wrapper should preserve the chosen billiard minimizer"
        );
    }

    #[test]
    fn top_level_capacity_matches_pruned_hk2017_on_non_products() {
        let kp = known_polytopes::simplex();
        let auto = ehz_capacity(&kp.polytope).expect("auto capacity");
        let hk = ehz_capacity_pruned(&kp.polytope).expect("hk2017 capacity");

        assert!(
            (auto.capacity() - hk.capacity()).abs() < 1e-10,
            "auto wrapper should fall back to pruned HK2017 on non-products"
        );
        assert_eq!(
            auto.best_sigma(),
            hk.best_sigma(),
            "auto wrapper should preserve the pruned HK2017 minimizer on non-products"
        );
    }
}

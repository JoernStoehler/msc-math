//! Tests for hk2017: capacity values for known polytopes (simplex, hypercube, products).
//!
//! Proposition: The EHZ capacity computed by `ehz_capacity_unpruned` and `ehz_capacity`
//! agrees with literature values for all named polytopes.
//! Reference: [def:ehz-capacity], [thm:hko-counterexample]
//!
//! Strategy: smoke tests (direct computation, small polytopes) + fixture-based
//! (pre-computed dataset for comprehensive coverage).

use crate::algorithms::hk2017::{ehz_capacity, ehz_capacity_unpruned};
use crate::geom::known_polytopes;

use std::path::PathBuf;
use std::sync::LazyLock;

use super::generate_capacity_fixtures::{
    load_test_dataset, literature_values, polytope_catalog, TestPolytope, FIXTURE_PATH,
};

/// Shared dataset loaded from cached fixture (fast, <1ms).
///
/// If the fixture is missing, panics with instructions to regenerate:
/// `cargo test --release regenerate_test_dataset -- --ignored --nocapture`
static DATASET: LazyLock<Vec<TestPolytope>> = LazyLock::new(|| {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(FIXTURE_PATH);
    load_test_dataset(&path)
});

// ── Smoke tests: direct capacity computation on small polytopes ──

/// Verify unpruned EHZ capacity of the 4-simplex (5 facets) against literature.
///
/// The simplex is the minimal non-trivial polytope. Exercises index arithmetic,
/// enumeration logic, and KKT solver with debug checks enabled.
/// Known value: c_EHZ = 0.25 = 1/(2n) for the 4-simplex (n=2 complex dimensions).
#[test]
fn simplex_capacity() {
    let kp = known_polytopes::simplex();
    let result = ehz_capacity_unpruned(&kp.polytope).expect("simplex should have capacity");
    assert!(
        (result.result.capacity - kp.capacity).abs() < 1e-6,
        "simplex capacity: got {}, expected {}",
        result.result.capacity,
        kp.capacity
    );
}

/// Verify unpruned EHZ capacity of the hypercube (8 facets) against literature.
///
/// Tests that enumeration handles regular geometry correctly.
/// Known value: c_EHZ = 4.0 for the unit hypercube [-1,1]^4.
#[test]
fn hypercube_capacity() {
    let kp = known_polytopes::hypercube();
    let result = ehz_capacity_unpruned(&kp.polytope).expect("hypercube should have capacity");
    assert!(
        (result.result.capacity - kp.capacity).abs() < 1e-6,
        "hypercube capacity: got {}, expected {}",
        result.result.capacity,
        kp.capacity
    );
}

/// Verify unpruned EHZ capacity of the Lagrangian triangle product (7 facets).
///
/// Lagrangian product of equilateral triangle (q-space) and unit square (p-space).
/// Tests product geometry handling.
#[test]
fn lagrangian_triangle_product_capacity() {
    let kp = known_polytopes::lagrangian_triangle_product();
    let result = ehz_capacity_unpruned(&kp.polytope)
        .expect("lagrangian triangle product should have capacity");
    assert!(
        (result.result.capacity - kp.capacity).abs() < 1e-6,
        "lagrangian triangle product capacity: got {}, expected {}",
        result.result.capacity,
        kp.capacity
    );
}

/// Verify pruned EHZ capacity of the Lagrangian triangle x square product (7 facets).
///
/// Tests that adjacency pruning correctly handles product structure.
/// Expected: capacity = 1.5 (optimal orbit uses 3 triangle facets and 2 square facets).
#[test]
fn triangle_square_capacity() {
    let kp = known_polytopes::lagrangian_triangle_square();
    let result = ehz_capacity(&kp.polytope).expect("Lagrangian triangle x square capacity");
    assert!(
        (result.result.capacity - kp.capacity).abs() < 1e-6,
        "Lagrangian triangle x square: got {}, expected {}",
        result.result.capacity,
        kp.capacity
    );
}

/// Verify pruned EHZ capacity of the symplectic triangle x square product (7 facets).
///
/// Symplectic product formula: c(A x_S B) = min(c(A), c(B)).
/// Expected: min(3*sqrt(3)/4, 1.0) = 1.0.
#[test]
fn symplectic_triangle_square_capacity() {
    let kp = known_polytopes::symplectic_triangle_square();
    let result = ehz_capacity(&kp.polytope).expect("symplectic triangle x square capacity");
    assert!(
        (result.result.capacity - kp.capacity).abs() < 1e-6,
        "symplectic triangle x square: got {}, expected {} (min formula)",
        result.result.capacity,
        kp.capacity
    );
}

// ── Fixture-based tests ──

/// Verify polytope_catalog() is deterministic (same seed -> same polytopes).
///
/// Calls polytope_catalog() twice and verifies identical output. Critical invariant
/// for fixture generation: non-determinism would silently invalidate the fixture.
#[test]
fn catalog_determinism() {
    let c1 = polytope_catalog();
    let c2 = polytope_catalog();
    assert_eq!(c1.len(), c2.len());
    for (a, b) in c1.iter().zip(c2.iter()) {
        assert_eq!(a.name, b.name);
        assert_eq!(
            a.polytope.normals_f64(),
            b.polytope.normals_f64(),
            "'{}': normals non-deterministic",
            a.name
        );
        assert_eq!(
            a.polytope.heights_f64(),
            b.polytope.heights_f64(),
            "'{}': heights non-deterministic",
            a.name
        );
    }
}

/// Detect fixture staleness: compares polytope names in fixture vs current catalog.
///
/// If this test warns, regenerate the fixture:
/// `cargo test --release regenerate_test_dataset -- --ignored --nocapture`
#[test]
fn fixture_staleness_check() {
    let catalog = polytope_catalog();
    let dataset = &*DATASET;

    let catalog_names: std::collections::HashSet<&str> =
        catalog.iter().map(|c| c.name.as_str()).collect();
    let fixture_names: std::collections::HashSet<&str> =
        dataset.iter().map(|tp| tp.name.as_str()).collect();

    let missing: Vec<_> = catalog_names.difference(&fixture_names).collect();
    let orphaned: Vec<_> = fixture_names.difference(&catalog_names).collect();

    for name in &missing {
        eprintln!("WARNING: catalog polytope '{}' not in fixture", name);
    }
    for name in &orphaned {
        eprintln!("WARNING: fixture polytope '{}' not in current catalog", name);
    }

    if !missing.is_empty() || !orphaned.is_empty() {
        eprintln!(
            "WARNING: fixture staleness detected ({} missing, {} orphaned). \
             Regenerate with: cargo test --release regenerate_test_dataset -- --ignored --nocapture",
            missing.len(),
            orphaned.len()
        );
    } else {
        eprintln!("Fixture covers all {} catalog polytopes", catalog.len());
    }
}

/// Verify known polytopes match literature capacity values from fixture (~0 cost).
///
/// Loads pre-computed capacities from the fixture and compares against
/// `known_polytopes::literature_values()`.
#[test]
fn literature_capacity_values() {
    let dataset = &*DATASET;
    let lit_values = literature_values();

    for &(name, expected) in &lit_values {
        if let Some(tp) = dataset.iter().find(|tp| tp.name == name) {
            let rel_err = (tp.capacity - expected).abs() / expected;
            assert!(
                rel_err < 1e-6,
                "'{}': fixture capacity {} disagrees with literature value {}, rel_error = {:.2e}",
                name,
                tp.capacity,
                expected,
                rel_err
            );
        } else {
            eprintln!(
                "WARNING: '{}' not in fixture, skipping literature check",
                name
            );
        }
    }

    eprintln!("Verified {} literature values from fixture", lit_values.len());
}

/// Verify all fixture polytopes have strictly positive capacity.
///
/// Proposition: c_EHZ(K) > 0 for any convex body K with nonempty interior.
#[test]
fn capacity_positive_on_all_polytopes() {
    let dataset = &*DATASET;
    for entry in dataset {
        assert!(
            entry.capacity > 0.0,
            "{}: capacity should be positive, got {}",
            entry.name,
            entry.capacity
        );
    }
}

/// Verify HK2017 and billiard agree on all Lagrangian products in the fixture.
///
/// The billiard algorithm is polynomial-time but restricted to Lagrangian products.
/// On the overlapping domain, both algorithms must produce the same capacity.
#[test]
fn billiard_cross_validation() {
    let dataset = &*DATASET;
    let mut checked = 0;
    for tp in dataset.iter() {
        if let Some(cap_billiard) = tp.capacity_billiard {
            let rel_err = (tp.capacity - cap_billiard).abs() / cap_billiard;
            assert!(
                rel_err < 1e-6,
                "'{}': HK2017 ({}) != billiard ({}) capacity, rel_error = {:.2e}",
                tp.name,
                tp.capacity,
                cap_billiard,
                rel_err
            );
            checked += 1;
        }
    }
    assert!(
        checked > 0,
        "expected at least one Lagrangian product in fixture for cross-validation"
    );
}

/// Sanity checks on the systolic ratio distribution across all fixture polytopes.
///
/// sys(K) = c_EHZ(K)^2 / (2 vol(K)). Checks: all positive, all finite, all < 100.
#[test]
fn sys_distribution_sanity_checks() {
    let dataset = &*DATASET;
    let sys_values: Vec<f64> = dataset
        .iter()
        .map(|e| e.capacity.powi(2) / (2.0 * e.volume))
        .collect();

    let min_sys = sys_values.iter().copied().fold(f64::INFINITY, f64::min);
    let max_sys = sys_values
        .iter()
        .copied()
        .fold(f64::NEG_INFINITY, f64::max);

    assert!(min_sys > 0.0, "all sys values should be positive");
    assert!(max_sys < 100.0, "sys values should be reasonable (< 100)");
}

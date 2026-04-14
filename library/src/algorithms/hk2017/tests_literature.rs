//! HK2017 literature and fixture-backed capacity tests.
//!
//! Split from mod.rs to keep the module router focused on architecture.

use super::*;
use crate::geom::known_polytopes;
use crate::kkt::saddle_point_solver::solve_kkt_for;

use std::path::PathBuf;
use std::sync::LazyLock;

use super::generate_capacity_fixtures::{
    literature_values, load_dataset_entries, polytope_catalog, DatasetEntry, FIXTURE_PATH,
};

/// Shared dataset loaded from cached fixture (scalar-only, no Polytope4D construction).
///
/// Uses `load_dataset_entries()` which skips `Polytope4D::new()` construction.
/// Tests in this module only need scalar fields (capacity, volume, name, etc.).
static DATASET: LazyLock<Vec<DatasetEntry>> = LazyLock::new(|| {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(FIXTURE_PATH);
    load_dataset_entries(&path)
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

/// Verify the known minimizing orbit of the 4D crosspolytope gives action = 4.0.
///
/// This is a fast certificate test (single KKT solve + orbit recovery, ~ms).
/// It proves c_EHZ(crosspolytope) ≤ 4.0 by exhibiting a feasible orbit with
/// action 4.0. The full enumeration proving c_EHZ = 4.0 (minimum over all
/// orbits) was done by `experiments/crosspolytope/main/main.rs` using
/// symmetry-reduced exhaustive search (see
/// `research/crosspolytope/design/main.md` for search completeness details).
///
/// Known minimizing orbit: subset {0, 3, 12, 15}, permutation [0, 12, 15, 3],
/// β = (0.25, 0.25, 0.25, 0.25). All transition edges have ω₀ = +1.0.
#[test]
fn crosspolytope_upper_bound() {
    use crate::algorithms::capacity_accumulator::CapacityResult;
    use crate::algorithms::hk2017::orbit_recovery::recover_and_verify;
    use crate::kkt::saddle_point_solver::KktOutcome;

    let kp = known_polytopes::crosspolytope();
    assert_eq!(kp.capacity, 4.0);

    // Solve KKT for the known minimizing permutation [0, 12, 15, 3].
    let perm = [0usize, 12, 15, 3];
    let outcome = solve_kkt_for(&kp.polytope, &perm);

    let kkt_result = match outcome {
        KktOutcome::Feasible(r) => r,
        other => panic!("expected Feasible, got {:?}", other),
    };

    // Verify β ≈ (0.25, 0.25, 0.25, 0.25).
    for (k, &b) in kkt_result.beta.iter().enumerate() {
        assert!((b - 0.25).abs() < 1e-10, "beta[{k}] = {b}, expected 0.25");
    }

    // Verify action = 0.5 / Q ≈ 4.0.
    let action = 0.5 / kkt_result.q_corrected;
    assert!(
        (action - 4.0).abs() < 1e-8,
        "action = {action}, expected 4.0"
    );

    // Orbit recovery: construct EhzResult and verify geometric validity.
    let ehz_result = EhzResult {
        result: CapacityResult {
            capacity: action,
            capacity_uncertain: action,
            best_permutation: perm.to_vec(),
            best_beta: kkt_result.beta.clone(),
            iterations: 1,
        },
        best_subset: vec![0, 3, 12, 15],
    };

    let recovery = recover_and_verify(&kp.polytope, &ehz_result).expect("orbit recovery failed");

    assert!(
        recovery.closure_error < 1e-8,
        "closure error {:.2e} too large",
        recovery.closure_error
    );
    assert!(
        recovery.max_violation < 1e-6,
        "max violation {:.2e} too large",
        recovery.max_violation
    );
    assert!(
        (recovery.action - 4.0).abs() < 1e-8,
        "recovered action = {}, expected 4.0",
        recovery.action
    );
}

// ── Fixture-based tests ──

/// Verify polytope_catalog() is deterministic (same seed -> same polytopes).
///
/// Calls polytope_catalog() twice and verifies identical output. Critical invariant
/// for fixture generation: non-determinism would silently invalidate the fixture.
#[test]
#[ignore] // ~17s debug, ~1s release: constructs all 33 polytopes twice. Run during fixture regeneration.
fn catalog_determinism() {
    let c1 = polytope_catalog();
    let c2 = polytope_catalog();
    assert_eq!(c1.len(), c2.len());
    for (a, b) in c1.iter().zip(c2.iter()) {
        assert_eq!(a.name, b.name);
        assert_eq!(
            a.polytope.dual_vertices_f64(),
            b.polytope.dual_vertices_f64(),
            "'{}': dual vertices non-deterministic",
            a.name
        );
    }
}

/// Detect fixture staleness: compares polytope names in fixture vs current catalog.
///
/// If this test warns, regenerate the fixture:
/// `cargo test --release regenerate_test_dataset -- --ignored --nocapture`
#[test]
#[ignore] // ~17s debug, ~1s release: constructs all 33 polytopes. Run during fixture regeneration.
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
        eprintln!(
            "WARNING: fixture polytope '{}' not in current catalog",
            name
        );
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

    eprintln!(
        "Verified {} literature values from fixture",
        lit_values.len()
    );
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
    let max_sys = sys_values.iter().copied().fold(f64::NEG_INFINITY, f64::max);

    assert!(min_sys > 0.0, "all sys values should be positive");
    assert!(max_sys < 100.0, "sys values should be reasonable (< 100)");
}

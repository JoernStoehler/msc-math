//! Tests for orbit_recovery: base point recovery and orbit verification.
//!
//! Proposition: for a valid EHZ result (sigma, beta), the recovered orbit gamma
//! has closure error ~ 0, lies on the correct facets (on-facet error ~ 0),
//! stays inside K (max violation ~ 0), and its computed action matches the capacity.
//!
//! Reference: [lem:base-point-recovery], [rem:beta-to-tau], [lem:shoelace]
//!
//! Strategy: fixture-based on known polytopes from `known_polytopes`.

use super::orbit_recovery::recover_and_verify;
use super::{ehz_capacity, EhzResult};
use crate::geom::known_polytopes;
use crate::geom::polytope::Polytope4D;

/// Tolerance for floating-point comparisons (closure, on-facet, action).
const TOL: f64 = 1e-8;

/// Tolerance for inequality constraint violations.
/// Slightly positive to allow numerical noise at breakpoints.
const INEQ_TOL: f64 = 1e-6;

/// Run the full recovery + verification pipeline on a known polytope and
/// check all error metrics against tolerances.
fn test_recovery(name: &str, polytope: &Polytope4D, expected_capacity: f64) {
    let result = ehz_capacity(polytope).unwrap_or_else(|| {
        panic!("{name}: capacity computation failed");
    });
    assert!(
        (result.result.capacity - expected_capacity).abs() < 1e-4,
        "{name}: capacity mismatch: got {}, expected {expected_capacity}",
        result.result.capacity
    );

    let recovery = recover_and_verify(polytope, &result).unwrap_or_else(|| {
        panic!("{name}: orbit recovery failed");
    });

    eprintln!(
        "{name}: max_violation={:.2e}, closure={:.2e}, action_err={:.2e}, segments={}",
        recovery.max_violation,
        recovery.closure_error,
        (recovery.action - result.result.capacity).abs(),
        recovery.facet_sequence.len(),
    );

    // Closure: orbit returns to its starting point.
    assert!(
        recovery.closure_error < TOL,
        "{name}: closure error {:.2e} exceeds tolerance",
        recovery.closure_error
    );

    // Inside K: orbit stays inside the polytope at all breakpoints.
    assert!(
        recovery.max_violation < INEQ_TOL,
        "{name}: max violation {:.2e} exceeds tolerance\n  facet_sequence={:?}\n  dwell_times={:?}",
        recovery.max_violation,
        recovery.facet_sequence,
        recovery.dwell_times,
    );

    // Action: computed action matches capacity.
    let action_error = (recovery.action - result.result.capacity).abs();
    assert!(
        action_error < TOL,
        "{name}: action error {:.2e} (computed {}, expected {})",
        action_error,
        recovery.action,
        result.result.capacity,
    );

    // Facet sequence matches the best permutation.
    assert_eq!(
        recovery.facet_sequence,
        result.result.best_permutation,
        "{name}: facet_sequence does not match best_permutation"
    );

    // Breakpoint count = permutation length + 1 (includes start and closure point).
    assert_eq!(
        recovery.breakpoints.len(),
        recovery.facet_sequence.len() + 1,
        "{name}: breakpoint count mismatch"
    );

    // Dwell time count matches permutation length.
    assert_eq!(
        recovery.dwell_times.len(),
        recovery.facet_sequence.len(),
        "{name}: dwell_times length mismatch"
    );
}

/// Helper to verify on-facet property: each breakpoint k lies on facet sigma(k).
fn check_on_facet(
    name: &str,
    polytope: &Polytope4D,
    result: &EhzResult,
) {
    let normals = polytope.normals_f64();
    let heights = polytope.heights_f64();
    let sigma = &result.result.best_permutation;

    let recovery = recover_and_verify(polytope, result).unwrap();

    // For each active segment k (dwell_times[k] > 0), breakpoint[k] should lie
    // on facet sigma(k): <n_{sigma(k)}, breakpoint[k]> ~ h_{sigma(k)}.
    let on_facet_error = (0..sigma.len())
        .filter(|&k| recovery.dwell_times[k] > 0.0)
        .map(|k| {
            let i = sigma[k];
            (normals[i].dot(&recovery.breakpoints[k]) - heights[i]).abs()
        })
        .fold(0.0_f64, f64::max);

    assert!(
        on_facet_error < TOL,
        "{name}: on-facet error {:.2e} exceeds tolerance",
        on_facet_error
    );
}

/// Recover orbit for the 4-simplex (F=5).
///
/// Minimal polytope. Known capacity = 2.0.
/// Exercises SVD solve and verification on a small system.
#[test]
fn simplex_recovery() {
    let kp = known_polytopes::simplex();
    test_recovery("simplex", &kp.polytope, kp.capacity);
}

/// On-facet check for the simplex.
#[test]
fn simplex_on_facet() {
    let kp = known_polytopes::simplex();
    let result = ehz_capacity(&kp.polytope).unwrap();
    check_on_facet("simplex", &kp.polytope, &result);
}

/// Recover orbit for the hypercube (F=8).
///
/// High symmetry, known capacity = 1.0. Lagrangian product structure
/// may produce a non-unique base point (solution_dim > 0).
#[test]
fn hypercube_recovery() {
    let kp = known_polytopes::hypercube();
    test_recovery("hypercube", &kp.polytope, kp.capacity);
}

/// Recover orbit for the crosspolytope (F=16).
///
/// Known capacity = 1.0. F=16 makes this too slow for debug mode.
#[test]
#[ignore]
fn crosspolytope_recovery() {
    let kp = known_polytopes::crosspolytope();
    test_recovery("crosspolytope", &kp.polytope, kp.capacity);
}

/// Recover orbit for the HKO pentagon (F=10).
///
/// The Haim-Kislev-Ostrover counterexample with sys > 1.
/// F=10 is fast with the pruned algorithm.
#[test]
fn hko_pentagon_recovery() {
    let kp = known_polytopes::hko_pentagon();
    test_recovery("hko_pentagon", &kp.polytope, kp.capacity);
}

/// Recover orbit for a Lagrangian triangle product (F=7).
///
/// Lagrangian products have special billiard structure; good cross-check.
#[test]
fn lagrangian_triangle_product_recovery() {
    let kp = known_polytopes::lagrangian_triangle_product();
    test_recovery("lagrangian_triangle_product", &kp.polytope, kp.capacity);
}

/// Recover orbit for a symplectic triangle product (F=7).
///
/// Non-Lagrangian product geometry.
#[test]
fn symplectic_triangle_product_recovery() {
    let kp = known_polytopes::symplectic_triangle_product();
    test_recovery("symplectic_triangle_product", &kp.polytope, kp.capacity);
}

/// Recover orbit for a Lagrangian triangle-square product (F=7).
///
/// Mixed product geometry: triangle x square.
#[test]
fn lagrangian_triangle_square_recovery() {
    let kp = known_polytopes::lagrangian_triangle_square();
    test_recovery("lagrangian_triangle_square", &kp.polytope, kp.capacity);
}

/// Recover orbit for a symplectic triangle-square product (F=7).
///
/// Another mixed product geometry.
#[test]
fn symplectic_triangle_square_recovery() {
    let kp = known_polytopes::symplectic_triangle_square();
    test_recovery("symplectic_triangle_square", &kp.polytope, kp.capacity);
}

/// Verify that dwell times are non-negative for all known polytopes.
///
/// Dwell times tau_k = T * h_{sigma(k)} * beta_k. Since T > 0, h > 0,
/// and beta_k > 0 (certified), all dwell times should be positive.
/// Skips polytopes with F > 10 (too slow for debug mode).
#[test]
fn dwell_times_positive() {
    for kp in known_polytopes::all_known() {
        if kp.polytope.facet_count() > 10 {
            continue;
        }
        let result = ehz_capacity(&kp.polytope).unwrap();
        let recovery = recover_and_verify(&kp.polytope, &result).unwrap();

        for (k, &tau) in recovery.dwell_times.iter().enumerate() {
            assert!(
                tau > 0.0,
                "{}: dwell_times[{k}] = {tau:.2e} is not positive",
                kp.name,
            );
        }
    }
}

/// Verify breakpoint count equals permutation length + 1.
///
/// The breakpoints array includes the starting point and the closure point,
/// so it has m+1 entries for an m-facet orbit.
#[test]
fn breakpoint_count_consistency() {
    for kp in known_polytopes::all_known() {
        if kp.polytope.facet_count() > 10 {
            continue;
        }
        let result = ehz_capacity(&kp.polytope).unwrap();
        let recovery = recover_and_verify(&kp.polytope, &result).unwrap();

        assert_eq!(
            recovery.breakpoints.len(),
            recovery.facet_sequence.len() + 1,
            "{}: expected {} breakpoints, got {}",
            kp.name,
            recovery.facet_sequence.len() + 1,
            recovery.breakpoints.len(),
        );
    }
}

/// Verify that the unpruned algorithm gives consistent recovery results.
///
/// Both pruned and unpruned algorithms should yield orbits with the same
/// computed action on the simplex (fast enough for unpruned in debug mode).
#[test]
fn unpruned_recovery_consistent() {
    use super::ehz_capacity_unpruned;

    let kp = known_polytopes::simplex();

    let result_pruned = ehz_capacity(&kp.polytope).unwrap();
    let result_unpruned = ehz_capacity_unpruned(&kp.polytope).unwrap();

    let recovery_pruned = recover_and_verify(&kp.polytope, &result_pruned).unwrap();
    let recovery_unpruned = recover_and_verify(&kp.polytope, &result_unpruned).unwrap();

    // Both should have valid orbits.
    assert!(recovery_pruned.closure_error < TOL);
    assert!(recovery_unpruned.closure_error < TOL);
    assert!(recovery_pruned.max_violation < INEQ_TOL);
    assert!(recovery_unpruned.max_violation < INEQ_TOL);

    // Actions should match (same polytope, same capacity).
    assert!(
        (recovery_pruned.action - recovery_unpruned.action).abs() < TOL,
        "action mismatch: pruned {}, unpruned {}",
        recovery_pruned.action,
        recovery_unpruned.action,
    );
}

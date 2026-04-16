//! HK2017 literature-backed capacity tests.
//!
//! Split from mod.rs to keep the module router focused on architecture.

use super::*;
use crate::algorithms::orbit_search::solve_sigma_stream;
use crate::algorithms::{aggregate_orbits, OrbitGuaranteeMode, OrbitSearchError, OrbitSolveBackend};
use crate::geom::known_polytopes;
use crate::kkt::saddle_point_solver::solve_kkt_for;
use crate::{ehz_capacity_pruned, ehz_capacity_unpruned};

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
        (result.capacity() - kp.capacity).abs() < 1e-6,
        "simplex capacity: got {}, expected {}",
        result.capacity(),
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
        (result.capacity() - kp.capacity).abs() < 1e-6,
        "hypercube capacity: got {}, expected {}",
        result.capacity(),
        kp.capacity
    );
}

/// Verify unpruned EHZ capacity of the Lagrangian triangle product (6 facets).
///
/// Lagrangian product of equilateral triangle (q-space) and unit square (p-space).
/// Tests product geometry handling.
#[test]
fn lagrangian_triangle_product_capacity() {
    let kp = known_polytopes::lagrangian_triangle_product();
    let result = ehz_capacity_unpruned(&kp.polytope)
        .expect("lagrangian triangle product should have capacity");
    assert!(
        (result.capacity() - kp.capacity).abs() < 1e-6,
        "lagrangian triangle product capacity: got {}, expected {}",
        result.capacity(),
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
    let result = ehz_capacity_pruned(&kp.polytope).expect("Lagrangian triangle x square capacity");
    assert!(
        (result.capacity() - kp.capacity).abs() < 1e-6,
        "Lagrangian triangle x square: got {}, expected {}",
        result.capacity(),
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
    let result = ehz_capacity_pruned(&kp.polytope).expect("symplectic triangle x square capacity");
    assert!(
        (result.capacity() - kp.capacity).abs() < 1e-6,
        "symplectic triangle x square: got {}, expected {} (min formula)",
        result.capacity(),
        kp.capacity
    );
}

/// Smoke-test the richer collector on a simple known polytope.
#[test]
fn simplex_minimum_orbits_collector() {
    let kp = known_polytopes::simplex();
    let (orbits, iterations) = solve_sigma_stream(
        &kp.polytope,
        OrbitSolveBackend::SaddlePoint,
        |visit| for_each_sigma_pruned(&kp.polytope, visit),
    )
    .expect("sigma solve stream should succeed on simplex");
    let result = aggregate_orbits(
        &kp.polytope,
        orbits,
        iterations,
        0.0,
        OrbitGuaranteeMode::BoundSafe,
    )
    .expect("orbit aggregation should succeed on simplex");

    assert!(!result.orbits.is_empty(), "collector must return at least one orbit");
    assert!(
        result.min_action_lower <= result.min_action_upper,
        "minimum-action interval should be ordered"
    );
    assert!(
        result
            .orbits
            .iter()
            .all(|orbit| orbit.action_lower <= result.min_action_upper),
        "gap=0 collector should only retain orbits that can still hit the minimum upper bound"
    );
}

/// Unsupported backends should fail explicitly rather than silently degrading.
#[test]
fn simplex_minimum_orbits_projected_backend_unsupported() {
    let kp = known_polytopes::simplex();
    let err = solve_sigma_stream(
        &kp.polytope,
        OrbitSolveBackend::Projected,
        |visit| for_each_sigma_pruned(&kp.polytope, visit),
    )
    .expect_err("projected backend is not wired into the shared collector yet");
    assert_eq!(err, OrbitSearchError::UnsupportedBackend);
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

    let orbit = crate::algorithms::OrbitKktData {
        sigma: perm.to_vec(),
        beta: kkt_result.beta.clone(),
        beta_margin: kkt_result.beta.iter().copied().fold(f64::INFINITY, f64::min),
        action,
        action_lower: action,
        action_upper: action,
        q: kkt_result.q_corrected,
        q_error_bound: 0.0,
        mu: None,
        xi: Some(kkt_result.xi),
        admissibility: crate::algorithms::OrbitAdmissibility::AdmissibleF64,
    };

    let recovery = recover_and_verify(&kp.polytope, &orbit).expect("orbit recovery failed");

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

// ── Cross-algorithm smoke tests ──

/// Verify HK2017 and billiard agree on small Lagrangian products.
///
/// The billiard algorithm is polynomial-time but restricted to Lagrangian products.
/// On the overlapping domain, both algorithms must produce the same capacity.
/// Broad cross-algorithm validation lives in
/// `experiments/verification/correctness/`.
#[test]
fn billiard_agrees_with_hk2017_on_small_lagrangian_products() {
    for kp in [
        known_polytopes::hypercube(),
        known_polytopes::lagrangian_triangle_product(),
        known_polytopes::lagrangian_triangle_square(),
    ] {
        let hk = ehz_capacity_pruned(&kp.polytope).expect("HK2017 capacity");
        let billiard =
            crate::ehz_capacity_billiard(&kp.polytope).expect("billiard should have capacity");
        let rel_err = (hk.capacity() - billiard.capacity()).abs() / billiard.capacity();
        assert!(
            rel_err < 1e-6,
            "{}: HK2017 ({}) != billiard ({}) capacity, rel_error = {:.2e}",
            kp.name,
            hk.capacity(),
            billiard.capacity(),
            rel_err
        );
    }
}

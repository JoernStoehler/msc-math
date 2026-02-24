use super::recover::{recover_base_point, verify_orbit};
use super::{ehz_capacity, ehz_capacity_unpruned};
use crate::geom::known_polytopes;

/// Tolerance for floating-point comparisons.
const TOL: f64 = 1e-8;

/// Tolerance for inequality constraint violations.
/// Slightly positive to allow numerical noise.
const INEQ_TOL: f64 = 1e-6;

/// Helper: run full recovery + verification pipeline on a known polytope.
fn test_recovery(name: &str, polytope: &crate::geom::polytope::Polytope4D, expected_capacity: f64) {
    let result = ehz_capacity(polytope).unwrap_or_else(|| {
        panic!("{name}: capacity computation failed");
    });
    assert!(
        (result.capacity - expected_capacity).abs() < 1e-4,
        "{name}: capacity mismatch: got {}, expected {expected_capacity}",
        result.capacity
    );

    let recovery = recover_base_point(polytope, &result).unwrap_or_else(|| {
        panic!("{name}: base point recovery failed");
    });
    let verification = verify_orbit(polytope, &result, &recovery);

    eprintln!("{name}: solution_dim={}, max_violation={:.2e}, closure={:.2e}, on_facet={:.2e}, action_err={:.2e}",
        recovery.solution_dim, recovery.max_violation,
        verification.closure_error, verification.on_facet_error, verification.action_error);

    // Closure: orbit returns to starting point
    assert!(
        verification.closure_error < TOL,
        "{name}: closure error {:.2e} exceeds tolerance",
        verification.closure_error
    );

    // On-facet: each segment lies on the correct facet
    assert!(
        verification.on_facet_error < TOL,
        "{name}: on-facet error {:.2e} exceeds tolerance",
        verification.on_facet_error
    );

    // Inside K: orbit stays inside the polytope at breakpoints
    assert!(
        recovery.max_violation < INEQ_TOL,
        "{name}: max inequality violation {:.2e} exceeds tolerance (solution_dim={})\n  base_point={:?}\n  perm={:?}\n  dwell_times={:?}",
        recovery.max_violation, recovery.solution_dim,
        recovery.base_point, result.best_permutation, recovery.dwell_times
    );

    // Action: computed action matches capacity
    assert!(
        verification.action_error < TOL,
        "{name}: action error {:.2e} (computed {}, expected {})",
        verification.action_error,
        verification.computed_action,
        result.capacity
    );
}

/// Recover base point for the 4-simplex (F=5).
///
/// **What:** Full recovery + verification pipeline on minimal polytope.
/// **Why debug mode:** Exercises SVD solve and verification with debug checks.
/// **Why this polytope:** F=5 simplex, instant in debug. Known capacity = 2.0.
#[test]
fn simplex_recovery() {
    let kp = known_polytopes::simplex();
    test_recovery("simplex", &kp.polytope, kp.capacity);
}

/// Recover base point for the hypercube (F=8).
///
/// **What:** Full recovery + verification on a symmetric polytope.
/// **Why this polytope:** High symmetry, known capacity = 1.0.
/// Lagrangian product structure may produce non-unique b.
#[test]
fn hypercube_recovery() {
    let kp = known_polytopes::hypercube();
    test_recovery("hypercube", &kp.polytope, kp.capacity);
}

/// Recover base point for the crosspolytope (F=16).
///
/// **What:** Full recovery + verification on the dual of the hypercube.
/// **Why this polytope:** F=16, known capacity = 1.0. Different geometry from hypercube.
/// **Why #[ignore]:** F=16 → exponential enumeration too slow in debug mode.
/// **Run with:** `cargo test --release crosspolytope_recovery -- --ignored`
#[test]
#[ignore]
fn crosspolytope_recovery() {
    let kp = known_polytopes::crosspolytope();
    test_recovery("crosspolytope", &kp.polytope, kp.capacity);
}

/// Recover base point for the HKO pentagon (F=10).
///
/// **What:** Full recovery + verification on the Haim-Kislev-Ostrover counterexample.
/// **Why this polytope:** The famous counterexample with sys > 1. F=10.
/// **Why debug mode:** F=10 is still fast with the pruned algorithm.
#[test]
fn hko_pentagon_recovery() {
    let kp = known_polytopes::hko_pentagon();
    test_recovery("hko_pentagon", &kp.polytope, kp.capacity);
}

/// Recover base point for a Lagrangian triangle product (F=7).
///
/// **What:** Full recovery + verification on a Lagrangian product.
/// **Why this polytope:** Lagrangian products have special structure;
/// the billiard algorithm applies. Good cross-check.
#[test]
fn lagrangian_triangle_product_recovery() {
    let kp = known_polytopes::lagrangian_triangle_product();
    test_recovery("lagrangian_triangle_product", &kp.polytope, kp.capacity);
}

/// Recover base point for a symplectic triangle product (F=7).
///
/// **What:** Full recovery + verification on a non-Lagrangian product.
/// **Why this polytope:** F=7, different geometry from Lagrangian products.
#[test]
fn symplectic_triangle_product_recovery() {
    let kp = known_polytopes::symplectic_triangle_product();
    test_recovery("symplectic_triangle_product", &kp.polytope, kp.capacity);
}

/// Recover base point for a Lagrangian triangle-square product (F=7).
///
/// **What:** Full recovery + verification.
/// **Why this polytope:** Mixed product geometry.
#[test]
fn lagrangian_triangle_square_recovery() {
    let kp = known_polytopes::lagrangian_triangle_square();
    test_recovery("lagrangian_triangle_square", &kp.polytope, kp.capacity);
}

/// Recover base point for a symplectic triangle-square product (F=7).
///
/// **What:** Full recovery + verification.
/// **Why this polytope:** Another mixed product geometry.
#[test]
fn symplectic_triangle_square_recovery() {
    let kp = known_polytopes::symplectic_triangle_square();
    test_recovery("symplectic_triangle_square", &kp.polytope, kp.capacity);
}

/// Verify solution_dim is consistent with number of active facets.
///
/// **What:** Check that solution_dim = 4 − rank(N_S) ≤ 4, and that
/// rank(N_S) ≤ min(4, m_active). Normals can be linearly dependent
/// (e.g., hypercube has parallel normal pairs), so we cannot assume
/// rank = min(4, m_active).
/// **Why:** Validates the rank computation in recover_base_point.
/// Skips crosspolytope (F=16, too slow in debug).
#[test]
fn solution_dim_consistency() {
    for kp in known_polytopes::all_known() {
        // Skip crosspolytope (F=16) — too slow for debug mode capacity computation.
        if kp.polytope.facet_count() > 10 {
            continue;
        }
        let result = ehz_capacity(&kp.polytope).unwrap();
        let recovery = recover_base_point(&kp.polytope, &result).unwrap();

        let active_count = recovery
            .dwell_times
            .iter()
            .filter(|&&t| t > 0.0)
            .count();

        // solution_dim ∈ [0, 4] and rank = 4 − solution_dim ≤ active_count
        let rank = 4 - recovery.solution_dim;
        assert!(
            recovery.solution_dim <= 4,
            "{}: solution_dim {} exceeds 4",
            kp.name,
            recovery.solution_dim,
        );
        assert!(
            rank <= active_count,
            "{}: rank {} exceeds active_count {} (solution_dim = {})",
            kp.name,
            rank,
            active_count,
            recovery.solution_dim,
        );

        eprintln!(
            "{}: active_count={}, rank={}, solution_dim={}",
            kp.name, active_count, rank, recovery.solution_dim
        );
    }
}

/// Verify that unpruned algorithm gives same recovery as pruned.
///
/// **What:** Consistency check between pruned and unpruned algorithms.
/// **Why this polytope:** Simplex is fast enough for unpruned in debug.
#[test]
fn unpruned_recovery_matches() {
    let kp = known_polytopes::simplex();

    let result_pruned = ehz_capacity(&kp.polytope).unwrap();
    let result_unpruned = ehz_capacity_unpruned(&kp.polytope).unwrap();

    let recovery_pruned = recover_base_point(&kp.polytope, &result_pruned).unwrap();
    let recovery_unpruned = recover_base_point(&kp.polytope, &result_unpruned).unwrap();

    let v_pruned = verify_orbit(&kp.polytope, &result_pruned, &recovery_pruned);
    let v_unpruned = verify_orbit(&kp.polytope, &result_unpruned, &recovery_unpruned);

    // Both should have valid orbits
    assert!(v_pruned.closure_error < TOL);
    assert!(v_unpruned.closure_error < TOL);
    assert!(v_pruned.on_facet_error < TOL);
    assert!(v_unpruned.on_facet_error < TOL);

    // Actions should match (same polytope, same capacity)
    assert!(
        (v_pruned.computed_action - v_unpruned.computed_action).abs() < TOL,
        "action mismatch: pruned {}, unpruned {}",
        v_pruned.computed_action,
        v_unpruned.computed_action
    );
}

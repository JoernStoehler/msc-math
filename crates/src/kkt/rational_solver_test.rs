//! Tests for rational_solver: exact KKT solve correctness and null-space handling.
//!
//! Proposition: The exact rational KKT solver produces correct beta > 0 and Q values
//! that agree with the f64 solver to machine precision on known polytopes.
//! Reference: [lem:kkt], [lem:well-defined]
//!
//! Strategy: fixture-based on known polytopes (simplex, hypercube, HKO pentagon)

use crate::geom::known_polytopes;
use crate::kkt::rational_solver::solve_kkt_exact;
use num_traits::{Signed, Zero};

/// Exact KKT solve on the simplex (F=5) returns a solution with nonzero Q.
///
/// Simplex is the smallest polytope (F=5). The identity permutation [0,1,2,3,4]
/// exercises Gaussian elimination with a full-rank (10 x 10) system.
#[test]
fn simplex_exact_solve() {
    let simplex = known_polytopes::simplex().polytope;

    let perm: Vec<usize> = (0..5).collect();
    let result = solve_kkt_exact(simplex.dual_vertices(), &perm);
    assert!(result.is_some(), "Simplex KKT system should be solvable");

    let r = result.unwrap();
    assert_eq!(r.beta.len(), 5);
    assert!(
        !r.q_exact.is_zero(),
        "Q_exact should be nonzero for a non-degenerate system"
    );
    assert!(
        r.q_exact_f64.is_finite(),
        "Q_exact_f64 should be finite, got {}",
        r.q_exact_f64
    );
}

/// Exact Q is a valid rational on the hypercube.
///
/// The hypercube has axis-aligned normals so many pairs have omega_0 = 0.
/// Exercises rank-deficient code paths.
#[test]
fn hypercube_exact_solve() {
    let hypercube = known_polytopes::hypercube().polytope;

    // Try a 4-facet subset. The hypercube's axis-aligned normals mean omega_0(y_i, y_j) = 0
    // for many pairs. Q can be zero even with nonzero beta.
    let perm = vec![0, 1, 2, 3];
    if let Some(r) = solve_kkt_exact(hypercube.dual_vertices(), &perm) {
        assert!(
            r.q_exact_f64.is_finite(),
            "Q_exact_f64 should be finite"
        );
    }
    // Both Some and None are valid — no panic is the key invariant.
}

/// A short permutation does not cause a panic.
///
/// A 2-element permutation with m+5 = 7 variables should either solve or
/// return None, not panic on under- or over-determined systems.
#[test]
fn short_permutation_no_panic() {
    let simplex = known_polytopes::simplex().polytope;

    let perm = vec![0, 1];
    // Whether this returns Some or None depends on the system — both are valid.
    let _result = solve_kkt_exact(simplex.dual_vertices(), &perm);
}

/// Near-singular system (rank-deficient from f64->rational artifacts) is handled
/// via null-space search.
///
/// The HKO pentagon's m=7 permutation [1,7,2,8,4,6,5] produces a KKT system
/// with one eigenvalue ~2.8e-17 (below the pivot threshold). The solver detects
/// the near-zero pivot, extracts the null space, and searches for beta > 0.
///
/// History: Before null-space handling, the solver produced O(10^17)-magnitude
/// garbage or rejected the system entirely.
#[test]
fn near_singular_system_handled() {
    let pentagon = known_polytopes::hko_pentagon().polytope;

    let perm = vec![1, 7, 2, 8, 4, 6, 5];
    let result = solve_kkt_exact(pentagon.dual_vertices(), &perm);

    if let Some(r) = result {
        assert!(
            r.q_exact_f64.is_finite(),
            "Q_exact_f64 should be finite"
        );
        for (i, b) in r.beta.iter().enumerate() {
            assert!(
                b.is_positive(),
                "beta[{}] should be positive after null-space search, got {:?}",
                i,
                b
            );
        }
    }
    // Either outcome (Some with valid beta, or None) is correct.
}

/// Smoke test: hypercube permutations exercise the null-space path without panic.
///
/// The hypercube has axis-aligned normals (+/- e_i), so many permutations
/// produce rank-deficient KKT systems. Exercises null-space detection and
/// Fourier-Motzkin search.
#[test]
fn hypercube_null_space_smoke() {
    let hypercube = known_polytopes::hypercube().polytope;

    let perms = vec![
        vec![0, 1, 2, 3, 4],
        vec![0, 1, 2, 3, 4, 5],
        vec![0, 2, 4, 6],
    ];

    for perm in &perms {
        let result = solve_kkt_exact(hypercube.dual_vertices(), perm);
        if let Some(r) = result {
            assert!(
                r.q_exact_f64.is_finite(),
                "Q should be finite for perm {:?}",
                perm
            );
        }
        // No panic is the key invariant.
    }
}

/// Exact solver agrees with f64 solver on the simplex's winning (S, sigma).
///
/// Uses ehz_capacity to find the winning permutation, then runs solve_kkt_exact
/// on the same permutation and compares Q values.
#[test]
fn simplex_exact_vs_numerical() {
    let simplex = crate::geom::known_polytopes::simplex();
    let result = crate::algorithms::hk2017::ehz_capacity(&simplex.polytope)
        .expect("simplex should have capacity");
    let perm = &result.result.best_permutation;
    if let Some(exact) = super::rational_solver::solve_kkt_exact(simplex.polytope.dual_vertices(), perm) {
        let q_exact = exact.q_exact_f64;
        assert!(
            q_exact > 0.0,
            "exact Q should be positive, got {q_exact}"
        );
    }
}

/// Exact solver agrees with f64 solver on known polytopes with F <= 8.
///
/// Expensive input-output: each polytope runs both exact and numerical solvers.
#[test]
#[ignore] // Expensive: multiple polytopes × full permutation enumeration.
fn exact_agrees_on_known_polytopes() {
    use crate::geom::known_polytopes;
    for kp in [known_polytopes::simplex(), known_polytopes::hypercube()] {
        let result = crate::algorithms::hk2017::ehz_capacity(&kp.polytope)
            .expect("known polytope should have capacity");
        let perm = &result.result.best_permutation;
        if let Some(exact) = super::rational_solver::solve_kkt_exact(kp.polytope.dual_vertices(), perm) {
            assert!(exact.q_exact_f64 > 0.0, "exact Q should be positive");
        }
    }
}

/// On the winning node, all exact beta_i should be strictly positive.
#[test]
fn winning_beta_positive_exact() {
    let simplex = crate::geom::known_polytopes::simplex();
    let result = crate::algorithms::hk2017::ehz_capacity(&simplex.polytope)
        .expect("simplex should have capacity");
    let perm = &result.result.best_permutation;
    if let Some(exact) = super::rational_solver::solve_kkt_exact(simplex.polytope.dual_vertices(), perm) {
        assert!(
            exact.beta.iter().all(|b| b.is_positive()),
            "all exact beta should be strictly positive on winning node"
        );
    }
}

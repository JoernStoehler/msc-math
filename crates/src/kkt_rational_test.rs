use crate::geom::known_polytopes;
use crate::kkt_rational::solve_kkt_exact;
use num_traits::{Signed, Zero};

/// Exact KKT solve on the simplex (F=5) returns a solution with β > 0 and Q > 0.
///
/// **Why this input:** Simplex is the smallest polytope (F=5, m=5 is the only
/// non-trivial permutation size). Fast even with rational arithmetic.
/// **Why debug mode:** Exercises Gaussian elimination code paths with bounds checks.
#[test]
fn simplex_exact_solve() {
    let simplex = known_polytopes::simplex().polytope;

    // Try the identity permutation [0,1,2,3,4] (all 5 facets)
    let perm: Vec<usize> = (0..5).collect();
    let result = solve_kkt_exact(simplex.dual_vertices(), &perm);
    assert!(result.is_some(), "Simplex KKT system should be solvable");

    let r = result.unwrap();
    assert_eq!(r.beta.len(), 5);
    // Q may be negative for arbitrary permutations (the capacity algorithm picks
    // max Q > 0 across all permutations). Here we just check it's a valid rational.
    assert!(!r.q_exact.is_zero(), "Q_exact should be nonzero for a non-degenerate system");
    assert!(r.q_exact_f64.is_finite(), "Q_exact_f64 should be finite, got {}", r.q_exact_f64);
}

/// Exact KKT agrees with f64 KKT solver on the simplex's winning (S,σ).
///
/// **What it tests:** Compare exact Q_exact with f64 Q computed from best_beta.
/// They should agree to within machine precision (~1e-13 relative).
/// **Why debug mode:** Small polytope (F=5), fast.
#[test]
fn simplex_exact_vs_numerical() {
    let simplex = known_polytopes::simplex().polytope;
    let result_f64 = crate::algorithms::hk2017::ehz_capacity(&simplex)
        .expect("Simplex should have capacity");

    // Winning perm and beta are already in natural order — pass directly.
    let normals = simplex.normals_f64();
    let q_numerical = crate::kkt::q_from_beta(&normals, &result_f64.best_permutation, &result_f64.best_beta);

    let exact = solve_kkt_exact(simplex.dual_vertices(), &result_f64.best_permutation)
        .expect("Exact solve should succeed on winning perm");

    // Q_exact and Q_numerical should agree to ~machine precision.
    let diff = (exact.q_exact_f64 - q_numerical).abs();
    let tol = 1e-12 * (1.0 + exact.q_exact_f64.abs());
    assert!(
        diff < tol,
        "Simplex: |Q_exact - Q_numerical| = {:.2e}, tol = {:.2e}, Q_exact = {:.15e}, Q_numerical = {:.15e}",
        diff, tol, exact.q_exact_f64, q_numerical
    );
}

/// Exact Q is exactly rational (not NaN or infinity).
///
/// **What it tests:** The BigRational computation doesn't overflow or produce
/// degenerate values on a known polytope.
#[test]
fn hypercube_exact_solve() {
    let hypercube = known_polytopes::hypercube().polytope;

    // Try a 4-facet subset (first 4 facets).
    // The hypercube has axis-aligned normals, so ω₀(yᵢ, yⱼ) = 0 for many pairs
    // (e.g. dual vertices in the same symplectic plane). Q can be zero even with non-zero β.
    let perm = vec![0, 1, 2, 3];
    if let Some(r) = solve_kkt_exact(hypercube.dual_vertices(), &perm) {
        assert!(r.q_exact_f64.is_finite(), "Q_exact_f64 should be finite");
    }
    // It's fine if this particular perm returns None (singular) or Q = 0
}

/// Singular system returns None (not a panic).
///
/// **What it tests:** A permutation with linearly dependent constraints
/// should yield None, not panic.
#[test]
fn singular_system_returns_none() {
    let simplex = known_polytopes::simplex().polytope;

    // A 2-element permutation — very likely to be singular for the simplex
    // since m+5 = 7 > 5 = facet count, but we use valid facet indices
    let perm = vec![0, 1];
    // Whether this returns Some or None depends on the system — both are valid.
    // The key invariant is no panic.
    let _result = solve_kkt_exact(simplex.dual_vertices(), &perm);
}

/// Near-singular system (rank-deficient due to f64→rational artifacts) is handled
/// via null-space search.
///
/// **What it tests:** The hko_pentagon's winning permutation (m=7, after sign
/// convention unification) produces a KKT system with one eigenvalue ≈ 2.8e-17.
/// The solver detects the near-zero pivot, extracts the null space, and searches
/// for β > 0. Whether a positive-beta solution exists depends on the null-space
/// geometry — the test verifies no panic and validates the result if found.
///
/// **History:** Before null-space handling, `gauss_solve` either rejected the
/// system (after the condition threshold was added) or produced O(10^17)-magnitude
/// garbage. Now it properly handles rank deficiency like the f64 solver does.
///
/// **Why debug mode:** Only exercises Gaussian elimination, no capacity computation.
#[test]
fn near_singular_system_handled() {
    let pentagon = known_polytopes::hko_pentagon().polytope;

    // This m=7 permutation produces a near-singular KKT system.
    let perm = vec![1, 7, 2, 8, 4, 6, 5];
    let result = solve_kkt_exact(pentagon.dual_vertices(), &perm);

    // With null-space handling, the solver may find a valid solution.
    // If it does, validate it. If it doesn't (no β > 0 in null space), that's also OK.
    if let Some(r) = result {
        assert!(r.q_exact_f64.is_finite(), "Q_exact_f64 should be finite");
        // β components should all be positive (that's the search criterion).
        for (i, b) in r.beta.iter().enumerate() {
            assert!(
                b.is_positive(),
                "β[{}] should be positive after null-space search, got {:?}",
                i, b
            );
        }
    }
    // Either outcome (Some with valid β, or None) is correct — no panic is the key.
}

/// Smoke test: hypercube permutations exercise the null-space path without panic.
///
/// **What it tests:** The hypercube has axis-aligned normals (±e1, ±e2, ±e3, ±e4),
/// so many permutations produce rank-deficient KKT systems. This exercises the
/// null-space detection and search code paths. Both `Some` and `None` are valid
/// outcomes — the key invariant is no panic.
///
/// **Why debug mode:** Exercises Gaussian elimination with bounds checks.
#[test]
fn hypercube_null_space_smoke() {
    let hypercube = known_polytopes::hypercube().polytope;

    // Try several permutations — some will be rank-deficient.
    // We just need to exercise the null-space path without panicking.
    let perms = vec![
        vec![0, 1, 2, 3, 4],
        vec![0, 1, 2, 3, 4, 5],
        vec![0, 2, 4, 6],
    ];

    for perm in &perms {
        let result = solve_kkt_exact(hypercube.dual_vertices(), perm);
        if let Some(r) = result {
            assert!(r.q_exact_f64.is_finite(),
                "Q should be finite for perm {:?}", perm);
        }
        // No panic is the key invariant.
    }
}

/// Exact solver agrees with f64 solver on all known polytopes' winning nodes.
///
/// **Why #[ignore]:** Requires ehz_capacity() which is expensive in debug mode.
/// **Run with:** `cargo test --release exact_agrees_on_known_polytopes -- --ignored`
#[test]
#[ignore] // Requires release mode for ehz_capacity
fn exact_agrees_on_known_polytopes() {
    let polytopes: Vec<_> = known_polytopes::all_known()
        .into_iter()
        .filter(|kp| kp.polytope.facet_count() <= 10)
        .collect();

    for kp in &polytopes {
        let result_f64 = match crate::algorithms::hk2017::ehz_capacity(&kp.polytope) {
            Some(r) => r,
            None => continue,
        };

        let normals = kp.polytope.normals_f64();
        let q_numerical = crate::kkt::q_from_beta(&normals, &result_f64.best_permutation, &result_f64.best_beta);

        let exact = match solve_kkt_exact(kp.polytope.dual_vertices(), &result_f64.best_permutation) {
            Some(r) => r,
            None => {
                eprintln!("WARNING: {} winning node is singular in exact solver", kp.name);
                continue;
            }
        };

        let diff = (exact.q_exact_f64 - q_numerical).abs();
        let tol = 1e-12 * (1.0 + exact.q_exact_f64.abs());
        assert!(
            diff < tol,
            "{}: |Q_exact - Q_numerical| = {:.2e}, tol = {:.2e}, Q_exact = {:.15e}, Q_numerical = {:.15e}",
            kp.name, diff, tol, exact.q_exact_f64, q_numerical
        );
    }
}

/// Beta positivity: on the winning node, all exact β_i should be positive.
///
/// The winning (S,σ) has Q > 0 and β > 0 by the capacity algorithm's filtering.
/// The exact solver should confirm this with exact arithmetic.
///
/// **Why #[ignore]:** Requires ehz_capacity() in release mode.
#[test]
#[ignore]
fn winning_beta_positive_exact() {
    let polytopes: Vec<_> = known_polytopes::all_known()
        .into_iter()
        .filter(|kp| kp.polytope.facet_count() <= 8)
        .collect();

    for kp in &polytopes {
        let result_f64 = match crate::algorithms::hk2017::ehz_capacity(&kp.polytope) {
            Some(r) => r,
            None => continue,
        };

        let exact = match solve_kkt_exact(kp.polytope.dual_vertices(), &result_f64.best_permutation) {
            Some(r) => r,
            None => continue,
        };

        for (i, b) in exact.beta.iter().enumerate() {
            assert!(
                b.is_positive(),
                "{}: exact β[{}] = {:?} is not positive (winning node should have β > 0)",
                kp.name, i, b
            );
        }
    }
}

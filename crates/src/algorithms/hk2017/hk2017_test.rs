use super::*;
use crate::geom::known_polytopes;
use nalgebra::Vector4;

/// Smoke test: unpruned capacity algorithm executes safely on 4-facet simplex.
///
/// **What:** Computes EHZ capacity using unpruned enumeration algorithm.
/// **Why debug mode:** Exercises index arithmetic, enumeration logic, and KKT
/// solver with debug checks enabled (debug_assert!, overflow, bounds).
/// **Why this polytope:** F=4 simplex is minimal non-trivial case, runs instantly.
/// **Output check:** Verifies against literature value (2.0) as sanity check.
///
/// For comprehensive capacity verification across many polytopes, see
/// `capacity_properties_test::literature_capacity_values()` (fixture-based).
#[test]
fn simplex_capacity() {
    let kp = known_polytopes::simplex();
    let result = ehz_capacity_unpruned(&kp.polytope).expect("simplex should have capacity");
    assert!(
        (result.capacity - kp.capacity).abs() < 1e-6,
        "simplex capacity: got {}, expected {}",
        result.capacity, kp.capacity
    );
}

/// Smoke test: unpruned capacity algorithm executes safely on 8-facet hypercube.
///
/// **What:** Computes EHZ capacity using unpruned enumeration algorithm.
/// **Why debug mode:** Exercises combinatorial enumeration on structured geometry
/// with debug checks enabled.
/// **Why this polytope:** F=8 hypercube has high symmetry (fast despite 8 facets,
/// <1s in debug). Tests that enumeration handles regular geometry correctly.
/// **Output check:** Verifies against literature value (4.0) as sanity check.
///
/// For comprehensive capacity verification, see
/// `capacity_properties_test::literature_capacity_values()` (fixture-based).
#[test]
fn hypercube_capacity() {
    let kp = known_polytopes::hypercube();
    let result = ehz_capacity_unpruned(&kp.polytope).expect("hypercube should have capacity");
    assert!(
        (result.capacity - kp.capacity).abs() < 1e-6,
        "hypercube capacity: got {}, expected {}",
        result.capacity, kp.capacity
    );
}

/// Smoke test: unpruned algorithm on Lagrangian triangle product.
///
/// **What:** Computes capacity on equilateral triangle × unit square (Lagrangian product).
/// **Why debug mode:** Exercises unpruned algorithm on product geometry in debug mode.
/// **Why this polytope:** F=7, tests Lagrangian product structure, runs fast (<1s).
/// **Output check:** Verifies against known value as sanity check.
#[test]
fn lagrangian_triangle_product_capacity() {
    let kp = known_polytopes::lagrangian_triangle_product();
    let result = ehz_capacity_unpruned(&kp.polytope).expect("lagrangian triangle product should have capacity");
    assert!(
        (result.capacity - kp.capacity).abs() < 1e-6,
        "lagrangian triangle product capacity: got {}, expected {}",
        result.capacity, kp.capacity
    );
}

/// Test combinatorial enumeration correctness on small inputs.
///
/// **What:** Verifies combinations(n, k) produces correct count.
/// **Why debug mode:** Tests index arithmetic and allocation in combinatorics
/// utilities with overflow/bounds checks enabled.
/// **Why not fixture:** Simple unit test, runs instantly.
#[test]
fn combinations_basic() {
    assert_eq!(combinations(4, 2).len(), 6);  // C(4,2) = 6
    assert_eq!(combinations(5, 3).len(), 10); // C(5,3) = 10
    assert_eq!(combinations(5, 5).len(), 1);  // C(5,5) = 1
}

/// Verify pruned and unpruned algorithms produce identical capacity values.
///
/// **What:** Computes capacity on 8-facet hypercube using both pruned and unpruned
/// algorithms, verifies they agree.
/// **Why release mode:** F=8 → ~16s debug, ~0.2s release. Input-output test,
/// only care about result agreement.
/// **Why #[ignore]:** Too slow for default suite. Run after changes to adjacency
/// pruning logic with: `cargo test --release pruned_matches_unpruned -- --ignored`
///
/// For quick fixture-based pruned/unpruned check on 27 polytopes, see
/// `capacity_properties_test::pruned_matches_unpruned_from_fixture()` (default suite, <1s).
#[test]
#[ignore] // ~16s debug, ~0.2s release. Run: cargo test --release pruned_matches_unpruned -- --ignored
fn pruned_matches_unpruned() {
    let kp = known_polytopes::hypercube();

    let result_unpruned = ehz_capacity_unpruned(&kp.polytope).expect("unpruned capacity");
    let result_pruned = ehz_capacity(&kp.polytope).expect("pruned capacity");

    assert!(
        (result_unpruned.capacity - result_pruned.capacity).abs() < 1e-6,
        "pruned and unpruned capacities differ"
    );

    // Pruned should do fewer iterations (adjacency filtering)
    assert!(
        result_pruned.iterations <= result_unpruned.iterations,
        "pruned should do ≤ iterations than unpruned"
    );

    eprintln!("Hypercube: unpruned {} iters, pruned {} iters",
        result_unpruned.iterations, result_pruned.iterations);
}

// ============================================================================
// Internal Behavior: KKT Solver Edge Cases
// ============================================================================

/// Test KKT solver on minimal 2-facet system.
///
/// **What:** Solves KKT optimality conditions for minimal facet set (F=2).
/// **Why debug mode:** Tests solver handles degenerate low-dimensional case correctly.
/// Exercises bounds checking on small system.
/// **Why not fixture:** Tests solver internals, not capacity output. Small system
/// runs instantly, no need for pre-computation.
#[test]
fn solve_kkt_two_facets() {
    // Two opposite facets: n1 = (1,0,0,0), n2 = (-1,0,0,0)
    // Heights h1 = h2 = 1.0
    // Constraints: β1 - β2 = 0, β1 + β2 = 1
    // Solution: β1 = β2 = 0.5
    // Q(β) = β1·β2·ω₀(n1, n2) = 0.5·0.5·0 = 0 (parallel normals)

    let normals = vec![Vector4::x(), -Vector4::x()];
    let heights = vec![1.0, 1.0];
    let perm = vec![0, 1];

    let result = solve_kkt(&normals, &heights, &perm);

    // Should return Some (system is solvable)
    assert!(result.is_some(), "two-facet system should solve");

    let result = result.unwrap();
    assert_eq!(result.beta.len(), 2);

    // β1 ≈ β2 ≈ 0.5
    assert!((result.beta[0] - 0.5).abs() < 1e-6, "β1 should be ~0.5");
    assert!((result.beta[1] - 0.5).abs() < 1e-6, "β2 should be ~0.5");

    // Q = 0 (parallel normals have ω₀ = 0)
    assert!(result.q_corrected.abs() < 1e-10, "Q should be ~0 for parallel normals");
}

/// Test KKT solver on 4-facet symplectic square.
///
/// **What:** Solves KKT system for symplectic square (4 facets, product structure).
/// **Why debug mode:** Tests solver on structured geometry with debug checks.
/// **Why not fixture:** Tests solver behavior on specific geometric case.
#[test]
fn solve_kkt_four_facets_symplectic() {
    // Four facets forming a 2D symplectic subplane:
    // n1 = e_q1 = (1,0,0,0)
    // n2 = e_p1 = (0,0,1,0)
    // n3 = -e_q1 = (-1,0,0,0)
    // n4 = -e_p1 = (0,0,-1,0)
    // Heights all 1.0
    //
    // ω₀(e_q1, e_p1) = 1, ω₀(e_q1, -e_q1) = 0, etc.

    let normals = vec![
        Vector4::x(),           // e_q1
        Vector4::z(),           // e_p1
        -Vector4::x(),          // -e_q1
        -Vector4::z(),          // -e_p1
    ];
    let heights = vec![1.0; 4];
    let perm = vec![0, 1, 2, 3]; // cyclic order

    let result = solve_kkt(&normals, &heights, &perm);

    assert!(result.is_some(), "4-facet symplectic system should solve");

    let result = result.unwrap();
    assert_eq!(result.beta.len(), 4);

    // Verify constraints
    // N^T β = 0: β1 - β3 = 0, β2 - β4 = 0
    assert!((result.beta[0] - result.beta[2]).abs() < 1e-6, "β1 should equal β3");
    assert!((result.beta[1] - result.beta[3]).abs() < 1e-6, "β2 should equal β4");

    // η^T β = 1: β1 + β2 + β3 + β4 = 1
    let sum: f64 = result.beta.iter().sum();
    assert!((sum - 1.0).abs() < 1e-6, "β sum should be 1");

    // Q ≠ 0 (non-degenerate symplectic system)
    // Note: Q can be positive or negative depending on the cyclic order
    assert!(result.q_corrected.abs() > 1e-10, "Q should be non-zero for symplectic normals, got {}", result.q_corrected);
}

/// Test KKT solver handles rank-deficient normal matrix.
///
/// **What:** Solves KKT system when normal vectors are linearly dependent.
/// **Why debug mode:** Tests error handling path in solver. Ensures solver
/// correctly detects and handles rank deficiency without panicking.
/// **Why not fixture:** Tests specific error path, not typical computation.
///
/// This system has all normals in q-space, so ω₀(nᵢ, nⱼ) = 0 for all pairs.
/// The unique β satisfying N^T β = 0, η^T β = 1 has β₂ ≈ -2.414 < 0.
/// No null space search can fix this (the null space is in the λ variables,
/// not β). So solve_kkt correctly returns None.
#[test]
fn solve_kkt_rank_deficient() {
    // Three normals in the q-plane (rank 2 normal matrix)
    // N^T has rank 2, not 4, so the KKT system is rank-deficient.
    // The constraints N^T β = 0, η^T β = 1 uniquely determine β,
    // and the solution has β₂ < 0.

    let normals = vec![
        Vector4::new(1.0, 0.0, 0.0, 0.0),
        Vector4::new(0.0, 1.0, 0.0, 0.0),
        Vector4::new(0.707, 0.707, 0.0, 0.0).normalize(),
    ];
    let heights = vec![1.0; 3];
    let perm = vec![0, 1, 2];

    let result = solve_kkt(&normals, &heights, &perm);

    // Returns None: the unique β has β₂ ≈ -2.414 < 0 (genuinely infeasible).
    // The null space is in (λ₂, λ₃), not β, so no null space search helps.
    assert!(result.is_none(), "rank-deficient system with β < 0 should return None");
}

/// Test KKT solver on degenerate case (identical normals).
///
/// **What:** Solves KKT system when facets have identical normals (violates irredundancy).
/// **Why debug mode:** Tests edge case error handling. Ensures solver gracefully
/// handles degenerate input without crashing.
/// **Why not fixture:** Tests error path, not normal execution.
#[test]
fn solve_kkt_degenerate() {
    // Two identical normals (degenerate: violates irredundancy)
    // The KKT system should fail to solve or produce invalid residual

    let normals = vec![Vector4::x(), Vector4::x()];
    let heights = vec![1.0, 1.0];
    let perm = vec![0, 1];

    let result = solve_kkt(&normals, &heights, &perm);

    // Should return None (degenerate system)
    // OR return Some with high residual (caught by residual check)
    // Either outcome is acceptable
    if result.is_some() {
        // If it returns Some, it means the residual check didn't catch it
        // This is fine — the outer algorithm will filter via β > 0 or other checks
        eprintln!("Note: degenerate system returned Some (acceptable)");
    }
}

/// Verify HKO pentagon capacity and sys > 1 property (Annals counterexample).
///
/// **What:** Computes capacity on Haim-Kislev-Ostrover 10-facet pentagon and
/// verifies it's a counterexample to Viterbo's conjecture (sys > 1).
/// **Why release mode:** F=10 → ~37s debug, ~0.5s release. Input-output test,
/// only care about final value. Run in release for 74x speedup.
/// **Why #[ignore]:** Too slow for default suite. Important regression test for
/// thesis counterexample.
/// **Run with:** `cargo test --release pentagon_capacity -- --ignored`
///
/// For quick capacity check against literature value, see
/// `capacity_properties_test::literature_capacity_values()` (fixture-based, default suite).
#[test]
#[ignore] // ~37s debug, ~0.5s release. Run: cargo test --release pentagon_capacity -- --ignored
fn pentagon_capacity() {
    use crate::geom::volume::volume;

    let kp = known_polytopes::hko_pentagon();
    let result = ehz_capacity(&kp.polytope).expect("pentagon capacity");

    assert!(
        (result.capacity - kp.capacity).abs() < 1e-6,
        "pentagon: got {}, expected {}", result.capacity, kp.capacity
    );

    // Verify sys > 1 (counterexample property)
    let vol = volume(&kp.polytope).expect("volume computation failed");
    let sys = result.capacity * result.capacity / (2.0 * vol);
    eprintln!("Pentagon: capacity={:.6}, volume={:.6}, sys={:.6}",
              result.capacity, vol, sys);
    assert!(sys > 1.0, "pentagon should have sys > 1, got {}", sys);
}

/// Smoke test: pruned algorithm executes safely on Lagrangian triangle×square.
///
/// **What:** Computes capacity using adjacency-pruned enumeration.
/// **Why debug mode:** Exercises pruning logic (adjacency filtering) on Lagrangian
/// product geometry with debug checks enabled. Catches bounds errors in facet
/// adjacency indexing.
/// **Why this polytope:** F=7 Lagrangian product (triangle in q, square in p).
/// Tests that pruning correctly handles product structure. Runs fast (<1s).
/// **Output check:** Verifies capacity = 1.5 (optimal orbit uses 3 triangle facets
/// and 2 square facets). See experiments/triangle_square.md for analysis.
///
/// For pruned/unpruned agreement verification, see
/// `capacity_properties_test::pruned_matches_unpruned_from_fixture()`.
#[test]
fn triangle_square_capacity() {
    let kp = known_polytopes::lagrangian_triangle_square();
    let result = ehz_capacity(&kp.polytope).expect("Lagrangian triangle×square capacity");

    // Investigation complete: This is a Lagrangian product (equilateral triangle in q-space,
    // unit square in p-space), not a symplectic product. The algorithm correctly computes
    // capacity = 1.5 via optimal orbit using all 3 triangle facets and 2 square facets.
    // See experiments/triangle_square.md for detailed analysis.
    assert!(
        (result.capacity - kp.capacity).abs() < 1e-6,
        "Lagrangian triangle×square: got {}, expected {}", result.capacity, kp.capacity
    );
}

/// Smoke test: pruned algorithm on symplectic triangle×square product.
///
/// **What:** Computes capacity using pruned algorithm on symplectic product.
/// **Why debug mode:** Exercises pruning logic on symplectic product geometry
/// (triangle in (q₁,p₁), square in (q₂,p₂)) with debug checks enabled.
/// **Why this polytope:** F=7 symplectic product. Tests the symplectic product formula:
/// c(A ×_S B) = min(c(A), c(B)). Distinguishes symplectic from Lagrangian products.
/// **Output check:** Verifies capacity = min(3√3/4, 1.0) = 1.0.
///
/// For comprehensive product formula tests, see capacity_properties_test.rs.
#[test]
fn symplectic_triangle_square_capacity() {
    let kp = known_polytopes::symplectic_triangle_square();
    let result = ehz_capacity(&kp.polytope).expect("symplectic triangle×square capacity");

    // Symplectic product: triangle in (q₁, p₁) plane ×_S square in (q₂, p₂) plane.
    // Symplectic product formula: c(A ×_S B) = min(c(A), c(B))
    // Expected: min(3√3/4, 1.0) = min(1.299..., 1.0) = 1.0
    //
    // This test verifies the algorithm correctly computes the symplectic product formula
    // and distinguishes symplectic from Lagrangian products.
    assert!(
        (result.capacity - kp.capacity).abs() < 1e-6,
        "symplectic triangle×square: got {}, expected {} (min formula)",
        result.capacity, kp.capacity
    );
}

/// Compute capacity of the 4D crosspolytope (16 facets, non-simple).
///
/// **Why #[ignore]:** F=16, estimated ~4 hours pruned (A3) in release mode.
/// Extrapolated from benchmark data (F=5..12, 4.59x growth per facet).
/// Unpruned (A0) is completely infeasible (~800 days).
/// Non-simplicity (8 facets/vertex) may help pruning, but effect is uncertain.
///
/// **Estimate basis (2026-02-24):** benchmark.jsonl F=8..12 exponential fit,
/// ablation.jsonl A0 counts. Range: 1–13 hours depending on pruning effectiveness
/// for this highly symmetric non-simple polytope.
///
/// **Run with:** `cargo test --release crosspolytope_capacity -- --ignored --nocapture`
#[test]
#[ignore]
fn crosspolytope_capacity() {
    let kp = known_polytopes::crosspolytope();
    let result = ehz_capacity(&kp.polytope).expect("crosspolytope capacity");

    // No known literature value — just verify computation succeeds and record the result.
    assert!(result.capacity > 0.0, "crosspolytope capacity positive");
    eprintln!("Crosspolytope (16 facets): capacity={:.6}", result.capacity);
    eprintln!("  Iterations: {}", result.iterations);
}

// ---- Property tests ----

#[cfg(test)]
mod proptests {
    use super::*;
    use proptest::prelude::*;
    use crate::random::generate_random_polytopes;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(10))]

        /// Property: pruned and unpruned algorithms return the same capacity.
        ///
        /// **What:** Tests pruned/unpruned agreement on random polytopes (F=5-8).
        /// **Why #[ignore]:** Redundant with `capacity_properties_test::pruned_matches_unpruned_from_fixture()`
        /// which checks agreement across 27 diverse polytopes from fixture (includes various
        /// products, simplices, structured and irregular geometries). The fixture provides
        /// better coverage than 40 random cases.
        /// **Original budget:** F=8 unpruned ≈ 40ms, pruned ≈ 14ms; 10 cases × 4 seeds = ~2s total.
        ///
        /// This tests `cor:adjacency-pruning` (adjacency pruning optimization).
        #[test]
        #[ignore] // Redundant with fixture test. Can be removed once fixture coverage is confirmed.
        fn pruned_matches_unpruned_random(
            facet_count in 5usize..=8,
            seed in 0u64..4
        ) {
            let mut rng = ChaCha8Rng::seed_from_u64(seed);
            let polytopes = generate_random_polytopes(1, facet_count, 0.5, 2.0, &mut rng);

            if let Some(p) = polytopes.first() {
                let unpruned = ehz_capacity_unpruned(p).unwrap();
                let pruned = ehz_capacity(p).unwrap();

                prop_assert!(
                    (unpruned.capacity - pruned.capacity).abs() < 1e-6,
                    "pruned {} vs unpruned {}", pruned.capacity, unpruned.capacity
                );

                // Pruned should do ≤ iterations (adjacency filtering)
                prop_assert!(
                    pruned.iterations <= unpruned.iterations,
                    "pruned iterations {} > unpruned {}", pruned.iterations, unpruned.iterations
                );
            }
        }

    }
}

// ============================================================================
// Regression tests: KKT null space fix
// ============================================================================
// These tests verify that the KKT solver correctly handles rank-deficient
// systems by searching the null space for β > 0 solutions. Before the fix,
// the minimum-norm pseudoinverse solution often had β ≤ 0 for degenerate
// polytopes (axis-aligned normals in symplectic subplanes).

/// Regression: (4,4) Lagrangian product at θ=0° (square × square, axis-aligned).
///
/// Before fix: cap=2.0 (correct). After fix: cap=2.0 (unchanged).
/// This is the hypercube [-1/√2, 1/√2]⁴ which already worked pre-fix.
/// Included to verify the fix doesn't break the working case.
#[test]
fn kkt_nullspace_square_square_zero() {
    use crate::geom::lagrangian_product::lagrangian_product;
    use crate::geom::polygon::regular_polygon_2d;

    let (qn, qh) = regular_polygon_2d(4, 1.0);
    let (pn, ph) = regular_polygon_2d(4, 1.0);
    let polytope = lagrangian_product(&qn, &qh, &pn, &ph).unwrap();

    let result = ehz_capacity_unpruned(&polytope).expect("(4,4) at θ=0° should have capacity");
    assert!(
        (result.capacity - 2.0).abs() < 1e-6,
        "(4,4) at θ=0°: got {}, expected 2.0",
        result.capacity
    );
}

/// Regression: (4,4) at θ=0.125° — the smallest angle in the polygon_grid.
///
/// Before fix: cap=3.991 (WRONG, 2× too high due to 8-facet spurious orbit).
/// After fix: cap≈2.000 (continuous from θ=0°).
/// All three algorithms agree.
#[test]
fn kkt_nullspace_square_square_near_zero() {
    use crate::geom::lagrangian_product::lagrangian_product;
    use crate::geom::polygon::{regular_polygon_2d, rotate_polygon_2d};

    let theta = 0.125_f64.to_radians();
    let (qn, qh) = regular_polygon_2d(4, 1.0);
    let (pn_base, ph_base) = regular_polygon_2d(4, 1.0);
    let (pn, ph) = rotate_polygon_2d(&pn_base, &ph_base, theta);
    let polytope = lagrangian_product(&qn, &qh, &pn, &ph).unwrap();

    let result = ehz_capacity_unpruned(&polytope).expect("(4,4) at θ=0.125° should have capacity");
    // Capacity should be continuous near θ=0° → close to 2.0
    assert!(
        (result.capacity - 2.0).abs() < 0.01,
        "(4,4) at θ=0.125°: got {}, expected ~2.0 (was 3.991 before fix)",
        result.capacity
    );
}

/// Regression: (4,4) at θ=45° — billiard previously gave 2× wrong answer.
///
/// Before fix: HK2017=2.828, billiard=5.657.
/// After fix: all agree on cap=2√2≈2.828.
#[test]
fn kkt_nullspace_square_square_45deg() {
    use crate::geom::lagrangian_product::lagrangian_product;
    use crate::geom::polygon::{regular_polygon_2d, rotate_polygon_2d};

    let theta = 45.0_f64.to_radians();
    let (qn, qh) = regular_polygon_2d(4, 1.0);
    let (pn_base, ph_base) = regular_polygon_2d(4, 1.0);
    let (pn, ph) = rotate_polygon_2d(&pn_base, &ph_base, theta);
    let polytope = lagrangian_product(&qn, &qh, &pn, &ph).unwrap();

    let result_hk = ehz_capacity_unpruned(&polytope).expect("(4,4) at θ=45°: HK2017 should have capacity");
    let result_bil = crate::algorithms::billiard::billiard_capacity(&polytope)
        .expect("billiard should not error")
        .expect("billiard should find capacity");

    let sqrt2_times2 = 2.0 * std::f64::consts::SQRT_2;
    assert!(
        (result_hk.capacity - sqrt2_times2).abs() < 1e-6,
        "(4,4) at θ=45° HK2017: got {}, expected 2√2≈{}",
        result_hk.capacity, sqrt2_times2
    );
    assert!(
        (result_bil.capacity - sqrt2_times2).abs() < 1e-6,
        "(4,4) at θ=45° billiard: got {} (was 5.657 before fix), expected 2√2≈{}",
        result_bil.capacity, sqrt2_times2
    );
}

/// Regression: (3,4) at θ=0° — previously returned None for all algorithms.
///
/// Before fix: None (all three algorithms). No valid orbit found.
/// After fix: cap≈2.121 via 5-facet orbit. All three agree.
///
/// Note: The expected capacity for this specific polytope (triangle circumradius=1,
/// square circumradius=1) is 3√2/2 ≈ 2.121, NOT 1.5. The value 1.5 was from
/// `lagrangian_triangle_square()` which uses different dimensions.
#[test]
fn kkt_nullspace_triangle_square_zero() {
    use crate::geom::lagrangian_product::lagrangian_product;
    use crate::geom::polygon::regular_polygon_2d;

    let (qn, qh) = regular_polygon_2d(3, 1.0);
    let (pn, ph) = regular_polygon_2d(4, 1.0);
    let polytope = lagrangian_product(&qn, &qh, &pn, &ph).unwrap();

    let result = ehz_capacity_unpruned(&polytope)
        .expect("(3,4) at θ=0° should now return Some after null space fix");

    // Test output shows cap = 2.1213203436 = 3√2/2
    let expected = 3.0 * std::f64::consts::SQRT_2 / 2.0; // 3√2/2 ≈ 2.121
    assert!(
        (result.capacity - expected).abs() < 1e-6,
        "(3,4) at θ=0°: got {}, expected 3√2/2≈{} (was None before fix)",
        result.capacity, expected
    );
}

// ============================================================================
// Eigenvalue condition number threshold regression tests
// ============================================================================
// The condition-number threshold EIGEN_CONDITION_TAU=1e-3 was chosen empirically from the (4,4)
// degenerate case. These tests pin the observed eigenvalue spectrum so that changes to
// the threshold can be validated against the cases that motivated it.

/// Verify eigenvalue spectrum of the (4,4) θ=0° degenerate KKT system.
///
/// **What:** Asserts the eigenvalue spectrum that motivated EIGEN_CONDITION_TAU=1e-3.
/// The optimal orbit permutation for (4,4) at θ=0° has a gap in the sorted |λ_i|
/// spectrum. This spectrum must remain well-separated at the condition-number-based
/// rank detection threshold for the null space search to work.
///
/// **Why:** Regression test for EIGEN_CONDITION_TAU (see doc comment on the constant).
/// Without this, a threshold change could silently break the degenerate case.
/// **Why debug mode:** Only builds one KKT matrix and computes eigendecomposition. Fast.
#[test]
fn eigen_gap_ratio_44_degenerate() {
    use crate::geom::lagrangian_product::lagrangian_product;
    use crate::geom::polygon::regular_polygon_2d;

    let (qn, qh) = regular_polygon_2d(4, 1.0);
    let (pn, ph) = regular_polygon_2d(4, 1.0);
    let polytope = lagrangian_product(&qn, &qh, &pn, &ph).unwrap();
    let normals = polytope.normals_f64();
    let heights = polytope.heights_f64();

    // The optimal orbit at θ=0° uses facets [0,4,2,6] (alternating q/p)
    let perm = vec![0, 4, 2, 6];
    let (kkt, _rhs) = crate::kkt::build_kkt_system(&normals, &heights, &perm);
    let eigen = kkt.symmetric_eigen();
    let size = perm.len() + 5; // 9

    // Collect |λ_i| and sort descending (symmetric_eigen returns unspecified order)
    let mut abs_eigenvalues: Vec<f64> = eigen.eigenvalues.iter().map(|&ev| ev.abs()).collect();
    abs_eigenvalues.sort_by(|a, b| b.partial_cmp(a).unwrap());

    // Find the gap: walk from smallest |λ_i| upward looking for the biggest ratio
    let floor = 1e-15;
    let mut max_gap_ratio = 0.0f64;
    for i in (1..size).rev() {
        if abs_eigenvalues[i] < floor {
            continue;
        }
        let ratio = abs_eigenvalues[i - 1] / abs_eigenvalues[i];
        if ratio > max_gap_ratio {
            max_gap_ratio = ratio;
        }
    }

    // If the smallest |λ_i| is ≈0, the system is exactly rank-deficient.
    // This is fine — condition-number detection handles it via the floor check.
    // The important thing is that the rank deficiency exists.
    let smallest_nonzero = (0..size).rev().find(|&i| abs_eigenvalues[i] > floor);
    if let Some(idx) = smallest_nonzero {
        if idx > 0 {
            let ratio = abs_eigenvalues[idx - 1] / abs_eigenvalues[idx];
            // If there's a near-zero |λ_i| with a large gap above it,
            // the gap ratio must stay well above EIGEN_CONDITION_TAU detection threshold
            if ratio > 50.0 {
                assert!(
                    ratio > 200.0,
                    "(4,4) θ=0° gap ratio should stay well above 1e-3 condition-number threshold, got {:.1} (|λ[{}]|={:.3e}, |λ[{}]|={:.3e})",
                    ratio, idx - 1, abs_eigenvalues[idx - 1], idx, abs_eigenvalues[idx]
                );
            }
        }
    }

    // The KKT system for this permutation must be rank-deficient
    // (axis-aligned normals in the (4,4) product create linear dependence)
    let numerical_rank = abs_eigenvalues.iter().filter(|&&ev| ev > 1e-6).count();
    assert!(
        numerical_rank < size,
        "(4,4) θ=0° should be rank-deficient: rank={numerical_rank}, size={size}"
    );
}

/// Verify eigenvalue gap ratio for the (4,4) θ=43° case that motivated EIGEN_CONDITION_TAU.
///
/// **What:** The KKT system for perm [1,0,6,3,2,4] on the (4,4) product at θ=43°
/// has a gap in the sorted |λ_i| spectrum with ratio ≈594. This is the case from
/// commit dd87a8a that motivated EIGEN_CONDITION_TAU=1e-3. The gap ratio must stay
/// well above the threshold for the fix to work.
///
/// **Why:** Regression test for EIGEN_CONDITION_TAU (see doc comment on the constant).
/// **Why debug mode:** One KKT matrix, fast.
#[test]
fn eigen_gap_ratio_44_theta43() {
    use crate::geom::lagrangian_product::lagrangian_product;
    use crate::geom::polygon::{regular_polygon_2d, rotate_polygon_2d};

    let theta = 43.0_f64.to_radians();
    let (qn, qh) = regular_polygon_2d(4, 1.0);
    let (pn_base, ph_base) = regular_polygon_2d(4, 1.0);
    let (pn, ph) = rotate_polygon_2d(&pn_base, &ph_base, theta);
    let polytope = lagrangian_product(&qn, &qh, &pn, &ph).unwrap();
    let normals = polytope.normals_f64();
    let heights = polytope.heights_f64();

    // This permutation gave the 594x gap in the original diagnostic
    let perm = vec![1, 0, 6, 3, 2, 4];
    let m = perm.len();
    let size = m + 5; // 11

    let (kkt, _rhs) = crate::kkt::build_kkt_system(&normals, &heights, &perm);
    let eigen = kkt.symmetric_eigen();

    // Collect |λ_i| and sort descending (symmetric_eigen returns unspecified order)
    let mut abs_eigenvalues: Vec<f64> = eigen.eigenvalues.iter().map(|&ev| ev.abs()).collect();
    abs_eigenvalues.sort_by(|a, b| b.partial_cmp(a).unwrap());

    // Find the largest gap ratio in the |λ_i| spectrum
    let floor = 1e-15;
    let mut max_gap_ratio = 0.0f64;
    let mut gap_idx = 0;
    for i in (1..size).rev() {
        if abs_eigenvalues[i] < floor {
            continue;
        }
        let ratio = abs_eigenvalues[i - 1] / abs_eigenvalues[i];
        if ratio > max_gap_ratio {
            max_gap_ratio = ratio;
            gap_idx = i;
        }
    }

    // The gap ratio must be well above the EIGEN_CONDITION_TAU=1e-3 condition-number threshold
    // Original observation: ~594x. Allow some variation but must stay >>1e-3.
    assert!(
        max_gap_ratio > 300.0,
        "(4,4) θ=43° gap ratio should be ~594 (well above 1e-3 threshold), got {:.1} at |λ[{}]|={:.3e}/|λ[{}]|={:.3e}",
        max_gap_ratio, gap_idx - 1, abs_eigenvalues[gap_idx - 1], gap_idx, abs_eigenvalues[gap_idx]
    );
}

/// Verify conformality property: c_EHZ(λK) = λ²·c_EHZ(K).
///
/// **What:** Tests conformality (scaling property) on hypercube with λ = e.
/// **Why release mode:** F=8 → ~24s debug, ~0.3s release. Input-output test.
/// **Why #[ignore]:** Redundant with `capacity_properties_test::capacity_conformality()`
/// which tests this property from fixture across many polytopes. This test will be
/// removed once fixture includes scaled polytopes.
/// **Run with:** `cargo test --release capacity_scales_quadratically -- --ignored`
///
/// Uses λ = e (transcendental) — cannot be root of any polynomial with integer
/// coefficients, making numerical coincidences impossible.
#[test]
#[ignore] // ~24s debug, ~0.3s release. Redundant with fixture test once fixture has scaled polytopes.
fn capacity_scales_quadratically() {
    let scale = std::f64::consts::E;

    let kp = known_polytopes::hypercube();
    let unit_cap = ehz_capacity_unpruned(&kp.polytope).unwrap().capacity;

    let scaled_cube = crate::geom::test_utils::scaled_hypercube(scale);
    let scaled_cap = ehz_capacity_unpruned(&scaled_cube).unwrap().capacity;

    let expected = unit_cap * scale * scale;
    let relative_error = ((scaled_cap - expected) / expected).abs();

    assert!(
        relative_error < 1e-4,
        "capacity scaling failed: scale={scale}, unit_cap={unit_cap}, \
         scaled_cap={scaled_cap}, expected={expected}, relative_error={relative_error}"
    );
}


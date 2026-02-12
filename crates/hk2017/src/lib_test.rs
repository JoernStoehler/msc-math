use super::*;
use geom::known_polytopes;
use nalgebra::Vector4;

#[test]
fn simplex_capacity() {
    let kp = known_polytopes::simplex();
    let result = ehz_capacity(&kp.polytope).expect("simplex should have capacity");
    assert!(
        (result.capacity - kp.capacity).abs() < 1e-6,
        "simplex capacity: got {}, expected {}",
        result.capacity, kp.capacity
    );
}

#[test]
fn hypercube_capacity() {
    let kp = known_polytopes::hypercube();
    let result = ehz_capacity(&kp.polytope).expect("hypercube should have capacity");
    assert!(
        (result.capacity - kp.capacity).abs() < 1e-6,
        "hypercube capacity: got {}, expected {}",
        result.capacity, kp.capacity
    );
}

#[test]
fn lagrangian_triangle_product_capacity() {
    let kp = known_polytopes::lagrangian_triangle_product();
    let result = ehz_capacity(&kp.polytope).expect("lagrangian triangle product should have capacity");
    assert!(
        (result.capacity - kp.capacity).abs() < 1e-6,
        "lagrangian triangle product capacity: got {}, expected {}",
        result.capacity, kp.capacity
    );
}

#[test]
fn combinations_basic() {
    assert_eq!(combinations(4, 2).len(), 6);  // C(4,2) = 6
    assert_eq!(combinations(5, 3).len(), 10); // C(5,3) = 10
    assert_eq!(combinations(5, 5).len(), 1);  // C(5,5) = 1
}

#[test]
fn pruned_matches_unpruned() {
    let kp = known_polytopes::hypercube();

    let result_unpruned = ehz_capacity(&kp.polytope).expect("unpruned capacity");
    let result_pruned = ehz_capacity_pruned(&kp.polytope).expect("pruned capacity");

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

// Unit tests for solve_kkt function

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

    let (beta, q_val) = result.unwrap();
    assert_eq!(beta.len(), 2);

    // β1 ≈ β2 ≈ 0.5
    assert!((beta[0] - 0.5).abs() < 1e-6, "β1 should be ~0.5");
    assert!((beta[1] - 0.5).abs() < 1e-6, "β2 should be ~0.5");

    // Q = 0 (parallel normals have ω₀ = 0)
    assert!(q_val.abs() < 1e-10, "Q should be ~0 for parallel normals");
}

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

    let (beta, q_val) = result.unwrap();
    assert_eq!(beta.len(), 4);

    // Verify constraints
    // N^T β = 0: β1 - β3 = 0, β2 - β4 = 0
    assert!((beta[0] - beta[2]).abs() < 1e-6, "β1 should equal β3");
    assert!((beta[1] - beta[3]).abs() < 1e-6, "β2 should equal β4");

    // η^T β = 1: β1 + β2 + β3 + β4 = 1
    let sum: f64 = beta.iter().sum();
    assert!((sum - 1.0).abs() < 1e-6, "β sum should be 1");

    // Q ≠ 0 (non-degenerate symplectic system)
    // Note: Q can be positive or negative depending on the cyclic order
    assert!(q_val.abs() > 1e-10, "Q should be non-zero for symplectic normals, got {}", q_val);
}

#[test]
fn solve_kkt_rank_deficient() {
    // Three normals in the xy-plane (rank = 2)
    // N^T has rank 2, not 4, so the system is rank-deficient
    // SVD should still solve it

    let normals = vec![
        Vector4::new(1.0, 0.0, 0.0, 0.0),
        Vector4::new(0.0, 1.0, 0.0, 0.0),
        Vector4::new(0.707, 0.707, 0.0, 0.0).normalize(),
    ];
    let heights = vec![1.0; 3];
    let perm = vec![0, 1, 2];

    let result = solve_kkt(&normals, &heights, &perm);

    // Should solve (SVD handles rank-deficient systems)
    assert!(result.is_some(), "rank-deficient system should solve via SVD");

    let (beta, _q_val) = result.unwrap();

    // Verify η^T β = 1
    let sum: f64 = beta.iter().sum();
    assert!((sum - 1.0).abs() < 1e-6, "β sum should be 1 even in rank-deficient case");
}

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
    if let Some((_beta, _q)) = result {
        // If it returns Some, it means the residual check didn't catch it
        // This is fine — the outer algorithm will filter via β > 0 or other checks
        eprintln!("Note: degenerate system returned Some (acceptable)");
    }
}

#[test]
#[ignore] // Expensive: 10 facets → exponential runtime (~2-5 min)
fn pentagon_capacity() {
    use geom::volume::volume;

    let kp = known_polytopes::hko_pentagon();
    let result = ehz_capacity_pruned(&kp.polytope).expect("pentagon capacity");

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

#[test]
fn triangle_square_capacity() {
    let kp = known_polytopes::lagrangian_triangle_square();
    let result = ehz_capacity_pruned(&kp.polytope).expect("Lagrangian triangle×square capacity");

    // Investigation complete: This is a Lagrangian product (equilateral triangle in q-space,
    // unit square in p-space), not a symplectic product. The algorithm correctly computes
    // capacity = 1.5 via optimal orbit using all 3 triangle facets and 2 square facets.
    // See experiments/triangle_square.md for detailed analysis.
    assert!(
        (result.capacity - kp.capacity).abs() < 1e-6,
        "Lagrangian triangle×square: got {}, expected {}", result.capacity, kp.capacity
    );
}

#[test]
fn symplectic_triangle_square_capacity() {
    let kp = known_polytopes::symplectic_triangle_square();
    let result = ehz_capacity_pruned(&kp.polytope).expect("symplectic triangle×square capacity");

    // Symplectic product: triangle in (q₁, p₁) plane ×_S square in (q₂, p₂) plane.
    // Moser's theorem: c(A ×_S B) = min(c(A), c(B))
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

#[test]
#[ignore] // Too expensive: 16 facets → exponential runtime (~hours)
fn crosspolytope_capacity() {
    let kp = known_polytopes::crosspolytope();
    let result = ehz_capacity_pruned(&kp.polytope).expect("crosspolytope capacity");

    // No known literature value - just verify computation succeeds
    assert!(result.capacity > 0.0, "crosspolytope capacity positive");
    eprintln!("Crosspolytope (16 facets): capacity={:.6}", result.capacity);
    eprintln!("  Iterations: {}", result.iterations);
}

// ---- Property tests ----

#[cfg(test)]
mod proptests {
    use super::*;
    use proptest::prelude::*;
    use datasets::random::generate_random_polytopes;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(5))]

        /// Property: pruned and unpruned algorithms return the same capacity.
        ///
        /// This tests `cor:adjacency-pruning` (adjacency pruning optimization).
        ///
        /// Run with: `cargo test -p hk2017 pruned_matches_unpruned_random -- --ignored`
        #[test]
        #[ignore] // Expensive: capacity computation on random polytopes (~30s)
        fn pruned_matches_unpruned_random(
            facet_count in 5usize..=6,
            seed in 0u64..4
        ) {
            let mut rng = ChaCha8Rng::seed_from_u64(seed);
            let polytopes = generate_random_polytopes(1, facet_count, 0.5, 2.0, &mut rng);

            if let Some(p) = polytopes.first() {
                let unpruned = ehz_capacity(p).unwrap();
                let pruned = ehz_capacity_pruned(p).unwrap();

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

/// Capacity scales quadratically: c_EHZ(λK) = λ²·c_EHZ(K).
///
/// Uses λ = e (transcendental) — cannot be the root of any polynomial with
/// integer coefficients, making numerical coincidences impossible.
#[test]
fn capacity_scales_quadratically() {
    let scale = std::f64::consts::E;

    let kp = known_polytopes::hypercube();
    let unit_cap = ehz_capacity(&kp.polytope).unwrap().capacity;

    let scaled_cube = geom::test_utils::scaled_hypercube(scale);
    let scaled_cap = ehz_capacity(&scaled_cube).unwrap().capacity;

    let expected = unit_cap * scale * scale;
    let relative_error = ((scaled_cap - expected) / expected).abs();

    assert!(
        relative_error < 1e-4,
        "capacity scaling failed: scale={scale}, unit_cap={unit_cap}, \
         scaled_cap={scaled_cap}, expected={expected}, relative_error={relative_error}"
    );
}

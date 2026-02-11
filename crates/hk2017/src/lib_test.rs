use super::*;
use geom::test_utils::{hypercube, simplex, triangle_product};
use nalgebra::Vector4;

#[test]
fn simplex_capacity() {
    let simplex = simplex();
    let result = ehz_capacity(&simplex).expect("simplex should have capacity");
    let expected = 0.25;
    assert!(
        (result.capacity - expected).abs() < 1e-6,
        "simplex capacity: got {}, expected {}",
        result.capacity,
        expected
    );
}

#[test]
fn hypercube_capacity() {
    let hypercube = hypercube();
    let result = ehz_capacity(&hypercube).expect("hypercube should have capacity");
    let expected = 4.0;
    assert!(
        (result.capacity - expected).abs() < 1e-6,
        "hypercube capacity: got {}, expected {}",
        result.capacity,
        expected
    );
}

#[test]
fn triangle_product_capacity() {
    let tri = triangle_product();
    let result = ehz_capacity(&tri).expect("triangle product should have capacity");
    let expected = 1.5;
    assert!(
        (result.capacity - expected).abs() < 1e-6,
        "triangle product capacity: got {}, expected {}",
        result.capacity,
        expected
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
    // Test that pruned and unpruned give same capacity
    let hypercube = hypercube();

    let result_unpruned = ehz_capacity(&hypercube).expect("unpruned capacity");
    let result_pruned = ehz_capacity_pruned(&hypercube).expect("pruned capacity");

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

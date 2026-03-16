//! Tests for saddle_point_solver: eigendecomposition-based KKT solver correctness.
//!
//! Proposition: The saddle-point solver returns beta > 0 satisfying the KKT system
//! M x = b with Q error bound |Q(beta_0) - Q_tilde| <= E.
//! Reference: [lem:kkt], [lem:q-error-bound]
//!
//! Strategy: fixture-based on known polytopes with known-good permutations,
//! plus synthetic tests for edge cases (rank-deficient, degenerate).

use super::saddle_point_solver::*;
use super::qp_assembly::build_augmented_system;
use crate::geom::known_polytopes;
use nalgebra::{DMatrix, DVector};

// ── Helpers ──

fn assert_approx(a: f64, b: f64, tol: f64, msg: &str) {
    assert!(
        (a - b).abs() < tol,
        "{}: |{} - {}| = {} >= {}",
        msg, a, b, (a - b).abs(), tol
    );
}

/// Try ALL cyclic permutations of ALL subsets of sizes 2..=f on a polytope.
/// Returns the best Q found and whether any valid solution exists.
///
/// For each subset of m facets, tries all m!/m = (m-1)! distinct cyclic orderings.
/// This is feasible for small polytopes (f <= 8) but combinatorially expensive
/// for larger ones.
fn find_best_q_exhaustive(polytope: &crate::geom::polytope::Polytope4D) -> (f64, bool) {
    let f = polytope.facet_count();
    let mut best_q = 0.0f64;
    let mut found = false;

    // Try all subset sizes from 2 to min(f, 6).
    for size in 2..=f.min(6) {
        for_each_combination(f, size, &mut |subset| {
            // Generate all permutations of this subset (not just rotations).
            // Fix first element to avoid cyclic duplicates, permute the rest.
            let mut rest: Vec<usize> = subset[1..].to_vec();
            loop {
                let mut perm = vec![subset[0]];
                perm.extend_from_slice(&rest);
                let (kkt, rhs) = build_augmented_system(polytope, &perm);
                if let Some(result) = solve_saddle_point(&kkt, &rhs) {
                    if result.beta.iter().all(|&b| b > EPS_BETA_POSITIVE)
                        && result.q_corrected > EPS_Q_POSITIVE
                    {
                        found = true;
                        if result.q_corrected > best_q {
                            best_q = result.q_corrected;
                        }
                    }
                }
                if !next_permutation(&mut rest) {
                    break;
                }
            }
        });
    }

    (best_q, found)
}

/// Advance to the next lexicographic permutation. Returns false if already at last.
fn next_permutation(arr: &mut [usize]) -> bool {
    let n = arr.len();
    if n <= 1 {
        return false;
    }
    // Find largest i such that arr[i] < arr[i+1]
    let mut i = n - 1;
    loop {
        if i == 0 {
            return false; // already at last permutation
        }
        i -= 1;
        if arr[i] < arr[i + 1] {
            break;
        }
    }
    // Find largest j > i such that arr[i] < arr[j]
    let mut j = n - 1;
    while arr[j] <= arr[i] {
        j -= 1;
    }
    arr.swap(i, j);
    arr[i + 1..].reverse();
    true
}

/// Call `f` with every k-element subset of {0, ..., n-1}.
fn for_each_combination(n: usize, k: usize, f: &mut impl FnMut(&[usize])) {
    let mut indices: Vec<usize> = (0..k).collect();
    loop {
        f(&indices);
        // Advance to next combination
        let mut i = k;
        loop {
            if i == 0 {
                return; // exhausted all combinations
            }
            i -= 1;
            indices[i] += 1;
            if indices[i] <= n - k + i {
                break;
            }
        }
        for j in (i + 1)..k {
            indices[j] = indices[j - 1] + 1;
        }
    }
}

// ── Known polytope tests ──

/// Simplex (5 facets): solver finds valid orbits and capacity matches.
#[test]
fn simplex_capacity_via_exhaustive_search() {
    let simplex = known_polytopes::simplex();
    let (best_q, found) = find_best_q_exhaustive(&simplex.polytope);

    assert!(found, "should find at least one valid orbit on simplex");
    let capacity = 0.5 / best_q;
    assert!(
        (capacity - simplex.capacity).abs() < 1e-4 * simplex.capacity,
        "simplex capacity mismatch: got {}, expected {}",
        capacity, simplex.capacity
    );
}

/// Lagrangian triangle product (6 facets): solver finds valid orbits.
#[test]
fn lagrangian_triangle_product_finds_valid_solution() {
    let tri_prod = known_polytopes::lagrangian_triangle_product();
    let (best_q, found) = find_best_q_exhaustive(&tri_prod.polytope);

    assert!(found, "should find at least one valid orbit on triangle product");
    let capacity = 0.5 / best_q;
    assert!(
        (capacity - tri_prod.capacity).abs() < 1e-4 * tri_prod.capacity,
        "triangle product capacity mismatch: got {}, expected {}",
        capacity, tri_prod.capacity
    );
}

/// Convenience wrapper: verify it returns the same as direct assembly + solve.
#[test]
fn solve_kkt_for_matches_direct() {
    let simplex = known_polytopes::simplex();
    let polytope = &simplex.polytope;
    let perm = vec![0, 1, 2];

    let result_direct = {
        let (kkt, rhs) = build_augmented_system(polytope, &perm);
        solve_saddle_point(&kkt, &rhs)
    };

    let result_convenience = solve_kkt_for(polytope, &perm);

    match (result_direct, result_convenience) {
        (Some(d), Some(c)) => {
            assert_approx(d.q_corrected, c.q_corrected, 1e-12, "Q should match");
            assert_eq!(d.beta.len(), c.beta.len());
            for i in 0..d.beta.len() {
                assert_approx(d.beta[i], c.beta[i], 1e-12, &format!("beta[{}]", i));
            }
        }
        (None, None) => {} // both fail: acceptable
        _ => panic!("direct and convenience should agree on Some/None"),
    }
}

// ── Error bound tests ──

/// The Q error bound E is always non-negative for all solutions found.
#[test]
fn q_error_bound_nonnegative_and_small() {
    let simplex = known_polytopes::simplex();
    let polytope = &simplex.polytope;
    let f = polytope.facet_count();

    let mut checked = 0;
    for size in 2..=f.min(6) {
        for_each_combination(f, size, &mut |subset| {
            let perm = subset.to_vec();
            let (kkt, rhs) = build_augmented_system(polytope, &perm);
            if let Some(result) = solve_saddle_point(&kkt, &rhs) {
                assert!(
                    result.q_error_bound >= 0.0,
                    "error bound should be non-negative, got {}",
                    result.q_error_bound
                );
                assert!(
                    result.q_error_bound < 1e-6,
                    "error bound too large: {:.2e}",
                    result.q_error_bound
                );
                checked += 1;
            }
        });
    }
    assert!(checked > 0, "should have found at least one solution to check");
}

// ── Inertia tests ──

/// The inertia counts should sum to the matrix size.
#[test]
fn inertia_sums_to_size() {
    let simplex = known_polytopes::simplex();
    let polytope = &simplex.polytope;
    let perm = vec![0, 1, 2];
    let (kkt, rhs) = build_augmented_system(polytope, &perm);

    if let Some(result) = solve_saddle_point(&kkt, &rhs) {
        let m = perm.len();
        let size = m + 5;
        assert_eq!(
            result.n_positive + result.n_negative + result.n_zero,
            size,
            "inertia should sum to matrix size {}",
            size
        );
    }
}

// ── Degenerate / edge case tests ──

/// Zero matrix should return None (all eigenvalues are zero).
#[test]
fn zero_matrix_returns_none() {
    let size = 8;
    let kkt = DMatrix::zeros(size, size);
    let rhs = DVector::zeros(size);
    assert!(solve_saddle_point(&kkt, &rhs).is_none());
}

/// Identity matrix with standard RHS: verify it doesn't panic.
#[test]
fn identity_matrix_trivial_solve() {
    let size = 8;
    let kkt = DMatrix::identity(size, size);
    let mut rhs = DVector::zeros(size);
    rhs[size - 1] = 1.0;

    // Not a real KKT matrix structure, but should not panic.
    let _result = solve_saddle_point(&kkt, &rhs);
}

/// Two-facet permutation: the minimal case (m=2, size=7).
#[test]
fn two_facet_permutation() {
    let simplex = known_polytopes::simplex();
    let polytope = &simplex.polytope;
    let perm = vec![0, 1];
    let (kkt, rhs) = build_augmented_system(polytope, &perm);

    // May or may not find a solution. Just verify no panic.
    let _result = solve_saddle_point(&kkt, &rhs);
}

// ── Constraint satisfaction ──

/// All returned solutions satisfy the normalization constraint (eta^T beta = 1).
#[test]
fn normalization_constraint_satisfied() {
    let simplex = known_polytopes::simplex();
    let polytope = &simplex.polytope;
    let f = polytope.facet_count();
    let heights = polytope.heights_f64();

    let mut checked = 0;
    for size in 2..=f.min(6) {
        for_each_combination(f, size, &mut |subset| {
            let perm = subset.to_vec();
            let (kkt, rhs) = build_augmented_system(polytope, &perm);
            if let Some(result) = solve_saddle_point(&kkt, &rhs) {
                let eta_dot_beta: f64 = result.beta.iter().enumerate()
                    .map(|(idx, &b)| b * heights[perm[idx]])
                    .sum();
                assert!(
                    (eta_dot_beta - 1.0).abs() < 1e-6,
                    "normalization violated: eta^T beta = {}, expected 1.0 (perm {:?})",
                    eta_dot_beta, perm
                );
                checked += 1;
            }
        });
    }
    assert!(checked > 0, "should have checked at least one solution");
}

/// All returned solutions satisfy the closure constraint (N^T beta = 0).
#[test]
fn closure_constraint_satisfied() {
    let simplex = known_polytopes::simplex();
    let polytope = &simplex.polytope;
    let f = polytope.facet_count();
    let normals = polytope.normals_f64();

    let mut checked = 0;
    for size in 2..=f.min(6) {
        for_each_combination(f, size, &mut |subset| {
            let perm = subset.to_vec();
            let (kkt, rhs) = build_augmented_system(polytope, &perm);
            if let Some(result) = solve_saddle_point(&kkt, &rhs) {
                for d in 0..4 {
                    let sum: f64 = result.beta.iter().enumerate()
                        .map(|(idx, &b)| b * normals[perm[idx]][d])
                        .sum();
                    assert!(
                        sum.abs() < 1e-6,
                        "closure[{}] violated: sum = {:.2e} (perm {:?})",
                        d, sum, perm
                    );
                }
                checked += 1;
            }
        });
    }
    assert!(checked > 0, "should have checked at least one solution");
}

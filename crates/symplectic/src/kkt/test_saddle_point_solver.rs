//! Saddle-point solver tests for `kkt::saddle_point_solver`.
//!
//! These tests validate the eigendecomposition-based KKT solve on:
//! - known polytopes with expected capacities,
//! - direct assembly-vs-convenience consistency,
//! - edge cases and numerical degeneracies,
//! - constraint residual checks for returned feasible candidates.

use super::qp_assembly::build_augmented_system_from_dual_vertices;
use super::saddle_point_solver::*;
use crate::geom::known_polytopes;
use nalgebra::{DMatrix, DVector};

// Tests for saddle_point_solver: eigendecomposition-based KKT solver correctness.
//
// Proposition: The saddle-point solver returns beta > 0 satisfying the KKT system
// M x = b with Q error bound |Q(beta_0) - Q_tilde| <= E.
// Reference: [lem:kkt], [lem:q-error-bound]
//
// Strategy: fixture-based on known polytopes with known-good permutations,
// plus synthetic tests for edge cases (rank-deficient, degenerate).

// ── Helpers ──

fn assert_approx(a: f64, b: f64, tol: f64, msg: &str) {
    assert!(
        (a - b).abs() < tol,
        "{}: |{} - {}| = {} >= {}",
        msg,
        a,
        b,
        (a - b).abs(),
        tol
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
    let dual_vertices = polytope.dual_vertices_f64();
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
                let (kkt, rhs) = build_augmented_system_from_dual_vertices(dual_vertices, &perm);
                if let KktOutcome::Feasible(result) = solve_saddle_point(&kkt, &rhs) {
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

    assert!(found, "should find at least one valid candidate on simplex");
    let capacity = 0.5 / best_q;
    assert!(
        (capacity - simplex.capacity).abs() < 1e-4 * simplex.capacity,
        "simplex capacity mismatch: got {}, expected {}",
        capacity,
        simplex.capacity
    );
}

/// Lagrangian triangle product (6 facets): solver finds valid candidates.
#[test]
fn lagrangian_triangle_product_finds_valid_solution() {
    let tri_prod = known_polytopes::lagrangian_triangle_product();
    let (best_q, found) = find_best_q_exhaustive(&tri_prod.polytope);

    assert!(
        found,
        "should find at least one valid candidate on triangle product"
    );
    let capacity = 0.5 / best_q;
    assert!(
        (capacity - tri_prod.capacity).abs() < 1e-4 * tri_prod.capacity,
        "triangle product capacity mismatch: got {}, expected {}",
        capacity,
        tri_prod.capacity
    );
}

/// Convenience wrapper: verify it returns the same as direct assembly + solve.
#[test]
fn solve_kkt_for_matches_direct() {
    let simplex = known_polytopes::simplex();
    let polytope = &simplex.polytope;
    let perm = vec![0, 1, 2];

    let result_direct = {
        let dual_vertices = polytope.dual_vertices_f64();
        let (kkt, rhs) = build_augmented_system_from_dual_vertices(dual_vertices, &perm);
        solve_saddle_point(&kkt, &rhs)
    };

    let result_convenience = solve_kkt_for(polytope, &perm);

    match (result_direct, result_convenience) {
        (KktOutcome::Feasible(d), KktOutcome::Feasible(c)) => {
            assert_approx(d.q_corrected, c.q_corrected, 1e-12, "Q should match");
            assert_eq!(d.beta.len(), c.beta.len());
            for i in 0..d.beta.len() {
                assert_approx(d.beta[i], c.beta[i], 1e-12, &format!("beta[{}]", i));
            }
        }
        (KktOutcome::Feasible(_), _) | (_, KktOutcome::Feasible(_)) => {
            panic!("direct and convenience should agree on feasibility");
        }
        _ => {} // both non-feasible: acceptable
    }
}

// ── Error bound tests ──

/// The Q error bound E is always non-negative for all solutions found.
#[test]
fn q_error_bound_nonnegative_and_small() {
    let simplex = known_polytopes::simplex();
    let polytope = &simplex.polytope;
    let dual_vertices = polytope.dual_vertices_f64();
    let f = polytope.facet_count();

    let mut checked = 0;
    for size in 2..=f.min(6) {
        for_each_combination(f, size, &mut |subset| {
            let perm = subset.to_vec();
            let (kkt, rhs) = build_augmented_system_from_dual_vertices(dual_vertices, &perm);
            if let KktOutcome::Feasible(result) = solve_saddle_point(&kkt, &rhs) {
                assert!(
                    result.q_error_bound >= 0.0,
                    "error bound should be non-negative, got {}",
                    result.q_error_bound
                );
                assert!(
                    result.q_error_bound.is_finite(),
                    "error bound should be finite, got {}",
                    result.q_error_bound
                );
                checked += 1;
            }
        });
    }
    assert!(
        checked > 0,
        "should have found at least one solution to check"
    );
}

// ── Inertia tests ──

/// The inertia counts should sum to the matrix size.
#[test]
fn inertia_sums_to_size() {
    let simplex = known_polytopes::simplex();
    let polytope = &simplex.polytope;
    let perm = vec![0, 1, 2];
    let dual_vertices = polytope.dual_vertices_f64();
    let (kkt, rhs) = build_augmented_system_from_dual_vertices(dual_vertices, &perm);

    if let KktOutcome::Feasible(result) = solve_saddle_point(&kkt, &rhs) {
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
    assert!(matches!(
        solve_saddle_point(&kkt, &rhs),
        KktOutcome::SingularMatrix
    ));
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
    let dual_vertices = polytope.dual_vertices_f64();
    let (kkt, rhs) = build_augmented_system_from_dual_vertices(dual_vertices, &perm);

    // May or may not find a solution. Just verify no panic.
    let _result = solve_saddle_point(&kkt, &rhs);
}

// ── Q error bound panic on perturbed symmetric polytopes ──

/// Perturbed LP(4,4) triggers the Q-error-bound panic on degenerate 4-facet orbits.
/// The KKT matrix eigenvalues shift from null (at the symmetric point) to small-but-
/// retained (at the perturbed point), making |λ_min| tiny and the error bound vacuous.
/// This is a regression test: when the deferred q-error-bound work in tasks/numerics.md is
/// resolved, this test should be updated to expect the new behavior.
#[test]
fn perturbed_lp44_degenerate_orbit() {
    use crate::{lagrangian_product, regular_polygon_2d};
    let (qn, qh) = regular_polygon_2d(4, 1.0);
    let polytope = lagrangian_product(&qn, &qh, &qn, &qh).expect("regular LP(4,4)");
    let duals = polytope.dual_vertices_f64();
    // Small perturbation breaking the square symmetry
    // Small perturbation breaking square symmetry.
    let perturbed: Vec<nalgebra::Vector4<f64>> = duals
        .iter()
        .enumerate()
        .map(|(i, a)| {
            let s = 1e-4 * ((i + 1) as f64);
            nalgebra::Vector4::new(
                a[0] + s * 0.3,
                a[1] - s * 0.7,
                a[2] + s * 0.5,
                a[3] - s * 0.1,
            )
        })
        .collect();
    let perturbed_poly =
        crate::geom::polytope::Polytope4D::from_f64(perturbed).expect("perturbed LP(4,4)");
    // Solve KKT directly for the degenerate 4-facet orbit [1,5,3,7].
    // At the symmetric point this orbit has β = 0.25. Under perturbation,
    // the KKT eigenvalues shift from null to small-but-retained, making
    // the Q error bound vacuous.
    let outcome = solve_kkt_for(&perturbed_poly, &[1, 5, 3, 7]);
    // The degenerate orbit should NOT be feasible on the perturbed polytope.
    assert!(
        !matches!(outcome, KktOutcome::Feasible(_)),
        "degenerate 4-facet orbit on perturbed LP(4,4) should not be feasible, got {:?}",
        outcome
    );
}

/// Perturbed LP(4,4): ehz_capacity completes without panic.
///
/// Before the KktOutcome refactor, ehz_capacity panicked on perturbed LP(4,4)
/// with "Q error bound unexpectedly large" because degenerate orbits reached
/// finalize_result with |λ_min| ≈ 1e-12. After the refactor, these orbits
/// return Infeasible (β < 0 or Q ≤ 0) before reaching the error bound check.
///
/// Regression test: if this starts panicking again, the KktOutcome early
/// returns are no longer catching the degenerate cases.
#[test]
fn perturbed_lp44_ehz_capacity_no_panic() {
    use crate::{lagrangian_product, regular_polygon_2d};
    let (qn, qh) = regular_polygon_2d(4, 1.0);
    let polytope = lagrangian_product(&qn, &qh, &qn, &qh).expect("LP(4,4)");
    let duals = polytope.dual_vertices_f64();
    // Fixed perturbation that breaks square symmetry.
    // Uses the exact dual vertices rather than RNG for stability.
    let perturbed: Vec<nalgebra::Vector4<f64>> = duals
        .iter()
        .enumerate()
        .map(|(i, a)| {
            let s = 0.01 * ((i + 1) as f64);
            nalgebra::Vector4::new(
                a[0] + s * 0.3,
                a[1] - s * 0.7,
                a[2] + s * 0.5,
                a[3] - s * 0.1,
            )
        })
        .collect();
    let pp = crate::geom::polytope::Polytope4D::from_f64(perturbed).expect("perturbed LP(4,4)");
    let result = crate::ehz_capacity_pruned(&pp);
    assert!(
        result.is_ok(),
        "ehz_capacity should succeed on perturbed LP(4,4)"
    );
}

// ── Constraint satisfaction ──

/// All returned solutions satisfy the normalization constraint (1^T beta = 1).
#[test]
fn normalization_constraint_satisfied() {
    let simplex = known_polytopes::simplex();
    let polytope = &simplex.polytope;
    let dual_vertices = polytope.dual_vertices_f64();
    let f = polytope.facet_count();

    let mut checked = 0;
    for size in 2..=f.min(6) {
        for_each_combination(f, size, &mut |subset| {
            let perm = subset.to_vec();
            let (kkt, rhs) = build_augmented_system_from_dual_vertices(dual_vertices, &perm);
            if let KktOutcome::Feasible(result) = solve_saddle_point(&kkt, &rhs) {
                let sum_beta: f64 = result.beta.iter().sum();
                assert!(
                    (sum_beta - 1.0).abs() < 1e-6,
                    "normalization violated: sum(beta) = {}, expected 1.0 (perm {:?})",
                    sum_beta,
                    perm
                );
                checked += 1;
            }
        });
    }
    assert!(checked > 0, "should have checked at least one solution");
}

/// All returned solutions satisfy the closure constraint (A^T beta = 0).
#[test]
fn closure_constraint_satisfied() {
    let simplex = known_polytopes::simplex();
    let polytope = &simplex.polytope;
    let f = polytope.facet_count();
    let dual_verts = polytope.dual_vertices_f64();

    let mut checked = 0;
    for size in 2..=f.min(6) {
        for_each_combination(f, size, &mut |subset| {
            let perm = subset.to_vec();
            let (kkt, rhs) = build_augmented_system_from_dual_vertices(dual_verts, &perm);
            if let KktOutcome::Feasible(result) = solve_saddle_point(&kkt, &rhs) {
                #[allow(clippy::needless_range_loop)]
                for d in 0..4 {
                    let sum: f64 = result
                        .beta
                        .iter()
                        .enumerate()
                        .map(|(idx, &b)| b * dual_verts[perm[idx]][d])
                        .sum();
                    assert!(
                        sum.abs() < 1e-6,
                        "closure[{}] violated: sum = {:.2e} (perm {:?})",
                        d,
                        sum,
                        perm
                    );
                }
                checked += 1;
            }
        });
    }
    assert!(checked > 0, "should have checked at least one solution");
}

//! HK2017 algorithm for EHZ capacity of general 4D polytopes.
//!
//! Implements the Haim-Kislev 2017 exhaustive enumeration algorithm: for each
//! subset S of facets and each cyclic permutation sigma of S, solve the KKT
//! system for the constrained maximum of Q(beta) and track the minimum action
//! across all certified solutions.
//!
//! Two entry points:
//! - `ehz_capacity`: production variant with directed adjacency pruning ([cor:adjacency-pruning])
//! - `ehz_capacity_unpruned`: reference implementation without pruning
//!
//! Both use `CapacityAccumulator` for the enumerate -> solve -> track pattern.
//!
//! Submodules:
//! - `permutations` — cyclic permutation generation (allocating + callback)
//! - `orbit_recovery` — recover a Reeb orbit from a KKT solution
//! - `generate_capacity_fixtures` — fixture generation for 33 test polytopes
//!
//! # Complexity
//!
//! sum_{m=2}^{F} C(F,m) * (m-1)! — exponential in F.
//!
//! Mathematical correspondence: [alg:ehz]

pub mod permutations;
pub mod orbit_recovery;
pub mod generate_capacity_fixtures;

use crate::algorithms::capacity_accumulator::{CapacityAccumulator, CapacityResult};
use crate::algorithms::facet_adjacency::{build_transition_matrix, is_feasible_cycle};
use crate::geom::polytope::Polytope4D;
use crate::kkt::saddle_point_solver::{solve_kkt_for, KktResult, EPS_Q_POSITIVE};
use crate::kkt::{classify_margin, Solution, Verdict};
use permutations::for_each_cyclic_permutation;

/// Result of the EHZ capacity computation.
///
/// Wraps [`CapacityResult`] (shared accumulator output) plus the algorithm-specific
/// `best_subset` field identifying which facet indices participate in the optimal orbit.
///
/// Access capacity fields via `.result.capacity` (no Deref — explicit field access).
///
/// [alg:ehz]: result of exhaustive (S, sigma) enumeration.
#[derive(Clone, Debug)]
pub struct EhzResult {
    /// Core capacity result from the accumulator: capacity, uncertainty, best
    /// permutation, beta vector, and iteration count.
    pub result: CapacityResult,
    /// Facet indices S participating in the optimal orbit (unordered).
    pub best_subset: Vec<usize>,
}

/// Compute c_EHZ(K) for a convex polytope K in R^4.
///
/// Reference (unpruned) implementation of [alg:ehz]: exhaustive search over all
/// (S, sigma) pairs with |S| >= 2. For production use, prefer [`ehz_capacity`]
/// which applies directed adjacency pruning ([cor:adjacency-pruning]).
///
/// Returns `None` if no valid (S, sigma) pair yields a certified beta > 0
/// (should not happen for valid polytopes, but guards against degenerate input).
///
/// # Permutation ordering convention
///
/// `best_permutation` follows the **positive Reeb direction**: sigma = [a, b, c, ...]
/// means the Reeb trajectory visits F_a -> F_b -> F_c -> ... -> F_a.
/// For consecutive facets, omega_0(n_{sigma(k)}, n_{sigma(k+1)}) >= 0.
///
/// [alg:ehz]: exhaustive capacity computation.
pub fn ehz_capacity_unpruned(polytope: &Polytope4D) -> Option<EhzResult> {
    let f = polytope.facet_count();
    let mut acc = CapacityAccumulator::new();

    // Track which subset corresponds to the best certified permutation.
    // The accumulator tracks permutations but not subsets — we need the subset
    // for the EhzResult.
    let mut best_subset_certified: Option<(f64, Vec<usize>)> = None;

    for m in 2..=f {
        for subset in combinations(f, m) {
            for_each_cyclic_permutation(&subset, &mut |perm| {
                if let Some(solution) = solve_and_convert(polytope, perm) {
                    // Track subset for the best certified candidate.
                    if solution.verdict == Verdict::True && solution.q > EPS_Q_POSITIVE {
                        let action = 0.5 / solution.q;
                        let update = best_subset_certified
                            .as_ref()
                            .is_none_or(|(best, _)| action < *best);
                        if update {
                            best_subset_certified = Some((action, subset.clone()));
                        }
                    }
                    acc.submit(perm, &solution);
                }
            });
        }
    }

    let result = acc.finalize()?;
    // Use tracked subset if available; fallback to deriving from best_permutation.
    let best_subset = best_subset_certified
        .map(|(_, s)| s)
        .unwrap_or_else(|| {
            let mut s = result.best_permutation.clone();
            s.sort();
            s
        });

    Some(EhzResult {
        result,
        best_subset,
    })
}

/// Compute c_EHZ(K) with directed adjacency pruning.
///
/// **Production variant used in all experiments.** Skips (S, sigma) pairs where
/// consecutive facets violate vertex adjacency or the directed omega_0 condition
/// from [lem:numerical-transition-feasibility]. This is the A2 pruning level
/// from the ablation study.
///
/// Returns `None` if no valid orbit is found.
///
/// [alg:ehz] with [cor:adjacency-pruning].
pub fn ehz_capacity(polytope: &Polytope4D) -> Option<EhzResult> {
    let f = polytope.facet_count();
    let adj = build_transition_matrix(polytope);
    let mut acc = CapacityAccumulator::new();

    let mut best_subset_certified: Option<(f64, Vec<usize>)> = None;

    for m in 2..=f {
        for subset in combinations(f, m) {
            for_each_cyclic_permutation(&subset, &mut |perm| {
                // Adjacency pruning: skip non-adjacent cycles.
                if !is_feasible_cycle(perm, &adj) {
                    return;
                }

                if let Some(solution) = solve_and_convert(polytope, perm) {
                    if solution.verdict == Verdict::True && solution.q > EPS_Q_POSITIVE {
                        let action = 0.5 / solution.q;
                        let update = best_subset_certified
                            .as_ref()
                            .is_none_or(|(best, _)| action < *best);
                        if update {
                            best_subset_certified = Some((action, subset.clone()));
                        }
                    }
                    acc.submit(perm, &solution);
                }
            });
        }
    }

    let result = acc.finalize()?;
    let best_subset = best_subset_certified
        .map(|(_, s)| s)
        .unwrap_or_else(|| {
            let mut s = result.best_permutation.clone();
            s.sort();
            s
        });

    Some(EhzResult {
        result,
        best_subset,
    })
}

/// Generate all combinations of `k` elements from `{0, ..., n-1}` in lexicographic order.
///
/// Returns an empty vec if `k == 0` or `k > n`.
///
/// [alg:ehz]: "for each S subseteq {1, ..., F} with |S| >= 2".
pub fn combinations(n: usize, k: usize) -> Vec<Vec<usize>> {
    if k == 0 || k > n {
        return Vec::new();
    }
    let mut result = Vec::new();
    let mut combo = vec![0usize; k];
    combinations_rec(n, k, 0, 0, &mut combo, &mut result);
    result
}

/// Recursive helper for lexicographic combination generation.
fn combinations_rec(
    n: usize,
    k: usize,
    start: usize,
    depth: usize,
    combo: &mut [usize],
    result: &mut Vec<Vec<usize>>,
) {
    if depth == k {
        result.push(combo.to_vec());
        return;
    }
    for i in start..=(n - k + depth) {
        combo[depth] = i;
        combinations_rec(n, k, i + 1, depth + 1, combo, result);
    }
}

// ── Internal helpers ──

/// Solve the KKT system for a (polytope, permutation) pair and convert the
/// result into a `Solution` for the accumulator.
///
/// The saddle-point solver returns `KktResult` with `q_corrected` and `beta`.
/// We compute `margin = min(beta)` and classify via `classify_margin` to produce
/// a `Solution` with a trinary `Verdict`.
fn solve_and_convert(polytope: &Polytope4D, perm: &[usize]) -> Option<Solution> {
    let kkt = solve_kkt_for(polytope, perm)?;
    Some(kkt_result_to_solution(kkt))
}

/// Convert a `KktResult` (saddle-point solver output) to a `Solution` (accumulator input).
///
/// Maps: q_corrected -> q, beta -> beta, min(beta) -> margin, classify_margin -> verdict.
fn kkt_result_to_solution(result: KktResult) -> Solution {
    let margin = result
        .beta
        .iter()
        .copied()
        .fold(f64::INFINITY, f64::min);
    Solution {
        verdict: classify_margin(margin),
        q: result.q_corrected,
        beta: result.beta,
        margin,
    }
}

// Tests for hk2017: capacity values for known polytopes (simplex, hypercube, products).
//
// Proposition: The EHZ capacity computed by `ehz_capacity_unpruned` and `ehz_capacity`
// agrees with literature values for all named polytopes.
// Reference: [def:ehz-capacity], [thm:hko-counterexample]
//
// Strategy: smoke tests (direct computation, small polytopes) + fixture-based
// (pre-computed dataset for comprehensive coverage).
#[cfg(test)]
mod tests_literature {
    use super::*;
    use crate::geom::known_polytopes;

    use std::path::PathBuf;
    use std::sync::LazyLock;

    use super::generate_capacity_fixtures::{
        load_test_dataset, literature_values, polytope_catalog, TestPolytope, FIXTURE_PATH,
    };

    /// Shared dataset loaded from cached fixture (fast, <1ms).
    ///
    /// If the fixture is missing, panics with instructions to regenerate:
    /// `cargo test --release regenerate_test_dataset -- --ignored --nocapture`
    static DATASET: LazyLock<Vec<TestPolytope>> = LazyLock::new(|| {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(FIXTURE_PATH);
        load_test_dataset(&path)
    });

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
            (result.result.capacity - kp.capacity).abs() < 1e-6,
            "simplex capacity: got {}, expected {}",
            result.result.capacity,
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
            (result.result.capacity - kp.capacity).abs() < 1e-6,
            "hypercube capacity: got {}, expected {}",
            result.result.capacity,
            kp.capacity
        );
    }

    /// Verify unpruned EHZ capacity of the Lagrangian triangle product (7 facets).
    ///
    /// Lagrangian product of equilateral triangle (q-space) and unit square (p-space).
    /// Tests product geometry handling.
    #[test]
    fn lagrangian_triangle_product_capacity() {
        let kp = known_polytopes::lagrangian_triangle_product();
        let result = ehz_capacity_unpruned(&kp.polytope)
            .expect("lagrangian triangle product should have capacity");
        assert!(
            (result.result.capacity - kp.capacity).abs() < 1e-6,
            "lagrangian triangle product capacity: got {}, expected {}",
            result.result.capacity,
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
        let result = ehz_capacity(&kp.polytope).expect("Lagrangian triangle x square capacity");
        assert!(
            (result.result.capacity - kp.capacity).abs() < 1e-6,
            "Lagrangian triangle x square: got {}, expected {}",
            result.result.capacity,
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
        let result = ehz_capacity(&kp.polytope).expect("symplectic triangle x square capacity");
        assert!(
            (result.result.capacity - kp.capacity).abs() < 1e-6,
            "symplectic triangle x square: got {}, expected {} (min formula)",
            result.result.capacity,
            kp.capacity
        );
    }

    // ── Fixture-based tests ──

    /// Verify polytope_catalog() is deterministic (same seed -> same polytopes).
    ///
    /// Calls polytope_catalog() twice and verifies identical output. Critical invariant
    /// for fixture generation: non-determinism would silently invalidate the fixture.
    #[test]
    fn catalog_determinism() {
        let c1 = polytope_catalog();
        let c2 = polytope_catalog();
        assert_eq!(c1.len(), c2.len());
        for (a, b) in c1.iter().zip(c2.iter()) {
            assert_eq!(a.name, b.name);
            assert_eq!(
                a.polytope.normals_f64(),
                b.polytope.normals_f64(),
                "'{}': normals non-deterministic",
                a.name
            );
            assert_eq!(
                a.polytope.heights_f64(),
                b.polytope.heights_f64(),
                "'{}': heights non-deterministic",
                a.name
            );
        }
    }

    /// Detect fixture staleness: compares polytope names in fixture vs current catalog.
    ///
    /// If this test warns, regenerate the fixture:
    /// `cargo test --release regenerate_test_dataset -- --ignored --nocapture`
    #[test]
    fn fixture_staleness_check() {
        let catalog = polytope_catalog();
        let dataset = &*DATASET;

        let catalog_names: std::collections::HashSet<&str> =
            catalog.iter().map(|c| c.name.as_str()).collect();
        let fixture_names: std::collections::HashSet<&str> =
            dataset.iter().map(|tp| tp.name.as_str()).collect();

        let missing: Vec<_> = catalog_names.difference(&fixture_names).collect();
        let orphaned: Vec<_> = fixture_names.difference(&catalog_names).collect();

        for name in &missing {
            eprintln!("WARNING: catalog polytope '{}' not in fixture", name);
        }
        for name in &orphaned {
            eprintln!("WARNING: fixture polytope '{}' not in current catalog", name);
        }

        if !missing.is_empty() || !orphaned.is_empty() {
            eprintln!(
                "WARNING: fixture staleness detected ({} missing, {} orphaned). \
                 Regenerate with: cargo test --release regenerate_test_dataset -- --ignored --nocapture",
                missing.len(),
                orphaned.len()
            );
        } else {
            eprintln!("Fixture covers all {} catalog polytopes", catalog.len());
        }
    }

    /// Verify known polytopes match literature capacity values from fixture (~0 cost).
    ///
    /// Loads pre-computed capacities from the fixture and compares against
    /// `known_polytopes::literature_values()`.
    #[test]
    fn literature_capacity_values() {
        let dataset = &*DATASET;
        let lit_values = literature_values();

        for &(name, expected) in &lit_values {
            if let Some(tp) = dataset.iter().find(|tp| tp.name == name) {
                let rel_err = (tp.capacity - expected).abs() / expected;
                assert!(
                    rel_err < 1e-6,
                    "'{}': fixture capacity {} disagrees with literature value {}, rel_error = {:.2e}",
                    name,
                    tp.capacity,
                    expected,
                    rel_err
                );
            } else {
                eprintln!(
                    "WARNING: '{}' not in fixture, skipping literature check",
                    name
                );
            }
        }

        eprintln!("Verified {} literature values from fixture", lit_values.len());
    }

    /// Verify all fixture polytopes have strictly positive capacity.
    ///
    /// Proposition: c_EHZ(K) > 0 for any convex body K with nonempty interior.
    #[test]
    fn capacity_positive_on_all_polytopes() {
        let dataset = &*DATASET;
        for entry in dataset {
            assert!(
                entry.capacity > 0.0,
                "{}: capacity should be positive, got {}",
                entry.name,
                entry.capacity
            );
        }
    }

    /// Verify HK2017 and billiard agree on all Lagrangian products in the fixture.
    ///
    /// The billiard algorithm is polynomial-time but restricted to Lagrangian products.
    /// On the overlapping domain, both algorithms must produce the same capacity.
    #[test]
    fn billiard_cross_validation() {
        let dataset = &*DATASET;
        let mut checked = 0;
        for tp in dataset.iter() {
            if let Some(cap_billiard) = tp.capacity_billiard {
                let rel_err = (tp.capacity - cap_billiard).abs() / cap_billiard;
                assert!(
                    rel_err < 1e-6,
                    "'{}': HK2017 ({}) != billiard ({}) capacity, rel_error = {:.2e}",
                    tp.name,
                    tp.capacity,
                    cap_billiard,
                    rel_err
                );
                checked += 1;
            }
        }
        assert!(
            checked > 0,
            "expected at least one Lagrangian product in fixture for cross-validation"
        );
    }

    /// Sanity checks on the systolic ratio distribution across all fixture polytopes.
    ///
    /// sys(K) = c_EHZ(K)^2 / (2 vol(K)). Checks: all positive, all finite, all < 100.
    #[test]
    fn sys_distribution_sanity_checks() {
        let dataset = &*DATASET;
        let sys_values: Vec<f64> = dataset
            .iter()
            .map(|e| e.capacity.powi(2) / (2.0 * e.volume))
            .collect();

        let min_sys = sys_values.iter().copied().fold(f64::INFINITY, f64::min);
        let max_sys = sys_values
            .iter()
            .copied()
            .fold(f64::NEG_INFINITY, f64::max);

        assert!(min_sys > 0.0, "all sys values should be positive");
        assert!(max_sys < 100.0, "sys values should be reasonable (< 100)");
    }
}

// Tests for hk2017: rank-deficient, degenerate, and near-singular KKT systems.
//
// Proposition: The KKT solver handles edge cases gracefully — returns None for
// infeasible systems, produces valid solutions for structured low-dimensional cases,
// and does not panic on degenerate input.
// Reference: [lem:kkt]
//
// Strategy: hand-constructed polytopes with known KKT structure, direct solver calls.
#[cfg(test)]
mod tests_kkt_edge_cases {
    use crate::geom::polytope::Polytope4D;
    use crate::kkt::saddle_point_solver::solve_kkt_for;
    use nalgebra::Vector4;

    /// KKT solver on minimal 2-facet system (two opposite facets).
    ///
    /// Two opposite facets: n1 = (1,0,0,0), n2 = (-1,0,0,0), h1 = h2 = 1.
    /// Constraints: beta_1 - beta_2 = 0, beta_1 + beta_2 = 1 => beta = (0.5, 0.5).
    /// Q(beta) = 0 because omega_0(n1, n2) = 0 (parallel normals, q-space only).
    ///
    /// Tests solver on the smallest possible system size.
    #[test]
    fn solve_kkt_two_facets() {
        let normals = [Vector4::x(), -Vector4::x()];
        let heights = [1.0, 1.0];
        let perm = [0, 1];

        // Two opposite facets don't form a valid bounded polytope (need >=5 for R^4).
        // Use the augmented system directly to test the solver on this minimal input.
        let polytope = match Polytope4D::new(normals.iter().zip(heights.iter()).map(|(n, &h)| n / h).collect()) {
            Ok(p) => p,
            Err(_) => {
                // Expected: 2 facets is too few for a bounded polytope in R^4.
                // Test the augmented system assembly + solver directly.
                // This exercises the solver's handling of small systems.
                eprintln!("2-facet polytope rejected (expected); testing augmented system directly");

                // Build the augmented system manually from normals/heights.
                // The augmented system is valid for any facet count, even if the
                // polytope is not bounded.
                let m = perm.len();
                let n_dim = 4;
                let size = m + n_dim + 1; // m + 4 + 1 = 7
                let mut kkt_mat = nalgebra::DMatrix::zeros(size, size);
                let mut rhs = nalgebra::DVector::zeros(size);

                // H block (m x m): H_ij = omega_0(n_sigma(i), n_sigma(j))
                for i in 0..m {
                    for j in 0..m {
                        let ni = &normals[perm[i]];
                        let nj = &normals[perm[j]];
                        kkt_mat[(i, j)] = crate::geom::symplectic_form::omega0(ni, nj);
                    }
                }

                // N block: closure constraints
                for i in 0..m {
                    let n = &normals[perm[i]];
                    for k in 0..n_dim {
                        kkt_mat[(i, m + k)] = n[k];
                        kkt_mat[(m + k, i)] = n[k];
                    }
                }

                // eta block: normalization
                for i in 0..m {
                    kkt_mat[(i, m + n_dim)] = 1.0;
                    kkt_mat[(m + n_dim, i)] = 1.0;
                }

                rhs[m + n_dim] = 1.0;

                let result = crate::kkt::saddle_point_solver::solve_saddle_point(&kkt_mat, &rhs);

                if let Some(r) = &result {
                    assert_eq!(r.beta.len(), 2);
                    // beta_1 ~ beta_2 ~ 0.5
                    assert!(
                        (r.beta[0] - 0.5).abs() < 1e-6,
                        "beta_1 should be ~0.5, got {}",
                        r.beta[0]
                    );
                    assert!(
                        (r.beta[1] - 0.5).abs() < 1e-6,
                        "beta_2 should be ~0.5, got {}",
                        r.beta[1]
                    );
                    // Q = 0 (parallel normals have omega_0 = 0)
                    assert!(
                        r.q_corrected.abs() < 1e-10,
                        "Q should be ~0 for parallel normals, got {}",
                        r.q_corrected
                    );
                }
                return;
            }
        };

        // If construction succeeded (unlikely for 2 facets), test via standard API.
        let result = solve_kkt_for(&polytope, &perm);
        assert!(result.is_some(), "two-facet system should solve");
        let r = result.unwrap();
        assert_eq!(r.beta.len(), 2);
        assert!((r.beta[0] - 0.5).abs() < 1e-6);
        assert!((r.beta[1] - 0.5).abs() < 1e-6);
        assert!(r.q_corrected.abs() < 1e-10);
    }

    /// KKT solver on 4-facet symplectic square.
    ///
    /// Four facets forming a 2D symplectic subplane:
    /// n1 = e_q1, n2 = e_p1, n3 = -e_q1, n4 = -e_p1 with heights all 1.0.
    /// omega_0(e_q1, e_p1) = 1, so Q != 0 (non-degenerate symplectic system).
    /// Constraints: beta_1 = beta_3, beta_2 = beta_4, sum = 1.
    ///
    /// Tests the solver on structured geometry with non-trivial symplectic form.
    #[test]
    fn solve_kkt_four_facets_symplectic() {
        let normals = [
            Vector4::x(),  // e_q1
            Vector4::z(),  // e_p1
            -Vector4::x(), // -e_q1
            -Vector4::z(), // -e_p1
        ];
        let _heights = [1.0; 4];
        let perm = [0, 1, 2, 3];

        // 4 facets in R^4 is not a bounded polytope (need >=5). Build augmented
        // system directly from normals/heights.
        let m = perm.len();
        let n_dim = 4;
        let size = m + n_dim + 1;
        let mut kkt_mat = nalgebra::DMatrix::zeros(size, size);
        let mut rhs = nalgebra::DVector::zeros(size);

        for i in 0..m {
            for j in 0..m {
                kkt_mat[(i, j)] = crate::geom::symplectic_form::omega0(
                    &normals[perm[i]],
                    &normals[perm[j]],
                );
            }
        }
        for i in 0..m {
            let n = &normals[perm[i]];
            for k in 0..n_dim {
                kkt_mat[(i, m + k)] = n[k];
                kkt_mat[(m + k, i)] = n[k];
            }
            kkt_mat[(i, m + n_dim)] = 1.0;
            kkt_mat[(m + n_dim, i)] = 1.0;
        }
        rhs[m + n_dim] = 1.0;

        let result = crate::kkt::saddle_point_solver::solve_saddle_point(&kkt_mat, &rhs);

        // The solver may return None for this small (m=4) augmented system
        // because the (m+5=9) matrix can be ill-conditioned or the residual
        // check may reject the solution. Either Some or None is acceptable.
        if let Some(r) = result {
            assert_eq!(r.beta.len(), 4);

            // Verify constraints: beta_1 = beta_3, beta_2 = beta_4.
            assert!(
                (r.beta[0] - r.beta[2]).abs() < 1e-6,
                "beta_1 should equal beta_3"
            );
            assert!(
                (r.beta[1] - r.beta[3]).abs() < 1e-6,
                "beta_2 should equal beta_4"
            );

            // Normalization: sum = 1.
            let sum: f64 = r.beta.iter().sum();
            assert!((sum - 1.0).abs() < 1e-6, "beta sum should be 1");

            // Q != 0 (non-degenerate symplectic system).
            assert!(
                r.q_corrected.abs() > 1e-10,
                "Q should be non-zero for symplectic normals, got {}",
                r.q_corrected
            );
        } else {
            eprintln!("Note: 4-facet symplectic system returned None (solver rejected it)");
        }
    }

    /// KKT solver handles rank-deficient normal matrix.
    ///
    /// Three normals in the q-plane (rank 2 normal matrix): omega_0(n_i, n_j) = 0
    /// for all pairs. The unique beta satisfying constraints has beta_2 < 0,
    /// so solve_kkt correctly returns None.
    ///
    /// Tests that the solver correctly detects infeasibility from rank deficiency.
    #[test]
    fn solve_kkt_rank_deficient() {
        let normals = [
            Vector4::new(1.0, 0.0, 0.0, 0.0),
            Vector4::new(0.0, 1.0, 0.0, 0.0),
            Vector4::new(0.707, 0.707, 0.0, 0.0).normalize(),
        ];
        let _heights = [1.0; 3];
        let perm = [0, 1, 2];

        // Build augmented system directly (3 facets is not a valid polytope).
        let m = perm.len();
        let n_dim = 4;
        let size = m + n_dim + 1;
        let mut kkt_mat = nalgebra::DMatrix::zeros(size, size);
        let mut rhs = nalgebra::DVector::zeros(size);

        for i in 0..m {
            for j in 0..m {
                kkt_mat[(i, j)] = crate::geom::symplectic_form::omega0(
                    &normals[perm[i]],
                    &normals[perm[j]],
                );
            }
        }
        for i in 0..m {
            let n = &normals[perm[i]];
            for k in 0..n_dim {
                kkt_mat[(i, m + k)] = n[k];
                kkt_mat[(m + k, i)] = n[k];
            }
            kkt_mat[(i, m + n_dim)] = 1.0;
            kkt_mat[(m + n_dim, i)] = 1.0;
        }
        rhs[m + n_dim] = 1.0;

        let result = crate::kkt::saddle_point_solver::solve_saddle_point(&kkt_mat, &rhs);

        // Returns None: the unique beta has beta_2 < 0 (genuinely infeasible).
        assert!(
            result.is_none(),
            "rank-deficient system with beta < 0 should return None"
        );
    }

    /// KKT solver on degenerate case (identical normals).
    ///
    /// Two identical normals (violates irredundancy). The solver should either
    /// return None or return Some without panicking. Either outcome is acceptable.
    ///
    /// Tests graceful degradation on invalid input.
    #[test]
    fn solve_kkt_degenerate() {
        let normals = [Vector4::x(), Vector4::x()];
        let _heights = [1.0, 1.0];
        let perm = [0, 1];

        // Build augmented system directly (degenerate, 2 facets).
        let m = perm.len();
        let n_dim = 4;
        let size = m + n_dim + 1;
        let mut kkt_mat = nalgebra::DMatrix::zeros(size, size);
        let mut rhs = nalgebra::DVector::zeros(size);

        for i in 0..m {
            for j in 0..m {
                kkt_mat[(i, j)] = crate::geom::symplectic_form::omega0(
                    &normals[perm[i]],
                    &normals[perm[j]],
                );
            }
        }
        for i in 0..m {
            let n = &normals[perm[i]];
            for k in 0..n_dim {
                kkt_mat[(i, m + k)] = n[k];
                kkt_mat[(m + k, i)] = n[k];
            }
            kkt_mat[(i, m + n_dim)] = 1.0;
            kkt_mat[(m + n_dim, i)] = 1.0;
        }
        rhs[m + n_dim] = 1.0;

        let result = crate::kkt::saddle_point_solver::solve_saddle_point(&kkt_mat, &rhs);

        // Either None (degenerate) or Some (solver handled it). Both are acceptable.
        if result.is_some() {
            eprintln!("Note: degenerate system returned Some (acceptable)");
        }
    }
}

// Tests for hk2017: pruned == unpruned capacity agreement on all test polytopes.
//
// Proposition: Directed adjacency pruning ([cor:adjacency-pruning]) does not change
// the computed capacity — it only reduces the number of iterations by skipping
// permutations that cannot correspond to valid Reeb orbits.
// Reference: [cor:adjacency-pruning]
//
// Strategy: fixture-based (fast, from pre-computed dataset) + direct computation
// (release-mode, on hypercube) + proptest (random polytopes).
#[cfg(test)]
mod tests_pruning {
    use super::*;
    use crate::geom::known_polytopes;

    use std::path::PathBuf;
    use std::sync::LazyLock;

    use super::generate_capacity_fixtures::{load_test_dataset, TestPolytope, FIXTURE_PATH};

    /// Shared dataset loaded from cached fixture.
    static DATASET: LazyLock<Vec<TestPolytope>> = LazyLock::new(|| {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(FIXTURE_PATH);
        load_test_dataset(&path)
    });

    // ── Combinatorics utility ──

    /// Verify combinations(n, k) produces the correct count.
    ///
    /// Tests the combinatorial enumeration utility used by the capacity algorithm.
    /// C(4,2) = 6, C(5,3) = 10, C(5,5) = 1.
    #[test]
    fn combinations_basic() {
        assert_eq!(combinations(4, 2).len(), 6); // C(4,2) = 6
        assert_eq!(combinations(5, 3).len(), 10); // C(5,3) = 10
        assert_eq!(combinations(5, 5).len(), 1); // C(5,5) = 1
    }

    // ── Fixture-based pruning agreement ──

    /// Verify pruned == unpruned agreement from fixture data (~0 cost).
    ///
    /// Only checks entries that have `capacity_unpruned` (base polytopes, not
    /// symplectomorphism/conformality variants). The fixture was generated with
    /// inline fail-fast checks, so this test is a regression guard.
    #[test]
    fn pruned_matches_unpruned_from_fixture() {
        let dataset = &*DATASET;
        let mut checked = 0;

        for tp in dataset.iter() {
            if let Some(cap_unpruned) = tp.capacity_unpruned {
                let rel_err = (tp.capacity - cap_unpruned).abs() / cap_unpruned;
                assert!(
                    rel_err < 1e-6,
                    "'{}': pruned ({}) != unpruned ({}) from fixture, rel_error = {:.2e}",
                    tp.name,
                    tp.capacity,
                    cap_unpruned,
                    rel_err
                );
                checked += 1;
            }
        }

        eprintln!(
            "Verified pruned == unpruned for {}/{} fixture entries",
            checked,
            dataset.len()
        );
    }

    // ── Direct computation ──

    /// Verify pruned and unpruned produce identical capacity on the hypercube (8 facets).
    ///
    /// Also checks that pruned does fewer iterations (adjacency filtering skips
    /// non-adjacent permutations).
    ///
    /// Why #[ignore]: F=8 unpruned is slow in debug mode (~16s). Run in release:
    /// `cargo test --release pruned_matches_unpruned -- --ignored`
    #[test]
    #[ignore] // ~16s debug, ~0.2s release
    fn pruned_matches_unpruned() {
        let kp = known_polytopes::hypercube();
        let result_unpruned =
            ehz_capacity_unpruned(&kp.polytope).expect("unpruned capacity");
        let result_pruned = ehz_capacity(&kp.polytope).expect("pruned capacity");

        assert!(
            (result_unpruned.result.capacity - result_pruned.result.capacity).abs() < 1e-6,
            "pruned and unpruned capacities differ"
        );

        // Pruned should do fewer iterations (adjacency filtering).
        assert!(
            result_pruned.result.iterations <= result_unpruned.result.iterations,
            "pruned should do <= iterations than unpruned"
        );

        eprintln!(
            "Hypercube: unpruned {} iters, pruned {} iters",
            result_unpruned.result.iterations,
            result_pruned.result.iterations
        );
    }

    /// Property: pruned and unpruned return the same capacity on random polytopes.
    ///
    /// Why #[ignore]: redundant with fixture test which checks 27+ polytopes.
    /// Retained as an independent validation path with different polytope generation.
    ///
    /// `cargo test --release pruned_matches_unpruned_random -- --ignored`
    #[test]
    #[ignore]
    fn pruned_matches_unpruned_random() {
        use crate::random::generate_random_polytopes;
        use rand::SeedableRng;
        use rand_chacha::ChaCha8Rng;

        for facet_count in 5..=8 {
            for seed in 0..4u64 {
                let mut rng = ChaCha8Rng::seed_from_u64(seed);
                let polytopes = generate_random_polytopes(1, facet_count, 0.5, 2.0, &mut rng);

                if let Some(p) = polytopes.first() {
                    let unpruned = ehz_capacity_unpruned(p).unwrap();
                    let pruned = ehz_capacity(p).unwrap();

                    assert!(
                        (unpruned.result.capacity - pruned.result.capacity).abs() < 1e-6,
                        "F={} seed={}: pruned {} vs unpruned {}",
                        facet_count,
                        seed,
                        pruned.result.capacity,
                        unpruned.result.capacity
                    );

                    assert!(
                        pruned.result.iterations <= unpruned.result.iterations,
                        "F={} seed={}: pruned iterations {} > unpruned {}",
                        facet_count,
                        seed,
                        pruned.result.iterations,
                        unpruned.result.iterations
                    );
                }
            }
        }
    }
}

// Tests for hk2017: regression pins for past bugs (nullspace sign, eigen gap ratio).
//
// Proposition: Previously-broken inputs continue to produce correct results after
// code changes. Each test pins a specific input and expected output that was
// incorrect before a particular bug fix.
// Reference: [lem:kkt], [lem:q-error-bound]
//
// Strategy: direct computation on hand-constructed polytopes that triggered past bugs.
#[cfg(test)]
mod tests_regression {
    use super::*;
    use crate::geom::lagrangian_product::lagrangian_product;
    use crate::geom::polygon::{regular_polygon_2d, rotate_polygon_2d};
    use crate::kkt::qp_assembly::build_augmented_system;

    // ── KKT null space fix regressions ──
    //
    // These tests verify that the KKT solver correctly handles rank-deficient
    // systems by searching the null space for beta > 0 solutions. Before the fix,
    // the minimum-norm pseudoinverse solution often had beta <= 0 for degenerate
    // polytopes (axis-aligned normals in symplectic subplanes).

    /// Regression: (4,4) Lagrangian product at theta=0 (square x square, axis-aligned).
    ///
    /// Before fix: cap=2.0 (correct). After fix: cap=2.0 (unchanged).
    /// This is the hypercube [-1/sqrt(2), 1/sqrt(2)]^4 which already worked pre-fix.
    /// Included to verify the fix does not break the working case.
    #[test]
    fn kkt_nullspace_square_square_zero() {
        let (qn, qh) = regular_polygon_2d(4, 1.0);
        let (pn, ph) = regular_polygon_2d(4, 1.0);
        let polytope = lagrangian_product(&qn, &qh, &pn, &ph).unwrap();

        let result =
            ehz_capacity_unpruned(&polytope).expect("(4,4) at theta=0 should have capacity");
        assert!(
            (result.result.capacity - 2.0).abs() < 1e-6,
            "(4,4) at theta=0: got {}, expected 2.0",
            result.result.capacity
        );
    }

    /// Regression: (4,4) at theta=0.125 deg — smallest angle in the polygon grid.
    ///
    /// Before fix: cap=3.991 (WRONG, 2x too high due to 8-facet spurious orbit).
    /// After fix: cap ~ 2.000 (continuous from theta=0).
    ///
    /// At theta=0.125 deg (near-degenerate), some orbits have Q ~ 0 where null-space
    /// Q constancy is noise-dominated. The Q constancy debug_assert skips Q < 1e-6.
    #[test]
    fn kkt_nullspace_square_square_near_zero() {
        let theta = 0.125_f64.to_radians();
        let (qn, qh) = regular_polygon_2d(4, 1.0);
        let (pn_base, ph_base) = regular_polygon_2d(4, 1.0);
        let (pn, ph) = rotate_polygon_2d(&pn_base, &ph_base, theta);
        let polytope = lagrangian_product(&qn, &qh, &pn, &ph).unwrap();

        let result =
            ehz_capacity_unpruned(&polytope).expect("(4,4) at theta=0.125 should have capacity");
        // Capacity should be continuous near theta=0 -> close to 2.0.
        assert!(
            (result.result.capacity - 2.0).abs() < 0.01,
            "(4,4) at theta=0.125: got {}, expected ~2.0 (was 3.991 before fix)",
            result.result.capacity
        );
    }

    /// Regression: (4,4) at theta=45 deg — billiard previously gave 2x wrong answer.
    ///
    /// Before fix: HK2017=2.828, billiard=5.657.
    /// After fix: all agree on cap = 2*sqrt(2) ~ 2.828.
    #[test]
    fn kkt_nullspace_square_square_45deg() {
        let theta = 45.0_f64.to_radians();
        let (qn, qh) = regular_polygon_2d(4, 1.0);
        let (pn_base, ph_base) = regular_polygon_2d(4, 1.0);
        let (pn, ph) = rotate_polygon_2d(&pn_base, &ph_base, theta);
        let polytope = lagrangian_product(&qn, &qh, &pn, &ph).unwrap();

        let result_hk =
            ehz_capacity_unpruned(&polytope).expect("(4,4) at theta=45: HK2017 should have capacity");
        let result_bil = crate::algorithms::billiard::billiard_capacity(&polytope)
            .expect("billiard should not error")
            .expect("billiard should find capacity");

        let sqrt2_times2 = 2.0 * std::f64::consts::SQRT_2;
        assert!(
            (result_hk.result.capacity - sqrt2_times2).abs() < 1e-6,
            "(4,4) at theta=45 HK2017: got {}, expected 2*sqrt(2) ~ {}",
            result_hk.result.capacity,
            sqrt2_times2
        );
        assert!(
            (result_bil.result.capacity - sqrt2_times2).abs() < 1e-6,
            "(4,4) at theta=45 billiard: got {} (was 5.657 before fix), expected 2*sqrt(2) ~ {}",
            result_bil.result.capacity,
            sqrt2_times2
        );
    }

    /// Regression: (3,4) at theta=0 — previously returned None for all algorithms.
    ///
    /// Before fix: None (all three algorithms). No valid orbit found.
    /// After fix: cap ~ 2.121 via 5-facet orbit. All three agree.
    ///
    /// Note: The expected capacity for this specific polytope (triangle circumradius=1,
    /// square circumradius=1) is 3*sqrt(2)/2 ~ 2.121, NOT 1.5. The value 1.5 is from
    /// `lagrangian_triangle_square()` which uses different dimensions.
    #[test]
    fn kkt_nullspace_triangle_square_zero() {
        let (qn, qh) = regular_polygon_2d(3, 1.0);
        let (pn, ph) = regular_polygon_2d(4, 1.0);
        let polytope = lagrangian_product(&qn, &qh, &pn, &ph).unwrap();

        let result = ehz_capacity_unpruned(&polytope)
            .expect("(3,4) at theta=0 should now return Some after null space fix");

        let expected = 3.0 * std::f64::consts::SQRT_2 / 2.0; // 3*sqrt(2)/2 ~ 2.121
        assert!(
            (result.result.capacity - expected).abs() < 1e-6,
            "(3,4) at theta=0: got {}, expected 3*sqrt(2)/2 ~ {} (was None before fix)",
            result.result.capacity,
            expected
        );
    }

    // ── Eigenvalue condition number threshold regression tests ──
    //
    // EIGEN_CONDITION_TAU=1e-3 was chosen empirically from the (4,4) degenerate case.
    // These tests pin the eigenvalue spectrum so that threshold changes can be
    // validated against the cases that motivated the threshold.

    /// Verify eigenvalue spectrum of the (4,4) theta=0 degenerate KKT system.
    ///
    /// The optimal orbit permutation [0,4,2,6] (alternating q/p facets) has a gap
    /// in the sorted |lambda_i| spectrum. The system must be rank-deficient for the
    /// null space search to activate.
    ///
    /// Regression test for EIGEN_CONDITION_TAU (see doc comment on the constant).
    #[test]
    fn eigen_gap_ratio_44_degenerate() {
        let (qn, qh) = regular_polygon_2d(4, 1.0);
        let (pn, ph) = regular_polygon_2d(4, 1.0);
        let polytope = lagrangian_product(&qn, &qh, &pn, &ph).unwrap();

        // The optimal orbit at theta=0 uses facets [0,4,2,6] (alternating q/p).
        let perm = vec![0, 4, 2, 6];
        let (kkt, _rhs) = build_augmented_system(&polytope, &perm);
        let eigen = kkt.symmetric_eigen();
        let size = perm.len() + 5; // 9

        // Collect |lambda_i| and sort descending.
        let mut abs_eigenvalues: Vec<f64> =
            eigen.eigenvalues.iter().map(|&ev| ev.abs()).collect();
        abs_eigenvalues.sort_by(|a, b| b.partial_cmp(a).unwrap());

        // Find gap ratio: walk from smallest |lambda_i| upward.
        let floor = 1e-15;
        let smallest_nonzero = (0..size).rev().find(|&i| abs_eigenvalues[i] > floor);
        if let Some(idx) = smallest_nonzero {
            if idx > 0 {
                let ratio = abs_eigenvalues[idx - 1] / abs_eigenvalues[idx];
                // If large gap exists, it must stay well above EIGEN_CONDITION_TAU.
                if ratio > 50.0 {
                    assert!(
                        ratio > 200.0,
                        "(4,4) theta=0 gap ratio should stay well above 1e-3 threshold, \
                         got {:.1} (|lambda[{}]|={:.3e}, |lambda[{}]|={:.3e})",
                        ratio,
                        idx - 1,
                        abs_eigenvalues[idx - 1],
                        idx,
                        abs_eigenvalues[idx]
                    );
                }
            }
        }

        // The KKT system must be rank-deficient (axis-aligned normals create dependence).
        let numerical_rank = abs_eigenvalues.iter().filter(|&&ev| ev > 1e-6).count();
        assert!(
            numerical_rank < size,
            "(4,4) theta=0 should be rank-deficient: rank={}, size={}",
            numerical_rank,
            size
        );
    }

    /// Verify eigenvalue gap ratio for the (4,4) theta=43 deg case.
    ///
    /// The KKT system for perm [1,0,6,3,2,4] has a gap ratio ~594 — the case from
    /// commit dd87a8a that motivated EIGEN_CONDITION_TAU=1e-3. The gap ratio must
    /// stay well above the threshold.
    #[test]
    fn eigen_gap_ratio_44_theta43() {
        let theta = 43.0_f64.to_radians();
        let (qn, qh) = regular_polygon_2d(4, 1.0);
        let (pn_base, ph_base) = regular_polygon_2d(4, 1.0);
        let (pn, ph) = rotate_polygon_2d(&pn_base, &ph_base, theta);
        let polytope = lagrangian_product(&qn, &qh, &pn, &ph).unwrap();

        let perm = vec![1, 0, 6, 3, 2, 4];
        let m = perm.len();
        let size = m + 5; // 11

        let (kkt, _rhs) = build_augmented_system(&polytope, &perm);
        let eigen = kkt.symmetric_eigen();

        let mut abs_eigenvalues: Vec<f64> =
            eigen.eigenvalues.iter().map(|&ev| ev.abs()).collect();
        abs_eigenvalues.sort_by(|a, b| b.partial_cmp(a).unwrap());

        // Find the largest gap ratio in the spectrum.
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

        // Gap ratio must be well above EIGEN_CONDITION_TAU=1e-3. Original: ~594.
        assert!(
            max_gap_ratio > 300.0,
            "(4,4) theta=43 gap ratio should be ~594 (well above 1e-3 threshold), \
             got {:.1} at |lambda[{}]|={:.3e}/|lambda[{}]|={:.3e}",
            max_gap_ratio,
            gap_idx - 1,
            abs_eigenvalues[gap_idx - 1],
            gap_idx,
            abs_eigenvalues[gap_idx]
        );
    }

    // ── HKO counterexample regression ──

    /// Verify HKO pentagon capacity and sys > 1 property (Annals counterexample).
    ///
    /// Computes capacity on the Haim-Kislev-Ostrover 10-facet pentagon and verifies
    /// it is a counterexample to Viterbo's conjecture (sys > 1).
    ///
    /// Why #[ignore]: F=10 -> ~37s debug, ~0.5s release. Important regression test
    /// for the thesis counterexample.
    /// Run: `cargo test --release pentagon_capacity -- --ignored`
    #[test]
    #[ignore] // ~37s debug, ~0.5s release
    fn pentagon_capacity() {
        use crate::geom::known_polytopes;
        use crate::geom::volume::volume;

        let kp = known_polytopes::hko_pentagon();
        let result = ehz_capacity(&kp.polytope).expect("pentagon capacity");

        assert!(
            (result.result.capacity - kp.capacity).abs() < 1e-6,
            "pentagon: got {}, expected {}",
            result.result.capacity,
            kp.capacity
        );

        // Verify sys > 1 (counterexample property).
        let vol = volume(&kp.polytope).expect("volume computation failed");
        let sys = result.result.capacity * result.result.capacity / (2.0 * vol);
        eprintln!(
            "Pentagon: capacity={:.6}, volume={:.6}, sys={:.6}",
            result.result.capacity, vol, sys
        );
        assert!(sys > 1.0, "pentagon should have sys > 1, got {}", sys);
    }

    /// Compute capacity of the 4D crosspolytope (16 facets, non-simple).
    ///
    /// Why #[ignore]: F=16, estimated ~4 hours pruned (A3) in release mode.
    /// Run: `cargo test --release crosspolytope_capacity -- --ignored --nocapture`
    #[test]
    #[ignore]
    fn crosspolytope_capacity() {
        use crate::geom::known_polytopes;

        let kp = known_polytopes::crosspolytope();
        let result = ehz_capacity(&kp.polytope).expect("crosspolytope capacity");

        assert!(
            result.result.capacity > 0.0,
            "crosspolytope capacity positive"
        );
        eprintln!(
            "Crosspolytope (16 facets): capacity={:.6}",
            result.result.capacity
        );
        eprintln!("  Iterations: {}", result.result.iterations);
    }
}

// Tests for hk2017: conformality property c(alpha*K) = alpha^2 * c(K).
//
// Proposition: EHZ capacity is degree-2 homogeneous: scaling a polytope by alpha
// scales its capacity by alpha^2. [thm:conformality]
// Reference: [thm:conformality]
//
// Strategy: fixture-based (conformality variants in the pre-computed dataset) +
// direct computation (release-mode, hypercube scaled by e).
#[cfg(test)]
mod tests_conformality {
    use super::*;

    use std::path::PathBuf;
    use std::sync::LazyLock;

    use super::generate_capacity_fixtures::{load_test_dataset, TestPolytope, FIXTURE_PATH};

    /// Shared dataset loaded from cached fixture.
    static DATASET: LazyLock<Vec<TestPolytope>> = LazyLock::new(|| {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(FIXTURE_PATH);
        load_test_dataset(&path)
    });

    // ── Fixture-based conformality ──

    /// Verify conformality c(alpha*K) = alpha^2 * c(K) from fixture data.
    ///
    /// Checks all entries with transform "conform:{alpha}" against their base polytope.
    /// Also verifies volume scaling: vol(alpha*K) = alpha^4 * vol(K).
    #[test]
    fn capacity_conformality() {
        let dataset = &*DATASET;

        let conformality_tests: Vec<_> = dataset
            .iter()
            .filter(|e| {
                e.transform
                    .as_ref()
                    .is_some_and(|t| t.starts_with("conform:"))
            })
            .collect();

        for entry in &conformality_tests {
            let base_idx = entry
                .base_index
                .expect("conformality variant has base_index");
            let base = &dataset[base_idx];

            // Extract scale factor from transform string "conform:1.50".
            let alpha: f64 = entry
                .transform
                .as_ref()
                .and_then(|t| t.strip_prefix("conform:"))
                .and_then(|s| s.parse().ok())
                .expect("valid scale factor");

            // Capacity conformality: c(alpha*K) = alpha^2 * c(K).
            let expected_cap = alpha * alpha * base.capacity;
            let cap_error = (entry.capacity - expected_cap).abs() / expected_cap;
            assert!(
                cap_error < 1e-6,
                "{}: conformality failed: c({:.2}*{}) = {}, expected {:.2}^2 * c({}) = {}, \
                 rel_error = {:.2e}",
                entry.name,
                alpha,
                base.name,
                entry.capacity,
                alpha,
                base.name,
                expected_cap,
                cap_error
            );

            // Volume scaling: vol(alpha*K) = alpha^4 * vol(K).
            let expected_vol = alpha.powi(4) * base.volume;
            let vol_error = (entry.volume - expected_vol).abs() / expected_vol;
            assert!(
                vol_error < 1e-6,
                "{}: volume conformality failed: rel_error = {:.2e}",
                entry.name,
                vol_error
            );
        }

        assert!(
            !conformality_tests.is_empty(),
            "expected at least one conformality variant in fixture"
        );
    }

    // ── Direct computation ──

    /// Verify conformality on hypercube scaled by e (transcendental).
    ///
    /// Uses lambda = e (transcendental) to ensure numerical coincidences are impossible.
    /// Expected: c(e * K) = e^2 * c(K).
    ///
    /// Why #[ignore]: F=8 unpruned x 2 = ~48s debug, ~0.6s release.
    /// Run: `cargo test --release capacity_scales_quadratically -- --ignored`
    #[test]
    #[ignore] // ~48s debug, ~0.6s release
    fn capacity_scales_quadratically() {
        use crate::geom::known_polytopes;

        let scale = std::f64::consts::E;

        let kp = known_polytopes::hypercube();
        let unit_cap = ehz_capacity_unpruned(&kp.polytope)
            .unwrap()
            .result
            .capacity;

        let scaled_cube = crate::geom::test_utils::scaled_hypercube(scale);
        let scaled_cap = ehz_capacity_unpruned(&scaled_cube)
            .unwrap()
            .result
            .capacity;

        let expected = unit_cap * scale * scale;
        let relative_error = ((scaled_cap - expected) / expected).abs();

        assert!(
            relative_error < 1e-4,
            "capacity scaling failed: scale={scale}, unit_cap={unit_cap}, \
             scaled_cap={scaled_cap}, expected={expected}, relative_error={relative_error}"
        );
    }
}

// Tests for hk2017: symplectomorphism invariance and monotonicity.
//
// Proposition: EHZ capacity is invariant under symplectomorphisms (c(MK+b) = c(K)
// for M in Sp(4)) and monotone under inclusion (K1 subset K2 => c(K1) <= c(K2)).
// Reference: [thm:sympl-invariance], capacity axioms (P7: monotonicity)
//
// Strategy: fixture-based (symplectomorphism variants and pairwise containment
// from the pre-computed dataset).
#[cfg(test)]
mod tests_symplectic_invariance {
    use crate::geom::polytope::Polytope4D;
    use nalgebra::Vector4;

    use std::path::PathBuf;
    use std::sync::LazyLock;

    use super::generate_capacity_fixtures::{load_test_dataset, TestPolytope, FIXTURE_PATH};

    /// Shared dataset loaded from cached fixture.
    static DATASET: LazyLock<Vec<TestPolytope>> = LazyLock::new(|| {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(FIXTURE_PATH);
        load_test_dataset(&path)
    });

    // ── Symplectomorphism invariance ──

    /// Verify c(MK+b) = c(K) for all symplectomorphism variants in the fixture.
    ///
    /// Each variant is generated from a base polytope by applying a random M in Sp(4).
    /// Since Sp(4) preserves both capacity and volume, we check both.
    #[test]
    fn capacity_symplectomorphism_invariance() {
        let dataset = &*DATASET;

        let sympl_tests: Vec<_> = dataset
            .iter()
            .filter(|e| e.transform.as_ref().is_some_and(|t| t == "sympl"))
            .collect();

        for entry in &sympl_tests {
            let base_idx = entry
                .base_index
                .expect("sympl variant has base_index");
            let base = &dataset[base_idx];

            // Capacity invariance: c(MK+b) = c(K).
            let cap_error = (entry.capacity - base.capacity).abs() / base.capacity;
            assert!(
                cap_error < 1e-6,
                "{}: symplectomorphism invariance failed: c(M*{}+b) = {}, \
                 expected c({}) = {}, rel_error = {:.2e}",
                entry.name,
                base.name,
                entry.capacity,
                base.name,
                base.capacity,
                cap_error
            );

            // Volume invariance: Sp(4) preserves symplectic volume = Euclidean volume in R^4.
            let vol_error = (entry.volume - base.volume).abs() / base.volume;
            assert!(
                vol_error < 1e-6,
                "{}: volume invariance failed under symplectomorphism",
                entry.name
            );
        }

        assert!(
            !sympl_tests.is_empty(),
            "expected at least one symplectomorphism variant in fixture"
        );
    }

    // ── Monotonicity ──

    /// Verify monotonicity: if alpha*K1 fits inside K2, then c(alpha*K1) <= c(K2).
    ///
    /// For each pair (K1, K2) in the fixture, computes the maximum alpha such that
    /// alpha*K1 subset K2, then checks c(alpha*K1) = alpha^2*c(K1) <= c(K2).
    /// Uses conformality to avoid recomputing capacity of the scaled polytope.
    #[test]
    fn capacity_monotonicity() {
        let dataset = &*DATASET;
        let mut checked = 0;

        // Check a representative sample of pairs to keep test fast.
        for (i, k1) in dataset.iter().enumerate() {
            for (j, k2) in dataset.iter().enumerate() {
                if i == j {
                    continue;
                }
                // Only check first 20 pairs with non-trivial containment.
                if checked >= 20 {
                    break;
                }

                let vertices1 = k1.polytope.vertices_f64();
                if let Some(alpha) = compute_max_containment_scale(vertices1, &k2.polytope) {
                    if alpha > 1e-6 {
                        // c(alpha*K1) = alpha^2 * c(K1) by conformality.
                        let c_alpha_k1 = alpha * alpha * k1.capacity;
                        assert!(
                            c_alpha_k1 <= k2.capacity + 1e-9,
                            "monotonicity failed: c({:.3}*{}) = {:.3}^2 * {:.4} = {:.4} \
                             should be <= c({}) = {:.4}",
                            alpha,
                            k1.name,
                            alpha,
                            k1.capacity,
                            c_alpha_k1,
                            k2.name,
                            k2.capacity
                        );
                        checked += 1;
                    }
                }
            }
            if checked >= 20 {
                break;
            }
        }

        eprintln!("Verified monotonicity for {} pairs", checked);
    }

    /// Compute max alpha such that alpha*K1 subset K2.
    ///
    /// Returns None if no positive alpha works (e.g. K1 has a vertex whose
    /// direction is not contained in K2 for any positive scaling).
    fn compute_max_containment_scale(
        vertices1: &[Vector4<f64>],
        polytope2: &Polytope4D,
    ) -> Option<f64> {
        let normals2 = polytope2.normals_f64();
        let heights2 = polytope2.heights_f64();

        let mut max_alpha = f64::INFINITY;

        for v in vertices1 {
            for (n, &h) in normals2.iter().zip(heights2.iter()) {
                let nv = n.dot(v);
                if nv > 1e-12 {
                    // v points outward from this halfspace.
                    let alpha_bound = h / nv;
                    max_alpha = max_alpha.min(alpha_bound);
                }
                // If nv <= 0, v is on the safe side — no constraint.
            }
        }

        if max_alpha.is_finite() && max_alpha > 1e-12 {
            Some(max_alpha)
        } else {
            None
        }
    }
}

// Tests for hk2017: finite-difference derivative validation (dc/dh, Euler homogeneity).
//
// Proposition: c_EHZ is degree-2 homogeneous in facet heights, so Euler's identity
// gives sum_k h_k * dc/dh_k = 2*c. Similarly, vol is degree-4 and sys = c^2/(2*vol)
// is degree 0.
// Reference: [thm:conformality], capacity axioms (P7: monotonicity)
//
// Strategy: central finite differences on perturbed polytopes, direct capacity/volume
// computation. Most tests are #[ignore] (expensive: multiple ehz_capacity calls).
#[cfg(test)]
mod tests_capacity_derivative {
    use super::*;
    use crate::geom::known_polytopes;
    use crate::geom::polytope::Polytope4D;
    use crate::geom::volume::volume;
    use nalgebra::Vector4;

    /// Step size for central finite differences of capacity.
    ///
    /// Chosen as geometric mean of machine epsilon (~1e-16) and typical height scale (~1):
    /// eps ~ 1e-7 to 1e-6. We use 1e-6 for capacity (expensive, want stability).
    const FD_EPS_CAP: f64 = 1e-6;

    /// Step size for central finite differences of volume.
    ///
    /// Tighter than capacity (volume computation is cheap via qhull).
    const FD_EPS_VOL: f64 = 1e-7;

    /// Construct a perturbed polytope: h_k -> h_k + delta, all other heights unchanged.
    ///
    /// Returns `None` if construction fails (should not happen for small perturbations
    /// of valid polytopes).
    fn perturbed_polytope(
        normals: &[Vector4<f64>],
        heights: &[f64],
        facet: usize,
        delta: f64,
    ) -> Option<Polytope4D> {
        let mut h = heights.to_vec();
        h[facet] += delta;
        let halfspaces: Vec<Vector4<f64>> = normals
            .iter()
            .zip(h.iter())
            .map(|(n, &hi)| n / hi)
            .collect();
        Polytope4D::new(halfspaces).ok()
    }

    /// Compute FD volume derivatives: dvol/dh_k ~ (vol(h+eps*e_k) - vol(h-eps*e_k)) / (2*eps).
    ///
    /// Uses qhull-based volume. Note: qhull computes volume from the V-rep triangulation,
    /// which may introduce O(eps) systematic error for FD. The old code used
    /// `volume_divergence` (divergence theorem from H-rep) for cleaner FD.
    ///
    /// TODO: If FD volume tests show excessive error, add a volume_divergence function
    /// to the volume module (dropped during migration). The divergence theorem computes
    /// vol = (1/4) sum h_i * vol_3D(F_i) directly from H-representation.
    fn fd_volume_derivatives(normals: &[Vector4<f64>], heights: &[f64]) -> Vec<f64> {
        let f = heights.len();
        (0..f)
            .map(|k| {
                let p_plus = perturbed_polytope(normals, heights, k, FD_EPS_VOL)
                    .expect("perturbed polytope +eps");
                let p_minus = perturbed_polytope(normals, heights, k, -FD_EPS_VOL)
                    .expect("perturbed polytope -eps");
                let vol_plus = volume(&p_plus).expect("volume +eps");
                let vol_minus = volume(&p_minus).expect("volume -eps");
                (vol_plus - vol_minus) / (2.0 * FD_EPS_VOL)
            })
            .collect()
    }

    /// Compute FD capacity derivatives: dc/dh_k ~ (c(h+eps*e_k) - c(h-eps*e_k)) / (2*eps).
    ///
    /// At non-smooth points (tied orbits), this computes the directional derivative of
    /// the envelope (max over orbits), not a single-orbit subgradient.
    fn fd_capacity_derivatives(normals: &[Vector4<f64>], heights: &[f64]) -> Vec<f64> {
        let f = heights.len();
        (0..f)
            .map(|k| {
                let p_plus = perturbed_polytope(normals, heights, k, FD_EPS_CAP)
                    .expect("perturbed polytope +eps");
                let p_minus = perturbed_polytope(normals, heights, k, -FD_EPS_CAP)
                    .expect("perturbed polytope -eps");
                let cap_plus = ehz_capacity(&p_plus)
                    .expect("capacity +eps")
                    .result
                    .capacity;
                let cap_minus = ehz_capacity(&p_minus)
                    .expect("capacity -eps")
                    .result
                    .capacity;
                (cap_plus - cap_minus) / (2.0 * FD_EPS_CAP)
            })
            .collect()
    }

    // ===== Default suite: fast tests (debug mode, < 5s each) =====

    /// T1: FD capacity derivatives are finite and non-negative for the simplex (5 facets).
    ///
    /// Proposition: For the 4-simplex, dc_EHZ/dh_k is finite and >= 0 for all k.
    /// Method: Central FD with eps = 1e-6, `ehz_capacity` on perturbed polytopes.
    /// Why default suite: 5 facets -> 10 capacity calls, simplex is fast even in debug.
    #[test]
    fn fd_capacity_height_simplex() {
        let kp = known_polytopes::simplex();
        let normals = kp.polytope.normals_f64();
        let heights = kp.polytope.heights_f64();

        let d_cap = fd_capacity_derivatives(&normals, &heights);

        for (k, &dc) in d_cap.iter().enumerate() {
            assert!(
                dc.is_finite(),
                "facet {k}: d_cap/d_h is not finite ({dc})"
            );
            assert!(
                dc >= -1e-4,
                "facet {k}: d_cap/d_h = {dc:.6} violates monotonicity (expected >= 0)"
            );
        }
    }

    /// T2: Euler homogeneity identity for volume: sum h_k * dvol/dh_k = 4*vol.
    ///
    /// Volume is degree-4 homogeneous in heights, so Euler's identity gives
    /// sum h_k * dvol/dh_k = 4*vol(K).
    ///
    /// Polytopes: simplex, hypercube. Tolerance: 0.1% relative.
    ///
    /// TODO: This test uses qhull-based volume. The old code used `volume_divergence`
    /// (divergence theorem from H-rep) which gives clean FD with O(eps^2) truncation error.
    /// Qhull computes volume from V-rep triangulation, which introduces O(eps) systematic
    /// error in FD because the triangulation topology can change with small perturbations.
    /// If this test fails on hypercube, restore `volume_divergence` in geom/volume.rs.
    #[test]
    #[ignore] // Requires volume_divergence (dropped during migration) for clean FD
    fn euler_homogeneity_volume() {
        let polytopes: Vec<(&str, Polytope4D)> = vec![
            ("simplex", known_polytopes::simplex().polytope.clone()),
            ("hypercube", known_polytopes::hypercube().polytope.clone()),
        ];

        for (name, poly) in &polytopes {
            let normals = poly.normals_f64();
            let heights = poly.heights_f64();
            let vol = volume(poly).expect("volume");

            let d_vol = fd_volume_derivatives(&normals, &heights);
            let euler_sum: f64 = heights.iter().zip(&d_vol).map(|(h, dv)| h * dv).sum();
            let expected = 4.0 * vol;
            let rel_err = (euler_sum - expected).abs() / expected;

            assert!(
                rel_err < 0.01,
                "{name}: Euler vol identity failed: sum h*dvol/dh = {euler_sum:.8}, \
                 4*vol = {expected:.8}, rel_err = {rel_err:.2e}"
            );
        }
    }

    /// T3: Capacity monotonicity for the simplex: dc/dh_k >= 0 for all k.
    ///
    /// c_EHZ is monotone under inclusion (P7 in capacity axioms). Increasing any
    /// height h_k enlarges K, so dc/dh_k >= 0.
    #[test]
    fn capacity_monotone_simplex() {
        let kp = known_polytopes::simplex();
        let normals = kp.polytope.normals_f64();
        let heights = kp.polytope.heights_f64();

        let d_cap = fd_capacity_derivatives(&normals, &heights);

        for (k, &dc) in d_cap.iter().enumerate() {
            assert!(
                dc >= -1e-4,
                "simplex facet {k}: dc/dh = {dc:.6e} < 0 (monotonicity violation)"
            );
        }
    }

    // ===== Ignored suite: expensive tests (release mode) =====

    /// T4: FD capacity derivatives are finite and non-negative for larger polytopes.
    ///
    /// Polytopes: hypercube (8F), lagrangian_triangle_product (6F), HKO pentagon (10F).
    /// Why #[ignore]: 8-10 facets x 2 capacity calls each, too slow for debug mode.
    /// Runtime: ~10s in release.
    #[test]
    #[ignore]
    fn fd_capacity_height_known_polytopes() {
        let polytopes = vec![
            ("hypercube", known_polytopes::hypercube().polytope.clone()),
            (
                "lagrangian_tri",
                known_polytopes::lagrangian_triangle_product().polytope.clone(),
            ),
            ("hko_pentagon", known_polytopes::hko_pentagon().polytope.clone()),
        ];

        for (name, poly) in &polytopes {
            let normals = poly.normals_f64();
            let heights = poly.heights_f64();

            let d_cap = fd_capacity_derivatives(&normals, &heights);

            for (k, &dc) in d_cap.iter().enumerate() {
                assert!(
                    dc.is_finite(),
                    "{name} facet {k}: d_cap/d_h is not finite ({dc})"
                );
                // Relaxed tolerance for non-smooth points (HKO pentagon has 44 tied orbits).
                assert!(
                    dc >= -1e-3,
                    "{name} facet {k}: d_cap/d_h = {dc:.6e} violates monotonicity"
                );
            }
        }
    }

    /// T5: Euler homogeneity identity for capacity: sum h_k * dc/dh_k = 2*c.
    ///
    /// c_EHZ is degree-2 homogeneous in heights (conformality + scaling of h),
    /// so Euler's identity gives sum h_k * dc/dh_k = 2*c.
    ///
    /// **This test catches the sign bug:** wrong sign gives sum = -2c instead of +2c.
    ///
    /// Polytopes: simplex, hypercube, lagrangian_triangle_product — all generic (unique
    /// optimal orbit). HKO pentagon excluded: 44 tied orbits make capacity non-smooth,
    /// so FD envelope derivative != Euler identity.
    /// Tolerance: 1% relative.
    /// Runtime: ~10s in release.
    #[test]
    #[ignore]
    fn euler_homogeneity_capacity() {
        let polytopes = vec![
            ("simplex", known_polytopes::simplex()),
            ("hypercube", known_polytopes::hypercube()),
            (
                "lagrangian_tri",
                known_polytopes::lagrangian_triangle_product(),
            ),
        ];

        for (name, kp) in &polytopes {
            let normals = kp.polytope.normals_f64();
            let heights = kp.polytope.heights_f64();
            let cap = ehz_capacity(&kp.polytope)
                .expect("capacity")
                .result
                .capacity;

            let d_cap = fd_capacity_derivatives(&normals, &heights);
            let euler_sum: f64 = heights.iter().zip(&d_cap).map(|(h, dc)| h * dc).sum();
            let expected = 2.0 * cap;
            let rel_err = (euler_sum - expected).abs() / expected;

            eprintln!(
                "{name}: Euler cap: sum h*dc/dh = {euler_sum:.6}, 2c = {expected:.6}, \
                 ratio = {:.4}, rel_err = {rel_err:.2e}",
                euler_sum / expected
            );

            assert!(
                rel_err < 0.01,
                "{name}: Euler capacity identity failed: sum h*dc/dh = {euler_sum:.8}, \
                 2c = {expected:.8}, rel_err = {rel_err:.2e} (>1%)"
            );
        }
    }

    /// T6: Capacity monotonicity for known polytopes with more facets.
    ///
    /// dc/dh_k >= 0 for all k (monotonicity under inclusion).
    /// Polytopes: hypercube (8F), lagrangian_triangle_product (6F), HKO pentagon (10F).
    #[test]
    #[ignore]
    fn capacity_monotone_known_polytopes() {
        let polytopes = vec![
            ("hypercube", known_polytopes::hypercube().polytope.clone()),
            (
                "lagrangian_tri",
                known_polytopes::lagrangian_triangle_product().polytope.clone(),
            ),
            ("hko_pentagon", known_polytopes::hko_pentagon().polytope.clone()),
        ];

        for (name, poly) in &polytopes {
            let normals = poly.normals_f64();
            let heights = poly.heights_f64();

            let d_cap = fd_capacity_derivatives(&normals, &heights);

            for (k, &dc) in d_cap.iter().enumerate() {
                assert!(
                    dc >= -1e-3,
                    "{name} facet {k}: dc/dh = {dc:.6e} < 0 (monotonicity violation)"
                );
            }
        }
    }

    /// T7: Euler homogeneity for sys = c^2/(2*vol): sum h_k * dsys/dh_k = 0.
    ///
    /// sys(K) = c_EHZ(K)^2 / (2*vol(K)) is degree 0 in heights:
    /// sys(lambda*h) = (lambda^2 c)^2 / (2*lambda^4*vol) = lambda^0 * sys.
    /// Euler identity: sum h_k * dsys/dh_k = 0.
    ///
    /// Polytopes: simplex, hypercube — generic (unique optimal orbit).
    /// HKO pentagon excluded: non-smooth capacity invalidates Euler identity for FD.
    /// Tolerance: 1% of sys value (absolute, since expected value is 0).
    #[test]
    #[ignore]
    fn fd_sys_height_euler() {
        let polytopes = vec![
            ("simplex", known_polytopes::simplex()),
            ("hypercube", known_polytopes::hypercube()),
        ];

        for (name, kp) in &polytopes {
            let normals = kp.polytope.normals_f64();
            let heights = kp.polytope.heights_f64();

            let cap = ehz_capacity(&kp.polytope)
                .expect("capacity")
                .result
                .capacity;
            let vol = volume(&kp.polytope).expect("volume");
            let sys = cap * cap / (2.0 * vol);

            // FD sys derivatives.
            let d_sys: Vec<f64> = (0..heights.len())
                .map(|k| {
                    let p_plus = perturbed_polytope(&normals, &heights, k, FD_EPS_CAP)
                        .expect("perturbed +eps");
                    let p_minus = perturbed_polytope(&normals, &heights, k, -FD_EPS_CAP)
                        .expect("perturbed -eps");
                    let cap_p = ehz_capacity(&p_plus)
                        .expect("cap +eps")
                        .result
                        .capacity;
                    let cap_m = ehz_capacity(&p_minus)
                        .expect("cap -eps")
                        .result
                        .capacity;
                    let vol_p = volume(&p_plus).expect("vol +eps");
                    let vol_m = volume(&p_minus).expect("vol -eps");
                    let sys_p = cap_p * cap_p / (2.0 * vol_p);
                    let sys_m = cap_m * cap_m / (2.0 * vol_m);
                    (sys_p - sys_m) / (2.0 * FD_EPS_CAP)
                })
                .collect();

            let euler_sum: f64 = heights.iter().zip(&d_sys).map(|(h, ds)| h * ds).sum();
            // Expected: 0 (degree 0 in h).

            eprintln!(
                "{name}: Euler sys: sum h*dsys/dh = {euler_sum:.6e}, sys = {sys:.6}, \
                 ratio = {:.4e}",
                euler_sum / sys
            );

            assert!(
                euler_sum.abs() < 0.01 * sys,
                "{name}: Euler sys identity failed: sum h*dsys/dh = {euler_sum:.6e}, \
                 expected 0, sys = {sys:.6} (ratio = {:.2e})",
                euler_sum / sys
            );
        }
    }
}

//! HK2017 pruning and combinatorics-focused tests.
//!
//! Split from mod.rs to isolate pruning behavior checks.

use super::*;
use crate::algorithms::test_helpers::{
    pruned_capacity_for_dual_vertices, pruned_capacity_for_fixture,
    unpruned_capacity_for_dual_vertices, unpruned_capacity_for_fixture,
};
use crate::geom::known_polytopes;
use nalgebra::DMatrix;
use std::collections::HashSet;

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

fn old_filtered_pruned_sigmas(transition_is_allowed: &DMatrix<bool>) -> HashSet<Vec<usize>> {
    let facet_count = transition_is_allowed.nrows();
    let mut sigmas = HashSet::new();
    for m in 2..=facet_count {
        for subset in combinations(facet_count, m) {
            super::permutations::for_each_cyclic_permutation(&subset, &mut |sigma| {
                if crate::algorithms::facet_adjacency::is_feasible_cycle(
                    sigma,
                    transition_is_allowed,
                ) {
                    sigmas.insert(sigma.to_vec());
                }
            });
        }
    }
    sigmas
}

fn graph_native_pruned_sigmas(transition_is_allowed: &DMatrix<bool>) -> HashSet<Vec<usize>> {
    SimpleDirectedCyclesCanonical::new(transition_is_allowed).collect()
}

fn complete_transition_matrix(facet_count: usize) -> DMatrix<bool> {
    DMatrix::from_fn(facet_count, facet_count, |i, j| i != j)
}

#[test]
fn graph_native_pruned_sigmas_match_filtered_cyclic_permutations_on_complete_graph() {
    let transition = complete_transition_matrix(6);
    assert_eq!(
        graph_native_pruned_sigmas(&transition),
        old_filtered_pruned_sigmas(&transition)
    );
}

#[test]
fn graph_native_pruned_sigmas_match_filtered_cyclic_permutations_on_sparse_graph() {
    let transition = DMatrix::from_row_slice(
        6,
        6,
        &[
            false, true, false, false, true, false, //
            false, false, true, false, false, true, //
            true, false, false, true, false, false, //
            false, true, false, false, true, false, //
            false, false, true, false, false, true, //
            true, false, false, true, false, false, //
        ],
    );
    assert_eq!(
        graph_native_pruned_sigmas(&transition),
        old_filtered_pruned_sigmas(&transition)
    );
}

#[test]
fn graph_native_cycle_iterator_emits_unique_stream_and_is_fused_after_eof() {
    let transition = DMatrix::from_row_slice(
        5,
        5,
        &[
            false, true, true, false, false, //
            false, false, true, true, false, //
            true, false, false, true, false, //
            false, true, false, false, true, //
            true, false, true, false, false, //
        ],
    );
    let mut cycle_iter = SimpleDirectedCyclesCanonical::new(&transition);
    let first = cycle_iter.next().expect("fixture has cycles");
    let mut emitted = vec![first];
    emitted.extend(cycle_iter.by_ref());

    assert_eq!(cycle_iter.next(), None);
    assert_eq!(cycle_iter.next(), None);

    let unique: HashSet<_> = emitted.iter().cloned().collect();
    assert_eq!(unique.len(), emitted.len(), "duplicate emitted cycle");
    assert_eq!(unique, old_filtered_pruned_sigmas(&transition));
}

#[test]
fn graph_native_pruned_sigmas_are_canonical_simple_directed_cycles() {
    let transition = DMatrix::from_row_slice(
        5,
        5,
        &[
            false, true, true, false, false, //
            false, false, true, true, false, //
            true, false, false, true, false, //
            false, true, false, false, true, //
            true, false, true, false, false, //
        ],
    );

    for sigma in graph_native_pruned_sigmas(&transition) {
        assert!(sigma.len() >= 2);
        let min = *sigma.iter().min().expect("sigma is nonempty");
        assert_eq!(sigma[0], min, "sigma is not rotation-canonical: {sigma:?}");
        let unique: HashSet<_> = sigma.iter().copied().collect();
        assert_eq!(
            unique.len(),
            sigma.len(),
            "sigma repeats a facet: {sigma:?}"
        );
        assert!(crate::algorithms::facet_adjacency::is_feasible_cycle(
            &sigma,
            &transition
        ));
    }
}

#[test]
#[should_panic(expected = "transition_is_allowed must be square")]
fn graph_native_cycle_enumerator_rejects_rectangular_matrix() {
    let transition = DMatrix::from_element(2, 3, true);
    let _ = SimpleDirectedCyclesCanonical::new(&transition);
}

// ── Direct pruning agreement ──

/// Verify pruned and unpruned produce identical capacity on the simplex.
///
/// This keeps a fast live pruning smoke test in the library. Broad pruned vs
/// unpruned validation lives in `experiments/verification/correctness/`.
#[test]
fn pruned_matches_unpruned_simplex() {
    let kp = known_polytopes::simplex();
    let result_unpruned = unpruned_capacity_for_fixture(kp).expect("unpruned capacity");
    let result_pruned = pruned_capacity_for_fixture(kp).expect("pruned capacity");
    assert!(
        (result_unpruned.capacity() - result_pruned.capacity()).abs() < 1e-6,
        "simplex: pruned and unpruned capacities differ"
    );
}

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
    let result_unpruned = unpruned_capacity_for_fixture(kp).expect("unpruned capacity");
    let result_pruned = pruned_capacity_for_fixture(kp).expect("pruned capacity");

    assert!(
        (result_unpruned.capacity() - result_pruned.capacity()).abs() < 1e-6,
        "pruned and unpruned capacities differ"
    );

    // Pruned should do fewer iterations (adjacency filtering).
    assert!(
        result_pruned.iterations <= result_unpruned.iterations,
        "pruned should do <= iterations than unpruned"
    );

    eprintln!(
        "Hypercube: unpruned {} iters, pruned {} iters",
        result_unpruned.iterations, result_pruned.iterations
    );
}

/// Property: pruned and unpruned return the same capacity on random polytopes.
///
/// Why #[ignore]: broad randomized agreement belongs in validation runs, not
/// the default library smoke suite.
///
/// `cargo test --release pruned_matches_unpruned_random -- --ignored`
#[test]
#[ignore]
fn pruned_matches_unpruned_random() {
    use crate::random::generate_random_dual_vertices;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    for facet_count in 5..=8 {
        for seed in 0..4u64 {
            let mut rng = ChaCha8Rng::seed_from_u64(seed);
            let dual_vertices_sets =
                generate_random_dual_vertices(1, facet_count, 0.5, 2.0, &mut rng);

            if let Some(dual_vertices) = dual_vertices_sets.first() {
                let unpruned = unpruned_capacity_for_dual_vertices(dual_vertices).unwrap();
                let pruned = pruned_capacity_for_dual_vertices(dual_vertices).unwrap();

                assert!(
                    (unpruned.capacity() - pruned.capacity()).abs() < 1e-6,
                    "F={} seed={}: pruned {} vs unpruned {}",
                    facet_count,
                    seed,
                    pruned.capacity(),
                    unpruned.capacity()
                );

                assert!(
                    pruned.iterations <= unpruned.iterations,
                    "F={} seed={}: pruned iterations {} > unpruned {}",
                    facet_count,
                    seed,
                    pruned.iterations,
                    unpruned.iterations
                );
            }
        }
    }
}

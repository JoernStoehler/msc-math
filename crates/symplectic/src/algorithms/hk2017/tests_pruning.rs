//! HK2017 pruning and combinatorics-focused tests.
//!
//! Split from mod.rs to isolate pruning behavior checks.

use super::*;
use crate::geom::known_polytopes;
use crate::{ehz_capacity_pruned, ehz_capacity_unpruned};

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

// ── Direct pruning agreement ──

/// Verify pruned and unpruned produce identical capacity on the simplex.
///
/// This keeps a fast live pruning smoke test in the library. Broad pruned vs
/// unpruned validation lives in `experiments/verification/correctness/`.
#[test]
fn pruned_matches_unpruned_simplex() {
    let kp = known_polytopes::simplex();
    let result_unpruned = ehz_capacity_unpruned(&kp.polytope).expect("unpruned capacity");
    let result_pruned = ehz_capacity_pruned(&kp.polytope).expect("pruned capacity");
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
    let result_unpruned = ehz_capacity_unpruned(&kp.polytope).expect("unpruned capacity");
    let result_pruned = ehz_capacity_pruned(&kp.polytope).expect("pruned capacity");

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
    use crate::random::generate_random_polytopes;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    for facet_count in 5..=8 {
        for seed in 0..4u64 {
            let mut rng = ChaCha8Rng::seed_from_u64(seed);
            let polytopes = generate_random_polytopes(1, facet_count, 0.5, 2.0, &mut rng);

            if let Some(p) = polytopes.first() {
                let unpruned = ehz_capacity_unpruned(p).unwrap();
                let pruned = ehz_capacity_pruned(p).unwrap();

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

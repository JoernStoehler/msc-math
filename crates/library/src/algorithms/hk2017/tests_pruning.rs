//! HK2017 tests_pruning tests split from `mod.rs` for module-level maintainability.

use super::*;
use crate::geom::known_polytopes;

use std::path::PathBuf;
use std::sync::LazyLock;

use super::generate_capacity_fixtures::{load_dataset_entries, DatasetEntry, FIXTURE_PATH};

/// Shared dataset loaded from cached fixture (scalar-only, no Polytope4D construction).
static DATASET: LazyLock<Vec<DatasetEntry>> = LazyLock::new(|| {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(FIXTURE_PATH);
    load_dataset_entries(&path)
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
    let result_unpruned = ehz_capacity_unpruned(&kp.polytope).expect("unpruned capacity");
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
        result_unpruned.result.iterations, result_pruned.result.iterations
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

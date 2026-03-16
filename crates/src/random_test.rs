//! Tests for random: sample validity and deterministic reproducibility.
//!
//! Proposition: sample_random_polytope produces valid polytopes; same seed
//! yields identical results; generate_random_polytopes fills to requested count.
//!
//! Strategy: fixture-based with fixed seeds, proptest for validation invariants.

use super::random::*;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

/// Verify same seed produces identical accept/reject outcomes (determinism).
#[test]
fn deterministic_sampling() {
    let mut rng1 = ChaCha8Rng::seed_from_u64(42);
    let mut rng2 = ChaCha8Rng::seed_from_u64(42);

    let results1: Vec<_> = (0..20)
        .map(|_| sample_random_polytope(6, 0.5, 2.0, &mut rng1).is_ok())
        .collect();
    let results2: Vec<_> = (0..20)
        .map(|_| sample_random_polytope(6, 0.5, 2.0, &mut rng2).is_ok())
        .collect();
    assert_eq!(results1, results2);
}

/// Verify at least one F=5 polytope is accepted out of 200 samples.
#[test]
fn some_polytopes_accepted_f5() {
    let mut rng = ChaCha8Rng::seed_from_u64(0);
    let mut accepted = 0;
    let n = 200;
    for _ in 0..n {
        if sample_random_polytope(5, 0.5, 2.0, &mut rng).is_ok() {
            accepted += 1;
        }
    }
    assert!(
        accepted > 0,
        "expected at least 1 accepted polytope out of {n} attempts with F=5"
    );
}

/// Verify generate_random_polytopes returns exactly the requested count.
#[test]
fn generate_fills_to_requested_count() {
    let mut rng = ChaCha8Rng::seed_from_u64(123);
    let polytopes = generate_random_polytopes(3, 5, 0.5, 2.0, &mut rng);
    assert_eq!(polytopes.len(), 3);
    for p in &polytopes {
        assert_eq!(p.facet_count(), 5);
    }
}

// ---- Property tests ----

#[cfg(test)]
mod proptests {
    use super::*;
    use crate::geom::polytope::Polytope4D;
    use proptest::prelude::*;

    proptest! {
        /// Property: every polytope accepted by sample_random_polytope passes
        /// full revalidation via Polytope4D::new.
        ///
        /// Limited to 5-6 facets and 4 seeds to keep runtime bounded.
        #[test]
        fn random_polytopes_pass_validation(
            facet_count in 5usize..=6,
            seed in 0u64..4
        ) {
            let mut rng = ChaCha8Rng::seed_from_u64(seed);

            let result = sample_random_polytope(facet_count, 0.5, 2.0, &mut rng);

            if let Ok(polytope) = result {
                let normals = polytope.normals_f64();
                let heights = polytope.heights_f64();
                let halfspaces: Vec<nalgebra::Vector4<f64>> = normals
                    .iter()
                    .zip(heights.iter())
                    .map(|(n, &h)| n / h)
                    .collect();
                let revalidated = Polytope4D::new(halfspaces);
                prop_assert!(
                    revalidated.is_ok(),
                    "accepted polytope failed revalidation: {:?}",
                    revalidated.err()
                );
            }
        }
    }
}

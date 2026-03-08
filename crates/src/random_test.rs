use super::*;
use rand::SeedableRng;

#[test]
fn deterministic_sampling() {
    let mut rng1 = ChaCha8Rng::seed_from_u64(42);
    let mut rng2 = ChaCha8Rng::seed_from_u64(42);

    // Generate several samples from each — results should match
    let results1: Vec<_> = (0..20)
        .map(|_| sample_random_polytope(6, 0.5, 2.0, &mut rng1).is_ok())
        .collect();
    let results2: Vec<_> = (0..20)
        .map(|_| sample_random_polytope(6, 0.5, 2.0, &mut rng2).is_ok())
        .collect();
    assert_eq!(results1, results2);
}

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
    use proptest::prelude::*;

    proptest! {
        /// Property: every polytope accepted by sample_random_polytope passes full validation.
        ///
        /// This ensures the rejection sampling loop doesn't have bugs that let
        /// invalid polytopes through.
        ///
        /// NOTE: Limited to 5-6 facets and 4 seeds to keep runtime <10min.
        /// Random polytope generation involves qhull and can be slow.
        #[test]
        fn random_polytopes_pass_validation(
            facet_count in 5usize..=6,
            seed in 0u64..4
        ) {
            let mut rng = ChaCha8Rng::seed_from_u64(seed);
            let h_min = 0.5;
            let h_max = 2.0;

            // Attempt to sample
            let result = sample_random_polytope(facet_count, h_min, h_max, &mut rng);

            // If accepted, it must pass Polytope4D::new() revalidation
            if let Ok(polytope) = result {
                let normals = polytope.normals_f64().to_vec();
                let heights = polytope.heights_f64().to_vec();

                // Validate should succeed (it already did in sample_random_polytope,
                // but we verify the polytope is still valid after construction)
                let revalidate = Polytope4D::new(normals, heights);
                prop_assert!(
                    revalidate.is_ok(),
                    "accepted polytope failed revalidation: {:?}",
                    revalidate.err()
                );
            }
            // If rejected, that's fine — rejection sampling is allowed to reject
        }
    }
}

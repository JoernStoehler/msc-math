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

//! Property tests for volume computation.
//!
//! Proposition: vol(K) > 0 for all valid bounded 4D polytopes.
//! Exact values: simplex = 1/24, hypercube = 16, crosspolytope = 32/3.
//! Reference: [def:volume]
//!
//! Strategy: fixture-based (known polytopes) + random polytopes (40 cases)

use crate::geom::known_polytopes;
use crate::geom::test_utils::random_bounded_polytope;
use crate::geom::volume::volume;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

#[test]
fn volume_positive_on_known_polytopes() {
    let cases = vec![
        ("simplex", known_polytopes::simplex(), 1.0 / 24.0),
        ("hypercube", known_polytopes::hypercube(), 16.0),
        ("crosspolytope", known_polytopes::crosspolytope(), 32.0 / 3.0),
    ];

    for (name, kp, expected) in cases {
        let vol = volume(&kp.polytope).unwrap_or_else(|_| panic!("{name} volume computation"));

        assert!(
            (vol - expected).abs() / expected < 1e-6,
            "{name}: volume = {vol}, expected = {expected}"
        );

        assert!(vol > 0.0, "{name}: volume should be positive");
    }
}

#[test]
fn volume_positive_on_random_polytopes() {
    let mut tested = 0;

    for facet_count in 5..=8 {
        for i in 0..10 {
            let seed = 12345u64 + (facet_count as u64 * 100) + (i as u64);
            let mut rng = ChaCha8Rng::seed_from_u64(seed);

            let polytope = random_bounded_polytope(facet_count, &mut rng);
            let vol = volume(&polytope).expect("volume computation");
            assert!(
                vol > 0.0,
                "f={facet_count}: volume should be positive, got {vol}"
            );
            tested += 1;
        }
    }

    assert!(
        tested > 0,
        "Should have tested at least some random polytopes"
    );
}

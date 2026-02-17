//! Property tests for volume computation.
//!
//! Verifies mathematical properties:
//! - Positivity: vol(K) > 0 for all valid polytopes
//! - Accuracy: Known polytopes have expected volumes

use crate::geom::test_utils::{crosspolytope, hypercube, random_bounded_polytope, simplex};
use crate::geom::volume::volume;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

#[test]
fn volume_positive_on_known_polytopes() {
    let cases = vec![
        ("simplex", simplex(), 1.0 / 24.0),
        ("hypercube", hypercube(), 16.0),
        ("crosspolytope", crosspolytope(), 32.0 / 3.0),
    ];

    for (name, polytope, expected) in cases {
        let vol = volume(&polytope).expect(&format!("{} volume computation", name));

        assert!(
            (vol - expected).abs() / expected < 1e-6,
            "{}: volume = {}, expected = {}",
            name, vol, expected
        );

        assert!(vol > 0.0, "{}: volume should be positive", name);
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
                "f={}: volume should be positive, got {}",
                facet_count, vol
            );
            tested += 1;
        }
    }

    assert!(tested > 0, "Should have tested at least some random polytopes");
    println!(
        "Verified volume > 0 for {} random polytopes",
        tested
    );
}

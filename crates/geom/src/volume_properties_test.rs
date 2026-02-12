//! Property tests for volume computation.
//!
//! Verifies mathematical properties:
//! - Positivity: vol(K) > 0 for all valid polytopes
//! - Accuracy: Known polytopes have expected volumes

use crate::test_utils::{crosspolytope, hypercube, simplex};
use crate::volume::volume;
use crate::polytope::Polytope4D;
use nalgebra::Vector4;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

#[test]
fn volume_positive_on_known_polytopes() {
    // Known polytopes with exact expected volumes
    let cases = vec![
        ("simplex", simplex(), 1.0 / 24.0),
        ("hypercube", hypercube(), 16.0),
        ("crosspolytope", crosspolytope(), 32.0 / 3.0),
    ];

    for (name, polytope, expected) in cases {
        let vol = volume(&polytope).expect(&format!("{} volume computation", name));

        // Verify expected value
        assert!(
            (vol - expected).abs() / expected < 1e-6,
            "{}: volume = {}, expected = {}",
            name, vol, expected
        );

        // Verify positivity
        assert!(vol > 0.0, "{}: volume should be positive", name);
    }
}

#[test]
fn volume_positive_on_random_polytopes() {
    // Generate small random polytopes (recompute every run, ~10 sec)
    // Note: Some random configurations may be unbounded - we skip those
    let mut tested = 0;

    for facet_count in 5..=8 {
        for i in 0..10 {
            // Try multiple seeds; skip invalid polytopes
            let test_rng_seed = 12345u64 + (facet_count as u64 * 100) + (i as u64);
            let mut test_rng = ChaCha8Rng::seed_from_u64(test_rng_seed);

            if let Ok(polytope) = generate_random_polytope_or_skip(facet_count, &mut test_rng) {
                let vol = volume(&polytope).expect("volume computation");
                assert!(
                    vol > 0.0,
                    "f={}: volume should be positive, got {}",
                    facet_count, vol
                );
                tested += 1;
            }
        }
    }

    assert!(tested > 0, "Should have tested at least some random polytopes");
    println!(
        "✓ Verified volume > 0 for {} valid random polytopes",
        tested
    );
}

/// Try to generate a random polytope, return None if it's unbounded
fn generate_random_polytope_or_skip(
    facet_count: usize,
    rng: &mut rand_chacha::ChaCha8Rng,
) -> Result<Polytope4D, ()> {
    for _ in 0..5 {
        // Retry a few times
        let normals: Vec<Vector4<f64>> = (0..facet_count)
            .map(|_| {
                let v = Vector4::new(
                    rng.sample(rand_distr::StandardNormal),
                    rng.sample(rand_distr::StandardNormal),
                    rng.sample(rand_distr::StandardNormal),
                    rng.sample(rand_distr::StandardNormal),
                );
                v.normalize()
            })
            .collect();

        let heights: Vec<f64> = (0..facet_count)
            .map(|_| rng.gen_range(0.5..2.0))
            .collect();

        if let Ok(polytope) = Polytope4D::new(normals, heights) {
            return Ok(polytope);
        }
    }
    Err(())
}


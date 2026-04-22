//! Random polytope generation via rejection sampling.
//!
//! Provides deterministic (seeded) random polytope sampling for dataset
//! generation and property testing. Normals are uniformly distributed on S^3
//! (via 4D standard normal normalization) and heights are uniform in a
//! configurable range.
//!
//! Mathematical correspondence: the sampling distribution is uniform over
//! normal directions (Haar measure on S^3) with independent height scaling.

use crate::geom::polytope::{ConstructionError, Polytope4D};
use nalgebra::Vector4;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use rand_distr::{Distribution, StandardNormal, Uniform};

/// Rejection threshold for near-zero vectors when sampling on S^3.
/// Probability of ||v|| < 1e-10 for 4D standard normal is astronomically small.
const EPS_NEAR_ZERO: f64 = 1e-10;

/// Sample a single random unit vector on S^3 (uniform distribution via Muller's method).
fn random_unit_s3(rng: &mut ChaCha8Rng) -> Vector4<f64> {
    loop {
        let x: f64 = StandardNormal.sample(rng);
        let y: f64 = StandardNormal.sample(rng);
        let z: f64 = StandardNormal.sample(rng);
        let w: f64 = StandardNormal.sample(rng);
        let v = Vector4::new(x, y, z, w);
        let norm = v.norm();
        if norm > EPS_NEAR_ZERO {
            return v / norm;
        }
    }
}

/// Attempt to sample a single valid polytope.
///
/// Returns `Ok(polytope)` if the sample passes full validation (including
/// exact rational vertex enumeration), or `Err(error)` if it fails any check.
///
/// # Arguments
///
/// * `facet_count` - Number of halfspaces (must be >= 5)
/// * `h_min`, `h_max` - Height range (0 < h_min <= h_max)
/// * `rng` - Deterministic random number generator
pub fn sample_random_polytope(
    facet_count: usize,
    h_min: f64,
    h_max: f64,
    rng: &mut ChaCha8Rng,
) -> Result<Polytope4D, ConstructionError> {
    let h_dist = Uniform::new(h_min, h_max);

    let normals: Vec<Vector4<f64>> = (0..facet_count).map(|_| random_unit_s3(rng)).collect();
    let heights: Vec<f64> = (0..facet_count).map(|_| h_dist.sample(rng)).collect();

    // Convert (normal, height) to dual vertex: a_i = n_i / h_i
    let halfspaces: Vec<Vector4<f64>> = normals
        .iter()
        .zip(heights.iter())
        .map(|(n, &h)| n / h)
        .collect();

    Polytope4D::from_f64(halfspaces)
}

/// Generate a single polytope attempt with an independent seed.
/// The (master_seed, attempt) pair fully determines the attempt.
///
/// Uses blake3 key derivation to produce a 32-byte seed from
/// (master_seed, attempt), then seeds ChaCha8Rng for the actual
/// random number generation.
pub fn generate_polytope(
    facet_count: usize,
    h_min: f64,
    h_max: f64,
    master_seed: u64,
    attempt: u64,
) -> Result<Polytope4D, ConstructionError> {
    let mut key_material = [0u8; 16];
    key_material[..8].copy_from_slice(&master_seed.to_le_bytes());
    key_material[8..].copy_from_slice(&attempt.to_le_bytes());
    let seed = blake3::derive_key("polytope-gen", &key_material);
    let mut rng = ChaCha8Rng::from_seed(seed);
    sample_random_polytope(facet_count, h_min, h_max, &mut rng)
}

/// Generate random polytopes via rejection sampling.
///
/// Keeps sampling until `count` valid polytopes are found.
pub fn generate_random_polytopes(
    count: usize,
    facet_count: usize,
    h_min: f64,
    h_max: f64,
    rng: &mut ChaCha8Rng,
) -> Vec<Polytope4D> {
    let mut accepted = Vec::with_capacity(count);
    while accepted.len() < count {
        if let Ok(p) = sample_random_polytope(facet_count, h_min, h_max, rng) {
            accepted.push(p);
        }
    }
    accepted
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    // Tests for random: sample validity and deterministic reproducibility.
    //
    // Proposition: sample_random_polytope produces valid polytopes; same seed
    // yields identical results; generate_random_polytopes fills to requested count.
    //
    // Strategy: fixture-based with fixed seeds, proptest for validation invariants.

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

    /// generate_polytope with different attempts produces different RNG streams.
    #[test]
    fn generate_polytope_different_attempts() {
        // Both may succeed or fail, but the RNG streams must differ.
        // We check by comparing the raw seeds derived from blake3.
        let mut key0 = [0u8; 16];
        key0[..8].copy_from_slice(&42u64.to_le_bytes());
        key0[8..].copy_from_slice(&0u64.to_le_bytes());
        let seed0 = blake3::derive_key("polytope-gen", &key0);

        let mut key1 = [0u8; 16];
        key1[..8].copy_from_slice(&42u64.to_le_bytes());
        key1[8..].copy_from_slice(&1u64.to_le_bytes());
        let seed1 = blake3::derive_key("polytope-gen", &key1);

        assert_ne!(
            seed0, seed1,
            "different attempts must produce different seeds"
        );
    }

    /// generate_polytope is reproducible: same (master_seed, attempt) → same result.
    #[test]
    fn generate_polytope_reproducible() {
        let r1 = generate_polytope(6, 0.5, 2.0, 99, 0);
        let r2 = generate_polytope(6, 0.5, 2.0, 99, 0);
        assert_eq!(r1.is_ok(), r2.is_ok());
        if let (Ok(p1), Ok(p2)) = (r1, r2) {
            assert_eq!(p1.incidence(), p2.incidence());
        }
    }

    // ---- Property tests ----

    #[cfg(test)]
    mod proptests {
        use super::*;
        use crate::geom::polytope::Polytope4D;
        use proptest::prelude::*;

        proptest! {
            #![proptest_config(proptest::prelude::ProptestConfig::with_cases(8))]
            /// Property: every polytope accepted by sample_random_polytope passes
            /// full revalidation via Polytope4D::new.
            ///
            /// 8 cases in default suite (each runs vertex enumeration).
            /// Already limited to 5-6 facets and 4 seeds.
            #[test]
            fn random_polytopes_pass_validation(
                facet_count in 5usize..=6,
                seed in 0u64..4
            ) {
                let mut rng = ChaCha8Rng::seed_from_u64(seed);

                let result = sample_random_polytope(facet_count, 0.5, 2.0, &mut rng);

                if let Ok(polytope) = result {
                    let duals = polytope.dual_vertices_f64();
                    let revalidated = Polytope4D::from_f64(duals.to_vec());
                    prop_assert!(
                        revalidated.is_ok(),
                        "accepted polytope failed revalidation: {:?}",
                        revalidated.err()
                    );
                }
            }
        }
    }
}

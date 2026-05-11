//! Random accepted dual-vertex generation via rejection sampling.
//!
//! Provides deterministic (seeded) random dual-vertex sampling for dataset
//! generation and property testing. Normals are uniformly distributed on S^3
//! (via 4D standard normal normalization) and heights are uniform in a
//! configurable range.
//!
//! Mathematical correspondence: the sampling distribution is uniform over
//! normal directions (Haar measure on S^3) with independent height scaling.

use crate::geom::polytope::{ConstructionError, Polytope4D};
use euclidean_polytopes::sample_random_dual_vertices_f64;
use nalgebra::Vector4;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

fn validate_sampling_parameters(
    facet_count: usize,
    h_min: f64,
    h_max: f64,
) -> Result<(), ConstructionError> {
    if facet_count < 5 {
        return Err(ConstructionError::TooFewFacets(facet_count));
    }
    if !h_min.is_finite() || !h_max.is_finite() || h_min <= 0.0 || h_min >= h_max {
        return Err(ConstructionError::F64Conversion(format!(
            "random height range must satisfy finite 0 < h_min < h_max, got h_min={h_min}, h_max={h_max}"
        )));
    }
    Ok(())
}

/// Attempt to sample a single accepted dual-vertex set.
///
/// Returns `Ok(dual_vertices)` if the sample passes full validation (including
/// exact rational vertex enumeration), or `Err(error)` if it fails any check.
///
/// # Arguments
///
/// * `facet_count` - Number of halfspaces (must be >= 5)
/// * `h_min`, `h_max` - Height range (0 < h_min < h_max)
/// * `rng` - Deterministic random number generator
pub fn sample_random_dual_vertices(
    facet_count: usize,
    h_min: f64,
    h_max: f64,
    rng: &mut ChaCha8Rng,
) -> Result<Vec<Vector4<f64>>, ConstructionError> {
    validate_sampling_parameters(facet_count, h_min, h_max)?;

    let dual_vertices = sample_random_dual_vertices_f64(facet_count, h_min, h_max, rng);
    Polytope4D::from_f64(dual_vertices.clone())?;
    Ok(dual_vertices)
}

/// Generate a single accepted dual-vertex attempt with an independent seed.
/// The (master_seed, attempt) pair fully determines the attempt.
///
/// Uses blake3 key derivation to produce a 32-byte seed from
/// (master_seed, attempt), then seeds ChaCha8Rng for the actual
/// random number generation.
pub fn generate_dual_vertices(
    facet_count: usize,
    h_min: f64,
    h_max: f64,
    master_seed: u64,
    attempt: u64,
) -> Result<Vec<Vector4<f64>>, ConstructionError> {
    let mut key_material = [0u8; 16];
    key_material[..8].copy_from_slice(&master_seed.to_le_bytes());
    key_material[8..].copy_from_slice(&attempt.to_le_bytes());
    let seed = blake3::derive_key("polytope-gen", &key_material);
    let mut rng = ChaCha8Rng::from_seed(seed);
    sample_random_dual_vertices(facet_count, h_min, h_max, &mut rng)
}

/// Generate accepted random dual-vertex sets via rejection sampling.
///
/// Keeps sampling until `count` valid dual-vertex sets are found.
///
/// # Panics
///
/// Panics immediately if `facet_count`, `h_min`, or `h_max` cannot define a
/// valid sampling distribution. Use [`sample_random_dual_vertices`] for a fallible
/// one-attempt API.
pub fn generate_random_dual_vertices(
    count: usize,
    facet_count: usize,
    h_min: f64,
    h_max: f64,
    rng: &mut ChaCha8Rng,
) -> Vec<Vec<Vector4<f64>>> {
    validate_sampling_parameters(facet_count, h_min, h_max)
        .expect("invalid random dual-vertex sampling parameters");

    let mut accepted = Vec::with_capacity(count);
    while accepted.len() < count {
        if let Ok(dual_vertices) = sample_random_dual_vertices(facet_count, h_min, h_max, rng) {
            accepted.push(dual_vertices);
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
    // Proposition: sample_random_dual_vertices produces valid accepted dual
    // vertices; same seed yields identical results; generate_random_dual_vertices
    // fills to requested count.
    //
    // Strategy: fixture-based with fixed seeds, proptest for validation invariants.

    /// Verify same seed produces identical accept/reject outcomes (determinism).
    #[test]
    fn deterministic_sampling() {
        let mut rng1 = ChaCha8Rng::seed_from_u64(42);
        let mut rng2 = ChaCha8Rng::seed_from_u64(42);

        let results1: Vec<_> = (0..20)
            .map(|_| sample_random_dual_vertices(6, 0.5, 2.0, &mut rng1).is_ok())
            .collect();
        let results2: Vec<_> = (0..20)
            .map(|_| sample_random_dual_vertices(6, 0.5, 2.0, &mut rng2).is_ok())
            .collect();
        assert_eq!(results1, results2);
    }

    /// Verify at least one F=5 dual-vertex set is accepted out of 200 samples.
    #[test]
    fn some_polytopes_accepted_f5() {
        let mut rng = ChaCha8Rng::seed_from_u64(0);
        let mut accepted = 0;
        let n = 200;
        for _ in 0..n {
            if sample_random_dual_vertices(5, 0.5, 2.0, &mut rng).is_ok() {
                accepted += 1;
            }
        }
        assert!(
            accepted > 0,
            "expected at least 1 accepted dual-vertex set out of {n} attempts with F=5"
        );
    }

    /// Verify generate_random_dual_vertices returns exactly the requested count.
    #[test]
    fn generate_fills_to_requested_count() {
        let mut rng = ChaCha8Rng::seed_from_u64(123);
        let dual_vertices_sets = generate_random_dual_vertices(3, 5, 0.5, 2.0, &mut rng);
        assert_eq!(dual_vertices_sets.len(), 3);
        for dual_vertices in &dual_vertices_sets {
            assert_eq!(dual_vertices.len(), 5);
        }
    }

    #[test]
    fn sample_rejects_impossible_facet_count_before_rejection_loop() {
        let mut rng = ChaCha8Rng::seed_from_u64(123);
        let err = sample_random_dual_vertices(4, 0.5, 2.0, &mut rng).unwrap_err();
        assert_eq!(err, ConstructionError::TooFewFacets(4));
    }

    #[test]
    fn sample_rejects_invalid_height_range_before_distribution_construction() {
        let mut rng = ChaCha8Rng::seed_from_u64(123);
        let err = sample_random_dual_vertices(5, 1.0, 1.0, &mut rng).unwrap_err();
        assert_eq!(
            err,
            ConstructionError::F64Conversion(
                "random height range must satisfy finite 0 < h_min < h_max, got h_min=1, h_max=1"
                    .to_string()
            )
        );
    }

    /// generate_dual_vertices with different attempts produces different RNG streams.
    #[test]
    fn generate_dual_vertices_different_attempts() {
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

    /// generate_dual_vertices is reproducible: same (master_seed, attempt) -> same result.
    #[test]
    fn generate_dual_vertices_reproducible() {
        let r1 = generate_dual_vertices(6, 0.5, 2.0, 99, 0);
        let r2 = generate_dual_vertices(6, 0.5, 2.0, 99, 0);
        assert_eq!(r1.is_ok(), r2.is_ok());
        if let (Ok(dual_vertices1), Ok(dual_vertices2)) = (r1, r2) {
            assert_eq!(dual_vertices1, dual_vertices2);
        }
    }

    // ---- Property tests ----

    #[cfg(test)]
    mod proptests {
        use super::*;
        use proptest::prelude::*;

        proptest! {
            #![proptest_config(proptest::prelude::ProptestConfig::with_cases(8))]
            /// Property: every sample accepted by sample_random_dual_vertices passes
            /// full revalidation via the private construction pipeline.
            ///
            /// 8 cases in default suite (each runs vertex enumeration).
            /// Already limited to 5-6 facets and 4 seeds.
            #[test]
            fn random_dual_vertices_pass_validation(
                facet_count in 5usize..=6,
                seed in 0u64..4
            ) {
                let mut rng = ChaCha8Rng::seed_from_u64(seed);

                let result = sample_random_dual_vertices(facet_count, 0.5, 2.0, &mut rng);

                if let Ok(dual_vertices) = result {
                    let revalidated = Polytope4D::from_f64(dual_vertices);
                    prop_assert!(
                        revalidated.is_ok(),
                        "accepted dual vertices failed revalidation: {:?}",
                        revalidated.err()
                    );
                }
            }
        }
    }
}

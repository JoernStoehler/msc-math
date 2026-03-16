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

    Polytope4D::new(halfspaces)
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

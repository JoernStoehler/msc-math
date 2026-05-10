//! Random Euclidean candidate data for normalized `R^4` polytopes.
//!
//! This module samples ordinary Euclidean dual vertices only. It does not
//! decide whether the resulting normalized halfspaces define a bounded,
//! non-redundant, or otherwise valid polytope.

use nalgebra::Vector4;
use rand::Rng;
use rand_distr::{Distribution, StandardNormal, Uniform};

/// Rejection threshold for near-zero normal samples before normalization.
///
/// The probability that a 4D standard normal vector has norm below this
/// threshold is negligible, but the check keeps the normalization contract
/// explicit.
const EPS_NEAR_ZERO: f64 = 1e-10;

/// Sample candidate normalized dual vertices for a polytope in `R^4`.
///
/// The output is a flat list of vectors `a_i` for normalized halfspaces
/// `<a_i, x> <= 1`. For each index, this samples a unit normal `n_i` uniformly
/// on `S^3`, samples an independent height `h_i` uniformly in
/// `[h_min, h_max)`, and returns `a_i = n_i / h_i`.
///
/// This function only samples candidates. It does not construct a polytope,
/// validate boundedness, validate non-redundancy, or recover incidence.
///
/// # Panics
///
/// Panics when `facet_count < 5`, or when the height range does not satisfy
/// finite `0 < h_min < h_max`.
pub fn sample_random_dual_vertices_f64<R: Rng + ?Sized>(
    facet_count: usize,
    h_min: f64,
    h_max: f64,
    rng: &mut R,
) -> Vec<Vector4<f64>> {
    assert!(
        facet_count >= 5,
        "facet_count must be at least 5, got {facet_count}"
    );
    assert!(
        h_min.is_finite() && h_max.is_finite() && 0.0 < h_min && h_min < h_max,
        "height range must satisfy finite 0 < h_min < h_max, got h_min={h_min}, h_max={h_max}"
    );

    let height_distribution = Uniform::new(h_min, h_max);
    let normals: Vec<_> = (0..facet_count).map(|_| random_unit_s3(rng)).collect();
    let heights: Vec<f64> = (0..facet_count)
        .map(|_| height_distribution.sample(rng))
        .collect();

    normals
        .iter()
        .zip(heights.iter())
        .map(|(normal, &height)| normal / height)
        .collect()
}

fn random_unit_s3<R: Rng + ?Sized>(rng: &mut R) -> Vector4<f64> {
    loop {
        let x: f64 = StandardNormal.sample(rng);
        let y: f64 = StandardNormal.sample(rng);
        let z: f64 = StandardNormal.sample(rng);
        let w: f64 = StandardNormal.sample(rng);
        let normal = Vector4::new(x, y, z, w);
        let norm = normal.norm();
        if norm > EPS_NEAR_ZERO {
            return normal / norm;
        }
    }
}

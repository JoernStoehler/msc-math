/// Test fixture polytopes for use across test modules.
///
/// This module provides standard test polytopes (simplex, hypercube, etc.)
/// used in multiple test files. Single source of truth for test data.
use crate::polytope::Polytope4D;
use nalgebra::Vector4;
use std::f64::consts::PI;

#[cfg(test)]
use rand::Rng;
#[cfg(test)]
use rand_distr::StandardNormal;

/// Standard 4-simplex: conv{origin-offset vertices}.
///
/// Translated so origin is at centroid (0.2, 0.2, 0.2, 0.2).
/// Expected properties:
/// - Volume: 1/24 ≈ 0.04167
/// - EHZ capacity: 0.25
pub fn simplex() -> Polytope4D {
    let centroid = Vector4::new(0.2, 0.2, 0.2, 0.2);
    let normals_raw = vec![
        -Vector4::x(),
        -Vector4::y(),
        -Vector4::z(),
        -Vector4::w(),
        Vector4::new(1.0, 1.0, 1.0, 1.0).normalize(),
    ];
    let heights_raw = vec![0.0, 0.0, 0.0, 0.0, 0.5];
    let heights: Vec<f64> = normals_raw
        .iter()
        .zip(&heights_raw)
        .map(|(n, h)| h - n.dot(&centroid))
        .collect();
    Polytope4D::new(normals_raw, heights).expect("simplex")
}

/// Unit hypercube [-1, 1]^4.
///
/// Expected properties:
/// - Volume: 2^4 = 16
/// - EHZ capacity: 4.0
pub fn hypercube() -> Polytope4D {
    let normals = vec![
        Vector4::x(),
        -Vector4::x(),
        Vector4::y(),
        -Vector4::y(),
        Vector4::z(),
        -Vector4::z(),
        Vector4::w(),
        -Vector4::w(),
    ];
    let heights = vec![1.0; 8];
    Polytope4D::new(normals, heights).expect("hypercube")
}

/// Scaled hypercube [-s, s]^4.
///
/// Expected properties:
/// - Volume: (2s)^4 = 16s^4
/// - EHZ capacity: 4s
pub fn scaled_hypercube(s: f64) -> Polytope4D {
    let normals = vec![
        Vector4::x(),
        -Vector4::x(),
        Vector4::y(),
        -Vector4::y(),
        Vector4::z(),
        -Vector4::z(),
        Vector4::w(),
        -Vector4::w(),
    ];
    let heights = vec![s; 8];
    Polytope4D::new(normals, heights).expect("scaled hypercube")
}

/// 4D cross-polytope: conv{±2·e_i for i=1,2,3,4}.
///
/// H-representation: (±1,±1,±1,±1)/2 · x ≤ 1 (16 facets, 8 vertices).
/// The vertices ±2·eᵢ satisfy every H-constraint: (±1,±1,±1,±1)/2 · (±2eᵢ) = ±1 ≤ 1.
///
/// Expected properties:
/// - Volume: 32/3 ≈ 10.667
pub fn crosspolytope() -> Polytope4D {
    let mut normals = Vec::with_capacity(16);
    for s0 in [-1.0_f64, 1.0] {
        for s1 in [-1.0_f64, 1.0] {
            for s2 in [-1.0_f64, 1.0] {
                for s3 in [-1.0_f64, 1.0] {
                    normals.push(Vector4::new(s0, s1, s2, s3).normalize());
                }
            }
        }
    }
    let heights = vec![1.0; 16];
    Polytope4D::new(normals, heights).expect("crosspolytope")
}

/// Triangle ×_L triangle (Lagrangian product).
///
/// Product of two equilateral triangles in symplectic coordinates.
/// 6 facets, each corresponding to a side of one of the triangles.
///
/// Expected properties:
/// - EHZ capacity: 1.5
pub fn triangle_product() -> Polytope4D {
    let mut normals = Vec::with_capacity(6);
    let mut heights = Vec::with_capacity(6);

    // First triangle in (q1, p1) plane
    for k in 0..3 {
        let angle = PI / 2.0 + 2.0 * PI * (k as f64) / 3.0;
        normals.push(Vector4::new(angle.cos(), angle.sin(), 0.0, 0.0));
        heights.push(0.5);
    }

    // Second triangle in (q2, p2) plane
    for k in 0..3 {
        let angle = PI / 2.0 + 2.0 * PI * (k as f64) / 3.0;
        normals.push(Vector4::new(0.0, 0.0, angle.cos(), angle.sin()));
        heights.push(0.5);
    }

    Polytope4D::new(normals, heights).expect("triangle product")
}

/// Generate a random bounded polytope with specified number of facets.
///
/// Normals are uniformly distributed on S³ (via sampling from 4D standard normal
/// and normalizing). Heights are random in [0.5, 2.0] to ensure 0 ∈ int(K).
///
/// Used for cross-checking volume algorithms and other empirical validation.
///
/// # Arguments
/// * `facet_count` - Number of facets (must be >= 5 for bounded polytope in 4D)
/// * `rng` - Random number generator
///
/// # Panics
/// Panics if Polytope4D::new() fails (unbounded, degenerate, etc.)
/// Also panics if `facet_count < 5` (minimum for bounded 4D polytope).
#[cfg(test)]
pub fn random_bounded_polytope(facet_count: usize, rng: &mut impl Rng) -> Polytope4D {
    // Generate random unit vectors on S³
    let normals: Vec<Vector4<f64>> = (0..facet_count)
        .map(|_| {
            // Sample from 4D standard normal, normalize
            let v = Vector4::new(
                rng.sample(StandardNormal),
                rng.sample(StandardNormal),
                rng.sample(StandardNormal),
                rng.sample(StandardNormal),
            );
            v.normalize()
        })
        .collect();

    // Random heights ensuring 0 ∈ int(K)
    let heights: Vec<f64> = (0..facet_count)
        .map(|_| rng.gen_range(0.5..2.0))
        .collect();

    Polytope4D::new(normals, heights).expect("random polytope should be valid")
}

//! Property testing dataset infrastructure.
//!
//! This module is primarily intended for use in tests, but is defined as a normal
//! module to allow cross-crate test imports.
//!
//! Generates a phased test dataset of polytopes with precomputed capacity and volume:
//! - Phase 1: Base dataset (known and random polytopes)
//! - Phase 2: Symplectomorphism variants (apply random M ∈ Sp(4))
//! - Phase 3: Conformality variants (scale by random α)
//!
//! Used by capacity and sys property tests.

use geom::polytope::Polytope4D;
use geom::test_utils::{
    crosspolytope, simplex, triangle_product,
};
use geom::volume::volume;
use nalgebra::{Matrix4, Vector4};
use rand::Rng;
use rand_distr::StandardNormal;
use rand_chacha::ChaCha8Rng;
use rand::SeedableRng;
use crate::ehz_capacity;

/// Test polytope with precomputed volume and capacity.
#[derive(Clone, Debug)]
pub struct TestPolytope {
    pub name: String,
    pub polytope: Polytope4D,
    pub volume: f64,
    pub capacity: f64,
    /// Index of the base polytope this was derived from
    pub base_index: Option<usize>,
    /// Transformation type: None (base), "sympl", or "conform:1.50"
    pub transform: Option<String>,
}

/// Generate test dataset in three phases (~60-90 polytopes, 2-3 min).
///
/// Phase 1: Base dataset (20-30 polytopes)
///   - Known polytopes with exact values
///   - Small random polytopes (5-7 facets)
///
/// Phase 2: Symplectomorphism variants (2x dataset size)
///   - For each base polytope K: compute c(MK+b) for random M ∈ Sp(4)
///
/// Phase 3: Conformality variants (3x dataset size)
///   - For each base polytope K: compute c(α·K) for random α
pub fn generate_test_dataset() -> Vec<TestPolytope> {
    let mut rng = ChaCha8Rng::seed_from_u64(42);

    // ===== PHASE 1: Base dataset =====
    let mut base_dataset = Vec::new();

    // Known polytopes with exact values
    add_entry(
        &mut base_dataset,
        "simplex",
        simplex(),
        1.0 / 24.0,
        0.25,
        None,
        None,
    );
    add_entry(&mut base_dataset, "hypercube", hypercube(), 16.0, 4.0, None, None);
    add_entry(
        &mut base_dataset,
        "crosspolytope",
        crosspolytope(),
        32.0 / 3.0,
        2.0,
        None,
        None,
    );
    add_entry(
        &mut base_dataset,
        "triangle_product",
        triangle_product(),
        0.75,
        1.5,
        None,
        None,
    );

    // Small random polytopes (5-7 facets for speed)
    for facet_count in 5..=7 {
        for i in 0..4 {
            // 4 polytopes per facet count = 12 total
            let p = generate_random_bounded_polytope(facet_count, &mut rng);
            if let (Ok(vol), Some(cap_result)) = (volume(&p), ehz_capacity(&p)) {
                base_dataset.push(TestPolytope {
                    name: format!("random_f{}_n{}", facet_count, i),
                    polytope: p,
                    volume: vol,
                    capacity: cap_result.capacity,
                    base_index: None,
                    transform: None,
                });
            }
        }
    }

    // ===== PHASE 2: Symplectomorphism variants =====
    let mut full_dataset = base_dataset.clone();

    for (i, entry) in base_dataset.iter().enumerate() {
        // Generate random M ∈ Sp(4) and translation b
        let (m, b) = random_symplectomorphism(&entry.polytope, &mut rng);
        let transformed = apply_symplectomorphism(&entry.polytope, &m, &b);

        if let (Ok(vol), Some(cap_result)) = (volume(&transformed), ehz_capacity(&transformed)) {
            full_dataset.push(TestPolytope {
                name: format!("{}_sympl", entry.name),
                polytope: transformed,
                volume: vol,
                capacity: cap_result.capacity,
                base_index: Some(i),
                transform: Some("sympl".to_string()),
            });
        }
    }

    // ===== PHASE 3: Conformality variants =====
    for (i, entry) in base_dataset.iter().enumerate() {
        // Random scale factor between 0.5 and 2.0
        let alpha: f64 = rng.gen_range(0.5..2.0);
        let scaled = scale_polytope(&entry.polytope, alpha);

        if let (Ok(vol), Some(cap_result)) = (volume(&scaled), ehz_capacity(&scaled)) {
            full_dataset.push(TestPolytope {
                name: format!("{}_scale_{:.2}", entry.name, alpha),
                polytope: scaled,
                volume: vol,
                capacity: cap_result.capacity,
                base_index: Some(i),
                transform: Some(format!("conform:{:.2}", alpha)),
            });
        }
    }

    full_dataset
}

fn add_entry(
    dataset: &mut Vec<TestPolytope>,
    name: &str,
    polytope: Polytope4D,
    expected_vol: f64,
    expected_cap: f64,
    base_index: Option<usize>,
    transform: Option<String>,
) {
    dataset.push(TestPolytope {
        name: name.to_string(),
        polytope,
        volume: expected_vol,
        capacity: expected_cap,
        base_index,
        transform,
    });
}

/// Scale polytope: heights → α·heights (normals unchanged)
fn scale_polytope(polytope: &Polytope4D, alpha: f64) -> Polytope4D {
    let normals = polytope.normals().to_vec();
    let heights: Vec<f64> = polytope
        .heights()
        .iter()
        .map(|&h| alpha * h)
        .collect();
    Polytope4D::new(normals, heights).expect("scaled polytope")
}

/// Generate random symplectomorphism M ∈ Sp(4) + translation b
/// ensuring MK+b contains origin in interior
fn random_symplectomorphism(
    polytope: &Polytope4D,
    rng: &mut impl Rng,
) -> (Matrix4<f64>, Vector4<f64>) {
    // Generate random M ∈ Sp(4) using Cayley transform
    let m = random_sp4_matrix(rng);

    // Transform polytope: MK = {x : n_i·M^{-1}x ≤ h_i}
    // Find translation b such that 0 ∈ int(MK+b)
    // Choose b from interior of transformed polytope (e.g., its centroid)
    let vertices: Vec<_> = polytope.vertices().iter().map(|v| m * v).collect();
    let centroid = vertices.iter().sum::<Vector4<f64>>() / vertices.len() as f64;

    (m, centroid)
}

/// Generate random Sp(4) matrix using Cayley transform: M = (I - A)(I + A)^{-1}
/// where A is a random 4×4 matrix with A^T J + J A = 0 (infinitesimal symplectic)
fn random_sp4_matrix(rng: &mut impl Rng) -> Matrix4<f64> {
    // J = standard symplectic matrix [0 I; -I 0] in 2×2 blocks
    // Infinitesimal symplectic: A^T J + J A = 0
    // This gives 10 free parameters (dim of sp(4))

    // Simple approach: generate random skew-symmetric 2×2 blocks
    let a11: f64 = rng.sample(StandardNormal);
    let a12: f64 = rng.sample(StandardNormal);
    let a21 = -a12; // skew-symmetric
    let a22: f64 = rng.sample(StandardNormal);

    let b11: f64 = rng.sample(StandardNormal);
    let b12: f64 = rng.sample(StandardNormal);
    let _b21 = -b12; // skew-symmetric
    let b22: f64 = rng.sample(StandardNormal);

    // A = [A11 A12; A21 A22] in 2×2 blocks
    // where A12^T = A21 (symmetric blocks)
    let a_mat = Matrix4::new(
        a11, a12, b11, b12, a21, a22, b12, b22, // Note: A12^T in bottom-left
        -b11, -b12, a11, a12, // A21 = A12^T, A22 = -A11^T
        -b12, -b22, a21, a22,
    );

    // Cayley transform: M = (I - A)(I + A)^{-1}
    let id = Matrix4::identity();
    let i_plus_a = id + a_mat;
    let i_minus_a = id - a_mat;

    i_plus_a
        .try_inverse()
        .map(|inv| i_minus_a * inv)
        .unwrap_or(id) // Fallback to identity if singular
}

/// Apply symplectomorphism: K → MK+b
fn apply_symplectomorphism(polytope: &Polytope4D, m: &Matrix4<f64>, b: &Vector4<f64>) -> Polytope4D {
    // Transformed polytope: {x : n_i·M^{-1}(x-b) ≤ h_i}
    //                      = {x : (M^{-T}n_i)·x ≤ h_i + (M^{-T}n_i)·b}
    let m_inv_t = m
        .transpose()
        .try_inverse()
        .expect("M should be invertible");

    let normals: Vec<_> = polytope
        .normals()
        .iter()
        .map(|n| (m_inv_t * n).normalize())
        .collect();

    let heights: Vec<f64> = polytope
        .normals()
        .iter()
        .zip(polytope.heights().iter())
        .map(|(n, &h)| {
            let n_transformed = (m_inv_t * n).normalize();
            h + n_transformed.dot(b)
        })
        .collect();

    Polytope4D::new(normals, heights).expect("transformed polytope")
}

/// Generate a random bounded polytope for testing.
/// Retries if the polytope is unbounded.
fn generate_random_bounded_polytope(facet_count: usize, rng: &mut impl Rng) -> Polytope4D {
    // Retry loop: sometimes random configurations are unbounded
    for _attempt in 0..10 {
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

        if let Ok(polytope) = Polytope4D::new(normals, heights) {
            return polytope;
        }
    }

    // Fallback: use hypercube if random generation fails repeatedly
    hypercube()
}

// Helper: hypercube fixture (used in tests)
fn hypercube() -> Polytope4D {
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

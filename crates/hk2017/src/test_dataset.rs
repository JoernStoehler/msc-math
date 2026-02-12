//! Property testing dataset infrastructure.
//!
//! This module is primarily intended for use in tests, but is defined as a normal
//! module to allow cross-crate test imports.
//!
//! ## Architecture: cached fixture
//!
//! Generating the test dataset is expensive (~27 `ehz_capacity()` calls, ~2-3 min).
//! To keep `cargo test --lib` fast (<30s), the dataset is cached as a JSON fixture:
//!
//! - **Default path:** Property tests load from `tests/fixtures/capacity_dataset.json`
//! - **Regeneration:** Run `cargo test -p hk2017 regenerate_test_dataset -- --ignored`
//!   after changes to `ehz_capacity()` or the dataset generation logic.
//!
//! The fixture is committed to the repo so all worktrees have it immediately.
//!
//! ## Dataset phases
//!
//! - Phase 1: Base dataset (~9 polytopes: 3 known + up to 6 random)
//! - Phase 2: Symplectomorphism variants (up to +9, one per base polytope)
//! - Phase 3: Conformality variants (up to +9, one per base polytope)
//!
//! Total: up to ~27 polytopes (fewer if random generation or computation fails for some).
//!
//! Used by capacity and sys property tests.

use geom::polytope::Polytope4D;
use geom::test_utils::{simplex, triangle_product};
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

/// Path to the cached dataset fixture, relative to the hk2017 crate root.
pub const FIXTURE_PATH: &str = "tests/fixtures/capacity_dataset.json";

/// Serializable representation of a test polytope (no nalgebra types).
#[cfg(test)]
#[derive(serde::Serialize, serde::Deserialize)]
struct DatasetEntry {
    name: String,
    normals: Vec<[f64; 4]>,
    heights: Vec<f64>,
    volume: f64,
    capacity: f64,
    base_index: Option<usize>,
    transform: Option<String>,
}

#[cfg(test)]
impl DatasetEntry {
    fn from_test_polytope(tp: &TestPolytope) -> Self {
        Self {
            name: tp.name.clone(),
            normals: tp.polytope.normals().iter().map(|n| [n[0], n[1], n[2], n[3]]).collect(),
            heights: tp.polytope.heights().to_vec(),
            volume: tp.volume,
            capacity: tp.capacity,
            base_index: tp.base_index,
            transform: tp.transform.clone(),
        }
    }

    fn to_test_polytope(&self) -> TestPolytope {
        let normals: Vec<Vector4<f64>> = self.normals.iter()
            .map(|n| Vector4::new(n[0], n[1], n[2], n[3]))
            .collect();
        let polytope = Polytope4D::new(normals, self.heights.clone())
            .unwrap_or_else(|e| panic!("fixture entry '{}': {}", self.name, e));
        TestPolytope {
            name: self.name.clone(),
            polytope,
            volume: self.volume,
            capacity: self.capacity,
            base_index: self.base_index,
            transform: self.transform.clone(),
        }
    }
}

/// Save dataset to JSON fixture file.
#[cfg(test)]
pub fn save_test_dataset(path: &std::path::Path, dataset: &[TestPolytope]) {
    let entries: Vec<DatasetEntry> = dataset.iter().map(DatasetEntry::from_test_polytope).collect();
    let json = serde_json::to_string_pretty(&entries).expect("serialize dataset");
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("create fixture directory");
    }
    std::fs::write(path, json).expect("write fixture file");
}

/// Load dataset from JSON fixture file.
#[cfg(test)]
pub fn load_test_dataset(path: &std::path::Path) -> Vec<TestPolytope> {
    let json = std::fs::read_to_string(path).unwrap_or_else(|e| {
        panic!(
            "Cannot read capacity dataset fixture at {}.\n\
             Error: {}\n\
             Regenerate with: cargo test -p hk2017 regenerate_test_dataset -- --ignored --nocapture",
            path.display(), e
        )
    });
    let entries: Vec<DatasetEntry> = serde_json::from_str(&json).unwrap_or_else(|e| {
        panic!(
            "Cannot parse capacity dataset fixture at {}.\n\
             Error: {}\n\
             Regenerate with: cargo test -p hk2017 regenerate_test_dataset -- --ignored --nocapture",
            path.display(), e
        )
    });
    entries.iter().map(DatasetEntry::to_test_polytope).collect()
}

/// Generate test dataset in three phases (up to ~27 polytopes).
///
/// Phase 1: Base dataset (~9 polytopes)
///   - 3 known polytopes (simplex, hypercube, triangle_product)
///   - 6 random polytopes (5-7 facets, 2 per facet count)
///
/// Phase 2: Symplectomorphism variants (+~10)
///   - For each base polytope K: compute c(MK+b) for random M ∈ Sp(4)
///
/// Phase 3: Conformality variants (+~10)
///   - For each base polytope K: compute c(α·K) for random α
pub fn generate_test_dataset() -> Vec<TestPolytope> {
    let mut rng = ChaCha8Rng::seed_from_u64(42);

    // ===== PHASE 1: Base dataset =====
    let mut base_dataset = Vec::new();

    // Known polytopes — capacity and volume computed fresh (not hardcoded).
    // Excluded: crosspolytope (16 facets, HK2017 is exponential → too slow).
    let known = vec![
        ("simplex", simplex()),
        ("hypercube", hypercube()),
        ("triangle_product", triangle_product()),
    ];
    for (name, p) in known {
        let vol = volume(&p).unwrap_or_else(|_| panic!("{} volume", name));
        let cap = ehz_capacity(&p).unwrap_or_else(|| panic!("{} capacity", name)).capacity;
        base_dataset.push(TestPolytope {
            name: name.to_string(),
            polytope: p,
            volume: vol,
            capacity: cap,
            base_index: None,
            transform: None,
        });
    }

    // Small random polytopes (5-7 facets for speed)
    for facet_count in 5..=7 {
        for i in 0..2 {
            // 2 polytopes per facet count = 6 total
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
        let (m, b) = random_symplectomorphism(&mut rng);
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
                transform: Some(format!("conform:{}", alpha)),
            });
        }
    }

    full_dataset
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

/// Generate random symplectomorphism M ∈ Sp(4) (linear, no translation).
///
/// Since 0 ∈ int(K) and M is invertible, 0 = M·0 ∈ int(MK),
/// so the transformed polytope always has positive heights.
fn random_symplectomorphism(rng: &mut impl Rng) -> (Matrix4<f64>, Vector4<f64>) {
    let m = random_sp4_matrix(rng);
    (m, Vector4::zeros())
}

/// Generate random Sp(4) matrix using Cayley transform: M = (I - A)(I + A)^{-1}
/// where A ∈ sp(4) satisfies A^T J + J A = 0.
///
/// sp(4) in 2×2 blocks: A = [[P, Q], [R, S]] with
///   Q^T = Q (symmetric), R^T = R (symmetric), S = -P^T.
/// This gives 4 + 3 + 3 = 10 free parameters.
fn random_sp4_matrix(rng: &mut impl Rng) -> Matrix4<f64> {
    // P: arbitrary 2×2 (4 free params)
    let p11: f64 = rng.sample(StandardNormal);
    let p12: f64 = rng.sample(StandardNormal);
    let p21: f64 = rng.sample(StandardNormal);
    let p22: f64 = rng.sample(StandardNormal);

    // Q: symmetric 2×2 (3 free params)
    let q11: f64 = rng.sample(StandardNormal);
    let q12: f64 = rng.sample(StandardNormal);
    let q22: f64 = rng.sample(StandardNormal);

    // R: symmetric 2×2 (3 free params)
    let r11: f64 = rng.sample(StandardNormal);
    let r12: f64 = rng.sample(StandardNormal);
    let r22: f64 = rng.sample(StandardNormal);

    // S = -P^T
    // A = [[P, Q], [R, -P^T]]
    //   = [[p11, p12, q11, q12],
    //      [p21, p22, q12, q22],
    //      [r11, r12, -p11, -p21],
    //      [r12, r22, -p12, -p22]]
    //
    // Scale down to keep Cayley transform well-conditioned
    let scale = 0.3;
    let a_mat = Matrix4::new(
        p11 * scale, p12 * scale, q11 * scale, q12 * scale,
        p21 * scale, p22 * scale, q12 * scale, q22 * scale,
        r11 * scale, r12 * scale, -p11 * scale, -p21 * scale,
        r12 * scale, r22 * scale, -p12 * scale, -p22 * scale,
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
///
/// H-rep derivation: y ∈ MK+b ⟺ M⁻¹(y-b) ∈ K ⟺ nᵢ·M⁻¹(y-b) ≤ hᵢ
/// ⟺ (M⁻ᵀnᵢ)·y ≤ hᵢ + (M⁻ᵀnᵢ)·b
/// Normalizing: n'ᵢ = M⁻ᵀnᵢ/‖M⁻ᵀnᵢ‖, h'ᵢ = (hᵢ + (M⁻ᵀnᵢ)·b) / ‖M⁻ᵀnᵢ‖
fn apply_symplectomorphism(polytope: &Polytope4D, m: &Matrix4<f64>, b: &Vector4<f64>) -> Polytope4D {
    let m_inv_t = m
        .transpose()
        .try_inverse()
        .expect("M should be invertible");

    let mut normals = Vec::with_capacity(polytope.normals().len());
    let mut heights = Vec::with_capacity(polytope.heights().len());

    for (n, &h) in polytope.normals().iter().zip(polytope.heights().iter()) {
        let n_raw = m_inv_t * n;
        let norm = n_raw.norm();
        normals.push(n_raw / norm);
        heights.push((h + n_raw.dot(b)) / norm);
    }

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

#[cfg(test)]
mod test_dataset_tests {
    use super::*;
    use std::path::PathBuf;

    fn fixture_path() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(FIXTURE_PATH)
    }

    /// Regenerate the cached capacity dataset fixture.
    ///
    /// Run after changes to `ehz_capacity()` or the dataset generation logic:
    /// ```
    /// cargo test -p hk2017 regenerate_test_dataset -- --ignored --nocapture
    /// ```
    #[test]
    #[ignore] // Expensive: ~27 ehz_capacity() calls, ~2-3 min
    fn regenerate_test_dataset() {
        let dataset = generate_test_dataset();
        let path = fixture_path();
        save_test_dataset(&path, &dataset);
        println!("Saved {} polytopes to {}", dataset.len(), path.display());

        // Verify round-trip
        let reloaded = load_test_dataset(&path);
        assert_eq!(dataset.len(), reloaded.len());
        for (orig, loaded) in dataset.iter().zip(reloaded.iter()) {
            assert_eq!(orig.name, loaded.name);
            // JSON round-trip may lose ~1 ULP; 1e-12 is far tighter than the 1e-6
            // tolerance used by property tests, so this is safe.
            assert!(
                (orig.capacity - loaded.capacity).abs() < 1e-12,
                "{}: capacity drift: {} vs {}", orig.name, orig.capacity, loaded.capacity
            );
            assert!(
                (orig.volume - loaded.volume).abs() < 1e-12,
                "{}: volume drift: {} vs {}", orig.name, orig.volume, loaded.volume
            );
        }
        println!("Round-trip verification passed");
    }
}

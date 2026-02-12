//! Property testing dataset infrastructure.
//!
//! This module is primarily intended for use in tests, but is defined as a normal
//! module to allow cross-crate test imports.
//!
//! ## Architecture: catalog + cached fixture
//!
//! The polytope catalog (`polytope_catalog()`) deterministically generates ~27 test
//! polytopes. Computing their capacities is expensive (~2-3 min), so values are
//! cached as a JSON fixture:
//!
//! - **Default path:** Property tests load from `tests/fixtures/capacity_dataset.json`
//! - **Regeneration:** Run `cargo test -p hk2017 regenerate_test_dataset -- --ignored`
//!   after changes to `ehz_capacity_pruned()` or the catalog generation logic.
//!
//! The fixture is committed to the repo so all worktrees have it immediately.
//!
//! ## Catalog phases
//!
//! - Phase 1: Base polytopes (known and random)
//! - Phase 2: Symplectomorphism variants (apply random M ∈ Sp(4))
//! - Phase 3: Conformality variants (scale by random α)
//!
//! ## Capacity variants
//!
//! - `capacity`: from `ehz_capacity_pruned()` — the production code path.
//! - `capacity_unpruned`: from `ehz_capacity()` — only for base polytopes,
//!   to verify pruned ≈ unpruned agreement. Variants skip the expensive
//!   unpruned computation.

use geom::polytope::Polytope4D;
use geom::test_utils::{simplex, lagrangian_triangle_product};
use nalgebra::{Matrix4, Vector4};
use rand::Rng;
use rand_distr::StandardNormal;
use rand_chacha::ChaCha8Rng;
use rand::SeedableRng;

/// Entry in the polytope catalog (no computed values).
#[derive(Clone, Debug)]
pub struct CatalogEntry {
    pub name: String,
    pub polytope: Polytope4D,
    /// Index of the base polytope this was derived from (None for base polytopes).
    pub base_index: Option<usize>,
    /// Transformation type: None (base), "sympl", or "conform:1.50".
    pub transform: Option<String>,
}

/// Test polytope with precomputed volume and capacity.
#[derive(Clone, Debug)]
pub struct TestPolytope {
    pub name: String,
    pub polytope: Polytope4D,
    pub volume: f64,
    /// EHZ capacity from `ehz_capacity_pruned()` (production code path).
    pub capacity: f64,
    /// EHZ capacity from `ehz_capacity()` (unpruned). Only set for base polytopes.
    pub capacity_unpruned: Option<f64>,
    /// Index of the base polytope this was derived from.
    pub base_index: Option<usize>,
    /// Transformation type: None (base), "sympl", or "conform:1.50".
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
    #[serde(default)]
    capacity_unpruned: Option<f64>,
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
            capacity_unpruned: tp.capacity_unpruned,
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
            capacity_unpruned: self.capacity_unpruned,
            base_index: self.base_index,
            transform: self.transform.clone(),
        }
    }
}

/// Save dataset to JSON fixture file (atomic: writes to temp file, then renames).
#[cfg(test)]
pub fn save_test_dataset(path: &std::path::Path, dataset: &[TestPolytope]) {
    let entries: Vec<DatasetEntry> = dataset.iter().map(DatasetEntry::from_test_polytope).collect();
    let json = serde_json::to_string_pretty(&entries).expect("serialize dataset");
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("create fixture directory");
    }
    // Atomic write: temp file in same directory, then rename.
    let tmp_path = path.with_extension("json.tmp");
    std::fs::write(&tmp_path, json).expect("write temp fixture file");
    std::fs::rename(&tmp_path, path).expect("rename temp fixture to final path");
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

/// Deterministically generate the test polytope catalog (~0ms, no capacity computation).
///
/// This is the single source of truth for which polytopes exist in the test suite.
/// Both the default suite (staleness check) and fixture generation call this function.
///
/// ## Phases
///
/// - Phase 1: 3 known polytopes + 6 random (5-7 facets, 2 each) = 9 base
/// - Phase 2: 1 symplectomorphism variant per base = 9 variants
/// - Phase 3: 1 conformality variant per base = 9 variants
/// - Total: ~27 polytopes
pub fn polytope_catalog() -> Vec<CatalogEntry> {
    let mut rng = ChaCha8Rng::seed_from_u64(42);

    // ===== PHASE 1: Base polytopes =====
    let mut base_entries = Vec::new();

    // Known polytopes.
    // Excluded: crosspolytope (16 facets, HK2017 is exponential → too slow).
    let known = vec![
        ("simplex", simplex()),
        ("hypercube", hypercube()),
        ("lagrangian_triangle_product", lagrangian_triangle_product()),
    ];
    for (name, p) in known {
        base_entries.push(CatalogEntry {
            name: name.to_string(),
            polytope: p,
            base_index: None,
            transform: None,
        });
    }

    // Small random polytopes (5-7 facets for speed)
    for facet_count in 5..=7 {
        for i in 0..2 {
            let p = generate_random_bounded_polytope(facet_count, &mut rng);
            base_entries.push(CatalogEntry {
                name: format!("random_f{}_n{}", facet_count, i),
                polytope: p,
                base_index: None,
                transform: None,
            });
        }
    }

    // ===== PHASE 2: Symplectomorphism variants =====
    let mut full_catalog = base_entries.clone();

    for (i, entry) in base_entries.iter().enumerate() {
        let (m, b) = random_symplectomorphism(&mut rng);
        let transformed = apply_symplectomorphism(&entry.polytope, &m, &b);
        full_catalog.push(CatalogEntry {
            name: format!("{}_sympl", entry.name),
            polytope: transformed,
            base_index: Some(i),
            transform: Some("sympl".to_string()),
        });
    }

    // ===== PHASE 3: Conformality variants =====
    for (i, entry) in base_entries.iter().enumerate() {
        let alpha: f64 = rng.gen_range(0.5..2.0);
        let scaled = scale_polytope(&entry.polytope, alpha);
        full_catalog.push(CatalogEntry {
            name: format!("{}_scale_{:.2}", entry.name, alpha),
            polytope: scaled,
            base_index: Some(i),
            transform: Some(format!("conform:{}", alpha)),
        });
    }

    full_catalog
}

/// Known literature capacity values for validation.
pub const LITERATURE_VALUES: &[(&str, f64)] = &[
    ("simplex", 0.25),
    ("hypercube", 4.0),
    ("lagrangian_triangle_product", 1.5),
];

/// Generate test dataset with fail-fast inline validation.
///
/// Calls `polytope_catalog()`, then computes capacities:
/// - Base polytopes: both `ehz_capacity_pruned()` and `ehz_capacity()`, assert agreement.
/// - Variants: `ehz_capacity_pruned()` only.
/// - Inline checks: literature values, conformality, symplectomorphism invariance.
///
/// Fails fast on any validation error — no point computing remaining polytopes
/// if a bug is already detected.
pub fn generate_test_dataset() -> Vec<TestPolytope> {
    use crate::{ehz_capacity, ehz_capacity_pruned};
    use geom::volume::volume;

    let catalog = polytope_catalog();
    let mut dataset: Vec<TestPolytope> = Vec::with_capacity(catalog.len());

    for entry in &catalog {
        let vol = volume(&entry.polytope)
            .unwrap_or_else(|e| panic!("'{}': volume computation failed: {}", entry.name, e));

        let cap_pruned = ehz_capacity_pruned(&entry.polytope)
            .unwrap_or_else(|| panic!("'{}': ehz_capacity_pruned() returned None", entry.name))
            .capacity;

        let cap_unpruned = if entry.base_index.is_none() {
            // Base polytope: also compute unpruned, verify agreement
            let unpruned = ehz_capacity(&entry.polytope)
                .unwrap_or_else(|| panic!("'{}': ehz_capacity() returned None", entry.name))
                .capacity;

            let rel_err = (cap_pruned - unpruned).abs() / unpruned;
            assert!(
                rel_err < 1e-6,
                "FAIL-FAST '{}': pruned ({}) ≠ unpruned ({}) capacity, rel_error = {:.2e}",
                entry.name, cap_pruned, unpruned, rel_err
            );

            Some(unpruned)
        } else {
            None
        };

        // Fail-fast: literature values
        for &(lit_name, lit_cap) in LITERATURE_VALUES {
            if entry.name == lit_name {
                let rel_err = (cap_pruned - lit_cap).abs() / lit_cap;
                assert!(
                    rel_err < 1e-6,
                    "FAIL-FAST '{}': capacity {} disagrees with literature value {}, rel_error = {:.2e}",
                    entry.name, cap_pruned, lit_cap, rel_err
                );
            }
        }

        // Fail-fast: symplectomorphism invariance
        if entry.transform.as_deref() == Some("sympl") {
            let base_idx = entry.base_index.unwrap();
            let base_cap = dataset[base_idx].capacity;
            let rel_err = (cap_pruned - base_cap).abs() / base_cap;
            assert!(
                rel_err < 1e-6,
                "FAIL-FAST '{}': c(MK) = {} ≠ c(K) = {} for base '{}', rel_error = {:.2e}",
                entry.name, cap_pruned, base_cap, dataset[base_idx].name, rel_err
            );
        }

        // Fail-fast: conformality c(αK) = α²·c(K)
        if let Some(transform) = &entry.transform {
            if let Some(alpha_str) = transform.strip_prefix("conform:") {
                let alpha: f64 = alpha_str.parse().expect("valid scale factor");
                let base_idx = entry.base_index.unwrap();
                let expected = alpha * alpha * dataset[base_idx].capacity;
                let rel_err = (cap_pruned - expected).abs() / expected;
                assert!(
                    rel_err < 1e-6,
                    "FAIL-FAST '{}': c({:.2}·K) = {} ≠ {:.2}²·c(K) = {} for base '{}', rel_error = {:.2e}",
                    entry.name, alpha, cap_pruned, alpha, expected, dataset[base_idx].name, rel_err
                );
            }
        }

        dataset.push(TestPolytope {
            name: entry.name.clone(),
            polytope: entry.polytope.clone(),
            volume: vol,
            capacity: cap_pruned,
            capacity_unpruned: cap_unpruned,
            base_index: entry.base_index,
            transform: entry.transform.clone(),
        });

        eprintln!("  {} — cap={:.6}, vol={:.6}{}", entry.name, cap_pruned, vol,
            if cap_unpruned.is_some() { " (unpruned verified)" } else { "" });
    }

    dataset
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
    /// Run after changes to `ehz_capacity_pruned()` or the catalog generation logic:
    /// ```
    /// cargo test -p hk2017 regenerate_test_dataset -- --ignored --nocapture
    /// ```
    #[test]
    #[ignore] // Expensive: ~27 capacity computations, ~2-3 min
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
            assert_eq!(
                orig.capacity_unpruned.is_some(),
                loaded.capacity_unpruned.is_some(),
                "{}: capacity_unpruned presence mismatch", orig.name
            );
            if let (Some(orig_unp), Some(loaded_unp)) = (orig.capacity_unpruned, loaded.capacity_unpruned) {
                assert!(
                    (orig_unp - loaded_unp).abs() < 1e-12,
                    "{}: capacity_unpruned drift: {} vs {}", orig.name, orig_unp, loaded_unp
                );
            }
        }
        println!("Round-trip verification passed");
    }
}

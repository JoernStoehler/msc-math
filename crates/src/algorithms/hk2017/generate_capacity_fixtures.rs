//! Property testing dataset infrastructure for EHZ capacity validation.
//!
//! Generates a deterministic catalog of ~33 test polytopes (known, random,
//! symplectomorphism variants, conformality variants) and computes their
//! capacities for use as regression fixtures.
//!
//! ## Architecture: catalog + cached fixture
//!
//! The polytope catalog (`polytope_catalog()`) deterministically generates ~33 test
//! polytopes. Computing their capacities is expensive (~2-3 min), so values are
//! cached as a JSON fixture:
//!
//! - **Default path:** `tests/fixtures/capacity_dataset.json` (relative to crate root)
//! - **Regeneration:** Run `cargo test --release regenerate_test_dataset -- --ignored`
//!
//! The fixture is committed to the repo so all worktrees have it immediately.
//!
//! ## Catalog phases
//!
//! - Phase 1: Base polytopes (3 known + 8 random) = 11 base
//! - Phase 2: Symplectomorphism variants (1 per base) = 11 variants
//! - Phase 3: Conformality variants (1 per base, random scale) = 11 variants
//! - Total: ~33 polytopes
//!
//! Mathematical correspondence: [thm:sympl-invariance], [thm:conformality]

use crate::geom::known_polytopes;
use crate::geom::polytope::Polytope4D;
use nalgebra::{Matrix4, Vector4};
use rand::Rng;
use rand_chacha::ChaCha8Rng;
use rand_distr::StandardNormal;
use rand::SeedableRng;

/// Entry in the polytope catalog (no computed values yet).
#[derive(Clone, Debug)]
pub struct CatalogEntry {
    /// Human-readable name for reporting and fixture lookup.
    pub name: String,
    /// The polytope instance.
    pub polytope: Polytope4D,
    /// Index of the base polytope this was derived from (None for base polytopes).
    pub base_index: Option<usize>,
    /// Transformation type: None (base), "sympl", or "conform:{alpha}".
    pub transform: Option<String>,
}

/// Test polytope with precomputed volume and capacity values.
#[derive(Clone, Debug)]
pub struct TestPolytope {
    /// Human-readable name.
    pub name: String,
    /// The polytope instance.
    pub polytope: Polytope4D,
    /// 4D volume of the polytope.
    pub volume: f64,
    /// EHZ capacity from `ehz_capacity()` (pruned, production code path).
    pub capacity: f64,
    /// EHZ capacity from `ehz_capacity_unpruned()`. Only set for base polytopes.
    pub capacity_unpruned: Option<f64>,
    /// EHZ capacity from `billiard_capacity()`. Only set for Lagrangian products.
    pub capacity_billiard: Option<f64>,
    /// Index of the base polytope this was derived from.
    pub base_index: Option<usize>,
    /// Transformation type.
    pub transform: Option<String>,
}

/// Path to the cached dataset fixture, relative to the crate root.
pub const FIXTURE_PATH: &str = "tests/fixtures/capacity_dataset.json";

/// Catalog version tag. Bump when `polytope_catalog()` changes (new polytopes,
/// seed changes, transformation logic changes). The fixture stores this version;
/// consumer tests check it on load and panic with a regeneration message on mismatch.
pub const CATALOG_VERSION: u32 = 1;

/// Serializable representation of a test polytope (no nalgebra types).
///
/// Scalar-only: contains all fixture data except `Polytope4D`. Most fixture tests
/// only need scalar fields (capacity, volume, etc.) and can use `load_dataset_entries()`
/// to skip the expensive `Polytope4D::new()` call during deserialization.
#[cfg(test)]
#[derive(serde::Serialize, serde::Deserialize)]
pub(crate) struct DatasetEntry {
    pub(crate) name: String,
    pub(crate) normals: Vec<[f64; 4]>,
    pub(crate) heights: Vec<f64>,
    pub(crate) volume: f64,
    pub(crate) capacity: f64,
    #[serde(default)]
    pub(crate) capacity_unpruned: Option<f64>,
    #[serde(default)]
    pub(crate) capacity_billiard: Option<f64>,
    pub(crate) base_index: Option<usize>,
    pub(crate) transform: Option<String>,
}

#[cfg(test)]
impl DatasetEntry {
    fn from_test_polytope(tp: &TestPolytope) -> Self {
        let duals = tp.polytope.dual_vertices_f64();
        // Derive unit normals and heights from dual vertices for JSON fixture format:
        // n_i = a_i / ||a_i||, h_i = 1 / ||a_i||
        let normals: Vec<[f64; 4]> = duals
            .iter()
            .map(|a| {
                let norm = a.norm();
                let n = a / norm;
                [n[0], n[1], n[2], n[3]]
            })
            .collect();
        let heights: Vec<f64> = duals.iter().map(|a| 1.0 / a.norm()).collect();
        Self {
            name: tp.name.clone(),
            normals,
            heights,
            volume: tp.volume,
            capacity: tp.capacity,
            capacity_unpruned: tp.capacity_unpruned,
            capacity_billiard: tp.capacity_billiard,
            base_index: tp.base_index,
            transform: tp.transform.clone(),
        }
    }

    fn to_test_polytope(&self) -> TestPolytope {
        // Reconstruct dual vertices a_i = n_i / h_i from stored normals and heights.
        // Polytope4D::new() expects dual-vertex (halfspace) representation.
        let halfspaces: Vec<Vector4<f64>> = self
            .normals
            .iter()
            .zip(self.heights.iter())
            .map(|(n, &h)| Vector4::new(n[0] / h, n[1] / h, n[2] / h, n[3] / h))
            .collect();
        let polytope = Polytope4D::from_f64(halfspaces)
            .unwrap_or_else(|e| panic!("fixture entry '{}': {}", self.name, e));
        TestPolytope {
            name: self.name.clone(),
            polytope,
            volume: self.volume,
            capacity: self.capacity,
            capacity_unpruned: self.capacity_unpruned,
            capacity_billiard: self.capacity_billiard,
            base_index: self.base_index,
            transform: self.transform.clone(),
        }
    }
}

/// Top-level JSON wrapper with catalog version tag.
#[cfg(test)]
#[derive(serde::Serialize, serde::Deserialize)]
struct DatasetFile {
    catalog_version: u32,
    entries: Vec<DatasetEntry>,
}

/// Save dataset to JSON fixture file (atomic: writes to temp file, then renames).
#[cfg(test)]
pub(crate) fn save_test_dataset(path: &std::path::Path, dataset: &[TestPolytope]) {
    let file = DatasetFile {
        catalog_version: CATALOG_VERSION,
        entries: dataset.iter().map(DatasetEntry::from_test_polytope).collect(),
    };
    let json = serde_json::to_string_pretty(&file).expect("serialize dataset");
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("create fixture directory");
    }
    // Atomic write: temp file in same directory, then rename.
    let tmp_path = path.with_extension("json.tmp");
    std::fs::write(&tmp_path, json).expect("write temp fixture file");
    std::fs::rename(&tmp_path, path).expect("rename temp fixture to final path");
}

/// Read and parse the fixture file, checking catalog version.
#[cfg(test)]
fn read_fixture(path: &std::path::Path) -> DatasetFile {
    let json = std::fs::read_to_string(path).unwrap_or_else(|e| {
        panic!(
            "Cannot read capacity dataset fixture at {}.\n\
             Error: {}\n\
             Regenerate with: cargo test --release regenerate_test_dataset -- --ignored --nocapture",
            path.display(),
            e
        )
    });
    let file: DatasetFile = serde_json::from_str(&json).unwrap_or_else(|e| {
        panic!(
            "Cannot parse capacity dataset fixture at {}.\n\
             Error: {}\n\
             Regenerate with: cargo test --release regenerate_test_dataset -- --ignored --nocapture",
            path.display(),
            e
        )
    });
    assert!(
        file.catalog_version == CATALOG_VERSION,
        "Fixture catalog_version ({}) != code CATALOG_VERSION ({}).\n\
         The polytope catalog has changed since the fixture was generated.\n\
         Regenerate with: cargo test --release regenerate_test_dataset -- --ignored --nocapture",
        file.catalog_version,
        CATALOG_VERSION,
    );
    file
}

/// Load dataset from JSON fixture file.
#[cfg(test)]
pub(crate) fn load_test_dataset(path: &std::path::Path) -> Vec<TestPolytope> {
    read_fixture(path)
        .entries
        .iter()
        .map(DatasetEntry::to_test_polytope)
        .collect()
}

/// Load dataset from JSON fixture file as scalar entries (no `Polytope4D` construction).
///
/// Returns `Vec<DatasetEntry>` with all fixture fields except `Polytope4D`.
/// Skips the expensive `Polytope4D::new()` calls in `load_test_dataset()`.
/// Use for tests that only need scalar fields (capacity, volume, name, etc.).
#[cfg(test)]
pub(crate) fn load_dataset_entries(path: &std::path::Path) -> Vec<DatasetEntry> {
    read_fixture(path).entries
}

/// Deterministically generate the test polytope catalog (no capacity computation).
///
/// This is the single source of truth for which polytopes exist in the test suite.
/// Both fixture regeneration and staleness checks call this function.
///
/// ## Phases
///
/// - Phase 1: 3 known polytopes + 8 random (5-8 facets, 2 each) = 11 base
/// - Phase 2: 1 symplectomorphism variant per base = 11 variants
/// - Phase 3: 1 conformality variant per base = 11 variants
/// - Total: ~33 polytopes
///
/// [thm:sympl-invariance], [thm:conformality]: variant generation properties.
pub fn polytope_catalog() -> Vec<CatalogEntry> {
    let mut rng = ChaCha8Rng::seed_from_u64(42);

    // Phase 1: Base polytopes.
    let mut base_entries = Vec::new();

    // Known polytopes (from geom::known_polytopes, single source of truth).
    // Excluded: crosspolytope (16 facets, HK2017 is exponential -> too slow).
    let known = vec![
        known_polytopes::simplex(),
        known_polytopes::hypercube(),
        known_polytopes::lagrangian_triangle_product(),
    ];
    for kp in known {
        base_entries.push(CatalogEntry {
            name: kp.name.to_string(),
            polytope: kp.polytope.clone(),
            base_index: None,
            transform: None,
        });
    }

    // Small random polytopes (5-8 facets).
    for facet_count in 5..=8 {
        for i in 0..2 {
            let p = crate::geom::test_utils::random_bounded_polytope(facet_count, &mut rng);
            base_entries.push(CatalogEntry {
                name: format!("random_f{}_n{}", facet_count, i),
                polytope: p,
                base_index: None,
                transform: None,
            });
        }
    }

    // Phase 2: Symplectomorphism variants.
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

    // Phase 3: Conformality variants.
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
///
/// Delegates to `geom::known_polytopes::literature_values()` (single source of truth).
/// Excludes polytopes without a literature cross-check.
pub fn literature_values() -> Vec<(&'static str, f64)> {
    crate::geom::known_polytopes::literature_values()
}

/// Generate test dataset with fail-fast inline validation.
///
/// Calls `polytope_catalog()`, then computes capacities:
/// - Base polytopes: both `ehz_capacity()` and `ehz_capacity_unpruned()`, asserts agreement.
/// - Variants: `ehz_capacity()` only.
/// - Inline checks: literature values, conformality, symplectomorphism invariance.
///
/// Fails fast on any validation error.
pub fn generate_test_dataset() -> Vec<TestPolytope> {
    use crate::algorithms::hk2017::{ehz_capacity, ehz_capacity_unpruned};
    use crate::geom::volume::volume;

    let catalog = polytope_catalog();
    let mut dataset: Vec<TestPolytope> = Vec::with_capacity(catalog.len());

    for entry in &catalog {
        let vol = volume(&entry.polytope)
            .unwrap_or_else(|e| panic!("'{}': volume computation failed: {}", entry.name, e));

        let pruned_result = ehz_capacity(&entry.polytope)
            .unwrap_or_else(|| panic!("'{}': ehz_capacity() returned None", entry.name));
        let cap_pruned = pruned_result.result.capacity;

        // Log numerical gap if nonzero.
        let gap = pruned_result.result.numerical_gap();
        if gap > 0.0 {
            eprintln!(
                "  {} -- NUMERICAL GAP: certified={:.6} uncertain={:.6} gap={:.2e}",
                entry.name, pruned_result.result.capacity, pruned_result.result.capacity_uncertain, gap
            );
        }

        let cap_unpruned = if entry.base_index.is_none() {
            // Base polytope: also compute unpruned, verify agreement.
            let unpruned_result = ehz_capacity_unpruned(&entry.polytope)
                .unwrap_or_else(|| {
                    panic!("'{}': ehz_capacity_unpruned() returned None", entry.name)
                });
            let unpruned = unpruned_result.result.capacity;

            let rel_err = (cap_pruned - unpruned).abs() / unpruned;
            assert!(
                rel_err < 1e-6,
                "FAIL-FAST '{}': pruned ({}) != unpruned ({}) capacity, rel_error = {:.2e}",
                entry.name,
                cap_pruned,
                unpruned,
                rel_err
            );

            Some(unpruned)
        } else {
            None
        };

        // Try billiard algorithm (succeeds only for Lagrangian products).
        let cap_billiard =
            match crate::algorithms::billiard::billiard_capacity(&entry.polytope) {
                Ok(Some(result)) => {
                    let rel_err = (result.result.capacity - cap_pruned).abs() / cap_pruned;
                    assert!(
                        rel_err < 1e-6,
                        "FAIL-FAST '{}': billiard ({}) != HK2017 ({}) capacity, rel_error = {:.2e}",
                        entry.name,
                        result.result.capacity,
                        cap_pruned,
                        rel_err
                    );
                    eprintln!(
                        "  {} -- billiard={:.6} (agrees with HK2017)",
                        entry.name, result.result.capacity
                    );
                    Some(result.result.capacity)
                }
                Ok(None) => {
                    eprintln!("  {} -- billiard returned None", entry.name);
                    None
                }
                Err(_) => None, // Not a Lagrangian product.
            };

        // Fail-fast: literature values.
        for (lit_name, lit_cap) in literature_values() {
            if entry.name == lit_name {
                let rel_err = (cap_pruned - lit_cap).abs() / lit_cap;
                assert!(
                    rel_err < 1e-6,
                    "FAIL-FAST '{}': capacity {} disagrees with literature value {}, rel_error = {:.2e}",
                    entry.name, cap_pruned, lit_cap, rel_err
                );
            }
        }

        // Fail-fast: symplectomorphism invariance.
        if entry.transform.as_deref() == Some("sympl") {
            let base_idx = entry.base_index.unwrap();
            let base_cap = dataset[base_idx].capacity;
            let rel_err = (cap_pruned - base_cap).abs() / base_cap;
            assert!(
                rel_err < 1e-6,
                "FAIL-FAST '{}': c(MK) = {} != c(K) = {} for base '{}', rel_error = {:.2e}",
                entry.name,
                cap_pruned,
                base_cap,
                dataset[base_idx].name,
                rel_err
            );
        }

        // Fail-fast: conformality c(alpha*K) = alpha^2 * c(K).
        if let Some(transform) = &entry.transform {
            if let Some(alpha_str) = transform.strip_prefix("conform:") {
                let alpha: f64 = alpha_str.parse().expect("valid scale factor");
                let base_idx = entry.base_index.unwrap();
                let expected = alpha * alpha * dataset[base_idx].capacity;
                let rel_err = (cap_pruned - expected).abs() / expected;
                assert!(
                    rel_err < 1e-6,
                    "FAIL-FAST '{}': c({:.2}*K) = {} != {:.2}^2*c(K) = {} for base '{}', rel_error = {:.2e}",
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
            capacity_billiard: cap_billiard,
            base_index: entry.base_index,
            transform: entry.transform.clone(),
        });

        eprintln!(
            "  {} -- cap={:.6}, vol={:.6}{}",
            entry.name,
            cap_pruned,
            vol,
            if cap_unpruned.is_some() {
                " (unpruned verified)"
            } else {
                ""
            }
        );
    }

    dataset
}

/// Scale polytope: dual vertices -> (1/alpha) * dual vertices.
///
/// Equivalent to heights -> alpha * heights, normals unchanged.
/// Used for conformality variant generation.
fn scale_polytope(polytope: &Polytope4D, alpha: f64) -> Polytope4D {
    let halfspaces: Vec<Vector4<f64>> = polytope
        .dual_vertices_f64()
        .iter()
        .map(|a| a / alpha)
        .collect();
    Polytope4D::from_f64(halfspaces).expect("scaled polytope")
}

/// Generate a random symplectomorphism M in Sp(4) (linear, no translation).
///
/// Since 0 in int(K) and M is invertible, 0 = M*0 in int(MK),
/// so the transformed polytope always has positive heights.
fn random_symplectomorphism(rng: &mut impl Rng) -> (Matrix4<f64>, Vector4<f64>) {
    let m = random_sp4_matrix(rng);
    (m, Vector4::zeros())
}

/// Generate random Sp(4) matrix using Cayley transform: M = (I - A)(I + A)^{-1}
/// where A in sp(4) satisfies A^T J + J A = 0.
///
/// sp(4) in 2x2 blocks: A = [[P, Q], [R, S]] with
///   Q^T = Q (symmetric), R^T = R (symmetric), S = -P^T.
/// This gives 4 + 3 + 3 = 10 free parameters.
fn random_sp4_matrix(rng: &mut impl Rng) -> Matrix4<f64> {
    let p11: f64 = rng.sample(StandardNormal);
    let p12: f64 = rng.sample(StandardNormal);
    let p21: f64 = rng.sample(StandardNormal);
    let p22: f64 = rng.sample(StandardNormal);

    let q11: f64 = rng.sample(StandardNormal);
    let q12: f64 = rng.sample(StandardNormal);
    let q22: f64 = rng.sample(StandardNormal);

    let r11: f64 = rng.sample(StandardNormal);
    let r12: f64 = rng.sample(StandardNormal);
    let r22: f64 = rng.sample(StandardNormal);

    // Scale down to keep Cayley transform well-conditioned.
    let scale = 0.3;
    let a_mat = Matrix4::new(
        p11 * scale, p12 * scale, q11 * scale, q12 * scale,
        p21 * scale, p22 * scale, q12 * scale, q22 * scale,
        r11 * scale, r12 * scale, -p11 * scale, -p21 * scale,
        r12 * scale, r22 * scale, -p12 * scale, -p22 * scale,
    );

    // Cayley transform: M = (I - A)(I + A)^{-1}.
    let id = Matrix4::identity();
    let i_plus_a = id + a_mat;
    let i_minus_a = id - a_mat;

    i_plus_a
        .try_inverse()
        .map(|inv| i_minus_a * inv)
        .unwrap_or(id)
}

/// Apply symplectomorphism: K -> MK + b.
///
/// H-rep derivation: y in MK+b iff M^{-1}(y-b) in K iff n_i * M^{-1}(y-b) <= h_i
/// iff (M^{-T} n_i) * y <= h_i + (M^{-T} n_i) * b.
fn apply_symplectomorphism(
    polytope: &Polytope4D,
    m: &Matrix4<f64>,
    b: &Vector4<f64>,
) -> Polytope4D {
    let m_inv_t = m
        .transpose()
        .try_inverse()
        .expect("M should be invertible");

    let duals = polytope.dual_vertices_f64();

    let mut halfspaces = Vec::with_capacity(duals.len());
    for a in duals {
        // Transform: a_i^T x <= 1 under x -> Mx + b gives
        // (M^{-T} a_i)^T y <= 1 + a_i^T M^{-1} b = 1 + (M^{-T} a_i)^T b
        // New dual vertex: a'_i = (M^{-T} a_i) / (1 + (M^{-T} a_i)^T b)
        let a_raw = m_inv_t * a;
        let rhs_new = 1.0 + a_raw.dot(b);
        halfspaces.push(a_raw / rhs_new);
    }

    Polytope4D::from_f64(halfspaces).expect("transformed polytope")
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
    /// Run after changes to `ehz_capacity()` or the catalog generation logic:
    /// ```text
    /// cargo test --release regenerate_test_dataset -- --ignored --nocapture
    /// ```
    #[test]
    #[ignore] // ~27 capacity computations; ~1s in release, ~2-3 min in debug
    fn regenerate_test_dataset() {
        let dataset = generate_test_dataset();
        let path = fixture_path();
        save_test_dataset(&path, &dataset);
        println!("Saved {} polytopes to {}", dataset.len(), path.display());

        // Verify round-trip.
        let reloaded = load_test_dataset(&path);
        assert_eq!(dataset.len(), reloaded.len());
        for (orig, loaded) in dataset.iter().zip(reloaded.iter()) {
            assert_eq!(orig.name, loaded.name);
            // JSON round-trip may lose ~1 ULP; 1e-12 is far tighter than 1e-6 property tests.
            assert!(
                (orig.capacity - loaded.capacity).abs() < 1e-12,
                "{}: capacity drift: {} vs {}",
                orig.name,
                orig.capacity,
                loaded.capacity
            );
            assert!(
                (orig.volume - loaded.volume).abs() < 1e-12,
                "{}: volume drift: {} vs {}",
                orig.name,
                orig.volume,
                loaded.volume
            );
            assert_eq!(
                orig.capacity_unpruned.is_some(),
                loaded.capacity_unpruned.is_some(),
                "{}: capacity_unpruned presence mismatch",
                orig.name
            );
            if let (Some(orig_unp), Some(loaded_unp)) =
                (orig.capacity_unpruned, loaded.capacity_unpruned)
            {
                assert!(
                    (orig_unp - loaded_unp).abs() < 1e-12,
                    "{}: capacity_unpruned drift: {} vs {}",
                    orig.name,
                    orig_unp,
                    loaded_unp
                );
            }
            assert_eq!(
                orig.capacity_billiard.is_some(),
                loaded.capacity_billiard.is_some(),
                "{}: capacity_billiard presence mismatch",
                orig.name
            );
            if let (Some(orig_bil), Some(loaded_bil)) =
                (orig.capacity_billiard, loaded.capacity_billiard)
            {
                assert!(
                    (orig_bil - loaded_bil).abs() < 1e-12,
                    "{}: capacity_billiard drift: {} vs {}",
                    orig.name,
                    orig_bil,
                    loaded_bil
                );
            }
        }
        println!("Round-trip verification passed");
    }
}

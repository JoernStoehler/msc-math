//! Tests for hk2017: conformality property c(alpha*K) = alpha^2 * c(K).
//!
//! Proposition: EHZ capacity is degree-2 homogeneous: scaling a polytope by alpha
//! scales its capacity by alpha^2. [thm:conformality]
//! Reference: [thm:conformality]
//!
//! Strategy: fixture-based (conformality variants in the pre-computed dataset) +
//! direct computation (release-mode, hypercube scaled by e).

use crate::algorithms::hk2017::ehz_capacity_unpruned;

use std::path::PathBuf;
use std::sync::LazyLock;

use super::generate_capacity_fixtures::{load_test_dataset, TestPolytope, FIXTURE_PATH};

/// Shared dataset loaded from cached fixture.
static DATASET: LazyLock<Vec<TestPolytope>> = LazyLock::new(|| {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(FIXTURE_PATH);
    load_test_dataset(&path)
});

// ── Fixture-based conformality ──

/// Verify conformality c(alpha*K) = alpha^2 * c(K) from fixture data.
///
/// Checks all entries with transform "conform:{alpha}" against their base polytope.
/// Also verifies volume scaling: vol(alpha*K) = alpha^4 * vol(K).
#[test]
fn capacity_conformality() {
    let dataset = &*DATASET;

    let conformality_tests: Vec<_> = dataset
        .iter()
        .filter(|e| {
            e.transform
                .as_ref()
                .is_some_and(|t| t.starts_with("conform:"))
        })
        .collect();

    for entry in &conformality_tests {
        let base_idx = entry
            .base_index
            .expect("conformality variant has base_index");
        let base = &dataset[base_idx];

        // Extract scale factor from transform string "conform:1.50".
        let alpha: f64 = entry
            .transform
            .as_ref()
            .and_then(|t| t.strip_prefix("conform:"))
            .and_then(|s| s.parse().ok())
            .expect("valid scale factor");

        // Capacity conformality: c(alpha*K) = alpha^2 * c(K).
        let expected_cap = alpha * alpha * base.capacity;
        let cap_error = (entry.capacity - expected_cap).abs() / expected_cap;
        assert!(
            cap_error < 1e-6,
            "{}: conformality failed: c({:.2}*{}) = {}, expected {:.2}^2 * c({}) = {}, \
             rel_error = {:.2e}",
            entry.name,
            alpha,
            base.name,
            entry.capacity,
            alpha,
            base.name,
            expected_cap,
            cap_error
        );

        // Volume scaling: vol(alpha*K) = alpha^4 * vol(K).
        let expected_vol = alpha.powi(4) * base.volume;
        let vol_error = (entry.volume - expected_vol).abs() / expected_vol;
        assert!(
            vol_error < 1e-6,
            "{}: volume conformality failed: rel_error = {:.2e}",
            entry.name,
            vol_error
        );
    }

    assert!(
        !conformality_tests.is_empty(),
        "expected at least one conformality variant in fixture"
    );
}

// ── Direct computation ──

/// Verify conformality on hypercube scaled by e (transcendental).
///
/// Uses lambda = e (transcendental) to ensure numerical coincidences are impossible.
/// Expected: c(e * K) = e^2 * c(K).
///
/// Why #[ignore]: F=8 unpruned x 2 = ~48s debug, ~0.6s release.
/// Run: `cargo test --release capacity_scales_quadratically -- --ignored`
#[test]
#[ignore] // ~48s debug, ~0.6s release
fn capacity_scales_quadratically() {
    use crate::geom::known_polytopes;

    let scale = std::f64::consts::E;

    let kp = known_polytopes::hypercube();
    let unit_cap = ehz_capacity_unpruned(&kp.polytope)
        .unwrap()
        .result
        .capacity;

    let scaled_cube = crate::geom::test_utils::scaled_hypercube(scale);
    let scaled_cap = ehz_capacity_unpruned(&scaled_cube)
        .unwrap()
        .result
        .capacity;

    let expected = unit_cap * scale * scale;
    let relative_error = ((scaled_cap - expected) / expected).abs();

    assert!(
        relative_error < 1e-4,
        "capacity scaling failed: scale={scale}, unit_cap={unit_cap}, \
         scaled_cap={scaled_cap}, expected={expected}, relative_error={relative_error}"
    );
}

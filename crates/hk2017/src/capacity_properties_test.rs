//! Property tests for EHZ capacity computation.
//!
//! Verifies mathematical properties:
//! - Positivity: c_EHZ(K) > 0 for all valid polytopes
//! - Conformality: c_EHZ(λK) = λ²·c_EHZ(K)
//! - Symplectomorphism invariance: c_EHZ(MK+b) = c_EHZ(K)
//! - Monotonicity: K₁ ⊆ K₂ ⟹ c_EHZ(K₁) ≤ c_EHZ(K₂)

use crate::test_dataset::{load_test_dataset, TestPolytope, FIXTURE_PATH};
use geom::polytope::Polytope4D;
use nalgebra::Vector4;
use std::path::PathBuf;
use std::sync::LazyLock;

/// Shared dataset loaded from cached fixture (fast, <1ms).
///
/// If the fixture is missing, panics with instructions to regenerate:
/// `cargo test -p hk2017 regenerate_test_dataset -- --ignored --nocapture`
static DATASET: LazyLock<Vec<TestPolytope>> = LazyLock::new(|| {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(FIXTURE_PATH);
    load_test_dataset(&path)
});

#[test]
fn capacity_positive_on_all_polytopes() {
    let dataset = &*DATASET;

    for entry in dataset {
        assert!(
            entry.capacity > 0.0,
            "{}: capacity should be positive, got {}",
            entry.name, entry.capacity
        );
    }

    println!("✓ Verified capacity > 0 for {} polytopes", dataset.len());
}

#[test]
fn capacity_conformality() {
    let dataset = &*DATASET;

    // Extract conformality variants (scaled versions)
    let conformality_tests: Vec<_> = dataset
        .iter()
        .filter(|e| {
            e.transform
                .as_ref()
                .map_or(false, |t| t.starts_with("conform:"))
        })
        .collect();

    for entry in &conformality_tests {
        let base_idx = entry
            .base_index
            .expect("conformality variant has base_index");
        let base = &dataset[base_idx];

        // Extract scale factor from transform string "conform:1.50"
        let alpha: f64 = entry
            .transform
            .as_ref()
            .and_then(|t| t.strip_prefix("conform:"))
            .and_then(|s| s.parse().ok())
            .expect("valid scale factor");

        // Test conformality: c(α·K) = α²·c(K)
        let expected_cap = alpha * alpha * base.capacity;
        let cap_error = (entry.capacity - expected_cap).abs() / expected_cap;
        assert!(
            cap_error < 1e-6,
            "{}: conformality failed: c({:.2}·{}) = {}, expected {:.2}²·c({}) = {}, rel_error = {:.2e}",
            entry.name, alpha, base.name, entry.capacity,
            alpha, base.name, expected_cap, cap_error
        );

        // Volume scaling: vol(α·K) = α⁴·vol(K)
        let expected_vol = alpha.powi(4) * base.volume;
        let vol_error = (entry.volume - expected_vol).abs() / expected_vol;
        assert!(
            vol_error < 1e-6,
            "{}: volume conformality failed: rel_error = {:.2e}",
            entry.name, vol_error
        );
    }

    println!(
        "✓ Verified conformality c(α·K) = α²·c(K) for {} cases",
        conformality_tests.len()
    );
}

#[test]
fn capacity_symplectomorphism_invariance() {
    let dataset = &*DATASET;

    // Extract symplectomorphism variants
    let sympl_tests: Vec<_> = dataset
        .iter()
        .filter(|e| e.transform.as_ref().map_or(false, |t| t == "sympl"))
        .collect();

    for entry in &sympl_tests {
        let base_idx = entry.base_index.expect("sympl variant has base_index");
        let base = &dataset[base_idx];

        // Test invariance: c(MK+b) = c(K)
        let cap_error = (entry.capacity - base.capacity).abs() / base.capacity;
        assert!(
            cap_error < 1e-6,
            "{}: symplectomorphism invariance failed: c(M·{}+b) = {}, expected c({}) = {}, rel_error = {:.2e}",
            entry.name, base.name, entry.capacity,
            base.name, base.capacity, cap_error
        );

        // Volume should also be invariant (Sp(4) preserves volume)
        let vol_error = (entry.volume - base.volume).abs() / base.volume;
        assert!(
            vol_error < 1e-6,
            "{}: volume invariance failed under symplectomorphism",
            entry.name
        );
    }

    println!(
        "✓ Verified symplectomorphism invariance c(MK+b) = c(K) for {} cases",
        sympl_tests.len()
    );
}

#[test]
fn capacity_monotonicity() {
    let dataset = &*DATASET;

    let mut monotonicity_pairs = Vec::new();

    // For each pair (K1, K2) in dataset, check if we can fit α·K1 ⊆ K2
    for (i, k1) in dataset.iter().enumerate() {
        for (j, k2) in dataset.iter().enumerate() {
            if i == j {
                continue;
            }

            // Find max α such that α·K1 ⊆ K2
            let vertices1 = k1.polytope.vertices();
            let max_alpha = compute_max_containment_scale(&vertices1, &k2.polytope);

            if let Some(alpha) = max_alpha {
                if alpha > 1e-6 {
                    // Non-trivial containment
                    monotonicity_pairs.push((i, j, alpha));
                }
            }
        }
    }

    println!("Found {} monotonicity test pairs", monotonicity_pairs.len());

    for (i, j, alpha) in monotonicity_pairs.iter().take(20) {
        // Test first 20 pairs
        let k1 = &dataset[*i];
        let k2 = &dataset[*j];

        // Test: c(α·K1) ≤ c(K2)
        // We know c(K1) from dataset. By conformality: c(α·K1) = α²·c(K1)
        let c_alpha_k1 = alpha * alpha * k1.capacity;

        assert!(
            c_alpha_k1 <= k2.capacity + 1e-9,
            "monotonicity failed: c({:.3}·{}) = {:.3}²·{:.4} = {:.4} should be ≤ c({}) = {:.4}",
            alpha, k1.name, alpha, k1.capacity, c_alpha_k1,
            k2.name, k2.capacity
        );
    }

    println!(
        "✓ Verified monotonicity c(α·K1) ≤ c(K2) for {} pairs",
        monotonicity_pairs.len().min(20)
    );
}

#[test]
fn sys_distribution_sanity_checks() {
    let dataset = &*DATASET;

    let sys_values: Vec<f64> = dataset
        .iter()
        .map(|e| e.capacity.powi(2) / (2.0 * e.volume))
        .collect();

    let min_sys = sys_values.iter().copied().fold(f64::INFINITY, f64::min);
    let max_sys = sys_values.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let mean_sys = sys_values.iter().sum::<f64>() / sys_values.len() as f64;

    println!("sys distribution over {} polytopes:", dataset.len());
    println!("  min:  {:.6}", min_sys);
    println!("  max:  {:.6}", max_sys);
    println!("  mean: {:.6}", mean_sys);
    println!(
        "  above 1.0: {}/{}",
        sys_values.iter().filter(|&&s| s > 1.0).count(),
        sys_values.len()
    );

    assert!(min_sys > 0.0, "all sys values should be positive");
    assert!(max_sys < 100.0, "sys values should be reasonable (< 100)");
}

/// Compute max α such that α·K1 ⊆ K2
/// Returns None if no such α exists
fn compute_max_containment_scale(
    vertices1: &[Vector4<f64>],
    polytope2: &Polytope4D,
) -> Option<f64> {
    let normals2 = polytope2.normals();
    let heights2 = polytope2.heights();

    let mut max_alpha = f64::INFINITY;

    for v in vertices1 {
        for (n, &h) in normals2.iter().zip(heights2.iter()) {
            let nv = n.dot(v);
            if nv > 1e-12 {
                // v points outward from this halfspace
                let alpha_bound = h / nv;
                max_alpha = max_alpha.min(alpha_bound);
            }
            // If nv <= 0, v points inward or is on boundary - no constraint
        }
    }

    if max_alpha.is_finite() && max_alpha > 1e-12 {
        Some(max_alpha)
    } else {
        None
    }
}

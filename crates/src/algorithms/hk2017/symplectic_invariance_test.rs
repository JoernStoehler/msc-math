//! Tests for hk2017: symplectomorphism invariance and monotonicity.
//!
//! Proposition: EHZ capacity is invariant under symplectomorphisms (c(MK+b) = c(K)
//! for M in Sp(4)) and monotone under inclusion (K1 ⊆ K2 => c(K1) <= c(K2)).
//! Reference: [thm:sympl-invariance], capacity axioms (P7: monotonicity)
//!
//! Strategy: fixture-based (symplectomorphism variants and pairwise containment
//! from the pre-computed dataset).

use crate::geom::polytope::Polytope4D;
use nalgebra::Vector4;

use std::path::PathBuf;
use std::sync::LazyLock;

use super::generate_capacity_fixtures::{load_test_dataset, TestPolytope, FIXTURE_PATH};

/// Shared dataset loaded from cached fixture.
static DATASET: LazyLock<Vec<TestPolytope>> = LazyLock::new(|| {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(FIXTURE_PATH);
    load_test_dataset(&path)
});

// ── Symplectomorphism invariance ──

/// Verify c(MK+b) = c(K) for all symplectomorphism variants in the fixture.
///
/// Each variant is generated from a base polytope by applying a random M in Sp(4).
/// Since Sp(4) preserves both capacity and volume, we check both.
#[test]
fn capacity_symplectomorphism_invariance() {
    let dataset = &*DATASET;

    let sympl_tests: Vec<_> = dataset
        .iter()
        .filter(|e| e.transform.as_ref().is_some_and(|t| t == "sympl"))
        .collect();

    for entry in &sympl_tests {
        let base_idx = entry
            .base_index
            .expect("sympl variant has base_index");
        let base = &dataset[base_idx];

        // Capacity invariance: c(MK+b) = c(K).
        let cap_error = (entry.capacity - base.capacity).abs() / base.capacity;
        assert!(
            cap_error < 1e-6,
            "{}: symplectomorphism invariance failed: c(M*{}+b) = {}, \
             expected c({}) = {}, rel_error = {:.2e}",
            entry.name,
            base.name,
            entry.capacity,
            base.name,
            base.capacity,
            cap_error
        );

        // Volume invariance: Sp(4) preserves symplectic volume = Euclidean volume in R^4.
        let vol_error = (entry.volume - base.volume).abs() / base.volume;
        assert!(
            vol_error < 1e-6,
            "{}: volume invariance failed under symplectomorphism",
            entry.name
        );
    }

    assert!(
        !sympl_tests.is_empty(),
        "expected at least one symplectomorphism variant in fixture"
    );
}

// ── Monotonicity ──

/// Verify monotonicity: if alpha*K1 fits inside K2, then c(alpha*K1) <= c(K2).
///
/// For each pair (K1, K2) in the fixture, computes the maximum alpha such that
/// alpha*K1 ⊆ K2, then checks c(alpha*K1) = alpha^2*c(K1) <= c(K2).
/// Uses conformality to avoid recomputing capacity of the scaled polytope.
#[test]
fn capacity_monotonicity() {
    let dataset = &*DATASET;
    let mut checked = 0;

    // Check a representative sample of pairs to keep test fast.
    for (i, k1) in dataset.iter().enumerate() {
        for (j, k2) in dataset.iter().enumerate() {
            if i == j {
                continue;
            }
            // Only check first 20 pairs with non-trivial containment.
            if checked >= 20 {
                break;
            }

            let vertices1 = k1.polytope.vertices_f64();
            if let Some(alpha) = compute_max_containment_scale(vertices1, &k2.polytope) {
                if alpha > 1e-6 {
                    // c(alpha*K1) = alpha^2 * c(K1) by conformality.
                    let c_alpha_k1 = alpha * alpha * k1.capacity;
                    assert!(
                        c_alpha_k1 <= k2.capacity + 1e-9,
                        "monotonicity failed: c({:.3}*{}) = {:.3}^2 * {:.4} = {:.4} \
                         should be <= c({}) = {:.4}",
                        alpha,
                        k1.name,
                        alpha,
                        k1.capacity,
                        c_alpha_k1,
                        k2.name,
                        k2.capacity
                    );
                    checked += 1;
                }
            }
        }
        if checked >= 20 {
            break;
        }
    }

    eprintln!("Verified monotonicity for {} pairs", checked);
}

/// Compute max alpha such that alpha*K1 ⊆ K2.
///
/// Returns None if no positive alpha works (e.g. K1 has a vertex whose
/// direction is not contained in K2 for any positive scaling).
fn compute_max_containment_scale(
    vertices1: &[Vector4<f64>],
    polytope2: &Polytope4D,
) -> Option<f64> {
    let normals2 = polytope2.normals_f64();
    let heights2 = polytope2.heights_f64();

    let mut max_alpha = f64::INFINITY;

    for v in vertices1 {
        for (n, &h) in normals2.iter().zip(heights2.iter()) {
            let nv = n.dot(v);
            if nv > 1e-12 {
                // v points outward from this halfspace.
                let alpha_bound = h / nv;
                max_alpha = max_alpha.min(alpha_bound);
            }
            // If nv <= 0, v is on the safe side — no constraint.
        }
    }

    if max_alpha.is_finite() && max_alpha > 1e-12 {
        Some(max_alpha)
    } else {
        None
    }
}

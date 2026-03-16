//! Tests for tube algorithm: capacity computation on symplectic polytopes.
//!
//! Proposition: The tube algorithm correctly computes c_EHZ(K) for symplectic
//! polytopes (those with no Lagrangian 2-faces), agreeing with the HK2017
//! exhaustive algorithm.
//! Reference: [alg:tube], [def:symplectic-polytope]
//!
//! Strategy: fixture-based comparison against known capacity values and
//! cross-validation with hk2017::ehz_capacity. Uses `check_symplectic` to
//! determine which known polytopes are eligible for the tube algorithm.

use crate::algorithms::tube::{check_symplectic, tube_capacity, TubeError};
use crate::geom::known_polytopes;

/// Tolerance for capacity comparison with known values and cross-validation.
const CAPACITY_TOL: f64 = 1e-4;

// ── Symplecticity classification ──

#[test]
fn check_symplectic_classifies_all_known_polytopes() {
    // Document which known polytopes are symplectic. The tube algorithm
    // only applies to symplectic polytopes (no Lagrangian 2-faces).
    for kp in known_polytopes::all_known() {
        let result = check_symplectic(&kp.polytope);
        // Just verify no panics. The classification is polytope-dependent.
        eprintln!(
            "{}: {}",
            kp.name,
            if result.is_ok() {
                "symplectic"
            } else {
                "not symplectic"
            }
        );
    }
}

#[test]
fn check_symplectic_reports_lagrangian_facets() {
    // For non-symplectic polytopes, the error should identify the offending facet pair.
    let kp = known_polytopes::simplex();
    if let Err(TubeError::HasLagrangian2Face { facet_i, facet_j }) =
        check_symplectic(&kp.polytope)
    {
        let f = kp.polytope.facet_count();
        assert!(facet_i < f, "facet_i {} out of range", facet_i);
        assert!(facet_j < f, "facet_j {} out of range", facet_j);
        assert_ne!(facet_i, facet_j, "should be distinct facets");
    }
    // If simplex is symplectic, this test just passes.
}

// ── Error conditions ──

#[test]
fn tube_error_on_non_symplectic_polytope() {
    // For any polytope that check_symplectic rejects, tube_capacity should
    // also return an error.
    for kp in known_polytopes::all_known() {
        if check_symplectic(&kp.polytope).is_err() {
            let result = tube_capacity(&kp.polytope);
            assert!(
                result.is_err(),
                "{}: tube_capacity should return error for non-symplectic polytope",
                kp.name,
            );
            return; // One example suffices.
        }
    }
    // All known polytopes are symplectic — test is vacuously true.
}

// ── Capacity computation on symplectic polytopes ──

#[test]
fn tube_capacity_on_symplectic_polytopes() {
    // For every known polytope that passes check_symplectic, verify
    // tube_capacity returns a result (or at least doesn't error).
    for kp in known_polytopes::all_known() {
        if check_symplectic(&kp.polytope).is_err() {
            continue; // Skip non-symplectic.
        }

        let result = tube_capacity(&kp.polytope);
        assert!(
            result.is_ok(),
            "{}: tube_capacity should not error on symplectic polytope",
            kp.name,
        );

        if let Ok(Some(tr)) = result {
            // Capacity should be positive and finite.
            assert!(
                tr.capacity > 0.0 && tr.capacity.is_finite(),
                "{}: capacity should be positive and finite, got {}",
                kp.name,
                tr.capacity,
            );

            // Sequence should have at least 2 facets.
            assert!(
                tr.best_sequence.len() >= 2,
                "{}: orbit must visit at least 2 facets, got {:?}",
                kp.name,
                tr.best_sequence,
            );

            // All facet indices should be valid.
            let f = kp.polytope.facet_count();
            for &idx in &tr.best_sequence {
                assert!(
                    idx < f,
                    "{}: facet index {} out of range (F={})",
                    kp.name,
                    idx,
                    f,
                );
            }

            // Simple orbit: no repeated facets.
            let mut seen = std::collections::HashSet::new();
            for &idx in &tr.best_sequence {
                assert!(
                    seen.insert(idx),
                    "{}: repeated facet {} in sequence {:?}",
                    kp.name,
                    idx,
                    tr.best_sequence,
                );
            }

            // Should have explored at least one tube.
            assert!(
                tr.tubes_explored > 0,
                "{}: should explore at least one tube",
                kp.name,
            );

            eprintln!(
                "{}: capacity = {:.6} (known = {:.6}), explored {} tubes, pruned {}",
                kp.name, tr.capacity, kp.capacity, tr.tubes_explored, tr.tubes_pruned
            );
        }
    }
}

#[test]
fn tube_capacity_matches_known_values() {
    // For symplectic polytopes with known capacity values, verify agreement.
    for kp in known_polytopes::all_known() {
        if check_symplectic(&kp.polytope).is_err() {
            continue;
        }

        if let Ok(Some(tr)) = tube_capacity(&kp.polytope) {
            assert!(
                (tr.capacity - kp.capacity).abs() < CAPACITY_TOL,
                "{}: tube capacity {:.6} != known {:.6} (diff = {:.2e})",
                kp.name,
                tr.capacity,
                kp.capacity,
                (tr.capacity - kp.capacity).abs(),
            );
        }
    }
}

// ── Cross-validation with HK2017 ──

#[test]
#[ignore] // Depends on hk2017 module (wave 3)
fn tube_agrees_with_hk2017_on_all_symplectic() {
    use crate::algorithms::hk2017::ehz_capacity;

    for kp in known_polytopes::all_known() {
        if check_symplectic(&kp.polytope).is_err() {
            continue;
        }

        let hk_result = ehz_capacity(&kp.polytope);
        let tube_result = tube_capacity(&kp.polytope);

        if let (Some(hk), Ok(Some(tb))) = (hk_result, tube_result) {
            assert!(
                (tb.capacity - hk.result.capacity).abs() < CAPACITY_TOL,
                "{}: tube {:.6} != hk2017 {:.6}",
                kp.name,
                tb.capacity,
                hk.result.capacity,
            );
        }
    }
}

// ── Diagnostic tests ──

#[test]
fn tube_capacity_returns_none_or_some_consistently() {
    // Run tube_capacity twice on the same polytope — should give same result.
    for kp in known_polytopes::all_known() {
        if check_symplectic(&kp.polytope).is_err() {
            continue;
        }

        let r1 = tube_capacity(&kp.polytope);
        let r2 = tube_capacity(&kp.polytope);

        match (&r1, &r2) {
            (Ok(Some(a)), Ok(Some(b))) => {
                assert!(
                    (a.capacity - b.capacity).abs() < 1e-10,
                    "{}: inconsistent capacity: {:.6} vs {:.6}",
                    kp.name,
                    a.capacity,
                    b.capacity,
                );
            }
            (Ok(None), Ok(None)) => {} // Both found nothing — consistent.
            _ => panic!(
                "{}: inconsistent results: {:?} vs {:?}",
                kp.name,
                r1.as_ref().map(|r| r.as_ref().map(|t| t.capacity)),
                r2.as_ref().map(|r| r.as_ref().map(|t| t.capacity)),
            ),
        }
    }
}

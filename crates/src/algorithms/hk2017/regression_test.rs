//! Tests for hk2017: regression pins for past bugs (nullspace sign, eigen gap ratio).
//!
//! Proposition: Previously-broken inputs continue to produce correct results after
//! code changes. Each test pins a specific input and expected output that was
//! incorrect before a particular bug fix.
//! Reference: [lem:kkt], [lem:q-error-bound]
//!
//! Strategy: direct computation on hand-constructed polytopes that triggered past bugs.

use crate::algorithms::hk2017::{ehz_capacity, ehz_capacity_unpruned};
use crate::geom::lagrangian_product::lagrangian_product;
use crate::geom::polygon::{regular_polygon_2d, rotate_polygon_2d};
use crate::kkt::qp_assembly::build_augmented_system;

// ── KKT null space fix regressions ──
//
// These tests verify that the KKT solver correctly handles rank-deficient
// systems by searching the null space for beta > 0 solutions. Before the fix,
// the minimum-norm pseudoinverse solution often had beta <= 0 for degenerate
// polytopes (axis-aligned normals in symplectic subplanes).

/// Regression: (4,4) Lagrangian product at theta=0 (square x square, axis-aligned).
///
/// Before fix: cap=2.0 (correct). After fix: cap=2.0 (unchanged).
/// This is the hypercube [-1/sqrt(2), 1/sqrt(2)]^4 which already worked pre-fix.
/// Included to verify the fix does not break the working case.
#[test]
fn kkt_nullspace_square_square_zero() {
    let (qn, qh) = regular_polygon_2d(4, 1.0);
    let (pn, ph) = regular_polygon_2d(4, 1.0);
    let polytope = lagrangian_product(&qn, &qh, &pn, &ph).unwrap();

    let result =
        ehz_capacity_unpruned(&polytope).expect("(4,4) at theta=0 should have capacity");
    assert!(
        (result.result.capacity - 2.0).abs() < 1e-6,
        "(4,4) at theta=0: got {}, expected 2.0",
        result.result.capacity
    );
}

/// Regression: (4,4) at theta=0.125 deg — smallest angle in the polygon grid.
///
/// Before fix: cap=3.991 (WRONG, 2x too high due to 8-facet spurious orbit).
/// After fix: cap ~ 2.000 (continuous from theta=0).
#[test]
fn kkt_nullspace_square_square_near_zero() {
    let theta = 0.125_f64.to_radians();
    let (qn, qh) = regular_polygon_2d(4, 1.0);
    let (pn_base, ph_base) = regular_polygon_2d(4, 1.0);
    let (pn, ph) = rotate_polygon_2d(&pn_base, &ph_base, theta);
    let polytope = lagrangian_product(&qn, &qh, &pn, &ph).unwrap();

    let result =
        ehz_capacity_unpruned(&polytope).expect("(4,4) at theta=0.125 should have capacity");
    // Capacity should be continuous near theta=0 -> close to 2.0.
    assert!(
        (result.result.capacity - 2.0).abs() < 0.01,
        "(4,4) at theta=0.125: got {}, expected ~2.0 (was 3.991 before fix)",
        result.result.capacity
    );
}

/// Regression: (4,4) at theta=45 deg — billiard previously gave 2x wrong answer.
///
/// Before fix: HK2017=2.828, billiard=5.657.
/// After fix: all agree on cap = 2*sqrt(2) ~ 2.828.
#[test]
fn kkt_nullspace_square_square_45deg() {
    let theta = 45.0_f64.to_radians();
    let (qn, qh) = regular_polygon_2d(4, 1.0);
    let (pn_base, ph_base) = regular_polygon_2d(4, 1.0);
    let (pn, ph) = rotate_polygon_2d(&pn_base, &ph_base, theta);
    let polytope = lagrangian_product(&qn, &qh, &pn, &ph).unwrap();

    let result_hk =
        ehz_capacity_unpruned(&polytope).expect("(4,4) at theta=45: HK2017 should have capacity");
    let result_bil = crate::algorithms::billiard::billiard_capacity(&polytope)
        .expect("billiard should not error")
        .expect("billiard should find capacity");

    let sqrt2_times2 = 2.0 * std::f64::consts::SQRT_2;
    assert!(
        (result_hk.result.capacity - sqrt2_times2).abs() < 1e-6,
        "(4,4) at theta=45 HK2017: got {}, expected 2*sqrt(2) ~ {}",
        result_hk.result.capacity,
        sqrt2_times2
    );
    assert!(
        (result_bil.result.capacity - sqrt2_times2).abs() < 1e-6,
        "(4,4) at theta=45 billiard: got {} (was 5.657 before fix), expected 2*sqrt(2) ~ {}",
        result_bil.result.capacity,
        sqrt2_times2
    );
}

/// Regression: (3,4) at theta=0 — previously returned None for all algorithms.
///
/// Before fix: None (all three algorithms). No valid orbit found.
/// After fix: cap ~ 2.121 via 5-facet orbit. All three agree.
///
/// Note: The expected capacity for this specific polytope (triangle circumradius=1,
/// square circumradius=1) is 3*sqrt(2)/2 ~ 2.121, NOT 1.5. The value 1.5 is from
/// `lagrangian_triangle_square()` which uses different dimensions.
#[test]
fn kkt_nullspace_triangle_square_zero() {
    let (qn, qh) = regular_polygon_2d(3, 1.0);
    let (pn, ph) = regular_polygon_2d(4, 1.0);
    let polytope = lagrangian_product(&qn, &qh, &pn, &ph).unwrap();

    let result = ehz_capacity_unpruned(&polytope)
        .expect("(3,4) at theta=0 should now return Some after null space fix");

    let expected = 3.0 * std::f64::consts::SQRT_2 / 2.0; // 3*sqrt(2)/2 ~ 2.121
    assert!(
        (result.result.capacity - expected).abs() < 1e-6,
        "(3,4) at theta=0: got {}, expected 3*sqrt(2)/2 ~ {} (was None before fix)",
        result.result.capacity,
        expected
    );
}

// ── Eigenvalue condition number threshold regression tests ──
//
// EIGEN_CONDITION_TAU=1e-3 was chosen empirically from the (4,4) degenerate case.
// These tests pin the eigenvalue spectrum so that threshold changes can be
// validated against the cases that motivated the threshold.

/// Verify eigenvalue spectrum of the (4,4) theta=0 degenerate KKT system.
///
/// The optimal orbit permutation [0,4,2,6] (alternating q/p facets) has a gap
/// in the sorted |lambda_i| spectrum. The system must be rank-deficient for the
/// null space search to activate.
///
/// Regression test for EIGEN_CONDITION_TAU (see doc comment on the constant).
#[test]
fn eigen_gap_ratio_44_degenerate() {
    let (qn, qh) = regular_polygon_2d(4, 1.0);
    let (pn, ph) = regular_polygon_2d(4, 1.0);
    let polytope = lagrangian_product(&qn, &qh, &pn, &ph).unwrap();

    // The optimal orbit at theta=0 uses facets [0,4,2,6] (alternating q/p).
    let perm = vec![0, 4, 2, 6];
    let (kkt, _rhs) = build_augmented_system(&polytope, &perm);
    let eigen = kkt.symmetric_eigen();
    let size = perm.len() + 5; // 9

    // Collect |lambda_i| and sort descending.
    let mut abs_eigenvalues: Vec<f64> =
        eigen.eigenvalues.iter().map(|&ev| ev.abs()).collect();
    abs_eigenvalues.sort_by(|a, b| b.partial_cmp(a).unwrap());

    // Find gap ratio: walk from smallest |lambda_i| upward.
    let floor = 1e-15;
    let smallest_nonzero = (0..size).rev().find(|&i| abs_eigenvalues[i] > floor);
    if let Some(idx) = smallest_nonzero {
        if idx > 0 {
            let ratio = abs_eigenvalues[idx - 1] / abs_eigenvalues[idx];
            // If large gap exists, it must stay well above EIGEN_CONDITION_TAU.
            if ratio > 50.0 {
                assert!(
                    ratio > 200.0,
                    "(4,4) theta=0 gap ratio should stay well above 1e-3 threshold, \
                     got {:.1} (|lambda[{}]|={:.3e}, |lambda[{}]|={:.3e})",
                    ratio,
                    idx - 1,
                    abs_eigenvalues[idx - 1],
                    idx,
                    abs_eigenvalues[idx]
                );
            }
        }
    }

    // The KKT system must be rank-deficient (axis-aligned normals create dependence).
    let numerical_rank = abs_eigenvalues.iter().filter(|&&ev| ev > 1e-6).count();
    assert!(
        numerical_rank < size,
        "(4,4) theta=0 should be rank-deficient: rank={}, size={}",
        numerical_rank,
        size
    );
}

/// Verify eigenvalue gap ratio for the (4,4) theta=43 deg case.
///
/// The KKT system for perm [1,0,6,3,2,4] has a gap ratio ~594 — the case from
/// commit dd87a8a that motivated EIGEN_CONDITION_TAU=1e-3. The gap ratio must
/// stay well above the threshold.
#[test]
fn eigen_gap_ratio_44_theta43() {
    let theta = 43.0_f64.to_radians();
    let (qn, qh) = regular_polygon_2d(4, 1.0);
    let (pn_base, ph_base) = regular_polygon_2d(4, 1.0);
    let (pn, ph) = rotate_polygon_2d(&pn_base, &ph_base, theta);
    let polytope = lagrangian_product(&qn, &qh, &pn, &ph).unwrap();

    let perm = vec![1, 0, 6, 3, 2, 4];
    let m = perm.len();
    let size = m + 5; // 11

    let (kkt, _rhs) = build_augmented_system(&polytope, &perm);
    let eigen = kkt.symmetric_eigen();

    let mut abs_eigenvalues: Vec<f64> =
        eigen.eigenvalues.iter().map(|&ev| ev.abs()).collect();
    abs_eigenvalues.sort_by(|a, b| b.partial_cmp(a).unwrap());

    // Find the largest gap ratio in the spectrum.
    let floor = 1e-15;
    let mut max_gap_ratio = 0.0f64;
    let mut gap_idx = 0;
    for i in (1..size).rev() {
        if abs_eigenvalues[i] < floor {
            continue;
        }
        let ratio = abs_eigenvalues[i - 1] / abs_eigenvalues[i];
        if ratio > max_gap_ratio {
            max_gap_ratio = ratio;
            gap_idx = i;
        }
    }

    // Gap ratio must be well above EIGEN_CONDITION_TAU=1e-3. Original: ~594.
    assert!(
        max_gap_ratio > 300.0,
        "(4,4) theta=43 gap ratio should be ~594 (well above 1e-3 threshold), \
         got {:.1} at |lambda[{}]|={:.3e}/|lambda[{}]|={:.3e}",
        max_gap_ratio,
        gap_idx - 1,
        abs_eigenvalues[gap_idx - 1],
        gap_idx,
        abs_eigenvalues[gap_idx]
    );
}

// ── HKO counterexample regression ──

/// Verify HKO pentagon capacity and sys > 1 property (Annals counterexample).
///
/// Computes capacity on the Haim-Kislev-Ostrover 10-facet pentagon and verifies
/// it is a counterexample to Viterbo's conjecture (sys > 1).
///
/// Why #[ignore]: F=10 -> ~37s debug, ~0.5s release. Important regression test
/// for the thesis counterexample.
/// Run: `cargo test --release pentagon_capacity -- --ignored`
#[test]
#[ignore] // ~37s debug, ~0.5s release
fn pentagon_capacity() {
    use crate::geom::known_polytopes;
    use crate::geom::volume::volume;

    let kp = known_polytopes::hko_pentagon();
    let result = ehz_capacity(&kp.polytope).expect("pentagon capacity");

    assert!(
        (result.result.capacity - kp.capacity).abs() < 1e-6,
        "pentagon: got {}, expected {}",
        result.result.capacity,
        kp.capacity
    );

    // Verify sys > 1 (counterexample property).
    let vol = volume(&kp.polytope).expect("volume computation failed");
    let sys = result.result.capacity * result.result.capacity / (2.0 * vol);
    eprintln!(
        "Pentagon: capacity={:.6}, volume={:.6}, sys={:.6}",
        result.result.capacity, vol, sys
    );
    assert!(sys > 1.0, "pentagon should have sys > 1, got {}", sys);
}

/// Compute capacity of the 4D crosspolytope (16 facets, non-simple).
///
/// Why #[ignore]: F=16, estimated ~4 hours pruned (A3) in release mode.
/// Run: `cargo test --release crosspolytope_capacity -- --ignored --nocapture`
#[test]
#[ignore]
fn crosspolytope_capacity() {
    use crate::geom::known_polytopes;

    let kp = known_polytopes::crosspolytope();
    let result = ehz_capacity(&kp.polytope).expect("crosspolytope capacity");

    assert!(
        result.result.capacity > 0.0,
        "crosspolytope capacity positive"
    );
    eprintln!(
        "Crosspolytope (16 facets): capacity={:.6}",
        result.result.capacity
    );
    eprintln!("  Iterations: {}", result.result.iterations);
}

//! Unit tests for orbit-search result helpers and resolution seams.
//!
//! These tests verify:
//! - exact fallback upgrades a known admissible winner,
//! - boundsafe guarantee mode resolves indeterminate minima to exact orbits.
//!
//! Behavior is preserved while moving inline tests out of `orbit_search.rs` to
//! keep the production module focused on implementation.

use super::*;
use crate::ehz_capacity_pruned;
use crate::geom::known_polytopes;
use crate::geom::lagrangian_product::lagrangian_product;
use crate::geom::polygon::regular_polygon_2d;
use crate::geom::polytope::Polytope4D;
use crate::geom::rational_arithmetic::rat;
use crate::kkt::rational_solver::solve_kkt_exact;

fn rational_scaled_cube_half() -> Polytope4D {
    let z = rat(0);
    let two = rat(2);
    let dual_vertices = vec![
        [z.clone(), two.clone(), z.clone(), z.clone()],
        [-two.clone(), z.clone(), z.clone(), z.clone()],
        [z.clone(), -two.clone(), z.clone(), z.clone()],
        [two.clone(), z.clone(), z.clone(), z.clone()],
        [z.clone(), z.clone(), z.clone(), two.clone()],
        [z.clone(), z.clone(), -two.clone(), z.clone()],
        [z.clone(), z.clone(), z.clone(), -two.clone()],
        [z.clone(), z.clone(), two, z.clone()],
    ];
    Polytope4D::new(dual_vertices).expect("exact rational scaled cube")
}

#[test]
fn exact_resolution_upgrades_known_winner() {
    let kp = known_polytopes::simplex();
    let result = ehz_capacity_pruned(&kp.polytope).expect("ehz_capacity should succeed");
    let orbit = solve_orbit_sigma(
        &kp.polytope,
        result.best_sigma(),
        OrbitSolveBackend::SaddlePoint,
    )
    .expect("saddle-point solve should succeed");

    let exact = resolve_orbit_exact(&kp.polytope, &orbit)
        .expect("exact fallback should certify the known winner");

    assert_eq!(exact.admissibility, OrbitAdmissibility::AdmissibleExact);
    assert_eq!(exact.sigma, orbit.sigma);
    assert_eq!(exact.q_error_bound, 0.0);
    assert_eq!(exact.action_lower, exact.action_upper);
}

#[test]
fn boundsafe_resolves_indeterminate_argmin() {
    let kp = known_polytopes::simplex();
    let result = ehz_capacity_pruned(&kp.polytope).expect("ehz_capacity should succeed");
    let mut orbit = solve_orbit_sigma(
        &kp.polytope,
        result.best_sigma(),
        OrbitSolveBackend::SaddlePoint,
    )
    .expect("saddle-point solve should succeed");
    orbit.admissibility = OrbitAdmissibility::IndeterminateF64;

    let mut orbits = vec![orbit];
    resolve_orbits_for_guarantee(&kp.polytope, &mut orbits, OrbitGuaranteeMode::BoundSafe)
        .expect("boundsafe resolution should succeed");

    assert_eq!(orbits.len(), 1);
    assert_eq!(orbits[0].admissibility, OrbitAdmissibility::AdmissibleExact);
    assert_eq!(orbits[0].action_lower, orbits[0].action_upper);
}

#[test]
fn minimasafe_does_not_accept_spurious_square_product_minimum() {
    // Keep this test non-ignored. It pins the former failure mode where
    // `MinimaSafe` trusted a thresholded exact-rational fallback and returned a
    // capacity below the cube squeeze bound.
    let (qn, qh) = regular_polygon_2d(4, 1.0);
    let (pn, ph) = regular_polygon_2d(4, 1.0);
    let polytope = lagrangian_product(&qn, &qh, &pn, &ph).expect("square product");

    let (orbits, iterations) =
        solve_sigma_stream(&polytope, OrbitSolveBackend::SaddlePoint, |visit| {
            crate::algorithms::hk2017::for_each_sigma_unpruned(&polytope, visit)
        })
        .expect("square product sigma stream should solve");

    let result = aggregate_orbits(
        &polytope,
        orbits,
        iterations,
        0.0,
        OrbitGuaranteeMode::MinimaSafe,
    )
    .expect("MinimaSafe aggregation should succeed");

    let best = result.best_orbit();
    assert!(
        (result.capacity() - 2.0).abs() < 1e-6,
        "MinimaSafe accepted a false square-product minimum: got {}, expected 2.0; best sigma={:?}, admissibility={:?}, beta_margin={}",
        result.capacity(),
        best.sigma,
        best.admissibility,
        best.beta_margin
    );
}

#[test]
fn minimasafe_accepts_exact_rational_scaled_cube() {
    let polytope = rational_scaled_cube_half();
    assert!(
        solve_kkt_exact(polytope.dual_vertices(), &[0, 3, 4, 2, 6]).is_none(),
        "the square-product bad sigma must be boundary/inadmissible on the exact rational cube"
    );

    let (orbits, iterations) =
        solve_sigma_stream(&polytope, OrbitSolveBackend::SaddlePoint, |visit| {
            crate::algorithms::hk2017::for_each_sigma_unpruned(&polytope, visit)
        })
        .expect("exact rational cube sigma stream should solve");

    let result = aggregate_orbits(
        &polytope,
        orbits,
        iterations,
        0.0,
        OrbitGuaranteeMode::MinimaSafe,
    )
    .expect("MinimaSafe aggregation should succeed on exact rational cube");

    assert!(
        (result.capacity() - 1.0).abs() < 1e-10,
        "exact rational cube [-1/2,1/2]^4 should have capacity 1.0, got {}",
        result.capacity()
    );
}

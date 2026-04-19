//! Unit tests for orbit-search result helpers and resolution seams.
//!
//! These tests verify:
//! - exact fallback upgrades a known admissible winner,
//! - boundsafe guarantee mode resolves indeterminate minima to exact orbits.
//! Behavior is preserved while moving inline tests out of `orbit_search.rs` to
//! keep the production module focused on implementation.

use super::*;
use crate::ehz_capacity_pruned;
use crate::geom::known_polytopes;

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

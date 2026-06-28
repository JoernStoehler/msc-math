use super::pruning_roundoff_fixture;
use crate::fallback_route::aggregate_orbits_with_local_exact_fallback;
use crate::{
    exact_binary64_dual_vertex_arrays, exact_binary64_transition_matrix_assuming_origin_interior,
};
use symplectic::algorithms::hk2017::SimpleDirectedCyclesCanonical;
use symplectic::{
    solve_orbit_sigma_saddle_point, OrbitAdmissibility, OrbitGuaranteeMode, OrbitKktData,
    OrbitSearchResult,
};

/// Demonstrates that fallback guarantee modes are scope choices, not aliases.
///
/// Each mode solves a different retained-candidate problem:
/// - `BoundSafe` makes the reported minimum-action endpoints exact-safe.
/// - `MinimaSafe` also resolves retained candidates overlapping the minimum
///   window.
/// - `AllSafe` resolves every retained indeterminate candidate.
///
/// None of these modes discovers sigmas that were not retained; that global
/// limitation is demonstrated separately by `retained_candidate_fallback_limit`.
#[test]
fn fallback_guarantee_modes_resolve_different_retained_subsets() {
    let (exact_vertices, retained) = retained_scope_demo_candidates();

    let boundsafe = run_mode(
        &exact_vertices,
        retained.clone(),
        OrbitGuaranteeMode::BoundSafe,
    );
    assert_eq!(admissible_exact_count(&boundsafe), 1);
    assert_eq!(indeterminate_count(&boundsafe), 2);

    let minimasafe = run_mode(
        &exact_vertices,
        retained.clone(),
        OrbitGuaranteeMode::MinimaSafe,
    );
    assert_eq!(admissible_exact_count(&minimasafe), 2);
    assert_eq!(indeterminate_count(&minimasafe), 1);

    let allsafe = run_mode(&exact_vertices, retained, OrbitGuaranteeMode::AllSafe);
    assert_eq!(admissible_exact_count(&allsafe), 3);
    assert_eq!(indeterminate_count(&allsafe), 0);
}

fn run_mode(
    exact_vertices: &[[num_rational::BigRational; 4]],
    retained: Vec<OrbitKktData>,
    mode: OrbitGuaranteeMode,
) -> OrbitSearchResult {
    aggregate_orbits_with_local_exact_fallback(exact_vertices, retained, 3, 100.0, mode)
        .expect("scope-demo retained candidates should certify")
}

fn admissible_exact_count(result: &OrbitSearchResult) -> usize {
    result
        .orbits
        .iter()
        .filter(|orbit| orbit.admissibility == OrbitAdmissibility::AdmissibleExact)
        .count()
}

fn indeterminate_count(result: &OrbitSearchResult) -> usize {
    result
        .orbits
        .iter()
        .filter(|orbit| orbit.admissibility == OrbitAdmissibility::IndeterminateF64)
        .count()
}

fn retained_scope_demo_candidates() -> (Vec<[num_rational::BigRational; 4]>, Vec<OrbitKktData>) {
    let dual_vertices = pruning_roundoff_fixture();
    let exact_vertices = exact_binary64_dual_vertex_arrays(&dual_vertices);
    let exact_transition =
        exact_binary64_transition_matrix_assuming_origin_interior(&exact_vertices);

    let mut certifiable = Vec::new();
    for sigma in SimpleDirectedCyclesCanonical::new(&exact_transition) {
        let Ok(orbit) = solve_orbit_sigma_saddle_point(&dual_vertices, &sigma) else {
            continue;
        };
        let Some(exact_action) = certify_single_action(&exact_vertices, &orbit) else {
            continue;
        };
        certifiable.push((orbit, exact_action));
    }
    certifiable.sort_by(|(left, left_action), (right, right_action)| {
        left_action
            .total_cmp(right_action)
            .then_with(|| left.sigma.cmp(&right.sigma))
    });

    let (endpoint, endpoint_action) = certifiable
        .first()
        .cloned()
        .expect("scope fixture should have an exact-certifiable endpoint candidate");
    let (overlap, _) = certifiable
        .iter()
        .find(|(_, action)| *action > endpoint_action + 1e-9)
        .cloned()
        .expect("scope fixture should have a second exact-certifiable candidate");
    let (outside, outside_action) = certifiable
        .iter()
        .find(|(_, action)| *action > endpoint_action + 1.0)
        .cloned()
        .expect("scope fixture should have an outside exact-certifiable candidate");

    let mut endpoint = mark_indeterminate(endpoint);
    endpoint.action = endpoint_action;
    endpoint.action_lower = endpoint_action;
    endpoint.action_upper = endpoint_action;

    let mut overlap = mark_indeterminate(overlap);
    overlap.action_lower = endpoint_action;
    overlap.action_upper = endpoint_action + 0.1;

    let mut outside = mark_indeterminate(outside);
    outside.action_lower = outside_action;
    outside.action_upper = outside_action;

    (exact_vertices, vec![endpoint, overlap, outside])
}

fn certify_single_action(
    exact_vertices: &[[num_rational::BigRational; 4]],
    orbit: &OrbitKktData,
) -> Option<f64> {
    aggregate_orbits_with_local_exact_fallback(
        exact_vertices,
        vec![mark_indeterminate(orbit.clone())],
        1,
        0.0,
        OrbitGuaranteeMode::AllSafe,
    )
    .ok()
    .and_then(|result| result.orbits.first().map(|orbit| orbit.action))
}

fn mark_indeterminate(mut orbit: OrbitKktData) -> OrbitKktData {
    orbit.admissibility = OrbitAdmissibility::IndeterminateF64;
    orbit
}

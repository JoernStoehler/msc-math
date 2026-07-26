//! Unit tests for orbit-search result helpers and resolution seams.
//!
//! These tests verify:
//! - exact fallback upgrades a known admissible winner,
//! - boundsafe guarantee mode resolves indeterminate minima to exact orbits.
//!
//! Behavior is preserved while moving inline tests out of `orbit_search.rs` to
//! keep the production module focused on implementation.

use super::*;
use crate::algorithms::test_helpers::{
    certified_pruned_capacity_for_fixture, pruned_capacity_for_fixture,
};
use crate::geom::known_polytopes;
use crate::geom::lagrangian_product::lagrangian_product;
use crate::geom::polygon::regular_polygon_2d;
use crate::geom::rational_arithmetic::{f64_to_rational, frac, rat, rational_to_f64};
use crate::kkt::rational_solver::{solve_kkt_exact, ExactKktResult};
use num_traits::Zero;

fn exact_dual_vertex_arrays(
    dual_vertices: &[nalgebra::Vector4<f64>],
) -> Vec<[num_rational::BigRational; 4]> {
    dual_vertices
        .iter()
        .map(|a| {
            [
                f64_to_rational(a[0]),
                f64_to_rational(a[1]),
                f64_to_rational(a[2]),
                f64_to_rational(a[3]),
            ]
        })
        .collect()
}

fn rational_scaled_cube_half() -> (
    Vec<[num_rational::BigRational; 4]>,
    Vec<nalgebra::Vector4<f64>>,
) {
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
    let dual_vertices_f64 = dual_vertices
        .iter()
        .map(|a| {
            nalgebra::Vector4::new(
                rational_to_f64(&a[0]),
                rational_to_f64(&a[1]),
                rational_to_f64(&a[2]),
                rational_to_f64(&a[3]),
            )
        })
        .collect();
    (dual_vertices, dual_vertices_f64)
}

#[test]
fn exact_resolution_upgrades_known_winner() {
    let kp = known_polytopes::simplex();
    let result = pruned_capacity_for_fixture(kp).expect("ehz_capacity should succeed");
    let dual_vertices = &kp.dual_vertices_f64;
    let orbit = solve_orbit_sigma_saddle_point(dual_vertices, result.best_sigma())
        .expect("saddle-point solve should succeed");

    let exact = resolve_orbit_exact_with_dual_vertices_exact(&kp.dual_vertices, &orbit)
        .expect("exact fallback should certify the known winner");

    assert_eq!(exact.admissibility, OrbitAdmissibility::AdmissibleExact);
    assert_eq!(exact.sigma, orbit.sigma);
    assert_eq!(exact.q_error_bound, 0.0);
    assert_eq!(exact.action_lower, exact.action_upper);
}

#[test]
fn exact_action_is_rounded_after_exact_reciprocal() {
    let q = frac(11, 6);
    let action = rational_to_f64(&exact_action_from_q(&q));
    let round_q_then_divide = 0.5 / rational_to_f64(&q);

    assert_eq!(action, rational_to_f64(&frac(3, 11)));
    assert_ne!(
        action.to_bits(),
        round_q_then_divide.to_bits(),
        "q = 11/6 distinguishes exact reciprocal conversion from rounding q first"
    );
}

#[test]
fn exact_positive_action_tests_rational_sign_before_conversion() {
    let tiny_positive_q =
        num_rational::BigRational::new(1.into(), num_bigint::BigInt::from(1u8) << 1075);
    assert_eq!(
        rational_to_f64(&tiny_positive_q),
        0.0,
        "control: this positive rational underflows when converted to f64"
    );

    let action = exact_positive_action_from_q(&tiny_positive_q)
        .expect("positive exact Q must remain admissible despite f64 underflow");
    assert_eq!(action * (tiny_positive_q.clone() + tiny_positive_q), rat(1));
    assert!(exact_positive_action_from_q(&rat(0)).is_none());
    assert!(exact_positive_action_from_q(&rat(-1)).is_none());
}

#[test]
fn sigma_stream_info_tracing_matches_plain_result() {
    let kp = known_polytopes::simplex();
    let dual_vertices = &kp.dual_vertices_f64;

    let plain = solve_sigma_stream_with_dual_vertices(dual_vertices, |visit| {
        let facet_count = dual_vertices.len();
        crate::algorithms::hk2017::for_each_sigma_unpruned_facet_count(facet_count, visit)
    })
    .expect("plain sigma stream should solve");

    let subscriber = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_writer(std::io::sink)
        .finish();
    let traced = tracing::subscriber::with_default(subscriber, || {
        solve_sigma_stream_with_dual_vertices(dual_vertices, |visit| {
            let facet_count = dual_vertices.len();
            crate::algorithms::hk2017::for_each_sigma_unpruned_facet_count(facet_count, visit)
        })
    })
    .expect("traced sigma stream should solve");

    assert_eq!(traced, plain);
}

#[test]
fn boundsafe_resolves_indeterminate_argmin() {
    let kp = known_polytopes::simplex();
    let result = pruned_capacity_for_fixture(kp).expect("ehz_capacity should succeed");
    let dual_vertices = &kp.dual_vertices_f64;
    let mut orbit = solve_orbit_sigma_saddle_point(dual_vertices, result.best_sigma())
        .expect("saddle-point solve should succeed");
    orbit.admissibility = OrbitAdmissibility::IndeterminateF64;

    let mut orbits = vec![orbit];
    resolve_orbits_for_guarantee_with_dual_vertices_exact(
        &kp.dual_vertices,
        &mut orbits,
        OrbitGuaranteeMode::BoundSafe,
    )
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
    let dual_vertices = lagrangian_product(&qn, &qh, &pn, &ph).expect("square product");
    let dual_vertices_exact = exact_dual_vertex_arrays(&dual_vertices);

    let (orbits, iterations) = solve_sigma_stream_with_dual_vertices(&dual_vertices, |visit| {
        let facet_count = dual_vertices.len();
        crate::algorithms::hk2017::for_each_sigma_unpruned_facet_count(facet_count, visit)
    })
    .expect("square product sigma stream should solve");

    let result = aggregate_orbits_with_dual_vertices_exact(
        &dual_vertices_exact,
        orbits,
        iterations,
        0.0,
        OrbitGuaranteeMode::MinimaSafe,
    )
    .expect("MinimaSafe aggregation should succeed");

    let best = result.best_orbit();
    assert!(
        (result.min_action - 2.0).abs() < 1e-6,
        "MinimaSafe accepted a false square-product minimum: got {}, expected 2.0; best sigma={:?}, admissibility={:?}, beta_margin={}",
        result.min_action,
        best.sigma,
        best.admissibility,
        best.beta_margin
    );
}

#[test]
fn minimasafe_accepts_exact_rational_scaled_cube() {
    let (dual_vertices_exact, dual_vertices) = rational_scaled_cube_half();
    assert!(
        solve_kkt_exact(&dual_vertices_exact, &[0, 3, 4, 2, 6]).is_none(),
        "the square-product bad sigma must be boundary/inadmissible on the exact rational cube"
    );

    let (orbits, iterations) = solve_sigma_stream_with_dual_vertices(&dual_vertices, |visit| {
        let facet_count = dual_vertices.len();
        crate::algorithms::hk2017::for_each_sigma_unpruned_facet_count(facet_count, visit)
    })
    .expect("exact rational cube sigma stream should solve");

    let result = aggregate_orbits_with_dual_vertices_exact(
        &dual_vertices_exact,
        orbits,
        iterations,
        0.0,
        OrbitGuaranteeMode::MinimaSafe,
    )
    .expect("MinimaSafe aggregation should succeed on exact rational cube");

    assert!(
        (result.min_action - 1.0).abs() < 1e-10,
        "exact rational cube [-1/2,1/2]^4 should have capacity 1.0, got {}",
        result.min_action
    );
}

#[test]
fn exact_fallback_invariant_rejects_bad_equalities() {
    let (dual_vertices_exact, _) = rational_scaled_cube_half();
    let sigma = [0, 3, 4, 2, 6];
    let beta = vec![frac(1, 5); sigma.len()];
    let exact = ExactKktResult {
        beta,
        q_exact: rat(1),
        q_exact_f64: 1.0,
        mu: std::array::from_fn(|_| rat(0)),
        xi: rat(0),
    };

    assert!(
        !exact_kkt_result_satisfies_constraints_with_dual_vertices_exact(
            &dual_vertices_exact,
            &sigma,
            &exact
        ),
        "positive beta alone must not count as an exact fallback certificate"
    );
}

#[test]
fn certified_pruned_wrapper_returns_exact_simplex_minimizers() {
    let kp = known_polytopes::simplex();
    let result = certified_pruned_capacity_for_fixture(
        kp,
        num_rational::BigRational::zero(),
        CertifiedOrbitSetMode::MinimizersOnly,
    )
    .expect("certified simplex capacity");

    assert_eq!(result.capacity_exact, frac(1, 4));
    assert_eq!(result.action_gap_exact, num_rational::BigRational::zero());
    assert!(!result.minimizers.is_empty());
    assert_eq!(result.orbits, result.minimizers);
    assert!(result
        .minimizers
        .iter()
        .all(|orbit| orbit.action_exact == result.capacity_exact));
    assert!(result.exact_resolutions > 0);
}

#[test]
fn certified_gap_window_returns_only_exact_orbits_inside_gap() {
    let kp = known_polytopes::simplex();
    let gap = frac(1, 4);
    let dual_vertices = &kp.dual_vertices_f64;
    let dual_vertices_exact = &kp.dual_vertices;
    let (orbits, iterations) = solve_sigma_stream_with_dual_vertices(dual_vertices, |visit| {
            let transition_is_allowed = crate::algorithms::facet_adjacency::build_transition_matrix_from_facet_intersections_and_omega(
                &kp.facet_intersection_is_nonempty,
                &kp.omega_signs,
            );
            crate::algorithms::hk2017::for_each_sigma_pruned_by_transition(
                &transition_is_allowed,
                visit,
            )
    })
        .expect("simplex sigma stream should solve");

    let result = aggregate_certified_orbits_with_dual_vertices_exact(
        dual_vertices_exact,
        orbits,
        iterations,
        gap.clone(),
        CertifiedOrbitSetMode::GapWindow,
    )
    .expect("certified simplex gap window");

    let cutoff = result.capacity_exact.clone() + gap;
    assert!(!result.orbits.is_empty());
    assert!(result
        .orbits
        .iter()
        .all(|orbit| orbit.action_exact <= cutoff));
    assert!(result
        .minimizers
        .iter()
        .all(|orbit| orbit.action_exact == result.capacity_exact));
}

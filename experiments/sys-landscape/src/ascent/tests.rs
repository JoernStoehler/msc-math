use super::compute::{
    admissible_active_orbits, compute_capacity_result, compute_sys, compute_sys_from_capacity,
    maximin_subgradient_direction, AscentMode,
};
use crate::SysLandscapePolytopeCache;
use nalgebra::Vector4;
use symplectic::classify_facets_from_dual_vertices;
use symplectic::derivatives::clarke_directional_derivative_a;
use symplectic::geom::polygon::regular_polygon_2d;
use symplectic::{OrbitAdmissibility, OrbitKktData, OrbitSearchResult};

fn triangle_product_cache() -> SysLandscapePolytopeCache {
    let (qn, qh) = regular_polygon_2d(3, 1.0);
    let (pn, ph) = regular_polygon_2d(3, 1.0);
    SysLandscapePolytopeCache::from_lagrangian_product(&qn, &qh, &pn, &ph)
        .expect("triangle product should construct")
}

#[test]
fn compute_sys_from_capacity_matches_compute_sys() {
    let polytope = triangle_product_cache();
    let capacity =
        compute_capacity_result(&polytope).expect("known polytope should have a capacity result");
    let cached = compute_sys_from_capacity(&polytope, &capacity)
        .expect("cached capacity result should produce sys");
    let direct = compute_sys(&polytope).expect("known polytope should produce sys");

    assert!(
        (cached - direct).abs() < 1e-12,
        "cached={cached}, direct={direct}"
    );
}

#[test]
fn admissible_active_orbits_ignore_indeterminate_candidates() {
    let admissible = OrbitKktData {
        sigma: vec![0, 1],
        beta: vec![0.5, 0.5],
        beta_margin: 0.5,
        action: 1.0,
        action_lower: 1.0,
        action_upper: 1.0,
        q: 0.5,
        q_error_bound: 0.0,
        mu: Some([0.0; 4]),
        xi: Some(1.0),
        admissibility: OrbitAdmissibility::AdmissibleF64,
    };
    let indeterminate = OrbitKktData {
        sigma: vec![0, 2],
        beta: vec![1e-16, 1.0],
        beta_margin: 1e-16,
        action: 0.9,
        action_lower: 0.8,
        action_upper: 1.1,
        q: 0.55,
        q_error_bound: 0.1,
        mu: Some([0.0; 4]),
        xi: Some(1.0),
        admissibility: OrbitAdmissibility::IndeterminateF64,
    };
    let result = OrbitSearchResult {
        orbits: vec![indeterminate, admissible.clone()],
        min_action: admissible.action,
        min_action_lower: 0.8,
        min_action_upper: 1.0,
        iterations: 2,
    };

    let active = admissible_active_orbits(&result);
    assert_eq!(active.len(), 1);
    assert_eq!(active[0].sigma, admissible.sigma);
}

#[test]
fn maximin_direction_finds_improving_switch_direction() {
    let subdiff = vec![
        vec![Vector4::new(1.0, 0.0, 0.0, 0.0)],
        vec![Vector4::new(0.0, 1.0, 0.0, 0.0)],
    ];

    let direction = maximin_subgradient_direction(&subdiff, 1, AscentMode::General)
        .expect("switching pair should admit a positive maximin direction");
    let predicted = clarke_directional_derivative_a(&subdiff, &direction)
        .expect("nonempty subdifferential should evaluate");

    assert!(
        predicted > 0.99,
        "predicted directional derivative = {predicted}"
    );
    assert!(direction[0][0] > 0.99, "direction = {:?}", direction[0]);
    assert!(direction[0][1] > 0.99, "direction = {:?}", direction[0]);
}

#[test]
fn maximin_direction_respects_lp_coordinate_bounds() {
    let polytope = triangle_product_cache();
    let classification = classify_facets_from_dual_vertices(&polytope.dual_vertices_f64)
        .expect("triangle product should classify as a Lagrangian product");
    let facet_count = polytope.facet_count();
    let q_idx = classification.q_indices[0];
    let p_idx = classification.p_indices[0];

    let mut g1 = vec![Vector4::zeros(); facet_count];
    g1[q_idx] = Vector4::new(1.0, 2.0, 9.0, 11.0);
    g1[p_idx] = Vector4::new(8.0, 6.0, 1.0, 2.0);

    let mut g2 = vec![Vector4::zeros(); facet_count];
    g2[q_idx] = Vector4::new(2.0, 1.0, 7.0, 5.0);
    g2[p_idx] = Vector4::new(4.0, 3.0, 2.0, 1.0);

    let subdiff = vec![g1, g2];
    let direction = maximin_subgradient_direction(
        &subdiff,
        facet_count,
        AscentMode::LagrangianProduct {
            classification: &classification,
        },
    )
    .expect("LP-bounded switching pair should admit a positive direction");

    assert!(
        direction[q_idx][2].abs() < 1e-9,
        "direction = {:?}",
        direction[q_idx]
    );
    assert!(
        direction[q_idx][3].abs() < 1e-9,
        "direction = {:?}",
        direction[q_idx]
    );
    assert!(
        direction[p_idx][0].abs() < 1e-9,
        "direction = {:?}",
        direction[p_idx]
    );
    assert!(
        direction[p_idx][1].abs() < 1e-9,
        "direction = {:?}",
        direction[p_idx]
    );
}

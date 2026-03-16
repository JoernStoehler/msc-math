//! Tests for lagrangian_product: facet count, Q/P classification, volume.
//!
//! Proposition: lagrangian_product(P, Q) produces a polytope with
//!   F = |P| + |Q| facets, and vol_4 = area(P) * area(Q).
//! Reference: [def:lagrangian-product]
//!
//! Strategy: fixture-based (triangle x triangle, pentagon x pentagon, etc.)

use crate::geom::lagrangian_product::lagrangian_product;
use crate::geom::polygon::{polygon_area, regular_polygon_2d, rotate_polygon_2d};
use crate::geom::polytope::ConstructionError;
use crate::geom::volume::volume;
use nalgebra::Vector2;
use std::f64::consts::PI;

#[test]
fn triangle_x_triangle_has_6_facets() {
    let (qn, qh) = regular_polygon_2d(3, 1.0);
    let (pn, ph) = regular_polygon_2d(3, 1.0);
    let polytope = lagrangian_product(&qn, &qh, &pn, &ph).unwrap();
    assert_eq!(polytope.facet_count(), 6);
}

#[test]
fn pentagon_x_pentagon_has_10_facets() {
    let (qn, qh) = regular_polygon_2d(5, 1.0);
    let (pn, ph) = regular_polygon_2d(5, 1.0);
    let polytope = lagrangian_product(&qn, &qh, &pn, &ph).unwrap();
    assert_eq!(polytope.facet_count(), 10);
}

#[test]
fn triangle_x_square_has_7_facets() {
    let (qn, qh) = regular_polygon_2d(3, 1.0);
    let (pn, ph) = regular_polygon_2d(4, 1.0);
    let polytope = lagrangian_product(&qn, &qh, &pn, &ph).unwrap();
    assert_eq!(polytope.facet_count(), 7);
}

#[test]
fn volume_equals_product_of_areas() {
    // For several polygon pairs, check vol_4(P x_L Q) = area(P) * area(Q).
    let pairs = [(3, 3), (3, 4), (4, 4), (3, 5), (5, 5)];
    for (n1, n2) in pairs {
        let (qn, qh) = regular_polygon_2d(n1, 1.0);
        let (pn, ph) = regular_polygon_2d(n2, 1.0);
        let polytope = lagrangian_product(&qn, &qh, &pn, &ph).unwrap();

        let vol4 = volume(&polytope).unwrap();
        let area_q = polygon_area(&qn, &qh).unwrap();
        let area_p = polygon_area(&pn, &ph).unwrap();
        let expected = area_q * area_p;

        let rel_err = (vol4 - expected).abs() / expected;
        assert!(
            rel_err < 1e-6,
            "({n1},{n2}): vol4={vol4}, area_q*area_p={expected}, rel_err={rel_err}"
        );
    }
}

#[test]
fn rotated_pentagon_product_matches_hko_volume() {
    // Regular pentagon with our convention (starting angle pi/2) vs HKO (starting angle pi/5).
    // Both are regular pentagons with circumradius 1 -> same area/volume.
    let (qn, qh) = regular_polygon_2d(5, 1.0);
    let (pn_base, ph_base) = regular_polygon_2d(5, 1.0);
    let (pn, ph) = rotate_polygon_2d(&pn_base, &ph_base, PI / 2.0);

    let our_polytope = lagrangian_product(&qn, &qh, &pn, &ph).unwrap();
    let hko = crate::geom::known_polytopes::hko_pentagon();

    // Same facet count
    assert_eq!(our_polytope.facet_count(), hko.polytope.facet_count());

    // Same volume
    let our_vol = volume(&our_polytope).unwrap();
    let hko_vol = volume(&hko.polytope).unwrap();
    let rel_err = (our_vol - hko_vol).abs() / hko_vol;
    assert!(
        rel_err < 1e-6,
        "volume mismatch: ours={our_vol}, hko={hko_vol}, rel_err={rel_err}"
    );
}

#[test]
fn rotated_product_volume_independent_of_angle() {
    // vol(P x_L R(theta)Q) = area(P) * area(Q), independent of theta
    let (qn, qh) = regular_polygon_2d(5, 1.0);
    let (pn, ph) = regular_polygon_2d(5, 1.0);

    let angles = [0.0, PI / 10.0, PI / 5.0, PI / 3.0, PI / 2.0];
    let expected_area = polygon_area(&qn, &qh).unwrap();
    let expected_vol = expected_area * expected_area;

    for &theta in &angles {
        let (rpn, rph) = rotate_polygon_2d(&pn, &ph, theta);
        let polytope = lagrangian_product(&qn, &qh, &rpn, &rph).unwrap();
        let vol = volume(&polytope).unwrap();
        let rel_err = (vol - expected_vol).abs() / expected_vol;
        assert!(
            rel_err < 1e-6,
            "theta={theta:.3}: vol={vol}, expected={expected_vol}, rel_err={rel_err}"
        );
    }
}

// ---- Error propagation: Polytope4D::new errors pass through ----

#[test]
fn rejects_too_few_total_facets() {
    // Triangle (3 facets) + 1 p-facet = 4 total < 5 minimum.
    let (qn, qh) = regular_polygon_2d(3, 1.0);
    let pn = vec![Vector2::new(1.0, 0.0)];
    let ph = vec![1.0];
    let err = lagrangian_product(&qn, &qh, &pn, &ph).unwrap_err();
    assert_eq!(err, ConstructionError::TooFewFacets(4));
}

#[test]
fn rejects_unbounded_single_factor() {
    // Triangle in q-space (3 facets) + 2 normals in p-space (not enough to bound p-space).
    // Total = 5 facets, but unbounded in the -p direction.
    let (qn, qh) = regular_polygon_2d(3, 1.0);
    let pn = vec![Vector2::new(1.0, 0.0), Vector2::new(0.0, 1.0)];
    let ph = vec![1.0, 1.0];
    let err = lagrangian_product(&qn, &qh, &pn, &ph).unwrap_err();
    assert_eq!(err, ConstructionError::Unbounded);
}

/// Q-type facets have normals in the q-plane (components [0,1] nonzero, [2,3] zero).
#[test]
fn q_type_facets_in_q_plane() {
    let (qn, qh) = regular_polygon_2d(3, 1.0);
    let (pn, ph) = regular_polygon_2d(3, 1.0);
    let polytope = lagrangian_product(&qn, &qh, &pn, &ph).unwrap();
    let normals = polytope.normals_f64();

    // First 3 facets are from q-polygon: p-components should be zero
    for i in 0..3 {
        assert!(
            normals[i][2].abs() < 1e-12 && normals[i][3].abs() < 1e-12,
            "facet {i} should be Q-type: n = {:?}",
            normals[i]
        );
    }

    // Last 3 facets are from p-polygon: q-components should be zero
    for i in 3..6 {
        assert!(
            normals[i][0].abs() < 1e-12 && normals[i][1].abs() < 1e-12,
            "facet {i} should be P-type: n = {:?}",
            normals[i]
        );
    }
}

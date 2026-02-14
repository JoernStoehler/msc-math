use super::*;
use crate::polygon::{polygon_area, regular_polygon_2d, rotate_polygon_2d};
use crate::volume::volume;
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
    // For several polygon pairs, check vol4(P x_L Q) = area(P) * area(Q).
    let pairs: Vec<(usize, usize)> = vec![(3, 3), (3, 4), (4, 4), (3, 5), (5, 5)];
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
fn rotated_pentagon_product_has_same_volume_as_hko() {
    // Our convention uses starting angle π/2, HK-O uses π/5.
    // Both are regular pentagons with circumradius 1, so same area/volume.
    // Since sys is invariant under Sp(4) (rotations that map one pentagon to the other),
    // both constructions have the same sys value.
    let (qn, qh) = regular_polygon_2d(5, 1.0);
    let (pn_base, ph_base) = regular_polygon_2d(5, 1.0);
    let (pn, ph) = rotate_polygon_2d(&pn_base, &ph_base, PI / 2.0);

    let our_polytope = lagrangian_product(&qn, &qh, &pn, &ph).unwrap();
    let hko = crate::known_polytopes::hko_pentagon();

    // Same facet count
    assert_eq!(our_polytope.facet_count(), hko.polytope.facet_count());

    // Same heights (both use circumradius 1 -> inradius = cos(π/5))
    let our_h = our_polytope.heights();
    let hko_h = hko.polytope.heights();
    for i in 0..10 {
        assert!(
            (our_h[i] - hko_h[i]).abs() < 1e-10,
            "height[{i}] mismatch: ours={}, theirs={}",
            our_h[i],
            hko_h[i]
        );
    }

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
    // vol(P x_L R(θ)Q) = area(P) * area(Q), independent of θ
    let (qn, qh) = regular_polygon_2d(5, 1.0);
    let (pn, ph) = regular_polygon_2d(5, 1.0);

    let angles = [0.0, PI / 10.0, PI / 5.0, PI / 3.0, PI / 2.0];
    let expected_area = polygon_area(&qn, &qh).unwrap();
    let expected_vol = expected_area * expected_area; // same polygon both sides

    for &theta in &angles {
        let (rpn, rph) = rotate_polygon_2d(&pn, &ph, theta);
        let polytope = lagrangian_product(&qn, &qh, &rpn, &rph).unwrap();
        let vol = volume(&polytope).unwrap();
        let rel_err = (vol - expected_vol).abs() / expected_vol;
        assert!(
            rel_err < 1e-6,
            "θ={theta:.3}: vol={vol}, expected={expected_vol}, rel_err={rel_err}"
        );
    }
}

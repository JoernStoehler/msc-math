//! Construct 4D Lagrangian products from pairs of 2D convex polygons.
//!
//! A Lagrangian product P x_L Q places polygon P in q-space (q_1, q_2)
//! and polygon Q in p-space (p_1, p_2). The 4D polytope has facets from both
//! factors, with normals embedded into the respective Lagrangian subspaces:
//!
//! - P-facets: n = (n_P, 0, 0) in R^4 (components [0,1], q-space)
//! - Q-facets: n = (0, 0, n_Q) in R^4 (components [2,3], p-space)
//!
//! Coordinates: (q_1, q_2, p_1, p_2). See `symplectic_form` module for J_0 and omega_0.
//!
//! Volume: vol_4(P x_L Q) = area(P) * area(Q) (Fubini's theorem on
//! complementary Lagrangian subspaces).
//!
//! Mathematical correspondence: [def:lagrangian-product]

use crate::geom::polytope::{ConstructionError, Polytope4D};
use nalgebra::{Vector2, Vector4};

/// Build a 4D Lagrangian product from two 2D polygons.
///
/// `q_normals`/`q_heights`: polygon P in q-space (q_1, q_2).
/// `p_normals`/`p_heights`: polygon Q in p-space (p_1, p_2).
///
/// Embeds each 2D normal into 4D and constructs via `Polytope4D::new`.
/// Requires `q_normals.len() + p_normals.len() >= 5` (Polytope4D minimum).
///
/// Mathematical correspondence: [def:lagrangian-product]
pub fn lagrangian_product(
    q_normals: &[Vector2<f64>],
    q_heights: &[f64],
    p_normals: &[Vector2<f64>],
    p_heights: &[f64],
) -> Result<Polytope4D, ConstructionError> {
    // Dual vertex representation: a_i = n_i / h_i, embedded in 4D
    let halfspaces: Vec<Vector4<f64>> = q_normals
        .iter()
        .zip(q_heights.iter())
        .map(|(n, &h)| Vector4::new(n[0], n[1], 0.0, 0.0) / h)
        .chain(
            p_normals
                .iter()
                .zip(p_heights.iter())
                .map(|(n, &h)| Vector4::new(0.0, 0.0, n[0], n[1]) / h),
        )
        .collect();

    Polytope4D::from_f64(halfspaces)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geom::polygon::{polygon_area, regular_polygon_2d, rotate_polygon_2d};
    use crate::geom::volume::volume;
    use std::f64::consts::PI;

    // Tests for lagrangian_product: facet count, Q/P classification, volume.
    //
    // Proposition: lagrangian_product(P, Q) produces a polytope with
    //   F = |P| + |Q| facets, and vol_4 = area(P) * area(Q).
    // Reference: [def:lagrangian-product]
    //
    // Strategy: fixture-based (triangle x triangle, pentagon x pentagon, etc.)

    /// Verify triangle x_L triangle has 3+3 = 6 facets.
    #[test]
    fn triangle_x_triangle_has_6_facets() {
        let (qn, qh) = regular_polygon_2d(3, 1.0);
        let (pn, ph) = regular_polygon_2d(3, 1.0);
        let polytope = lagrangian_product(&qn, &qh, &pn, &ph).unwrap();
        assert_eq!(polytope.facet_count(), 6);
    }

    /// Verify pentagon x_L pentagon has 5+5 = 10 facets.
    #[test]
    fn pentagon_x_pentagon_has_10_facets() {
        let (qn, qh) = regular_polygon_2d(5, 1.0);
        let (pn, ph) = regular_polygon_2d(5, 1.0);
        let polytope = lagrangian_product(&qn, &qh, &pn, &ph).unwrap();
        assert_eq!(polytope.facet_count(), 10);
    }

    /// Verify triangle x_L square has 3+4 = 7 facets.
    #[test]
    fn triangle_x_square_has_7_facets() {
        let (qn, qh) = regular_polygon_2d(3, 1.0);
        let (pn, ph) = regular_polygon_2d(4, 1.0);
        let polytope = lagrangian_product(&qn, &qh, &pn, &ph).unwrap();
        assert_eq!(polytope.facet_count(), 7);
    }

    /// Verify vol_4(P x_L Q) = area(P) * area(Q) for several polygon pairs.
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

    /// Verify rotated pentagon product has same facet count and volume as HKO pentagon.
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

    /// Verify Lagrangian product volume is independent of rotation angle.
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

    /// Verify lagrangian_product rejects inputs with fewer than 5 total facets.
    #[test]
    fn rejects_too_few_total_facets() {
        // Triangle (3 facets) + 1 p-facet = 4 total < 5 minimum.
        let (qn, qh) = regular_polygon_2d(3, 1.0);
        let pn = vec![Vector2::new(1.0, 0.0)];
        let ph = vec![1.0];
        let err = lagrangian_product(&qn, &qh, &pn, &ph).unwrap_err();
        assert_eq!(err, ConstructionError::TooFewFacets(4));
    }

    /// Verify lagrangian_product rejects unbounded p-factor (too few p-normals).
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
        for (i, n) in normals.iter().enumerate().take(3) {
            assert!(
                n[2].abs() < 1e-12 && n[3].abs() < 1e-12,
                "facet {i} should be Q-type: n = {:?}",
                n
            );
        }

        // Last 3 facets are from p-polygon: q-components should be zero
        for (i, n) in normals.iter().enumerate().take(6).skip(3) {
            assert!(
                n[0].abs() < 1e-12 && n[1].abs() < 1e-12,
                "facet {i} should be P-type: n = {:?}",
                n
            );
        }
    }
}

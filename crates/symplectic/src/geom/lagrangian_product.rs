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

use crate::exact::{exact_vertices_with_incidence, ExactPolytopeError};
use crate::geom::rational_arithmetic::f64_to_rational;
use nalgebra::{Vector2, Vector4};
use num_rational::BigRational;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LagrangianProductError {
    MismatchedInputLengths {
        factor: &'static str,
        normals: usize,
        heights: usize,
    },
    NonFiniteNormal {
        factor: &'static str,
        index: usize,
        coordinate: usize,
    },
    NonFiniteHeight {
        factor: &'static str,
        index: usize,
    },
    NonPositiveHeight {
        factor: &'static str,
        index: usize,
    },
    DegenerateDualVertex(usize),
    DuplicateDualVertices {
        i: usize,
        j: usize,
    },
    Exact(ExactPolytopeError),
}

impl From<ExactPolytopeError> for LagrangianProductError {
    fn from(error: ExactPolytopeError) -> Self {
        Self::Exact(error)
    }
}

/// Build a 4D Lagrangian product from two 2D polygons.
///
/// `q_normals`/`q_heights`: polygon P in q-space (q_1, q_2).
/// `p_normals`/`p_heights`: polygon Q in p-space (p_1, p_2).
///
/// Embeds each 2D normal into 4D and validates the resulting exact dual
/// vertices as a bounded irredundant 4D polytope.
///
/// Mathematical correspondence: [def:lagrangian-product]
pub fn lagrangian_product(
    q_normals: &[Vector2<f64>],
    q_heights: &[f64],
    p_normals: &[Vector2<f64>],
    p_heights: &[f64],
) -> Result<Vec<Vector4<f64>>, LagrangianProductError> {
    validate_polygon_factor("q", q_normals, q_heights)?;
    validate_polygon_factor("p", p_normals, p_heights)?;

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

    validate_dual_vertices_f64(&halfspaces)?;

    let exact_dual_vertices = dual_vertices_f64_to_exact(&halfspaces);
    exact_vertices_with_incidence(&exact_dual_vertices)?;
    Ok(halfspaces)
}

fn validate_polygon_factor(
    factor: &'static str,
    normals: &[Vector2<f64>],
    heights: &[f64],
) -> Result<(), LagrangianProductError> {
    if normals.len() != heights.len() {
        return Err(LagrangianProductError::MismatchedInputLengths {
            factor,
            normals: normals.len(),
            heights: heights.len(),
        });
    }
    for (index, normal) in normals.iter().enumerate() {
        for coordinate in 0..2 {
            if !normal[coordinate].is_finite() {
                return Err(LagrangianProductError::NonFiniteNormal {
                    factor,
                    index,
                    coordinate,
                });
            }
        }
    }
    for (index, height) in heights.iter().enumerate() {
        if !height.is_finite() {
            return Err(LagrangianProductError::NonFiniteHeight { factor, index });
        }
        if *height <= 0.0 {
            return Err(LagrangianProductError::NonPositiveHeight { factor, index });
        }
    }
    Ok(())
}

fn validate_dual_vertices_f64(
    dual_vertices: &[Vector4<f64>],
) -> Result<(), LagrangianProductError> {
    for (i, dual_vertex) in dual_vertices.iter().enumerate() {
        for coordinate in 0..4 {
            if !dual_vertex[coordinate].is_finite() {
                return Err(LagrangianProductError::NonFiniteNormal {
                    factor: "dual",
                    index: i,
                    coordinate,
                });
            }
        }
        if dual_vertex.norm() < 1e-15 {
            return Err(LagrangianProductError::DegenerateDualVertex(i));
        }
    }

    for i in 0..dual_vertices.len() {
        for j in (i + 1)..dual_vertices.len() {
            let max_norm = dual_vertices[i].norm().max(dual_vertices[j].norm());
            if (dual_vertices[i] - dual_vertices[j]).norm() < 1e-8 * max_norm {
                return Err(LagrangianProductError::DuplicateDualVertices { i, j });
            }
        }
    }

    Ok(())
}

fn dual_vertices_f64_to_exact(dual_vertices: &[Vector4<f64>]) -> Vec<Vector4<BigRational>> {
    dual_vertices
        .iter()
        .map(|a| {
            Vector4::new(
                f64_to_rational(a[0]),
                f64_to_rational(a[1]),
                f64_to_rational(a[2]),
                f64_to_rational(a[3]),
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geom::polygon::{polygon_area, regular_polygon_2d, rotate_polygon_2d};
    use euclidean_polytopes::volume_from_incidence_exact;
    use num_traits::ToPrimitive;
    use std::f64::consts::PI;

    fn exact_vertices_and_incidence(
        dual_vertices: &[Vector4<f64>],
    ) -> (Vec<Vector4<BigRational>>, nalgebra::DMatrix<bool>) {
        let exact_dual_vertices = dual_vertices_f64_to_exact(dual_vertices);
        let exact = exact_vertices_with_incidence(&exact_dual_vertices)
            .expect("validated Lagrangian product should enumerate vertices");
        (exact.vertices, exact.vertex_facet_incidence)
    }

    fn euclidean_volume_f64(
        vertices: &[Vector4<BigRational>],
        incidence: &nalgebra::DMatrix<bool>,
    ) -> f64 {
        volume_from_incidence_exact(vertices, incidence)
            .to_f64()
            .unwrap_or(f64::NAN)
    }

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
        let dual_vertices = lagrangian_product(&qn, &qh, &pn, &ph).unwrap();
        assert_eq!(dual_vertices.len(), 6);
    }

    /// Verify pentagon x_L pentagon has 5+5 = 10 facets.
    #[test]
    fn pentagon_x_pentagon_has_10_facets() {
        let (qn, qh) = regular_polygon_2d(5, 1.0);
        let (pn, ph) = regular_polygon_2d(5, 1.0);
        let dual_vertices = lagrangian_product(&qn, &qh, &pn, &ph).unwrap();
        assert_eq!(dual_vertices.len(), 10);
    }

    /// Verify triangle x_L square has 3+4 = 7 facets.
    #[test]
    fn triangle_x_square_has_7_facets() {
        let (qn, qh) = regular_polygon_2d(3, 1.0);
        let (pn, ph) = regular_polygon_2d(4, 1.0);
        let dual_vertices = lagrangian_product(&qn, &qh, &pn, &ph).unwrap();
        assert_eq!(dual_vertices.len(), 7);
    }

    /// Verify vol_4(P x_L Q) = area(P) * area(Q) for several polygon pairs.
    #[test]
    fn volume_equals_product_of_areas() {
        // For several polygon pairs, check vol_4(P x_L Q) = area(P) * area(Q).
        let pairs = [(3, 3), (3, 4), (4, 4), (3, 5), (5, 5)];
        for (n1, n2) in pairs {
            let (qn, qh) = regular_polygon_2d(n1, 1.0);
            let (pn, ph) = regular_polygon_2d(n2, 1.0);
            let dual_vertices = lagrangian_product(&qn, &qh, &pn, &ph).unwrap();
            let (vertices, incidence) = exact_vertices_and_incidence(&dual_vertices);

            let vol4 = euclidean_volume_f64(&vertices, &incidence);
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

        let our_dual_vertices = lagrangian_product(&qn, &qh, &pn, &ph).unwrap();
        let hko = crate::geom::known_polytopes::hko_pentagon();

        // Same facet count
        assert_eq!(our_dual_vertices.len(), hko.facet_count());

        // Same volume
        let (our_vertices, our_incidence) = exact_vertices_and_incidence(&our_dual_vertices);
        let our_vol = euclidean_volume_f64(&our_vertices, &our_incidence);
        let hko_vertices: Vec<Vector4<BigRational>> = hko
            .vertices
            .iter()
            .map(|v| Vector4::new(v[0].clone(), v[1].clone(), v[2].clone(), v[3].clone()))
            .collect();
        let hko_vol = euclidean_volume_f64(&hko_vertices, &hko.vertex_facet_incidence);
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
            let dual_vertices = lagrangian_product(&qn, &qh, &rpn, &rph).unwrap();
            let (vertices, incidence) = exact_vertices_and_incidence(&dual_vertices);
            let vol = euclidean_volume_f64(&vertices, &incidence);
            let rel_err = (vol - expected_vol).abs() / expected_vol;
            assert!(
                rel_err < 1e-6,
                "theta={theta:.3}: vol={vol}, expected={expected_vol}, rel_err={rel_err}"
            );
        }
    }

    // ---- Error propagation: exact flat polytope validation errors pass through ----

    /// Verify lagrangian_product rejects inputs with fewer than 5 total facets.
    #[test]
    fn rejects_too_few_total_facets() {
        // Triangle (3 facets) + 1 p-facet = 4 total < 5 minimum.
        let (qn, qh) = regular_polygon_2d(3, 1.0);
        let pn = vec![Vector2::new(1.0, 0.0)];
        let ph = vec![1.0];
        let err = lagrangian_product(&qn, &qh, &pn, &ph).unwrap_err();
        assert_eq!(
            err,
            LagrangianProductError::Exact(ExactPolytopeError::TooFewFacets(4))
        );
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
        assert_eq!(
            err,
            LagrangianProductError::Exact(ExactPolytopeError::Unbounded)
        );
    }

    #[test]
    fn rejects_mismatched_polygon_input_lengths() {
        let (qn, qh) = regular_polygon_2d(3, 1.0);
        let (pn, mut ph) = regular_polygon_2d(3, 1.0);
        ph.pop();

        let err = lagrangian_product(&qn, &qh, &pn, &ph).unwrap_err();
        assert_eq!(
            err,
            LagrangianProductError::MismatchedInputLengths {
                factor: "p",
                normals: 3,
                heights: 2
            }
        );
    }

    #[test]
    fn rejects_nonpositive_and_nonfinite_heights_before_exact_conversion() {
        let (qn, qh) = regular_polygon_2d(3, 1.0);
        let (pn, mut ph) = regular_polygon_2d(3, 1.0);
        ph[0] = 0.0;
        let err = lagrangian_product(&qn, &qh, &pn, &ph).unwrap_err();
        assert_eq!(
            err,
            LagrangianProductError::NonPositiveHeight {
                factor: "p",
                index: 0
            }
        );

        ph[0] = f64::NAN;
        let err = lagrangian_product(&qn, &qh, &pn, &ph).unwrap_err();
        assert_eq!(
            err,
            LagrangianProductError::NonFiniteHeight {
                factor: "p",
                index: 0
            }
        );
    }

    #[test]
    fn rejects_duplicate_lagrangian_product_facets() {
        let (mut qn, mut qh) = regular_polygon_2d(3, 1.0);
        let (pn, ph) = regular_polygon_2d(3, 1.0);
        qn[1] = qn[0];
        qh[1] = qh[0];

        let err = lagrangian_product(&qn, &qh, &pn, &ph).unwrap_err();
        assert_eq!(
            err,
            LagrangianProductError::DuplicateDualVertices { i: 0, j: 1 }
        );
    }

    /// Q-type facets have dual vertices in the q-plane (components [0,1] nonzero, [2,3] zero).
    #[test]
    fn q_type_facets_in_q_plane() {
        let (qn, qh) = regular_polygon_2d(3, 1.0);
        let (pn, ph) = regular_polygon_2d(3, 1.0);
        let duals = lagrangian_product(&qn, &qh, &pn, &ph).unwrap();

        // First 3 facets are from q-polygon: p-components should be zero
        for (i, a) in duals.iter().enumerate().take(3) {
            assert!(
                a[2].abs() < 1e-12 && a[3].abs() < 1e-12,
                "facet {i} should be Q-type: a = {:?}",
                a
            );
        }

        // Last 3 facets are from p-polygon: q-components should be zero
        for (i, a) in duals.iter().enumerate().take(6).skip(3) {
            assert!(
                a[0].abs() < 1e-12 && a[1].abs() < 1e-12,
                "facet {i} should be P-type: a = {:?}",
                a
            );
        }
    }
}

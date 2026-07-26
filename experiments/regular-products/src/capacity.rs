//! Production product-capacity adapter for regular-product sweeps.
//!
//! The sweep needs an exact capacity and one deterministic minimizing word for
//! its bounce count. It does not need legacy billiard iterations or a full
//! orbit payload.

use crate::ProductPolytopeCache;
use nalgebra::Vector4;
use num_rational::BigRational;
use symplectic::capacity_4d::{
    check_dual_vertex_norm_bounds, check_facet_count, check_finite_dual_vertices,
    check_primal_vertex_norm_bounds, product_qp_minimizers, CapacityBounds4d, PolytopeGeometry4d,
    QpCandidateFamily4d,
};

#[derive(Clone, Debug, PartialEq)]
pub struct ProductMinimum {
    pub capacity_exact: BigRational,
    pub capacity_bounds: CapacityBounds4d,
    pub sigma: Vec<usize>,
}

pub fn product_minimum(polytope: &ProductPolytopeCache) -> Result<ProductMinimum, String> {
    check_facet_count(polytope.dual_vertices_f64.len())
        .map_err(|error| format!("facet-count check failed: {error}"))?;
    check_finite_dual_vertices(&polytope.dual_vertices_f64)
        .map_err(|error| format!("finite-coordinate check failed: {error}"))?;
    check_dual_vertex_norm_bounds(&polytope.dual_vertices_f64)
        .map_err(|error| format!("dual-vertex norm check failed: {error}"))?;

    let geometry = PolytopeGeometry4d {
        dual_vertices: polytope.dual_vertices_f64.clone(),
        dual_vertices_exact: vectors_from_arrays(&polytope.dual_vertices),
        primal_vertices_exact: vectors_from_arrays(&polytope.vertices),
        vertex_facet_incidence: polytope.vertex_facet_incidence.clone(),
    };
    check_primal_vertex_norm_bounds(&geometry)
        .map_err(|error| format!("primal-vertex norm check failed: {error}"))?;

    let minimizers = product_qp_minimizers(&geometry)
        .map_err(|error| format!("product minimizer search failed: {error}"))?;
    if minimizers.family() != QpCandidateFamily4d::ProductClosureVertex {
        return Err("product route returned the wrong candidate family".to_string());
    }
    let candidate = minimizers
        .candidates()
        .iter()
        .min_by(|left, right| left.sigma().cmp(right.sigma()))
        .expect("a successful product minimizer search returns a candidate");

    Ok(ProductMinimum {
        capacity_exact: candidate.action_exact().clone(),
        capacity_bounds: minimizers.bounds(),
        sigma: candidate.sigma().to_vec(),
    })
}

fn vectors_from_arrays(data: &[[BigRational; 4]]) -> Vec<Vector4<BigRational>> {
    data.iter()
        .map(|row| {
            Vector4::new(
                row[0].clone(),
                row[1].clone(),
                row[2].clone(),
                row[3].clone(),
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_traits::ToPrimitive;
    use symplectic::capacity_4d::qp_minimizers_from_dual_vertices;
    use symplectic::geom::polygon::regular_polygon_2d;

    #[test]
    fn cached_geometry_matches_the_raw_input_product_route() {
        let (q_normals, q_heights) = regular_polygon_2d(5, 1.0);
        let (p_normals, p_heights) = regular_polygon_2d(5, 1.0);
        let polytope = ProductPolytopeCache::from_lagrangian_product(
            &q_normals, &q_heights, &p_normals, &p_heights,
        )
        .expect("pentagon product");
        let observed = product_minimum(&polytope).expect("cached product geometry");
        let raw = qp_minimizers_from_dual_vertices(&polytope.dual_vertices_f64)
            .expect("raw product geometry");
        let raw_first = raw
            .candidates()
            .iter()
            .min_by(|left, right| left.sigma().cmp(right.sigma()))
            .expect("raw product minimizer");

        assert_eq!(observed.capacity_exact, *raw_first.action_exact());
        assert_eq!(observed.sigma, raw_first.sigma());
        assert!(observed
            .capacity_exact
            .to_f64()
            .expect("capacity fits f64")
            .is_finite());

        let repeated = product_minimum(&polytope).expect("repeated product solve must succeed");
        assert_eq!(observed.sigma, repeated.sigma);
    }
}

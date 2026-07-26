//! Cached exact/f64 data for Lagrangian products of planar polygons.

use euclidean_polytopes::{
    all_points_are_extreme_exact, facet_intersection_is_nonempty_from_vertex_facet_incidence,
    origin_in_interior_of_conv_exact, polar_vertices_exact_rational_assuming_origin_interior,
};
use nalgebra::{DMatrix, Vector2, Vector4};
use num_rational::BigRational;
use symplectic::exact::omega_signs_exact;

const EPS_ZERO_NORM: f64 = 1e-12;
const EPS_DUPLICATE_RELATIVE: f64 = 1e-10;

#[derive(Clone, Debug)]
pub struct ProductPolytopeCache {
    pub dual_vertices: Vec<[BigRational; 4]>,
    pub vertices: Vec<[BigRational; 4]>,
    pub vertex_facet_incidence: DMatrix<bool>,
    pub facet_intersection_is_nonempty: DMatrix<bool>,
    pub omega_signs: DMatrix<i8>,
    pub dual_vertices_f64: Vec<Vector4<f64>>,
}

impl ProductPolytopeCache {
    pub fn from_lagrangian_product(
        q_normals: &[Vector2<f64>],
        q_heights: &[f64],
        p_normals: &[Vector2<f64>],
        p_heights: &[f64],
    ) -> Option<Self> {
        if q_normals.len() != q_heights.len() || p_normals.len() != p_heights.len() {
            return None;
        }
        let mut dual_vertices = Vec::with_capacity(q_normals.len() + p_normals.len());
        for (normal, height) in q_normals.iter().zip(q_heights) {
            if !height.is_finite() || *height <= 0.0 {
                return None;
            }
            dual_vertices.push(Vector4::new(
                normal[0] / height,
                normal[1] / height,
                0.0,
                0.0,
            ));
        }
        for (normal, height) in p_normals.iter().zip(p_heights) {
            if !height.is_finite() || *height <= 0.0 {
                return None;
            }
            dual_vertices.push(Vector4::new(
                0.0,
                0.0,
                normal[0] / height,
                normal[1] / height,
            ));
        }
        Self::from_f64_dual_vertices(dual_vertices)
    }

    pub fn facet_count(&self) -> usize {
        self.dual_vertices.len()
    }

    fn from_f64_dual_vertices(dual_vertices_f64: Vec<Vector4<f64>>) -> Option<Self> {
        validate_f64_dual_vertices(&dual_vertices_f64)?;
        let dual_vertices = dual_vertices_f64
            .iter()
            .map(|a| {
                Some(std::array::from_fn(|c| {
                    BigRational::from_float(a[c]).expect("finite f64 was validated")
                }))
            })
            .collect::<Option<Vec<_>>>()?;
        let dual_vectors = vectors_from_arrays(&dual_vertices);

        if !origin_in_interior_of_conv_exact(&dual_vectors)
            || !all_points_are_extreme_exact(&dual_vectors)
        {
            return None;
        }

        let polar = polar_vertices_exact_rational_assuming_origin_interior(&dual_vectors);
        let vertices = arrays_from_vectors(&polar.vertices);
        let facet_intersection_is_nonempty =
            facet_intersection_is_nonempty_from_vertex_facet_incidence(
                &polar.vertex_facet_incidence,
            );
        let omega_signs = omega_signs_exact(&dual_vectors);

        Some(Self {
            dual_vertices,
            vertices,
            vertex_facet_incidence: polar.vertex_facet_incidence,
            facet_intersection_is_nonempty,
            omega_signs,
            dual_vertices_f64,
        })
    }
}

fn validate_f64_dual_vertices(dual_vertices_f64: &[Vector4<f64>]) -> Option<()> {
    if dual_vertices_f64.len() < 5 {
        return None;
    }
    for a in dual_vertices_f64 {
        if !a.iter().all(|value| value.is_finite()) || a.norm() < EPS_ZERO_NORM {
            return None;
        }
    }
    for i in 0..dual_vertices_f64.len() {
        for j in i + 1..dual_vertices_f64.len() {
            let max_norm = dual_vertices_f64[i].norm().max(dual_vertices_f64[j].norm());
            if (dual_vertices_f64[i] - dual_vertices_f64[j]).norm()
                < EPS_DUPLICATE_RELATIVE * max_norm
            {
                return None;
            }
        }
    }
    Some(())
}

fn vectors_from_arrays(data: &[[BigRational; 4]]) -> Vec<Vector4<BigRational>> {
    data.iter()
        .map(|v| Vector4::new(v[0].clone(), v[1].clone(), v[2].clone(), v[3].clone()))
        .collect()
}

fn arrays_from_vectors(data: &[Vector4<BigRational>]) -> Vec<[BigRational; 4]> {
    data.iter()
        .map(|v| [v[0].clone(), v[1].clone(), v[2].clone(), v[3].clone()])
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::exact_volume_reference_as_f64;
    use symplectic::classify_facets_from_dual_vertices;
    use symplectic::geom::polygon::{regular_polygon_2d, rotate_polygon_2d};

    fn pentagon_product(theta: f64) -> ProductPolytopeCache {
        let (qn, qh) = regular_polygon_2d(5, 1.0);
        let (pn_base, ph_base) = regular_polygon_2d(5, 1.0);
        let (pn, ph) = rotate_polygon_2d(&pn_base, &ph_base, theta);
        ProductPolytopeCache::from_lagrangian_product(&qn, &qh, &pn, &ph)
            .expect("pentagon product should construct")
    }

    #[test]
    fn pentagon_product_has_ten_facets() {
        assert_eq!(pentagon_product(0.0).facet_count(), 10);
    }

    #[test]
    fn pentagon_product_classifies_as_lagrangian_product() {
        let polytope = pentagon_product(0.2);
        let classification = classify_facets_from_dual_vertices(&polytope.dual_vertices_f64)
            .expect("product classification should succeed");
        assert_eq!(classification.q_indices.len(), 5);
        assert_eq!(classification.p_indices.len(), 5);
    }

    #[test]
    fn pentagon_product_volume_is_rotation_invariant() {
        let base = pentagon_product(0.0);
        let base_volume =
            exact_volume_reference_as_f64(&base.vertices, &base.vertex_facet_incidence);
        for theta in [0.1, 0.3, std::f64::consts::PI / 5.0] {
            let rotated = pentagon_product(theta);
            let rotated_volume =
                exact_volume_reference_as_f64(&rotated.vertices, &rotated.vertex_facet_incidence);
            assert!((base_volume - rotated_volume).abs() <= 1e-9);
        }
    }
}

//! Slow executable references for numerical audits and retained evaluators.
//!
//! These functions are deliberately separate from ordinary experiment
//! computation. Production-style f64 callers should use the corresponding
//! `euclidean_polytopes` f64 API directly.

use euclidean_polytopes::volume_from_incidence_exact;
use nalgebra::{DMatrix, Vector4};
use num_rational::BigRational;
use num_traits::ToPrimitive;

/// Exact-binary64 rational volume reference rounded once to f64.
///
/// Use this for reference comparisons or artifacts whose contract explicitly
/// names rational-arithmetic volume. It does not return an exact value.
///
/// Ordinary f64 `sys` and feature computations with exact-derived incidence
/// should call `euclidean_polytopes::volume_from_incidence_f64` on cached f64
/// vertices. A retained comparison under
/// `experiments/sys-datascience/methods/generic-ridge-tail-stage1/` measured a
/// maximum relative volume error of `9.51e-15` across 512 generic F10 rows and
/// a `16,427x` aggregate worker-time ratio.
pub fn exact_volume_as_f64(vertices: &[[BigRational; 4]], incidence: &DMatrix<bool>) -> f64 {
    let vertices: Vec<Vector4<BigRational>> = vertices
        .iter()
        .map(|v| Vector4::new(v[0].clone(), v[1].clone(), v[2].clone(), v[3].clone()))
        .collect();
    ToPrimitive::to_f64(&volume_from_incidence_exact(&vertices, incidence)).unwrap_or(f64::NAN)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SysLandscapePolytopeCache;
    use symplectic::geom::polygon::regular_polygon_2d;

    #[test]
    fn rounded_reference_matches_conversion_of_exact_result() {
        let (qn, qh) = regular_polygon_2d(3, 1.0);
        let (pn, ph) = regular_polygon_2d(3, 1.0);
        let polytope = SysLandscapePolytopeCache::from_lagrangian_product(&qn, &qh, &pn, &ph)
            .expect("triangle product should construct");
        let vertices: Vec<Vector4<BigRational>> = polytope
            .vertices
            .iter()
            .map(|v| Vector4::new(v[0].clone(), v[1].clone(), v[2].clone(), v[3].clone()))
            .collect();
        let exact = volume_from_incidence_exact(&vertices, &polytope.vertex_facet_incidence);

        assert_eq!(
            exact_volume_as_f64(&polytope.vertices, &polytope.vertex_facet_incidence),
            exact.to_f64().expect("exact volume should fit in f64")
        );
    }
}

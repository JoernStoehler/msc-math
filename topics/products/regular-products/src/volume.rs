//! Exact-volume reference used by retained regular-product producers.

use euclidean_polytopes::volume_from_incidence_exact;
use nalgebra::{DMatrix, Vector4};
use num_rational::BigRational;
use num_traits::ToPrimitive;

/// Compute exact rational volume, then round once for retained `f64` rows.
pub fn exact_volume_reference_as_f64(
    vertices: &[[BigRational; 4]],
    incidence: &DMatrix<bool>,
) -> f64 {
    let vertices: Vec<Vector4<BigRational>> = vertices
        .iter()
        .map(|v| Vector4::new(v[0].clone(), v[1].clone(), v[2].clone(), v[3].clone()))
        .collect();
    ToPrimitive::to_f64(&volume_from_incidence_exact(&vertices, incidence)).unwrap_or(f64::NAN)
}

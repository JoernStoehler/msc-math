//! Test module for `crate::lib` top-level capacity dispatch behavior.

use euclidean_polytopes::volume_from_incidence_exact;
use nalgebra::{DMatrix, Vector4};
use num_rational::BigRational;
use num_traits::ToPrimitive;

use super::*;

pub(crate) fn euclidean_volume_f64(
    vertices: &[[BigRational; 4]],
    incidence: &DMatrix<bool>,
) -> f64 {
    let vertices: Vec<Vector4<BigRational>> = vertices
        .iter()
        .map(|v| Vector4::new(v[0].clone(), v[1].clone(), v[2].clone(), v[3].clone()))
        .collect();
    ToPrimitive::to_f64(&volume_from_incidence_exact(&vertices, incidence)).unwrap_or(f64::NAN)
}

#[test]
fn top_level_capacity_matches_billiard_on_lagrangian_products() {
    let kp = known_polytopes::lagrangian_triangle_product();
    let auto = ehz_capacity(&kp.polytope).expect("auto capacity");
    let billiard =
        ehz_capacity_billiard(&kp.polytope).expect("billiard should accept Lagrangian product");

    assert!(
        (auto.capacity() - billiard.capacity()).abs() < 1e-10,
        "auto wrapper should agree with billiard on product inputs"
    );
    assert_eq!(
        auto.best_sigma(),
        billiard.best_sigma(),
        "auto wrapper should preserve the chosen billiard minimizer"
    );
}

#[test]
fn top_level_capacity_matches_pruned_hk2017_on_non_products() {
    let kp = known_polytopes::simplex();
    let auto = ehz_capacity(&kp.polytope).expect("auto capacity");
    let hk = ehz_capacity_pruned(&kp.polytope).expect("hk2017 capacity");

    assert!(
        (auto.capacity() - hk.capacity()).abs() < 1e-10,
        "auto wrapper should fall back to pruned HK2017 on non-products"
    );
    assert_eq!(
        auto.best_sigma(),
        hk.best_sigma(),
        "auto wrapper should preserve the pruned HK2017 minimizer on non-products"
    );
}

#[test]
fn systolic_ratio_matches_definition() {
    assert_eq!(systolic_ratio(3.0, 2.0), 9.0 / 4.0);
}

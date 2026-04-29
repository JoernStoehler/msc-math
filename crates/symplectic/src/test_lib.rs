//! Test module for `crate::lib` top-level capacity dispatch behavior.

use super::*;

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

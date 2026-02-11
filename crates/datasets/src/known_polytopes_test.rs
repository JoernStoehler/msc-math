use super::*;
use crate::validation::validate_polytope;

#[test]
fn all_known_polytopes_pass_validation() {
    for kp in all_known() {
        let result = validate_polytope(kp.polytope.normals(), kp.polytope.heights());
        assert!(
            result.is_ok(),
            "{}: validation failed: {}",
            kp.name,
            result.unwrap_err()
        );
    }
}

#[test]
fn simplex_has_5_facets() {
    assert_eq!(simplex().polytope.facet_count(), 5);
}

#[test]
fn hypercube_has_8_facets() {
    assert_eq!(hypercube().polytope.facet_count(), 8);
}

#[test]
fn crosspolytope_has_16_facets() {
    assert_eq!(crosspolytope().polytope.facet_count(), 16);
}

#[test]
fn hko_pentagon_has_10_facets() {
    assert_eq!(hko_pentagon().polytope.facet_count(), 10);
}

#[test]
fn triangle_product_has_6_facets() {
    assert_eq!(triangle_product().polytope.facet_count(), 6);
}

#[test]
fn lagrangian_tri_sq_has_7_facets() {
    assert_eq!(lagrangian_triangle_square().polytope.facet_count(), 7);
}

#[test]
fn all_known_capacities_positive() {
    for kp in all_known() {
        assert!(kp.capacity > 0.0, "{}: capacity should be > 0", kp.name);
    }
}

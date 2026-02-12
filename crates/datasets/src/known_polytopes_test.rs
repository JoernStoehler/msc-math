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

/// Verify symplectic_triangle_product capacity (3√3/4) against HK2017 algorithm.
#[test]
fn symplectic_triangle_product_capacity() {
    let kp = symplectic_triangle_product();
    let result = hk2017::ehz_capacity(&kp.polytope)
        .expect("symplectic_triangle_product should have capacity");
    let expected = 3.0 * 3.0_f64.sqrt() / 4.0;
    assert!(
        (result.capacity - expected).abs() < 1e-6,
        "symplectic_triangle_product capacity: got {}, expected {} (3√3/4)",
        result.capacity,
        expected
    );
}

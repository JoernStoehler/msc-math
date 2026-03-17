//! Tests for cross_product_4d: perpendicularity, magnitude, and orientation.
//!
//! Proposition: For a, b, c in R^4, d = a x b x c satisfies:
//!   <d, a> = <d, b> = <d, c> = 0 (perpendicularity),
//!   ||d|| = vol_3(parallelepiped(a, b, c)) (magnitude),
//!   d = 0 iff a, b, c linearly dependent (non-degeneracy).
//!
//! Strategy: fixture-based (basis vectors, specific vectors) + proptest 256 cases

use crate::geom::cross_product_4d::cross_product_4d;
use nalgebra::Vector4;

/// Verify the 4D cross product is perpendicular to all three input vectors.
#[test]
fn perpendicular_to_all_inputs() {
    let a = Vector4::new(1.0, 2.0, 3.0, 4.0);
    let b = Vector4::new(5.0, 6.0, 7.0, 8.0);
    let c = Vector4::new(9.0, 10.0, 11.0, 13.0); // not coplanar with a, b
    let d = cross_product_4d(a, b, c);
    assert!(
        d.dot(&a).abs() < 1e-10,
        "not perpendicular to a: {}",
        d.dot(&a)
    );
    assert!(
        d.dot(&b).abs() < 1e-10,
        "not perpendicular to b: {}",
        d.dot(&b)
    );
    assert!(
        d.dot(&c).abs() < 1e-10,
        "not perpendicular to c: {}",
        d.dot(&c)
    );
}

/// Verify cross(e_1, e_2, e_3) has unit norm and is parallel to e_4.
#[test]
fn standard_basis_produces_unit_vector() {
    // cross(e_1, e_2, e_3) should be +/-e_4 with norm 1
    let e1 = Vector4::x();
    let e2 = Vector4::y();
    let e3 = Vector4::z();
    let d = cross_product_4d(e1, e2, e3);
    assert!(
        (d.norm() - 1.0).abs() < 1e-10,
        "norm should be 1, got {}",
        d.norm()
    );
    // d should be parallel to e_4
    assert!(
        (d[3].abs() - 1.0).abs() < 1e-10,
        "should be +/-e_4, got {:?}",
        d
    );
}

/// Verify the cross product is zero when inputs are linearly dependent.
#[test]
fn zero_for_linearly_dependent_inputs() {
    let a = Vector4::new(1.0, 0.0, 0.0, 0.0);
    let b = Vector4::new(0.0, 1.0, 0.0, 0.0);
    let c = Vector4::new(1.0, 1.0, 0.0, 0.0); // c = a + b
    let d = cross_product_4d(a, b, c);
    assert!(
        d.norm() < 1e-10,
        "should be zero for dependent inputs, got {:?}",
        d
    );
}

/// Verify ||a x b x c|| equals the 3D parallelepiped volume spanned by a, b, c.
#[test]
fn magnitude_equals_parallelepiped_volume() {
    // For orthonormal vectors, the parallelepiped volume is 1
    let e1 = Vector4::x();
    let e2 = Vector4::y();
    let e3 = Vector4::z();
    let d = cross_product_4d(e1, e2, e3);
    assert!(
        (d.norm() - 1.0).abs() < 1e-10,
        "parallelepiped volume should be 1 for orthonormal basis, got {}",
        d.norm()
    );

    // For scaled vectors, volume scales as the product of norms
    let a = 2.0 * Vector4::x();
    let b = 3.0 * Vector4::y();
    let c = 5.0 * Vector4::z();
    let d = cross_product_4d(a, b, c);
    assert!(
        (d.norm() - 30.0).abs() < 1e-8,
        "parallelepiped volume should be 2*3*5=30, got {}",
        d.norm()
    );
}

/// Verify swapping the first two arguments negates the cross product.
#[test]
fn antisymmetric_in_first_two_args() {
    let a = Vector4::new(1.0, 2.0, 3.0, 4.0);
    let b = Vector4::new(5.0, 6.0, 7.0, 8.0);
    let c = Vector4::new(9.0, 10.0, 11.0, 13.0);
    let d_abc = cross_product_4d(a, b, c);
    let d_bac = cross_product_4d(b, a, c);
    let sum = d_abc + d_bac;
    assert!(
        sum.norm() < 1e-10,
        "swapping first two args should negate: sum = {:?}",
        sum
    );
}

#[cfg(test)]
mod proptests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        /// Property: cross product is perpendicular to all three inputs.
        #[test]
        fn perpendicular_to_inputs(
            a in prop::array::uniform4(-10.0f64..10.0f64),
            b in prop::array::uniform4(-10.0f64..10.0f64),
            c in prop::array::uniform4(-10.0f64..10.0f64)
        ) {
            let a_vec = Vector4::from(a);
            let b_vec = Vector4::from(b);
            let c_vec = Vector4::from(c);

            let d = cross_product_4d(a_vec, b_vec, c_vec);

            // Skip coplanar cases (zero cross product)
            if d.norm() > 1e-10 {
                let tol = 1e-10 * d.norm() * a_vec.norm().max(b_vec.norm()).max(c_vec.norm());

                prop_assert!(
                    d.dot(&a_vec).abs() < tol,
                    "not perpendicular to a: d.a={}",
                    d.dot(&a_vec)
                );
                prop_assert!(
                    d.dot(&b_vec).abs() < tol,
                    "not perpendicular to b: d.b={}",
                    d.dot(&b_vec)
                );
                prop_assert!(
                    d.dot(&c_vec).abs() < tol,
                    "not perpendicular to c: d.c={}",
                    d.dot(&c_vec)
                );
            }
        }
    }
}

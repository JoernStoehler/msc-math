//! 4D cross product: the unique vector perpendicular to three vectors in R^4.
//!
//! Given three linearly independent vectors a, b, c in R^4, their cross product
//! d = a x b x c is the unique (up to sign) vector perpendicular to all three,
//! with magnitude equal to the volume of the parallelepiped they span.
//!
//! Used by: `validation.rs` (boundedness check via positive span).
//!
//! Mathematical correspondence: [def:cross-product-4d]

use nalgebra::Vector4;

/// Cross product of three vectors in R^4.
///
/// Defined as the cofactor expansion of the formal determinant:
/// ```text
/// | e_1  e_2  e_3  e_4 |
/// | a_1  a_2  a_3  a_4 |
/// | b_1  b_2  b_3  b_4 |
/// | c_1  c_2  c_3  c_4 |
/// ```
///
/// Each component d_k = (-1)^k times the 3x3 minor obtained by deleting column k.
///
/// # Properties
///
/// - **Perpendicular**: <d, a> = <d, b> = <d, c> = 0
/// - **Magnitude**: ||d|| = vol_3(parallelepiped(a, b, c))
/// - **Zero iff dependent**: d = 0 iff a, b, c are linearly dependent
///
/// Mathematical correspondence: [def:cross-product-4d]
pub fn cross_product_4d(a: Vector4<f64>, b: Vector4<f64>, c: Vector4<f64>) -> Vector4<f64> {
    // Precompute 2x2 minors of the (b, c) submatrix: bc_ij = b[i]*c[j] - b[j]*c[i]
    let bc_01 = b[0] * c[1] - b[1] * c[0];
    let bc_02 = b[0] * c[2] - b[2] * c[0];
    let bc_03 = b[0] * c[3] - b[3] * c[0];
    let bc_12 = b[1] * c[2] - b[2] * c[1];
    let bc_13 = b[1] * c[3] - b[3] * c[1];
    let bc_23 = b[2] * c[3] - b[3] * c[2];

    // Cofactor expansion along the first row (the "a" row)
    let d0 = a[1] * bc_23 - a[2] * bc_13 + a[3] * bc_12;
    let d1 = -(a[0] * bc_23 - a[2] * bc_03 + a[3] * bc_02);
    let d2 = a[0] * bc_13 - a[1] * bc_03 + a[3] * bc_01;
    let d3 = -(a[0] * bc_12 - a[1] * bc_02 + a[2] * bc_01);

    Vector4::new(d0, d1, d2, d3)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Tests for cross_product_4d: perpendicularity, magnitude, and orientation.
    //
    // Proposition: For a, b, c in R^4, d = a x b x c satisfies:
    //   <d, a> = <d, b> = <d, c> = 0 (perpendicularity),
    //   ||d|| = vol_3(parallelepiped(a, b, c)) (magnitude),
    //   d = 0 iff a, b, c linearly dependent (non-degeneracy).
    //
    // Strategy: fixture-based (basis vectors, specific vectors) + proptest 256 cases

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
}

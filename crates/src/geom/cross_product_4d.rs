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

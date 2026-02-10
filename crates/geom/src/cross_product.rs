/// 4D cross product: vector perpendicular to three vectors in R⁴.
///
/// Defined as the cofactor expansion of:
/// ```text
/// | e₁  e₂  e₃  e₄ |
/// | a₁  a₂  a₃  a₄ |
/// | b₁  b₂  b₃  b₄ |
/// | c₁  c₂  c₃  c₄ |
/// ```
///
/// Properties:
/// - ⟨d, a⟩ = ⟨d, b⟩ = ⟨d, c⟩ = 0
/// - ‖d‖ = volume of the parallelepiped spanned by a, b, c
use nalgebra::Vector4;

pub fn cross_product_4d(a: Vector4<f64>, b: Vector4<f64>, c: Vector4<f64>) -> Vector4<f64> {
    let d0 = a[1] * (b[2] * c[3] - b[3] * c[2]) - a[2] * (b[1] * c[3] - b[3] * c[1])
        + a[3] * (b[1] * c[2] - b[2] * c[1]);
    let d1 = -(a[0] * (b[2] * c[3] - b[3] * c[2]) - a[2] * (b[0] * c[3] - b[3] * c[0])
        + a[3] * (b[0] * c[2] - b[2] * c[0]));
    let d2 = a[0] * (b[1] * c[3] - b[3] * c[1]) - a[1] * (b[0] * c[3] - b[3] * c[0])
        + a[3] * (b[0] * c[1] - b[1] * c[0]);
    let d3 = -(a[0] * (b[1] * c[2] - b[2] * c[1]) - a[1] * (b[0] * c[2] - b[2] * c[0])
        + a[2] * (b[0] * c[1] - b[1] * c[0]));
    Vector4::new(d0, d1, d2, d3)
}

#[cfg(test)]
#[path = "cross_product_test.rs"]
mod cross_product_test;

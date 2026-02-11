use nalgebra::{Matrix2, Matrix4, Vector4};

/// The standard symplectic matrix J in R^2: [[0, -1], [1, 0]].
pub fn j2() -> Matrix2<f64> {
    Matrix2::new(0.0, -1.0, 1.0, 0.0)
}

/// The standard symplectic matrix J₀ in R^4 (coordinates q1, q2, p1, p2):
/// J₀ = \[\[0, -I₂\], \[I₂, 0\]\], satisfying J₀² = -I₄ and ω₀(u,v) = ⟨J₀ u, v⟩.
pub fn j4() -> Matrix4<f64> {
    #[rustfmt::skip]
    let m = Matrix4::new(
         0.0,  0.0, -1.0,  0.0,
         0.0,  0.0,  0.0, -1.0,
         1.0,  0.0,  0.0,  0.0,
         0.0,  1.0,  0.0,  0.0,
    );
    m
}

/// Standard symplectic form: ω₀(u, v) = ⟨J₀ u, v⟩.
///
/// In coordinates (q1, q2, p1, p2):
///   ω₀(u, v) = u_q1 v_p1 - u_p1 v_q1 + u_q2 v_p2 - u_p2 v_q2
///
/// This is the direct formula, giving 1:1 correspondence with the mathematical definition.
/// Equivalently: ω₀(u, v) = ⟨J₀ u, v⟩ where J₀ = [[0, -I₂], [I₂, 0]].
pub fn omega0(u: &Vector4<f64>, v: &Vector4<f64>) -> f64 {
    u[0] * v[2] - u[2] * v[0] + u[1] * v[3] - u[3] * v[1]
}

#[cfg(test)]
#[path = "symplectic_test.rs"]
mod symplectic_test;

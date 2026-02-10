use super::*;
use nalgebra::{Matrix2, Matrix4, Vector4};

#[test]
fn j2_squared_is_minus_identity() {
    let j = j2();
    let j_sq = j * j;
    assert_eq!(j_sq, -Matrix2::identity());
}

#[test]
fn j4_squared_is_minus_identity() {
    let j = j4();
    let j_sq = j * j;
    assert!((j_sq + Matrix4::identity()).norm() < 1e-15);
}

#[test]
fn omega0_antisymmetric() {
    let u = Vector4::new(1.0, 2.0, 3.0, 4.0);
    let v = Vector4::new(5.0, 6.0, 7.0, 8.0);
    assert!((omega0(&u, &v) + omega0(&v, &u)).abs() < 1e-15);
}

#[test]
fn omega0_basis_vectors() {
    let e1 = Vector4::x(); // q1
    let e2 = Vector4::y(); // q2
    let e3 = Vector4::z(); // p1
    let e4 = Vector4::w(); // p2

    // ω₀(q1, p1) = -1  (since J₀ q1 = -p1, and ⟨-p1, p1⟩... wait)
    // J₀ e1 = (0,0,1,0) = e3. So ω₀(e1, e3) = ⟨e3, e3⟩ = 1? No.
    // Let me compute: J₀ = [[0,-I],[I,0]], so J₀ (1,0,0,0) = (0,0,1,0).
    // ω₀(e1, e3) = ⟨J₀ e1, e3⟩ = ⟨(0,0,1,0), (0,0,1,0)⟩ = 1.
    // But in standard symplectic: ω₀ = dq1∧dp1 + dq2∧dp2.
    // ω₀(q1, p1) = 1. Yes that's correct.
    assert!((omega0(&e1, &e3) - 1.0).abs() < 1e-15);  // ω₀(q1, p1) = 1
    assert!((omega0(&e2, &e4) - 1.0).abs() < 1e-15);  // ω₀(q2, p2) = 1
    assert!((omega0(&e1, &e2)).abs() < 1e-15);          // ω₀(q1, q2) = 0
    assert!((omega0(&e3, &e4)).abs() < 1e-15);          // ω₀(p1, p2) = 0
    assert!((omega0(&e1, &e4)).abs() < 1e-15);          // ω₀(q1, p2) = 0
    assert!((omega0(&e3, &e1) + 1.0).abs() < 1e-15);   // ω₀(p1, q1) = -1
}

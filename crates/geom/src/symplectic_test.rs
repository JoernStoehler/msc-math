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

    // ω₀ = dq1∧dp1 + dq2∧dp2 (standard symplectic form)
    assert!((omega0(&e1, &e3) - 1.0).abs() < 1e-15);  // ω₀(q1, p1) = 1
    assert!((omega0(&e2, &e4) - 1.0).abs() < 1e-15);  // ω₀(q2, p2) = 1
    assert!((omega0(&e1, &e2)).abs() < 1e-15);          // ω₀(q1, q2) = 0
    assert!((omega0(&e3, &e4)).abs() < 1e-15);          // ω₀(p1, p2) = 0
    assert!((omega0(&e1, &e4)).abs() < 1e-15);          // ω₀(q1, p2) = 0
    assert!((omega0(&e3, &e1) + 1.0).abs() < 1e-15);   // ω₀(p1, q1) = -1
}

#[test]
fn omega0_formula_equals_matrix_form() {
    // Prove that the direct formula u[0]*v[2] - u[2]*v[0] + u[1]*v[3] - u[3]*v[1]
    // is equivalent to the matrix form ⟨J₀ u, v⟩.
    let test_cases = vec![
        (Vector4::new(1.0, 0.0, 0.0, 0.0), Vector4::new(0.0, 0.0, 1.0, 0.0)),
        (Vector4::new(0.0, 1.0, 0.0, 0.0), Vector4::new(0.0, 0.0, 0.0, 1.0)),
        (Vector4::new(1.0, 2.0, 3.0, 4.0), Vector4::new(5.0, 6.0, 7.0, 8.0)),
        (Vector4::new(-1.5, 2.3, -0.7, 4.2), Vector4::new(3.1, -2.8, 1.9, -0.4)),
        (Vector4::new(0.0, 0.0, 0.0, 0.0), Vector4::new(1.0, 2.0, 3.0, 4.0)),
    ];

    for (u, v) in test_cases {
        let formula_result = u[0] * v[2] - u[2] * v[0] + u[1] * v[3] - u[3] * v[1];
        let matrix_result = (j4() * u).dot(&v);
        let omega_result = omega0(&u, &v);

        assert!(
            (formula_result - matrix_result).abs() < 1e-14,
            "Formula vs matrix: u={:?}, v={:?}, formula={}, matrix={}",
            u,
            v,
            formula_result,
            matrix_result
        );
        assert!(
            (omega_result - matrix_result).abs() < 1e-14,
            "omega0 vs matrix: u={:?}, v={:?}, omega={}, matrix={}",
            u,
            v,
            omega_result,
            matrix_result
        );
    }
}

#[cfg(test)]
mod proptests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        /// Property test: ω₀ antisymmetry ω₀(u, v) = -ω₀(v, u)
        #[test]
        fn omega0_is_antisymmetric(
            u in prop::array::uniform4(-10.0f64..10.0f64),
            v in prop::array::uniform4(-10.0f64..10.0f64)
        ) {
            let u_vec = Vector4::from(u);
            let v_vec = Vector4::from(v);

            let omega_uv = omega0(&u_vec, &v_vec);
            let omega_vu = omega0(&v_vec, &u_vec);

            // Tolerance scaled by magnitude (floating point precision)
            let tol = 1e-13 * u_vec.norm() * v_vec.norm();

            prop_assert!(
                (omega_uv + omega_vu).abs() < tol,
                "antisymmetry failed: ω₀(u,v)={}, ω₀(v,u)={}, sum={}, tol={}",
                omega_uv,
                omega_vu,
                omega_uv + omega_vu,
                tol
            );
        }
    }
}

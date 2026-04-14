//! Standard symplectic form and J_0 matrix for R^4.
//!
//! Provides the symplectic geometry primitives used throughout the crate:
//! the standard symplectic matrix J_0, its 2D counterpart J_2, and the
//! standard symplectic form omega_0(u, v) = <J_0 u, v>.
//!
//! Coordinate convention: (q_1, q_2, p_1, p_2), where q = position and p = momentum.
//! This convention is used consistently throughout the crate.
//!
//! Mathematical correspondence: [def:symplectic-form], [def:J0]

use nalgebra::{Matrix2, Matrix4, Vector4};

/// The standard symplectic matrix J_2 in R^2: [[0, -1], [1, 0]].
///
/// Satisfies J_2^2 = -I_2.
///
/// Mathematical correspondence: [def:J0] (2D case)
pub fn j2() -> Matrix2<f64> {
    Matrix2::new(0.0, -1.0, 1.0, 0.0)
}

/// The standard symplectic matrix J_0 in R^4 (coordinates q_1, q_2, p_1, p_2).
///
/// Block form: J_0 = [[0, -I_2], [I_2, 0]], satisfying J_0^2 = -I_4.
/// The symplectic form is omega_0(u, v) = <J_0 u, v>.
///
/// Mathematical correspondence: [def:J0]
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

/// Standard symplectic form: omega_0(u, v) = <J_0 u, v>.
///
/// In coordinates (q_1, q_2, p_1, p_2), this expands to:
///   omega_0(u, v) = u_q1 * v_p1 - u_p1 * v_q1 + u_q2 * v_p2 - u_p2 * v_q2
///
/// Implements the coordinate expansion directly for performance (no matrix multiply).
///
/// # Properties
///
/// - **Antisymmetric**: omega_0(u, v) = -omega_0(v, u)
/// - **Bilinear**: linear in each argument
/// - **Non-degenerate**: if omega_0(u, v) = 0 for all v, then u = 0
///
/// Mathematical correspondence: [def:symplectic-form]
pub fn omega0(u: &Vector4<f64>, v: &Vector4<f64>) -> f64 {
    u[0] * v[2] - u[2] * v[0] + u[1] * v[3] - u[3] * v[1]
}

#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra::{Matrix2, Matrix4, Vector4};

    // Tests for symplectic_form: omega_0 properties and J_0 matrix identities.
    //
    // Proposition: omega_0(u, v) = -omega_0(v, u) (antisymmetry), J_0^2 = -I_4,
    // omega_0(e_qi, e_pi) = 1 (canonical basis pairing), and the coordinate formula
    // agrees with the matrix form <J_0 u, v>.
    //
    // Strategy: fixture-based (basis vectors, specific vectors) + proptest 256 cases

    /// Verify J_2^2 = -I_2.
    #[test]
    fn j2_squared_is_minus_identity() {
        let j = j2();
        let j_sq = j * j;
        assert_eq!(j_sq, -Matrix2::identity());
    }

    /// Verify J_4^2 = -I_4 (complex structure property).
    #[test]
    fn j4_squared_is_minus_identity() {
        let j = j4();
        let j_sq = j * j;
        assert!(
            (j_sq + Matrix4::identity()).norm() < 1e-15,
            "J_0^2 + I should be zero, norm = {}",
            (j_sq + Matrix4::identity()).norm()
        );
    }

    /// Verify omega_0(u, v) + omega_0(v, u) = 0 (antisymmetry).
    #[test]
    fn omega0_antisymmetric() {
        let u = Vector4::new(1.0, 2.0, 3.0, 4.0);
        let v = Vector4::new(5.0, 6.0, 7.0, 8.0);
        assert!(
            (omega0(&u, &v) + omega0(&v, &u)).abs() < 1e-15,
            "omega_0(u,v) + omega_0(v,u) = {}",
            omega0(&u, &v) + omega0(&v, &u)
        );
    }

    /// Verify canonical basis pairings: omega_0(q_i, p_i) = 1 and cross-terms vanish.
    #[test]
    fn omega0_basis_vectors() {
        let e1 = Vector4::x(); // q_1
        let e2 = Vector4::y(); // q_2
        let e3 = Vector4::z(); // p_1
        let e4 = Vector4::w(); // p_2

        // Canonical pairings: omega_0 = dq_1 ^ dp_1 + dq_2 ^ dp_2
        assert!((omega0(&e1, &e3) - 1.0).abs() < 1e-15); // omega_0(q_1, p_1) = 1
        assert!((omega0(&e2, &e4) - 1.0).abs() < 1e-15); // omega_0(q_2, p_2) = 1
        assert!(omega0(&e1, &e2).abs() < 1e-15); // omega_0(q_1, q_2) = 0
        assert!(omega0(&e3, &e4).abs() < 1e-15); // omega_0(p_1, p_2) = 0
        assert!(omega0(&e1, &e4).abs() < 1e-15); // omega_0(q_1, p_2) = 0
        assert!((omega0(&e3, &e1) + 1.0).abs() < 1e-15); // omega_0(p_1, q_1) = -1
    }

    /// Verify omega_0(u, u) = 0 for a specific vector (consequence of antisymmetry).
    #[test]
    fn omega0_self_pairing_is_zero() {
        // omega_0(u, u) = 0 for all u (consequence of antisymmetry)
        let u = Vector4::new(3.15, -2.71, 1.41, 0.57);
        assert!(
            omega0(&u, &u).abs() < 1e-14,
            "omega_0(u, u) should be 0, got {}",
            omega0(&u, &u)
        );
    }

    /// Verify the coordinate formula agrees with the matrix form <J_0 u, v>.
    #[test]
    fn omega0_formula_equals_matrix_form() {
        // Verify: the direct formula u[0]*v[2] - u[2]*v[0] + u[1]*v[3] - u[3]*v[1]
        // is equivalent to the matrix form <J_0 u, v>.
        let test_cases = [
            (Vector4::x(), Vector4::z()),
            (Vector4::y(), Vector4::w()),
            (
                Vector4::new(1.0, 2.0, 3.0, 4.0),
                Vector4::new(5.0, 6.0, 7.0, 8.0),
            ),
            (
                Vector4::new(-1.5, 2.3, -0.7, 4.2),
                Vector4::new(3.1, -2.8, 1.9, -0.4),
            ),
            (Vector4::zeros(), Vector4::new(1.0, 2.0, 3.0, 4.0)),
        ];

        let j = j4();
        for (u, v) in &test_cases {
            let matrix_result = (j * u).dot(v);
            let omega_result = omega0(u, v);

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

    /// Lagrangian subspace check: omega_0 vanishes on {q_1, q_2} and on {p_1, p_2}.
    #[test]
    fn lagrangian_subspace_q_plane() {
        let e1 = Vector4::x(); // q_1
        let e2 = Vector4::y(); // q_2
        assert!(
            omega0(&e1, &e2).abs() < 1e-15,
            "q-plane should be Lagrangian: omega_0(q_1, q_2) = {}",
            omega0(&e1, &e2)
        );
    }

    /// Verify omega_0 vanishes on the p-plane {p_1, p_2} (Lagrangian subspace).
    #[test]
    fn lagrangian_subspace_p_plane() {
        let e3 = Vector4::z(); // p_1
        let e4 = Vector4::w(); // p_2
        assert!(
            omega0(&e3, &e4).abs() < 1e-15,
            "p-plane should be Lagrangian: omega_0(p_1, p_2) = {}",
            omega0(&e3, &e4)
        );
    }

    #[cfg(test)]
    mod proptests {
        use super::*;
        use proptest::prelude::*;

        proptest! {
            /// Property: omega_0(u, v) = -omega_0(v, u) for all u, v.
            #[test]
            fn omega0_is_antisymmetric(
                u in prop::array::uniform4(-10.0f64..10.0f64),
                v in prop::array::uniform4(-10.0f64..10.0f64)
            ) {
                let u_vec = Vector4::from(u);
                let v_vec = Vector4::from(v);

                let omega_uv = omega0(&u_vec, &v_vec);
                let omega_vu = omega0(&v_vec, &u_vec);

                // Tolerance scaled by magnitude (floating-point precision)
                let tol = 1e-13 * u_vec.norm() * v_vec.norm();

                prop_assert!(
                    (omega_uv + omega_vu).abs() < tol.max(1e-15),
                    "antisymmetry failed: omega_0(u,v)={}, omega_0(v,u)={}, sum={}",
                    omega_uv, omega_vu, omega_uv + omega_vu
                );
            }

            /// Property: omega_0(u, u) = 0 for all u (consequence of antisymmetry).
            #[test]
            fn omega0_self_pairing_zero(
                u in prop::array::uniform4(-10.0f64..10.0f64)
            ) {
                let u_vec = Vector4::from(u);
                let result = omega0(&u_vec, &u_vec);

                prop_assert!(
                    result.abs() < 1e-13,
                    "omega_0(u, u) should be 0, got {}",
                    result
                );
            }
        }
    }
}

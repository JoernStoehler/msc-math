//! HK2017 symplectic invariance tests.
//!
//! Split from mod.rs to keep module routing and docs short.

use crate::algorithms::test_helpers::{
    unpruned_capacity_for_dual_vertices, unpruned_capacity_for_fixture,
};
use nalgebra::Matrix4;

// ── Symplectomorphism invariance ──

/// Verify c(MK) = c(K) on the simplex for a symplectic rotation.
///
/// This is a small live smoke test for the current HK2017 implementation.
/// Broad symplectic-invariance validation lives in
/// `experiments/verification/correctness/`.
#[test]
fn capacity_symplectomorphism_invariance_simplex() {
    let kp = crate::geom::known_polytopes::simplex();
    let theta = 0.37_f64;
    let m = rotate_q1_p1(theta);
    let transformed = apply_symplectic_linear_map(&kp.dual_vertices_f64, &m);

    let base_cap = unpruned_capacity_for_fixture(kp)
        .expect("simplex capacity")
        .capacity();
    let transformed_cap = unpruned_capacity_for_dual_vertices(&transformed)
        .expect("transformed simplex capacity")
        .capacity();
    let relative_error = ((transformed_cap - base_cap) / base_cap).abs();
    assert!(
        relative_error < 1e-6,
        "simplex symplectic invariance failed: base_cap={base_cap}, \
         transformed_cap={transformed_cap}, relative_error={relative_error}"
    );
}

fn rotate_q1_p1(theta: f64) -> Matrix4<f64> {
    let c = theta.cos();
    let s = theta.sin();
    Matrix4::new(
        c, 0.0, -s, 0.0, 0.0, 1.0, 0.0, 0.0, s, 0.0, c, 0.0, 0.0, 0.0, 0.0, 1.0,
    )
}

fn apply_symplectic_linear_map(
    dual_vertices: &[nalgebra::Vector4<f64>],
    m: &Matrix4<f64>,
) -> Vec<nalgebra::Vector4<f64>> {
    let m_inv_t = m
        .transpose()
        .try_inverse()
        .expect("symplectic map should be invertible");
    dual_vertices.iter().map(|a| m_inv_t * a).collect()
}

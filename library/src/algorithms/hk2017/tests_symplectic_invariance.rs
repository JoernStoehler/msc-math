//! HK2017 symplectic invariance tests.
//!
//! Split from mod.rs to keep module routing and docs short.

use crate::geom::polytope::Polytope4D;
use nalgebra::Matrix4;
use crate::ehz_capacity_unpruned;

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
    let transformed = apply_symplectic_linear_map(&kp.polytope, &m);

    let base_cap = ehz_capacity_unpruned(&kp.polytope)
        .expect("simplex capacity")
        .capacity();
    let transformed_cap = ehz_capacity_unpruned(&transformed)
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

fn apply_symplectic_linear_map(polytope: &Polytope4D, m: &Matrix4<f64>) -> Polytope4D {
    let m_inv_t = m
        .transpose()
        .try_inverse()
        .expect("symplectic map should be invertible");
    Polytope4D::from_f64(
        polytope
            .dual_vertices_f64()
            .iter()
            .map(|a| m_inv_t * a)
            .collect(),
    )
    .expect("transformed polytope")
}

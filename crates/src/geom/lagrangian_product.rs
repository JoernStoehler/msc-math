//! Construct 4D Lagrangian products from pairs of 2D convex polygons.
//!
//! A Lagrangian product P x_L Q places polygon P in q-space (q_1, q_2)
//! and polygon Q in p-space (p_1, p_2). The 4D polytope has facets from both
//! factors, with normals embedded into the respective Lagrangian subspaces:
//!
//! - P-facets (Q-type): n = (n_P, 0, 0) in R^4 (components [0,1])
//! - Q-facets (P-type): n = (0, 0, n_Q) in R^4 (components [2,3])
//!
//! Coordinates: (q_1, q_2, p_1, p_2). See `symplectic_form` module for J_0 and omega_0.
//!
//! Volume: vol_4(P x_L Q) = area(P) * area(Q) (Fubini's theorem on
//! complementary Lagrangian subspaces).
//!
//! Mathematical correspondence: [def:lagrangian-product]

use crate::geom::polytope::{ConstructionError, Polytope4D};
use nalgebra::{Vector2, Vector4};

/// Build a 4D Lagrangian product from two 2D polygons.
///
/// `q_normals`/`q_heights`: polygon P in q-space (q_1, q_2).
/// `p_normals`/`p_heights`: polygon Q in p-space (p_1, p_2).
///
/// Embeds each 2D normal into 4D and constructs via `Polytope4D::new`.
/// Requires `q_normals.len() + p_normals.len() >= 5` (Polytope4D minimum).
///
/// Mathematical correspondence: [def:lagrangian-product]
pub fn lagrangian_product(
    q_normals: &[Vector2<f64>],
    q_heights: &[f64],
    p_normals: &[Vector2<f64>],
    p_heights: &[f64],
) -> Result<Polytope4D, ConstructionError> {
    // Dual vertex representation: a_i = n_i / h_i, embedded in 4D
    let halfspaces: Vec<Vector4<f64>> = q_normals
        .iter()
        .zip(q_heights.iter())
        .map(|(n, &h)| Vector4::new(n[0], n[1], 0.0, 0.0) / h)
        .chain(
            p_normals
                .iter()
                .zip(p_heights.iter())
                .map(|(n, &h)| Vector4::new(0.0, 0.0, n[0], n[1]) / h),
        )
        .collect();

    Polytope4D::new(halfspaces)
}

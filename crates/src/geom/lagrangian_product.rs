/// Construct 4D Lagrangian products from pairs of 2D convex polygons.
///
/// A **Lagrangian product** P ×_L Q places polygon P in q-space (q₁, q₂)
/// and polygon Q in p-space (p₁, p₂). The 4D polytope has facets from both
/// factors, with normals embedded into the respective Lagrangian subspaces:
///
/// - P-facets: n̂ = (n_P, 0, 0) in ℝ⁴ (components \[0,1\])
/// - Q-facets: n̂ = (0, 0, n_Q) in ℝ⁴ (components \[2,3\])
///
/// **Coordinates**: (q₁, q₂, p₁, p₂). See `symplectic` module for J₀ and ω₀.
///
/// **Volume**: vol₄(P ×_L Q) = area(P) · area(Q) (Fubini's theorem on
/// complementary subspaces).
use crate::geom::polytope::{ConstructionError, Polytope4D};
use nalgebra::{Vector2, Vector4};

/// Build a 4D Lagrangian product from two 2D polygons.
///
/// `q_normals`/`q_heights`: polygon P in q-space (q₁, q₂).
/// `p_normals`/`p_heights`: polygon Q in p-space (p₁, p₂).
///
/// Requires `q_normals.len() + p_normals.len() >= 5` (Polytope4D minimum).
pub fn lagrangian_product(
    q_normals: &[Vector2<f64>],
    q_heights: &[f64],
    p_normals: &[Vector2<f64>],
    p_heights: &[f64],
) -> Result<Polytope4D, ConstructionError> {
    let normals_4d: Vec<Vector4<f64>> = q_normals
        .iter()
        .map(|n| Vector4::new(n[0], n[1], 0.0, 0.0))
        .chain(p_normals.iter().map(|n| Vector4::new(0.0, 0.0, n[0], n[1])))
        .collect();

    let heights: Vec<f64> = q_heights.iter().chain(p_heights.iter()).copied().collect();

    Polytope4D::new(normals_4d, heights)
}

#[cfg(test)]
#[path = "lagrangian_product_test.rs"]
mod lagrangian_product_test;

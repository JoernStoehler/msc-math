//! Vertex enumeration for 4D polytopes via qhull subprocess.
//!
//! See Definition 3.2 (Polytope) in thesis/chapter-algorithm.tex:
//!   A polytope is a bounded convex subset K ⊂ ℝ⁴ that contains 0 in its
//!   interior and is an intersection of finitely many closed half-spaces.
//!   An irredundant H-representation is {(n̂ᵢ, ĥᵢ)}ᵢ₌₁^F with n̂ᵢ ∈ S³, ĥᵢ > 0,
//!   such that K = ⋂ᵢ { x : ⟨x, n̂ᵢ⟩ ≤ ĥᵢ }.

use nalgebra::Vector4;

/// Enumerate all vertices of a polytope given its H-representation.
///
/// Uses `qhalf` subprocess for robust vertex enumeration.
///
/// Used internally by [`Polytope4D::new()`](crate::polytope::Polytope4D::new) to precompute vertices at
/// construction time. External callers should use [`Polytope4D::vertices()`](crate::polytope::Polytope4D::vertices).
///
/// See Definition 3.2 (Polytope), Definition 3.4 (Facets) in thesis/chapter-algorithm.tex.
pub(crate) fn compute_vertices(
    normals: &[Vector4<f64>],
    heights: &[f64],
) -> Result<Vec<Vector4<f64>>, crate::qhull::QhullError> {
    crate::qhull::halfspace_intersection_4d(normals, heights)
}

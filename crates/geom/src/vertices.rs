/// Vertex enumeration for 4D polytopes.
///
/// Enumerate all vertices of a polytope K = { x | n_i · x ≤ h_i }.
///
/// A vertex is a point where exactly 4 (or more) linearly independent
/// constraints are active. We find candidates by solving all C(F,4) systems
/// of 4 equations, then keep only feasible points.
use nalgebra::{Matrix4, Vector4};

const EPS_FEASIBILITY: f64 = 1e-8;
const EPS_DEDUP: f64 = 1e-8;

/// Enumerate all vertices of a polytope given its H-representation.
///
/// Used internally by `Polytope4D::new()` to precompute vertices at
/// construction time. External callers should use `Polytope4D::vertices()`.
pub(crate) fn compute_vertices(
    normals: &[Vector4<f64>],
    heights: &[f64],
) -> Vec<Vector4<f64>> {
    let f = normals.len();

    let candidates = (0..f)
        .flat_map(|i| (i + 1..f).map(move |j| (i, j)))
        .flat_map(|(i, j)| (j + 1..f).map(move |k| (i, j, k)))
        .flat_map(|(i, j, k)| (k + 1..f).map(move |l| (i, j, k, l)))
        .filter_map(|(i, j, k, l)| {
            let mat = Matrix4::from_rows(&[
                normals[i].transpose(),
                normals[j].transpose(),
                normals[k].transpose(),
                normals[l].transpose(),
            ]);
            let rhs = Vector4::new(heights[i], heights[j], heights[k], heights[l]);
            mat.lu().solve(&rhs)
        })
        .filter(|x| (0..f).all(|m| normals[m].dot(x) <= heights[m] + EPS_FEASIBILITY));

    // Deduplicate: fold keeps only vertices not yet seen
    candidates.fold(Vec::new(), |mut acc, x| {
        if !acc.iter().any(|v: &Vector4<f64>| (v - x).norm() < EPS_DEDUP) {
            acc.push(x);
        }
        acc
    })
}

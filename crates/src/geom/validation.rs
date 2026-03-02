/// Polytope validation: boundedness and irredundancy checks for R^4 polytopes.
///
/// These checks enforce the mathematical definition of a polytope in H-representation:
/// - **Bounded**: normals positively span R^4 (recession cone = {0})
/// - **Irredundant**: each facet has incident vertices of affine rank 3
///
/// Both checks are called from `Polytope4D::new()` to enforce invariants at construction.
use crate::constants::EPS_FACET_INCIDENCE;
use crate::geom::cross_product::cross_product_4d;
use nalgebra::Vector4;

/// Threshold for positive-span check: n_ℓ · d > EPS means "has positive component."
///
/// **Why 1e-9:** Same as polytope.rs EPS_UNIT. The cross-product direction d is
/// computed from 3 unit normals, so ‖d‖ = O(1). Inner products n_ℓ · d near
/// zero indicate the normal is nearly orthogonal to d; 1e-9 distinguishes
/// genuine positive components from floating-point noise.
const EPS_UNIT: f64 = 1e-9;

/// Check that the normals positively span R^4, i.e., the polytope is bounded.
///
/// # Mathematical Background
///
/// K bounded ⟺ rec(K) = {0} ⟺ normals positively span R^4, where
/// "positively span" means: for every nonzero d, some n_ℓ · d > 0.
///
/// # Algorithm
///
/// 1. Check rank(normals) = 4.
/// 2. For each triple (i,j,k), compute 1D kernel d via 4D cross product.
///    Verify some normal outside {i,j,k} has positive and some has negative
///    inner product with d.
///
/// Complexity: O(F³).
pub fn check_bounded(normals: &[Vector4<f64>]) -> bool {
    let f = normals.len();

    // Rank check: normals must span R^4.
    let mat = nalgebra::DMatrix::from_fn(f, 4, |r, c| normals[r][c]);
    let svd = mat.svd(false, false);
    let rank = svd.singular_values.iter().filter(|&&s| s > 1e-8).count();
    if rank < 4 {
        return false;
    }

    for i in 0..f {
        for j in (i + 1)..f {
            for k in (j + 1)..f {
                let d = cross_product_4d(normals[i], normals[j], normals[k]);
                if d.norm() < 1e-12 {
                    continue; // dependent triple
                }
                let d = d.normalize();

                let has_pos = (0..f)
                    .filter(|&l| l != i && l != j && l != k)
                    .any(|l| normals[l].dot(&d) > EPS_UNIT);
                let has_neg = (0..f)
                    .filter(|&l| l != i && l != j && l != k)
                    .any(|l| normals[l].dot(&d) < -EPS_UNIT);

                if !has_pos || !has_neg {
                    return false;
                }
            }
        }
    }
    true
}

/// Check that every facet is irredundant: its incident vertices affinely span
/// a 3D subspace (the facet hyperplane).
///
/// Returns `Some(i)` if facet `i` is redundant, `None` if all are irredundant.
pub fn find_redundant_facet(
    normals: &[Vector4<f64>],
    heights: &[f64],
    vertices: &[Vector4<f64>],
) -> Option<usize> {
    let f = normals.len();

    for i in 0..f {
        // Collect vertices incident to facet i: n_i · v ≈ h_i
        let incident: Vec<Vector4<f64>> = vertices
            .iter()
            .filter(|v| (normals[i].dot(v) - heights[i]).abs() < EPS_FACET_INCIDENCE)
            .cloned()
            .collect();

        if incident.is_empty() {
            return Some(i);
        }

        if affine_rank(&incident) < 3 {
            return Some(i);
        }
    }
    None
}

/// Compute the affine rank of a set of points.
/// Affine rank = dimension of their affine span = rank of centered points matrix.
pub fn affine_rank(points: &[Vector4<f64>]) -> usize {
    if points.len() <= 1 {
        return 0;
    }

    let base = points[0];
    let centered: Vec<Vector4<f64>> = points[1..].iter().map(|p| p - base).collect();

    let n = centered.len();
    let mat = nalgebra::DMatrix::from_fn(n, 4, |r, c| centered[r][c]);

    let svd = mat.svd(false, false);
    let threshold = 1e-8;
    svd.singular_values
        .iter()
        .filter(|&&s| s > threshold)
        .count()
}

#[cfg(test)]
#[path = "validation_test.rs"]
mod validation_test;

/// Polytope validation: boundedness check for R^4 polytopes.
///
/// Enforces: **Bounded** — normals positively span R^4 (recession cone = {0}).
/// Called from `Polytope4D::new()` as a fast-fail pre-filter before the
/// expensive rational pipeline (which also checks boundedness exactly).
///
/// Irredundancy is checked exactly by the rational pipeline in rational.rs.
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

#[cfg(test)]
#[path = "validation_test.rs"]
mod validation_test;

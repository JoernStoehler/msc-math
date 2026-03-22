//! Polytope boundedness check for R^4 polytopes (f64 fast-fail pre-filter).
//!
//! Provides a floating-point boundedness check that runs before the expensive
//! exact rational pipeline. The rational pipeline in `vertex_enumeration.rs`
//! performs the authoritative boundedness and irredundancy checks over Q.
//!
//! Mathematical correspondence: [lem:positive-span]

use crate::geom::cross_product_4d::cross_product_4d;
use nalgebra::Vector4;

/// Threshold for positive-span check: n_l · d > EPS means "has positive component."
///
/// **Why 1e-9:** The cross-product direction d is computed from 3 unit normals,
/// so ||d|| = O(1). Inner products n_l · d near zero indicate the normal is
/// nearly orthogonal to d; 1e-9 distinguishes genuine positive components from
/// floating-point noise.
const EPS_UNIT: f64 = 1e-9;

/// Check that the normals positively span R^4, i.e., the polytope is bounded.
///
/// K bounded iff rec(K) = {0} iff normals positively span R^4, where
/// "positively span" means: for every nonzero d, some n_l · d > 0.
///
/// # Algorithm
///
/// 1. Check rank(normals) = 4 via SVD.
/// 2. For each triple (i,j,k), compute the 1D kernel direction d via 4D cross
///    product. Verify some normal outside {i,j,k} has positive and some has
///    negative inner product with d.
///
/// Complexity: O(F^3) where F is the number of facets.
///
/// Mathematical correspondence: [lem:positive-span]
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
mod tests {
    use super::*;

    // Tests for validation: boundedness accept/reject on known polytopes.
    //
    // Proposition: check_bounded correctly classifies bounded vs unbounded normal sets.
    // Reference: [lem:positive-span]
    //
    // Strategy: fixture-based on simplex, hypercube, and adversarial unbounded configurations.

    /// Helper: simplex normals (5 facets, origin at centroid).
    fn simplex_normals() -> Vec<Vector4<f64>> {
        vec![
            -Vector4::x(),
            -Vector4::y(),
            -Vector4::z(),
            -Vector4::w(),
            Vector4::new(1.0, 1.0, 1.0, 1.0).normalize(),
        ]
    }

    /// Helper: hypercube [-1,1]^4 normals (8 facets).
    fn hypercube_normals() -> Vec<Vector4<f64>> {
        vec![
            Vector4::x(),
            -Vector4::x(),
            Vector4::y(),
            -Vector4::y(),
            Vector4::z(),
            -Vector4::z(),
            Vector4::w(),
            -Vector4::w(),
        ]
    }

    // ---- Boundedness ----

    /// Verify simplex normals form a positively spanning set (bounded polytope).
    #[test]
    fn simplex_is_bounded() {
        assert!(check_bounded(&simplex_normals()));
    }

    /// Verify hypercube normals form a positively spanning set (bounded polytope).
    #[test]
    fn hypercube_is_bounded() {
        assert!(check_bounded(&hypercube_normals()));
    }

    /// Verify check_bounded rejects normals all pointing in roughly +x direction.
    #[test]
    fn unbounded_normals_detected() {
        // All normals point roughly in the +x direction: fails positive spanning.
        let normals = vec![
            Vector4::new(1.0, 0.1, 0.0, 0.0).normalize(),
            Vector4::new(1.0, -0.1, 0.0, 0.0).normalize(),
            Vector4::new(1.0, 0.0, 0.1, 0.0).normalize(),
            Vector4::new(1.0, 0.0, -0.1, 0.0).normalize(),
            Vector4::new(1.0, 0.0, 0.0, 0.1).normalize(),
        ];
        assert!(!check_bounded(&normals));
    }

    /// Verify check_bounded rejects rank-deficient normals (rank < 4).
    #[test]
    fn rank_deficient_normals_unbounded() {
        // Only 3 linearly independent directions: rank < 4.
        let normals = vec![
            Vector4::x(),
            -Vector4::x(),
            Vector4::y(),
            -Vector4::y(),
            Vector4::z(),
        ];
        assert!(!check_bounded(&normals));
    }
}

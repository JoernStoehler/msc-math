//! Lagrangian product validation and facet classification.
//!
//! A Lagrangian product K = K_q x_L K_p has facets of two types:
//! - q-facets: normal = (n_q, 0), n_q in R^2 (components [0,1])
//! - p-facets: normal = (0, n_p), n_p in R^2 (components [2,3])
//!
//! Classification fails (returns `BilliardError`) if any facet normal has both
//! q and p components, or if there are fewer than 3 facets of either type
//! (the minimum for a polygon).
//!
//! Mathematical correspondence: [lem:lagrangian-facets]

use super::BilliardError;
use nalgebra::Vector4;

/// Tolerance for classifying facet dual vertices as q-type or p-type.
///
/// A facet is q-type if ||(a[2], a[3])||^2 / ||a||^2 < EPS (normal direction lies in L_q),
/// p-type if ||(a[0], a[1])||^2 / ||a||^2 < EPS (normal direction lies in L_p).
/// For exact Lagrangian products, the "other" component pair is exactly zero.
/// The 1e-10 threshold is well above machine epsilon squared but far below any
/// genuine mixed normal.
const EPS_LAGRANGIAN_NORMAL: f64 = 1e-10;

/// Classification of facets into q-type and p-type.
///
/// [lem:lagrangian-facets]: every facet of a Lagrangian product K_q x_L K_p
/// is either a q-facet or a p-facet.
#[derive(Debug, Clone)]
pub struct FacetClassification {
    /// Indices of q-facets (normals in the Lagrangian subspace L_q).
    pub q_indices: Vec<usize>,
    /// Indices of p-facets (normals in the Lagrangian subspace L_p).
    pub p_indices: Vec<usize>,
}

impl FacetClassification {
    /// Returns the billiard bounce count `k` if `sigma` has valid alternating
    /// q/p block structure for this classification, and `None` otherwise.
    ///
    /// A valid billiard sigma has the form `Q_1 P_1 ... Q_k P_k` where each
    /// block has length 1 or 2 and contains only facet indices of the
    /// corresponding type.
    pub fn bounce_count_for_sigma(&self, sigma: &[usize]) -> Option<usize> {
        if sigma.is_empty() {
            return None;
        }

        let mut i = 0usize;
        let mut expect_q = true;
        let mut q_blocks = 0usize;
        let mut p_blocks = 0usize;

        while i < sigma.len() {
            let is_expected_type = |idx: usize| {
                if expect_q {
                    self.q_indices.contains(&idx)
                } else {
                    self.p_indices.contains(&idx)
                }
            };

            if !is_expected_type(sigma[i]) {
                return None;
            }

            let mut block_len = 1usize;
            if i + 1 < sigma.len() && is_expected_type(sigma[i + 1]) {
                block_len += 1;
                if i + 2 < sigma.len() && is_expected_type(sigma[i + 2]) {
                    return None;
                }
            }

            if expect_q {
                q_blocks += 1;
            } else {
                p_blocks += 1;
            }

            expect_q = !expect_q;
            i += block_len;
        }

        (q_blocks > 0 && q_blocks == p_blocks).then_some(q_blocks)
    }

    /// Mask a dual-vertex direction in place so it preserves Lagrangian product
    /// structure.
    ///
    /// q-facets may move only in q-components; p-facets may move only in
    /// p-components.
    pub fn mask_dual_direction_in_place(&self, direction: &mut [Vector4<f64>]) {
        debug_assert_eq!(direction.len(), self.q_indices.len() + self.p_indices.len());

        for &idx in &self.q_indices {
            if let Some(slot) = direction.get_mut(idx) {
                slot[2] = 0.0;
                slot[3] = 0.0;
            }
        }
        for &idx in &self.p_indices {
            if let Some(slot) = direction.get_mut(idx) {
                slot[0] = 0.0;
                slot[1] = 0.0;
            }
        }
    }

    /// Return an LP-preserving masked copy of a dual direction.
    pub fn masked_dual_direction(&self, direction: &[Vector4<f64>]) -> Vec<Vector4<f64>> {
        let mut masked = direction.to_vec();
        self.mask_dual_direction_in_place(&mut masked);
        masked
    }
}

/// Classify flat dual vertices into q-type and p-type facets.
///
/// Input contract: `dual_vertices` is the ordered facet-dual list for the same
/// Lagrangian-product polytope that later billiard enumeration will solve.
/// Returns error if any facet normal is neither purely q-type nor purely
/// p-type, or if there are fewer than 3 facets of either type.
///
/// [lem:lagrangian-facets]: classification criterion for Lagrangian product facets.
pub fn classify_facets_from_dual_vertices(
    dual_vertices: &[Vector4<f64>],
) -> Result<FacetClassification, BilliardError> {
    let mut q_indices = Vec::new();
    let mut p_indices = Vec::new();

    for (i, a) in dual_vertices.iter().enumerate() {
        let norm_sq = a[0] * a[0] + a[1] * a[1] + a[2] * a[2] + a[3] * a[3];
        let q_norm_sq = a[0] * a[0] + a[1] * a[1];
        let p_norm_sq = a[2] * a[2] + a[3] * a[3];

        if p_norm_sq / norm_sq < EPS_LAGRANGIAN_NORMAL {
            // Normal direction is (n_q, 0): q-type.
            q_indices.push(i);
        } else if q_norm_sq / norm_sq < EPS_LAGRANGIAN_NORMAL {
            // Normal direction is (0, n_p): p-type.
            p_indices.push(i);
        } else {
            let n = a / a.norm();
            return Err(BilliardError::NotLagrangianProduct {
                facet: i,
                normal: [n[0], n[1], n[2], n[3]],
            });
        }
    }

    if q_indices.len() < 3 {
        return Err(BilliardError::TooFewFacets {
            facet_type: "q",
            count: q_indices.len(),
        });
    }
    if p_indices.len() < 3 {
        return Err(BilliardError::TooFewFacets {
            facet_type: "p",
            count: p_indices.len(),
        });
    }

    Ok(FacetClassification {
        q_indices,
        p_indices,
    })
}

/// Classify facets from the ordered facet-dual list.
#[cfg(test)]
pub(crate) fn classify_facets(
    dual_vertices: &[Vector4<f64>],
) -> Result<FacetClassification, BilliardError> {
    classify_facets_from_dual_vertices(dual_vertices)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geom::known_polytopes;

    #[test]
    fn mask_dual_direction_preserves_allowed_components() {
        let kp = known_polytopes::lagrangian_triangle_product();
        let classification = classify_facets(kp.polytope.dual_vertices_f64())
            .expect("triangle product should classify as a Lagrangian product");
        let direction: Vec<Vector4<f64>> = (0..kp.polytope.facet_count())
            .map(|i| {
                Vector4::new(
                    i as f64 + 1.0,
                    i as f64 + 11.0,
                    i as f64 + 21.0,
                    i as f64 + 31.0,
                )
            })
            .collect();

        let masked = classification.masked_dual_direction(&direction);
        let mut masked_in_place = direction.clone();
        classification.mask_dual_direction_in_place(&mut masked_in_place);

        assert_eq!(masked, masked_in_place);
        for &idx in &classification.q_indices {
            assert_eq!(masked[idx][0], direction[idx][0]);
            assert_eq!(masked[idx][1], direction[idx][1]);
            assert_eq!(masked[idx][2], 0.0);
            assert_eq!(masked[idx][3], 0.0);
        }
        for &idx in &classification.p_indices {
            assert_eq!(masked[idx][0], 0.0);
            assert_eq!(masked[idx][1], 0.0);
            assert_eq!(masked[idx][2], direction[idx][2]);
            assert_eq!(masked[idx][3], direction[idx][3]);
        }
    }
}

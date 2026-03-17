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

use crate::geom::polytope::Polytope4D;
use super::BilliardError;

/// Tolerance for classifying facet normals as q-type or p-type.
///
/// A facet is q-type if ||(n[2], n[3])||^2 < EPS, p-type if ||(n[0], n[1])||^2 < EPS.
/// Normals are unit vectors, so the "other" component pair squared is O(eps_machine^2)
/// for exact Lagrangian products. The 1e-10 threshold is well above machine epsilon
/// squared (~1e-32) but far below any genuine mixed normal.
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

/// Classify facets of a polytope into q-type and p-type.
///
/// Returns error if any facet normal is neither purely q-type nor purely p-type,
/// or if there are fewer than 3 facets of either type.
///
/// [lem:lagrangian-facets]: classification criterion for Lagrangian product facets.
pub fn classify_facets(polytope: &Polytope4D) -> Result<FacetClassification, BilliardError> {
    let normals = polytope.normals_f64();
    let mut q_indices = Vec::new();
    let mut p_indices = Vec::new();

    for (i, n) in normals.iter().enumerate() {
        let q_norm_sq = n[0] * n[0] + n[1] * n[1];
        let p_norm_sq = n[2] * n[2] + n[3] * n[3];

        if p_norm_sq < EPS_LAGRANGIAN_NORMAL {
            // Normal is (n_q, 0): q-type.
            q_indices.push(i);
        } else if q_norm_sq < EPS_LAGRANGIAN_NORMAL {
            // Normal is (0, n_p): p-type.
            p_indices.push(i);
        } else {
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

/// Lagrangian product validation and facet classification.
///
/// A Lagrangian product K = K_q ×_L K_p has facets of two types:
/// - q-facets: normal = (n_q, 0), n_q ∈ R² (components [0,1])
/// - p-facets: normal = (0, n_p), n_p ∈ R² (components [2,3])
///
/// See chapter-billiard.tex, Lemma 6.1 (lem:lagrangian-facets).
use crate::geom::polytope::Polytope4D;

use crate::BilliardError;

/// Tolerance for classifying facet normals as q-type or p-type.
const EPS_LAGRANGIAN_NORMAL: f64 = 1e-10;

/// Classification of facets into q-type and p-type.
#[derive(Debug, Clone)]
pub struct FacetClassification {
    /// Indices of q-facets (normals in L_q).
    pub q_indices: Vec<usize>,
    /// Indices of p-facets (normals in L_p).
    pub p_indices: Vec<usize>,
}

/// Classify facets of a polytope into q-type and p-type.
///
/// Returns error if any facet normal is neither purely q-type nor purely p-type.
pub fn classify_facets(polytope: &Polytope4D) -> Result<FacetClassification, BilliardError> {
    let normals = polytope.normals();
    let mut q_indices = Vec::new();
    let mut p_indices = Vec::new();

    for (i, n) in normals.iter().enumerate() {
        let q_norm_sq = n[0] * n[0] + n[1] * n[1];
        let p_norm_sq = n[2] * n[2] + n[3] * n[3];

        if p_norm_sq < EPS_LAGRANGIAN_NORMAL {
            // Normal is (n_q, 0): q-type
            q_indices.push(i);
        } else if q_norm_sq < EPS_LAGRANGIAN_NORMAL {
            // Normal is (0, n_p): p-type
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

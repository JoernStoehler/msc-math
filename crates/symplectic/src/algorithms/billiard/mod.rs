//! Billiard algorithm for EHZ capacity of Lagrangian product polytopes.
//!
//! Computes c_EHZ(K_q x_L K_p) for Lagrangian products where K_q, K_p are
//! convex polygons in R^2. This exploits the block structure of billiard orbits
//! in Lagrangian products to enumerate only O(n^3) candidate permutations
//! instead of the O(n!) of the general HK2017 algorithm.
//!
//! See [thm:billiard-characterization]: c_EHZ equals the minimum K_p-degree-length
//! billiard trajectory in K_q, and [thm:bounce-bound]: the minimiser has at most
//! 3 bounces.
//!
//! Submodules:
//! - `block_enumeration` — block structure enumeration for Q/P facets
//! - `facet_classification` — classify facets into q-space and p-space types
//! - `kkt_benchmark` — KKT solver performance measurement
//!
//! Mathematical correspondence: [thm:billiard-characterization], [thm:bounce-bound]

mod block_enumeration;
pub mod facet_classification;
#[cfg(test)]
mod kkt_benchmark;

#[cfg(test)]
use crate::algorithms::facet_adjacency::build_transition_matrix_from_facet_intersections_and_omega;
use crate::algorithms::facet_adjacency::is_feasible_cycle;
use crate::algorithms::orbit_search::solve_sigma_stream_with_dual_vertices;
use crate::algorithms::{OrbitKktData, OrbitSearchError};
use block_enumeration::{enumerate_blocks, enumerate_k_bounce_sigmas};
#[cfg(test)]
use facet_classification::classify_facets;
use nalgebra::{DMatrix, Vector4};

#[cfg(test)]
mod tests;

/// Error type for the billiard algorithm.
///
/// Returned when the polytope is not a valid Lagrangian product: either a facet
/// has mixed q/p normal components, or there are too few facets of one type.
#[derive(Debug, Clone)]
pub enum BilliardError {
    /// A facet normal has both q and p components (not a Lagrangian product).
    NotLagrangianProduct {
        /// Index of the offending facet.
        facet: usize,
        /// The facet's normal vector [n0, n1, n2, n3].
        normal: [f64; 4],
    },
    /// Too few facets of a given type (need at least 3 for a polygon).
    TooFewFacets {
        /// Which facet type is deficient ("q" or "p").
        facet_type: &'static str,
        /// How many facets of this type were found.
        count: usize,
    },
    /// Orbit search failed after Lagrangian-product classification succeeded.
    OrbitSearch(OrbitSearchError),
}

impl std::fmt::Display for BilliardError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BilliardError::NotLagrangianProduct { facet, normal } => {
                write!(
                    f,
                    "facet {} has mixed normal [{:.4}, {:.4}, {:.4}, {:.4}]: not a Lagrangian product",
                    facet, normal[0], normal[1], normal[2], normal[3]
                )
            }
            BilliardError::TooFewFacets { facet_type, count } => {
                write!(
                    f,
                    "only {} {}-facets (need at least 3 for a polygon)",
                    count, facet_type
                )
            }
            BilliardError::OrbitSearch(err) => {
                write!(f, "billiard orbit search failed: {err:?}")
            }
        }
    }
}

impl std::error::Error for BilliardError {}

/// Returns the billiard bounce count `k` encoded by `sigma`.
///
/// The polytope must be a valid Lagrangian product. If `sigma` does not match
/// the alternating billiard block structure `Q_1 P_1 ... Q_k P_k` with each
/// block of length 1 or 2, returns `Ok(None)`.
#[cfg(test)]
pub(crate) fn bounce_count_from_sigma(
    dual_vertices: &[Vector4<f64>],
    sigma: &[usize],
) -> Result<Option<usize>, BilliardError> {
    let classification = classify_facets(dual_vertices)?;
    Ok(classification.bounce_count_for_sigma(sigma))
}

/// Returns the billiard bounce count `k` encoded by `sigma`.
///
/// Input contract: `q_facet_indices` and `p_facet_indices` are disjoint facet
/// index lists for the same ordered facet set as `sigma`.
///
/// Public because branch-analysis experiments need to classify already-solved
/// orbits and filtered sigma streams from flat facet data.
pub fn bounce_count_from_sigma_for_facets(
    q_facet_indices: &[usize],
    p_facet_indices: &[usize],
    sigma: &[usize],
) -> Option<usize> {
    facet_classification::FacetClassification {
        q_indices: q_facet_indices.to_vec(),
        p_indices: p_facet_indices.to_vec(),
    }
    .bounce_count_for_sigma(sigma)
}

/// Visit every billiard sigma for flat Lagrangian-product facet data.
///
/// Input contract: q/p facet indices refer to the same ordered facet set as
/// `facet_intersection_is_nonempty` and `transition_is_allowed`.
///
/// Public because some experiments inspect or filter the billiard candidate
/// stream before solving, while [`solve_billiard_candidates`] is the ordinary
/// solve-all frontend.
pub fn for_each_sigma_from_facets(
    q_facet_indices: &[usize],
    p_facet_indices: &[usize],
    facet_intersection_is_nonempty: &DMatrix<bool>,
    transition_is_allowed: &DMatrix<bool>,
    mut visit: impl FnMut(&[usize]),
) {
    assert_eq!(
        facet_intersection_is_nonempty.shape(),
        transition_is_allowed.shape(),
        "facet_intersection_is_nonempty and transition_is_allowed must have the same shape"
    );
    let facet_count = facet_intersection_is_nonempty.nrows();
    assert_eq!(
        facet_intersection_is_nonempty.ncols(),
        facet_count,
        "facet_intersection_is_nonempty must be square"
    );
    assert!(
        q_facet_indices
            .iter()
            .chain(p_facet_indices.iter())
            .all(|&facet| facet < facet_count),
        "q_facet_indices and p_facet_indices must index the facet matrices"
    );

    let q_blocks = enumerate_blocks(q_facet_indices, facet_intersection_is_nonempty);
    let p_blocks = enumerate_blocks(p_facet_indices, facet_intersection_is_nonempty);

    for k in 2..=3 {
        enumerate_k_bounce_sigmas(k, &q_blocks, &p_blocks, |sigma| {
            if !is_feasible_cycle(sigma, transition_is_allowed) {
                return;
            }
            visit(sigma);
        });
    }
}

/// Solve every billiard candidate from flat Lagrangian-product data.
///
/// Input contract: all inputs use the same facet ordering. q/p facet indices
/// are the Lagrangian product classification, `facet_intersection_is_nonempty`
/// is the undirected facet-incidence relation, and `transition_is_allowed` is
/// the directed omega-aware transition relation.
pub fn solve_billiard_candidates(
    dual_vertices: &[Vector4<f64>],
    q_facet_indices: &[usize],
    p_facet_indices: &[usize],
    facet_intersection_is_nonempty: &DMatrix<bool>,
    transition_is_allowed: &DMatrix<bool>,
) -> Result<(Vec<OrbitKktData>, u64), OrbitSearchError> {
    assert_eq!(
        transition_is_allowed.shape(),
        (dual_vertices.len(), dual_vertices.len()),
        "transition_is_allowed must be square with one row/column per dual vertex"
    );

    solve_sigma_stream_with_dual_vertices(dual_vertices, |visit| {
        for_each_sigma_from_facets(
            q_facet_indices,
            p_facet_indices,
            facet_intersection_is_nonempty,
            transition_is_allowed,
            visit,
        )
    })
}

/// Visit every billiard sigma for a valid flat Lagrangian-product facet set.
#[cfg(test)]
pub(crate) fn for_each_sigma(
    dual_vertices: &[Vector4<f64>],
    facet_intersection_is_nonempty: &DMatrix<bool>,
    omega_signs: &DMatrix<i8>,
    visit: impl FnMut(&[usize]),
) -> Result<(), BilliardError> {
    let classification = classify_facets(dual_vertices)?;
    let transition_is_allowed = build_transition_matrix_from_facet_intersections_and_omega(
        facet_intersection_is_nonempty,
        omega_signs,
    );

    for_each_sigma_from_facets(
        &classification.q_indices,
        &classification.p_indices,
        facet_intersection_is_nonempty,
        &transition_is_allowed,
        visit,
    );

    Ok(())
}

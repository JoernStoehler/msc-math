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

use crate::algorithms::facet_adjacency::{build_transition_matrix, is_feasible_cycle};
use crate::geom::polytope::Polytope4D;
use block_enumeration::{enumerate_blocks, enumerate_k_bounce_sigmas};
use facet_classification::classify_facets;

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
        }
    }
}

impl std::error::Error for BilliardError {}

/// Returns the billiard bounce count `k` encoded by `sigma`.
///
/// The polytope must be a valid Lagrangian product. If `sigma` does not match
/// the alternating billiard block structure `Q_1 P_1 ... Q_k P_k` with each
/// block of length 1 or 2, returns `Ok(None)`.
pub fn bounce_count_from_sigma(
    polytope: &Polytope4D,
    sigma: &[usize],
) -> Result<Option<usize>, BilliardError> {
    let classification = classify_facets(polytope)?;
    Ok(classification.bounce_count_for_sigma(sigma))
}

/// Visit every billiard sigma for a valid Lagrangian product polytope.
pub fn for_each_sigma(
    polytope: &Polytope4D,
    mut visit: impl FnMut(&[usize]),
) -> Result<(), BilliardError> {
    let classification = classify_facets(polytope)?;
    let adj = polytope.vertex_adjacency();
    let directed_adj = build_transition_matrix(polytope);
    let q_blocks = enumerate_blocks(&classification.q_indices, adj);
    let p_blocks = enumerate_blocks(&classification.p_indices, adj);

    for k in 2..=3 {
        enumerate_k_bounce_sigmas(k, &q_blocks, &p_blocks, |sigma| {
            if !is_feasible_cycle(sigma, &directed_adj) {
                return;
            }
            visit(sigma);
        });
    }

    Ok(())
}

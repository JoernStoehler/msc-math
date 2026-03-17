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
//! Uses `CapacityAccumulator` for the enumerate -> solve -> track pattern,
//! mirroring the hk2017 module's accumulator usage.
//!
//! Submodules:
//! - `block_enumeration` — block structure enumeration for Q/P facets
//! - `facet_classification` — classify facets into q-space and p-space types
//! - `kkt_benchmark` — KKT solver performance measurement
//!
//! Mathematical correspondence: [thm:billiard-characterization], [thm:bounce-bound]

pub mod block_enumeration;
pub mod facet_classification;
pub mod kkt_benchmark;

#[cfg(test)]
#[path = "capacity_test.rs"]
mod capacity_test;

use crate::algorithms::capacity_accumulator::CapacityAccumulator;
use crate::algorithms::facet_adjacency::{
    build_adjacency_matrix, build_directed_adjacency_matrix, is_adjacent_cycle,
};
use crate::geom::polytope::Polytope4D;
use crate::kkt::saddle_point_solver::{solve_kkt_for, KktResult, EPS_Q_POSITIVE};
use crate::kkt::{classify_margin, Solution};
use block_enumeration::{enumerate_blocks, enumerate_k_bounce_sigmas};
use facet_classification::classify_facets;

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

/// Result of the billiard capacity computation.
///
/// Wraps [`CapacityResult`](crate::algorithms::capacity_accumulator::CapacityResult)
/// (shared accumulator output) plus the algorithm-specific `bounce_count` field
/// indicating the k value (2 or 3) of the optimal orbit.
///
/// Access capacity fields via `.result.capacity` (no Deref -- explicit field access).
///
/// [thm:billiard-characterization]: result of block-structured enumeration.
#[derive(Clone, Debug)]
pub struct BilliardResult {
    /// Core capacity result from the accumulator: capacity, uncertainty, best
    /// permutation, beta vector, and iteration count.
    pub result: crate::algorithms::capacity_accumulator::CapacityResult,
    /// Number of bounces (k value) of the optimal orbit (2 or 3).
    pub bounce_count: usize,
}

/// Compute c_EHZ for a Lagrangian product polytope.
///
/// Returns error if the polytope is not a Lagrangian product (some facet normal
/// has both q and p components, or too few facets of one type).
///
/// Returns `Ok(None)` if no valid orbit is found (should not happen for valid
/// Lagrangian products, but guards against degenerate input).
///
/// [thm:billiard-characterization], [thm:bounce-bound]: enumerates block permutations
/// sigma = (Q_1 P_1 ... Q_k P_k) for k in {2, 3}.
pub fn billiard_capacity(
    polytope: &Polytope4D,
) -> Result<Option<BilliardResult>, BilliardError> {
    // Step 1: classify facets into q-type and p-type.
    let classification = classify_facets(polytope)?;

    // Step 2: build adjacency matrices.
    // Undirected: for block building (same-type adjacent pairs).
    let adj = build_adjacency_matrix(polytope);
    // Directed: for cycle pruning (omega_0 transition feasibility).
    let directed_adj = build_directed_adjacency_matrix(polytope);

    // Step 3: enumerate blocks.
    let q_blocks = enumerate_blocks(&classification.q_indices, &adj);
    let p_blocks = enumerate_blocks(&classification.p_indices, &adj);

    // Step 4: for k = 2, 3, enumerate sigma sequences and solve KKT.
    let mut acc = CapacityAccumulator::new();
    // Track bounce count for certified candidates (accumulator doesn't track this).
    let mut best_bounce_certified: Option<(f64, usize)> = None;

    for k in 2..=3 {
        enumerate_k_bounce_sigmas(k, &q_blocks, &p_blocks, |sigma| {
            // Directed adjacency pruning: skip cycles violating omega_0 condition.
            if !is_adjacent_cycle(sigma, &directed_adj) {
                return;
            }

            if let Some(solution) = solve_and_convert(polytope, sigma) {
                // Track bounce count for certified candidates.
                if solution.verdict == crate::kkt::Verdict::True
                    && solution.q > EPS_Q_POSITIVE
                {
                    let action = 0.5 / solution.q;
                    let update = best_bounce_certified
                        .as_ref()
                        .is_none_or(|(best, _)| action < *best);
                    if update {
                        best_bounce_certified = Some((action, k));
                    }
                }
                acc.submit(sigma, &solution);
            }
        });
    }

    let result = match acc.finalize() {
        Some(r) => r,
        None => return Ok(None),
    };

    let bounce_count = best_bounce_certified.map_or(2, |(_, k)| k);

    Ok(Some(BilliardResult {
        result,
        bounce_count,
    }))
}

/// Solve the KKT system for a (polytope, permutation) pair and convert the
/// result into a `Solution` for the accumulator.
///
/// Same conversion logic as `hk2017::solve_and_convert`.
fn solve_and_convert(polytope: &Polytope4D, perm: &[usize]) -> Option<Solution> {
    let kkt = solve_kkt_for(polytope, perm)?;
    Some(kkt_result_to_solution(kkt))
}

/// Convert a `KktResult` to a `Solution`.
///
/// Maps: q_corrected -> q, beta -> beta, min(beta) -> margin, classify_margin -> verdict.
fn kkt_result_to_solution(result: KktResult) -> Solution {
    let margin = result
        .beta
        .iter()
        .copied()
        .fold(f64::INFINITY, f64::min);
    Solution {
        verdict: classify_margin(margin),
        q: result.q_corrected,
        beta: result.beta,
        margin,
    }
}

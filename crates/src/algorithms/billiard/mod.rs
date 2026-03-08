/// EHZ capacity computation for Lagrangian products via the billiard algorithm.
///
/// Computes c_EHZ(K_q ×_L K_p) for Lagrangian products where K_q, K_p are
/// convex polygons in R².
///
/// See `[thm:billiard-characterization]` (thesis): c_EHZ equals the minimum
/// K_p°-length billiard trajectory in K_q, and `[thm:bounce-bound]`: the
/// minimiser has at most 3 bounces. This module enumerates block-structured
/// permutations σ = (Q₁P₁···QₖPₖ) for k ∈ {2, 3}, the computational
/// realisation of those two theorems.
mod enumerate;
mod lagrangian;

use crate::algorithms::hk2017::{build_adjacency_matrix, build_directed_adjacency_matrix, is_adjacent_cycle};
use crate::geom::polytope::Polytope4D;
use crate::kkt::{solve_kkt, EPS_BETA_POSITIVE, EPS_Q_POSITIVE};
use enumerate::{enumerate_blocks, enumerate_k_bounce_sigmas};
use lagrangian::classify_facets;

/// Error type for the billiard algorithm.
#[derive(Debug, Clone)]
pub enum BilliardError {
    /// The polytope is not a Lagrangian product: a facet normal has both
    /// q and p components.
    NotLagrangianProduct {
        facet: usize,
        normal: [f64; 4],
    },
    /// Too few facets of a given type (need at least 3 for a polygon).
    TooFewFacets {
        facet_type: &'static str,
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
#[derive(Clone, Debug)]
pub struct BilliardResult {
    /// The EHZ capacity c_EHZ(K) from certified check (β_i > +EPS).
    pub capacity: f64,
    /// Capacity from uncertain check (-EPS < β_i). Always ≤ capacity.
    /// See `hk2017::EhzResult::capacity_uncertain` for full documentation.
    pub capacity_uncertain: f64,
    /// The cyclic permutation σ in **physical orbit direction**.
    /// σ[k] → σ[k+1] is the direction of the Reeb orbit.
    pub best_permutation: Vec<usize>,
    /// The β vector at the optimum.
    pub best_beta: Vec<f64>,
    /// Number of bounces (k value) of the optimal orbit.
    pub bounce_count: usize,
    /// Total number of (S, σ) pairs evaluated (KKT solves).
    pub iterations: u64,
}

/// Compute c_EHZ for a Lagrangian product polytope.
///
/// Returns error if the polytope is not a Lagrangian product
/// (i.e., some facet normal has both q and p components).
///
/// Returns `Ok(None)` if no valid orbit is found (should not happen for
/// valid Lagrangian products, but guards against degenerate input).
pub fn billiard_capacity(polytope: &Polytope4D) -> Result<Option<BilliardResult>, BilliardError> {
    // Step 1: classify facets
    let classification = classify_facets(polytope)?;

    // Step 2: build adjacency matrices
    let adj = build_adjacency_matrix(polytope); // undirected: for block building (same-type pairs)
    let directed_adj = build_directed_adjacency_matrix(polytope); // directed: for cycle pruning (ω₀ condition)

    // Step 3: enumerate blocks
    let q_blocks = enumerate_blocks(&classification.q_indices, &adj);
    let p_blocks = enumerate_blocks(&classification.p_indices, &adj);

    // Step 4: for k = 2, 3, enumerate and solve
    let mut best_certified: Option<(f64, Vec<usize>, Vec<f64>, usize)> = None;
    let mut best_uncertain: Option<(f64, Vec<usize>, Vec<f64>, usize)> = None;
    let mut iterations: u64 = 0;

    let normals = polytope.normals_f64();
    let heights = polytope.heights_f64();

    for k in 2..=3 {
        enumerate_k_bounce_sigmas(k, &q_blocks, &p_blocks, |sigma| {
            // Directed adjacency pruning: skip cycles where consecutive facets
            // violate the ω₀ transition feasibility condition `[cor:adjacency-pruning]`.
            // directed_adj uses physical convention: adj[i][j] = transition F_i → F_j feasible.
            if !is_adjacent_cycle(sigma, &directed_adj) {
                return;
            }
            iterations += 1;

            if let Some(result) = solve_kkt(normals, heights, sigma) {
                let q_val = result.q_corrected;
                if q_val <= EPS_Q_POSITIVE {
                    return;
                }
                let beta_min = result.beta.iter().cloned().fold(f64::INFINITY, f64::min);
                let action = 0.5 / q_val;

                if beta_min > EPS_BETA_POSITIVE {
                    let update = best_certified.as_ref().is_none_or(|b| action < b.0);
                    if update {
                        best_certified =
                            Some((action, sigma.to_vec(), result.beta.clone(), k));
                    }
                }

                if beta_min > -EPS_BETA_POSITIVE {
                    let update = best_uncertain.as_ref().is_none_or(|b| action < b.0);
                    if update {
                        best_uncertain = Some((action, sigma.to_vec(), result.beta, k));
                    }
                }
            }
        });
    }

    Ok(best_certified.map(
        |(capacity, best_permutation, best_beta, bounce_count)| {
            let uncertain_cap = best_uncertain.map_or(capacity, |b| b.0);

            // Safety net: if an UNKNOWN orbit achieves significantly lower action than
            // the best certified orbit, the reported capacity might be wrong and we
            // cannot resolve it at f64 precision. Fail loudly rather than publish a
            // potentially false result.
            // Tolerance 1e-10: capacity values are O(1)–O(10), so 1e-10 is ~10 orders
            // below typical values. Previous 1e-12 was too tight — triggered on
            // gap=4.93e-12 for capacity≈3.0 (relative 1.6e-12, f64 rounding noise).
            // If this fires, investigate whether the UNKNOWN orbit is genuinely better.
            let gap = capacity - uncertain_cap;
            assert!(
                gap <= 1e-10,
                "Numerical gap: certified capacity {:.6e} > uncertain capacity {:.6e} \
                 (gap = {:.6e}). An UNKNOWN orbit achieves lower action than the best \
                 certified orbit. Cannot resolve at f64 precision.",
                capacity, uncertain_cap, gap,
            );

            // Sanity: winning orbit has positive capacity.
            assert!(capacity > 0.0, "capacity must be positive, got {:.2e}", capacity);
            assert!(capacity.is_finite(), "capacity must be finite, got {:.2e}", capacity);

            // Candidate already stores perm and β in natural (positive Reeb) order.
            BilliardResult {
                capacity,
                capacity_uncertain: uncertain_cap,
                best_permutation,
                best_beta,
                bounce_count,
                iterations,
            }
        },
    ))
}

#[cfg(test)]
mod bench_kkt;

#[cfg(test)]
#[path = "billiard_test.rs"]
mod billiard_test;

/// EHZ capacity computation for Lagrangian products via the billiard algorithm.
///
/// Computes c_EHZ(K_q ×_L K_p) for Lagrangian products where K_q, K_p are
/// convex polygons in R². Exploits the sigma structure lemma and 3-bounce
/// bound to restrict enumeration to polynomial cost.
///
/// See chapter-billiard.tex, Section 6.
mod enumerate;
mod lagrangian;

use crate::algorithms::hk2017::build_adjacency_matrix;
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
    /// The EHZ capacity c_EHZ(K) from strict check (β_i > +EPS).
    pub capacity: f64,
    /// Capacity from lenient check (β_i > -EPS). Always ≤ capacity.
    /// See `hk2017::EhzResult::capacity_lenient` for full documentation.
    pub capacity_lenient: f64,
    /// The cyclic permutation σ achieving the minimum action.
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

    // Step 2: build adjacency matrix
    let adj = build_adjacency_matrix(polytope);

    // Step 3: enumerate blocks
    let q_blocks = enumerate_blocks(&classification.q_indices, &adj);
    let p_blocks = enumerate_blocks(&classification.p_indices, &adj);

    // Step 4: for k = 2, 3, enumerate and solve
    let mut best_strict: Option<(f64, Vec<usize>, Vec<f64>, usize)> = None;
    let mut best_lenient: Option<(f64, Vec<usize>, Vec<f64>, usize)> = None;
    let mut iterations: u64 = 0;

    let normals = polytope.normals();
    let heights = polytope.heights();

    for k in 2..=3 {
        enumerate_k_bounce_sigmas(k, &q_blocks, &p_blocks, |sigma| {
            iterations += 1;

            if let Some((beta, q_val)) = solve_kkt(normals, heights, sigma) {
                if q_val <= EPS_Q_POSITIVE {
                    return;
                }
                let beta_min = beta.iter().cloned().fold(f64::INFINITY, f64::min);
                let action = 0.5 / q_val;

                if beta_min > EPS_BETA_POSITIVE {
                    let update = best_strict.as_ref().is_none_or(|b| action < b.0);
                    if update {
                        best_strict =
                            Some((action, sigma.to_vec(), beta.clone(), k));
                    }
                }

                if beta_min > -EPS_BETA_POSITIVE {
                    let update = best_lenient.as_ref().is_none_or(|b| action < b.0);
                    if update {
                        best_lenient = Some((action, sigma.to_vec(), beta, k));
                    }
                }
            }
        });
    }

    Ok(best_strict.map(
        |(capacity, best_permutation, best_beta, bounce_count)| {
            let lenient_cap = best_lenient.map_or(capacity, |b| b.0);
            BilliardResult {
                capacity,
                capacity_lenient: lenient_cap,
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

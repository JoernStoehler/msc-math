/// EHZ capacity computation for Lagrangian products via the billiard algorithm.
///
/// Computes c_EHZ(K_q ×_L K_p) for Lagrangian products where K_q, K_p are
/// convex polygons in R². Exploits the sigma structure lemma and 3-bounce
/// bound to restrict enumeration to polynomial cost.
///
/// See chapter-billiard.tex, Section 6.
mod enumerate;
mod kkt;
mod lagrangian;

use enumerate::{enumerate_blocks, enumerate_k_bounce_sigmas};
use geom::polytope::Polytope4D;
use kkt::{solve_kkt, EPS_BETA_POSITIVE, EPS_Q_POSITIVE};
use lagrangian::classify_facets;

/// Tolerance for vertex-facet incidence in adjacency matrix.
const EPS_FACET_INCIDENCE: f64 = 1e-8;

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
    /// The EHZ capacity c_EHZ(K).
    pub capacity: f64,
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
    let mut best: Option<(f64, Vec<usize>, Vec<f64>, usize)> = None;
    let mut iterations: u64 = 0;

    let normals = polytope.normals();
    let heights = polytope.heights();

    for k in 2..=3 {
        enumerate_k_bounce_sigmas(k, &q_blocks, &p_blocks, |sigma| {
            iterations += 1;

            if let Some((beta, q_val)) = solve_kkt(normals, heights, sigma) {
                if beta.iter().all(|&b| b > EPS_BETA_POSITIVE) && q_val > EPS_Q_POSITIVE {
                    let action = 0.5 / q_val;
                    let update = match &best {
                        None => true,
                        Some((best_a, _, _, _)) => action < *best_a,
                    };
                    if update {
                        best = Some((action, sigma.to_vec(), beta, k));
                    }
                }
            }
        });
    }

    Ok(best.map(|(capacity, best_permutation, best_beta, bounce_count)| {
        BilliardResult {
            capacity,
            best_permutation,
            best_beta,
            bounce_count,
            iterations,
        }
    }))
}

/// Build facet adjacency matrix: adj[i][j] = true iff facets i and j share a vertex.
fn build_adjacency_matrix(polytope: &Polytope4D) -> Vec<Vec<bool>> {
    let f = polytope.facet_count();
    let normals = polytope.normals();
    let heights = polytope.heights();
    let mut adj = vec![vec![false; f]; f];

    for v in polytope.vertices() {
        let incident: Vec<usize> = (0..f)
            .filter(|&i| (normals[i].dot(v) - heights[i]).abs() < EPS_FACET_INCIDENCE)
            .collect();
        for &i in &incident {
            for &j in &incident {
                adj[i][j] = true;
            }
        }
    }

    adj
}

#[cfg(test)]
mod bench_kkt;

#[cfg(test)]
mod lib_test;

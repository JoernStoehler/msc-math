//! Undirected and directed (ω₀-aware) facet adjacency matrices.
//!
//! Facet adjacency determines which facet transitions are geometrically possible
//! in Reeb dynamics on a polytope boundary. The undirected matrix records which
//! facet pairs share a vertex; the directed matrix additionally enforces the
//! symplectic sign condition ω₀(nᵢ, nⱼ) ≥ 0 from [lem:numerical-transition-feasibility].
//!
//! Used by hk2017 and billiard algorithms for pruning infeasible permutations.
//!
//! Mathematical correspondence: [lem:numerical-transition-feasibility], [cor:adjacency-pruning]

use crate::geom::polytope::Polytope4D;

/// Undirected facet adjacency: `adj[i][j] = true` iff facets Fᵢ and Fⱼ share at least one vertex.
///
/// The diagonal is false (a facet is not adjacent to itself). This is safe because
/// `is_adjacent_cycle` operates on distinct-element permutations.
///
/// Uses the exact adjacency matrix precomputed in `Polytope4D` over Q.
///
/// [lem:numerical-transition-feasibility]: transition F_i -> F_j requires F_i, F_j to share a vertex.
pub fn build_adjacency_matrix(polytope: &Polytope4D) -> Vec<Vec<bool>> {
    let f = polytope.facet_count();
    let poly_adj = polytope.adjacency();

    let mut adj = vec![vec![false; f]; f];
    for i in 0..f {
        for j in 0..f {
            adj[i][j] = poly_adj[(i, j)];
        }
    }
    adj
}

/// Directed facet adjacency in the physical Reeb direction:
/// `adj[i][j] = true` iff the transition Fᵢ → Fⱼ is feasible.
///
/// Combines two conditions:
/// 1. Vertex adjacency: Fᵢ ∩ Fⱼ ≠ ∅ (from undirected adjacency)
/// 2. Symplectic sign: ω₀(nᵢ, nⱼ) ≥ 0
///
/// The sign condition uses the exact `omega_signs` matrix from the rational pipeline
/// (values in {−1, 0, +1}), so there is no f64 tolerance ambiguity near ω₀ = 0.
///
/// Note: `omega_signs[(i,j)]` stores `sign(ω₀(yᵢ, yⱼ))` where yᵢ = aᵢ are dual vertices
/// (outward normals divided by height). Since hᵢ, hⱼ > 0, this has the same sign as
/// `ω₀(nᵢ, nⱼ)`.
///
/// [lem:numerical-transition-feasibility]: F_i -> F_j requires vertex adjacency + omega_0(n_i, n_j) >= 0.
/// [cor:adjacency-pruning]: this directed adjacency can prune infeasible permutations.
pub fn build_directed_adjacency_matrix(polytope: &Polytope4D) -> Vec<Vec<bool>> {
    let f = polytope.facet_count();
    let vertex_adj = build_adjacency_matrix(polytope);
    let omega_signs = polytope.omega_signs();

    let mut adj = vec![vec![false; f]; f];
    for i in 0..f {
        for j in 0..f {
            if !vertex_adj[i][j] {
                continue;
            }
            // Transition Fᵢ → Fⱼ allowed when ω₀(nᵢ, nⱼ) ≥ 0,
            // i.e. omega_signs[(i,j)] ∈ {0, +1}.
            adj[i][j] = omega_signs[(i, j)] >= 0;
        }
    }
    adj
}

/// Check if a cyclic permutation forms an adjacent cycle in the given adjacency matrix.
///
/// Returns true iff every consecutive pair `(perm[k], perm[k+1 mod m])` is adjacent.
/// Works with both undirected and directed adjacency matrices.
pub fn is_adjacent_cycle(perm: &[usize], adj: &[Vec<bool>]) -> bool {
    let m = perm.len();
    if m == 0 {
        return true;
    }
    (0..m).all(|k| adj[perm[k]][perm[(k + 1) % m]])
}

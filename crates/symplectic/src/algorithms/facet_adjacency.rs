//! Directed (ω₀-aware) facet adjacency and cycle checks.
//!
//! Undirected adjacency (vertex-sharing) is precomputed in `Polytope4D::adjacency()`.
//! This module adds the symplectic sign condition ω₀(nᵢ, nⱼ) ≥ 0 to produce
//! directed adjacency, and provides cycle-checking utilities.
//!
//! Used by hk2017 and billiard algorithms for pruning infeasible permutations.
//!
//! Mathematical correspondence: [lem:numerical-transition-feasibility], [cor:adjacency-pruning]

use crate::geom::polytope::Polytope4D;
use nalgebra::DMatrix;

#[cfg(test)]
mod tests;

/// Directed facet adjacency in the physical Reeb direction:
/// `adj[(i,j)] = true` iff the transition Fᵢ → Fⱼ is feasible.
///
/// Combines two conditions:
/// 1. Vertex adjacency: Fᵢ ∩ Fⱼ ≠ ∅ (from `polytope.vertex_adjacency()`)
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
pub fn build_transition_matrix(polytope: &Polytope4D) -> DMatrix<bool> {
    let f = polytope.facet_count();
    let vertex_adj = polytope.vertex_adjacency();
    let omega_signs = polytope.omega_signs();

    DMatrix::from_fn(f, f, |i, j| vertex_adj[(i, j)] && omega_signs[(i, j)] >= 0)
}

/// Check if a cyclic permutation forms an adjacent cycle in the given adjacency matrix.
///
/// Returns true iff every consecutive pair `(perm[k], perm[k+1 mod m])` is adjacent.
/// Works with both undirected and directed adjacency matrices.
pub fn is_feasible_cycle(perm: &[usize], adj: &DMatrix<bool>) -> bool {
    let m = perm.len();
    if m == 0 {
        return true;
    }
    (0..m).all(|k| adj[(perm[k], perm[(k + 1) % m])])
}

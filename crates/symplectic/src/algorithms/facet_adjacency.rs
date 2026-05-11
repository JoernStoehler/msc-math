//! Directed (omega_0-aware) transition matrices and cycle checks.
//!
//! Facet-intersection nonemptiness and omega signs are computed by
//! geometry/exact validation code. This module adds the symplectic sign
//! condition `omega_0(n_i, n_j) >= 0` to produce the directed transition
//! relation used by HK2017 pruning and billiard enumeration.
//!
//! Used by hk2017 and billiard algorithms for pruning infeasible permutations.
//!
//! Mathematical correspondence: [lem:numerical-transition-feasibility], [cor:adjacency-pruning]

use nalgebra::DMatrix;

#[cfg(test)]
mod tests;

/// Directed transition feasibility in the physical Reeb direction from flat inputs:
/// `transition_is_allowed[(i,j)] = true` iff the transition F_i -> F_j is feasible.
///
/// Combines two conditions:
/// 1. Facet intersection nonemptiness: Fᵢ ∩ Fⱼ ≠ ∅
/// 2. Symplectic sign: ω₀(nᵢ, nⱼ) ≥ 0
///
/// The sign condition uses the exact `omega_signs` matrix from the rational pipeline
/// (values in {−1, 0, +1}), so there is no f64 tolerance ambiguity near ω₀ = 0.
///
/// Note: `omega_signs[(i,j)]` stores `sign(ω₀(yᵢ, yⱼ))` where yᵢ = aᵢ are dual vertices
/// (outward normals divided by height). Since hᵢ, hⱼ > 0, this has the same sign as
/// `ω₀(nᵢ, nⱼ)`.
///
/// [lem:numerical-transition-feasibility]: F_i -> F_j requires facet intersection nonemptiness + omega_0(n_i, n_j) >= 0.
/// [cor:adjacency-pruning]: this directed transition relation can prune infeasible permutations.
pub fn build_transition_matrix_from_facet_intersections_and_omega(
    facet_intersection_is_nonempty: &DMatrix<bool>,
    omega_signs: &DMatrix<i8>,
) -> DMatrix<bool> {
    assert_eq!(
        facet_intersection_is_nonempty.nrows(),
        facet_intersection_is_nonempty.ncols(),
        "facet_intersection_is_nonempty must be square"
    );
    assert_eq!(
        omega_signs.nrows(),
        omega_signs.ncols(),
        "omega_signs must be square"
    );
    assert_eq!(
        facet_intersection_is_nonempty.shape(),
        omega_signs.shape(),
        "facet_intersection_is_nonempty and omega_signs must have the same shape"
    );

    DMatrix::from_fn(
        facet_intersection_is_nonempty.nrows(),
        facet_intersection_is_nonempty.ncols(),
        |i, j| facet_intersection_is_nonempty[(i, j)] && omega_signs[(i, j)] >= 0,
    )
}

/// Check if a cyclic permutation forms a feasible cycle in the given transition matrix.
///
/// Returns true iff every consecutive pair `(perm[k], perm[k+1 mod m])` is allowed.
pub fn is_feasible_cycle(perm: &[usize], transition_is_allowed: &DMatrix<bool>) -> bool {
    let m = perm.len();
    if m == 0 {
        return true;
    }
    (0..m).all(|k| transition_is_allowed[(perm[k], perm[(k + 1) % m])])
}

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

    DMatrix::from_fn(f, f, |i, j| {
        vertex_adj[(i, j)] && omega_signs[(i, j)] >= 0
    })
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geom::known_polytopes;

    // Tests for facet_adjacency: directed adjacency and cycle checks.
    //
    // Proposition: Directed adjacency correctly combines vertex-sharing and omega_0-sign conditions.
    // Reference: [lem:numerical-transition-feasibility], [cor:adjacency-pruning]
    //
    // Strategy: fixture-based on simplex, hypercube, and Lagrangian products.

    /// Simplex (5 facets): every pair of facets shares a vertex (complete graph).
    /// Undirected adjacency should be all-true except diagonal.
    #[test]
    #[allow(clippy::needless_range_loop)]
    fn simplex_undirected_is_complete() {
        let kp = known_polytopes::simplex();
        let adj = kp.polytope.vertex_adjacency();
        let f = kp.polytope.facet_count();
        assert_eq!(f, 5);

        for i in 0..f {
            for j in 0..f {
                if i == j {
                    assert!(!adj[(i, j)], "diagonal should be false");
                } else {
                    assert!(adj[(i, j)], "simplex facets {i} and {j} should be adjacent");
                }
            }
        }
    }

    /// Hypercube (8 facets in 4D): opposite facets are not adjacent.
    /// Each facet should be adjacent to exactly 6 others (all except itself and its opposite).
    #[test]
    #[allow(clippy::needless_range_loop)]
    fn hypercube_undirected_excludes_opposite_facets() {
        let kp = known_polytopes::hypercube();
        let adj = kp.polytope.vertex_adjacency();
        let f = kp.polytope.facet_count();
        assert_eq!(f, 8);

        for i in 0..f {
            let neighbor_count: usize = (0..f).filter(|&j| adj[(i, j)]).count();
            // In a 4D hypercube, each facet (a 3D cube) shares vertices with 6 of 7 other facets.
            assert_eq!(
                neighbor_count, 6,
                "facet {i} should have 6 neighbors, got {neighbor_count}"
            );
        }
    }

    /// Undirected adjacency is symmetric: adj[(i,j)] == adj[(j,i)].
    #[test]
    #[allow(clippy::needless_range_loop)]
    fn undirected_adjacency_is_symmetric() {
        for kp in known_polytopes::all_known() {
            let adj = kp.polytope.vertex_adjacency();
            let f = kp.polytope.facet_count();
            for i in 0..f {
                for j in 0..f {
                    assert_eq!(
                        adj[(i, j)], adj[(j, i)],
                        "asymmetric undirected adjacency at ({i},{j}) for {}",
                        kp.name
                    );
                }
            }
        }
    }

    /// Directed adjacency is a subset of undirected: if directed[(i,j)], then undirected[(i,j)].
    #[test]
    fn directed_is_subset_of_undirected() {
        for kp in known_polytopes::all_known() {
            let undirected = kp.polytope.vertex_adjacency();
            let directed = build_transition_matrix(&kp.polytope);
            let f = kp.polytope.facet_count();
            for i in 0..f {
                for j in 0..f {
                    if directed[(i, j)] {
                        assert!(
                            undirected[(i, j)],
                            "directed[{i}][{j}] true but undirected false for {}",
                            kp.name
                        );
                    }
                }
            }
        }
    }

    /// Directed adjacency is NOT symmetric in general (omega_0 is antisymmetric).
    /// If omega_0(n_i, n_j) > 0, then omega_0(n_j, n_i) < 0, so directed[(i,j)] and directed[(j,i)]
    /// cannot both be true unless omega_0(n_i, n_j) = 0.
    #[test]
    fn directed_adjacency_antisymmetry_property() {
        for kp in known_polytopes::all_known() {
            let directed = build_transition_matrix(&kp.polytope);
            let omega_signs = kp.polytope.omega_signs();
            let f = kp.polytope.facet_count();
            for i in 0..f {
                for j in (i + 1)..f {
                    if directed[(i, j)] && directed[(j, i)] {
                        // Both directions allowed only when omega_0(n_i, n_j) = 0
                        assert_eq!(
                            omega_signs[(i, j)],
                            0,
                            "both directed[{i}][{j}] and directed[{j}][{i}] true \
                             but omega_signs != 0 for {}",
                            kp.name
                        );
                    }
                }
            }
        }
    }

    /// Directed adjacency strictly prunes compared to undirected on generic polytopes.
    /// For polytopes where omega_0 signs are nonzero (generic case), directed should have
    /// strictly fewer true entries than undirected.
    #[test]
    fn directed_prunes_vs_undirected() {
        // The simplex is generic enough that directed should prune some edges
        let kp = known_polytopes::simplex();
        let undirected = kp.polytope.vertex_adjacency();
        let directed = build_transition_matrix(&kp.polytope);
        let count_undirected: usize = undirected.iter().filter(|&&v| v).count();
        let count_directed: usize = directed.iter().filter(|&&v| v).count();

        assert!(
            count_directed < count_undirected,
            "directed ({count_directed}) should have fewer edges than undirected ({count_undirected})"
        );
    }

    /// is_feasible_cycle: a valid cycle on a complete adjacency graph always returns true.
    #[test]
    fn is_feasible_cycle_complete_graph() {
        // 4-facet complete graph (all true except diagonal)
        let adj = DMatrix::from_fn(4, 4, |i, j| i != j);
        assert!(is_feasible_cycle(&[0, 1, 2, 3], &adj));
        assert!(is_feasible_cycle(&[3, 2, 1, 0], &adj));
        assert!(is_feasible_cycle(&[0, 2, 1, 3], &adj));
    }

    /// is_feasible_cycle: returns false when a transition is missing.
    #[test]
    fn is_feasible_cycle_missing_edge() {
        // 4 facets, but 0->2 is not adjacent
        #[rustfmt::skip]
        let data = vec![
            false, true, false, true,
            true, false, true, true,
            false, true, false, true,
            true, true, true, false,
        ];
        let adj = DMatrix::from_row_slice(4, 4, &data);
        // 0->2 missing, so [0,2,1,3] should fail at the 0->2 step
        assert!(!is_feasible_cycle(&[0, 2, 1, 3], &adj));
        // But [0,1,2,3] should work: 0->1 ok, 1->2 ok, 2->3 ok, 3->0 ok
        assert!(is_feasible_cycle(&[0, 1, 2, 3], &adj));
    }

    /// is_feasible_cycle: empty permutation is trivially adjacent.
    #[test]
    fn is_feasible_cycle_empty() {
        let adj = DMatrix::from_element(0, 0, false);
        assert!(is_feasible_cycle(&[], &adj));
    }

    /// is_feasible_cycle: single-element permutation is trivially adjacent.
    #[test]
    fn is_feasible_cycle_single_element() {
        let adj = DMatrix::from_element(1, 1, false);
        // Single element: checks adj[(0,0)] which is false.
        assert!(!is_feasible_cycle(&[0], &adj));
    }

    /// Lagrangian product: directed adjacency respects the Q/P facet structure.
    /// Q-type facets have normals in the (q1,q2,0,0) subspace and P-type in (0,0,p1,p2).
    /// omega_0 between two Q-type normals is 0 (both in Lagrangian subspace), similarly for P-type.
    /// omega_0 between a Q-type and P-type normal is generically nonzero.
    #[test]
    fn lagrangian_product_q_q_transitions_bidirectional() {
        let kp = known_polytopes::lagrangian_triangle_product();
        let directed = build_transition_matrix(&kp.polytope);
        let undirected = kp.polytope.vertex_adjacency();
        let omega_signs = kp.polytope.omega_signs();
        let f = kp.polytope.facet_count();

        // For pairs where omega_signs == 0 and undirected adjacent,
        // both directions should be allowed in the directed matrix.
        for i in 0..f {
            for j in 0..f {
                if undirected[(i, j)] && omega_signs[(i, j)] == 0 {
                    assert!(
                        directed[(i, j)] && directed[(j, i)],
                        "omega_0=0 pair ({i},{j}) should be bidirectional in directed adjacency"
                    );
                }
            }
        }
    }
}

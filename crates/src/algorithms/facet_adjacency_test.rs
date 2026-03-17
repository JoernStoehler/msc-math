//! Tests for facet_adjacency: undirected and directed adjacency correctness.
//!
//! Proposition: Adjacency matrices correctly encode vertex-sharing and ω₀-sign conditions.
//! Reference: [lem:numerical-transition-feasibility], [cor:adjacency-pruning]
//!
//! Strategy: fixture-based on simplex, hypercube, and Lagrangian products.

use crate::algorithms::facet_adjacency::{
    build_adjacency_matrix, build_directed_adjacency_matrix, is_adjacent_cycle,
};
use crate::geom::known_polytopes;

/// Simplex (5 facets): every pair of facets shares a vertex (complete graph).
/// Undirected adjacency should be all-true except diagonal.
#[test]
#[allow(clippy::needless_range_loop)]
fn simplex_undirected_is_complete() {
    let kp = known_polytopes::simplex();
    let adj = build_adjacency_matrix(&kp.polytope);
    let f = kp.polytope.facet_count();
    assert_eq!(f, 5);

    for i in 0..f {
        for j in 0..f {
            if i == j {
                assert!(!adj[i][j], "diagonal should be false");
            } else {
                assert!(adj[i][j], "simplex facets {i} and {j} should be adjacent");
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
    let adj = build_adjacency_matrix(&kp.polytope);
    let f = kp.polytope.facet_count();
    assert_eq!(f, 8);

    for i in 0..f {
        let neighbor_count: usize = (0..f).filter(|&j| adj[i][j]).count();
        // In a 4D hypercube, each facet (a 3D cube) shares vertices with 6 of 7 other facets.
        assert_eq!(
            neighbor_count, 6,
            "facet {i} should have 6 neighbors, got {neighbor_count}"
        );
    }
}

/// Undirected adjacency is symmetric: adj[i][j] == adj[j][i].
#[test]
#[allow(clippy::needless_range_loop)]
fn undirected_adjacency_is_symmetric() {
    for kp in known_polytopes::all_known() {
        let adj = build_adjacency_matrix(&kp.polytope);
        let f = kp.polytope.facet_count();
        for i in 0..f {
            for j in 0..f {
                assert_eq!(
                    adj[i][j], adj[j][i],
                    "asymmetric undirected adjacency at ({i},{j}) for {}",
                    kp.name
                );
            }
        }
    }
}

/// Directed adjacency is a subset of undirected: if directed[i][j], then undirected[i][j].
#[test]
fn directed_is_subset_of_undirected() {
    for kp in known_polytopes::all_known() {
        let undirected = build_adjacency_matrix(&kp.polytope);
        let directed = build_directed_adjacency_matrix(&kp.polytope);
        let f = kp.polytope.facet_count();
        for i in 0..f {
            for j in 0..f {
                if directed[i][j] {
                    assert!(
                        undirected[i][j],
                        "directed[{i}][{j}] true but undirected false for {}",
                        kp.name
                    );
                }
            }
        }
    }
}

/// Directed adjacency is NOT symmetric in general (ω₀ is antisymmetric).
/// If ω₀(nᵢ, nⱼ) > 0, then ω₀(nⱼ, nᵢ) < 0, so directed[i][j] and directed[j][i]
/// cannot both be true unless ω₀(nᵢ, nⱼ) = 0.
#[test]
fn directed_adjacency_antisymmetry_property() {
    for kp in known_polytopes::all_known() {
        let directed = build_directed_adjacency_matrix(&kp.polytope);
        let omega_signs = kp.polytope.omega_signs();
        let f = kp.polytope.facet_count();
        for i in 0..f {
            for j in (i + 1)..f {
                if directed[i][j] && directed[j][i] {
                    // Both directions allowed only when ω₀(nᵢ, nⱼ) = 0
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
/// For polytopes where ω₀ signs are nonzero (generic case), directed should have
/// strictly fewer true entries than undirected.
#[test]
fn directed_prunes_vs_undirected() {
    // The simplex is generic enough that directed should prune some edges
    let kp = known_polytopes::simplex();
    let undirected = build_adjacency_matrix(&kp.polytope);
    let directed = build_directed_adjacency_matrix(&kp.polytope);
    let count_undirected: usize = undirected.iter().flat_map(|row| row.iter()).filter(|&&v| v).count();
    let count_directed: usize = directed.iter().flat_map(|row| row.iter()).filter(|&&v| v).count();

    assert!(
        count_directed < count_undirected,
        "directed ({count_directed}) should have fewer edges than undirected ({count_undirected})"
    );
}

/// is_adjacent_cycle: a valid cycle on a complete adjacency graph always returns true.
#[test]
fn is_adjacent_cycle_complete_graph() {
    // 4-facet complete graph (all true except diagonal)
    let adj = vec![
        vec![false, true, true, true],
        vec![true, false, true, true],
        vec![true, true, false, true],
        vec![true, true, true, false],
    ];
    assert!(is_adjacent_cycle(&[0, 1, 2, 3], &adj));
    assert!(is_adjacent_cycle(&[3, 2, 1, 0], &adj));
    assert!(is_adjacent_cycle(&[0, 2, 1, 3], &adj));
}

/// is_adjacent_cycle: returns false when a transition is missing.
#[test]
fn is_adjacent_cycle_missing_edge() {
    // 4 facets, but 0→2 is not adjacent
    let adj = vec![
        vec![false, true, false, true],
        vec![true, false, true, true],
        vec![false, true, false, true],
        vec![true, true, true, false],
    ];
    // 0→2 missing, so [0,2,1,3] should fail at the 0→2 step
    assert!(!is_adjacent_cycle(&[0, 2, 1, 3], &adj));
    // But [0,1,2,3] should work: 0→1 ok, 1→2 ok, 2→3 ok, 3→0 ok
    assert!(is_adjacent_cycle(&[0, 1, 2, 3], &adj));
}

/// is_adjacent_cycle: empty permutation is trivially adjacent.
#[test]
fn is_adjacent_cycle_empty() {
    let adj: Vec<Vec<bool>> = vec![];
    assert!(is_adjacent_cycle(&[], &adj));
}

/// is_adjacent_cycle: single-element permutation is trivially adjacent.
#[test]
fn is_adjacent_cycle_single_element() {
    let adj = vec![vec![false]];
    // Single element: checks adj[0][0] which is false, but m=1 means
    // the cycle is (0) → (0), i.e. adj[0][0]. This is false for self-loops.
    assert!(!is_adjacent_cycle(&[0], &adj));
}

/// Lagrangian product: directed adjacency respects the Q/P facet structure.
/// Q-type facets have normals in the (q₁,q₂,0,0) subspace and P-type in (0,0,p₁,p₂).
/// ω₀ between two Q-type normals is 0 (both in Lagrangian subspace), similarly for P-type.
/// ω₀ between a Q-type and P-type normal is generically nonzero.
#[test]
fn lagrangian_product_q_q_transitions_bidirectional() {
    let kp = known_polytopes::lagrangian_triangle_product();
    let directed = build_directed_adjacency_matrix(&kp.polytope);
    let undirected = build_adjacency_matrix(&kp.polytope);
    let omega_signs = kp.polytope.omega_signs();
    let f = kp.polytope.facet_count();

    // For pairs where omega_signs == 0 and undirected adjacent,
    // both directions should be allowed in the directed matrix.
    for i in 0..f {
        for j in 0..f {
            if undirected[i][j] && omega_signs[(i, j)] == 0 {
                assert!(
                    directed[i][j] && directed[j][i],
                    "ω₀=0 pair ({i},{j}) should be bidirectional in directed adjacency"
                );
            }
        }
    }
}

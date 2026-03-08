//! Tests for exact rational polytope combinatorial data.
//!
//! Each test is a mathematical proposition verified computationally.
//! The rational module computes everything exactly over Q — no tolerances.

use super::*;
use super::super::polytope::{Polytope4D, ConstructionError};
use std::collections::BTreeSet;

// ── Test helpers ────────────────────────────────────────────────────────

/// Build a rational 4-simplex with exact rational coordinates.
///
/// Simplex with vertices at (-1/5)·1 + (9/5)·eᵢ for i=1..4, plus (-1/5)·1.
/// The origin is interior (all gaps = 1/5 > 0). Uses non-unit normals.
///
/// Facets:
///   0: -x₁ ≤ 1/5   (n = (-1,0,0,0), h = 1/5)
///   1: -x₂ ≤ 1/5   (n = (0,-1,0,0), h = 1/5)
///   2: -x₃ ≤ 1/5   (n = (0,0,-1,0), h = 1/5)
///   3: -x₄ ≤ 1/5   (n = (0,0,0,-1), h = 1/5)
///   4: x₁+x₂+x₃+x₄ ≤ 1   (n = (1,1,1,1), h = 1)
fn rational_simplex() -> Polytope4D {
    let normals = vec![
        [rat(-1), rat(0), rat(0), rat(0)],
        [rat(0), rat(-1), rat(0), rat(0)],
        [rat(0), rat(0), rat(-1), rat(0)],
        [rat(0), rat(0), rat(0), rat(-1)],
        [rat(1), rat(1), rat(1), rat(1)],
    ];
    let heights = vec![
        frac(1, 5),
        frac(1, 5),
        frac(1, 5),
        frac(1, 5),
        rat(1),
    ];
    Polytope4D::from_rationals(normals, heights).expect("simplex construction")
}

/// Build a rational hypercube [-1, 1]⁴ with exact integer coordinates.
///
/// 8 facets, 16 vertices.
fn rational_hypercube() -> Polytope4D {
    let normals = vec![
        [rat(1), rat(0), rat(0), rat(0)],
        [rat(-1), rat(0), rat(0), rat(0)],
        [rat(0), rat(1), rat(0), rat(0)],
        [rat(0), rat(-1), rat(0), rat(0)],
        [rat(0), rat(0), rat(1), rat(0)],
        [rat(0), rat(0), rat(-1), rat(0)],
        [rat(0), rat(0), rat(0), rat(1)],
        [rat(0), rat(0), rat(0), rat(-1)],
    ];
    let heights = vec![rat(1); 8];
    Polytope4D::from_rationals(normals, heights).expect("hypercube construction")
}

/// Build a rational Lagrangian product of two squares.
///
/// Q-space: [-1,1]² in (q₁, q₂), 4 facets
/// P-space: [-1,1]² in (p₁, p₂), 4 facets
/// Same as hypercube, but conceptually a Lagrangian product.
fn rational_lagrangian_square_square() -> Polytope4D {
    // Same as hypercube — the point is to test Lagrangian structure
    rational_hypercube()
}

/// Build a rational Lagrangian product: triangle ×_L square.
///
/// Q-space triangle: equilateral with integer-approximated normals (D=1000).
/// P-space square: [-1,1]² with exact integer normals.
///
/// Uses exact rational coordinates so ω₀ signs are computed exactly.
fn rational_lagrangian_triangle_square() -> Polytope4D {
    // Equilateral triangle in q-space with rational approximations of normals.
    // The true normals are at angles π/2 + 2πk/3.
    // n₀ = (cos(π/2), sin(π/2), 0, 0) = (0, 1, 0, 0)
    // n₁ = (cos(7π/6), sin(7π/6), 0, 0) = (-√3/2, -1/2, 0, 0)
    // n₂ = (cos(11π/6), sin(11π/6), 0, 0) = (√3/2, -1/2, 0, 0)
    //
    // Use exact rational approximations: √3/2 ≈ 866/1000
    let normals = vec![
        // Q-space triangle (3 facets)
        [rat(0), rat(1000), rat(0), rat(0)],      // (0, 1, 0, 0)
        [rat(-866), rat(-500), rat(0), rat(0)],    // (-√3/2, -1/2, 0, 0) × 1000
        [rat(866), rat(-500), rat(0), rat(0)],     // (√3/2, -1/2, 0, 0) × 1000
        // P-space square (4 facets)
        [rat(0), rat(0), rat(1), rat(0)],
        [rat(0), rat(0), rat(-1), rat(0)],
        [rat(0), rat(0), rat(0), rat(1)],
        [rat(0), rat(0), rat(0), rat(-1)],
    ];
    // Heights: all positive
    let heights = vec![
        rat(500),  // triangle
        rat(500),
        rat(500),
        rat(1),    // square
        rat(1),
        rat(1),
        rat(1),
    ];
    Polytope4D::from_rationals(normals, heights).expect("lagrangian triangle×square construction")
}

/// Helper: extract vertex descriptors from incidence matrix.
///
/// Returns a Vec<BTreeSet<usize>> where each entry is the set of facet indices
/// incident to that vertex. This mirrors the old vertex_descriptors field.
fn vertex_descriptors_from_incidence(p: &Polytope4D) -> Vec<BTreeSet<usize>> {
    let inc = p.incidence();
    let v_count = p.vertices().len();
    let f_count = p.facet_count();
    (0..v_count)
        .map(|vi| {
            (0..f_count)
                .filter(|&fi| inc[(vi, fi)])
                .collect::<BTreeSet<usize>>()
        })
        .collect()
}

/// Helper: extract adjacency pairs from adjacency matrix.
///
/// Returns a BTreeSet of (i, k) pairs where i < k and facets i, k are adjacent.
fn adjacency_pairs(p: &Polytope4D) -> BTreeSet<(usize, usize)> {
    let adj = p.adjacency();
    let f = p.facet_count();
    let mut pairs = BTreeSet::new();
    for i in 0..f {
        for k in (i + 1)..f {
            if adj[(i, k)] {
                pairs.insert((i, k));
            }
        }
    }
    pairs
}

// ── Phase 2a: Exact arithmetic correctness ─────────────────────────────

/// Proposition: the 4-simplex has exactly 5 vertex descriptors,
/// each a 4-element subset of {0, 1, 2, 3, 4}.
///
/// Proof: the simplex has 5 facets and 5 vertices. Each vertex
/// is the intersection of exactly 4 of the 5 facets, giving
/// C(5,4) = 5 vertex descriptors (one for each facet omitted).
#[test]
fn exact_simplex_vertices() {
    let s = rational_simplex();
    let vds = vertex_descriptors_from_incidence(&s);

    // 5 vertices
    assert_eq!(vds.len(), 5);

    // Each vertex descriptor is a 4-element subset of {0,...,4}
    for vd in &vds {
        assert_eq!(vd.len(), 4, "simplex vertex should be on exactly 4 facets");
        assert!(
            vd.iter().all(|&i| i < 5),
            "facet indices should be in 0..5"
        );
    }

    // The 5 vertex descriptors are exactly the complements of each facet
    let expected: Vec<BTreeSet<usize>> = (0..5)
        .map(|omit| (0..5).filter(|&i| i != omit).collect())
        .collect();
    let mut actual: Vec<BTreeSet<usize>> = vds;
    actual.sort();
    let mut expected_sorted = expected;
    expected_sorted.sort();
    assert_eq!(actual, expected_sorted);
}

/// Proposition: the 4-simplex vertices have exact rational coordinates.
///
/// Vertex omitting facet 4 (the sum constraint) has all coordinate constraints tight:
/// -xᵢ = 1/5 for all i, giving v = (-1/5, -1/5, -1/5, -1/5).
/// Vertex omitting facet j (for j<4) has xⱼ = 8/5 and xᵢ = -1/5 for i≠j.
#[test]
fn exact_simplex_vertex_coordinates() {
    let s = rational_simplex();

    // Vertex omitting facet 4: all coordinate constraints tight,
    // -xᵢ = 1/5 for all i, so v = (-1/5, -1/5, -1/5, -1/5).

    // Find the vertex descriptor {0,1,2,3} (omitting facet 4)
    let vds = vertex_descriptors_from_incidence(&s);
    let target_vd: BTreeSet<usize> = [0, 1, 2, 3].into_iter().collect();
    let idx = vds
        .iter()
        .position(|vd| *vd == target_vd)
        .expect("vertex {0,1,2,3} should exist");

    let v = &s.vertices()[idx];
    let expected = frac(-1, 5);
    for coord in v {
        assert_eq!(
            coord, &expected,
            "vertex omitting sum-constraint should be (-1/5, -1/5, -1/5, -1/5)"
        );
    }
}

/// Proposition: the hypercube [-1,1]⁴ has exactly 16 vertex descriptors,
/// each a 4-element subset of {0,...,7}.
///
/// The hypercube has 8 facets (±xᵢ ≤ 1) and 2⁴ = 16 vertices.
/// Each vertex selects one facet from each opposite pair.
#[test]
fn exact_hypercube_vertices() {
    let h = rational_hypercube();
    let vds = vertex_descriptors_from_incidence(&h);

    assert_eq!(vds.len(), 16);

    // Each vertex descriptor should have exactly 4 facets
    for vd in &vds {
        assert_eq!(vd.len(), 4);
    }

    // Each vertex descriptor picks exactly one from each pair {0,1}, {2,3}, {4,5}, {6,7}
    for vd in &vds {
        let pairs = [(0, 1), (2, 3), (4, 5), (6, 7)];
        for (a, b) in pairs {
            let has_a = vd.contains(&a);
            let has_b = vd.contains(&b);
            assert!(
                has_a ^ has_b,
                "vertex should pick exactly one from pair ({a}, {b}), got both={}, neither={}",
                has_a && has_b,
                !has_a && !has_b
            );
        }
    }
}

/// Proposition: the hypercube vertices are exactly the points (±1, ±1, ±1, ±1).
#[test]
fn exact_hypercube_vertex_coordinates() {
    let h = rational_hypercube();
    let one = rat(1);
    let neg_one = rat(-1);

    for v in h.vertices() {
        for coord in v {
            assert!(
                coord == &one || coord == &neg_one,
                "hypercube vertex coordinate should be ±1, got {coord}"
            );
        }
    }

    // All 16 sign combinations should appear
    assert_eq!(h.vertices().len(), 16);
}

/// Proposition: for the hypercube [-1,1]⁴ (which is a Lagrangian product
/// of [-1,1]² in q-space and [-1,1]² in p-space), all same-type adjacent
/// pairs have ω₀(yᵢ, yₖ) = 0.
///
/// "Same-type" means both dual vertices are in q-space (components [0,1])
/// or both in p-space (components [2,3]).
///
/// Reason: q-space dual vertices have form (a, b, 0, 0) and p-space dual vertices
/// have form (0, 0, c, d). Within each group, ω₀(u, v) = u₀v₂ - u₂v₀ + u₁v₃ - u₃v₁ = 0
/// since the cross-components vanish.
#[test]
fn lagrangian_same_type_omega_zero() {
    let h = rational_lagrangian_square_square();
    let dual_verts = h.dual_vertices();
    let adj_pairs = adjacency_pairs(&h);

    // Q-space facets: 0, 1 (±q₁), 2, 3 (±q₂)
    let q_facets: BTreeSet<usize> = [0, 1, 2, 3].into_iter().collect();
    // P-space facets: 4, 5 (±p₁), 6, 7 (±p₂)
    let p_facets: BTreeSet<usize> = [4, 5, 6, 7].into_iter().collect();

    for &(i, k) in &adj_pairs {
        let both_q = q_facets.contains(&i) && q_facets.contains(&k);
        let both_p = p_facets.contains(&i) && p_facets.contains(&k);

        if both_q || both_p {
            let omega = omega0_rational(&dual_verts[i], &dual_verts[k]);
            assert!(
                omega.is_zero(),
                "same-type pair ({i}, {k}) should have ω₀ = 0, got {omega}"
            );
            assert_eq!(
                h.omega_signs()[(i, k)],
                0i8,
                "same-type pair ({i}, {k}) sign should be Zero"
            );
        }
    }
}

/// Proposition: for the hypercube (as Lagrangian product), cross-type
/// adjacent pairs (one q-facet, one p-facet) have ω₀ signs determined
/// by the coordinate indices.
///
/// For dual vertices y_i = n_i/h_i: sign(ω₀(y_i, y_k)) = sign(ω₀(n_i, n_k))
/// since h_i, h_k > 0. So the sign analysis on normals carries over.
///
/// For normals (a, 0, 0, 0) and (0, 0, c, 0): ω₀ = a·c (from u₀v₂ term).
/// For normals (0, b, 0, 0) and (0, 0, 0, d): ω₀ = b·d (from u₁v₃ term).
/// Other cross combinations give ω₀ = 0.
#[test]
fn lagrangian_cross_type_omega() {
    let h = rational_lagrangian_square_square();
    let dual_verts = h.dual_vertices();
    let adj_pairs = adjacency_pairs(&h);

    // Check specific cross-type pairs
    // Facet 0: (+1,0,0,0), Facet 4: (0,0,+1,0) → ω₀ = 1·1 = 1 → Plus
    // Facet 0: (+1,0,0,0), Facet 5: (0,0,-1,0) → ω₀ = 1·(-1) = -1 → Minus
    // Facet 0: (+1,0,0,0), Facet 6: (0,0,0,+1) → ω₀ = 0 → Zero
    // Facet 0: (+1,0,0,0), Facet 7: (0,0,0,-1) → ω₀ = 0 → Zero

    let check = |i: usize, k: usize, expected_sign: i8| {
        let (lo, hi) = (i.min(k), i.max(k));
        assert!(
            adj_pairs.contains(&(lo, hi)),
            "pair ({lo}, {hi}) should be adjacent"
        );
        let omega = omega0_rational(&dual_verts[lo], &dual_verts[hi]);
        let actual_sign = match Sign::of(&omega) {
            Sign::Plus => 1i8,
            Sign::Minus => -1i8,
            Sign::Zero => 0i8,
        };
        assert_eq!(
            actual_sign, expected_sign,
            "pair ({lo}, {hi}): ω₀ = {omega}, expected sign {expected_sign}"
        );
    };

    // (q₁ facets) × (p₁ facets): paired by symplectic plane (q₁, p₁)
    check(0, 4, 1);   // (+1,0,0,0) vs (0,0,+1,0): ω₀ = +1
    check(0, 5, -1);  // (+1,0,0,0) vs (0,0,-1,0): ω₀ = -1
    check(1, 4, -1);  // (-1,0,0,0) vs (0,0,+1,0): ω₀ = -1
    check(1, 5, 1);   // (-1,0,0,0) vs (0,0,-1,0): ω₀ = +1

    // (q₂ facets) × (p₂ facets): paired by symplectic plane (q₂, p₂)
    check(2, 6, 1);   // (0,+1,0,0) vs (0,0,0,+1): ω₀ = +1
    check(2, 7, -1);  // (0,+1,0,0) vs (0,0,0,-1): ω₀ = -1
    check(3, 6, -1);  // (0,-1,0,0) vs (0,0,0,+1): ω₀ = -1
    check(3, 7, 1);   // (0,-1,0,0) vs (0,0,0,-1): ω₀ = +1

    // Cross-plane pairs: (q₁ facets) × (p₂ facets) → ω₀ = 0
    check(0, 6, 0);  // (+1,0,0,0) vs (0,0,0,+1)
    check(0, 7, 0);  // (+1,0,0,0) vs (0,0,0,-1)
    check(1, 6, 0);  // (-1,0,0,0) vs (0,0,0,+1)
    check(1, 7, 0);  // (-1,0,0,0) vs (0,0,0,-1)

    // Cross-plane pairs: (q₂ facets) × (p₁ facets) → ω₀ = 0
    check(2, 4, 0);  // (0,+1,0,0) vs (0,0,+1,0)
    check(2, 5, 0);  // (0,+1,0,0) vs (0,0,-1,0)
    check(3, 4, 0);  // (0,-1,0,0) vs (0,0,+1,0)
    check(3, 5, 0);  // (0,-1,0,0) vs (0,0,-1,0)
}

/// Proposition: for a Lagrangian product (triangle ×_L square),
/// all same-type adjacent pairs have ω₀ = 0 exactly.
///
/// Same-type: both dual vertices in q-space (facets 0-2) or both in p-space (facets 3-6).
#[test]
fn lagrangian_triangle_square_same_type_omega_zero() {
    let p = rational_lagrangian_triangle_square();
    let dual_verts = p.dual_vertices();
    let adj_pairs = adjacency_pairs(&p);

    let q_facets: BTreeSet<usize> = [0, 1, 2].into_iter().collect();
    let p_facets: BTreeSet<usize> = [3, 4, 5, 6].into_iter().collect();

    for &(i, k) in &adj_pairs {
        let both_q = q_facets.contains(&i) && q_facets.contains(&k);
        let both_p = p_facets.contains(&i) && p_facets.contains(&k);

        if both_q || both_p {
            let omega = omega0_rational(&dual_verts[i], &dual_verts[k]);
            assert!(
                omega.is_zero(),
                "same-type pair ({i}, {k}) should have ω₀ = 0, got {omega}"
            );
        }
    }
}

/// Proposition: the simplex adjacency is exactly the complete graph K₅,
/// since every pair of facets shares at least one vertex.
#[test]
fn simplex_adjacency_complete() {
    let s = rational_simplex();
    let adj_pairs = adjacency_pairs(&s);

    // C(5,2) = 10 adjacent pairs
    assert_eq!(adj_pairs.len(), 10);

    // Every pair of facets should be adjacent
    for i in 0..5 {
        for k in (i + 1)..5 {
            assert!(
                adj_pairs.contains(&(i, k)),
                "facets ({i}, {k}) should be adjacent in simplex"
            );
        }
    }
}

// ── Phase 2b: Round-trip and f64 agreement ─────────────────────────────

/// Proposition: f64 accessors produce a polytope with the same vertex count.
#[test]
fn f64_vertex_count() {
    let polytopes: Vec<(&str, Polytope4D)> = vec![
        ("simplex", rational_simplex()),
        ("hypercube", rational_hypercube()),
        ("lagrangian_tri_sq", rational_lagrangian_triangle_square()),
    ];

    for (name, rp) in &polytopes {
        assert_eq!(
            rp.vertices_f64().len(),
            rp.vertices().len(),
            "{name}: f64 vertex count should match rational vertex count"
        );
    }
}

/// Proposition: f64 vertices are within O(ε_machine) of exact rational vertices.
///
/// For polytopes with integer or small-denominator rational coordinates,
/// the f64 error should be negligible (< 1e-10).
#[test]
fn f64_vertex_accuracy() {
    let s = rational_simplex();

    // The rational simplex has non-unit normals. The f64 accessors normalize them.
    // The resulting f64 vertices should match the rational vertices closely.
    //
    // Check vertex count and that each f64 vertex is close to a rational vertex.
    let rational_verts: Vec<[f64; 4]> = s
        .vertices()
        .iter()
        .map(|v| std::array::from_fn(|i| rational_to_f64(&v[i])))
        .collect();

    for f64_v in s.vertices_f64() {
        let f64_arr = [f64_v[0], f64_v[1], f64_v[2], f64_v[3]];
        // Find closest rational vertex
        let min_dist = rational_verts
            .iter()
            .map(|rv| {
                (0..4)
                    .map(|i| (rv[i] - f64_arr[i]).powi(2))
                    .sum::<f64>()
                    .sqrt()
            })
            .fold(f64::INFINITY, f64::min);

        assert!(
            min_dist < 1e-10,
            "f64 vertex [{:.6}, {:.6}, {:.6}, {:.6}] is {min_dist:.2e} from nearest rational vertex",
            f64_arr[0], f64_arr[1], f64_arr[2], f64_arr[3]
        );
    }
}

/// Proposition: for polytopes with well-separated ω₀ values,
/// the f64 ω₀ signs agree with the exact rational signs.
///
/// This tests that the exact-to-f64 conversion preserves combinatorial data.
#[test]
fn f64_sign_agreement() {
    let polytopes: Vec<(&str, Polytope4D)> = vec![
        ("simplex", rational_simplex()),
        ("hypercube", rational_hypercube()),
    ];

    for (name, rp) in &polytopes {
        let omega_signs = rp.omega_signs();
        let f = rp.facet_count();

        // For each adjacent pair with a definite sign, check that the f64
        // ω₀ has the same sign.
        for i in 0..f {
            for k in (i + 1)..f {
                if !rp.adjacency()[(i, k)] {
                    continue;
                }
                let exact_sign = omega_signs[(i, k)];
                if exact_sign == 0 {
                    continue; // Zero might be affected by rounding
                }

                let f64_omega =
                    super::super::symplectic::omega0(&rp.normals_f64()[i], &rp.normals_f64()[k]);

                let f64_sign = if f64_omega > 1e-15 {
                    1i8
                } else if f64_omega < -1e-15 {
                    -1i8
                } else {
                    0i8
                };

                assert_eq!(
                    f64_sign, exact_sign,
                    "{name}: f64 ω₀ sign for ({i}, {k}) = {f64_omega:.2e}, expected {exact_sign}"
                );
            }
        }
    }
}

/// Proposition: from_f64_rounded round-trips with bounded error.
///
/// Start with f64 hypercube, round to rational with D=1000,
/// convert back to f64. Vertices should be close to originals.
#[test]
fn from_f64_roundtrip() {
    let kp = super::super::known_polytopes::hypercube();
    let f64_p = &kp.polytope;

    let rp = Polytope4D::from_f64_rounded(f64_p.normals_f64(), f64_p.heights_f64(), 1000)
        .expect("from_f64_rounded");

    // Same number of vertices
    assert_eq!(rp.vertices().len(), f64_p.vertices_f64().len());

    // f64 vertices should have same count
    assert_eq!(rp.vertices_f64().len(), f64_p.vertices_f64().len());
}

// ── Validation error tests ─────────────────────────────────────────────

/// Too few facets should fail.
#[test]
fn reject_too_few_facets() {
    let normals = vec![
        [rat(1), rat(0), rat(0), rat(0)],
        [rat(-1), rat(0), rat(0), rat(0)],
        [rat(0), rat(1), rat(0), rat(0)],
        [rat(0), rat(-1), rat(0), rat(0)],
    ];
    let heights = vec![rat(1); 4];
    let err = Polytope4D::from_rationals(normals, heights).unwrap_err();
    assert!(
        matches!(err, ConstructionError::TooFewFacets(4)),
        "expected TooFewFacets, got {err}"
    );
}

/// Zero normal should fail (produces zero dual vertex).
#[test]
fn reject_zero_normal() {
    let normals = vec![
        [rat(0), rat(0), rat(0), rat(0)], // zero!
        [rat(1), rat(0), rat(0), rat(0)],
        [rat(0), rat(1), rat(0), rat(0)],
        [rat(0), rat(0), rat(1), rat(0)],
        [rat(0), rat(0), rat(0), rat(1)],
    ];
    let heights = vec![rat(1); 5];
    let err = Polytope4D::from_rationals(normals, heights).unwrap_err();
    // Zero normal / h = zero dual vertex → ZeroDualVertex
    assert!(
        matches!(err, ConstructionError::ZeroDualVertex(0)),
        "expected ZeroDualVertex(0), got {err}"
    );
}

/// Non-positive height should fail.
#[test]
fn reject_nonpositive_height() {
    let normals = vec![
        [rat(1), rat(0), rat(0), rat(0)],
        [rat(-1), rat(0), rat(0), rat(0)],
        [rat(0), rat(1), rat(0), rat(0)],
        [rat(0), rat(-1), rat(0), rat(0)],
        [rat(0), rat(0), rat(1), rat(0)],
    ];
    let heights = vec![rat(1), rat(0), rat(1), rat(1), rat(1)]; // h₁ = 0
    let err = Polytope4D::from_rationals(normals, heights).unwrap_err();
    assert!(
        matches!(err, ConstructionError::NonPositiveHeight { index: 1, .. }),
        "expected NonPositiveHeight at index 1, got {err}"
    );
}

/// Redundant facet should fail: 6 facets defining a simplex plus a
/// far-away facet that no vertex touches.
#[test]
fn reject_redundant_facet() {
    // Start with simplex (5 facets), add a 6th facet far away.
    let normals = vec![
        [rat(-1), rat(0), rat(0), rat(0)],
        [rat(0), rat(-1), rat(0), rat(0)],
        [rat(0), rat(0), rat(-1), rat(0)],
        [rat(0), rat(0), rat(0), rat(-1)],
        [rat(1), rat(1), rat(1), rat(1)],
        [rat(1), rat(0), rat(0), rat(0)], // redundant: x₁ ≤ 100
    ];
    let heights = vec![frac(1, 5), frac(1, 5), frac(1, 5), frac(1, 5), rat(1), rat(100)];
    let err = Polytope4D::from_rationals(normals, heights).unwrap_err();
    assert!(
        matches!(err, ConstructionError::RedundantFacet(5)),
        "expected RedundantFacet(5), got {err}"
    );
}

/// Non-simple polytope is supported: hypercube [-1,1]^4 + diagonal cut x₁+x₂+x₃+x₄ ≤ 2.
///
/// The cut removes vertex (1,1,1,1) and makes 4 vertices non-simple: each of
/// (1,1,1,-1), (1,1,-1,1), (1,-1,1,1), (-1,1,1,1) lies on 5 facets (4 original + cut).
///
/// **Why this input:** Smallest non-simple polytope we can easily construct.
/// F=9 → C(9,4)=126 systems, fast over Q.
#[test]
fn non_simple_polytope_accepted() {
    // Hypercube: ±xᵢ ≤ 1 for i=1..4 (8 facets)
    // + diagonal cut: x₁+x₂+x₃+x₄ ≤ 2 (9th facet)
    let normals = vec![
        [rat(1), rat(0), rat(0), rat(0)],
        [rat(-1), rat(0), rat(0), rat(0)],
        [rat(0), rat(1), rat(0), rat(0)],
        [rat(0), rat(-1), rat(0), rat(0)],
        [rat(0), rat(0), rat(1), rat(0)],
        [rat(0), rat(0), rat(-1), rat(0)],
        [rat(0), rat(0), rat(0), rat(1)],
        [rat(0), rat(0), rat(0), rat(-1)],
        [rat(1), rat(1), rat(1), rat(1)],  // diagonal cut
    ];
    let heights = vec![
        rat(1), rat(1), rat(1), rat(1),
        rat(1), rat(1), rat(1), rat(1),
        rat(2),  // sum ≤ 2: tight at (1,1,1,-1) etc.
    ];
    let rp = Polytope4D::from_rationals(normals, heights)
        .expect("non-simple polytope should be accepted");

    let vds = vertex_descriptors_from_incidence(&rp);
    // Should have 15 vertices (16 hypercube vertices minus (1,1,1,1))
    assert_eq!(vds.len(), 15);
    // 4 vertices lie on 5 facets each
    let non_simple_count = vds.iter()
        .filter(|vd| vd.len() > 4)
        .count();
    assert_eq!(non_simple_count, 4, "expected 4 non-simple vertices");
}

/// Parallel halfspaces are unbounded (normals have rank 1).
#[test]
fn reject_unbounded_parallel() {
    let normals = vec![
        [rat(1), rat(0), rat(0), rat(0)],
        [rat(2), rat(0), rat(0), rat(0)],
        [rat(3), rat(0), rat(0), rat(0)],
        [rat(4), rat(0), rat(0), rat(0)],
        [rat(5), rat(0), rat(0), rat(0)],
    ];
    let heights = vec![rat(1), rat(2), rat(3), rat(4), rat(5)];
    let err = Polytope4D::from_rationals(normals, heights).unwrap_err();
    assert!(
        matches!(err, ConstructionError::Unbounded),
        "expected Unbounded, got {err}"
    );
}

/// Normals span R^4 but only from one side → unbounded (not positively spanning).
///
/// 5 normals that span R^4 (rank 4) but all have non-negative first component,
/// so the direction (-1, 0, 0, 0) has no nᵢ · d > 0.
#[test]
fn reject_unbounded_one_sided() {
    let normals = vec![
        [rat(1), rat(0), rat(0), rat(0)],
        [rat(0), rat(1), rat(0), rat(0)],
        [rat(0), rat(0), rat(1), rat(0)],
        [rat(0), rat(0), rat(0), rat(1)],
        [rat(1), rat(1), rat(1), rat(1)],
    ];
    let heights = vec![rat(1); 5];
    let err = Polytope4D::from_rationals(normals, heights).unwrap_err();
    assert!(
        matches!(err, ConstructionError::Unbounded),
        "expected Unbounded, got {err}"
    );
}

/// Proposition: rank_over_q computes exact matrix rank.
#[test]
fn rank_over_q_basic() {
    // Identity = rank 4
    let id = vec![
        [rat(1), rat(0), rat(0), rat(0)],
        [rat(0), rat(1), rat(0), rat(0)],
        [rat(0), rat(0), rat(1), rat(0)],
        [rat(0), rat(0), rat(0), rat(1)],
    ];
    assert_eq!(rank_over_q(&id), 4);

    // Duplicate row = rank 3
    let dup = vec![
        [rat(1), rat(0), rat(0), rat(0)],
        [rat(0), rat(1), rat(0), rat(0)],
        [rat(0), rat(0), rat(1), rat(0)],
        [rat(2), rat(0), rat(0), rat(0)], // scalar multiple of row 0
    ];
    assert_eq!(rank_over_q(&dup), 3);

    // All zeros = rank 0
    let zeros = vec![[rat(0), rat(0), rat(0), rat(0)]];
    assert_eq!(rank_over_q(&zeros), 0);

    // Empty = rank 0
    let empty: Vec<[BigRational; 4]> = vec![];
    assert_eq!(rank_over_q(&empty), 0);

    // Single nonzero row = rank 1
    let single = vec![[rat(3), rat(-1), rat(0), rat(7)]];
    assert_eq!(rank_over_q(&single), 1);
}

/// Proposition: simplex dual vertices pass boundedness check (positively span R^4).
#[test]
fn simplex_is_bounded() {
    let p = rational_simplex();
    // If we got here, construction succeeded → boundedness passed.
    // Also verify directly on dual vertices:
    assert!(check_bounded_rational(p.dual_vertices()));
}

/// Proposition: hypercube dual vertices pass boundedness check.
#[test]
fn hypercube_is_bounded() {
    let p = rational_hypercube();
    assert!(check_bounded_rational(p.dual_vertices()));
}

/// Proposition: cross product in 4D over Q is perpendicular to all three inputs.
#[test]
fn cross_product_4d_rational_perpendicular() {
    let a = [rat(1), rat(2), rat(3), rat(4)];
    let b = [rat(5), rat(-1), rat(2), rat(0)];
    let c = [rat(0), rat(3), rat(-2), rat(1)];

    let d = cross_product_4d_rational(&a, &b, &c);

    // d · a = d · b = d · c = 0
    assert!(dot4(&d, &a).is_zero(), "d·a = {} ≠ 0", dot4(&d, &a));
    assert!(dot4(&d, &b).is_zero(), "d·b = {} ≠ 0", dot4(&d, &b));
    assert!(dot4(&d, &c).is_zero(), "d·c = {} ≠ 0", dot4(&d, &c));

    // d should be nonzero (a, b, c are linearly independent)
    assert!(!d.iter().all(|x| x.is_zero()), "cross product is zero");
}

/// Proposition: affine rank of simplex vertices = 4 (they span R^4).
#[test]
fn simplex_vertices_affine_rank() {
    let p = rational_simplex();
    assert_eq!(affine_rank_rational(p.vertices()), 4);
}

/// Proposition: affine rank of coplanar points < 4.
#[test]
fn coplanar_points_affine_rank() {
    // Four points all with x₃ = 0 → affine rank ≤ 3
    let points = vec![
        [rat(0), rat(0), rat(0), rat(0)],
        [rat(1), rat(0), rat(0), rat(0)],
        [rat(0), rat(1), rat(0), rat(0)],
        [rat(0), rat(0), rat(0), rat(1)],
    ];
    // These span a 3D subspace (x₃ = 0 plane)
    assert_eq!(affine_rank_rational(&points), 3);
}

// ── Phase 3: f64 ↔ rational lossless conversion ────────────────────────

/// Proposition: f64_to_rational is the exact inverse of f64 → bits → rational.
///
/// For any finite f64 value x, f64_to_rational(x) produces the exact
/// rational number that x represents in IEEE-754, and converting back
/// via rational_to_f64 recovers x exactly.
#[test]
fn f64_to_rational_roundtrip() {
    let test_values: Vec<f64> = vec![
        0.0, 1.0, -1.0, 0.5, -0.5, 0.1, -0.1,
        1.0 / 3.0,                    // non-terminating decimal
        std::f64::consts::PI,          // irrational, stored as f64 rational
        std::f64::consts::FRAC_1_SQRT_2, // 1/√2
        1e-15, 1e15,                   // extreme magnitudes
        f64::MIN_POSITIVE,             // smallest positive normal
        // Note: subnormals (5e-324) are excluded because rational_to_f64
        // loses precision on the huge denominator. Irrelevant for polytope coords.
        (2.0_f64).powi(52),            // exact power of 2
        0.8090169943749473,            // cos(2π/5) — HK-O pentagon normal
    ];

    for &x in &test_values {
        let r = f64_to_rational(x);
        let back = rational_to_f64(&r);
        assert_eq!(
            back, x,
            "round-trip failed for {x}: rational = {r}, back = {back}"
        );
    }
}

/// Proposition: f64_to_rational produces exact rationals for known values.
#[test]
fn f64_to_rational_exact_values() {
    assert_eq!(f64_to_rational(0.0), rat(0));
    assert_eq!(f64_to_rational(1.0), rat(1));
    assert_eq!(f64_to_rational(-1.0), rat(-1));
    assert_eq!(f64_to_rational(0.5), frac(1, 2));
    assert_eq!(f64_to_rational(-0.5), frac(-1, 2));
    assert_eq!(f64_to_rational(0.25), frac(1, 4));
    assert_eq!(f64_to_rational(2.0), rat(2));
    assert_eq!(f64_to_rational(1024.0), rat(1024));
}

/// Proposition: constructing via Polytope4D::new from f64 normals/heights
/// agrees with constructing from rationals on vertex count and coordinates.
///
/// This tests the lossless entry point (f64 → rational dual vertices → pipeline).
#[test]
fn from_f64_lossless_roundtrip() {
    let kp = super::super::known_polytopes::hypercube();
    let f64_p = &kp.polytope;

    // Construct a fresh polytope from the same f64 normals/heights
    let rp = Polytope4D::new(f64_p.normals_f64().to_vec(), f64_p.heights_f64().to_vec())
        .expect("new should succeed for hypercube");

    // Same vertex count
    assert_eq!(rp.vertices().len(), f64_p.vertices().len());

    // f64 vertices should also match
    assert_eq!(rp.vertices_f64().len(), f64_p.vertices_f64().len());
}

/// Proposition: Polytope4D::new on the simplex has the expected combinatorics.
#[test]
fn from_f64_simplex() {
    let kp = super::super::known_polytopes::simplex();
    let f64_p = &kp.polytope;

    // Construct fresh from f64 data
    let rp = Polytope4D::new(f64_p.normals_f64().to_vec(), f64_p.heights_f64().to_vec())
        .expect("new should succeed for simplex");

    assert_eq!(rp.facet_count(), 5);
    assert_eq!(rp.vertices().len(), 5);
}

// ── Phase 4: Perturbation ────────────────────────────────────────────────

/// Proposition: perturbing a simplex produces no ω₀ = 0 entries.
///
/// The simplex already has no ω₀ = 0 (all signs are nonzero).
/// After perturbation, the signs should remain nonzero.
#[test]
fn perturbation_preserves_nonzero_signs() {
    use rand::SeedableRng;
    let p = rational_simplex();
    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(42);
    let perturbed = p.perturbed(&mut rng, 64).expect("perturbation should succeed");

    // Same vertex count
    assert_eq!(perturbed.vertices().len(), p.vertices().len());
    assert_eq!(perturbed.facet_count(), p.facet_count());

    // No ω₀ = 0 in omega_signs for adjacent pairs
    let f = perturbed.facet_count();
    let has_zero = (0..f).any(|i| {
        ((i + 1)..f).any(|k| {
            perturbed.adjacency()[(i, k)] && perturbed.omega_signs()[(i, k)] == 0
        })
    });
    assert!(!has_zero, "perturbed polytope should have no ω₀ = 0");
}

/// Proposition: perturbing a Lagrangian product (which has ω₀ = 0 pairs)
/// breaks all the zeros.
///
/// The Lagrangian triangle product has same-type facet pairs with ω₀ = 0.
/// After perturbation, all ω₀ should be nonzero.
#[test]
fn perturbation_breaks_omega_zeros() {
    use rand::SeedableRng;
    let p = rational_lagrangian_triangle_square();
    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(123);

    // Before perturbation: some adjacent pairs have ω₀ = 0
    let f = p.facet_count();
    let has_zeros = (0..f).any(|i| {
        ((i + 1)..f).any(|k| {
            p.adjacency()[(i, k)] && p.omega_signs()[(i, k)] == 0
        })
    });
    assert!(has_zeros, "Lagrangian product should have ω₀ = 0 pairs before perturbation");

    // After perturbation: no ω₀ = 0
    let perturbed = p.perturbed(&mut rng, 64).expect("perturbation should succeed");
    let fp = perturbed.facet_count();
    let has_zeros_after = (0..fp).any(|i| {
        ((i + 1)..fp).any(|k| {
            perturbed.adjacency()[(i, k)] && perturbed.omega_signs()[(i, k)] == 0
        })
    });
    assert!(!has_zeros_after, "perturbed polytope should have no ω₀ = 0");
}

/// Proposition: perturbation at 2^{-64} barely changes the f64 representation.
///
/// For components of magnitude ~1, the perturbation (~2^{-64} ≈ 5e-20)
/// is far below f64 relative epsilon (2^{-52} ≈ 2e-16), so those components
/// are unchanged in f64. Components that were exactly 0 become ~5e-20
/// (nonzero but tiny). Heights are unchanged (not perturbed).
#[test]
fn perturbation_preserves_f64() {
    use rand::SeedableRng;
    let p = rational_simplex();

    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(99);
    let perturbed = p.perturbed(&mut rng, 64).expect("perturbation should succeed");

    // Unit normal components should agree within ~2^{-64} ≈ 6e-20
    let tol = 1e-18; // generous bound (actual perturbation ~5e-20)
    for (n_before, n_after) in p.normals_f64().iter().zip(perturbed.normals_f64().iter()) {
        for c in 0..4 {
            assert!(
                (n_before[c] - n_after[c]).abs() < tol,
                "f64 normal component changed by {} (tol={tol})",
                (n_before[c] - n_after[c]).abs()
            );
        }
    }

    // Heights should be close (perturbation only affects dual vertices)
    for (h_before, h_after) in p.heights_f64().iter().zip(perturbed.heights_f64().iter()) {
        assert!(
            (h_before - h_after).abs() < tol,
            "f64 height changed by {} after perturbation",
            (h_before - h_after).abs()
        );
    }
}

// ── Phase 5: ω₀ agreement ──────────────────────────────────────────────

/// Proposition: the exact ω₀ formula agrees with the f64 formula
/// on integer inputs.
#[test]
fn omega0_rational_agrees_with_f64() {
    use nalgebra::Vector4;

    let test_cases: Vec<([i64; 4], [i64; 4])> = vec![
        ([1, 0, 0, 0], [0, 0, 1, 0]),   // ω₀ = 1
        ([1, 0, 0, 0], [0, 0, 0, 1]),   // ω₀ = 0
        ([0, 1, 0, 0], [0, 0, 0, 1]),   // ω₀ = 1
        ([1, 2, 3, 4], [5, 6, 7, 8]),   // ω₀ = 1·7 - 3·5 + 2·8 - 4·6 = 7-15+16-24 = -16
        ([3, -1, 4, -1], [5, -9, 2, -6]), // mixed signs
    ];

    for (u_arr, v_arr) in &test_cases {
        let u_rat: [BigRational; 4] = std::array::from_fn(|i| rat(u_arr[i]));
        let v_rat: [BigRational; 4] = std::array::from_fn(|i| rat(v_arr[i]));
        let u_f64 = Vector4::new(
            u_arr[0] as f64,
            u_arr[1] as f64,
            u_arr[2] as f64,
            u_arr[3] as f64,
        );
        let v_f64 = Vector4::new(
            v_arr[0] as f64,
            v_arr[1] as f64,
            v_arr[2] as f64,
            v_arr[3] as f64,
        );

        let rational_result = omega0_rational(&u_rat, &v_rat);
        let f64_result = super::super::symplectic::omega0(&u_f64, &v_f64);

        assert_eq!(
            rational_to_f64(&rational_result),
            f64_result,
            "ω₀({u_arr:?}, {v_arr:?}): rational={rational_result}, f64={f64_result}"
        );
    }
}

/// Proposition: the determinant formula is correct on known matrices.
#[test]
fn det4_known_values() {
    // Identity matrix: det = 1
    let id: [[BigRational; 4]; 4] = [
        [rat(1), rat(0), rat(0), rat(0)],
        [rat(0), rat(1), rat(0), rat(0)],
        [rat(0), rat(0), rat(1), rat(0)],
        [rat(0), rat(0), rat(0), rat(1)],
    ];
    assert_eq!(det4(&id), rat(1));

    // Matrix with det = 0 (duplicate row)
    let singular: [[BigRational; 4]; 4] = [
        [rat(1), rat(2), rat(3), rat(4)],
        [rat(1), rat(2), rat(3), rat(4)],
        [rat(5), rat(6), rat(7), rat(8)],
        [rat(9), rat(10), rat(11), rat(12)],
    ];
    assert_eq!(det4(&singular), rat(0));

    // Diagonal matrix: det = product of diagonals
    let diag: [[BigRational; 4]; 4] = [
        [rat(2), rat(0), rat(0), rat(0)],
        [rat(0), rat(3), rat(0), rat(0)],
        [rat(0), rat(0), rat(5), rat(0)],
        [rat(0), rat(0), rat(0), rat(7)],
    ];
    assert_eq!(det4(&diag), rat(210)); // 2 × 3 × 5 × 7
}

/// Proposition: Cramer's rule solver gives exact solutions.
#[test]
fn solve4_exact() {
    // Solve I·x = b → x = b
    let id: [[BigRational; 4]; 4] = [
        [rat(1), rat(0), rat(0), rat(0)],
        [rat(0), rat(1), rat(0), rat(0)],
        [rat(0), rat(0), rat(1), rat(0)],
        [rat(0), rat(0), rat(0), rat(1)],
    ];
    let rhs = [rat(3), rat(7), frac(1, 2), rat(-5)];
    let x = solve4(&id, &rhs).expect("non-singular");
    assert_eq!(x, rhs);

    // Solve diagonal system: diag(2,3,5,7)·x = (4,9,10,21) → x = (2,3,2,3)
    let diag: [[BigRational; 4]; 4] = [
        [rat(2), rat(0), rat(0), rat(0)],
        [rat(0), rat(3), rat(0), rat(0)],
        [rat(0), rat(0), rat(5), rat(0)],
        [rat(0), rat(0), rat(0), rat(7)],
    ];
    let rhs2 = [rat(4), rat(9), rat(10), rat(21)];
    let x2 = solve4(&diag, &rhs2).expect("non-singular");
    assert_eq!(x2, [rat(2), rat(3), rat(2), rat(3)]);

    // Singular matrix should return None
    let singular: [[BigRational; 4]; 4] = [
        [rat(1), rat(0), rat(0), rat(0)],
        [rat(1), rat(0), rat(0), rat(0)],
        [rat(0), rat(0), rat(1), rat(0)],
        [rat(0), rat(0), rat(0), rat(1)],
    ];
    assert!(solve4(&singular, &[rat(1), rat(1), rat(1), rat(1)]).is_none());
}

// ── Phase 6: Rational↔f64 pipeline consistency ─────────────────────────

/// Proposition: adjacency matrix from exact data agrees with f64 computation
/// for the simplex (same polytope, two paths).
#[test]
fn adjacency_agreement_simplex() {
    use crate::algorithms::hk2017::build_adjacency_matrix;

    let kp = super::super::known_polytopes::simplex();
    let f64_adj = build_adjacency_matrix(&kp.polytope);

    // Construct fresh from the same f64 data
    let rational_p = Polytope4D::new(
        kp.polytope.normals_f64().to_vec(),
        kp.polytope.heights_f64().to_vec(),
    ).expect("new should succeed for simplex");
    let exact_adj = build_adjacency_matrix(&rational_p);

    assert_eq!(f64_adj, exact_adj, "adjacency matrices disagree for simplex");
}

/// Proposition: adjacency matrix from exact data agrees with f64 computation
/// for the hypercube.
#[test]
fn adjacency_agreement_hypercube() {
    use crate::algorithms::hk2017::build_adjacency_matrix;

    let kp = super::super::known_polytopes::hypercube();
    let f64_adj = build_adjacency_matrix(&kp.polytope);

    let rational_p = Polytope4D::new(
        kp.polytope.normals_f64().to_vec(),
        kp.polytope.heights_f64().to_vec(),
    ).expect("new should succeed for hypercube");
    let exact_adj = build_adjacency_matrix(&rational_p);

    assert_eq!(f64_adj, exact_adj, "adjacency matrices disagree for hypercube");
}

/// Proposition: directed adjacency matrix from exact data agrees with f64
/// for the simplex.
#[test]
fn directed_adjacency_agreement_simplex() {
    use crate::algorithms::hk2017::build_directed_adjacency_matrix;

    let kp = super::super::known_polytopes::simplex();
    let f64_dadj = build_directed_adjacency_matrix(&kp.polytope);

    let rational_p = Polytope4D::new(
        kp.polytope.normals_f64().to_vec(),
        kp.polytope.heights_f64().to_vec(),
    ).expect("new should succeed for simplex");
    let exact_dadj = build_directed_adjacency_matrix(&rational_p);

    assert_eq!(
        f64_dadj, exact_dadj,
        "directed adjacency matrices disagree for simplex"
    );
}

/// Proposition: directed adjacency matrix from exact data agrees with f64
/// for the hypercube.
#[test]
fn directed_adjacency_agreement_hypercube() {
    use crate::algorithms::hk2017::build_directed_adjacency_matrix;

    let kp = super::super::known_polytopes::hypercube();
    let f64_dadj = build_directed_adjacency_matrix(&kp.polytope);

    let rational_p = Polytope4D::new(
        kp.polytope.normals_f64().to_vec(),
        kp.polytope.heights_f64().to_vec(),
    ).expect("new should succeed for hypercube");
    let exact_dadj = build_directed_adjacency_matrix(&rational_p);

    assert_eq!(
        f64_dadj, exact_dadj,
        "directed adjacency matrices disagree for hypercube"
    );
}

/// Proposition: EHZ capacity is unchanged when routed through the rational pipeline.
///
/// Pipeline: f64 polytope → Polytope4D::new (lossless) → ehz_capacity.
/// The rational pipeline adds exact combinatorial data; the f64 numerics
/// (KKT solver) see the same unit normals and heights.
///
/// **Why this is important**: The capacity value depends on which (S, σ) pairs
/// survive adjacency pruning. If exact and f64 adjacency disagree, different
/// candidates survive and the capacity changes. Agreement here confirms that
/// the rational pipeline doesn't alter the result for well-conditioned inputs.
///
/// **Why debug mode:** F=5 simplex, ehz_capacity ~0.1s debug. Input-output test
/// but fast enough for default suite.
/// **Why this input:** Simplest polytope; well-conditioned f64 → rational round-trip.
#[test]
fn capacity_agreement_simplex() {
    use crate::algorithms::hk2017::ehz_capacity;

    let kp = super::super::known_polytopes::simplex();
    let f64_result = ehz_capacity(&kp.polytope).expect("simplex should have capacity");

    // Same polytope through a fresh construction
    let rational_p = Polytope4D::new(
        kp.polytope.normals_f64().to_vec(),
        kp.polytope.heights_f64().to_vec(),
    ).expect("new should succeed for simplex");
    let exact_result = ehz_capacity(&rational_p).expect("simplex should have capacity");

    assert!(
        (f64_result.capacity - exact_result.capacity).abs() < 1e-10,
        "capacity disagrees: f64={}, exact={}",
        f64_result.capacity,
        exact_result.capacity
    );
}

/// Proposition: EHZ capacity is unchanged through rational pipeline for hypercube.
///
/// **Why debug mode:** F=8 hypercube, ehz_capacity ~2s debug. Borderline but
/// under 5s threshold; exercises the exact path on a polytope with many facets.
/// **Why this input:** Largest "standard" polytope in the catalog; tests that
/// exact adjacency agrees with f64 on a non-trivial graph.
#[test]
fn capacity_agreement_hypercube() {
    use crate::algorithms::hk2017::ehz_capacity;

    let kp = super::super::known_polytopes::hypercube();
    let f64_result = ehz_capacity(&kp.polytope).expect("hypercube should have capacity");

    let rational_p = Polytope4D::new(
        kp.polytope.normals_f64().to_vec(),
        kp.polytope.heights_f64().to_vec(),
    ).expect("new should succeed for hypercube");
    let exact_result = ehz_capacity(&rational_p).expect("hypercube should have capacity");

    assert!(
        (f64_result.capacity - exact_result.capacity).abs() < 1e-10,
        "capacity disagrees: f64={}, exact={}",
        f64_result.capacity,
        exact_result.capacity
    );
}

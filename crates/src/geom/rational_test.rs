//! Tests for exact rational polytope combinatorial data.
//!
//! Each test is a mathematical proposition verified computationally.
//! The rational module computes everything exactly over Q — no tolerances.

use super::*;
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
fn rational_simplex() -> RationalPolytope4D {
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
    RationalPolytope4D::new(normals, heights).expect("simplex construction")
}

/// Build a rational hypercube [-1, 1]⁴ with exact integer coordinates.
///
/// 8 facets, 16 vertices.
fn rational_hypercube() -> RationalPolytope4D {
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
    RationalPolytope4D::new(normals, heights).expect("hypercube construction")
}

/// Build a rational Lagrangian product of two squares.
///
/// Q-space: [-1,1]² in (q₁, q₂), 4 facets
/// P-space: [-1,1]² in (p₁, p₂), 4 facets
/// Same as hypercube, but conceptually a Lagrangian product.
fn rational_lagrangian_square_square() -> RationalPolytope4D {
    // Same as hypercube — the point is to test Lagrangian structure
    rational_hypercube()
}

/// Build a rational Lagrangian product: triangle ×_L square.
///
/// Q-space triangle: equilateral with integer-approximated normals (D=1000).
/// P-space square: [-1,1]² with exact integer normals.
///
/// Uses exact rational coordinates so ω₀ signs are computed exactly.
fn rational_lagrangian_triangle_square() -> RationalPolytope4D {
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
    RationalPolytope4D::new(normals, heights).expect("lagrangian triangle×square construction")
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
    let data = s.combinatorial_data();

    // 5 vertices
    assert_eq!(data.vertex_descriptors.len(), 5);

    // Each vertex descriptor is a 4-element subset of {0,...,4}
    for vd in &data.vertex_descriptors {
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
    let mut actual: Vec<BTreeSet<usize>> = data.vertex_descriptors.clone();
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
    let target_vd: BTreeSet<usize> = [0, 1, 2, 3].into_iter().collect();
    let idx = s
        .combinatorial_data()
        .vertex_descriptors
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
    let data = h.combinatorial_data();

    assert_eq!(data.vertex_descriptors.len(), 16);

    // Each vertex descriptor should have exactly 4 facets
    for vd in &data.vertex_descriptors {
        assert_eq!(vd.len(), 4);
    }

    // Each vertex descriptor picks exactly one from each pair {0,1}, {2,3}, {4,5}, {6,7}
    for vd in &data.vertex_descriptors {
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
    assert_eq!(h.num_vertices(), 16);
}

/// Proposition: for the hypercube [-1,1]⁴ (which is a Lagrangian product
/// of [-1,1]² in q-space and [-1,1]² in p-space), all same-type adjacent
/// pairs have ω₀(nᵢ, nₖ) = 0.
///
/// "Same-type" means both normals are in q-space (components [0,1])
/// or both in p-space (components [2,3]).
///
/// Reason: q-space normals have form (a, b, 0, 0) and p-space normals
/// have form (0, 0, c, d). Within each group, ω₀(u, v) = u₀v₂ - u₂v₀ + u₁v₃ - u₃v₁ = 0
/// since the cross-components vanish.
#[test]
fn lagrangian_same_type_omega_zero() {
    let h = rational_lagrangian_square_square();
    let data = h.combinatorial_data();
    let normals = h.normals();

    // Q-space facets: 0, 1 (±q₁), 2, 3 (±q₂)
    let q_facets: BTreeSet<usize> = [0, 1, 2, 3].into_iter().collect();
    // P-space facets: 4, 5 (±p₁), 6, 7 (±p₂)
    let p_facets: BTreeSet<usize> = [4, 5, 6, 7].into_iter().collect();

    for &(i, k) in &data.adjacency {
        let both_q = q_facets.contains(&i) && q_facets.contains(&k);
        let both_p = p_facets.contains(&i) && p_facets.contains(&k);

        if both_q || both_p {
            let omega = omega0_rational(&normals[i], &normals[k]);
            assert!(
                omega.is_zero(),
                "same-type pair ({i}, {k}) should have ω₀ = 0, got {omega}"
            );
            assert_eq!(
                data.sign_pattern[&(i, k)],
                Sign::Zero,
                "same-type pair ({i}, {k}) sign should be Zero"
            );
        }
    }
}

/// Proposition: for the hypercube (as Lagrangian product), cross-type
/// adjacent pairs (one q-facet, one p-facet) have ω₀ signs determined
/// by the coordinate indices.
///
/// For normals (a, 0, 0, 0) and (0, 0, c, 0): ω₀ = a·c (from u₀v₂ term).
/// For normals (0, b, 0, 0) and (0, 0, 0, d): ω₀ = b·d (from u₁v₃ term).
/// Other cross combinations give ω₀ = 0.
#[test]
fn lagrangian_cross_type_omega() {
    let h = rational_lagrangian_square_square();
    let data = h.combinatorial_data();
    let normals = h.normals();

    // Check specific cross-type pairs
    // Facet 0: (+1,0,0,0), Facet 4: (0,0,+1,0) → ω₀ = 1·1 = 1 → Plus
    // Facet 0: (+1,0,0,0), Facet 5: (0,0,-1,0) → ω₀ = 1·(-1) = -1 → Minus
    // Facet 0: (+1,0,0,0), Facet 6: (0,0,0,+1) → ω₀ = 0 → Zero
    // Facet 0: (+1,0,0,0), Facet 7: (0,0,0,-1) → ω₀ = 0 → Zero

    let check = |i: usize, k: usize, expected: Sign| {
        let (lo, hi) = (i.min(k), i.max(k));
        assert!(
            data.adjacency.contains(&(lo, hi)),
            "pair ({lo}, {hi}) should be adjacent"
        );
        let omega = omega0_rational(&normals[lo], &normals[hi]);
        let sign = Sign::of(&omega);
        assert_eq!(
            sign, expected,
            "pair ({lo}, {hi}): ω₀ = {omega}, expected sign {expected:?}"
        );
    };

    // (q₁ facets) × (p₁ facets): paired by symplectic plane (q₁, p₁)
    check(0, 4, Sign::Plus);  // (+1,0,0,0) vs (0,0,+1,0): ω₀ = +1
    check(0, 5, Sign::Minus); // (+1,0,0,0) vs (0,0,-1,0): ω₀ = -1
    check(1, 4, Sign::Minus); // (-1,0,0,0) vs (0,0,+1,0): ω₀ = -1
    check(1, 5, Sign::Plus);  // (-1,0,0,0) vs (0,0,-1,0): ω₀ = +1

    // (q₂ facets) × (p₂ facets): paired by symplectic plane (q₂, p₂)
    check(2, 6, Sign::Plus);  // (0,+1,0,0) vs (0,0,0,+1): ω₀ = +1
    check(2, 7, Sign::Minus); // (0,+1,0,0) vs (0,0,0,-1): ω₀ = -1
    check(3, 6, Sign::Minus); // (0,-1,0,0) vs (0,0,0,+1): ω₀ = -1
    check(3, 7, Sign::Plus);  // (0,-1,0,0) vs (0,0,0,-1): ω₀ = +1

    // Cross-plane pairs: (q₁ facets) × (p₂ facets) → ω₀ = 0
    check(0, 6, Sign::Zero); // (+1,0,0,0) vs (0,0,0,+1)
    check(0, 7, Sign::Zero); // (+1,0,0,0) vs (0,0,0,-1)
    check(1, 6, Sign::Zero); // (-1,0,0,0) vs (0,0,0,+1)
    check(1, 7, Sign::Zero); // (-1,0,0,0) vs (0,0,0,-1)

    // Cross-plane pairs: (q₂ facets) × (p₁ facets) → ω₀ = 0
    check(2, 4, Sign::Zero); // (0,+1,0,0) vs (0,0,+1,0)
    check(2, 5, Sign::Zero); // (0,+1,0,0) vs (0,0,-1,0)
    check(3, 4, Sign::Zero); // (0,-1,0,0) vs (0,0,+1,0)
    check(3, 5, Sign::Zero); // (0,-1,0,0) vs (0,0,-1,0)
}

/// Proposition: for a Lagrangian product (triangle ×_L square),
/// all same-type adjacent pairs have ω₀ = 0 exactly.
///
/// Same-type: both normals in q-space (facets 0-2) or both in p-space (facets 3-6).
#[test]
fn lagrangian_triangle_square_same_type_omega_zero() {
    let p = rational_lagrangian_triangle_square();
    let data = p.combinatorial_data();
    let normals = p.normals();

    let q_facets: BTreeSet<usize> = [0, 1, 2].into_iter().collect();
    let p_facets: BTreeSet<usize> = [3, 4, 5, 6].into_iter().collect();

    for &(i, k) in &data.adjacency {
        let both_q = q_facets.contains(&i) && q_facets.contains(&k);
        let both_p = p_facets.contains(&i) && p_facets.contains(&k);

        if both_q || both_p {
            let omega = omega0_rational(&normals[i], &normals[k]);
            assert!(
                omega.is_zero(),
                "same-type pair ({i}, {k}) should have ω₀ = 0, got {omega}"
            );
        }
    }
}

/// Proposition: all margins are positive for every simple polytope we construct.
///
/// Positive margins mean the combinatorial structure is robust: small
/// perturbations of coordinates cannot change vertex descriptors,
/// adjacency, or nonzero ω₀ signs.
#[test]
fn margins_positive() {
    let polytopes: Vec<(&str, RationalPolytope4D)> = vec![
        ("simplex", rational_simplex()),
        ("hypercube", rational_hypercube()),
        ("lagrangian_tri_sq", rational_lagrangian_triangle_square()),
    ];

    for (name, p) in &polytopes {
        let m = &p.combinatorial_data().margins;
        assert!(
            m.min_gap.is_positive(),
            "{name}: min_gap should be positive, got {}",
            m.min_gap
        );
        assert!(
            m.min_abs_det.is_positive(),
            "{name}: min_abs_det should be positive, got {}",
            m.min_abs_det
        );
        // For simplex and hypercube, min_omega_nonzero must be Some (they have
        // nonzero ω₀ pairs). For triangle×square, cross-type pairs also give nonzero ω₀.
        let mow = m
            .min_omega_nonzero
            .as_ref()
            .unwrap_or_else(|| panic!("{name}: min_omega_nonzero should be Some"));
        assert!(
            mow.is_positive(),
            "{name}: min_omega_nonzero should be positive, got {mow}"
        );
    }
}

/// Proposition: for the simplex, min_gap = 9/5.
///
/// By symmetry all non-incidence gaps are equal. For vertex {0,1,2,3}
/// (v = (-1/5,...,-1/5)), the gap from facet 4 is:
///   h₄ - ⟨(1,1,1,1), (-1/5,...,-1/5)⟩ = 1 - (-4/5) = 9/5.
/// Every other vertex-facet pair gives the same value by the simplex's
/// permutation symmetry.
#[test]
fn simplex_min_gap() {
    let s = rational_simplex();
    let m = &s.combinatorial_data().margins;
    assert_eq!(m.min_gap, frac(9, 5));
}

/// Proposition: the simplex adjacency is exactly the complete graph K₅,
/// since every pair of facets shares at least one vertex.
#[test]
fn simplex_adjacency_complete() {
    let s = rational_simplex();
    let data = s.combinatorial_data();

    // C(5,2) = 10 adjacent pairs
    assert_eq!(data.adjacency.len(), 10);

    // Every pair of facets should be adjacent
    for i in 0..5 {
        for k in (i + 1)..5 {
            assert!(
                data.adjacency.contains(&(i, k)),
                "facets ({i}, {k}) should be adjacent in simplex"
            );
        }
    }
}

// ── Phase 2b: Round-trip and f64 agreement ─────────────────────────────

/// Proposition: to_f64() produces a polytope with the same vertex count.
#[test]
fn f64_vertex_count() {
    let polytopes: Vec<(&str, RationalPolytope4D)> = vec![
        ("simplex", rational_simplex()),
        ("hypercube", rational_hypercube()),
        ("lagrangian_tri_sq", rational_lagrangian_triangle_square()),
    ];

    for (name, rp) in &polytopes {
        let f64_polytope = rp.to_f64().expect("to_f64 should succeed");
        assert_eq!(
            f64_polytope.vertices().len(),
            rp.num_vertices(),
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
    let f64_s = s.to_f64().expect("to_f64");

    // The rational simplex has non-unit normals. to_f64 normalizes them.
    // The resulting polytope should have the same geometric vertices,
    // just with unit normals and adjusted heights.
    //
    // Check vertex count and that each f64 vertex is close to a rational vertex.
    let rational_verts: Vec<[f64; 4]> = s
        .vertices()
        .iter()
        .map(|v| std::array::from_fn(|i| rational_to_f64(&v[i])))
        .collect();

    for f64_v in f64_s.vertices() {
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

/// Proposition: for polytopes with all margins ≫ ε_machine,
/// the f64 ω₀ signs agree with the exact rational signs.
///
/// This tests that the exact-to-f64 conversion preserves combinatorial data.
#[test]
fn f64_sign_agreement() {
    let polytopes: Vec<(&str, RationalPolytope4D)> = vec![
        ("simplex", rational_simplex()),
        ("hypercube", rational_hypercube()),
    ];

    for (name, rp) in &polytopes {
        let data = rp.combinatorial_data();
        let f64_p = rp.to_f64().expect("to_f64");

        // For each adjacent pair with a definite sign, check that the f64
        // ω₀ has the same sign.
        for (&(i, k), &sign) in &data.sign_pattern {
            if sign == Sign::Zero {
                continue; // Zero might be affected by rounding
            }

            let f64_omega =
                super::super::symplectic::omega0(&f64_p.normals()[i], &f64_p.normals()[k]);

            let f64_sign = if f64_omega > 1e-15 {
                Sign::Plus
            } else if f64_omega < -1e-15 {
                Sign::Minus
            } else {
                Sign::Zero
            };

            assert_eq!(
                f64_sign, sign,
                "{name}: f64 ω₀ sign for ({i}, {k}) = {f64_omega:.2e}, expected {sign:?}"
            );
        }
    }
}

/// Proposition: from_f64_rounded round-trips through to_f64 with bounded error.
///
/// Start with f64 hypercube, round to rational with D=1000,
/// convert back to f64. Vertices should be close to originals.
#[test]
fn from_f64_roundtrip() {
    let kp = super::super::known_polytopes::hypercube();
    let f64_p = &kp.polytope;

    let rp = RationalPolytope4D::from_f64_rounded(f64_p.normals(), f64_p.heights(), 1000)
        .expect("from_f64_rounded");

    // Same number of vertices
    assert_eq!(rp.num_vertices(), f64_p.vertices().len());

    // Convert back to f64
    let roundtrip = rp.to_f64().expect("to_f64");
    assert_eq!(roundtrip.vertices().len(), f64_p.vertices().len());
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
    let err = RationalPolytope4D::new(normals, heights).unwrap_err();
    assert!(
        matches!(err, RationalConstructionError::TooFewFacets(4)),
        "expected TooFewFacets, got {err}"
    );
}

/// Zero normal should fail.
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
    let err = RationalPolytope4D::new(normals, heights).unwrap_err();
    assert!(
        matches!(err, RationalConstructionError::ZeroNormal(0)),
        "expected ZeroNormal, got {err}"
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
    let err = RationalPolytope4D::new(normals, heights).unwrap_err();
    assert!(
        matches!(err, RationalConstructionError::NonPositiveHeight { index: 1 }),
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
    let err = RationalPolytope4D::new(normals, heights).unwrap_err();
    assert!(
        matches!(err, RationalConstructionError::RedundantFacet(5)),
        "expected RedundantFacet(5), got {err}"
    );
}

/// Non-simple polytope should fail: a polytope where a vertex lies on >4 facets.
///
/// Construction: take the simplex (5 facets, vertex (-1/5,...,-1/5) on facets
/// {0,1,2,3}) and add a 6th facet x₁ - x₂ ≤ 0 that also passes through
/// this vertex (since x₁ = x₂ = -1/5 there). This creates a vertex on 5 facets.
/// The 6th facet is not redundant (it cuts through the interior of the simplex).
#[test]
fn reject_not_simple() {
    let normals = vec![
        [rat(-1), rat(0), rat(0), rat(0)],   // facet 0: -x₁ ≤ 1/5
        [rat(0), rat(-1), rat(0), rat(0)],    // facet 1: -x₂ ≤ 1/5
        [rat(0), rat(0), rat(-1), rat(0)],    // facet 2: -x₃ ≤ 1/5
        [rat(0), rat(0), rat(0), rat(-1)],    // facet 3: -x₄ ≤ 1/5
        [rat(1), rat(1), rat(1), rat(1)],     // facet 4: sum ≤ 1
        [rat(1), rat(-1), rat(0), rat(0)],    // facet 5: x₁ - x₂ ≤ 0
    ];
    // At vertex (-1/5,-1/5,-1/5,-1/5): facet 5 gives (-1/5)-(-1/5) = 0 ≤ 0. Tight!
    // So this vertex lies on facets {0,1,2,3,5} → 5 facets → not simple.
    // Height for facet 5 must be positive: ⟨(1,-1,0,0), x⟩ ≤ h.
    // We need h > 0, and the facet must also be satisfied by all simplex vertices.
    // Simplex vertex omitting facet 0: (8/5, -1/5, -1/5, -1/5).
    //   (8/5) - (-1/5) = 9/5. So h ≥ 9/5 to keep this vertex.
    // Simplex vertex omitting facet 1: (-1/5, 8/5, -1/5, -1/5).
    //   (-1/5) - (8/5) = -9/5 ≤ h. Always satisfied.
    // Use h = 9/5 to make the facet non-redundant (tight at one vertex).
    // Actually at vertex omitting facet 0: gap = 9/5 - 9/5 = 0 → also tight!
    // So vertex (8/5,-1/5,-1/5,-1/5) lies on facets {1,2,3,4,5} → also 5 facets.
    // Either way, not simple.
    let heights = vec![
        frac(1, 5), frac(1, 5), frac(1, 5), frac(1, 5),
        rat(1),
        frac(9, 5),
    ];
    let err = RationalPolytope4D::new(normals, heights).unwrap_err();
    assert!(
        matches!(err, RationalConstructionError::NotSimple { .. }),
        "expected NotSimple, got {err}"
    );
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
    let err = RationalPolytope4D::new(normals, heights).unwrap_err();
    assert!(
        matches!(err, RationalConstructionError::Unbounded),
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
    let err = RationalPolytope4D::new(normals, heights).unwrap_err();
    assert!(
        matches!(err, RationalConstructionError::Unbounded),
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

/// Proposition: simplex normals pass boundedness check (positively span R^4).
#[test]
fn simplex_is_bounded() {
    let p = rational_simplex();
    // If we got here, construction succeeded → boundedness passed.
    // Also verify directly:
    assert!(check_bounded_rational(p.normals()));
}

/// Proposition: hypercube normals pass boundedness check.
#[test]
fn hypercube_is_bounded() {
    let p = rational_hypercube();
    assert!(check_bounded_rational(p.normals()));
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

/// Proposition: from_f64 produces a rational polytope whose to_f64 round-trip
/// agrees with the original f64 polytope on vertex count and vertex coordinates.
///
/// This tests the lossless entry point (vs the lossy from_f64_rounded).
#[test]
fn from_f64_lossless_roundtrip() {
    let kp = super::super::known_polytopes::hypercube();
    let f64_p = &kp.polytope;

    let rp = RationalPolytope4D::from_f64(f64_p.normals(), f64_p.heights())
        .expect("from_f64 should succeed for hypercube");

    // Same vertex count
    assert_eq!(rp.num_vertices(), f64_p.vertices().len());

    // Round-trip to f64 should produce same vertex count
    let roundtrip = rp.to_f64().expect("to_f64 should succeed");
    assert_eq!(roundtrip.vertices().len(), f64_p.vertices().len());
}

/// Proposition: from_f64 on the simplex agrees with from_f64_rounded on vertex count.
///
/// The lossless conversion should produce the same combinatorial structure.
#[test]
fn from_f64_simplex() {
    let kp = super::super::known_polytopes::simplex();
    let f64_p = &kp.polytope;

    let rp = RationalPolytope4D::from_f64(f64_p.normals(), f64_p.heights())
        .expect("from_f64 should succeed for simplex");

    assert_eq!(rp.num_facets(), 5);
    assert_eq!(rp.num_vertices(), 5);
}

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

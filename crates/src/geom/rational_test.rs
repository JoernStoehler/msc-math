//! Tests for rational utility functions and Polytope4D integration.
//!
//! Tests here cover: Sign, omega0_rational, f64_to_rational, rational_to_f64,
//! perturbation, and Polytope4D pipeline consistency (adjacency, capacity).

use super::*;
use crate::geom::polytope::Polytope4D;
use std::collections::BTreeSet;

// ── Test helpers ────────────────────────────────────────────────────────

/// Build a rational 4-simplex (5 facets, 5 vertices, origin interior).
///
/// Facets: n_i = -e_i (i=0..3) with h=1/5, plus n_4 = (1,1,1,1) with h=1.
/// Origin is interior because all h_i > 0.
fn rational_simplex() -> Polytope4D {
    let normals = vec![
        [rat(-1), rat(0), rat(0), rat(0)],
        [rat(0), rat(-1), rat(0), rat(0)],
        [rat(0), rat(0), rat(-1), rat(0)],
        [rat(0), rat(0), rat(0), rat(-1)],
        [rat(1), rat(1), rat(1), rat(1)],
    ];
    let heights = vec![frac(1, 5), frac(1, 5), frac(1, 5), frac(1, 5), rat(1)];
    Polytope4D::from_rationals(normals, heights).expect("simplex construction")
}

/// Build a rational hypercube [-1, 1]⁴ (8 facets, 16 vertices).
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

/// Build a rational Lagrangian product of two squares (= hypercube).
fn rational_lagrangian_square_square() -> Polytope4D { rational_hypercube() }

/// Build a rational Lagrangian product: triangle ×_L square.
///
/// 7 facets: 3 triangle normals in q-space (scaled integers, h=500),
/// 4 axis-aligned square normals in p-space (h=1).
/// This is a Lagrangian product because q-normals have zero p-components and vice versa.
fn rational_lagrangian_triangle_square() -> Polytope4D {
    let normals = vec![
        [rat(0), rat(1000), rat(0), rat(0)],
        [rat(-866), rat(-500), rat(0), rat(0)],
        [rat(866), rat(-500), rat(0), rat(0)],
        [rat(0), rat(0), rat(1), rat(0)],
        [rat(0), rat(0), rat(-1), rat(0)],
        [rat(0), rat(0), rat(0), rat(1)],
        [rat(0), rat(0), rat(0), rat(-1)],
    ];
    let heights = vec![rat(500), rat(500), rat(500), rat(1), rat(1), rat(1), rat(1)];
    Polytope4D::from_rationals(normals, heights).expect("lagrangian triangle×square construction")
}

/// Extract adjacency pairs from adjacency matrix.
fn adjacency_pairs(p: &Polytope4D) -> BTreeSet<(usize, usize)> {
    let adj = p.adjacency();
    let f = p.facet_count();
    let mut pairs = BTreeSet::new();
    for i in 0..f {
        for k in (i + 1)..f {
            if adj[(i, k)] { pairs.insert((i, k)); }
        }
    }
    pairs
}

// ── ω₀ tests ────────────────────────────────────────────────────────────

/// Proposition: for the hypercube (Lagrangian product), all same-type adjacent
/// pairs have ω₀(yᵢ, yₖ) = 0.
#[test]
fn lagrangian_same_type_omega_zero() {
    let h = rational_lagrangian_square_square();
    let dual_verts = h.dual_vertices();
    let adj_pairs = adjacency_pairs(&h);
    let q_facets: BTreeSet<usize> = [0, 1, 2, 3].into_iter().collect();
    let p_facets: BTreeSet<usize> = [4, 5, 6, 7].into_iter().collect();
    for &(i, k) in &adj_pairs {
        let both_q = q_facets.contains(&i) && q_facets.contains(&k);
        let both_p = p_facets.contains(&i) && p_facets.contains(&k);
        if both_q || both_p {
            let omega = omega0_rational(&dual_verts[i], &dual_verts[k]);
            assert!(omega.is_zero(), "same-type pair ({i}, {k}) should have ω₀ = 0, got {omega}");
            assert_eq!(h.omega_signs()[(i, k)], 0i8, "same-type pair ({i}, {k}) sign should be Zero");
        }
    }
}

/// Proposition: for the hypercube (Lagrangian product), cross-type
/// adjacent pairs have ω₀ signs determined by the coordinate indices.
#[test]
fn lagrangian_cross_type_omega() {
    let h = rational_lagrangian_square_square();
    let dual_verts = h.dual_vertices();
    let adj_pairs = adjacency_pairs(&h);
    let check = |i: usize, k: usize, expected_sign: i8| {
        let (lo, hi) = (i.min(k), i.max(k));
        assert!(adj_pairs.contains(&(lo, hi)), "pair ({lo}, {hi}) should be adjacent");
        let omega = omega0_rational(&dual_verts[lo], &dual_verts[hi]);
        let actual_sign = match Sign::of(&omega) {
            Sign::Plus => 1i8, Sign::Minus => -1i8, Sign::Zero => 0i8,
        };
        assert_eq!(actual_sign, expected_sign, "pair ({lo}, {hi}): ω₀ = {omega}, expected sign {expected_sign}");
    };
    // (q₁ facets) × (p₁ facets)
    check(0, 4, 1); check(0, 5, -1); check(1, 4, -1); check(1, 5, 1);
    // (q₂ facets) × (p₂ facets)
    check(2, 6, 1); check(2, 7, -1); check(3, 6, -1); check(3, 7, 1);
    // Cross-plane: (q₁) × (p₂) → ω₀ = 0
    check(0, 6, 0); check(0, 7, 0); check(1, 6, 0); check(1, 7, 0);
    // Cross-plane: (q₂) × (p₁) → ω₀ = 0
    check(2, 4, 0); check(2, 5, 0); check(3, 4, 0); check(3, 5, 0);
}

/// Proposition: for a Lagrangian product (triangle ×_L square),
/// all same-type adjacent pairs have ω₀ = 0 exactly.
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
            assert!(omega.is_zero(), "same-type pair ({i}, {k}) should have ω₀ = 0, got {omega}");
        }
    }
}

/// Proposition: the simplex adjacency is exactly the complete graph K₅.
#[test]
fn simplex_adjacency_complete() {
    let s = rational_simplex();
    let adj_pairs = adjacency_pairs(&s);
    assert_eq!(adj_pairs.len(), 10);
    for i in 0..5 {
        for k in (i + 1)..5 {
            assert!(adj_pairs.contains(&(i, k)), "facets ({i}, {k}) should be adjacent in simplex");
        }
    }
}

// ── f64 agreement ───────────────────────────────────────────────────────

/// Proposition: f64 accessors produce a polytope with the same vertex count.
#[test]
fn f64_vertex_count() {
    let polytopes: Vec<(&str, Polytope4D)> = vec![
        ("simplex", rational_simplex()),
        ("hypercube", rational_hypercube()),
        ("lagrangian_tri_sq", rational_lagrangian_triangle_square()),
    ];
    for (name, rp) in &polytopes {
        assert_eq!(rp.vertices_f64().len(), rp.vertices().len(),
            "{name}: f64 vertex count should match rational vertex count");
    }
}

/// Proposition: f64 vertices are within O(ε_machine) of exact rational vertices.
#[test]
fn f64_vertex_accuracy() {
    let s = rational_simplex();
    let rational_verts: Vec<[f64; 4]> = s.vertices().iter()
        .map(|v| std::array::from_fn(|i| rational_to_f64(&v[i]))).collect();
    for f64_v in s.vertices_f64() {
        let f64_arr = [f64_v[0], f64_v[1], f64_v[2], f64_v[3]];
        let min_dist = rational_verts.iter()
            .map(|rv| (0..4).map(|i| (rv[i] - f64_arr[i]).powi(2)).sum::<f64>().sqrt())
            .fold(f64::INFINITY, f64::min);
        assert!(min_dist < 1e-10,
            "f64 vertex [{:.6}, {:.6}, {:.6}, {:.6}] is {min_dist:.2e} from nearest rational vertex",
            f64_arr[0], f64_arr[1], f64_arr[2], f64_arr[3]);
    }
}

/// Proposition: for polytopes with well-separated ω₀ values,
/// the f64 ω₀ signs agree with the exact rational signs.
#[test]
fn f64_sign_agreement() {
    let polytopes: Vec<(&str, Polytope4D)> = vec![
        ("simplex", rational_simplex()),
        ("hypercube", rational_hypercube()),
    ];
    for (name, rp) in &polytopes {
        let omega_signs = rp.omega_signs();
        let f = rp.facet_count();
        for i in 0..f {
            for k in (i + 1)..f {
                if !rp.adjacency()[(i, k)] { continue; }
                let exact_sign = omega_signs[(i, k)];
                if exact_sign == 0 { continue; }
                let f64_omega =
                    super::super::symplectic::omega0(&rp.normals_f64()[i], &rp.normals_f64()[k]);
                let f64_sign = if f64_omega > 1e-15 { 1i8 }
                    else if f64_omega < -1e-15 { -1i8 } else { 0i8 };
                assert_eq!(f64_sign, exact_sign,
                    "{name}: f64 ω₀ sign for ({i}, {k}) = {f64_omega:.2e}, expected {exact_sign}");
            }
        }
    }
}

/// Proposition: from_f64_rounded round-trips with bounded error.
#[test]
fn from_f64_roundtrip() {
    let kp = super::super::known_polytopes::hypercube();
    let f64_p = &kp.polytope;
    let normals = f64_p.normals_f64();
    let heights = f64_p.heights_f64();
    let rp = Polytope4D::from_f64_rounded(&normals, &heights, 1000)
        .expect("from_f64_rounded");
    assert_eq!(rp.vertices().len(), f64_p.vertices_f64().len());
    assert_eq!(rp.vertices_f64().len(), f64_p.vertices_f64().len());
}

// ── f64 ↔ rational lossless conversion ──────────────────────────────────

/// Proposition: f64_to_rational is the exact inverse of f64 → bits → rational.
#[test]
fn f64_to_rational_roundtrip() {
    let test_values: Vec<f64> = vec![
        0.0, 1.0, -1.0, 0.5, -0.5, 0.1, -0.1,
        1.0 / 3.0, std::f64::consts::PI, std::f64::consts::FRAC_1_SQRT_2,
        1e-15, 1e15, f64::MIN_POSITIVE, (2.0_f64).powi(52), 0.8090169943749473,
    ];
    for &x in &test_values {
        let r = f64_to_rational(x);
        let back = rational_to_f64(&r);
        assert_eq!(back, x, "round-trip failed for {x}: rational = {r}, back = {back}");
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

/// Proposition: Polytope4D::new from f64 normals/heights agrees with
/// rationals on vertex count.
#[test]
fn from_f64_lossless_roundtrip() {
    let kp = super::super::known_polytopes::hypercube();
    let f64_p = &kp.polytope;
    let halfspaces: Vec<nalgebra::Vector4<f64>> = f64_p.normals_f64().iter().zip(f64_p.heights_f64().iter()).map(|(n, &h)| n / h).collect();
    let rp = Polytope4D::new(halfspaces)
        .expect("new should succeed for hypercube");
    assert_eq!(rp.vertices().len(), f64_p.vertices().len());
    assert_eq!(rp.vertices_f64().len(), f64_p.vertices_f64().len());
}

/// Proposition: Polytope4D::new on the simplex has the expected combinatorics.
#[test]
fn from_f64_simplex() {
    let kp = super::super::known_polytopes::simplex();
    let f64_p = &kp.polytope;
    let halfspaces: Vec<nalgebra::Vector4<f64>> = f64_p.normals_f64().iter().zip(f64_p.heights_f64().iter()).map(|(n, &h)| n / h).collect();
    let rp = Polytope4D::new(halfspaces)
        .expect("new should succeed for simplex");
    assert_eq!(rp.facet_count(), 5);
    assert_eq!(rp.vertices().len(), 5);
}

// ── Perturbation ────────────────────────────────────────────────────────

/// Proposition: perturbing a simplex produces no ω₀ = 0 entries.
#[test]
fn perturbation_preserves_nonzero_signs() {
    use rand::SeedableRng;
    let p = rational_simplex();
    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(42);
    let perturbed = p.perturbed(&mut rng, 64).expect("perturbation should succeed");
    assert_eq!(perturbed.vertices().len(), p.vertices().len());
    assert_eq!(perturbed.facet_count(), p.facet_count());
    let f = perturbed.facet_count();
    let has_zero = (0..f).any(|i| {
        ((i + 1)..f).any(|k| perturbed.adjacency()[(i, k)] && perturbed.omega_signs()[(i, k)] == 0)
    });
    assert!(!has_zero, "perturbed polytope should have no ω₀ = 0");
}

/// Proposition: perturbing a Lagrangian product breaks all ω₀ = 0 pairs.
#[test]
fn perturbation_breaks_omega_zeros() {
    use rand::SeedableRng;
    let p = rational_lagrangian_triangle_square();
    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(123);
    let f = p.facet_count();
    let has_zeros = (0..f).any(|i| {
        ((i + 1)..f).any(|k| p.adjacency()[(i, k)] && p.omega_signs()[(i, k)] == 0)
    });
    assert!(has_zeros, "Lagrangian product should have ω₀ = 0 pairs before perturbation");
    let perturbed = p.perturbed(&mut rng, 64).expect("perturbation should succeed");
    let fp = perturbed.facet_count();
    let has_zeros_after = (0..fp).any(|i| {
        ((i + 1)..fp).any(|k| perturbed.adjacency()[(i, k)] && perturbed.omega_signs()[(i, k)] == 0)
    });
    assert!(!has_zeros_after, "perturbed polytope should have no ω₀ = 0");
}

/// Proposition: perturbation at 2^{-64} barely changes the f64 representation.
#[test]
fn perturbation_preserves_f64() {
    use rand::SeedableRng;
    let p = rational_simplex();
    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(99);
    let perturbed = p.perturbed(&mut rng, 64).expect("perturbation should succeed");
    let tol = 1e-18;
    for (n_before, n_after) in p.normals_f64().iter().zip(perturbed.normals_f64().iter()) {
        for c in 0..4 {
            assert!((n_before[c] - n_after[c]).abs() < tol,
                "f64 normal component changed by {} (tol={tol})", (n_before[c] - n_after[c]).abs());
        }
    }
    for (h_before, h_after) in p.heights_f64().iter().zip(perturbed.heights_f64().iter()) {
        assert!((h_before - h_after).abs() < tol,
            "f64 height changed by {} after perturbation", (h_before - h_after).abs());
    }
}

// ── ω₀ agreement ───────────────────────────────────────────────────────

/// Proposition: the exact ω₀ formula agrees with the f64 formula on integer inputs.
#[test]
fn omega0_rational_agrees_with_f64() {
    use nalgebra::Vector4;
    let test_cases: Vec<([i64; 4], [i64; 4])> = vec![
        ([1, 0, 0, 0], [0, 0, 1, 0]),
        ([1, 0, 0, 0], [0, 0, 0, 1]),
        ([0, 1, 0, 0], [0, 0, 0, 1]),
        ([1, 2, 3, 4], [5, 6, 7, 8]),
        ([3, -1, 4, -1], [5, -9, 2, -6]),
    ];
    for (u_arr, v_arr) in &test_cases {
        let u_rat: [BigRational; 4] = std::array::from_fn(|i| rat(u_arr[i]));
        let v_rat: [BigRational; 4] = std::array::from_fn(|i| rat(v_arr[i]));
        let u_f64 = Vector4::new(u_arr[0] as f64, u_arr[1] as f64, u_arr[2] as f64, u_arr[3] as f64);
        let v_f64 = Vector4::new(v_arr[0] as f64, v_arr[1] as f64, v_arr[2] as f64, v_arr[3] as f64);
        let rational_result = omega0_rational(&u_rat, &v_rat);
        let f64_result = super::super::symplectic::omega0(&u_f64, &v_f64);
        assert_eq!(rational_to_f64(&rational_result), f64_result,
            "ω₀({u_arr:?}, {v_arr:?}): rational={rational_result}, f64={f64_result}");
    }
}

// ── Pipeline consistency ────────────────────────────────────────────────

/// Proposition: adjacency matrix from exact data agrees with f64 for simplex.
#[test]
fn adjacency_agreement_simplex() {
    use crate::algorithms::hk2017::build_adjacency_matrix;
    let kp = super::super::known_polytopes::simplex();
    let f64_adj = build_adjacency_matrix(&kp.polytope);
    let halfspaces: Vec<nalgebra::Vector4<f64>> = kp.polytope.normals_f64().iter().zip(kp.polytope.heights_f64().iter()).map(|(n, &h)| n / h).collect();
    let rational_p = Polytope4D::new(halfspaces).expect("new should succeed for simplex");
    assert_eq!(f64_adj, build_adjacency_matrix(&rational_p), "adjacency disagree for simplex");
}

/// Proposition: adjacency matrix from exact data agrees with f64 for hypercube.
#[test]
fn adjacency_agreement_hypercube() {
    use crate::algorithms::hk2017::build_adjacency_matrix;
    let kp = super::super::known_polytopes::hypercube();
    let f64_adj = build_adjacency_matrix(&kp.polytope);
    let halfspaces: Vec<nalgebra::Vector4<f64>> = kp.polytope.normals_f64().iter().zip(kp.polytope.heights_f64().iter()).map(|(n, &h)| n / h).collect();
    let rational_p = Polytope4D::new(halfspaces).expect("new should succeed for hypercube");
    assert_eq!(f64_adj, build_adjacency_matrix(&rational_p), "adjacency disagree for hypercube");
}

/// Proposition: directed adjacency from exact data agrees with f64 for simplex.
#[test]
fn directed_adjacency_agreement_simplex() {
    use crate::algorithms::hk2017::build_directed_adjacency_matrix;
    let kp = super::super::known_polytopes::simplex();
    let f64_dadj = build_directed_adjacency_matrix(&kp.polytope);
    let halfspaces: Vec<nalgebra::Vector4<f64>> = kp.polytope.normals_f64().iter().zip(kp.polytope.heights_f64().iter()).map(|(n, &h)| n / h).collect();
    let rational_p = Polytope4D::new(halfspaces).expect("new should succeed for simplex");
    assert_eq!(f64_dadj, build_directed_adjacency_matrix(&rational_p),
        "directed adjacency disagree for simplex");
}

/// Proposition: directed adjacency from exact data agrees with f64 for hypercube.
#[test]
fn directed_adjacency_agreement_hypercube() {
    use crate::algorithms::hk2017::build_directed_adjacency_matrix;
    let kp = super::super::known_polytopes::hypercube();
    let f64_dadj = build_directed_adjacency_matrix(&kp.polytope);
    let halfspaces: Vec<nalgebra::Vector4<f64>> = kp.polytope.normals_f64().iter().zip(kp.polytope.heights_f64().iter()).map(|(n, &h)| n / h).collect();
    let rational_p = Polytope4D::new(halfspaces).expect("new should succeed for hypercube");
    assert_eq!(f64_dadj, build_directed_adjacency_matrix(&rational_p),
        "directed adjacency disagree for hypercube");
}

/// Proposition: EHZ capacity is unchanged through rational pipeline for simplex.
#[test]
fn capacity_agreement_simplex() {
    use crate::algorithms::hk2017::ehz_capacity;
    let kp = super::super::known_polytopes::simplex();
    let f64_result = ehz_capacity(&kp.polytope).expect("simplex should have capacity");
    let halfspaces: Vec<nalgebra::Vector4<f64>> = kp.polytope.normals_f64().iter().zip(kp.polytope.heights_f64().iter()).map(|(n, &h)| n / h).collect();
    let rational_p = Polytope4D::new(halfspaces).expect("new should succeed for simplex");
    let exact_result = ehz_capacity(&rational_p).expect("simplex should have capacity");
    assert!((f64_result.capacity - exact_result.capacity).abs() < 1e-10,
        "capacity disagrees: f64={}, exact={}", f64_result.capacity, exact_result.capacity);
}

/// Proposition: EHZ capacity is unchanged through rational pipeline for hypercube.
#[test]
fn capacity_agreement_hypercube() {
    use crate::algorithms::hk2017::ehz_capacity;
    let kp = super::super::known_polytopes::hypercube();
    let f64_result = ehz_capacity(&kp.polytope).expect("hypercube should have capacity");
    let halfspaces: Vec<nalgebra::Vector4<f64>> = kp.polytope.normals_f64().iter().zip(kp.polytope.heights_f64().iter()).map(|(n, &h)| n / h).collect();
    let rational_p = Polytope4D::new(halfspaces).expect("new should succeed for hypercube");
    let exact_result = ehz_capacity(&rational_p).expect("hypercube should have capacity");
    assert!((f64_result.capacity - exact_result.capacity).abs() < 1e-10,
        "capacity disagrees: f64={}, exact={}", f64_result.capacity, exact_result.capacity);
}

//! Named polytope constructors with known EHZ capacity values from the literature.
//!
//! Single source of truth for polytope definitions and their known capacity values.
//! Used by: test utilities, dataset generation, and capacity validation
//! (`hk2017` smoke tests and verification experiments).
//!
//! Each polytope is constructed once (on first access) and cached via `LazyLock`.
//! Constructors return `&'static KnownPolytope` — zero-cost after first call.
//!
//! Coordinates: (q_1, q_2, p_1, p_2). See `symplectic_form` module for J_0 and omega_0.
//!
//! Mathematical correspondence: [def:ehz-capacity], [thm:hko-counterexample]

use crate::geom::polytope::Polytope4D;
use crate::geom::rational_arithmetic::{frac, rat};
use nalgebra::{DMatrix, Vector4};
use num_rational::BigRational;
use std::f64::consts::PI;
use std::sync::LazyLock;

/// A polytope with a known capacity value and source reference.
#[derive(Clone, Debug)]
pub struct KnownPolytope {
    /// The polytope instance.
    pub polytope: Polytope4D,
    /// Dual vertices a_i, vertices of the polar body K°.
    pub dual_vertices: Vec<[BigRational; 4]>,
    /// Exact rational vertices of K.
    pub vertices: Vec<[BigRational; 4]>,
    /// Dual vertices a_i as f64 vectors.
    pub dual_vertices_f64: Vec<Vector4<f64>>,
    /// Vertices of K rounded to f64.
    pub vertices_f64: Vec<Vector4<f64>>,
    /// Vertex-facet incidence matrix.
    pub incidence: DMatrix<bool>,
    /// Facet-pair nonempty-intersection matrix.
    pub facet_intersection_is_nonempty: DMatrix<bool>,
    /// Symplectic sign matrix omega in {-1,0,+1}^{F x F}.
    pub omega_signs: DMatrix<i8>,
    /// Known EHZ capacity value.
    pub capacity: f64,
    /// Short human-readable name.
    pub name: &'static str,
    /// Literature reference for the capacity value.
    pub source: &'static str,
}

fn known_polytope(
    polytope: Polytope4D,
    capacity: f64,
    name: &'static str,
    source: &'static str,
) -> KnownPolytope {
    KnownPolytope {
        dual_vertices: polytope.dual_vertices().to_vec(),
        vertices: polytope.vertices().to_vec(),
        dual_vertices_f64: polytope.dual_vertices_f64().to_vec(),
        vertices_f64: polytope.vertices_f64().to_vec(),
        incidence: polytope.incidence().clone(),
        facet_intersection_is_nonempty: polytope.facet_intersection_is_nonempty().clone(),
        omega_signs: polytope.omega_signs().clone(),
        polytope,
        capacity,
        name,
        source,
    }
}

/// All known polytopes with verified or computed capacity values.
///
/// Returns references to the cached singletons. Each polytope is constructed
/// on first access (via its individual constructor) and never again.
pub fn all_known() -> Vec<&'static KnownPolytope> {
    vec![
        simplex(),
        hypercube(),
        crosspolytope(),
        hko_pentagon(),
        lagrangian_triangle_product(),
        symplectic_triangle_product(),
        lagrangian_triangle_square(),
        symplectic_triangle_square(),
    ]
}

/// Literature capacity values: (name, capacity) pairs.
///
/// Excludes polytopes without a verified literature source (e.g., crosspolytope
/// has only a computed value with no independent cross-check).
pub fn literature_values() -> Vec<(&'static str, f64)> {
    all_known()
        .into_iter()
        .filter(|kp| kp.source != "computed (no literature value)")
        .map(|kp| (kp.name, kp.capacity))
        .collect()
}

// ── Cached constructors ──────────────────────────────────────────────────

/// 4-simplex (5 facets), translated so the origin is at the centroid.
///
/// Standard simplex conv{0, e_1, e_2, e_3, e_4} with centroid at (0.2, 0.2, 0.2, 0.2).
/// After translation, all heights are positive.
///
/// Known capacity: 0.25 = 1/(2n) where n = 2 is the complex dimension (R^{2n} = R^4).
/// Source: Y. Nir thesis 2013; Siegel's Symplectic Capacities Project.
///
/// Mathematical correspondence: [def:ehz-capacity]
pub fn simplex() -> &'static KnownPolytope {
    static INSTANCE: LazyLock<KnownPolytope> = LazyLock::new(|| {
        // Standard simplex conv{0, e1..e4}, translated so origin = centroid (1/5, ...).
        // Dual vertices a_i = n_i / h_i (all integers):
        //   facets 1-4: a_i = -5 e_i
        //   facet 5:    a_5 = (5, 5, 5, 5)
        let z = rat(0);
        let dual_vertices = vec![
            [rat(-5), z.clone(), z.clone(), z.clone()],
            [z.clone(), rat(-5), z.clone(), z.clone()],
            [z.clone(), z.clone(), rat(-5), z.clone()],
            [z.clone(), z.clone(), z.clone(), rat(-5)],
            [rat(5), rat(5), rat(5), rat(5)],
        ];

        known_polytope(
            Polytope4D::new(dual_vertices).expect("simplex construction"),
            0.25,
            "simplex",
            "Y. Nir thesis 2013",
        )
    });
    &INSTANCE
}

/// Hypercube [-1,1]^4 (8 facets).
///
/// Known capacity: 4.0.
/// Source: HK2019 Ex 4.6, Rudolf 2022.
///
/// Mathematical correspondence: [def:ehz-capacity]
pub fn hypercube() -> &'static KnownPolytope {
    static INSTANCE: LazyLock<KnownPolytope> = LazyLock::new(|| {
        // [-1,1]^4: dual vertices ±e_i (all integers).
        let z = rat(0);
        let dual_vertices = vec![
            [rat(1), z.clone(), z.clone(), z.clone()],
            [rat(-1), z.clone(), z.clone(), z.clone()],
            [z.clone(), rat(1), z.clone(), z.clone()],
            [z.clone(), rat(-1), z.clone(), z.clone()],
            [z.clone(), z.clone(), rat(1), z.clone()],
            [z.clone(), z.clone(), rat(-1), z.clone()],
            [z.clone(), z.clone(), z.clone(), rat(1)],
            [z.clone(), z.clone(), z.clone(), rat(-1)],
        ];

        known_polytope(
            Polytope4D::new(dual_vertices).expect("hypercube construction"),
            4.0,
            "hypercube",
            "HK2019 Ex 4.6",
        )
    });
    &INSTANCE
}

/// 4D crosspolytope (hyperoctahedron, dual of tesseract). 16 facets.
///
/// Normals: all (+/-1, +/-1, +/-1, +/-1)/2, heights 1.0.
/// Capacity: 4.0 (computed by ehz_capacity; no literature cross-check available).
///
/// Mathematical correspondence: [def:ehz-capacity]
pub fn crosspolytope() -> &'static KnownPolytope {
    static INSTANCE: LazyLock<KnownPolytope> = LazyLock::new(|| {
        // 16 dual vertices (±1/2, ±1/2, ±1/2, ±1/2) — denominator 2.
        let mut dual_vertices = Vec::with_capacity(16);
        for &s0 in &[-1i64, 1] {
            for &s1 in &[-1i64, 1] {
                for &s2 in &[-1i64, 1] {
                    for &s3 in &[-1i64, 1] {
                        dual_vertices.push([frac(s0, 2), frac(s1, 2), frac(s2, 2), frac(s3, 2)]);
                    }
                }
            }
        }

        known_polytope(
            Polytope4D::new(dual_vertices).expect("crosspolytope construction"),
            4.0,
            "crosspolytope",
            "computed (no literature value)",
        )
    });
    &INSTANCE
}

/// HKO 2024 pentagon counterexample (10 facets).
///
/// Pentagon x_L (same pentagon rotated 90 degrees), a Lagrangian product.
/// Known capacity: 2*cos(pi/10)*(1 + cos(pi/5)) ~ 3.441.
/// This is the counterexample to Viterbo's conjecture (systolic ratio > 1).
///
/// Source: Haim-Kislev & Ostrover 2024, "A counterexample to the Viterbo conjecture".
///
/// Mathematical correspondence: [thm:hko-counterexample]
pub fn hko_pentagon() -> &'static KnownPolytope {
    static INSTANCE: LazyLock<KnownPolytope> = LazyLock::new(|| {
        let normals = vec![
            // Q-space pentagon (5 facets)
            Vector4::new(0.8090169943749473, 0.5877852522924731, 0.0, 0.0),
            Vector4::new(-0.3090169943749473, 0.9510565162951536, 0.0, 0.0),
            Vector4::new(-1.0, 0.0, 0.0, 0.0),
            Vector4::new(-0.30901699437494756, -0.9510565162951536, 0.0, 0.0),
            Vector4::new(0.8090169943749473, -0.5877852522924731, 0.0, 0.0),
            // P-space pentagon rotated 90 degrees (5 facets)
            Vector4::new(0.0, 0.0, 0.5877852522924732, -0.8090169943749475),
            Vector4::new(0.0, 0.0, 0.9510565162951536, 0.3090169943749474),
            Vector4::new(0.0, 0.0, 0.0, 1.0),
            Vector4::new(0.0, 0.0, -0.9510565162951536, 0.3090169943749476),
            Vector4::new(0.0, 0.0, -0.5877852522924731, -0.8090169943749475),
        ];
        let heights = vec![
            0.8090169943749473,
            0.8090169943749475,
            0.8090169943749475,
            0.8090169943749475,
            0.8090169943749472,
            0.8090169943749475,
            0.8090169943749475,
            0.8090169943749475,
            0.8090169943749475,
            0.8090169943749473,
        ];
        let halfspaces: Vec<Vector4<f64>> = normals
            .iter()
            .zip(heights.iter())
            .map(|(n, &h)| n / h)
            .collect();
        let capacity = 2.0 * (PI / 10.0).cos() * (1.0 + (PI / 5.0).cos());

        known_polytope(
            Polytope4D::from_f64(halfspaces).expect("HKO pentagon construction"),
            capacity,
            "hko_pentagon",
            "HK-O 2024 Prop 1.4",
        )
    });
    &INSTANCE
}

/// Equilateral triangle x_L triangle, Lagrangian product (6 facets).
///
/// Regular triangle with circumradius 1 in both q-space and p-space.
/// Known capacity: 1.5.
///
/// Mathematical correspondence: [def:lagrangian-product]
pub fn lagrangian_triangle_product() -> &'static KnownPolytope {
    static INSTANCE: LazyLock<KnownPolytope> = LazyLock::new(|| {
        let triangle_angles: Vec<f64> = (0..3)
            .map(|k| PI / 2.0 + 2.0 * PI * (k as f64) / 3.0)
            .collect();

        // Q-space triangle + P-space triangle (Lagrangian product)
        // Heights are 0.5, so halfspaces a_i = n_i / 0.5 = 2*n_i
        let halfspaces: Vec<Vector4<f64>> = triangle_angles
            .iter()
            .map(|a| Vector4::new(a.cos(), a.sin(), 0.0, 0.0) / 0.5)
            .chain(
                triangle_angles
                    .iter()
                    .map(|a| Vector4::new(0.0, 0.0, a.cos(), a.sin()) / 0.5),
            )
            .collect();

        known_polytope(
            Polytope4D::from_f64(halfspaces).expect("lagrangian triangle product construction"),
            1.5,
            "lagrangian_triangle_product",
            "LP verification (HK2017 algorithm + billiard)",
        )
    });
    &INSTANCE
}

/// Equilateral triangle x_S triangle, symplectic product (6 facets).
///
/// Two equilateral triangles (circumradius 1, inradius 0.5) in symplectic planes:
/// (q_1, p_1) = components [0,2] and (q_2, p_2) = components [1,3].
///
/// Known capacity: 3*sqrt(3)/4 ~ 1.299 (symplectic product formula:
/// c(A x_S B) = min(c(A), c(B)); both triangles have equal area 3*sqrt(3)/4).
///
/// Source: [prop:capacity-symplectic-product].
///
/// Mathematical correspondence: [def:symplectic-product]
pub fn symplectic_triangle_product() -> &'static KnownPolytope {
    static INSTANCE: LazyLock<KnownPolytope> = LazyLock::new(|| {
        let triangle_angles: Vec<f64> = (0..3)
            .map(|k| PI / 2.0 + 2.0 * PI * (k as f64) / 3.0)
            .collect();

        // First triangle in (q_1, p_1) plane, second in (q_2, p_2) plane
        let halfspaces: Vec<Vector4<f64>> = triangle_angles
            .iter()
            .map(|a| Vector4::new(a.cos(), 0.0, a.sin(), 0.0) / 0.5)
            .chain(
                triangle_angles
                    .iter()
                    .map(|a| Vector4::new(0.0, a.cos(), 0.0, a.sin()) / 0.5),
            )
            .collect();

        let area_tri = 3.0 * 3.0_f64.sqrt() / 4.0;

        known_polytope(
            Polytope4D::from_f64(halfspaces).expect("symplectic triangle product construction"),
            area_tri,
            "symplectic_triangle_product",
            "Symplectic product formula ([prop:capacity-symplectic-product])",
        )
    });
    &INSTANCE
}

/// Triangle x_L square (Lagrangian product, 7 facets).
///
/// Equilateral triangle (circumradius 1, inradius 0.5) in q-space,
/// unit square (side 1) in p-space. Both are Lagrangian subspaces.
///
/// Known capacity: 1.5 (verified via billiard + HK2017).
///
/// Mathematical correspondence: [def:lagrangian-product]
pub fn lagrangian_triangle_square() -> &'static KnownPolytope {
    static INSTANCE: LazyLock<KnownPolytope> = LazyLock::new(|| {
        let triangle_halfspaces = (0..3).map(|k| {
            let angle = PI / 2.0 + 2.0 * PI * (k as f64) / 3.0;
            Vector4::new(angle.cos(), angle.sin(), 0.0, 0.0) / 0.5
        });

        let square_halfspaces = [
            Vector4::new(0.0, 0.0, 1.0, 0.0) / 0.5,
            Vector4::new(0.0, 0.0, -1.0, 0.0) / 0.5,
            Vector4::new(0.0, 0.0, 0.0, 1.0) / 0.5,
            Vector4::new(0.0, 0.0, 0.0, -1.0) / 0.5,
        ];

        let halfspaces: Vec<Vector4<f64>> = triangle_halfspaces.chain(square_halfspaces).collect();

        known_polytope(
            Polytope4D::from_f64(halfspaces).expect("Lagrangian triangle x square construction"),
            1.5,
            "lagrangian_tri_sq",
            "HK2017 algorithm + billiard verification",
        )
    });
    &INSTANCE
}

/// Triangle x_S square (true symplectic product, 7 facets).
///
/// Equilateral triangle (circumradius 1, area = 3*sqrt(3)/4) in the (q_1, p_1) plane,
/// unit square (side 1, area = 1) in the (q_2, p_2) plane. Both planes are
/// symplectic and symplectically orthogonal.
///
/// Known capacity: min(3*sqrt(3)/4, 1) = 1.0 (symplectic product formula:
/// c(A x_S B) = min(c(A), c(B))).
///
/// Source: [prop:capacity-symplectic-product].
///
/// Mathematical correspondence: [def:symplectic-product]
pub fn symplectic_triangle_square() -> &'static KnownPolytope {
    static INSTANCE: LazyLock<KnownPolytope> = LazyLock::new(|| {
        let triangle_halfspaces = (0..3).map(|k| {
            let angle = PI / 2.0 + 2.0 * PI * (k as f64) / 3.0;
            Vector4::new(angle.cos(), 0.0, angle.sin(), 0.0) / 0.5
        });

        let square_halfspaces = [
            Vector4::new(0.0, 1.0, 0.0, 0.0) / 0.5,
            Vector4::new(0.0, -1.0, 0.0, 0.0) / 0.5,
            Vector4::new(0.0, 0.0, 0.0, 1.0) / 0.5,
            Vector4::new(0.0, 0.0, 0.0, -1.0) / 0.5,
        ];

        let halfspaces: Vec<Vector4<f64>> = triangle_halfspaces.chain(square_halfspaces).collect();

        let area_tri = 3.0 * 3.0_f64.sqrt() / 4.0;
        let area_sq = 1.0;

        known_polytope(
            Polytope4D::from_f64(halfspaces).expect("symplectic triangle x square construction"),
            area_tri.min(area_sq),
            "symplectic_tri_sq",
            "Symplectic product formula ([prop:capacity-symplectic-product])",
        )
    });
    &INSTANCE
}

#[cfg(test)]
mod tests {
    use super::*;

    /// All known polytopes pass construction and have at least 5 facets.
    #[test]
    fn all_known_polytopes_valid() {
        for kp in all_known() {
            assert!(
                kp.polytope.facet_count() >= 5,
                "{}: too few facets ({})",
                kp.name,
                kp.polytope.facet_count()
            );
        }
    }

    #[test]
    fn simplex_has_5_facets() {
        assert_eq!(simplex().polytope.facet_count(), 5);
    }

    #[test]
    fn hypercube_has_8_facets() {
        assert_eq!(hypercube().polytope.facet_count(), 8);
    }

    #[test]
    fn crosspolytope_has_16_facets() {
        assert_eq!(crosspolytope().polytope.facet_count(), 16);
    }

    #[test]
    fn hko_pentagon_has_10_facets() {
        assert_eq!(hko_pentagon().polytope.facet_count(), 10);
    }

    #[test]
    fn lagrangian_triangle_product_has_6_facets() {
        assert_eq!(lagrangian_triangle_product().polytope.facet_count(), 6);
    }

    #[test]
    fn symplectic_triangle_product_has_6_facets() {
        assert_eq!(symplectic_triangle_product().polytope.facet_count(), 6);
    }

    #[test]
    fn lagrangian_tri_sq_has_7_facets() {
        assert_eq!(lagrangian_triangle_square().polytope.facet_count(), 7);
    }

    #[test]
    fn symplectic_tri_sq_has_7_facets() {
        assert_eq!(symplectic_triangle_square().polytope.facet_count(), 7);
    }

    #[test]
    fn all_known_capacities_positive() {
        for kp in all_known() {
            assert!(kp.capacity > 0.0, "{}: capacity should be > 0", kp.name);
        }
    }

    /// literature_values() excludes polytopes without a literature cross-check.
    #[test]
    fn literature_values_excludes_computed_only() {
        let lit = literature_values();
        assert!(
            !lit.iter().any(|(name, _)| *name == "crosspolytope"),
            "crosspolytope should be excluded from literature values"
        );
    }
}

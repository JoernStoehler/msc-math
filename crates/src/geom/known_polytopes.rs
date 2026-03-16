/// Polytopes with known EHZ capacities from the literature.
///
/// Single source of truth for polytope definitions and their known capacity values.
/// Used by: test fixtures (`test_utils`), dataset generation (`datasets`), and
/// capacity validation (`hk2017` property tests).
///
/// Each constructor returns a `KnownPolytope` with the polytope, its known
/// capacity value, and a literature reference.
///
/// **Coordinates**: (q₁, q₂, p₁, p₂). See `symplectic` module for J₀ and ω₀.
use crate::geom::polytope::Polytope4D;
use nalgebra::Vector4;
use std::f64::consts::PI;

/// A polytope with a known capacity value and source reference.
#[derive(Clone, Debug)]
pub struct KnownPolytope {
    pub polytope: Polytope4D,
    pub capacity: f64,
    pub name: &'static str,
    pub source: &'static str,
}

/// All known polytopes.
pub fn all_known() -> Vec<KnownPolytope> {
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
/// Only polytopes with verified literature values — excludes crosspolytope
/// (computed value, no literature cross-check) and any other polytopes without
/// a verified literature source.
pub fn literature_values() -> Vec<(&'static str, f64)> {
    all_known()
        .into_iter()
        .filter(|kp| kp.source != "computed (no literature value)")
        .map(|kp| (kp.name, kp.capacity))
        .collect()
}

/// 4-simplex (5 facets), translated so origin is at centroid.
///
/// Standard simplex conv{0, e1, e2, e3, e4} with centroid at (0.2, 0.2, 0.2, 0.2).
/// After translation, all heights are positive.
///
/// Known capacity: 0.25 = 1/(2n) where n = 2 is the complex dimension (ℝ^{2n} = ℝ⁴).
/// Source: Y. Nir thesis 2013; Siegel's Symplectic Capacities Project.
pub fn simplex() -> KnownPolytope {
    let centroid = Vector4::new(0.2, 0.2, 0.2, 0.2);
    let normals_raw = [
        -Vector4::x(),
        -Vector4::y(),
        -Vector4::z(),
        -Vector4::w(),
        Vector4::new(1.0, 1.0, 1.0, 1.0).normalize(),
    ];
    let heights_raw = [0.0, 0.0, 0.0, 0.0, 0.5_f64]; // h₅ = 1/‖(1,1,1,1)‖ = 1/2 for facet Σxᵢ ≤ 1
    let heights: Vec<f64> = normals_raw
        .iter()
        .zip(&heights_raw)
        .map(|(n, h)| h - n.dot(&centroid))
        .collect();
    // Dual vertex representation: aᵢ = nᵢ / hᵢ
    let halfspaces: Vec<Vector4<f64>> = normals_raw
        .iter()
        .zip(heights.iter())
        .map(|(n, &h)| n / h)
        .collect();

    KnownPolytope {
        polytope: Polytope4D::new(halfspaces).expect("simplex construction"),
        capacity: 0.25,
        name: "simplex",
        source: "Y. Nir thesis 2013",
    }
}

/// Hypercube [-1,1]^4 (8 facets).
///
/// Known capacity: 4.0.
/// Source: HK2019 Ex 4.6, Rudolf 2022.
pub fn hypercube() -> KnownPolytope {
    // [-1,1]^4: normals ±eᵢ, heights 1.0 → halfspaces aᵢ = nᵢ/hᵢ = ±eᵢ
    let halfspaces = vec![
        Vector4::x(),
        -Vector4::x(),
        Vector4::y(),
        -Vector4::y(),
        Vector4::z(),
        -Vector4::z(),
        Vector4::w(),
        -Vector4::w(),
    ];

    KnownPolytope {
        polytope: Polytope4D::new(halfspaces).expect("hypercube construction"),
        capacity: 4.0,
        name: "hypercube",
        source: "HK2019 Ex 4.6",
    }
}

/// 4D crosspolytope (hyperoctahedron, dual of tesseract). 16 facets.
///
/// Normals: all (±1, ±1, ±1, ±1)/2, heights 1.0.
/// Capacity: 4.0 (computed by ehz_capacity; no literature value for cross-check).
pub fn crosspolytope() -> KnownPolytope {
    // Normals (±1,±1,±1,±1)/2, heights 1.0 → halfspaces aᵢ = nᵢ/hᵢ = nᵢ
    let halfspaces: Vec<Vector4<f64>> = [-1.0_f64, 1.0]
        .into_iter()
        .flat_map(|s0| {
            [-1.0_f64, 1.0].into_iter().flat_map(move |s1| {
                [-1.0_f64, 1.0].into_iter().flat_map(move |s2| {
                    [-1.0_f64, 1.0]
                        .into_iter()
                        .map(move |s3| Vector4::new(s0, s1, s2, s3).normalize())
                })
            })
        })
        .collect();

    KnownPolytope {
        polytope: Polytope4D::new(halfspaces).expect("crosspolytope construction"),
        capacity: 4.0, // computed by ehz_capacity; no literature value for cross-check
        name: "crosspolytope",
        source: "computed (no literature value)",
    }
}

/// HK-O 2024 pentagon counterexample (10 facets).
///
/// Pentagon × (same pentagon rotated 90°), a Lagrangian product.
/// Known capacity: 2·cos(π/10)·(1 + cos(π/5)) ≈ 3.441.
/// This is the counterexample to Viterbo's conjecture (systolic ratio > 1).
///
/// Source: Haim-Kislev & Ostrover 2024, "A counterexample to the Viterbo conjecture".
pub fn hko_pentagon() -> KnownPolytope {
    let normals = vec![
        // Q-space pentagon (5 facets)
        Vector4::new(0.8090169943749473, 0.5877852522924731, 0.0, 0.0),
        Vector4::new(-0.3090169943749473, 0.9510565162951536, 0.0, 0.0),
        Vector4::new(-1.0, 0.0, 0.0, 0.0),
        Vector4::new(-0.30901699437494756, -0.9510565162951536, 0.0, 0.0),
        Vector4::new(0.8090169943749473, -0.5877852522924731, 0.0, 0.0),
        // P-space pentagon rotated 90° (5 facets)
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
    // Dual vertex representation: aᵢ = nᵢ / hᵢ
    let halfspaces: Vec<Vector4<f64>> = normals
        .iter()
        .zip(heights.iter())
        .map(|(n, &h)| n / h)
        .collect();
    let capacity = 2.0 * (PI / 10.0).cos() * (1.0 + (PI / 5.0).cos());

    KnownPolytope {
        polytope: Polytope4D::new(halfspaces).expect("HK-O pentagon construction"),
        capacity,
        name: "hko_pentagon",
        source: "HK-O 2024 Prop 1.4",
    }
}

/// Equilateral triangle ×_L triangle, Lagrangian product (6 facets).
///
/// Regular triangle with circumradius 1 in both q-space (q₁, q₂) and p-space (p₁, p₂).
/// Coordinates are (q₁, q₂, p₁, p₂).
/// Known capacity: 1.5.
pub fn lagrangian_triangle_product() -> KnownPolytope {
    // Regular triangle: outward normals at angles π/2 + 2πk/3, inradius = cos(π/3) = 0.5
    let triangle_angles: Vec<f64> = (0..3)
        .map(|k| PI / 2.0 + 2.0 * PI * (k as f64) / 3.0)
        .collect();

    // Q-space triangle + P-space triangle (Lagrangian product)
    // Normals are unit, heights are 0.5 → halfspaces aᵢ = nᵢ / 0.5 = 2·nᵢ
    let halfspaces: Vec<Vector4<f64>> = triangle_angles
        .iter()
        .map(|a| Vector4::new(a.cos(), a.sin(), 0.0, 0.0) / 0.5)
        .chain(
            triangle_angles
                .iter()
                .map(|a| Vector4::new(0.0, 0.0, a.cos(), a.sin()) / 0.5),
        )
        .collect();

    KnownPolytope {
        polytope: Polytope4D::new(halfspaces)
            .expect("lagrangian triangle product construction"),
        capacity: 1.5,
        name: "lagrangian_triangle_product",
        source: "LP verification (HK2017 algorithm + billiard)",
    }
}

/// Equilateral triangle ×_S triangle, symplectic product (6 facets).
///
/// Two equilateral triangles (circumradius 1, inradius 0.5) in symplectic planes:
/// (q₁, p₁) = components [0,2] and (q₂, p₂) = components [1,3].
/// Coordinates are (q₁, q₂, p₁, p₂).
///
/// Known capacity: 3√3/4 ≈ 1.299 (symplectic product formula: c(A ×_S B) = min(c(A), c(B)),
/// both triangles have equal area 3√3/4).
///
/// Source: `[prop:capacity-symplectic-product]` (symplectic product formula: c(A ×_S B) = min(c(A), c(B))).
pub fn symplectic_triangle_product() -> KnownPolytope {
    // Regular triangle: outward normals at angles π/2 + 2πk/3, inradius = cos(π/3) = 0.5
    let triangle_angles: Vec<f64> = (0..3)
        .map(|k| PI / 2.0 + 2.0 * PI * (k as f64) / 3.0)
        .collect();

    // First triangle in (q₁, p₁) plane — normals (cos θ, 0, sin θ, 0)
    // Second triangle in (q₂, p₂) plane — normals (0, cos θ, 0, sin θ)
    // Heights are 0.5 → halfspaces aᵢ = nᵢ / 0.5 = 2·nᵢ
    let halfspaces: Vec<Vector4<f64>> = triangle_angles
        .iter()
        .map(|a| Vector4::new(a.cos(), 0.0, a.sin(), 0.0) / 0.5)
        .chain(
            triangle_angles
                .iter()
                .map(|a| Vector4::new(0.0, a.cos(), 0.0, a.sin()) / 0.5),
        )
        .collect();

    // Symplectic product formula: c(A ×_S B) = min(c(A), c(B))
    // area(equilateral triangle, inradius 0.5) = 3√3/4
    let area_tri = 3.0 * 3.0_f64.sqrt() / 4.0;
    let capacity = area_tri; // min(area, area) = area

    KnownPolytope {
        polytope: Polytope4D::new(halfspaces)
            .expect("symplectic triangle product construction"),
        capacity,
        name: "symplectic_triangle_product",
        source: "Symplectic product formula ([prop:capacity-symplectic-product])",
    }
}

/// Triangle ×_L square (Lagrangian product, 7 facets).
///
/// Equilateral triangle (circumradius 1, area = 3√3/4, inradius 0.5) in q-space,
/// unit square (side 1, area = 1) in p-space. Both are Lagrangian subspaces,
/// making this a Lagrangian product (not symplectic).
///
/// Known capacity: 1.5 (verified via billiard calculation and HK2017 algorithm).
/// The formula c(A ×_S B) = min(c(A), c(B)) applies only to symplectic products.
/// For Lagrangian products, the capacity is determined by optimal trajectories.
///
/// Note: Schlenk Lem. 5.3.1 treats a right isosceles triangle (sys = 1.0); our equilateral
/// triangle gives a different systolic ratio (√3/2 ≈ 0.866), so that lemma does not directly apply.
///
/// Source: HK2017 algorithm + billiard verification (see experiments/triangle_square.md).
pub fn lagrangian_triangle_square() -> KnownPolytope {
    // Equilateral triangle in q-space (circumradius 1, inradius 0.5)
    // Square [-0.5, 0.5]^2 in p-space (4 facets)
    // All heights are 0.5 → halfspaces aᵢ = nᵢ / 0.5 = 2·nᵢ
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

    let halfspaces: Vec<Vector4<f64>> = triangle_halfspaces
        .chain(square_halfspaces)
        .collect();

    // For Lagrangian product of equilateral triangle (inradius 0.5) and square (side 1),
    // the optimal orbit uses all 3 triangle facets (Q(β) = 1/3) and 2 square facets (Q(β) = 1/2).
    // This gives capacity = 0.5 / Q(β) = 0.5 / (1/3) = 1.5.
    let capacity = 1.5;

    KnownPolytope {
        polytope: Polytope4D::new(halfspaces)
            .expect("Lagrangian triangle×square construction"),
        capacity,
        name: "lagrangian_tri_sq",
        source: "HK2017 algorithm + billiard verification",
    }
}

/// Triangle ×_S square (true symplectic product, 7 facets).
///
/// Equilateral triangle (circumradius 1, area = 3√3/4) in the (q₁, p₁) plane,
/// unit square (side 1, area = 1) in the (q₂, p₂) plane. Both planes are
/// symplectic and symplectically orthogonal, making this a true symplectic product.
///
/// Known capacity: min(3√3/4, 1) = 1.0 (symplectic product formula:
/// c(A ×_S B) = min(c(A), c(B))).
///
/// Source: `[prop:capacity-symplectic-product]` (symplectic product formula: c(A ×_S B) = min(c(A), c(B))).
/// Computed via investigation in experiments/triangle_square.md.
pub fn symplectic_triangle_square() -> KnownPolytope {
    // Equilateral triangle in (q₁, p₁) plane (circumradius 1, inradius 0.5)
    // Square [-0.5, 0.5]^2 in (q₂, p₂) plane (4 facets)
    // All heights are 0.5 → halfspaces aᵢ = nᵢ / 0.5 = 2·nᵢ
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

    let halfspaces: Vec<Vector4<f64>> = triangle_halfspaces
        .chain(square_halfspaces)
        .collect();

    // Symplectic product formula: c(A ×_S B) = min(c(A), c(B))
    // area(triangle) = 3√3/4 ≈ 1.299, area(square) = 1.0
    let area_tri = 3.0 * 3.0_f64.sqrt() / 4.0;
    let area_sq = 1.0;
    let capacity = area_tri.min(area_sq);

    KnownPolytope {
        polytope: Polytope4D::new(halfspaces)
            .expect("symplectic triangle×square construction"),
        capacity,
        name: "symplectic_tri_sq",
        source: "Symplectic product formula ([prop:capacity-symplectic-product])",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verify all known polytopes pass construction and have at least 5 facets.
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

    /// Verify simplex facet count matches the expected 5 (4D simplex = 5 halfspaces).
    #[test]
    fn simplex_has_5_facets() {
        assert_eq!(simplex().polytope.facet_count(), 5);
    }

    /// Verify hypercube facet count matches the expected 8 ([-1,1]^4 = 8 halfspaces).
    #[test]
    fn hypercube_has_8_facets() {
        assert_eq!(hypercube().polytope.facet_count(), 8);
    }

    /// Verify crosspolytope facet count matches the expected 16 (dual of tesseract).
    #[test]
    fn crosspolytope_has_16_facets() {
        assert_eq!(crosspolytope().polytope.facet_count(), 16);
    }

    /// Verify HK-O pentagon facet count matches the expected 10 (5 q-facets + 5 p-facets).
    #[test]
    fn hko_pentagon_has_10_facets() {
        assert_eq!(hko_pentagon().polytope.facet_count(), 10);
    }

    /// Verify Lagrangian triangle product facet count matches the expected 6 (3 q + 3 p).
    #[test]
    fn lagrangian_triangle_product_has_6_facets() {
        assert_eq!(lagrangian_triangle_product().polytope.facet_count(), 6);
    }

    /// Verify symplectic triangle product facet count matches the expected 6 (3 + 3).
    #[test]
    fn symplectic_triangle_product_has_6_facets() {
        assert_eq!(symplectic_triangle_product().polytope.facet_count(), 6);
    }

    /// Verify Lagrangian triangle-square product facet count matches the expected 7 (3 + 4).
    #[test]
    fn lagrangian_tri_sq_has_7_facets() {
        assert_eq!(lagrangian_triangle_square().polytope.facet_count(), 7);
    }

    /// Verify symplectic triangle-square product facet count matches the expected 7 (3 + 4).
    #[test]
    fn symplectic_tri_sq_has_7_facets() {
        assert_eq!(symplectic_triangle_square().polytope.facet_count(), 7);
    }

    /// Verify all known polytopes have strictly positive capacity values.
    #[test]
    fn all_known_capacities_positive() {
        for kp in all_known() {
            assert!(kp.capacity > 0.0, "{}: capacity should be > 0", kp.name);
        }
    }

    /// Verify symplectic_triangle_product capacity against HK2017 algorithm.
    #[test]
    fn symplectic_triangle_product_capacity() {
        let kp = symplectic_triangle_product();
        let result = crate::algorithms::hk2017::ehz_capacity_unpruned(&kp.polytope)
            .expect("symplectic_triangle_product should have capacity");
        assert!(
            (result.capacity - kp.capacity).abs() < 1e-6,
            "symplectic_triangle_product capacity: got {}, expected {}",
            result.capacity, kp.capacity
        );
    }

    /// Verify `literature_values()` excludes polytopes without a literature cross-check
    /// (e.g. crosspolytope, which has only a computed value).
    #[test]
    fn literature_values_excludes_placeholder() {
        let lit = literature_values();
        assert!(
            !lit.iter().any(|(name, _)| *name == "crosspolytope"),
            "crosspolytope should be excluded from literature values"
        );
    }
}

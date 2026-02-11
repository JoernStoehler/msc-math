/// Constructors for polytopes with known EHZ capacities from the literature.
use geom::polytope::Polytope4D;
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

/// All known polytopes for the dataset.
pub fn all_known() -> Vec<KnownPolytope> {
    vec![
        simplex(),
        hypercube(),
        crosspolytope(),
        hko_pentagon(),
        triangle_product(),
        lagrangian_triangle_square(),
        symplectic_triangle_square(),
    ]
}

/// 4-simplex (5 facets), translated so origin is at centroid.
///
/// Standard simplex conv{0, e1, e2, e3, e4} with centroid at (0.2, 0.2, 0.2, 0.2).
/// After translation, all heights are positive.
///
/// Known capacity: 0.25 = 1/(2n) for n=2.
/// Source: Y. Nir thesis 2013; Siegel's Symplectic Capacities Project.
pub fn simplex() -> KnownPolytope {
    let centroid = Vector4::new(0.2, 0.2, 0.2, 0.2);
    let normals_raw = vec![
        -Vector4::x(),
        -Vector4::y(),
        -Vector4::z(),
        -Vector4::w(),
        Vector4::new(1.0, 1.0, 1.0, 1.0).normalize(),
    ];
    let heights_raw = vec![0.0, 0.0, 0.0, 0.0, 0.5];
    let heights: Vec<f64> = normals_raw
        .iter()
        .zip(&heights_raw)
        .map(|(n, h)| h - n.dot(&centroid))
        .collect();

    KnownPolytope {
        polytope: Polytope4D::new(normals_raw, heights).expect("simplex construction"),
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
    let normals = vec![
        Vector4::x(),
        -Vector4::x(),
        Vector4::y(),
        -Vector4::y(),
        Vector4::z(),
        -Vector4::z(),
        Vector4::w(),
        -Vector4::w(),
    ];
    let heights = vec![1.0; 8];

    KnownPolytope {
        polytope: Polytope4D::new(normals, heights).expect("hypercube construction"),
        capacity: 4.0,
        name: "hypercube",
        source: "HK2019 Ex 4.6",
    }
}

/// 4D crosspolytope (hyperoctahedron, dual of tesseract). 16 facets.
///
/// Normals: all (±1, ±1, ±1, ±1)/2, heights 1.0.
/// Capacity not yet computed from literature — will use stub (1.0) for now.
pub fn crosspolytope() -> KnownPolytope {
    let normals: Vec<Vector4<f64>> = [-1.0_f64, 1.0]
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
    let heights = vec![1.0; 16];

    KnownPolytope {
        polytope: Polytope4D::new(normals, heights).expect("crosspolytope construction"),
        capacity: 1.0, // placeholder — no literature value yet
        name: "crosspolytope",
        source: "placeholder (capacity unknown)",
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
    let capacity = 2.0 * (PI / 10.0).cos() * (1.0 + (PI / 5.0).cos());

    KnownPolytope {
        polytope: Polytope4D::new(normals, heights).expect("HK-O pentagon construction"),
        capacity,
        name: "hko_pentagon",
        source: "HK-O 2024 Prop 1.4",
    }
}

/// Equilateral triangle ×_L triangle, Lagrangian product (6 facets).
///
/// Regular triangle with circumradius 1 in both q-space and p-space.
/// Known capacity: 1.5.
pub fn triangle_product() -> KnownPolytope {
    // Regular triangle: outward normals at angles π/2 + 2πk/3, inradius = cos(π/3) = 0.5
    let triangle_angles: Vec<f64> = (0..3)
        .map(|k| PI / 2.0 + 2.0 * PI * (k as f64) / 3.0)
        .collect();

    // Q-space triangle + P-space triangle (Lagrangian product)
    let (normals, heights): (Vec<_>, Vec<_>) = triangle_angles
        .iter()
        .map(|a| (Vector4::new(a.cos(), a.sin(), 0.0, 0.0), 0.5))
        .chain(
            triangle_angles
                .iter()
                .map(|a| (Vector4::new(0.0, 0.0, a.cos(), a.sin()), 0.5)),
        )
        .unzip();

    KnownPolytope {
        polytope: Polytope4D::new(normals, heights).expect("triangle product construction"),
        capacity: 1.5,
        name: "triangle_product",
        source: "LP verification (see fixtures.rs)",
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
/// Note: Related to Schlenk Lem. 5.3.1, but Schlenk's result uses a right isosceles
/// triangle (sys = 1.0), whereas this construction uses an equilateral triangle (sys = √3/2 ≈ 0.866).
///
/// Source: HK2017 algorithm + billiard verification (see TRIANGLE_SQUARE_INVESTIGATION.md).
pub fn lagrangian_triangle_square() -> KnownPolytope {
    // Equilateral triangle in q-space (circumradius 1, inradius 0.5)
    let triangle_normals = (0..3).map(|k| {
        let angle = PI / 2.0 + 2.0 * PI * (k as f64) / 3.0;
        Vector4::new(angle.cos(), angle.sin(), 0.0, 0.0)
    });

    // Square [-0.5, 0.5]^2 in p-space (4 facets)
    let square_normals = [
        Vector4::new(0.0, 0.0, 1.0, 0.0),
        Vector4::new(0.0, 0.0, -1.0, 0.0),
        Vector4::new(0.0, 0.0, 0.0, 1.0),
        Vector4::new(0.0, 0.0, 0.0, -1.0),
    ];

    let (normals, heights): (Vec<_>, Vec<_>) = triangle_normals
        .chain(square_normals)
        .map(|n| (n, 0.5))
        .unzip();

    // For Lagrangian product of equilateral triangle (inradius 0.5) and square (side 1),
    // the optimal orbit uses all 3 triangle facets (Q(β) = 1/3) and 2 square facets (Q(β) = 1/2).
    // This gives capacity = 0.5 / Q(β) = 0.5 / (1/3) = 1.5.
    let capacity = 1.5;

    KnownPolytope {
        polytope: Polytope4D::new(normals, heights)
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
/// Known capacity: min(3√3/4, 1) = 1.0 (formula for symplectic products).
/// This verifies Moser's theorem: c(A ×_S B) = min(c(A), c(B)).
///
/// Source: Moser's theorem + functoriality of symplectic products.
/// Computed via investigation in TRIANGLE_SQUARE_INVESTIGATION.md.
pub fn symplectic_triangle_square() -> KnownPolytope {
    // Equilateral triangle in (q₁, p₁) plane (circumradius 1, inradius 0.5)
    // In 4D: normals = (cos θ, 0, sin θ, 0) for θ = π/2 + 2πk/3
    let triangle_normals = (0..3).map(|k| {
        let angle = PI / 2.0 + 2.0 * PI * (k as f64) / 3.0;
        Vector4::new(angle.cos(), 0.0, angle.sin(), 0.0)
    });

    // Square [-0.5, 0.5]^2 in (q₂, p₂) plane (4 facets)
    // In 4D: normals are (0, ±1, 0, 0) and (0, 0, 0, ±1)
    let square_normals = [
        Vector4::new(0.0, 1.0, 0.0, 0.0),
        Vector4::new(0.0, -1.0, 0.0, 0.0),
        Vector4::new(0.0, 0.0, 0.0, 1.0),
        Vector4::new(0.0, 0.0, 0.0, -1.0),
    ];

    let (normals, heights): (Vec<_>, Vec<_>) = triangle_normals
        .chain(square_normals)
        .map(|n| (n, 0.5))
        .unzip();

    // For symplectic product: c(A ×_S B) = min(c(A), c(B))
    // area(triangle) = 3√3/4 ≈ 1.299, area(square) = 1.0
    let area_tri = 3.0 * 3.0_f64.sqrt() / 4.0;
    let area_sq = 1.0;
    let capacity = area_tri.min(area_sq);

    KnownPolytope {
        polytope: Polytope4D::new(normals, heights)
            .expect("symplectic triangle×square construction"),
        capacity,
        name: "symplectic_tri_sq",
        source: "Moser's theorem (symplectic product formula)",
    }
}

#[cfg(test)]
#[path = "known_polytopes_test.rs"]
mod known_polytopes_test;

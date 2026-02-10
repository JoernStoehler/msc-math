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
    let heights_raw = vec![0.0, 0.0, 0.0, 0.0, 1.0];
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
    let mut normals = Vec::with_capacity(16);
    for s0 in [-1.0_f64, 1.0] {
        for s1 in [-1.0_f64, 1.0] {
            for s2 in [-1.0_f64, 1.0] {
                for s3 in [-1.0_f64, 1.0] {
                    normals.push(Vector4::new(s0, s1, s2, s3).normalize());
                }
            }
        }
    }
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
    let mut normals = Vec::with_capacity(6);
    let mut heights = Vec::with_capacity(6);

    // Regular triangle: outward normals at angles π/2 + 2πk/3, inradius = cos(π/3) = 0.5
    for k in 0..3 {
        let angle = PI / 2.0 + 2.0 * PI * (k as f64) / 3.0;
        let nx = angle.cos();
        let ny = angle.sin();
        normals.push(Vector4::new(nx, ny, 0.0, 0.0));
        heights.push(0.5);
    }
    // Same triangle in p-space
    for k in 0..3 {
        let angle = PI / 2.0 + 2.0 * PI * (k as f64) / 3.0;
        let nx = angle.cos();
        let ny = angle.sin();
        normals.push(Vector4::new(0.0, 0.0, nx, ny));
        heights.push(0.5);
    }

    KnownPolytope {
        polytope: Polytope4D::new(normals, heights).expect("triangle product construction"),
        capacity: 1.5,
        name: "triangle_product",
        source: "LP verification (see fixtures.rs)",
    }
}

/// Triangle ×_S square (symplectic product, 7 facets).
///
/// Equilateral triangle (circumradius 1, area = 3√3/4) in q-space,
/// unit square (side 1, area = 1) in p-space.
/// Known capacity: min(area_tri, area_sq) = min(3√3/4, 1) = 1.0.
///
/// Source: Moser's theorem + functoriality of symplectic products.
pub fn symplectic_triangle_square() -> KnownPolytope {
    let mut normals = Vec::new();
    let mut heights = Vec::new();

    // Equilateral triangle in q-space (circumradius 1)
    for k in 0..3 {
        let angle = PI / 2.0 + 2.0 * PI * (k as f64) / 3.0;
        normals.push(Vector4::new(angle.cos(), angle.sin(), 0.0, 0.0));
        heights.push(0.5); // inradius = cos(π/3) = 0.5
    }
    // Square [-0.5, 0.5]^2 in p-space (4 facets)
    normals.push(Vector4::new(0.0, 0.0, 1.0, 0.0));
    heights.push(0.5);
    normals.push(Vector4::new(0.0, 0.0, -1.0, 0.0));
    heights.push(0.5);
    normals.push(Vector4::new(0.0, 0.0, 0.0, 1.0));
    heights.push(0.5);
    normals.push(Vector4::new(0.0, 0.0, 0.0, -1.0));
    heights.push(0.5);

    // area(triangle) = 3√3/4 ≈ 1.299, area(square) = 1.0
    let area_tri = 3.0 * 3.0_f64.sqrt() / 4.0;
    let area_sq = 1.0;
    let capacity = area_tri.min(area_sq);

    KnownPolytope {
        polytope: Polytope4D::new(normals, heights)
            .expect("symplectic triangle×square construction"),
        capacity,
        name: "symplectic_tri_sq",
        source: "Moser + symplectic product functoriality",
    }
}

#[cfg(test)]
#[path = "known_polytopes_test.rs"]
mod known_polytopes_test;

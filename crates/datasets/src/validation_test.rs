use super::*;
use nalgebra::Vector4;

/// Helper: build the centered simplex (5 facets, origin at centroid).
fn simplex_normals_heights() -> (Vec<Vector4<f64>>, Vec<f64>) {
    // Standard simplex conv{0, e1, e2, e3, e4}.
    // Centroid = (0.2, 0.2, 0.2, 0.2).
    // Translate so centroid is at origin: vertices become v_i - centroid.
    // Facet normals don't change but heights do: h_i = h_i_old - n_i · centroid.
    //
    // Original normals and heights:
    //   -e1: h=0, -e2: h=0, -e3: h=0, -e4: h=0
    //   (1,1,1,1)/2: h=1
    //
    // After translation by -centroid:
    //   -e1: h = 0 - (-1)·0.2 = 0.2
    //   -e2: h = 0.2, -e3: h = 0.2, -e4: h = 0.2
    //   (1,1,1,1)/2: h = 1 - (1,1,1,1)/2 · (0.2,...) = 1 - 0.4 = 0.6
    let centroid = Vector4::new(0.2, 0.2, 0.2, 0.2);
    let normals = vec![
        -Vector4::x(),
        -Vector4::y(),
        -Vector4::z(),
        -Vector4::w(),
        Vector4::new(1.0, 1.0, 1.0, 1.0).normalize(),
    ];
    let heights_orig = vec![0.0, 0.0, 0.0, 0.0, 1.0];
    let heights: Vec<f64> = normals
        .iter()
        .zip(&heights_orig)
        .map(|(n, h)| h - n.dot(&centroid))
        .collect();
    (normals, heights)
}

/// Helper: build the hypercube [-1,1]^4 (8 facets).
fn hypercube_normals_heights() -> (Vec<Vector4<f64>>, Vec<f64>) {
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
    (normals, heights)
}

// ---- Duplicate check ----

#[test]
fn accept_distinct_halfspaces() {
    let (normals, _) = hypercube_normals_heights();
    assert!(check_no_duplicates(&normals).is_ok());
}

#[test]
fn reject_exact_duplicate_normals() {
    let normals = vec![
        Vector4::x(),
        Vector4::y(),
        Vector4::z(),
        Vector4::w(),
        Vector4::x(), // duplicate of [0]
    ];
    let err = check_no_duplicates(&normals).unwrap_err();
    assert_eq!(err, ValidationError::DuplicateHalfspaces(0, 4));
}

#[test]
fn accept_antiparallel_normals() {
    // Antiparallel normals are valid: they represent opposite-facing facets
    // (e.g., the hypercube has +e_x and -e_x).
    let normals = vec![
        Vector4::x(),
        Vector4::y(),
        Vector4::z(),
        Vector4::w(),
        -Vector4::x(),
        Vector4::new(1.0, 1.0, 0.0, 0.0).normalize(),
    ];
    assert!(check_no_duplicates(&normals).is_ok());
}

// ---- Boundedness ----

#[test]
fn simplex_is_bounded() {
    let (normals, _) = simplex_normals_heights();
    assert!(check_bounded(&normals).is_ok());
}

#[test]
fn hypercube_is_bounded() {
    let (normals, _) = hypercube_normals_heights();
    assert!(check_bounded(&normals).is_ok());
}

#[test]
fn unbounded_halfspaces() {
    // 5 normals all in roughly the same direction — unbounded toward -direction.
    let normals = vec![
        Vector4::new(1.0, 0.1, 0.0, 0.0).normalize(),
        Vector4::new(1.0, -0.1, 0.0, 0.0).normalize(),
        Vector4::new(1.0, 0.0, 0.1, 0.0).normalize(),
        Vector4::new(1.0, 0.0, -0.1, 0.0).normalize(),
        Vector4::new(1.0, 0.0, 0.0, 0.1).normalize(),
    ];
    let err = check_bounded(&normals).unwrap_err();
    assert_eq!(err, ValidationError::Unbounded);
}

// ---- Vertex enumeration ----

#[test]
fn simplex_has_5_vertices() {
    let (normals, heights) = simplex_normals_heights();
    let verts = enumerate_vertices(&normals, &heights);
    assert_eq!(
        verts.len(),
        5,
        "4D simplex should have 5 vertices, got {}",
        verts.len()
    );
}

#[test]
fn hypercube_has_16_vertices() {
    let (normals, heights) = hypercube_normals_heights();
    let verts = enumerate_vertices(&normals, &heights);
    assert_eq!(
        verts.len(),
        16,
        "4D hypercube should have 16 vertices, got {}",
        verts.len()
    );
}

#[test]
fn simplex_vertices_satisfy_constraints() {
    let (normals, heights) = simplex_normals_heights();
    let verts = enumerate_vertices(&normals, &heights);
    for v in &verts {
        for (n, &h) in normals.iter().zip(&heights) {
            assert!(
                n.dot(v) <= h + EPS_FEASIBILITY,
                "vertex {v:?} violates constraint n·v = {} > h = {}",
                n.dot(v),
                h
            );
        }
    }
}

// ---- Irredundancy ----

#[test]
fn simplex_is_irredundant() {
    let (normals, heights) = simplex_normals_heights();
    let verts = enumerate_vertices(&normals, &heights);
    assert!(check_irredundant(&normals, &heights, &verts).is_ok());
}

#[test]
fn hypercube_is_irredundant() {
    let (normals, heights) = hypercube_normals_heights();
    let verts = enumerate_vertices(&normals, &heights);
    assert!(check_irredundant(&normals, &heights, &verts).is_ok());
}

#[test]
fn detect_redundant_facet() {
    // Take hypercube and add a redundant halfspace (doesn't cut off anything).
    let (mut normals, mut heights) = hypercube_normals_heights();
    // This halfspace is strictly weaker than the existing x ≤ 1:
    normals.push(Vector4::x());
    heights.push(5.0); // x ≤ 5 is redundant since x ≤ 1 already holds

    let verts = enumerate_vertices(&normals, &heights);
    let err = check_irredundant(&normals, &heights, &verts).unwrap_err();
    match err {
        ValidationError::RedundantFacet(idx) => {
            assert_eq!(idx, 8, "redundant facet should be index 8 (the added one)");
        }
        other => panic!("expected RedundantFacet, got {other:?}"),
    }
}

// ---- Full pipeline ----

#[test]
fn validate_simplex() {
    let (normals, heights) = simplex_normals_heights();
    let polytope = validate_polytope(&normals, &heights).unwrap();
    assert_eq!(polytope.facet_count(), 5);
}

#[test]
fn validate_hypercube() {
    let (normals, heights) = hypercube_normals_heights();
    let polytope = validate_polytope(&normals, &heights).unwrap();
    assert_eq!(polytope.facet_count(), 8);
}

// ---- Cross product sanity ----

#[test]
fn cross_product_4d_perpendicular() {
    let a = Vector4::new(1.0, 0.0, 0.0, 0.0);
    let b = Vector4::new(0.0, 1.0, 0.0, 0.0);
    let c = Vector4::new(0.0, 0.0, 1.0, 0.0);
    let d = cross_product_4d(a, b, c);
    assert!(d.dot(&a).abs() < 1e-12);
    assert!(d.dot(&b).abs() < 1e-12);
    assert!(d.dot(&c).abs() < 1e-12);
    assert!(d.norm() > 0.5, "cross product should be nonzero");
}

// ---- Affine rank ----

#[test]
fn affine_rank_single_point() {
    let points = vec![Vector4::new(1.0, 2.0, 3.0, 4.0)];
    assert_eq!(affine_rank(&points), 0);
}

#[test]
fn affine_rank_collinear() {
    let points = vec![
        Vector4::new(0.0, 0.0, 0.0, 0.0),
        Vector4::new(1.0, 0.0, 0.0, 0.0),
        Vector4::new(2.0, 0.0, 0.0, 0.0),
    ];
    assert_eq!(affine_rank(&points), 1);
}

#[test]
fn affine_rank_3d() {
    let points = vec![
        Vector4::new(0.0, 0.0, 0.0, 0.0),
        Vector4::new(1.0, 0.0, 0.0, 0.0),
        Vector4::new(0.0, 1.0, 0.0, 0.0),
        Vector4::new(0.0, 0.0, 1.0, 0.0),
    ];
    assert_eq!(affine_rank(&points), 3);
}

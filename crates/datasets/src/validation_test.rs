use super::*;
use geom::polytope::{ConstructionError, Polytope4D};
use nalgebra::Vector4;

/// Helper: build the centered simplex (5 facets, origin at centroid).
fn simplex_normals_heights() -> (Vec<Vector4<f64>>, Vec<f64>) {
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

/// Helper: construct Polytope4D from normals and heights.
fn make_polytope(normals: &[Vector4<f64>], heights: &[f64]) -> Polytope4D {
    Polytope4D::new(normals.to_vec(), heights.to_vec()).expect("valid polytope")
}

// ---- Duplicate check (now handled by Polytope4D::new) ----

#[test]
fn accept_distinct_halfspaces() {
    let (normals, heights) = hypercube_normals_heights();
    assert!(Polytope4D::new(normals, heights).is_ok());
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
    let heights = vec![1.0; 5];
    let err = Polytope4D::new(normals, heights).unwrap_err();
    assert_eq!(err, ConstructionError::DuplicateHalfspaces { i: 0, j: 4 });
}

#[test]
fn accept_antiparallel_normals() {
    let (normals, heights) = hypercube_normals_heights();
    // Hypercube has +x and -x, etc. — these are antiparallel, not duplicates
    assert!(Polytope4D::new(normals, heights).is_ok());
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

// ---- Vertex enumeration (now via Polytope4D::vertices()) ----

#[test]
fn simplex_has_5_vertices() {
    let (normals, heights) = simplex_normals_heights();
    let polytope = make_polytope(&normals, &heights);
    assert_eq!(
        polytope.vertices().len(),
        5,
        "4D simplex should have 5 vertices, got {}",
        polytope.vertices().len()
    );
}

#[test]
fn hypercube_has_16_vertices() {
    let (normals, heights) = hypercube_normals_heights();
    let polytope = make_polytope(&normals, &heights);
    assert_eq!(
        polytope.vertices().len(),
        16,
        "4D hypercube should have 16 vertices, got {}",
        polytope.vertices().len()
    );
}

#[test]
fn simplex_vertices_satisfy_constraints() {
    let (normals, heights) = simplex_normals_heights();
    let polytope = make_polytope(&normals, &heights);
    for v in polytope.vertices() {
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
    let polytope = make_polytope(&normals, &heights);
    assert!(check_irredundant(&normals, &heights, polytope.vertices()).is_ok());
}

#[test]
fn hypercube_is_irredundant() {
    let (normals, heights) = hypercube_normals_heights();
    let polytope = make_polytope(&normals, &heights);
    assert!(check_irredundant(&normals, &heights, polytope.vertices()).is_ok());
}

#[test]
fn detect_redundant_facet() {
    let (mut normals, mut heights) = hypercube_normals_heights();
    normals.push(Vector4::x());
    heights.push(5.0); // x ≤ 5 is redundant since x ≤ 1 already holds

    // Construction now rejects duplicate normals — but this normal (+x)
    // duplicates normals[0]. Use a slightly different direction instead.
    // We need a truly redundant facet: one whose halfspace is implied by
    // the others, but with a distinct normal.
    let normals_red = vec![
        Vector4::x(),
        -Vector4::x(),
        Vector4::y(),
        -Vector4::y(),
        Vector4::z(),
        -Vector4::z(),
        Vector4::w(),
        -Vector4::w(),
        Vector4::new(1.0, 1.0, 0.0, 0.0).normalize(), // redundant: x+y ≤ √2·h, loose enough
    ];
    let heights_red = vec![1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 10.0];

    let polytope = make_polytope(&normals_red, &heights_red);
    let err = check_irredundant(&normals_red, &heights_red, polytope.vertices()).unwrap_err();
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
    use geom::cross_product::cross_product_4d;
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

use super::*;
use crate::geom::known_polytopes;
use nalgebra::{DMatrix, Vector4};
use num_rational::BigRational;

fn billiard_result(
    name: &str,
    dual_vertices: &[Vector4<f64>],
    dual_vertices_exact: &[[BigRational; 4]],
    facet_intersection_is_nonempty: &DMatrix<bool>,
    omega_signs: &DMatrix<i8>,
) -> crate::algorithms::OrbitSearchResult {
    let classification =
        facet_classification::classify_facets(dual_vertices).expect("valid Lagrangian product");
    let transition_is_allowed = crate::algorithms::facet_adjacency::build_transition_matrix_from_facet_intersections_and_omega(
        facet_intersection_is_nonempty,
        omega_signs,
    );
    let (orbits, iterations) = solve_billiard_candidates(
        dual_vertices,
        &classification.q_indices,
        &classification.p_indices,
        facet_intersection_is_nonempty,
        &transition_is_allowed,
    )
    .unwrap_or_else(|e| panic!("{name}: solve_billiard_candidates returned error: {e:?}"));

    crate::algorithms::aggregate_orbits_with_dual_vertices_exact(
        dual_vertices_exact,
        orbits,
        iterations,
        0.0,
        crate::algorithms::OrbitGuaranteeMode::BoundSafe,
    )
    .unwrap_or_else(|e| panic!("{name}: billiard aggregation returned error: {e:?}"))
}

fn billiard_result_from_known(
    name: &str,
    kp: &crate::geom::known_polytopes::KnownPolytope,
) -> crate::algorithms::OrbitSearchResult {
    billiard_result(
        name,
        kp.polytope.dual_vertices_f64(),
        kp.polytope.dual_vertices(),
        kp.polytope.facet_intersection_is_nonempty(),
        kp.polytope.omega_signs(),
    )
}

/// Helper: assert billiard capacity matches expected value within tolerance.
fn assert_capacity(
    name: &str,
    dual_vertices: &[Vector4<f64>],
    dual_vertices_exact: &[[BigRational; 4]],
    facet_intersection_is_nonempty: &DMatrix<bool>,
    omega_signs: &DMatrix<i8>,
    expected: f64,
    tol: f64,
) {
    let result = billiard_result(
        name,
        dual_vertices,
        dual_vertices_exact,
        facet_intersection_is_nonempty,
        omega_signs,
    );
    let diff = (result.capacity() - expected).abs();
    assert!(
        diff < tol,
        "{}: capacity {:.10} != expected {:.10} (diff {:.2e}, tol {:.2e})",
        name,
        result.capacity(),
        expected,
        diff,
        tol,
    );
}

// ============================================================
// Agreement tests: billiard vs known values
// ============================================================

/// Verify billiard capacity of the hypercube matches the known value (4.0).
#[test]
fn hypercube_capacity() {
    let kp = known_polytopes::hypercube();
    assert_capacity(
        "hypercube",
        kp.polytope.dual_vertices_f64(),
        kp.polytope.dual_vertices(),
        kp.polytope.facet_intersection_is_nonempty(),
        kp.polytope.omega_signs(),
        4.0,
        1e-8,
    );
}

/// Verify billiard capacity of the Lagrangian triangle product matches the known value (1.5).
#[test]
fn triangle_product_capacity() {
    let kp = known_polytopes::lagrangian_triangle_product();
    assert_capacity(
        "triangle_product",
        kp.polytope.dual_vertices_f64(),
        kp.polytope.dual_vertices(),
        kp.polytope.facet_intersection_is_nonempty(),
        kp.polytope.omega_signs(),
        1.5,
        1e-8,
    );
}

/// Verify billiard capacity of the Lagrangian triangle-square product matches the known value (1.5).
#[test]
fn triangle_square_capacity() {
    let kp = known_polytopes::lagrangian_triangle_square();
    assert_capacity(
        "triangle_square",
        kp.polytope.dual_vertices_f64(),
        kp.polytope.dual_vertices(),
        kp.polytope.facet_intersection_is_nonempty(),
        kp.polytope.omega_signs(),
        1.5,
        1e-8,
    );
}

/// Smoke-test the richer billiard collector on a known Lagrangian product.
#[test]
fn triangle_product_orbit_aggregation() {
    let kp = known_polytopes::lagrangian_triangle_product();
    let dual_vertices = kp.polytope.dual_vertices_f64();
    let dual_vertices_exact = kp.polytope.dual_vertices();
    let classification =
        facet_classification::classify_facets(dual_vertices).expect("valid Lagrangian product");
    let transition_is_allowed = crate::algorithms::facet_adjacency::build_transition_matrix_from_facet_intersections_and_omega(
        kp.polytope.facet_intersection_is_nonempty(),
        kp.polytope.omega_signs(),
    );
    let (orbits, iterations) = solve_billiard_candidates(
        dual_vertices,
        &classification.q_indices,
        &classification.p_indices,
        kp.polytope.facet_intersection_is_nonempty(),
        &transition_is_allowed,
    )
    .expect("billiard sigma solve stream should succeed");
    let result = crate::algorithms::aggregate_orbits_with_dual_vertices_exact(
        dual_vertices_exact,
        orbits,
        iterations,
        0.0,
        crate::algorithms::OrbitGuaranteeMode::BoundSafe,
    )
    .expect("billiard orbit aggregation should succeed");

    assert!(
        !result.orbits.is_empty(),
        "collector must return at least one orbit"
    );
    assert!(result.min_action_lower <= result.min_action_upper);
}

/// Verify billiard capacity of the HK-O pentagon counterexample matches the analytic formula.
///
/// **Why release mode:** ~50k KKT solves, too slow for debug suite.
#[test]
#[ignore] // 50k KKT solves -- slow in debug, run with --release --ignored
fn hko_pentagon_capacity() {
    let kp = known_polytopes::hko_pentagon();
    let expected =
        2.0 * (std::f64::consts::PI / 10.0).cos() * (1.0 + (std::f64::consts::PI / 5.0).cos());
    assert_capacity(
        "hko_pentagon",
        kp.polytope.dual_vertices_f64(),
        kp.polytope.dual_vertices(),
        kp.polytope.facet_intersection_is_nonempty(),
        kp.polytope.omega_signs(),
        expected,
        1e-6,
    );
}

// ============================================================
// Agreement tests: billiard vs hk2017
// ============================================================

/// Cross-algorithm check: billiard and hk2017 agree on hypercube capacity.
#[test]
#[ignore] // runs hk2017 live -- release-only cross-algorithm check
fn agrees_with_hk2017_hypercube() {
    let kp = known_polytopes::hypercube();
    let billiard = billiard_result_from_known("hypercube", kp);
    let hk = crate::ehz_capacity_pruned(&kp.polytope).unwrap();
    let diff = (billiard.capacity() - hk.capacity()).abs();
    assert!(
        diff < 1e-8,
        "hypercube: billiard {:.10} != hk2017 {:.10} (diff {:.2e})",
        billiard.capacity(),
        hk.capacity(),
        diff,
    );
}

/// Cross-algorithm check: billiard and hk2017 agree on triangle product capacity.
#[test]
#[ignore] // runs hk2017 live -- release-only cross-algorithm check
fn agrees_with_hk2017_triangle_product() {
    let kp = known_polytopes::lagrangian_triangle_product();
    let billiard = billiard_result_from_known("triangle_product", kp);
    let hk = crate::ehz_capacity_pruned(&kp.polytope).unwrap();
    let diff = (billiard.capacity() - hk.capacity()).abs();
    assert!(
        diff < 1e-8,
        "triangle_product: billiard {:.10} != hk2017 {:.10} (diff {:.2e})",
        billiard.capacity(),
        hk.capacity(),
        diff,
    );
}

/// Cross-algorithm check: billiard and hk2017 agree on triangle-square product capacity.
#[test]
#[ignore] // runs hk2017 live -- release-only cross-algorithm check
fn agrees_with_hk2017_triangle_square() {
    let kp = known_polytopes::lagrangian_triangle_square();
    let billiard = billiard_result_from_known("triangle_square", kp);
    let hk = crate::ehz_capacity_pruned(&kp.polytope).unwrap();
    let diff = (billiard.capacity() - hk.capacity()).abs();
    assert!(
        diff < 1e-8,
        "triangle_square: billiard {:.10} != hk2017 {:.10} (diff {:.2e})",
        billiard.capacity(),
        hk.capacity(),
        diff,
    );
}

/// Cross-algorithm check: billiard and hk2017 agree on HK-O pentagon capacity.
///
/// **Why release mode:** hk2017 on 10-facet pentagon is exponential (~60s even in release).
#[test]
#[ignore] // hk2017 on 10-facet pentagon takes ~60s; verified against known value instead
fn agrees_with_hk2017_hko_pentagon() {
    let kp = known_polytopes::hko_pentagon();
    let billiard = billiard_result_from_known("hko_pentagon", kp);
    let hk = crate::ehz_capacity_pruned(&kp.polytope).unwrap();
    let diff = (billiard.capacity() - hk.capacity()).abs();
    assert!(
        diff < 1e-8,
        "hko_pentagon: billiard {:.10} != hk2017 {:.10} (diff {:.2e})",
        billiard.capacity(),
        hk.capacity(),
        diff,
    );
}

// ============================================================
// Error handling tests
// ============================================================

/// Verify billiard algorithm rejects the simplex (not a Lagrangian product).
#[test]
fn rejects_non_lagrangian_product() {
    let kp = known_polytopes::simplex();
    let result = for_each_sigma(
        kp.polytope.dual_vertices_f64(),
        kp.polytope.facet_intersection_is_nonempty(),
        kp.polytope.omega_signs(),
        |_| {},
    );
    assert!(
        result.is_err(),
        "simplex should not be a Lagrangian product"
    );
}

/// Verify billiard algorithm rejects the symplectic triangle product
/// (normals in symplectic planes, not Lagrangian subspaces).
#[test]
fn rejects_symplectic_triangle_product() {
    let kp = known_polytopes::symplectic_triangle_product();
    let result = for_each_sigma(
        kp.polytope.dual_vertices_f64(),
        kp.polytope.facet_intersection_is_nonempty(),
        kp.polytope.omega_signs(),
        |_| {},
    );
    assert!(
        result.is_err(),
        "symplectic triangle product should not be a Lagrangian product"
    );
}

// ============================================================
// Property tests
// ============================================================

/// Verify the billiard iteration count stays within a polynomial bound.
///
/// **Why release mode:** ~50k KKT solves on the 10-facet pentagon.
#[test]
#[ignore] // 50k KKT solves -- slow in debug, run with --release --ignored
fn billiard_iterations_polynomial() {
    let kp = known_polytopes::hko_pentagon();
    let result = billiard_result_from_known("hko_pentagon", kp);
    // For 5+5 facets, expect on the order of 100k iterations.
    // If it exceeds 1M, something is wrong.
    assert!(
        result.iterations < 1_000_000,
        "pentagon: {} iterations exceeds polynomial bound",
        result.iterations,
    );
}

/// Check structural properties of the shared orbit/result surface on a
/// billiard-domain polytope.
fn assert_result_properties(
    name: &str,
    dual_vertices: &[Vector4<f64>],
    result: &crate::algorithms::OrbitSearchResult,
) {
    let bounce_count = bounce_count_from_sigma(dual_vertices, result.best_sigma())
        .expect("test polytope should be Lagrangian product")
        .expect("winning sigma should have valid billiard block structure");
    assert!(
        bounce_count == 2 || bounce_count == 3,
        "{}: bounce_count = {} (expected 2 or 3)",
        name,
        bounce_count,
    );

    // All beta positive.
    for (i, &b) in result.best_beta().iter().enumerate() {
        assert!(b > 0.0, "{}: beta[{}] = {:.2e} <= 0", name, i, b);
    }

    assert!(
        result.capacity() > 0.0,
        "{name}: billiard capacity should be positive"
    );
}

/// Verify structural properties of the shared result surface on small
/// Lagrangian products.
#[test]
fn result_properties() {
    for (name, kp) in lagrangian_test_cases_fast() {
        let result = billiard_result_from_known(name, kp);
        assert_result_properties(name, kp.polytope.dual_vertices_f64(), &result);
    }
}

/// Verify structural properties of the shared result surface on the HK-O
/// pentagon.
///
/// **Why release mode:** ~50k KKT solves, too slow for debug suite.
#[test]
#[ignore] // 50k KKT solves -- slow in debug, run with --release --ignored
fn result_properties_pentagon() {
    let kp = known_polytopes::hko_pentagon();
    let result = billiard_result_from_known("hko_pentagon", kp);
    assert_result_properties("hko_pentagon", kp.polytope.dual_vertices_f64(), &result);
}

/// Small Lagrangian products: fast in both debug and release.
fn lagrangian_test_cases_fast() -> Vec<(
    &'static str,
    &'static crate::geom::known_polytopes::KnownPolytope,
)> {
    vec![
        ("hypercube", known_polytopes::hypercube()),
        (
            "triangle_product",
            known_polytopes::lagrangian_triangle_product(),
        ),
        (
            "triangle_square",
            known_polytopes::lagrangian_triangle_square(),
        ),
    ]
}

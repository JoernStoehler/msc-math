use geom::known_polytopes;

use crate::billiard_capacity;

/// Helper: assert billiard capacity matches expected value within tolerance.
fn assert_capacity(name: &str, polytope: &geom::polytope::Polytope4D, expected: f64, tol: f64) {
    let result = billiard_capacity(polytope)
        .unwrap_or_else(|e| panic!("{}: billiard_capacity returned error: {}", name, e))
        .unwrap_or_else(|| panic!("{}: billiard_capacity returned None", name));

    let diff = (result.capacity - expected).abs();
    assert!(
        diff < tol,
        "{}: capacity {:.10} != expected {:.10} (diff {:.2e}, tol {:.2e})",
        name,
        result.capacity,
        expected,
        diff,
        tol,
    );
}

// ============================================================
// Agreement tests: billiard vs known values
// ============================================================

#[test]
fn hypercube_capacity() {
    let kp = known_polytopes::hypercube();
    assert_capacity("hypercube", &kp.polytope, 4.0, 1e-8);
}

#[test]
fn triangle_product_capacity() {
    let kp = known_polytopes::lagrangian_triangle_product();
    assert_capacity("triangle_product", &kp.polytope, 1.5, 1e-8);
}

#[test]
fn triangle_square_capacity() {
    let kp = known_polytopes::lagrangian_triangle_square();
    assert_capacity("triangle_square", &kp.polytope, 1.5, 1e-8);
}

#[test]
#[ignore] // 50k KKT solves — slow in debug, run with --release --ignored
fn hko_pentagon_capacity() {
    let kp = known_polytopes::hko_pentagon();
    let expected = 2.0 * (std::f64::consts::PI / 10.0).cos() * (1.0 + (std::f64::consts::PI / 5.0).cos());
    assert_capacity("hko_pentagon", &kp.polytope, expected, 1e-6);
}

// ============================================================
// Agreement tests: billiard vs hk2017
// ============================================================

#[test]
#[ignore] // runs hk2017 live — release-only cross-algorithm check
fn agrees_with_hk2017_hypercube() {
    let kp = known_polytopes::hypercube();
    let billiard = billiard_capacity(&kp.polytope).unwrap().unwrap();
    let hk = hk2017::ehz_capacity_pruned(&kp.polytope).unwrap();
    let diff = (billiard.capacity - hk.capacity).abs();
    assert!(
        diff < 1e-8,
        "hypercube: billiard {:.10} != hk2017 {:.10} (diff {:.2e})",
        billiard.capacity,
        hk.capacity,
        diff,
    );
}

#[test]
#[ignore] // runs hk2017 live — release-only cross-algorithm check
fn agrees_with_hk2017_triangle_product() {
    let kp = known_polytopes::lagrangian_triangle_product();
    let billiard = billiard_capacity(&kp.polytope).unwrap().unwrap();
    let hk = hk2017::ehz_capacity_pruned(&kp.polytope).unwrap();
    let diff = (billiard.capacity - hk.capacity).abs();
    assert!(
        diff < 1e-8,
        "triangle_product: billiard {:.10} != hk2017 {:.10} (diff {:.2e})",
        billiard.capacity,
        hk.capacity,
        diff,
    );
}

#[test]
#[ignore] // runs hk2017 live — release-only cross-algorithm check
fn agrees_with_hk2017_triangle_square() {
    let kp = known_polytopes::lagrangian_triangle_square();
    let billiard = billiard_capacity(&kp.polytope).unwrap().unwrap();
    let hk = hk2017::ehz_capacity_pruned(&kp.polytope).unwrap();
    let diff = (billiard.capacity - hk.capacity).abs();
    assert!(
        diff < 1e-8,
        "triangle_square: billiard {:.10} != hk2017 {:.10} (diff {:.2e})",
        billiard.capacity,
        hk.capacity,
        diff,
    );
}

#[test]
#[ignore] // hk2017 on 10-facet pentagon takes ~60s; capacity verified against known value instead
fn agrees_with_hk2017_hko_pentagon() {
    let kp = known_polytopes::hko_pentagon();
    let billiard = billiard_capacity(&kp.polytope).unwrap().unwrap();
    let hk = hk2017::ehz_capacity_pruned(&kp.polytope).unwrap();
    let diff = (billiard.capacity - hk.capacity).abs();
    assert!(
        diff < 1e-8,
        "hko_pentagon: billiard {:.10} != hk2017 {:.10} (diff {:.2e})",
        billiard.capacity,
        hk.capacity,
        diff,
    );
}

// ============================================================
// Error handling tests
// ============================================================

#[test]
fn rejects_non_lagrangian_product() {
    // Simplex has mixed normals — not a Lagrangian product.
    let kp = known_polytopes::simplex();
    let result = billiard_capacity(&kp.polytope);
    assert!(result.is_err(), "simplex should not be a Lagrangian product");
}

#[test]
fn rejects_symplectic_triangle_product() {
    // Symplectic product has normals in symplectic planes, not Lagrangian subspaces.
    let kp = known_polytopes::symplectic_triangle_product();
    let result = billiard_capacity(&kp.polytope);
    assert!(
        result.is_err(),
        "symplectic triangle product should not be a Lagrangian product"
    );
}

// ============================================================
// Property tests
// ============================================================

#[test]
#[ignore] // 50k KKT solves — slow in debug, run with --release --ignored
fn billiard_iterations_polynomial() {
    // The billiard iteration count should be bounded by O(n_q^3 * n_p^3).
    // For the pentagon (n_q = 5, n_p = 5), a generous bound: 5^3 * 5^3 * 36 * 64 = ~288M.
    // In practice it's much less. Just verify it's a reasonable number.
    let kp = known_polytopes::hko_pentagon();
    let result = billiard_capacity(&kp.polytope).unwrap().unwrap();
    // For 5+5 facets, expect on the order of 100k iterations.
    // If it exceeds 1M, something is wrong.
    assert!(
        result.iterations < 1_000_000,
        "pentagon: {} iterations exceeds polynomial bound",
        result.iterations,
    );
}

/// Check structural properties of a BilliardResult.
fn assert_result_properties(name: &str, result: &crate::BilliardResult) {
    // bounce_count is 2 or 3
    assert!(
        result.bounce_count == 2 || result.bounce_count == 3,
        "{}: bounce_count = {} (expected 2 or 3)",
        name,
        result.bounce_count,
    );

    // all beta positive
    for (i, &b) in result.best_beta.iter().enumerate() {
        assert!(
            b > 0.0,
            "{}: beta[{}] = {:.2e} <= 0",
            name,
            i,
            b,
        );
    }

    // permutation length matches 2k structure (between 2k and 4k)
    let k = result.bounce_count;
    let len = result.best_permutation.len();
    assert!(
        len >= 2 * k && len <= 4 * k,
        "{}: permutation len {} not in [{}, {}] for k={}",
        name,
        len,
        2 * k,
        4 * k,
        k,
    );
}

#[test]
fn result_properties() {
    // Fast polytopes only — exercises all code paths without pentagon's 50k KKT solves.
    // Pentagon covered by result_properties_pentagon (release-only).
    for (name, polytope) in lagrangian_test_cases_fast() {
        let result = billiard_capacity(&polytope).unwrap().unwrap();
        assert_result_properties(name, &result);
    }
}

#[test]
#[ignore] // 50k KKT solves — slow in debug, run with --release --ignored
fn result_properties_pentagon() {
    let kp = known_polytopes::hko_pentagon();
    let result = billiard_capacity(&kp.polytope).unwrap().unwrap();
    assert_result_properties("hko_pentagon", &result);
}

/// Small Lagrangian products: fast in both debug and release.
fn lagrangian_test_cases_fast() -> Vec<(&'static str, geom::polytope::Polytope4D)> {
    vec![
        ("hypercube", known_polytopes::hypercube().polytope),
        ("triangle_product", known_polytopes::lagrangian_triangle_product().polytope),
        ("triangle_square", known_polytopes::lagrangian_triangle_square().polytope),
    ]
}

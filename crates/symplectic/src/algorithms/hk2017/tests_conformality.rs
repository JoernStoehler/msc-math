//! HK2017 conformality tests.
//!
//! Split from mod.rs to keep module routing and docs short.

use crate::ehz_capacity_unpruned;

// ── Direct computation ──

/// Verify conformality c(alpha*K) = alpha^2 * c(K) on the simplex.
///
/// This is a small live smoke test for the current HK2017 implementation.
/// Broad conformality validation lives in `experiments/verification/correctness/`.
#[test]
fn capacity_conformality_simplex() {
    let scale = 1.7;
    let kp = crate::geom::known_polytopes::simplex();
    let scaled = crate::geom::polytope::Polytope4D::from_f64(
        kp.polytope
            .dual_vertices_f64()
            .iter()
            .map(|a| a / scale)
            .collect(),
    )
    .expect("scaled simplex");

    let base_cap = ehz_capacity_unpruned(&kp.polytope)
        .expect("simplex capacity")
        .capacity();
    let scaled_cap = ehz_capacity_unpruned(&scaled)
        .expect("scaled simplex capacity")
        .capacity();
    let expected = scale * scale * base_cap;
    let relative_error = ((scaled_cap - expected) / expected).abs();
    assert!(
        relative_error < 1e-6,
        "simplex conformality failed: scale={scale}, base_cap={base_cap}, \
         scaled_cap={scaled_cap}, expected={expected}, relative_error={relative_error}"
    );
}

/// Verify conformality on hypercube scaled by e (transcendental).
///
/// Uses lambda = e (transcendental) to ensure numerical coincidences are impossible.
/// Expected: c(e * K) = e^2 * c(K).
///
/// Why #[ignore]: F=8 unpruned x 2 is slower than the simplex smoke test.
/// Run: `cargo test --release capacity_scales_quadratically -- --ignored`
#[test]
#[ignore] // ~48s debug, ~0.6s release
fn capacity_scales_quadratically() {
    use crate::geom::known_polytopes;

    let scale = std::f64::consts::E;

    let kp = known_polytopes::hypercube();
    let unit_cap = ehz_capacity_unpruned(&kp.polytope).unwrap().capacity();

    let scaled_cube = crate::geom::polytope::Polytope4D::from_f64(
        crate::geom::test_utils::scaled_hypercube_dual_vertices_f64(scale),
    )
    .expect("scaled hypercube");
    let scaled_cap = ehz_capacity_unpruned(&scaled_cube).unwrap().capacity();

    let expected = unit_cap * scale * scale;
    let relative_error = ((scaled_cap - expected) / expected).abs();

    assert!(
        relative_error < 1e-4,
        "capacity scaling failed: scale={scale}, unit_cap={unit_cap}, \
         scaled_cap={scaled_cap}, expected={expected}, relative_error={relative_error}"
    );
}

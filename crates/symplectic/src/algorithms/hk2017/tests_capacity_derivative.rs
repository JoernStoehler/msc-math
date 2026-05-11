//! HK2017 capacity derivative tests.
//!
//! Split from mod.rs to keep module routing and docs short.

use crate::algorithms::test_helpers::{
    pruned_capacity_for_dual_vertices, pruned_capacity_for_fixture,
};
use crate::exact::exact_vertices_with_incidence;
use crate::geom::known_polytopes;
use crate::geom::rational_arithmetic::{f64_to_rational, rational_to_f64};
use crate::test_lib::euclidean_volume_f64;
use euclidean_polytopes::volume_from_incidence_exact;
use nalgebra::Vector4;

/// Step size for central finite differences of capacity.
///
/// Chosen as geometric mean of machine epsilon (~1e-16) and typical height scale (~1):
/// eps ~ 1e-7 to 1e-6. We use 1e-6 for capacity (expensive, want stability).
const FD_EPS_CAP: f64 = 1e-6;

/// Step size for central finite differences of volume.
///
/// Tighter than capacity because known-incidence volume is deterministic and cheap.
const FD_EPS_VOL: f64 = 1e-7;

/// Construct a perturbed polytope: h_k -> h_k + delta, all other heights unchanged.
///
/// Returns `None` if construction fails (should not happen for small perturbations
/// of valid polytopes).
fn perturbed_dual_vertices(
    normals: &[Vector4<f64>],
    heights: &[f64],
    facet: usize,
    delta: f64,
) -> Vec<Vector4<f64>> {
    let mut h = heights.to_vec();
    h[facet] += delta;
    normals
        .iter()
        .zip(h.iter())
        .map(|(n, &hi)| n / hi)
        .collect()
}

fn exact_vectors_from_dual_vertices_f64(
    dual_vertices: &[Vector4<f64>],
) -> Vec<Vector4<num_rational::BigRational>> {
    dual_vertices
        .iter()
        .map(|a| {
            Vector4::new(
                f64_to_rational(a[0]),
                f64_to_rational(a[1]),
                f64_to_rational(a[2]),
                f64_to_rational(a[3]),
            )
        })
        .collect()
}

fn volume_for_dual_vertices(dual_vertices: &[Vector4<f64>]) -> f64 {
    let exact_dual_vertices = exact_vectors_from_dual_vertices_f64(dual_vertices);
    let vertices_with_incidence =
        exact_vertices_with_incidence(&exact_dual_vertices).expect("perturbed vertices");
    rational_to_f64(&volume_from_incidence_exact(
        &vertices_with_incidence.vertices,
        &vertices_with_incidence.vertex_facet_incidence,
    ))
}

/// Compute FD volume derivatives: dvol/dh_k ~ (vol(h+eps*e_k) - vol(h-eps*e_k)) / (2*eps).
///
/// Uses the Euclidean exact known-incidence star triangulation.
fn fd_volume_derivatives(normals: &[Vector4<f64>], heights: &[f64]) -> Vec<f64> {
    let f = heights.len();
    (0..f)
        .map(|k| {
            let p_plus = perturbed_dual_vertices(normals, heights, k, FD_EPS_VOL);
            let p_minus = perturbed_dual_vertices(normals, heights, k, -FD_EPS_VOL);
            let vol_plus = volume_for_dual_vertices(&p_plus);
            let vol_minus = volume_for_dual_vertices(&p_minus);
            (vol_plus - vol_minus) / (2.0 * FD_EPS_VOL)
        })
        .collect()
}

/// Compute FD capacity derivatives: dc/dh_k ~ (c(h+eps*e_k) - c(h-eps*e_k)) / (2*eps).
///
/// At non-smooth points (tied orbits), this computes the directional derivative of
/// the envelope (max over orbits), not a single-orbit subgradient.
fn fd_capacity_derivatives(normals: &[Vector4<f64>], heights: &[f64]) -> Vec<f64> {
    let f = heights.len();
    (0..f)
        .map(|k| {
            let p_plus = perturbed_dual_vertices(normals, heights, k, FD_EPS_CAP);
            let p_minus = perturbed_dual_vertices(normals, heights, k, -FD_EPS_CAP);
            let cap_plus = pruned_capacity_for_dual_vertices(&p_plus)
                .expect("capacity +eps")
                .capacity();
            let cap_minus = pruned_capacity_for_dual_vertices(&p_minus)
                .expect("capacity -eps")
                .capacity();
            (cap_plus - cap_minus) / (2.0 * FD_EPS_CAP)
        })
        .collect()
}

// ===== Default suite: fast tests (debug mode, < 5s each) =====

/// Extract unit normals and heights from dual vertices: n_i = a_i/||a_i||, h_i = 1/||a_i||.
///
/// Used by FD tests that perturb heights to verify Euler homogeneity identities.
fn normals_and_heights(dual_vertices: &[Vector4<f64>]) -> (Vec<Vector4<f64>>, Vec<f64>) {
    let normals: Vec<Vector4<f64>> = dual_vertices.iter().map(|a| a / a.norm()).collect();
    let heights: Vec<f64> = dual_vertices.iter().map(|a| 1.0 / a.norm()).collect();
    (normals, heights)
}

/// T1: FD capacity derivatives are finite and non-negative for the simplex (5 facets).
///
/// Proposition: For the 4-simplex, dc_EHZ/dh_k is finite and >= 0 for all k.
/// Method: Central FD with eps = 1e-6, `ehz_capacity` on perturbed polytopes.
/// Why default suite: 5 facets -> 10 capacity calls, simplex is fast even in debug.
#[test]
fn fd_capacity_height_simplex() {
    let kp = known_polytopes::simplex();
    let (normals, heights) = normals_and_heights(&kp.dual_vertices_f64);

    let d_cap = fd_capacity_derivatives(&normals, &heights);

    for (k, &dc) in d_cap.iter().enumerate() {
        assert!(dc.is_finite(), "facet {k}: d_cap/d_h is not finite ({dc})");
        assert!(
            dc >= -1e-4,
            "facet {k}: d_cap/d_h = {dc:.6} violates monotonicity (expected >= 0)"
        );
    }
}

/// T2: Euler homogeneity identity for volume: sum h_k * dvol/dh_k = 4*vol.
///
/// Volume is degree-4 homogeneous in heights, so Euler's identity gives
/// sum h_k * dvol/dh_k = 4*vol(K).
///
/// Polytopes: simplex, hypercube. Tolerance: 0.1% relative.
///
#[test]
fn euler_homogeneity_volume() {
    let polytopes = vec![
        ("simplex", known_polytopes::simplex()),
        ("hypercube", known_polytopes::hypercube()),
    ];

    for (name, kp) in &polytopes {
        let (normals, heights) = normals_and_heights(&kp.dual_vertices_f64);
        let vol = euclidean_volume_f64(&kp.vertices, &kp.vertex_facet_incidence);

        let d_vol = fd_volume_derivatives(&normals, &heights);
        let euler_sum: f64 = heights.iter().zip(&d_vol).map(|(h, dv)| h * dv).sum();
        let expected = 4.0 * vol;
        let rel_err = (euler_sum - expected).abs() / expected;

        assert!(
            rel_err < 0.01,
            "{name}: Euler vol identity failed: sum h*dvol/dh = {euler_sum:.8}, \
             4*vol = {expected:.8}, rel_err = {rel_err:.2e}"
        );
    }
}

/// T3: Capacity monotonicity for the simplex: dc/dh_k >= 0 for all k.
///
/// c_EHZ is monotone under inclusion (P7 in capacity axioms). Increasing any
/// height h_k enlarges K, so dc/dh_k >= 0.
#[test]
fn capacity_monotone_simplex() {
    let kp = known_polytopes::simplex();
    let (normals, heights) = normals_and_heights(&kp.dual_vertices_f64);

    let d_cap = fd_capacity_derivatives(&normals, &heights);

    for (k, &dc) in d_cap.iter().enumerate() {
        assert!(
            dc >= -1e-4,
            "simplex facet {k}: dc/dh = {dc:.6e} < 0 (monotonicity violation)"
        );
    }
}

// ===== Ignored suite: expensive tests (release mode) =====

/// T4: FD capacity derivatives are finite and non-negative for larger polytopes.
///
/// Polytopes: hypercube (8F), lagrangian_triangle_product (6F), HKO pentagon (10F).
/// Why #[ignore]: 8-10 facets x 2 capacity calls each, too slow for debug mode.
/// Runtime: ~10s in release.
#[test]
#[ignore]
fn fd_capacity_height_known_polytopes() {
    let polytopes = vec![
        ("hypercube", known_polytopes::hypercube()),
        (
            "lagrangian_tri",
            known_polytopes::lagrangian_triangle_product(),
        ),
        ("hko_pentagon", known_polytopes::hko_pentagon()),
    ];

    for (name, kp) in &polytopes {
        let (normals, heights) = normals_and_heights(&kp.dual_vertices_f64);

        let d_cap = fd_capacity_derivatives(&normals, &heights);

        for (k, &dc) in d_cap.iter().enumerate() {
            assert!(
                dc.is_finite(),
                "{name} facet {k}: d_cap/d_h is not finite ({dc})"
            );
            // Relaxed tolerance for non-smooth points (HKO pentagon has 44 tied orbits).
            assert!(
                dc >= -1e-3,
                "{name} facet {k}: d_cap/d_h = {dc:.6e} violates monotonicity"
            );
        }
    }
}

/// T5: Euler homogeneity identity for capacity: sum h_k * dc/dh_k = 2*c.
///
/// c_EHZ is degree-2 homogeneous in heights (conformality + scaling of h),
/// so Euler's identity gives sum h_k * dc/dh_k = 2*c.
///
/// **This test catches the sign bug:** wrong sign gives sum = -2c instead of +2c.
///
/// Polytopes: simplex, hypercube, lagrangian_triangle_product — all generic (unique
/// optimal orbit). HKO pentagon excluded: 44 tied orbits make capacity non-smooth,
/// so FD envelope derivative != Euler identity.
/// Tolerance: 1% relative.
/// Runtime: ~10s in release.
#[test]
#[ignore]
fn euler_homogeneity_capacity() {
    let polytopes = vec![
        ("simplex", known_polytopes::simplex()),
        ("hypercube", known_polytopes::hypercube()),
        (
            "lagrangian_tri",
            known_polytopes::lagrangian_triangle_product(),
        ),
    ];

    for (name, kp) in &polytopes {
        let (normals, heights) = normals_and_heights(&kp.dual_vertices_f64);
        let cap = pruned_capacity_for_fixture(kp)
            .expect("capacity")
            .capacity();

        let d_cap = fd_capacity_derivatives(&normals, &heights);
        let euler_sum: f64 = heights.iter().zip(&d_cap).map(|(h, dc)| h * dc).sum();
        let expected = 2.0 * cap;
        let rel_err = (euler_sum - expected).abs() / expected;

        eprintln!(
            "{name}: Euler cap: sum h*dc/dh = {euler_sum:.6}, 2c = {expected:.6}, \
             ratio = {:.4}, rel_err = {rel_err:.2e}",
            euler_sum / expected
        );

        assert!(
            rel_err < 0.01,
            "{name}: Euler capacity identity failed: sum h*dc/dh = {euler_sum:.8}, \
             2c = {expected:.8}, rel_err = {rel_err:.2e} (>1%)"
        );
    }
}

/// T6: Capacity monotonicity for known polytopes with more facets.
///
/// dc/dh_k >= 0 for all k (monotonicity under inclusion).
/// Polytopes: hypercube (8F), lagrangian_triangle_product (6F), HKO pentagon (10F).
#[test]
#[ignore]
fn capacity_monotone_known_polytopes() {
    let polytopes = vec![
        ("hypercube", known_polytopes::hypercube()),
        (
            "lagrangian_tri",
            known_polytopes::lagrangian_triangle_product(),
        ),
        ("hko_pentagon", known_polytopes::hko_pentagon()),
    ];

    for (name, kp) in &polytopes {
        let (normals, heights) = normals_and_heights(&kp.dual_vertices_f64);

        let d_cap = fd_capacity_derivatives(&normals, &heights);

        for (k, &dc) in d_cap.iter().enumerate() {
            assert!(
                dc >= -1e-3,
                "{name} facet {k}: dc/dh = {dc:.6e} < 0 (monotonicity violation)"
            );
        }
    }
}

/// T7: Euler homogeneity for sys = c^2/(2*vol): sum h_k * dsys/dh_k = 0.
///
/// sys(K) = c_EHZ(K)^2 / (2*vol(K)) is degree 0 in heights:
/// sys(lambda*h) = (lambda^2 c)^2 / (2*lambda^4*vol) = lambda^0 * sys.
/// Euler identity: sum h_k * dsys/dh_k = 0.
///
/// Polytopes: simplex, hypercube — generic (unique optimal orbit).
/// HKO pentagon excluded: non-smooth capacity invalidates Euler identity for FD.
/// Tolerance: 1% of sys value (absolute, since expected value is 0).
#[test]
#[ignore]
fn fd_sys_height_euler() {
    let polytopes = vec![
        ("simplex", known_polytopes::simplex()),
        ("hypercube", known_polytopes::hypercube()),
    ];

    for (name, kp) in &polytopes {
        let (normals, heights) = normals_and_heights(&kp.dual_vertices_f64);

        let cap = pruned_capacity_for_fixture(kp)
            .expect("capacity")
            .capacity();
        let vol = euclidean_volume_f64(&kp.vertices, &kp.vertex_facet_incidence);
        let sys = cap * cap / (2.0 * vol);

        // FD sys derivatives.
        let d_sys: Vec<f64> = (0..heights.len())
            .map(|k| {
                let p_plus = perturbed_dual_vertices(&normals, &heights, k, FD_EPS_CAP);
                let p_minus = perturbed_dual_vertices(&normals, &heights, k, -FD_EPS_CAP);
                let cap_p = pruned_capacity_for_dual_vertices(&p_plus)
                    .expect("cap +eps")
                    .capacity();
                let cap_m = pruned_capacity_for_dual_vertices(&p_minus)
                    .expect("cap -eps")
                    .capacity();
                let vol_p = volume_for_dual_vertices(&p_plus);
                let vol_m = volume_for_dual_vertices(&p_minus);
                let sys_p = cap_p * cap_p / (2.0 * vol_p);
                let sys_m = cap_m * cap_m / (2.0 * vol_m);
                (sys_p - sys_m) / (2.0 * FD_EPS_CAP)
            })
            .collect();

        let euler_sum: f64 = heights.iter().zip(&d_sys).map(|(h, ds)| h * ds).sum();
        // Expected: 0 (degree 0 in h).

        eprintln!(
            "{name}: Euler sys: sum h*dsys/dh = {euler_sum:.6e}, sys = {sys:.6}, \
             ratio = {:.4e}",
            euler_sum / sys
        );

        assert!(
            euler_sum.abs() < 0.01 * sys,
            "{name}: Euler sys identity failed: sum h*dsys/dh = {euler_sum:.6e}, \
             expected 0, sys = {sys:.6} (ratio = {:.2e})",
            euler_sum / sys
        );
    }
}

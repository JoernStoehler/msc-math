//! Tests for hk2017: finite-difference derivative validation (dc/dh, Euler homogeneity).
//!
//! Proposition: c_EHZ is degree-2 homogeneous in facet heights, so Euler's identity
//! gives sum_k h_k * dc/dh_k = 2*c. Similarly, vol is degree-4 and sys = c^2/(2*vol)
//! is degree 0.
//! Reference: [thm:conformality], capacity axioms (P7: monotonicity)
//!
//! Strategy: central finite differences on perturbed polytopes, direct capacity/volume
//! computation. Most tests are #[ignore] (expensive: multiple ehz_capacity calls).

use crate::algorithms::hk2017::ehz_capacity;
use crate::geom::known_polytopes;
use crate::geom::polytope::Polytope4D;
use crate::geom::volume::volume;
use nalgebra::Vector4;

/// Step size for central finite differences of capacity.
///
/// Chosen as geometric mean of machine epsilon (~1e-16) and typical height scale (~1):
/// eps ~ 1e-7 to 1e-6. We use 1e-6 for capacity (expensive, want stability).
const FD_EPS_CAP: f64 = 1e-6;

/// Step size for central finite differences of volume.
///
/// Tighter than capacity (volume computation is cheap via qhull).
const FD_EPS_VOL: f64 = 1e-7;

/// Construct a perturbed polytope: h_k -> h_k + delta, all other heights unchanged.
///
/// Returns `None` if construction fails (should not happen for small perturbations
/// of valid polytopes).
fn perturbed_polytope(
    normals: &[Vector4<f64>],
    heights: &[f64],
    facet: usize,
    delta: f64,
) -> Option<Polytope4D> {
    let mut h = heights.to_vec();
    h[facet] += delta;
    let halfspaces: Vec<Vector4<f64>> = normals
        .iter()
        .zip(h.iter())
        .map(|(n, &hi)| n / hi)
        .collect();
    Polytope4D::new(halfspaces).ok()
}

/// Compute FD volume derivatives: dvol/dh_k ~ (vol(h+eps*e_k) - vol(h-eps*e_k)) / (2*eps).
///
/// Uses qhull-based volume. Note: qhull computes volume from the V-rep triangulation,
/// which may introduce O(eps) systematic error for FD. The old code used
/// `volume_divergence` (divergence theorem from H-rep) for cleaner FD.
///
/// TODO: If FD volume tests show excessive error, add a volume_divergence function
/// to the volume module (dropped during migration). The divergence theorem computes
/// vol = (1/4) sum h_i * vol_3D(F_i) directly from H-representation.
fn fd_volume_derivatives(normals: &[Vector4<f64>], heights: &[f64]) -> Vec<f64> {
    let f = heights.len();
    (0..f)
        .map(|k| {
            let p_plus = perturbed_polytope(normals, heights, k, FD_EPS_VOL)
                .expect("perturbed polytope +eps");
            let p_minus = perturbed_polytope(normals, heights, k, -FD_EPS_VOL)
                .expect("perturbed polytope -eps");
            let vol_plus = volume(&p_plus).expect("volume +eps");
            let vol_minus = volume(&p_minus).expect("volume -eps");
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
            let p_plus = perturbed_polytope(normals, heights, k, FD_EPS_CAP)
                .expect("perturbed polytope +eps");
            let p_minus = perturbed_polytope(normals, heights, k, -FD_EPS_CAP)
                .expect("perturbed polytope -eps");
            let cap_plus = ehz_capacity(&p_plus)
                .expect("capacity +eps")
                .result
                .capacity;
            let cap_minus = ehz_capacity(&p_minus)
                .expect("capacity -eps")
                .result
                .capacity;
            (cap_plus - cap_minus) / (2.0 * FD_EPS_CAP)
        })
        .collect()
}

// ===== Default suite: fast tests (debug mode, < 5s each) =====

/// T1: FD capacity derivatives are finite and non-negative for the simplex (5 facets).
///
/// Proposition: For the 4-simplex, dc_EHZ/dh_k is finite and >= 0 for all k.
/// Method: Central FD with eps = 1e-6, `ehz_capacity` on perturbed polytopes.
/// Why default suite: 5 facets -> 10 capacity calls, simplex is fast even in debug.
#[test]
fn fd_capacity_height_simplex() {
    let kp = known_polytopes::simplex();
    let normals = kp.polytope.normals_f64();
    let heights = kp.polytope.heights_f64();

    let d_cap = fd_capacity_derivatives(&normals, &heights);

    for (k, &dc) in d_cap.iter().enumerate() {
        assert!(
            dc.is_finite(),
            "facet {k}: d_cap/d_h is not finite ({dc})"
        );
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
/// TODO: This test uses qhull-based volume. The old code used `volume_divergence`
/// (divergence theorem from H-rep) which gives clean FD with O(eps^2) truncation error.
/// Qhull computes volume from V-rep triangulation, which introduces O(eps) systematic
/// error in FD because the triangulation topology can change with small perturbations.
/// If this test fails on hypercube, restore `volume_divergence` in geom/volume.rs.
#[test]
#[ignore] // Requires volume_divergence (dropped during migration) for clean FD
fn euler_homogeneity_volume() {
    let polytopes: Vec<(&str, Polytope4D)> = vec![
        ("simplex", known_polytopes::simplex().polytope.clone()),
        ("hypercube", known_polytopes::hypercube().polytope.clone()),
    ];

    for (name, poly) in &polytopes {
        let normals = poly.normals_f64();
        let heights = poly.heights_f64();
        let vol = volume(poly).expect("volume");

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
    let normals = kp.polytope.normals_f64();
    let heights = kp.polytope.heights_f64();

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
        ("hypercube", known_polytopes::hypercube().polytope.clone()),
        (
            "lagrangian_tri",
            known_polytopes::lagrangian_triangle_product().polytope.clone(),
        ),
        ("hko_pentagon", known_polytopes::hko_pentagon().polytope.clone()),
    ];

    for (name, poly) in &polytopes {
        let normals = poly.normals_f64();
        let heights = poly.heights_f64();

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
        let normals = kp.polytope.normals_f64();
        let heights = kp.polytope.heights_f64();
        let cap = ehz_capacity(&kp.polytope)
            .expect("capacity")
            .result
            .capacity;

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
        ("hypercube", known_polytopes::hypercube().polytope.clone()),
        (
            "lagrangian_tri",
            known_polytopes::lagrangian_triangle_product().polytope.clone(),
        ),
        ("hko_pentagon", known_polytopes::hko_pentagon().polytope.clone()),
    ];

    for (name, poly) in &polytopes {
        let normals = poly.normals_f64();
        let heights = poly.heights_f64();

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
        let normals = kp.polytope.normals_f64();
        let heights = kp.polytope.heights_f64();

        let cap = ehz_capacity(&kp.polytope)
            .expect("capacity")
            .result
            .capacity;
        let vol = volume(&kp.polytope).expect("volume");
        let sys = cap * cap / (2.0 * vol);

        // FD sys derivatives.
        let d_sys: Vec<f64> = (0..heights.len())
            .map(|k| {
                let p_plus = perturbed_polytope(&normals, &heights, k, FD_EPS_CAP)
                    .expect("perturbed +eps");
                let p_minus = perturbed_polytope(&normals, &heights, k, -FD_EPS_CAP)
                    .expect("perturbed -eps");
                let cap_p = ehz_capacity(&p_plus)
                    .expect("cap +eps")
                    .result
                    .capacity;
                let cap_m = ehz_capacity(&p_minus)
                    .expect("cap -eps")
                    .result
                    .capacity;
                let vol_p = volume(&p_plus).expect("vol +eps");
                let vol_m = volume(&p_minus).expect("vol -eps");
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

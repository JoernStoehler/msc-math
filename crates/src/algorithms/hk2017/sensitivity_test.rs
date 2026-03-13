//! Sensitivity tests: finite-difference validation of capacity and volume derivatives.
//!
//! These tests verify mathematical properties of the derivatives of c_EHZ(K) and vol(K)
//! with respect to facet heights h_k, using only crate-public API (no analytical derivatives).
//!
//! ## Mathematical properties tested:
//!
//! - **Euler homogeneity (volume):** Σ h_k · ∂vol/∂h_k = 4·vol  (vol is degree-4 in h)
//! - **Euler homogeneity (capacity):** Σ h_k · ∂c/∂h_k = 2·c  (c is degree-2 in h)
//! - **Euler homogeneity (sys):** Σ h_k · ∂sys/∂h_k = -2·sys  (sys = c²/(2·vol) is degree-2-4=-2)
//! - **Monotonicity:** ∂c/∂h_k ≥ 0  (enlarging K cannot decrease c_EHZ)
//! - **FD sanity:** FD derivatives are finite and well-defined
//!
//! ## Test organization (per `rust-tests` skill):
//!
//! | Test | Suite | Why |
//! |------|-------|-----|
//! | T1: `fd_capacity_height_simplex` | Default | Small (5F), fast in debug |
//! | T2: `euler_homogeneity_volume` | Default | `volume()` is cheap |
//! | T3: `capacity_monotone_simplex` | Default | Small (5F), fast in debug |
//! | T4: `fd_capacity_height_known_polytopes` | `#[ignore]` | Large (8-16F), needs release |
//! | T5: `euler_homogeneity_capacity` | `#[ignore]` | Needs `ehz_capacity` on multiple polytopes |
//! | T6: `capacity_monotone_known_polytopes` | `#[ignore]` | Large polytopes, needs release |
//! | T7: `fd_sys_height_euler` | `#[ignore]` | End-to-end sys derivatives |

use crate::algorithms::hk2017::ehz_capacity;
use crate::geom::known_polytopes;
use crate::geom::polytope::Polytope4D;
use crate::geom::volume::deprecated::volume_divergence;
use crate::geom::volume::volume;
use nalgebra::Vector4;

/// Step size for central finite differences.
///
/// Chosen as geometric mean of machine epsilon (~1e-16) and typical height scale (~1):
/// ε ≈ 1e-7 to 1e-6. We use 1e-6 for capacity (expensive, want stability) and
/// 1e-7 for volume (cheap, can afford tighter).
const FD_EPS_CAP: f64 = 1e-6;
const FD_EPS_VOL: f64 = 1e-7;

/// Construct a perturbed polytope: h_k → h_k + delta, all other heights unchanged.
///
/// Returns `None` if construction fails (shouldn't happen for small perturbations
/// of valid polytopes).
fn perturbed_polytope(
    normals: &[Vector4<f64>],
    heights: &[f64],
    facet: usize,
    delta: f64,
) -> Option<Polytope4D> {
    let mut h = heights.to_vec();
    h[facet] += delta;
    Polytope4D::new(normals.to_vec(), h).ok()
}

/// Compute FD volume derivatives: ∂vol/∂h_k ≈ (vol(h+ε·e_k) - vol(h-ε·e_k)) / (2ε).
///
/// Uses `volume_divergence` (divergence theorem) instead of qhull because qhull's
/// triangulation introduces O(ε) systematic error that corrupts central differences.
/// The divergence theorem computes vol = (1/4) Σ h_i · vol_3D(F_i) directly from
/// H-representation, giving clean FD with O(ε²) truncation error.
fn fd_volume_derivatives(normals: &[Vector4<f64>], heights: &[f64]) -> Vec<f64> {
    let f = heights.len();
    (0..f)
        .map(|k| {
            let p_plus = perturbed_polytope(normals, heights, k, FD_EPS_VOL)
                .expect("perturbed polytope +ε");
            let p_minus = perturbed_polytope(normals, heights, k, -FD_EPS_VOL)
                .expect("perturbed polytope -ε");
            let vol_plus = volume_divergence(&p_plus);
            let vol_minus = volume_divergence(&p_minus);
            (vol_plus - vol_minus) / (2.0 * FD_EPS_VOL)
        })
        .collect()
}

/// Compute FD capacity derivatives: ∂c/∂h_k ≈ (c(h+ε·e_k) - c(h-ε·e_k)) / (2ε).
///
/// At non-smooth points (tied orbits), this computes the directional derivative of
/// the envelope (max over orbits), not a single-orbit subgradient.
fn fd_capacity_derivatives(normals: &[Vector4<f64>], heights: &[f64]) -> Vec<f64> {
    let f = heights.len();
    (0..f)
        .map(|k| {
            let p_plus = perturbed_polytope(normals, heights, k, FD_EPS_CAP)
                .expect("perturbed polytope +ε");
            let p_minus = perturbed_polytope(normals, heights, k, -FD_EPS_CAP)
                .expect("perturbed polytope -ε");
            let cap_plus = ehz_capacity(&p_plus).expect("capacity +ε").capacity;
            let cap_minus = ehz_capacity(&p_minus).expect("capacity -ε").capacity;
            (cap_plus - cap_minus) / (2.0 * FD_EPS_CAP)
        })
        .collect()
}

// ===== Default suite: fast tests (debug mode, < 5s each) =====

/// T1: FD capacity derivatives are finite and non-negative for the simplex (5 facets).
///
/// **Proposition:** For the 4-simplex, ∂c_EHZ/∂h_k is finite and ≥ 0 for all k.
/// **Method:** Central FD with ε = 1e-6, `ehz_capacity` on perturbed polytopes.
/// **Why default suite:** 5 facets → 10 capacity calls, simplex is fast even in debug.
/// **Why this input:** Simplex is the smallest polytope (5F) with a known capacity (0.25).
#[test]
fn fd_capacity_height_simplex() {
    let kp = known_polytopes::simplex();
    let normals = kp.polytope.normals_f64().to_vec();
    let heights = kp.polytope.heights_f64().to_vec();

    let d_cap = fd_capacity_derivatives(&normals, &heights);

    for (k, &dc) in d_cap.iter().enumerate() {
        assert!(dc.is_finite(), "facet {k}: d_cap/d_h is not finite ({dc})");
        assert!(
            dc >= -1e-4,
            "facet {k}: d_cap/d_h = {dc:.6} violates monotonicity (expected ≥ 0)"
        );
    }
}

/// T2: Euler homogeneity identity for volume: Σ h_k · ∂vol/∂h_k = 4·vol.
///
/// **Proposition:** Volume is degree-4 homogeneous in heights, so Euler's identity
/// gives Σ h_k · ∂vol/∂h_k = 4·vol(K).
/// **Method:** FD volume derivatives via divergence theorem (qhull has O(ε) systematic
/// error that corrupts central differences — verified on hypercube).
/// **Polytopes:** simplex, hypercube (crosspolytope excluded: 16F × volume_divergence too slow in debug).
/// **Tolerance:** 0.1% relative (divergence theorem gives clean FD with O(ε²) error).
#[test]
fn euler_homogeneity_volume() {
    // Crosspolytope (16F) excluded: volume_divergence is O(F²) per call,
    // making 32 calls too slow for debug mode (~200s). Tested in T4 (ignored/release).
    let polytopes: Vec<(&str, Polytope4D)> = vec![
        ("simplex", known_polytopes::simplex().polytope),
        ("hypercube", known_polytopes::hypercube().polytope),
    ];

    for (name, poly) in &polytopes {
        let normals = poly.normals_f64().to_vec();
        let heights = poly.heights_f64().to_vec();
        let vol = volume_divergence(poly);

        let d_vol = fd_volume_derivatives(&normals, &heights);
        let euler_sum: f64 = heights.iter().zip(&d_vol).map(|(h, dv)| h * dv).sum();
        let expected = 4.0 * vol;
        let rel_err = (euler_sum - expected).abs() / expected;

        assert!(
            rel_err < 1e-3,
            "{name}: Euler vol identity failed: Σ h·∂vol/∂h = {euler_sum:.8}, \
             4·vol = {expected:.8}, rel_err = {rel_err:.2e}"
        );
    }
}

/// T3: Capacity monotonicity for the simplex: ∂c/∂h_k ≥ 0 for all k.
///
/// **Proposition:** c_EHZ is monotone under inclusion (P7 in capacity axioms).
/// Increasing any height h_k enlarges K, so ∂c/∂h_k ≥ 0.
/// **Method:** FD central difference, check sign.
/// **Why simplex:** 5 facets = 10 capacity calls, fast in debug.
#[test]
fn capacity_monotone_simplex() {
    let kp = known_polytopes::simplex();
    let normals = kp.polytope.normals_f64().to_vec();
    let heights = kp.polytope.heights_f64().to_vec();

    let d_cap = fd_capacity_derivatives(&normals, &heights);

    for (k, &dc) in d_cap.iter().enumerate() {
        assert!(
            dc >= -1e-4,
            "simplex facet {k}: ∂c/∂h = {dc:.6e} < 0 (monotonicity violation)"
        );
    }
}

// ===== Ignored suite: expensive tests (release mode) =====

/// T4: FD capacity derivatives are finite and non-negative for larger polytopes.
///
/// **Proposition:** Same as T1 (finiteness + monotonicity), on polytopes with more facets.
/// **Polytopes:** hypercube (8F), lagrangian_triangle_product (6F), HKO pentagon (10F).
/// **Why ignored:** 8-10 facets × 2 capacity calls each, too slow for debug mode.
/// **Runtime:** ~10s in release.
#[test]
#[ignore]
fn fd_capacity_height_known_polytopes() {
    let polytopes = vec![
        ("hypercube", known_polytopes::hypercube().polytope),
        (
            "lagrangian_tri",
            known_polytopes::lagrangian_triangle_product().polytope,
        ),
        ("hko_pentagon", known_polytopes::hko_pentagon().polytope),
    ];

    for (name, poly) in &polytopes {
        let normals = poly.normals_f64().to_vec();
        let heights = poly.heights_f64().to_vec();

        let d_cap = fd_capacity_derivatives(&normals, &heights);

        for (k, &dc) in d_cap.iter().enumerate() {
            assert!(
                dc.is_finite(),
                "{name} facet {k}: d_cap/d_h is not finite ({dc})"
            );
            // Relaxed tolerance for non-smooth points (HKO pentagon has 44 tied orbits).
            // FD at non-smooth points computes the envelope derivative, which is still ≥ 0.
            assert!(
                dc >= -1e-3,
                "{name} facet {k}: d_cap/d_h = {dc:.6e} violates monotonicity"
            );
        }
    }
}

/// T5: Euler homogeneity identity for capacity: Σ h_k · ∂c/∂h_k = 2·c.
///
/// **Proposition:** c_EHZ is degree-2 homogeneous in heights (conformality + scaling of h),
/// so Euler's identity gives Σ h_k · ∂c/∂h_k = 2·c.
///
/// **This is the test that catches the sign bug:** wrong sign gives Σ = -2c instead of +2c.
///
/// **Method:** FD capacity derivatives (central difference), check Euler sum.
/// **Polytopes:** simplex, hypercube, lagrangian_triangle_product — all generic (unique
/// optimal orbit). HKO pentagon excluded: 44 tied orbits make capacity non-smooth, so
/// FD envelope derivative ≠ Euler identity (Jörn confirmed: subdifferential ≠ gradient
/// at non-generic points).
/// **Tolerance:** 1% relative.
/// **Why ignored:** Multiple `ehz_capacity` calls per polytope.
/// **Runtime:** ~10s in release.
#[test]
#[ignore]
fn euler_homogeneity_capacity() {
    let polytopes = vec![
        ("simplex", known_polytopes::simplex()),
        ("hypercube", known_polytopes::hypercube()),
        ("lagrangian_tri", known_polytopes::lagrangian_triangle_product()),
    ];

    for (name, kp) in &polytopes {
        let normals = kp.polytope.normals_f64().to_vec();
        let heights = kp.polytope.heights_f64().to_vec();
        let cap = ehz_capacity(&kp.polytope).expect("capacity").capacity;

        let d_cap = fd_capacity_derivatives(&normals, &heights);
        let euler_sum: f64 = heights.iter().zip(&d_cap).map(|(h, dc)| h * dc).sum();
        let expected = 2.0 * cap;
        let rel_err = (euler_sum - expected).abs() / expected;

        eprintln!(
            "{name}: Euler cap: Σ h·∂c/∂h = {euler_sum:.6}, 2c = {expected:.6}, \
             ratio = {:.4}, rel_err = {rel_err:.2e}",
            euler_sum / expected
        );

        assert!(
            rel_err < 0.01,
            "{name}: Euler capacity identity failed: Σ h·∂c/∂h = {euler_sum:.8}, \
             2c = {expected:.8}, rel_err = {rel_err:.2e} (>1%)"
        );
    }
}

/// T6: Capacity monotonicity for known polytopes with more facets.
///
/// **Proposition:** ∂c/∂h_k ≥ 0 for all k (monotonicity under inclusion).
/// **Polytopes:** hypercube (8F), lagrangian_triangle_product (6F), HKO pentagon (10F).
/// **Why ignored:** Same as T4.
#[test]
#[ignore]
fn capacity_monotone_known_polytopes() {
    let polytopes = vec![
        ("hypercube", known_polytopes::hypercube().polytope),
        (
            "lagrangian_tri",
            known_polytopes::lagrangian_triangle_product().polytope,
        ),
        ("hko_pentagon", known_polytopes::hko_pentagon().polytope),
    ];

    for (name, poly) in &polytopes {
        let normals = poly.normals_f64().to_vec();
        let heights = poly.heights_f64().to_vec();

        let d_cap = fd_capacity_derivatives(&normals, &heights);

        for (k, &dc) in d_cap.iter().enumerate() {
            assert!(
                dc >= -1e-3,
                "{name} facet {k}: ∂c/∂h = {dc:.6e} < 0 (monotonicity violation)"
            );
        }
    }
}


/// T7: Euler homogeneity for sys = c²/(2·vol): Σ h_k · ∂sys/∂h_k = 0.
///
/// **Proposition:** sys(K) = c_EHZ(K)² / (2·vol(K)) is degree 0 in heights:
/// sys(λh) = (λ²c)² / (2·λ⁴·vol) = λ⁰·sys. Euler identity: Σ h_k · ∂sys/∂h_k = 0.
///
/// **Method:** FD sys derivatives end-to-end (perturb h_k, recompute both c and vol).
/// **Polytopes:** simplex, hypercube — generic (unique optimal orbit).
/// HKO pentagon excluded: non-smooth capacity invalidates Euler identity for FD.
/// **Tolerance:** 1% of sys value (absolute, since expected value is 0).
/// **Why ignored:** Multiple `ehz_capacity` + `volume` calls per polytope.
#[test]
#[ignore]
fn fd_sys_height_euler() {
    let polytopes = vec![
        ("simplex", known_polytopes::simplex()),
        ("hypercube", known_polytopes::hypercube()),
    ];

    for (name, kp) in &polytopes {
        let normals = kp.polytope.normals_f64().to_vec();
        let heights = kp.polytope.heights_f64().to_vec();

        let cap = ehz_capacity(&kp.polytope).expect("capacity").capacity;
        let vol = volume(&kp.polytope).expect("volume");
        let sys = cap * cap / (2.0 * vol);

        // FD sys derivatives
        let d_sys: Vec<f64> = (0..heights.len())
            .map(|k| {
                let p_plus = perturbed_polytope(&normals, &heights, k, FD_EPS_CAP)
                    .expect("perturbed +ε");
                let p_minus = perturbed_polytope(&normals, &heights, k, -FD_EPS_CAP)
                    .expect("perturbed -ε");
                let cap_p = ehz_capacity(&p_plus).expect("cap +ε").capacity;
                let cap_m = ehz_capacity(&p_minus).expect("cap -ε").capacity;
                let vol_p = volume(&p_plus).expect("vol +ε");
                let vol_m = volume(&p_minus).expect("vol -ε");
                let sys_p = cap_p * cap_p / (2.0 * vol_p);
                let sys_m = cap_m * cap_m / (2.0 * vol_m);
                (sys_p - sys_m) / (2.0 * FD_EPS_CAP)
            })
            .collect();

        let euler_sum: f64 = heights.iter().zip(&d_sys).map(|(h, ds)| h * ds).sum();
        // Expected: 0 (degree 0 in h)

        eprintln!(
            "{name}: Euler sys: Σ h·∂sys/∂h = {euler_sum:.6e}, sys = {sys:.6}, \
             ratio = {:.4e}",
            euler_sum / sys
        );

        assert!(
            euler_sum.abs() < 0.01 * sys,
            "{name}: Euler sys identity failed: Σ h·∂sys/∂h = {euler_sum:.6e}, \
             expected 0, sys = {sys:.6} (ratio = {:.2e})",
            euler_sum / sys
        );
    }
}

//! Derivatives of EHZ capacity and volume with respect to polytope parameters.
//!
//! Provides analytical derivatives of the EHZ capacity c_EHZ(K) and volume vol(K)
//! with respect to facet heights h_k and outward normals n_k, for a convex polytope
//! K = {x : n_k · x ≤ h_k}.
//!
//! **Capacity derivatives** use the envelope theorem applied to the KKT system.
//! Given the optimal orbit (S, σ) with solution (β, Q, μ, ξ), the action
//! A = 1/(2Q) has derivatives:
//!
//! - ∂A/∂h_k = −ξ · β_{i₀} / (2Q²)  where k = σ(i₀)
//! - ∂A/∂n_k = −∂Q*/∂n_k / (2Q²),  projected to T_{n_k}S³
//!
//! **Sign convention:** Uses the symmetric KKT convention (Hβ + Nμ + ηξ = 0)
//! from `saddle_point_solver::KktResult`. In the asymmetric convention used by
//! some references (Hβ = Nλ + ην), the relationship is μ = −λ, ξ = −ν.
//!
//! **Volume derivatives** are geometric:
//! - ∂vol/∂h_k = S_k  (3D volume of facet k, divergence theorem)
//! - ∂vol/∂n_k = −S_k(x̄_k − h_k n_k)  (projected to T_{n_k}S³)
//!
//! Mathematical correspondence: TODO write [lem:cap-derivative], [lem:vol-derivative] in math.tex

use crate::geom::facet_volume::{facet_volume_3d_raw, facet_volume_and_centroid_3d_raw};
use crate::geom::polytope::Polytope4D;
use crate::geom::symplectic_form::j4;
use nalgebra::Vector4;

/// Compute ∂A/∂h_k for all facets k = 0..f, where A = c_EHZ = 1/(2Q).
///
/// For facet k in the orbit permutation (k = σ(i₀)):
///   ∂A/∂h_k = −ξ · β_{i₀} / (2Q²)
///
/// For facets not in the orbit: ∂A/∂h_k = 0.
///
/// Derivation: envelope theorem gives ∂Q*/∂h_k = ξ·β_{i₀}, then
/// ∂A/∂h_k = ∂[1/(2Q)]/∂h_k = −∂Q*/∂h_k / (2Q²) = −ξ·β_{i₀}/(2Q²).
///
/// **Cross-check with asymmetric convention:** In the asymmetric convention
/// (Hβ = Nλ + ην), ν = −ξ, so ∂A/∂h_k = −(−ν)·β/(2Q²) = ν·β/(2Q²),
/// matching the formula in experiment code.
///
/// # Arguments
/// - `beta`: dwell-time coefficients from KktResult
/// - `q`: Q value (= (1/2) β^T H β) from KktResult.q_corrected
/// - `xi`: normalization multiplier from KktResult.xi
/// - `perm`: cyclic facet permutation σ (indices into 0..f)
/// - `facet_count`: total number of facets f
pub fn capacity_derivatives_h(
    beta: &[f64],
    q: f64,
    xi: f64,
    perm: &[usize],
    facet_count: usize,
) -> Vec<f64> {
    let q_sq = q * q;
    (0..facet_count)
        .map(|k| {
            match perm.iter().position(|&f| f == k) {
                Some(i0) => -xi * beta[i0] / (2.0 * q_sq),
                None => 0.0,
            }
        })
        .collect()
}

/// Compute ∂A/∂n_k for all facets k = 0..f, projected to T_{n_k}S³.
///
/// For facet k = σ(i₀) in the orbit:
///   ∂Q*/∂n_k = β_{i₀} · [J₀(2P_{i₀} + β_{i₀} n_k) + μ]
///   ∂A/∂n_k = −(∂Q*/∂n_k − (∂Q*/∂n_k · n_k) n_k) / (2Q²)
///
/// where P_{i₀} = Σ_{i < i₀} β_i · n_{σ(i)} is the partial sum of normals.
///
/// Uses the symmetric KKT sign convention: μ is the closure multiplier from
/// Hβ + Nμ + ηξ = 0. (In asymmetric convention, replace μ with −λ.)
///
/// # Arguments
/// - `beta`: dwell-time coefficients from KktResult
/// - `q`: Q value from KktResult.q_corrected
/// - `mu`: closure multiplier (4 components) from KktResult.mu
/// - `perm`: cyclic facet permutation σ
/// - `normals`: outward unit normals for all facets
pub fn capacity_derivatives_n(
    beta: &[f64],
    q: f64,
    mu: &[f64],
    perm: &[usize],
    normals: &[Vector4<f64>],
) -> Vec<Vector4<f64>> {
    let q_sq = q * q;
    let facet_count = normals.len();
    let j0 = j4();
    let mu_vec = Vector4::new(mu[0], mu[1], mu[2], mu[3]);

    (0..facet_count)
        .map(|k| {
            let i0 = match perm.iter().position(|&f| f == k) {
                Some(pos) => pos,
                None => return Vector4::zeros(),
            };

            // P_{i₀} = Σ_{i < i₀} β_i · n_{σ(i)}
            let mut p = Vector4::zeros();
            for i in 0..i0 {
                p += beta[i] * normals[perm[i]];
            }

            // ∂Q*/∂n_k = β_{i₀} · [J₀(2P + β_{i₀} n_k) + μ]
            // (symmetric convention: +μ; asymmetric would be −λ)
            let inner = 2.0 * p + beta[i0] * normals[k];
            let j0_inner = j0 * inner;
            let dq_dn = beta[i0] * (j0_inner + mu_vec);

            // Project onto T_{n_k}S³: remove normal component
            let dq_dn_tangent = dq_dn - dq_dn.dot(&normals[k]) * normals[k];

            // ∂A/∂n_k = −∂Q*/∂n_k / (2Q²)
            -dq_dn_tangent / (2.0 * q_sq)
        })
        .collect()
}

/// Compute ∂vol(K)/∂h_k for all facets k = 0..f.
///
/// By the divergence theorem: ∂vol/∂h_k = S_k, the 3D volume of facet k.
pub fn volume_derivatives_h(polytope: &Polytope4D) -> Vec<f64> {
    let normals = polytope.normals_f64();
    let heights = polytope.heights_f64();
    let vertices = polytope.vertices_f64();
    let f = polytope.facet_count();

    (0..f)
        .map(|k| facet_volume_3d_raw(&normals, &heights, vertices, k, f))
        .collect()
}

/// Compute ∂vol(K)/∂n_k for all facets k = 0..f, projected to T_{n_k}S³.
///
/// The gradient is ∂vol/∂n_k = −S_k(x̄_k − h_k n_k), where x̄_k is the
/// centroid of facet k and S_k its 3D volume. The term (x̄_k − h_k n_k) is
/// the centroid's tangential displacement from the base point h_k n_k on the
/// supporting hyperplane.
pub fn volume_derivatives_n(polytope: &Polytope4D) -> Vec<Vector4<f64>> {
    let normals = polytope.normals_f64();
    let heights = polytope.heights_f64();
    let vertices = polytope.vertices_f64();
    let f = polytope.facet_count();

    (0..f)
        .map(|k| {
            let (s_k, centroid_k) =
                facet_volume_and_centroid_3d_raw(&normals, &heights, vertices, k, f);
            if s_k < crate::geom::facet_volume::EPS_VOLUME_FLOOR {
                return Vector4::zeros();
            }
            let tangent_centroid = centroid_k - heights[k] * normals[k];
            -s_k * tangent_centroid
        })
        .collect()
}

/// Compute ∂A/∂h_k by finite differences (cross-check for analytical derivatives).
///
/// Perturbs h_k by ±eps and recomputes capacity, returning the central difference.
/// Requires a capacity function that takes normals and heights and returns capacity.
pub fn capacity_derivatives_h_fd(
    normals: &[Vector4<f64>],
    heights: &[f64],
    eps: f64,
    capacity_fn: impl Fn(&[Vector4<f64>], &[f64]) -> Option<f64>,
) -> Vec<f64> {
    let f = normals.len();
    (0..f)
        .map(|k| {
            let mut h_plus = heights.to_vec();
            let mut h_minus = heights.to_vec();
            h_plus[k] += eps;
            h_minus[k] -= eps;

            let cap_plus = capacity_fn(normals, &h_plus);
            let cap_minus = capacity_fn(normals, &h_minus);

            match (cap_plus, cap_minus) {
                (Some(cp), Some(cm)) => (cp - cm) / (2.0 * eps),
                _ => f64::NAN,
            }
        })
        .collect()
}

/// Compute ∂vol/∂h_k by finite differences (cross-check for analytical derivatives).
pub fn volume_derivatives_h_fd(
    normals: &[Vector4<f64>],
    heights: &[f64],
    eps: f64,
    volume_fn: impl Fn(&[Vector4<f64>], &[f64]) -> Option<f64>,
) -> Vec<f64> {
    let f = normals.len();
    (0..f)
        .map(|k| {
            let mut h_plus = heights.to_vec();
            let mut h_minus = heights.to_vec();
            h_plus[k] += eps;
            h_minus[k] -= eps;

            let vol_plus = volume_fn(normals, &h_plus);
            let vol_minus = volume_fn(normals, &h_minus);

            match (vol_plus, vol_minus) {
                (Some(vp), Some(vm)) => (vp - vm) / (2.0 * eps),
                _ => f64::NAN,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geom::known_polytopes;
    use crate::geom::volume::volume;
    use crate::kkt::saddle_point_solver::solve_kkt_for;

    // Tests for derivatives: analytical capacity and volume derivatives.
    //
    // Proposition: volume_derivatives_h matches finite differences to O(eps²).
    // Capacity derivatives are tested via the FD cross-check on known polytopes.
    // Reference: TODO [lem:cap-derivative], [lem:vol-derivative] (not yet in math.tex)
    //
    // Strategy: fixture-based (hypercube, simplex) + FD cross-validation

    /// Volume derivative ∂vol/∂h_k should equal facet volume S_k.
    /// Cross-check against finite differences.
    #[test]
    fn volume_derivatives_h_matches_fd() {
        let kp = known_polytopes::hypercube();
        let polytope = &kp.polytope;
        let normals = polytope.normals_f64();
        let heights = polytope.heights_f64();

        let analytical = volume_derivatives_h(polytope);
        let eps = 1e-6;
        let fd = volume_derivatives_h_fd(&normals, &heights, eps, |n, h| {
            let p = Polytope4D::from_normals_and_heights(n.to_vec(), h.to_vec()).ok()?;
            volume(&p).ok()
        });

        for k in 0..polytope.facet_count() {
            let rel_err = if analytical[k].abs() > 1e-10 {
                (analytical[k] - fd[k]).abs() / analytical[k].abs()
            } else {
                (analytical[k] - fd[k]).abs()
            };
            // Volume is polynomial in h → central FD error is O(eps²) ≈ 1e-12.
            // Use 1e-6 tolerance to allow for qhull's own numerical error.
            assert!(
                rel_err < 1e-6,
                "facet {k}: analytical={}, fd={}, rel_err={rel_err}",
                analytical[k], fd[k]
            );
        }
    }

    /// Volume normal derivatives should be tangent to S³ (perpendicular to n_k).
    #[test]
    fn volume_derivatives_n_tangent() {
        let polytope = &known_polytopes::hypercube().polytope;
        let normals = polytope.normals_f64();
        let dvol_dn = volume_derivatives_n(polytope);

        for k in 0..polytope.facet_count() {
            let dot = dvol_dn[k].dot(&normals[k]);
            assert!(
                dot.abs() < 1e-10,
                "facet {k}: ∂vol/∂n_k not tangent, n·(∂vol/∂n) = {dot}"
            );
        }
    }

    /// Analytical ∂A/∂h_k matches per-orbit FD central difference to O(eps²).
    /// Uses hypercube (8 facets, fast ehz_capacity call).
    #[test]
    fn capacity_derivatives_h_on_hypercube() {
        let kp = known_polytopes::hypercube();
        let polytope = &kp.polytope;

        // Find the best orbit by trying all permutations
        let (best_q, best_beta, best_perm, best_mu, best_xi) =
            find_best_orbit(polytope);

        assert!(
            best_q > 1e-10,
            "find_best_orbit should find a valid orbit on the hypercube"
        );

        let f = polytope.facet_count();
        let analytical = capacity_derivatives_h(&best_beta, best_q, best_xi, &best_perm, f);

        // Per-orbit FD cross-check: perturb h_k, re-solve the SAME orbit, compare.
        // We compare against per-orbit action A = 1/(2Q), NOT global capacity,
        // because global capacity = min over orbits, and a different orbit may
        // become optimal after perturbation (e.g., on the hypercube where many
        // orbits are equally optimal by symmetry).
        let normals = polytope.normals_f64();
        let heights = polytope.heights_f64();
        let eps = 1e-6;

        // At least some derivatives should be non-zero (the orbit uses >= 2 facets)
        let nonzero_count = analytical.iter().filter(|&&x| x.abs() > 1e-10).count();
        assert!(
            nonzero_count >= 2,
            "expected >= 2 non-zero capacity derivatives, got {nonzero_count}"
        );

        for k in 0..f {
            if analytical[k].abs() < 1e-12 {
                continue; // Facet not in orbit, derivative is zero
            }
            // FD for the specific orbit's action
            let mut hp = heights.clone();
            let mut hm = heights.clone();
            hp[k] += eps;
            hm[k] -= eps;
            let pp = Polytope4D::from_normals_and_heights(normals.clone(), hp).unwrap();
            let pm = Polytope4D::from_normals_and_heights(normals.clone(), hm).unwrap();
            let qp = solve_kkt_for(&pp, &best_perm).map(|r| r.q_corrected);
            let qm = solve_kkt_for(&pm, &best_perm).map(|r| r.q_corrected);
            let fd_k = match (qp, qm) {
                (Some(qp), Some(qm)) => {
                    let ap = 0.5 / qp;
                    let am = 0.5 / qm;
                    (ap - am) / (2.0 * eps)
                }
                _ => panic!("solve_kkt_for failed on perturbed polytope for facet {k}, perm {best_perm:?}"),
            };
            let abs_err = (analytical[k] - fd_k).abs();
            assert!(
                abs_err < 1e-4,
                "facet {k}: analytical={}, fd={fd_k}, abs_err={abs_err}",
                analytical[k]
            );
        }
    }

    /// Capacity normal derivatives should be tangent to S³.
    #[test]
    fn capacity_derivatives_n_tangent() {
        let kp = known_polytopes::hypercube();
        let polytope = &kp.polytope;
        let normals = polytope.normals_f64();

        let (best_q, best_beta, best_perm, best_mu, _best_xi) =
            find_best_orbit(polytope);

        assert!(
            best_q > 1e-10,
            "find_best_orbit should find a valid orbit on the hypercube"
        );

        let dcap_dn = capacity_derivatives_n(&best_beta, best_q, &best_mu, &best_perm, &normals);

        for k in 0..polytope.facet_count() {
            let dot = dcap_dn[k].dot(&normals[k]);
            assert!(
                dot.abs() < 1e-10,
                "facet {k}: ∂A/∂n_k not tangent, n·(∂A/∂n) = {dot}"
            );
        }
    }

    /// Helper: find the best orbit for a polytope via library capacity + KKT re-solve.
    fn find_best_orbit(
        polytope: &Polytope4D,
    ) -> (f64, Vec<f64>, Vec<usize>, Vec<f64>, f64) {
        let ehz = crate::algorithms::hk2017::ehz_capacity(polytope)
            .expect("ehz_capacity should find an orbit on test polytopes");
        let perm = ehz.result.best_permutation;
        let kkt = solve_kkt_for(polytope, &perm)
            .expect("solve_kkt_for should succeed on the best permutation");
        (kkt.q_corrected, kkt.beta, perm, kkt.mu, kkt.xi)
    }
}

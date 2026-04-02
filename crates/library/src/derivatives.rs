//! Derivatives of EHZ capacity and volume with respect to dual vertices a_k.
//!
//! For a polytope K = {x : a_k · x ≤ 1}, provides analytical derivatives of the
//! EHZ capacity c_EHZ(K) and volume vol(K) with respect to dual vertices a_k.
//!
//! **Capacity derivatives** use the envelope theorem applied to the KKT system.
//! Given the optimal orbit (S, σ) with solution (β, Q, μ, ξ):
//!
//! - ∂Q*/∂a_k = β_{i₀} [J₀(2P_{i₀} + β_{i₀} a_k) + μ]   where k = σ(i₀)
//! - ∂c/∂a_k = −∂Q*/∂a_k / (2Q²)
//!
//! **Volume derivatives** use the chain rule through h = 1/|a| and n = a/|a|:
//! - ∂vol/∂a_k = −(S_k / |a_k|³) a_k + (S_k / |a_k|) (x̄_k − (1/|a_k|²) a_k · x̄_k · a_k)   [... simplified below]
//!
//! Mathematical correspondence: [lem:cap-derivative], [lem:vol-derivative] in experiments/sys-optimization/math.tex

use crate::geom::facet_volume::facet_volume_and_centroid_3d_raw;
use crate::geom::polytope::Polytope4D;
use crate::geom::symplectic_form::j4;
use nalgebra::Vector4;

/// Compute ∂c/∂a_k for all facets k = 0..f, where c = 1/(2Q).
///
/// For facet k = σ(i₀) in the orbit:
///   ∂Q*/∂a_k = β_{i₀} · [J₀(2P_{i₀} + β_{i₀} a_k) + μ]
///   ∂c/∂a_k = −∂Q*/∂a_k / (2Q²)
///
/// where P_{i₀} = Σ_{j < i₀} β_j · a_{σ(j)} is the partial sum of dual vertices.
///
/// For facets not in the orbit: ∂c/∂a_k = 0.
///
/// # Arguments
/// - `beta`: dwell-time coefficients from KktResult
/// - `q`: Q value from KktResult.q_corrected
/// - `mu`: closure multiplier (4 components) from KktResult.mu
/// - `perm`: cyclic facet permutation σ
/// - `dual_vertices`: dual vertices a_i for all facets
pub fn capacity_derivatives_a(
    beta: &[f64],
    q: f64,
    mu: &[f64],
    perm: &[usize],
    dual_vertices: &[Vector4<f64>],
) -> Vec<Vector4<f64>> {
    let q_sq = q * q;
    let facet_count = dual_vertices.len();
    let j0 = j4();
    let mu_vec = Vector4::new(mu[0], mu[1], mu[2], mu[3]);

    (0..facet_count)
        .map(|k| {
            let i0 = match perm.iter().position(|&f| f == k) {
                Some(pos) => pos,
                None => return Vector4::zeros(),
            };

            // P_{i₀} = Σ_{j < i₀} β_j · a_{σ(j)}
            let mut p = Vector4::zeros();
            for i in 0..i0 {
                p += beta[i] * dual_vertices[perm[i]];
            }

            // ∂Q*/∂a_k = β_{i₀} · [J₀(2P + β_{i₀} a_k) + μ]
            let inner = 2.0 * p + beta[i0] * dual_vertices[k];
            let j0_inner = j0 * inner;
            let dq_da = beta[i0] * (j0_inner + mu_vec);

            // ∂c/∂a_k = −∂Q*/∂a_k / (2Q²)
            -dq_da / (2.0 * q_sq)
        })
        .collect()
}

/// Compute ∂vol(K)/∂a_k for all facets k = 0..f.
///
/// Uses the chain rule through h_k = 1/|a_k| and n_k = a_k/|a_k|:
///   ∂vol/∂a_k = (∂vol/∂h_k)(∂h_k/∂a_k) + (∂vol/∂n_k)^T (∂n_k/∂a_k)
///
/// where:
///   ∂vol/∂h_k = S_k (facet volume, divergence theorem)
///   ∂vol/∂n_k = −S_k(x̄_k − h_k n_k) (tangent centroid)
///   ∂h_k/∂a_k = −a_k / |a_k|³
///   ∂n_k/∂a_k = (I − n_k n_k^T) / |a_k|
pub fn volume_derivatives_a(polytope: &Polytope4D) -> Vec<Vector4<f64>> {
    let duals = polytope.dual_vertices_f64();
    let vertices = polytope.vertices_f64();
    let f = polytope.facet_count();

    (0..f)
        .map(|k| {
            let a = &duals[k];
            let a_norm = a.norm();
            let n = a / a_norm;
            let h = 1.0 / a_norm;

            let (s_k, centroid_k) =
                facet_volume_and_centroid_3d_raw(duals, vertices, k);
            if s_k < crate::geom::facet_volume::EPS_VOLUME_FLOOR {
                return Vector4::zeros();
            }

            // ∂vol/∂h_k = S_k
            // ∂h_k/∂a_k = −a / |a|³
            let dvol_dh = s_k;
            let dh_da = -a / (a_norm * a_norm * a_norm);

            // ∂vol/∂n_k = −S_k(x̄_k − h n_k)  (tangent component, already ⊥ n_k)
            let tangent_centroid = centroid_k - h * n;
            let dvol_dn = -s_k * tangent_centroid;

            // ∂n_k/∂a_k = (I − n n^T) / |a|
            // So (∂vol/∂n_k)^T (∂n_k/∂a_k) = (I − n n^T) dvol_dn / |a|
            // Since dvol_dn ⊥ n, the projection is identity: (I − n n^T) dvol_dn = dvol_dn
            let dn_contribution = dvol_dn / a_norm;

            dvol_dh * dh_da + dn_contribution
        })
        .collect()
}

/// Compute ∂c/∂a_k by finite differences (cross-check for analytical derivatives).
///
/// Perturbs a_k component-wise by ±eps and recomputes capacity, returning the
/// central difference gradient vector for each facet.
pub fn capacity_derivatives_a_fd(
    dual_vertices: &[Vector4<f64>],
    eps: f64,
    capacity_fn: impl Fn(&[Vector4<f64>]) -> Option<f64>,
) -> Vec<Vector4<f64>> {
    let f = dual_vertices.len();
    (0..f)
        .map(|k| {
            let mut grad = Vector4::zeros();
            for d in 0..4 {
                let mut a_plus = dual_vertices.to_vec();
                let mut a_minus = dual_vertices.to_vec();
                a_plus[k][d] += eps;
                a_minus[k][d] -= eps;

                let cap_plus = capacity_fn(&a_plus);
                let cap_minus = capacity_fn(&a_minus);

                grad[d] = match (cap_plus, cap_minus) {
                    (Some(cp), Some(cm)) => (cp - cm) / (2.0 * eps),
                    _ => f64::NAN,
                };
            }
            grad
        })
        .collect()
}

/// Compute ∂vol/∂a_k by finite differences (cross-check for analytical derivatives).
pub fn volume_derivatives_a_fd(
    dual_vertices: &[Vector4<f64>],
    eps: f64,
    volume_fn: impl Fn(&[Vector4<f64>]) -> Option<f64>,
) -> Vec<Vector4<f64>> {
    let f = dual_vertices.len();
    (0..f)
        .map(|k| {
            let mut grad = Vector4::zeros();
            for d in 0..4 {
                let mut a_plus = dual_vertices.to_vec();
                let mut a_minus = dual_vertices.to_vec();
                a_plus[k][d] += eps;
                a_minus[k][d] -= eps;

                let vol_plus = volume_fn(&a_plus);
                let vol_minus = volume_fn(&a_minus);

                grad[d] = match (vol_plus, vol_minus) {
                    (Some(vp), Some(vm)) => (vp - vm) / (2.0 * eps),
                    _ => f64::NAN,
                };
            }
            grad
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
    // Proposition: volume_derivatives_a matches finite differences to O(eps²).
    // Capacity derivatives are tested via the FD cross-check on known polytopes.
    // Reference: [lem:cap-derivative], [lem:vol-derivative] in experiments/sys-optimization/math.tex
    //
    // Strategy: fixture-based (hypercube) + FD cross-validation

    /// Volume derivative ∂vol/∂a_k should match finite differences.
    #[test]
    fn volume_derivatives_a_matches_fd() {
        let kp = known_polytopes::hypercube();
        let polytope = &kp.polytope;
        let duals = polytope.dual_vertices_f64();

        let analytical = volume_derivatives_a(polytope);
        let eps = 1e-6;
        let fd = volume_derivatives_a_fd(duals, eps, |a| {
            let p = Polytope4D::from_f64(a.to_vec()).ok()?;
            volume(&p).ok()
        });

        for k in 0..polytope.facet_count() {
            let err = (analytical[k] - fd[k]).norm();
            let scale = analytical[k].norm().max(fd[k].norm()).max(1e-10);
            let rel_err = err / scale;
            assert!(
                rel_err < 1e-4,
                "facet {k}: analytical={:?}, fd={:?}, rel_err={rel_err}",
                analytical[k], fd[k]
            );
        }
    }

    /// Analytical ∂c/∂a_k matches per-orbit FD central difference.
    #[test]
    #[ignore] // Calls ehz_capacity on F=8 hypercube. Fast in release, slow in debug.
    fn capacity_derivatives_a_on_hypercube() {
        let kp = known_polytopes::hypercube();
        let polytope = &kp.polytope;

        let (best_q, best_beta, best_perm, best_mu, _best_xi) =
            find_best_orbit(polytope);

        assert!(
            best_q > 1e-10,
            "find_best_orbit should find a valid orbit on the hypercube"
        );

        let duals = polytope.dual_vertices_f64();
        let analytical = capacity_derivatives_a(&best_beta, best_q, &best_mu, &best_perm, duals);

        let eps = 1e-6;

        // At least some derivatives should be non-zero
        let nonzero_count = analytical.iter().filter(|v| v.norm() > 1e-10).count();
        assert!(
            nonzero_count >= 2,
            "expected >= 2 non-zero capacity derivatives, got {nonzero_count}"
        );

        for k in 0..polytope.facet_count() {
            if analytical[k].norm() < 1e-12 {
                continue; // Facet not in orbit
            }
            // FD for the specific orbit's action
            for d in 0..4 {
                let mut ap = duals.to_vec();
                let mut am = duals.to_vec();
                ap[k][d] += eps;
                am[k][d] -= eps;
                let pp = Polytope4D::from_f64(ap).unwrap();
                let pm = Polytope4D::from_f64(am).unwrap();
                let qp = solve_kkt_for(&pp, &best_perm).feasible().map(|r| r.q_corrected);
                let qm = solve_kkt_for(&pm, &best_perm).feasible().map(|r| r.q_corrected);
                let fd_kd = match (qp, qm) {
                    (Some(qp), Some(qm)) => {
                        let ap = 0.5 / qp;
                        let am = 0.5 / qm;
                        (ap - am) / (2.0 * eps)
                    }
                    _ => continue,
                };
                let abs_err = (analytical[k][d] - fd_kd).abs();
                assert!(
                    abs_err < 1e-4,
                    "facet {k} dim {d}: analytical={}, fd={fd_kd}, abs_err={abs_err}",
                    analytical[k][d]
                );
            }
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
            .feasible()
            .expect("solve_kkt_for should succeed on the best permutation");
        (kkt.q_corrected, kkt.beta, perm, kkt.mu, kkt.xi)
    }
}

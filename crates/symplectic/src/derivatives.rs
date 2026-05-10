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
//! Mathematical correspondence: [lem:cap-derivative], [lem:vol-derivative] in
//! `formal/capacity-derivatives.tex`.

use crate::algorithms::OrbitKktData;
use crate::geom::facet_volume::facet_volume_and_centroid_3d_f64;
use crate::geom::polytope::Polytope4D;
use crate::geom::symplectic_form::j4;
use crate::geom::volume::volume_f64;
use crate::kkt::saddle_point_solver::KktResult;
use nalgebra::Vector4;

/// Gradient of one orbit/capacity-like quantity with respect to all dual
/// vertices `a_k`.
pub type OrbitGradientA = Vec<Vector4<f64>>;

/// Primitive Clarke-subdifferential representation: one gradient per orbit.
pub type ClarkeSubdiffA = Vec<OrbitGradientA>;

/// Facet-volume floor below which the volume derivative treats a facet as
/// degenerate and returns the zero contribution.
const VOLUME_DERIVATIVE_FACET_VOLUME_FLOOR: f64 = 1e-30;

/// Failure modes for derivative helpers layered above the low-level primitive.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DerivativeError {
    /// The supplied orbit payload does not carry the closure multiplier needed
    /// by the current derivative formula.
    MissingClosureMultiplier,
    /// The directional derivative of an empty Clarke-subdifferential is
    /// undefined.
    EmptySubdifferential,
}

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
) -> OrbitGradientA {
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

/// Compute ∂c/∂a_k for one saddle-point KKT solve and one sigma.
///
/// This helper captures the common current experiment boundary:
/// `(polytope, sigma, KktResult) -> gradient`.
pub fn capacity_derivatives_a_from_kkt_result(
    polytope: &Polytope4D,
    sigma: &[usize],
    kkt: &KktResult,
) -> OrbitGradientA {
    capacity_derivatives_a(
        &kkt.beta,
        kkt.q_corrected,
        &kkt.mu,
        sigma,
        polytope.dual_vertices_f64(),
    )
}

/// Compute ∂c/∂a_k from the shared orbit payload.
///
/// Returns an explicit error when the chosen orbit payload/backend does not
/// carry the closure multiplier required by the current derivative formula.
pub fn capacity_derivatives_a_from_orbit(
    polytope: &Polytope4D,
    orbit: &OrbitKktData,
) -> Result<OrbitGradientA, DerivativeError> {
    let mu = orbit.mu.ok_or(DerivativeError::MissingClosureMultiplier)?;
    Ok(capacity_derivatives_a(
        &orbit.beta,
        orbit.q,
        &mu,
        &orbit.sigma,
        polytope.dual_vertices_f64(),
    ))
}

fn systolic_ratio_gradient_a_from_parts(
    capacity: f64,
    volume: f64,
    d_capacity_da: &[Vector4<f64>],
    d_volume_da: &[Vector4<f64>],
) -> OrbitGradientA {
    let sys = crate::systolic_ratio(capacity, volume);
    d_capacity_da
        .iter()
        .zip(d_volume_da.iter())
        .map(|(dc, dv)| (capacity / volume) * dc - (sys / volume) * dv)
        .collect()
}

/// Compute ∂sys/∂a_k for all facets k = 0..f.
///
/// Uses the quotient rule for `sys = c^2 / (2V)` and combines the existing
/// capacity and volume dual-vertex gradients.
pub fn sys_gradient_a_from_kkt_result(
    polytope: &Polytope4D,
    sigma: &[usize],
    kkt: &KktResult,
) -> OrbitGradientA {
    let capacity = 1.0 / (2.0 * kkt.q_corrected);
    let volume = volume_f64(polytope);
    let d_capacity_da = capacity_derivatives_a_from_kkt_result(polytope, sigma, kkt);
    let d_volume_da = volume_derivatives_a(polytope);

    systolic_ratio_gradient_a_from_parts(capacity, volume, &d_capacity_da, &d_volume_da)
}

/// Compute ∂sys/∂a_k from the shared orbit payload.
///
/// Returns an explicit error when the chosen orbit payload/backend does not
/// carry the closure multiplier required by the current derivative formula.
pub fn sys_gradient_a_from_orbit(
    polytope: &Polytope4D,
    orbit: &OrbitKktData,
) -> Result<OrbitGradientA, DerivativeError> {
    let mu = orbit.mu.ok_or(DerivativeError::MissingClosureMultiplier)?;
    let capacity = 1.0 / (2.0 * orbit.q);
    let volume = volume_f64(polytope);
    let d_capacity_da = capacity_derivatives_a(
        &orbit.beta,
        orbit.q,
        &mu,
        &orbit.sigma,
        polytope.dual_vertices_f64(),
    );
    let d_volume_da = volume_derivatives_a(polytope);

    Ok(systolic_ratio_gradient_a_from_parts(
        capacity,
        volume,
        &d_capacity_da,
        &d_volume_da,
    ))
}

/// Assemble the per-orbit systolic-ratio gradients for a primitive
/// Clarke-subdifferential representation.
pub fn sys_subgradients_a(
    polytope: &Polytope4D,
    orbits: &[OrbitKktData],
) -> Result<ClarkeSubdiffA, DerivativeError> {
    orbits
        .iter()
        .map(|orbit| sys_gradient_a_from_orbit(polytope, orbit))
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
pub fn volume_derivatives_a(polytope: &Polytope4D) -> OrbitGradientA {
    let duals = polytope.dual_vertices_f64();
    let f = polytope.facet_count();

    (0..f)
        .map(|k| {
            let a = &duals[k];
            let a_norm = a.norm();
            let n = a / a_norm;
            let h = 1.0 / a_norm;

            let (s_k, centroid_k) = facet_volume_and_centroid_3d_f64(polytope, k);
            if s_k < VOLUME_DERIVATIVE_FACET_VOLUME_FLOOR {
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

/// Assemble the per-orbit capacity gradients for a Clarke-subdifferential.
pub fn capacity_subgradients_a(
    polytope: &Polytope4D,
    orbits: &[OrbitKktData],
) -> Result<ClarkeSubdiffA, DerivativeError> {
    orbits
        .iter()
        .map(|orbit| capacity_derivatives_a_from_orbit(polytope, orbit))
        .collect()
}

/// Directional derivative of one facet-indexed gradient in the perturbation
/// direction `d`.
pub fn directional_derivative_a(grad: &[Vector4<f64>], direction: &[Vector4<f64>]) -> f64 {
    grad.iter()
        .zip(direction.iter())
        .map(|(gk, dk)| gk.dot(dk))
        .sum()
}

/// Clarke directional derivative `min_i <g_i, d>` for a primitive gradient set.
pub fn clarke_directional_derivative_a(
    subdiff: &ClarkeSubdiffA,
    direction: &[Vector4<f64>],
) -> Result<f64, DerivativeError> {
    subdiff
        .iter()
        .map(|grad| directional_derivative_a(grad, direction))
        .min_by(|a, b| a.total_cmp(b))
        .ok_or(DerivativeError::EmptySubdifferential)
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
    use crate::algorithms::OrbitAdmissibility;
    use crate::geom::known_polytopes;
    use crate::geom::volume::volume_f64;
    use crate::kkt::saddle_point_solver::solve_kkt_for;

    // Tests for derivatives: analytical capacity and volume derivatives.
    //
    // Proposition: volume_derivatives_a matches finite differences to O(eps²).
    // Capacity derivatives are tested via the FD cross-check on known polytopes.
    // Reference: [lem:cap-derivative], [lem:vol-derivative] in
    // `formal/capacity-derivatives.tex`.
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
            Some(volume_f64(&p))
        });

        for k in 0..polytope.facet_count() {
            let err = (analytical[k] - fd[k]).norm();
            let scale = analytical[k].norm().max(fd[k].norm()).max(1e-10);
            let rel_err = err / scale;
            assert!(
                rel_err < 1e-4,
                "facet {k}: analytical={:?}, fd={:?}, rel_err={rel_err}",
                analytical[k],
                fd[k]
            );
        }
    }

    /// The KKT-result convenience helper should agree with the primitive
    /// derivative routine on the same saddle-point data.
    #[test]
    fn capacity_derivatives_from_kkt_result_matches_primitive() {
        let kp = known_polytopes::simplex();
        let polytope = &kp.polytope;
        let sigma = crate::ehz_capacity_pruned(polytope)
            .expect("simplex should have a certified best orbit")
            .best_sigma()
            .to_vec();
        let kkt = solve_kkt_for(polytope, &sigma)
            .feasible()
            .expect("best simplex orbit should re-solve");

        let direct = capacity_derivatives_a(
            &kkt.beta,
            kkt.q_corrected,
            &kkt.mu,
            &sigma,
            polytope.dual_vertices_f64(),
        );
        let wrapped = capacity_derivatives_a_from_kkt_result(polytope, &sigma, &kkt);

        assert_eq!(wrapped, direct);
    }

    /// Orbit-payload helper should fail explicitly when multiplier data is not
    /// available on the chosen backend/path.
    #[test]
    fn capacity_derivatives_from_orbit_requires_mu() {
        let kp = known_polytopes::simplex();
        let orbit = OrbitKktData {
            sigma: vec![0, 1],
            beta: vec![0.5, 0.5],
            beta_margin: 0.5,
            action: 1.0,
            action_lower: 1.0,
            action_upper: 1.0,
            q: 0.5,
            q_error_bound: 0.0,
            mu: None,
            xi: None,
            admissibility: OrbitAdmissibility::AdmissibleF64,
        };

        let err = capacity_derivatives_a_from_orbit(&kp.polytope, &orbit)
            .expect_err("orbit without mu should fail explicitly");
        assert_eq!(err, DerivativeError::MissingClosureMultiplier);
    }

    /// The KKT-result convenience helper should agree with the quotient-rule
    /// combination of capacity and volume gradients.
    #[test]
    fn sys_gradient_a_from_kkt_result_matches_formula() {
        let kp = known_polytopes::simplex();
        let polytope = &kp.polytope;
        let sigma = crate::ehz_capacity_pruned(polytope)
            .expect("simplex should have a certified best orbit")
            .best_sigma()
            .to_vec();
        let kkt = solve_kkt_for(polytope, &sigma)
            .feasible()
            .expect("best simplex orbit should re-solve");

        let capacity = 1.0 / (2.0 * kkt.q_corrected);
        let volume = volume_f64(polytope);
        let d_capacity_da = capacity_derivatives_a_from_kkt_result(polytope, &sigma, &kkt);
        let d_volume_da = volume_derivatives_a(polytope);
        let direct =
            systolic_ratio_gradient_a_from_parts(capacity, volume, &d_capacity_da, &d_volume_da);
        let wrapped = sys_gradient_a_from_kkt_result(polytope, &sigma, &kkt);

        for (k, (lhs, rhs)) in wrapped.iter().zip(direct.iter()).enumerate() {
            let err = (lhs - rhs).norm();
            assert!(
                err < 1e-12,
                "facet {k}: wrapped={lhs:?}, direct={rhs:?}, err={err}"
            );
        }
    }

    /// Orbit-payload helper should fail explicitly when multiplier data is not
    /// available on the chosen backend/path.
    #[test]
    fn sys_gradient_a_from_orbit_requires_mu() {
        let kp = known_polytopes::simplex();
        let orbit = OrbitKktData {
            sigma: vec![0, 1],
            beta: vec![0.5, 0.5],
            beta_margin: 0.5,
            action: 1.0,
            action_lower: 1.0,
            action_upper: 1.0,
            q: 0.5,
            q_error_bound: 0.0,
            mu: None,
            xi: None,
            admissibility: OrbitAdmissibility::AdmissibleF64,
        };

        let err = sys_gradient_a_from_orbit(&kp.polytope, &orbit)
            .expect_err("orbit without mu should fail explicitly");
        assert_eq!(err, DerivativeError::MissingClosureMultiplier);
    }

    /// Systolic-ratio subgradient assembly should fail explicitly when any
    /// orbit payload misses the multiplier needed by the derivative formula.
    #[test]
    fn sys_subgradients_a_requires_mu() {
        let kp = known_polytopes::simplex();
        let orbit = OrbitKktData {
            sigma: vec![0, 1],
            beta: vec![0.5, 0.5],
            beta_margin: 0.5,
            action: 1.0,
            action_lower: 1.0,
            action_upper: 1.0,
            q: 0.5,
            q_error_bound: 0.0,
            mu: None,
            xi: None,
            admissibility: OrbitAdmissibility::AdmissibleF64,
        };

        let err = sys_subgradients_a(&kp.polytope, &[orbit])
            .expect_err("orbit without mu should fail explicitly");
        assert_eq!(err, DerivativeError::MissingClosureMultiplier);
    }

    /// Clarke directional derivative should be the minimum directional slope
    /// over the supplied gradients.
    #[test]
    fn clarke_directional_derivative_takes_minimum() {
        let direction = vec![Vector4::new(1.0, 0.0, 0.0, 0.0)];
        let subdiff = vec![
            vec![Vector4::new(2.0, 0.0, 0.0, 0.0)],
            vec![Vector4::new(-3.0, 0.0, 0.0, 0.0)],
        ];

        let value = clarke_directional_derivative_a(&subdiff, &direction)
            .expect("nonempty subdifferential should have directional derivative");
        assert_eq!(value, -3.0);
    }

    /// Analytical ∂c/∂a_k matches per-orbit FD central difference.
    #[test]
    #[ignore] // Calls ehz_capacity on F=8 hypercube. Fast in release, slow in debug.
    fn capacity_derivatives_a_on_hypercube() {
        let kp = known_polytopes::hypercube();
        let polytope = &kp.polytope;

        let (best_q, best_beta, best_perm, best_mu, _best_xi) = find_best_orbit(polytope);

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
                let qp = solve_kkt_for(&pp, &best_perm)
                    .feasible()
                    .map(|r| r.q_corrected);
                let qm = solve_kkt_for(&pm, &best_perm)
                    .feasible()
                    .map(|r| r.q_corrected);
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
    fn find_best_orbit(polytope: &Polytope4D) -> (f64, Vec<f64>, Vec<usize>, Vec<f64>, f64) {
        let ehz = crate::ehz_capacity_pruned(polytope)
            .expect("ehz_capacity should find an orbit on test polytopes");
        let perm = ehz.best_sigma().to_vec();
        let kkt = solve_kkt_for(polytope, &perm)
            .feasible()
            .expect("solve_kkt_for should succeed on the best permutation");
        (kkt.q_corrected, kkt.beta, perm, kkt.mu, kkt.xi)
    }
}

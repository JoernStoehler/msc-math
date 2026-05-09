//! Exact dual-vertex capacity gradients from exact one-sigma KKT data.
//!
//! TODO: add [lem:cap-derivative] to formal math for the exact ordered-field
//! version of the dual-vertex capacity gradient formula.

use crate::exact::orbit::ExactOrbitKktData;
use crate::exact::polytope::ExactPolytope4D;
use algebraic_numbers::ExactScalar;

/// Compute the exact dual-vertex capacity gradient `∂c/∂a_k` from direct inputs.
pub(crate) fn capacity_derivatives_a_exact_raw<F: ExactScalar + 'static>(
    beta: &[F],
    q: &F,
    mu: &[F; 4],
    sigma: &[usize],
    dual_vertices: &[[F; 4]],
) -> Vec<[F; 4]> {
    let q_sq = q.clone() * q.clone();
    let two = F::one() + F::one();

    (0..dual_vertices.len())
        .map(|k| {
            let Some(i0) = sigma.iter().position(|&facet| facet == k) else {
                return std::array::from_fn(|_| F::zero());
            };

            let mut p: [F; 4] = std::array::from_fn(|_| F::zero());
            for i in 0..i0 {
                let dual = &dual_vertices[sigma[i]];
                for idx in 0..4 {
                    p[idx] = p[idx].clone() + beta[i].clone() * dual[idx].clone();
                }
            }

            let inner: [F; 4] = std::array::from_fn(|idx| {
                two.clone() * p[idx].clone() + beta[i0].clone() * dual_vertices[k][idx].clone()
            });
            let j0_inner = [
                -inner[2].clone(),
                -inner[3].clone(),
                inner[0].clone(),
                inner[1].clone(),
            ];
            let dq_da: [F; 4] = std::array::from_fn(|idx| {
                beta[i0].clone() * (j0_inner[idx].clone() + mu[idx].clone())
            });
            let scale = -(F::one() / (two.clone() * q_sq.clone()));
            std::array::from_fn(|idx| scale.clone() * dq_da[idx].clone())
        })
        .collect()
}

/// Compute the exact dual-vertex capacity gradient `∂c/∂a_k`.
pub fn capacity_derivatives_a_exact<F: ExactScalar + 'static>(
    polytope: &ExactPolytope4D<F>,
    orbit: &ExactOrbitKktData<F>,
) -> Vec<[F; 4]> {
    capacity_derivatives_a_exact_raw(
        &orbit.beta,
        &orbit.q,
        &orbit.mu,
        &orbit.sigma,
        polytope.dual_vertices(),
    )
}

#[cfg(test)]
mod tests {
    use super::capacity_derivatives_a_exact;
    use crate::derivatives::capacity_derivatives_a;
    use crate::exact::{solve_orbit_sigma_exact, ExactPolytope4D};
    use nalgebra::Vector4;
    use num_rational::BigRational;
    use num_traits::{ToPrimitive, Zero};

    fn q(n: i64) -> BigRational {
        BigRational::from_integer(n.into())
    }

    fn to_f64(value: &BigRational) -> f64 {
        value.to_f64().expect("small rational should fit in f64")
    }

    fn exact_simplex() -> ExactPolytope4D<BigRational> {
        let z = BigRational::zero();
        ExactPolytope4D::new(vec![
            [q(-5), z.clone(), z.clone(), z.clone()],
            [z.clone(), q(-5), z.clone(), z.clone()],
            [z.clone(), z.clone(), q(-5), z.clone()],
            [z.clone(), z.clone(), z.clone(), q(-5)],
            [q(5), q(5), q(5), q(5)],
        ])
        .expect("exact simplex")
    }

    #[test]
    fn simplex_gradient_is_nonzero_on_active_facets() {
        let polytope = exact_simplex();
        let sigma = [0usize, 1, 2, 3, 4];
        let orbit = solve_orbit_sigma_exact(&polytope, &sigma).expect("exact simplex sigma");
        let gradient = capacity_derivatives_a_exact(&polytope, &orbit);

        assert_eq!(gradient.len(), polytope.facet_count());
        assert!(gradient
            .iter()
            .any(|vec| vec.iter().any(|entry| !entry.is_zero())));
    }

    #[test]
    fn simplex_exact_gradient_matches_f64_formula() {
        let polytope = exact_simplex();
        let sigma = [0usize, 1, 2, 3, 4];
        let orbit = solve_orbit_sigma_exact(&polytope, &sigma).expect("exact simplex sigma");
        let exact_gradient = capacity_derivatives_a_exact(&polytope, &orbit);

        let beta_f64: Vec<f64> = orbit.beta.iter().map(to_f64).collect();
        let q_f64 = to_f64(&orbit.q);
        let mu_f64: Vec<f64> = orbit.mu.iter().map(to_f64).collect();
        let dual_vertices_f64: Vec<Vector4<f64>> = polytope
            .dual_vertices()
            .iter()
            .map(|dual| {
                Vector4::new(
                    to_f64(&dual[0]),
                    to_f64(&dual[1]),
                    to_f64(&dual[2]),
                    to_f64(&dual[3]),
                )
            })
            .collect();
        let float_gradient =
            capacity_derivatives_a(&beta_f64, q_f64, &mu_f64, &sigma, &dual_vertices_f64);

        for (exact, float) in exact_gradient.iter().zip(float_gradient.iter()) {
            for idx in 0..4 {
                assert!(
                    (to_f64(&exact[idx]) - float[idx]).abs() < 1.0e-12,
                    "gradient mismatch at component {idx}: exact={}, float={}",
                    to_f64(&exact[idx]),
                    float[idx]
                );
            }
        }
    }
}

//! Exact dual-vertex capacity gradients from exact one-sigma KKT data.
//!
//! TODO: add [lem:cap-derivative] to formal math for the exact ordered-field
//! version of the dual-vertex capacity gradient formula.

use crate::exact::orbit::ExactOrbitKktData;
use algebraic_numbers::ExactScalar;
use nalgebra::Vector4;

/// Compute the exact dual-vertex capacity gradient `∂c/∂a_k` from direct inputs.
///
/// Caller contract:
/// - `sigma` is a partial permutation of facet indices into `dual_vertices`;
/// - `beta.len() == sigma.len()`;
/// - `mu` and `q` come from the same one-sigma exact KKT solution.
pub fn capacity_derivatives_a_exact<F: ExactScalar + 'static>(
    beta: &[F],
    q: &F,
    mu: &Vector4<F>,
    sigma: &[usize],
    dual_vertices: &[Vector4<F>],
) -> Vec<Vector4<F>> {
    assert_eq!(beta.len(), sigma.len());
    assert!(is_partial_permutation(sigma, dual_vertices.len()));

    let q_sq = q.clone() * q.clone();
    let two = F::one() + F::one();

    (0..dual_vertices.len())
        .map(|k| {
            let Some(i0) = sigma.iter().position(|&facet| facet == k) else {
                return Vector4::new(F::zero(), F::zero(), F::zero(), F::zero());
            };

            let mut p = Vector4::new(F::zero(), F::zero(), F::zero(), F::zero());
            for i in 0..i0 {
                let dual = &dual_vertices[sigma[i]];
                for idx in 0..4 {
                    p[idx] = p[idx].clone() + beta[i].clone() * dual[idx].clone();
                }
            }

            let inner = Vector4::new(
                two.clone() * p[0].clone() + beta[i0].clone() * dual_vertices[k][0].clone(),
                two.clone() * p[1].clone() + beta[i0].clone() * dual_vertices[k][1].clone(),
                two.clone() * p[2].clone() + beta[i0].clone() * dual_vertices[k][2].clone(),
                two.clone() * p[3].clone() + beta[i0].clone() * dual_vertices[k][3].clone(),
            );
            let j0_inner = Vector4::new(
                -inner[2].clone(),
                -inner[3].clone(),
                inner[0].clone(),
                inner[1].clone(),
            );
            let dq_da = Vector4::new(
                beta[i0].clone() * (j0_inner[0].clone() + mu[0].clone()),
                beta[i0].clone() * (j0_inner[1].clone() + mu[1].clone()),
                beta[i0].clone() * (j0_inner[2].clone() + mu[2].clone()),
                beta[i0].clone() * (j0_inner[3].clone() + mu[3].clone()),
            );
            let scale = -(F::one() / (two.clone() * q_sq.clone()));
            Vector4::new(
                scale.clone() * dq_da[0].clone(),
                scale.clone() * dq_da[1].clone(),
                scale.clone() * dq_da[2].clone(),
                scale.clone() * dq_da[3].clone(),
            )
        })
        .collect()
}

fn is_partial_permutation(indices: &[usize], upper_bound: usize) -> bool {
    let mut seen = vec![false; upper_bound];
    for &index in indices {
        if index >= upper_bound || seen[index] {
            return false;
        }
        seen[index] = true;
    }
    true
}

/// Compute the exact dual-vertex capacity gradient from a solved exact orbit.
pub fn capacity_derivatives_a_exact_from_orbit<F: ExactScalar + 'static>(
    dual_vertices: &[Vector4<F>],
    orbit: &ExactOrbitKktData<F>,
) -> Vec<Vector4<F>> {
    capacity_derivatives_a_exact(
        &orbit.beta,
        &orbit.q,
        &orbit.mu,
        &orbit.sigma,
        dual_vertices,
    )
}

#[cfg(test)]
mod tests {
    use super::capacity_derivatives_a_exact_from_orbit;
    use crate::derivatives::capacity_derivatives_a;
    use crate::exact::solve_orbit_sigma_exact;
    use nalgebra::Vector4;
    use num_rational::BigRational;
    use num_traits::{ToPrimitive, Zero};

    fn q(n: i64) -> BigRational {
        BigRational::from_integer(n.into())
    }

    fn to_f64(value: &BigRational) -> f64 {
        value.to_f64().expect("small rational should fit in f64")
    }

    fn exact_simplex_dual_vertices() -> Vec<Vector4<BigRational>> {
        let z = BigRational::zero();
        vec![
            Vector4::new(q(-5), z.clone(), z.clone(), z.clone()),
            Vector4::new(z.clone(), q(-5), z.clone(), z.clone()),
            Vector4::new(z.clone(), z.clone(), q(-5), z.clone()),
            Vector4::new(z.clone(), z.clone(), z.clone(), q(-5)),
            Vector4::new(q(5), q(5), q(5), q(5)),
        ]
    }

    #[test]
    fn simplex_gradient_is_nonzero_on_active_facets() {
        let dual_vertices = exact_simplex_dual_vertices();
        let sigma = [0usize, 1, 2, 3, 4];
        let orbit = solve_orbit_sigma_exact(&dual_vertices, &sigma).expect("exact simplex sigma");
        let gradient = capacity_derivatives_a_exact_from_orbit(&dual_vertices, &orbit);

        assert_eq!(gradient.len(), dual_vertices.len());
        assert!(gradient
            .iter()
            .any(|vec| vec.iter().any(|entry| !entry.is_zero())));
    }

    #[test]
    fn simplex_exact_gradient_matches_f64_formula() {
        let dual_vertices = exact_simplex_dual_vertices();
        let sigma = [0usize, 1, 2, 3, 4];
        let orbit = solve_orbit_sigma_exact(&dual_vertices, &sigma).expect("exact simplex sigma");
        let exact_gradient = capacity_derivatives_a_exact_from_orbit(&dual_vertices, &orbit);

        let beta_f64: Vec<f64> = orbit.beta.iter().map(to_f64).collect();
        let q_f64 = to_f64(&orbit.q);
        let mu_f64: Vec<f64> = orbit.mu.iter().map(to_f64).collect();
        let dual_vertices_f64: Vec<Vector4<f64>> = dual_vertices
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

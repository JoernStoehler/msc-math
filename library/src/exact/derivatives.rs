//! Exact dual-vertex capacity gradients from exact one-sigma KKT data.
//!
//! TODO: add [lem:cap-derivative] to formal math for the exact ordered-field
//! version of the dual-vertex capacity gradient formula.

use crate::exact::orbit::ExactOrbitKktData;
use crate::exact::polytope::ExactPolytope4D;
use real_algebraic::OrderedField;

/// Compute the exact dual-vertex capacity gradient `∂c/∂a_k`.
pub fn capacity_derivatives_a_exact<F: OrderedField>(
    polytope: &ExactPolytope4D<F>,
    orbit: &ExactOrbitKktData<F>,
) -> Vec<[F; 4]> {
    let q_sq = orbit.q.clone() * orbit.q.clone();
    let two = F::from_i64(2);

    (0..polytope.facet_count())
        .map(|k| {
            let Some(i0) = orbit.sigma.iter().position(|&facet| facet == k) else {
                return zero_vec();
            };

            let mut p = zero_vec();
            for i in 0..i0 {
                p = add_vec(&p, &scale_vec(orbit.beta[i].clone(), &polytope.dual_vertices()[orbit.sigma[i]]));
            }

            let inner = add_vec(
                &scale_vec(two.clone(), &p),
                &scale_vec(orbit.beta[i0].clone(), &polytope.dual_vertices()[k]),
            );
            let dq_da = scale_vec(orbit.beta[i0].clone(), &add_vec(&apply_j0(&inner), &orbit.mu));
            scale_vec(-(F::one() / (two.clone() * q_sq.clone())), &dq_da)
        })
        .collect()
}

fn zero_vec<F: OrderedField>() -> [F; 4] {
    std::array::from_fn(|_| F::zero())
}

fn add_vec<F: OrderedField>(left: &[F; 4], right: &[F; 4]) -> [F; 4] {
    std::array::from_fn(|idx| left[idx].clone() + right[idx].clone())
}

fn scale_vec<F: OrderedField>(scalar: F, vector: &[F; 4]) -> [F; 4] {
    std::array::from_fn(|idx| scalar.clone() * vector[idx].clone())
}

fn apply_j0<F: OrderedField>(vector: &[F; 4]) -> [F; 4] {
    [
        -vector[2].clone(),
        -vector[3].clone(),
        vector[0].clone(),
        vector[1].clone(),
    ]
}

#[cfg(test)]
mod tests {
    use super::capacity_derivatives_a_exact;
    use crate::derivatives::capacity_derivatives_a;
    use crate::exact::{solve_orbit_sigma_exact, ExactPolytope4D};
    use real_algebraic::{OrderedField, Rational};
    use nalgebra::Vector4;

    fn exact_simplex() -> ExactPolytope4D<Rational> {
        let z = Rational::from_i64(0);
        ExactPolytope4D::new(vec![
            [Rational::from_i64(-5), z.clone(), z.clone(), z.clone()],
            [z.clone(), Rational::from_i64(-5), z.clone(), z.clone()],
            [z.clone(), z.clone(), Rational::from_i64(-5), z.clone()],
            [z.clone(), z.clone(), z.clone(), Rational::from_i64(-5)],
            [
                Rational::from_i64(5),
                Rational::from_i64(5),
                Rational::from_i64(5),
                Rational::from_i64(5),
            ],
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
        assert!(gradient.iter().any(|vec| vec.iter().any(|entry| !entry.is_zero())));
    }

    #[test]
    fn simplex_exact_gradient_matches_f64_formula() {
        let polytope = exact_simplex();
        let sigma = [0usize, 1, 2, 3, 4];
        let orbit = solve_orbit_sigma_exact(&polytope, &sigma).expect("exact simplex sigma");
        let exact_gradient = capacity_derivatives_a_exact(&polytope, &orbit);

        let beta_f64: Vec<f64> = orbit.beta.iter().map(OrderedField::to_f64).collect();
        let q_f64 = orbit.q.to_f64();
        let mu_f64: Vec<f64> = orbit.mu.iter().map(OrderedField::to_f64).collect();
        let dual_vertices_f64: Vec<Vector4<f64>> = polytope
            .dual_vertices()
            .iter()
            .map(|dual| Vector4::new(dual[0].to_f64(), dual[1].to_f64(), dual[2].to_f64(), dual[3].to_f64()))
            .collect();
        let float_gradient = capacity_derivatives_a(
            &beta_f64,
            q_f64,
            &mu_f64,
            &sigma,
            &dual_vertices_f64,
        );

        for (exact, float) in exact_gradient.iter().zip(float_gradient.iter()) {
            for idx in 0..4 {
                assert!(
                    (exact[idx].to_f64() - float[idx]).abs() < 1.0e-12,
                    "gradient mismatch at component {idx}: exact={}, float={}",
                    exact[idx].to_f64(),
                    float[idx]
                );
            }
        }
    }
}

use nalgebra::DMatrix;
use num_rational::BigRational;
use num_traits::{One, Signed, ToPrimitive};
use symplectic::algorithms::hk2017::SimpleDirectedCyclesCanonical;
use symplectic::kkt::rational_solver::solve_kkt_exact;

#[derive(Clone, Debug, PartialEq)]
pub struct ExactCapacityOrbit {
    pub sigma: Vec<usize>,
    pub beta_exact: Vec<BigRational>,
    pub q_exact: BigRational,
    pub action_exact: BigRational,
    pub action: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExactCapacityReport {
    pub capacity_exact: BigRational,
    pub capacity: f64,
    pub action_gap_exact: BigRational,
    pub minimizers: Vec<ExactCapacityOrbit>,
    pub orbits: Vec<ExactCapacityOrbit>,
    pub iterations: u64,
    pub exact_admissible_count: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExactCapacityError {
    InvalidTransitionMatrix,
    InvalidGap,
    NoAdmissibleOrbit,
}

/// Exact reference route over the complete visited transition-pruned sigma stream.
///
/// This route is intentionally separate from retained-candidate exact
/// certification. It does not call the f64 single-sigma solver and therefore
/// does not depend on f64 candidate retention. It is meant for small inputs,
/// route comparison, and false-certification audits.
pub fn solve_exact_capacity_for_transition_pruned_sigmas(
    dual_vertices_exact: &[[BigRational; 4]],
    transition_is_allowed: &DMatrix<bool>,
    action_gap_exact: BigRational,
) -> Result<ExactCapacityReport, ExactCapacityError> {
    if transition_is_allowed.nrows() != transition_is_allowed.ncols()
        || transition_is_allowed.nrows() != dual_vertices_exact.len()
    {
        return Err(ExactCapacityError::InvalidTransitionMatrix);
    }
    if action_gap_exact.is_negative() {
        return Err(ExactCapacityError::InvalidGap);
    }

    let mut iterations = 0u64;
    let mut exact_orbits = Vec::new();
    for sigma in SimpleDirectedCyclesCanonical::new(transition_is_allowed) {
        iterations += 1;
        if let Some(orbit) = exact_orbit(dual_vertices_exact, &sigma) {
            exact_orbits.push(orbit);
        }
    }
    if exact_orbits.is_empty() {
        return Err(ExactCapacityError::NoAdmissibleOrbit);
    }

    exact_orbits.sort_by(|a, b| {
        a.action_exact
            .cmp(&b.action_exact)
            .then_with(|| a.sigma.cmp(&b.sigma))
    });
    let capacity_exact = exact_orbits[0].action_exact.clone();
    let capacity = rational_to_f64(&capacity_exact);
    let window_cutoff = capacity_exact.clone() + action_gap_exact.clone();
    let exact_admissible_count = exact_orbits.len();

    let mut minimizers: Vec<ExactCapacityOrbit> = exact_orbits
        .iter()
        .filter(|orbit| orbit.action_exact == capacity_exact)
        .cloned()
        .collect();
    minimizers.sort_by(|a, b| a.sigma.cmp(&b.sigma));

    let orbits = exact_orbits
        .into_iter()
        .filter(|orbit| orbit.action_exact <= window_cutoff)
        .collect();

    Ok(ExactCapacityReport {
        capacity_exact,
        capacity,
        action_gap_exact,
        minimizers,
        orbits,
        iterations,
        exact_admissible_count,
    })
}

fn exact_orbit(
    dual_vertices_exact: &[[BigRational; 4]],
    sigma: &[usize],
) -> Option<ExactCapacityOrbit> {
    let exact = solve_kkt_exact(dual_vertices_exact, sigma)?;
    if !exact.q_exact.is_positive() {
        return None;
    }
    let action_exact = exact_action_from_q(&exact.q_exact);
    let action = rational_to_f64(&action_exact);
    Some(ExactCapacityOrbit {
        sigma: sigma.to_vec(),
        beta_exact: exact.beta,
        q_exact: exact.q_exact,
        action_exact,
        action,
    })
}

fn exact_action_from_q(q_exact: &BigRational) -> BigRational {
    BigRational::one() / (q_exact.clone() + q_exact.clone())
}

fn rational_to_f64(value: &BigRational) -> f64 {
    value.to_f64().unwrap_or(f64::NAN)
}

#[cfg(test)]
mod tests {
    use super::*;
    use euclidean_polytopes::{
        facet_intersection_is_nonempty_from_vertex_facet_incidence,
        polar_vertices_exact_rational_assuming_origin_interior, PolarVerticesExact,
    };
    use nalgebra::Vector4;
    use num_traits::Zero;
    use symplectic::algorithms::facet_adjacency::build_transition_matrix_from_facet_intersections_and_omega;
    use symplectic::exact::omega_signs_exact;
    use symplectic::geom::rational_arithmetic::f64_to_rational;

    #[test]
    fn exact_route_solves_small_generated_case_without_f64_candidate_filter() {
        let case = crate::generated_f64_cases_with_source_filter(
            1,
            99540836,
            &["seed99540836:F5:sample0:attempt5000000008".to_string()],
        )
        .pop()
        .expect("known generated case");
        let dual_vertices_exact = exact_dual_vertex_arrays(&case.dual_vertices);
        let transition = exact_binary64_transition_matrix(&dual_vertices_exact);

        let report = solve_exact_capacity_for_transition_pruned_sigmas(
            &dual_vertices_exact,
            &transition,
            BigRational::zero(),
        )
        .expect("small generated case has an exact orbit");

        assert_eq!(report.iterations, 9);
        assert_eq!(report.exact_admissible_count, 2);
        assert_eq!(report.minimizers.len(), 1);
        assert_eq!(report.orbits, report.minimizers);
    }

    fn exact_dual_vertex_arrays(dual_vertices: &[Vector4<f64>]) -> Vec<[BigRational; 4]> {
        dual_vertices
            .iter()
            .map(|vertex| {
                [
                    f64_to_rational(vertex[0]),
                    f64_to_rational(vertex[1]),
                    f64_to_rational(vertex[2]),
                    f64_to_rational(vertex[3]),
                ]
            })
            .collect()
    }

    fn exact_dual_vertex_vectors(dual_vertices: &[[BigRational; 4]]) -> Vec<Vector4<BigRational>> {
        dual_vertices
            .iter()
            .map(|vertex| {
                Vector4::new(
                    vertex[0].clone(),
                    vertex[1].clone(),
                    vertex[2].clone(),
                    vertex[3].clone(),
                )
            })
            .collect()
    }

    fn exact_binary64_transition_matrix(dual_vertices_exact: &[[BigRational; 4]]) -> DMatrix<bool> {
        let dual_vectors = exact_dual_vertex_vectors(dual_vertices_exact);
        let PolarVerticesExact {
            vertex_facet_incidence,
            ..
        } = polar_vertices_exact_rational_assuming_origin_interior(&dual_vectors);
        let facet_intersection_is_nonempty =
            facet_intersection_is_nonempty_from_vertex_facet_incidence(&vertex_facet_incidence);
        let omega_signs = omega_signs_exact(&dual_vectors);
        build_transition_matrix_from_facet_intersections_and_omega(
            &facet_intersection_is_nonempty,
            &omega_signs,
        )
    }
}

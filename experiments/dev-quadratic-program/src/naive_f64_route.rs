use nalgebra::{DMatrix, Matrix4, Vector4};
use symplectic::algorithms::hk2017::SimpleDirectedCyclesCanonical;
use symplectic::kkt::projection_solver::{
    solve_projected_critical_point_for_dual_vertices, ProjectedCriticalPoint,
};
use symplectic::omega0;

#[derive(Clone, Debug, PartialEq)]
pub struct NaiveF64Capacity {
    pub capacity: f64,
    pub sigma: Vec<usize>,
}

/// Deliberately careless f64 realization of the transition-pruned HK2017 route.
///
/// This is a control path for experiments. It does not use exact arithmetic,
/// tolerances, indeterminate states, fallback, or diagnostic counters.
pub fn capacity_naive_f64(dual_vertices: &[Vector4<f64>]) -> Option<NaiveF64Capacity> {
    if dual_vertices.len() < 5
        || dual_vertices
            .iter()
            .any(|vertex| !vertex.iter().all(|entry| entry.is_finite()))
    {
        return None;
    }
    let transition_is_allowed = literal_transition_matrix(dual_vertices);
    let mut best: Option<NaiveF64Capacity> = None;
    for sigma in SimpleDirectedCyclesCanonical::new(&transition_is_allowed) {
        let Some(action) = naive_action_for_sigma(dual_vertices, &sigma) else {
            continue;
        };
        if best
            .as_ref()
            .is_none_or(|current| action < current.capacity)
        {
            best = Some(NaiveF64Capacity {
                capacity: action,
                sigma,
            });
        }
    }
    best
}

fn naive_action_for_sigma(dual_vertices: &[Vector4<f64>], sigma: &[usize]) -> Option<f64> {
    let ProjectedCriticalPoint::Found(critical) =
        solve_projected_critical_point_for_dual_vertices(dual_vertices, sigma)
    else {
        return None;
    };
    if !critical.beta.iter().all(|entry| *entry > 0.0) || critical.q <= 0.0 {
        return None;
    }
    let action = 0.5 / critical.q;
    action.is_finite().then_some(action)
}

fn literal_transition_matrix(dual_vertices: &[Vector4<f64>]) -> DMatrix<bool> {
    let facet_intersection_is_nonempty = literal_facet_intersections(dual_vertices);
    DMatrix::from_fn(dual_vertices.len(), dual_vertices.len(), |i, j| {
        facet_intersection_is_nonempty[(i, j)]
            && omega0(&dual_vertices[i], &dual_vertices[j]) >= 0.0
    })
}

fn literal_facet_intersections(dual_vertices: &[Vector4<f64>]) -> DMatrix<bool> {
    let facet_count = dual_vertices.len();
    let mut result = DMatrix::from_element(facet_count, facet_count, false);
    for a in 0..facet_count {
        for b in a + 1..facet_count {
            for c in b + 1..facet_count {
                for d in c + 1..facet_count {
                    let facets = [a, b, c, d];
                    let Some(vertex) = literal_vertex(dual_vertices, facets) else {
                        continue;
                    };
                    if dual_vertices
                        .iter()
                        .all(|normal| normal.dot(&vertex) <= 1.0)
                    {
                        let incident = literal_incident_facets(dual_vertices, &vertex, facets);
                        for &i in &incident {
                            for &j in &incident {
                                result[(i, j)] = true;
                            }
                        }
                    }
                }
            }
        }
    }
    result
}

fn literal_vertex(dual_vertices: &[Vector4<f64>], facets: [usize; 4]) -> Option<Vector4<f64>> {
    let matrix = facet_matrix(dual_vertices, facets);
    if matrix.determinant() == 0.0 {
        return None;
    }
    matrix.lu().solve(&Vector4::repeat(1.0))
}

fn literal_incident_facets(
    dual_vertices: &[Vector4<f64>],
    vertex: &Vector4<f64>,
    defining_facets: [usize; 4],
) -> Vec<usize> {
    let mut incident = defining_facets.to_vec();
    for (facet, normal) in dual_vertices.iter().enumerate() {
        if !defining_facets.contains(&facet) && normal.dot(vertex) == 1.0 {
            incident.push(facet);
        }
    }
    incident.sort_unstable();
    incident.dedup();
    incident
}

fn facet_matrix(dual_vertices: &[Vector4<f64>], facets: [usize; 4]) -> Matrix4<f64> {
    let rows = facets.map(|idx| dual_vertices[idx]);
    Matrix4::new(
        rows[0][0], rows[0][1], rows[0][2], rows[0][3], rows[1][0], rows[1][1], rows[1][2],
        rows[1][3], rows[2][0], rows[2][1], rows[2][2], rows[2][3], rows[3][0], rows[3][1],
        rows[3][2], rows[3][3],
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        exact_binary64_dual_vertex_arrays,
        exact_binary64_transition_matrix_assuming_origin_interior,
        solve_exact_capacity_for_transition_pruned_sigmas,
    };
    use nalgebra::Vector4;
    use num_rational::BigRational;
    use num_traits::Zero;
    use std::fmt::Write;
    use symplectic::known_polytopes;

    #[test]
    fn naive_simplex_fixture_matches_exact_reference() {
        assert_matches_exact_reference(&known_polytopes::simplex().dual_vertices_f64);
    }

    #[test]
    fn naive_random_fixture_misses_exact_reference() {
        let dual_vertices = vec![
            Vector4::new(
                -0.7609176562997226,
                -0.5842245470076217,
                -0.6093220693528425,
                0.07216780853507296,
            ),
            Vector4::new(
                0.784069284213464,
                -0.5531443877418841,
                0.18211913477611671,
                -0.36079445513926356,
            ),
            Vector4::new(
                -0.043547885416314415,
                0.8556529705333096,
                0.8361784175796745,
                0.2857765173406991,
            ),
            Vector4::new(
                -0.2753007640820361,
                -0.48381690655215637,
                -0.8235951274500787,
                0.35426171198575546,
            ),
            Vector4::new(
                -0.12602783596581424,
                0.6516682410783413,
                0.1098373351502524,
                -0.5152232850628169,
            ),
        ];
        let naive = capacity_naive_f64(&dual_vertices).expect("naive route returns a value");
        let exact = exact_reference(&dual_vertices);
        let report = naive_random_miss_report(&dual_vertices);
        eprintln!("{report}");

        assert_eq!(naive.sigma, vec![0, 4, 3, 1, 2]);
        assert_eq!(exact.minimizers[0].sigma, vec![0, 3, 1, 4, 2]);
        assert!(
            (naive.capacity - exact.capacity).abs() > 1e-6,
            "generated random row should expose naive f64 failure:\n{report}\nnaive={naive:?}\nexact={exact:?}"
        );
    }

    fn assert_matches_exact_reference(dual_vertices: &[Vector4<f64>]) {
        let naive = capacity_naive_f64(dual_vertices).expect("naive route returns a value");
        let exact = exact_reference(dual_vertices);
        assert!(
            (naive.capacity - exact.capacity).abs() < 1e-10,
            "naive={naive:?}, exact={exact:?}"
        );
    }

    fn exact_reference(dual_vertices: &[Vector4<f64>]) -> crate::ExactCapacityReport {
        let exact_vertices = exact_binary64_dual_vertex_arrays(dual_vertices);
        let transition = exact_binary64_transition_matrix_assuming_origin_interior(&exact_vertices);
        solve_exact_capacity_for_transition_pruned_sigmas(
            &exact_vertices,
            &transition,
            BigRational::zero(),
        )
        .expect("exact reference capacity")
    }

    fn exact_reference_with_large_window(
        dual_vertices: &[Vector4<f64>],
    ) -> crate::ExactCapacityReport {
        let exact_vertices = exact_binary64_dual_vertex_arrays(dual_vertices);
        let transition = exact_binary64_transition_matrix_assuming_origin_interior(&exact_vertices);
        solve_exact_capacity_for_transition_pruned_sigmas(
            &exact_vertices,
            &transition,
            BigRational::new(100.into(), 1.into()),
        )
        .expect("exact reference capacity")
    }

    fn naive_random_miss_report(dual_vertices: &[Vector4<f64>]) -> String {
        let exact_vertices = exact_binary64_dual_vertex_arrays(dual_vertices);
        let exact_transition =
            exact_binary64_transition_matrix_assuming_origin_interior(&exact_vertices);
        let literal_transition = literal_transition_matrix(dual_vertices);
        let literal_facet_intersections = literal_facet_intersections(dual_vertices);
        let exact = exact_reference_with_large_window(dual_vertices);

        let mut literal_candidates: Vec<_> =
            SimpleDirectedCyclesCanonical::new(&literal_transition).collect();
        literal_candidates.sort();
        let mut literal_accepted = literal_candidates
            .iter()
            .filter_map(|sigma| {
                naive_action_for_sigma(dual_vertices, sigma).map(|action| (sigma, action))
            })
            .collect::<Vec<_>>();
        literal_accepted.sort_by(|(a_sigma, a_action), (b_sigma, b_action)| {
            a_action
                .total_cmp(b_action)
                .then_with(|| a_sigma.cmp(b_sigma))
        });

        let mut report = String::new();
        writeln!(
            report,
            "naive f64 random miss diagnostic; this does not repair the route"
        )
        .unwrap();
        writeln!(
            report,
            "status: the naive route is intentionally careless; this output explains one concrete wrong answer."
        )
        .unwrap();
        writeln!(
            report,
            "summary: exact_capacity={:.17}, naive_capacity={:.17}, exact_iterations={}, exact_admissible={}, naive_cycles={}, naive_accepted={}",
            exact.capacity,
            literal_accepted[0].1,
            exact.iterations,
            exact.exact_admissible_count,
            literal_candidates.len(),
            literal_accepted.len()
        )
        .unwrap();
        writeln!(
            report,
            "transition graph edge counts: exact_rational={}, naive_f64={}",
            true_count(&exact_transition),
            true_count(&literal_transition)
        )
        .unwrap();
        if let Some(minimizer) = exact.minimizers.first() {
            let missing_edges = missing_cycle_edges(&literal_transition, &minimizer.sigma);
            writeln!(
                report,
                "cause: exact minimizer sigma={:?} is not enumerated by the naive f64 route because the naive transition graph is missing edge(s) {:?}.",
                minimizer.sigma, missing_edges
            )
            .unwrap();
            writeln!(
                report,
                "forced f64 solve on exact minimizer: {}",
                forced_f64_status(dual_vertices, &minimizer.sigma)
            )
            .unwrap();
            writeln!(
            report,
            "finding: the f64 critical-point solve is fine on the missed sigma; the wrong capacity comes from naive f64 transition-graph construction before candidate solving."
        )
            .unwrap();
        }
        writeln!(
            report,
            "transition rule: edge i->j is allowed when facets i and j meet in the primal polytope and omega0(y_i,y_j) >= 0."
        )
        .unwrap();
        writeln!(
            report,
            "naive f64 facet-meet test: solve every 4-facet equality system in f64; keep x only if every inequality y_k.x <= 1.0 literally; then mark pairs of facets incident at kept x."
        )
        .unwrap();
        writeln!(report, "transition graph differences:").unwrap();
        write_transition_differences(
            &mut report,
            dual_vertices,
            &exact_transition,
            &literal_transition,
            &literal_facet_intersections,
        );
        write_missing_exact_edge_details(
            &mut report,
            dual_vertices,
            &exact_transition,
            &literal_transition,
        );
        writeln!(report, "exact_admissible_orbits_sorted_by_action:").unwrap();
        for orbit in &exact.orbits {
            writeln!(
                report,
                "  sigma={:?} action={:.17} naive_transition_graph={} forced_f64={}",
                orbit.sigma,
                orbit.action,
                literal_edge_status(&literal_transition, &orbit.sigma),
                forced_f64_status(dual_vertices, &orbit.sigma)
            )
            .unwrap();
        }
        writeln!(report, "naive_f64_accepted_orbits_sorted_by_action:").unwrap();
        for (sigma, action) in literal_accepted {
            writeln!(
                report,
                "  sigma={sigma:?} action={action:.17} exact_transition_graph={} forced_f64={}",
                exact_edge_status(&exact_transition, sigma),
                forced_f64_status(dual_vertices, sigma)
            )
            .unwrap();
        }
        report
    }

    fn true_count(matrix: &DMatrix<bool>) -> usize {
        matrix.iter().filter(|&&entry| entry).count()
    }

    fn write_transition_differences(
        report: &mut String,
        dual_vertices: &[Vector4<f64>],
        exact_transition: &DMatrix<bool>,
        literal_transition: &DMatrix<bool>,
        literal_facet_intersections: &DMatrix<bool>,
    ) {
        let mut difference_count = 0usize;
        for i in 0..exact_transition.nrows() {
            for j in 0..exact_transition.ncols() {
                if exact_transition[(i, j)] == literal_transition[(i, j)] {
                    continue;
                }
                difference_count += 1;
                writeln!(
                    report,
                    "  edge {i}->{j}: exact_rational_allows={}, naive_f64_allows={}, omega0_f64={:.17e}, naive_f64_says_facets_meet={}",
                    exact_transition[(i, j)],
                    literal_transition[(i, j)],
                    omega0(&dual_vertices[i], &dual_vertices[j]),
                    literal_facet_intersections[(i, j)]
                )
                .unwrap();
            }
        }
        if difference_count == 0 {
            writeln!(report, "  none").unwrap();
        }
    }

    fn write_missing_exact_edge_details(
        report: &mut String,
        dual_vertices: &[Vector4<f64>],
        exact_transition: &DMatrix<bool>,
        literal_transition: &DMatrix<bool>,
    ) {
        for i in 0..exact_transition.nrows() {
            for j in 0..exact_transition.ncols() {
                if !exact_transition[(i, j)] || literal_transition[(i, j)] {
                    continue;
                }
                writeln!(
                    report,
                "why naive f64 rejects exact edge {i}->{j}: omega0_f64={:.17e}, so the omega condition passes; rejection is only from the f64 facet-meet test.",
                    omega0(&dual_vertices[i], &dual_vertices[j])
                )
                .unwrap();
                if let Some(max_positive_slack) =
                    write_facet_pair_vertex_attempts(report, dual_vertices, i, j)
                {
                    writeln!(
                        report,
                        "  interpretation for edge {i}->{j}: every f64 vertex attempt that would prove the facets meet was rejected by a strict y_k.x <= 1.0 check; the largest positive violation above is {max_positive_slack:.17e}, i.e. roundoff at an equality-scale predicate."
                    )
                    .unwrap();
                }
            }
        }
    }

    fn write_facet_pair_vertex_attempts(
        report: &mut String,
        dual_vertices: &[Vector4<f64>],
        first: usize,
        second: usize,
    ) -> Option<f64> {
        let facet_count = dual_vertices.len();
        let mut largest_positive_slack: Option<f64> = None;
        for third in 0..facet_count {
            if third == first || third == second {
                continue;
            }
            for fourth in third + 1..facet_count {
                if fourth == first || fourth == second {
                    continue;
                }
                let mut facets = [first, second, third, fourth];
                facets.sort_unstable();
                let matrix = facet_matrix(dual_vertices, facets);
                let determinant = matrix.determinant();
                let Some(vertex) = matrix.lu().solve(&Vector4::repeat(1.0)) else {
                    writeln!(
                        report,
                        "  facets {facets:?}: no f64 solution, determinant={determinant:.17e}"
                    )
                    .unwrap();
                    continue;
                };
                let signed_slacks: Vec<f64> = dual_vertices
                    .iter()
                    .map(|normal| normal.dot(&vertex) - 1.0)
                    .collect();
                let (worst_facet, worst_slack) = signed_slacks
                    .iter()
                    .copied()
                    .enumerate()
                    .max_by(|(_, a), (_, b)| a.total_cmp(b))
                    .expect("nonempty slacks");
                let kept_by_naive = signed_slacks.iter().all(|slack| *slack <= 0.0);
                if !kept_by_naive && worst_slack > 0.0 {
                    largest_positive_slack = Some(
                        largest_positive_slack.map_or(worst_slack, |old| old.max(worst_slack)),
                    );
                }
                writeln!(
                    report,
                    "  facets {facets:?}: determinant={determinant:.17e}, kept_by_naive={}, worst_inequality=facet {worst_facet} has y.x-1={worst_slack:.17e}, all_y.x_minus_1={}",
                    kept_by_naive,
                    format_f64_list(&signed_slacks)
                )
                .unwrap();
            }
        }
        largest_positive_slack
    }

    fn format_f64_list(values: &[f64]) -> String {
        let mut result = String::from("[");
        for (idx, value) in values.iter().enumerate() {
            if idx > 0 {
                result.push_str(", ");
            }
            write!(result, "{value:.17e}").unwrap();
        }
        result.push(']');
        result
    }

    fn literal_edge_status(transition: &DMatrix<bool>, sigma: &[usize]) -> String {
        missing_edge_status(transition, sigma)
    }

    fn exact_edge_status(transition: &DMatrix<bool>, sigma: &[usize]) -> String {
        missing_edge_status(transition, sigma)
    }

    fn missing_edge_status(transition: &DMatrix<bool>, sigma: &[usize]) -> String {
        let missing_edges = missing_cycle_edges(transition, sigma);
        if missing_edges.is_empty() {
            "all_edges_present".to_string()
        } else {
            format!("missing_edges{missing_edges:?}")
        }
    }

    fn missing_cycle_edges(transition: &DMatrix<bool>, sigma: &[usize]) -> Vec<(usize, usize)> {
        sigma
            .iter()
            .copied()
            .zip(sigma.iter().copied().cycle().skip(1))
            .take(sigma.len())
            .filter(|&(i, j)| !transition[(i, j)])
            .collect()
    }

    fn forced_f64_status(dual_vertices: &[Vector4<f64>], sigma: &[usize]) -> String {
        match solve_projected_critical_point_for_dual_vertices(dual_vertices, sigma) {
            ProjectedCriticalPoint::Found(critical) => {
                let action = if critical.q > 0.0 {
                    0.5 / critical.q
                } else {
                    f64::NAN
                };
                format!(
                    "found q={:.17e} action={:.17} min_beta={:.17e} beta_positive={} q_positive={} flat_dirs={} residuals(stationarity={:.3e}, constraint={:.3e}) q_error_bound={:?}",
                    critical.q,
                    action,
                    critical.min_beta,
                    critical.beta.iter().all(|entry| *entry > 0.0),
                    critical.q > 0.0,
                    critical.flat_direction_count,
                    critical.stationarity_residual,
                    critical.constraint_residual,
                    critical.q_error_bound
                )
            }
            ProjectedCriticalPoint::NoConstraintSolution { residual } => {
                format!("no_constraint_solution residual={residual:.3e}")
            }
            ProjectedCriticalPoint::NoCriticalPoint {
                stationarity_residual,
                flat_direction_count,
            } => format!(
                "no_critical_point stationarity_residual={stationarity_residual:.3e} flat_dirs={flat_direction_count}"
            ),
        }
    }
}

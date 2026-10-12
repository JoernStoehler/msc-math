mod args;
mod output;

#[path = "../scan/input/mod.rs"]
mod input;

use algebraic_numbers::{solve_linear_system, LinearSystemSolution};
use euclidean_polytopes::{
    facet_intersection_is_nonempty_from_vertex_facet_incidence,
    polar_vertices_exact_rational_assuming_origin_interior, PolarVerticesExact,
};
use exp_dev_quadratic_program::ScanCase;
use nalgebra::{DMatrix, DVector, Vector4};
use num_rational::BigRational;
use num_traits::{One, Signed, ToPrimitive, Zero};
use output::{write_rows, KktErrorAuditRow};
use std::panic::{catch_unwind, AssertUnwindSafe};
use symplectic::algorithms::facet_adjacency::build_transition_matrix_from_facet_intersections_and_omega;
use symplectic::exact::omega_signs_exact;
use symplectic::geom::rational_arithmetic::f64_to_rational;
use symplectic::kkt::qp_assembly::{
    build_augmented_system_from_dual_vertices, build_qp_from_dual_vertices,
};
use symplectic::kkt::rational_solver::solve_kkt_exact;
use symplectic::{
    solve_pruned_hk2017_candidates, solve_unpruned_hk2017_candidates, OrbitAdmissibility,
    OrbitKktData,
};

const EPS_Q_POSITIVE: f64 = 1e-15;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum TernaryVerdict {
    True,
    False,
    Indet,
}

impl TernaryVerdict {
    fn as_json_label(self) -> &'static str {
        match self {
            TernaryVerdict::True => "true",
            TernaryVerdict::False => "false",
            TernaryVerdict::Indet => "indet",
        }
    }
}

fn main() {
    let args = args::parse_args();
    let cases = input::load_cases(&input::LoadCaseOptions {
        input_source: args.input_source,
        max_rows_per_family: args.max_rows_per_family,
        generated_samples_per_facet: args.generated_samples_per_facet,
        generated_seed: args.generated_seed,
        family_filter: args.family_filter,
        source_id_filter: args.source_id_filter,
    });

    let mut rows = Vec::new();
    for case in cases {
        rows.extend(audit_case(
            &case,
            args.max_candidates_per_case,
            args.enumeration,
        ));
    }
    write_rows(&args.output, &rows);
    eprintln!("wrote {} rows to {}", rows.len(), args.output.display());
}

fn audit_case(
    case: &ScanCase,
    max_candidates_per_case: usize,
    enumeration: args::Enumeration,
) -> Vec<KktErrorAuditRow> {
    let exact_input = exact_dual_vertex_arrays(&case.dual_vertices);
    let enumeration_label = match enumeration {
        args::Enumeration::PrunedExactBinary64 => "hk2017_pruned_exact_binary64",
        args::Enumeration::Unpruned => "hk2017_unpruned",
    };
    let candidate_result = match enumeration {
        args::Enumeration::Unpruned => solve_unpruned_hk2017_candidates(&case.dual_vertices),
        args::Enumeration::PrunedExactBinary64 => {
            let Ok(transition) = exact_binary64_transition_matrix(&exact_input) else {
                return vec![case_error_row(
                    case,
                    enumeration_label,
                    "exact_binary64_geometry_failed",
                )];
            };
            solve_pruned_hk2017_candidates(&case.dual_vertices, &transition)
        }
    };

    let Ok((mut candidates, iterations)) = candidate_result else {
        return vec![case_error_row(
            case,
            enumeration_label,
            "f64_candidate_solve_failed",
        )];
    };
    candidates.sort_by(|a, b| {
        a.action
            .total_cmp(&b.action)
            .then_with(|| a.sigma.cmp(&b.sigma))
    });

    candidates
        .into_iter()
        .take(row_limit(max_candidates_per_case))
        .enumerate()
        .map(|(action_rank, candidate)| {
            candidate_row(
                case,
                iterations,
                action_rank,
                &exact_input,
                candidate,
                enumeration_label,
            )
        })
        .collect()
}

fn candidate_row(
    case: &ScanCase,
    iterations: u64,
    action_rank: usize,
    exact_input: &[[BigRational; 4]],
    candidate: OrbitKktData,
    enumeration_label: &'static str,
) -> KktErrorAuditRow {
    let exact = solve_kkt_exact(exact_input, &candidate.sigma);
    let exact_positive = exact.is_some();
    let exact_q = exact.as_ref().map(|value| value.q_exact_f64);
    let exact_action = exact_q.map(|q| 0.5 / q);
    let exact_beta: Option<Vec<f64>> = exact.as_ref().map(|value| {
        value
            .beta
            .iter()
            .map(|beta| rational_to_f64(beta))
            .collect()
    });
    let exact_beta_margin = exact_beta
        .as_ref()
        .map(|beta| beta.iter().copied().fold(f64::INFINITY, f64::min));
    let exact_beta_error = exact_beta
        .as_ref()
        .map(|beta| vector_inf_distance(&candidate.beta, beta));

    let diagnostics = numerical_diagnostics(&case.dual_vertices, exact_input, &candidate);
    let verified_inverse_beta_radius = diagnostics
        .verified_kkt_inverse_inf_norm_bound
        .map(|inverse_norm| inverse_norm * diagnostics.exact_kkt_residual_inf_norm);
    let verified_inverse_beta_radius_verdict = verified_inverse_beta_radius
        .map(|radius| beta_radius_verdict(candidate.beta_margin, radius));
    let verified_inverse_beta_radius_covers_exact_beta = verified_inverse_beta_radius
        .zip(exact_beta_error)
        .map(|(radius, error)| error <= radius);

    let qp = build_qp_from_dual_vertices(&case.dual_vertices, &candidate.sigma);
    let verified_inverse_beta_radius_q_bound = verified_inverse_beta_radius
        .map(|radius| beta_radius_q_bound(&qp.h, &candidate.beta, radius));
    let verified_inverse_beta_radius_q_bound_covers_exact = verified_inverse_beta_radius_q_bound
        .zip(exact_q)
        .map(|(bound, q)| (candidate.q - q).abs() <= bound);

    KktErrorAuditRow {
        event: "qp_kkt_error_observation",
        family: case.family.clone(),
        source_id: case.source_id.clone(),
        input_source: case.input_source.clone(),
        enumeration: enumeration_label,
        facet_count: case.dual_vertices.len(),
        iterations,
        action_rank,
        sigma: candidate.sigma,
        status: "ok",
        f64_admissibility: Some(admissibility_label(candidate.admissibility)),
        current_f64_verdict: Some(current_f64_verdict(candidate.admissibility)),
        verified_inverse_beta_radius_verdict: verified_inverse_beta_radius_verdict
            .map(TernaryVerdict::as_json_label),
        exact_positive: Some(exact_positive),
        exact_q,
        exact_action,
        exact_beta_margin,
        f64_q: Some(candidate.q),
        f64_action: Some(candidate.action),
        f64_action_lower: Some(candidate.action_lower),
        f64_action_upper: Some(candidate.action_upper),
        f64_beta_margin: Some(candidate.beta_margin),
        f64_beta_inf_norm: Some(
            candidate
                .beta
                .iter()
                .map(|beta| beta.abs())
                .fold(0.0, f64::max),
        ),
        current_q_error_bound: Some(candidate.q_error_bound),
        current_q_bound_covers_exact: exact_q
            .map(|q| (candidate.q - q).abs() <= candidate.q_error_bound),
        verified_inverse_beta_radius,
        verified_inverse_beta_radius_covers_exact_beta,
        verified_inverse_beta_radius_q_bound,
        verified_inverse_beta_radius_q_bound_covers_exact,
        kkt_residual_inf_norm: Some(diagnostics.kkt_residual_inf_norm),
        exact_kkt_residual_inf_norm: Some(diagnostics.exact_kkt_residual_inf_norm),
        kkt_inverse_inf_norm: diagnostics.kkt_inverse_inf_norm,
        exact_inverse_residual_inf_norm: diagnostics.exact_inverse_residual_inf_norm,
        verified_kkt_inverse_inf_norm_bound: diagnostics.verified_kkt_inverse_inf_norm_bound,
        note: None,
    }
}

fn case_error_row(
    case: &ScanCase,
    enumeration_label: &'static str,
    note: &'static str,
) -> KktErrorAuditRow {
    KktErrorAuditRow {
        event: "qp_kkt_error_observation",
        family: case.family.clone(),
        source_id: case.source_id.clone(),
        input_source: case.input_source.clone(),
        enumeration: enumeration_label,
        facet_count: case.dual_vertices.len(),
        iterations: 0,
        action_rank: 0,
        sigma: Vec::new(),
        status: "case_error",
        f64_admissibility: None,
        current_f64_verdict: None,
        verified_inverse_beta_radius_verdict: None,
        exact_positive: None,
        exact_q: None,
        exact_action: None,
        exact_beta_margin: None,
        f64_q: None,
        f64_action: None,
        f64_action_lower: None,
        f64_action_upper: None,
        f64_beta_margin: None,
        f64_beta_inf_norm: None,
        current_q_error_bound: None,
        current_q_bound_covers_exact: None,
        verified_inverse_beta_radius: None,
        verified_inverse_beta_radius_covers_exact_beta: None,
        verified_inverse_beta_radius_q_bound: None,
        verified_inverse_beta_radius_q_bound_covers_exact: None,
        kkt_residual_inf_norm: None,
        exact_kkt_residual_inf_norm: None,
        kkt_inverse_inf_norm: None,
        exact_inverse_residual_inf_norm: None,
        verified_kkt_inverse_inf_norm_bound: None,
        note: Some(note),
    }
}

struct NumericalDiagnostics {
    kkt_residual_inf_norm: f64,
    exact_kkt_residual_inf_norm: f64,
    kkt_inverse_inf_norm: Option<f64>,
    exact_inverse_residual_inf_norm: Option<f64>,
    verified_kkt_inverse_inf_norm_bound: Option<f64>,
}

fn numerical_diagnostics(
    dual_vertices: &[Vector4<f64>],
    exact_input: &[[BigRational; 4]],
    candidate: &OrbitKktData,
) -> NumericalDiagnostics {
    let (kkt, rhs) = build_augmented_system_from_dual_vertices(dual_vertices, &candidate.sigma);
    let x = candidate_solution_vector(candidate);
    let residual = &kkt * &x - rhs;
    let kkt_residual_inf_norm = vector_inf_norm(&residual);
    let exact_kkt_residual_inf_norm =
        exact_kkt_residual_inf_norm(exact_input, &candidate.sigma, candidate);
    let kkt_inverse = kkt.try_inverse();
    let kkt_inverse_inf_norm = kkt_inverse.as_ref().map(matrix_inf_norm);
    let exact_inverse_residual_inf_norm = kkt_inverse
        .as_ref()
        .map(|inverse| exact_inverse_residual_inf_norm(exact_input, &candidate.sigma, inverse));
    let verified_kkt_inverse_inf_norm_bound = kkt_inverse_inf_norm
        .zip(exact_inverse_residual_inf_norm)
        .and_then(|(inverse_norm, inverse_residual)| {
            (inverse_residual < 1.0).then_some(inverse_norm / (1.0 - inverse_residual))
        });

    NumericalDiagnostics {
        kkt_residual_inf_norm,
        exact_kkt_residual_inf_norm,
        kkt_inverse_inf_norm,
        exact_inverse_residual_inf_norm,
        verified_kkt_inverse_inf_norm_bound,
    }
}

fn candidate_solution_vector(candidate: &OrbitKktData) -> DVector<f64> {
    let m = candidate.sigma.len();
    let mut x = DVector::zeros(m + 5);
    for (idx, beta) in candidate.beta.iter().enumerate() {
        x[idx] = *beta;
    }
    if let Some(mu) = candidate.mu {
        for (idx, value) in mu.iter().enumerate() {
            x[m + idx] = *value;
        }
    }
    if let Some(xi) = candidate.xi {
        x[m + 4] = xi;
    }
    x
}

/// Ternary predicate for `Positive(beta_exact)`, assuming
/// `||beta_f64 - beta_exact||_inf <= radius`.
fn beta_radius_verdict(beta_f64_min: f64, radius: f64) -> TernaryVerdict {
    if beta_f64_min > radius {
        TernaryVerdict::True
    } else if beta_f64_min < -radius {
        TernaryVerdict::False
    } else {
        TernaryVerdict::Indet
    }
}

fn beta_radius_q_bound(hessian: &DMatrix<f64>, beta: &[f64], radius: f64) -> f64 {
    let beta = DVector::from_column_slice(beta);
    let h_beta = hessian * beta;
    let linear_l1 = h_beta.iter().map(|value| value.abs()).sum::<f64>();
    let h_abs_sum = (0..hessian.nrows())
        .flat_map(|row| (0..hessian.ncols()).map(move |col| hessian[(row, col)].abs()))
        .sum::<f64>();
    radius * linear_l1 + 0.5 * radius * radius * h_abs_sum
}

fn exact_kkt_residual_inf_norm(
    dual_vertices: &[[BigRational; 4]],
    perm: &[usize],
    candidate: &OrbitKktData,
) -> f64 {
    let m = perm.len();
    let (matrix, rhs) = build_exact_kkt_matrix(dual_vertices, perm);
    let mut x = DVector::from_element(m + 5, BigRational::zero());
    for (idx, beta) in candidate.beta.iter().enumerate() {
        x[idx] = f64_to_rational(*beta);
    }
    if let Some(mu) = candidate.mu {
        for (idx, value) in mu.iter().enumerate() {
            x[m + idx] = f64_to_rational(*value);
        }
    }
    if let Some(xi) = candidate.xi {
        x[m + 4] = f64_to_rational(xi);
    }
    let residual = matrix * x - rhs;
    residual
        .iter()
        .map(|value| rational_to_f64(value).abs())
        .fold(0.0, f64::max)
}

fn exact_inverse_residual_inf_norm(
    dual_vertices: &[[BigRational; 4]],
    perm: &[usize],
    inverse: &DMatrix<f64>,
) -> f64 {
    let (matrix, _) = build_exact_kkt_matrix(dual_vertices, perm);
    let size = matrix.nrows();
    (0..size)
        .map(|row| {
            (0..size)
                .map(|col| {
                    let mut value = BigRational::zero();
                    for mid in 0..size {
                        value += &matrix[(row, mid)] * f64_to_rational(inverse[(mid, col)]);
                    }
                    if row == col {
                        value -= BigRational::one();
                    }
                    rational_to_f64(&value).abs()
                })
                .sum::<f64>()
        })
        .fold(0.0, f64::max)
}

fn build_exact_kkt_matrix(
    dual_vertices: &[[BigRational; 4]],
    perm: &[usize],
) -> (DMatrix<BigRational>, DVector<BigRational>) {
    let m = perm.len();
    let size = m + 5;
    let mut matrix = DMatrix::from_element(size, size, BigRational::zero());
    let mut rhs = DVector::from_element(size, BigRational::zero());

    for i in 0..m {
        for j in (i + 1)..m {
            let value = omega0_rational_local(&dual_vertices[perm[i]], &dual_vertices[perm[j]]);
            matrix[(i, j)] = value.clone();
            matrix[(j, i)] = value;
        }
    }
    for i in 0..m {
        for dim in 0..4 {
            let value = dual_vertices[perm[i]][dim].clone();
            matrix[(i, m + dim)] = value.clone();
            matrix[(m + dim, i)] = value;
        }
        matrix[(i, m + 4)] = BigRational::one();
        matrix[(m + 4, i)] = BigRational::one();
    }
    rhs[m + 4] = BigRational::one();
    (matrix, rhs)
}

fn exact_binary64_transition_matrix(
    dual_vertices_exact: &[[BigRational; 4]],
) -> Result<DMatrix<bool>, String> {
    catch_unwind(AssertUnwindSafe(|| {
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
    }))
    .map_err(|_| "exact binary64 transition matrix construction panicked".to_string())
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

#[allow(dead_code)]
fn exact_linear_kkt_diagnostics(
    dual_vertices: &[[BigRational; 4]],
    perm: &[usize],
) -> Option<Vec<BigRational>> {
    let m = perm.len();
    let (matrix, rhs) = build_exact_kkt_matrix(dual_vertices, perm);
    match solve_linear_system(&matrix, &rhs) {
        LinearSystemSolution::Inconsistent => None,
        LinearSystemSolution::Consistent { particular, .. } => {
            Some(particular.iter().take(m).cloned().collect())
        }
    }
}

fn current_f64_verdict(value: OrbitAdmissibility) -> &'static str {
    match value {
        OrbitAdmissibility::AdmissibleF64 | OrbitAdmissibility::AdmissibleExact => "true",
        OrbitAdmissibility::IndeterminateF64 => "indet",
    }
}

fn admissibility_label(value: OrbitAdmissibility) -> &'static str {
    match value {
        OrbitAdmissibility::AdmissibleF64 => "admissible_f64",
        OrbitAdmissibility::IndeterminateF64 => "indeterminate_f64",
        OrbitAdmissibility::AdmissibleExact => "admissible_exact",
    }
}

fn omega0_rational_local(u: &[BigRational; 4], v: &[BigRational; 4]) -> BigRational {
    &u[0] * &v[2] - &u[2] * &v[0] + &u[1] * &v[3] - &u[3] * &v[1]
}

fn rational_to_f64(value: &BigRational) -> f64 {
    value.to_f64().unwrap_or(f64::NAN)
}

fn vector_inf_norm(vector: &DVector<f64>) -> f64 {
    vector.iter().map(|value| value.abs()).fold(0.0, f64::max)
}

fn vector_inf_distance(left: &[f64], right: &[f64]) -> f64 {
    if left.len() != right.len() {
        return f64::NAN;
    }
    left.iter()
        .zip(right)
        .map(|(a, b)| (a - b).abs())
        .fold(0.0, f64::max)
}

fn matrix_inf_norm(matrix: &DMatrix<f64>) -> f64 {
    (0..matrix.nrows())
        .map(|row| {
            (0..matrix.ncols())
                .map(|col| matrix[(row, col)].abs())
                .sum::<f64>()
        })
        .fold(0.0, f64::max)
}

fn row_limit(max_candidates_per_case: usize) -> usize {
    if max_candidates_per_case == 0 {
        usize::MAX
    } else {
        max_candidates_per_case
    }
}

#[allow(dead_code)]
fn action_bounds_from_q_bound(q: f64, q_error_bound: f64) -> (f64, f64) {
    let q_upper = q + q_error_bound;
    let action_lower = 0.5 / q_upper;
    let q_lower = q - q_error_bound;
    let action_upper = if q_lower > EPS_Q_POSITIVE {
        0.5 / q_lower
    } else {
        f64::INFINITY
    };
    (action_lower, action_upper)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn beta_radius_verdict_is_ternary() {
        assert_eq!(beta_radius_verdict(0.2, 0.1), TernaryVerdict::True);
        assert_eq!(beta_radius_verdict(-0.2, 0.1), TernaryVerdict::False);
        assert_eq!(beta_radius_verdict(0.05, 0.1), TernaryVerdict::Indet);
    }

    #[test]
    fn q_bound_from_zero_radius_is_zero() {
        let hessian = DMatrix::identity(2, 2);
        assert_eq!(beta_radius_q_bound(&hessian, &[0.5, 0.5], 0.0), 0.0);
    }
}

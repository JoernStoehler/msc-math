//! v2 numerical-soundness comparison packet.
//!
//! This intentionally keeps solver centres, one-word exact targets, stream
//! policies, and scalar aggregation separate.  It is experiment-local: none
//! of these proposals changes a production capacity route.

use algebraic_numbers::{rank as exact_rank, solve_linear_system, LinearSystemSolution};
use exp_dev_quadratic_program::{
    exact_binary64_dual_vertex_arrays, exact_binary64_transition_matrix_assuming_origin_interior,
    generated_f64_cases_with_source_filter,
};
use nalgebra::{DMatrix, DVector, Vector4};
use num_rational::BigRational;
use num_traits::{One, Signed, Zero};
use serde::Serialize;
use std::{
    collections::BTreeMap,
    env,
    fs::{create_dir_all, File},
    io::{BufWriter, Write},
    path::PathBuf,
    time::Instant,
};
use symplectic::{
    algorithms::{
        facet_adjacency::build_transition_matrix_from_facet_intersections_and_omega,
        hk2017::SimpleDirectedCyclesCanonical,
    },
    geom::known_polytopes::{self, KnownPolytope},
    kkt::{
        projection_solver::{
            solve_projected, solve_projected_critical_point, ProjectedCriticalPoint,
        },
        q_value,
        qp_assembly::{build_augmented_system_from_dual_vertices, build_qp_from_dual_vertices},
        rational_solver::solve_kkt_exact,
        saddle_point_solver::{solve_kkt_for_dual_vertices, KktOutcome},
        Verdict,
    },
};

const RUN_ID: &str = "qp-soundness-v2";
const SCHEMA: &str = "qp-soundness-row-v2";
const POLICY_SCHEMA: &str = "qp-soundness-policy-v2";
const MAX_SMALL_STREAM: usize = 128;
const RELATIVE_WINDOWS: [f64; 5] = [0.0, 0.001, 0.01, 0.05, 0.20];

#[derive(Clone)]
struct Case {
    case_id: &'static str,
    cohort: &'static str,
    source_id: String,
    completeness: &'static str,
    target_kind: &'static str,
    intended_algebraic_status: &'static str,
    dual_f64: Vec<Vector4<f64>>,
    dual_exact: Vec<[BigRational; 4]>,
    sigmas: Vec<Vec<usize>>,
}

#[derive(Serialize, Clone)]
struct CenterRecord {
    center_id: String,
    center_contract: String,
    center_availability: String,
    center_unavailable_reason: Option<String>,
    center_beta_f64: Option<Vec<f64>>,
    center_mu_f64: Option<Vec<f64>>,
    center_xi_f64: Option<f64>,
    center_q_raw_f64: Option<f64>,
    center_q_constraint_residual_correction_f64: Option<f64>,
    center_q_corrected_f64: Option<f64>,
    center_action_from_positive_q_f64: Option<f64>,
    center_beta_margin_f64: Option<f64>,
    center_full_kkt_residual_norm_f64: Option<f64>,
    center_stationarity_residual_norm_f64: Option<f64>,
    center_closure_residual_norm_f64: Option<f64>,
    center_normalization_residual_abs_f64: Option<f64>,
    center_rank_f64: Option<usize>,
    center_timing_us: f64,
}

#[derive(Serialize, Clone)]
struct RawRow {
    run_id: String,
    schema_version: String,
    target_polytope_id: String,
    target_source_id: String,
    target_input_kind: String,
    intended_algebraic_target_status: String,
    supplied_stream_completeness: String,
    sigma_active_reeb_word: Vec<usize>,
    sigma_length: usize,
    lifecycle_transition_source: String,
    lifecycle_route_visited: bool,
    lifecycle_route_disposition: String,
    saddle_eig_actual_tier_status: String,
    saddle_eig_solver_outcome: String,
    saddle_eig_beta_predicate: String,
    f64_retained_by_saddle: bool,
    qp_constraint_matrix_c_f64: Vec<Vec<f64>>,
    qp_objective_hessian_h_f64: Vec<Vec<f64>>,
    kkt_augmented_matrix_m_f64: Vec<Vec<f64>>,
    kkt_augmented_rhs_b_f64: Vec<f64>,
    qp_constraint_matrix_c_exact: Vec<Vec<String>>,
    qp_objective_hessian_h_exact: Vec<Vec<String>>,
    kkt_augmented_matrix_m_exact: Vec<Vec<String>>,
    kkt_augmented_rhs_b_exact: Vec<String>,
    kkt_augmented_singular_values_f64: Vec<f64>,
    kkt_augmented_eigenvalues_f64: Vec<f64>,
    kkt_augmented_condition_number_f64: Option<f64>,
    centers: Vec<CenterRecord>,
    exact_row_reduction_system_status: String,
    exact_row_reduction_rank: Option<usize>,
    exact_row_reduction_nullity: Option<usize>,
    exact_row_reduction_beta_particular: Option<Vec<String>>,
    exact_positive_witness_status: String,
    exact_positive_witness_reason: String,
    exact_positive_witness_beta: Option<Vec<String>>,
    exact_positive_witness_q: Option<String>,
    exact_positive_witness_q_sign: String,
    exact_positive_witness_action: Option<String>,
    exact_action_availability: String,
    stage_timings_us: BTreeMap<String, f64>,
}

#[derive(Serialize)]
struct PolicyRow {
    schema_version: String,
    target_polytope_id: String,
    policy_id: String,
    policy_description: String,
    exactness_scope: String,
    requested_window_kind: String,
    requested_relative_gap: f64,
    supplied_stream_count: usize,
    policy_candidate_count: usize,
    policy_exact_resolution_count: usize,
    policy_exact_accept_count: usize,
    policy_min_action: Option<String>,
    policy_minimizer_active_words: Vec<Vec<usize>>,
    policy_window_active_words: Vec<Vec<usize>>,
    policy_window_cutoff: Option<String>,
    policy_fallback_trigger: String,
    policy_fallback_result: String,
    policy_stage_timing_us: f64,
}

#[derive(Serialize)]
struct FormulaRegistryEntry {
    formula_id: String,
    output_column: String,
    dependencies: Vec<(String, String)>,
    center: String,
    exact_target: String,
    hypothesis_status: String,
    status: String,
    consumers: Vec<String>,
    unavailable_rule: String,
}

fn rat(x: &BigRational) -> String {
    format!("{}/{}", x.numer(), x.denom())
}
fn matrix_rows<T: Copy>(m: &DMatrix<T>) -> Vec<Vec<T>> {
    (0..m.nrows())
        .map(|i| (0..m.ncols()).map(|j| m[(i, j)]).collect())
        .collect()
}
fn vector_rows<T: Copy>(v: &DVector<T>) -> Vec<T> {
    v.iter().copied().collect()
}
fn exact_matrix_rows(m: &DMatrix<BigRational>) -> Vec<Vec<String>> {
    (0..m.nrows())
        .map(|i| (0..m.ncols()).map(|j| rat(&m[(i, j)])).collect())
        .collect()
}

fn exact_kkt(
    dual: &[[BigRational; 4]],
    sigma: &[usize],
) -> (DMatrix<BigRational>, DVector<BigRational>) {
    let m = sigma.len();
    let mut out = DMatrix::from_element(m + 5, m + 5, BigRational::zero());
    for i in 0..m {
        for j in (i + 1)..m {
            let x = &dual[sigma[i]];
            let y = &dual[sigma[j]];
            let w = &x[0] * &y[2] - &x[2] * &y[0] + &x[1] * &y[3] - &x[3] * &y[1];
            out[(i, j)] = w.clone();
            out[(j, i)] = w;
        }
        for d in 0..4 {
            out[(i, m + d)] = dual[sigma[i]][d].clone();
            out[(m + d, i)] = dual[sigma[i]][d].clone();
        }
        out[(i, m + 4)] = BigRational::one();
        out[(m + 4, i)] = BigRational::one();
    }
    let mut rhs = DVector::from_element(m + 5, BigRational::zero());
    rhs[m + 4] = BigRational::one();
    (out, rhs)
}

fn classify_exact(sign_q: Option<&BigRational>) -> String {
    match sign_q {
        Some(q) if q.is_positive() => "positive".into(),
        Some(q) if q.is_zero() => "zero".into(),
        Some(_) => "negative".into(),
        None => "unavailable".into(),
    }
}
fn residual_atoms(
    m: &DMatrix<f64>,
    b: &DVector<f64>,
    x: &DVector<f64>,
    beta_len: usize,
) -> (f64, f64, f64, f64, f64) {
    let r = m * x - b;
    let full = r.norm();
    let stationarity = r.rows(0, beta_len).norm();
    let closure = r.rows(beta_len, 4).norm();
    let normalization = r[beta_len + 4].abs();
    let correction = (0..4)
        .map(|i| r[beta_len + i] * x[beta_len + i])
        .sum::<f64>()
        + r[beta_len + 4] * x[beta_len + 4];
    (full, stationarity, closure, normalization, correction)
}
fn center_from_solution(
    id: &str,
    contract: &str,
    x: Option<DVector<f64>>,
    h: &DMatrix<f64>,
    m: &DMatrix<f64>,
    b: &DVector<f64>,
    beta_len: usize,
    rank: Option<usize>,
    elapsed: f64,
) -> CenterRecord {
    let Some(x) = x else {
        return CenterRecord {
            center_id: id.into(),
            center_contract: contract.into(),
            center_availability: "unavailable".into(),
            center_unavailable_reason: Some(
                "factorization did not return a finite full-system solution".into(),
            ),
            center_beta_f64: None,
            center_mu_f64: None,
            center_xi_f64: None,
            center_q_raw_f64: None,
            center_q_constraint_residual_correction_f64: None,
            center_q_corrected_f64: None,
            center_action_from_positive_q_f64: None,
            center_beta_margin_f64: None,
            center_full_kkt_residual_norm_f64: None,
            center_stationarity_residual_norm_f64: None,
            center_closure_residual_norm_f64: None,
            center_normalization_residual_abs_f64: None,
            center_rank_f64: rank,
            center_timing_us: elapsed,
        };
    };
    let beta = x.rows(0, beta_len).iter().copied().collect::<Vec<_>>();
    let raw = q_value(h, &beta);
    let (full, stat, closure, norm, correction) = residual_atoms(m, b, &x, beta_len);
    let corrected = raw + correction;
    CenterRecord {
        center_id: id.into(),
        center_contract: contract.into(),
        center_availability: "available".into(),
        center_unavailable_reason: None,
        center_beta_margin_f64: Some(beta.iter().copied().fold(f64::INFINITY, f64::min)),
        center_beta_f64: Some(beta),
        center_mu_f64: Some(x.rows(beta_len, 4).iter().copied().collect()),
        center_xi_f64: Some(x[beta_len + 4]),
        center_q_raw_f64: Some(raw),
        center_q_constraint_residual_correction_f64: Some(correction),
        center_q_corrected_f64: Some(corrected),
        center_action_from_positive_q_f64: (corrected > 0.0).then(|| 0.5 / corrected),
        center_full_kkt_residual_norm_f64: Some(full),
        center_stationarity_residual_norm_f64: Some(stat),
        center_closure_residual_norm_f64: Some(closure),
        center_normalization_residual_abs_f64: Some(norm),
        center_rank_f64: rank,
        center_timing_us: elapsed,
    }
}

fn projection_centers(qp: &symplectic::kkt::QP) -> (CenterRecord, CenterRecord) {
    let start = Instant::now();
    let critical = solve_projected_critical_point(qp);
    let elapsed = start.elapsed().as_secs_f64() * 1e6;
    let critical_center = match critical {
        ProjectedCriticalPoint::Found(data) => CenterRecord { center_id:"projected_critical_proposal".into(), center_contract:"projected stationarity representative; no multiplier or residual correction supplied by this API".into(), center_availability:"available".into(), center_unavailable_reason:None, center_beta_margin_f64:Some(data.min_beta), center_beta_f64:Some(data.beta), center_mu_f64:None, center_xi_f64:None, center_q_raw_f64:Some(data.q), center_q_constraint_residual_correction_f64:None, center_q_corrected_f64:None, center_action_from_positive_q_f64:(data.q>0.0).then(||0.5/data.q), center_full_kkt_residual_norm_f64:None, center_stationarity_residual_norm_f64:Some(data.stationarity_residual), center_closure_residual_norm_f64:Some(data.constraint_residual), center_normalization_residual_abs_f64:None, center_rank_f64:None, center_timing_us:elapsed },
        ProjectedCriticalPoint::NoConstraintSolution{residual} => unavailable_center("projected_critical_proposal", "constraint projection", format!("no constraint solution; residual={residual}"), elapsed),
        ProjectedCriticalPoint::NoCriticalPoint{stationarity_residual,..} => unavailable_center("projected_critical_proposal", "constraint projection", format!("no projected critical point; residual={stationarity_residual}"), elapsed),
    };
    let start = Instant::now();
    let max_margin = solve_projected(qp);
    let elapsed = start.elapsed().as_secs_f64() * 1e6;
    let verdict = match max_margin.verdict {
        Verdict::True => "true",
        Verdict::False => "false",
        Verdict::Indeterminate => "indeterminate",
    };
    let max_center = CenterRecord {
        center_id: "projected_max_margin_proposal".into(),
        center_contract: format!(
            "projected critical affine family with max-margin LP; f64 verdict={verdict}"
        ),
        center_availability: "available".into(),
        center_unavailable_reason: None,
        center_beta_margin_f64: Some(max_margin.margin),
        center_beta_f64: Some(max_margin.beta),
        center_mu_f64: None,
        center_xi_f64: None,
        center_q_raw_f64: Some(max_margin.q),
        center_q_constraint_residual_correction_f64: None,
        center_q_corrected_f64: None,
        center_action_from_positive_q_f64: (max_margin.q > 0.0).then(|| 0.5 / max_margin.q),
        center_full_kkt_residual_norm_f64: None,
        center_stationarity_residual_norm_f64: None,
        center_closure_residual_norm_f64: None,
        center_normalization_residual_abs_f64: None,
        center_rank_f64: None,
        center_timing_us: elapsed,
    };
    (critical_center, max_center)
}
fn unavailable_center(id: &str, contract: &str, reason: String, elapsed: f64) -> CenterRecord {
    CenterRecord {
        center_id: id.into(),
        center_contract: contract.into(),
        center_availability: "unavailable".into(),
        center_unavailable_reason: Some(reason),
        center_beta_f64: None,
        center_mu_f64: None,
        center_xi_f64: None,
        center_q_raw_f64: None,
        center_q_constraint_residual_correction_f64: None,
        center_q_corrected_f64: None,
        center_action_from_positive_q_f64: None,
        center_beta_margin_f64: None,
        center_full_kkt_residual_norm_f64: None,
        center_stationarity_residual_norm_f64: None,
        center_closure_residual_norm_f64: None,
        center_normalization_residual_abs_f64: None,
        center_rank_f64: None,
        center_timing_us: elapsed,
    }
}

fn observe(case: &Case, sigma: &[usize]) -> RawRow {
    let qp = build_qp_from_dual_vertices(&case.dual_f64, sigma);
    let (m, b) = build_augmented_system_from_dual_vertices(&case.dual_f64, sigma);
    let svd = m.clone().svd(true, true);
    let singular_values = svd.singular_values.iter().copied().collect::<Vec<_>>();
    let max_sv = singular_values.iter().copied().fold(0.0, f64::max);
    let rank_tol = (max_sv * 1e-12).max(1e-14);
    let rank = singular_values.iter().filter(|x| **x > rank_tol).count();
    let eigenvalues = m
        .clone()
        .symmetric_eigen()
        .eigenvalues
        .iter()
        .copied()
        .collect::<Vec<_>>();
    let cond = singular_values
        .last()
        .filter(|x| **x > 0.0)
        .map(|x| max_sv / x);
    let mut timings = BTreeMap::new();
    let mut centers = Vec::new();
    let start = Instant::now();
    let saddle = solve_kkt_for_dual_vertices(&case.dual_f64, sigma);
    let saddle_us = start.elapsed().as_secs_f64() * 1e6;
    timings.insert("saddle_eig_accepted_us".into(), saddle_us);
    let (outcome, retained, beta_predicate, saddle_center) = match saddle {
        KktOutcome::Feasible(k) => {
            let mut x = DVector::zeros(sigma.len() + 5);
            for i in 0..sigma.len() {
                x[i] = k.beta[i];
            }
            for i in 0..4 {
                x[sigma.len() + i] = k.mu[i];
            }
            x[sigma.len() + 4] = k.xi;
            let mut c = center_from_solution("saddle_eig_accepted", "current production saddle/eigendecomposition accepted center; internal tier not exposed", Some(x), &qp.h, &m, &b, sigma.len(), Some(rank), saddle_us);
            c.center_q_corrected_f64 = Some(k.q_corrected);
            c.center_q_constraint_residual_correction_f64 = Some(k.q_correction);
            c.center_action_from_positive_q_f64 =
                (k.q_corrected > 0.0).then(|| 0.5 / k.q_corrected);
            let margin = k.beta.iter().copied().fold(f64::INFINITY, f64::min);
            let predicate = if margin > 1e-9 {
                "true"
            } else if margin < -1e-9 {
                "false"
            } else {
                "indeterminate"
            };
            ("feasible".into(), true, predicate.into(), c)
        }
        KktOutcome::Infeasible => (
            "infeasible".into(),
            false,
            "unavailable".into(),
            unavailable_center(
                "saddle_eig_accepted",
                "current production saddle/eigendecomposition accepted center",
                "production solver reported infeasible".into(),
                saddle_us,
            ),
        ),
        KktOutcome::SingularMatrix => (
            "singular_matrix".into(),
            false,
            "unavailable".into(),
            unavailable_center(
                "saddle_eig_accepted",
                "current production saddle/eigendecomposition accepted center",
                "production solver reported singular matrix".into(),
                saddle_us,
            ),
        ),
        KktOutcome::TypeCViolation => (
            "type_c_violation".into(),
            false,
            "unavailable".into(),
            unavailable_center(
                "saddle_eig_accepted",
                "current production saddle/eigendecomposition accepted center",
                "production solver reported Type-C violation".into(),
                saddle_us,
            ),
        ),
        KktOutcome::ConstraintViolation => (
            "constraint_violation".into(),
            false,
            "unavailable".into(),
            unavailable_center(
                "saddle_eig_accepted",
                "current production saddle/eigendecomposition accepted center",
                "production solver reported constraint violation".into(),
                saddle_us,
            ),
        ),
    };
    centers.push(saddle_center);
    let start = Instant::now();
    let svd_x = match (svd.u.as_ref(), svd.v_t.as_ref()) {
        (Some(u), Some(vt)) => {
            let mut x = DVector::zeros(m.ncols());
            for i in 0..rank {
                x += vt.row(i).transpose() * (u.column(i).dot(&b) / svd.singular_values[i]);
            }
            Some(x)
        }
        _ => None,
    };
    let svd_us = start.elapsed().as_secs_f64() * 1e6;
    timings.insert("svd_lstsq_proposal_us".into(), svd_us);
    centers.push(center_from_solution(
        "svd_lstsq_proposal",
        "unconditional truncated-SVD least-squares proposal",
        svd_x.clone(),
        &qp.h,
        &m,
        &b,
        sigma.len(),
        Some(rank),
        svd_us,
    ));
    let (critical, maxmargin) = projection_centers(&qp);
    timings.insert(
        "projected_critical_proposal_us".into(),
        critical.center_timing_us,
    );
    timings.insert(
        "projected_max_margin_proposal_us".into(),
        maxmargin.center_timing_us,
    );
    centers.push(critical);
    centers.push(maxmargin);
    let start = Instant::now();
    let lu_x = m.clone().lu().solve(&b);
    let lu_us = start.elapsed().as_secs_f64() * 1e6;
    timings.insert("lu_partial_pivot_proposal_us".into(), lu_us);
    centers.push(center_from_solution(
        "lu_partial_pivot_proposal",
        "nalgebra LU full-system solve; unavailable on singular factorization",
        lu_x,
        &qp.h,
        &m,
        &b,
        sigma.len(),
        Some(rank),
        lu_us,
    ));
    let start = Instant::now();
    let qr_x = m.clone().qr().solve(&b);
    let qr_us = start.elapsed().as_secs_f64() * 1e6;
    timings.insert("qr_proposal_us".into(), qr_us);
    centers.push(center_from_solution(
        "qr_proposal",
        "nalgebra QR full-system solve; proposal only",
        qr_x.clone(),
        &qp.h,
        &m,
        &b,
        sigma.len(),
        Some(rank),
        qr_us,
    ));
    let start = Instant::now();
    let refined = match (svd_x, qr_x) {
        (Some(x), Some(_)) => m.clone().qr().solve(&(&b - &m * &x)).map(|d| x + d),
        _ => None,
    };
    let refined_us = start.elapsed().as_secs_f64() * 1e6;
    timings.insert("refined_svd_lstsq_qr_proposal_us".into(), refined_us);
    centers.push(center_from_solution(
        "refined_svd_lstsq_proposal_qr_correction",
        "one QR correction of the SVD least-squares proposal",
        refined,
        &qp.h,
        &m,
        &b,
        sigma.len(),
        Some(rank),
        refined_us,
    ));
    let exact_start = Instant::now();
    let (exact_m, exact_b) = exact_kkt(&case.dual_exact, sigma);
    let linear = solve_linear_system(&exact_m, &exact_b);
    let exact = solve_kkt_exact(&case.dual_exact, sigma);
    let exact_us = exact_start.elapsed().as_secs_f64() * 1e6;
    timings.insert(
        "exact_row_reduction_and_positive_witness_us".into(),
        exact_us,
    );
    let (system_status, erank, enull, particular) = match linear {
        LinearSystemSolution::Inconsistent => (
            "inconsistent".into(),
            Some(exact_rank(&exact_m)),
            Some(exact_m.nrows() - exact_rank(&exact_m)),
            None,
        ),
        LinearSystemSolution::Consistent {
            particular,
            kernel_basis,
        } => {
            let null = kernel_basis.ncols();
            (
                if null == 0 {
                    "consistent_unique"
                } else {
                    "consistent_rank_deficient"
                }
                .into(),
                Some(exact_m.nrows() - null),
                Some(null),
                Some(particular.iter().take(sigma.len()).map(rat).collect()),
            )
        }
    };
    let (pstatus, preason, pbeta, pq, paction) = match exact {
        Some(x) if x.q_exact.is_positive() => {
            let q = x.q_exact;
            let action = rat(&(BigRational::one() / (q.clone() + q.clone())));
            (
                "exists".into(),
                "exact rational solver found a strict-positive beta and positive-Q witness".into(),
                Some(x.beta.iter().map(rat).collect()),
                Some(q),
                Some(action),
            )
        }
        Some(x) => (
            "exists_q_nonpositive".into(),
            "exact rational solver found a strict-positive beta witness, but its Q is nonpositive so action is unavailable".into(),
            Some(x.beta.iter().map(rat).collect()),
            Some(x.q_exact),
            None,
        ),
        None => (
            "none_or_q_nonpositive_conflated".into(),
            "exact API returned no positive witness; this conflates no strict-positive beta with any unexposed remaining exact distinction".into(),
            None,
            None,
            None,
        ),
    };
    let qsign = classify_exact(pq.as_ref());
    let availability = if paction.is_some() {
        "available"
    } else {
        "unavailable"
    }
    .into();
    let exact_c = (0..5)
        .map(|i| {
            (0..sigma.len())
                .map(|j| {
                    if i < 4 {
                        rat(&case.dual_exact[sigma[j]][i])
                    } else {
                        "1/1".into()
                    }
                })
                .collect()
        })
        .collect();
    let exact_rhs = exact_b.iter().map(rat).collect();
    RawRow {
        run_id: RUN_ID.into(),
        schema_version: SCHEMA.into(),
        target_polytope_id: case.case_id.into(),
        target_source_id: case.source_id.clone(),
        target_input_kind: case.target_kind.into(),
        intended_algebraic_target_status: case.intended_algebraic_status.into(),
        supplied_stream_completeness: case.completeness.into(),
        sigma_active_reeb_word: sigma.to_vec(),
        sigma_length: sigma.len(),
        lifecycle_transition_source: "declared fixture transition stream or named regression word"
            .into(),
        lifecycle_route_visited: true,
        lifecycle_route_disposition: if retained {
            "retained_by_saddle_f64"
        } else {
            "not_retained_by_saddle_f64"
        }
        .into(),
        saddle_eig_actual_tier_status:
            "unavailable: production public KktOutcome does not expose permissive/strict tier"
                .into(),
        saddle_eig_solver_outcome: outcome,
        saddle_eig_beta_predicate: beta_predicate,
        f64_retained_by_saddle: retained,
        qp_constraint_matrix_c_f64: matrix_rows(&qp.c),
        qp_objective_hessian_h_f64: matrix_rows(&qp.h),
        kkt_augmented_matrix_m_f64: matrix_rows(&m),
        kkt_augmented_rhs_b_f64: vector_rows(&b),
        qp_constraint_matrix_c_exact: exact_c,
        qp_objective_hessian_h_exact: (0..sigma.len())
            .map(|i| (0..sigma.len()).map(|j| rat(&exact_m[(i, j)])).collect())
            .collect(),
        kkt_augmented_matrix_m_exact: exact_matrix_rows(&exact_m),
        kkt_augmented_rhs_b_exact: exact_rhs,
        kkt_augmented_singular_values_f64: singular_values,
        kkt_augmented_eigenvalues_f64: eigenvalues,
        kkt_augmented_condition_number_f64: cond,
        centers,
        exact_row_reduction_system_status: system_status,
        exact_row_reduction_rank: erank,
        exact_row_reduction_nullity: enull,
        exact_row_reduction_beta_particular: particular,
        exact_positive_witness_status: pstatus,
        exact_positive_witness_reason: preason,
        exact_positive_witness_beta: pbeta,
        exact_positive_witness_q: pq.as_ref().map(rat),
        exact_positive_witness_q_sign: qsign,
        exact_positive_witness_action: paction,
        exact_action_availability: availability,
        stage_timings_us: timings,
    }
}

fn known_case(kp: &KnownPolytope, id: &'static str, cohort: &'static str, cap: usize) -> Case {
    let t = build_transition_matrix_from_facet_intersections_and_omega(
        &kp.facet_intersection_is_nonempty,
        &kp.omega_signs,
    );
    Case {
        case_id: id,
        cohort,
        source_id: kp.name.into(),
        completeness: if cap == usize::MAX {
            "transition_complete"
        } else {
            "transition_stream_capped"
        },
        target_kind: "original_rational",
        intended_algebraic_status: "available for this rational fixture",
        dual_f64: kp.dual_vertices_f64.clone(),
        dual_exact: kp.dual_vertices.clone(),
        sigmas: SimpleDirectedCyclesCanonical::new(&t).take(cap).collect(),
    }
}
fn generated_case() -> Case {
    let c = generated_f64_cases_with_source_filter(
        1,
        99_540_836,
        &["seed99540836:F5:sample0:attempt5000000008".into()],
    )
    .pop()
    .expect("generated F5 fixture");
    let exact = exact_binary64_dual_vertex_arrays(&c.dual_vertices);
    let t = exact_binary64_transition_matrix_assuming_origin_interior(&exact);
    Case {
        case_id: "ordinary_generated_F5",
        cohort: "ordinary_control",
        source_id: c.source_id,
        completeness: "transition_complete",
        target_kind: "stored_binary64_rational",
        intended_algebraic_status: "unavailable: generator original algebraic target not retained",
        dual_f64: c.dual_vertices,
        dual_exact: exact,
        sigmas: SimpleDirectedCyclesCanonical::new(&t).collect(),
    }
}
fn pinned_q4_p5_case() -> Case {
    let c = generated_f64_cases_with_source_filter(
        1,
        99_540_836,
        &["seed99540836:q4:p5:attempt405000000000".into()],
    )
    .pop()
    .expect("pinned q4:p5 fixture");
    let exact = exact_binary64_dual_vertex_arrays(&c.dual_vertices);
    let t = exact_binary64_transition_matrix_assuming_origin_interior(&exact);
    let sigmas = SimpleDirectedCyclesCanonical::new(&t).collect::<Vec<_>>();
    Case {
        case_id: "pinned_q4_p5",
        cohort: "pinned_transition_stream",
        source_id: c.source_id,
        completeness: "transition_complete_exact_binary64_stream",
        target_kind: "stored_binary64_rational",
        intended_algebraic_status: "unavailable: generator original algebraic target not retained",
        dual_f64: c.dual_vertices,
        dual_exact: exact,
        sigmas,
    }
}
fn hko_case(id: &'static str, sigma: Vec<usize>) -> Case {
    let kp = known_polytopes::hko_pentagon();
    Case {
        case_id: id,
        cohort: "post_selected_hko_regression",
        source_id: kp.name.into(),
        completeness: "matched_named_context_only",
        target_kind: "stored_binary64_rational",
        intended_algebraic_status:
            "unavailable: stored-binary64 rational target only; no algebraic transfer",
        dual_f64: kp.dual_vertices_f64.clone(),
        dual_exact: exact_binary64_dual_vertex_arrays(&kp.dual_vertices_f64),
        sigmas: vec![sigma],
    }
}
fn cases() -> Vec<Case> {
    let mut out = vec![
        known_case(
            known_polytopes::simplex(),
            "simplex_F5",
            "ordinary_control",
            MAX_SMALL_STREAM,
        ),
        known_case(
            known_polytopes::hypercube(),
            "hypercube_F8",
            "ordinary_control",
            MAX_SMALL_STREAM,
        ),
        known_case(
            known_polytopes::lagrangian_triangle_square(),
            "triangle_times_square_tie",
            "product_tie",
            MAX_SMALL_STREAM,
        ),
        generated_case(),
        pinned_q4_p5_case(),
    ];
    out.extend([
        hko_case("hko_beta_boundary", vec![0, 1, 6, 7, 3, 4, 5, 9]),
        hko_case(
            "hko_near_singular_false_acceptance",
            vec![1, 8, 7, 3, 4, 5, 9],
        ),
        hko_case("hko_residual_q_failure", vec![0, 1, 7, 3, 9, 5]),
        hko_case("hko_rank_deficient", vec![1, 7, 2, 8, 4, 6, 5]),
    ]);
    out.push(Case {
        case_id: "hypercube_exact_zero_beta_boundary",
        cohort: "boundary_regression",
        source_id: "hypercube".into(),
        completeness: "matched_named_context_only",
        target_kind: "original_rational",
        intended_algebraic_status: "available for this rational fixture",
        dual_f64: known_polytopes::hypercube().dual_vertices_f64.clone(),
        dual_exact: known_polytopes::hypercube().dual_vertices.clone(),
        sigmas: vec![vec![0, 2, 1, 5, 6]],
    });
    out
}

fn formula_registry() -> Vec<FormulaRegistryEntry> {
    let centers = [
        "saddle_eig_accepted",
        "svd_lstsq_proposal",
        "projected_critical_proposal",
        "projected_max_margin_proposal",
        "lu_partial_pivot_proposal",
        "qr_proposal",
        "refined_svd_lstsq_proposal_qr_correction",
    ];
    let mut entries = Vec::new();
    for center in centers {
        let c = center.to_string();
        let exact_q = "exact_positive_witness_q".to_string();
        entries.push(FormulaRegistryEntry {
            formula_id: format!("error_q_abs__{center}_q_raw__to_exact_positive_witness_q"),
            output_column: format!("error_q_abs__{center}_q_raw__to_exact_positive_witness_q"),
            dependencies: vec![
                (format!("{center}.center_q_raw_f64"), c.clone()),
                (exact_q.clone(), "exact_positive_witness".into()),
            ],
            center: c.clone(),
            exact_target: exact_q.clone(),
            hypothesis_status: "observational error only".into(),
            status: "available_when_matching_exact_positive_witness_exists".into(),
            consumers: vec!["soundness analyzer".into()],
            unavailable_rule: "unavailable without a same-sigma exact positive-Q witness".into(),
        });
        entries.push(FormulaRegistryEntry {
            formula_id: format!("bound_q_abs__{center}__eigen_residual_9over2"),
            output_column: format!("bound_q_abs__{center}__eigen_residual_9over2"),
            dependencies: vec![
                (
                    format!("{center}.center_full_kkt_residual_norm_f64"),
                    c.clone(),
                ),
                ("kkt_augmented_eigenvalues_f64".into(), c.clone()),
            ],
            center: c.clone(),
            exact_target: exact_q.clone(),
            hypothesis_status: "conjectured; ordinary f64 atoms are not verified enclosures".into(),
            status: "available_when_full_residual_and_nonzero_eigenvalue_exist".into(),
            consumers: vec!["soundness analyzer".into()],
            unavailable_rule: "unavailable for projected centres without a full KKT residual"
                .into(),
        });
        entries.push(FormulaRegistryEntry {
            formula_id: format!("bound_beta_l2__{center}__inverse_singular_residual"),
            output_column: format!("bound_beta_l2__{center}__inverse_singular_residual"),
            dependencies: vec![
                (
                    format!("{center}.center_full_kkt_residual_norm_f64"),
                    c.clone(),
                ),
                ("kkt_augmented_singular_values_f64".into(), c.clone()),
            ],
            center: c.clone(),
            exact_target: "exact_positive_witness_beta".into(),
            hypothesis_status: "heuristic inverse-norm diagnostic, not a verified beta enclosure"
                .into(),
            status: "available_when_full_residual_and_positive_singular_value_exist".into(),
            consumers: vec!["soundness analyzer".into()],
            unavailable_rule: "unavailable for projected centres without a full KKT residual"
                .into(),
        });
        entries.push(FormulaRegistryEntry {
            formula_id: format!("bound_q_abs__{center}__beta_radius_first_plus_quadratic"),
            output_column: format!("bound_q_abs__{center}__beta_radius_first_plus_quadratic"),
            dependencies: vec![
                (
                    format!("bound_beta_l2__{center}__inverse_singular_residual"),
                    c.clone(),
                ),
                ("qp_objective_hessian_h_f64".into(), c.clone()),
                (format!("{center}.center_beta_f64"), c.clone()),
            ],
            center: c.clone(),
            exact_target: exact_q.clone(),
            hypothesis_status: "heuristic Q propagation from the inverse-norm radius".into(),
            status: "available_when_beta-radius-is-computable".into(),
            consumers: vec!["soundness analyzer".into()],
            unavailable_rule: "unavailable when the radius or centre beta is unavailable".into(),
        });
        entries.push(FormulaRegistryEntry { formula_id: format!("interval_action__{center}__positive_q_monotone"), output_column: format!("interval_action__{center}__positive_q_monotone"), dependencies: vec![(format!("{center}.center_q_corrected_f64"), c.clone()), (format!("bound_q_abs__{center}__eigen_residual_9over2"), c.clone())], center:c.clone(), exact_target:"exact_positive_witness_action".into(), hypothesis_status:"conditionally justified only if the Q bound is justified and has positive lower endpoint".into(), status:"candidate_interval_not_verified".into(), consumers:vec!["selective fallback design".into()], unavailable_rule:"unavailable without corrected Q, candidate Q bound, and positive lower endpoint".into() });
        entries.push(FormulaRegistryEntry {
            formula_id: format!("predicate_beta_positive__{center}__from_radius"),
            output_column: format!("predicate_beta_positive__{center}__from_radius"),
            dependencies: vec![
                (format!("{center}.center_beta_margin_f64"), c.clone()),
                (
                    format!("bound_beta_l2__{center}__inverse_singular_residual"),
                    c.clone(),
                ),
            ],
            center: c.clone(),
            exact_target: "exact_positive_witness_status".into(),
            hypothesis_status: "heuristic ternary predicate from f64 margin minus heuristic radius"
                .into(),
            status: "available_when_margin_and_radius_exist".into(),
            consumers: vec!["policy comparison".into()],
            unavailable_rule: "unavailable when either atom is unavailable".into(),
        });
    }
    entries
}

fn exact_action(row: &RawRow) -> Option<BigRational> {
    row.exact_positive_witness_action
        .as_deref()
        .and_then(|s| s.split_once('/'))
        .and_then(|(n, d)| Some(BigRational::new(n.parse().ok()?, d.parse().ok()?)))
}
fn saddle_action(row: &RawRow) -> Option<f64> {
    row.centers
        .iter()
        .find(|c| c.center_id == "saddle_eig_accepted")
        .and_then(|c| c.center_action_from_positive_q_f64)
}
fn policy_rows(rows: &[RawRow]) -> Vec<PolicyRow> {
    let mut by: BTreeMap<&str, Vec<&RawRow>> = BTreeMap::new();
    for r in rows {
        by.entry(&r.target_polytope_id).or_default().push(r);
    }
    let mut out = Vec::new();
    for (case, rs) in by {
        let all: Vec<&RawRow> = rs
            .iter()
            .copied()
            .filter(|r| exact_action(r).is_some())
            .collect();
        let retained: Vec<&RawRow> = rs
            .iter()
            .copied()
            .filter(|r| r.f64_retained_by_saddle && exact_action(r).is_some())
            .collect();
        for (id, desc, scope, set, trigger) in [
            (
                "minimasafe_heuristic",
                "current f64-True saddle scalar only; unchecked f64-True is heuristic",
                "unknown",
                rs.iter()
                    .copied()
                    .filter(|r| r.saddle_eig_beta_predicate == "true" && saddle_action(r).is_some())
                    .collect(),
                "no exact fallback",
            ),
            (
                "exact_every_f64_retained",
                "exactly resolve every f64-retained saddle candidate",
                "retained set",
                retained.clone(),
                "every retained candidate",
            ),
            (
                "exact_every_supplied_sigma",
                "exactly resolve every supplied stream word",
                "supplied stream",
                all.clone(),
                "every supplied sigma",
            ),
        ] {
            for gap in RELATIVE_WINDOWS {
                let start = Instant::now();
                let min = set.iter().filter_map(|r| exact_action(r)).min();
                let cutoff = min.as_ref().map(|m| {
                    m.clone() * (BigRational::one() + BigRational::from_float(gap).unwrap())
                });
                let mins = min
                    .as_ref()
                    .map(|m| {
                        set.iter()
                            .filter(|r| exact_action(r).as_ref() == Some(m))
                            .map(|r| r.sigma_active_reeb_word.clone())
                            .collect()
                    })
                    .unwrap_or_default();
                let window = cutoff
                    .as_ref()
                    .map(|c| {
                        set.iter()
                            .filter(|r| exact_action(r).as_ref().is_some_and(|a| a <= c))
                            .map(|r| r.sigma_active_reeb_word.clone())
                            .collect()
                    })
                    .unwrap_or_default();
                out.push(PolicyRow {
                    schema_version: POLICY_SCHEMA.into(),
                    target_polytope_id: case.into(),
                    policy_id: id.into(),
                    policy_description: desc.into(),
                    exactness_scope: scope.into(),
                    requested_window_kind: "relative_to_policy_exact_minimum".into(),
                    requested_relative_gap: gap,
                    supplied_stream_count: rs.len(),
                    policy_candidate_count: set.len(),
                    policy_exact_resolution_count: if id == "minimasafe_heuristic" {
                        0
                    } else {
                        set.len()
                    },
                    policy_exact_accept_count: set.len(),
                    policy_min_action: min.as_ref().map(rat),
                    policy_minimizer_active_words: mins,
                    policy_window_active_words: window,
                    policy_window_cutoff: cutoff.as_ref().map(rat),
                    policy_fallback_trigger: trigger.into(),
                    policy_fallback_result: if min.is_some() {
                        "policy set had an exact positive-Q action".into()
                    } else {
                        "no exact positive-Q action in policy set".into()
                    },
                    policy_stage_timing_us: start.elapsed().as_secs_f64() * 1e6,
                });
            }
        }
        let f64_retained: Vec<&RawRow> = rs
            .iter()
            .copied()
            .filter(|r| r.f64_retained_by_saddle)
            .collect();
        let f64_anchor = f64_retained
            .iter()
            .filter_map(|r| saddle_action(r))
            .min_by(|a, b| a.total_cmp(b));
        for gap in RELATIVE_WINDOWS {
            let start = Instant::now();
            let selected: Vec<&RawRow> = match f64_anchor {
                Some(a) => f64_retained
                    .iter()
                    .copied()
                    .filter(|r| saddle_action(r).is_some_and(|x| x <= a * (1.0 + gap)))
                    .collect(),
                None => vec![],
            };
            let accepted: Vec<&RawRow> = selected
                .iter()
                .copied()
                .filter(|r| exact_action(r).is_some())
                .collect();
            let min = accepted.iter().filter_map(|r| exact_action(r)).min();
            let cutoff = min
                .as_ref()
                .map(|m| m.clone() * (BigRational::one() + BigRational::from_float(gap).unwrap()));
            out.push(PolicyRow{schema_version:POLICY_SCHEMA.into(),target_polytope_id:case.into(),policy_id:"selective_fallback_f64_anchored_window".into(),policy_description:"one-shot anchored rule: retain saddle candidates whose f64 action is at most f64 minimum times (1+relative gap), then exact-resolve that selected set; no iterative expansion".into(),exactness_scope:"selected f64-anchored window only".into(),requested_window_kind:"relative_f64_anchor_then_exact_report".into(),requested_relative_gap:gap,supplied_stream_count:rs.len(),policy_candidate_count:selected.len(),policy_exact_resolution_count:selected.len(),policy_exact_accept_count:accepted.len(),policy_min_action:min.as_ref().map(rat),policy_minimizer_active_words:min.as_ref().map(|m|accepted.iter().filter(|r|exact_action(r).as_ref()==Some(m)).map(|r|r.sigma_active_reeb_word.clone()).collect()).unwrap_or_default(),policy_window_active_words:cutoff.as_ref().map(|c|accepted.iter().filter(|r|exact_action(r).as_ref().is_some_and(|a|a<=c)).map(|r|r.sigma_active_reeb_word.clone()).collect()).unwrap_or_default(),policy_window_cutoff:cutoff.as_ref().map(rat),policy_fallback_trigger:"f64 saddle action overlaps one-shot anchored relative window".into(),policy_fallback_result:"exactly evaluated selected candidates".into(),policy_stage_timing_us:start.elapsed().as_secs_f64()*1e6});
        }
    }
    out
}

fn main() {
    let out = env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("artifacts/soundness-v2"));
    create_dir_all(&out).expect("create output");
    let start = Instant::now();
    let mut raw = Vec::new();
    for case in cases() {
        for sigma in &case.sigmas {
            raw.push(observe(&case, sigma));
        }
    }
    let policies = policy_rows(&raw);
    let mut w = BufWriter::new(File::create(out.join("raw_rows.jsonl")).expect("raw"));
    for row in &raw {
        serde_json::to_writer(&mut w, row).expect("raw json");
        w.write_all(b"\n").unwrap();
    }
    let mut w = BufWriter::new(File::create(out.join("policy_rows.jsonl")).expect("policy"));
    for row in &policies {
        serde_json::to_writer(&mut w, row).expect("policy json");
        w.write_all(b"\n").unwrap();
    }
    serde_json::to_writer_pretty(
        File::create(out.join("formula_registry.json")).unwrap(),
        &formula_registry(),
    )
    .unwrap();
    serde_json::to_writer_pretty(File::create(out.join("producer_summary.json")).unwrap(),&serde_json::json!({"run_id":RUN_ID,"schema_version":SCHEMA,"row_count":raw.len(),"policy_row_count":policies.len(),"elapsed_seconds":start.elapsed().as_secs_f64(),"artifact_contract":"raw and policy JSONL are generated; formulas are a registry, not a theorem claim"})).unwrap();
}

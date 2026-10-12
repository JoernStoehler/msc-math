use crate::geometry::{
    f64_combinatorics_profiled, f64_combinatorics_with_lp_transitions_profiled, F64Predicate,
};
use crate::validation::{validate_f64_polytope_input_with_policy_profiled, F64ValidationPolicy};
use good_lp::{constraint, default_solver, variable, variables, Expression, Solution, SolverModel};
use nalgebra::Vector4;
use serde::Serialize;
use std::time::Instant;

const EPS_LP_MARGIN: f64 = 1e-10;

#[derive(Clone, Debug, Serialize)]
pub struct DecisionComparisonReport {
    pub origin: DecisionComparisonRow,
    pub facet_presence_vertex_vs_per_facet_lp: DecisionComparisonRow,
    pub facet_presence_per_facet_lp_vs_batched_primal_lp: DecisionComparisonRow,
    pub facet_presence_per_facet_lp_vs_batched_polar_lp: DecisionComparisonRow,
    pub facet_pair_intersection: DecisionComparisonRow,
    pub omega_sign: SingleMethodDecisionRow,
}

#[derive(Clone, Debug, Serialize)]
pub struct DecisionComparisonRow {
    pub decision: &'static str,
    pub left_method: &'static str,
    pub right_method: &'static str,
    pub left_time_ms: f64,
    pub right_time_ms: f64,
    pub left_true_count: usize,
    pub left_false_count: usize,
    pub left_indeterminate_count: usize,
    pub left_error_count: usize,
    pub right_true_count: usize,
    pub right_false_count: usize,
    pub right_indeterminate_count: usize,
    pub right_error_count: usize,
    pub agreement_count: usize,
    pub disagreement_count: usize,
    pub left_indeterminate_right_decisive_count: usize,
    pub left_decisive_right_indeterminate_count: usize,
}

#[derive(Clone, Debug, Serialize)]
pub struct SingleMethodDecisionRow {
    pub decision: &'static str,
    pub method: &'static str,
    pub time_ms: f64,
    pub positive_count: usize,
    pub negative_count: usize,
    pub zero_count: usize,
    pub indeterminate_count: usize,
}

pub fn compare_f64_decisions(dual_vertices: &[Vector4<f64>]) -> DecisionComparisonReport {
    let (strict_validation, strict_validation_timing) =
        validate_f64_polytope_input_with_policy_profiled(
            dual_vertices,
            F64ValidationPolicy::Strict,
        );
    let (lp_validation, lp_validation_timing) =
        validate_f64_polytope_input_with_policy_profiled(dual_vertices, F64ValidationPolicy::Lp);
    let strict_combinatorics = f64_combinatorics_profiled(dual_vertices);
    let lp_combinatorics = f64_combinatorics_with_lp_transitions_profiled(dual_vertices);

    let origin = DecisionComparisonRow::from_statuses(
        "origin_in_interior",
        "strict_origin_predicate",
        "lp_origin",
        strict_validation_timing.origin_policy_predicate_ms,
        lp_validation_timing.origin_lp_diagnostic_ms
            + lp_validation_timing.origin_policy_predicate_ms,
        &[strict_validation.origin_status.label()],
        &[lp_validation.origin_status.label()],
    );

    let facet_presence_vertex_vs_per_facet_lp = match (&strict_combinatorics, &lp_combinatorics) {
        (Ok((strict, strict_timing)), Ok((lp, lp_timing))) => {
            DecisionComparisonRow::from_predicates(
                "facet_presence",
                "vertex_scan_coverage",
                "lp_facet_exists",
                strict_timing.vertex_scan_ms + strict_timing.facet_coverage_ms,
                lp_timing.lp_facet_statuses_ms,
                &strict.facet_statuses,
                &lp.facet_statuses,
            )
        }
        _ => DecisionComparisonRow::error(
            "facet_presence",
            "vertex_scan_coverage",
            "lp_facet_exists",
        ),
    };
    let per_facet_lp_statuses = lp_combinatorics
        .as_ref()
        .ok()
        .map(|(combinatorics, timing)| {
            (
                combinatorics.facet_statuses.clone(),
                timing.lp_facet_statuses_ms,
            )
        });
    let batched_primal = batched_primal_facet_presence(dual_vertices);
    let batched_polar = batched_polar_facet_presence(dual_vertices);

    let facet_presence_per_facet_lp_vs_batched_primal_lp =
        match (&per_facet_lp_statuses, &batched_primal) {
            (Some((per_facet, per_facet_time_ms)), Ok((batched, batched_time_ms))) => {
                DecisionComparisonRow::from_predicates(
                    "facet_presence",
                    "lp_facet_exists",
                    "batched_primal_margin_lp",
                    *per_facet_time_ms,
                    *batched_time_ms,
                    per_facet,
                    batched,
                )
            }
            _ => DecisionComparisonRow::error(
                "facet_presence",
                "lp_facet_exists",
                "batched_primal_margin_lp",
            ),
        };

    let facet_presence_per_facet_lp_vs_batched_polar_lp =
        match (&per_facet_lp_statuses, &batched_polar) {
            (Some((per_facet, per_facet_time_ms)), Ok((batched, batched_time_ms))) => {
                DecisionComparisonRow::from_predicates(
                    "facet_presence",
                    "lp_facet_exists",
                    "batched_polar_redundancy_lp",
                    *per_facet_time_ms,
                    *batched_time_ms,
                    per_facet,
                    batched,
                )
            }
            _ => DecisionComparisonRow::error(
                "facet_presence",
                "lp_facet_exists",
                "batched_polar_redundancy_lp",
            ),
        };

    let facet_pair_intersection = match (&strict_combinatorics, &lp_combinatorics) {
        (Ok((strict, strict_timing)), Ok((lp, lp_timing))) => {
            DecisionComparisonRow::from_predicates(
                "facet_pair_intersection",
                "vertex_incidence_pairs",
                "lp_facet_pair_intersects",
                strict_timing.vertex_scan_ms + strict_timing.facet_intersections_ms,
                lp_timing.lp_facet_intersections_ms,
                &upper_triangle(&strict.facet_intersections),
                &upper_triangle(&lp.facet_intersections),
            )
        }
        _ => DecisionComparisonRow::error(
            "facet_pair_intersection",
            "vertex_incidence_pairs",
            "lp_facet_pair_intersects",
        ),
    };

    let omega_sign = match lp_combinatorics {
        Ok((lp, lp_timing)) => {
            let mut positive_count = 0usize;
            let mut negative_count = 0usize;
            let mut zero_count = 0usize;
            for i in 0..lp.omega_signs.nrows() {
                for j in i + 1..lp.omega_signs.ncols() {
                    match lp.omega_signs[(i, j)] {
                        value if value > 0 => positive_count += 1,
                        value if value < 0 => negative_count += 1,
                        _ => zero_count += 1,
                    }
                }
            }
            SingleMethodDecisionRow {
                decision: "omega_sign",
                method: "f64_omega",
                time_ms: lp_timing.lp_omega_recompute_ms,
                positive_count,
                negative_count,
                zero_count,
                indeterminate_count: lp.omega_indeterminate_count,
            }
        }
        Err(()) => SingleMethodDecisionRow {
            decision: "omega_sign",
            method: "f64_omega",
            time_ms: 0.0,
            positive_count: 0,
            negative_count: 0,
            zero_count: 0,
            indeterminate_count: 0,
        },
    };

    DecisionComparisonReport {
        origin,
        facet_presence_vertex_vs_per_facet_lp,
        facet_presence_per_facet_lp_vs_batched_primal_lp,
        facet_presence_per_facet_lp_vs_batched_polar_lp,
        facet_pair_intersection,
        omega_sign,
    }
}

impl DecisionComparisonRow {
    fn from_predicates(
        decision: &'static str,
        left_method: &'static str,
        right_method: &'static str,
        left_time_ms: f64,
        right_time_ms: f64,
        left: &[F64Predicate],
        right: &[F64Predicate],
    ) -> Self {
        let left = left.iter().map(predicate_label).collect::<Vec<_>>();
        let right = right.iter().map(predicate_label).collect::<Vec<_>>();
        Self::from_statuses(
            decision,
            left_method,
            right_method,
            left_time_ms,
            right_time_ms,
            &left,
            &right,
        )
    }

    fn from_statuses(
        decision: &'static str,
        left_method: &'static str,
        right_method: &'static str,
        left_time_ms: f64,
        right_time_ms: f64,
        left: &[&str],
        right: &[&str],
    ) -> Self {
        let mut row = Self {
            decision,
            left_method,
            right_method,
            left_time_ms,
            right_time_ms,
            left_true_count: count_label(left, "true"),
            left_false_count: count_label(left, "false"),
            left_indeterminate_count: count_label(left, "indeterminate"),
            left_error_count: count_label(left, "error"),
            right_true_count: count_label(right, "true"),
            right_false_count: count_label(right, "false"),
            right_indeterminate_count: count_label(right, "indeterminate"),
            right_error_count: count_label(right, "error"),
            agreement_count: 0,
            disagreement_count: 0,
            left_indeterminate_right_decisive_count: 0,
            left_decisive_right_indeterminate_count: 0,
        };
        for (left, right) in left.iter().zip(right) {
            if left == right {
                row.agreement_count += 1;
            } else {
                row.disagreement_count += 1;
            }
            if *left == "indeterminate" && (*right == "true" || *right == "false") {
                row.left_indeterminate_right_decisive_count += 1;
            }
            if (*left == "true" || *left == "false") && *right == "indeterminate" {
                row.left_decisive_right_indeterminate_count += 1;
            }
        }
        row
    }

    fn error(
        decision: &'static str,
        left_method: &'static str,
        right_method: &'static str,
    ) -> Self {
        Self::from_statuses(
            decision,
            left_method,
            right_method,
            0.0,
            0.0,
            &["error"],
            &["error"],
        )
    }
}

fn batched_primal_facet_presence(
    dual_vertices: &[Vector4<f64>],
) -> Result<(Vec<F64Predicate>, f64), ()> {
    let started = Instant::now();
    let facet_count = dual_vertices.len();
    if facet_count < 5 {
        return Err(());
    }
    let mut vars = variables!();
    let points = (0..facet_count)
        .map(|_| {
            [
                vars.add(variable()),
                vars.add(variable()),
                vars.add(variable()),
                vars.add(variable()),
            ]
        })
        .collect::<Vec<_>>();
    let margins = (0..facet_count)
        .map(|_| vars.add(variable()))
        .collect::<Vec<_>>();
    let objective = margins
        .iter()
        .fold(Expression::from(0.0), |acc, margin| acc + *margin);
    let mut model = vars.maximise(objective).using(default_solver);
    for facet in 0..facet_count {
        model = model.with(constraint!(
            dot_expr(dual_vertices[facet], &points[facet]) == 1.0
        ));
        for (idx, normal) in dual_vertices.iter().enumerate() {
            if idx != facet {
                model = model.with(constraint!(
                    dot_expr(*normal, &points[facet]) <= 1.0 - margins[facet]
                ));
            }
        }
    }
    let Ok(solution) = model.solve() else {
        return Err(());
    };
    let statuses = margins
        .iter()
        .map(|margin| lp_margin_status(solution.value(*margin)))
        .collect::<Vec<_>>();
    Ok((statuses, started.elapsed().as_secs_f64() * 1000.0))
}

fn batched_polar_facet_presence(
    dual_vertices: &[Vector4<f64>],
) -> Result<(Vec<F64Predicate>, f64), ()> {
    let started = Instant::now();
    let facet_count = dual_vertices.len();
    if facet_count < 5 {
        return Err(());
    }
    let mut vars = variables!();
    let lambdas = (0..facet_count)
        .map(|_| {
            (0..facet_count)
                .map(|_| vars.add(variable().min(0.0)))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let residuals = (0..facet_count)
        .map(|_| vars.add(variable().min(0.0)))
        .collect::<Vec<_>>();
    let objective = residuals
        .iter()
        .fold(Expression::from(0.0), |acc, residual| acc + *residual);
    let mut model = vars.minimise(objective).using(default_solver);
    for facet in 0..facet_count {
        model = model.with(constraint!(lambdas[facet][facet] == 0.0));
        let sum_lambda = (0..facet_count)
            .filter(|idx| *idx != facet)
            .fold(Expression::from(0.0), |acc, idx| acc + lambdas[facet][idx]);
        model = model.with(constraint!(sum_lambda == 1.0));
        for coordinate in 0..4 {
            let reconstructed = (0..facet_count)
                .filter(|idx| *idx != facet)
                .fold(Expression::from(0.0), |acc, idx| {
                    acc + dual_vertices[idx][coordinate] * lambdas[facet][idx]
                });
            let delta = dual_vertices[facet][coordinate] - reconstructed;
            model = model.with(constraint!(delta.clone() <= residuals[facet]));
            model = model.with(constraint!(-delta <= residuals[facet]));
        }
    }
    let Ok(solution) = model.solve() else {
        return Err(());
    };
    let statuses = residuals
        .iter()
        .map(|residual| {
            let value = solution.value(*residual);
            if value > EPS_LP_MARGIN {
                F64Predicate::True
            } else {
                F64Predicate::False
            }
        })
        .collect::<Vec<_>>();
    Ok((statuses, started.elapsed().as_secs_f64() * 1000.0))
}

fn lp_margin_status(value: f64) -> F64Predicate {
    if value > EPS_LP_MARGIN {
        F64Predicate::True
    } else if value < -EPS_LP_MARGIN {
        F64Predicate::False
    } else {
        F64Predicate::Indeterminate
    }
}

fn dot_expr(normal: Vector4<f64>, x: &[good_lp::Variable; 4]) -> Expression {
    normal[0] * x[0] + normal[1] * x[1] + normal[2] * x[2] + normal[3] * x[3]
}

fn count_label(values: &[&str], label: &str) -> usize {
    values.iter().filter(|value| **value == label).count()
}

fn predicate_label(predicate: &F64Predicate) -> &'static str {
    match predicate {
        F64Predicate::True => "true",
        F64Predicate::False => "false",
        F64Predicate::Indeterminate => "indeterminate",
    }
}

fn upper_triangle(matrix: &nalgebra::DMatrix<F64Predicate>) -> Vec<F64Predicate> {
    let mut result = Vec::new();
    for i in 0..matrix.nrows() {
        for j in i + 1..matrix.ncols() {
            result.push(matrix[(i, j)]);
        }
    }
    result
}

use crate::args::{Config, RunMode};
use crate::events::{
    ContextFinished, ContextStarted, Observation, PredicateObservation, RunFinished, RunStarted,
};
use crate::output::{prepare_out_dir, JsonlWriter, TARGET_NAME};
use nalgebra::{DMatrix, DVector};
use num_rational::BigRational;
use num_traits::ToPrimitive;
use std::path::PathBuf;
use symplectic::geom::known_polytopes::{self, KnownPolytope};
use symplectic::kkt::projection_solver;
use symplectic::kkt::qp_assembly::build_qp_from_dual_vertices;
use symplectic::kkt::rational_solver::{solve_kkt_exact, ExactKktResult};
use symplectic::kkt::saddle_point_solver::{solve_kkt_for_dual_vertices, KktOutcome};
use symplectic::kkt::Verdict;

struct AuditContext {
    object_name: &'static str,
    object_family: &'static str,
    input_pair_kind: &'static str,
    sigma: Vec<usize>,
    sample_policy: &'static str,
}

struct ContextData {
    context_id: String,
    context_kind: &'static str,
    object_id: String,
    object_family: &'static str,
    input_pair_kind: &'static str,
    sigma: Vec<usize>,
    sample_policy: &'static str,
}

const OBJECT_FAMILY_KNOWN_POLYTOPE: &str = "known_polytope";
const INPUT_PAIR_RATIONAL_SOURCE_TO_F64: &str = "rational_source_to_f64";
const INPUT_PAIR_BINARY64_INPUT_TO_EXACT: &str = "binary64_input_to_exact";
const ORACLE_EXACT_RATIONAL: &str = "exact_rational";
const ORACLE_EXACT_BINARY64_INPUT: &str = "exact_binary64_input";
const ORACLE_MATHEMATICAL_IDENTITY: &str = "mathematical_identity";

pub fn run(config: Config) -> Result<PathBuf, String> {
    let out_dir = prepare_out_dir(config.out_dir.clone(), TARGET_NAME, config.mode.as_str())?;
    let events_path = out_dir.join("events.jsonl");
    let mut writer = JsonlWriter::create(&events_path)?;
    writer.write(&RunStarted {
        event: "run_started",
        target: TARGET_NAME,
        mode: config.mode,
        schema_version: 1,
    })?;

    let contexts = contexts_for_mode(config.mode);
    for context in &contexts {
        audit_context(config.mode, context, &mut writer)?;
    }

    writer.write(&RunFinished {
        event: "run_finished",
        mode: config.mode,
        contexts: contexts.len(),
        status: "ok",
    })?;
    writer.flush()?;
    Ok(out_dir)
}

fn contexts_for_mode(mode: RunMode) -> Vec<AuditContext> {
    let mut contexts = vec![AuditContext {
        object_name: "simplex",
        object_family: OBJECT_FAMILY_KNOWN_POLYTOPE,
        input_pair_kind: INPUT_PAIR_RATIONAL_SOURCE_TO_F64,
        sigma: vec![0, 2, 1, 3, 4],
        sample_policy: "smoke_known_winner",
    }];
    if mode == RunMode::Evidence {
        contexts.extend([
            AuditContext {
                object_name: "hypercube",
                object_family: OBJECT_FAMILY_KNOWN_POLYTOPE,
                input_pair_kind: INPUT_PAIR_RATIONAL_SOURCE_TO_F64,
                sigma: vec![0, 4, 1, 5],
                sample_policy: "known_winner",
            },
            AuditContext {
                object_name: "hko_pentagon",
                object_family: OBJECT_FAMILY_KNOWN_POLYTOPE,
                input_pair_kind: INPUT_PAIR_BINARY64_INPUT_TO_EXACT,
                sigma: vec![1, 8, 7, 3, 4, 5, 9],
                sample_policy: "hko_selected_winner",
            },
            AuditContext {
                object_name: "hko_pentagon",
                object_family: OBJECT_FAMILY_KNOWN_POLYTOPE,
                input_pair_kind: INPUT_PAIR_BINARY64_INPUT_TO_EXACT,
                sigma: vec![1, 7, 2, 8, 4, 6, 5],
                sample_policy: "hko_rank_deficient_diagnostic",
            },
        ]);
    }
    contexts
}

fn audit_context(
    mode: RunMode,
    context: &AuditContext,
    writer: &mut JsonlWriter,
) -> Result<(), String> {
    let polytope = find_known_polytope(context.object_name)?;
    let data = ContextData {
        context_id: format!("{}:{}", context.object_name, sigma_key(&context.sigma)),
        context_kind: "sigma_node",
        object_id: format!("known:{}", context.object_name),
        object_family: context.object_family,
        input_pair_kind: context.input_pair_kind,
        sigma: context.sigma.clone(),
        sample_policy: context.sample_policy,
    };
    writer.write(&ContextStarted {
        event: "context_started",
        mode,
        context_id: data.context_id.clone(),
        context_kind: data.context_kind,
        object_id: data.object_id.clone(),
        object_family: data.object_family,
        input_pair_kind: data.input_pair_kind,
        sigma: data.sigma.clone(),
        sample_policy: data.sample_policy,
    })?;

    let exact = solve_kkt_exact(&polytope.dual_vertices, &context.sigma);
    let qp = build_qp_from_dual_vertices(&polytope.dual_vertices_f64, &context.sigma);
    emit_matrix_diagnostics(mode, &data, &qp.c, &qp.h, writer)?;
    emit_exact_status(mode, &data, exact.as_ref(), writer)?;
    audit_projection(mode, &data, &qp, exact.as_ref(), writer)?;
    audit_saddle(mode, &data, polytope, exact.as_ref(), writer)?;

    writer.write(&ContextFinished {
        event: "context_finished",
        mode,
        context_id: data.context_id,
        status: "ok",
    })?;
    Ok(())
}

fn find_known_polytope(name: &str) -> Result<&'static KnownPolytope, String> {
    known_polytopes::all_known()
        .into_iter()
        .find(|polytope| polytope.name == name)
        .ok_or_else(|| format!("known polytope not found: {name}"))
}

fn emit_matrix_diagnostics(
    mode: RunMode,
    data: &ContextData,
    c: &DMatrix<f64>,
    h: &DMatrix<f64>,
    writer: &mut JsonlWriter,
) -> Result<(), String> {
    let svd = c.clone().svd(false, false);
    let sigma_max = svd.singular_values.iter().copied().fold(0.0, f64::max);
    let sigma_min = svd
        .singular_values
        .iter()
        .copied()
        .filter(|value| *value > 1e-15)
        .fold(f64::INFINITY, f64::min);
    emit_numeric(
        mode,
        data,
        "matrix_assembly",
        "matrix_assembly",
        "sigma_max_c",
        None,
        Some(sigma_max),
        None,
        None,
        None,
        Some("diagnostic_no_oracle"),
        writer,
    )?;
    emit_numeric(
        mode,
        data,
        "matrix_assembly",
        "matrix_assembly",
        "sigma_min_c",
        None,
        finite_option(sigma_min),
        None,
        None,
        None,
        Some("diagnostic_no_oracle"),
        writer,
    )?;

    let eigen = h.clone().symmetric_eigen();
    for (index, value) in eigen.eigenvalues.iter().copied().enumerate() {
        emit_numeric(
            mode,
            data,
            "matrix_assembly",
            "matrix_assembly",
            "h_eigenvalue",
            Some(index),
            Some(value),
            None,
            None,
            None,
            Some("diagnostic_no_oracle"),
            writer,
        )?;
    }
    Ok(())
}

fn emit_exact_status(
    mode: RunMode,
    data: &ContextData,
    exact: Option<&ExactKktResult>,
    writer: &mut JsonlWriter,
) -> Result<(), String> {
    emit_predicate(
        mode,
        "exact_kkt_oracle",
        data,
        "solve",
        "beta_positive_exists",
        None,
        Some(exact.is_some()),
        None,
        "ok",
        Some("exact oracle is a positive witness when true; beta witness need not be unique"),
        writer,
    )
}

fn audit_projection(
    mode: RunMode,
    data: &ContextData,
    qp: &symplectic::kkt::QP,
    exact: Option<&ExactKktResult>,
    writer: &mut JsonlWriter,
) -> Result<(), String> {
    let solution = projection_solver::solve_projected(qp);
    let exact_q = exact.map(|value| value.q_exact.clone());
    emit_numeric_with_exact(
        mode,
        data,
        "projection_kkt",
        "solve",
        "q",
        None,
        Some(solution.q),
        exact_q.as_ref(),
        Some(oracle_kind_for(data)),
        None,
        writer,
    )?;
    emit_numeric(
        mode,
        data,
        "projection_kkt",
        "solve",
        "margin",
        None,
        Some(solution.margin),
        exact.map(|value| {
            value
                .beta
                .iter()
                .map(rational_to_f64)
                .fold(f64::INFINITY, f64::min)
        }),
        exact.map(|_| oracle_kind_for(data)),
        None,
        exact.map(|_| {
            "exact positive witness margin; not necessarily max-margin when solution set is nonunique"
        }),
        writer,
    )?;

    for (index, beta) in solution.beta.iter().copied().enumerate() {
        let exact_beta = exact.and_then(|value| value.beta.get(index));
        emit_numeric_with_exact(
            mode,
            data,
            "projection_kkt",
            "solve",
            "beta",
            Some(index),
            Some(beta),
            exact_beta,
            Some(oracle_kind_for(data)),
            Some("exact beta is a positive witness and may not be the same max-margin point"),
            writer,
        )?;
    }

    emit_predicate(
        mode,
        "projection_kkt",
        data,
        "solve",
        "beta_positive",
        Some(verdict_name(solution.verdict)),
        Some(exact.is_some()),
        trinary_disagrees(Some(verdict_name(solution.verdict)), Some(exact.is_some())),
        "ok",
        None,
        writer,
    )?;

    if !solution.beta.is_empty() {
        let beta = DVector::from_column_slice(&solution.beta);
        let residual = (&qp.c * beta - &qp.d).norm();
        emit_numeric(
            mode,
            data,
            "projection_kkt",
            "solve",
            "constraint_residual_norm",
            None,
            Some(residual),
            Some(0.0),
            Some(ORACLE_MATHEMATICAL_IDENTITY),
            None,
            None,
            writer,
        )?;
    }
    Ok(())
}

fn audit_saddle(
    mode: RunMode,
    data: &ContextData,
    polytope: &KnownPolytope,
    exact: Option<&ExactKktResult>,
    writer: &mut JsonlWriter,
) -> Result<(), String> {
    let outcome = solve_kkt_for_dual_vertices(&polytope.dual_vertices_f64, &data.sigma);
    let exact_q = exact.map(|value| value.q_exact.clone());
    match outcome {
        KktOutcome::Feasible(result) => {
            emit_numeric_with_exact(
                mode,
                data,
                "saddle_kkt",
                "solve",
                "q",
                None,
                Some(result.q_corrected),
                exact_q.as_ref(),
                Some(oracle_kind_for(data)),
                None,
                writer,
            )?;
            emit_numeric(
                mode,
                data,
                "saddle_kkt",
                "solve",
                "q_error_bound",
                None,
                Some(result.q_error_bound),
                None,
                None,
                None,
                Some("stored f64 a-posteriori bound; not itself checked as an oracle value here"),
                writer,
            )?;
            for (index, beta) in result.beta.iter().copied().enumerate() {
                let exact_beta = exact.and_then(|value| value.beta.get(index));
                emit_numeric_with_exact(
                    mode,
                    data,
                    "saddle_kkt",
                    "solve",
                    "beta",
                    Some(index),
                    Some(beta),
                    exact_beta,
                    Some(oracle_kind_for(data)),
                    Some(
                        "exact beta is a positive witness and may not be the same max-margin point",
                    ),
                    writer,
                )?;
            }
            emit_predicate(
                mode,
                "saddle_kkt",
                data,
                "solve",
                "beta_positive",
                Some("true"),
                Some(exact.is_some()),
                trinary_disagrees(Some("true"), Some(exact.is_some())),
                "ok",
                None,
                writer,
            )?;
            emit_inertia(mode, data, "n_positive", result.n_positive, writer)?;
            emit_inertia(mode, data, "n_negative", result.n_negative, writer)?;
            emit_inertia(mode, data, "n_zero", result.n_zero, writer)?;
        }
        other => {
            emit_predicate(
                mode,
                "saddle_kkt",
                data,
                "solve",
                "beta_positive",
                Some(outcome_name(&other)),
                Some(exact.is_some()),
                trinary_disagrees(Some(outcome_name(&other)), Some(exact.is_some())),
                "ok",
                None,
                writer,
            )?;
        }
    }
    Ok(())
}

fn emit_inertia(
    mode: RunMode,
    data: &ContextData,
    variable: &'static str,
    value: usize,
    writer: &mut JsonlWriter,
) -> Result<(), String> {
    emit_numeric(
        mode,
        data,
        "saddle_kkt",
        "solve",
        variable,
        None,
        Some(value as f64),
        None,
        None,
        None,
        Some("integer inertia diagnostic emitted as numeric event"),
        writer,
    )
}

#[allow(clippy::too_many_arguments)]
fn emit_numeric_with_exact(
    mode: RunMode,
    data: &ContextData,
    algorithm: &'static str,
    stage: &'static str,
    variable: &'static str,
    component: Option<usize>,
    f64_value: Option<f64>,
    exact_value: Option<&BigRational>,
    oracle_kind: Option<&'static str>,
    note: Option<&'static str>,
    writer: &mut JsonlWriter,
) -> Result<(), String> {
    let oracle_f64 = exact_value.map(rational_to_f64);
    emit_numeric(
        mode,
        data,
        algorithm,
        stage,
        variable,
        component,
        f64_value,
        oracle_f64,
        oracle_kind.filter(|_| exact_value.is_some()),
        exact_value.map(|value| value.to_string()),
        note.filter(|_| exact_value.is_some()),
        writer,
    )
}

#[allow(clippy::too_many_arguments)]
fn emit_numeric(
    mode: RunMode,
    data: &ContextData,
    algorithm: &'static str,
    stage: &'static str,
    variable: &'static str,
    component: Option<usize>,
    f64_value: Option<f64>,
    oracle_f64: Option<f64>,
    oracle_kind: Option<&'static str>,
    exact: Option<String>,
    note: Option<&'static str>,
    writer: &mut JsonlWriter,
) -> Result<(), String> {
    let f64_value = f64_value.and_then(finite_option);
    let oracle_f64 = oracle_f64.and_then(finite_option);
    let abs_error = match (f64_value, oracle_f64) {
        (Some(lhs), Some(rhs)) => Some((lhs - rhs).abs()),
        _ => None,
    };
    let rel_error = match (abs_error, oracle_f64) {
        (Some(error), Some(rhs)) if rhs.abs() > 0.0 => Some(error / rhs.abs()),
        _ => None,
    };
    writer.write(&Observation {
        event: "observation",
        mode,
        algorithm,
        stage,
        context_id: data.context_id.clone(),
        context_kind: data.context_kind,
        object_id: data.object_id.clone(),
        object_family: data.object_family,
        input_pair_kind: data.input_pair_kind,
        sigma: data.sigma.clone(),
        variable,
        component,
        sample_policy: data.sample_policy,
        status: "ok",
        f64: f64_value,
        oracle_kind,
        exact,
        oracle_f64,
        abs_error,
        rel_error,
        note,
    })
}

#[allow(clippy::too_many_arguments)]
fn emit_predicate(
    mode: RunMode,
    algorithm: &'static str,
    data: &ContextData,
    stage: &'static str,
    predicate: &'static str,
    f64_trinary: Option<&'static str>,
    oracle_binary: Option<bool>,
    disagrees_with_oracle: Option<bool>,
    status: &'static str,
    note: Option<&'static str>,
    writer: &mut JsonlWriter,
) -> Result<(), String> {
    writer.write(&PredicateObservation {
        event: "predicate_observation",
        mode,
        algorithm,
        stage,
        context_id: data.context_id.clone(),
        context_kind: data.context_kind,
        object_id: data.object_id.clone(),
        object_family: data.object_family,
        input_pair_kind: data.input_pair_kind,
        sigma: data.sigma.clone(),
        predicate,
        sample_policy: data.sample_policy,
        status,
        f64_trinary,
        oracle_kind: oracle_binary.map(|_| oracle_kind_for(data)),
        oracle_binary,
        disagrees_with_oracle,
        note,
    })
}

fn finite_option(value: f64) -> Option<f64> {
    if value.is_finite() {
        Some(value)
    } else {
        None
    }
}

fn rational_to_f64(value: &BigRational) -> f64 {
    value.to_f64().unwrap_or(f64::NAN)
}

fn oracle_kind_for(data: &ContextData) -> &'static str {
    match data.input_pair_kind {
        INPUT_PAIR_BINARY64_INPUT_TO_EXACT => ORACLE_EXACT_BINARY64_INPUT,
        _ => ORACLE_EXACT_RATIONAL,
    }
}

fn trinary_disagrees(f64_trinary: Option<&str>, oracle_binary: Option<bool>) -> Option<bool> {
    let oracle = oracle_binary?;
    match f64_trinary? {
        "true" => Some(!oracle),
        "false" => Some(oracle),
        _ => Some(true),
    }
}

fn verdict_name(verdict: Verdict) -> &'static str {
    match verdict {
        Verdict::True => "true",
        Verdict::False => "false",
        Verdict::Indeterminate => "indeterminate",
    }
}

fn outcome_name(outcome: &KktOutcome) -> &'static str {
    match outcome {
        KktOutcome::Feasible(_) => "true",
        KktOutcome::Infeasible => "false",
        KktOutcome::SingularMatrix => "singular_matrix",
        KktOutcome::TypeCViolation => "type_c_violation",
        KktOutcome::ConstraintViolation => "constraint_violation",
    }
}

fn sigma_key(sigma: &[usize]) -> String {
    sigma
        .iter()
        .map(|value| value.to_string())
        .collect::<Vec<_>>()
        .join("-")
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;
    use std::fs;

    #[test]
    fn smoke_run_emits_exact_oracle_comparisons() {
        let out_dir = tempfile::tempdir().unwrap();
        let config = Config {
            mode: RunMode::Smoke,
            out_dir: Some(out_dir.path().to_path_buf()),
        };
        run(config).unwrap();

        let events = fs::read_to_string(out_dir.path().join("events.jsonl")).unwrap();
        let rows = events
            .lines()
            .map(|line| serde_json::from_str::<Value>(line).unwrap())
            .collect::<Vec<_>>();

        assert!(rows.iter().any(|row| row["event"] == "run_started"));
        assert!(rows.iter().any(|row| {
            row["event"] == "observation"
                && row["variable"] == "q"
                && row["input_pair_kind"] == INPUT_PAIR_RATIONAL_SOURCE_TO_F64
                && row.get("exact").is_some()
                && row.get("abs_error").is_some()
        }));
        assert!(rows.iter().any(|row| {
            row["event"] == "predicate_observation"
                && row["predicate"] == "beta_positive"
                && row.get("disagrees_with_oracle").is_some()
        }));
    }

    #[test]
    fn evidence_run_labels_hko_as_binary64_input() {
        let out_dir = tempfile::tempdir().unwrap();
        let config = Config {
            mode: RunMode::Evidence,
            out_dir: Some(out_dir.path().to_path_buf()),
        };
        run(config).unwrap();

        let events = fs::read_to_string(out_dir.path().join("events.jsonl")).unwrap();
        let rows = events
            .lines()
            .map(|line| serde_json::from_str::<Value>(line).unwrap())
            .collect::<Vec<_>>();

        assert!(rows.iter().any(|row| {
            row["event"] == "context_started"
                && row["object_id"] == "known:hko_pentagon"
                && row["input_pair_kind"] == INPUT_PAIR_BINARY64_INPUT_TO_EXACT
        }));
        assert!(rows.iter().any(|row| {
            row["event"] == "predicate_observation"
                && row["object_id"] == "known:hko_pentagon"
                && row["oracle_kind"] == ORACLE_EXACT_BINARY64_INPUT
        }));
    }
}

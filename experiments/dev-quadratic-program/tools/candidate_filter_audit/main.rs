#[path = "../scan/input/mod.rs"]
mod input;

use euclidean_polytopes::{
    facet_intersection_is_nonempty_from_vertex_facet_incidence,
    polar_vertices_exact_rational_assuming_origin_interior, PolarVerticesExact,
};
use exp_dev_quadratic_program::ScanCase;
use nalgebra::{DMatrix, Vector4};
use num_rational::BigRational;
use num_traits::{Signed, ToPrimitive};
use serde::Serialize;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::path::PathBuf;
use symplectic::algorithms::facet_adjacency::build_transition_matrix_from_facet_intersections_and_omega;
use symplectic::algorithms::hk2017::permutations::for_each_cyclic_permutation;
use symplectic::algorithms::hk2017::{combinations, SimpleDirectedCyclesCanonical};
use symplectic::exact::omega_signs_exact;
use symplectic::geom::rational_arithmetic::f64_to_rational;
use symplectic::kkt::rational_solver::solve_kkt_exact;
use symplectic::{solve_orbit_sigma_saddle_point, OrbitAdmissibility, OrbitSolveError};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Enumeration {
    PrunedExactBinary64,
    Unpruned,
}

#[derive(Clone, Debug)]
struct Args {
    output: PathBuf,
    input_source: input::InputSource,
    max_rows_per_family: usize,
    generated_samples_per_facet: usize,
    generated_seed: u64,
    family_filter: Vec<String>,
    source_id_filter: Vec<String>,
    enumeration: Enumeration,
    max_sigmas_per_case: usize,
    max_examples_per_case: usize,
}

#[derive(Clone, Debug, Serialize)]
struct CaseSummaryRow {
    event: &'static str,
    family: String,
    source_id: String,
    input_source: String,
    enumeration: &'static str,
    facet_count: usize,
    status: &'static str,
    sigmas_examined: u64,
    sigmas_truncated: bool,
    exact_admissible_q_positive: u64,
    f64_retained: u64,
    f64_true: u64,
    f64_indet: u64,
    f64_inadmissible: u64,
    f64_numerical_failure: u64,
    false_discard_exact_admissible: u64,
    false_discard_exact_minimizer: u64,
    exact_minimizer_count: u64,
    exact_capacity: Option<f64>,
    min_false_discard_action: Option<f64>,
    note: Option<&'static str>,
}

#[derive(Clone, Debug, Serialize)]
struct ExampleRow {
    event: &'static str,
    family: String,
    source_id: String,
    input_source: String,
    enumeration: &'static str,
    sigma: Vec<usize>,
    exact_action: f64,
    exact_q: f64,
    f64_status: &'static str,
    exact_minimizer: bool,
}

fn main() {
    let args = parse_args();
    let cases = input::load_cases(&input::LoadCaseOptions {
        input_source: args.input_source,
        max_rows_per_family: args.max_rows_per_family,
        generated_samples_per_facet: args.generated_samples_per_facet,
        generated_seed: args.generated_seed,
        family_filter: args.family_filter.clone(),
        source_id_filter: args.source_id_filter.clone(),
    });

    let file = File::create(&args.output)
        .unwrap_or_else(|err| panic!("create {}: {err}", args.output.display()));
    let mut writer = BufWriter::new(file);
    for case in cases {
        let (summary, examples) = audit_case(&case, &args);
        serde_json::to_writer(&mut writer, &summary)
            .unwrap_or_else(|err| panic!("serialize summary: {err}"));
        writer
            .write_all(b"\n")
            .unwrap_or_else(|err| panic!("write summary: {err}"));
        for example in examples {
            serde_json::to_writer(&mut writer, &example)
                .unwrap_or_else(|err| panic!("serialize example: {err}"));
            writer
                .write_all(b"\n")
                .unwrap_or_else(|err| panic!("write example: {err}"));
        }
    }
    writer
        .flush()
        .unwrap_or_else(|err| panic!("flush {}: {err}", args.output.display()));
}

fn audit_case(case: &ScanCase, args: &Args) -> (CaseSummaryRow, Vec<ExampleRow>) {
    let exact_input = exact_dual_vertex_arrays(&case.dual_vertices);
    let enumeration_label = enumeration_label(args.enumeration);
    let transition = match args.enumeration {
        Enumeration::Unpruned => None,
        Enumeration::PrunedExactBinary64 => match exact_binary64_transition_matrix(&exact_input) {
            Ok(transition) => Some(transition),
            Err(_) => {
                return (
                    CaseSummaryRow {
                        event: "qp_candidate_filter_summary",
                        family: case.family.clone(),
                        source_id: case.source_id.clone(),
                        input_source: case.input_source.clone(),
                        enumeration: enumeration_label,
                        facet_count: case.dual_vertices.len(),
                        status: "exact_binary64_geometry_failed",
                        sigmas_examined: 0,
                        sigmas_truncated: false,
                        exact_admissible_q_positive: 0,
                        f64_retained: 0,
                        f64_true: 0,
                        f64_indet: 0,
                        f64_inadmissible: 0,
                        f64_numerical_failure: 0,
                        false_discard_exact_admissible: 0,
                        false_discard_exact_minimizer: 0,
                        exact_minimizer_count: 0,
                        exact_capacity: None,
                        min_false_discard_action: None,
                        note: Some("transition construction failed"),
                    },
                    Vec::new(),
                );
            }
        },
    };

    let mut observations = Vec::new();
    let mut sigmas_seen = 0u64;
    let mut truncated = false;
    let mut observe = |sigma: &[usize]| -> bool {
        if args.max_sigmas_per_case != 0 && sigmas_seen as usize >= args.max_sigmas_per_case {
            truncated = true;
            return false;
        }
        sigmas_seen += 1;
        observations.push(observe_sigma(case, &exact_input, sigma));
        true
    };

    match (&transition, args.enumeration) {
        (Some(transition), Enumeration::PrunedExactBinary64) => {
            for sigma in SimpleDirectedCyclesCanonical::new(transition) {
                if !observe(&sigma) {
                    break;
                }
            }
        }
        (None, Enumeration::Unpruned) => 'unpruned: {
            for m in 2..=case.dual_vertices.len() {
                for subset in combinations(case.dual_vertices.len(), m) {
                    let mut should_stop = false;
                    for_each_cyclic_permutation(&subset, &mut |perm| {
                        if !should_stop && !observe(perm) {
                            should_stop = true;
                        }
                    });
                    if should_stop {
                        break 'unpruned;
                    }
                }
            }
        }
        _ => unreachable!("transition shape is aligned with enumeration"),
    }

    let exact_capacity = observations
        .iter()
        .filter_map(|obs| obs.exact_action_exact.as_ref())
        .min()
        .cloned();
    let exact_capacity_f64 = exact_capacity.as_ref().map(rational_to_f64);
    let exact_minimizer_count = exact_capacity
        .as_ref()
        .map(|capacity| {
            observations
                .iter()
                .filter(|obs| obs.exact_action_exact.as_ref() == Some(capacity))
                .count() as u64
        })
        .unwrap_or(0);

    let false_discard_exact_minimizer = exact_capacity
        .as_ref()
        .map(|capacity| {
            observations
                .iter()
                .filter(|obs| {
                    obs.exact_action_exact.as_ref() == Some(capacity)
                        && !matches!(obs.f64_status, F64Status::True | F64Status::Indet)
                })
                .count() as u64
        })
        .unwrap_or(0);
    let min_false_discard_action = observations
        .iter()
        .filter(|obs| {
            obs.exact_action_exact.is_some()
                && !matches!(obs.f64_status, F64Status::True | F64Status::Indet)
        })
        .filter_map(|obs| obs.exact_action_exact.as_ref())
        .min()
        .map(rational_to_f64);

    let false_discard_examples = observations
        .iter()
        .filter(|obs| {
            obs.exact_action_exact.is_some()
                && !matches!(obs.f64_status, F64Status::True | F64Status::Indet)
        })
        .take(args.max_examples_per_case)
        .map(|obs| ExampleRow {
            event: "qp_candidate_filter_false_discard_example",
            family: case.family.clone(),
            source_id: case.source_id.clone(),
            input_source: case.input_source.clone(),
            enumeration: enumeration_label,
            sigma: obs.sigma.clone(),
            exact_action: obs
                .exact_action_exact
                .as_ref()
                .map(rational_to_f64)
                .expect("filtered to exact-admissible"),
            exact_q: obs.exact_q_f64.expect("filtered to exact-admissible"),
            f64_status: obs.f64_status.label(),
            exact_minimizer: obs.exact_action_exact.as_ref() == exact_capacity.as_ref(),
        })
        .collect();

    (
        CaseSummaryRow {
            event: "qp_candidate_filter_summary",
            family: case.family.clone(),
            source_id: case.source_id.clone(),
            input_source: case.input_source.clone(),
            enumeration: enumeration_label,
            facet_count: case.dual_vertices.len(),
            status: "ok",
            sigmas_examined: sigmas_seen,
            sigmas_truncated: truncated,
            exact_admissible_q_positive: observations
                .iter()
                .filter(|obs| obs.exact_action_exact.is_some())
                .count() as u64,
            f64_retained: observations
                .iter()
                .filter(|obs| matches!(obs.f64_status, F64Status::True | F64Status::Indet))
                .count() as u64,
            f64_true: observations
                .iter()
                .filter(|obs| obs.f64_status == F64Status::True)
                .count() as u64,
            f64_indet: observations
                .iter()
                .filter(|obs| obs.f64_status == F64Status::Indet)
                .count() as u64,
            f64_inadmissible: observations
                .iter()
                .filter(|obs| obs.f64_status == F64Status::Inadmissible)
                .count() as u64,
            f64_numerical_failure: observations
                .iter()
                .filter(|obs| obs.f64_status == F64Status::NumericalFailure)
                .count() as u64,
            false_discard_exact_admissible: observations
                .iter()
                .filter(|obs| {
                    obs.exact_action_exact.is_some()
                        && !matches!(obs.f64_status, F64Status::True | F64Status::Indet)
                })
                .count() as u64,
            false_discard_exact_minimizer,
            exact_minimizer_count,
            exact_capacity: exact_capacity_f64,
            min_false_discard_action,
            note: None,
        },
        false_discard_examples,
    )
}

fn observe_sigma(
    case: &ScanCase,
    exact_input: &[[BigRational; 4]],
    sigma: &[usize],
) -> SigmaObservation {
    let exact = solve_kkt_exact(exact_input, sigma).filter(|result| result.q_exact.is_positive());
    let exact_action_exact = exact
        .as_ref()
        .map(|result| exact_action_from_q(&result.q_exact));
    let exact_q_f64 = exact.as_ref().map(|result| result.q_exact_f64);
    let f64_status = match solve_orbit_sigma_saddle_point(&case.dual_vertices, sigma) {
        Ok(orbit) => match orbit.admissibility {
            OrbitAdmissibility::AdmissibleF64 | OrbitAdmissibility::AdmissibleExact => {
                F64Status::True
            }
            OrbitAdmissibility::IndeterminateF64 => F64Status::Indet,
        },
        Err(OrbitSolveError::Inadmissible) => F64Status::Inadmissible,
        Err(OrbitSolveError::NumericalFailure) => F64Status::NumericalFailure,
    };

    SigmaObservation {
        sigma: sigma.to_vec(),
        exact_action_exact,
        exact_q_f64,
        f64_status,
    }
}

#[derive(Clone, Debug)]
struct SigmaObservation {
    sigma: Vec<usize>,
    exact_action_exact: Option<BigRational>,
    exact_q_f64: Option<f64>,
    f64_status: F64Status,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum F64Status {
    True,
    Indet,
    Inadmissible,
    NumericalFailure,
}

impl F64Status {
    fn label(self) -> &'static str {
        match self {
            F64Status::True => "true",
            F64Status::Indet => "indet",
            F64Status::Inadmissible => "inadmissible",
            F64Status::NumericalFailure => "numerical_failure",
        }
    }
}

fn exact_action_from_q(q_exact: &BigRational) -> BigRational {
    BigRational::new(1.into(), 2.into()) / q_exact
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

fn enumeration_label(enumeration: Enumeration) -> &'static str {
    match enumeration {
        Enumeration::PrunedExactBinary64 => "hk2017_pruned_exact_binary64",
        Enumeration::Unpruned => "hk2017_unpruned",
    }
}

fn rational_to_f64(value: &BigRational) -> f64 {
    value.to_f64().unwrap_or(f64::NAN)
}

fn parse_args() -> Args {
    let argv: Vec<String> = std::env::args().collect();
    let mut output = PathBuf::from("/tmp/qp-candidate-filter-audit.jsonl");
    let mut input_source = input::InputSource::Generated;
    let mut max_rows_per_family = 1usize;
    let mut generated_samples_per_facet = 1usize;
    let mut generated_seed = 0x5eed_f64_u64;
    let mut family_filter = Vec::new();
    let mut source_id_filter = Vec::new();
    let mut enumeration = Enumeration::PrunedExactBinary64;
    let mut max_sigmas_per_case = 0usize;
    let mut max_examples_per_case = 5usize;

    let mut i = 1;
    while i < argv.len() {
        match argv[i].as_str() {
            "--output" => {
                output = PathBuf::from(value(&argv, i, "--output"));
                i += 2;
            }
            "--input-source" => {
                input_source = match value(&argv, i, "--input-source") {
                    "all" => input::InputSource::All,
                    "generated" => input::InputSource::Generated,
                    "artifacts" => input::InputSource::Artifacts,
                    "edge-fixtures" => input::InputSource::EdgeFixtures,
                    other => panic!(
                        "--input-source must be all, generated, artifacts, or edge-fixtures, got {other}"
                    ),
                };
                i += 2;
            }
            "--max-rows-per-family" => {
                max_rows_per_family = value(&argv, i, "--max-rows-per-family")
                    .parse()
                    .expect("--max-rows-per-family must be a non-negative integer");
                i += 2;
            }
            "--generated-samples-per-facet" => {
                generated_samples_per_facet = value(&argv, i, "--generated-samples-per-facet")
                    .parse()
                    .expect("--generated-samples-per-facet must be a non-negative integer");
                i += 2;
            }
            "--generated-seed" => {
                generated_seed = value(&argv, i, "--generated-seed")
                    .parse()
                    .expect("--generated-seed must be a u64");
                i += 2;
            }
            "--family-filter" => {
                family_filter.extend(
                    value(&argv, i, "--family-filter")
                        .split(',')
                        .filter(|family| !family.is_empty())
                        .map(str::to_string),
                );
                i += 2;
            }
            "--source-id-filter" => {
                source_id_filter.extend(
                    value(&argv, i, "--source-id-filter")
                        .split(',')
                        .filter(|source_id| !source_id.is_empty())
                        .map(str::to_string),
                );
                i += 2;
            }
            "--enumeration" => {
                enumeration = match value(&argv, i, "--enumeration") {
                    "pruned-exact-binary64" => Enumeration::PrunedExactBinary64,
                    "unpruned" => Enumeration::Unpruned,
                    other => panic!(
                        "--enumeration must be pruned-exact-binary64 or unpruned, got {other}"
                    ),
                };
                i += 2;
            }
            "--max-sigmas-per-case" => {
                max_sigmas_per_case = value(&argv, i, "--max-sigmas-per-case")
                    .parse()
                    .expect("--max-sigmas-per-case must be a non-negative integer");
                i += 2;
            }
            "--max-examples-per-case" => {
                max_examples_per_case = value(&argv, i, "--max-examples-per-case")
                    .parse()
                    .expect("--max-examples-per-case must be a non-negative integer");
                i += 2;
            }
            "--help" | "-h" => {
                print_help();
                std::process::exit(0);
            }
            other => panic!("unknown argument: {other}"),
        }
    }

    Args {
        output,
        input_source,
        max_rows_per_family,
        generated_samples_per_facet,
        generated_seed,
        family_filter,
        source_id_filter,
        enumeration,
        max_sigmas_per_case,
        max_examples_per_case,
    }
}

fn value<'a>(argv: &'a [String], i: usize, flag: &str) -> &'a str {
    argv.get(i + 1)
        .map(String::as_str)
        .unwrap_or_else(|| panic!("{flag} requires a value"))
}

fn print_help() {
    println!(
        "Usage: qp-candidate-filter-audit [--output PATH] [--input-source all|generated|artifacts|edge-fixtures]\n\
         [--max-rows-per-family N] [--generated-samples-per-facet N] [--generated-seed U64]\n\
         [--family-filter FAMILY[,FAMILY...]] [--source-id-filter SOURCE_ID[,SOURCE_ID...]]\n\
         [--enumeration pruned-exact-binary64|unpruned] [--max-sigmas-per-case N]\n\
         [--max-examples-per-case N]\n\
         N=0 means no cap for rows or sigmas."
    );
}

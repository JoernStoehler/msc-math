//! Flow-graph proof-risk verification rows.
//!
//! This executable records public-output falsifier rows for
//! `formal/flow-graph-proof-risk.tex`.  It compares exact flow-graph search
//! outputs against current exact HK/QP aggregation and direct exact closed-word
//! resolution.  It does not inspect private tube internals and does not prove
//! tube semantics or cutoff lower-bound certificates.

use dev_capacity_validation::{
    create_jsonl_writer, mode_output_path, parse_run_mode, run_mode_label, write_json_line,
    RunMode, RunModeArgError, VerificationPolytopeCache,
};
use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{ToPrimitive, Zero};
use serde::Serialize;
use std::collections::BTreeMap;
use std::io::Write;
use std::path::Path;
use symplectic::algorithms::facet_adjacency::build_transition_matrix_from_facet_intersections_and_omega;
use symplectic::algorithms::flow_graph::exact_search::{
    search_closed_orbits_exact, ExactActionCutoffPolicy, ExactFlowGraphOrbit,
    ExactFlowGraphSearchError, ExactFlowGraphSearchResult,
};
use symplectic::algorithms::flow_graph::exact_tube::{
    resolve_closed_word_exact, ExactClosedWordOutcome, ExactFlatTubeInput,
};
use symplectic::algorithms::hk2017::{
    for_each_sigma_pruned_by_transition, solve_pruned_hk2017_candidates,
};
use symplectic::geom::known_polytopes;
use symplectic::random::generate_dual_vertices;
use symplectic::{
    aggregate_certified_orbits_with_dual_vertices_exact, CertifiedOrbitSearchResult,
    CertifiedOrbitSetMode,
};

const H_MIN: f64 = 0.5;
const H_MAX: f64 = 2.0;
const MASTER_SEED: u64 = 20260605;
const PRODUCER_COMMAND_SMOKE: &str =
    "cargo run -p dev-capacity-validation --release --bin flow-graph-proof-risk";
const PRODUCER_COMMAND_FULL: &str =
    "cargo run -p dev-capacity-validation --release --bin flow-graph-proof-risk -- --full";

#[derive(Clone, Copy)]
struct GeneratedCase {
    case_name: &'static str,
    facet_count: usize,
    attempt: u64,
    smoke: bool,
}

#[derive(Clone, Copy)]
struct Provenance {
    producer_command: &'static str,
    master_seed: Option<u64>,
    facet_count: Option<usize>,
    attempt: Option<u64>,
}

#[derive(Serialize)]
struct ProofRiskRow {
    claim_id: &'static str,
    check_id: &'static str,
    case_name: String,
    mode: &'static str,
    producer_command: &'static str,
    master_seed: Option<u64>,
    facet_count: Option<usize>,
    attempt: Option<u64>,
    passed: bool,
    detail: String,
    facets: Option<Vec<usize>>,
    outcome_kind: Option<&'static str>,
    singular_status: Option<&'static str>,
    singular_min_action_exact: Option<String>,
    singular_max_action_exact: Option<String>,
    fg_capacity_exact: Option<String>,
    certified_hk_qp_capacity_exact: Option<String>,
    outcome_action_exact: Option<String>,
    fg_capacity_f64: Option<f64>,
    certified_hk_qp_capacity_f64: Option<f64>,
    outcome_action_f64: Option<f64>,
    checked_word_count: Option<usize>,
    retained_word_count: Option<usize>,
    direct_positive_word_count: Option<usize>,
    direct_empty_or_no_orbit_word_count: Option<usize>,
    action_cutoff_word_count: Option<usize>,
    action_cutoff_intersection_count: Option<u64>,
}

fn main() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mode = match parse_run_mode(std::env::args().skip(1)) {
        Ok(mode) => mode,
        Err(RunModeArgError::Help) => {
            print_usage();
            return;
        }
        Err(RunModeArgError::Unknown(arg)) => {
            eprintln!("unknown argument: {arg}");
            print_usage();
            std::process::exit(2);
        }
    };

    let output_path = mode_output_path(
        manifest_dir,
        "flow-graph-proof-risk",
        "smoke-flow-graph-proof-risk.jsonl",
        "flow-graph-proof-risk.jsonl",
        mode,
    );
    let mode_label = run_mode_label(mode);
    let producer_command = producer_command(mode);
    let mut rows = Vec::new();

    for case in selected_cases(mode) {
        rows.extend(rows_for_generated_case(mode_label, producer_command, case));
    }
    rows.extend(zero_omega_rejection_rows(mode_label, producer_command));
    rows.extend(specific_closed_word_rows(mode_label, producer_command));

    let mut writer = create_jsonl_writer(&output_path);
    let mut failures = Vec::new();
    for row in &rows {
        if !row.passed {
            failures.push(format!(
                "{} {} {}: {}",
                row.claim_id, row.check_id, row.case_name, row.detail
            ));
        }
        write_json_line(&mut writer, row);
    }
    writer.flush().expect("flush flow-graph proof-risk rows");

    println!("wrote {}", output_path.display());
    if !failures.is_empty() {
        eprintln!("flow-graph proof-risk verification failed:");
        for failure in failures {
            eprintln!("- {failure}");
        }
        std::process::exit(1);
    }
}

fn print_usage() {
    eprintln!(
        "Usage: cargo run -p dev-capacity-validation --release --bin flow-graph-proof-risk [-- --full]"
    );
    eprintln!("Default smoke mode writes flow-graph-proof-risk/smoke-flow-graph-proof-risk.jsonl.");
    eprintln!(
        "--full also runs generated_F7_attempt31 and writes flow-graph-proof-risk/flow-graph-proof-risk.jsonl."
    );
}

fn producer_command(mode: RunMode) -> &'static str {
    match mode {
        RunMode::Smoke => PRODUCER_COMMAND_SMOKE,
        RunMode::Full => PRODUCER_COMMAND_FULL,
    }
}

fn selected_cases(mode: RunMode) -> Vec<GeneratedCase> {
    let cases = vec![
        GeneratedCase {
            case_name: "generated_F5_attempt60",
            facet_count: 5,
            attempt: 60,
            smoke: true,
        },
        GeneratedCase {
            case_name: "generated_F6_attempt3",
            facet_count: 6,
            attempt: 3,
            smoke: true,
        },
        GeneratedCase {
            case_name: "generated_F7_attempt31",
            facet_count: 7,
            attempt: 31,
            smoke: false,
        },
    ];

    match mode {
        RunMode::Smoke => cases.into_iter().filter(|case| case.smoke).collect(),
        RunMode::Full => cases,
    }
}

fn rows_for_generated_case(
    mode: &'static str,
    producer_command: &'static str,
    case: GeneratedCase,
) -> Vec<ProofRiskRow> {
    let provenance = Provenance {
        producer_command,
        master_seed: Some(MASTER_SEED),
        facet_count: Some(case.facet_count),
        attempt: Some(case.attempt),
    };
    let Some(polytope) = deterministic_exact_admissible_case(case) else {
        return vec![failed_row(
            "FG-PR-4",
            "generated_case_reconstructs_exact_admissible_polytope",
            case.case_name,
            mode,
            provenance,
            "deterministic generated polytope did not pass exact-admissible cache reconstruction",
        )];
    };
    let input = exact_input(&polytope);
    let fg_zero = exact_search(
        &input,
        BigRational::zero(),
        ExactActionCutoffPolicy::Disabled,
    );
    let certified_hk_qp = certified_hk_qp_capacity(&polytope, BigRational::zero())
        .map_err(|error| format!("certified HK/QP aggregation failed: {error:?}"));
    let direct_zero = match &fg_zero {
        Ok(fg) => direct_resolver_summary(&input, &fg.capacity_action),
        Err(detail) => Err(format!("direct resolver summary skipped after {detail}")),
    };
    let gap_one = BigRational::from_integer(1.into());
    let cutoff_disabled = exact_search(&input, gap_one.clone(), ExactActionCutoffPolicy::Disabled);
    let cutoff_enabled = exact_search(&input, gap_one, ExactActionCutoffPolicy::Enabled);

    vec![
        exact_fg_capacity_matches_certified_hk_qp(
            mode,
            case.case_name,
            provenance,
            &fg_zero,
            &certified_hk_qp,
        ),
        exact_retained_words_match_direct_resolver(
            mode,
            case.case_name,
            provenance,
            &fg_zero,
            &direct_zero,
        ),
        cutoff_enabled_matches_disabled(
            mode,
            case.case_name,
            provenance,
            &cutoff_disabled,
            &cutoff_enabled,
        ),
        retained_positive_word_resolves_directly(
            mode,
            case.case_name,
            provenance,
            &input,
            &fg_zero,
        ),
    ]
}

fn deterministic_exact_admissible_case(case: GeneratedCase) -> Option<VerificationPolytopeCache> {
    let dual_vertices =
        generate_dual_vertices(case.facet_count, H_MIN, H_MAX, MASTER_SEED, case.attempt).ok()?;
    VerificationPolytopeCache::from_f64_dual_vertices(dual_vertices)
}

fn exact_input(polytope: &VerificationPolytopeCache) -> ExactFlatTubeInput<'_> {
    ExactFlatTubeInput {
        dual_vertices: &polytope.dual_vertices,
        facet_intersection_is_nonempty: &polytope.facet_intersection_is_nonempty,
        omega_signs: &polytope.omega_signs,
    }
}

fn exact_fg_capacity_matches_certified_hk_qp(
    mode: &'static str,
    case_name: &str,
    provenance: Provenance,
    fg: &Result<ExactFlowGraphSearchResult, String>,
    hk_qp: &Result<CertifiedOrbitSearchResult, String>,
) -> ProofRiskRow {
    let fg = match fg {
        Ok(result) => result,
        Err(detail) => {
            return failed_row(
                "FG-PR-4",
                "exact_fg_capacity_matches_certified_hk_qp",
                case_name,
                mode,
                provenance,
                detail.clone(),
            );
        }
    };
    let hk_qp = match hk_qp {
        Ok(result) => result,
        Err(detail) => {
            return failed_row(
                "FG-PR-4",
                "exact_fg_capacity_matches_certified_hk_qp",
                case_name,
                mode,
                provenance,
                detail.clone(),
            );
        }
    };
    let passed = fg.capacity_action == hk_qp.capacity_exact;
    ProofRiskRow {
        claim_id: "FG-PR-4",
        check_id: "exact_fg_capacity_matches_certified_hk_qp",
        case_name: case_name.to_string(),
        mode,
        producer_command: provenance.producer_command,
        master_seed: provenance.master_seed,
        facet_count: provenance.facet_count,
        attempt: provenance.attempt,
        passed,
        detail: if passed {
            "exact FG capacity equals current certified HK/QP capacity".to_string()
        } else {
            "exact FG capacity differs from current certified HK/QP capacity".to_string()
        },
        facets: None,
        outcome_kind: None,
        singular_status: None,
        singular_min_action_exact: None,
        singular_max_action_exact: None,
        fg_capacity_exact: Some(fg.capacity_action.to_string()),
        certified_hk_qp_capacity_exact: Some(hk_qp.capacity_exact.to_string()),
        outcome_action_exact: None,
        fg_capacity_f64: rational_to_f64_opt(&fg.capacity_action),
        certified_hk_qp_capacity_f64: rational_to_f64_opt(&hk_qp.capacity_exact),
        outcome_action_f64: None,
        checked_word_count: Some(fg.checked_word_count),
        retained_word_count: Some(fg.orbits.len()),
        direct_positive_word_count: None,
        direct_empty_or_no_orbit_word_count: None,
        action_cutoff_word_count: Some(fg.action_cutoff_word_count),
        action_cutoff_intersection_count: Some(fg.action_cutoff_intersection_count),
    }
}

fn exact_retained_words_match_direct_resolver(
    mode: &'static str,
    case_name: &str,
    provenance: Provenance,
    fg: &Result<ExactFlowGraphSearchResult, String>,
    direct: &Result<DirectResolverSummary, String>,
) -> ProofRiskRow {
    let fg = match fg {
        Ok(result) => result,
        Err(detail) => {
            return failed_row(
                "FG-PR-5",
                "retained_words_match_direct_exact_resolver",
                case_name,
                mode,
                provenance,
                detail.clone(),
            );
        }
    };
    let direct = match direct {
        Ok(summary) => summary,
        Err(detail) => {
            return failed_row(
                "FG-PR-5",
                "retained_words_match_direct_exact_resolver",
                case_name,
                mode,
                provenance,
                detail.clone(),
            );
        }
    };
    let actual = orbit_map(&fg.orbits);
    let passed = actual == direct.retained_words;
    ProofRiskRow {
        claim_id: "FG-PR-5",
        check_id: "retained_words_match_direct_exact_resolver",
        case_name: case_name.to_string(),
        mode,
        producer_command: provenance.producer_command,
        master_seed: provenance.master_seed,
        facet_count: provenance.facet_count,
        attempt: provenance.attempt,
        passed,
        detail: if passed {
            "search-retained capacity words match direct exact resolver outcomes under the FG convention"
                .to_string()
        } else {
            format!(
                "retained word mismatch: search retained {}, direct resolver retained {}",
                actual.len(),
                direct.retained_words.len()
            )
        },
        facets: None,
        outcome_kind: None,
        singular_status: None,
        singular_min_action_exact: None,
        singular_max_action_exact: None,
        fg_capacity_exact: Some(fg.capacity_action.to_string()),
        certified_hk_qp_capacity_exact: None,
        outcome_action_exact: None,
        fg_capacity_f64: rational_to_f64_opt(&fg.capacity_action),
        certified_hk_qp_capacity_f64: None,
        outcome_action_f64: None,
        checked_word_count: Some(fg.checked_word_count),
        retained_word_count: Some(fg.orbits.len()),
        direct_positive_word_count: Some(direct.positive_word_count),
        direct_empty_or_no_orbit_word_count: Some(direct.empty_or_no_orbit_word_count),
        action_cutoff_word_count: Some(fg.action_cutoff_word_count),
        action_cutoff_intersection_count: Some(fg.action_cutoff_intersection_count),
    }
}

fn cutoff_enabled_matches_disabled(
    mode: &'static str,
    case_name: &str,
    provenance: Provenance,
    baseline: &Result<ExactFlowGraphSearchResult, String>,
    cutoff: &Result<ExactFlowGraphSearchResult, String>,
) -> ProofRiskRow {
    let baseline = match baseline {
        Ok(result) => result,
        Err(detail) => {
            return failed_row(
                "FG-PR-5",
                "cutoff_enabled_matches_disabled",
                case_name,
                mode,
                provenance,
                format!("cutoff-disabled search failed: {detail}"),
            );
        }
    };
    let cutoff = match cutoff {
        Ok(result) => result,
        Err(detail) => {
            return failed_row(
                "FG-PR-5",
                "cutoff_enabled_matches_disabled",
                case_name,
                mode,
                provenance,
                format!("cutoff-enabled search failed: {detail}"),
            );
        }
    };

    let same_capacity = cutoff.capacity_action == baseline.capacity_action;
    let same_retained_words = cutoff.orbits == baseline.orbits;
    let cutoff_exercised =
        cutoff.action_cutoff_word_count > 0 && cutoff.action_cutoff_intersection_count > 0;
    let passed = same_capacity && same_retained_words && cutoff_exercised;
    ProofRiskRow {
        claim_id: "FG-PR-5",
        check_id: "cutoff_enabled_matches_disabled",
        case_name: case_name.to_string(),
        mode,
        producer_command: provenance.producer_command,
        master_seed: provenance.master_seed,
        facet_count: provenance.facet_count,
        attempt: provenance.attempt,
        passed,
        detail: if !same_capacity || !same_retained_words {
            "cutoff-enabled exact search changed capacity or retained words".to_string()
        } else if !cutoff_exercised {
            "cutoff-enabled exact search matched output but did not exercise cutoff intersections"
                .to_string()
        } else {
            "cutoff-enabled exact search matches disabled output; cutoff intersections were exercised, but this row is not a cutoff lower-bound certificate"
                .to_string()
        },
        facets: None,
        outcome_kind: None,
        singular_status: None,
        singular_min_action_exact: None,
        singular_max_action_exact: None,
        fg_capacity_exact: Some(cutoff.capacity_action.to_string()),
        certified_hk_qp_capacity_exact: None,
        outcome_action_exact: None,
        fg_capacity_f64: rational_to_f64_opt(&cutoff.capacity_action),
        certified_hk_qp_capacity_f64: None,
        outcome_action_f64: None,
        checked_word_count: Some(cutoff.checked_word_count),
        retained_word_count: Some(cutoff.orbits.len()),
        direct_positive_word_count: None,
        direct_empty_or_no_orbit_word_count: None,
        action_cutoff_word_count: Some(cutoff.action_cutoff_word_count),
        action_cutoff_intersection_count: Some(cutoff.action_cutoff_intersection_count),
    }
}

fn retained_positive_word_resolves_directly(
    mode: &'static str,
    case_name: &str,
    provenance: Provenance,
    input: &ExactFlatTubeInput<'_>,
    fg: &Result<ExactFlowGraphSearchResult, String>,
) -> ProofRiskRow {
    let fg = match fg {
        Ok(result) => result,
        Err(detail) => {
            return failed_row(
                "FG-PR-3",
                "retained_positive_word_resolves_directly",
                case_name,
                mode,
                provenance,
                detail.clone(),
            );
        }
    };
    let Some(orbit) = fg.orbits.first() else {
        return failed_row(
            "FG-PR-3",
            "retained_positive_word_resolves_directly",
            case_name,
            mode,
            provenance,
            "exact search returned no retained positive word",
        );
    };
    closed_word_outcome_row(
        "FG-PR-3",
        "retained_positive_word_resolves_directly",
        case_name,
        mode,
        provenance,
        input,
        &orbit.facets,
        Some(&orbit.action),
        "retained capacity word resolves directly as a positive exact closed word with matching action",
    )
}

fn zero_omega_rejection_rows(
    mode: &'static str,
    producer_command: &'static str,
) -> Vec<ProofRiskRow> {
    let fixtures = [
        known_polytopes::hko_pentagon(),
        known_polytopes::lagrangian_triangle_product(),
        known_polytopes::lagrangian_triangle_square(),
    ];
    fixtures
        .into_iter()
        .map(|fixture| {
            let provenance = Provenance {
                producer_command,
                master_seed: None,
                facet_count: Some(fixture.dual_vertices.len()),
                attempt: None,
            };
            let input = ExactFlatTubeInput {
                dual_vertices: &fixture.dual_vertices,
                facet_intersection_is_nonempty: &fixture.facet_intersection_is_nonempty,
                omega_signs: &fixture.omega_signs,
            };
            let result = search_closed_orbits_exact(
                &input,
                BigRational::zero(),
                ExactActionCutoffPolicy::Disabled,
            );
            let passed = matches!(
                result,
                Err(ExactFlowGraphSearchError::UnsupportedZeroOmegaTransition { .. })
            );
            ProofRiskRow {
                claim_id: "FG-PR-3",
                check_id: "zero_omega_fixture_rejected",
                case_name: fixture.name.to_string(),
                mode,
                producer_command: provenance.producer_command,
                master_seed: provenance.master_seed,
                facet_count: provenance.facet_count,
                attempt: provenance.attempt,
                passed,
                detail: if passed {
                    "known zero-omega fixture is rejected before exact FG capacity output"
                        .to_string()
                } else {
                    format!("expected UnsupportedZeroOmegaTransition, got {result:?}")
                },
                facets: None,
                outcome_kind: None,
                singular_status: None,
                singular_min_action_exact: None,
                singular_max_action_exact: None,
                fg_capacity_exact: None,
                certified_hk_qp_capacity_exact: None,
                outcome_action_exact: None,
                fg_capacity_f64: None,
                certified_hk_qp_capacity_f64: None,
                outcome_action_f64: None,
                checked_word_count: None,
                retained_word_count: None,
                direct_positive_word_count: None,
                direct_empty_or_no_orbit_word_count: None,
                action_cutoff_word_count: None,
                action_cutoff_intersection_count: None,
            }
        })
        .collect()
}

fn specific_closed_word_rows(
    mode: &'static str,
    producer_command: &'static str,
) -> Vec<ProofRiskRow> {
    let mut rows = Vec::new();
    let hypercube = known_polytopes::hypercube();
    let hypercube_provenance = Provenance {
        producer_command,
        master_seed: None,
        facet_count: Some(hypercube.dual_vertices.len()),
        attempt: None,
    };
    let hypercube_input = ExactFlatTubeInput {
        dual_vertices: &hypercube.dual_vertices,
        facet_intersection_is_nonempty: &hypercube.facet_intersection_is_nonempty,
        omega_signs: &hypercube.omega_signs,
    };
    rows.push(closed_word_expected_kind_row(
        "FG-PR-3",
        "positive_singular_word_is_typed_unsupported",
        hypercube.name,
        mode,
        hypercube_provenance,
        &hypercube_input,
        &[0, 4, 1, 5],
        |outcome| {
            matches!(
                outcome,
                ExactClosedWordOutcome::UnsupportedPositiveSingular {
                    singular_status: "singular_all_points",
                    min_action: Some(min_action),
                    max_action: Some(max_action),
                } if *min_action == integer_rational(4) && *max_action == integer_rational(4)
            )
        },
        "known positive-action singular hypercube word is typed unsupported, not accepted as a capacity output",
    ));

    let length_three_case = GeneratedCase {
        case_name: "generated_F5_attempt60",
        facet_count: 5,
        attempt: 60,
        smoke: true,
    };
    let length_three_provenance = Provenance {
        producer_command,
        master_seed: Some(MASTER_SEED),
        facet_count: Some(length_three_case.facet_count),
        attempt: Some(length_three_case.attempt),
    };
    if let Some(length_three_polytope) = deterministic_exact_admissible_case(length_three_case) {
        let length_three_input = exact_input(&length_three_polytope);
        rows.push(closed_word_expected_kind_row(
            "FG-PR-3",
            "length_three_word_is_zero_time_no_orbit",
            length_three_case.case_name,
            mode,
            length_three_provenance,
            &length_three_input,
            &[0, 2, 4],
            |outcome| {
                matches!(
                    outcome,
                    ExactClosedWordOutcome::ZeroActionNoOrbit {
                        action: Some(action),
                        singular_status: Some("length_three_zero_time"),
                        ..
                    } if action.is_zero()
                )
            },
            "known length-three word resolves as a structural zero-time no-orbit, not a singular rejection",
        ));
    } else {
        rows.push(failed_row(
            "FG-PR-3",
            "length_three_word_is_zero_time_no_orbit",
            length_three_case.case_name,
            mode,
            length_three_provenance,
            "F5 attempt60 did not pass exact-admissible cache reconstruction",
        ));
    }

    let case = GeneratedCase {
        case_name: "generated_F7_attempt31",
        facet_count: 7,
        attempt: 31,
        smoke: false,
    };
    let provenance = Provenance {
        producer_command,
        master_seed: Some(MASTER_SEED),
        facet_count: Some(case.facet_count),
        attempt: Some(case.attempt),
    };
    let Some(polytope) = deterministic_exact_admissible_case(case) else {
        return vec![failed_row(
            "FG-PR-3",
            "specific_closed_word_reconstructs_case",
            case.case_name,
            mode,
            provenance,
            "F7 attempt31 did not pass exact-admissible cache reconstruction",
        )];
    };
    let input = exact_input(&polytope);
    let zero_action_word = closed_word_expected_kind_row(
        "FG-PR-3",
        "zero_action_word_is_typed_no_orbit",
        case.case_name,
        mode,
        provenance,
        &input,
        &[0, 4, 2, 6],
        |outcome| {
            matches!(
                outcome,
                ExactClosedWordOutcome::ZeroActionNoOrbit {
                    action: Some(action),
                    ..
                } if action.is_zero()
            )
        },
        "known zero-action F7 word resolves as typed no-orbit, not positive capacity output",
    );

    let positive_word = closed_word_outcome_row(
        "FG-PR-3",
        "known_positive_f7_word_resolves_directly",
        case.case_name,
        mode,
        provenance,
        &input,
        &[0, 1, 5, 6, 4, 2],
        None,
        "known positive F7 word resolves directly as positive exact closed word",
    );

    rows.push(zero_action_word);
    rows.push(positive_word);
    rows
}

fn integer_rational(value: i64) -> BigRational {
    BigRational::from_integer(BigInt::from(value))
}

fn closed_word_outcome_row(
    claim_id: &'static str,
    check_id: &'static str,
    case_name: &str,
    mode: &'static str,
    provenance: Provenance,
    input: &ExactFlatTubeInput<'_>,
    sigma: &[usize],
    expected_action: Option<&BigRational>,
    success_detail: &'static str,
) -> ProofRiskRow {
    closed_word_expected_kind_row(
        claim_id,
        check_id,
        case_name,
        mode,
        provenance,
        input,
        sigma,
        |outcome| match outcome {
            ExactClosedWordOutcome::PositiveOrbit { action, .. } => expected_action
                .map(|expected| expected == action)
                .unwrap_or(true),
            _ => false,
        },
        success_detail,
    )
}

fn closed_word_expected_kind_row<F>(
    claim_id: &'static str,
    check_id: &'static str,
    case_name: &str,
    mode: &'static str,
    provenance: Provenance,
    input: &ExactFlatTubeInput<'_>,
    sigma: &[usize],
    predicate: F,
    success_detail: &'static str,
) -> ProofRiskRow
where
    F: FnOnce(&ExactClosedWordOutcome) -> bool,
{
    let result = match resolve_closed_word_exact(input, sigma) {
        Ok((result, _metrics)) => result,
        Err(error) => {
            return failed_row(
                claim_id,
                check_id,
                case_name,
                mode,
                provenance,
                format!("closed-word resolver failed for {sigma:?}: {error:?}"),
            );
        }
    };
    let passed = predicate(&result.outcome);
    let outcome_action = outcome_action(&result.outcome);
    ProofRiskRow {
        claim_id,
        check_id,
        case_name: case_name.to_string(),
        mode,
        producer_command: provenance.producer_command,
        master_seed: provenance.master_seed,
        facet_count: provenance.facet_count,
        attempt: provenance.attempt,
        passed,
        detail: if passed {
            success_detail.to_string()
        } else {
            format!(
                "closed word {sigma:?} had unexpected outcome {:?}",
                result.outcome
            )
        },
        facets: Some(sigma.to_vec()),
        outcome_kind: Some(outcome_kind(&result.outcome)),
        singular_status: singular_status(&result.outcome),
        singular_min_action_exact: singular_min_action(&result.outcome)
            .map(|value| value.to_string()),
        singular_max_action_exact: singular_max_action(&result.outcome)
            .map(|value| value.to_string()),
        fg_capacity_exact: None,
        certified_hk_qp_capacity_exact: None,
        outcome_action_exact: outcome_action.as_ref().map(ToString::to_string),
        fg_capacity_f64: None,
        certified_hk_qp_capacity_f64: None,
        outcome_action_f64: outcome_action.as_ref().and_then(rational_to_f64_opt),
        checked_word_count: None,
        retained_word_count: None,
        direct_positive_word_count: None,
        direct_empty_or_no_orbit_word_count: None,
        action_cutoff_word_count: None,
        action_cutoff_intersection_count: None,
    }
}

fn exact_search(
    input: &ExactFlatTubeInput<'_>,
    action_threshold: BigRational,
    policy: ExactActionCutoffPolicy,
) -> Result<ExactFlowGraphSearchResult, String> {
    search_closed_orbits_exact(input, action_threshold, policy)
        .map_err(|error| format!("exact FG search failed: {error:?}"))
}

fn certified_hk_qp_capacity(
    polytope: &VerificationPolytopeCache,
    action_gap_exact: BigRational,
) -> Result<CertifiedOrbitSearchResult, symplectic::OrbitSearchError> {
    let transition_is_allowed = build_transition_matrix_from_facet_intersections_and_omega(
        &polytope.facet_intersection_is_nonempty,
        &polytope.omega_signs,
    );
    let (orbits, iterations) =
        solve_pruned_hk2017_candidates(&polytope.dual_vertices_f64, &transition_is_allowed)?;
    aggregate_certified_orbits_with_dual_vertices_exact(
        &polytope.dual_vertices,
        orbits,
        iterations,
        action_gap_exact,
        CertifiedOrbitSetMode::GapWindow,
    )
}

struct DirectResolverSummary {
    retained_words: BTreeMap<Vec<usize>, BigRational>,
    positive_word_count: usize,
    empty_or_no_orbit_word_count: usize,
}

fn direct_resolver_summary(
    input: &ExactFlatTubeInput<'_>,
    action_cutoff: &BigRational,
) -> Result<DirectResolverSummary, String> {
    let transition_is_allowed = build_transition_matrix_from_facet_intersections_and_omega(
        input.facet_intersection_is_nonempty,
        input.omega_signs,
    );
    let mut retained_words = BTreeMap::new();
    let mut positive_word_count = 0usize;
    let mut empty_or_no_orbit_word_count = 0usize;
    let mut failure = None;

    for_each_sigma_pruned_by_transition(&transition_is_allowed, |sigma| {
        if failure.is_some() {
            return;
        }
        let result = match resolve_closed_word_exact(input, sigma) {
            Ok((result, _metrics)) => result,
            Err(error) => {
                failure = Some(format!("direct resolver failed for {sigma:?}: {error:?}"));
                return;
            }
        };
        match result.outcome {
            ExactClosedWordOutcome::EmptyTube | ExactClosedWordOutcome::NonStrictNoOrbit { .. } => {
                empty_or_no_orbit_word_count += 1;
            }
            ExactClosedWordOutcome::ZeroActionNoOrbit {
                singular_status: Some("length_three_zero_time"),
                ..
            }
            | ExactClosedWordOutcome::ZeroActionNoOrbit {
                singular_status: None,
                ..
            } => {
                empty_or_no_orbit_word_count += 1;
            }
            ExactClosedWordOutcome::ZeroActionNoOrbit {
                singular_status: Some(singular_status),
                ..
            } => {
                failure = Some(format!(
                    "direct resolver hit unsupported non-length-three singular no-orbit word {sigma:?}: {singular_status}"
                ));
            }
            ExactClosedWordOutcome::PositiveOrbit { action, .. } => {
                positive_word_count += 1;
                if action <= *action_cutoff {
                    let canonical = canonical_cyclic_word(sigma);
                    if let Some(previous) = retained_words.insert(canonical.clone(), action.clone())
                    {
                        if previous != action {
                            failure = Some(format!(
                                "cyclic duplicate {canonical:?} had inconsistent actions"
                            ));
                        }
                    }
                }
            }
            ExactClosedWordOutcome::UnsupportedPositiveSingular { .. } => {
                failure = Some(format!(
                    "direct resolver hit unsupported positive singular word {sigma:?}"
                ));
            }
        }
    });

    match failure {
        Some(detail) => Err(detail),
        None => Ok(DirectResolverSummary {
            retained_words,
            positive_word_count,
            empty_or_no_orbit_word_count,
        }),
    }
}

fn orbit_map(orbits: &[ExactFlowGraphOrbit]) -> BTreeMap<Vec<usize>, BigRational> {
    let mut map = BTreeMap::new();
    for orbit in orbits {
        let canonical = canonical_cyclic_word(&orbit.facets);
        if let Some(previous) = map.insert(canonical.clone(), orbit.action.clone()) {
            assert_eq!(
                previous, orbit.action,
                "cyclic duplicate {canonical:?} had inconsistent exact actions"
            );
        }
    }
    map
}

fn canonical_cyclic_word(word: &[usize]) -> Vec<usize> {
    assert!(!word.is_empty(), "cyclic word must be nonempty");
    (0..word.len())
        .map(|offset| {
            word.iter()
                .cycle()
                .skip(offset)
                .take(word.len())
                .copied()
                .collect::<Vec<_>>()
        })
        .min()
        .expect("nonempty cyclic word has rotations")
}

fn outcome_kind(outcome: &ExactClosedWordOutcome) -> &'static str {
    match outcome {
        ExactClosedWordOutcome::EmptyTube => "empty_tube",
        ExactClosedWordOutcome::ZeroActionNoOrbit { .. } => "zero_action_no_orbit",
        ExactClosedWordOutcome::NonStrictNoOrbit { .. } => "non_strict_no_orbit",
        ExactClosedWordOutcome::PositiveOrbit { .. } => "positive_orbit",
        ExactClosedWordOutcome::UnsupportedPositiveSingular { .. } => {
            "unsupported_positive_singular"
        }
    }
}

fn outcome_action(outcome: &ExactClosedWordOutcome) -> Option<BigRational> {
    match outcome {
        ExactClosedWordOutcome::ZeroActionNoOrbit { action, .. } => action.clone(),
        ExactClosedWordOutcome::NonStrictNoOrbit { action, .. }
        | ExactClosedWordOutcome::PositiveOrbit { action, .. } => Some(action.clone()),
        ExactClosedWordOutcome::EmptyTube
        | ExactClosedWordOutcome::UnsupportedPositiveSingular { .. } => None,
    }
}

fn singular_status(outcome: &ExactClosedWordOutcome) -> Option<&'static str> {
    match outcome {
        ExactClosedWordOutcome::ZeroActionNoOrbit {
            singular_status, ..
        } => *singular_status,
        ExactClosedWordOutcome::UnsupportedPositiveSingular {
            singular_status, ..
        } => Some(singular_status),
        _ => None,
    }
}

fn singular_min_action(outcome: &ExactClosedWordOutcome) -> Option<BigRational> {
    match outcome {
        ExactClosedWordOutcome::UnsupportedPositiveSingular { min_action, .. } => {
            min_action.clone()
        }
        _ => None,
    }
}

fn singular_max_action(outcome: &ExactClosedWordOutcome) -> Option<BigRational> {
    match outcome {
        ExactClosedWordOutcome::UnsupportedPositiveSingular { max_action, .. } => {
            max_action.clone()
        }
        _ => None,
    }
}

fn rational_to_f64_opt(value: &BigRational) -> Option<f64> {
    value.to_f64()
}

fn failed_row(
    claim_id: &'static str,
    check_id: &'static str,
    case_name: impl Into<String>,
    mode: &'static str,
    provenance: Provenance,
    detail: impl Into<String>,
) -> ProofRiskRow {
    ProofRiskRow {
        claim_id,
        check_id,
        case_name: case_name.into(),
        mode,
        producer_command: provenance.producer_command,
        master_seed: provenance.master_seed,
        facet_count: provenance.facet_count,
        attempt: provenance.attempt,
        passed: false,
        detail: detail.into(),
        facets: None,
        outcome_kind: None,
        singular_status: None,
        singular_min_action_exact: None,
        singular_max_action_exact: None,
        fg_capacity_exact: None,
        certified_hk_qp_capacity_exact: None,
        outcome_action_exact: None,
        fg_capacity_f64: None,
        certified_hk_qp_capacity_f64: None,
        outcome_action_f64: None,
        checked_word_count: None,
        retained_word_count: None,
        direct_positive_word_count: None,
        direct_empty_or_no_orbit_word_count: None,
        action_cutoff_word_count: None,
        action_cutoff_intersection_count: None,
    }
}

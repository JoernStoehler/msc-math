use exp_dev_f64_capacity::{
    load_retained_artifact_cases, scan_case_with_options_profiled, F64CapacityMethod,
    F64ValidationPolicy, ProductSimplificationPolicy, ScanCase, ScanOptions, ScanRow,
    ScanTimingBreakdown,
};
use exp_performance::args::{selected_run_mode, split_inline_arg, take_value, RunMode};
use exp_performance::jsonl::JsonlWriter;
use exp_performance::output_dir::prepare_out_dir;
use exp_performance::timing::timed;
use serde::Serialize;
use std::env;
use std::path::PathBuf;
use std::process::ExitCode;
use tracing::info_span;
use tracing_subscriber::fmt::format::FmtSpan;

const TARGET_NAME: &str = "f64-capacity-e2e";
const SMOKE_MAX_ROWS_PER_FAMILY: usize = 2;
const PRODUCTION_MAX_ROWS_PER_FAMILY: usize = 100;

#[derive(Clone, Debug, Serialize)]
struct Config {
    mode: RunMode,
    max_rows_per_family: usize,
    max_cases: Option<usize>,
    case_filter: CaseFilter,
    method_filter: MethodFilter,
    trace: bool,
    out_dir: Option<PathBuf>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
enum CaseFilter {
    All,
    RandomProductF12,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
enum MethodFilter {
    All,
    ProductBilliardOrHk,
}

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
enum Method {
    CandidateDefault,
    ProductBilliardOrHk,
    Strict,
    Lp,
}

impl Method {
    fn label(self) -> &'static str {
        match self {
            Self::CandidateDefault => "lp_origin_vertex",
            Self::ProductBilliardOrHk => "lp_origin_vertex_product_billiard_or_hk",
            Self::Strict => "strict",
            Self::Lp => "lp",
        }
    }

    fn validation_policy(self) -> F64ValidationPolicy {
        match self {
            Self::CandidateDefault => F64ValidationPolicy::LpOriginVertex,
            Self::ProductBilliardOrHk => F64ValidationPolicy::LpOriginVertex,
            Self::Strict => F64ValidationPolicy::Strict,
            Self::Lp => F64ValidationPolicy::Lp,
        }
    }

    fn capacity_method(self) -> F64CapacityMethod {
        match self {
            Self::ProductBilliardOrHk => F64CapacityMethod::ProductBilliardOrHk,
            Self::CandidateDefault | Self::Strict | Self::Lp => {
                F64CapacityMethod::TransitionPrunedHk
            }
        }
    }
}

const METHODS: [Method; 4] = [
    Method::CandidateDefault,
    Method::ProductBilliardOrHk,
    Method::Strict,
    Method::Lp,
];

#[derive(Serialize)]
struct PhaseEvent {
    target: &'static str,
    mode: RunMode,
    family: String,
    source_id: String,
    facet_count: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    original_facet_count: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    product_rounding_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    product_rounding_max_minor_over_major: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    product_rounding_max_abs_change: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    product_simplification_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    removed_facet_count: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    product_simplification_delta_bound: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    capacity_ratio_upper_bound: Option<f64>,
    sample: usize,
    method: &'static str,
    phase: &'static str,
    elapsed_ms: f64,
    status: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    validation_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    trust_class: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    agreement_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    outcome: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    validation_bundle_time_ms: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    capacity_bundle_time_ms: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    capacity_ran: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    validation_sanity_ms: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    validation_origin_lp_diagnostic_ms: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    validation_origin_policy_predicate_ms: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    validation_combinatorics_ms: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    validation_classification_ms: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    validation_geometry_vertex_scan_ms: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    validation_geometry_facet_intersections_ms: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    validation_geometry_omega_signs_ms: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    validation_lp_facet_statuses_ms: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    validation_lp_facet_intersections_ms: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    validation_lp_omega_recompute_ms: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    capacity_combinatorics_ms: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    capacity_transition_matrix_ms: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    capacity_candidate_solve_ms: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    capacity_candidate_kkt_solve_ms: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    capacity_candidate_non_kkt_ms: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    capacity_report_ms: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    capacity_geometry_vertex_scan_ms: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    capacity_geometry_facet_intersections_ms: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    capacity_geometry_omega_signs_ms: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    capacity_lp_facet_statuses_ms: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    capacity_lp_facet_intersections_ms: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    capacity_lp_omega_recompute_ms: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sigma_count: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    admissible_f64_count: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    indeterminate_f64_count: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inadmissible_count: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    numerical_failure_count: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    facet_intersection_true_count: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    facet_intersection_false_count: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    facet_intersection_indeterminate_count: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    omega_indeterminate_count: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vertex_indeterminate_count: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    abs_action_error: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rel_action_error: Option<f64>,
}

fn main() -> ExitCode {
    match run() {
        Ok(out_dir) => {
            println!("{}", out_dir.display());
            ExitCode::SUCCESS
        }
        Err(message) => {
            eprintln!("{message}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<PathBuf, String> {
    let config = parse_args(env::args().skip(1))?;
    validate_config(&config)?;
    let out_dir = prepare_out_dir(config.out_dir.clone(), TARGET_NAME, config.mode.as_str())?;
    if config.trace {
        init_tracing()?;
    }

    let cases = selected_cases(&config);
    let mut phase_events = JsonlWriter::create(&out_dir.join("phase-events.jsonl"))?;
    for (sample, case) in cases.into_iter().enumerate() {
        for &method in selected_methods(config.method_filter) {
            profile_case_method(&config, sample, &case, method, &mut phase_events)?;
        }
    }
    phase_events.flush()?;

    Ok(out_dir)
}

fn selected_cases(config: &Config) -> Vec<ScanCase> {
    let max_rows_per_family = match config.case_filter {
        CaseFilter::All => config.max_rows_per_family,
        CaseFilter::RandomProductF12 => 0,
    };
    let cases = load_retained_artifact_cases(max_rows_per_family);
    let filtered = cases.into_iter().filter(|case| match config.case_filter {
        CaseFilter::All => true,
        CaseFilter::RandomProductF12 => {
            case.family == "random_product" && case.dual_vertices.len() == 12
        }
    });
    match config.max_cases {
        Some(limit) => filtered.take(limit).collect(),
        None => filtered.collect(),
    }
}

fn selected_methods(filter: MethodFilter) -> &'static [Method] {
    match filter {
        MethodFilter::All => &METHODS,
        MethodFilter::ProductBilliardOrHk => &[Method::ProductBilliardOrHk],
    }
}

fn profile_case_method(
    config: &Config,
    sample: usize,
    case: &ScanCase,
    method: Method,
    phase_events: &mut JsonlWriter,
) -> Result<(), String> {
    let _span = info_span!(
        "performance_sample",
        target = TARGET_NAME,
        family = case.family.as_str(),
        source_id = case.source_id.as_str(),
        facet_count = case.dual_vertices.len(),
        method = method.label(),
    )
    .entered();
    let ((row, timing), elapsed_ms) = timed(|| phase_f64_capacity_e2e(case.clone(), method));
    phase_events.write(&event_from_row(
        config, sample, method, elapsed_ms, row, timing,
    ))
}

#[inline(never)]
fn phase_f64_capacity_e2e(case: ScanCase, method: Method) -> (ScanRow, ScanTimingBreakdown) {
    scan_case_with_options_profiled(
        case,
        &ScanOptions {
            audit_generated: false,
            audit_simplified: false,
            validation_policy: method.validation_policy(),
            capacity_method: method.capacity_method(),
            product_simplification: ProductSimplificationPolicy::None,
            product_simplification_delta: 1e-8,
        },
    )
}

fn event_from_row(
    config: &Config,
    sample: usize,
    method: Method,
    elapsed_ms: f64,
    row: ScanRow,
    timing: ScanTimingBreakdown,
) -> PhaseEvent {
    let capacity_ran = row.outcome != "not_run";
    let capacity_timing = timing.capacity.as_ref();
    PhaseEvent {
        target: TARGET_NAME,
        mode: config.mode,
        family: row.family,
        source_id: row.source_id,
        facet_count: row.facet_count,
        original_facet_count: row.original_facet_count,
        product_rounding_status: Some(row.product_rounding_status),
        product_rounding_max_minor_over_major: row.product_rounding_max_minor_over_major,
        product_rounding_max_abs_change: row.product_rounding_max_abs_change,
        product_simplification_status: Some(row.product_simplification_status),
        removed_facet_count: Some(row.removed_facet_count),
        product_simplification_delta_bound: row.product_simplification_delta_bound,
        capacity_ratio_upper_bound: row.capacity_ratio_upper_bound,
        sample,
        method: method.label(),
        phase: "f64_capacity_e2e",
        elapsed_ms,
        status: "ok",
        error: None,
        validation_status: Some(row.validation_status),
        trust_class: Some(row.trust_class),
        agreement_status: Some(row.agreement_status),
        outcome: Some(row.outcome),
        validation_bundle_time_ms: Some(row.validation_time_ms),
        capacity_bundle_time_ms: capacity_ran.then_some(row.f64_time_ms),
        capacity_ran: Some(capacity_ran),
        validation_sanity_ms: Some(timing.validation.sanity_ms),
        validation_origin_lp_diagnostic_ms: Some(timing.validation.origin_lp_diagnostic_ms),
        validation_origin_policy_predicate_ms: Some(timing.validation.origin_policy_predicate_ms),
        validation_combinatorics_ms: Some(timing.validation.combinatorics_ms),
        validation_classification_ms: Some(timing.validation.classification_ms),
        validation_geometry_vertex_scan_ms: Some(timing.validation.geometry.vertex_scan_ms),
        validation_geometry_facet_intersections_ms: Some(
            timing.validation.geometry.facet_intersections_ms,
        ),
        validation_geometry_omega_signs_ms: Some(timing.validation.geometry.omega_signs_ms),
        validation_lp_facet_statuses_ms: Some(timing.validation.geometry.lp_facet_statuses_ms),
        validation_lp_facet_intersections_ms: Some(
            timing.validation.geometry.lp_facet_intersections_ms,
        ),
        validation_lp_omega_recompute_ms: Some(timing.validation.geometry.lp_omega_recompute_ms),
        capacity_combinatorics_ms: capacity_timing.map(|timing| timing.combinatorics_ms),
        capacity_transition_matrix_ms: capacity_timing.map(|timing| timing.transition_matrix_ms),
        capacity_candidate_solve_ms: capacity_timing.map(|timing| timing.candidate_solve_ms),
        capacity_candidate_kkt_solve_ms: capacity_timing
            .map(|timing| timing.candidate_kkt_solve_ms),
        capacity_candidate_non_kkt_ms: capacity_timing.map(|timing| timing.candidate_non_kkt_ms),
        capacity_report_ms: capacity_timing.map(|timing| timing.report_ms),
        capacity_geometry_vertex_scan_ms: capacity_timing
            .map(|timing| timing.geometry.vertex_scan_ms),
        capacity_geometry_facet_intersections_ms: capacity_timing
            .map(|timing| timing.geometry.facet_intersections_ms),
        capacity_geometry_omega_signs_ms: capacity_timing
            .map(|timing| timing.geometry.omega_signs_ms),
        capacity_lp_facet_statuses_ms: capacity_timing
            .map(|timing| timing.geometry.lp_facet_statuses_ms),
        capacity_lp_facet_intersections_ms: capacity_timing
            .map(|timing| timing.geometry.lp_facet_intersections_ms),
        capacity_lp_omega_recompute_ms: capacity_timing
            .map(|timing| timing.geometry.lp_omega_recompute_ms),
        sigma_count: Some(row.sigma_count),
        admissible_f64_count: Some(row.admissible_f64_count),
        indeterminate_f64_count: Some(row.indeterminate_f64_count),
        inadmissible_count: Some(row.inadmissible_count),
        numerical_failure_count: Some(row.numerical_failure_count),
        facet_intersection_true_count: Some(row.facet_intersection_true_count),
        facet_intersection_false_count: Some(row.facet_intersection_false_count),
        facet_intersection_indeterminate_count: Some(row.facet_intersection_indeterminate_count),
        omega_indeterminate_count: Some(row.omega_indeterminate_count),
        vertex_indeterminate_count: Some(row.vertex_indeterminate_count),
        abs_action_error: row.abs_action_error,
        rel_action_error: row.rel_action_error,
    }
}

fn parse_args(args: impl Iterator<Item = String>) -> Result<Config, String> {
    let args: Vec<String> = args.collect();
    let mut config = config_for_mode(selected_run_mode(&args)?);

    let mut args = args.into_iter().peekable();
    while let Some(arg) = args.next() {
        let (flag, inline_value) = split_inline_arg(arg);
        match flag.as_str() {
            "--mode" => {
                let _ = take_value("--mode", inline_value, &mut args)?;
            }
            "--out-dir" => {
                let value = take_value("--out-dir", inline_value, &mut args)?;
                config.out_dir = Some(PathBuf::from(value));
            }
            "--trace" => {
                if inline_value.is_some() {
                    return Err("--trace does not take a value".to_owned());
                }
                config.trace = true;
            }
            "--case-filter" => {
                let value = take_value("--case-filter", inline_value, &mut args)?;
                config.case_filter = match value.as_str() {
                    "all" => CaseFilter::All,
                    "random_product_f12" => CaseFilter::RandomProductF12,
                    other => {
                        return Err(format!(
                            "--case-filter must be all or random_product_f12, got {other}"
                        ))
                    }
                };
            }
            "--method-filter" => {
                let value = take_value("--method-filter", inline_value, &mut args)?;
                config.method_filter = match value.as_str() {
                    "all" => MethodFilter::All,
                    "product_billiard_or_hk" => MethodFilter::ProductBilliardOrHk,
                    other => {
                        return Err(format!(
                            "--method-filter must be all or product_billiard_or_hk, got {other}"
                        ))
                    }
                };
            }
            "--max-cases" => {
                config.max_cases = Some(
                    take_value("--max-cases", inline_value, &mut args)?
                        .parse()
                        .map_err(|_| "--max-cases must be a positive integer".to_string())?,
                );
            }
            "--help" | "-h" => {
                print_usage();
                std::process::exit(0);
            }
            other => return Err(format!("unknown argument: {other}\n\n{}", usage())),
        }
    }

    Ok(config)
}

fn config_for_mode(mode: RunMode) -> Config {
    let max_rows_per_family = match mode {
        RunMode::Smoke => SMOKE_MAX_ROWS_PER_FAMILY,
        RunMode::Production => PRODUCTION_MAX_ROWS_PER_FAMILY,
    };
    Config {
        mode,
        max_rows_per_family,
        max_cases: None,
        case_filter: CaseFilter::All,
        method_filter: MethodFilter::All,
        trace: false,
        out_dir: None,
    }
}

fn validate_config(config: &Config) -> Result<(), String> {
    if config.max_rows_per_family == 0 {
        return Err("mode max_rows_per_family must be at least 1".to_string());
    }
    Ok(())
}

fn init_tracing() -> Result<(), String> {
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_target(false)
        .with_span_events(FmtSpan::CLOSE)
        .compact()
        .try_init()
        .map_err(|error| format!("initialize tracing subscriber: {error}"))
}

fn print_usage() {
    println!("{}", usage());
}

fn usage() -> &'static str {
    "Usage: cargo run -p exp-performance --release --bin f64-capacity-e2e -- \\
        --mode production --out-dir /tmp/perf-f64-capacity-e2e\n\
\n\
Options:\n\
  --mode MODE          Named run mode: smoke or production [default: smoke]\n\
  --case-filter FILTER Input cohort: all or random_product_f12 [default: all]\n\
  --method-filter FILTER Methods: all or product_billiard_or_hk [default: all]\n\
  --max-cases N        Stop after N selected input rows\n\
  --out-dir PATH       Output directory [default: /tmp/msc-math-performance/<target>-<mode>-<time>-pid<PID>]\n\
  --trace              Emit tracing span close events to stderr\n\
  --help               Print this help text"
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(values: &[&str]) -> Config {
        parse_args(values.iter().map(|value| value.to_string())).unwrap()
    }

    #[test]
    fn smoke_mode_is_default() {
        let config = parse(&[]);
        assert_eq!(config.mode, RunMode::Smoke);
        assert_eq!(config.max_rows_per_family, SMOKE_MAX_ROWS_PER_FAMILY);
    }

    #[test]
    fn production_mode_selects_documented_profile_size() {
        let config = parse(&["--mode", "production"]);
        assert_eq!(config.mode, RunMode::Production);
        assert_eq!(config.max_rows_per_family, PRODUCTION_MAX_ROWS_PER_FAMILY);
    }

    #[test]
    fn ad_hoc_input_selector_flags_are_rejected() {
        for flag in ["--max-rows-per-family", "--input-source", "--method"] {
            assert!(parse_args([flag.to_string(), "1".to_string()].into_iter()).is_err());
        }
    }

    #[test]
    fn product_f12_diagnostic_filters_are_accepted() {
        let config = parse(&[
            "--case-filter",
            "random_product_f12",
            "--method-filter",
            "product_billiard_or_hk",
            "--max-cases",
            "3",
        ]);
        assert_eq!(config.case_filter, CaseFilter::RandomProductF12);
        assert_eq!(config.method_filter, MethodFilter::ProductBilliardOrHk);
        assert_eq!(config.max_cases, Some(3));
    }
}

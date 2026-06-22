use exp_dev_quadratic_program::{
    compare_f64_decisions, generated_f64_cases, load_retained_artifact_cases, ScanCase,
};
mod support;
use serde::Serialize;
use std::env;
use std::path::PathBuf;
use std::process::ExitCode;
use support::args::{selected_run_mode, split_inline_arg, take_value, RunMode};
use support::jsonl::JsonlWriter;
use support::output_dir::prepare_out_dir;

const TARGET_NAME: &str = "f64-decision-compare";
const SMOKE_MAX_ROWS_PER_FAMILY: usize = 2;
const PRODUCTION_MAX_ROWS_PER_FAMILY: usize = 100;
const SMOKE_GENERATED_SAMPLES_PER_FACET: usize = 1;
const PRODUCTION_GENERATED_SAMPLES_PER_FACET: usize = 5;
const DEFAULT_GENERATED_SEED: u64 = 99_599_604;

#[derive(Clone, Debug, Serialize)]
struct Config {
    mode: RunMode,
    max_rows_per_family: usize,
    generated_samples_per_facet: usize,
    generated_seed: u64,
    input_cohort: InputCohort,
    out_dir: Option<PathBuf>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
enum InputCohort {
    RetainedArtifacts,
    GeneratedF64,
    All,
}

#[derive(Serialize)]
struct ComparisonEvent {
    target: &'static str,
    mode: RunMode,
    family: String,
    source_id: String,
    facet_count: usize,
    sample: usize,
    decision: &'static str,
    left_method: String,
    right_method: Option<String>,
    left_time_ms: f64,
    right_time_ms: Option<f64>,
    left_true_count: usize,
    left_false_count: usize,
    left_indeterminate_count: usize,
    left_error_count: usize,
    right_true_count: Option<usize>,
    right_false_count: Option<usize>,
    right_indeterminate_count: Option<usize>,
    right_error_count: Option<usize>,
    agreement_count: Option<usize>,
    disagreement_count: Option<usize>,
    left_indeterminate_right_decisive_count: Option<usize>,
    left_decisive_right_indeterminate_count: Option<usize>,
    behavior_key: String,
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
    let out_dir = prepare_out_dir(config.out_dir.clone(), TARGET_NAME, config.mode.as_str())?;
    let mut events = JsonlWriter::create(&out_dir.join("decision-events.jsonl"))?;
    let cases = selected_cases(&config);
    for (sample, case) in cases.into_iter().enumerate() {
        write_case_events(&config, sample, &case, &mut events)?;
    }
    events.flush()?;
    Ok(out_dir)
}

fn selected_cases(config: &Config) -> Vec<ScanCase> {
    let mut cases = Vec::new();
    if matches!(
        config.input_cohort,
        InputCohort::RetainedArtifacts | InputCohort::All
    ) {
        cases.extend(load_retained_artifact_cases(config.max_rows_per_family));
    }
    if matches!(
        config.input_cohort,
        InputCohort::GeneratedF64 | InputCohort::All
    ) {
        cases.extend(generated_f64_cases(
            config.generated_samples_per_facet,
            config.generated_seed,
        ));
    }
    cases
}

fn write_case_events(
    config: &Config,
    sample: usize,
    case: &ScanCase,
    events: &mut JsonlWriter,
) -> Result<(), String> {
    let report = compare_f64_decisions(&case.dual_vertices);
    for row in [
        report.origin,
        report.facet_presence_vertex_vs_per_facet_lp,
        report.facet_presence_per_facet_lp_vs_batched_primal_lp,
        report.facet_presence_per_facet_lp_vs_batched_polar_lp,
        report.facet_pair_intersection,
    ] {
        events.write(&ComparisonEvent {
            target: TARGET_NAME,
            mode: config.mode,
            family: case.family.clone(),
            source_id: case.source_id.clone(),
            facet_count: case.dual_vertices.len(),
            sample,
            decision: row.decision,
            left_method: row.left_method.to_string(),
            right_method: Some(row.right_method.to_string()),
            left_time_ms: row.left_time_ms,
            right_time_ms: Some(row.right_time_ms),
            left_true_count: row.left_true_count,
            left_false_count: row.left_false_count,
            left_indeterminate_count: row.left_indeterminate_count,
            left_error_count: row.left_error_count,
            right_true_count: Some(row.right_true_count),
            right_false_count: Some(row.right_false_count),
            right_indeterminate_count: Some(row.right_indeterminate_count),
            right_error_count: Some(row.right_error_count),
            agreement_count: Some(row.agreement_count),
            disagreement_count: Some(row.disagreement_count),
            left_indeterminate_right_decisive_count: Some(
                row.left_indeterminate_right_decisive_count,
            ),
            left_decisive_right_indeterminate_count: Some(
                row.left_decisive_right_indeterminate_count,
            ),
            behavior_key: comparison_behavior_key(
                row.left_true_count,
                row.left_false_count,
                row.left_indeterminate_count,
                row.right_true_count,
                row.right_false_count,
                row.right_indeterminate_count,
                row.disagreement_count,
            ),
        })?;
    }
    events.write(&ComparisonEvent {
        target: TARGET_NAME,
        mode: config.mode,
        family: case.family.clone(),
        source_id: case.source_id.clone(),
        facet_count: case.dual_vertices.len(),
        sample,
        decision: report.omega_sign.decision,
        left_method: report.omega_sign.method.to_string(),
        right_method: None,
        left_time_ms: report.omega_sign.time_ms,
        right_time_ms: None,
        left_true_count: report.omega_sign.positive_count,
        left_false_count: report.omega_sign.negative_count,
        left_indeterminate_count: report.omega_sign.indeterminate_count,
        left_error_count: 0,
        right_true_count: None,
        right_false_count: None,
        right_indeterminate_count: None,
        right_error_count: None,
        agreement_count: None,
        disagreement_count: None,
        left_indeterminate_right_decisive_count: None,
        left_decisive_right_indeterminate_count: None,
        behavior_key: format!(
            "pos{}:neg{}:zero{}:indet{}",
            report.omega_sign.positive_count,
            report.omega_sign.negative_count,
            report.omega_sign.zero_count,
            report.omega_sign.indeterminate_count
        ),
    })
}

fn comparison_behavior_key(
    left_true: usize,
    left_false: usize,
    left_indeterminate: usize,
    right_true: usize,
    right_false: usize,
    right_indeterminate: usize,
    disagreement: usize,
) -> String {
    format!(
        "left_t{left_true}_f{left_false}_i{left_indeterminate}:right_t{right_true}_f{right_false}_i{right_indeterminate}:d{disagreement}"
    )
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
            "--input-cohort" => {
                let value = take_value("--input-cohort", inline_value, &mut args)?;
                config.input_cohort = match value.as_str() {
                    "retained_artifacts" => InputCohort::RetainedArtifacts,
                    "generated_f64" => InputCohort::GeneratedF64,
                    "all" => InputCohort::All,
                    other => {
                        return Err(format!(
                            "--input-cohort must be retained_artifacts, generated_f64, or all, got {other}"
                        ))
                    }
                };
            }
            "--generated-samples-per-facet" => {
                config.generated_samples_per_facet =
                    take_value("--generated-samples-per-facet", inline_value, &mut args)?
                        .parse()
                        .map_err(|_| {
                            "--generated-samples-per-facet must be a positive integer".to_string()
                        })?;
            }
            "--generated-seed" => {
                config.generated_seed = take_value("--generated-seed", inline_value, &mut args)?
                    .parse()
                    .map_err(|_| "--generated-seed must be a u64".to_string())?;
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
    let generated_samples_per_facet = match mode {
        RunMode::Smoke => SMOKE_GENERATED_SAMPLES_PER_FACET,
        RunMode::Production => PRODUCTION_GENERATED_SAMPLES_PER_FACET,
    };
    Config {
        mode,
        max_rows_per_family,
        generated_samples_per_facet,
        generated_seed: DEFAULT_GENERATED_SEED,
        input_cohort: InputCohort::RetainedArtifacts,
        out_dir: None,
    }
}

fn print_usage() {
    println!("{}", usage());
}

fn usage() -> &'static str {
    "Usage: cargo run -p exp-dev-quadratic-program --release --bin f64-decision-compare -- \\
        --mode production --input-cohort retained_artifacts --out-dir /tmp/perf-f64-decision-compare"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke_mode_is_default() {
        let config = parse_args(std::iter::empty()).expect("default config");
        assert!(matches!(config.mode, RunMode::Smoke));
        assert_eq!(config.max_rows_per_family, SMOKE_MAX_ROWS_PER_FAMILY);
        assert_eq!(
            config.generated_samples_per_facet,
            SMOKE_GENERATED_SAMPLES_PER_FACET
        );
        assert_eq!(config.input_cohort, InputCohort::RetainedArtifacts);
    }

    #[test]
    fn production_mode_selects_documented_profile_size() {
        let config = parse_args(["--mode".to_string(), "production".to_string()].into_iter())
            .expect("production config");
        assert!(matches!(config.mode, RunMode::Production));
        assert_eq!(config.max_rows_per_family, PRODUCTION_MAX_ROWS_PER_FAMILY);
        assert_eq!(
            config.generated_samples_per_facet,
            PRODUCTION_GENERATED_SAMPLES_PER_FACET
        );
    }

    #[test]
    fn generated_input_cohort_is_explicit() {
        let config = parse_args(
            [
                "--input-cohort".to_string(),
                "generated_f64".to_string(),
                "--generated-samples-per-facet".to_string(),
                "2".to_string(),
                "--generated-seed".to_string(),
                "7".to_string(),
            ]
            .into_iter(),
        )
        .expect("generated config");
        assert_eq!(config.input_cohort, InputCohort::GeneratedF64);
        assert_eq!(config.generated_samples_per_facet, 2);
        assert_eq!(config.generated_seed, 7);
    }
}

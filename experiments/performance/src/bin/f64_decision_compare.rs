use exp_dev_f64_capacity::{compare_f64_decisions, load_retained_artifact_cases, ScanCase};
use exp_performance::args::{selected_run_mode, split_inline_arg, take_value, RunMode};
use exp_performance::jsonl::JsonlWriter;
use exp_performance::output_dir::prepare_out_dir;
use serde::Serialize;
use std::env;
use std::path::PathBuf;
use std::process::ExitCode;

const TARGET_NAME: &str = "f64-decision-compare";
const SMOKE_MAX_ROWS_PER_FAMILY: usize = 2;
const PRODUCTION_MAX_ROWS_PER_FAMILY: usize = 100;

#[derive(Clone, Debug, Serialize)]
struct Config {
    mode: RunMode,
    max_rows_per_family: usize,
    out_dir: Option<PathBuf>,
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
    let cases = load_retained_artifact_cases(config.max_rows_per_family);
    for (sample, case) in cases.into_iter().enumerate() {
        write_case_events(&config, sample, &case, &mut events)?;
    }
    events.flush()?;
    Ok(out_dir)
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
        out_dir: None,
    }
}

fn print_usage() {
    println!("{}", usage());
}

fn usage() -> &'static str {
    "Usage: cargo run -p exp-performance --release --bin f64-decision-compare -- \\
        --mode production --out-dir /tmp/perf-f64-decision-compare"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke_mode_is_default() {
        let config = parse_args(std::iter::empty()).expect("default config");
        assert!(matches!(config.mode, RunMode::Smoke));
        assert_eq!(config.max_rows_per_family, SMOKE_MAX_ROWS_PER_FAMILY);
    }

    #[test]
    fn production_mode_selects_documented_profile_size() {
        let config = parse_args(["--mode".to_string(), "production".to_string()].into_iter())
            .expect("production config");
        assert!(matches!(config.mode, RunMode::Production));
        assert_eq!(config.max_rows_per_family, PRODUCTION_MAX_ROWS_PER_FAMILY);
    }
}

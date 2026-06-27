use exp_dev_quadratic_program::F64CapacityOutcome;
use exp_performance::args::{selected_run_mode, split_inline_arg, take_value, RunMode};
use exp_performance::capacity_route_support::{
    accepted_fixture, capacity_fixture_from_dual_vertices, f64_transition_pruned_once,
    pruned_f64_then_exact_once, CapacityFixture, DEFAULT_H_MAX, DEFAULT_H_MIN, DEFAULT_SEED,
};
use exp_performance::jsonl::JsonlWriter;
use exp_performance::output_dir::prepare_out_dir;
use serde::Serialize;
use std::env;
use std::path::PathBuf;
use std::process::ExitCode;
use std::time::Instant;

const TARGET_NAME: &str = "capacity-paths-random";

#[derive(Clone, Debug, Serialize)]
struct Config {
    mode: RunMode,
    facet_counts: Vec<usize>,
    samples: usize,
    seed: u64,
    h_min: f64,
    h_max: f64,
    out_dir: Option<PathBuf>,
}

#[derive(Serialize)]
struct MetadataRow {
    target: &'static str,
    mode: RunMode,
    facet_counts: Vec<usize>,
    samples: usize,
    seed: u64,
    h_min: f64,
    h_max: f64,
}

#[derive(Serialize)]
struct SetupEvent {
    target: &'static str,
    mode: RunMode,
    facet_count: usize,
    sample: usize,
    seed: u64,
    fixture_attempts: u64,
    exact_transition_ms: f64,
    allowed_transitions: usize,
}

#[derive(Serialize)]
struct PathEvent {
    target: &'static str,
    mode: RunMode,
    facet_count: usize,
    sample: usize,
    seed: u64,
    path: &'static str,
    measurement_scope: &'static str,
    elapsed_ms: f64,
    status: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    capacity: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    iterations: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    raw_orbits: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    returned_orbits: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sigma_count: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    admissible_f64_count: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    indeterminate_f64_count: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    capacity_abs_diff_from_fallback: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
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
    write_metadata(&config, &out_dir)?;

    let mut setup_events = JsonlWriter::create(&out_dir.join("setup-events.jsonl"))?;
    let mut path_events = JsonlWriter::create(&out_dir.join("path-events.jsonl"))?;
    for &facet_count in &config.facet_counts {
        for sample in 0..config.samples {
            profile_sample(
                &config,
                facet_count,
                sample,
                &mut setup_events,
                &mut path_events,
            )?;
        }
    }
    setup_events.flush()?;
    path_events.flush()?;
    Ok(out_dir)
}

fn write_metadata(config: &Config, out_dir: &std::path::Path) -> Result<(), String> {
    let mut writer = JsonlWriter::create(&out_dir.join("metadata.jsonl"))?;
    writer.write(&MetadataRow {
        target: TARGET_NAME,
        mode: config.mode,
        facet_counts: config.facet_counts.clone(),
        samples: config.samples,
        seed: config.seed,
        h_min: config.h_min,
        h_max: config.h_max,
    })?;
    writer.flush()
}

fn profile_sample(
    config: &Config,
    facet_count: usize,
    sample: usize,
    setup_events: &mut JsonlWriter,
    path_events: &mut JsonlWriter,
) -> Result<(), String> {
    let accepted = accepted_fixture(facet_count, sample, config.seed, config.h_min, config.h_max)?;
    let setup_start = Instant::now();
    let fixture = capacity_fixture_from_dual_vertices(accepted.dual_vertices_f64)?;
    let exact_transition_ms = elapsed_ms(setup_start);
    let allowed_transitions = fixture
        .transition_is_allowed
        .iter()
        .filter(|&&allowed| allowed)
        .count();
    setup_events.write(&SetupEvent {
        target: TARGET_NAME,
        mode: config.mode,
        facet_count,
        sample,
        seed: config.seed,
        fixture_attempts: accepted.fixture_attempts,
        exact_transition_ms,
        allowed_transitions,
    })?;

    let fallback = pruned_hk_exact(config, facet_count, sample, &fixture);
    let fallback_capacity = fallback.capacity;
    path_events.write(&fallback)?;

    let mut f64_event = f64_hk(config, facet_count, sample, &fixture);
    f64_event.capacity_abs_diff_from_fallback = match (f64_event.capacity, fallback_capacity) {
        (Some(left), Some(right)) => Some((left - right).abs()),
        _ => None,
    };
    path_events.write(&f64_event)
}

fn pruned_hk_exact(
    config: &Config,
    facet_count: usize,
    sample: usize,
    fixture: &CapacityFixture,
) -> PathEvent {
    let started = Instant::now();
    match pruned_f64_then_exact_once(fixture) {
        Ok(result) => PathEvent {
            target: TARGET_NAME,
            mode: config.mode,
            facet_count,
            sample,
            seed: config.seed,
            path: "exact_transition_pruned_f64_then_exact_fallback",
            measurement_scope: "after_exact_transition_setup",
            elapsed_ms: elapsed_ms(started),
            status: "ok",
            capacity: Some(result.capacity),
            iterations: Some(result.iterations),
            raw_orbits: Some(result.raw_orbits),
            returned_orbits: Some(result.returned_orbits),
            sigma_count: None,
            admissible_f64_count: None,
            indeterminate_f64_count: None,
            capacity_abs_diff_from_fallback: None,
            error: None,
        },
        Err(error) => error_event(
            config,
            facet_count,
            sample,
            "exact_transition_pruned_f64_then_exact_fallback",
            "after_exact_transition_setup",
            elapsed_ms(started),
            format!("{error:?}"),
        ),
    }
}

fn f64_hk(
    config: &Config,
    facet_count: usize,
    sample: usize,
    fixture: &CapacityFixture,
) -> PathEvent {
    let started = Instant::now();
    let report = f64_transition_pruned_once(fixture);
    let elapsed_ms = elapsed_ms(started);
    match report.outcome {
        F64CapacityOutcome::Success { capacity, .. } => PathEvent {
            target: TARGET_NAME,
            mode: config.mode,
            facet_count,
            sample,
            seed: config.seed,
            path: "f64_transition_pruned_hk",
            measurement_scope: "full_f64_route_after_fixture_setup",
            elapsed_ms,
            status: "ok",
            capacity: Some(capacity),
            iterations: None,
            raw_orbits: None,
            returned_orbits: None,
            sigma_count: Some(report.sigma_count),
            admissible_f64_count: Some(report.admissible_f64_count),
            indeterminate_f64_count: Some(report.indeterminate_f64_count),
            capacity_abs_diff_from_fallback: None,
            error: None,
        },
        F64CapacityOutcome::Failure { reason } => error_event(
            config,
            facet_count,
            sample,
            "f64_transition_pruned_hk",
            "full_f64_route_after_fixture_setup",
            elapsed_ms,
            format!("{reason:?}"),
        ),
    }
}

fn error_event(
    config: &Config,
    facet_count: usize,
    sample: usize,
    path: &'static str,
    measurement_scope: &'static str,
    elapsed_ms: f64,
    error: String,
) -> PathEvent {
    PathEvent {
        target: TARGET_NAME,
        mode: config.mode,
        facet_count,
        sample,
        seed: config.seed,
        path,
        measurement_scope,
        elapsed_ms,
        status: "error",
        capacity: None,
        iterations: None,
        raw_orbits: None,
        returned_orbits: None,
        sigma_count: None,
        admissible_f64_count: None,
        indeterminate_f64_count: None,
        capacity_abs_diff_from_fallback: None,
        error: Some(error),
    }
}

fn elapsed_ms(started: Instant) -> f64 {
    started.elapsed().as_secs_f64() * 1000.0
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
                config.out_dir = Some(PathBuf::from(take_value(
                    "--out-dir",
                    inline_value,
                    &mut args,
                )?));
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
    let (facet_counts, samples) = match mode {
        RunMode::Smoke => (vec![6], 1),
        RunMode::Production => (vec![6, 10], 5),
    };
    Config {
        mode,
        facet_counts,
        samples,
        seed: DEFAULT_SEED,
        h_min: DEFAULT_H_MIN,
        h_max: DEFAULT_H_MAX,
        out_dir: None,
    }
}

fn validate_config(config: &Config) -> Result<(), String> {
    if config.samples == 0 {
        return Err("sample count must be positive".to_string());
    }
    if config
        .facet_counts
        .iter()
        .any(|&facet_count| facet_count < 5)
    {
        return Err("facet counts must be at least 5".to_string());
    }
    if !config.h_min.is_finite()
        || !config.h_max.is_finite()
        || config.h_min <= 0.0
        || config.h_min >= config.h_max
    {
        return Err("height range must satisfy finite 0 < h_min < h_max".to_string());
    }
    Ok(())
}

fn print_usage() {
    println!("{}", usage());
}

fn usage() -> &'static str {
    "Usage: cargo run -p exp-performance --release --bin capacity-paths-random -- \\
        --mode production --out-dir /tmp/capacity-paths-random\n\
\n\
Options:\n\
  --mode MODE          Named run mode: smoke or production [default: smoke]\n\
  --out-dir PATH       Output directory [default: /tmp/msc-math-performance/<target>-<mode>-<time>-pid<PID>]\n\
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
        assert_eq!(config.facet_counts, vec![6]);
        assert_eq!(config.samples, 1);
    }

    #[test]
    fn production_mode_profiles_f6_and_f10() {
        let config = parse(&["--mode", "production"]);
        assert_eq!(config.mode, RunMode::Production);
        assert_eq!(config.facet_counts, vec![6, 10]);
        assert_eq!(config.samples, 5);
    }
}

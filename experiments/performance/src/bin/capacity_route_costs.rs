use exp_dev_quadratic_program::F64CapacityOutcome;
use exp_performance::args::{selected_run_mode, split_inline_arg, take_value, RunMode};
use exp_performance::capacity_route_support::{
    accepted_fixture, capacity_fixture_from_dual_vertices, exact_transition_pruned_once,
    f64_transition_pruned_once, pruned_f64_then_exact_once, CapacityFixture, DEFAULT_H_MAX,
    DEFAULT_H_MIN, DEFAULT_SEED,
};
use exp_performance::jsonl::JsonlWriter;
use exp_performance::output_dir::prepare_out_dir;
use exp_performance::runtime_context::{
    hardware_context, load_sample, process_cpu_sample, HardwareContext, LoadSample,
};
use serde::Serialize;
use std::env;
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::time::Instant;

const TARGET_NAME: &str = "capacity-route-costs";
const SMOKE_FAST_REPETITIONS: usize = 100;
const PRODUCTION_FAST_REPETITIONS: usize = 1_000;

#[derive(Clone, Debug, Serialize)]
struct Config {
    mode: RunMode,
    facet_counts: Vec<usize>,
    samples: usize,
    seed: u64,
    h_min: f64,
    h_max: f64,
    fast_repetitions: usize,
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
    fast_repetitions: usize,
    hardware: HardwareContext,
    initial_load: LoadSample,
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
    status: &'static str,
    repetitions: usize,
    wall_ms: f64,
    per_call_wall_ms: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    self_cpu_ms: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    self_cpu_over_wall: Option<f64>,
    load_before: LoadSample,
    load_after: LoadSample,
    #[serde(skip_serializing_if = "Option::is_none")]
    capacity: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    iterations: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sigma_count: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    raw_orbits: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    returned_orbits: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    admissible_f64_count: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    indeterminate_f64_count: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exact_admissible_count: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    capacity_abs_diff_from_reference: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

struct Measurement<T> {
    value: T,
    context: MeasurementContext,
}

#[derive(Clone, Copy)]
struct MeasurementContext {
    repetitions: usize,
    wall_ms: f64,
    self_cpu_ms: Option<f64>,
    load_before: LoadSample,
    load_after: LoadSample,
}

fn main() -> ExitCode {
    match run() {
        Ok(out_dir) => {
            println!("out_dir={}", out_dir.display());
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
    print_run_context(&config);

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

fn write_metadata(config: &Config, out_dir: &Path) -> Result<(), String> {
    let mut writer = JsonlWriter::create(&out_dir.join("metadata.jsonl"))?;
    writer.write(&MetadataRow {
        target: TARGET_NAME,
        mode: config.mode,
        facet_counts: config.facet_counts.clone(),
        samples: config.samples,
        seed: config.seed,
        h_min: config.h_min,
        h_max: config.h_max,
        fast_repetitions: config.fast_repetitions,
        hardware: hardware_context(),
        initial_load: load_sample(),
    })?;
    writer.flush()
}

fn print_run_context(config: &Config) {
    let hardware = hardware_context();
    let load = load_sample();
    println!(
        "target={TARGET_NAME} mode={} facet_counts={:?} samples={} seed={} fast_repetitions={}",
        config.mode.as_str(),
        config.facet_counts,
        config.samples,
        config.seed,
        config.fast_repetitions
    );
    println!(
        "hardware host={} os={} arch={} logical_cpus={} kernel={} cpu={}",
        hardware.hostname.as_deref().unwrap_or("unknown"),
        hardware.os,
        hardware.arch,
        hardware
            .logical_cpus
            .map(|value| value.to_string())
            .unwrap_or_else(|| "unknown".to_string()),
        hardware.kernel_release.as_deref().unwrap_or("unknown"),
        hardware.cpu_model.as_deref().unwrap_or("unknown")
    );
    println!(
        "initial_load load1={} load5={} load15={} runnable={}/{}",
        optional_f64(load.load1),
        optional_f64(load.load5),
        optional_f64(load.load15),
        optional_u64(load.runnable_entities),
        optional_u64(load.total_entities)
    );
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

    let exact_reference = exact_transition_pruned(config, facet_count, sample, &fixture);
    let reference_capacity = exact_reference.capacity;
    path_events.write(&exact_reference)?;
    print_path_line(&exact_reference);

    let mut fallback = pruned_f64_then_exact_fallback(config, facet_count, sample, &fixture);
    fallback.capacity_abs_diff_from_reference =
        capacity_abs_diff(fallback.capacity, reference_capacity);
    path_events.write(&fallback)?;
    print_path_line(&fallback);

    let mut f64_event = f64_transition_pruned(config, facet_count, sample, &fixture);
    f64_event.capacity_abs_diff_from_reference =
        capacity_abs_diff(f64_event.capacity, reference_capacity);
    path_events.write(&f64_event)?;
    print_path_line(&f64_event);
    Ok(())
}

fn exact_transition_pruned(
    config: &Config,
    facet_count: usize,
    sample: usize,
    fixture: &CapacityFixture,
) -> PathEvent {
    let measured = measure(|| exact_transition_pruned_once(fixture));
    let Measurement { value, context } = measured;
    match value {
        Ok(report) => path_event(
            config,
            facet_count,
            sample,
            "exact_transition_pruned_sigmas",
            "after_exact_transition_setup",
            context,
            Some(report.capacity),
            Some(report.iterations),
            None,
            None,
            Some(report.orbits.len()),
            None,
            None,
            Some(report.exact_admissible_count),
            None,
        ),
        Err(error) => error_event(
            config,
            facet_count,
            sample,
            "exact_transition_pruned_sigmas",
            "after_exact_transition_setup",
            context,
            format!("{error:?}"),
        ),
    }
}

fn pruned_f64_then_exact_fallback(
    config: &Config,
    facet_count: usize,
    sample: usize,
    fixture: &CapacityFixture,
) -> PathEvent {
    let measured = measure_repeated(config.fast_repetitions, || {
        pruned_f64_then_exact_once(fixture)
    });
    let Measurement { value, context } = measured;
    match value {
        Ok(result) => path_event(
            config,
            facet_count,
            sample,
            "exact_transition_pruned_f64_then_exact_fallback",
            "after_exact_transition_setup",
            context,
            Some(result.capacity),
            Some(result.iterations),
            None,
            Some(result.raw_orbits),
            Some(result.returned_orbits),
            None,
            None,
            None,
            None,
        ),
        Err(error) => error_event(
            config,
            facet_count,
            sample,
            "exact_transition_pruned_f64_then_exact_fallback",
            "after_exact_transition_setup",
            context,
            format!("{error:?}"),
        ),
    }
}

fn f64_transition_pruned(
    config: &Config,
    facet_count: usize,
    sample: usize,
    fixture: &CapacityFixture,
) -> PathEvent {
    let measured = measure_repeated(config.fast_repetitions, || {
        f64_transition_pruned_once(fixture)
    });
    let Measurement { value, context } = measured;
    match value.outcome {
        F64CapacityOutcome::Success { capacity, .. } => path_event(
            config,
            facet_count,
            sample,
            "f64_transition_pruned_hk",
            "full_f64_route",
            context,
            Some(capacity),
            None,
            Some(value.sigma_count),
            None,
            None,
            Some(value.admissible_f64_count),
            Some(value.indeterminate_f64_count),
            None,
            None,
        ),
        F64CapacityOutcome::Failure { reason } => error_event(
            config,
            facet_count,
            sample,
            "f64_transition_pruned_hk",
            "full_f64_route",
            context,
            format!("{reason:?}"),
        ),
    }
}

fn measure<T>(operation: impl FnOnce() -> T) -> Measurement<T> {
    let load_before = load_sample();
    let cpu_before = process_cpu_sample();
    let started = Instant::now();
    let value = operation();
    let wall_ms = elapsed_ms(started);
    let cpu_after = process_cpu_sample();
    let load_after = load_sample();
    let self_cpu_ms = match (cpu_before, cpu_after) {
        (Some(before), Some(after)) => after.elapsed_ms_since(before),
        _ => None,
    };
    Measurement {
        value,
        context: MeasurementContext {
            repetitions: 1,
            wall_ms,
            self_cpu_ms,
            load_before,
            load_after,
        },
    }
}

fn measure_repeated<T>(repetitions: usize, mut operation: impl FnMut() -> T) -> Measurement<T> {
    assert!(repetitions > 0);
    let load_before = load_sample();
    let cpu_before = process_cpu_sample();
    let started = Instant::now();
    let mut last = None;
    for _ in 0..repetitions {
        last = Some(std::hint::black_box(operation()));
    }
    let wall_ms = elapsed_ms(started);
    let cpu_after = process_cpu_sample();
    let load_after = load_sample();
    let self_cpu_ms = match (cpu_before, cpu_after) {
        (Some(before), Some(after)) => after.elapsed_ms_since(before),
        _ => None,
    };
    Measurement {
        value: last.expect("repetitions is positive"),
        context: MeasurementContext {
            repetitions,
            wall_ms,
            self_cpu_ms,
            load_before,
            load_after,
        },
    }
}

fn path_event(
    config: &Config,
    facet_count: usize,
    sample: usize,
    path: &'static str,
    measurement_scope: &'static str,
    measured: MeasurementContext,
    capacity: Option<f64>,
    iterations: Option<u64>,
    sigma_count: Option<u64>,
    raw_orbits: Option<usize>,
    returned_orbits: Option<usize>,
    admissible_f64_count: Option<usize>,
    indeterminate_f64_count: Option<usize>,
    exact_admissible_count: Option<usize>,
    capacity_abs_diff_from_reference: Option<f64>,
) -> PathEvent {
    let self_cpu_over_wall = measured
        .self_cpu_ms
        .filter(|_| measured.wall_ms > 0.0)
        .map(|self_cpu_ms| self_cpu_ms / measured.wall_ms);
    let per_call_wall_ms = measured.wall_ms / measured.repetitions as f64;
    PathEvent {
        target: TARGET_NAME,
        mode: config.mode,
        facet_count,
        sample,
        seed: config.seed,
        path,
        measurement_scope,
        status: "ok",
        repetitions: measured.repetitions,
        wall_ms: measured.wall_ms,
        per_call_wall_ms,
        self_cpu_ms: measured.self_cpu_ms,
        self_cpu_over_wall,
        load_before: measured.load_before,
        load_after: measured.load_after,
        capacity,
        iterations,
        sigma_count,
        raw_orbits,
        returned_orbits,
        admissible_f64_count,
        indeterminate_f64_count,
        exact_admissible_count,
        capacity_abs_diff_from_reference,
        error: None,
    }
}

fn error_event(
    config: &Config,
    facet_count: usize,
    sample: usize,
    path: &'static str,
    measurement_scope: &'static str,
    measured: MeasurementContext,
    error: String,
) -> PathEvent {
    let mut event = path_event(
        config,
        facet_count,
        sample,
        path,
        measurement_scope,
        measured,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    );
    event.status = "error";
    event.error = Some(error);
    event
}

fn print_path_line(event: &PathEvent) {
    println!(
        "sample F={} sample={} path={} status={} repetitions={} wall_ms={:.3} per_call_wall_ms={:.6} self_cpu_ms={} load1={}->{} capacity={} diff_from_reference={}",
        event.facet_count,
        event.sample,
        event.path,
        event.status,
        event.repetitions,
        event.wall_ms,
        event.per_call_wall_ms,
        optional_f64(event.self_cpu_ms),
        optional_f64(event.load_before.load1),
        optional_f64(event.load_after.load1),
        optional_f64(event.capacity),
        optional_f64(event.capacity_abs_diff_from_reference)
    );
}

fn capacity_abs_diff(left: Option<f64>, right: Option<f64>) -> Option<f64> {
    Some((left? - right?).abs())
}

fn elapsed_ms(started: Instant) -> f64 {
    started.elapsed().as_secs_f64() * 1000.0
}

fn optional_f64(value: Option<f64>) -> String {
    value
        .map(|value| format!("{value:.6}"))
        .unwrap_or_else(|| "unknown".to_string())
}

fn optional_u64(value: Option<u64>) -> String {
    value
        .map(|value| value.to_string())
        .unwrap_or_else(|| "unknown".to_string())
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
    let (facet_counts, samples, fast_repetitions) = match mode {
        RunMode::Smoke => (vec![6], 1, SMOKE_FAST_REPETITIONS),
        RunMode::Production => (vec![6, 10], 1, PRODUCTION_FAST_REPETITIONS),
    };
    Config {
        mode,
        facet_counts,
        samples,
        seed: DEFAULT_SEED,
        h_min: DEFAULT_H_MIN,
        h_max: DEFAULT_H_MAX,
        fast_repetitions,
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
    "Usage: cargo run -p exp-performance --release --bin capacity-route-costs -- \\
        --mode production --out-dir /tmp/capacity-route-costs-production\n\
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
        assert_eq!(config.samples, 1);
    }
}

use exp_performance::args::{split_inline_arg, take_value};
use exp_performance::capacity_route_support::{
    accepted_fixture, capacity_fixture_from_dual_vertices, exact_geometry_from_dual_vertices,
    exact_transition_pruned_once, f64_transition_pruned_from_dual_vertices,
    pruned_f64_then_exact_from_geometry, CapacityPath, DEFAULT_H_MAX, DEFAULT_H_MIN, DEFAULT_SEED,
};
use exp_performance::jsonl::JsonlWriter;
use exp_performance::output_dir::prepare_out_dir;
use serde::Serialize;
use std::env;
use std::hint::black_box;
use std::path::PathBuf;
use std::process::ExitCode;
use std::time::Instant;

const TARGET_NAME: &str = "capacity-profile-one";

#[derive(Clone, Debug, Serialize)]
struct Config {
    path: CapacityPath,
    facet_count: usize,
    sample: usize,
    repetitions: usize,
    seed: u64,
    h_min: f64,
    h_max: f64,
    out_dir: Option<PathBuf>,
}

#[derive(Serialize)]
struct SummaryRow {
    target: &'static str,
    path: &'static str,
    measurement_scope: &'static str,
    facet_count: usize,
    sample: usize,
    repetitions: usize,
    seed: u64,
    h_min: f64,
    h_max: f64,
    elapsed_ms: f64,
    per_repetition_ms: f64,
    last_capacity: f64,
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
    let out_dir = prepare_out_dir(config.out_dir.clone(), TARGET_NAME, config.path.label())?;
    let accepted = accepted_fixture(
        config.facet_count,
        config.sample,
        config.seed,
        config.h_min,
        config.h_max,
    )?;

    let mut last = None;
    match config.path {
        CapacityPath::F64TransitionPrunedHk => {
            let started = Instant::now();
            for _ in 0..config.repetitions {
                last = Some(f64_transition_pruned_from_dual_vertices(
                    &accepted.dual_vertices_f64,
                ));
                black_box(last);
            }
            write_summary(
                &config,
                &out_dir,
                started.elapsed().as_secs_f64() * 1000.0,
                last,
            )?;
        }
        CapacityPath::ExactTransitionPrunedF64ThenExactFallback => {
            let geometry = exact_geometry_from_dual_vertices(accepted.dual_vertices_f64);
            let started = Instant::now();
            for _ in 0..config.repetitions {
                last = Some(pruned_f64_then_exact_from_geometry(&geometry));
                black_box(last);
            }
            write_summary(
                &config,
                &out_dir,
                started.elapsed().as_secs_f64() * 1000.0,
                last,
            )?;
        }
        CapacityPath::ExactTransitionPrunedSigmas => {
            let fixture = capacity_fixture_from_dual_vertices(accepted.dual_vertices_f64)?;
            let started = Instant::now();
            for _ in 0..config.repetitions {
                let report = exact_transition_pruned_once(&fixture)
                    .map_err(|error| format!("exact route failed: {error:?}"))?;
                last = Some(report.capacity);
                black_box(last);
            }
            write_summary(
                &config,
                &out_dir,
                started.elapsed().as_secs_f64() * 1000.0,
                last,
            )?;
        }
    }

    Ok(out_dir)
}

fn write_summary(
    config: &Config,
    out_dir: &std::path::Path,
    elapsed_ms: f64,
    last: Option<f64>,
) -> Result<(), String> {
    let mut writer = JsonlWriter::create(&out_dir.join("profile-summary.jsonl"))?;
    writer.write(&SummaryRow {
        target: TARGET_NAME,
        path: config.path.label(),
        measurement_scope: measurement_scope(config.path),
        facet_count: config.facet_count,
        sample: config.sample,
        repetitions: config.repetitions,
        seed: config.seed,
        h_min: config.h_min,
        h_max: config.h_max,
        elapsed_ms,
        per_repetition_ms: elapsed_ms / config.repetitions as f64,
        last_capacity: last.expect("repetitions is positive"),
    })?;
    writer.flush()
}

fn parse_args(args: impl Iterator<Item = String>) -> Result<Config, String> {
    let mut config = Config {
        path: CapacityPath::ExactTransitionPrunedF64ThenExactFallback,
        facet_count: 10,
        sample: 1,
        repetitions: 100,
        seed: DEFAULT_SEED,
        h_min: DEFAULT_H_MIN,
        h_max: DEFAULT_H_MAX,
        out_dir: None,
    };
    let mut args = args.peekable();
    while let Some(arg) = args.next() {
        let (flag, inline_value) = split_inline_arg(arg);
        match flag.as_str() {
            "--path" => {
                config.path = CapacityPath::parse(&take_value("--path", inline_value, &mut args)?)?
            }
            "--facet-count" => {
                config.facet_count = take_value("--facet-count", inline_value, &mut args)?
                    .parse()
                    .map_err(|_| "--facet-count must be a positive integer".to_string())?
            }
            "--sample" => {
                config.sample = take_value("--sample", inline_value, &mut args)?
                    .parse()
                    .map_err(|_| "--sample must be a nonnegative integer".to_string())?
            }
            "--repetitions" => {
                config.repetitions = take_value("--repetitions", inline_value, &mut args)?
                    .parse()
                    .map_err(|_| "--repetitions must be a positive integer".to_string())?
            }
            "--out-dir" => {
                config.out_dir = Some(PathBuf::from(take_value(
                    "--out-dir",
                    inline_value,
                    &mut args,
                )?))
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

fn validate_config(config: &Config) -> Result<(), String> {
    if config.facet_count < 5 {
        return Err("--facet-count must be at least 5".to_string());
    }
    if config.repetitions == 0 {
        return Err("--repetitions must be positive".to_string());
    }
    if config.path == CapacityPath::ExactTransitionPrunedSigmas && config.repetitions > 1 {
        return Err(
            "--path exact is slow; use --repetitions 1 unless you intentionally edit this guard"
                .to_string(),
        );
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
    "Usage: cargo run -p exp-performance --release --bin capacity-profile-one -- \\
        --path f64 --facet-count 10 --sample 1 --repetitions 100 --out-dir /tmp/capacity-profile-one-f64\n\
\n\
Options:\n\
  --path PATH           f64_transition_pruned_hk/f64, exact_transition_pruned_f64_then_exact_fallback/fallback, or exact_transition_pruned_sigmas/exact [default: fallback]\n\
  --facet-count N      Random fixture facet count [default: 10]\n\
  --sample N           Deterministic random fixture sample index [default: 1]\n\
  --repetitions N      Repetitions after fixture construction [default: 100; exact path requires 1]\n\
  --out-dir PATH       Output directory [default: /tmp/msc-math-performance/<target>-<path>-<time>-pid<PID>]\n\
  --help               Print this help text"
}

fn measurement_scope(path: CapacityPath) -> &'static str {
    match path {
        CapacityPath::F64TransitionPrunedHk => "full_f64_route",
        CapacityPath::ExactTransitionPrunedF64ThenExactFallback => "after_exact_geometry_setup",
        CapacityPath::ExactTransitionPrunedSigmas => "after_exact_transition_setup",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(values: &[&str]) -> Config {
        parse_args(values.iter().map(|value| value.to_string())).unwrap()
    }

    #[test]
    fn parses_existing_path_aliases() {
        assert_eq!(
            parse(&["--path", "f64"]).path,
            CapacityPath::F64TransitionPrunedHk
        );
        assert_eq!(
            parse(&["--path", "fallback"]).path,
            CapacityPath::ExactTransitionPrunedF64ThenExactFallback
        );
        assert_eq!(
            parse(&["--path", "exact"]).path,
            CapacityPath::ExactTransitionPrunedSigmas
        );
    }

    #[test]
    fn exact_path_rejects_repeated_default_benchmarking() {
        let config = parse(&["--path", "exact", "--repetitions", "2"]);
        assert!(validate_config(&config).is_err());
    }
}

use exp_performance::args::{selected_run_mode, split_inline_arg, take_value, RunMode};
use exp_performance::jsonl::JsonlWriter;
use exp_performance::output_dir::prepare_out_dir;
use exp_performance::timing::timed;
use nalgebra::DMatrix;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use serde::Serialize;
use std::env;
use std::path::PathBuf;
use std::process::ExitCode;
use symplectic::algorithms::hk2017::SimpleDirectedCyclesCanonical;
use tracing::info_span;
use tracing_subscriber::fmt::format::FmtSpan;

const TARGET_NAME: &str = "hk2017-cycle-enumeration";
const DEFAULT_SEED: u64 = 42;
const DEFAULT_EDGE_PROBABILITY: f64 = 0.25;
const SAMPLE_SEED_STRIDE: u64 = 7919;

#[derive(Clone, Debug, Serialize)]
struct Config {
    mode: RunMode,
    facet_counts: Vec<usize>,
    samples: usize,
    seed: u64,
    edge_probability: f64,
    trace: bool,
    out_dir: Option<PathBuf>,
}

#[derive(Serialize)]
struct PhaseEvent {
    target: &'static str,
    mode: RunMode,
    facet_count: usize,
    sample: usize,
    seed: u64,
    phase: &'static str,
    elapsed_ms: f64,
    status: &'static str,
    edge_probability: f64,
    graph_seed: u64,
    allowed_edges: usize,
    cycles: u64,
    checksum: u64,
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

    let phase_events_path = out_dir.join("phase-events.jsonl");

    let mut phase_events = JsonlWriter::create(&phase_events_path)?;
    for &facet_count in &config.facet_counts {
        for sample in 0..config.samples {
            let _span = info_span!(
                "performance_sample",
                target = TARGET_NAME,
                facet_count,
                sample
            )
            .entered();
            let graph_seed = graph_seed(config.seed, facet_count, sample);
            let transition =
                random_transition_matrix(facet_count, config.edge_probability, graph_seed);
            let allowed_edges = transition.iter().filter(|&&allowed| allowed).count();
            let ((cycles, checksum), elapsed_ms) = timed(|| enumerate_cycles(&transition));
            phase_events.write(&PhaseEvent {
                target: TARGET_NAME,
                mode: config.mode,
                facet_count,
                sample,
                seed: config.seed,
                phase: "enumerate_cycles",
                elapsed_ms,
                status: "ok",
                edge_probability: config.edge_probability,
                graph_seed,
                allowed_edges,
                cycles,
                checksum,
            })?;
        }
    }
    phase_events.flush()?;

    Ok(out_dir)
}

#[inline(never)]
fn enumerate_cycles(transition: &DMatrix<bool>) -> (u64, u64) {
    let mut cycles = 0u64;
    let mut checksum = 0u64;
    let mut cycle_iter = SimpleDirectedCyclesCanonical::new(transition);
    for sigma in cycle_iter.by_ref() {
        cycles += 1;
        checksum = checksum.wrapping_add(cycle_checksum(&sigma));
    }
    cycle_iter.emit_trace_summary();
    (cycles, checksum)
}

fn graph_seed(master_seed: u64, facet_count: usize, sample: usize) -> u64 {
    master_seed ^ ((facet_count as u64) << 32) ^ (sample as u64).wrapping_mul(SAMPLE_SEED_STRIDE)
}

fn cycle_checksum(sigma: &[usize]) -> u64 {
    // Make the timed loop depend on the emitted cycle contents.
    sigma.iter().fold(sigma.len() as u64, |acc, &facet| {
        acc.wrapping_mul(1_000_003).wrapping_add(facet as u64)
    })
}

fn random_transition_matrix(facet_count: usize, edge_probability: f64, seed: u64) -> DMatrix<bool> {
    let mut rng = ChaCha8Rng::seed_from_u64(seed);
    DMatrix::from_fn(facet_count, facet_count, |i, j| {
        i != j && rng.gen_bool(edge_probability)
    })
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
        RunMode::Production => (vec![10, 11], 8),
    };
    Config {
        mode,
        facet_counts,
        samples,
        seed: DEFAULT_SEED,
        edge_probability: DEFAULT_EDGE_PROBABILITY,
        trace: false,
        out_dir: None,
    }
}

fn validate_config(config: &Config) -> Result<(), String> {
    if config.samples == 0 {
        return Err("mode sample count must be at least 1".to_owned());
    }
    if config
        .facet_counts
        .iter()
        .any(|&facet_count| facet_count < 2)
    {
        return Err("mode facet-count entries must be at least 2".to_owned());
    }
    if !config.edge_probability.is_finite() || !(0.0..=1.0).contains(&config.edge_probability) {
        return Err(format!(
            "mode edge_probability must be finite and in [0,1], got {}",
            config.edge_probability
        ));
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
    "Usage: cargo run -p exp-performance --release --bin hk2017-cycle-enumeration -- \\
        --mode production --out-dir /tmp/perf-hk2017-cycles\n\
\n\
Options:\n\
  --mode MODE              Named run mode: smoke or production [default: smoke]\n\
  --out-dir PATH           Output directory [default: /tmp/msc-math-performance/<target>-<mode>-<time>-pid<PID>]\n\
  --trace                  Emit tracing span close events to stderr\n\
  --help                   Print this help text"
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
        assert_eq!(config.edge_probability, DEFAULT_EDGE_PROBABILITY);
    }

    #[test]
    fn production_mode_selects_documented_profile_size() {
        let config = parse(&["--mode", "production"]);
        assert_eq!(config.mode, RunMode::Production);
        assert_eq!(config.facet_counts, vec![10, 11]);
        assert_eq!(config.samples, 8);
    }

    #[test]
    fn ad_hoc_input_selector_flags_are_rejected() {
        for flag in [
            "--seed",
            "--facet-counts",
            "--samples",
            "--edge-probability",
        ] {
            assert!(parse_args([flag.to_string(), "1".to_string()].into_iter()).is_err());
        }
    }
}

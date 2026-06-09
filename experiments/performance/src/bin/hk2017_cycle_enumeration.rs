use exp_performance::{
    prepare_out_dir, run_environment, timed, unix_timestamp_secs, write_json_file, JsonlWriter,
    RunEnvironment,
};
use nalgebra::DMatrix;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use serde::Serialize;
use std::env::{self, ArgsOs};
use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use symplectic::algorithms::hk2017::SimpleDirectedCyclesCanonical;
use tracing::info_span;
use tracing_subscriber::fmt::format::FmtSpan;

const TARGET_NAME: &str = "hk2017-cycle-enumeration";
const DEFAULT_SEED: u64 = 42;
const DEFAULT_EDGE_PROBABILITY: f64 = 0.25;

#[derive(Clone, Debug, Serialize)]
struct Config {
    facet_counts: Vec<usize>,
    samples: usize,
    seed: u64,
    edge_probability: f64,
    trace: bool,
    out_dir: Option<PathBuf>,
}

#[derive(Clone, Debug)]
struct Invocation {
    program: OsString,
    args: Vec<String>,
}

#[derive(Serialize)]
struct RunMetadata {
    target: &'static str,
    started_unix_secs: u64,
    cwd: String,
    command: Vec<String>,
    environment: RunEnvironment,
    config: ConfigForMetadata,
    files: OutputFiles,
}

#[derive(Serialize)]
struct ConfigForMetadata {
    facet_counts: Vec<usize>,
    samples: usize,
    seed: u64,
    edge_probability: f64,
    trace: bool,
}

#[derive(Serialize)]
struct OutputFiles {
    phase_events_jsonl: String,
    run_metadata_json: String,
}

#[derive(Serialize)]
struct PhaseEvent {
    target: &'static str,
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
    let invocation = Invocation::from_env(env::args_os())?;
    let config = parse_args(invocation.args.iter().cloned())?;
    validate_config(&config)?;
    let out_dir = prepare_out_dir(config.out_dir.clone(), TARGET_NAME)?;
    if config.trace {
        init_tracing()?;
    }

    let phase_events_path = out_dir.join("phase-events.jsonl");
    let metadata_path = out_dir.join("run-metadata.json");
    let metadata = RunMetadata {
        target: TARGET_NAME,
        started_unix_secs: unix_timestamp_secs()?,
        cwd: env::current_dir()
            .map_err(|error| format!("read current directory: {error}"))?
            .display()
            .to_string(),
        command: invocation.command_for_metadata(),
        environment: run_environment(),
        config: ConfigForMetadata {
            facet_counts: config.facet_counts.clone(),
            samples: config.samples,
            seed: config.seed,
            edge_probability: config.edge_probability,
            trace: config.trace,
        },
        files: OutputFiles {
            phase_events_jsonl: phase_events_path.display().to_string(),
            run_metadata_json: metadata_path.display().to_string(),
        },
    };
    write_json_file(&metadata_path, &metadata)?;

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
            let graph_seed =
                config.seed ^ ((facet_count as u64) << 32) ^ (sample as u64).wrapping_mul(7919);
            let transition =
                random_transition_matrix(facet_count, config.edge_probability, graph_seed);
            let allowed_edges = transition.iter().filter(|&&allowed| allowed).count();
            let ((cycles, checksum), elapsed_ms) = timed(|| enumerate_cycles(&transition));
            phase_events.write(&PhaseEvent {
                target: TARGET_NAME,
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

fn cycle_checksum(sigma: &[usize]) -> u64 {
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
    let mut config = Config {
        facet_counts: vec![11],
        samples: 8,
        seed: DEFAULT_SEED,
        edge_probability: DEFAULT_EDGE_PROBABILITY,
        trace: false,
        out_dir: None,
    };

    let mut args = args.peekable();
    while let Some(arg) = args.next() {
        let (flag, inline_value) = split_inline_arg(arg);
        match flag.as_str() {
            "--facet-counts" => {
                let value = take_value("--facet-counts", inline_value, &mut args)?;
                config.facet_counts = parse_facet_counts(&value)?;
            }
            "--samples" => {
                let value = take_value("--samples", inline_value, &mut args)?;
                config.samples = value
                    .parse()
                    .map_err(|_| format!("--samples must be a positive integer, got {value}"))?;
            }
            "--seed" => {
                let value = take_value("--seed", inline_value, &mut args)?;
                config.seed = value
                    .parse()
                    .map_err(|_| format!("--seed must be a u64, got {value}"))?;
            }
            "--edge-probability" => {
                let value = take_value("--edge-probability", inline_value, &mut args)?;
                config.edge_probability = value
                    .parse()
                    .map_err(|_| format!("--edge-probability must be a finite f64, got {value}"))?;
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

fn validate_config(config: &Config) -> Result<(), String> {
    if config.samples == 0 {
        return Err("--samples must be at least 1".to_owned());
    }
    if config
        .facet_counts
        .iter()
        .any(|&facet_count| facet_count < 2)
    {
        return Err("--facet-counts entries must be at least 2".to_owned());
    }
    if !config.edge_probability.is_finite() || !(0.0..=1.0).contains(&config.edge_probability) {
        return Err(format!(
            "--edge-probability must be finite and in [0,1], got {}",
            config.edge_probability
        ));
    }
    Ok(())
}

impl Invocation {
    fn from_env(args: ArgsOs) -> Result<Self, String> {
        let mut args = args.into_iter();
        let program = args
            .next()
            .ok_or_else(|| "missing argv[0] program name".to_owned())?;
        let args = args
            .map(|arg| {
                arg.into_string().map_err(|arg| {
                    format!("non-utf8 command argument: {}", Path::new(&arg).display())
                })
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self { program, args })
    }

    fn command_for_metadata(&self) -> Vec<String> {
        let mut command = Vec::with_capacity(self.args.len() + 1);
        command.push(Path::new(&self.program).display().to_string());
        command.extend(self.args.iter().cloned());
        command
    }
}

fn split_inline_arg(arg: String) -> (String, Option<String>) {
    match arg.split_once('=') {
        Some((flag, value)) => (flag.to_owned(), Some(value.to_owned())),
        None => (arg, None),
    }
}

fn take_value(
    flag: &str,
    inline_value: Option<String>,
    args: &mut impl Iterator<Item = String>,
) -> Result<String, String> {
    match inline_value {
        Some(value) => Ok(value),
        None => args
            .next()
            .ok_or_else(|| format!("{flag} requires a value")),
    }
}

fn parse_facet_counts(value: &str) -> Result<Vec<usize>, String> {
    let facet_counts: Result<Vec<_>, _> = value
        .split(',')
        .map(|part| {
            let trimmed = part.trim();
            trimmed
                .parse::<usize>()
                .map_err(|_| format!("invalid facet count: {trimmed}"))
        })
        .collect();
    let facet_counts = facet_counts?;
    if facet_counts.is_empty() {
        return Err("--facet-counts must contain at least one count".to_owned());
    }
    Ok(facet_counts)
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
        --facet-counts 10,11 --samples 8 --edge-probability 0.25 \\
        --out-dir /tmp/perf-hk2017-cycles\n\
\n\
Options:\n\
  --facet-counts LIST      Comma-separated node/facet counts [default: 11]\n\
  --samples N              Random directed graphs per facet count [default: 8]\n\
  --seed N                 Master seed for deterministic graph generation [default: 42]\n\
  --edge-probability P     Directed edge probability in [0,1], excluding self-edges [default: 0.25]\n\
  --out-dir PATH           Output directory [default: /tmp/msc-math-performance/<target>-<time>-pid<PID>]\n\
  --trace                  Emit tracing span close events to stderr\n\
  --help                   Print this help text"
}

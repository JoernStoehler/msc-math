use exp_dev_gradient_ascent::{default_output_dir, run_smoke, SmokeConfig};
use std::path::PathBuf;

fn main() {
    let config = parse_args();
    let summary = run_smoke(&config).expect("dev-gradient-ascent smoke command failed");
    println!("{}", summary.out_dir);
}

fn parse_args() -> SmokeConfig {
    let mut out_dir: Option<PathBuf> = None;
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--out-dir" => {
                out_dir = Some(PathBuf::from(
                    args.next().expect("--out-dir requires a path"),
                ));
            }
            "--help" | "-h" => {
                print_usage();
                std::process::exit(0);
            }
            other => panic!("unsupported argument: {other}"),
        }
    }
    SmokeConfig {
        out_dir: out_dir.unwrap_or_else(default_output_dir),
    }
}

fn print_usage() {
    eprintln!(
        "Usage: dev-gradient-ascent-smoke [--out-dir PATH]\n\nWrites schema-smoke artifacts under PATH, or a unique /tmp directory by default."
    );
}

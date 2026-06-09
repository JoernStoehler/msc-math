mod smoke;

use smoke::{default_output_path, run_prediction_smoke};
use std::path::PathBuf;

fn parse_output_path() -> PathBuf {
    let mut args = std::env::args().skip(1);
    let mut output = PathBuf::from(default_output_path());
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--output" => {
                let Some(path) = args.next() else {
                    panic!("--output requires a path");
                };
                output = PathBuf::from(path);
            }
            "--help" | "-h" => {
                eprintln!(
                    "Usage: local-sys-prediction-smoke [--output <path>]\n\
                     Default output: {}",
                    default_output_path()
                );
                std::process::exit(0);
            }
            other => panic!("unsupported argument: {other}"),
        }
    }
    output
}

fn main() {
    let output_path = parse_output_path();
    let report = run_prediction_smoke(&output_path)
        .unwrap_or_else(|err| panic!("local sys prediction smoke failed: {err:?}"));
    report.print(&output_path);
    if !report.has_required_success() {
        eprintln!("local-sys-prediction-smoke did not produce a successful generic row");
        std::process::exit(2);
    }
}

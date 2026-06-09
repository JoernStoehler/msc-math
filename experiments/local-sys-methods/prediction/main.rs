use exp_local_sys_methods::{default_output_path, run_prediction_smoke};
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
    let rows = run_prediction_smoke(&output_path)
        .unwrap_or_else(|err| panic!("local sys prediction smoke failed: {err:?}"));
    let successful = rows.iter().filter(|row| row.status == "ok").count();
    let generic_success = rows
        .iter()
        .any(|row| row.basepoint_name.starts_with("random_f10") && row.status == "ok");
    println!(
        "local-sys-prediction-smoke: wrote {} rows to {}",
        rows.len(),
        output_path.display()
    );
    println!("  successful rows: {successful}");
    println!("  generic basepoint success: {generic_success}");
    if successful == 0 || !generic_success {
        eprintln!("local-sys-prediction-smoke did not produce a successful generic row");
        std::process::exit(2);
    }
}

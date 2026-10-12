use std::path::PathBuf;

#[derive(Clone, Debug)]
pub(crate) struct Args {
    pub(crate) output: PathBuf,
    pub(crate) max_rows_per_family: usize,
    pub(crate) repetitions: usize,
}

pub(crate) fn parse_args() -> Args {
    let argv: Vec<String> = std::env::args().collect();
    let mut output = PathBuf::from("/tmp/f64-capacity-benchmark.jsonl");
    let mut max_rows_per_family = 8usize;
    let mut repetitions = 3usize;
    let mut i = 1;
    while i < argv.len() {
        match argv[i].as_str() {
            "--output" => {
                output = PathBuf::from(value(&argv, i, "--output"));
                i += 2;
            }
            "--max-rows-per-family" => {
                max_rows_per_family = value(&argv, i, "--max-rows-per-family")
                    .parse()
                    .expect("--max-rows-per-family must be usize");
                i += 2;
            }
            "--repetitions" => {
                repetitions = value(&argv, i, "--repetitions")
                    .parse()
                    .expect("--repetitions must be positive usize");
                assert!(repetitions > 0, "--repetitions must be positive");
                i += 2;
            }
            "--help" | "-h" => {
                print_help();
                std::process::exit(0);
            }
            other => panic!("unknown argument: {other}"),
        }
    }
    Args {
        output,
        max_rows_per_family,
        repetitions,
    }
}

fn value<'a>(argv: &'a [String], i: usize, flag: &str) -> &'a str {
    argv.get(i + 1)
        .map(String::as_str)
        .unwrap_or_else(|| panic!("{flag} requires a value"))
}

fn print_help() {
    println!(
        "Usage: f64-capacity-benchmark [--output PATH] [--max-rows-per-family N] [--repetitions N]\n\
         N=0 scans every row in each retained artifact. Exact-backed recomputation is not attempted."
    );
}

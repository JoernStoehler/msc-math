use std::path::PathBuf;

#[derive(Clone, Debug)]
pub(crate) struct Args {
    pub(crate) input: PathBuf,
    pub(crate) json_output: Option<PathBuf>,
}

pub(crate) fn parse_args() -> Args {
    let argv: Vec<String> = std::env::args().collect();
    let mut input = PathBuf::from("/tmp/f64-capacity-scan.jsonl");
    let mut json_output = None;
    let mut i = 1;
    while i < argv.len() {
        match argv[i].as_str() {
            "--input" => {
                input = PathBuf::from(
                    argv.get(i + 1)
                        .map(String::as_str)
                        .expect("--input requires a value"),
                );
                i += 2;
            }
            "--json-output" => {
                json_output = Some(PathBuf::from(
                    argv.get(i + 1)
                        .map(String::as_str)
                        .expect("--json-output requires a value"),
                ));
                i += 2;
            }
            "--help" | "-h" => {
                println!("Usage: f64-capacity-analyze [--input PATH] [--json-output PATH]");
                std::process::exit(0);
            }
            other => panic!("unknown argument: {other}"),
        }
    }
    Args { input, json_output }
}

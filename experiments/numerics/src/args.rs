use std::path::PathBuf;

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RunMode {
    Smoke,
    Evidence,
}

impl RunMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Smoke => "smoke",
            Self::Evidence => "evidence",
        }
    }

    fn parse(value: &str) -> Result<Self, String> {
        match value {
            "smoke" => Ok(Self::Smoke),
            "evidence" => Ok(Self::Evidence),
            other => Err(format!("--mode must be smoke or evidence, got {other}")),
        }
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub struct Config {
    pub mode: RunMode,
    pub out_dir: Option<PathBuf>,
}

pub fn parse_args(args: impl Iterator<Item = String>) -> Result<Config, String> {
    let mut config = Config {
        mode: RunMode::Smoke,
        out_dir: None,
    };
    let mut args = args.peekable();
    while let Some(arg) = args.next() {
        let (flag, inline_value) = split_inline_arg(arg);
        match flag.as_str() {
            "--mode" => {
                let value = take_value("--mode", inline_value, &mut args)?;
                config.mode = RunMode::parse(&value)?;
            }
            "--out-dir" => {
                let value = take_value("--out-dir", inline_value, &mut args)?;
                config.out_dir = Some(PathBuf::from(value));
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

fn print_usage() {
    println!("{}", usage());
}

fn usage() -> &'static str {
    "Usage: cargo run -p exp-numerics --release --bin audit-numerical-errors -- \\
        --mode evidence --out-dir /tmp/numerics-audit\n\
\n\
Options:\n\
  --mode MODE       Named run mode: smoke or evidence [default: smoke]\n\
  --out-dir PATH    Output directory [default: /tmp/msc-math-numerics/<target>-<mode>-<time>-pid<PID>]\n\
  --help            Print this help text"
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(values: &[&str]) -> Config {
        parse_args(values.iter().map(|value| value.to_string())).unwrap()
    }

    #[test]
    fn defaults_to_smoke() {
        assert_eq!(parse(&[]).mode, RunMode::Smoke);
    }

    #[test]
    fn parses_evidence_mode_and_out_dir() {
        let config = parse(&["--mode=evidence", "--out-dir", "/tmp/n"]);
        assert_eq!(config.mode, RunMode::Evidence);
        assert_eq!(config.out_dir, Some(PathBuf::from("/tmp/n")));
    }

    #[test]
    fn rejects_unknown_mode() {
        assert!(parse_args(["--mode".to_owned(), "production".to_owned()].into_iter()).is_err());
    }
}

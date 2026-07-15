use adaptive_multilevel_splitting::{
    run_packet, run_synthetic_packet, ArtifactSink, Config, Manifest, ProductionOracle,
    SourceIdentity, SyntheticOracle, ADAPTIVE_BUDGET, IID_BUDGET,
};
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Mode {
    Synthetic,
    Production,
}

struct Args {
    mode: Mode,
    config: PathBuf,
    artifacts: PathBuf,
    force_synthetic_hit: bool,
}

#[derive(Serialize)]
struct PrintedOutcome {
    artifact_kind: &'static str,
    adaptive_attempts: usize,
    iid_attempts: usize,
    stopped_on_sys_gt_one: bool,
    artifacts: PathBuf,
}

fn main() -> ExitCode {
    match real_main() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("error: {error}");
            ExitCode::FAILURE
        }
    }
}

fn real_main() -> Result<(), String> {
    let args = parse_args()?;
    if args.mode == Mode::Production && args.force_synthetic_hit {
        return Err("--force-synthetic-hit is prohibited in production mode".into());
    }
    let config = Config::from_path(&args.config)?;
    let source = source_identity(args.mode)?;
    if args.mode == Mode::Production && !source.source_tree_clean {
        return Err(
            "production target execution refuses a dirty or untracked source tree; commit the reviewed packet and dependencies first"
                .into(),
        );
    }
    let artifact_kind = match args.mode {
        Mode::Synthetic => "synthetic_target_free",
        Mode::Production => "production_target",
    };
    let manifest = Manifest {
        artifact_kind: artifact_kind.into(),
        config_identity: config.identity(),
        exact_config: config.clone(),
        source,
        adaptive_budget: ADAPTIVE_BUDGET,
        iid_budget: IID_BUDGET,
        target_probability_estimate: None,
        factor_exchange_quotiented: false,
    };
    let sink = ArtifactSink::create(&args.artifacts, &manifest)?;
    let source_revision = manifest.source.git_revision.clone();
    let outcome = match args.mode {
        Mode::Synthetic => {
            let mut adaptive = SyntheticOracle::new(args.force_synthetic_hit);
            let mut iid = SyntheticOracle::new(false);
            run_synthetic_packet(&config, &source_revision, &mut adaptive, &mut iid, &sink)?
        }
        Mode::Production => {
            let mut adaptive = ProductionOracle::default();
            let mut iid = ProductionOracle::default();
            run_packet(&config, &source_revision, &mut adaptive, &mut iid, &sink)?
        }
    };
    let printed = PrintedOutcome {
        artifact_kind: match args.mode {
            Mode::Synthetic => "synthetic_target_free",
            Mode::Production => "production_target",
        },
        adaptive_attempts: outcome.adaptive_attempts,
        iid_attempts: outcome.iid_attempts,
        stopped_on_sys_gt_one: outcome.stopped.is_some(),
        artifacts: args.artifacts,
    };
    println!(
        "{}",
        serde_json::to_string_pretty(&printed).map_err(|e| e.to_string())?
    );
    Ok(())
}

fn parse_args() -> Result<Args, String> {
    let mut values = env::args().skip(1);
    let mode = match values.next().as_deref() {
        Some("synthetic") => Mode::Synthetic,
        Some("production") => Mode::Production,
        _ => return Err(usage()),
    };
    let mut config = None;
    let mut artifacts = None;
    let mut force_synthetic_hit = false;
    while let Some(argument) = values.next() {
        match argument.as_str() {
            "--config" => {
                config = Some(PathBuf::from(
                    values
                        .next()
                        .ok_or_else(|| "--config needs a path".to_owned())?,
                ));
            }
            "--artifacts" => {
                artifacts = Some(PathBuf::from(
                    values
                        .next()
                        .ok_or_else(|| "--artifacts needs a path".to_owned())?,
                ));
            }
            "--force-synthetic-hit" => force_synthetic_hit = true,
            _ => return Err(format!("unknown argument {argument:?}\n{}", usage())),
        }
    }
    Ok(Args {
        mode,
        config: config.ok_or_else(usage)?,
        artifacts: artifacts.ok_or_else(usage)?,
        force_synthetic_hit,
    })
}

fn usage() -> String {
    "usage: adaptive-multilevel-splitting (synthetic|production) --config PATH --artifacts NEW_DIRECTORY [--force-synthetic-hit]".into()
}

fn source_identity(mode: Mode) -> Result<SourceIdentity, String> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let revision = git_output(root, &["rev-parse", "HEAD"])?;
    let status = git_output(root, &["status", "--porcelain", "--untracked-files=normal"])?;
    let executable = env::current_exe().map_err(|e| format!("locate current executable: {e}"))?;
    let lock = root.join("Cargo.lock");
    Ok(SourceIdentity {
        git_revision: revision,
        source_tree_clean: status.is_empty(),
        executable_sha256: file_sha256(&executable)?,
        cargo_lock_sha256: file_sha256(&lock)?,
        production_target: mode == Mode::Production,
    })
}

fn git_output(root: &Path, arguments: &[&str]) -> Result<String, String> {
    let output = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(arguments)
        .output()
        .map_err(|e| format!("run git {arguments:?}: {e}"))?;
    if !output.status.success() {
        return Err(format!(
            "git {arguments:?} failed: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }
    String::from_utf8(output.stdout)
        .map(|value| value.trim().to_owned())
        .map_err(|e| format!("git output was not UTF-8: {e}"))
}

fn file_sha256(path: &Path) -> Result<String, String> {
    let bytes = fs::read(path).map_err(|e| format!("read {path:?} for identity: {e}"))?;
    Ok(format!("{:x}", Sha256::digest(bytes)))
}

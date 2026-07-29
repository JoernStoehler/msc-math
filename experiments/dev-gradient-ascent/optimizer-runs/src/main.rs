use optimizer_runs::manifest::load_and_resolve;
use optimizer_runs::output::{prepare_empty_directory, write_json};
use optimizer_runs::run_plan;
use optimizer_runs::schema::RunProvenance;
use std::env;
use std::path::PathBuf;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    if let Err(error) = real_main() {
        eprintln!("optimizer run failed: {error}");
        std::process::exit(1);
    }
}

fn real_main() -> Result<(), String> {
    let (manifest_path, out_dir, plan_only) = parse_args()?;
    let (git_commit, git_dirty) = git_state();
    let manifest_blake3 = file_blake3(&manifest_path)?;
    let executable_blake3 = file_blake3(
        &env::current_exe().map_err(|error| format!("locate current executable: {error}"))?,
    )?;
    let (plan, source_pool) = load_and_resolve(&manifest_path)?;
    prepare_empty_directory(&out_dir)?;
    let resolved_bytes =
        serde_json::to_vec(&plan).map_err(|error| format!("serialize resolved plan: {error}"))?;
    let resolved_hash = blake3::hash(&resolved_bytes).to_hex().to_string();
    write_json(&out_dir.join("resolved-plan.json"), &plan)?;
    let provenance = RunProvenance {
        schema_version: 1,
        manifest_path: manifest_path.display().to_string(),
        manifest_blake3,
        resolved_plan_hash: resolved_hash,
        git_commit,
        git_dirty,
        executable: "optimizer-runs".to_string(),
        executable_blake3,
        started_unix_ms: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|error| error.to_string())?
            .as_millis(),
    };
    write_json(&out_dir.join("run-provenance.json"), &provenance)?;
    if plan_only {
        println!(
            "resolved {} runs without evaluating sys; plan is in {}",
            plan.runs.len(),
            out_dir.display()
        );
        return Ok(());
    }
    run_plan(&plan, &source_pool, &out_dir)?;
    println!(
        "completed {} runs in {}",
        plan.runs.len(),
        out_dir.display()
    );
    Ok(())
}

fn file_blake3(path: &std::path::Path) -> Result<String, String> {
    let bytes = std::fs::read(path)
        .map_err(|error| format!("read {} for hashing: {error}", path.display()))?;
    Ok(blake3::hash(&bytes).to_hex().to_string())
}

fn parse_args() -> Result<(PathBuf, PathBuf, bool), String> {
    let mut args = env::args().skip(1);
    let mut manifest = None;
    let mut out = None;
    let mut plan_only = false;
    while let Some(argument) = args.next() {
        match argument.as_str() {
            "--manifest" => {
                manifest = Some(PathBuf::from(
                    args.next().ok_or("--manifest requires a path")?,
                ));
            }
            "--out" => {
                out = Some(PathBuf::from(args.next().ok_or("--out requires a path")?));
            }
            "--plan-only" => plan_only = true,
            "--help" | "-h" => {
                println!(
                    "usage: optimizer-runs --manifest PLAN.json --out EMPTY_DIR [--plan-only]"
                );
                std::process::exit(0);
            }
            _ => return Err(format!("unknown argument {argument:?}")),
        }
    }
    Ok((
        manifest.ok_or("missing --manifest")?,
        out.ok_or("missing --out")?,
        plan_only,
    ))
}

fn git_state() -> (String, bool) {
    let commit = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .ok()
        .filter(|output| output.status.success())
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .map(|value| value.trim().to_string())
        .unwrap_or_else(|| "unknown".to_string());
    let dirty = Command::new("git")
        .args(["status", "--porcelain"])
        .output()
        .ok()
        .filter(|output| output.status.success())
        .is_some_and(|output| !output.stdout.is_empty());
    (commit, dirty)
}

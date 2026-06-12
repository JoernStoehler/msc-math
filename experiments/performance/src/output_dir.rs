use std::fs;
use std::path::PathBuf;
use std::process;
use std::time::{SystemTime, UNIX_EPOCH};

pub const DEFAULT_TARGET_ROOT: &str = "/tmp/msc-math-performance";

pub fn prepare_out_dir(
    out_dir: Option<PathBuf>,
    target_name: &str,
    run_mode: &str,
) -> Result<PathBuf, String> {
    let path = match out_dir {
        Some(path) => path,
        None => PathBuf::from(DEFAULT_TARGET_ROOT).join(format!(
            "{target_name}-{run_mode}-{}-pid{}",
            unix_timestamp_secs()?,
            process::id()
        )),
    };
    fs::create_dir_all(&path).map_err(|error| format!("create {}: {error}", path.display()))?;
    Ok(path)
}

fn unix_timestamp_secs() -> Result<u64, String> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .map_err(|error| format!("system clock before unix epoch: {error}"))
}

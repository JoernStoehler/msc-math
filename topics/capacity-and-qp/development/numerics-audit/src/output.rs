use serde::Serialize;
use std::fs::{self, File, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use std::process;
use std::time::{SystemTime, UNIX_EPOCH};

pub const TARGET_NAME: &str = "audit-numerical-errors";
const DEFAULT_ROOT: &str = "/tmp/msc-math-numerics";

pub fn prepare_out_dir(
    out_dir: Option<PathBuf>,
    target_name: &str,
    run_mode: &str,
) -> Result<PathBuf, String> {
    let path = match out_dir {
        Some(path) => path,
        None => PathBuf::from(DEFAULT_ROOT).join(format!(
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

pub struct JsonlWriter {
    writer: BufWriter<File>,
}

impl JsonlWriter {
    pub fn create(path: &Path) -> Result<Self, String> {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)
            .map_err(|error| format!("create {}: {error}", path.display()))?;
        Ok(Self {
            writer: BufWriter::new(file),
        })
    }

    pub fn write<T: Serialize>(&mut self, value: &T) -> Result<(), String> {
        serde_json::to_writer(&mut self.writer, value)
            .map_err(|error| format!("serialize jsonl row: {error}"))?;
        self.writer
            .write_all(b"\n")
            .map_err(|error| format!("write jsonl newline: {error}"))
    }

    pub fn flush(&mut self) -> Result<(), String> {
        self.writer
            .flush()
            .map_err(|error| format!("flush jsonl writer: {error}"))
    }
}

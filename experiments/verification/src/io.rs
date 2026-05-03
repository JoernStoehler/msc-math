//! Run-mode parsing and JSONL output helpers for verification binaries.

use serde::Serialize;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};

#[derive(Clone, Copy)]
pub enum RunMode {
    Smoke,
    Full,
}

#[derive(Debug)]
pub enum RunModeArgError {
    Help,
    Unknown(String),
}

pub fn parse_run_mode<I>(args: I) -> Result<RunMode, RunModeArgError>
where
    I: IntoIterator<Item = String>,
{
    let mut full = false;

    for arg in args {
        match arg.as_str() {
            "--full" => full = true,
            "--help" | "-h" => return Err(RunModeArgError::Help),
            other => return Err(RunModeArgError::Unknown(other.to_string())),
        }
    }

    Ok(if full { RunMode::Full } else { RunMode::Smoke })
}

pub fn run_mode_label(mode: RunMode) -> &'static str {
    match mode {
        RunMode::Smoke => "smoke",
        RunMode::Full => "full",
    }
}

pub fn mode_output_path(
    manifest_dir: &Path,
    subdir: &str,
    smoke_name: &str,
    full_name: &str,
    mode: RunMode,
) -> PathBuf {
    let output_dir = manifest_dir.join(subdir);
    match mode {
        RunMode::Smoke => output_dir.join(smoke_name),
        RunMode::Full => output_dir.join(full_name),
    }
}

pub fn create_jsonl_writer(path: &Path) -> BufWriter<File> {
    std::fs::create_dir_all(
        path.parent()
            .expect("jsonl output path must have a parent directory"),
    )
    .expect("failed to create jsonl output directory");
    BufWriter::new(File::create(path).expect("failed to create jsonl output"))
}

pub fn write_json_line<T: Serialize>(writer: &mut BufWriter<File>, row: &T) {
    serde_json::to_writer(&mut *writer, row).expect("serialize jsonl row");
    writeln!(&mut *writer).expect("write jsonl newline");
}

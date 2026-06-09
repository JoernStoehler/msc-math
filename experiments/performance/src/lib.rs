use serde::Serialize;
use std::fs::{self, File, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use std::process::{self, Command};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

pub const DEFAULT_TARGET_ROOT: &str = "/tmp/msc-math-performance";

#[derive(Clone, Debug, Serialize)]
pub struct RunEnvironment {
    pub git_head: Option<String>,
    pub git_dirty: Option<bool>,
    pub git_status_short: Option<String>,
    pub rustc_version: Option<String>,
    pub uname: Option<String>,
    pub cpu_model: Option<String>,
}

#[derive(Debug)]
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

pub fn prepare_out_dir(out_dir: Option<PathBuf>, target_name: &str) -> Result<PathBuf, String> {
    let path = match out_dir {
        Some(path) => path,
        None => {
            let started = unix_timestamp_secs()?;
            PathBuf::from(DEFAULT_TARGET_ROOT)
                .join(format!("{target_name}-{started}-pid{}", process::id()))
        }
    };
    fs::create_dir_all(&path).map_err(|error| format!("create {}: {error}", path.display()))?;
    Ok(path)
}

pub fn write_json_file<T: Serialize>(path: &Path, value: &T) -> Result<(), String> {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)
        .map_err(|error| format!("create {}: {error}", path.display()))?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, value)
        .map_err(|error| format!("write {}: {error}", path.display()))
}

pub fn timed<T>(operation: impl FnOnce() -> T) -> (T, f64) {
    let start = Instant::now();
    let value = operation();
    (value, ms(start.elapsed()))
}

pub fn timed_result<T, E>(operation: impl FnOnce() -> Result<T, E>) -> (Result<T, E>, f64) {
    let start = Instant::now();
    let value = operation();
    (value, ms(start.elapsed()))
}

pub fn ms(duration: Duration) -> f64 {
    duration.as_secs_f64() * 1000.0
}

pub fn unix_timestamp_secs() -> Result<u64, String> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .map_err(|error| format!("system clock before unix epoch: {error}"))
}

pub fn run_environment() -> RunEnvironment {
    let git_status_short = command_stdout("git", &["status", "--short"]);
    RunEnvironment {
        git_head: command_stdout("git", &["rev-parse", "HEAD"]),
        git_dirty: git_status_short
            .as_ref()
            .map(|status| !status.trim().is_empty()),
        git_status_short,
        rustc_version: command_stdout("rustc", &["--version"]),
        uname: command_stdout("uname", &["-a"]),
        cpu_model: cpu_model(),
    }
}

pub fn command_stdout(program: &str, args: &[&str]) -> Option<String> {
    let output = Command::new(program).args(args).output().ok()?;
    if !output.status.success() {
        return None;
    }
    let value = String::from_utf8(output.stdout).ok()?;
    Some(value.trim().to_owned())
}

fn cpu_model() -> Option<String> {
    let cpuinfo = fs::read_to_string("/proc/cpuinfo").ok()?;
    cpuinfo.lines().find_map(|line| {
        let (key, value) = line.split_once(':')?;
        (key.trim() == "model name").then(|| value.trim().to_owned())
    })
}

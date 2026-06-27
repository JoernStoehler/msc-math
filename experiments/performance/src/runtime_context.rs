use serde::Serialize;
use std::fs;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Debug, Serialize)]
pub struct HardwareContext {
    pub os: &'static str,
    pub arch: &'static str,
    pub logical_cpus: Option<usize>,
    pub hostname: Option<String>,
    pub kernel_release: Option<String>,
    pub cpu_model: Option<String>,
}

#[derive(Clone, Copy, Debug, Serialize)]
pub struct LoadSample {
    pub unix_time_s: f64,
    pub load1: Option<f64>,
    pub load5: Option<f64>,
    pub load15: Option<f64>,
    pub runnable_entities: Option<u64>,
    pub total_entities: Option<u64>,
}

#[derive(Clone, Copy, Debug)]
pub struct ProcessCpuSample {
    ticks: u64,
    ticks_per_second: Option<f64>,
}

impl ProcessCpuSample {
    pub fn elapsed_ms_since(self, earlier: Self) -> Option<f64> {
        let ticks_per_second = self.ticks_per_second.or(earlier.ticks_per_second)?;
        Some((self.ticks.saturating_sub(earlier.ticks) as f64 / ticks_per_second) * 1000.0)
    }
}

pub fn hardware_context() -> HardwareContext {
    HardwareContext {
        os: std::env::consts::OS,
        arch: std::env::consts::ARCH,
        logical_cpus: std::thread::available_parallelism().ok().map(usize::from),
        hostname: read_trimmed("/proc/sys/kernel/hostname")
            .or_else(|| std::env::var("HOSTNAME").ok()),
        kernel_release: read_trimmed("/proc/sys/kernel/osrelease"),
        cpu_model: read_cpu_model(),
    }
}

pub fn load_sample() -> LoadSample {
    let (load1, load5, load15, runnable_entities, total_entities) =
        read_loadavg().unwrap_or((None, None, None, None, None));
    LoadSample {
        unix_time_s: unix_time_s(),
        load1,
        load5,
        load15,
        runnable_entities,
        total_entities,
    }
}

pub fn process_cpu_sample() -> Option<ProcessCpuSample> {
    let ticks = read_process_cpu_ticks()?;
    Some(ProcessCpuSample {
        ticks,
        ticks_per_second: ticks_per_second(),
    })
}

fn read_trimmed(path: &str) -> Option<String> {
    fs::read_to_string(path)
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

fn read_cpu_model() -> Option<String> {
    let cpuinfo = fs::read_to_string("/proc/cpuinfo").ok()?;
    for line in cpuinfo.lines() {
        let Some((key, value)) = line.split_once(':') else {
            continue;
        };
        let key = key.trim();
        if key == "model name" || key == "Hardware" || key == "Processor" {
            let value = value.trim();
            if !value.is_empty() {
                return Some(value.to_string());
            }
        }
    }
    None
}

type LoadAvgFields = (
    Option<f64>,
    Option<f64>,
    Option<f64>,
    Option<u64>,
    Option<u64>,
);

fn read_loadavg() -> Option<LoadAvgFields> {
    let loadavg = fs::read_to_string("/proc/loadavg").ok()?;
    let mut parts = loadavg.split_whitespace();
    let load1 = parts.next().and_then(|value| value.parse().ok());
    let load5 = parts.next().and_then(|value| value.parse().ok());
    let load15 = parts.next().and_then(|value| value.parse().ok());
    let (runnable_entities, total_entities) = parts
        .next()
        .and_then(|value| value.split_once('/'))
        .map(|(running, total)| (running.parse().ok(), total.parse().ok()))
        .unwrap_or((None, None));
    Some((load1, load5, load15, runnable_entities, total_entities))
}

fn read_process_cpu_ticks() -> Option<u64> {
    let stat = fs::read_to_string("/proc/self/stat").ok()?;
    let after_comm = stat.rsplit_once(") ")?.1;
    let fields: Vec<&str> = after_comm.split_whitespace().collect();
    let utime = fields.get(11)?.parse::<u64>().ok()?;
    let stime = fields.get(12)?.parse::<u64>().ok()?;
    Some(utime + stime)
}

fn ticks_per_second() -> Option<f64> {
    let output = Command::new("getconf").arg("CLK_TCK").output().ok()?;
    if !output.status.success() {
        return None;
    }
    std::str::from_utf8(&output.stdout)
        .ok()?
        .trim()
        .parse::<f64>()
        .ok()
        .filter(|value| value.is_finite() && *value > 0.0)
}

fn unix_time_s() -> f64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs_f64())
        .unwrap_or(f64::NAN)
}

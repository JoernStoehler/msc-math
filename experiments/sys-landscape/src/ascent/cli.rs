use std::path::{Path, PathBuf};

/// Parsed CLI arguments shared across ascent binaries.
pub struct AscentArgs {
    pub n: usize,
    pub n_start: usize,
    pub seed: u64,
    pub out: PathBuf,
    pub fresh: bool,
    pub no_db_update: bool,
    pub seed_time_budget_secs: f64,
    /// Name prefix for the seed — used to build polytope names (e.g. `general_42`).
    pub prefix: String,
}

#[derive(Clone, Debug)]
pub struct AscentOutputPaths {
    pub summary: PathBuf,
    pub trace: PathBuf,
    pub cache: PathBuf,
    pub computed_polytopes: PathBuf,
}

impl AscentOutputPaths {
    pub fn from_summary_path(summary: PathBuf) -> Self {
        let trace = trace_path_for(&summary);
        let cache = cache_path_for(&summary);
        let computed_polytopes = computed_polytopes_path_for(&summary);
        Self {
            summary,
            trace,
            cache,
            computed_polytopes,
        }
    }
}

pub fn smoke_output_path(label: &str, file_name: &str) -> PathBuf {
    let stamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("system clock before UNIX_EPOCH")
        .as_millis();
    let dir = std::env::temp_dir().join(format!("{label}-{}-{stamp}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("create smoke output dir");
    dir.join(file_name)
}

/// Parse ascent CLI arguments. Callers pass the binary's default seed, default
/// sample count, default output path, and a name prefix (`"general"` or
/// `"products"`).
///
/// Recognized flags:
/// `--n`, `--n-start`, `--seed`, `--out`, `--fresh`, `--db-update`,
/// `--no-db-update`, `--seed-time-budget-secs`.
pub fn parse_ascent_args(
    default_seed: u64,
    default_n: usize,
    default_seed_time_budget_secs: f64,
    default_out: PathBuf,
    prefix: &str,
) -> AscentArgs {
    let argv: Vec<String> = std::env::args().collect();

    let mut n: usize = default_n;
    let mut n_start: usize = 0;
    let mut seed: u64 = default_seed;
    let mut out: Option<PathBuf> = None;
    let mut fresh = false;
    let mut no_db_update = true;
    let mut seed_time_budget_secs: f64 = default_seed_time_budget_secs;

    let mut i = 1;
    while i < argv.len() {
        let arg = argv[i].as_str();
        let value = || -> &str {
            argv.get(i + 1)
                .map(|s| s.as_str())
                .unwrap_or_else(|| panic!("{arg} requires a value"))
        };
        match arg {
            "--n" => {
                n = value().parse().expect("--n must be a non-negative integer");
                i += 2;
            }
            "--n-start" => {
                n_start = value()
                    .parse()
                    .expect("--n-start must be a non-negative integer");
                i += 2;
            }
            "--seed" => {
                seed = value().parse().expect("--seed must be a u64");
                i += 2;
            }
            "--out" => {
                out = Some(PathBuf::from(value()));
                i += 2;
            }
            "--seed-time-budget-secs" => {
                seed_time_budget_secs = value()
                    .parse()
                    .expect("--seed-time-budget-secs must be an f64");
                i += 2;
            }
            "--fresh" => {
                fresh = true;
                i += 1;
            }
            "--db-update" => {
                no_db_update = false;
                i += 1;
            }
            "--no-db-update" => {
                no_db_update = true;
                i += 1;
            }
            other => panic!("unknown argument: {other}"),
        }
    }

    AscentArgs {
        n,
        n_start,
        seed,
        out: out.unwrap_or(default_out),
        fresh,
        no_db_update,
        seed_time_budget_secs,
        prefix: prefix.to_string(),
    }
}

/// Derive the trace file path from the summary file path.
///
/// `foo/bar.jsonl` -> `foo/bar-trace.jsonl`.
/// `foo/bar-endpoints.jsonl` -> `foo/bar-trace.jsonl`.
pub fn trace_path_for(summary_path: &Path) -> PathBuf {
    sibling_path_with_suffix(summary_path, "trace")
}

/// Derive the shard-local cache file path from the summary file path.
///
/// `foo/bar.jsonl` -> `foo/bar-cache.jsonl`.
/// `foo/bar-endpoints.jsonl` -> `foo/bar-cache.jsonl`.
pub fn cache_path_for(summary_path: &Path) -> PathBuf {
    sibling_path_with_suffix(summary_path, "cache")
}

/// Derive the computed-polytope file path from the summary file path.
///
/// `foo/bar.jsonl` -> `foo/bar-computed-polytopes.jsonl`.
/// `foo/bar-endpoints.jsonl` -> `foo/bar-computed-polytopes.jsonl`.
pub fn computed_polytopes_path_for(summary_path: &Path) -> PathBuf {
    sibling_path_with_suffix(summary_path, "computed-polytopes")
}

fn sibling_path_with_suffix(summary_path: &Path, suffix: &str) -> PathBuf {
    let stem = summary_path
        .file_stem()
        .expect("summary path must have a file name")
        .to_string_lossy()
        .into_owned();
    let stem = stem.strip_suffix("-endpoints").unwrap_or(&stem);
    let ext = summary_path
        .extension()
        .map(|e| e.to_string_lossy().into_owned())
        .unwrap_or_else(|| "jsonl".to_string());
    let parent = summary_path.parent().unwrap_or_else(|| Path::new("."));
    parent.join(format!("{stem}-{suffix}.{ext}"))
}

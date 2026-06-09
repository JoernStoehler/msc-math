use super::cli::AscentOutputPaths;
use super::expensive_cache::ExpensiveComputationCacheRow;
use super::rows::{ComputedPolytopeRow, SeedResult, SummaryRow, TraceRow};
use num_rational::BigRational;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::collections::HashSet;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;
use std::sync::{Arc, Mutex};
use symplectic::database::PolytopeRecord;

pub struct AscentWriters {
    summary: Arc<Mutex<BufWriter<File>>>,
    trace: Arc<Mutex<BufWriter<File>>>,
    cache: Arc<Mutex<BufWriter<File>>>,
    computed_polytopes: Arc<Mutex<BufWriter<File>>>,
}

/// Load the set of already-completed seed names from a summary .jsonl file.
///
/// Used for resume: seeds whose `name` appears in the existing file are skipped
/// by `run_parallel_seeds`. Missing file or malformed lines return an empty set.
pub fn load_completed_names(path: &Path) -> HashSet<String> {
    let mut names = HashSet::new();
    if let Ok(file) = File::open(path) {
        let reader = BufReader::new(file);
        for line in reader.lines().map_while(Result::ok) {
            let line = line.trim().to_string();
            if line.is_empty() {
                continue;
            }
            if let Ok(v) = serde_json::from_str::<serde_json::Value>(&line) {
                if let Some(name) = v.get("name").and_then(|n| n.as_str()) {
                    names.insert(name.to_string());
                }
            }
        }
    }
    names
}

/// Open the summary, trace, and cache writers wrapped in `Arc<Mutex<_>>`
/// so the parallel runner can share them across threads.
///
/// If `fresh` is true, all files are deleted before opening. Files are
/// opened with `create + append` so resume semantics preserve any rows
/// written by an interrupted prior run.
pub fn open_ascent_writers(paths: &AscentOutputPaths, fresh: bool) -> AscentWriters {
    if fresh {
        let _ = std::fs::remove_file(&paths.summary);
        let _ = std::fs::remove_file(&paths.trace);
        let _ = std::fs::remove_file(&paths.cache);
        let _ = std::fs::remove_file(&paths.computed_polytopes);
    }
    for path in [
        &paths.summary,
        &paths.trace,
        &paths.cache,
        &paths.computed_polytopes,
    ] {
        if let Some(parent) = path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
    }
    let summary_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&paths.summary)
        .unwrap_or_else(|e| panic!("failed to open summary file {:?}: {e}", paths.summary));
    let trace_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&paths.trace)
        .unwrap_or_else(|e| panic!("failed to open trace file {:?}: {e}", paths.trace));
    let cache_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&paths.cache)
        .unwrap_or_else(|e| panic!("failed to open cache file {:?}: {e}", paths.cache));
    let computed_polytopes_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&paths.computed_polytopes)
        .unwrap_or_else(|e| {
            panic!(
                "failed to open computed-polytopes file {:?}: {e}",
                paths.computed_polytopes
            )
        });
    AscentWriters {
        summary: Arc::new(Mutex::new(BufWriter::new(summary_file))),
        trace: Arc::new(Mutex::new(BufWriter::new(trace_file))),
        cache: Arc::new(Mutex::new(BufWriter::new(cache_file))),
        computed_polytopes: Arc::new(Mutex::new(BufWriter::new(computed_polytopes_file))),
    }
}

/// Append one seed's trace rows, cache row, computed-polytope rows, then summary row.
///
/// Crash-safety invariant: **trace, cache, and computed-polytope rows for a
/// seed are on disk before that seed's summary row is on disk**. Write order is
/// trace (+ flush) → cache (+ flush) → computed-polytopes (+ flush) → summary
/// (+ flush). Legacy skip-resume can read the summary file with
/// `load_completed_names`, so a seed counts as "completed" only after its full
/// payload is flushed. The datascience ascent producers now rerun shard control
/// flow and use the expensive-computation cache for cost control instead.
///
/// Caveat: `BufWriter::flush` only pushes bytes to the OS page cache — it does
/// not `fsync`. The invariant therefore holds against process-level kills
/// (SIGKILL, slurm SIGTERM) where the kernel survives and the page cache drains
/// to disk normally. It does NOT hold against a kernel panic or node hard
/// crash, which can lose page-cache bytes in either order. LICCA's real failure
/// mode is slurm SIGTERM, so page-cache flush is enough in practice.
///
/// Locks each writer independently and never holds more than one lock at a
/// time, so two threads writing different seeds cannot deadlock.
pub fn write_seed_result(result: &SeedResult, writers: &AscentWriters) {
    // Trace rows first — must be on disk before the summary row that marks
    // the seed as completed (see crash-safety invariant above).
    {
        let mut w = writers.trace.lock().expect("trace writer mutex poisoned");
        for row in &result.trace {
            let row_json = serde_json::to_string(row)
                .expect("TraceRow serialization is infallible for f64/String fields");
            writeln!(w, "{row_json}").expect("failed to write trace row");
        }
        w.flush().expect("failed to flush trace rows");
    }
    // Cache row second — the expensive endpoint payload must precede summary.
    let cache_json = serde_json::to_string(&result.final_record)
        .expect("PolytopeRecord serialization is infallible for stored row fields");
    {
        let mut w = writers.cache.lock().expect("cache writer mutex poisoned");
        writeln!(w, "{cache_json}").expect("failed to write cache row");
        w.flush().expect("failed to flush cache row");
    }
    // Computed-polytope rows record all retained capacity computations for this
    // seed. They must precede summary for the same resume reason as trace/cache.
    {
        let mut w = writers
            .computed_polytopes
            .lock()
            .expect("computed-polytopes writer mutex poisoned");
        for row in &result.computed_polytopes {
            let row_json = serde_json::to_string(row)
                .expect("ComputedPolytopeRow serialization should succeed");
            writeln!(w, "{row_json}").expect("failed to write computed-polytope row");
        }
        w.flush().expect("failed to flush computed-polytope rows");
    }
    // Summary row last — only after trace, cache, and computed-polytopes are durable.
    let summary_json = serde_json::to_string(&result.summary)
        .expect("SummaryRow serialization is infallible for f64/String fields");
    {
        let mut w = writers
            .summary
            .lock()
            .expect("summary writer mutex poisoned");
        writeln!(w, "{summary_json}").expect("failed to write summary row");
        w.flush().expect("failed to flush summary row");
    }
}
/// Canonicalize all shard output files after a parallel run.
///
/// Takes `writers` by value so the summary, trace, and cache BufWriters are
/// dropped at the top of this function. That drop flushes the BufWriters and
/// closes the underlying files before we re-open them for reading. The caller
/// must not clone the writers elsewhere; after `run_parallel_seeds` returns,
/// the writer struct is the sole owner and passing it here releases it.
///
/// Behavior:
/// 1. Parse `summary_path` line-by-line as `SummaryRow`, tolerating malformed
///    lines (same style as `load_completed_names`). Sort by `name` lexicographic.
///    Write to a tempfile then atomic-rename.
/// 2. Parse `trace_path` as `TraceRow`, sort by `(name, phase, iteration)`,
///    then dedup adjacent rows by the same key. The dedup step removes
///    duplicate trace rows introduced by crash-resume: `write_seed_result`
///    writes trace before summary, so a crash before the summary flush leaves
///    orphan trace rows that get rewritten when the seed is re-run. Sort +
///    dedup reduces these to a single copy.
/// 3. Parse `cache_path` as `PolytopeRecord`, sort by exact rational dual
///    vertices, reject conflicting duplicate records, and dedup identical
///    cache rows introduced by crash-resume.
///
/// After this function returns, all three files are byte-identical across runs
/// that processed the same seed set, regardless of thread count or
/// crash/resume history (modulo per-seed `total_time_ms` which is wall-clock
/// noise).
///
/// Sort convention: summary and trace row order is **lexicographic on `name`**
/// (trace additionally by `phase`, `iteration` within a name); cache row order
/// is by exact rational dual-vertex key. Because seed names are `{prefix}_{i}`
/// with `i` rendered as a decimal string, the row order is NOT numeric: e.g.
/// `general_10` < `general_2` < `general_20` < `general_3`. Downstream
/// `analyze.py` must parse the integer out of the name if it needs numeric
/// ordering; it must not assume JSONL row index equals seed index.
pub fn finalize_ascent_output(paths: &AscentOutputPaths, writers: AscentWriters) {
    // Drop writers first so the BufWriters flush and the files are closed
    // before we re-open them below. Explicit drop (not just letting it fall
    // out of scope) to make the ordering requirement legible.
    drop(writers);

    // --- Summary file: sort by name, atomic-rename. ---
    let mut summary_rows = read_rows::<SummaryRow>(&paths.summary);
    summary_rows.sort_by(|a, b| a.name.cmp(&b.name));
    reject_conflicting_adjacent(&summary_rows, |row| row.name.clone(), "summary");
    write_rows_atomic(&paths.summary, &summary_rows);

    // --- Trace file: sort by (name, phase, iteration), dedup, atomic-rename. ---
    let mut trace_rows = read_rows::<TraceRow>(&paths.trace);
    trace_rows.sort_by(|a, b| {
        a.name
            .cmp(&b.name)
            .then_with(|| a.phase.cmp(&b.phase))
            .then_with(|| a.iteration.cmp(&b.iteration))
    });
    // Remove duplicates from crash-resume (see doc comment on `write_seed_result`).
    // dedup_by keeps the first of each adjacent run of equal keys.
    trace_rows
        .dedup_by(|a, b| a.name == b.name && a.phase == b.phase && a.iteration == b.iteration);
    write_rows_atomic(&paths.trace, &trace_rows);

    // --- Cache file: sort by exact dual vertices, dedup identical records. ---
    let mut cache_rows = read_rows::<PolytopeRecord>(&paths.cache);
    cache_rows.sort_by_key(cache_key);
    reject_conflicting_adjacent(&cache_rows, cache_key, "cache");
    cache_rows.dedup_by(|a, b| cache_key(a) == cache_key(b));
    write_rows_atomic(&paths.cache, &cache_rows);

    // --- Computed polytopes: sort by stable result id, dedup identical rows. ---
    let mut computed_polytope_rows = read_rows::<ComputedPolytopeRow>(&paths.computed_polytopes);
    computed_polytope_rows.sort_by(|a, b| a.result_id.cmp(&b.result_id));
    reject_conflicting_adjacent(
        &computed_polytope_rows,
        |row| row.result_id.clone(),
        "computed-polytope",
    );
    computed_polytope_rows.dedup_by(|a, b| a.result_id == b.result_id);
    write_rows_atomic(&paths.computed_polytopes, &computed_polytope_rows);
}

pub fn write_expensive_computation_cache_rows(
    paths: &AscentOutputPaths,
    rows: &[ExpensiveComputationCacheRow],
) {
    write_rows_atomic(&paths.expensive_computations_cache, rows);
}

fn read_rows<T: DeserializeOwned>(path: &Path) -> Vec<T> {
    let mut rows = Vec::new();
    if let Ok(file) = File::open(path) {
        let reader = BufReader::new(file);
        for line in reader.lines().map_while(Result::ok) {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            if let Ok(row) = serde_json::from_str::<T>(line) {
                rows.push(row);
            }
        }
    }
    rows
}

fn write_rows_atomic<T: Serialize>(path: &Path, rows: &[T]) {
    let tmp = path.with_extension("jsonl.tmp");
    {
        let f = File::create(&tmp).unwrap_or_else(|e| panic!("failed to create {tmp:?}: {e}"));
        let mut w = BufWriter::new(f);
        for row in rows {
            let s = serde_json::to_string(row).expect("JSONL row serialization should succeed");
            writeln!(w, "{s}").expect("failed to write tmp row");
        }
        w.flush().expect("failed to flush tmp rows");
    }
    std::fs::rename(&tmp, path)
        .unwrap_or_else(|e| panic!("failed to rename {tmp:?} -> {path:?}: {e}"));
}

fn reject_conflicting_adjacent<T: Serialize>(rows: &[T], key: impl Fn(&T) -> String, label: &str) {
    for pair in rows.windows(2) {
        let left_key = key(&pair[0]);
        if left_key != key(&pair[1]) {
            continue;
        }
        let left_json =
            serde_json::to_string(&pair[0]).expect("JSONL row serialization should succeed");
        let right_json =
            serde_json::to_string(&pair[1]).expect("JSONL row serialization should succeed");
        if left_json != right_json {
            panic!("conflicting duplicate {label} row for key {left_key}");
        }
    }
}

fn cache_key(record: &PolytopeRecord) -> String {
    record
        .dual_vertices_rational
        .iter()
        .map(rational_vec4_key)
        .collect::<Vec<_>>()
        .join("|")
}

fn rational_vec4_key(row: &[BigRational; 4]) -> String {
    row.iter()
        .map(|value| format!("{}/{}", value.numer(), value.denom()))
        .collect::<Vec<_>>()
        .join(",")
}

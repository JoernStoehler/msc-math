//! Shared helpers for sys-landscape experiments.
//!
//! Experiments studying the systolic ratio as a global function on polytope space:
//! random-sample, random-product-sample, gradient-ascent-general,
//! gradient-ascent-products, rotated-regular-products, rejection-calibration.
//! Local cell geometry experiments moved to exp-combinatorial-cells.

use nalgebra::{Matrix4, Vector4};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use symplectic::geom::polytope::Polytope4D;
use symplectic::geom::skeleton::Skeleton;
use symplectic::geom::symplectic_form::omega0;

// ============================================================================
// Step bound constants
// ============================================================================

/// Maximum step size cap (prevents infinite steps when no combinatorial bound exists).
/// Typical boundary distances are O(0.01)-O(1); 100.0 is well beyond any real boundary.
/// Used both by `compute_step_bound_detailed` for "unbounded" classification and by
/// the ascent binaries' overshoot guard (`if t_max < MAX_STEP_SIZE`). Must be a single
/// source of truth — if tuned, both semantics must agree.
pub const MAX_STEP_SIZE: f64 = 100.0;

/// Numerical zero threshold for rates and slacks.
/// Set near machine epsilon (~1e-16); guards against treating f64 noise as a
/// meaningful direction or rate. Used in step bounds and gradient checks.
/// If changed: values much larger risk missing real boundaries; much smaller risks
/// false positives from floating-point noise.
const EPS_NUMERICAL_ZERO: f64 = 1e-15;

// ============================================================================
// Boundary event types
// ============================================================================

/// Classification of a combinatorial boundary event.
#[derive(Debug, Clone)]
pub enum EventType {
    /// A vertex's slack with respect to a non-incident facet reaches zero.
    IncidenceFlip {
        vertex_index: usize,
        new_facet: usize,
    },
    /// sign(omega_0(a_i, a_j)) changes for ridge-adjacent facets i, j.
    OmegaFlip { facet_i: usize, facet_j: usize },
    /// |a_k + t*d_k| -> 0 (dual vertex degenerates).
    DualVertexDegen { facet: usize },
    /// t_max was capped at MAX_STEP_SIZE (no real boundary found).
    Unbounded,
}

/// First boundary event along a direction in dual-vertex space.
#[derive(Debug, Clone)]
pub struct BoundaryEvent {
    /// Step size at which the boundary is crossed.
    pub t_max: f64,
    /// What type of boundary event occurs.
    pub event: EventType,
}

// ============================================================================
// Enriched step-bound computation in a-space
// ============================================================================

/// Compute the first boundary event along a direction in dual-vertex space.
/// [lem:step-bound-incidence] incidence flip detection, [lem:step-bound-omega] omega_0 flip detection
///
/// For step a'_k(t) = a_k + t*d_k, the combinatorial type changes when:
/// 1. **Incidence flip:** a vertex's slack w.r.t. a non-incident facet reaches zero.
/// 2. **omega_0 flip:** sign(omega_0(a_i, a_j)) changes for ridge-adjacent facets.
/// 3. **Dual vertex degeneration:** |a_k + t*d_k| -> 0.
///
/// Source: `exp-combinatorial-cells/cell-widths/run.rs` (enriched version with omega_0 detection).
/// Cell-widths data shows omega_0 flips account for 30.5% of boundary events in per-facet probes
/// (cell-widths/logbook.md). The old `compute_step_bound_a` missed these entirely.
pub fn compute_step_bound_detailed(
    polytope: &Polytope4D,
    direction: &[Vector4<f64>],
) -> BoundaryEvent {
    let duals = polytope.dual_vertices_f64();
    let vertices = polytope.vertices_f64();
    let f = polytope.facet_count();
    let skeleton = Skeleton::compute(polytope);

    let mut best = BoundaryEvent {
        t_max: f64::INFINITY,
        event: EventType::Unbounded,
    };

    // --- Vertex-facet incidence checks ---
    for (vi, vertex_facets) in skeleton.vertex_facets.iter().enumerate() {
        let v = &vertices[vi];

        if vertex_facets.len() == 4 {
            let a_mat = Matrix4::from_rows(&[
                duals[vertex_facets[0]].transpose(),
                duals[vertex_facets[1]].transpose(),
                duals[vertex_facets[2]].transpose(),
                duals[vertex_facets[3]].transpose(),
            ]);

            let a_inv = match a_mat.try_inverse() {
                Some(inv) => inv,
                None => continue,
            };

            let rhs = Vector4::new(
                direction[vertex_facets[0]].dot(v),
                direction[vertex_facets[1]].dot(v),
                direction[vertex_facets[2]].dot(v),
                direction[vertex_facets[3]].dot(v),
            );

            let dv_dt = -(a_inv * rhs);

            for j in 0..f {
                if vertex_facets.contains(&j) {
                    continue;
                }
                let slack = 1.0 - duals[j].dot(v);
                let rate = -direction[j].dot(v) - duals[j].dot(&dv_dt);
                if rate < -EPS_NUMERICAL_ZERO {
                    let t_crit = slack / (-rate);
                    if t_crit > 0.0 && t_crit < best.t_max {
                        best = BoundaryEvent {
                            t_max: t_crit,
                            event: EventType::IncidenceFlip {
                                vertex_index: vi,
                                new_facet: j,
                            },
                        };
                    }
                }
            }
        } else {
            // Non-simple vertex (>4 incident facets). Conservative bound.
            let max_d = direction
                .iter()
                .map(|dk| dk.norm())
                .fold(0.0f64, f64::max);
            for (j, a_j) in duals.iter().enumerate() {
                if vertex_facets.contains(&j) {
                    continue;
                }
                let slack = 1.0 - a_j.dot(v);
                let max_rate = max_d * v.norm() + a_j.norm() * max_d * v.norm();
                if max_rate > EPS_NUMERICAL_ZERO {
                    let t_crit = slack / max_rate;
                    if t_crit > 0.0 && t_crit < best.t_max {
                        best = BoundaryEvent {
                            t_max: t_crit,
                            event: EventType::IncidenceFlip {
                                vertex_index: vi,
                                new_facet: j,
                            },
                        };
                    }
                }
            }
        }
    }

    // --- omega_0 sign preservation for ridge-adjacent pairs ---
    for ridge in &skeleton.ridges {
        let i = ridge.facets[0];
        let j = ridge.facets[1];
        let c = omega0(&duals[i], &duals[j]);
        let b = omega0(&direction[i], &duals[j]) + omega0(&duals[i], &direction[j]);
        let a_coeff = omega0(&direction[i], &direction[j]);

        let roots = if a_coeff.abs() > EPS_NUMERICAL_ZERO {
            let disc = b * b - 4.0 * a_coeff * c;
            if disc < 0.0 {
                vec![]
            } else {
                let sqrt_disc = disc.sqrt();
                vec![
                    (-b - sqrt_disc) / (2.0 * a_coeff),
                    (-b + sqrt_disc) / (2.0 * a_coeff),
                ]
            }
        } else if b.abs() > EPS_NUMERICAL_ZERO {
            vec![-c / b]
        } else {
            vec![]
        };

        for t_flip in roots {
            if t_flip > EPS_NUMERICAL_ZERO && t_flip < best.t_max {
                best = BoundaryEvent {
                    t_max: t_flip,
                    event: EventType::OmegaFlip {
                        facet_i: i,
                        facet_j: j,
                    },
                };
            }
        }
    }

    // --- Dual vertex degeneration: |a_k + t*d_k| -> 0 ---
    for k in 0..f {
        let a_coeff = direction[k].norm_squared();
        let b = 2.0 * duals[k].dot(&direction[k]);
        let c = duals[k].norm_squared();
        let disc = b * b - 4.0 * a_coeff * c;
        if disc >= 0.0 && a_coeff > EPS_NUMERICAL_ZERO {
            let sqrt_disc = disc.sqrt();
            for &sign in &[-1.0, 1.0] {
                let t_crit = (-b + sign * sqrt_disc) / (2.0 * a_coeff);
                if t_crit > EPS_NUMERICAL_ZERO && t_crit < best.t_max {
                    best = BoundaryEvent {
                        t_max: t_crit,
                        event: EventType::DualVertexDegen { facet: k },
                    };
                }
            }
        }
    }

    if best.t_max > MAX_STEP_SIZE {
        best = BoundaryEvent {
            t_max: MAX_STEP_SIZE,
            event: EventType::Unbounded,
        };
    }

    best
}

/// Convenience wrapper: returns just the step bound (t_max) without event details.
///
/// Drop-in replacement for the old `compute_step_bound_a`. Callers that only need
/// the scalar step bound use this; callers that need to classify boundary events
/// use `compute_step_bound_detailed` directly.
pub fn compute_step_bound(polytope: &Polytope4D, direction: &[Vector4<f64>]) -> f64 {
    compute_step_bound_detailed(polytope, direction).t_max
}

// ============================================================================
// Shared CLI + I/O for ascent experiments
// ============================================================================
//
// `gradient-ascent-general` and `gradient-ascent-products` have byte-identical
// `SummaryRow` and `TraceRow` schemas and duplicate CLI/resume/writer code.
// This section centralizes the shared plumbing so both binaries call
// `run_parallel_seeds` with a `process` closure that does the experiment-specific
// per-seed work (polytope generation, gradient ascent, escape strategies).
//
// Architecture B (rayon `par_iter` at dataset level) moves parallelism into this
// shared runner; per-seed RNG streams (`seed + i`) guarantee byte-reproducibility
// regardless of thread assignment (see D1 in vectorized-bouncing-gray.md).

/// One row per seed — the main analysis dataset.
///
/// Schema is byte-identical between `gradient-ascent-general` and
/// `gradient-ascent-products`. `polytope_type` is set by the experiment:
/// general passes the literal `"general"` (see `gradient-ascent-general/run.rs`
/// line 507); products passes `lagrangian_{q_f}x{p_f}` where `q_f` and `p_f`
/// are the facet counts of the two Lagrangian factors (see
/// `gradient-ascent-products/run.rs` line 443, `bucket_name`).
#[derive(Debug, Serialize, Deserialize)]
pub struct SummaryRow {
    pub name: String,
    pub seed_index: usize,
    pub polytope_type: String,
    pub facet_count: usize,
    pub starting_sys: f64,
    pub final_sys: f64,
    pub total_delta: f64,
    pub n_ascent_phases: usize,
    pub n_gradient_iters_total: usize,
    pub n_escape_overshoot: usize,
    pub n_escape_wiggle: usize,
    pub best_strategy: String,
    pub total_time_ms: f64,
    pub final_dual_vertices: Vec<[f64; 4]>,
}

/// One row per iteration per ascent phase — diagnostic trace.
#[derive(Debug, Serialize, Deserialize)]
pub struct TraceRow {
    pub name: String,
    pub phase: usize,
    pub iteration: usize,
    pub step_type: String,
    pub t_fraction: f64,
    pub t_actual: f64,
    pub sys_before: f64,
    pub sys_after: f64,
    pub delta_sys: f64,
    pub gradient_norm: f64,
}

/// Result of processing one seed: the summary row plus its trace rows.
pub struct SeedResult {
    pub summary: SummaryRow,
    pub trace: Vec<TraceRow>,
}

/// Parsed CLI arguments shared across ascent binaries.
pub struct AscentArgs {
    pub n: usize,
    pub n_start: usize,
    pub seed: u64,
    pub out: PathBuf,
    pub fresh: bool,
    pub no_db_update: bool,
    /// Name prefix for the seed — used to build polytope names (e.g. `general_42`).
    pub prefix: String,
}

/// Parse ascent CLI arguments. Callers pass the binary's default seed, default
/// output path, and a name prefix (`"general"` or `"products"`).
///
/// Recognized flags: `--n`, `--n-start`, `--seed`, `--out`, `--fresh`, `--no-db-update`.
pub fn parse_ascent_args(default_seed: u64, default_out: PathBuf, prefix: &str) -> AscentArgs {
    let argv: Vec<String> = std::env::args().collect();

    let mut n: usize = 10;
    let mut n_start: usize = 0;
    let mut seed: u64 = default_seed;
    let mut out: Option<PathBuf> = None;
    let mut fresh = false;
    let mut no_db_update = false;

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
                n_start = value().parse().expect("--n-start must be a non-negative integer");
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
            "--fresh" => {
                fresh = true;
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
        prefix: prefix.to_string(),
    }
}

/// Derive the trace file path from the summary file path.
///
/// `foo/bar.jsonl` -> `foo/bar-trace.jsonl`.
pub fn trace_path_for(summary_path: &Path) -> PathBuf {
    let stem = summary_path
        .file_stem()
        .expect("summary path must have a file name")
        .to_string_lossy()
        .into_owned();
    let ext = summary_path
        .extension()
        .map(|e| e.to_string_lossy().into_owned())
        .unwrap_or_else(|| "jsonl".to_string());
    let parent = summary_path.parent().unwrap_or_else(|| Path::new("."));
    parent.join(format!("{stem}-trace.{ext}"))
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

/// Open the summary + trace writers wrapped in `Arc<Mutex<BufWriter<File>>>`
/// so the parallel runner can share them across threads.
///
/// If `fresh` is true, both files are deleted before opening. Files are
/// opened with `create + append` so resume semantics preserve any rows
/// written by an interrupted prior run.
pub fn open_ascent_writers(
    summary_path: &Path,
    trace_path: &Path,
    fresh: bool,
) -> (
    Arc<Mutex<BufWriter<File>>>,
    Arc<Mutex<BufWriter<File>>>,
) {
    if fresh {
        let _ = std::fs::remove_file(summary_path);
        let _ = std::fs::remove_file(trace_path);
    }
    if let Some(parent) = summary_path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if let Some(parent) = trace_path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let summary_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(summary_path)
        .unwrap_or_else(|e| panic!("failed to open summary file {summary_path:?}: {e}"));
    let trace_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(trace_path)
        .unwrap_or_else(|e| panic!("failed to open trace file {trace_path:?}: {e}"));
    (
        Arc::new(Mutex::new(BufWriter::new(summary_file))),
        Arc::new(Mutex::new(BufWriter::new(trace_file))),
    )
}

/// Append one seed's trace rows, then its summary row, to the shared writers.
///
/// Crash-safety invariant: **trace rows for a seed are on disk before that
/// seed's summary row is on disk**. Write order is trace (+ flush) → summary
/// (+ flush). `load_completed_names` reads only the summary file, so a seed
/// counts as "completed" only after its summary row is flushed, which by this
/// invariant implies all its trace rows are already flushed. A SIGKILL between
/// the two flushes leaves orphan trace rows with no matching summary row:
/// resume re-runs the seed and appends a second copy of its trace rows. The
/// duplicates are removed by `finalize_ascent_output` (sort + dedup on
/// `(name, phase, iteration)`).
///
/// Caveat: `BufWriter::flush` only pushes bytes to the OS page cache — it does
/// not `fsync`. The invariant therefore holds against process-level kills
/// (SIGKILL, slurm SIGTERM) where the kernel survives and the page cache drains
/// to disk normally. It does NOT hold against a kernel panic or node hard
/// crash, which can lose page-cache bytes in either order. LICCA's real failure
/// mode is slurm SIGTERM, so page-cache flush is enough in practice.
///
/// Locks summary and trace independently — NEVER holds both locks at the same
/// time — so two threads writing different seeds cannot deadlock. Each lock is
/// held for the duration of one serde_json serialization + write_all (ms),
/// which is negligible compared to per-seed ascent cost (~seconds).
pub fn write_result(
    result: &SeedResult,
    summary_writer: &Arc<Mutex<BufWriter<File>>>,
    trace_writer: &Arc<Mutex<BufWriter<File>>>,
) {
    // Trace rows first — must be on disk before the summary row that marks
    // the seed as completed (see crash-safety invariant above).
    {
        let mut w = trace_writer.lock().expect("trace writer mutex poisoned");
        for row in &result.trace {
            let row_json = serde_json::to_string(row)
                .expect("TraceRow serialization is infallible for f64/String fields");
            writeln!(w, "{row_json}").expect("failed to write trace row");
        }
        w.flush().expect("failed to flush trace rows");
    }
    // Summary row second — only after trace is durable.
    let summary_json = serde_json::to_string(&result.summary)
        .expect("SummaryRow serialization is infallible for f64/String fields");
    {
        let mut w = summary_writer.lock().expect("summary writer mutex poisoned");
        writeln!(w, "{summary_json}").expect("failed to write summary row");
        w.flush().expect("failed to flush summary row");
    }
}

/// Parallel seed loop with per-seed RNG streams.
///
/// Invariants:
/// - Seed i is identified by global index; the closure MUST use `seed_i`
///   (= `args.seed.wrapping_add(i as u64)`) to construct its RNG and do all
///   per-seed work. Precondition: `args.seed + args.n_start + args.n` must not
///   overflow u64; `wrapping_add` only aliases seed streams across the global
///   batch at `seed ≈ u64::MAX`, far above any realistic ascent run.
///   The per-seed JSON payloads for index i are byte-reproducible
///   regardless of which thread processes it. **File-level byte reproducibility
///   requires the caller to invoke `finalize_ascent_output` after this function
///   returns** — rayon scheduling determines the append order within both
///   output files, and `finalize_ascent_output` is what canonicalizes row order.
/// - `completed` is checked before calling `process`; resume semantics
///   therefore hold across crashes (see `write_result` for the on-disk
///   ordering invariant that backs this).
/// - Writers are locked only during append (ms); contention is negligible
///   against per-seed ascent cost (~seconds).
/// - Seed name format is `"{prefix}_{i}"`, matching the historical naming
///   used by both ascent binaries before the refactor.
///
/// Lock acquisition order inside the rayon closure is strictly:
/// `db` (outside `write_result`, in the per-experiment closure) → `trace`
/// (inside `write_result`) → `summary` (inside `write_result`) → `best`
/// (here, after `write_result` returns). Each lock is released before the
/// next is acquired — no nesting — so two threads cannot form a deadlock
/// cycle regardless of which seed each is processing.
///
/// Panic propagation: a panic inside `process` (or inside `write_result`)
/// poisons any mutex held across the panic point. Subsequent seeds that
/// call `.lock().expect("... poisoned")` will then fan the panic out and
/// crash the binary. This is **intended**: on LICCA, a panicking seed
/// crashes the slurm job, which requeues, and `load_completed_names`
/// resumes by skipping seeds already written to the summary file. Do not
/// convert these `.expect` calls to recover-and-continue without also
/// updating the resume story.
pub fn run_parallel_seeds<F>(
    args: &AscentArgs,
    completed: &HashSet<String>,
    writers: &(
        Arc<Mutex<BufWriter<File>>>,
        Arc<Mutex<BufWriter<File>>>,
    ),
    best: &Arc<Mutex<(f64, String)>>,
    process: F,
) where
    F: Fn(usize, u64) -> Option<SeedResult> + Send + Sync,
{
    let end = args.n_start + args.n;
    (args.n_start..end).into_par_iter().for_each(|i| {
        let name = format!("{}_{}", args.prefix, i);
        if completed.contains(&name) {
            return;
        }
        let seed_i = args.seed.wrapping_add(i as u64);
        if let Some(result) = process(i, seed_i) {
            write_result(&result, &writers.0, &writers.1);
            let mut b = best.lock().expect("best-tracker mutex poisoned");
            // Strict `>`: on ties, the first-to-arrive winner is kept. With rayon
            // scheduling the arrival order is non-deterministic, so the reported
            // "best" name can vary between runs when multiple seeds hit the same
            // `final_sys`. Cosmetic only — `best` is printed to stdout and never
            // written to JSONL. The canonicalized summary file (sorted by name
            // in `finalize_ascent_output`) remains byte-reproducible.
            if result.summary.final_sys > b.0 {
                *b = (result.summary.final_sys, result.summary.name.clone());
            }
        }
    });
}

/// Canonicalize both output files after a parallel run.
///
/// Takes `writers` by value so the `Arc<Mutex<BufWriter<File>>>` pair is
/// dropped at the top of this function — that drop flushes the BufWriters and
/// closes the underlying files before we re-open them for reading. The caller
/// must not clone the writers elsewhere; after `run_parallel_seeds` returns,
/// the writer tuple is the sole owner and passing it here releases it.
///
/// Behavior:
/// 1. Parse `summary_path` line-by-line as `SummaryRow`, tolerating malformed
///    lines (same style as `load_completed_names`). Sort by `name` lexicographic.
///    Write to `summary_path.with_extension("jsonl.tmp")` then atomic-rename.
/// 2. Parse `trace_path` as `TraceRow`, sort by `(name, phase, iteration)`,
///    then dedup adjacent rows by the same key. The dedup step removes
///    duplicate trace rows introduced by crash-resume: `write_result` writes
///    trace before summary, so a crash between the two flushes leaves orphan
///    trace rows that get rewritten when the seed is re-run. Sort + dedup
///    reduces these to a single copy.
/// 3. Atomic-rename trace tempfile.
///
/// After this function returns, both files are byte-identical across runs
/// that processed the same seed set, regardless of thread count or crash/resume
/// history (modulo per-seed `total_time_ms` which is wall-clock noise).
///
/// Sort convention: row order in both files is **lexicographic on `name`**
/// (trace additionally by `phase`, `iteration` within a name). Because seed
/// names are `{prefix}_{i}` with `i` rendered as a decimal string, the row
/// order is NOT numeric: e.g. `general_10` < `general_2` < `general_20` <
/// `general_3`. Downstream `analyze.py` must parse the integer out of the
/// name if it needs numeric ordering; it must not assume JSONL row index
/// equals seed index.
pub fn finalize_ascent_output(
    summary_path: &Path,
    trace_path: &Path,
    writers: (Arc<Mutex<BufWriter<File>>>, Arc<Mutex<BufWriter<File>>>),
) {
    // Drop writers first so the BufWriters flush and the files are closed
    // before we re-open them below. Explicit drop (not just letting it fall
    // out of scope) to make the ordering requirement legible.
    drop(writers);

    // --- Summary file: sort by name, atomic-rename. ---
    let mut summary_rows: Vec<SummaryRow> = Vec::new();
    if let Ok(file) = File::open(summary_path) {
        let reader = BufReader::new(file);
        for line in reader.lines().map_while(Result::ok) {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            if let Ok(row) = serde_json::from_str::<SummaryRow>(line) {
                summary_rows.push(row);
            }
        }
    }
    summary_rows.sort_by(|a, b| a.name.cmp(&b.name));
    let summary_tmp = summary_path.with_extension("jsonl.tmp");
    {
        let f = File::create(&summary_tmp)
            .unwrap_or_else(|e| panic!("failed to create {summary_tmp:?}: {e}"));
        let mut w = BufWriter::new(f);
        for row in &summary_rows {
            let s = serde_json::to_string(row)
                .expect("SummaryRow serialization is infallible for f64/String fields");
            writeln!(w, "{s}").expect("failed to write summary tmp row");
        }
        w.flush().expect("failed to flush summary tmp");
    }
    std::fs::rename(&summary_tmp, summary_path)
        .unwrap_or_else(|e| panic!("failed to rename {summary_tmp:?} -> {summary_path:?}: {e}"));

    // --- Trace file: sort by (name, phase, iteration), dedup, atomic-rename. ---
    let mut trace_rows: Vec<TraceRow> = Vec::new();
    if let Ok(file) = File::open(trace_path) {
        let reader = BufReader::new(file);
        for line in reader.lines().map_while(Result::ok) {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            if let Ok(row) = serde_json::from_str::<TraceRow>(line) {
                trace_rows.push(row);
            }
        }
    }
    trace_rows.sort_by(|a, b| {
        a.name
            .cmp(&b.name)
            .then_with(|| a.phase.cmp(&b.phase))
            .then_with(|| a.iteration.cmp(&b.iteration))
    });
    // Remove duplicates from crash-resume (see doc comment on `write_result`).
    // dedup_by keeps the first of each adjacent run of equal keys.
    trace_rows.dedup_by(|a, b| {
        a.name == b.name && a.phase == b.phase && a.iteration == b.iteration
    });
    let trace_tmp = trace_path.with_extension("jsonl.tmp");
    {
        let f = File::create(&trace_tmp)
            .unwrap_or_else(|e| panic!("failed to create {trace_tmp:?}: {e}"));
        let mut w = BufWriter::new(f);
        for row in &trace_rows {
            let s = serde_json::to_string(row)
                .expect("TraceRow serialization is infallible for f64/String fields");
            writeln!(w, "{s}").expect("failed to write trace tmp row");
        }
        w.flush().expect("failed to flush trace tmp");
    }
    std::fs::rename(&trace_tmp, trace_path)
        .unwrap_or_else(|e| panic!("failed to rename {trace_tmp:?} -> {trace_path:?}: {e}"));
}

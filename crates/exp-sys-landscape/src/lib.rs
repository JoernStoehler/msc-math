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
/// general sets `"general"`, products sets `"products_split_{q}_{p}"`.
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

/// Append one seed's summary + trace rows to the shared writers.
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
    let summary_json = serde_json::to_string(&result.summary)
        .expect("SummaryRow serialization is infallible for f64/String fields");
    {
        let mut w = summary_writer.lock().expect("summary writer mutex poisoned");
        writeln!(w, "{summary_json}").expect("failed to write summary row");
        w.flush().expect("failed to flush summary row");
    }
    {
        let mut w = trace_writer.lock().expect("trace writer mutex poisoned");
        for row in &result.trace {
            let row_json = serde_json::to_string(row)
                .expect("TraceRow serialization is infallible for f64/String fields");
            writeln!(w, "{row_json}").expect("failed to write trace row");
        }
        w.flush().expect("failed to flush trace rows");
    }
}

/// Parallel seed loop with per-seed RNG streams.
///
/// Invariants:
/// - Seed i is identified by global index; the closure MUST use `seed_i`
///   (= `args.seed.wrapping_add(i as u64)`) to construct its RNG and do all
///   per-seed work. The output for index i is byte-reproducible regardless
///   of which thread processes it.
/// - `completed` is checked before calling `process`; resume semantics
///   therefore hold across crashes.
/// - Writers are locked only during append (ms); contention is negligible
///   against per-seed ascent cost (~seconds).
/// - Seed name format is `"{prefix}_{i}"`, matching the historical naming
///   used by both ascent binaries before the refactor.
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
            if result.summary.final_sys > b.0 {
                *b = (result.summary.final_sys, result.summary.name.clone());
            }
        }
    });
}

//! Shared ascent, row, and writer helpers for sys-landscape experiments.

use crate::{euclidean_volume_f64, SysLandscapePolytopeCache};
use good_lp::{constraint, default_solver, variable, variables, Expression, Solution, SolverModel};
use nalgebra::Vector4;
use num_rational::BigRational;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use symplectic::algorithms::billiard::facet_classification::FacetClassification;
use symplectic::database::{OrbitScalars, PolytopeRecord};
use symplectic::derivatives::{
    capacity_subgradients_a, clarke_directional_derivative_a, systolic_ratio_gradient_a,
    volume_derivatives_a,
};
use symplectic::{systolic_ratio, OrbitAdmissibility, OrbitKktData, OrbitSearchResult};

/// Numerical zero threshold for gradient checks.
const EPS_NUMERICAL_ZERO: f64 = 1e-15;

/// Relative tie tolerance for admissible orbit actions in the scalar capacity
/// minimum. This keeps the nonsmooth direction model aligned with
/// `OrbitSearchResult::capacity()`, which already ignores indeterminate
/// candidates.
const ACTIVE_ORBIT_RTOL: f64 = 1e-9;

// ============================================================================
// Shared ascent state and direction selection
// ============================================================================

/// Shared local state for one ascent iteration.
///
/// This packages the active-orbit capacity result together with the smooth
/// volume term. It does not choose a single orbit branch.
#[derive(Clone, Debug)]
pub struct ActiveSysState {
    pub capacity: OrbitSearchResult,
    pub vol: f64,
    pub sys: f64,
}

/// Mode-specific projection for the ascent direction.
#[derive(Clone, Copy, Debug)]
pub enum AscentMode<'a> {
    General,
    LagrangianProduct {
        classification: &'a FacetClassification,
    },
}

/// Compute the active-orbit local state for one polytope.
pub fn compute_active_sys_state(polytope: &SysLandscapePolytopeCache) -> Option<ActiveSysState> {
    let capacity = compute_capacity_result(polytope)?;
    let vol = euclidean_volume_f64(&polytope.vertices, &polytope.vertex_facet_incidence);
    if vol <= 0.0 {
        return None;
    }
    let sys = systolic_ratio(capacity.capacity(), vol);
    sys.is_finite()
        .then_some(ActiveSysState { capacity, vol, sys })
}

/// Compute sys = c_EHZ(K)^2 / (2 vol(K)) from a cached capacity result.
///
/// `capacity` must come from the same `polytope`.
pub fn compute_sys_from_capacity(
    polytope: &SysLandscapePolytopeCache,
    capacity: &OrbitSearchResult,
) -> Option<f64> {
    let vol = euclidean_volume_f64(&polytope.vertices, &polytope.vertex_facet_incidence);
    if vol <= 0.0 {
        return None;
    }
    let cap = capacity.capacity();
    let sys = systolic_ratio(cap, vol);
    sys.is_finite().then_some(sys)
}

/// Compute sys = c_EHZ(K)^2 / (2 vol(K)) for a polytope using HK2017.
pub fn compute_sys(polytope: &SysLandscapePolytopeCache) -> Option<f64> {
    let vol = euclidean_volume_f64(&polytope.vertices, &polytope.vertex_facet_incidence);
    if vol <= 0.0 {
        return None;
    }
    let cap = compute_capacity_result(polytope)?.capacity();
    let sys = systolic_ratio(cap, vol);
    sys.is_finite().then_some(sys)
}

/// Compute the active-orbit capacity result.
pub fn compute_capacity_result(polytope: &SysLandscapePolytopeCache) -> Option<OrbitSearchResult> {
    crate::capacity_auto(
        &polytope.dual_vertices_f64,
        &polytope.dual_vertices,
        &polytope.facet_intersection_is_nonempty,
        &polytope.omega_signs,
    )
    .ok()
}

pub fn orbit_scalars_from_result(result: &OrbitSearchResult) -> OrbitScalars {
    let best = result.best_orbit();
    OrbitScalars {
        iterations: result.iterations,
        returned_orbit_count: result.orbits.len(),
        best_beta_margin: best.beta_margin,
        best_q_error_bound: best.q_error_bound,
        best_has_mu: best.mu.is_some(),
        best_has_xi: best.xi.is_some(),
        best_is_admissible_exact: matches!(best.admissibility, OrbitAdmissibility::AdmissibleExact),
        best_is_indeterminate_f64: matches!(
            best.admissibility,
            OrbitAdmissibility::IndeterminateF64
        ),
    }
}

fn flatten_gradient(grad: &[Vector4<f64>]) -> Vec<f64> {
    grad.iter()
        .flat_map(|vk| [vk[0], vk[1], vk[2], vk[3]])
        .collect()
}

fn unflatten_direction(flat: &[f64]) -> Vec<Vector4<f64>> {
    flat.chunks_exact(4)
        .map(|chunk| Vector4::new(chunk[0], chunk[1], chunk[2], chunk[3]))
        .collect()
}

fn coordinate_bounds(flat_idx: usize, mode: AscentMode<'_>) -> (f64, f64) {
    let facet = flat_idx / 4;
    let component = flat_idx % 4;

    match mode {
        AscentMode::General => (-1.0, 1.0),
        AscentMode::LagrangianProduct { classification } => {
            let q_forbidden = classification.q_indices.contains(&facet) && component >= 2;
            let p_forbidden = classification.p_indices.contains(&facet) && component < 2;
            if q_forbidden || p_forbidden {
                (0.0, 0.0)
            } else {
                (-1.0, 1.0)
            }
        }
    }
}

fn maximin_subgradient_direction(
    subdiff: &[Vec<Vector4<f64>>],
    facet_count: usize,
    mode: AscentMode<'_>,
) -> Option<Vec<Vector4<f64>>> {
    let dim = facet_count * 4;
    let mut vars = variables!();
    let direction_vars: Vec<_> = (0..dim)
        .map(|flat_idx| {
            let (min, max) = coordinate_bounds(flat_idx, mode);
            vars.add(variable().min(min).max(max))
        })
        .collect();
    let t_var = vars.add(variable().min(f64::NEG_INFINITY));

    let mut model = vars.maximise(Expression::from(t_var)).using(default_solver);
    for grad in subdiff {
        let flat_grad = flatten_gradient(grad);
        let mut lhs = Expression::from(0.0);
        for (coeff, var) in flat_grad.iter().zip(&direction_vars) {
            if *coeff != 0.0 {
                lhs += *coeff * *var;
            }
        }
        model = model.with(constraint!(lhs >= t_var));
    }

    let solution = model.solve().ok()?;
    let flat_direction: Vec<f64> = direction_vars
        .iter()
        .map(|var| solution.value(*var))
        .collect();
    let direction = unflatten_direction(&flat_direction);
    let predicted = clarke_directional_derivative_a(subdiff, &direction).ok()?;

    (predicted > EPS_NUMERICAL_ZERO).then_some(direction)
}

fn admissible_active_orbits(result: &OrbitSearchResult) -> Vec<&OrbitKktData> {
    let tol = ACTIVE_ORBIT_RTOL * result.min_action.abs().max(1.0);
    let active: Vec<&OrbitKktData> = result
        .orbits
        .iter()
        .filter(|orbit| {
            matches!(
                orbit.admissibility,
                OrbitAdmissibility::AdmissibleF64 | OrbitAdmissibility::AdmissibleExact
            )
        })
        .filter(|orbit| (orbit.action - result.min_action).abs() <= tol)
        .collect();

    if active.is_empty() {
        vec![result.best_orbit()]
    } else {
        active
    }
}

/// Build the ascent direction for a single polytope state.
///
/// With a single active orbit, this reduces to that branch gradient. At
/// switching points, it solves a maximin LP for a feasible direction `d`
/// satisfying `max_d min_i <∇sys_i, d>` under box bounds on the ambient
/// coordinates.
pub fn ascent_direction(
    polytope: &SysLandscapePolytopeCache,
    state: &ActiveSysState,
    mode: AscentMode<'_>,
) -> Option<Vec<Vector4<f64>>> {
    let active_orbits: Vec<OrbitKktData> = admissible_active_orbits(&state.capacity)
        .into_iter()
        .cloned()
        .collect();
    let d_volume_da = volume_derivatives_a(
        &polytope.dual_vertices_f64,
        &polytope.vertices_f64,
        &polytope.vertex_facet_incidence,
    )
    .ok()?;
    let d_capacity_da =
        capacity_subgradients_a(&polytope.dual_vertices_f64, &active_orbits).ok()?;
    let subdiff: Vec<Vec<Vector4<f64>>> = d_capacity_da
        .iter()
        .map(|capacity_gradient| {
            systolic_ratio_gradient_a(
                state.capacity.capacity(),
                state.vol,
                capacity_gradient,
                &d_volume_da,
            )
        })
        .collect();
    match subdiff.as_slice() {
        [] => None,
        [single] => {
            let mut direction = single.clone();
            if let AscentMode::LagrangianProduct { classification } = mode {
                classification.mask_dual_direction_in_place(&mut direction);
            }
            Some(direction)
        }
        _ => maximin_subgradient_direction(&subdiff, polytope.facet_count(), mode),
    }
}

/// Try a step in dual-vertex space: a_k(t) = a_k + t * d_k.
pub fn apply_dual_step(
    duals: &[Vector4<f64>],
    direction: &[Vector4<f64>],
    t: f64,
) -> Option<(SysLandscapePolytopeCache, f64)> {
    let new_duals: Vec<Vector4<f64>> = duals
        .iter()
        .zip(direction)
        .map(|(a, d)| a + t * d)
        .collect();
    let polytope = SysLandscapePolytopeCache::from_f64_dual_vertices(new_duals)?;
    let sys = compute_sys(&polytope)?;
    Some((polytope, sys))
}

pub fn rational_vec4_to_strings(data: &[[BigRational; 4]]) -> Vec<[String; 4]> {
    data.iter()
        .map(|row| std::array::from_fn(|i| format!("{}/{}", row[i].numer(), row[i].denom())))
        .collect()
}

pub fn dual_vertices_rational_strings(polytope: &SysLandscapePolytopeCache) -> Vec<[String; 4]> {
    rational_vec4_to_strings(&polytope.dual_vertices)
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
/// general passes the literal `"general"` (see `gradient-ascent-general/main.rs`
/// line 507); products passes `lagrangian_{q_f}x{p_f}` where `q_f` and `p_f`
/// are the facet counts of the two Lagrangian factors (see
/// `gradient-ascent-products/main.rs` line 443, `bucket_name`).
///
/// The row stores both exact rational endpoint geometry and the legacy `f64`
/// endpoint dual vertices. The exact fields are the durable join surface for
/// later normalized datasets; the `f64` field remains for backwards-compatible
/// plotting and quick inspection.
#[derive(Debug, Serialize, Deserialize)]
pub struct SummaryRow {
    pub name: String,
    pub seed_index: usize,
    #[serde(default)]
    pub source_name: String,
    #[serde(default)]
    pub lineage_id: String,
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
    #[serde(default)]
    pub starting_dual_vertices_rational: Vec<[String; 4]>,
    #[serde(default)]
    pub final_dual_vertices_rational: Vec<[String; 4]>,
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
    pub final_polytope: SysLandscapePolytopeCache,
    pub final_record: PolytopeRecord,
}

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
) -> (Arc<Mutex<BufWriter<File>>>, Arc<Mutex<BufWriter<File>>>) {
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
        let mut w = summary_writer
            .lock()
            .expect("summary writer mutex poisoned");
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
    writers: &(Arc<Mutex<BufWriter<File>>>, Arc<Mutex<BufWriter<File>>>),
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
    trace_rows
        .dedup_by(|a, b| a.name == b.name && a.phase == b.phase && a.iteration == b.iteration);
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

#[cfg(test)]
mod tests {
    use super::*;
    use symplectic::classify_facets_from_dual_vertices;
    use symplectic::geom::polygon::regular_polygon_2d;

    fn triangle_product_cache() -> SysLandscapePolytopeCache {
        let (qn, qh) = regular_polygon_2d(3, 1.0);
        let (pn, ph) = regular_polygon_2d(3, 1.0);
        SysLandscapePolytopeCache::from_lagrangian_product(&qn, &qh, &pn, &ph)
            .expect("triangle product should construct")
    }

    #[test]
    fn compute_sys_from_capacity_matches_compute_sys() {
        let polytope = triangle_product_cache();
        let capacity = compute_capacity_result(&polytope)
            .expect("known polytope should have a capacity result");
        let cached = compute_sys_from_capacity(&polytope, &capacity)
            .expect("cached capacity result should produce sys");
        let direct = compute_sys(&polytope).expect("known polytope should produce sys");

        assert!(
            (cached - direct).abs() < 1e-12,
            "cached={cached}, direct={direct}"
        );
    }

    #[test]
    fn admissible_active_orbits_ignore_indeterminate_candidates() {
        let admissible = OrbitKktData {
            sigma: vec![0, 1],
            beta: vec![0.5, 0.5],
            beta_margin: 0.5,
            action: 1.0,
            action_lower: 1.0,
            action_upper: 1.0,
            q: 0.5,
            q_error_bound: 0.0,
            mu: Some([0.0; 4]),
            xi: Some(1.0),
            admissibility: OrbitAdmissibility::AdmissibleF64,
        };
        let indeterminate = OrbitKktData {
            sigma: vec![0, 2],
            beta: vec![1e-16, 1.0],
            beta_margin: 1e-16,
            action: 0.9,
            action_lower: 0.8,
            action_upper: 1.1,
            q: 0.55,
            q_error_bound: 0.1,
            mu: Some([0.0; 4]),
            xi: Some(1.0),
            admissibility: OrbitAdmissibility::IndeterminateF64,
        };
        let result = OrbitSearchResult {
            orbits: vec![indeterminate, admissible.clone()],
            min_action: admissible.action,
            min_action_lower: 0.8,
            min_action_upper: 1.0,
            iterations: 2,
        };

        let active = admissible_active_orbits(&result);
        assert_eq!(active.len(), 1);
        assert_eq!(active[0].sigma, admissible.sigma);
    }

    #[test]
    fn maximin_direction_finds_improving_switch_direction() {
        let subdiff = vec![
            vec![Vector4::new(1.0, 0.0, 0.0, 0.0)],
            vec![Vector4::new(0.0, 1.0, 0.0, 0.0)],
        ];

        let direction = maximin_subgradient_direction(&subdiff, 1, AscentMode::General)
            .expect("switching pair should admit a positive maximin direction");
        let predicted = clarke_directional_derivative_a(&subdiff, &direction)
            .expect("nonempty subdifferential should evaluate");

        assert!(
            predicted > 0.99,
            "predicted directional derivative = {predicted}"
        );
        assert!(direction[0][0] > 0.99, "direction = {:?}", direction[0]);
        assert!(direction[0][1] > 0.99, "direction = {:?}", direction[0]);
    }

    #[test]
    fn maximin_direction_respects_lp_coordinate_bounds() {
        let polytope = triangle_product_cache();
        let classification = classify_facets_from_dual_vertices(&polytope.dual_vertices_f64)
            .expect("triangle product should classify as a Lagrangian product");
        let facet_count = polytope.facet_count();
        let q_idx = classification.q_indices[0];
        let p_idx = classification.p_indices[0];

        let mut g1 = vec![Vector4::zeros(); facet_count];
        g1[q_idx] = Vector4::new(1.0, 2.0, 9.0, 11.0);
        g1[p_idx] = Vector4::new(8.0, 6.0, 1.0, 2.0);

        let mut g2 = vec![Vector4::zeros(); facet_count];
        g2[q_idx] = Vector4::new(2.0, 1.0, 7.0, 5.0);
        g2[p_idx] = Vector4::new(4.0, 3.0, 2.0, 1.0);

        let subdiff = vec![g1, g2];
        let direction = maximin_subgradient_direction(
            &subdiff,
            facet_count,
            AscentMode::LagrangianProduct {
                classification: &classification,
            },
        )
        .expect("LP-bounded switching pair should admit a positive direction");

        assert!(
            direction[q_idx][2].abs() < 1e-9,
            "direction = {:?}",
            direction[q_idx]
        );
        assert!(
            direction[q_idx][3].abs() < 1e-9,
            "direction = {:?}",
            direction[q_idx]
        );
        assert!(
            direction[p_idx][0].abs() < 1e-9,
            "direction = {:?}",
            direction[p_idx]
        );
        assert!(
            direction[p_idx][1].abs() < 1e-9,
            "direction = {:?}",
            direction[p_idx]
        );
    }
}

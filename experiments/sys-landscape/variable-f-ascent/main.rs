//! Variable-F gradient ascent: test whether allowing facet count to grow
//! (F → F+1) unlocks higher sys values than fixed-F optimization.
//!
//! Two research questions:
//! - RQ1: Can F=10 local maxima be improved by embedding into F=11 space?
//!   Take a local max, add a barely-non-redundant facet, run gradient ascent.
//! - RQ2: Four-way comparison from the same random F=10 start:
//!   Path A: F=10 gradient ascent
//!   Path B: add facet → F=11 gradient ascent
//!   Path C: random F=11 gradient ascent (baseline)
//!   Path D: F=10 ascent → add facet → F=11 ascent (optimize first, then expand)
//!
//! Gradient ascent algorithm copied from gradient-ascent-general/main.rs
//! (self-contained per experiment convention).
//!
//! Usage: cargo run -p exp-sys-landscape --release --bin sys-variable-f-ascent
//! Flags: --fresh  (clear existing data and rerun)
//!        --smoke  (run one bounded probe against temp output/cache)
//!        --out <path>  (override output JSONL path)
//! Input Artifacts: experiments/sys-landscape/gradient-ascent-general/gradient-ascent-general.jsonl
//! Output Artifacts: variable-f-ascent/variable-f-ascent.jsonl
//!         variable-f-ascent/cache.jsonl

use exp_sys_landscape::{
    compute_step_bound, continuation_cache_path, experiment_path, package_root,
    orbit_scalars_from_result, CONTINUATION_EXPERIMENT_DIR, GRADIENT_ASCENT_GENERAL_DIR,
    dual_vertices_rational_strings,
};
use nalgebra::Vector4;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use rand_distr::{Distribution, StandardNormal};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::PathBuf;
use std::time::{Instant, SystemTime, UNIX_EPOCH};
use symplectic::database::{load, save, DualVerticesKey, PolytopeRecord, SigmaAction};
use symplectic::derivatives::{capacity_derivatives_a_from_kkt_result, volume_derivatives_a};
use symplectic::geom::polytope::Polytope4D;
use symplectic::geom::skeleton::Skeleton;
use symplectic::geom::volume::volume;
use symplectic::kkt::saddle_point_solver::solve_kkt_for;
use symplectic::random::sample_random_polytope;

type Db = HashMap<DualVerticesKey, PolytopeRecord>;

// ============================================================================
// Configuration
// ============================================================================

/// Master seed for RQ2 random polytope generation. Different from
/// gradient-ascent-general (seed 42) to avoid overlap.
const SEED: u64 = 43;

/// Number of random facet placements per F=10 local max in RQ1.
/// 5 per source: enough to detect improvement if it occurs at ≥20% rate
/// (binomial P(0/5) = 33% miss rate). Increase for higher-confidence
/// improvement rates.
const N_PLACEMENTS_RQ1: usize = 5;

/// Number of random starting polytopes for RQ2.
/// 10 seeds: matches gradient-ascent-general. Increase for tighter
/// confidence intervals on mean sys.
const N_SEEDS_RQ2: usize = 10;

/// Base facet count.
const FACET_COUNT: usize = 10;

/// Height range for random F=10 generation.
const H_MIN: f64 = 0.8;
const H_MAX: f64 = 1.2;

/// Depth parameter for facet addition: a_{F+1} = n / (h_K(n) - ε).
/// 1e-3 used in facet-splitting experiment (SPLITTING_EPSILONS range
/// [1e-3, 1e-4]). Chosen as upper end: small enough that the (F+1)-polytope
/// is close to the F-polytope; large enough that the new facet is robustly
/// non-redundant at f64 precision. If changed, verify that Polytope4D
/// construction doesn't produce RedundantFacet errors at smaller ε.
const FACET_EPSILON: f64 = 1e-3;

// --- Gradient ascent parameters (copied from gradient-ascent-general) ---

/// Maximum gradient ascent iterations per phase.
const MAX_ITERATIONS: usize = 30;

/// Minimum improvement per iteration to continue. Well above f64 noise
/// (~1e-15) but small enough to capture meaningful steps. Matches
/// gradient-ascent-general. If changed, re-check convergence rates.
const CONVERGENCE_THRESHOLD: f64 = 1e-6;

/// Step fractions of t_max for within-bound line search.
const STEP_FRACTIONS: &[f64] = &[0.1, 0.25, 0.5, 0.75, 0.95];

/// Multipliers beyond t_max for crossing combinatorial boundaries.
const OVERSHOOT_MULTIPLIERS: &[f64] = &[1.5, 2.0, 3.0];

/// Prevents pathological steps when t_max is huge.
const MAX_STEP_SIZE: f64 = 100.0;

/// Number of random dual-vertex perturbations per escape round.
const N_WIGGLES: usize = 5;

/// Multiplicative perturbation scale for dual vertex components: a_k[i] -> a_k[i] * (1 + 0.05 * N(0,1)).
/// Per-facet displacement has expected norm ~0.05 * |a_k| (unit-scale dual vertices: ~0.05).
/// Local cell-width probes and prior ascent runs suggest this scale is large
/// enough to cross boundaries often, while still small enough to keep most
/// perturbed polytopes constructible. See
/// `research/combinatorial-cells.md` and
/// `research/sys-landscape.md`.
/// If changed: much smaller (e.g. 0.01) reduces boundary-crossing probability and escape
/// effectiveness. Much larger (e.g. 0.2) risks producing degenerate polytopes
/// (Polytope4D::from_f64 failure) or landing too far from the current optimum.
const WIGGLE_STRENGTH: f64 = 0.05;

/// Maximum rounds of escape attempts after convergence.
const MAX_ESCAPE_ROUNDS: usize = 3;

/// Per-trial time budget. 180s is 1.5x gradient-ascent-general's 120s,
/// accounting for F=11 having ~1.5x more orbits than F=10
/// (C(11,4)/C(10,4) = 330/210 ≈ 1.57). Initial run (2026-04-04):
/// max trial time was ~59s, so 180s is generous.
const TRIAL_TIME_BUDGET_SECS: f64 = 180.0;

/// Smoke runs are bounded to one gradient-ascent iteration and no escape rounds.
const SMOKE_TRIAL_TIME_BUDGET_SECS: f64 = 30.0;

/// Numerical zero threshold for gradient norms, rates, and slack comparisons.
/// Near machine epsilon for unit-scale f64. Matches gradient-ascent-general.
const EPS: f64 = 1e-15;

// ============================================================================
// Output schema
// ============================================================================

#[derive(Debug, Serialize)]
struct ResultRow {
    /// "rq1" or "rq2"
    rq: String,
    /// "f10_localmax_then_f11", "f10_ascent", "f10_add_then_f11", "random_f11", "f10_ascent_then_f11"
    path: String,
    /// Seed/source identifier
    name: String,
    /// External source or seed group that defines the lineage
    source_name: String,
    /// Stable lineage identifier across related paths or placements
    lineage_id: String,
    /// Parent trial row when one exists in this dataset
    #[serde(skip_serializing_if = "Option::is_none")]
    direct_parent_trial: Option<String>,
    /// Facet count at start of gradient ascent
    starting_f: usize,
    /// sys before any optimization in this trial
    starting_sys: f64,
    /// sys immediately after facet addition (before ascent), or null
    sys_after_addition: Option<f64>,
    /// sys after gradient ascent
    final_sys: f64,
    /// final_sys - starting_sys (of the source F=10 polytope for RQ1, of start for RQ2)
    delta_vs_source: f64,
    /// Total gradient iterations across all phases
    n_iterations: usize,
    /// Number of ascent phases (initial + escape rounds)
    n_phases: usize,
    /// Facet placement direction (unit vector), or null
    placement_direction: Option<[f64; 4]>,
    /// Whether the added facet is still non-redundant at the end
    facet_remained_active: Option<bool>,
    /// Wall-clock time for this trial
    total_time_ms: f64,
    /// Exact dual vertices at the start of the ascent stage
    starting_dual_vertices_rational: Vec<[String; 4]>,
    /// Exact dual vertices immediately after facet addition, if applicable
    #[serde(skip_serializing_if = "Option::is_none")]
    after_addition_dual_vertices_rational: Option<Vec<[String; 4]>>,
    /// Exact dual vertices at the endpoint
    final_dual_vertices_rational: Vec<[String; 4]>,
    /// Final dual vertices
    final_dual_vertices: Vec<[f64; 4]>,
}

/// For loading gradient-ascent-general results.
#[derive(Debug, Deserialize)]
struct GradientAscentRow {
    name: String,
    final_sys: f64,
    final_dual_vertices: Vec<[f64; 4]>,
}

// ============================================================================
// Gradient step in a-space (copied from gradient-ascent-general)
// ============================================================================

fn compute_sys(polytope: &Polytope4D, db: &mut Db) -> Option<f64> {
    let vol = volume(polytope);
    if vol <= 0.0 {
        return None;
    }
    let cap = compute_capacity(polytope, db)?;
    persist_scalar_fields(polytope, vol, cap, db);
    let sys = cap * cap / (2.0 * vol);
    sys.is_finite().then_some(sys)
}

fn try_step_a(
    duals: &[Vector4<f64>],
    direction: &[Vector4<f64>],
    t: f64,
    db: &mut Db,
) -> Option<(Polytope4D, f64)> {
    let new_duals: Vec<Vector4<f64>> = duals
        .iter()
        .zip(direction)
        .map(|(a, d)| a + t * d)
        .collect();
    let polytope = Polytope4D::from_f64(new_duals).ok()?;
    let sys = compute_sys(&polytope, db)?;
    Some((polytope, sys))
}

fn compute_capacity(polytope: &Polytope4D, db: &mut Db) -> Option<f64> {
    let key: DualVerticesKey = polytope.dual_vertices().to_vec();
    if let Some(record) = db.get(&key) {
        if let Some(cap) = record.capacity {
            return Some(cap);
        }
    }
    let r = symplectic::ehz_capacity(polytope).ok()?;
    let cap = r.capacity();
    let record = db
        .entry(key)
        .or_insert_with(|| PolytopeRecord::from_polytope(polytope));
    record.capacity = Some(cap);
    if record.capacity_err.is_none() {
        record.capacity_err = Some(0.0);
    }
    Some(cap)
}

fn compute_capacity_result(polytope: &Polytope4D, db: &mut Db) -> Option<(f64, Vec<usize>)> {
    let key: DualVerticesKey = polytope.dual_vertices().to_vec();
    // Check cache: need both capacity and best permutation
    if let Some(record) = db.get(&key) {
        if let (Some(cap), Some(sigmas)) = (record.capacity, record.sigmas.as_ref()) {
            if let Some(best) = sigmas.first() {
                return Some((cap, best.perm.clone()));
            }
        }
    }
    let r = symplectic::ehz_capacity(polytope).ok()?;
    let cap = r.capacity();
    let perm = r.best_sigma().to_vec();
    let record = db
        .entry(key)
        .or_insert_with(|| PolytopeRecord::from_polytope(polytope));
    record.capacity = Some(cap);
    if record.capacity_err.is_none() {
        record.capacity_err = Some(0.0);
    }
    if record.sigma_gap_cutoff.is_none() {
        record.sigma_gap_cutoff = Some(0.0);
    }
    record.sigmas = Some(vec![SigmaAction {
        perm: perm.clone(),
        action: cap,
    }]);
    if record.orbit_scalars.is_none() {
        record.orbit_scalars = Some(orbit_scalars_from_result(&r));
    }
    Some((cap, perm))
}

// ============================================================================
// Gradient ascent with integrated overshoot (copied from gradient-ascent-general)
// ============================================================================

struct AscentResult {
    final_polytope: Polytope4D,
    final_sys: f64,
    n_iters: usize,
    n_phases: usize,
}

/// Single gradient ascent phase: iterate until convergence or budget.
/// Gradient: d(sys)/d(a_k) = (cap * d(cap)/d(a_k) - sys * d(vol)/d(a_k)) / vol
// TODO: add [lem:sys-sensitivity] to formal math (see gradient-correctness experiment)
fn gradient_ascent_phase(
    start: &Polytope4D,
    t0: Instant,
    budget: f64,
    db: &mut Db,
) -> Option<(Polytope4D, f64, usize)> {
    gradient_ascent_phase_limited(start, t0, budget, db, MAX_ITERATIONS)
}

fn gradient_ascent_phase_limited(
    start: &Polytope4D,
    t0: Instant,
    budget: f64,
    db: &mut Db,
    max_iterations: usize,
) -> Option<(Polytope4D, f64, usize)> {
    let mut current = Polytope4D::from_f64(start.dual_vertices_f64().to_vec()).ok()?;
    let mut current_sys = compute_sys(&current, db)?;
    let mut n_iters = 0usize;

    for iter in 0..max_iterations {
        if t0.elapsed().as_secs_f64() > budget {
            break;
        }

        let (cap, best_perm) = compute_capacity_result(&current, db)?;
        let kkt = solve_kkt_for(&current, &best_perm).feasible()?;
        let vol = volume(&current);
        if vol <= 0.0 {
            return None;
        }
        let sys = cap * cap / (2.0 * vol);
        let duals = current.dual_vertices_f64();

        let d_vol_a = volume_derivatives_a(&current);
        let d_cap_a = capacity_derivatives_a_from_kkt_result(&current, &best_perm, &kkt);
        let d_sys_a: Vec<Vector4<f64>> = d_vol_a
            .iter()
            .zip(d_cap_a.iter())
            .map(|(dv, dc)| (cap * dc - sys * dv) / vol)
            .collect();

        let gradient_norm = d_sys_a.iter().map(|d| d.norm_squared()).sum::<f64>().sqrt();
        if gradient_norm < EPS {
            break;
        }

        let t_max = compute_step_bound(&current, &d_sys_a);
        if t_max <= 0.0 {
            break;
        }

        let mut best: Option<(Polytope4D, f64)> = None;

        for &frac in STEP_FRACTIONS {
            let t = frac * t_max;
            if let Some((p, new_sys)) = try_step_a(duals, &d_sys_a, t, db) {
                if new_sys > sys && best.as_ref().is_none_or(|b| new_sys > b.1) {
                    best = Some((p, new_sys));
                }
            }
        }

        if t_max < MAX_STEP_SIZE {
            for &mult in OVERSHOOT_MULTIPLIERS {
                let t = mult * t_max;
                if let Some((p, new_sys)) = try_step_a(duals, &d_sys_a, t, db) {
                    if new_sys > sys && best.as_ref().is_none_or(|b| new_sys > b.1) {
                        best = Some((p, new_sys));
                    }
                }
            }
        }

        match best {
            Some((new_polytope, new_sys)) => {
                let delta = new_sys - sys;
                current = new_polytope;
                current_sys = new_sys;
                n_iters = iter + 1;
                if delta < CONVERGENCE_THRESHOLD {
                    break;
                }
            }
            None => break,
        }
    }

    Some((current, current_sys, n_iters))
}

fn wiggle(polytope: &Polytope4D, rng: &mut ChaCha8Rng) -> Option<Polytope4D> {
    let duals = polytope.dual_vertices_f64();
    let new_duals: Vec<Vector4<f64>> = duals
        .iter()
        .map(|a| {
            a.map(|c| {
                let noise: f64 = StandardNormal.sample(rng);
                c * (1.0 + WIGGLE_STRENGTH * noise)
            })
        })
        .collect();
    Polytope4D::from_f64(new_duals).ok()
}

/// Full gradient ascent: initial phase + escape rounds.
fn full_ascent(
    start: &Polytope4D,
    rng: &mut ChaCha8Rng,
    budget: f64,
    db: &mut Db,
) -> Option<AscentResult> {
    full_ascent_limited(
        start,
        rng,
        budget,
        db,
        MAX_ITERATIONS,
        MAX_ESCAPE_ROUNDS,
        N_WIGGLES,
    )
}

fn full_ascent_limited(
    start: &Polytope4D,
    rng: &mut ChaCha8Rng,
    budget: f64,
    db: &mut Db,
    max_iterations: usize,
    max_escape_rounds: usize,
    n_wiggles: usize,
) -> Option<AscentResult> {
    let t0 = Instant::now();

    let (mut best_polytope, mut best_sys, mut total_iters) =
        gradient_ascent_phase_limited(start, t0, budget, db, max_iterations)?;
    let mut n_phases = 1usize;

    for _round in 0..max_escape_rounds {
        if t0.elapsed().as_secs_f64() > budget {
            break;
        }
        let mut escaped = false;
        for _ in 0..n_wiggles {
            if t0.elapsed().as_secs_f64() > budget {
                break;
            }
            if let Some(wiggled) = wiggle(&best_polytope, rng) {
                if let Some((p, s, iters)) =
                    gradient_ascent_phase_limited(&wiggled, t0, budget, db, max_iterations)
                {
                    n_phases += 1;
                    total_iters += iters;
                    if s > best_sys + CONVERGENCE_THRESHOLD {
                        best_sys = s;
                        best_polytope = p;
                        escaped = true;
                        break;
                    }
                }
            }
        }
        if !escaped {
            break;
        }
    }

    Some(AscentResult {
        final_polytope: best_polytope,
        final_sys: best_sys,
        n_iters: total_iters,
        n_phases,
    })
}

// ============================================================================
// Facet addition
// ============================================================================

/// Add a barely-non-redundant facet to a polytope.
///
/// New dual vertex: a_{F+1} = n / (h_K(n) - ε) where h_K(n) = max_v ⟨n,v⟩.
/// This creates an (F+1)-facet polytope that is close to the original:
/// the new halfspace ⟨n,x⟩ ≤ h_K(n) - ε shaves a thin sliver.
///
/// Pattern from facet-splitting/main.rs.
// TODO: add [lem:facet-addition] to formal math (dual vertex ↔ halfspace correspondence)
fn add_facet(polytope: &Polytope4D, direction: &Vector4<f64>, epsilon: f64) -> Option<Polytope4D> {
    let vertices = polytope.vertices_f64();
    let h_k_n = vertices
        .iter()
        .map(|v| direction.dot(v))
        .fold(f64::NEG_INFINITY, f64::max);
    let new_h = h_k_n - epsilon;
    if new_h <= 0.0 {
        return None;
    }
    let mut new_duals: Vec<Vector4<f64>> = polytope.dual_vertices_f64().to_vec();
    new_duals.push(direction / new_h);
    Polytope4D::from_f64(new_duals).ok()
}

/// Sample a random unit direction in R^4.
/// rand_distr::UnitSphere only supports 3D, so we use 4 Gaussians + normalize
/// (standard method for uniform sampling on S^{n-1}).
fn random_direction(rng: &mut ChaCha8Rng) -> Vector4<f64> {
    let x: f64 = StandardNormal.sample(rng);
    let y: f64 = StandardNormal.sample(rng);
    let z: f64 = StandardNormal.sample(rng);
    let w: f64 = StandardNormal.sample(rng);
    let v = Vector4::new(x, y, z, w);
    v.normalize()
}

/// Check whether the last facet (index F) is still non-redundant.
/// A facet is "active" if it has at least one incident vertex.
fn last_facet_active(polytope: &Polytope4D) -> bool {
    let f = polytope.facet_count();
    let last_idx = f - 1;
    let skeleton = Skeleton::compute(polytope);
    skeleton
        .vertex_facets
        .iter()
        .any(|facets| facets.contains(&last_idx))
}

// ============================================================================
// Loading F=10 local maxima from gradient-ascent-general
// ============================================================================

fn load_local_maxima(path: &std::path::Path) -> Vec<(String, f64, Polytope4D)> {
    let mut results = Vec::new();
    let file = match File::open(path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!(
                "WARNING: cannot load local maxima from {}: {e}",
                path.display()
            );
            return results;
        }
    };
    let reader = BufReader::new(file);
    for line in reader.lines().map_while(Result::ok) {
        let line = line.trim().to_string();
        if line.is_empty() {
            continue;
        }
        if let Ok(row) = serde_json::from_str::<GradientAscentRow>(&line) {
            let duals: Vec<Vector4<f64>> = row
                .final_dual_vertices
                .iter()
                .map(|a| Vector4::new(a[0], a[1], a[2], a[3]))
                .collect();
            if let Ok(p) = Polytope4D::from_f64(duals) {
                results.push((row.name, row.final_sys, p));
            }
        }
    }
    results
}

// ============================================================================
// Resume support
// ============================================================================

fn load_completed_names(path: &std::path::Path) -> HashSet<String> {
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

// ============================================================================
// Helpers
// ============================================================================

fn dvs_to_array(polytope: &Polytope4D) -> Vec<[f64; 4]> {
    polytope
        .dual_vertices_f64()
        .iter()
        .map(|a| [a[0], a[1], a[2], a[3]])
        .collect()
}

fn persist_scalar_fields(polytope: &Polytope4D, vol: f64, cap: f64, db: &mut Db) {
    let key: DualVerticesKey = polytope.dual_vertices().to_vec();
    let record = db
        .entry(key)
        .or_insert_with(|| PolytopeRecord::from_polytope(polytope));
    if record.volume.is_none() {
        record.volume = Some(vol);
    }
    if record.volume_err.is_none() {
        record.volume_err = Some(0.0);
    }
    if record.capacity.is_none() {
        record.capacity = Some(cap);
    }
    if record.capacity_err.is_none() {
        record.capacity_err = Some(0.0);
    }
}

fn write_row(row: &ResultRow, writer: &mut BufWriter<File>) {
    serde_json::to_writer(&mut *writer, row).expect("write row");
    writeln!(writer).expect("newline");
}

fn smoke_paths() -> (PathBuf, PathBuf) {
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock before UNIX_EPOCH")
        .as_millis();
    let smoke_dir = std::env::temp_dir().join(format!(
        "sys-variable-f-ascent-smoke-{}-{stamp}",
        std::process::id()
    ));
    std::fs::create_dir_all(&smoke_dir).expect("create smoke temp dir");
    (
        smoke_dir.join("smoke-variable-f-ascent.jsonl"),
        smoke_dir.join("smoke-cache.jsonl"),
    )
}

fn smoke_run(
    package_root: &std::path::Path,
    output_path: &std::path::Path,
    cache_path: &std::path::Path,
    writer: &mut BufWriter<File>,
    rng: &mut ChaCha8Rng,
    db: &mut Db,
) {
    println!("Smoke mode: temp output {}", output_path.display());
    println!("Smoke mode: temp cache   {}", cache_path.display());

    let ga_path = package_root.join("gradient-ascent-general/gradient-ascent-general.jsonl");
    let local_maxima = load_local_maxima(&ga_path);
    println!(
        "Smoke mode: loaded {} local maxima from {}",
        local_maxima.len(),
        ga_path.display()
    );

    let (trial_name, source_name, source_sys, start_polytope) =
        if let Some((src_name, src_sys, src_polytope)) = local_maxima.first() {
            (
                format!("smoke_{src_name}"),
                src_name.clone(),
                *src_sys,
                src_polytope.clone(),
            )
        } else {
            println!("Smoke mode: no local maxima found, sampling a random F=10 polytope.");
            let mut sampled = None;
            for _ in 0..100 {
                if let Ok(p) = sample_random_polytope(FACET_COUNT, H_MIN, H_MAX, rng) {
                    sampled = Some(p);
                    break;
                }
            }
            let polytope = sampled.expect("smoke fallback polytope generation");
            let sys = compute_sys(&polytope, db).expect("compute smoke start sys");
            (
                "smoke_random_f10".to_string(),
                "smoke_random_f10".to_string(),
                sys,
                polytope,
            )
        };

    let start_sys = compute_sys(&start_polytope, db).expect("compute smoke start sys");
    let dir = random_direction(rng);
    let f11_polytope =
        add_facet(&start_polytope, &dir, FACET_EPSILON).expect("smoke facet addition");
    let sys_after_add = compute_sys(&f11_polytope, db);
    let t0 = Instant::now();
    let result = full_ascent_limited(
        &f11_polytope,
        rng,
        SMOKE_TRIAL_TIME_BUDGET_SECS,
        db,
        1,
        0,
        0,
    )
    .expect("smoke ascent");
    let active = last_facet_active(&result.final_polytope);

    let row = ResultRow {
        rq: "smoke".into(),
        path: "smoke_probe".into(),
        name: trial_name,
        source_name: source_name.clone(),
        lineage_id: format!("smoke::{source_name}"),
        direct_parent_trial: None,
        starting_f: 11,
        starting_sys: start_sys,
        sys_after_addition: sys_after_add,
        final_sys: result.final_sys,
        delta_vs_source: result.final_sys - source_sys,
        n_iterations: result.n_iters,
        n_phases: result.n_phases,
        placement_direction: Some([dir[0], dir[1], dir[2], dir[3]]),
        facet_remained_active: Some(active),
        total_time_ms: t0.elapsed().as_secs_f64() * 1000.0,
        starting_dual_vertices_rational: dual_vertices_rational_strings(&f11_polytope),
        after_addition_dual_vertices_rational: Some(dual_vertices_rational_strings(&f11_polytope)),
        final_dual_vertices_rational: dual_vertices_rational_strings(&result.final_polytope),
        final_dual_vertices: dvs_to_array(&result.final_polytope),
    };
    write_row(&row, writer);
    writer.flush().expect("flush smoke output");
    save(cache_path, db).expect("save smoke cache");
}

// ============================================================================
// Main
// ============================================================================

fn main() {
    let t_global = Instant::now();
    let base = package_root().join(CONTINUATION_EXPERIMENT_DIR);
    let default_output_path = base.join("variable-f-ascent.jsonl");

    println!("variable-f-ascent: variable-F gradient ascent experiment\n");

    // CLI args
    let args: Vec<String> = std::env::args().collect();
    let mut smoke = false;
    let fresh = args.iter().any(|a| a == "--fresh");
    let mut out_path: Option<PathBuf> = None;

    let mut i = 1usize;
    while i < args.len() {
        match args[i].as_str() {
            "--fresh" => {
                i += 1;
            }
            "--smoke" => {
                smoke = true;
                i += 1;
            }
            "--out" => {
                let value = args.get(i + 1).expect("--out requires a value");
                out_path = Some(PathBuf::from(value));
                i += 2;
            }
            other => {
                panic!("unknown argument: {other}");
            }
        }
    }

    let (output_path, cache_path) = if smoke {
        let (smoke_output_path, smoke_cache_path) = smoke_paths();
        (out_path.unwrap_or(smoke_output_path), smoke_cache_path)
    } else {
        (out_path.unwrap_or(default_output_path), continuation_cache_path())
    };

    let completed = if smoke {
        HashSet::new()
    } else if fresh {
        let _ = std::fs::remove_file(&output_path);
        HashSet::new()
    } else {
        load_completed_names(&output_path)
    };

    if completed.is_empty() {
        println!("Starting fresh run.");
    } else {
        println!("Resuming: {} trials already completed.", completed.len());
    }

    if let Some(parent) = output_path.parent() {
        std::fs::create_dir_all(parent).expect("create output dir");
    }

    let output_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&output_path)
        .expect("open output JSONL");
    let mut writer = BufWriter::new(output_file);

    let mut rng = ChaCha8Rng::seed_from_u64(SEED);

    // Local capacity cache — avoids recomputing ehz_capacity on reruns
    // without bloating the shared sys-landscape family cache with thousands
    // of intermediate gradient-step polytopes.
    let mut db: Db = load(&cache_path).expect("load cache");
    println!("Cache: {} entries\n", db.len());

    if smoke {
        smoke_run(
            &package_root(),
            &output_path,
            &cache_path,
            &mut writer,
            &mut rng,
            &mut db,
        );
        println!("Smoke complete.");
        println!("Output: {}", output_path.display());
        return;
    }

    // =========================================================================
    // RQ1: Can F=10 local maxima be improved in F=11 space?
    // =========================================================================

    println!("=== RQ1: Improving F=10 local maxima in F=11 space ===\n");

    // Load local maxima from gradient-ascent-general
    let ga_path = experiment_path(GRADIENT_ASCENT_GENERAL_DIR, "gradient-ascent-general.jsonl");
    let local_maxima = load_local_maxima(&ga_path);
    println!(
        "Loaded {} local maxima from gradient-ascent-general.\n",
        local_maxima.len()
    );

    let mut rq1_improved = 0usize;
    let mut rq1_total = 0usize;
    let mut best_rq1_sys = 0.0f64;

    for (src_name, src_sys, src_polytope) in &local_maxima {
        for placement_idx in 0..N_PLACEMENTS_RQ1 {
            let trial_name = format!("rq1_{src_name}_p{placement_idx}");
            if completed.contains(&trial_name) {
                continue;
            }

            let t0 = Instant::now();
            let dir = random_direction(&mut rng);

            // Add facet
            let f11_polytope = match add_facet(src_polytope, &dir, FACET_EPSILON) {
                Some(p) => p,
                None => {
                    println!("  [{trial_name}] facet addition failed (h_K(n) - ε ≤ 0)");
                    continue;
                }
            };

            let sys_after_add = compute_sys(&f11_polytope, &mut db);
            let sys_after_add_val = match sys_after_add {
                Some(s) => s,
                None => {
                    println!("  [{trial_name}] sys computation failed after addition");
                    continue;
                }
            };

            // Run gradient ascent on F=11 polytope
            match full_ascent(&f11_polytope, &mut rng, TRIAL_TIME_BUDGET_SECS, &mut db) {
                Some(result) => {
                    let delta = result.final_sys - src_sys;
                    rq1_total += 1;
                    let improved = result.final_sys > *src_sys + CONVERGENCE_THRESHOLD;
                    if improved {
                        rq1_improved += 1;
                    }
                    if result.final_sys > best_rq1_sys {
                        best_rq1_sys = result.final_sys;
                    }

                    let active = last_facet_active(&result.final_polytope);

                    let row = ResultRow {
                        rq: "rq1".into(),
                        path: "f10_localmax_then_f11".into(),
                        name: trial_name.clone(),
                        source_name: src_name.clone(),
                        lineage_id: format!("rq1::{src_name}"),
                        direct_parent_trial: None,
                        starting_f: 11,
                        starting_sys: *src_sys,
                        sys_after_addition: Some(sys_after_add_val),
                        final_sys: result.final_sys,
                        delta_vs_source: delta,
                        n_iterations: result.n_iters,
                        n_phases: result.n_phases,
                        placement_direction: Some([dir[0], dir[1], dir[2], dir[3]]),
                        facet_remained_active: Some(active),
                        total_time_ms: t0.elapsed().as_secs_f64() * 1000.0,
                        starting_dual_vertices_rational: dual_vertices_rational_strings(
                            &f11_polytope,
                        ),
                        after_addition_dual_vertices_rational: Some(
                            dual_vertices_rational_strings(&f11_polytope),
                        ),
                        final_dual_vertices_rational: dual_vertices_rational_strings(
                            &result.final_polytope,
                        ),
                        final_dual_vertices: dvs_to_array(&result.final_polytope),
                    };
                    write_row(&row, &mut writer);

                    let marker = if improved { " *** IMPROVED ***" } else { "" };
                    println!(
                        "  [{trial_name}] src_sys={:.4} → add={:.4} → final={:.4} (Δ={delta:+.4}), \
                         active={active}, {:.1}s{marker}",
                        src_sys, sys_after_add_val, result.final_sys,
                        t0.elapsed().as_secs_f64(),
                    );

                    if result.final_sys > 1.0 {
                        eprintln!(
                            "*** VITERBO VIOLATION: {} sys={:.6} ***",
                            trial_name, result.final_sys
                        );
                    }
                }
                None => {
                    println!("  [{trial_name}] gradient ascent failed");
                }
            }
        }
    }

    println!("\nRQ1 summary: {rq1_improved}/{rq1_total} trials improved over F=10 local max.");
    println!("Best RQ1 sys: {best_rq1_sys:.6}\n");

    // =========================================================================
    // RQ2: Three-way comparison from random F=10 starts
    // =========================================================================

    println!("=== RQ2: Four-way comparison ===\n");

    // Generate F=10 starting polytopes
    let mut rq2_starts: Vec<(String, Polytope4D)> = Vec::new();
    let mut attempts = 0usize;
    while rq2_starts.len() < N_SEEDS_RQ2 {
        attempts += 1;
        if attempts > N_SEEDS_RQ2 * 100 {
            eprintln!("WARNING: gave up generating F=10 polytopes after {attempts} attempts");
            break;
        }
        if let Ok(p) = sample_random_polytope(FACET_COUNT, H_MIN, H_MAX, &mut rng) {
            let name = format!("rq2_seed{}", rq2_starts.len());
            rq2_starts.push((name, p));
        }
    }
    println!("Generated {} F=10 starting polytopes.\n", rq2_starts.len());

    // Generate F=11 random polytopes for Path C (same count)
    let mut rq2_f11_random: Vec<(String, Polytope4D)> = Vec::new();
    attempts = 0;
    while rq2_f11_random.len() < N_SEEDS_RQ2 {
        attempts += 1;
        if attempts > N_SEEDS_RQ2 * 100 {
            eprintln!("WARNING: gave up generating F=11 polytopes after {attempts} attempts");
            break;
        }
        if let Ok(p) = sample_random_polytope(FACET_COUNT + 1, H_MIN, H_MAX, &mut rng) {
            let name = format!("rq2_seed{}", rq2_f11_random.len());
            rq2_f11_random.push((name, p));
        }
    }
    println!(
        "Generated {} F=11 random polytopes.\n",
        rq2_f11_random.len()
    );

    for (idx, (seed_name, start_polytope)) in rq2_starts.iter().enumerate() {
        let start_sys = match compute_sys(start_polytope, &mut db) {
            Some(s) => s,
            None => {
                println!("[{seed_name}] sys computation failed, skipping");
                continue;
            }
        };
        println!("[{seed_name}] starting sys={start_sys:.4}");

        // --- Path A: F=10 gradient ascent ---
        // Always recomputed (even on resume) because Path D needs the in-memory
        // polytope. With a warm cache.jsonl, capacity lookups are all hits and
        // this takes <1s per seed (~10s total for 10 seeds).
        let path_a_name = format!("{seed_name}_pathA_f10");
        let mut path_a_result: Option<(Polytope4D, f64)> = None;
        {
            let t0 = Instant::now();
            match full_ascent(start_polytope, &mut rng, TRIAL_TIME_BUDGET_SECS, &mut db) {
                Some(result) => {
                    if !completed.contains(&path_a_name) {
                        let row = ResultRow {
                            rq: "rq2".into(),
                            path: "f10_ascent".into(),
                            name: path_a_name.clone(),
                            source_name: seed_name.clone(),
                            lineage_id: format!("rq2::{seed_name}"),
                            direct_parent_trial: None,
                            starting_f: 10,
                            starting_sys: start_sys,
                            sys_after_addition: None,
                            final_sys: result.final_sys,
                            delta_vs_source: result.final_sys - start_sys,
                            n_iterations: result.n_iters,
                            n_phases: result.n_phases,
                            placement_direction: None,
                            facet_remained_active: None,
                            total_time_ms: t0.elapsed().as_secs_f64() * 1000.0,
                            starting_dual_vertices_rational: dual_vertices_rational_strings(
                                start_polytope,
                            ),
                            after_addition_dual_vertices_rational: None,
                            final_dual_vertices_rational: dual_vertices_rational_strings(
                                &result.final_polytope,
                            ),
                            final_dual_vertices: dvs_to_array(&result.final_polytope),
                        };
                        write_row(&row, &mut writer);
                    }
                    println!(
                        "  [A: F=10 ascent] final_sys={:.4} (Δ={:+.4}), {:.1}s",
                        result.final_sys,
                        result.final_sys - start_sys,
                        t0.elapsed().as_secs_f64(),
                    );
                    path_a_result = Some((result.final_polytope, result.final_sys));
                }
                None => println!("  [A: F=10 ascent] FAILED"),
            }
        }

        // --- Path D: F=10 ascent → add facet → F=11 ascent ---
        let path_d_name = format!("{seed_name}_pathD_f10then11");
        if !completed.contains(&path_d_name) {
            if let Some((ref a_polytope, a_sys)) = path_a_result {
                let t0 = Instant::now();
                let dir = random_direction(&mut rng);

                match add_facet(a_polytope, &dir, FACET_EPSILON) {
                    Some(f11_polytope) => {
                        let sys_after_add = compute_sys(&f11_polytope, &mut db);
                        match full_ascent(&f11_polytope, &mut rng, TRIAL_TIME_BUDGET_SECS, &mut db)
                        {
                            Some(result) => {
                                let active = last_facet_active(&result.final_polytope);
                                let row =
                                    ResultRow {
                                        rq: "rq2".into(),
                                        path: "f10_ascent_then_f11".into(),
                                        name: path_d_name.clone(),
                                        source_name: seed_name.clone(),
                                        lineage_id: format!("rq2::{seed_name}"),
                                        direct_parent_trial: Some(path_a_name.clone()),
                                        starting_f: 11,
                                        starting_sys: start_sys,
                                        sys_after_addition: sys_after_add,
                                        final_sys: result.final_sys,
                                        delta_vs_source: result.final_sys - start_sys,
                                        n_iterations: result.n_iters,
                                        n_phases: result.n_phases,
                                        placement_direction: Some([dir[0], dir[1], dir[2], dir[3]]),
                                        facet_remained_active: Some(active),
                                        total_time_ms: t0.elapsed().as_secs_f64() * 1000.0,
                                        starting_dual_vertices_rational:
                                            dual_vertices_rational_strings(&f11_polytope),
                                        after_addition_dual_vertices_rational: Some(
                                            dual_vertices_rational_strings(&f11_polytope),
                                        ),
                                        final_dual_vertices_rational:
                                            dual_vertices_rational_strings(&result.final_polytope),
                                        final_dual_vertices: dvs_to_array(&result.final_polytope),
                                    };
                                write_row(&row, &mut writer);
                                println!(
                                    "  [D: F=10→F=11] a_sys={a_sys:.4} → add={:.4} → final={:.4} (Δ={:+.4}), active={active}, {:.1}s",
                                    sys_after_add.unwrap_or(f64::NAN),
                                    result.final_sys,
                                    result.final_sys - start_sys,
                                    t0.elapsed().as_secs_f64(),
                                );
                            }
                            None => println!("  [D: F=10→F=11] gradient ascent FAILED"),
                        }
                    }
                    None => println!("  [D: F=10→F=11] facet addition FAILED"),
                }
            }
        }

        // --- Path B: add facet → F=11 gradient ascent ---
        let path_b_name = format!("{seed_name}_pathB_f11add");
        if !completed.contains(&path_b_name) {
            let t0 = Instant::now();
            let dir = random_direction(&mut rng);

            match add_facet(start_polytope, &dir, FACET_EPSILON) {
                Some(f11_polytope) => {
                    let sys_after_add = compute_sys(&f11_polytope, &mut db);
                    match full_ascent(&f11_polytope, &mut rng, TRIAL_TIME_BUDGET_SECS, &mut db) {
                        Some(result) => {
                            let active = last_facet_active(&result.final_polytope);
                            let row = ResultRow {
                                rq: "rq2".into(),
                                path: "f10_add_then_f11".into(),
                                name: path_b_name.clone(),
                                source_name: seed_name.clone(),
                                lineage_id: format!("rq2::{seed_name}"),
                                direct_parent_trial: None,
                                starting_f: 11,
                                starting_sys: start_sys,
                                sys_after_addition: sys_after_add,
                                final_sys: result.final_sys,
                                delta_vs_source: result.final_sys - start_sys,
                                n_iterations: result.n_iters,
                                n_phases: result.n_phases,
                                placement_direction: Some([dir[0], dir[1], dir[2], dir[3]]),
                                facet_remained_active: Some(active),
                                total_time_ms: t0.elapsed().as_secs_f64() * 1000.0,
                                starting_dual_vertices_rational: dual_vertices_rational_strings(
                                    &f11_polytope,
                                ),
                                after_addition_dual_vertices_rational: Some(
                                    dual_vertices_rational_strings(&f11_polytope),
                                ),
                                final_dual_vertices_rational: dual_vertices_rational_strings(
                                    &result.final_polytope,
                                ),
                                final_dual_vertices: dvs_to_array(&result.final_polytope),
                            };
                            write_row(&row, &mut writer);
                            println!(
                                "  [B: add+F=11] add_sys={:.4} → final={:.4} (Δ={:+.4}), active={active}, {:.1}s",
                                sys_after_add.unwrap_or(f64::NAN),
                                result.final_sys,
                                result.final_sys - start_sys,
                                t0.elapsed().as_secs_f64(),
                            );
                        }
                        None => println!("  [B: add+F=11] gradient ascent FAILED"),
                    }
                }
                None => {
                    // Consume the rng state that would have been used by full_ascent
                    println!("  [B: add+F=11] facet addition FAILED");
                }
            }
        }

        // --- Path C: random F=11 gradient ascent ---
        let path_c_name = format!("{seed_name}_pathC_f11rand");
        if !completed.contains(&path_c_name) {
            if let Some((_, f11_polytope)) = rq2_f11_random.get(idx) {
                let t0 = Instant::now();
                let f11_start_sys = compute_sys(f11_polytope, &mut db);
                match full_ascent(f11_polytope, &mut rng, TRIAL_TIME_BUDGET_SECS, &mut db) {
                    Some(result) => {
                        let row = ResultRow {
                            rq: "rq2".into(),
                            path: "random_f11".into(),
                            name: path_c_name.clone(),
                            source_name: seed_name.clone(),
                            lineage_id: format!("rq2::{seed_name}"),
                            direct_parent_trial: None,
                            starting_f: 11,
                            starting_sys: f11_start_sys.unwrap_or(f64::NAN),
                            sys_after_addition: None,
                            final_sys: result.final_sys,
                            delta_vs_source: result.final_sys - f11_start_sys.unwrap_or(f64::NAN),
                            n_iterations: result.n_iters,
                            n_phases: result.n_phases,
                            placement_direction: None,
                            facet_remained_active: None,
                            total_time_ms: t0.elapsed().as_secs_f64() * 1000.0,
                            starting_dual_vertices_rational: dual_vertices_rational_strings(
                                f11_polytope,
                            ),
                            after_addition_dual_vertices_rational: None,
                            final_dual_vertices_rational: dual_vertices_rational_strings(
                                &result.final_polytope,
                            ),
                            final_dual_vertices: dvs_to_array(&result.final_polytope),
                        };
                        write_row(&row, &mut writer);
                        println!(
                            "  [C: random F=11] start={:.4} → final={:.4} (Δ={:+.4}), {:.1}s",
                            f11_start_sys.unwrap_or(f64::NAN),
                            result.final_sys,
                            result.final_sys - f11_start_sys.unwrap_or(f64::NAN),
                            t0.elapsed().as_secs_f64(),
                        );
                    }
                    None => println!("  [C: random F=11] FAILED"),
                }
            }
        }

        println!();
    }

    // =========================================================================
    // Final summary
    // =========================================================================

    writer.flush().expect("flush output");
    save(&cache_path, &db).expect("save cache");

    println!("========================================");
    println!(
        "Cache: {} entries (saved to {})",
        db.len(),
        cache_path.display()
    );
    println!("Total time: {:.1}s", t_global.elapsed().as_secs_f64());
    println!("Output: {}", output_path.display());
}

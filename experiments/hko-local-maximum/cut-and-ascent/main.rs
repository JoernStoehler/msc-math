//! Cut-and-ascent on HKO2024: add a facet (F=10→F=11), then run gradient ascent.
//!
//! Tests whether HKO2024 is a local maximum in the F=11 polytope space.
//! The facet-splitting experiment showed 536/536 cuts decrease sys, but
//! did not run gradient ascent afterward. This experiment closes that gap.
//!
//! Algorithm: for each random direction n on S³, add a barely-non-redundant
//! facet a_{F+1} = n / (h_K(n) - ε), then run gradient ascent with overshoot
//! and wiggle escape. Same ascent algorithm as gradient-ascent-general.
//!
//! Usage: cargo run -p exp-hko-local-maximum --release --bin hko-cut-and-ascent
//! Flags: --fresh  (clear existing data and rerun)
//! Input Artifacts: None (starts from the hardcoded HKO2024 polytope).
//! Output Artifacts: cut-and-ascent/cut-and-ascent.jsonl

use nalgebra::{Matrix4, Vector4};
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use rand_distr::{Distribution, StandardNormal};
use serde::Serialize;
use std::collections::HashSet;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::time::Instant;
use symplectic::derivatives::{capacity_derivatives_a_from_kkt_result, volume_derivatives_a};
use symplectic::geom::known_polytopes;
use symplectic::geom::polytope::Polytope4D;
use symplectic::geom::skeleton::Skeleton;
use symplectic::geom::symplectic_form::omega0;
use symplectic::geom::volume::volume;
use symplectic::kkt::saddle_point_solver::solve_kkt_for;

// ============================================================================
// Configuration
// ============================================================================

/// Reproducible RNG seed.
const SEED: u64 = 44;

/// Number of random facet placements to test.
/// 5 preliminary trials (2026-04-04) showed 0/5 improved. Increase to
/// 50-100 for statistical confidence.
const N_PLACEMENTS: usize = 20;
const SMOKE_N_PLACEMENTS: usize = 1;

/// Depth parameter for facet addition: a_{F+1} = n / (h_K(n) - ε).
/// 1e-3 used in facet-splitting experiment (SPLITTING_EPSILONS range
/// [1e-3, 1e-4]). If changed, verify Polytope4D construction doesn't
/// produce RedundantFacet errors at smaller ε.
const FACET_EPSILON: f64 = 1e-3;

// --- Gradient ascent parameters (copied from gradient-ascent-general) ---

/// Maximum gradient ascent iterations per phase.
const MAX_ITERATIONS: usize = 30;

/// Minimum improvement per iteration to continue. Well above f64 noise
/// (~1e-15) but small enough to capture meaningful steps.
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
/// Cell-widths data (research/combinatorial-cells/design/cell-widths.md): median non-orbit cell width = 0.124, median
/// orbit cell width = 0.258. So per-facet displacement ~0.05 is ~40% of the narrowest
/// median cell width. With F=11 facets all perturbed simultaneously, boundary crossing
/// is highly likely — confirmed by data: wiggle dominated overshoot as escape strategy
/// (research/sys-landscape/design/gradient-ascent-general.md, research/sys-landscape/design/gradient-ascent-products.md).
/// If changed: much smaller (e.g. 0.01) reduces boundary-crossing probability and escape
/// effectiveness. Much larger (e.g. 0.2) risks producing degenerate polytopes
/// (Polytope4D::from_f64 failure) or landing too far from the current optimum.
const WIGGLE_STRENGTH: f64 = 0.05;

/// Maximum rounds of escape attempts after convergence.
const MAX_ESCAPE_ROUNDS: usize = 3;

/// Per-trial time budget. 180s for F=11.
const TRIAL_TIME_BUDGET_SECS: f64 = 180.0;
const SMOKE_TRIAL_TIME_BUDGET_SECS: f64 = 8.0;

/// Numerical zero threshold for gradient norms, rates, and slack comparisons.
/// Near machine epsilon for unit-scale f64.
const EPS: f64 = 1e-15;

// ============================================================================
// Output schema
// ============================================================================

#[derive(Debug, Serialize)]
struct ResultRow {
    name: String,
    placement_direction: [f64; 4],
    epsilon: f64,
    hko_sys: f64,
    sys_after_cut: f64,
    final_sys: f64,
    delta_vs_hko: f64,
    n_iterations: usize,
    n_phases: usize,
    facet_remained_active: bool,
    total_time_ms: f64,
    final_dual_vertices: Vec<[f64; 4]>,
}

// ============================================================================
// Step bound in a-space (enriched, from exp-combinatorial-cells/cell-widths)
// ============================================================================

/// Compute the first boundary event along a direction in dual-vertex space.
/// [lem:step-bound-incidence] incidence flip detection, [lem:step-bound-omega] omega_0 flip detection
///
/// For step a'_k(t) = a_k + t*d_k, the combinatorial type changes when:
/// 1. **Incidence flip:** a vertex's slack w.r.t. a non-incident facet reaches zero.
/// 2. **omega_0 flip:** sign(omega_0(a_i, a_j)) changes for ridge-adjacent facets.
/// 3. **Dual vertex degeneration:** |a_k + t*d_k| -> 0.
///
/// Copied from exp-sys-landscape/src/lib.rs (cannot cross-crate import from
/// exp-sys-landscape). Cell-widths data shows omega_0 flips account for 30.5%
/// of boundary events in per-facet probes (research/combinatorial-cells/design/cell-widths.md).
fn compute_step_bound(polytope: &Polytope4D, direction: &[Vector4<f64>]) -> f64 {
    let duals = polytope.dual_vertices_f64();
    let vertices = polytope.vertices_f64();
    let f = polytope.facet_count();
    let skeleton = Skeleton::compute(polytope);

    let mut t_max = f64::INFINITY;

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
                if rate < -EPS {
                    let t_crit = slack / (-rate);
                    if t_crit > 0.0 && t_crit < t_max {
                        t_max = t_crit;
                    }
                }
            }
        } else {
            // Non-simple vertex (>4 incident facets). Conservative bound.
            let max_d = direction.iter().map(|dk| dk.norm()).fold(0.0f64, f64::max);
            for (j, a_j) in duals.iter().enumerate() {
                if vertex_facets.contains(&j) {
                    continue;
                }
                let slack = 1.0 - a_j.dot(v);
                let max_rate = max_d * v.norm() + a_j.norm() * max_d * v.norm();
                if max_rate > EPS {
                    let t_crit = slack / max_rate;
                    if t_crit > 0.0 && t_crit < t_max {
                        t_max = t_crit;
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

        let roots = if a_coeff.abs() > EPS {
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
        } else if b.abs() > EPS {
            vec![-c / b]
        } else {
            vec![]
        };

        for t_flip in roots {
            if t_flip > EPS && t_flip < t_max {
                t_max = t_flip;
            }
        }
    }

    // --- Dual vertex degeneration: |a_k + t*d_k| -> 0 ---
    for k in 0..f {
        let a_coeff = direction[k].norm_squared();
        let b = 2.0 * duals[k].dot(&direction[k]);
        let c = duals[k].norm_squared();
        let disc = b * b - 4.0 * a_coeff * c;
        if disc >= 0.0 && a_coeff > EPS {
            let sqrt_disc = disc.sqrt();
            for &sign in &[-1.0, 1.0] {
                let t_crit = (-b + sign * sqrt_disc) / (2.0 * a_coeff);
                if t_crit > EPS && t_crit < t_max {
                    t_max = t_crit;
                }
            }
        }
    }

    t_max.min(MAX_STEP_SIZE)
}

// ============================================================================
// Gradient ascent infrastructure (copied from gradient-ascent-general)
// ============================================================================

fn compute_sys(polytope: &Polytope4D) -> Option<f64> {
    let vol = volume(polytope);
    if vol <= 0.0 {
        return None;
    }
    let cap = compute_capacity(polytope)?;
    let sys = cap * cap / (2.0 * vol);
    sys.is_finite().then_some(sys)
}

fn try_step_a(
    duals: &[Vector4<f64>],
    direction: &[Vector4<f64>],
    t: f64,
) -> Option<(Polytope4D, f64)> {
    let new_duals: Vec<Vector4<f64>> = duals
        .iter()
        .zip(direction)
        .map(|(a, d)| a + t * d)
        .collect();
    let polytope = Polytope4D::from_f64(new_duals).ok()?;
    let sys = compute_sys(&polytope)?;
    Some((polytope, sys))
}

fn compute_capacity(polytope: &Polytope4D) -> Option<f64> {
    symplectic::ehz_capacity(polytope)
        .ok()
        .map(|r| r.capacity())
}

fn compute_capacity_result(polytope: &Polytope4D) -> Option<(f64, Vec<usize>)> {
    let r = symplectic::ehz_capacity(polytope).ok()?;
    Some((r.capacity(), r.best_sigma().to_vec()))
}

/// Single gradient ascent phase: iterate until convergence or budget.
/// Gradient: d(sys)/d(a_k) = (cap * d(cap)/d(a_k) - sys * d(vol)/d(a_k)) / vol
// TODO: add [lem:sys-sensitivity] to formal math (see gradient-correctness experiment)
fn gradient_ascent_phase(
    start: &Polytope4D,
    t0: Instant,
    budget: f64,
) -> Option<(Polytope4D, f64, usize)> {
    let mut current = Polytope4D::from_f64(start.dual_vertices_f64().to_vec()).ok()?;
    let mut current_sys = compute_sys(&current)?;
    let mut n_iters = 0usize;

    for iter in 0..MAX_ITERATIONS {
        if t0.elapsed().as_secs_f64() > budget {
            break;
        }

        let (cap, best_perm) = compute_capacity_result(&current)?;
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
            if let Some((p, new_sys)) = try_step_a(duals, &d_sys_a, t) {
                if new_sys > sys && best.as_ref().is_none_or(|b| new_sys > b.1) {
                    best = Some((p, new_sys));
                }
            }
        }

        if t_max < MAX_STEP_SIZE {
            for &mult in OVERSHOOT_MULTIPLIERS {
                let t = mult * t_max;
                if let Some((p, new_sys)) = try_step_a(duals, &d_sys_a, t) {
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

struct AscentResult {
    final_polytope: Polytope4D,
    final_sys: f64,
    n_iters: usize,
    n_phases: usize,
}

fn full_ascent(start: &Polytope4D, rng: &mut ChaCha8Rng, budget: f64) -> Option<AscentResult> {
    let t0 = Instant::now();

    let (mut best_polytope, mut best_sys, mut total_iters) =
        gradient_ascent_phase(start, t0, budget)?;
    let mut n_phases = 1usize;

    for _round in 0..MAX_ESCAPE_ROUNDS {
        if t0.elapsed().as_secs_f64() > budget {
            break;
        }
        let mut escaped = false;
        for _ in 0..N_WIGGLES {
            if t0.elapsed().as_secs_f64() > budget {
                break;
            }
            if let Some(wiggled) = wiggle(&best_polytope, rng) {
                if let Some((p, s, iters)) = gradient_ascent_phase(&wiggled, t0, budget) {
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
/// This creates an (F+1)-facet polytope that is close to the original.
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

fn random_direction(rng: &mut ChaCha8Rng) -> Vector4<f64> {
    let x: f64 = StandardNormal.sample(rng);
    let y: f64 = StandardNormal.sample(rng);
    let z: f64 = StandardNormal.sample(rng);
    let w: f64 = StandardNormal.sample(rng);
    let v = Vector4::new(x, y, z, w);
    v.normalize()
}

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

fn dvs_to_array(polytope: &Polytope4D) -> Vec<[f64; 4]> {
    polytope
        .dual_vertices_f64()
        .iter()
        .map(|a| [a[0], a[1], a[2], a[3]])
        .collect()
}

// ============================================================================
// Main
// ============================================================================

fn main() {
    let t_global = Instant::now();
    let base = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("cut-and-ascent");
    let args: Vec<String> = std::env::args().collect();
    let fresh = args.iter().any(|a| a == "--fresh");
    let smoke = args.iter().any(|a| a == "--smoke");
    let output_path = if smoke {
        base.join("cut-and-ascent-smoke.jsonl")
    } else {
        base.join("cut-and-ascent.jsonl")
    };

    println!("cut-and-ascent: facet addition + gradient ascent on HKO2024\n");

    std::fs::create_dir_all(&base).expect("create output dir");

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

    let output_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&output_path)
        .expect("open output JSONL");
    let mut writer = BufWriter::new(output_file);

    let mut rng = ChaCha8Rng::seed_from_u64(SEED);
    let n_placements = if smoke {
        SMOKE_N_PLACEMENTS
    } else {
        N_PLACEMENTS
    };
    let trial_budget = if smoke {
        SMOKE_TRIAL_TIME_BUDGET_SECS
    } else {
        TRIAL_TIME_BUDGET_SECS
    };

    // Load HKO2024
    let hko = known_polytopes::hko_pentagon();
    let hko_polytope = &hko.polytope;
    let hko_sys = compute_sys(hko_polytope).expect("HKO2024 sys");
    println!(
        "HKO2024: sys={hko_sys:.6}, F={}\n",
        hko_polytope.facet_count()
    );

    let mut n_improved = 0usize;
    let mut n_total = 0usize;

    for i in 0..n_placements {
        let trial_name = if smoke {
            "smoke".to_string()
        } else {
            format!("hko_p{i}")
        };
        if completed.contains(&trial_name) {
            continue;
        }

        let t0 = Instant::now();
        let dir = random_direction(&mut rng);

        let f11_polytope = match add_facet(hko_polytope, &dir, FACET_EPSILON) {
            Some(p) => p,
            None => {
                println!("[{trial_name}] facet addition failed");
                continue;
            }
        };

        let sys_after_cut = match compute_sys(&f11_polytope) {
            Some(s) => s,
            None => {
                println!("[{trial_name}] sys computation failed after cut");
                continue;
            }
        };

        match full_ascent(&f11_polytope, &mut rng, trial_budget) {
            Some(result) => {
                let delta = result.final_sys - hko_sys;
                n_total += 1;
                let improved = delta > CONVERGENCE_THRESHOLD;
                if improved {
                    n_improved += 1;
                }

                let active = last_facet_active(&result.final_polytope);

                let row = ResultRow {
                    name: trial_name.clone(),
                    placement_direction: [dir[0], dir[1], dir[2], dir[3]],
                    epsilon: FACET_EPSILON,
                    hko_sys,
                    sys_after_cut,
                    final_sys: result.final_sys,
                    delta_vs_hko: delta,
                    n_iterations: result.n_iters,
                    n_phases: result.n_phases,
                    facet_remained_active: active,
                    total_time_ms: t0.elapsed().as_secs_f64() * 1000.0,
                    final_dual_vertices: dvs_to_array(&result.final_polytope),
                };

                serde_json::to_writer(&mut writer, &row).expect("write row");
                writeln!(writer).expect("newline");

                let marker = if improved { " *** IMPROVED ***" } else { "" };
                println!(
                    "[{trial_name}] cut={sys_after_cut:.6} → final={:.6} (Δ={delta:+.6}), \
                     active={active}, {:.1}s{marker}",
                    result.final_sys,
                    t0.elapsed().as_secs_f64(),
                );

                if improved {
                    eprintln!(
                        "*** HKO2024 IMPROVEMENT: {} sys={:.6} > {:.6} ***",
                        trial_name, result.final_sys, hko_sys
                    );
                }
            }
            None => {
                println!("[{trial_name}] gradient ascent failed");
            }
        }
    }

    writer.flush().expect("flush output");

    println!("\n========================================");
    println!("Improved: {n_improved}/{n_total}");
    println!("Total time: {:.1}s", t_global.elapsed().as_secs_f64());
    println!("Output: {}", output_path.display());
}

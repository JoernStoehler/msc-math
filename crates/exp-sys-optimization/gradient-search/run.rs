//! Gradient search optimizer: reads seeds, optimizes sys, writes results.
//!
//! Reads polytopes from seeds.jsonl, runs gradient ascent + overshoot + wiggle
//! on each, writes results to results.jsonl. Skips seeds already in results.
//!
//! Usage: cargo run --bin gradient_search --release
//! Input: gradient-search/seeds.jsonl
//! Output: gradient-search/results.jsonl (append mode)

use nalgebra::{Matrix4, Vector4};
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use rand_distr::{Distribution, StandardNormal};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::time::Instant;
use symplectic::algorithms::hk2017::ehz_capacity;
use symplectic::derivatives::{capacity_derivatives_h, volume_derivatives_h};
use symplectic::geom::polytope::Polytope4D;
use symplectic::geom::skeleton::Skeleton;
use symplectic::geom::volume::volume;
use symplectic::kkt::saddle_point_solver::solve_kkt_for;

// ============================================================================
// Configuration
// ============================================================================

/// 30 iters is enough for convergence in >95% of cases (gradient-descent found
/// most converge in 5-15). Increasing adds runtime but rarely improves final sys.
const MAX_GRADIENT_ITERS: usize = 30;
/// Well above f64 noise (~1e-15) but small enough to capture meaningful steps.
/// At this threshold, convergence means <0.0001% change per iteration.
const CONVERGENCE_THRESHOLD: f64 = 1e-6;
/// Geometric-ish spacing from conservative (0.1) to aggressive (0.95). We pick
/// the fraction giving highest sys, so more fractions = better search at cost of
/// more capacity evaluations. 5 fractions is a good tradeoff.
const STEP_FRACTIONS: &[f64] = &[0.1, 0.25, 0.5, 0.75, 0.95];
/// Multipliers beyond t_max for crossing combinatorial boundaries. The step bound
/// t_max is where the combinatorial type first changes. Steps at 1.5-3x t_max
/// land in neighboring cells. Larger multipliers risk invalid polytopes (negative
/// heights) but explore further.
const OVERSHOOT_FRACTIONS: &[f64] = &[1.5, 2.0, 3.0];
/// Prevents pathological steps when t_max is huge (gradient nearly parallel to a
/// constraint). Heights are O(1), so steps beyond 100 are unreasonable.
const MAX_STEP_SIZE: f64 = 100.0;
/// Number of random height perturbations per escape round. 5 gives reasonable
/// coverage of nearby combinatorial cells without excessive cost.
const N_WIGGLES: usize = 5;
/// ~5% height perturbation. Small enough to stay near the current optimum,
/// large enough to cross combinatorial boundaries (typical vertex slack is O(0.1)).
const WIGGLE_STRENGTH: f64 = 0.05;
/// Max rounds of escape attempts (overshoot + wiggle). 3 rounds balances
/// exploration vs compute: each round tries 5 wiggles + gradient ascent.
const MAX_ESCAPE_ROUNDS: usize = 3;
/// Per-seed time budget prevents any single seed from hogging compute.
/// 120s is generous: most seeds converge in <10s, but F=10 with many escape
/// rounds can take longer.
const SEED_TIME_BUDGET_SECS: f64 = 120.0;
/// Numerical zero threshold for gradient directions and slack comparisons.
/// Well below f64 relative error (~1e-16) for unit-scale polytopes.
const EPS: f64 = 1e-15;
/// Hard floor on heights after wiggling: prevents near-degenerate polytopes
/// where qhull or the KKT solver would fail numerically.
const MIN_HEIGHT_AFTER_WIGGLE: f64 = 0.01;

// ============================================================================
// I/O schemas
// ============================================================================

#[derive(Debug, Deserialize)]
struct SeedRow {
    seed_id: u64,
    facet_count: usize,
    normals: Vec<[f64; 4]>,
    heights: Vec<f64>,
}

#[derive(Debug, Serialize)]
struct ResultRow {
    seed_id: u64,
    starting_f: usize,
    final_f: usize,
    starting_sys: f64,
    final_sys: f64,
    n_gradient_phases: usize,
    n_gradient_iters_total: usize,
    n_escape_moves: usize,
    best_normals: Vec<[f64; 4]>,
    best_heights: Vec<f64>,
    total_time_ms: f64,
}

// ============================================================================
// Helpers
// ============================================================================

/// Compute systolic ratio sys(K) = c_EHZ(K)² / (2 vol(K)).
/// Returns None if volume or capacity computation fails.
fn compute_sys(polytope: &Polytope4D) -> Option<f64> {
    let vol = volume(polytope).ok().filter(|&v| v > 0.0)?;
    let ehz = ehz_capacity(polytope)?;
    let cap = ehz.result.capacity;
    let sys = cap * cap / (2.0 * vol);
    sys.is_finite().then_some(sys)
}

/// Construct a Polytope4D from normals and heights via dual vertices a_i = n_i / h_i.
/// Returns None if the dual vertices don't form a valid polytope.
fn reconstruct(normals: &[Vector4<f64>], heights: &[f64]) -> Option<Polytope4D> {
    let dvs: Vec<Vector4<f64>> = normals
        .iter()
        .zip(heights.iter())
        .map(|(n, &h)| n / h)
        .collect();
    Polytope4D::from_f64(dvs).ok()
}

/// Parse normals from serialized [f64; 4] arrays into nalgebra Vector4.
/// Component order: (q1, q2, p1, p2) per crate coordinate convention.
fn parse_normals(raw: &[[f64; 4]]) -> Vec<Vector4<f64>> {
    raw.iter().map(|n| Vector4::new(n[0], n[1], n[2], n[3])).collect()
}

// ============================================================================
// Step bound (h-only, from gradient-descent experiment)
// ============================================================================

fn step_bound_h(polytope: &Polytope4D, direction: &[f64]) -> f64 {
    let normals = polytope.normals_f64();
    let heights = polytope.heights_f64();
    let vertices = polytope.vertices_f64();
    let f = polytope.facet_count();
    let skeleton = Skeleton::compute(polytope);
    let mut t_max = f64::INFINITY;

    for (vi, vf) in skeleton.vertex_facets.iter().enumerate() {
        let v = &vertices[vi];
        if vf.len() == 4 {
            let n_mat = Matrix4::from_rows(&[
                normals[vf[0]].transpose(),
                normals[vf[1]].transpose(),
                normals[vf[2]].transpose(),
                normals[vf[3]].transpose(),
            ]);
            if let Some(n_inv) = n_mat.try_inverse() {
                let g = Vector4::new(
                    direction[vf[0]], direction[vf[1]],
                    direction[vf[2]], direction[vf[3]],
                );
                let dv = n_inv * g;
                for j in 0..f {
                    if vf.contains(&j) { continue; }
                    let slack = heights[j] - normals[j].dot(v);
                    let rate = direction[j] - normals[j].dot(&dv);
                    if rate < -EPS {
                        let tc = slack / (-rate);
                        if tc > 0.0 && tc < t_max { t_max = tc; }
                    }
                }
            }
        } else {
            let max_g = direction.iter().map(|x| x.abs()).fold(0.0f64, f64::max);
            if max_g > EPS {
                for j in 0..f {
                    if vf.contains(&j) { continue; }
                    let slack = heights[j] - normals[j].dot(v);
                    let tc = slack / max_g;
                    if tc > 0.0 && tc < t_max { t_max = tc; }
                }
            }
        }
    }

    for k in 0..f {
        if direction[k] < -EPS {
            let tc = heights[k] / (-direction[k]);
            if tc > 0.0 && tc < t_max { t_max = tc; }
        }
    }

    t_max.min(MAX_STEP_SIZE)
}

// ============================================================================
// Gradient ascent (h-only) + overshoot
// ============================================================================

fn gradient_ascent(
    normals: &[Vector4<f64>],
    heights: &[f64],
    t0: Instant,
    budget: f64,
) -> Option<(Vec<Vector4<f64>>, Vec<f64>, f64, usize)> {
    let f = normals.len();
    let mut cur_h = heights.to_vec();
    let mut best_h = heights.to_vec();

    // Initial eval
    let polytope = reconstruct(normals, &cur_h)?;
    let ehz = ehz_capacity(&polytope)?;
    let mut cap = ehz.result.capacity;
    let mut vol = volume(&polytope).ok().filter(|&v| v > 0.0)?;
    let mut sys = cap * cap / (2.0 * vol);
    let mut perm = ehz.result.best_permutation;
    let mut best_sys = sys;
    let mut n_iters = 0;

    for _iter in 0..MAX_GRADIENT_ITERS {
        if t0.elapsed().as_secs_f64() > budget { break; }

        let current = reconstruct(normals, &cur_h)?;
        let kkt = solve_kkt_for(&current, &perm)?;

        // d(sys)/dh_k by quotient rule on sys = cap² / (2 vol):
        //   d(sys)/dh_k = (2 cap · dcap/dh_k · 2vol - cap² · 2 · dvol/dh_k) / (2vol)²
        //               = (cap · dcap/dh_k - sys · dvol/dh_k) / vol
        // where dcap/dh_k from envelope theorem, dvol/dh_k = facet volume S_k.
        // See crates/src/derivatives.rs for the component derivations.
        let dc = capacity_derivatives_h(&kkt.beta, kkt.q_corrected, kkt.xi, &perm, f);
        let dv = volume_derivatives_h(&current);
        let d_sys: Vec<f64> = dc.iter().zip(dv.iter())
            .map(|(&dc, &dv)| (cap * dc - sys * dv) / vol)
            .collect();

        let gn = d_sys.iter().map(|x| x * x).sum::<f64>().sqrt();
        if gn < EPS { break; }

        let t_max = step_bound_h(&current, &d_sys);
        if t_max <= 0.0 { break; }

        // Line search within step bound
        let mut best_step: Option<(Vec<f64>, f64, f64, f64, Vec<usize>)> = None;
        for &frac in STEP_FRACTIONS {
            let t = frac * t_max;
            let nh: Vec<f64> = (0..f).map(|k| cur_h[k] + t * d_sys[k]).collect();
            if nh.iter().any(|&h| h <= 0.0) { continue; }
            if let Some(p) = reconstruct(normals, &nh) {
                if let Some(e) = ehz_capacity(&p) {
                    if let Ok(v) = volume(&p) {
                        if v > 0.0 {
                            let c = e.result.capacity;
                            let s = c * c / (2.0 * v);
                            if s.is_finite() && s > sys
                                && best_step.as_ref().is_none_or(|b| s > b.1)
                            {
                                best_step = Some((nh, s, c, v, e.result.best_permutation));
                            }
                        }
                    }
                }
            }
        }

        // Also try overshooting to cross combinatorial boundaries
        for &frac in OVERSHOOT_FRACTIONS {
            let t = frac * t_max;
            let nh: Vec<f64> = (0..f).map(|k| cur_h[k] + t * d_sys[k]).collect();
            if nh.iter().any(|&h| h <= 0.0) { continue; }
            if let Some(p) = reconstruct(normals, &nh) {
                if let Some(e) = ehz_capacity(&p) {
                    if let Ok(v) = volume(&p) {
                        if v > 0.0 {
                            let c = e.result.capacity;
                            let s = c * c / (2.0 * v);
                            if s.is_finite() && s > sys
                                && best_step.as_ref().is_none_or(|b| s > b.1)
                            {
                                best_step = Some((nh, s, c, v, e.result.best_permutation));
                            }
                        }
                    }
                }
            }
        }

        match best_step {
            Some((nh, new_sys, new_cap, new_vol, new_perm)) => {
                let delta = new_sys - sys;
                cur_h = nh;
                sys = new_sys;
                cap = new_cap;
                vol = new_vol;
                perm = new_perm;
                n_iters += 1;
                if sys > best_sys {
                    best_sys = sys;
                    best_h = cur_h.clone();
                }
                if delta < CONVERGENCE_THRESHOLD { break; }
            }
            None => break,
        }
    }

    Some((normals.to_vec(), best_h, best_sys, n_iters))
}

// ============================================================================
// Escape moves
// ============================================================================

fn wiggle(
    normals: &[Vector4<f64>],
    heights: &[f64],
    rng: &mut ChaCha8Rng,
) -> Option<(Vec<Vector4<f64>>, Vec<f64>)> {
    let new_h: Vec<f64> = heights
        .iter()
        .map(|&h| {
            let noise: f64 = StandardNormal.sample(rng);
            (h * (1.0 + WIGGLE_STRENGTH * noise)).max(MIN_HEIGHT_AFTER_WIGGLE)
        })
        .collect();
    reconstruct(normals, &new_h)?;
    Some((normals.to_vec(), new_h))
}

// ============================================================================
// Per-seed processing
// ============================================================================

fn process_seed(seed: &SeedRow) -> Option<ResultRow> {
    let t0 = Instant::now();
    let budget = SEED_TIME_BUDGET_SECS;
    let mut rng = ChaCha8Rng::seed_from_u64(seed.seed_id);

    let normals = parse_normals(&seed.normals);
    let heights = seed.heights.clone();

    let starting_sys = {
        let p = reconstruct(&normals, &heights)?;
        compute_sys(&p)?
    };

    let mut best_normals = normals.clone();
    let mut best_heights = heights.clone();
    let mut best_sys = starting_sys;
    let mut cur_normals = normals;
    let mut cur_heights = heights;
    let mut n_phases = 0usize;
    let mut n_iters_total = 0usize;
    let mut n_escapes = 0usize;

    // Initial gradient ascent
    if let Some((n, h, s, it)) = gradient_ascent(&cur_normals, &cur_heights, t0, budget) {
        n_phases += 1;
        n_iters_total += it;
        cur_normals = n;
        cur_heights = h;
        if s > best_sys {
            best_sys = s;
            best_normals = cur_normals.clone();
            best_heights = cur_heights.clone();
        }
    }

    // Escape rounds
    for _round in 0..MAX_ESCAPE_ROUNDS {
        if t0.elapsed().as_secs_f64() > budget {
            break;
        }
        let mut escaped = false;

        for _ in 0..N_WIGGLES {
            if t0.elapsed().as_secs_f64() > budget {
                break;
            }
            if let Some((wn, wh)) = wiggle(&cur_normals, &cur_heights, &mut rng) {
                if let Some((n, h, s, it)) = gradient_ascent(&wn, &wh, t0, budget) {
                    n_phases += 1;
                    n_iters_total += it;
                    if s > best_sys + CONVERGENCE_THRESHOLD {
                        cur_normals = n;
                        cur_heights = h;
                        best_sys = s;
                        best_normals = cur_normals.clone();
                        best_heights = cur_heights.clone();
                        n_escapes += 1;
                        escaped = true;
                        break;
                    }
                }
            }
        }
        if escaped {
            continue;
        }
        break;
    }

    Some(ResultRow {
        seed_id: seed.seed_id,
        starting_f: seed.facet_count,
        final_f: best_normals.len(),
        starting_sys,
        final_sys: best_sys,
        n_gradient_phases: n_phases,
        n_gradient_iters_total: n_iters_total,
        n_escape_moves: n_escapes,
        best_normals: best_normals.iter().map(|n| [n[0], n[1], n[2], n[3]]).collect(),
        best_heights,
        total_time_ms: t0.elapsed().as_secs_f64() * 1000.0,
    })
}

// ============================================================================
// Main
// ============================================================================

fn main() {
    let t_global = Instant::now();
    let base = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("gradient-search");

    // Read seeds
    let seeds_path = base.join("seeds.jsonl");
    let seeds: Vec<SeedRow> = {
        let f = std::fs::File::open(&seeds_path)
            .unwrap_or_else(|e| panic!("cannot open {}: {e}\nRun generate_seeds first.", seeds_path.display()));
        BufReader::new(f)
            .lines()
            .filter_map(|l| l.ok())
            .filter_map(|l| serde_json::from_str(&l).ok())
            .collect()
    };
    eprintln!("Loaded {} seeds from {}", seeds.len(), seeds_path.display());

    // Read already-completed seed_ids
    let results_path = base.join("results.jsonl");
    let done: HashSet<u64> = if results_path.exists() {
        let f = std::fs::File::open(&results_path).unwrap();
        BufReader::new(f)
            .lines()
            .filter_map(|l| l.ok())
            .filter_map(|l| {
                serde_json::from_str::<serde_json::Value>(&l)
                    .ok()?
                    .get("seed_id")?
                    .as_u64()
            })
            .collect()
    } else {
        HashSet::new()
    };

    let todo: Vec<&SeedRow> = seeds.iter().filter(|s| !done.contains(&s.seed_id)).collect();
    eprintln!("{} already done, {} to process", done.len(), todo.len());

    if todo.is_empty() {
        eprintln!("Nothing to do.");
        return;
    }

    // Process
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&results_path)
        .unwrap_or_else(|e| panic!("cannot open {}: {e}", results_path.display()));
    let mut writer = BufWriter::new(file);

    let total = todo.len();
    let mut processed = 0usize;
    // Tracks the best sys seen in this run only (not previously-completed seeds).
    let mut best_global = 0.0f64;

    for seed in &todo {
        let row = process_seed(seed);
        processed += 1;

        if let Some(ref r) = row {
            writeln!(writer, "{}", serde_json::to_string(r).unwrap()).unwrap();
            writer.flush().unwrap();

            if r.final_sys > best_global {
                best_global = r.final_sys;
            }

            if processed % 100 == 0 || r.final_sys > 0.9 {
                eprintln!(
                    "[{:>5}/{} {:>5.0}s] seed={} F={} sys={:.4}>{:.4} best={:.4}",
                    processed, total, t_global.elapsed().as_secs_f64(),
                    seed.seed_id, seed.facet_count,
                    r.starting_sys, r.final_sys, best_global,
                );
            }
            if r.final_sys > 1.0 {
                eprintln!(
                    "*** VITERBO VIOLATION: seed={} sys={:.6} F={} ***",
                    seed.seed_id, r.final_sys, seed.facet_count,
                );
            }
        }
    }

    eprintln!(
        "Done: {}/{} in {:.1}s, best_sys={:.6}",
        processed, total, t_global.elapsed().as_secs_f64(), best_global,
    );
}

//! Gradient correctness: validate analytical ∂c/∂a_k, ∂vol/∂a_k, ∂sys/∂a_k against
//! finite differences under 4 progressively adversarial conditions.
//!
//! Q1: Generic random polytopes — FD step-size sweep, dimension scaling
//! Q2: Non-generic geometry — Lagrangian products with symmetry-degenerate orbits
//! Q3: Near-degeneracy — small action gap between best and second-best orbit
//! Q4: Barely-cutting facets — near-redundant halfspaces
//!
//! Architecture:
//! 1. `cargo run --release --bin gradient_correctness` generates 4 JSONL files
//! 2. Python script reads JSONL, produces figures and summary table
//!
//! Self-contained: generates all polytopes internally (no dependency on other datasets).

use std::panic;
use nalgebra::Vector4;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use rand_distr::{Distribution, StandardNormal, Uniform};
use serde::Serialize;
use std::f64::consts::PI;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::time::Instant;
use symplectic::algorithms::hk2017::combinations;
use symplectic::algorithms::hk2017::permutations::for_each_cyclic_permutation;
use symplectic::derivatives::{
    capacity_derivatives_a, capacity_derivatives_a_fd, volume_derivatives_a, volume_derivatives_a_fd,
};
use symplectic::geom::polygon::random_polygon_2d;
use symplectic::kkt::saddle_point_solver::{solve_kkt_for, KktResult, EPS_Q_POSITIVE};
use symplectic::{ehz_capacity, lagrangian_product, regular_polygon_2d, rotate_polygon_2d};
use symplectic::{volume, Polytope4D};
use symplectic::geom::facet_volume::facet_volume_3d;
use symplectic::random::generate_random_polytopes;

// ============================================================================
// Constants
// ============================================================================

/// Base seed for deterministic RNG across all phases.
const SEED_BASE: u64 = 7777;

/// FD epsilon sweep for Q1 (13 values spanning truncation ↔ roundoff tradeoff).
const FD_EPSILONS: &[f64] = &[
    1e-2, 3e-3, 1e-3, 3e-4, 1e-4, 3e-5, 1e-5, 3e-6, 1e-6, 3e-7, 1e-7, 3e-8, 1e-8,
];

/// Single sweet-spot epsilon for Q2/Q3/Q4 (structural tests, not step-size tests).
const FD_SWEET_SPOT: f64 = 1e-5;

/// Q1: polytopes per facet count.
const Q1_POLYTOPES_PER_F: usize = 20;

/// Q3: max candidates to generate when filling gap bins.
const Q3_MAX_CANDIDATES: usize = 2000;

/// Q3: max polytopes per gap bin.
const Q3_PER_BIN: usize = 20;

/// Q4: base polytopes to augment.
const Q4_BASE_COUNT: usize = 10;

/// Q4: barely-cutting delta values.
const Q4_DELTAS: &[f64] = &[1e-1, 1e-2, 1e-3, 1e-4, 1e-5];

/// Floor for relative error computation (avoids 0/0).
const REL_ERROR_FLOOR: f64 = 1e-12;

/// Minimum beta for certified orbit in Q3 enumeration.
const EPS_BETA_CERTIFIED: f64 = 1e-9;

// ============================================================================
// Output schema
// ============================================================================

#[derive(Debug, Serialize)]
struct GradientRow {
    polytope_id: String,
    facet_count: usize,
    polytope_class: String,

    capacity: f64,
    volume: f64,
    sys: f64,
    orbit_length: usize,

    target: String,
    fd_epsilon: f64,

    analytical: Vec<[f64; 4]>,
    fd: Vec<[f64; 4]>,

    max_rel_error: f64,
    mean_rel_error: f64,
    max_abs_error: f64,
    cosine_sim_min: f64,

    // Q3-specific
    action_gap: Option<f64>,
    second_best_action: Option<f64>,
    orbit_switched_in_fd: Option<bool>,

    // Q4-specific
    barely_cutting_delta: Option<f64>,
    min_facet_volume: Option<f64>,

    time_ms: f64,
}

// ============================================================================
// Helper functions
// ============================================================================

/// Convert Vec<Vector4<f64>> to Vec<[f64; 4]> for serialization.
fn to_raw(vecs: &[Vector4<f64>]) -> Vec<[f64; 4]> {
    vecs.iter().map(|v| [v[0], v[1], v[2], v[3]]).collect()
}

/// Compute ∂sys/∂a_k analytically via quotient rule.
/// sys = c² / (2·vol), so ∂sys/∂a_k = (c · ∂c/∂a_k − sys · ∂vol/∂a_k) / vol.
fn sys_derivatives_a(
    d_cap: &[Vector4<f64>],
    d_vol: &[Vector4<f64>],
    cap: f64,
    vol: f64,
    sys: f64,
) -> Vec<Vector4<f64>> {
    d_vol.iter()
        .zip(d_cap.iter())
        .map(|(dv, dc)| (cap * dc - sys * dv) / vol)
        .collect()
}

/// Compute ∂sys/∂a_k by central finite differences.
/// Perturbs each a_k[d] by ±eps, recomputes sys = c²/(2·vol), central difference.
fn sys_derivatives_a_fd(dual_vertices: &[Vector4<f64>], eps: f64) -> Vec<Vector4<f64>> {
    let f = dual_vertices.len();
    (0..f)
        .map(|k| {
            let mut grad = Vector4::zeros();
            for d in 0..4 {
                let mut a_plus = dual_vertices.to_vec();
                let mut a_minus = dual_vertices.to_vec();
                a_plus[k][d] += eps;
                a_minus[k][d] -= eps;

                let sys_plus = compute_sys_from_duals(&a_plus);
                let sys_minus = compute_sys_from_duals(&a_minus);

                grad[d] = match (sys_plus, sys_minus) {
                    (Some(sp), Some(sm)) => (sp - sm) / (2.0 * eps),
                    _ => f64::NAN,
                };
            }
            grad
        })
        .collect()
}

/// Compute sys = c²/(2·vol) from dual vertices, returning None if construction or capacity fails.
fn compute_sys_from_duals(duals: &[Vector4<f64>]) -> Option<f64> {
    let p = Polytope4D::from_f64(duals.to_vec()).ok()?;
    let cap = ehz_capacity_safe(&p)?.result.capacity;
    let vol = volume(&p).ok()?;
    if vol <= 0.0 { return None; }
    Some(cap * cap / (2.0 * vol))
}

/// Compute error metrics between analytical and FD gradient vectors.
/// Returns (max_rel_error, mean_rel_error, max_abs_error, cosine_sim_min).
fn compute_error_metrics(analytical: &[Vector4<f64>], fd: &[Vector4<f64>]) -> (f64, f64, f64, f64) {
    let mut max_rel = 0.0_f64;
    let mut sum_rel = 0.0_f64;
    let mut count_rel = 0usize;
    let mut max_abs = 0.0_f64;
    let mut cos_min = 1.0_f64;

    for (a, f) in analytical.iter().zip(fd.iter()) {
        // Skip if FD has NaN
        if f.iter().any(|x| x.is_nan()) {
            continue;
        }

        let a_norm = a.norm();
        let f_norm = f.norm();
        let diff = a - f;
        let abs_err = diff.norm();

        if abs_err > max_abs {
            max_abs = abs_err;
        }

        let denom = a_norm.max(f_norm).max(REL_ERROR_FLOOR);
        let rel_err = abs_err / denom;
        if rel_err > max_rel {
            max_rel = rel_err;
        }

        // Only include in mean if the gradient is non-negligible
        if denom > REL_ERROR_FLOOR {
            sum_rel += rel_err;
            count_rel += 1;
        }

        // Cosine similarity (only if both nonzero)
        if a_norm > REL_ERROR_FLOOR && f_norm > REL_ERROR_FLOOR {
            let cos = a.dot(f) / (a_norm * f_norm);
            if cos < cos_min {
                cos_min = cos;
            }
        }
    }

    let mean_rel = if count_rel > 0 { sum_rel / count_rel as f64 } else { 0.0 };
    (max_rel, mean_rel, max_abs, cos_min)
}

/// Sample a random unit vector on S³ (Muller's method).
/// Copied from crates/src/random.rs (private there).
fn random_unit_s3(rng: &mut ChaCha8Rng) -> Vector4<f64> {
    loop {
        let x: f64 = StandardNormal.sample(rng);
        let y: f64 = StandardNormal.sample(rng);
        let z: f64 = StandardNormal.sample(rng);
        let w: f64 = StandardNormal.sample(rng);
        let v = Vector4::new(x, y, z, w);
        let norm = v.norm();
        if norm > 1e-10 {
            return v / norm;
        }
    }
}

/// Safe wrapper around ehz_capacity that catches panics (e.g. Q-correction panic
/// on near-degenerate polytopes). Returns None on panic instead of crashing.
fn ehz_capacity_safe(polytope: &Polytope4D) -> Option<symplectic::EhzResult> {
    let polytope = polytope.clone();
    panic::catch_unwind(panic::AssertUnwindSafe(|| ehz_capacity(&polytope)))
        .ok()
        .flatten()
}

/// Information about a polytope needed for gradient validation.
struct PolytopeInfo {
    polytope: Polytope4D,
    cap: f64,
    vol: f64,
    sys: f64,
    best_perm: Vec<usize>,
    kkt: KktResult,
}

/// Compute capacity, volume, sys, best orbit, and KKT for a polytope.
/// Returns None if capacity computation fails.
fn analyze_polytope(polytope: &Polytope4D) -> Option<PolytopeInfo> {
    let ehz = ehz_capacity_safe(polytope)?;
    let cap = ehz.result.capacity;
    let vol = volume(polytope).ok()?;
    if vol <= 0.0 { return None; }
    let sys = cap * cap / (2.0 * vol);
    let best_perm = ehz.result.best_permutation.clone();
    let kkt = solve_kkt_for(polytope, &best_perm)?;
    Some(PolytopeInfo { polytope: polytope.clone(), cap, vol, sys, best_perm, kkt })
}

/// Validate all three derivative targets for a polytope at a given FD epsilon.
/// Returns 3 GradientRows (capacity, volume, sys).
fn validate_derivatives(
    info: &PolytopeInfo,
    eps: f64,
    polytope_id: &str,
    polytope_class: &str,
    // Q3-specific fields
    action_gap: Option<f64>,
    second_best_action: Option<f64>,
    orbit_switched_in_fd: Option<bool>,
    // Q4-specific fields
    barely_cutting_delta: Option<f64>,
    min_facet_volume: Option<f64>,
) -> Vec<GradientRow> {
    let duals = info.polytope.dual_vertices_f64();
    let f = duals.len();

    let mut rows = Vec::with_capacity(3);

    // --- Capacity ---
    let t0 = Instant::now();
    let d_cap_analytical = capacity_derivatives_a(
        &info.kkt.beta, info.kkt.q_corrected, &info.kkt.mu, &info.best_perm, &duals,
    );
    let d_cap_fd = capacity_derivatives_a_fd(&duals, eps, |perturbed| {
        let p = Polytope4D::from_f64(perturbed.to_vec()).ok()?;
        Some(ehz_capacity_safe(&p)?.result.capacity)
    });
    let (mr, mnr, ma, cs) = compute_error_metrics(&d_cap_analytical, &d_cap_fd);
    rows.push(GradientRow {
        polytope_id: polytope_id.to_string(),
        facet_count: f,
        polytope_class: polytope_class.to_string(),
        capacity: info.cap,
        volume: info.vol,
        sys: info.sys,
        orbit_length: info.best_perm.len(),
        target: "capacity".to_string(),
        fd_epsilon: eps,
        analytical: to_raw(&d_cap_analytical),
        fd: to_raw(&d_cap_fd),
        max_rel_error: mr,
        mean_rel_error: mnr,
        max_abs_error: ma,
        cosine_sim_min: cs,
        action_gap,
        second_best_action,
        orbit_switched_in_fd,
        barely_cutting_delta,
        min_facet_volume,
        time_ms: t0.elapsed().as_secs_f64() * 1000.0,
    });

    // --- Volume ---
    let t0 = Instant::now();
    let d_vol_analytical = volume_derivatives_a(&info.polytope);
    let d_vol_fd = volume_derivatives_a_fd(&duals, eps, |perturbed| {
        let p = Polytope4D::from_f64(perturbed.to_vec()).ok()?;
        volume(&p).ok()
    });
    let (mr, mnr, ma, cs) = compute_error_metrics(&d_vol_analytical, &d_vol_fd);
    rows.push(GradientRow {
        polytope_id: polytope_id.to_string(),
        facet_count: f,
        polytope_class: polytope_class.to_string(),
        capacity: info.cap,
        volume: info.vol,
        sys: info.sys,
        orbit_length: info.best_perm.len(),
        target: "volume".to_string(),
        fd_epsilon: eps,
        analytical: to_raw(&d_vol_analytical),
        fd: to_raw(&d_vol_fd),
        max_rel_error: mr,
        mean_rel_error: mnr,
        max_abs_error: ma,
        cosine_sim_min: cs,
        action_gap,
        second_best_action,
        orbit_switched_in_fd: None, // volume doesn't depend on orbit choice
        barely_cutting_delta,
        min_facet_volume,
        time_ms: t0.elapsed().as_secs_f64() * 1000.0,
    });

    // --- Sys ---
    let t0 = Instant::now();
    let d_sys_analytical = sys_derivatives_a(&d_cap_analytical, &d_vol_analytical, info.cap, info.vol, info.sys);
    let d_sys_fd = sys_derivatives_a_fd(&duals, eps);
    let (mr, mnr, ma, cs) = compute_error_metrics(&d_sys_analytical, &d_sys_fd);
    rows.push(GradientRow {
        polytope_id: polytope_id.to_string(),
        facet_count: f,
        polytope_class: polytope_class.to_string(),
        capacity: info.cap,
        volume: info.vol,
        sys: info.sys,
        orbit_length: info.best_perm.len(),
        target: "sys".to_string(),
        fd_epsilon: eps,
        analytical: to_raw(&d_sys_analytical),
        fd: to_raw(&d_sys_fd),
        max_rel_error: mr,
        mean_rel_error: mnr,
        max_abs_error: ma,
        cosine_sim_min: cs,
        action_gap,
        second_best_action,
        orbit_switched_in_fd,
        barely_cutting_delta,
        min_facet_volume,
        time_ms: t0.elapsed().as_secs_f64() * 1000.0,
    });

    rows
}

/// Write a batch of rows to a JSONL writer.
fn write_rows(writer: &mut BufWriter<File>, rows: &[GradientRow]) {
    for row in rows {
        let json = serde_json::to_string(row).expect("serialize row");
        writeln!(writer, "{}", json).expect("write row");
    }
}

/// Enumerate all certified orbits for a polytope.
/// Returns (action, permutation, kkt_result) sorted by action ascending.
fn enumerate_all_orbits(polytope: &Polytope4D) -> Vec<(f64, Vec<usize>, KktResult)> {
    let f = polytope.facet_count();
    let mut orbits = Vec::new();

    for m in 2..=f {
        for subset in combinations(f, m) {
            for_each_cyclic_permutation(&subset, &mut |perm| {
                if let Some(kkt) = solve_kkt_for(polytope, perm) {
                    let min_beta = kkt.beta.iter().copied().fold(f64::INFINITY, f64::min);
                    if min_beta > EPS_BETA_CERTIFIED && kkt.q_corrected > EPS_Q_POSITIVE {
                        let action = 0.5 / kkt.q_corrected;
                        orbits.push((action, perm.to_vec(), kkt));
                    }
                }
            });
        }
    }

    orbits.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    orbits
}

/// Check if any FD perturbation of dual vertices causes orbit switching.
/// Perturbs each a_k[d] by ±eps and checks if ehz_capacity returns a different best_permutation.
fn check_orbit_switching(duals: &[Vector4<f64>], eps: f64, original_perm: &[usize]) -> bool {
    for k in 0..duals.len() {
        for d in 0..4 {
            for sign in &[1.0, -1.0] {
                let mut perturbed = duals.to_vec();
                perturbed[k][d] += sign * eps;
                if let Ok(p) = Polytope4D::from_f64(perturbed) {
                    if let Some(ehz) = ehz_capacity_safe(&p) {
                        if ehz.result.best_permutation != original_perm {
                            return true;
                        }
                    }
                }
            }
        }
    }
    false
}

/// Add a barely-cutting facet to a polytope near a given vertex.
/// Returns None if construction fails after several attempts.
fn add_barely_cutting_facet(
    polytope: &Polytope4D,
    delta: f64,
    rng: &mut ChaCha8Rng,
) -> Option<Polytope4D> {
    let vertices = polytope.vertices_f64();
    let duals = polytope.dual_vertices_f64();

    // Try multiple vertex/direction combinations
    for _ in 0..50 {
        // Pick a random vertex
        let idx = Uniform::from(0..vertices.len()).sample(rng);
        let v = &vertices[idx];

        // Random unit direction for the new halfspace normal
        let n = random_unit_s3(rng);

        // Height: h = n·v − δ, so the hyperplane passes δ inside vertex v
        let h = n.dot(v) - delta;
        if h <= 0.0 {
            continue; // Origin would be outside, skip
        }

        let a_new = n / h;

        let mut new_duals: Vec<Vector4<f64>> = duals.to_vec();
        new_duals.push(a_new);

        if let Ok(p) = Polytope4D::from_f64(new_duals) {
            return Some(p);
        }
    }

    None
}

// ============================================================================
// Phase Q1: Generic random polytopes
// ============================================================================

fn run_q1(base_dir: &str) {
    let path = format!("{}/gradient-correctness-q1-generic.jsonl", base_dir);
    let file = File::create(&path).expect("create Q1 JSONL");
    let mut writer = BufWriter::new(file);

    let facet_counts = [5, 6, 7, 8, 9, 10];
    let mut total_rows = 0;

    for &f_count in &facet_counts {
        let mut rng = ChaCha8Rng::seed_from_u64(SEED_BASE + f_count as u64);
        let polytopes = generate_random_polytopes(Q1_POLYTOPES_PER_F, f_count, 0.5, 2.0, &mut rng);

        for (i, polytope) in polytopes.iter().enumerate() {
            let info = match analyze_polytope(polytope) {
                Some(info) => info,
                None => {
                    eprintln!("  Q1: F={} polytope {} — capacity or KKT failed, skipping", f_count, i);
                    continue;
                }
            };

            for &eps in FD_EPSILONS {
                let id = format!("generic_F{}_{:03}", f_count, i);
                let rows = validate_derivatives(
                    &info, eps, &id, "random",
                    None, None, None, None, None,
                );
                write_rows(&mut writer, &rows);
                total_rows += rows.len();
            }

            if (i + 1) % 5 == 0 {
                println!("  Q1: F={} — {}/{} polytopes done", f_count, i + 1, Q1_POLYTOPES_PER_F);
            }
        }
    }

    writer.flush().expect("flush Q1");
    println!("Q1 done: {} rows written to {}", total_rows, path);
}

// ============================================================================
// Phase Q2: Non-generic geometry (Lagrangian products)
// ============================================================================

fn run_q2(base_dir: &str) {
    let path = format!("{}/gradient-correctness-q2-nongeneric.jsonl", base_dir);
    let file = File::create(&path).expect("create Q2 JSONL");
    let mut writer = BufWriter::new(file);
    let mut total_rows = 0;

    let regular_pairs = [(3, 3), (3, 4), (4, 4), (3, 5), (4, 5), (5, 5)];
    let rotation_angles = [PI / 7.0, PI / 5.0, PI / 3.0];
    let random_pairs = [(3, 3), (3, 4), (4, 4), (5, 5)];
    let random_per_pair = 5;
    // Skip polytopes with F > 8 to avoid F=10 bottleneck (LP(5,5)=F10, LP(4,5)=F9).
    // Q1 already covers F=9-10 generic polytopes; Q2 focuses on structural questions.
    let max_facet_q2: usize = 8;

    // Regular Lagrangian products
    for &(n1, n2) in &regular_pairs {
        if n1 + n2 > max_facet_q2 {
            println!("  Q2: skipping LP({},{}) — F={} > {}", n1, n2, n1 + n2, max_facet_q2);
            continue;
        }
        let (qn, qh) = regular_polygon_2d(n1, 1.0);
        let (pn, ph) = regular_polygon_2d(n2, 1.0);
        let polytope = lagrangian_product(&qn, &qh, &pn, &ph).expect("regular LP");
        let id = format!("lp_regular_{}_{}", n1, n2);

        if let Some(info) = analyze_polytope(&polytope) {
            let rows = validate_derivatives(
                &info, FD_SWEET_SPOT, &id, "lagrangian_regular",
                None, None, None, None, None,
            );
            write_rows(&mut writer, &rows);
            total_rows += rows.len();
        } else {
            eprintln!("  Q2: regular LP({},{}) — failed", n1, n2);
        }
    }

    // Rotated Lagrangian products
    for &(n1, n2) in &regular_pairs {
        if n1 + n2 > max_facet_q2 {
            continue;
        }
        let (qn, qh) = regular_polygon_2d(n1, 1.0);
        for (ai, &theta) in rotation_angles.iter().enumerate() {
            let (pn, ph) = regular_polygon_2d(n2, 1.0);
            let (pn_rot, ph_rot) = rotate_polygon_2d(&pn, &ph, theta);
            let polytope = lagrangian_product(&qn, &qh, &pn_rot, &ph_rot).expect("rotated LP");
            let id = format!("lp_rotated_{}_{}_{}", n1, n2, ai);

            if let Some(info) = analyze_polytope(&polytope) {
                let rows = validate_derivatives(
                    &info, FD_SWEET_SPOT, &id, "lagrangian_rotated",
                    None, None, None, None, None,
                );
                write_rows(&mut writer, &rows);
                total_rows += rows.len();
            } else {
                eprintln!("  Q2: rotated LP({},{},θ={:.3}) — failed", n1, n2, theta);
            }
        }
    }

    // Random Lagrangian products
    let mut rng = ChaCha8Rng::seed_from_u64(SEED_BASE + 100);
    for &(n1, n2) in &random_pairs {
        if n1 + n2 > max_facet_q2 {
            continue;
        }
        for j in 0..random_per_pair {
            let (qn, qh) = random_polygon_2d(n1, 0.5, 2.0, &mut rng);
            let (pn, ph) = random_polygon_2d(n2, 0.5, 2.0, &mut rng);
            let polytope = match lagrangian_product(&qn, &qh, &pn, &ph) {
                Ok(p) => p,
                Err(e) => {
                    eprintln!("  Q2: random LP({},{},{}) — construction failed: {:?}", n1, n2, j, e);
                    continue;
                }
            };
            let id = format!("lp_random_{}_{}_{:02}", n1, n2, j);

            if let Some(info) = analyze_polytope(&polytope) {
                let rows = validate_derivatives(
                    &info, FD_SWEET_SPOT, &id, "lagrangian_random",
                    None, None, None, None, None,
                );
                write_rows(&mut writer, &rows);
                total_rows += rows.len();
            } else {
                eprintln!("  Q2: random LP({},{},{}) — failed", n1, n2, j);
            }
        }
    }

    writer.flush().expect("flush Q2");
    println!("Q2 done: {} rows written to {}", total_rows, path);
}

// ============================================================================
// Phase Q3: Near-degeneracy
// ============================================================================

/// Gap bins for Q3: (lower_bound, upper_bound, label).
const Q3_GAP_BINS: [(f64, f64, &str); 4] = [
    (1e-1, f64::INFINITY, "large"),
    (1e-2, 1e-1, "medium"),
    (1e-4, 1e-2, "small"),
    (0.0, 1e-4, "tiny"),
];

fn run_q3(base_dir: &str) {
    let path = format!("{}/gradient-correctness-q3-degeneracy.jsonl", base_dir);
    let file = File::create(&path).expect("create Q3 JSONL");
    let mut writer = BufWriter::new(file);

    // Bin counts
    let mut bin_counts = [0usize; 4];
    let mut total_rows = 0;
    let mut generated = 0;

    let mut rng = ChaCha8Rng::seed_from_u64(SEED_BASE + 300);
    let f_count = 6; // Small F for tractable orbit enumeration

    println!("  Q3: Generating candidates (F={})...", f_count);

    while generated < Q3_MAX_CANDIDATES && bin_counts.iter().any(|&c| c < Q3_PER_BIN) {
        // Generate a batch of polytopes
        let polytopes = generate_random_polytopes(10, f_count, 0.5, 2.0, &mut rng);

        for polytope in &polytopes {
            generated += 1;

            // All bins full?
            if bin_counts.iter().all(|&c| c >= Q3_PER_BIN) {
                break;
            }

            // Enumerate all certified orbits
            let orbits = enumerate_all_orbits(polytope);
            if orbits.len() < 2 {
                continue; // Need at least 2 orbits for a gap
            }

            let best_action = orbits[0].0;
            let second_action = orbits[1].0;
            let gap = second_action - best_action;

            // Find which bin this goes in
            let bin_idx = Q3_GAP_BINS.iter().position(|&(lo, hi, _)| gap >= lo && gap < hi);
            let bin_idx = match bin_idx {
                Some(idx) if bin_counts[idx] < Q3_PER_BIN => idx,
                _ => continue, // No matching bin or bin full
            };

            // Compute gradient validation for this polytope
            let info = match analyze_polytope(polytope) {
                Some(info) => info,
                None => continue,
            };

            // Check orbit switching during FD
            let duals = polytope.dual_vertices_f64();
            let switched = check_orbit_switching(&duals, FD_SWEET_SPOT, &info.best_perm);

            let id = format!("degeneracy_{}_{:03}", Q3_GAP_BINS[bin_idx].2, bin_counts[bin_idx]);
            let rows = validate_derivatives(
                &info, FD_SWEET_SPOT, &id, "near_degenerate",
                Some(gap), Some(second_action), Some(switched),
                None, None,
            );
            write_rows(&mut writer, &rows);
            total_rows += rows.len();
            bin_counts[bin_idx] += 1;

            if generated % 100 == 0 {
                println!(
                    "  Q3: {} candidates tested, bins: large={}, medium={}, small={}, tiny={}",
                    generated, bin_counts[0], bin_counts[1], bin_counts[2], bin_counts[3],
                );
            }
        }
    }

    writer.flush().expect("flush Q3");
    println!(
        "Q3 done: {} rows, {} candidates tested, bins: large={}, medium={}, small={}, tiny={}",
        total_rows, generated, bin_counts[0], bin_counts[1], bin_counts[2], bin_counts[3],
    );
}

// ============================================================================
// Phase Q4: Barely-cutting facets
// ============================================================================

fn run_q4(base_dir: &str) {
    let path = format!("{}/gradient-correctness-q4-redundant.jsonl", base_dir);
    let file = File::create(&path).expect("create Q4 JSONL");
    let mut writer = BufWriter::new(file);
    let mut total_rows = 0;

    let mut rng = ChaCha8Rng::seed_from_u64(SEED_BASE + 400);
    let f_count = 6;
    let base_polytopes = generate_random_polytopes(Q4_BASE_COUNT, f_count, 0.5, 2.0, &mut rng);

    for (i, base) in base_polytopes.iter().enumerate() {
        for &delta in Q4_DELTAS {
            let augmented = match add_barely_cutting_facet(base, delta, &mut rng) {
                Some(p) => p,
                None => {
                    eprintln!("  Q4: base {} delta={:.0e} — construction failed, skipping", i, delta);
                    continue;
                }
            };

            let info = match analyze_polytope(&augmented) {
                Some(info) => info,
                None => {
                    eprintln!("  Q4: base {} delta={:.0e} — capacity failed, skipping", i, delta);
                    continue;
                }
            };

            // Compute minimum facet volume (for the barely-cutting facet analysis)
            let min_fv = {
                let vols: Vec<f64> = (0..augmented.facet_count())
                    .map(|k| facet_volume_3d(&augmented, k))
                    .filter(|&fv| fv > 0.0)
                    .collect();
                vols.iter().copied().fold(f64::INFINITY, f64::min)
            };

            let id = format!("barely_cutting_{:02}_d{:.0e}", i, delta);
            let rows = validate_derivatives(
                &info, FD_SWEET_SPOT, &id, "barely_cutting",
                None, None, None,
                Some(delta), Some(min_fv),
            );
            write_rows(&mut writer, &rows);
            total_rows += rows.len();
        }
        println!("  Q4: base polytope {}/{} done", i + 1, Q4_BASE_COUNT);
    }

    writer.flush().expect("flush Q4");
    println!("Q4 done: {} rows written", total_rows);
}

// ============================================================================
// Main
// ============================================================================

fn main() {
    let base_dir = "gradient-correctness";
    let args: Vec<String> = std::env::args().collect();

    // Optional: pass phase names to run only specific phases.
    // E.g. `cargo run --release --bin gradient_correctness -- q2 q3 q4`
    let run_all = args.len() <= 1;
    let phases: Vec<&str> = if run_all {
        vec!["q1", "q2", "q3", "q4"]
    } else {
        args[1..].iter().map(|s| s.as_str()).collect()
    };

    println!("=== Gradient Correctness Experiment ===");
    println!("Phases: {:?}\n", phases);

    let t0 = Instant::now();

    if phases.contains(&"q1") {
        println!("--- Phase Q1: Generic random polytopes ---");
        let t_q1 = Instant::now();
        run_q1(base_dir);
        println!("  Q1 time: {:.1}s\n", t_q1.elapsed().as_secs_f64());
    }

    if phases.contains(&"q2") {
        println!("--- Phase Q2: Non-generic geometry ---");
        let t_q2 = Instant::now();
        run_q2(base_dir);
        println!("  Q2 time: {:.1}s\n", t_q2.elapsed().as_secs_f64());
    }

    if phases.contains(&"q3") {
        println!("--- Phase Q3: Near-degeneracy ---");
        let t_q3 = Instant::now();
        run_q3(base_dir);
        println!("  Q3 time: {:.1}s\n", t_q3.elapsed().as_secs_f64());
    }

    if phases.contains(&"q4") {
        println!("--- Phase Q4: Barely-cutting facets ---");
        let t_q4 = Instant::now();
        run_q4(base_dir);
        println!("  Q4 time: {:.1}s\n", t_q4.elapsed().as_secs_f64());
    }

    println!("=== Total time: {:.1}s ===", t0.elapsed().as_secs_f64());
}

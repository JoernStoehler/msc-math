//! Facet-splitting experiment: test HKO2024's maximality in the F=11 polytope space.
//!
//! Goal: Test whether facet-splitting directions around HKO2024 produce any
//! sys increase in the 11-facet ambient space.
//! Input Artifacts: None (starts from the hardcoded HKO2024 polytope).
//! Output Artifacts: experiments/hko-local-maximum/facet-splitting/hko-neighborhood-splitting.jsonl
//!
//! Adds a cutting halfspace to HKO2024 (10 facets) to create an 11-facet polytope
//! K' ⊊ K, and checks whether sys(K') > sys(K) for any cutting direction.
//!
//! Split from gradient-is-zero/main.rs (Phase B).
//!
//! Architecture:
//! 1. `cargo run --bin hko-facet-splitting --release` generates dataset
//! 2. Writes hko-neighborhood-splitting.jsonl
//! 3. Python script (analyze.py) reads JSONL, produces figures
//!
//! Methodology: we add a cutting halfspace ⟨n,x⟩ ≤ h_K(n) - ε to create an
//! (F+1)-facet polytope K' ⊊ K. This is the only non-trivial direction from
//! HKO2024 in the F=11 ambient space: adding a halfspace is an intersection,
//! so K' ⊆ K always. When h = h_K(n) the halfspace is redundant (K' = K);
//! when h < h_K(n) it cuts. To make K *larger* we'd need to relax an existing
//! halfspace, which is already covered by gradient-analysis's (n,h) gradient analysis.

use nalgebra::Vector4;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use rand_distr::{Distribution, StandardNormal};
use serde::Serialize;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::time::Instant;
use symplectic::ehz_capacity;
use symplectic::geom::known_polytopes;
use symplectic::geom::polytope::Polytope4D;
use symplectic::geom::volume::volume;

/// Number of angular samples per representative facet normal for facet-splitting.
/// HKO2024 = pentagon ×_L pentagon: all Q-space normals equivalent, all P-space equivalent.
/// Only 2 representatives needed (facet 0 = Q-space, facet 5 = P-space).
const N_SPLITTING_SAMPLES_PER_FACET: usize = 100;

/// Number of random mixed directions (neither purely Q nor purely P space).
const N_SPLITTING_MIXED: usize = 50;

/// Number of random control directions for facet-splitting.
const N_SPLITTING_CONTROL: usize = 20;

/// Small epsilon for facet-splitting (how deep to cut).
const SPLITTING_EPSILONS: &[f64] = &[1e-3, 1e-4];

// ============================================================================
// Output schema
// ============================================================================

#[derive(Debug, Serialize)]
struct SplittingRow {
    // Cutting direction
    source_facet: usize, // which existing facet normal this is near (usize::MAX for control, usize::MAX-1 for mixed)
    angular_offset: f64, // angle from source facet normal (radians)
    cutting_normal: [f64; 4],
    epsilon: f64,
    // Results
    sys_original: f64,
    sys_split: f64,
    delta_sys: f64,
    capacity_split: f64,
    volume_split: f64,
    facet_count_split: usize,
    n_valid_orbits: usize,
    best_subset: Vec<usize>,
    best_permutation: Vec<usize>,
    // Gradient at split polytope
    d_sys_d_h_new: f64, // ∂sys/∂h_{F+1} — the splitting gradient
    construction_ok: bool,
    time_ms: f64,
}

// ============================================================================
// Helpers
// ============================================================================

/// Safely compute sys for a polytope, catching panics from degenerate geometry.
fn safe_sys(polytope: &Polytope4D) -> Option<(f64, f64, f64)> {
    let vol = volume(polytope);
    if vol <= 0.0 {
        return None;
    }
    let cap = ehz_capacity(polytope)
        .ok()
        .map(|r| r.capacity())
        .unwrap_or(f64::NAN);
    if !cap.is_finite() {
        return None;
    }
    let sys = cap * cap / (2.0 * vol);
    if sys.is_finite() {
        Some((sys, vol, cap))
    } else {
        None
    }
}

/// Sample directions near a given facet normal on S³.
/// Returns (direction, angular_offset) pairs.
fn sample_near_normal(
    normal: &Vector4<f64>,
    n_samples: usize,
    rng: &mut ChaCha8Rng,
) -> Vec<(Vector4<f64>, f64)> {
    let mut results = Vec::with_capacity(n_samples);

    // Build an orthonormal basis for T_{normal}S³ (3D tangent space)
    let mut basis = Vec::new();
    let candidates = [
        Vector4::new(1.0, 0.0, 0.0, 0.0),
        Vector4::new(0.0, 1.0, 0.0, 0.0),
        Vector4::new(0.0, 0.0, 1.0, 0.0),
        Vector4::new(0.0, 0.0, 0.0, 1.0),
    ];
    for c in &candidates {
        let proj = *c - c.dot(normal) * normal;
        // Gram-Schmidt against existing basis vectors
        let mut v = proj;
        for b in &basis {
            v -= v.dot(b) * b;
        }
        if v.norm() > 0.1 {
            basis.push(v.normalize());
        }
        if basis.len() == 3 {
            break;
        }
    }

    // Sample at various angular offsets
    let angular_scales: [f64; 10] = [0.01, 0.02, 0.05, 0.1, 0.2, 0.3, 0.5, 0.7, 1.0, 1.5];
    let samples_per_scale = n_samples / angular_scales.len();

    for &angle in &angular_scales {
        for _ in 0..samples_per_scale {
            // Random tangent direction
            let t0: f64 = StandardNormal.sample(rng);
            let t1: f64 = StandardNormal.sample(rng);
            let t2: f64 = StandardNormal.sample(rng);
            let tangent = t0 * basis[0] + t1 * basis[1] + t2 * basis[2];
            let tangent = tangent.normalize();

            // Rotate normal by angle in the tangent direction
            let dir = (normal * angle.cos() + tangent * angle.sin()).normalize();
            let actual_angle = normal.dot(&dir).clamp(-1.0, 1.0).acos();
            results.push((dir, actual_angle));
        }
    }

    results
}

// ============================================================================
// Phase B: Facet-splitting
// ============================================================================

fn run_phase_b(base_dir: &std::path::Path) {
    println!("═══════════════════════════════════════════════════════════");
    println!("Phase B: Facet-splitting (F=11) — test maximality beyond F=10");
    println!("═══════════════════════════════════════════════════════════\n");

    let known = known_polytopes::hko_pentagon();
    let polytope = &known.polytope;
    let f = polytope.facet_count();
    let duals = polytope.dual_vertices_f64();
    let normals: Vec<Vector4<f64>> = duals.iter().map(|a| a / a.norm()).collect();
    let vertices = polytope.vertices_f64();

    let vol_orig = volume(polytope);
    let cap_orig = ehz_capacity(polytope).expect("ehz").capacity();
    let sys_orig = cap_orig * cap_orig / (2.0 * vol_orig);
    println!("HKO2024 baseline: F={f}, sys={sys_orig:.10}");

    let splitting_path = base_dir.join("facet-splitting/hko-neighborhood-splitting.jsonl");
    let splitting_file = File::create(&splitting_path).expect("create splitting JSONL");
    let mut split_writer = BufWriter::new(splitting_file);

    let mut rng = ChaCha8Rng::seed_from_u64(123);
    let mut total_directions = 0usize;
    let mut total_ok = 0usize;
    let mut best_delta = f64::NEG_INFINITY;
    let mut best_direction_info = String::new();

    // HKO2024 = pentagon ×_L pentagon. Lagrangian product symmetry means:
    // - All 5 Q-space normals (facets 0-4) are equivalent under 5-fold rotation
    // - All 5 P-space normals (facets 5-9) are equivalent under 5-fold rotation
    // So we only need 2 representative facets, not 10.
    let representative_facets = [0usize, 5]; // Q-space rep, P-space rep
    for &facet_k in &representative_facets {
        println!(
            "\nFacet {facet_k} (representative): normal = [{:.4}, {:.4}, {:.4}, {:.4}]",
            normals[facet_k][0], normals[facet_k][1], normals[facet_k][2], normals[facet_k][3]
        );

        let samples =
            sample_near_normal(&normals[facet_k], N_SPLITTING_SAMPLES_PER_FACET, &mut rng);

        for (dir, angular_offset) in &samples {
            for &eps in SPLITTING_EPSILONS {
                let t_split = Instant::now();
                total_directions += 1;

                // Compute support function h_K(n) = max_v <n, v>
                let h_k_n = vertices
                    .iter()
                    .map(|v| dir.dot(v))
                    .fold(f64::NEG_INFINITY, f64::max);

                // Add cutting halfspace: <n, x> <= h_K(n) - eps
                // In dual vertex form: a = n / h, so new dual vertex = dir / (h_k_n - eps)
                let new_h = h_k_n - eps;
                if new_h <= 0.0 {
                    continue;
                }
                let mut new_duals: Vec<Vector4<f64>> = duals.to_vec();
                new_duals.push(dir / new_h);

                match Polytope4D::from_f64(new_duals) {
                    Ok(split_poly) => {
                        let (split_sys, split_vol, split_cap) = match safe_sys(&split_poly) {
                            Some(v) => v,
                            None => continue,
                        };
                        let delta = split_sys - sys_orig;

                        // Use library ehz_capacity for orbit info (cheaper than instrumented)
                        let lib_result = ehz_capacity(&split_poly).ok();
                        let n_valid = 0; // not computed (instrumented too expensive for F=11)
                        let best_sub = lib_result
                            .as_ref()
                            .map(|r| r.best_subset())
                            .unwrap_or_default();
                        let best_perm = lib_result
                            .as_ref()
                            .map(|r| r.best_sigma().to_vec())
                            .unwrap_or_default();
                        let d_sys_d_h_new = f64::NAN; // skip per-direction gradient (too expensive)

                        total_ok += 1;
                        if delta > best_delta {
                            best_delta = delta;
                            best_direction_info = format!(
                                "facet={facet_k}, angle={angular_offset:.4}, eps={eps:.1e}, Δsys={delta:.6e}"
                            );
                        }

                        let row = SplittingRow {
                            source_facet: facet_k,
                            angular_offset: *angular_offset,
                            cutting_normal: [dir[0], dir[1], dir[2], dir[3]],
                            epsilon: eps,
                            sys_original: sys_orig,
                            sys_split: split_sys,
                            delta_sys: delta,
                            capacity_split: split_cap,
                            volume_split: split_vol,
                            facet_count_split: split_poly.facet_count(),
                            n_valid_orbits: n_valid,
                            best_subset: best_sub,
                            best_permutation: best_perm,
                            d_sys_d_h_new,
                            construction_ok: true,
                            time_ms: t_split.elapsed().as_secs_f64() * 1000.0,
                        };
                        serde_json::to_writer(&mut split_writer, &row).expect("write splitting");
                        writeln!(split_writer).expect("newline");
                    }
                    Err(_) => {
                        // Construction failed (degenerate geometry at small eps)
                        let row = SplittingRow {
                            source_facet: facet_k,
                            angular_offset: *angular_offset,
                            cutting_normal: [dir[0], dir[1], dir[2], dir[3]],
                            epsilon: eps,
                            sys_original: sys_orig,
                            sys_split: f64::NAN,
                            delta_sys: f64::NAN,
                            capacity_split: f64::NAN,
                            volume_split: f64::NAN,
                            facet_count_split: 0,
                            n_valid_orbits: 0,
                            best_subset: vec![],
                            best_permutation: vec![],
                            d_sys_d_h_new: f64::NAN,
                            construction_ok: false,
                            time_ms: t_split.elapsed().as_secs_f64() * 1000.0,
                        };
                        serde_json::to_writer(&mut split_writer, &row).expect("write splitting");
                        writeln!(split_writer).expect("newline");
                    }
                }
            }
        }

        // Progress
        println!(
            "  Tested {} directions so far, {} OK, best Δsys={:.6e}",
            total_directions, total_ok, best_delta
        );
    }

    // Mixed directions: components in both Q and P space (breaks Lagrangian product structure)
    println!("\n--- Mixed directions (Q+P space) ---");
    for i in 0..N_SPLITTING_MIXED {
        let t0: f64 = StandardNormal.sample(&mut rng);
        let t1: f64 = StandardNormal.sample(&mut rng);
        let t2: f64 = StandardNormal.sample(&mut rng);
        let t3: f64 = StandardNormal.sample(&mut rng);
        let dir = Vector4::new(t0, t1, t2, t3).normalize();

        // Ensure direction has components in both Q and P space
        let q_norm = (dir[0] * dir[0] + dir[1] * dir[1]).sqrt();
        let p_norm = (dir[2] * dir[2] + dir[3] * dir[3]).sqrt();
        if q_norm < 0.1 || p_norm < 0.1 {
            continue; // skip nearly-pure Q or P directions
        }

        let min_angle = normals
            .iter()
            .map(|n| n.dot(&dir).clamp(-1.0, 1.0).acos())
            .fold(f64::INFINITY, f64::min);

        for &eps in SPLITTING_EPSILONS {
            let t_split = Instant::now();
            total_directions += 1;

            let h_k_n = vertices
                .iter()
                .map(|v| dir.dot(v))
                .fold(f64::NEG_INFINITY, f64::max);

            let new_h = h_k_n - eps;
            if new_h <= 0.0 {
                continue;
            }
            let mut new_duals: Vec<Vector4<f64>> = duals.to_vec();
            new_duals.push(dir / new_h);

            if let Ok(split_poly) = Polytope4D::from_f64(new_duals) {
                let (split_sys, split_vol, split_cap) = match safe_sys(&split_poly) {
                    Some(v) => v,
                    None => continue,
                };
                let delta = split_sys - sys_orig;

                let lib_result = ehz_capacity(&split_poly).ok();
                let n_valid = 0;
                let best_sub = lib_result
                    .as_ref()
                    .map(|r| r.best_subset())
                    .unwrap_or_default();
                let best_perm = lib_result
                    .as_ref()
                    .map(|r| r.best_sigma().to_vec())
                    .unwrap_or_default();

                total_ok += 1;
                if delta > best_delta {
                    best_delta = delta;
                    best_direction_info = format!(
                        "mixed #{i}, angle_to_nearest={min_angle:.4}, eps={eps:.1e}, Δsys={delta:.6e}"
                    );
                }

                if i < 5 || delta > -1e-6 {
                    println!(
                        "  Mixed #{i}: angle={min_angle:.4}, q={q_norm:.3}, p={p_norm:.3}, \
                         eps={eps:.1e}, Δsys={delta:.6e}"
                    );
                }

                let row = SplittingRow {
                    source_facet: usize::MAX - 1, // sentinel for "mixed"
                    angular_offset: min_angle,
                    cutting_normal: [dir[0], dir[1], dir[2], dir[3]],
                    epsilon: eps,
                    sys_original: sys_orig,
                    sys_split: split_sys,
                    delta_sys: delta,
                    capacity_split: split_cap,
                    volume_split: split_vol,
                    facet_count_split: split_poly.facet_count(),
                    n_valid_orbits: n_valid,
                    best_subset: best_sub,
                    best_permutation: best_perm,
                    d_sys_d_h_new: f64::NAN,
                    construction_ok: true,
                    time_ms: t_split.elapsed().as_secs_f64() * 1000.0,
                };
                serde_json::to_writer(&mut split_writer, &row).expect("write splitting");
                writeln!(split_writer).expect("newline");
            }
        }
    }
    println!(
        "  Mixed: tested {} total, {} OK, best Δsys={:.6e}",
        total_directions, total_ok, best_delta
    );

    // Control: random directions far from any facet normal
    println!("\n--- Control: random directions ---");
    for i in 0..N_SPLITTING_CONTROL {
        let t0: f64 = StandardNormal.sample(&mut rng);
        let t1: f64 = StandardNormal.sample(&mut rng);
        let t2: f64 = StandardNormal.sample(&mut rng);
        let t3: f64 = StandardNormal.sample(&mut rng);
        let dir = Vector4::new(t0, t1, t2, t3).normalize();

        // Check angular distance to nearest facet normal
        let min_angle = normals
            .iter()
            .map(|n| n.dot(&dir).clamp(-1.0, 1.0).acos())
            .fold(f64::INFINITY, f64::min);

        for &eps in SPLITTING_EPSILONS {
            let t_split = Instant::now();
            total_directions += 1;

            let h_k_n = vertices
                .iter()
                .map(|v| dir.dot(v))
                .fold(f64::NEG_INFINITY, f64::max);

            let new_h = h_k_n - eps;
            if new_h <= 0.0 {
                continue;
            }
            let mut new_duals2: Vec<Vector4<f64>> = duals.to_vec();
            new_duals2.push(dir / new_h);

            if let Ok(split_poly) = Polytope4D::from_f64(new_duals2) {
                let (split_sys, split_vol, split_cap) = match safe_sys(&split_poly) {
                    Some(v) => v,
                    None => continue,
                };
                let delta = split_sys - sys_orig;

                let lib_result = ehz_capacity(&split_poly).ok();
                let n_valid = 0;
                let best_sub = lib_result
                    .as_ref()
                    .map(|r| r.best_subset())
                    .unwrap_or_default();
                let best_perm = lib_result
                    .as_ref()
                    .map(|r| r.best_sigma().to_vec())
                    .unwrap_or_default();
                let d_sys_d_h_new = f64::NAN;

                total_ok += 1;
                println!(
                    "  Control #{i}: angle_to_nearest={min_angle:.4}, eps={eps:.1e}, \
                         Δsys={delta:.6e}, d_sys_d_h_new={d_sys_d_h_new:.6e}"
                );

                let row = SplittingRow {
                    source_facet: usize::MAX, // sentinel for "control"
                    angular_offset: min_angle,
                    cutting_normal: [dir[0], dir[1], dir[2], dir[3]],
                    epsilon: eps,
                    sys_original: sys_orig,
                    sys_split: split_sys,
                    delta_sys: delta,
                    capacity_split: split_cap,
                    volume_split: split_vol,
                    facet_count_split: split_poly.facet_count(),
                    n_valid_orbits: n_valid,
                    best_subset: best_sub,
                    best_permutation: best_perm,
                    d_sys_d_h_new,
                    construction_ok: true,
                    time_ms: t_split.elapsed().as_secs_f64() * 1000.0,
                };
                serde_json::to_writer(&mut split_writer, &row).expect("write splitting");
                writeln!(split_writer).expect("newline");
            }
        }
    }

    split_writer.flush().expect("flush splitting");

    println!("\n--- Facet-splitting summary ---");
    println!("  Directions tested: {total_directions}");
    println!("  Successful constructions: {total_ok}");
    println!("  Best Δsys: {best_delta:.6e}");
    println!("  Best direction: {best_direction_info}");
    if best_delta <= 0.0 {
        println!("  → HKO2024 is a LOCAL MAXIMUM even under facet-splitting (F=11)");
    } else {
        println!("  → HKO2024 is NOT a local max under facet-splitting — improvement found!");
    }
    println!("  Wrote {}", splitting_path.display());
}

// ============================================================================
// Main
// ============================================================================

fn main() {
    let t0 = Instant::now();
    let base_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));

    println!("Facet-splitting: HKO2024 maximality test in F=11 space\n");

    run_phase_b(base_dir);

    let elapsed = t0.elapsed().as_secs_f64();
    println!("\n═══════════════════════════════════════════════════════════");
    println!("Total time: {elapsed:.1}s");
    println!("═══════════════════════════════════════════════════════════");
}

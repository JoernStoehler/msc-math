//! Second-order analysis of flat directions at HKO2024.
//!
//! Goal: Test whether the first-order-flat directions at HKO2024 remain
//! descending at second order, both for basis directions and random flat probes.
//! Input Artifacts: None (starts from the hardcoded HKO2024 polytope).
//! Output Artifacts: experiments/hko-local-maximum/second-order/second-order-base.jsonl
//!         experiments/hko-local-maximum/second-order/second-order-curves.jsonl
//!         experiments/hko-local-maximum/second-order/second-order-random.jsonl
//!
//! Phase 1: Compute per-orbit ∇_{a_i} sys in R^40 for all near-optimal orbits,
//!          build gradient matrix G, SVD → flat directions (null space of G).
//! Phase 2: For each flat direction d, evaluate sys(HKO + ε·d) on the 28 nonzero
//!          ±ε points from `EPSILON_GRID`; the base point lives in
//!          `second-order-base.jsonl`.
//!
//! Replaces the broken Phase C LP (subdifferential-lp/phase_c_lp_test.py) with
//! clean a_i-space computation. The old script reads normals/heights fields that
//! no longer exist in the JSONL after the a_i migration.
//!
//! Output Artifacts:
//!   second-order-base.jsonl   — SVD, flat directions, gradient matrix
//!   second-order-curves.jsonl — sys(±ε) for each flat direction
//!   second-order-random.jsonl — random normalized coefficient directions in ker(G)
//!
//! Mathematical basis: Danskin's theorem gives D_d⁺ sys = min_i (∇sys_i · d).
//! Flat directions d satisfy ∇sys_i · d = 0 for all active orbits i.
//! Second-order analysis: if sys(K + εd) < sys(K) for all ε ≠ 0 and all flat d,
//! then K is a strict local maximum. See formal/hko-local-maximum/second-order.tex for formal statement.

use exp_hko_local_maximum::ehz_capacity_instrumented;
use nalgebra::{DMatrix, Vector4};
use rand::Rng as _;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use serde::Serialize;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::time::Instant;
use symplectic::algorithms::OrbitKktData;
use symplectic::derivatives::{capacity_derivatives_a_from_orbit, volume_derivatives_a};
use symplectic::ehz_capacity;
use symplectic::geom::known_polytopes;
use symplectic::geom::polytope::Polytope4D;
use symplectic::geom::volume::volume;

/// Gap threshold for near-optimal orbits. All 150 HKO2024 orbits have gap < 1.3e-15,
/// so any threshold well above machine epsilon includes all of them. Using 1e-10
/// to be precise about "machine-precision degenerate" while excluding genuinely
/// suboptimal orbits.
const NEAR_OPTIMAL_GAP: f64 = 1e-10;

/// SVD rank threshold: singular values below this fraction of σ_max are treated as null.
/// 1e-8 is well above machine epsilon (~1e-16) but below any physically meaningful
/// gradient magnitude. The gradient norms are O(1), so σ > 1e-8 × σ_max means the
/// direction carries real gradient information.
const SVD_RANK_THRESHOLD: f64 = 1e-8;

/// Epsilon grid for finite-difference curves.
/// Fine range: resolves curvature near ε=0 (FD noise is O(ε²) for first-order-flat functions).
/// Medium range: transition zone.
/// Coarse range: reaches characteristic radius ~0.035 per component (lagrangian-boundary experiment).
const EPSILON_GRID: &[f64] = &[
    5e-5, 1e-4, 2e-4, 5e-4, 1e-3, 2e-3, 5e-3, 1e-2, 1.5e-2, 2e-2, 2.5e-2, 3e-2, 3.5e-2, 4e-2,
];

/// Number of random directions in ker(G) to sample for negative-definiteness check.
/// 100 normalized random coefficient directions give broad coverage of the 15D
/// flat subspace without needing to persist full curve traces.
const N_RANDOM_DIRECTIONS: usize = 100;

/// Epsilon values for the random-direction curvature check.
/// Fewer points than the full grid — we only need the curvature sign, not a detailed curve.
/// Using the medium range where FD noise is low and curvature signal is clear.
const EPSILON_RANDOM: &[f64] = &[1e-4, 5e-4, 1e-3, 5e-3];

/// RNG seed for reproducibility.
const RANDOM_SEED: u64 = 42;

// ============================================================================
// Output schemas
// ============================================================================

#[derive(Debug, Serialize)]
struct BaseRow {
    facet_count: usize,
    dual_vertices: Vec<[f64; 4]>,
    sys_base: f64,
    capacity_base: f64,
    volume_base: f64,
    n_orbits_total: usize,
    n_near_optimal: usize,
    singular_values: Vec<f64>,
    rank: usize,
    n_flat_directions: usize,
    /// Each flat direction is 10 vectors in R^4 (one per facet), flattened as [[f64; 4]; 10].
    flat_directions: Vec<Vec<[f64; 4]>>,
    /// Per-orbit sys gradient in R^40, stored as Vec<[f64; 4]> per orbit.
    gradient_matrix: Vec<Vec<[f64; 4]>>,
    time_phase1_ms: f64,
}

#[derive(Debug, Serialize)]
struct CurveRow {
    direction_index: usize,
    epsilon: f64,
    sys: f64,
    capacity: f64,
    volume: f64,
    delta_sys: f64,
    time_ms: f64,
}

// ============================================================================
// Instrumented HK2017 — collects ALL valid orbits (copied from gradient-analysis)
// ============================================================================

// ============================================================================
// Per-orbit sys gradient in a_i space (R^40)
// ============================================================================

/// Compute ∇_{a_i} sys for a single orbit, returned as Vec<Vector4> (one per facet).
///
/// ∇_{a_i} sys = (c · ∇_{a_i} c_orbit - sys · ∇_{a_i} vol) / vol
///
/// This is the quotient rule applied to sys = c²/(2·vol):
/// ∂sys/∂a_k = (c/vol) · ∂c/∂a_k − (sys/vol) · ∂vol/∂a_k.
/// Per-orbit: uses ∂c_i/∂a_k from [lem:cap-derivative] and ∂vol/∂a_k from [lem:vol-derivative].
/// // TODO: add [lem:sys-derivative] to formal math — quotient rule for sys = c²/(2vol)
///
/// Requires re-solving KKT to obtain the multiplier μ (not stored in JSONL).
fn orbit_sys_gradient_a(
    polytope: &Polytope4D,
    orbit: &OrbitKktData,
    vol: f64,
    cap: f64,
    sys: f64,
    d_vol_a: &[Vector4<f64>],
) -> Vec<Vector4<f64>> {
    let d_cap_a = capacity_derivatives_a_from_orbit(polytope, orbit)
        .expect("second-order stores orbit payloads with closure multipliers");

    d_vol_a
        .iter()
        .zip(d_cap_a.iter())
        .map(|(dv, dc)| (cap * dc - sys * dv) / vol)
        .collect()
}

/// Flatten Vec<Vector4> (10 facets × R^4) into a 40-element f64 slice.
fn flatten_gradient(grad: &[Vector4<f64>]) -> Vec<f64> {
    grad.iter().flat_map(|v| [v[0], v[1], v[2], v[3]]).collect()
}

/// Unflatten 40 f64s back into 10 × [f64; 4] for JSONL output.
fn unflatten_to_arrays(flat: &[f64]) -> Vec<[f64; 4]> {
    flat.chunks(4).map(|c| [c[0], c[1], c[2], c[3]]).collect()
}

// ============================================================================
// Phase 1: Gradient matrix and flat directions
// ============================================================================

fn run_phase1(polytope: &Polytope4D) -> (BaseRow, Vec<Vec<f64>>) {
    let f = polytope.facet_count();
    let dim = f * 4; // 40 for F=10

    // Compute base quantities
    let vol = volume(polytope);
    let instr = ehz_capacity_instrumented(polytope).expect("no valid orbits");

    let cap = instr.capacity;
    let sys = cap * cap / (2.0 * vol);

    println!("  Base: sys={sys:.10}, c={cap:.10}, vol={vol:.10}");
    println!("  Total valid orbits: {}", instr.orbits.len());

    // Filter near-optimal
    let best_action = instr.orbits[0].action;
    let near_optimal: Vec<&OrbitKktData> = instr
        .orbits
        .iter()
        .filter(|o| {
            let gap = (o.action - best_action) / best_action;
            gap < NEAR_OPTIMAL_GAP
        })
        .collect();
    println!(
        "  Near-optimal (gap < {NEAR_OPTIMAL_GAP:.0e}): {} orbits",
        near_optimal.len()
    );
    if let Some(worst) = near_optimal.last() {
        let gap = (worst.action - best_action) / best_action;
        println!("  Worst near-optimal gap: {gap:.2e}");
    }

    // Volume derivatives (same for all orbits)
    let d_vol_a = volume_derivatives_a(polytope);

    // Build gradient matrix: each row is one orbit's ∇_{a_i} sys ∈ R^40
    let mut gradient_rows: Vec<Vec<f64>> = Vec::with_capacity(near_optimal.len());
    let mut gradient_matrix_arrays: Vec<Vec<[f64; 4]>> = Vec::with_capacity(near_optimal.len());

    for orbit in &near_optimal {
        let grad = orbit_sys_gradient_a(polytope, orbit, vol, cap, sys, &d_vol_a);
        let flat = flatten_gradient(&grad);
        gradient_matrix_arrays.push(grad.iter().map(|v| [v[0], v[1], v[2], v[3]]).collect());
        gradient_rows.push(flat);
    }

    // Build DMatrix for SVD (num_orbits × 40)
    let n_orbits = gradient_rows.len();
    let g_matrix = DMatrix::from_fn(n_orbits, dim, |i, j| gradient_rows[i][j]);

    println!("\n  Gradient matrix: {}×{}", n_orbits, dim);

    // SVD
    let svd = g_matrix.svd(false, true);
    let singular_values: Vec<f64> = svd.singular_values.iter().cloned().collect();

    let sigma_max = singular_values[0];
    let threshold = sigma_max * SVD_RANK_THRESHOLD;
    let rank = singular_values.iter().filter(|&&s| s > threshold).count();
    let n_flat = dim - rank;

    println!("  SVD: σ_max={sigma_max:.6e}, threshold={threshold:.6e}");
    println!("  Rank: {rank} (of {dim})");
    println!("  Flat directions: {n_flat}");
    println!(
        "  Top 10 singular values: {:?}",
        singular_values
            .iter()
            .take(10)
            .map(|s| format!("{s:.4e}"))
            .collect::<Vec<_>>()
    );
    if rank < dim {
        println!("  Singular values near rank boundary:");
        let start = rank.saturating_sub(2);
        let end = (rank + 3).min(singular_values.len());
        for (i, &s) in singular_values[start..end].iter().enumerate() {
            let idx = start + i;
            let marker = if idx == rank {
                " ← rank boundary"
            } else {
                ""
            };
            println!("    σ[{idx}] = {s:.6e}{marker}");
        }
    }

    // Extract flat directions from V^T (rows beyond rank)
    let v_t = svd.v_t.expect("SVD v_t should exist");
    let mut flat_directions: Vec<Vec<f64>> = Vec::with_capacity(n_flat);
    let mut flat_directions_arrays: Vec<Vec<[f64; 4]>> = Vec::with_capacity(n_flat);

    for i in rank..dim {
        let row: Vec<f64> = (0..dim).map(|j| v_t[(i, j)]).collect();
        flat_directions_arrays.push(unflatten_to_arrays(&row));
        flat_directions.push(row);
    }

    let duals_raw: Vec<[f64; 4]> = polytope
        .dual_vertices_f64()
        .iter()
        .map(|a| [a[0], a[1], a[2], a[3]])
        .collect();

    let base_row = BaseRow {
        facet_count: f,
        dual_vertices: duals_raw,
        sys_base: sys,
        capacity_base: cap,
        volume_base: vol,
        n_orbits_total: instr.orbits.len(),
        n_near_optimal: near_optimal.len(),
        singular_values,
        rank,
        n_flat_directions: n_flat,
        flat_directions: flat_directions_arrays,
        gradient_matrix: gradient_matrix_arrays,
        time_phase1_ms: 0.0, // filled by caller
    };

    (base_row, flat_directions)
}

// ============================================================================
// Phase 2: Finite-difference curves along flat directions
// ============================================================================

fn run_phase2(
    polytope: &Polytope4D,
    sys_base: f64,
    flat_directions: &[Vec<f64>],
    writer: &mut BufWriter<File>,
) {
    let duals = polytope.dual_vertices_f64();
    let f = polytope.facet_count();

    println!(
        "\n  Evaluating {} directions × {} ε values (×2 for ±) = {} capacity evaluations",
        flat_directions.len(),
        EPSILON_GRID.len(),
        flat_directions.len() * EPSILON_GRID.len() * 2,
    );

    for (dir_idx, direction) in flat_directions.iter().enumerate() {
        let t_dir = Instant::now();
        let mut n_ok = 0;
        let mut n_fail = 0;

        for &eps_abs in EPSILON_GRID {
            for &sign in &[1.0, -1.0] {
                let eps = sign * eps_abs;
                let t_eval = Instant::now();

                // Perturb dual vertices: a_k + ε · d_k
                let perturbed: Vec<Vector4<f64>> = (0..f)
                    .map(|k| {
                        let d_k = Vector4::new(
                            direction[4 * k],
                            direction[4 * k + 1],
                            direction[4 * k + 2],
                            direction[4 * k + 3],
                        );
                        duals[k] + eps * d_k
                    })
                    .collect();

                // Construct polytope
                let perturbed_poly = match Polytope4D::from_f64(perturbed) {
                    Ok(p) => p,
                    Err(_) => {
                        n_fail += 1;
                        continue;
                    }
                };

                // Compute capacity
                let cap = match ehz_capacity(&perturbed_poly) {
                    Ok(r) => r.capacity(),
                    Err(_) => {
                        n_fail += 1;
                        continue;
                    }
                };

                // Compute volume
                let vol = volume(&perturbed_poly);
                if vol <= 0.0 {
                    n_fail += 1;
                    continue;
                }

                let sys_val = cap * cap / (2.0 * vol);
                let delta = sys_val - sys_base;
                let time_ms = t_eval.elapsed().as_secs_f64() * 1000.0;

                let row = CurveRow {
                    direction_index: dir_idx,
                    epsilon: eps,
                    sys: sys_val,
                    capacity: cap,
                    volume: vol,
                    delta_sys: delta,
                    time_ms,
                };
                serde_json::to_writer(&mut *writer, &row).expect("write curve row");
                writeln!(writer).expect("newline");
                n_ok += 1;
            }
        }

        let dir_time = t_dir.elapsed().as_secs_f64();
        println!("  Direction {dir_idx}: {n_ok} ok, {n_fail} failed, {dir_time:.1}s");
    }
}

// ============================================================================
// Phase 3: Random directions in ker(G) for negative-definiteness check
// ============================================================================

/// Sample a random unit vector in the flat subspace by generating bounded random
/// coefficients in the flat basis and normalizing.
fn random_flat_direction(flat_basis: &[Vec<f64>], rng: &mut ChaCha8Rng) -> (Vec<f64>, Vec<f64>) {
    let dim = flat_basis[0].len(); // 40
    let n_flat = flat_basis.len(); // 15

    // Random coefficients in [-1, 1], then normalize
    let coeffs: Vec<f64> = (0..n_flat).map(|_| rng.gen_range(-1.0..1.0)).collect();
    let norm_coeffs: f64 = coeffs.iter().map(|c| c * c).sum::<f64>().sqrt();
    let normalized_coeffs: Vec<f64> = coeffs.iter().map(|c| c / norm_coeffs).collect();

    let mut direction = vec![0.0; dim];
    for (i, &normalized_c) in normalized_coeffs.iter().enumerate() {
        for j in 0..dim {
            direction[j] += normalized_c * flat_basis[i][j];
        }
    }
    (direction, normalized_coeffs)
}

/// Compute symmetric curvature ratio at a single epsilon:
/// r(ε) = (sys(+ε) + sys(-ε) - 2·sys(0)) / ε²
fn curvature_at_epsilon(
    polytope: &Polytope4D,
    direction: &[f64],
    eps: f64,
    sys_base: f64,
) -> Option<f64> {
    let duals = polytope.dual_vertices_f64();
    let f = polytope.facet_count();

    let eval = |sign: f64| -> Option<f64> {
        let e = sign * eps;
        let perturbed: Vec<Vector4<f64>> = (0..f)
            .map(|k| {
                let d_k = Vector4::new(
                    direction[4 * k],
                    direction[4 * k + 1],
                    direction[4 * k + 2],
                    direction[4 * k + 3],
                );
                duals[k] + e * d_k
            })
            .collect();
        let poly = Polytope4D::from_f64(perturbed).ok()?;
        let cap = ehz_capacity(&poly).ok()?.capacity();
        let vol = volume(&poly);
        if vol <= 0.0 {
            return None;
        }
        Some(cap * cap / (2.0 * vol))
    };

    let sys_plus = eval(1.0)?;
    let sys_minus = eval(-1.0)?;
    Some((sys_plus + sys_minus - 2.0 * sys_base) / (eps * eps))
}

#[derive(Debug, Serialize)]
struct RandomDirectionRow {
    direction_index: usize,
    /// Median curvature over EPSILON_RANDOM values.
    curvature: f64,
    /// Individual curvature ratios at each epsilon.
    curvatures_by_eps: Vec<f64>,
    /// The random direction as coefficients in the flat basis (for reproducibility).
    flat_basis_coefficients: Vec<f64>,
    time_ms: f64,
}

fn run_phase3(
    polytope: &Polytope4D,
    sys_base: f64,
    flat_directions: &[Vec<f64>],
    writer: &mut BufWriter<File>,
) {
    let n_flat = flat_directions.len();
    let dim = flat_directions[0].len();
    let mut rng = ChaCha8Rng::seed_from_u64(RANDOM_SEED);

    println!(
        "\n  Sampling {} random directions in {}D flat subspace, {} ε values each",
        N_RANDOM_DIRECTIONS,
        n_flat,
        EPSILON_RANDOM.len(),
    );

    let mut n_negative = 0;
    let mut n_ambiguous = 0;
    let mut n_positive = 0;
    let mut worst_curvature = f64::NEG_INFINITY;

    for dir_idx in 0..N_RANDOM_DIRECTIONS {
        let t_dir = Instant::now();
        let (direction, normalized_coeffs) = random_flat_direction(flat_directions, &mut rng);

        // Compute curvature at each epsilon
        let mut curvatures: Vec<f64> = Vec::new();
        for &eps in EPSILON_RANDOM {
            if let Some(curv) = curvature_at_epsilon(polytope, &direction, eps, sys_base) {
                curvatures.push(curv);
            }
        }

        // Median curvature
        let median = if curvatures.is_empty() {
            f64::NAN
        } else {
            let mut sorted = curvatures.clone();
            sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
            sorted[sorted.len() / 2]
        };

        if median < -1e-6 {
            n_negative += 1;
        } else if median > 1e-6 {
            n_positive += 1;
        } else {
            n_ambiguous += 1;
        }
        if median > worst_curvature {
            worst_curvature = median;
        }

        let row = RandomDirectionRow {
            direction_index: dir_idx,
            curvature: median,
            curvatures_by_eps: curvatures,
            flat_basis_coefficients: normalized_coeffs,
            time_ms: t_dir.elapsed().as_secs_f64() * 1000.0,
        };
        serde_json::to_writer(&mut *writer, &row).expect("write random row");
        writeln!(writer).expect("newline");

        if dir_idx % 20 == 19 {
            println!(
                "  {}/{}: {} negative, {} ambiguous, {} positive, worst={:.4e}",
                dir_idx + 1,
                N_RANDOM_DIRECTIONS,
                n_negative,
                n_ambiguous,
                n_positive,
                worst_curvature
            );
        }
    }

    println!("\n  Summary: {n_negative} negative, {n_ambiguous} ambiguous, {n_positive} positive");
    println!("  Worst (most positive) curvature: {worst_curvature:.4e}");
    if n_positive == 0 {
        println!(
            "  → No positive curvature found among {} random directions",
            N_RANDOM_DIRECTIONS
        );
    } else {
        println!(
            "  → WARNING: {} directions with positive curvature!",
            n_positive
        );
    }
}

#[derive(Debug, Clone, Copy)]
struct Args {
    smoke: bool,
}

fn print_usage() {
    eprintln!(
        r#"Usage: hko-second-order [options]

Optional flags:
  --help, -h          Show this help message and exit.
  --smoke              Run smoke mode and exit after phase 1 probe."#
    );
}

fn usage_error(message: String) -> ! {
    eprintln!("error: {message}\n");
    print_usage();
    std::process::exit(2);
}

fn parse_args() -> Args {
    let mut args = Args { smoke: false };

    for arg in std::env::args().skip(1) {
        match arg.as_str() {
            "--help" | "-h" => {
                print_usage();
                std::process::exit(0);
            }
            "--smoke" => {
                args.smoke = true;
            }
            other => usage_error(format!("unknown argument: {other}")),
        }
    }

    args
}

// ============================================================================
// Main
// ============================================================================

fn main() {
    let t0 = Instant::now();
    let base_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let out_dir = base_dir.join("second-order");
    let args = parse_args();
    let smoke = args.smoke;

    println!("═══════════════════════════════════════════════════════════");
    println!("Second-order analysis of flat directions at HKO2024");
    println!("═══════════════════════════════════════════════════════════\n");

    // Load HKO2024
    let known = known_polytopes::hko_pentagon();
    let polytope = &known.polytope;
    println!("HKO2024: F={}, known sys≈{:.6}", polytope.facet_count(), {
        let v = volume(polytope);
        known.capacity * known.capacity / (2.0 * v)
    });

    // Phase 1: Gradient matrix and flat directions
    println!("\n--- Phase 1: Gradient matrix and flat directions ---");
    let t_phase1 = Instant::now();
    let (mut base_row, flat_directions) = run_phase1(polytope);
    base_row.time_phase1_ms = t_phase1.elapsed().as_secs_f64() * 1000.0;
    println!("  Phase 1 time: {:.0}ms", base_row.time_phase1_ms);

    // Cross-check: sys_base matches known
    let sys_diff =
        (base_row.sys_base - known.capacity * known.capacity / (2.0 * base_row.volume_base)).abs();
    assert!(
        sys_diff < 1e-8,
        "sys_base mismatch: computed={:.10}, expected={:.10}",
        base_row.sys_base,
        known.capacity * known.capacity / (2.0 * base_row.volume_base)
    );

    std::fs::create_dir_all(&out_dir).expect("create output dir");

    if smoke {
        if let Some(direction) = flat_directions.first() {
            let eps = EPSILON_GRID[0];
            let curv = curvature_at_epsilon(polytope, direction, eps, base_row.sys_base)
                .expect("smoke curvature probe failed");
            println!("  Smoke curvature at ε={eps:.1e}: {curv:.6e}");
        } else {
            println!("\n  Smoke mode: no flat directions, exiting after phase 1.");
        }
        println!("\n═══════════════════════════════════════════════════════════");
        println!("Total time: {:.1}s", t0.elapsed().as_secs_f64());
        println!("═══════════════════════════════════════════════════════════");
        return;
    }

    // Write base JSONL
    let base_path = out_dir.join("second-order-base.jsonl");
    let base_file = File::create(&base_path).expect("create base JSONL");
    let mut base_writer = BufWriter::new(base_file);
    serde_json::to_writer(&mut base_writer, &base_row).expect("write base row");
    writeln!(base_writer).expect("newline");
    base_writer.flush().expect("flush base");
    println!("  Wrote {}", base_path.display());

    // Phase 2: Finite-difference curves
    if flat_directions.is_empty() {
        println!("\n  No flat directions — 0 ∈ interior of conv(gradients).");
        println!("  HKO2024 is a strict first-order local max. No second-order analysis needed.");
    } else {
        println!(
            "\n--- Phase 2: Finite-difference curves along {} flat directions ---",
            flat_directions.len()
        );
        let t_phase2 = Instant::now();

        let curves_path = out_dir.join("second-order-curves.jsonl");
        let curves_file = File::create(&curves_path).expect("create curves JSONL");
        let mut curves_writer = BufWriter::new(curves_file);

        run_phase2(
            polytope,
            base_row.sys_base,
            &flat_directions,
            &mut curves_writer,
        );

        curves_writer.flush().expect("flush curves");
        let phase2_time = t_phase2.elapsed().as_secs_f64();
        println!("  Phase 2 time: {phase2_time:.1}s");
        println!("  Wrote {}", curves_path.display());

        // Phase 3: Random directions for negative-definiteness check
        println!(
            "\n--- Phase 3: Random directions in flat subspace ({} samples) ---",
            N_RANDOM_DIRECTIONS
        );
        let t_phase3 = Instant::now();

        let random_path = out_dir.join("second-order-random.jsonl");
        let random_file = File::create(&random_path).expect("create random JSONL");
        let mut random_writer = BufWriter::new(random_file);

        run_phase3(
            polytope,
            base_row.sys_base,
            &flat_directions,
            &mut random_writer,
        );

        random_writer.flush().expect("flush random");
        let phase3_time = t_phase3.elapsed().as_secs_f64();
        println!("  Phase 3 time: {phase3_time:.1}s");
        println!("  Wrote {}", random_path.display());
    }

    let total = t0.elapsed().as_secs_f64();
    println!("\n═══════════════════════════════════════════════════════════");
    println!("Total time: {total:.1}s");
    println!("═══════════════════════════════════════════════════════════");
}

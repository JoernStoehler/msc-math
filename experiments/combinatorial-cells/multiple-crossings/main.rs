//! Multiple Crossings: multi-boundary traversal along gradient and random directions.
//!
//! Location: experiments/combinatorial-cells/multiple-crossings/main.rs
//!
//! Walks along a direction in dual-vertex space, iteratively stepping past each
//! combinatorial boundary for a total distance budget. Tracks sys at each step to
//! measure whether the gradient direction maintains improvement across multiple
//! boundary crossings.
//!
//! For each polytope: gradient sweep, neg-gradient sweep, 2 dense random sweeps.
//!
//! Split from combinatorial-structure (Pass 4).
//!
//! Input Artifacts: experiments/combinatorial-cells/polytopes.jsonl (owned cache)
//! Filter: F <= 10 (HK2017 is exponential in F)
//! Output Artifacts: experiments/combinatorial-cells/multiple-crossings/combinatorial-boundaries-sweep.jsonl

use exp_combinatorial_cells::euclidean_volume_f64;
use exp_combinatorial_cells::{
    compute_step_bound_detailed, ehz_capacity_instrumented, name_from_record, EventType,
};
use nalgebra::Vector4;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use rand_distr::{Distribution, StandardNormal};
use serde::Serialize;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use std::time::Instant;
use symplectic::database;
use symplectic::derivatives::{capacity_derivatives_a_from_kkt_result, volume_derivatives_a};
use symplectic::geom::polytope::Polytope4D;
use symplectic::kkt::saddle_point_solver::solve_kkt_for_dual_vertices;

// ============================================================================
// Configuration
// ============================================================================

/// Maximum facet count to process (HK2017 cost is exponential).
const MAX_FACET_COUNT: usize = 10;

/// Maximum step size cap (prevents infinite steps when no combinatorial bound exists).
/// Typical boundary distances are O(0.01)-O(1); 100.0 is well beyond any real boundary.
const MAX_STEP_SIZE: f64 = 100.0;

/// Numerical zero threshold for rates and slacks.
/// Set near machine epsilon (~1e-16); guards against treating f64 noise as a
/// meaningful direction or rate.
const EPS_NUMERICAL_ZERO: f64 = 1e-15;

/// Absolute floor for epsilon values, prevents sub-machine-epsilon perturbations.
const EPS_FLOOR: f64 = 1e-8;

/// Random seed for reproducibility.
const SEED: u64 = 42;

/// Distance budget for multi-boundary sweep.
/// Typical gradient t_max is O(0.1); 1.0 is ~10x a typical step.
/// If changed: larger budgets cross more boundaries but risk accumulating
/// numerical error from repeated polytope reconstruction.
const SWEEP_BUDGET: f64 = 1.0;

/// Step-over epsilon for multi-boundary sweep: fraction of t_max to step past boundary.
const SWEEP_STEP_FRACTION: f64 = 1e-3;

// ============================================================================
// Output schema
// ============================================================================

/// Multi-boundary sweep row (one row per polytope per direction).
#[derive(Debug, Serialize)]
struct SweepRow {
    polytope_name: String,
    facet_count: usize,
    direction_type: String,
    /// Total distance budget for the sweep.
    budget: f64,
    /// Number of boundaries crossed before budget or failure.
    n_boundaries: usize,
    /// Cumulative distances at each boundary.
    boundary_distances: Vec<f64>,
    /// Event types at each boundary.
    event_types: Vec<String>,
    /// sys value after each boundary crossing (NaN if EHZ failed).
    sys_values: Vec<f64>,
    /// sys at start of sweep.
    sys_start: f64,
    /// Whether the sweep ended due to construction failure (vs budget exhausted).
    ended_by_failure: bool,
    /// Failure reason if ended_by_failure is true.
    failure_reason: String,
    /// Total distance traveled.
    total_distance: f64,
}

// ============================================================================
// Database helpers
// ============================================================================

// ============================================================================
// Enriched step-bound computation in a-space
// ============================================================================

// ============================================================================
// Sensitivity computation
// ============================================================================

/// Compute d(sys)/d(a_k) for all facets.
/// [lem:sys-gradient-a] gradient of sys in dual vertices
///
/// sys = c^2/(2V), so by the quotient rule:
///   d(sys)/d(a_k) = (c * dc/d(a_k) - sys * dV/d(a_k)) / V
fn compute_sys_gradient_a(
    polytope: &Polytope4D,
    vol: f64,
    cap: f64,
    sys: f64,
    kkt: &symplectic::kkt::saddle_point_solver::KktResult,
    perm: &[usize],
) -> Vec<Vector4<f64>> {
    let d_vol_a = volume_derivatives_a(
        polytope.dual_vertices_f64(),
        polytope.vertices_f64(),
        polytope.incidence(),
    )
    .expect("combinatorial-cell polytope has valid finite geometry");
    let d_cap_a = capacity_derivatives_a_from_kkt_result(polytope.dual_vertices_f64(), perm, kkt);

    d_vol_a
        .iter()
        .zip(d_cap_a.iter())
        .map(|(dv, dc)| (cap * dc - sys * dv) / vol)
        .collect()
}

/// Compute sys for a polytope using the default root capacity wrapper.
/// Returns (sys, capacity, volume, best_perm, kkt).
fn compute_sys(
    polytope: &Polytope4D,
) -> Option<(
    f64,
    f64,
    f64,
    Vec<usize>,
    symplectic::kkt::saddle_point_solver::KktResult,
)> {
    let vol = euclidean_volume_f64(polytope.vertices(), polytope.incidence());
    if vol <= 0.0 {
        return None;
    }

    let ehz = exp_combinatorial_cells::capacity_auto(polytope).ok()?;

    let cap = ehz.capacity();
    if !cap.is_finite() || cap <= 0.0 {
        return None;
    }

    let perm = ehz.best_sigma().to_vec();
    let dual_vertices = polytope.dual_vertices_f64();
    let kkt = solve_kkt_for_dual_vertices(dual_vertices, &perm).feasible()?;
    let sys = cap * cap / (2.0 * vol);

    if sys.is_finite() {
        Some((sys, cap, vol, perm, kkt))
    } else {
        None
    }
}

// ============================================================================
// Multi-boundary sweep
// ============================================================================

/// Walk along a direction, iteratively stepping past each boundary.
/// Computes sys at each step to track whether optimization direction maintains improvement.
/// Returns the sweep row with all boundaries encountered.
fn multi_boundary_sweep(
    start_duals: &[Vector4<f64>],
    direction: &[Vector4<f64>],
    budget: f64,
    sys_start: f64,
    polytope_name: &str,
    direction_type: &str,
) -> SweepRow {
    let mut current_duals = start_duals.to_vec();
    let mut total_distance = 0.0;
    let mut boundary_distances = Vec::new();
    let mut event_types = Vec::new();
    let mut sys_values = Vec::new();
    let mut ended_by_failure = false;
    let mut failure_reason = String::new();

    loop {
        // Build polytope at current position
        let poly = match Polytope4D::from_f64(current_duals.clone()) {
            Ok(p) => p,
            Err(e) => {
                ended_by_failure = true;
                failure_reason = format!("polytope construction: {e}");
                break;
            }
        };

        // Find next boundary
        let boundary =
            compute_step_bound_detailed(&poly, direction, EPS_NUMERICAL_ZERO, MAX_STEP_SIZE);

        if matches!(boundary.event, EventType::Unbounded) {
            break;
        }

        let t = boundary.t_max;
        if total_distance + t > budget {
            break;
        }

        total_distance += t;
        boundary_distances.push(total_distance);
        event_types.push(boundary.event.name().to_string());

        // Step just past the boundary
        let step_over = (SWEEP_STEP_FRACTION * t).max(EPS_FLOOR);
        let t_step = t + step_over;
        total_distance += step_over;

        // Update current duals
        for (a, d) in current_duals.iter_mut().zip(direction.iter()) {
            *a += t_step * d;
        }

        // Compute sys at this position (cheap attempt, NaN on failure)
        let sys_here = Polytope4D::from_f64(current_duals.clone())
            .ok()
            .and_then(|p| compute_sys(&p))
            .map(|(s, _, _, _, _)| s)
            .unwrap_or(f64::NAN);
        sys_values.push(sys_here);
    }

    SweepRow {
        polytope_name: polytope_name.to_string(),
        facet_count: start_duals.len(),
        direction_type: direction_type.to_string(),
        budget,
        n_boundaries: boundary_distances.len(),
        boundary_distances,
        event_types,
        sys_values,
        sys_start,
        ended_by_failure,
        failure_reason,
        total_distance,
    }
}

// ============================================================================
// Main
// ============================================================================

fn main() {
    let t0 = Instant::now();
    let base_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut rng = ChaCha8Rng::seed_from_u64(SEED);

    println!("Combinatorial Sweep: multi-boundary traversal\n");

    // =========================================================================
    // Load starting polytopes from database
    // =========================================================================

    println!("Loading starting polytopes from owned cache (F <= {MAX_FACET_COUNT})...");

    let owned_db_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("polytopes.jsonl");
    let db = database::load_many(&[owned_db_path.as_path()]).expect("failed to load database");

    let mut polytopes: Vec<(String, Polytope4D)> = Vec::new();

    for (idx, (_, record)) in db.iter().enumerate() {
        let f = record.dual_vertices_rational.len();
        if f > MAX_FACET_COUNT {
            continue;
        }
        let p = match record.to_polytope() {
            Ok(p) => p,
            Err(e) => {
                eprintln!("  db entry {idx}: reconstruction failed: {e}");
                continue;
            }
        };
        let name = name_from_record(record, idx);
        polytopes.push((name, p));
    }

    let n_polytopes = polytopes.len();
    println!("  {n_polytopes} polytopes loaded from database (F <= {MAX_FACET_COUNT})\n");

    if n_polytopes == 0 {
        eprintln!("ERROR: No polytopes in database. Run sys-random-sample and sys-random-product-sample first.");
        std::process::exit(1);
    }

    // =========================================================================
    // Open output file
    // =========================================================================

    let out_dir = base_dir.join("multiple-crossings");
    let sweep_path = out_dir.join("combinatorial-boundaries-sweep.jsonl");
    let sweep_file = File::create(&sweep_path)
        .unwrap_or_else(|err| panic!("create sweep JSONL {}: {err}", sweep_path.display()));
    let mut sweep_writer = BufWriter::new(sweep_file);

    // =========================================================================
    // Process each polytope
    // =========================================================================

    let mut total_sweep = 0usize;
    let mut n_skipped = 0usize;

    for (idx, (name, polytope)) in polytopes.iter().enumerate() {
        let t_poly = Instant::now();
        let f = polytope.facet_count();
        let duals = polytope.dual_vertices_f64();

        // =====================================================================
        // Base computation: instrumented EHZ for gradient
        // =====================================================================

        let base = (|| {
            let instrumented = ehz_capacity_instrumented(polytope)?;
            let vol = euclidean_volume_f64(polytope.vertices(), polytope.incidence());
            if vol <= 0.0 {
                return None;
            }
            let cap = instrumented.capacity;
            let sys = cap * cap / (2.0 * vol);
            let perm = instrumented.best_permutation;
            let dual_vertices = polytope.dual_vertices_f64();
            let kkt = solve_kkt_for_dual_vertices(dual_vertices, &perm).feasible()?;
            Some((cap, vol, sys, perm, kkt))
        })();

        let (cap, vol, sys, perm, kkt) = match base {
            Some(t) => t,
            None => {
                n_skipped += 1;
                continue;
            }
        };

        let d_sys_a = compute_sys_gradient_a(polytope, vol, cap, sys, &kkt, &perm);

        // =====================================================================
        // Multi-boundary sweeps: gradient + neg-gradient + 2 dense random
        // =====================================================================

        // Gradient sweep
        let grad_norm: f64 = d_sys_a.iter().map(|v| v.norm_squared()).sum::<f64>().sqrt();
        if grad_norm > EPS_NUMERICAL_ZERO {
            let grad_dir: Vec<Vector4<f64>> = d_sys_a.iter().map(|v| v / grad_norm).collect();
            let row = multi_boundary_sweep(duals, &grad_dir, SWEEP_BUDGET, sys, name, "gradient");
            serde_json::to_writer(&mut sweep_writer, &row).unwrap();
            writeln!(sweep_writer).unwrap();
            total_sweep += 1;

            // Negative gradient
            let neg_dir: Vec<Vector4<f64>> = grad_dir.iter().map(|v| -v).collect();
            let row =
                multi_boundary_sweep(duals, &neg_dir, SWEEP_BUDGET, sys, name, "neg_gradient");
            serde_json::to_writer(&mut sweep_writer, &row).unwrap();
            writeln!(sweep_writer).unwrap();
            total_sweep += 1;
        }

        // 2 dense random sweeps
        for i in 0..2 {
            let raw: Vec<Vector4<f64>> = (0..f)
                .map(|_| {
                    Vector4::new(
                        StandardNormal.sample(&mut rng),
                        StandardNormal.sample(&mut rng),
                        StandardNormal.sample(&mut rng),
                        StandardNormal.sample(&mut rng),
                    )
                })
                .collect();
            let norm: f64 = raw.iter().map(|v| v.norm_squared()).sum::<f64>().sqrt();
            if norm > EPS_NUMERICAL_ZERO {
                let dir: Vec<Vector4<f64>> = raw.iter().map(|v| v / norm).collect();
                let row = multi_boundary_sweep(
                    duals,
                    &dir,
                    SWEEP_BUDGET,
                    sys,
                    name,
                    &format!("dense_random_{i}"),
                );
                serde_json::to_writer(&mut sweep_writer, &row).unwrap();
                writeln!(sweep_writer).unwrap();
                total_sweep += 1;
            }
        }

        // =====================================================================
        // Progress reporting
        // =====================================================================

        let elapsed = t_poly.elapsed().as_secs_f64();
        if (idx + 1) % 10 == 0 || idx + 1 == n_polytopes {
            println!(
                "  [{}/{}] {}: F={}, {:.1}s",
                idx + 1,
                n_polytopes,
                name,
                f,
                elapsed
            );
        }
    }

    // =========================================================================
    // Flush and report
    // =========================================================================

    sweep_writer.flush().unwrap();

    let total_time = t0.elapsed().as_secs_f64();
    println!("\nDone in {total_time:.1}s.");
    println!("  Sweep rows: {total_sweep}");
    if n_skipped > 0 {
        println!("  Skipped:        {n_skipped} (base computation failed)");
    }
}

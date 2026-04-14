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
//! Input: experiments/combinatorial-cells/polytopes.jsonl (owned cache)
//! Filter: F <= 10 (HK2017 is exponential in F)
//! Output: experiments/combinatorial-cells/multiple-crossings/combinatorial-boundaries-sweep.jsonl

use nalgebra::{Matrix4, Vector4};
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use rand_distr::{Distribution, StandardNormal};
use serde::Serialize;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use std::time::Instant;
use symplectic::database::{self, PolytopeRecord, Source};
use symplectic::algorithms::facet_adjacency::{build_transition_matrix, is_feasible_cycle};
use symplectic::algorithms::hk2017::combinations;
use symplectic::algorithms::hk2017::permutations::for_each_cyclic_permutation;
use symplectic::derivatives::{capacity_derivatives_a, volume_derivatives_a};
use symplectic::geom::polytope::Polytope4D;
use symplectic::geom::skeleton::Skeleton;
use symplectic::geom::symplectic_form::omega0;
use symplectic::geom::volume::volume;
use symplectic::kkt::saddle_point_solver::{solve_kkt_for, KktOutcome, EPS_BETA_POSITIVE, EPS_Q_POSITIVE};

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
// Boundary event types
// ============================================================================

/// Classification of a combinatorial boundary event.
#[derive(Debug, Clone)]
enum EventType {
    /// A vertex's slack with respect to a non-incident facet reaches zero.
    IncidenceFlip { vertex_index: usize, new_facet: usize },
    /// sign(omega_0(a_i, a_j)) changes for ridge-adjacent facets i, j.
    OmegaFlip { facet_i: usize, facet_j: usize },
    /// |a_k + t*d_k| -> 0 (dual vertex degenerates).
    DualVertexDegen { facet: usize },
    /// t_max was capped at MAX_STEP_SIZE (no real boundary found).
    Unbounded,
}

impl EventType {
    fn name(&self) -> &'static str {
        match self {
            EventType::IncidenceFlip { .. } => "incidence_flip",
            EventType::OmegaFlip { .. } => "omega_flip",
            EventType::DualVertexDegen { .. } => "dual_vertex_degen",
            EventType::Unbounded => "unbounded",
        }
    }
}

/// Result of the enriched step-bound computation.
#[derive(Debug, Clone)]
struct BoundaryEvent {
    t_max: f64,
    event: EventType,
}

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

/// Derive a human-readable name from a database record's Source.
fn name_from_record(record: &PolytopeRecord, index: usize) -> String {
    match &record.source {
        Some(Source::Random { facet_count_target, attempt, .. }) => {
            format!("random_F{facet_count_target}_a{attempt}")
        }
        Some(Source::LagrangianProduct { n1, n2, .. }) => {
            format!("product_{n1}x{n2}_{index}")
        }
        Some(Source::Known { name }) => name.clone(),
        None => format!("polytope_{index}"),
    }
}

// ============================================================================
// Instrumented EHZ capacity -- collects ALL valid orbits
// ============================================================================

#[derive(Debug, Clone)]
struct ValidOrbit {
    action: f64,
    permutation: Vec<usize>,
}

struct InstrumentedResult {
    capacity: f64,
    best_permutation: Vec<usize>,
    n_valid_orbits: usize,
    /// Q_second_best - Q_best (action gap). f64::INFINITY if only one orbit.
    orbit_gap: f64,
}

/// Enumerate all valid orbits via HK2017, return best + orbit gap.
fn ehz_capacity_instrumented(polytope: &Polytope4D) -> Option<InstrumentedResult> {
    let f = polytope.facet_count();
    let adj = build_transition_matrix(polytope);

    let mut orbits: Vec<ValidOrbit> = Vec::new();

    for m in 2..=f {
        for subset in combinations(f, m) {
            for_each_cyclic_permutation(&subset, &mut |perm| {
                if !is_feasible_cycle(perm, &adj) {
                    return;
                }

                if let KktOutcome::Feasible(kkt_result) = solve_kkt_for(polytope, perm) {
                    let q_val = kkt_result.q_corrected;
                    if q_val <= EPS_Q_POSITIVE {
                        return;
                    }
                    let beta_min = kkt_result
                        .beta
                        .iter()
                        .cloned()
                        .fold(f64::INFINITY, f64::min);
                    if beta_min > EPS_BETA_POSITIVE {
                        orbits.push(ValidOrbit {
                            action: 0.5 / q_val,
                            permutation: perm.to_vec(),
                        });
                    }
                }
            });
        }
    }

    if orbits.is_empty() {
        return None;
    }

    orbits.sort_by(|a, b| a.action.partial_cmp(&b.action).unwrap());

    let best = orbits[0].clone();
    let n_valid = orbits.len();
    let orbit_gap = if orbits.len() >= 2 {
        orbits[1].action - orbits[0].action
    } else {
        f64::INFINITY
    };

    Some(InstrumentedResult {
        capacity: best.action,
        best_permutation: best.permutation.clone(),
        n_valid_orbits: n_valid,
        orbit_gap,
    })
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
fn compute_step_bound_detailed(
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
            let det_facets = vertex_facets;
            let a_mat = Matrix4::from_rows(&[
                duals[det_facets[0]].transpose(),
                duals[det_facets[1]].transpose(),
                duals[det_facets[2]].transpose(),
                duals[det_facets[3]].transpose(),
            ]);

            let a_inv = match a_mat.try_inverse() {
                Some(inv) => inv,
                None => continue,
            };

            let rhs = Vector4::new(
                direction[det_facets[0]].dot(v),
                direction[det_facets[1]].dot(v),
                direction[det_facets[2]].dot(v),
                direction[det_facets[3]].dot(v),
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
            let max_d = direction.iter().map(|dk| dk.norm()).fold(0.0f64, f64::max);
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
    let duals = polytope.dual_vertices_f64();

    let d_vol_a = volume_derivatives_a(polytope);
    let d_cap_a = capacity_derivatives_a(&kkt.beta, kkt.q_corrected, &kkt.mu, perm, duals);

    d_vol_a
        .iter()
        .zip(d_cap_a.iter())
        .map(|(dv, dc)| (cap * dc - sys * dv) / vol)
        .collect()
}

// ============================================================================
// Polytope construction at perturbed parameter
// ============================================================================

/// Construct a polytope at a'_k = a_k + t*d_k.
fn construct_at_t(
    duals: &[Vector4<f64>],
    direction: &[Vector4<f64>],
    t: f64,
) -> Option<Polytope4D> {
    let new_duals: Vec<Vector4<f64>> = duals
        .iter()
        .zip(direction.iter())
        .map(|(a, d)| a + t * d)
        .collect();
    Polytope4D::from_f64(new_duals).ok()
}

/// Compute sys for a polytope using standard (non-instrumented) EHZ.
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
    let vol = volume(polytope).ok()?;
    if vol <= 0.0 {
        return None;
    }

    let ehz = symplectic::algorithms::hk2017::ehz_capacity(polytope)?;

    let cap = ehz.result.capacity;
    if !cap.is_finite() || cap <= 0.0 {
        return None;
    }

    let perm = ehz.result.best_permutation;
    let kkt = solve_kkt_for(polytope, &perm).feasible()?;
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
        let boundary = compute_step_bound_detailed(&poly, direction);

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
        polytope_name: String::new(),
        facet_count: start_duals.len(),
        direction_type: String::new(),
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
    let db = database::load_many(&[owned_db_path.as_path()])
        .expect("failed to load database");

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
    let sweep_file =
        File::create(out_dir.join("combinatorial-boundaries-sweep.jsonl"))
            .expect("create sweep JSONL");
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
            let vol = volume(polytope).ok().filter(|&v| v > 0.0)?;
            let cap = instrumented.capacity;
            let sys = cap * cap / (2.0 * vol);
            let perm = instrumented.best_permutation;
            let kkt = solve_kkt_for(polytope, &perm).feasible()?;
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
            let mut row = multi_boundary_sweep(duals, &grad_dir, SWEEP_BUDGET, sys);
            row.polytope_name = name.clone();
            row.direction_type = "gradient".to_string();
            serde_json::to_writer(&mut sweep_writer, &row).unwrap();
            writeln!(sweep_writer).unwrap();
            total_sweep += 1;

            // Negative gradient
            let neg_dir: Vec<Vector4<f64>> = grad_dir.iter().map(|v| -v).collect();
            let mut row = multi_boundary_sweep(duals, &neg_dir, SWEEP_BUDGET, sys);
            row.polytope_name = name.clone();
            row.direction_type = "neg_gradient".to_string();
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
                let mut row = multi_boundary_sweep(duals, &dir, SWEEP_BUDGET, sys);
                row.polytope_name = name.clone();
                row.direction_type = format!("dense_random_{i}");
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

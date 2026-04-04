//! Cell Widths: per-facet cell width measurement in dual-vertex space.
//!
//! Location: crates/exp-combinatorial-cells/cell-widths/run.rs
//!
//! For polytopes K = {x : a_k . x <= 1}, the combinatorial type (vertex-facet incidence,
//! omega_0 sign pattern) is constant within open regions of dual-vertex space R^{4F}. This
//! experiment measures the cell width per facet by probing random S^3 directions in each
//! facet's R^4 subspace. No EHZ capacity computation needed -- only boundary detection.
//!
//! For each facet k, probe N_FACET_DIRS random S^3 directions in R^4_k.
//! Measures cell width per facet, anisotropy, orbit-facet narrowness.
//!
//! Split from combinatorial-structure (Pass 1).
//!
//! Input: data/polytopes.jsonl (polytope database)
//! Filter: F <= 10 (HK2017 is exponential in F)
//! Output: combinatorial-boundaries-profiling.jsonl

use database::{PolytopeRecord, Source};
use nalgebra::{Matrix4, Vector4};
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use rand_distr::{Distribution, StandardNormal};
use serde::Serialize;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use std::time::Instant;
use symplectic::algorithms::facet_adjacency::{build_transition_matrix, is_feasible_cycle};
use symplectic::algorithms::hk2017::combinations;
use symplectic::algorithms::hk2017::permutations::for_each_cyclic_permutation;
use symplectic::geom::polytope::Polytope4D;
use symplectic::geom::skeleton::Skeleton;
use symplectic::geom::symplectic_form::omega0;
use symplectic::kkt::saddle_point_solver::{solve_kkt_for, KktOutcome, EPS_BETA_POSITIVE, EPS_Q_POSITIVE};

// ============================================================================
// Configuration
// ============================================================================

/// Maximum facet count to process (HK2017 cost is exponential).
const MAX_FACET_COUNT: usize = 10;

/// Number of random S^3 directions per facet for cell profiling.
/// 10 directions in R^4 give reasonable coverage of S^3.
const N_FACET_DIRS: usize = 10;

/// Maximum step size cap (prevents infinite steps when no combinatorial bound exists).
/// Typical boundary distances are O(0.01)-O(1); 100.0 is well beyond any real boundary.
/// If changed: only affects the "unbounded" classification, not actual boundary detection.
const MAX_STEP_SIZE: f64 = 100.0;

/// Numerical zero threshold for rates and slacks.
/// Set near machine epsilon (~1e-16); guards against treating f64 noise as a
/// meaningful direction or rate. Used in step bounds and gradient checks.
/// If changed: values much larger risk missing real boundaries; much smaller risks
/// false positives from floating-point noise.
const EPS_NUMERICAL_ZERO: f64 = 1e-15;

/// Random seed for reproducibility.
const SEED: u64 = 42;

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
// Direction types
// ============================================================================

/// A probe direction in dual-vertex space R^{4F}.
#[derive(Debug, Clone)]
struct Direction {
    /// Type label for the JSONL output.
    dir_type: String,
    /// Index within the type.
    index: usize,
    /// Which facet this direction perturbs (None for global/dense directions).
    facet_index: Option<usize>,
    /// Direction vector: one Vector4 per facet. Step: a'_k(t) = a_k + t*d[k].
    d: Vec<Vector4<f64>>,
}

// ============================================================================
// Output schema
// ============================================================================

/// Per-facet cell profiling row.
#[derive(Debug, Serialize)]
struct ProfilingRow {
    polytope_name: String,
    facet_count: usize,
    facet_index: usize,
    facet_in_orbit: bool,
    direction_index: usize,
    t_max: f64,
    event_type: String,
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
// Direction construction
// ============================================================================

/// Build per-facet directions: N_FACET_DIRS random S^3 directions per facet.
/// Each direction is nonzero only in R^4_k for facet k.
fn build_facet_directions(f: usize, rng: &mut ChaCha8Rng) -> Vec<Direction> {
    let mut dirs = Vec::with_capacity(f * N_FACET_DIRS);
    for k in 0..f {
        for i in 0..N_FACET_DIRS {
            let raw: Vector4<f64> = Vector4::new(
                StandardNormal.sample(rng),
                StandardNormal.sample(rng),
                StandardNormal.sample(rng),
                StandardNormal.sample(rng),
            );
            let norm = raw.norm();
            if norm > EPS_NUMERICAL_ZERO {
                let mut d = vec![Vector4::zeros(); f];
                d[k] = raw / norm;
                dirs.push(Direction {
                    dir_type: "facet".to_string(),
                    index: i,
                    facet_index: Some(k),
                    d,
                });
            }
        }
    }
    dirs
}

// ============================================================================
// Main
// ============================================================================

fn main() {
    let t0 = Instant::now();
    let base_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut rng = ChaCha8Rng::seed_from_u64(SEED);

    println!("Combinatorial Profiling: per-facet cell width measurement\n");

    // =========================================================================
    // Load starting polytopes from database
    // =========================================================================

    println!("Loading starting polytopes from database (F <= {MAX_FACET_COUNT})...");

    let db_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../data/polytopes.jsonl");
    let db = database::load(&db_path).expect("failed to load database");

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

    let out_dir = base_dir.join("combinatorial-profiling");
    let profiling_file =
        File::create(out_dir.join("combinatorial-boundaries-profiling.jsonl"))
            .expect("create profiling JSONL");
    let mut profiling_writer = BufWriter::new(profiling_file);

    // =========================================================================
    // Process each polytope
    // =========================================================================

    let mut total_profiling = 0usize;
    let mut n_skipped = 0usize;

    for (idx, (name, polytope)) in polytopes.iter().enumerate() {
        let t_poly = Instant::now();
        let f = polytope.facet_count();

        // =====================================================================
        // Base computation: instrumented EHZ for orbit membership
        // =====================================================================

        let base = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let instrumented = ehz_capacity_instrumented(polytope)?;
            let perm = instrumented.best_permutation;
            Some((perm,))
        }));

        let (perm,) = match base {
            Ok(Some(t)) => t,
            Ok(None) | Err(_) => {
                n_skipped += 1;
                continue;
            }
        };

        // Which facets are in the optimal orbit?
        let orbit_facets: Vec<bool> = (0..f).map(|k| perm.contains(&k)).collect();

        // =====================================================================
        // Per-facet cell profiling (cheap, no EHZ)
        // =====================================================================

        let facet_dirs = build_facet_directions(f, &mut rng);

        for dir in &facet_dirs {
            let boundary = compute_step_bound_detailed(polytope, &dir.d);
            let k = dir.facet_index.unwrap();

            let row = ProfilingRow {
                polytope_name: name.clone(),
                facet_count: f,
                facet_index: k,
                facet_in_orbit: orbit_facets[k],
                direction_index: dir.index,
                t_max: boundary.t_max,
                event_type: boundary.event.name().to_string(),
            };

            serde_json::to_writer(&mut profiling_writer, &row).unwrap();
            writeln!(profiling_writer).unwrap();
            total_profiling += 1;
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

    profiling_writer.flush().unwrap();

    let total_time = t0.elapsed().as_secs_f64();
    println!("\nDone in {total_time:.1}s.");
    println!("  Profiling rows: {total_profiling}");
    if n_skipped > 0 {
        println!("  Skipped:        {n_skipped} (base computation failed)");
    }
}

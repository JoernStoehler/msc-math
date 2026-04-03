//! Combinatorial Boundaries: characterize combinatorial type cells in dual-vertex space.
//!
//! For polytopes K = {x : a_k · x ≤ 1}, the combinatorial type (vertex-facet incidence,
//! ω₀ sign pattern) is constant within open regions of dual-vertex space R^{4F}. This
//! experiment characterizes these cells: their shape (per-facet cross-sections), convexity,
//! relationship to the gradient and optimal orbit, and what happens at boundaries.
//!
//! Three passes:
//! - Pass 1 (per-facet profiling): for each facet k, probe 10 random S³ directions in R⁴_k.
//!   Measures cell width per facet, anisotropy, orbit-facet narrowness. No EHZ needed.
//! - Pass 2 (global probes): gradient + neg-gradient + 5 dense random directions.
//!   Full boundary anatomy + crossing evaluation + gradient measurement.
//! - Pass 3 (convexity testing): sample direction pairs from Pass 1, check if midpoint
//!   of two interior points has the same combinatorial type.
//!
//! Input: data/polytopes.jsonl (polytope database, populated by random-sweep + random-product-sweep)
//! Filter: F ≤ 10 (HK2017 is exponential in F)

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

/// Number of random S³ directions per facet for cell profiling (Pass 1).
/// 10 directions in R⁴ give reasonable coverage of S³.
const N_FACET_DIRS: usize = 10;

/// Number of dense random directions for global probes (Pass 2).
const N_GLOBAL_DENSE: usize = 5;

/// Number of direction pairs sampled per polytope for convexity testing (Pass 3).
/// Mix of same-facet and cross-facet pairs.
const N_CONVEXITY_PAIRS: usize = 20;

/// Maximum step size cap (prevents infinite steps when no combinatorial bound exists).
/// Typical boundary distances are O(0.01)–O(1); 100.0 is well beyond any real boundary.
/// If changed: only affects the "unbounded" classification, not actual boundary detection.
const MAX_STEP_SIZE: f64 = 100.0;

/// Numerical zero threshold for rates and slacks.
/// Set near machine epsilon (~1e-16); guards against treating f64 noise as a
/// meaningful direction or rate. Used in step bounds and gradient checks.
/// If changed: values much larger risk missing real boundaries; much smaller risks
/// false positives from floating-point noise.
const EPS_NUMERICAL_ZERO: f64 = 1e-15;

/// Epsilon fractions for crossing evaluation: proportional to t_max.
/// Geometric progression from 1e-4 to 0.1 of t_max. Start close to the boundary
/// (1e-4 × t_max) for accuracy; fall back to larger fractions if construction or
/// EHZ fails near the boundary. Floor at 1e-8 prevents sub-machine-epsilon steps.
/// Validated: 100% crossing success rate at 873/873 boundaries with this sequence.
/// If changed: smaller fractions improve accuracy but risk numerical failure;
/// larger fractions degrade continuity measurements.
const CROSSING_EPS_FRACTIONS: &[f64] = &[1e-4, 1e-3, 1e-2, 5e-2, 0.1];

/// Fraction of t_max used for the "before boundary" evaluation point.
/// 1e-4 × t_max places us close enough that sys_before ≈ sys(original) while
/// being safely on the pre-boundary side.
const BEFORE_EPS_FRACTION: f64 = 1e-4;

/// Absolute floor for epsilon values, prevents sub-machine-epsilon perturbations.
const EPS_FLOOR: f64 = 1e-8;

/// Random seed for reproducibility.
const SEED: u64 = 42;

/// Whether to run gradient measurement across boundaries (Pass 2 Phase 3).
const RUN_GRADIENT: bool = true;

/// Distance budget for multi-boundary sweep (Pass 4).
/// Typical gradient t_max is O(0.1); 1.0 is ~10× a typical step.
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
    /// sign(ω₀(a_i, a_j)) changes for ridge-adjacent facets i, j.
    OmegaFlip { facet_i: usize, facet_j: usize },
    /// |a_k + t·d_k| → 0 (dual vertex degenerates).
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
    /// Direction vector: one Vector4 per facet. Step: a'_k(t) = a_k + t·d[k].
    d: Vec<Vector4<f64>>,
}

// ============================================================================
// Output schemas
// ============================================================================

/// Pass 1: per-facet cell profiling row.
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

/// Pass 2: boundary anatomy row (one per global direction).
#[derive(Debug, Serialize)]
struct AnatomyRow {
    polytope_name: String,
    source_dataset: String,
    facet_count: usize,
    sys: f64,
    volume: f64,
    capacity: f64,
    orbit_perm: String,
    orbit_gap: f64,
    n_valid_orbits: usize,

    direction_type: String,
    direction_index: usize,

    t_max: f64,
    event_type: String,
    event_vertex: Option<usize>,
    event_facet_new: Option<usize>,
    event_facet_pair: Option<[usize; 2]>,
    event_facet_degen: Option<usize>,

    vertex_count: usize,
    all_vertices_simple: bool,

    time_ms: f64,
}

/// Pass 2: crossing evaluation row.
#[derive(Debug, Serialize)]
struct CrossingRow {
    polytope_name: String,
    facet_count: usize,
    direction_type: String,
    direction_index: usize,
    t_max: f64,
    event_type: String,

    eps_used: f64,

    sys_before: f64,
    capacity_before: f64,
    volume_before: f64,
    orbit_before: String,
    vertex_count_before: usize,

    sys_after: f64,
    capacity_after: f64,
    volume_after: f64,
    orbit_after: String,
    vertex_count_after: usize,
    construction_ok_after: bool,

    delta_sys: f64,
    orbit_changed: bool,
    vertex_count_changed: bool,
}

/// Pass 2: gradient crossing row.
#[derive(Debug, Serialize)]
struct GradientRow {
    polytope_name: String,
    facet_count: usize,
    direction_type: String,
    direction_index: usize,
    t_max: f64,
    event_type: String,

    gradient_norm_before: f64,
    gradient_dot_direction_before: f64,

    gradient_norm_after: f64,
    gradient_dot_direction_after: f64,

    gradient_norm_jump: f64,
    directional_deriv_jump: f64,
    gradient_angle_change_deg: f64,
}

/// Pass 3: convexity testing row.
#[derive(Debug, Serialize)]
struct ConvexityRow {
    polytope_name: String,
    facet_count: usize,
    dir1_facet: usize,
    dir1_index: usize,
    dir2_facet: usize,
    dir2_index: usize,
    t1_max: f64,
    t2_max: f64,
    midpoint_same_incidence: bool,
    midpoint_same_omega_signs: bool,
    midpoint_same_transitions: bool,
    midpoint_construction_ok: bool,
}

/// Pass 4: multi-boundary sweep row (one row per polytope per direction).
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

/// Derive a source dataset string from a database record's Source.
fn source_dataset_from_record(record: &PolytopeRecord) -> String {
    match &record.source {
        Some(Source::Random { .. }) => "random-sweep".to_string(),
        Some(Source::LagrangianProduct { .. }) => "random-product-sweep".to_string(),
        Some(Source::Known { .. }) => "known".to_string(),
        None => "unknown".to_string(),
    }
}

// ============================================================================
// Instrumented EHZ capacity — collects ALL valid orbits
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
/// Copied from hko-neighborhood/run.rs, simplified (we only need gap, not all orbits).
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
/// [lem:step-bound-incidence] incidence flip detection, [lem:step-bound-omega] ω₀ flip detection
///
/// For step a'_k(t) = a_k + t·d_k, the combinatorial type changes when:
/// 1. **Incidence flip:** a vertex's slack w.r.t. a non-incident facet reaches zero.
///    For simple vertex v determined by facets {j1,..,j4}, we have a_ji · v = 1.
///    Differentiating: d_ji · v + a_ji · dv/dt = 0 ⟹ dv/dt = −A_v⁻¹ (d_{det} · v)
///    where A_v = [a_{j1}; ...; a_{j4}] and (d_{det} · v)_i = d_{ji} · v.
///    For non-incident facet j: slack = 1 − a_j · v, rate = −d_j · v − a_j · dv/dt.
/// 2. **ω₀ flip:** sign(ω₀(a_i, a_j)) changes for ridge-adjacent facets.
///    ω₀(a_i(t), a_j(t)) is quadratic in t; exact roots via quadratic formula.
/// 3. **Dual vertex degeneration:** |a_k + t·d_k| → 0.
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

    // --- ω₀ sign preservation for ridge-adjacent pairs ---
    // sign(ω₀(n_i, n_j)) = sign(ω₀(a_i, a_j)) since |a_k| > 0 and ω₀ is bilinear.
    // Along the path a_k(t) = a_k + t·d_k, bilinearity gives:
    //   ω₀(a_i(t), a_j(t)) = c + b·t + a·t²
    // where c = ω₀(a_i, a_j), b = ω₀(d_i, a_j) + ω₀(a_i, d_j), a = ω₀(d_i, d_j).
    // Sign flips at the smallest positive root of this quadratic.
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

    // --- Dual vertex degeneration: |a_k + t·d_k| → 0 ---
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

/// Build per-facet directions: N_FACET_DIRS random S³ directions per facet.
/// Each direction is nonzero only in R⁴_k for facet k.
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

/// Build global directions: gradient + neg-gradient + N_GLOBAL_DENSE dense random.
fn build_global_directions(
    d_sys_a: &[Vector4<f64>],
    f: usize,
    rng: &mut ChaCha8Rng,
) -> Vec<Direction> {
    let mut dirs = Vec::new();

    // Gradient direction (normalized)
    let grad_norm: f64 = d_sys_a.iter().map(|v| v.norm_squared()).sum::<f64>().sqrt();
    if grad_norm > EPS_NUMERICAL_ZERO {
        let normalized: Vec<Vector4<f64>> = d_sys_a.iter().map(|v| v / grad_norm).collect();
        dirs.push(Direction {
            dir_type: "gradient".to_string(),
            index: 0,
            facet_index: None,
            d: normalized,
        });
    }

    // Negative gradient
    if grad_norm > EPS_NUMERICAL_ZERO {
        let neg: Vec<Vector4<f64>> = d_sys_a.iter().map(|v| -v / grad_norm).collect();
        dirs.push(Direction {
            dir_type: "neg_gradient".to_string(),
            index: 0,
            facet_index: None,
            d: neg,
        });
    }

    // Dense random directions (uniform on S^{4F-1})
    for i in 0..N_GLOBAL_DENSE {
        let raw: Vec<Vector4<f64>> = (0..f)
            .map(|_| {
                Vector4::new(
                    StandardNormal.sample(rng),
                    StandardNormal.sample(rng),
                    StandardNormal.sample(rng),
                    StandardNormal.sample(rng),
                )
            })
            .collect();
        let norm: f64 = raw.iter().map(|v| v.norm_squared()).sum::<f64>().sqrt();
        if norm > EPS_NUMERICAL_ZERO {
            let normalized: Vec<Vector4<f64>> = raw.iter().map(|v| v / norm).collect();
            dirs.push(Direction {
                dir_type: "dense_random".to_string(),
                index: i,
                facet_index: None,
                d: normalized,
            });
        }
    }

    dirs
}

// ============================================================================
// Sensitivity computation
// ============================================================================

/// Compute d(sys)/d(a_k) for all facets.
/// [lem:sys-gradient-a] gradient of sys in dual vertices
///
/// sys = c²/(2V), so by the quotient rule:
///   d(sys)/d(a_k) = (c · dc/d(a_k) - sys · dV/d(a_k)) / V
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

/// Construct a polytope at a'_k = a_k + t·d_k.
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
/// Wrapped in catch_unwind because the KKT solver can panic on
/// near-singular systems (pre-existing issue in saddle_point_solver).
fn compute_sys(
    polytope: &Polytope4D,
) -> Option<(
    f64,
    f64,
    f64,
    Vec<usize>,
    symplectic::kkt::saddle_point_solver::KktResult,
)> {
    std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        compute_sys_inner(polytope)
    })).ok()?
}

fn compute_sys_inner(
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

/// Format a permutation as a compact string, e.g., "[0,3,2,1]".
fn perm_to_string(perm: &[usize]) -> String {
    format!(
        "[{}]",
        perm.iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(",")
    )
}

// ============================================================================
// Phase 2: Crossing evaluation
// ============================================================================

/// Evaluate sys/orbit on both sides of a boundary.
fn evaluate_crossing(
    duals: &[Vector4<f64>],
    direction: &[Vector4<f64>],
    boundary: &BoundaryEvent,
) -> Option<CrossingRow> {
    match boundary.event {
        EventType::Unbounded | EventType::DualVertexDegen { .. } => return None,
        _ => {}
    }

    let t = boundary.t_max;
    if t <= 0.0 {
        return None;
    }

    let eps_before = (BEFORE_EPS_FRACTION * t).max(EPS_FLOOR);
    let poly_before = construct_at_t(duals, direction, t - eps_before)?;
    let (sys_b, cap_b, vol_b, perm_b, _) = compute_sys(&poly_before)?;
    let skel_before = Skeleton::compute(&poly_before);

    let mut eps_used = 0.0;
    let mut sys_a = f64::NAN;
    let mut cap_a = f64::NAN;
    let mut vol_a = f64::NAN;
    let mut perm_a: Vec<usize> = vec![];
    let mut vc_after = 0usize;
    let mut construction_ok = false;

    for &frac in CROSSING_EPS_FRACTIONS {
        let eps = (frac * t).max(EPS_FLOOR);
        if let Some(poly_after) = construct_at_t(duals, direction, t + eps) {
            if let Some((s, c, v, p, _)) = compute_sys(&poly_after) {
                let skel_after = Skeleton::compute(&poly_after);
                eps_used = eps;
                sys_a = s;
                cap_a = c;
                vol_a = v;
                perm_a = p;
                vc_after = skel_after.vertex_facets.len();
                construction_ok = true;
                break;
            }
        }
    }

    let orbit_b_str = perm_to_string(&perm_b);
    let orbit_a_str = perm_to_string(&perm_a);

    Some(CrossingRow {
        polytope_name: String::new(),
        facet_count: duals.len(),
        direction_type: String::new(),
        direction_index: 0,
        t_max: t,
        event_type: String::new(),
        eps_used,
        sys_before: sys_b,
        capacity_before: cap_b,
        volume_before: vol_b,
        orbit_before: orbit_b_str.clone(),
        vertex_count_before: skel_before.vertex_facets.len(),
        sys_after: sys_a,
        capacity_after: cap_a,
        volume_after: vol_a,
        orbit_after: orbit_a_str.clone(),
        vertex_count_after: vc_after,
        construction_ok_after: construction_ok,
        delta_sys: sys_a - sys_b,
        orbit_changed: orbit_b_str != orbit_a_str,
        vertex_count_changed: skel_before.vertex_facets.len() != vc_after,
    })
}

// ============================================================================
// Phase 3: Gradient crossing evaluation
// ============================================================================

/// Compute gradient info on both sides of a boundary.
fn evaluate_gradient_crossing(
    duals: &[Vector4<f64>],
    direction: &[Vector4<f64>],
    boundary: &BoundaryEvent,
) -> Option<GradientRow> {
    match boundary.event {
        EventType::Unbounded | EventType::DualVertexDegen { .. } => return None,
        _ => {}
    }

    let t = boundary.t_max;
    if t <= 0.0 {
        return None;
    }

    let eps_before = (BEFORE_EPS_FRACTION * t).max(EPS_FLOOR);
    let poly_before = construct_at_t(duals, direction, t - eps_before)?;
    let (sys_b, cap_b, vol_b, perm_b, kkt_b) = compute_sys(&poly_before)?;
    let d_sys_a_b =
        compute_sys_gradient_a(&poly_before, vol_b, cap_b, sys_b, &kkt_b, &perm_b);

    let mut d_sys_a_after = None;
    for &frac in CROSSING_EPS_FRACTIONS {
        let eps = (frac * t).max(EPS_FLOOR);
        if let Some(poly_after) = construct_at_t(duals, direction, t + eps) {
            if let Some((sys_a, cap_a, vol_a, perm_a, kkt_a)) = compute_sys(&poly_after) {
                d_sys_a_after = Some(compute_sys_gradient_a(
                    &poly_after, vol_a, cap_a, sys_a, &kkt_a, &perm_a,
                ));
                break;
            }
        }
    }

    let d_sys_a_a = d_sys_a_after?;

    let norm_b: f64 = d_sys_a_b.iter().map(|v| v.norm_squared()).sum::<f64>().sqrt();
    let norm_a: f64 = d_sys_a_a.iter().map(|v| v.norm_squared()).sum::<f64>().sqrt();

    let dd_b: f64 = d_sys_a_b
        .iter()
        .zip(direction.iter())
        .map(|(g, d)| g.dot(d))
        .sum();
    let dd_a: f64 = d_sys_a_a
        .iter()
        .zip(direction.iter())
        .map(|(g, d)| g.dot(d))
        .sum();

    let dot: f64 = d_sys_a_b
        .iter()
        .zip(d_sys_a_a.iter())
        .map(|(a, b)| a.dot(b))
        .sum();
    let angle_rad = if norm_b > EPS_NUMERICAL_ZERO && norm_a > EPS_NUMERICAL_ZERO {
        (dot / (norm_b * norm_a)).clamp(-1.0, 1.0).acos()
    } else {
        f64::NAN
    };

    Some(GradientRow {
        polytope_name: String::new(),
        facet_count: duals.len(),
        direction_type: String::new(),
        direction_index: 0,
        t_max: boundary.t_max,
        event_type: String::new(),
        gradient_norm_before: norm_b,
        gradient_dot_direction_before: dd_b,
        gradient_norm_after: norm_a,
        gradient_dot_direction_after: dd_a,
        gradient_norm_jump: norm_a - norm_b,
        directional_deriv_jump: dd_a - dd_b,
        gradient_angle_change_deg: angle_rad.to_degrees(),
    })
}

// ============================================================================
// Combinatorial type comparison (for convexity testing)
// ============================================================================

/// Combinatorial type signature: sorted vertex-facet incidence + omega signs.
/// Two polytopes with the same signature have the same combinatorial type.
struct CombinatorialType {
    /// Sorted list of sorted vertex-facet incidence vectors.
    vertex_facets: Vec<Vec<usize>>,
    /// Sorted list of (facet_i, facet_j, sign_positive) for ridge-adjacent pairs.
    omega_signs: Vec<(usize, usize, bool)>,
}

fn combinatorial_type(polytope: &Polytope4D) -> CombinatorialType {
    let skeleton = Skeleton::compute(polytope);
    let duals = polytope.dual_vertices_f64();

    let mut vf: Vec<Vec<usize>> = skeleton
        .vertex_facets
        .iter()
        .map(|facets| {
            let mut f = facets.clone();
            f.sort();
            f
        })
        .collect();
    vf.sort();

    let mut omega_signs: Vec<(usize, usize, bool)> = skeleton
        .ridges
        .iter()
        .map(|r| {
            let i = r.facets[0].min(r.facets[1]);
            let j = r.facets[0].max(r.facets[1]);
            let sign = omega0(&duals[i], &duals[j]) >= 0.0;
            (i, j, sign)
        })
        .collect();
    omega_signs.sort();

    CombinatorialType {
        vertex_facets: vf,
        omega_signs,
    }
}

fn same_incidence(a: &CombinatorialType, b: &CombinatorialType) -> bool {
    a.vertex_facets == b.vertex_facets
}

fn same_omega(a: &CombinatorialType, b: &CombinatorialType) -> bool {
    a.omega_signs == b.omega_signs
}

/// Compare transition matrices (vertex adjacency + ω₀ signs → directed facet graph).
/// This is what actually determines which Reeb orbits are feasible.
fn same_transitions(base: &Polytope4D, other: &Polytope4D) -> bool {
    let t1 = build_transition_matrix(base);
    let t2 = build_transition_matrix(other);
    t1 == t2
}


// ============================================================================
// Pass 4: Multi-boundary sweep
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

    println!("Combinatorial Boundaries: cell geometry characterization\n");

    // =========================================================================
    // Load starting polytopes from database
    // =========================================================================

    println!("Loading starting polytopes from database (F ≤ {MAX_FACET_COUNT})...");

    let db_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../data/polytopes.jsonl");
    let db = database::load(&db_path).expect("failed to load database");

    let mut polytopes: Vec<(String, String, Polytope4D)> = Vec::new();

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
        let source = source_dataset_from_record(record);
        polytopes.push((name, source, p));
    }

    let n_polytopes = polytopes.len();
    println!("  {n_polytopes} polytopes loaded from database (F ≤ {MAX_FACET_COUNT})\n");

    if n_polytopes == 0 {
        eprintln!("ERROR: No polytopes in database. Run base-random-sweep and base-random-product-sweep first.");
        std::process::exit(1);
    }

    // =========================================================================
    // Open output files
    // =========================================================================

    let out_dir = base_dir.join("combinatorial-boundaries");

    let profiling_file =
        File::create(out_dir.join("combinatorial-boundaries-profiling.jsonl")).expect("create profiling JSONL");
    let mut profiling_writer = BufWriter::new(profiling_file);

    let anatomy_file =
        File::create(out_dir.join("combinatorial-boundaries-anatomy.jsonl")).expect("create anatomy JSONL");
    let mut anatomy_writer = BufWriter::new(anatomy_file);

    let crossing_file =
        File::create(out_dir.join("combinatorial-boundaries-crossing.jsonl")).expect("create crossing JSONL");
    let mut crossing_writer = BufWriter::new(crossing_file);

    let mut gradient_writer = if RUN_GRADIENT {
        let f =
            File::create(out_dir.join("combinatorial-boundaries-gradient.jsonl")).expect("create gradient JSONL");
        Some(BufWriter::new(f))
    } else {
        None
    };

    let convexity_file =
        File::create(out_dir.join("combinatorial-boundaries-convexity.jsonl")).expect("create convexity JSONL");
    let mut convexity_writer = BufWriter::new(convexity_file);

    let sweep_file =
        File::create(out_dir.join("combinatorial-boundaries-sweep.jsonl")).expect("create sweep JSONL");
    let mut sweep_writer = BufWriter::new(sweep_file);

    // =========================================================================
    // Process each polytope
    // =========================================================================

    let mut total_profiling = 0usize;
    let mut total_anatomy = 0usize;
    let mut total_crossing = 0usize;
    let mut total_gradient = 0usize;
    let mut total_convexity = 0usize;
    let mut total_sweep = 0usize;
    let mut n_skipped = 0usize;

    for (idx, (name, source, polytope)) in polytopes.iter().enumerate() {
        let t_poly = Instant::now();
        let f = polytope.facet_count();
        let duals = polytope.dual_vertices_f64();

        // =====================================================================
        // Base computation: instrumented EHZ for orbit gap + gradient
        // =====================================================================

        // Base computation wrapped in catch_unwind — the KKT solver can panic
        // on near-singular systems (pre-existing issue in saddle_point_solver).
        let base = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let instrumented = ehz_capacity_instrumented(polytope)?;
            let vol = volume(polytope).ok().filter(|&v| v > 0.0)?;
            let cap = instrumented.capacity;
            let sys = cap * cap / (2.0 * vol);
            let perm = instrumented.best_permutation;
            let orbit_gap = instrumented.orbit_gap;
            let n_valid_orbits = instrumented.n_valid_orbits;
            let kkt = solve_kkt_for(polytope, &perm).feasible()?;
            Some((cap, vol, sys, perm, orbit_gap, n_valid_orbits, kkt))
        }));

        let (cap, vol, sys, perm, orbit_gap, n_valid_orbits, kkt) = match base {
            Ok(Some(t)) => t,
            Ok(None) | Err(_) => {
                n_skipped += 1;
                continue;
            }
        };

        let d_sys_a = compute_sys_gradient_a(polytope, vol, cap, sys, &kkt, &perm);

        let skeleton = Skeleton::compute(polytope);
        let vertex_count = skeleton.vertex_facets.len();
        let all_simple = skeleton.vertex_facets.iter().all(|vf| vf.len() == 4);
        let orbit_str = perm_to_string(&perm);

        // Which facets are in the optimal orbit?
        let orbit_facets: Vec<bool> = (0..f).map(|k| perm.contains(&k)).collect();

        // Base combinatorial type (for convexity testing)
        let base_type = combinatorial_type(polytope);

        // =====================================================================
        // Pass 1: Per-facet cell profiling (cheap, no EHZ)
        // =====================================================================

        let facet_dirs = build_facet_directions(f, &mut rng);

        // Store profiling results for convexity testing
        struct FacetProbe {
            facet: usize,
            dir_index: usize,
            t_max: f64,
            direction: Vec<Vector4<f64>>,
        }
        let mut probes: Vec<FacetProbe> = Vec::new();

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

            // Store for convexity testing (skip unbounded)
            if boundary.t_max < MAX_STEP_SIZE {
                probes.push(FacetProbe {
                    facet: k,
                    dir_index: dir.index,
                    t_max: boundary.t_max,
                    direction: dir.d.clone(),
                });
            }
        }

        // =====================================================================
        // Pass 2: Global probes (moderate, with EHZ)
        // =====================================================================

        let global_dirs = build_global_directions(&d_sys_a, f, &mut rng);

        for dir in &global_dirs {
            let t_dir = Instant::now();

            let boundary = compute_step_bound_detailed(polytope, &dir.d);

            let (ev_vertex, ev_facet_new, ev_facet_pair, ev_facet_degen) =
                match &boundary.event {
                    EventType::IncidenceFlip {
                        vertex_index,
                        new_facet,
                    } => (Some(*vertex_index), Some(*new_facet), None, None),
                    EventType::OmegaFlip { facet_i, facet_j } => {
                        (None, None, Some([*facet_i, *facet_j]), None)
                    }
                    EventType::DualVertexDegen { facet } => {
                        (None, None, None, Some(*facet))
                    }
                    EventType::Unbounded => (None, None, None, None),
                };

            let anatomy_row = AnatomyRow {
                polytope_name: name.clone(),
                source_dataset: source.clone(),
                facet_count: f,
                sys,
                volume: vol,
                capacity: cap,
                orbit_perm: orbit_str.clone(),
                orbit_gap,
                n_valid_orbits,
                direction_type: dir.dir_type.clone(),
                direction_index: dir.index,
                t_max: boundary.t_max,
                event_type: boundary.event.name().to_string(),
                event_vertex: ev_vertex,
                event_facet_new: ev_facet_new,
                event_facet_pair: ev_facet_pair,
                event_facet_degen: ev_facet_degen,
                vertex_count,
                all_vertices_simple: all_simple,
                time_ms: t_dir.elapsed().as_secs_f64() * 1000.0,
            };

            serde_json::to_writer(&mut anatomy_writer, &anatomy_row).unwrap();
            writeln!(anatomy_writer).unwrap();
            total_anatomy += 1;

            // Crossing evaluation
            if let Some(mut crossing_row) = evaluate_crossing(duals, &dir.d, &boundary) {
                crossing_row.polytope_name = name.clone();
                crossing_row.direction_type = dir.dir_type.clone();
                crossing_row.direction_index = dir.index;
                crossing_row.event_type = boundary.event.name().to_string();

                serde_json::to_writer(&mut crossing_writer, &crossing_row).unwrap();
                writeln!(crossing_writer).unwrap();
                total_crossing += 1;
            }

            // Gradient crossing
            if RUN_GRADIENT {
                if let Some(mut grad_row) =
                    evaluate_gradient_crossing(duals, &dir.d, &boundary)
                {
                    grad_row.polytope_name = name.clone();
                    grad_row.direction_type = dir.dir_type.clone();
                    grad_row.direction_index = dir.index;
                    grad_row.event_type = boundary.event.name().to_string();

                    if let Some(ref mut w) = gradient_writer {
                        serde_json::to_writer(&mut *w, &grad_row).unwrap();
                        writeln!(w).unwrap();
                        total_gradient += 1;
                    }
                }
            }
        }

        // =====================================================================
        // Pass 3: Convexity testing
        // =====================================================================

        if probes.len() >= 2 {
            let n_pairs = N_CONVEXITY_PAIRS.min(probes.len() * (probes.len() - 1) / 2);

            // Sample pairs: mix of same-facet and cross-facet
            let mut pairs_tested = 0;
            let mut pair_idx = 0;

            // Deterministic sampling: stride through all pairs
            let total_pairs = probes.len() * (probes.len() - 1) / 2;
            let stride = if total_pairs > n_pairs {
                total_pairs / n_pairs
            } else {
                1
            };

            'pairs: for i in 0..probes.len() {
                for j in (i + 1)..probes.len() {
                    if pair_idx % stride == 0 {
                        let p1 = &probes[i];
                        let p2 = &probes[j];

                        // Interior points: 0.5 * t_max along each direction
                        let t1 = 0.5 * p1.t_max;
                        let t2 = 0.5 * p2.t_max;

                        // Midpoint direction: 0.5 * (t1*d1 + t2*d2)
                        let mid_dir: Vec<Vector4<f64>> = p1
                            .direction
                            .iter()
                            .zip(p2.direction.iter())
                            .map(|(d1, d2)| 0.5 * (t1 * d1 + t2 * d2))
                            .collect();

                        // Construct polytope at midpoint (a + mid_dir, i.e. t=1)
                        let mut construction_ok = false;
                        let mut same_incidence_val = false;
                        let mut same_omega_val = false;
                        let mut same_transitions_val = false;

                        if let Some(mid_poly) = construct_at_t(duals, &mid_dir, 1.0) {
                            construction_ok = true;
                            let mid_type = combinatorial_type(&mid_poly);
                            same_incidence_val = same_incidence(&base_type, &mid_type);
                            same_omega_val = same_omega(&base_type, &mid_type);
                            same_transitions_val = same_transitions(polytope, &mid_poly);
                        }

                        let row = ConvexityRow {
                            polytope_name: name.clone(),
                            facet_count: f,
                            dir1_facet: p1.facet,
                            dir1_index: p1.dir_index,
                            dir2_facet: p2.facet,
                            dir2_index: p2.dir_index,
                            t1_max: p1.t_max,
                            t2_max: p2.t_max,
                            midpoint_same_incidence: same_incidence_val,
                            midpoint_same_omega_signs: same_omega_val,
                            midpoint_same_transitions: same_transitions_val,
                            midpoint_construction_ok: construction_ok,
                        };

                        serde_json::to_writer(&mut convexity_writer, &row).unwrap();
                        writeln!(convexity_writer).unwrap();
                        total_convexity += 1;
                        pairs_tested += 1;

                        if pairs_tested >= n_pairs {
                            break 'pairs;
                        }
                    }
                    pair_idx += 1;
                }
            }
        }

        // =====================================================================
        // Pass 4: Multi-boundary sweep (gradient + 2 dense random)
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

    profiling_writer.flush().unwrap();
    anatomy_writer.flush().unwrap();
    crossing_writer.flush().unwrap();
    if let Some(ref mut w) = gradient_writer {
        w.flush().unwrap();
    }
    convexity_writer.flush().unwrap();
    sweep_writer.flush().unwrap();

    let total_time = t0.elapsed().as_secs_f64();
    println!("\nDone in {total_time:.1}s.");
    println!("  Profiling rows: {total_profiling}");
    println!("  Anatomy rows:   {total_anatomy}");
    println!("  Crossing rows:  {total_crossing}");
    if RUN_GRADIENT {
        println!("  Gradient rows:  {total_gradient}");
    }
    println!("  Sweep rows:     {total_sweep}");
    println!("  Convexity rows: {total_convexity}");
    if n_skipped > 0 {
        println!("  Skipped:        {n_skipped} (base computation failed)");
    }
}

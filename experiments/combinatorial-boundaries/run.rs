//! Combinatorial Boundaries: characterize what happens at combinatorial type changes.
//!
//! For polytopes K = {x : a_k · x ≤ 1}, the combinatorial type (vertex-facet incidence,
//! ω₀ sign pattern) is constant within open regions of dual-vertex space. This experiment
//! characterizes the boundaries between these regions: what events occur, how sys and the
//! gradient behave across them, and how dense they are.
//!
//! Three phases:
//! - Phase 1 (anatomy): classify the first boundary event along each direction
//! - Phase 2 (crossing): measure sys/orbit before and after each boundary
//! - Phase 3 (gradient): measure gradient jump across each boundary
//!
//! Architecture:
//! 1. `cargo run --bin combinatorial_boundaries --release` generates datasets
//! 2. Writes to combinatorial-boundaries/*.jsonl
//! 3. Python script reads JSONL, produces figures
//!
//! Input: random-sweep/random-sweep.jsonl, random-product-sweep/random-product-sweep.jsonl
//! Filter: F ≤ 10 (HK2017 is exponential in F)

use nalgebra::{Matrix4, Vector4};
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use rand_distr::{Distribution, StandardNormal};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::time::Instant;
use symplectic::algorithms::hk2017::ehz_capacity;
use symplectic::derivatives::{capacity_derivatives_a, volume_derivatives_a};
use symplectic::geom::polytope::Polytope4D;
use symplectic::geom::skeleton::Skeleton;
use symplectic::geom::volume::volume;
use symplectic::kkt::saddle_point_solver::{solve_kkt_for, KktResult};

// ============================================================================
// Configuration
// ============================================================================

/// Maximum facet count to process (HK2017 cost is exponential).
const MAX_FACET_COUNT: usize = 10;

/// Number of random directions to probe per polytope.
/// 10 gives reasonable statistics; each costs only ~0.1ms for the step bound.
const N_RANDOM_DIRECTIONS: usize = 10;

/// Maximum step size cap (prevents infinite steps when no combinatorial bound exists).
const MAX_STEP_SIZE: f64 = 100.0;

/// Numerical zero threshold for rates and slacks.
const EPS_NUMERICAL_ZERO: f64 = 1e-15;

/// Random seed for reproducibility.
const SEED: u64 = 42;

/// Whether to run Phase 3 (gradient across boundaries).
/// Set to false if gradient-correctness hasn't validated the gradient formula yet.
const RUN_PHASE_3: bool = true;

// ============================================================================
// Boundary event types
// ============================================================================

/// Classification of a combinatorial boundary event.
#[derive(Debug, Clone)]
enum EventType {
    /// A vertex's slack with respect to a non-incident facet reaches zero.
    /// The vertex is about to gain a new incident facet.
    IncidenceFlip {
        vertex_index: usize,
        new_facet: usize,
    },
    /// ω₀(n_i, n_j) changes sign for ridge-adjacent facets i, j.
    /// Not triggered by h-only perturbations (normals fixed), but needed for (h,n) extension.
    #[allow(dead_code)]
    OmegaFlip {
        facet_i: usize,
        facet_j: usize,
    },
    /// Height h_k = 1/|a_k| approaches zero (facet goes to infinity).
    HeightZero {
        facet: usize,
    },
    /// t_max was capped at MAX_STEP_SIZE (no real boundary found).
    Unbounded,
}

impl EventType {
    fn name(&self) -> &'static str {
        match self {
            EventType::IncidenceFlip { .. } => "incidence_flip",
            EventType::OmegaFlip { .. } => "omega_flip",
            EventType::HeightZero { .. } => "height_zero",
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

/// A probe direction in the dual-vertex space, represented as height perturbations.
#[derive(Debug, Clone)]
struct Direction {
    /// Type label for the JSONL output.
    dir_type: String,
    /// Index within the type (0 for gradient, 0..N-1 for random, 0..F-1 for coordinate).
    index: usize,
    /// Height perturbation vector (one entry per facet).
    g_h: Vec<f64>,
}

// ============================================================================
// Output schemas
// ============================================================================

#[derive(Debug, Serialize)]
struct AnatomyRow {
    polytope_name: String,
    source_dataset: String,
    facet_count: usize,
    sys: f64,
    volume: f64,
    capacity: f64,
    orbit_perm: String,

    direction_type: String,
    direction_index: usize,

    t_max: f64,
    event_type: String,
    event_vertex: Option<usize>,
    event_facet_new: Option<usize>,
    event_facet_pair: Option<[usize; 2]>,
    event_facet_vanish: Option<usize>,

    vertex_count: usize,
    all_vertices_simple: bool,

    time_ms: f64,
}

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

// ============================================================================
// Input deserialization
// ============================================================================

#[derive(Debug, Deserialize)]
struct InputRow {
    name: String,
    #[serde(alias = "facet_count")]
    facet_count: usize,
    dual_vertices: Vec<[f64; 4]>,
}

// ============================================================================
// Enriched step-bound computation
// ============================================================================

/// Compute the first boundary event along the h-direction g_h.
///
/// For step a'_k = a_k * h_k / (h_k + t * g_h[k]) (i.e., heights change, normals fixed):
/// - Vertex-facet incidence: slack s_j(v) = h_j - n_j · v reaches zero
/// - Height positivity: h_k + t * g_h[k] > 0
///
/// Returns the binding event and the critical t value.
///
/// Adapted from sys-optimization/run.rs compute_step_bound() with event tracking.
// TODO: add [lem:first-boundary] to math.tex
fn compute_step_bound_detailed(
    polytope: &Polytope4D,
    direction: &[f64],
) -> BoundaryEvent {
    let duals = polytope.dual_vertices_f64();
    let normals: Vec<Vector4<f64>> = duals.iter().map(|a| a / a.norm()).collect();
    let heights: Vec<f64> = duals.iter().map(|a| 1.0 / a.norm()).collect();
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
            // Simple vertex: exactly 4 determining facets.
            // v = N_v^{-1} h_v, so dv/dt = N_v^{-1} g_h[det].
            let det_facets = vertex_facets;
            let n_mat = Matrix4::from_rows(&[
                normals[det_facets[0]].transpose(),
                normals[det_facets[1]].transpose(),
                normals[det_facets[2]].transpose(),
                normals[det_facets[3]].transpose(),
            ]);

            let n_inv = match n_mat.try_inverse() {
                Some(inv) => inv,
                None => continue,
            };

            let g_det = Vector4::new(
                direction[det_facets[0]],
                direction[det_facets[1]],
                direction[det_facets[2]],
                direction[det_facets[3]],
            );

            let dv_dt = n_inv * g_det;

            for j in 0..f {
                if vertex_facets.contains(&j) {
                    continue;
                }
                let slack = heights[j] - normals[j].dot(v);
                let rate = direction[j] - normals[j].dot(&dv_dt);
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
            for j in 0..f {
                if vertex_facets.contains(&j) {
                    continue;
                }
                let slack = heights[j] - normals[j].dot(v);
                if direction[j] < -EPS_NUMERICAL_ZERO {
                    continue;
                }
                let max_g = direction.iter().map(|x| x.abs()).fold(0.0f64, f64::max);
                if max_g > EPS_NUMERICAL_ZERO {
                    let t_crit = slack / max_g;
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

    // --- Height positivity ---
    for k in 0..f {
        if direction[k] < -EPS_NUMERICAL_ZERO {
            let t_crit = heights[k] / (-direction[k]);
            if t_crit > 0.0 && t_crit < best.t_max {
                best = BoundaryEvent {
                    t_max: t_crit,
                    event: EventType::HeightZero { facet: k },
                };
            }
        }
    }

    // Cap at MAX_STEP_SIZE
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

/// Build the set of probe directions for a polytope.
///
/// Directions are in height-space: each direction[k] represents d(h_k)/dt.
/// Given a gradient in dual-vertex space d(sys)/d(a_k), we project to height space via:
///   d(sys)/d(h_k) = d(sys)/d(a_k) · (-n_k / h_k²)
fn build_directions(
    _polytope: &Polytope4D,
    d_sys_h: &[f64],
    f: usize,
    rng: &mut ChaCha8Rng,
) -> Vec<Direction> {
    let mut dirs = Vec::new();

    // 1. Gradient direction (height-only)
    let grad_norm: f64 = d_sys_h.iter().map(|x| x * x).sum::<f64>().sqrt();
    if grad_norm > EPS_NUMERICAL_ZERO {
        let normalized: Vec<f64> = d_sys_h.iter().map(|x| x / grad_norm).collect();
        dirs.push(Direction {
            dir_type: "gradient".to_string(),
            index: 0,
            g_h: normalized,
        });
    }

    // 2. Negative gradient direction
    if grad_norm > EPS_NUMERICAL_ZERO {
        let neg_normalized: Vec<f64> = d_sys_h.iter().map(|x| -x / grad_norm).collect();
        dirs.push(Direction {
            dir_type: "neg_gradient".to_string(),
            index: 0,
            g_h: neg_normalized,
        });
    }

    // 3. Random directions (uniform on S^{F-1} via Gaussian normalization)
    for i in 0..N_RANDOM_DIRECTIONS {
        let raw: Vec<f64> = (0..f)
            .map(|_| StandardNormal.sample(rng))
            .collect();
        let norm: f64 = raw.iter().map(|x| x * x).sum::<f64>().sqrt();
        if norm > EPS_NUMERICAL_ZERO {
            let normalized: Vec<f64> = raw.iter().map(|x| x / norm).collect();
            dirs.push(Direction {
                dir_type: "random".to_string(),
                index: i,
                g_h: normalized,
            });
        }
    }

    // 4. Coordinate directions (one facet at a time)
    for k in 0..f {
        let mut g_h = vec![0.0; f];
        g_h[k] = 1.0;
        dirs.push(Direction {
            dir_type: "coordinate".to_string(),
            index: k,
            g_h,
        });
    }

    dirs
}

// ============================================================================
// Sensitivity computation (adapted from sys-optimization/run.rs)
// ============================================================================

/// Compute d(sys)/d(h_k) for all facets. Returns (d_sys_h, d_sys_a).
fn compute_sys_gradient(
    polytope: &Polytope4D,
    vol: f64,
    cap: f64,
    sys: f64,
    kkt: &KktResult,
    perm: &[usize],
) -> (Vec<f64>, Vec<Vector4<f64>>) {
    let duals = polytope.dual_vertices_f64();
    let f = duals.len();

    let d_vol_a = volume_derivatives_a(polytope);
    let d_cap_a = capacity_derivatives_a(
        &kkt.beta,
        kkt.q_corrected,
        &kkt.mu,
        perm,
        duals,
    );

    let d_sys_a: Vec<Vector4<f64>> = d_vol_a
        .iter()
        .zip(d_cap_a.iter())
        .map(|(dv, dc)| (cap * dc - sys * dv) / vol)
        .collect();

    let d_sys_h: Vec<f64> = (0..f)
        .map(|k| {
            let a_norm = duals[k].norm();
            let n = duals[k] / a_norm;
            let h = 1.0 / a_norm;
            d_sys_a[k].dot(&(-n / (h * h)))
        })
        .collect();

    (d_sys_h, d_sys_a)
}

// ============================================================================
// Polytope construction at perturbed parameter
// ============================================================================

/// Construct a polytope with heights shifted: h'_k = h_k + t * g_h[k].
///
/// Dual vertices become a'_k = n_k / h'_k = a_k * (h_k / h'_k).
fn construct_at_t(
    duals: &[Vector4<f64>],
    direction: &[f64],
    t: f64,
) -> Option<Polytope4D> {
    let new_duals: Vec<Vector4<f64>> = duals
        .iter()
        .zip(direction.iter())
        .map(|(a, &g)| {
            let a_norm = a.norm();
            let h = 1.0 / a_norm;
            let h_new = h + t * g;
            if h_new <= 0.0 {
                return Vector4::zeros(); // Will fail construction
            }
            // a'_k = n_k / h'_k = (a / |a|) / h'_k = a * (|a| * h'_k)^{-1} ...
            // Actually: a = n/h, so n = a*h = a/|a|, h = 1/|a|.
            // a' = n / h' = (a / |a|) / h' = a / (|a| * h')
            *a / (a_norm * h_new)
        })
        .collect();

    Polytope4D::from_f64(new_duals).ok()
}

/// Compute sys for a polytope. Returns (sys, capacity, volume, best_perm, kkt).
///
/// Catches panics from the capacity accumulator (numerical precision issues
/// on near-degenerate polytopes near combinatorial boundaries).
fn compute_sys(
    polytope: &Polytope4D,
) -> Option<(f64, f64, f64, Vec<usize>, KktResult)> {
    let vol = volume(polytope).ok()?;
    if vol <= 0.0 {
        return None;
    }

    // ehz_capacity can panic on near-degenerate polytopes (certified > uncertain gap).
    let ehz = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        ehz_capacity(polytope)
    }))
    .ok()
    .flatten()?;

    let cap = ehz.result.capacity;
    if !cap.is_finite() || cap <= 0.0 {
        return None;
    }

    let perm = ehz.result.best_permutation;
    let kkt = solve_kkt_for(polytope, &perm)?;
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
    direction: &[f64],
    boundary: &BoundaryEvent,
) -> Option<CrossingRow> {
    // Skip uncrossable events
    match boundary.event {
        EventType::Unbounded | EventType::HeightZero { .. } => return None,
        _ => {}
    }

    let t = boundary.t_max;
    if t <= 0.0 {
        return None;
    }

    // Before boundary: use proportional epsilon (1e-4 × t_max), with floor at 1e-8
    let eps_before = (1e-4 * t).max(1e-8);
    let poly_before = construct_at_t(duals, direction, t - eps_before)?;
    let (sys_b, cap_b, vol_b, perm_b, _) = compute_sys(&poly_before)?;
    let skel_before = Skeleton::compute(&poly_before);

    // After boundary — try proportional epsilons: 1e-4, 1e-3, 1e-2, 5e-2 of t_max
    let after_fractions = [1e-4, 1e-3, 1e-2, 5e-2, 0.1];
    let mut eps_used = 0.0;
    let mut sys_a = f64::NAN;
    let mut cap_a = f64::NAN;
    let mut vol_a = f64::NAN;
    let mut perm_a: Vec<usize> = vec![];
    let mut vc_after = 0usize;
    let mut construction_ok = false;

    for &frac in &after_fractions {
        let eps = (frac * t).max(1e-8);
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
        polytope_name: String::new(), // filled by caller
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
    direction: &[f64],
    boundary: &BoundaryEvent,
) -> Option<GradientRow> {
    match boundary.event {
        EventType::Unbounded | EventType::HeightZero { .. } => return None,
        _ => {}
    }

    let t = boundary.t_max;
    if t <= 0.0 {
        return None;
    }
    let f = duals.len();

    // Before boundary
    let eps_before = (1e-4 * t).max(1e-8);
    let poly_before = construct_at_t(duals, direction, t - eps_before)?;
    let (sys_b, cap_b, vol_b, perm_b, kkt_b) = compute_sys(&poly_before)?;
    let (d_sys_h_b, _) =
        compute_sys_gradient(&poly_before, vol_b, cap_b, sys_b, &kkt_b, &perm_b);

    // After boundary — proportional epsilons
    let after_fractions = [1e-4, 1e-3, 1e-2, 5e-2, 0.1];
    let mut after_grad = None;
    for &frac in &after_fractions {
        let eps = (frac * t).max(1e-8);
        if let Some(poly_after) = construct_at_t(duals, direction, t + eps) {
            if let Some((sys_a, cap_a, vol_a, perm_a, kkt_a)) = compute_sys(&poly_after) {
                let (d_sys_h_a, _) =
                    compute_sys_gradient(&poly_after, vol_a, cap_a, sys_a, &kkt_a, &perm_a);
                after_grad = Some(d_sys_h_a);
                break;
            }
        }
    }

    let d_sys_h_a = after_grad?;

    // Gradient norms
    let norm_b: f64 = d_sys_h_b.iter().map(|x| x * x).sum::<f64>().sqrt();
    let norm_a: f64 = d_sys_h_a.iter().map(|x| x * x).sum::<f64>().sqrt();

    // Directional derivatives (gradient dot direction)
    let dd_b: f64 = d_sys_h_b
        .iter()
        .zip(direction.iter())
        .map(|(g, d)| g * d)
        .sum();
    let dd_a: f64 = d_sys_h_a
        .iter()
        .zip(direction.iter())
        .map(|(g, d)| g * d)
        .sum();

    // Angle between gradients
    let dot: f64 = d_sys_h_b
        .iter()
        .zip(d_sys_h_a.iter())
        .map(|(a, b)| a * b)
        .sum();
    let angle_rad = if norm_b > EPS_NUMERICAL_ZERO && norm_a > EPS_NUMERICAL_ZERO {
        (dot / (norm_b * norm_a)).clamp(-1.0, 1.0).acos()
    } else {
        f64::NAN
    };

    Some(GradientRow {
        polytope_name: String::new(),
        facet_count: f,
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
// Data loading
// ============================================================================

fn load_polytopes_from_jsonl(
    path: &std::path::Path,
    source: &str,
) -> Vec<(String, String, Polytope4D)> {
    let file = match File::open(path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("WARNING: Could not open {}: {e}", path.display());
            eprintln!("  Run the corresponding experiment binary first.");
            return Vec::new();
        }
    };
    let reader = BufReader::new(file);
    let mut polytopes = Vec::new();

    for (line_no, line) in reader.lines().enumerate() {
        let line = match line {
            Ok(l) => l,
            Err(e) => {
                eprintln!("  Line {}: read error: {e}", line_no + 1);
                continue;
            }
        };
        if line.trim().is_empty() {
            continue;
        }

        let row: InputRow = match serde_json::from_str(&line) {
            Ok(r) => r,
            Err(e) => {
                eprintln!("  Line {}: parse error: {e}", line_no + 1);
                continue;
            }
        };

        if row.facet_count > MAX_FACET_COUNT {
            continue;
        }

        let duals: Vec<Vector4<f64>> = row
            .dual_vertices
            .iter()
            .map(|a| Vector4::new(a[0], a[1], a[2], a[3]))
            .collect();

        match Polytope4D::from_f64(duals) {
            Ok(p) => polytopes.push((row.name, source.to_string(), p)),
            Err(e) => {
                eprintln!("  {}: construction failed: {e}", row.name);
            }
        }
    }

    polytopes
}

// ============================================================================
// Main
// ============================================================================

fn main() {
    let t0 = Instant::now();
    let base_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut rng = ChaCha8Rng::seed_from_u64(SEED);

    println!("Combinatorial Boundaries: characterize boundary events\n");

    // =========================================================================
    // Load starting polytopes
    // =========================================================================

    println!("Loading starting polytopes (F ≤ {MAX_FACET_COUNT})...");

    let random_sweep_path = base_dir.join("random-sweep/random-sweep.jsonl");
    let random_product_path = base_dir.join("random-product-sweep/random-product-sweep.jsonl");

    let mut polytopes: Vec<(String, String, Polytope4D)> = Vec::new();

    let rs = load_polytopes_from_jsonl(&random_sweep_path, "random-sweep");
    println!("  random-sweep: {} polytopes loaded", rs.len());
    polytopes.extend(rs);

    let rp = load_polytopes_from_jsonl(&random_product_path, "random-product-sweep");
    println!("  random-product-sweep: {} polytopes loaded", rp.len());
    polytopes.extend(rp);

    let n_polytopes = polytopes.len();
    println!("  Total: {n_polytopes} polytopes\n");

    if n_polytopes == 0 {
        eprintln!("ERROR: No polytopes loaded. Run random_sweep and random_product_sweep first.");
        std::process::exit(1);
    }

    // =========================================================================
    // Open output files
    // =========================================================================

    let anatomy_path = base_dir.join("combinatorial-boundaries/combinatorial-boundaries-anatomy.jsonl");
    let crossing_path = base_dir.join("combinatorial-boundaries/combinatorial-boundaries-crossing.jsonl");

    let anatomy_file = File::create(&anatomy_path).expect("create anatomy JSONL");
    let mut anatomy_writer = BufWriter::new(anatomy_file);

    let crossing_file = File::create(&crossing_path).expect("create crossing JSONL");
    let mut crossing_writer = BufWriter::new(crossing_file);

    let gradient_path = base_dir.join("combinatorial-boundaries/combinatorial-boundaries-gradient.jsonl");
    let mut gradient_writer = if RUN_PHASE_3 {
        let f = File::create(&gradient_path).expect("create gradient JSONL");
        Some(BufWriter::new(f))
    } else {
        None
    };

    // =========================================================================
    // Process each polytope
    // =========================================================================

    let mut total_anatomy = 0usize;
    let mut total_crossing = 0usize;
    let mut total_gradient = 0usize;
    let mut n_skipped = 0usize;

    for (idx, (name, source, polytope)) in polytopes.iter().enumerate() {
        let t_poly = Instant::now();
        let f = polytope.facet_count();

        // Compute base sys + gradient
        let base_result = match compute_sys(polytope) {
            Some(r) => r,
            None => {
                eprintln!("  {name}: base sys computation failed, skipping");
                n_skipped += 1;
                continue;
            }
        };
        let (sys, cap, vol, perm, kkt) = base_result;
        let (d_sys_h, _d_sys_a) = compute_sys_gradient(polytope, vol, cap, sys, &kkt, &perm);

        let skeleton = Skeleton::compute(polytope);
        let vertex_count = skeleton.vertex_facets.len();
        let all_simple = skeleton.vertex_facets.iter().all(|vf| vf.len() == 4);
        let orbit_str = perm_to_string(&perm);

        // Build directions
        let directions = build_directions(polytope, &d_sys_h, f, &mut rng);
        let duals = polytope.dual_vertices_f64();

        for dir in &directions {
            let t_dir = Instant::now();

            // Phase 1: Boundary anatomy
            let boundary = compute_step_bound_detailed(polytope, &dir.g_h);

            let (ev_vertex, ev_facet_new, ev_facet_pair, ev_facet_vanish) = match &boundary.event {
                EventType::IncidenceFlip { vertex_index, new_facet } => {
                    (Some(*vertex_index), Some(*new_facet), None, None)
                }
                EventType::OmegaFlip { facet_i, facet_j } => {
                    (None, None, Some([*facet_i, *facet_j]), None)
                }
                EventType::HeightZero { facet } => {
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
                direction_type: dir.dir_type.clone(),
                direction_index: dir.index,
                t_max: boundary.t_max,
                event_type: boundary.event.name().to_string(),
                event_vertex: ev_vertex,
                event_facet_new: ev_facet_new,
                event_facet_pair: ev_facet_pair,
                event_facet_vanish: ev_facet_vanish,
                vertex_count,
                all_vertices_simple: all_simple,
                time_ms: t_dir.elapsed().as_secs_f64() * 1000.0,
            };

            serde_json::to_writer(&mut anatomy_writer, &anatomy_row).unwrap();
            writeln!(anatomy_writer).unwrap();
            total_anatomy += 1;

            // Phase 2: Crossing evaluation
            if let Some(mut crossing_row) =
                evaluate_crossing(duals, &dir.g_h, &boundary)
            {
                crossing_row.polytope_name = name.clone();
                crossing_row.direction_type = dir.dir_type.clone();
                crossing_row.direction_index = dir.index;
                crossing_row.event_type = boundary.event.name().to_string();

                serde_json::to_writer(&mut crossing_writer, &crossing_row).unwrap();
                writeln!(crossing_writer).unwrap();
                total_crossing += 1;
            }

            // Phase 3: Gradient crossing
            if RUN_PHASE_3 {
                if let Some(mut grad_row) =
                    evaluate_gradient_crossing(duals, &dir.g_h, &boundary)
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

        let elapsed = t_poly.elapsed().as_secs_f64();
        if (idx + 1) % 10 == 0 || idx + 1 == n_polytopes {
            println!(
                "  [{}/{}] {}: F={}, {} directions, {:.1}s",
                idx + 1,
                n_polytopes,
                name,
                f,
                directions.len(),
                elapsed
            );
        }
    }

    // =========================================================================
    // Flush and report
    // =========================================================================

    anatomy_writer.flush().unwrap();
    crossing_writer.flush().unwrap();
    if let Some(ref mut w) = gradient_writer {
        w.flush().unwrap();
    }

    let total_time = t0.elapsed().as_secs_f64();
    println!("\nDone in {total_time:.1}s.");
    println!("  Anatomy rows:  {total_anatomy}");
    println!("  Crossing rows: {total_crossing}");
    if RUN_PHASE_3 {
        println!("  Gradient rows: {total_gradient}");
    }
    if n_skipped > 0 {
        println!("  Skipped:       {n_skipped} (base sys computation failed)");
    }
}

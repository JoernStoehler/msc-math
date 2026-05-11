//! Boundary Characterization: full EHZ at boundaries + crossing analysis + gradient measurement.
//!
//! Location: experiments/combinatorial-cells/boundary-characterization/main.rs
//!
//! For polytopes K = {x : a_k . x <= 1}, probes the gradient direction, negative gradient,
//! and N_GLOBAL_DENSE random directions. At each boundary:
//! - Records boundary anatomy (event type, t_max, orbit gap, etc.)
//! - Evaluates crossing (sys before/after, orbit switch)
//! - Measures gradient change across the boundary
//!
//! The gradient JSONL is also used by gradient-discontinuity/analyze.py.
//!
//! Split from combinatorial-structure (Pass 2).
//!
//! Input Artifacts: experiments/combinatorial-cells/polytopes.jsonl (owned cache)
//! Filter: F <= 10 (HK2017 is exponential in F)
//! Output Artifacts: experiments/combinatorial-cells/boundary-characterization/combinatorial-boundaries-anatomy.jsonl,
//!         experiments/combinatorial-cells/boundary-characterization/combinatorial-boundaries-crossing.jsonl,
//!         experiments/combinatorial-cells/boundary-characterization/combinatorial-boundaries-gradient.jsonl

use euclidean_polytopes::vertex_facets_from_vertex_facet_incidence;
use exp_combinatorial_cells::euclidean_volume_f64;
#[path = "../src/flat_polytope.rs"]
mod flat_polytope;

use crate::flat_polytope::CellPolytopeCache;
use exp_combinatorial_cells::{
    compute_step_bound_detailed, ehz_capacity_instrumented, name_from_record,
    source_dataset_from_record, BoundaryEvent, EventType,
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
use symplectic::kkt::saddle_point_solver::solve_kkt_for_dual_vertices;

// ============================================================================
// Configuration
// ============================================================================

/// Maximum facet count to process (HK2017 cost is exponential).
const MAX_FACET_COUNT: usize = 10;

fn construct_at_t(
    duals: &[Vector4<f64>],
    direction: &[Vector4<f64>],
    t: f64,
) -> Option<CellPolytopeCache> {
    let new_duals: Vec<Vector4<f64>> = duals
        .iter()
        .zip(direction.iter())
        .map(|(a, d)| a + t * d)
        .collect();
    CellPolytopeCache::from_f64(new_duals)
}

/// Number of dense random directions for global probes.
const N_GLOBAL_DENSE: usize = 5;

/// Maximum step size cap (prevents infinite steps when no combinatorial bound exists).
/// Typical boundary distances are O(0.01)-O(1); 100.0 is well beyond any real boundary.
const MAX_STEP_SIZE: f64 = 100.0;

/// Numerical zero threshold for rates and slacks.
/// Set near machine epsilon (~1e-16); guards against treating f64 noise as a
/// meaningful direction or rate.
const EPS_NUMERICAL_ZERO: f64 = 1e-15;

/// Epsilon fractions for crossing evaluation: proportional to t_max.
/// Geometric progression from 1e-4 to 0.1 of t_max. Start close to the boundary
/// (1e-4 * t_max) for accuracy; fall back to larger fractions if construction or
/// EHZ fails near the boundary. Floor at 1e-8 prevents sub-machine-epsilon steps.
/// Validated: 100% crossing success rate at 873/873 boundaries with this sequence.
const CROSSING_EPS_FRACTIONS: &[f64] = &[1e-4, 1e-3, 1e-2, 5e-2, 0.1];

/// Fraction of t_max used for the "before boundary" evaluation point.
/// 1e-4 * t_max places us close enough that sys_before ~= sys(original) while
/// being safely on the pre-boundary side.
const BEFORE_EPS_FRACTION: f64 = 1e-4;

/// Absolute floor for epsilon values, prevents sub-machine-epsilon perturbations.
const EPS_FLOOR: f64 = 1e-8;

/// Random seed for reproducibility.
const SEED: u64 = 42;

/// Whether to run gradient measurement across boundaries.
const RUN_GRADIENT: bool = true;

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
// Output schemas
// ============================================================================

/// Boundary anatomy row (one per global direction).
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

/// Crossing evaluation row.
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

/// Gradient crossing row.
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
// Direction construction
// ============================================================================

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
/// sys = c^2/(2V), so by the quotient rule:
///   d(sys)/d(a_k) = (c * dc/d(a_k) - sys * dV/d(a_k)) / V
fn compute_sys_gradient_a(
    polytope: &CellPolytopeCache,
    vol: f64,
    cap: f64,
    sys: f64,
    kkt: &symplectic::kkt::saddle_point_solver::KktResult,
    perm: &[usize],
) -> Vec<Vector4<f64>> {
    let d_vol_a = volume_derivatives_a(
        &polytope.dual_vertices_f64,
        &polytope.vertices_f64,
        &polytope.vertex_facet_incidence,
    )
    .expect("combinatorial-cell polytope has valid finite geometry");
    let d_cap_a = capacity_derivatives_a_from_kkt_result(&polytope.dual_vertices_f64, perm, kkt);

    d_vol_a
        .iter()
        .zip(d_cap_a.iter())
        .map(|(dv, dc)| (cap * dc - sys * dv) / vol)
        .collect()
}

/// Compute sys for a polytope using the default root capacity wrapper.
/// Returns (sys, capacity, volume, best_perm, kkt).
fn compute_sys(
    polytope: &CellPolytopeCache,
) -> Option<(
    f64,
    f64,
    f64,
    Vec<usize>,
    symplectic::kkt::saddle_point_solver::KktResult,
)> {
    let vol = euclidean_volume_f64(&polytope.vertices, &polytope.vertex_facet_incidence);
    if vol <= 0.0 {
        return None;
    }

    let ehz = exp_combinatorial_cells::capacity_auto(
        &polytope.dual_vertices,
        &polytope.dual_vertices_f64,
        &polytope.facet_intersection_is_nonempty,
        &polytope.omega_signs,
    )
    .ok()?;

    let cap = ehz.capacity();
    if !cap.is_finite() || cap <= 0.0 {
        return None;
    }

    let perm = ehz.best_sigma().to_vec();
    let kkt = solve_kkt_for_dual_vertices(&polytope.dual_vertices_f64, &perm).feasible()?;
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
// Crossing evaluation
// ============================================================================

/// Evaluate sys/orbit on both sides of a boundary.
fn evaluate_crossing(
    duals: &[Vector4<f64>],
    direction: &[Vector4<f64>],
    boundary: &BoundaryEvent,
    polytope_name: &str,
    direction_type: &str,
    direction_index: usize,
    event_type: &str,
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
    let vertex_count_before =
        vertex_facets_from_vertex_facet_incidence(&poly_before.vertex_facet_incidence).len();

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
                eps_used = eps;
                sys_a = s;
                cap_a = c;
                vol_a = v;
                perm_a = p;
                vc_after =
                    vertex_facets_from_vertex_facet_incidence(&poly_after.vertex_facet_incidence)
                        .len();
                construction_ok = true;
                break;
            }
        }
    }

    let orbit_b_str = perm_to_string(&perm_b);
    let orbit_a_str = perm_to_string(&perm_a);

    Some(CrossingRow {
        polytope_name: polytope_name.to_string(),
        facet_count: duals.len(),
        direction_type: direction_type.to_string(),
        direction_index,
        t_max: t,
        event_type: event_type.to_string(),
        eps_used,
        sys_before: sys_b,
        capacity_before: cap_b,
        volume_before: vol_b,
        orbit_before: orbit_b_str.clone(),
        vertex_count_before,
        sys_after: sys_a,
        capacity_after: cap_a,
        volume_after: vol_a,
        orbit_after: orbit_a_str.clone(),
        vertex_count_after: vc_after,
        construction_ok_after: construction_ok,
        delta_sys: sys_a - sys_b,
        orbit_changed: orbit_b_str != orbit_a_str,
        vertex_count_changed: vertex_count_before != vc_after,
    })
}

// ============================================================================
// Gradient crossing evaluation
// ============================================================================

/// Compute gradient info on both sides of a boundary.
fn evaluate_gradient_crossing(
    duals: &[Vector4<f64>],
    direction: &[Vector4<f64>],
    boundary: &BoundaryEvent,
    polytope_name: &str,
    direction_type: &str,
    direction_index: usize,
    event_type: &str,
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
    let d_sys_a_b = compute_sys_gradient_a(&poly_before, vol_b, cap_b, sys_b, &kkt_b, &perm_b);

    let mut d_sys_a_after = None;
    for &frac in CROSSING_EPS_FRACTIONS {
        let eps = (frac * t).max(EPS_FLOOR);
        if let Some(poly_after) = construct_at_t(duals, direction, t + eps) {
            if let Some((sys_a, cap_a, vol_a, perm_a, kkt_a)) = compute_sys(&poly_after) {
                d_sys_a_after = Some(compute_sys_gradient_a(
                    &poly_after,
                    vol_a,
                    cap_a,
                    sys_a,
                    &kkt_a,
                    &perm_a,
                ));
                break;
            }
        }
    }

    let d_sys_a_a = d_sys_a_after?;

    let norm_b: f64 = d_sys_a_b
        .iter()
        .map(|v| v.norm_squared())
        .sum::<f64>()
        .sqrt();
    let norm_a: f64 = d_sys_a_a
        .iter()
        .map(|v| v.norm_squared())
        .sum::<f64>()
        .sqrt();

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
        polytope_name: polytope_name.to_string(),
        facet_count: duals.len(),
        direction_type: direction_type.to_string(),
        direction_index,
        t_max: boundary.t_max,
        event_type: event_type.to_string(),
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
// Main
// ============================================================================

fn main() {
    let t0 = Instant::now();
    let base_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut rng = ChaCha8Rng::seed_from_u64(SEED);

    println!("Combinatorial Anatomy: boundary crossing analysis\n");

    // =========================================================================
    // Load starting polytopes from database
    // =========================================================================

    println!("Loading starting polytopes from owned cache (F <= {MAX_FACET_COUNT})...");

    let owned_db_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("polytopes.jsonl");
    let db = database::load_many(&[owned_db_path.as_path()]).expect("failed to load database");

    let mut polytopes: Vec<(String, String, CellPolytopeCache)> = Vec::new();

    for (idx, (_, record)) in db.iter().enumerate() {
        let f = record.dual_vertices_rational.len();
        if f > MAX_FACET_COUNT {
            continue;
        }
        let p = match CellPolytopeCache::from_rational_parts(
            record.dual_vertices_rational.clone(),
            record.vertices_rational.clone(),
        ) {
            Some(p) => p,
            None => {
                eprintln!("  db entry {idx}: reconstruction failed");
                continue;
            }
        };
        let name = name_from_record(record, idx);
        let source = source_dataset_from_record(record);
        polytopes.push((name, source, p));
    }

    let n_polytopes = polytopes.len();
    println!("  {n_polytopes} polytopes loaded from database (F <= {MAX_FACET_COUNT})\n");

    if n_polytopes == 0 {
        eprintln!("ERROR: No polytopes in database. Run sys-random-sample and sys-random-product-sample first.");
        std::process::exit(1);
    }

    // =========================================================================
    // Open output files
    // =========================================================================

    let out_dir = base_dir.join("boundary-characterization");

    let anatomy_path = out_dir.join("combinatorial-boundaries-anatomy.jsonl");
    let anatomy_file = File::create(&anatomy_path)
        .unwrap_or_else(|err| panic!("create anatomy JSONL {}: {err}", anatomy_path.display()));
    let mut anatomy_writer = BufWriter::new(anatomy_file);

    let crossing_path = out_dir.join("combinatorial-boundaries-crossing.jsonl");
    let crossing_file = File::create(&crossing_path)
        .unwrap_or_else(|err| panic!("create crossing JSONL {}: {err}", crossing_path.display()));
    let mut crossing_writer = BufWriter::new(crossing_file);

    let mut gradient_writer = if RUN_GRADIENT {
        let gradient_path = out_dir.join("combinatorial-boundaries-gradient.jsonl");
        let f = File::create(&gradient_path).unwrap_or_else(|err| {
            panic!("create gradient JSONL {}: {err}", gradient_path.display())
        });
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
        let duals = &polytope.dual_vertices_f64;

        // =====================================================================
        // Base computation: instrumented EHZ for orbit gap + gradient
        // =====================================================================

        let base = (|| {
            let instrumented = ehz_capacity_instrumented(
                &polytope.dual_vertices_f64,
                &polytope.facet_intersection_is_nonempty,
                &polytope.omega_signs,
            )?;
            let vol = euclidean_volume_f64(&polytope.vertices, &polytope.vertex_facet_incidence);
            if vol <= 0.0 {
                return None;
            }
            let cap = instrumented.capacity;
            let sys = cap * cap / (2.0 * vol);
            let perm = instrumented.best_permutation;
            let orbit_gap = instrumented.orbit_gap;
            let n_valid_orbits = instrumented.n_valid_orbits;
            let dual_vertices = &polytope.dual_vertices_f64;
            let kkt = solve_kkt_for_dual_vertices(dual_vertices, &perm).feasible()?;
            Some((cap, vol, sys, perm, orbit_gap, n_valid_orbits, kkt))
        })();

        let (cap, vol, sys, perm, orbit_gap, n_valid_orbits, kkt) = match base {
            Some(t) => t,
            None => {
                n_skipped += 1;
                continue;
            }
        };

        let d_sys_a = compute_sys_gradient_a(polytope, vol, cap, sys, &kkt, &perm);

        let vertex_facets_by_vertex =
            vertex_facets_from_vertex_facet_incidence(&polytope.vertex_facet_incidence);
        let vertex_count = vertex_facets_by_vertex.len();
        let all_simple = vertex_facets_by_vertex.iter().all(|vf| vf.len() == 4);
        let orbit_str = perm_to_string(&perm);

        // =====================================================================
        // Global probes (with EHZ)
        // =====================================================================

        let global_dirs = build_global_directions(&d_sys_a, f, &mut rng);

        for dir in &global_dirs {
            let t_dir = Instant::now();

            let boundary = compute_step_bound_detailed(
                &polytope.dual_vertices_f64,
                &polytope.vertices_f64,
                &polytope.vertex_facet_incidence,
                &dir.d,
                EPS_NUMERICAL_ZERO,
                MAX_STEP_SIZE,
            );

            let (ev_vertex, ev_facet_new, ev_facet_pair, ev_facet_degen) = match &boundary.event {
                EventType::IncidenceFlip {
                    vertex_index,
                    new_facet,
                } => (Some(*vertex_index), Some(*new_facet), None, None),
                EventType::OmegaFlip { facet_i, facet_j } => {
                    (None, None, Some([*facet_i, *facet_j]), None)
                }
                EventType::DualVertexDegen { facet } => (None, None, None, Some(*facet)),
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
            if let Some(crossing_row) = evaluate_crossing(
                duals,
                &dir.d,
                &boundary,
                name,
                &dir.dir_type,
                dir.index,
                boundary.event.name(),
            ) {
                serde_json::to_writer(&mut crossing_writer, &crossing_row).unwrap();
                writeln!(crossing_writer).unwrap();
                total_crossing += 1;
            }

            // Gradient crossing
            if RUN_GRADIENT {
                if let Some(grad_row) = evaluate_gradient_crossing(
                    duals,
                    &dir.d,
                    &boundary,
                    name,
                    &dir.dir_type,
                    dir.index,
                    boundary.event.name(),
                ) {
                    if let Some(ref mut w) = gradient_writer {
                        serde_json::to_writer(&mut *w, &grad_row).unwrap();
                        writeln!(w).unwrap();
                        total_gradient += 1;
                    }
                }
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

    anatomy_writer.flush().unwrap();
    crossing_writer.flush().unwrap();
    if let Some(ref mut w) = gradient_writer {
        w.flush().unwrap();
    }

    let total_time = t0.elapsed().as_secs_f64();
    println!("\nDone in {total_time:.1}s.");
    println!("  Anatomy rows:   {total_anatomy}");
    println!("  Crossing rows:  {total_crossing}");
    if RUN_GRADIENT {
        println!("  Gradient rows:  {total_gradient}");
    }
    if n_skipped > 0 {
        println!("  Skipped:        {n_skipped} (base computation failed)");
    }
}

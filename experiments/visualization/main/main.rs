//! JSON export of polytope geometry for the interactive visualization.
//!
//! Exports the full combinatorial skeleton (vertices, edges, ridges),
//! Reeb vectors, and Reeb orbit trajectories as a single JSON file
//! consumed by the Three.js viewer in `experiments/visualization/main/viz/`.
//!
//! Trajectories include:
//! - All closed Reeb orbits found by the HK2017 algorithm (min-action and others)
//! - Displaced variants of the min-action orbit to illustrate twisting
//!
//! Input Artifacts: None (exports built-in known polytopes only).
//! Output Artifacts: None (writes the JSON path passed on the CLI).
use nalgebra::{DMatrix, Vector4};
use serde::Serialize;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use symplectic::algorithms::{OrbitAdmissibility, OrbitKktData};
use symplectic::algorithms::hk2017::orbit_recovery::recover_and_verify;
use symplectic::algorithms::facet_adjacency::build_transition_matrix;
use symplectic::algorithms::hk2017::combinations;
use symplectic::algorithms::hk2017::permutations::for_each_cyclic_permutation;
use symplectic::geom::reeb_trajectory;
use symplectic::geom::known_polytopes::{self, KnownPolytope};
use symplectic::kkt::saddle_point_solver::{solve_kkt_for, KktOutcome, EPS_BETA_POSITIVE, EPS_Q_POSITIVE};
use symplectic::geom::skeleton::Skeleton;

/// Maximum number of orbits to export per polytope (keeps data.js manageable).
const MAX_ORBITS: usize = 20;

/// Displacement magnitude for displaced orbit visualization.
const DISPLACEMENT_EPS: f64 = 0.02;

/// Max facet count for orbit computation (skip crosspolytope F=16).
const MAX_FACETS_FOR_ORBIT: usize = 12;

/// Top-level JSON structure for visualization.
#[derive(Serialize)]
pub struct VizExport {
    pub name: String,
    pub source: String,
    pub capacity: f64,
    pub facet_count: usize,
    pub vertex_count: usize,
    pub edge_count: usize,
    pub ridge_count: usize,
    /// Dual vertices a_i where K = {x : a_i^T x <= 1}, each `[a₁, a₂, a₃, a₄]`.
    pub dual_vertices: Vec<[f64; 4]>,
    /// Reeb flow directions per facet: `reeb_vectors[i] = J₀ · a_i`.
    pub reeb_vectors: Vec<[f64; 4]>,
    /// Vertices, each `[x₁, x₂, x₃, x₄]`.
    pub vertices: Vec<[f64; 4]>,
    /// Edges as pairs of vertex indices.
    pub edges: Vec<[usize; 2]>,
    /// 2-faces (ridges).
    pub ridges: Vec<VizRidge>,
    /// Per-vertex facet incidence.
    pub vertex_facets: Vec<Vec<usize>>,
    /// Reeb trajectories: recovered orbits and displaced variants.
    pub trajectories: Vec<VizTrajectory>,
    /// Volume of the polytope.
    pub volume: f64,
    /// Systolic ratio: c_EHZ² / (2 · volume).
    pub systolic_ratio: f64,
}

#[derive(Serialize)]
pub struct VizRidge {
    pub facets: [usize; 2],
    pub vertices: Vec<usize>,
}

#[derive(Serialize)]
pub struct VizTrajectory {
    /// Human-readable label for UI display.
    pub label: String,
    pub start_facet: usize,
    pub closed: bool,
    pub segments: Vec<VizSegment>,
}

#[derive(Serialize)]
pub struct VizSegment {
    pub start: [f64; 4],
    pub end: [f64; 4],
    pub facet: usize,
}

fn v4_to_array(v: &Vector4<f64>) -> [f64; 4] {
    [v[0], v[1], v[2], v[3]]
}

// ============================================================================
// Orbit collection: find ALL valid (S, σ, β) orbits
// ============================================================================

/// A valid orbit found by exhaustive enumeration.
struct CollectedOrbit {
    action: f64,
    permutation: Vec<usize>, // positive Reeb direction
    beta: Vec<f64>,          // matching permutation order
    subset: Vec<usize>,
}

/// Check if a cyclic permutation forms an adjacent cycle.
/// Local predicate because HK2017 enumeration keeps this helper internal.
fn is_adjacent_cycle(perm: &[usize], adj: &DMatrix<bool>) -> bool {
    let m = perm.len();
    (0..m).all(|k| adj[(perm[k], perm[(k + 1) % m])])
}

/// Collect ALL valid Reeb orbits for the polytope.
///
/// Uses the same algorithm as `ehz_capacity` (directed adjacency pruning),
/// but collects all certified orbits instead of just the best one.
/// Returns orbits sorted by action (ascending).
fn collect_all_orbits(polytope: &symplectic::geom::polytope::Polytope4D) -> Vec<CollectedOrbit> {
    let f = polytope.facet_count();
    let adj = build_transition_matrix(polytope);

    let mut orbits: Vec<CollectedOrbit> = Vec::new();

    for m in 2..=f {
        for subset in combinations(f, m) {
            for_each_cyclic_permutation(&subset, &mut |perm| {
                if !is_adjacent_cycle(perm, &adj) {
                    return;
                }

                if let KktOutcome::Feasible(result) = solve_kkt_for(polytope, perm) {
                    let q_val = result.q_corrected;
                    if q_val <= EPS_Q_POSITIVE {
                        return;
                    }
                    let beta_min = result
                        .beta
                        .iter()
                        .cloned()
                        .fold(f64::INFINITY, f64::min);
                    if beta_min <= EPS_BETA_POSITIVE {
                        return; // not certified
                    }
                    let action = 0.5 / q_val;

                    orbits.push(CollectedOrbit {
                        action,
                        permutation: perm.to_vec(),
                        beta: result.beta,
                        subset: subset.clone(),
                    });
                }
            });
        }
    }

    orbits.sort_by(|a, b| a.action.partial_cmp(&b.action).unwrap());
    orbits
}

// ============================================================================
// Orbit → VizTrajectory conversion
// ============================================================================

/// Convert a collected orbit into the solved-orbit payload used by recovery.
fn orbit_to_orbit_kkt_data(orbit: &CollectedOrbit) -> OrbitKktData {
    OrbitKktData {
        sigma: orbit.permutation.clone(),
        beta: orbit.beta.clone(),
        beta_margin: orbit.beta.iter().copied().fold(f64::INFINITY, f64::min),
        action: orbit.action,
        action_lower: orbit.action,
        action_upper: orbit.action,
        q: 0.5 / orbit.action,
        q_error_bound: 0.0,
        mu: None,
        xi: None,
        admissibility: OrbitAdmissibility::AdmissibleF64,
    }
}

/// Recover a Reeb orbit and convert to VizTrajectory.
///
/// Returns None if recovery fails or the orbit has excessive violations.
fn orbit_to_viz_trajectory(
    polytope: &symplectic::geom::polytope::Polytope4D,
    orbit: &CollectedOrbit,
    label: String,
) -> Option<VizTrajectory> {
    let orbit_data = orbit_to_orbit_kkt_data(orbit);
    let recovery = recover_and_verify(polytope, &orbit_data)?;

    // Validate orbit quality
    if recovery.closure_error > 1e-6 {
        eprintln!(
            "  WARN orbit {}: closure_error={:.2e} (too large, skipping)",
            label, recovery.closure_error
        );
        return None;
    }
    if recovery.max_violation > 1e-4 {
        eprintln!(
            "  WARN orbit {}: max_violation={:.2e} (too large, skipping)",
            label, recovery.max_violation
        );
        return None;
    }

    let sigma = &orbit_data.sigma;
    let m = sigma.len();

    // breakpoints[k] → breakpoints[k+1] is a segment on facet σ[k]
    let segments: Vec<VizSegment> = (0..m)
        .map(|k| VizSegment {
            start: v4_to_array(&recovery.breakpoints[k]),
            end: v4_to_array(&recovery.breakpoints[k + 1]),
            facet: sigma[k],
        })
        .collect();

    Some(VizTrajectory {
        label,
        start_facet: sigma[0],
        closed: true,
        segments,
    })
}

// ============================================================================
// Displaced orbit generation
// ============================================================================

/// Compute two displacement directions spanning the tangent space of the ridge
/// where the orbit's base point γ(0) lies.
///
/// For a closed orbit with permutation σ = [σ(0), ..., σ(m-1)], the base point
/// γ(0) lies on the ridge F_σ(0) ∩ F_σ(m-1) (transition from the last facet
/// back to the first). The ridge tangent space is 2D (perpendicular to both
/// normals n_σ(0) and n_σ(m-1)). We return an orthonormal basis for it.
fn ridge_displacement_directions(
    polytope: &symplectic::geom::polytope::Polytope4D,
    first_facet: usize,
    last_facet: usize,
) -> Vec<Vector4<f64>> {
    let duals = polytope.dual_vertices_f64();
    let n0 = duals[first_facet].normalize();
    let n1 = duals[last_facet].normalize();

    // Find two vectors perpendicular to both n0 and n1 via Gram-Schmidt
    // on standard basis candidates.
    let candidates = [
        Vector4::new(1.0, 0.0, 0.0, 0.0),
        Vector4::new(0.0, 1.0, 0.0, 0.0),
        Vector4::new(0.0, 0.0, 1.0, 0.0),
        Vector4::new(0.0, 0.0, 0.0, 1.0),
    ];

    let mut basis: Vec<Vector4<f64>> = Vec::new();
    for e in &candidates {
        // Project out both normal components
        let mut v: Vector4<f64> = e - n0.dot(e) * n0;
        v -= n1.dot(&v) * n1;
        // Project out already-found basis vectors (Gram-Schmidt)
        for b in &basis {
            v -= b.dot(&v) * b;
        }
        let norm = v.norm();
        if norm > 1e-8 {
            basis.push(v / norm);
            if basis.len() == 2 {
                break;
            }
        }
    }
    basis
}

/// Generate displaced trajectories by perturbing the base point of an orbit.
///
/// For a closed orbit σ = [σ(0), ..., σ(m-1)], γ(0) lies on the ridge
/// F_σ(0) ∩ F_σ(m-1). We displace along both directions spanning the
/// ridge's 2D tangent space.
fn generate_displaced_trajectories(
    polytope: &symplectic::geom::polytope::Polytope4D,
    orbit: &CollectedOrbit,
    skeleton: &Skeleton,
) -> Vec<VizTrajectory> {
    let orbit_data = orbit_to_orbit_kkt_data(orbit);
    let recovery = match recover_and_verify(polytope, &orbit_data) {
        Some(r) => r,
        None => return vec![],
    };

    let perm = &orbit_data.sigma;
    let start_facet = perm[0];
    let last_facet = perm[perm.len() - 1];
    let directions = ridge_displacement_directions(polytope, start_facet, last_facet);
    eprintln!(
        "  Ridge F_{} ∩ F_{}: {} displacement direction(s)",
        start_facet, last_facet, directions.len()
    );

    let _ = skeleton; // skeleton available if needed for facet_centroid, unused here

    let max_segments = orbit.permutation.len();
    let mut trajectories = Vec::new();
    for (i, disp) in directions.iter().enumerate() {
        // Try +ε first; if that pushes outside K, try -ε (the other side of the ridge).
        let mut displaced_start = recovery.breakpoints[0] + DISPLACEMENT_EPS * disp;
        let mut traj = reeb_trajectory::simulate_with(polytope, displaced_start, start_facet, max_segments, 1e-6);
        if traj.segments.is_empty() {
            displaced_start = recovery.breakpoints[0] - DISPLACEMENT_EPS * disp;
            traj = reeb_trajectory::simulate_with(polytope, displaced_start, start_facet, max_segments, 1e-6);
        }
        if traj.segments.is_empty() {
            eprintln!("  displaced v{}: simulation returned 0 segments in both directions", i + 1);
            continue;
        }

        trajectories.push(VizTrajectory {
            label: format!("displaced v{} (ε={})", i + 1, DISPLACEMENT_EPS),
            start_facet,
            closed: traj.closed,
            segments: traj
                .segments
                .iter()
                .map(|s| VizSegment {
                    start: v4_to_array(&s.start),
                    end: v4_to_array(&s.end),
                    facet: s.facet,
                })
                .collect(),
        });
    }

    trajectories
}

// ============================================================================
// Main trajectory generation (replaces old placeholder)
// ============================================================================

/// Generate all trajectories for a polytope:
/// 1. All closed Reeb orbits from HK2017 (min-action and others)
/// 2. Displaced variants of the min-action orbit
///
/// Returns (trajectories, computed_capacity). The capacity comes from the
/// minimum-action orbit found during enumeration, avoiding a redundant
/// `ehz_capacity()` call.
fn generate_trajectories(
    polytope: &symplectic::geom::polytope::Polytope4D,
    skeleton: &Skeleton,
) -> (Vec<VizTrajectory>, Option<f64>) {
    // Skip polytopes with too many facets (exponential cost)
    if polytope.facet_count() > MAX_FACETS_FOR_ORBIT {
        eprintln!(
            "  Skipping orbit computation (F={}, too many facets). Using placeholder.",
            polytope.facet_count()
        );
        return (generate_placeholder_trajectory(polytope, skeleton), None);
    }

    // Collect all valid orbits
    let all_orbits = collect_all_orbits(polytope);
    if all_orbits.is_empty() {
        eprintln!("  No valid orbits found. Using placeholder.");
        return (generate_placeholder_trajectory(polytope, skeleton), None);
    }

    let min_action = all_orbits[0].action;
    eprintln!(
        "  Found {} orbits (min action = {:.6}, max action = {:.6})",
        all_orbits.len(),
        min_action,
        all_orbits.last().unwrap().action
    );

    let mut trajectories = Vec::new();

    // Convert orbits to VizTrajectories (cap at MAX_ORBITS)
    let mut min_action_count = 0usize;
    for (i, orbit) in all_orbits.iter().enumerate() {
        if trajectories.len() >= MAX_ORBITS {
            eprintln!(
                "  Capped at {} orbits (skipped {})",
                MAX_ORBITS,
                all_orbits.len() - MAX_ORBITS
            );
            break;
        }

        let is_min = (orbit.action - min_action).abs() < 1e-8;
        let label = if is_min {
            min_action_count += 1;
            if min_action_count == 1 {
                format!("min-action orbit (c={:.4})", orbit.action)
            } else {
                format!("min-action orbit #{} (c={:.4})", min_action_count, orbit.action)
            }
        } else {
            format!("orbit #{} (action={:.4})", i + 1, orbit.action)
        };

        match orbit_to_viz_trajectory(polytope, orbit, label.clone()) {
            Some(traj) => {
                eprintln!(
                    "  {} → {} segments, facets {:?}",
                    label,
                    traj.segments.len(),
                    orbit.permutation
                );
                trajectories.push(traj);
            }
            None => {
                eprintln!("  {} → recovery failed, skipping", label);
            }
        }
    }

    // Generate displaced variants of the first min-action orbit
    if let Some(first_orbit) = all_orbits.first() {
        let displaced = generate_displaced_trajectories(polytope, first_orbit, skeleton);
        for d in displaced {
            eprintln!(
                "  {} → {} segments, closed={}",
                d.label,
                d.segments.len(),
                d.closed
            );
            trajectories.push(d);
        }
    }

    if trajectories.is_empty() {
        eprintln!("  All orbit recoveries failed. Using placeholder.");
        return (generate_placeholder_trajectory(polytope, skeleton), Some(min_action));
    }

    (trajectories, Some(min_action))
}

/// Fallback: generate a single forward-simulated trajectory (the old behavior).
fn generate_placeholder_trajectory(
    polytope: &symplectic::geom::polytope::Polytope4D,
    skeleton: &Skeleton,
) -> Vec<VizTrajectory> {
    for fi in 0..polytope.facet_count() {
        let centroid = skeleton.facet_centroid(polytope, fi);
        let traj = reeb_trajectory::simulate_with(polytope, centroid, fi, 100, 1e-6);

        if !traj.segments.is_empty() {
            return vec![VizTrajectory {
                label: "placeholder trajectory".to_string(),
                start_facet: fi,
                closed: traj.closed,
                segments: traj
                    .segments
                    .iter()
                    .map(|s| VizSegment {
                        start: v4_to_array(&s.start),
                        end: v4_to_array(&s.end),
                        facet: s.facet,
                    })
                    .collect(),
            }];
        }
    }

    vec![]
}

// ============================================================================
// Export
// ============================================================================

/// Look up a known polytope by name. Returns `None` for unknown names.
fn lookup_known(name: &str) -> Option<&'static KnownPolytope> {
    known_polytopes::all_known()
        .into_iter()
        .find(|kp| kp.name == name)
}

/// Export a known polytope to a JSON file.
///
/// Returns an error message if the polytope name is unknown or IO fails.
pub fn export(name: &str, output: &Path) -> Result<(), String> {
    let kp = lookup_known(name).ok_or_else(|| {
        let names: Vec<&str> = known_polytopes::all_known()
            .iter()
            .map(|kp| kp.name)
            .collect();
        format!("Unknown polytope '{name}'. Available: {}", names.join(", "))
    })?;

    let polytope = &kp.polytope;
    let skeleton = Skeleton::compute(polytope);

    // Reeb vectors (from dual vertices a_i: R_i = J_0 a_i)
    let reeb_vectors: Vec<[f64; 4]> = polytope
        .dual_vertices_f64()
        .iter()
        .map(|a| v4_to_array(&reeb_trajectory::reeb_direction(a)))
        .collect();

    // Trajectories: real Reeb orbits + displaced variants
    eprintln!("Computing orbits for {}...", kp.name);
    let (trajectories, computed_capacity) = generate_trajectories(polytope, &skeleton);

    // Use capacity from orbit enumeration if available, else fall back to known value
    let capacity = computed_capacity.unwrap_or(kp.capacity);

    // Compute volume and systolic ratio
    let vol = symplectic::geom::volume::volume(polytope).unwrap_or(0.0);
    let systolic_ratio = if vol > 0.0 {
        capacity * capacity / (2.0 * vol)
    } else {
        0.0
    };

    let export = VizExport {
        name: kp.name.to_string(),
        source: kp.source.to_string(),
        capacity,
        facet_count: polytope.facet_count(),
        vertex_count: polytope.vertices_f64().len(),
        edge_count: skeleton.edges.len(),
        ridge_count: skeleton.ridges.len(),
        dual_vertices: polytope.dual_vertices_f64().iter().map(v4_to_array).collect(),
        reeb_vectors,
        vertices: polytope.vertices_f64().iter().map(v4_to_array).collect(),
        edges: skeleton.edges.clone(),
        ridges: skeleton
            .ridges
            .iter()
            .map(|r| VizRidge {
                facets: r.facets,
                vertices: r.vertices.clone(),
            })
            .collect(),
        vertex_facets: skeleton.vertex_facets.clone(),
        trajectories,
        volume: vol,
        systolic_ratio,
    };

    let file =
        File::create(output).map_err(|e| format!("Cannot create {}: {e}", output.display()))?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, &export)
        .map_err(|e| format!("JSON serialization failed: {e}"))?;

    eprintln!(
        "Exported {}: {} vertices, {} edges, {} ridges, {} trajectories → {}",
        kp.name,
        export.vertex_count,
        export.edge_count,
        export.ridge_count,
        export.trajectories.len(),
        output.display()
    );

    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <polytope-name> <output-path>", args[0]);
        eprintln!("Available polytopes:");
        for kp in known_polytopes::all_known() {
            eprintln!("  - {}", kp.name);
        }
        std::process::exit(1);
    }

    let name = &args[1];
    let output_path = Path::new(&args[2]);

    match export(name, output_path) {
        Ok(()) => {}
        Err(e) => {
            eprintln!("Error: {e}");
            std::process::exit(1);
        }
    }
}

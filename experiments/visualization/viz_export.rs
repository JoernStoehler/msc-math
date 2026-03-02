/// JSON export of polytope geometry for the interactive visualization.
///
/// Exports the full combinatorial skeleton (vertices, edges, ridges),
/// Reeb vectors, and Reeb orbit trajectories as a single JSON file
/// consumed by the Three.js viewer in `experiments/viz/`.
///
/// Trajectories include:
/// - All closed Reeb orbits found by the HK2017 algorithm (min-action and others)
/// - Displaced variants of the min-action orbit to illustrate twisting
use nalgebra::Vector4;
use serde::Serialize;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use symplectic::algorithms::hk2017::recover::{recover_base_point, verify_orbit};
use symplectic::algorithms::hk2017::{
    build_directed_adjacency_matrix, combinations, ehz_capacity, EhzResult,
};
use symplectic::algorithms::hk2017::permutations::for_each_cyclic_permutation;
use symplectic::geom::reeb_trajectory;
use symplectic::known_polytopes::{self, KnownPolytope};
use symplectic::kkt::{solve_kkt, EPS_BETA_POSITIVE, EPS_Q_POSITIVE};
use symplectic::Skeleton;

/// Maximum number of orbits to export per polytope (keeps data.js manageable).
const MAX_ORBITS: usize = 20;

/// Displacement magnitudes for displaced orbit visualization.
const DISPLACEMENT_EPSILONS: &[f64] = &[0.02, 0.08];

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
    /// Facet normals, each `[n₁, n₂, n₃, n₄]`.
    pub normals: Vec<[f64; 4]>,
    /// Facet heights.
    pub heights: Vec<f64>,
    /// Reeb flow directions per facet: `reeb_vectors[i] = J₀ · normals[i]`.
    /// (The full Reeb vector is R_i = (2/h_i) J₀ n_i, but we export only
    /// the direction for visualization, since the factor 2/h_i rescales time.)
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
    permutation: Vec<usize>, // physical (reversed) ordering
    beta: Vec<f64>,          // matching permutation order
    subset: Vec<usize>,
}

/// Check if a cyclic permutation forms an adjacent cycle.
/// Copied from crates/src/algorithms/hk2017/mod.rs (3 lines, not pub).
fn is_adjacent_cycle(perm: &[usize], adj: &[Vec<bool>]) -> bool {
    let m = perm.len();
    (0..m).all(|k| adj[perm[k]][perm[(k + 1) % m]])
}

/// Collect ALL valid Reeb orbits for the polytope.
///
/// Uses the same algorithm as `ehz_capacity` (directed adjacency pruning),
/// but collects all certified orbits instead of just the best one.
/// Returns orbits sorted by action (ascending).
fn collect_all_orbits(polytope: &symplectic::Polytope4D) -> Vec<CollectedOrbit> {
    let f = polytope.facet_count();
    let normals = polytope.normals();
    let heights = polytope.heights();
    let adj = build_directed_adjacency_matrix(polytope);

    let mut orbits: Vec<CollectedOrbit> = Vec::new();

    for m in 2..=f {
        for subset in combinations(f, m) {
            for_each_cyclic_permutation(&subset, &mut |perm| {
                if !is_adjacent_cycle(perm, &adj) {
                    return;
                }

                if let Some(result) = solve_kkt(normals, heights, perm) {
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

                    // Reverse from algebraic to physical ordering
                    let mut phys_perm = perm.to_vec();
                    phys_perm.reverse();
                    let mut phys_beta = result.beta;
                    phys_beta.reverse();

                    orbits.push(CollectedOrbit {
                        action,
                        permutation: phys_perm,
                        beta: phys_beta,
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

/// Convert a collected orbit into an EhzResult for use with recover_base_point.
fn orbit_to_ehz_result(orbit: &CollectedOrbit) -> EhzResult {
    EhzResult {
        capacity: orbit.action,
        capacity_uncertain: orbit.action,
        best_subset: orbit.subset.clone(),
        best_permutation: orbit.permutation.clone(),
        best_beta: orbit.beta.clone(),
        iterations: 0,
    }
}

/// Recover a Reeb orbit and convert to VizTrajectory.
///
/// Returns None if recovery fails or the orbit has excessive violations.
fn orbit_to_viz_trajectory(
    polytope: &symplectic::Polytope4D,
    orbit: &CollectedOrbit,
    label: String,
) -> Option<VizTrajectory> {
    let result = orbit_to_ehz_result(orbit);
    let recovery = recover_base_point(polytope, &result)?;
    let verification = verify_orbit(polytope, &result, &recovery);

    // Validate orbit quality
    if verification.closure_error > 1e-6 {
        eprintln!(
            "  WARN orbit {}: closure_error={:.2e} (too large, skipping)",
            label, verification.closure_error
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

    let sigma = &result.best_permutation;
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

/// Compute a displacement direction tangent to a facet and perpendicular to its Reeb vector.
///
/// Returns a unit vector in the tangent space of facet `facet_idx` that is
/// perpendicular to the Reeb direction on that facet.
fn displacement_direction(
    polytope: &symplectic::Polytope4D,
    facet_idx: usize,
) -> Option<Vector4<f64>> {
    let n = &polytope.normals()[facet_idx];
    let reeb = reeb_trajectory::reeb_vector(n);

    // Find a vector perpendicular to both n (facet normal) and reeb (flow direction).
    // The facet tangent space is 3D (perpendicular to n). Within it, perpendicular
    // to the Reeb direction gives a 2D subspace. We pick one vector from it.
    //
    // Strategy: try standard basis vectors, project out n and reeb components.
    let candidates = [
        Vector4::new(1.0, 0.0, 0.0, 0.0),
        Vector4::new(0.0, 1.0, 0.0, 0.0),
        Vector4::new(0.0, 0.0, 1.0, 0.0),
        Vector4::new(0.0, 0.0, 0.0, 1.0),
    ];

    let n_unit = n.normalize();
    let reeb_unit = reeb.normalize();

    for e in &candidates {
        // Project out n component
        let mut v = e - n_unit.dot(e) * n_unit;
        // Project out reeb component
        v -= reeb_unit.dot(&v) * reeb_unit;
        let norm = v.norm();
        if norm > 1e-8 {
            return Some(v / norm);
        }
    }
    None
}

/// Generate displaced trajectories by perturbing the base point of an orbit.
fn generate_displaced_trajectories(
    polytope: &symplectic::Polytope4D,
    orbit: &CollectedOrbit,
    skeleton: &Skeleton,
) -> Vec<VizTrajectory> {
    let result = orbit_to_ehz_result(orbit);
    let recovery = match recover_base_point(polytope, &result) {
        Some(r) => r,
        None => return vec![],
    };

    let start_facet = result.best_permutation[0];
    let disp = match displacement_direction(polytope, start_facet) {
        Some(d) => d,
        None => return vec![],
    };

    let _ = skeleton; // skeleton available if needed for facet_centroid, unused here

    let mut trajectories = Vec::new();
    for &eps in DISPLACEMENT_EPSILONS {
        let displaced_start = recovery.base_point + eps * disp;

        let traj = reeb_trajectory::simulate(polytope, displaced_start, start_facet, 200, 1e-6);
        if traj.segments.is_empty() {
            continue;
        }

        trajectories.push(VizTrajectory {
            label: format!("displaced (ε={eps})"),
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
fn generate_trajectories(
    polytope: &symplectic::Polytope4D,
    skeleton: &Skeleton,
) -> Vec<VizTrajectory> {
    // Skip polytopes with too many facets (exponential cost)
    if polytope.facet_count() > MAX_FACETS_FOR_ORBIT {
        eprintln!(
            "  Skipping orbit computation (F={}, too many facets). Using placeholder.",
            polytope.facet_count()
        );
        return generate_placeholder_trajectory(polytope, skeleton);
    }

    // Collect all valid orbits
    let all_orbits = collect_all_orbits(polytope);
    if all_orbits.is_empty() {
        eprintln!("  No valid orbits found. Using placeholder.");
        return generate_placeholder_trajectory(polytope, skeleton);
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
        return generate_placeholder_trajectory(polytope, skeleton);
    }

    trajectories
}

/// Fallback: generate a single forward-simulated trajectory (the old behavior).
fn generate_placeholder_trajectory(
    polytope: &symplectic::Polytope4D,
    skeleton: &Skeleton,
) -> Vec<VizTrajectory> {
    for fi in 0..polytope.facet_count() {
        let centroid = reeb_trajectory::facet_centroid(polytope, skeleton, fi);
        let traj = reeb_trajectory::simulate(polytope, centroid, fi, 100, 1e-6);

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
fn lookup_known(name: &str) -> Option<KnownPolytope> {
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

    // Reeb vectors
    let reeb_vectors: Vec<[f64; 4]> = polytope
        .normals()
        .iter()
        .map(|n| v4_to_array(&reeb_trajectory::reeb_vector(n)))
        .collect();

    // Trajectories: real Reeb orbits + displaced variants
    eprintln!("Computing orbits for {}...", kp.name);
    let trajectories = generate_trajectories(polytope, &skeleton);

    // Compute capacity from the orbit data (use ehz_capacity for consistency)
    let capacity = if polytope.facet_count() <= MAX_FACETS_FOR_ORBIT {
        ehz_capacity(polytope).map_or(kp.capacity, |r| r.capacity)
    } else {
        kp.capacity
    };

    // Compute volume and systolic ratio
    let vol = symplectic::volume(polytope).unwrap_or(0.0);
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
        vertex_count: polytope.vertices().len(),
        edge_count: skeleton.edges.len(),
        ridge_count: skeleton.ridges.len(),
        normals: polytope.normals().iter().map(v4_to_array).collect(),
        heights: polytope.heights().to_vec(),
        reeb_vectors,
        vertices: polytope.vertices().iter().map(v4_to_array).collect(),
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

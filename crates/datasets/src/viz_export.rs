/// JSON export of polytope geometry for the interactive visualization.
///
/// Exports the full combinatorial skeleton (vertices, edges, ridges),
/// Reeb vectors, and sample Reeb trajectories as a single JSON file
/// consumed by the Three.js viewer in `experiments/viz/`.
use geom::known_polytopes::{self, KnownPolytope};
use geom::reeb_trajectory;
use geom::skeleton::Skeleton;
use nalgebra::Vector4;
use serde::Serialize;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

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
    /// Reeb vectors per facet: `reeb_vectors[i] = J₀ · normals[i]`.
    pub reeb_vectors: Vec<[f64; 4]>,
    /// Vertices, each `[x₁, x₂, x₃, x₄]`.
    pub vertices: Vec<[f64; 4]>,
    /// Edges as pairs of vertex indices.
    pub edges: Vec<[usize; 2]>,
    /// 2-faces (ridges).
    pub ridges: Vec<VizRidge>,
    /// Per-vertex facet incidence.
    pub vertex_facets: Vec<Vec<usize>>,
    /// Sample Reeb trajectories from different starting points.
    pub trajectories: Vec<VizTrajectory>,
}

#[derive(Serialize)]
pub struct VizRidge {
    pub facets: [usize; 2],
    pub vertices: Vec<usize>,
}

#[derive(Serialize)]
pub struct VizTrajectory {
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

    // Trajectories: one per facet, starting from facet vertex centroid
    let trajectories = generate_trajectories(polytope, &skeleton);

    let export = VizExport {
        name: kp.name.to_string(),
        source: kp.source.to_string(),
        capacity: kp.capacity,
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
    };

    let file = File::create(output).map_err(|e| format!("Cannot create {}: {e}", output.display()))?;
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

fn generate_trajectories(
    polytope: &geom::polytope::Polytope4D,
    skeleton: &Skeleton,
) -> Vec<VizTrajectory> {
    let mut trajectories = Vec::new();

    for fi in 0..polytope.facet_count() {
        let centroid = reeb_trajectory::facet_centroid(polytope, skeleton, fi);

        let traj = reeb_trajectory::simulate(polytope, skeleton, centroid, fi, 100, 1e-6);

        if traj.segments.is_empty() {
            continue;
        }

        trajectories.push(VizTrajectory {
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
        });
    }

    trajectories
}

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

mod models;
mod orbit_collection;
mod trajectories;

use euclidean_polytopes::volume_from_incidence_exact;
use models::{v4_to_array, VizExport, VizRidge};
use nalgebra::{DMatrix, Vector4};
use num_rational::BigRational;
use num_traits::ToPrimitive;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use symplectic::geom::known_polytopes::{self, KnownPolytope};
use symplectic::geom::skeleton::Skeleton;
use trajectories::generate_trajectories;

fn euclidean_volume_f64(vertices: &[[BigRational; 4]], incidence: &DMatrix<bool>) -> f64 {
    let vertices: Vec<Vector4<BigRational>> = vertices
        .iter()
        .map(|v| Vector4::new(v[0].clone(), v[1].clone(), v[2].clone(), v[3].clone()))
        .collect();
    ToPrimitive::to_f64(&volume_from_incidence_exact(&vertices, incidence)).unwrap_or(f64::NAN)
}

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

    let reeb_vectors: Vec<[f64; 4]> = polytope
        .dual_vertices_f64()
        .iter()
        .map(|a| v4_to_array(&symplectic::geom::reeb_trajectory::reeb_direction(a)))
        .collect();

    eprintln!("Computing orbits for {}...", kp.name);
    let (trajectories, computed_capacity) = generate_trajectories(polytope, &skeleton);

    let capacity = computed_capacity.unwrap_or(kp.capacity);
    let vol = euclidean_volume_f64(polytope.vertices(), polytope.incidence());
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
        dual_vertices: polytope
            .dual_vertices_f64()
            .iter()
            .map(v4_to_array)
            .collect(),
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

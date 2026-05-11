//! JSON export of polytope geometry for the interactive visualization.
//!
//! Exports the full combinatorial data (vertices, edges, two-faces),
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

use euclidean_polytopes::{
    edges_from_vertex_facet_incidence, two_faces_from_vertex_facet_incidence,
    vertex_facets_from_vertex_facet_incidence, volume_from_incidence_exact,
};
use models::{v4_to_array, VizExport, VizTwoFace};
use nalgebra::{DMatrix, Vector4};
use num_rational::BigRational;
use num_traits::ToPrimitive;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use symplectic::geom::known_polytopes::{self, KnownPolytope};
use trajectories::generate_trajectories;

const EPS_BASIS_DEGENERATE: f64 = 1e-12;
const EPS_COLLINEAR: f64 = 1e-10;

fn euclidean_volume_f64(vertices: &[[BigRational; 4]], incidence: &DMatrix<bool>) -> f64 {
    let vertices: Vec<Vector4<BigRational>> = vertices
        .iter()
        .map(|v| Vector4::new(v[0].clone(), v[1].clone(), v[2].clone(), v[3].clone()))
        .collect();
    ToPrimitive::to_f64(&volume_from_incidence_exact(&vertices, incidence)).unwrap_or(f64::NAN)
}

fn sort_two_face_vertices(all_vertices: &[Vector4<f64>], indices: &[usize]) -> Vec<usize> {
    if indices.len() < 3 {
        return indices.to_vec();
    }

    let coords: Vec<Vector4<f64>> = indices.iter().map(|&index| all_vertices[index]).collect();
    let centroid = coords.iter().copied().sum::<Vector4<f64>>() / coords.len() as f64;
    let d1_raw = coords[0] - centroid;
    let d1_norm = d1_raw.norm();
    if d1_norm < EPS_BASIS_DEGENERATE {
        return indices.to_vec();
    }
    let d1 = d1_raw / d1_norm;

    let Some(d2) = coords.iter().skip(1).find_map(|vertex| {
        let rel = *vertex - centroid;
        let proj = rel - d1 * rel.dot(&d1);
        (proj.norm() > EPS_COLLINEAR).then(|| proj.normalize())
    }) else {
        return indices.to_vec();
    };

    let mut indexed_angles: Vec<(f64, usize)> = coords
        .iter()
        .enumerate()
        .map(|(position, vertex)| {
            let rel = *vertex - centroid;
            (rel.dot(&d2).atan2(rel.dot(&d1)), position)
        })
        .collect();
    indexed_angles.sort_by(|left, right| left.0.partial_cmp(&right.0).unwrap());
    indexed_angles
        .into_iter()
        .map(|(_, position)| indices[position])
        .collect()
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

    let incidence = &kp.vertex_facet_incidence;
    let vertices = &kp.vertices_f64;
    let edges = edges_from_vertex_facet_incidence(incidence);
    let two_faces = two_faces_from_vertex_facet_incidence(incidence);
    let vertex_facets = vertex_facets_from_vertex_facet_incidence(incidence);

    let reeb_vectors: Vec<[f64; 4]> = kp
        .dual_vertices_f64
        .iter()
        .map(|a| v4_to_array(&symplectic::geom::reeb_trajectory::reeb_direction(a)))
        .collect();

    eprintln!("Computing orbits for {}...", kp.name);
    let (trajectories, computed_capacity) = generate_trajectories(
        &kp.dual_vertices_f64,
        &kp.vertices_f64,
        &kp.vertex_facet_incidence,
        &kp.facet_intersection_is_nonempty,
        &kp.omega_signs,
    );

    let capacity = computed_capacity.unwrap_or(kp.capacity);
    let vol = euclidean_volume_f64(&kp.vertices, &kp.vertex_facet_incidence);
    let systolic_ratio = if vol > 0.0 {
        capacity * capacity / (2.0 * vol)
    } else {
        0.0
    };

    let export = VizExport {
        name: kp.name.to_string(),
        source: kp.source.to_string(),
        capacity,
        facet_count: kp.facet_count(),
        vertex_count: vertices.len(),
        edge_count: edges.len(),
        two_face_count: two_faces.len(),
        dual_vertices: kp.dual_vertices_f64.iter().map(v4_to_array).collect(),
        reeb_vectors,
        vertices: vertices.iter().map(v4_to_array).collect(),
        edges,
        two_faces: two_faces
            .iter()
            .map(|two_face| VizTwoFace {
                facets: two_face.facets,
                vertices: sort_two_face_vertices(vertices, &two_face.vertices),
            })
            .collect(),
        vertex_facets,
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
        export.two_face_count,
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

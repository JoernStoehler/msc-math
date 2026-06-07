//! Emit flow-graph tube visualization data as JSON.
//!
//! Default target: the current f64 mismatch row
//! `facet_count=7, master_seed=20260605, attempt=31, sigma=[0,4,2,6]`.

use exp_combinatorial_cells::flat_polytope::CellPolytopeCache;
use serde::Serialize;
use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::path::PathBuf;
use symplectic::algorithms::facet_adjacency::build_transition_matrix_from_facet_intersections_and_omega;
use symplectic::algorithms::flow_graph::{
    closed_tube_visualization_snapshot_f64, face_polygon_snapshot_f64, FacePolygonSnapshot,
    FlatTubeInput, TubeVisualizationSnapshot,
};
use symplectic::random::generate_dual_vertices;

const DEFAULT_MASTER_SEED: u64 = 20260605;
const DEFAULT_FACET_COUNT: usize = 7;
const DEFAULT_ATTEMPT: u64 = 31;
const DEFAULT_SIGMA: &[usize] = &[0, 4, 2, 6];
const H_MIN: f64 = 0.5;
const H_MAX: f64 = 2.0;

#[derive(Debug)]
struct Args {
    facet_count: usize,
    master_seed: u64,
    attempt: u64,
    sigma: Vec<usize>,
    cutoff: f64,
    output: Option<PathBuf>,
}

#[derive(Debug, Serialize)]
struct VisualizationData {
    facet_count: usize,
    master_seed: u64,
    attempt: u64,
    sigma: Vec<usize>,
    cutoff: f64,
    dual_vertices_f64: Vec<[f64; 4]>,
    facet_intersection_is_nonempty: Vec<Vec<bool>>,
    omega_signs: Vec<Vec<i8>>,
    all_two_faces: Vec<FacePolygonSnapshot>,
    closed_tube: Option<TubeVisualizationSnapshot>,
}

fn main() -> Result<(), String> {
    let args = parse_args();
    let dual_vertices = generate_dual_vertices(
        args.facet_count,
        H_MIN,
        H_MAX,
        args.master_seed,
        args.attempt,
    )
    .map_err(|error| format!("generate dual vertices: {error:?}"))?;
    let polytope = CellPolytopeCache::from_f64(dual_vertices)
        .ok_or_else(|| "generated polytope was rejected by CellPolytopeCache".to_string())?;
    let input = FlatTubeInput::new(
        &polytope.dual_vertices_f64,
        &polytope.facet_intersection_is_nonempty,
        &polytope.omega_signs,
    );
    let transition_is_allowed = build_transition_matrix_from_facet_intersections_and_omega(
        &polytope.facet_intersection_is_nonempty,
        &polytope.omega_signs,
    );

    let mut all_two_faces = Vec::new();
    for first in 0..polytope.facet_count() {
        for second in 0..polytope.facet_count() {
            if first == second {
                continue;
            }
            if !transition_is_allowed[(first, second)] {
                continue;
            }
            all_two_faces.push(
                face_polygon_snapshot_f64(&input, first, second)
                    .map_err(|error| format!("snapshot face [{first},{second}]: {error:?}"))?,
            );
        }
    }

    let closed_tube = closed_tube_visualization_snapshot_f64(&input, &args.sigma, args.cutoff)
        .map_err(|error| format!("snapshot closed tube {:?}: {error:?}", args.sigma))?;

    let data = VisualizationData {
        facet_count: polytope.facet_count(),
        master_seed: args.master_seed,
        attempt: args.attempt,
        sigma: args.sigma,
        cutoff: args.cutoff,
        dual_vertices_f64: polytope
            .dual_vertices_f64
            .iter()
            .map(|a| [a[0], a[1], a[2], a[3]])
            .collect(),
        facet_intersection_is_nonempty: matrix_rows(&polytope.facet_intersection_is_nonempty),
        omega_signs: matrix_rows(&polytope.omega_signs),
        all_two_faces,
        closed_tube,
    };

    write_json(&data, args.output.as_ref())?;
    Ok(())
}

fn matrix_rows<T: Copy>(matrix: &nalgebra::DMatrix<T>) -> Vec<Vec<T>> {
    (0..matrix.nrows())
        .map(|row| (0..matrix.ncols()).map(|col| matrix[(row, col)]).collect())
        .collect()
}

fn write_json(data: &VisualizationData, output: Option<&PathBuf>) -> Result<(), String> {
    match output {
        Some(path) => {
            let file = File::create(path)
                .map_err(|error| format!("create output {}: {error}", path.display()))?;
            serde_json::to_writer_pretty(BufWriter::new(file), data)
                .map_err(|error| format!("serialize output: {error}"))?;
        }
        None => {
            let stdout = io::stdout();
            let mut handle = stdout.lock();
            serde_json::to_writer_pretty(&mut handle, data)
                .map_err(|error| format!("serialize stdout: {error}"))?;
            writeln!(handle).map_err(|error| format!("write newline: {error}"))?;
        }
    }
    Ok(())
}

fn parse_args() -> Args {
    let mut args = Args {
        facet_count: DEFAULT_FACET_COUNT,
        master_seed: DEFAULT_MASTER_SEED,
        attempt: DEFAULT_ATTEMPT,
        sigma: DEFAULT_SIGMA.to_vec(),
        cutoff: f64::INFINITY,
        output: None,
    };
    let mut iter = std::env::args().skip(1);
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--facet-count" => {
                args.facet_count = iter
                    .next()
                    .expect("--facet-count needs a value")
                    .parse()
                    .expect("--facet-count must be usize");
            }
            "--master-seed" => {
                args.master_seed = iter
                    .next()
                    .expect("--master-seed needs a value")
                    .parse()
                    .expect("--master-seed must be u64");
            }
            "--attempt" => {
                args.attempt = iter
                    .next()
                    .expect("--attempt needs a value")
                    .parse()
                    .expect("--attempt must be u64");
            }
            "--sigma" => {
                args.sigma = iter
                    .next()
                    .expect("--sigma needs a comma-separated value")
                    .split(',')
                    .map(|part| part.parse().expect("sigma entries must be usize"))
                    .collect();
            }
            "--cutoff" => {
                let value = iter.next().expect("--cutoff needs a value");
                args.cutoff = if value == "inf" || value == "infinity" {
                    f64::INFINITY
                } else {
                    value.parse().expect("--cutoff must be f64")
                };
            }
            "--output" => {
                args.output = Some(PathBuf::from(iter.next().expect("--output needs a path")));
            }
            "--help" | "-h" => {
                println!(
                    "Usage: flow-graph-visualize-tube-data [--facet-count N] [--master-seed N] [--attempt N] [--sigma 0,4,2,6] [--cutoff inf] [--output PATH]"
                );
                std::process::exit(0);
            }
            other => panic!("unknown argument {other}"),
        }
    }
    args
}

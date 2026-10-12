use exp_dev_quadratic_program::{load_retained_artifact_cases, ScanCase};
use nalgebra::{Matrix4, Vector4};
use serde::Serialize;

const EPS_DET: f64 = 1e-12;
const EPS_INEQUALITY: f64 = 1e-9;

fn main() {
    let args = Args::parse();
    let cases = load_cases(args.max_rows_per_family);
    let mut emitted = 0usize;
    for case in cases {
        for event in near_singular_events(&case) {
            println!(
                "{}",
                serde_json::to_string(&event).expect("serialize near-singular event")
            );
            emitted += 1;
            if emitted == args.max_events {
                return;
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Args {
    max_rows_per_family: usize,
    max_events: usize,
}

impl Args {
    fn parse() -> Self {
        let mut args = std::env::args().skip(1);
        let mut parsed = Self {
            max_rows_per_family: 8,
            max_events: 20,
        };
        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--max-rows-per-family" => {
                    parsed.max_rows_per_family = args
                        .next()
                        .expect("--max-rows-per-family value")
                        .parse()
                        .expect("--max-rows-per-family is usize");
                }
                "--max-events" => {
                    parsed.max_events = args
                        .next()
                        .expect("--max-events value")
                        .parse()
                        .expect("--max-events is usize");
                }
                "--help" | "-h" => {
                    eprintln!(
                        "usage: f64-capacity-near-singular [--max-rows-per-family N] [--max-events N]"
                    );
                    std::process::exit(0);
                }
                _ => panic!("unknown argument: {arg}"),
            }
        }
        parsed
    }
}

#[derive(Serialize)]
struct NearSingularEvent {
    family: String,
    source_id: String,
    facet_count: usize,
    facets: [usize; 4],
    determinant: f64,
    least_squares_residual_norm: Option<f64>,
    least_squares_max_abs_coord: Option<f64>,
    recovered_vertex_count: usize,
    first_recovered_vertex: Option<[f64; 4]>,
    first_recovered_vertex_max_abs_coord: Option<f64>,
    bounded_recovered_vertex_count_1e3: usize,
}

fn near_singular_events(case: &ScanCase) -> Vec<NearSingularEvent> {
    let recovered_vertices = recovered_feasible_vertices(&case.dual_vertices);
    let mut events = Vec::new();
    for i in 0..case.dual_vertices.len() {
        for j in i + 1..case.dual_vertices.len() {
            for k in j + 1..case.dual_vertices.len() {
                for l in k + 1..case.dual_vertices.len() {
                    let facets = [i, j, k, l];
                    let matrix = facet_matrix(&case.dual_vertices, facets);
                    let determinant = matrix.determinant();
                    if determinant.abs() > EPS_DET {
                        continue;
                    }
                    let least_squares = least_squares_solution(matrix);
                    let matching_vertices = recovered_vertices
                        .iter()
                        .filter(|vertex| all_facets_active(&case.dual_vertices, vertex, facets))
                        .collect::<Vec<_>>();
                    let first = matching_vertices
                        .first()
                        .map(|vertex| vector_to_array(vertex));
                    events.push(NearSingularEvent {
                        family: case.family.clone(),
                        source_id: case.source_id.clone(),
                        facet_count: case.dual_vertices.len(),
                        facets,
                        determinant,
                        least_squares_residual_norm: least_squares
                            .as_ref()
                            .map(|solution| (matrix * solution - Vector4::repeat(1.0)).norm()),
                        least_squares_max_abs_coord: least_squares.as_ref().map(max_abs_coord),
                        recovered_vertex_count: matching_vertices.len(),
                        first_recovered_vertex: first,
                        first_recovered_vertex_max_abs_coord: matching_vertices
                            .first()
                            .map(|vertex| max_abs_coord(vertex)),
                        bounded_recovered_vertex_count_1e3: matching_vertices
                            .iter()
                            .filter(|vertex| max_abs_coord(vertex) <= 1e3)
                            .count(),
                    });
                }
            }
        }
    }
    events
}

fn recovered_feasible_vertices(dual_vertices: &[Vector4<f64>]) -> Vec<Vector4<f64>> {
    let mut vertices = Vec::new();
    for i in 0..dual_vertices.len() {
        for j in i + 1..dual_vertices.len() {
            for k in j + 1..dual_vertices.len() {
                for l in k + 1..dual_vertices.len() {
                    let facets = [i, j, k, l];
                    let matrix = facet_matrix(dual_vertices, facets);
                    if matrix.determinant().abs() <= EPS_DET {
                        continue;
                    }
                    let Some(vertex) = matrix.lu().solve(&Vector4::repeat(1.0)) else {
                        continue;
                    };
                    if is_feasible(dual_vertices, &vertex)
                        && !vertices
                            .iter()
                            .any(|known: &Vector4<f64>| (known - vertex).norm() <= 1e-8)
                    {
                        vertices.push(vertex);
                    }
                }
            }
        }
    }
    vertices
}

fn least_squares_solution(matrix: Matrix4<f64>) -> Option<Vector4<f64>> {
    matrix
        .svd(true, true)
        .solve(&Vector4::repeat(1.0), EPS_DET)
        .ok()
}

fn facet_matrix(dual_vertices: &[Vector4<f64>], facets: [usize; 4]) -> Matrix4<f64> {
    let rows = facets.map(|idx| dual_vertices[idx]);
    Matrix4::new(
        rows[0][0], rows[0][1], rows[0][2], rows[0][3], rows[1][0], rows[1][1], rows[1][2],
        rows[1][3], rows[2][0], rows[2][1], rows[2][2], rows[2][3], rows[3][0], rows[3][1],
        rows[3][2], rows[3][3],
    )
}

fn all_facets_active(
    dual_vertices: &[Vector4<f64>],
    vertex: &Vector4<f64>,
    facets: [usize; 4],
) -> bool {
    facets
        .iter()
        .all(|&facet| (dual_vertices[facet].dot(vertex) - 1.0).abs() <= EPS_INEQUALITY)
}

fn is_feasible(dual_vertices: &[Vector4<f64>], vertex: &Vector4<f64>) -> bool {
    dual_vertices
        .iter()
        .all(|normal| normal.dot(vertex) - 1.0 <= EPS_INEQUALITY)
}

fn vector_to_array(vector: &Vector4<f64>) -> [f64; 4] {
    [vector[0], vector[1], vector[2], vector[3]]
}

fn max_abs_coord(vector: &Vector4<f64>) -> f64 {
    vector.iter().map(|entry| entry.abs()).fold(0.0, f64::max)
}

fn load_cases(max_rows_per_family: usize) -> Vec<ScanCase> {
    load_retained_artifact_cases(max_rows_per_family)
}

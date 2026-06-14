use exp_dev_f64_capacity::{array_vertices_to_vectors, ScanCase};
use nalgebra::{Matrix4, Vector4};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use symplectic::known_polytopes;

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

#[derive(Deserialize)]
struct RandomRow {
    name: String,
    facet_count: usize,
    dual_vertices: Vec<[f64; 4]>,
    capacity: f64,
}

#[derive(Deserialize)]
struct RandomProductRow {
    name: String,
    facet_count: usize,
    dual_vertices: Vec<[f64; 4]>,
    capacity: f64,
}

#[derive(Deserialize)]
struct AscentEndpointRow {
    name: String,
    facet_count: usize,
    final_dual_vertices: Vec<[f64; 4]>,
    #[serde(default)]
    final_capacity: f64,
}

fn load_cases(max_rows_per_family: usize) -> Vec<ScanCase> {
    let produce = repo_root().join("experiments/sys-datascience/produce");
    let mut cases = Vec::new();
    cases.extend(load_random_rows(
        &produce.join("random.jsonl"),
        "random",
        max_rows_per_family,
    ));
    cases.extend(load_random_product_rows(
        &produce.join("random-product.jsonl"),
        "random_product",
        max_rows_per_family,
    ));
    cases.extend(load_ascent_rows(
        &produce.join("ascent-general-endpoints.jsonl"),
        "ascent_general_endpoint",
        max_rows_per_family,
    ));
    cases.extend(load_ascent_rows(
        &produce.join("ascent-product-endpoints.jsonl"),
        "ascent_product_endpoint",
        max_rows_per_family,
    ));
    cases.push(hko_case());
    cases
}

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("experiments/dev-f64-capacity should have a repo-root grandparent")
        .to_path_buf()
}

fn hko_case() -> ScanCase {
    let hko = known_polytopes::hko_pentagon();
    ScanCase {
        family: "hko2024_f64".to_string(),
        source_id: "known_polytopes::hko_pentagon_rounded_f64".to_string(),
        input_source: "hard_fixture".to_string(),
        generated_attempt: None,
        generator_seed: None,
        requested_facet_count: Some(hko.dual_vertices_f64.len()),
        dual_vertices: hko.dual_vertices_f64.clone(),
        audit_capacity_label: Some(hko.capacity),
        artifact_capacity_label: Some(hko.capacity),
        audit_sigma_label: None,
    }
}

fn load_random_rows(path: &Path, family: &str, max_rows_per_family: usize) -> Vec<ScanCase> {
    let rows = read_jsonl::<RandomRow>(path);
    select_spread_by_facet_count(rows, max_rows_per_family, |row| row.facet_count)
        .map(|row| ScanCase {
            family: family.to_string(),
            source_id: format!("{}:F{}", row.name, row.facet_count),
            input_source: "artifact_replay".to_string(),
            generated_attempt: None,
            generator_seed: None,
            requested_facet_count: Some(row.facet_count),
            dual_vertices: array_vertices_to_vectors(&row.dual_vertices),
            audit_capacity_label: Some(row.capacity),
            artifact_capacity_label: Some(row.capacity),
            audit_sigma_label: None,
        })
        .collect()
}

fn load_random_product_rows(
    path: &Path,
    family: &str,
    max_rows_per_family: usize,
) -> Vec<ScanCase> {
    let rows = read_jsonl::<RandomProductRow>(path);
    select_spread_by_facet_count(rows, max_rows_per_family, |row| row.facet_count)
        .map(|row| ScanCase {
            family: family.to_string(),
            source_id: format!("{}:F{}", row.name, row.facet_count),
            input_source: "artifact_replay".to_string(),
            generated_attempt: None,
            generator_seed: None,
            requested_facet_count: Some(row.facet_count),
            dual_vertices: array_vertices_to_vectors(&row.dual_vertices),
            audit_capacity_label: Some(row.capacity),
            artifact_capacity_label: Some(row.capacity),
            audit_sigma_label: None,
        })
        .collect()
}

fn load_ascent_rows(path: &Path, family: &str, max_rows_per_family: usize) -> Vec<ScanCase> {
    read_jsonl::<AscentEndpointRow>(path)
        .into_iter()
        .take(row_limit(max_rows_per_family))
        .map(|row| ScanCase {
            family: family.to_string(),
            source_id: format!("{}:F{}", row.name, row.facet_count),
            input_source: "artifact_replay".to_string(),
            generated_attempt: None,
            generator_seed: None,
            requested_facet_count: Some(row.facet_count),
            dual_vertices: array_vertices_to_vectors(&row.final_dual_vertices),
            audit_capacity_label: (row.final_capacity > 0.0).then_some(row.final_capacity),
            artifact_capacity_label: (row.final_capacity > 0.0).then_some(row.final_capacity),
            audit_sigma_label: None,
        })
        .collect()
}

fn row_limit(max_rows_per_family: usize) -> usize {
    if max_rows_per_family == 0 {
        usize::MAX
    } else {
        max_rows_per_family
    }
}

fn select_spread_by_facet_count<T>(
    rows: Vec<T>,
    max_rows_per_family: usize,
    facet_count: impl Fn(&T) -> usize,
) -> std::vec::IntoIter<T> {
    if max_rows_per_family == 0 || rows.len() <= max_rows_per_family {
        return rows.into_iter();
    }

    let selected = selected_row_indices(&rows, max_rows_per_family, facet_count);
    let mut rows_by_index = rows.into_iter().map(Some).collect::<Vec<_>>();
    selected
        .into_iter()
        .map(|idx| rows_by_index[idx].take().expect("selected row exists"))
        .collect::<Vec<_>>()
        .into_iter()
}

fn selected_row_indices<T>(
    rows: &[T],
    max_rows_per_family: usize,
    facet_count: impl Fn(&T) -> usize,
) -> Vec<usize> {
    let mut selected = Vec::new();
    let mut used = vec![false; rows.len()];
    let mut seen_facets = Vec::new();
    for (idx, row) in rows.iter().enumerate() {
        let facet = facet_count(row);
        if !seen_facets.contains(&facet) {
            selected.push(idx);
            used[idx] = true;
            seen_facets.push(facet);
            if selected.len() == max_rows_per_family {
                return selected;
            }
        }
    }
    for (idx, was_used) in used.iter().enumerate() {
        if !was_used {
            selected.push(idx);
            if selected.len() == max_rows_per_family {
                break;
            }
        }
    }
    selected
}

fn read_jsonl<T: for<'de> Deserialize<'de>>(path: &Path) -> Vec<T> {
    let file = File::open(path).unwrap_or_else(|e| panic!("open {}: {e}", path.display()));
    let reader = BufReader::new(file);
    reader
        .lines()
        .enumerate()
        .filter_map(|(line_idx, line)| {
            let line =
                line.unwrap_or_else(|e| panic!("read {}:{}: {e}", path.display(), line_idx + 1));
            let line = line.trim();
            (!line.is_empty()).then(|| {
                serde_json::from_str(line).unwrap_or_else(|e| {
                    panic!("parse {}:{} as JSON: {e}", path.display(), line_idx + 1)
                })
            })
        })
        .collect()
}

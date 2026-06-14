use crate::{
    array_vertices_to_vectors, product_preprocess::round_known_product_dual_vertices, ScanCase,
};
use serde::Deserialize;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use symplectic::known_polytopes;

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

pub fn load_retained_artifact_cases(max_rows_per_family: usize) -> Vec<ScanCase> {
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

pub fn hko_case() -> ScanCase {
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

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("experiments/dev-f64-capacity should have a repo-root grandparent")
        .to_path_buf()
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
            dual_vertices: round_known_product_dual_vertices(&array_vertices_to_vectors(
                &row.dual_vertices,
            )),
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
            dual_vertices: if family == "ascent_product_endpoint" {
                round_known_product_dual_vertices(&array_vertices_to_vectors(
                    &row.final_dual_vertices,
                ))
            } else {
                array_vertices_to_vectors(&row.final_dual_vertices)
            },
            audit_capacity_label: (row.final_capacity > 0.0).then_some(row.final_capacity),
            artifact_capacity_label: (row.final_capacity > 0.0).then_some(row.final_capacity),
            audit_sigma_label: None,
        })
        .collect()
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

use crate::{array_vertices_to_vectors, product::round_known_product_dual_vertices, ScanCase};
use serde::Deserialize;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use symplectic::known_polytopes;

const HKO_FAMILY: &str = "hko2024_f64";
const HKO_SOURCE_ID: &str = "known_polytopes::hko_pentagon_rounded_f64";

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

pub fn load_retained_artifact_cases(max_rows_per_family: usize) -> Vec<ScanCase> {
    load_retained_artifact_cases_filtered(max_rows_per_family, &[], &[])
}

pub fn load_retained_artifact_cases_filtered(
    max_rows_per_family: usize,
    family_filter: &[String],
    source_id_filter: &[String],
) -> Vec<ScanCase> {
    let produce = repo_root().join("experiments/polytope-datasets");
    let mut cases = Vec::new();
    let row_cap = retained_artifact_row_cap(max_rows_per_family, source_id_filter);

    if should_load_jsonl_family("random", family_filter, source_id_filter) {
        cases.extend(load_random_rows(
            &produce.join("random.jsonl"),
            "random",
            row_cap,
        ));
    }
    if should_load_jsonl_family("random_product", family_filter, source_id_filter) {
        cases.extend(load_random_product_rows(
            &produce.join("random-product.jsonl"),
            "random_product",
            row_cap,
        ));
    }
    if filters_allow_case(HKO_FAMILY, HKO_SOURCE_ID, family_filter, source_id_filter) {
        cases.push(hko_case());
    }

    cases
        .into_iter()
        .filter(|case| {
            filters_allow_case(
                &case.family,
                &case.source_id,
                family_filter,
                source_id_filter,
            )
        })
        .collect()
}

fn should_load_jsonl_family(
    family: &str,
    family_filter: &[String],
    source_id_filter: &[String],
) -> bool {
    family_filter_allows(family, family_filter)
        && !source_filter_needs_no_jsonl_artifacts(source_id_filter)
}

fn filters_allow_case(
    family: &str,
    source_id: &str,
    family_filter: &[String],
    source_id_filter: &[String],
) -> bool {
    family_filter_allows(family, family_filter)
        && source_id_filter_allows(source_id, source_id_filter)
}

fn family_filter_allows(family: &str, family_filter: &[String]) -> bool {
    family_filter.is_empty() || family_filter.iter().any(|item| item == family)
}

fn source_id_filter_allows(source_id: &str, source_id_filter: &[String]) -> bool {
    source_id_filter.is_empty() || source_id_filter.iter().any(|item| item == source_id)
}

fn source_filter_needs_no_jsonl_artifacts(source_id_filter: &[String]) -> bool {
    !source_id_filter.is_empty()
        && source_id_filter.iter().all(|source_id| {
            source_id == HKO_SOURCE_ID
                || source_id.starts_with("edge:")
                || source_id.starts_with("seed")
        })
}

fn retained_artifact_row_cap(max_rows_per_family: usize, source_id_filter: &[String]) -> usize {
    if source_id_filter.is_empty() {
        max_rows_per_family
    } else {
        0
    }
}

pub fn hko_case() -> ScanCase {
    let hko = known_polytopes::hko_pentagon();
    ScanCase {
        family: HKO_FAMILY.to_string(),
        source_id: HKO_SOURCE_ID.to_string(),
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
        .ancestors()
        .nth(2)
        .expect("experiments/dev-quadratic-program should live under repo-root/experiments")
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

#[cfg(test)]
mod tests {
    use super::*;

    fn filter(value: &str) -> Vec<String> {
        vec![value.to_string()]
    }

    #[test]
    fn hko_source_filter_loads_hard_fixture_without_artifact_jsonl() {
        let cases = load_retained_artifact_cases_filtered(1, &[], &filter(HKO_SOURCE_ID));

        assert_eq!(cases.len(), 1);
        assert_eq!(cases[0].family, HKO_FAMILY);
        assert_eq!(cases[0].source_id, HKO_SOURCE_ID);
        assert_eq!(cases[0].input_source, "hard_fixture");
    }

    #[test]
    fn hko_family_filter_loads_hard_fixture_without_artifact_jsonl() {
        let cases = load_retained_artifact_cases_filtered(1, &filter(HKO_FAMILY), &[]);

        assert_eq!(cases.len(), 1);
        assert_eq!(cases[0].family, HKO_FAMILY);
        assert_eq!(cases[0].input_source, "hard_fixture");
    }

    #[test]
    fn family_and_source_filters_must_both_match() {
        let cases =
            load_retained_artifact_cases_filtered(1, &filter("random"), &filter(HKO_SOURCE_ID));

        assert!(cases.is_empty());
    }

    #[test]
    fn non_artifact_source_filters_do_not_require_artifact_jsonl_payloads() {
        assert!(load_retained_artifact_cases_filtered(
            1,
            &[],
            &filter("edge:duplicate_dual_vertices")
        )
        .is_empty());
        assert!(load_retained_artifact_cases_filtered(
            1,
            &[],
            &filter("seed99540836:F5:sample0:attempt0")
        )
        .is_empty());
    }

    #[test]
    fn source_id_filter_disables_pre_filter_row_cap() {
        assert_eq!(retained_artifact_row_cap(3, &[]), 3);
        assert_eq!(retained_artifact_row_cap(3, &filter("random-42:F8")), 0);
    }

    #[test]
    fn default_artifact_scan_uses_only_retained_families() {
        let families = load_retained_artifact_cases(1)
            .into_iter()
            .map(|case| case.family)
            .collect::<Vec<_>>();

        assert_eq!(families, ["random", "random_product", HKO_FAMILY]);
    }
}

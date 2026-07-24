//! Real-data branch/degeneracy diagnostic for gradient-ascent development.
//!
//! This command reads retained sys-landscape table rows, recomputes real
//! capacity/orbit data, and records how many admissible sigmas are close to
//! the minimum action under several relative windows.

use exp_sys_landscape::{compute_sys_from_capacity, SysLandscapePolytopeCache};
use nalgebra::Vector4;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fs::{self, File};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};
use std::time::{Instant, SystemTime, UNIX_EPOCH};
use symplectic::{
    aggregate_orbits_with_dual_vertices_exact, classify_facets_from_dual_vertices,
    solve_billiard_candidates, solve_pruned_hk2017_candidates, OrbitAdmissibility,
    OrbitGuaranteeMode, OrbitSearchError, OrbitSearchResult,
};

const DEFAULT_MAX_ROWS: usize = 24;
const DEFAULT_THRESHOLDS: &[f64] = &[1.0e-12, 1.0e-9, 1.0e-6, 1.0e-3, 1.0e-2];

#[derive(Debug)]
struct Cli {
    polytope_table: PathBuf,
    provenance_table: PathBuf,
    out_dir: PathBuf,
    max_rows: usize,
    thresholds_relative: Vec<f64>,
}

#[derive(Clone, Debug, Deserialize)]
struct PolytopeRow {
    poly_id: String,
    facet_count: usize,
    capacity: f64,
    volume: f64,
    sys: f64,
    dual_vertices_f64: Vec<[f64; 4]>,
}

#[derive(Clone, Debug, Deserialize)]
struct ProvenanceRow {
    poly_id: String,
    dataset: String,
    role: String,
    source_name: String,
    seed_index: Option<usize>,
    best_strategy: Option<String>,
}

#[derive(Clone, Debug)]
struct SelectedRow {
    polytope: PolytopeRow,
    provenance: Vec<ProvenanceRow>,
    selection_buckets: BTreeSet<String>,
}

#[derive(Serialize)]
struct FixtureSelectionRow {
    poly_id: String,
    selection_buckets: Vec<String>,
    datasets: Vec<String>,
    roles: Vec<String>,
    source_names: Vec<String>,
    seed_indices: Vec<usize>,
    best_strategies: Vec<String>,
    input_facet_count: usize,
    input_capacity: f64,
    input_volume: f64,
    input_sys: f64,
}

#[derive(Serialize)]
struct BranchSetDiagnosticRow {
    poly_id: String,
    selection_buckets: Vec<String>,
    datasets: Vec<String>,
    input_facet_count: usize,
    input_sys: f64,
    recomputed_sys: Option<f64>,
    recomputed_sys_delta: Option<f64>,
    threshold_relative: f64,
    min_action: Option<f64>,
    returned_orbit_count: Option<usize>,
    admissible_orbit_count: Option<usize>,
    near_active_raw_orbit_count: Option<usize>,
    near_active_raw_sigma_lengths: Vec<usize>,
    near_active_raw_sigmas: Vec<Vec<usize>>,
    near_active_distinct_cyclic_class_count: Option<usize>,
    near_active_canonical_cyclic_sigmas: Vec<Vec<usize>>,
    action_gap_to_second: Option<f64>,
    action_gap_to_last_near_active: Option<f64>,
    degeneracy_label: String,
    orbit_iterations: Option<u64>,
    failure: Option<String>,
}

#[derive(Serialize)]
struct ComputeBudgetReport {
    command: String,
    polytope_table: String,
    provenance_table: String,
    selected_rows: usize,
    thresholds_relative: Vec<f64>,
    max_action_gap_relative: f64,
    successful_recomputations: usize,
    failed_recomputations: usize,
    total_orbit_iterations: u64,
    elapsed_ms: f64,
}

#[derive(Serialize)]
struct Summary {
    method: String,
    selected_rows: usize,
    diagnostic_rows: usize,
    successful_recomputations: usize,
    failed_recomputations: usize,
    thresholds_relative: Vec<f64>,
    max_action_gap_relative: f64,
    selection_bucket_counts: BTreeMap<String, usize>,
    dataset_counts: BTreeMap<String, usize>,
    degeneracy_counts: BTreeMap<String, usize>,
    out_dir: String,
    caveat: String,
}

pub fn run_from_args(argv: impl IntoIterator<Item = impl Into<String>>) {
    let cli = parse_args_from(argv);
    fs::create_dir_all(&cli.out_dir).expect("failed to create output directory");
    let t0 = Instant::now();

    let polytopes: Vec<PolytopeRow> = load_jsonl(&cli.polytope_table);
    let provenance_rows: Vec<ProvenanceRow> = load_jsonl(&cli.provenance_table);
    let provenance_by_poly_id = provenance_by_poly_id(provenance_rows);
    let selected = select_rows(&polytopes, &provenance_by_poly_id, cli.max_rows);
    let max_action_gap_relative = cli
        .thresholds_relative
        .iter()
        .copied()
        .filter(|value| value.is_finite() && *value >= 0.0)
        .max_by(|a, b| a.total_cmp(b))
        .unwrap_or(0.0);

    let fixture_rows: Vec<FixtureSelectionRow> =
        selected.values().map(fixture_selection_row).collect();

    let mut diagnostic_rows = Vec::new();
    let mut successful_recomputations = 0usize;
    let mut failed_recomputations = 0usize;
    let mut total_orbit_iterations = 0u64;

    for row in selected.values() {
        match recompute_state(&row.polytope, max_action_gap_relative) {
            Ok(state) => {
                successful_recomputations += 1;
                total_orbit_iterations += state.orbit_iterations;
                for &threshold in &cli.thresholds_relative {
                    diagnostic_rows.push(branch_diagnostic_row(row, threshold, Ok(&state)));
                }
            }
            Err(err) => {
                failed_recomputations += 1;
                for &threshold in &cli.thresholds_relative {
                    diagnostic_rows.push(branch_diagnostic_row(row, threshold, Err(err.as_str())));
                }
            }
        }
    }

    write_jsonl(cli.out_dir.join("fixture-selection.jsonl"), &fixture_rows)
        .expect("failed to write fixture-selection.jsonl");
    write_jsonl(
        cli.out_dir.join("branch-set-diagnostic.jsonl"),
        &diagnostic_rows,
    )
    .expect("failed to write branch-set-diagnostic.jsonl");

    let report = ComputeBudgetReport {
        command: "dev-gradient-ascent-branch-diagnostic".to_string(),
        polytope_table: cli.polytope_table.display().to_string(),
        provenance_table: cli.provenance_table.display().to_string(),
        selected_rows: selected.len(),
        thresholds_relative: cli.thresholds_relative.clone(),
        max_action_gap_relative,
        successful_recomputations,
        failed_recomputations,
        total_orbit_iterations,
        elapsed_ms: t0.elapsed().as_secs_f64() * 1000.0,
    };
    write_json(cli.out_dir.join("compute-budget-report.json"), &report)
        .expect("failed to write compute-budget-report.json");

    let summary = Summary {
        method: "dev-gradient-ascent-branch-diagnostic".to_string(),
        selected_rows: selected.len(),
        diagnostic_rows: diagnostic_rows.len(),
        successful_recomputations,
        failed_recomputations,
        thresholds_relative: cli.thresholds_relative,
        max_action_gap_relative,
        selection_bucket_counts: count_selection_buckets(selected.values()),
        dataset_counts: count_datasets(selected.values()),
        degeneracy_counts: count_degeneracy_labels(&diagnostic_rows),
        out_dir: cli.out_dir.display().to_string(),
        caveat: "branch/degeneracy diagnostic only; this does not certify local maximality"
            .to_string(),
    };
    write_json(cli.out_dir.join("summary.json"), &summary).expect("failed to write summary.json");

    println!("{}", cli.out_dir.display());
}

#[derive(Debug)]
struct RecomputedState {
    sys: f64,
    min_action: f64,
    returned_orbit_count: usize,
    orbit_iterations: u64,
    admissible_actions_and_sigmas: Vec<(f64, Vec<usize>)>,
}

fn recompute_state(
    row: &PolytopeRow,
    max_action_gap_relative: f64,
) -> Result<RecomputedState, String> {
    let dual_vertices: Vec<Vector4<f64>> = row
        .dual_vertices_f64
        .iter()
        .map(|v| Vector4::new(v[0], v[1], v[2], v[3]))
        .collect();
    let polytope = SysLandscapePolytopeCache::from_f64_dual_vertices(dual_vertices)
        .ok_or_else(|| "failed_to_construct_polytope".to_string())?;
    let action_gap = (row.capacity * max_action_gap_relative).max(0.0);
    let capacity = capacity_auto_with_gap(&polytope, action_gap)
        .map_err(|err| format!("failed_to_compute_capacity_with_gap:{err:?}"))?;
    let sys = compute_sys_from_capacity(&polytope, &capacity)
        .ok_or_else(|| "failed_to_compute_sys_from_capacity".to_string())?;
    let mut admissible_actions_and_sigmas: Vec<(f64, Vec<usize>)> = capacity
        .orbits
        .iter()
        .filter(|orbit| {
            matches!(
                orbit.admissibility,
                OrbitAdmissibility::AdmissibleF64 | OrbitAdmissibility::AdmissibleExact
            )
        })
        .map(|orbit| (orbit.action, orbit.sigma.clone()))
        .collect();
    admissible_actions_and_sigmas.sort_by(|a, b| a.0.total_cmp(&b.0));

    Ok(RecomputedState {
        sys,
        min_action: capacity.min_action,
        returned_orbit_count: capacity.orbits.len(),
        orbit_iterations: capacity.iterations,
        admissible_actions_and_sigmas,
    })
}

fn capacity_auto_with_gap(
    polytope: &SysLandscapePolytopeCache,
    action_gap: f64,
) -> Result<OrbitSearchResult, OrbitSearchError> {
    if classify_facets_from_dual_vertices(&polytope.dual_vertices_f64).is_ok() {
        let classification = classify_facets_from_dual_vertices(&polytope.dual_vertices_f64)
            .map_err(|_| OrbitSearchError::NumericalFailure)?;
        let transition_is_allowed =
            symplectic::algorithms::facet_adjacency::build_transition_matrix_from_facet_intersections_and_omega(
                &polytope.facet_intersection_is_nonempty,
                &polytope.omega_signs,
            );
        let (orbits, iterations) = solve_billiard_candidates(
            &polytope.dual_vertices_f64,
            &classification.q_indices,
            &classification.p_indices,
            &polytope.facet_intersection_is_nonempty,
            &transition_is_allowed,
        )?;
        return aggregate_orbits_with_dual_vertices_exact(
            &polytope.dual_vertices,
            orbits,
            iterations,
            action_gap,
            OrbitGuaranteeMode::AllSafe,
        );
    }

    let transition_is_allowed =
        symplectic::algorithms::facet_adjacency::build_transition_matrix_from_facet_intersections_and_omega(
            &polytope.facet_intersection_is_nonempty,
            &polytope.omega_signs,
        );
    let (orbits, iterations) =
        solve_pruned_hk2017_candidates(&polytope.dual_vertices_f64, &transition_is_allowed)?;
    aggregate_orbits_with_dual_vertices_exact(
        &polytope.dual_vertices,
        orbits,
        iterations,
        action_gap,
        OrbitGuaranteeMode::AllSafe,
    )
}

fn branch_diagnostic_row(
    row: &SelectedRow,
    threshold: f64,
    state: Result<&RecomputedState, &str>,
) -> BranchSetDiagnosticRow {
    let selection_buckets = sorted_set(&row.selection_buckets);
    let datasets = row_datasets(&row.provenance);
    match state {
        Ok(state) => {
            let cutoff = state.min_action * (1.0 + threshold);
            let near_active: Vec<(f64, Vec<usize>)> = state
                .admissible_actions_and_sigmas
                .iter()
                .cloned()
                .filter(|(action, _)| *action <= cutoff)
                .collect();
            let action_gap_to_second = state
                .admissible_actions_and_sigmas
                .get(1)
                .map(|(action, _)| action - state.min_action);
            let action_gap_to_last_near_active = near_active
                .last()
                .map(|(action, _)| action - state.min_action);
            let near_active_raw_sigmas: Vec<Vec<usize>> =
                near_active.iter().map(|(_, sigma)| sigma.clone()).collect();
            let near_active_canonical_cyclic_sigmas =
                distinct_cyclic_class_representatives(&near_active_raw_sigmas);
            let degeneracy_label = degeneracy_label(near_active_canonical_cyclic_sigmas.len());
            BranchSetDiagnosticRow {
                poly_id: row.polytope.poly_id.clone(),
                selection_buckets,
                datasets,
                input_facet_count: row.polytope.facet_count,
                input_sys: row.polytope.sys,
                recomputed_sys: Some(state.sys),
                recomputed_sys_delta: Some(state.sys - row.polytope.sys),
                threshold_relative: threshold,
                min_action: Some(state.min_action),
                returned_orbit_count: Some(state.returned_orbit_count),
                admissible_orbit_count: Some(state.admissible_actions_and_sigmas.len()),
                near_active_raw_orbit_count: Some(near_active_raw_sigmas.len()),
                near_active_raw_sigma_lengths: near_active_raw_sigmas
                    .iter()
                    .map(Vec::len)
                    .collect(),
                near_active_raw_sigmas,
                near_active_distinct_cyclic_class_count: Some(
                    near_active_canonical_cyclic_sigmas.len(),
                ),
                near_active_canonical_cyclic_sigmas,
                action_gap_to_second,
                action_gap_to_last_near_active,
                degeneracy_label,
                orbit_iterations: Some(state.orbit_iterations),
                failure: None,
            }
        }
        Err(err) => BranchSetDiagnosticRow {
            poly_id: row.polytope.poly_id.clone(),
            selection_buckets,
            datasets,
            input_facet_count: row.polytope.facet_count,
            input_sys: row.polytope.sys,
            recomputed_sys: None,
            recomputed_sys_delta: None,
            threshold_relative: threshold,
            min_action: None,
            returned_orbit_count: None,
            admissible_orbit_count: None,
            near_active_raw_orbit_count: None,
            near_active_raw_sigma_lengths: Vec::new(),
            near_active_raw_sigmas: Vec::new(),
            near_active_distinct_cyclic_class_count: None,
            near_active_canonical_cyclic_sigmas: Vec::new(),
            action_gap_to_second: None,
            action_gap_to_last_near_active: None,
            degeneracy_label: "inconclusive".to_string(),
            orbit_iterations: None,
            failure: Some(err.to_string()),
        },
    }
}

/// Returns the lexicographically smallest cyclic rotation of a nonempty sigma word.
///
/// All `word.len()` rotations are compared, so repeated minima need no separate
/// tie-breaking assumption. The result is one deterministic representative of
/// the cyclic class; it does not alter the raw word returned by the orbit solver.
fn canonical_cyclic_rotation(word: &[usize]) -> Vec<usize> {
    assert!(
        !word.is_empty(),
        "sigma words must be nonempty to have a cyclic rotation"
    );
    (0..word.len())
        .map(|start| {
            word.iter()
                .cycle()
                .skip(start)
                .take(word.len())
                .copied()
                .collect()
        })
        .min()
        .expect("nonempty sigma has at least one rotation")
}

fn distinct_cyclic_class_representatives(words: &[Vec<usize>]) -> Vec<Vec<usize>> {
    words
        .iter()
        .map(|word| canonical_cyclic_rotation(word))
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

fn degeneracy_label(near_active_count: usize) -> String {
    match near_active_count {
        0 => "inconclusive".to_string(),
        1 => "large_gap".to_string(),
        2..=4 => "narrow_gap".to_string(),
        _ => "high_degeneracy".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::{canonical_cyclic_rotation, distinct_cyclic_class_representatives};

    #[test]
    fn canonical_rotation_identifies_all_rotations() {
        assert_eq!(canonical_cyclic_rotation(&[3, 1, 2]), vec![1, 2, 3]);
        assert_eq!(canonical_cyclic_rotation(&[2, 3, 1]), vec![1, 2, 3]);
        assert_eq!(canonical_cyclic_rotation(&[1, 2, 3]), vec![1, 2, 3]);
    }

    #[test]
    fn canonical_rotation_handles_repeated_minima() {
        assert_eq!(canonical_cyclic_rotation(&[2, 1, 3, 1]), vec![1, 2, 1, 3]);
        assert_eq!(canonical_cyclic_rotation(&[1, 3, 1, 2]), vec![1, 2, 1, 3]);
    }

    #[test]
    fn distinct_representatives_deduplicate_rotations_only() {
        let representatives = distinct_cyclic_class_representatives(&[
            vec![3, 1, 2],
            vec![2, 3, 1],
            vec![2, 1, 3],
            vec![1, 3, 2],
        ]);
        assert_eq!(representatives, vec![vec![1, 2, 3], vec![1, 3, 2]]);
    }
}

fn select_rows(
    polytopes: &[PolytopeRow],
    provenance_by_poly_id: &HashMap<String, Vec<ProvenanceRow>>,
    max_rows: usize,
) -> BTreeMap<String, SelectedRow> {
    let per_bucket = (max_rows / 4).max(1);
    let mut selected = BTreeMap::new();

    let mut by_sys: Vec<&PolytopeRow> = polytopes.iter().collect();
    by_sys.sort_by(|a, b| b.sys.total_cmp(&a.sys).then(a.poly_id.cmp(&b.poly_id)));
    for row in by_sys.into_iter().take(per_bucket) {
        add_selected(&mut selected, row, provenance_by_poly_id, "top_sys");
    }

    select_by_dataset(
        polytopes,
        provenance_by_poly_id,
        &mut selected,
        "gradient_ascent_general",
        per_bucket,
    );
    select_by_dataset(
        polytopes,
        provenance_by_poly_id,
        &mut selected,
        "gradient_ascent_products",
        per_bucket,
    );
    select_by_dataset(
        polytopes,
        provenance_by_poly_id,
        &mut selected,
        "random_sample",
        per_bucket,
    );
    select_by_dataset(
        polytopes,
        provenance_by_poly_id,
        &mut selected,
        "random_product_sample",
        per_bucket,
    );

    if selected.len() > max_rows {
        selected.into_iter().take(max_rows).collect()
    } else {
        selected
    }
}

fn select_by_dataset(
    polytopes: &[PolytopeRow],
    provenance_by_poly_id: &HashMap<String, Vec<ProvenanceRow>>,
    selected: &mut BTreeMap<String, SelectedRow>,
    dataset: &str,
    limit: usize,
) {
    let mut candidates: Vec<&PolytopeRow> = polytopes
        .iter()
        .filter(|row| {
            provenance_by_poly_id
                .get(&row.poly_id)
                .is_some_and(|rows| rows.iter().any(|prov| prov.dataset == dataset))
        })
        .collect();
    candidates.sort_by(|a, b| b.sys.total_cmp(&a.sys).then(a.poly_id.cmp(&b.poly_id)));
    for row in candidates.into_iter().take(limit) {
        add_selected(selected, row, provenance_by_poly_id, dataset);
    }
}

fn add_selected(
    selected: &mut BTreeMap<String, SelectedRow>,
    row: &PolytopeRow,
    provenance_by_poly_id: &HashMap<String, Vec<ProvenanceRow>>,
    bucket: &str,
) {
    let provenance = provenance_by_poly_id
        .get(&row.poly_id)
        .cloned()
        .unwrap_or_default();
    selected
        .entry(row.poly_id.clone())
        .and_modify(|entry| {
            entry.selection_buckets.insert(bucket.to_string());
        })
        .or_insert_with(|| {
            let mut selection_buckets = BTreeSet::new();
            selection_buckets.insert(bucket.to_string());
            SelectedRow {
                polytope: row.clone(),
                provenance,
                selection_buckets,
            }
        });
}

fn fixture_selection_row(row: &SelectedRow) -> FixtureSelectionRow {
    FixtureSelectionRow {
        poly_id: row.polytope.poly_id.clone(),
        selection_buckets: sorted_set(&row.selection_buckets),
        datasets: row_datasets(&row.provenance),
        roles: row_roles(&row.provenance),
        source_names: row_source_names(&row.provenance),
        seed_indices: row_seed_indices(&row.provenance),
        best_strategies: row_best_strategies(&row.provenance),
        input_facet_count: row.polytope.facet_count,
        input_capacity: row.polytope.capacity,
        input_volume: row.polytope.volume,
        input_sys: row.polytope.sys,
    }
}

fn parse_args_from(argv: impl IntoIterator<Item = impl Into<String>>) -> Cli {
    let mut cli = Cli {
        polytope_table: default_tables_dir().join("polytope-table.jsonl"),
        provenance_table: default_tables_dir().join("polytope-provenance-table.jsonl"),
        out_dir: default_output_dir(),
        max_rows: DEFAULT_MAX_ROWS,
        thresholds_relative: DEFAULT_THRESHOLDS.to_vec(),
    };

    let mut args = argv.into_iter().map(Into::into).skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--polytope-table" => {
                cli.polytope_table =
                    PathBuf::from(args.next().expect("--polytope-table requires a path"));
            }
            "--provenance-table" => {
                cli.provenance_table =
                    PathBuf::from(args.next().expect("--provenance-table requires a path"));
            }
            "--out-dir" => {
                cli.out_dir = PathBuf::from(args.next().expect("--out-dir requires a path"));
            }
            "--max-rows" => {
                cli.max_rows = args
                    .next()
                    .expect("--max-rows requires an integer")
                    .parse()
                    .expect("--max-rows must be an integer");
            }
            "--thresholds-relative" => {
                cli.thresholds_relative = args
                    .next()
                    .expect("--thresholds-relative requires comma-separated f64 values")
                    .split(',')
                    .map(|value| {
                        value
                            .parse()
                            .expect("--thresholds-relative entries must be f64")
                    })
                    .collect();
            }
            "--help" | "-h" => {
                print_usage();
                std::process::exit(0);
            }
            other => panic!("unsupported argument: {other}"),
        }
    }
    cli
}

fn print_usage() {
    eprintln!(
        "Usage: dev-gradient-ascent-branch-diagnostic [--polytope-table PATH] [--provenance-table PATH] [--out-dir PATH] [--max-rows N] [--thresholds-relative CSV]"
    );
}

fn default_tables_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../polytope-invariant-table")
}

fn default_output_dir() -> PathBuf {
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock before UNIX_EPOCH")
        .as_millis();
    std::env::temp_dir().join(format!(
        "dev-gradient-ascent-branch-diagnostic-{}-{stamp}",
        std::process::id()
    ))
}

fn load_jsonl<T: for<'de> Deserialize<'de>>(path: &Path) -> Vec<T> {
    let file =
        File::open(path).unwrap_or_else(|err| panic!("failed to open {}: {err}", path.display()));
    let reader = BufReader::new(file);
    reader
        .lines()
        .enumerate()
        .filter_map(|(idx, line)| {
            let line = line.unwrap_or_else(|err| {
                panic!("failed to read {}:{}: {err}", path.display(), idx + 1)
            });
            (!line.trim().is_empty()).then(|| {
                serde_json::from_str(&line).unwrap_or_else(|err| {
                    panic!("failed to parse {}:{}: {err}", path.display(), idx + 1)
                })
            })
        })
        .collect()
}

fn provenance_by_poly_id(rows: Vec<ProvenanceRow>) -> HashMap<String, Vec<ProvenanceRow>> {
    let mut by_poly_id: HashMap<String, Vec<ProvenanceRow>> = HashMap::new();
    for row in rows {
        by_poly_id.entry(row.poly_id.clone()).or_default().push(row);
    }
    by_poly_id
}

fn row_datasets(rows: &[ProvenanceRow]) -> Vec<String> {
    sorted_strings(rows.iter().map(|row| row.dataset.clone()))
}

fn row_roles(rows: &[ProvenanceRow]) -> Vec<String> {
    sorted_strings(rows.iter().map(|row| row.role.clone()))
}

fn row_source_names(rows: &[ProvenanceRow]) -> Vec<String> {
    sorted_strings(rows.iter().map(|row| row.source_name.clone()))
}

fn row_seed_indices(rows: &[ProvenanceRow]) -> Vec<usize> {
    let mut values: Vec<usize> = rows.iter().filter_map(|row| row.seed_index).collect();
    values.sort_unstable();
    values.dedup();
    values
}

fn row_best_strategies(rows: &[ProvenanceRow]) -> Vec<String> {
    sorted_strings(rows.iter().filter_map(|row| row.best_strategy.clone()))
}

fn sorted_set(values: &BTreeSet<String>) -> Vec<String> {
    values.iter().cloned().collect()
}

fn sorted_strings(values: impl Iterator<Item = String>) -> Vec<String> {
    let mut values: Vec<String> = values.collect();
    values.sort();
    values.dedup();
    values
}

fn count_selection_buckets<'a>(
    rows: impl Iterator<Item = &'a SelectedRow>,
) -> BTreeMap<String, usize> {
    let mut counts = BTreeMap::new();
    for row in rows {
        for bucket in &row.selection_buckets {
            *counts.entry(bucket.clone()).or_insert(0) += 1;
        }
    }
    counts
}

fn count_datasets<'a>(rows: impl Iterator<Item = &'a SelectedRow>) -> BTreeMap<String, usize> {
    let mut counts = BTreeMap::new();
    for row in rows {
        for dataset in row_datasets(&row.provenance) {
            *counts.entry(dataset).or_insert(0) += 1;
        }
    }
    counts
}

fn count_degeneracy_labels(rows: &[BranchSetDiagnosticRow]) -> BTreeMap<String, usize> {
    let mut counts = BTreeMap::new();
    for row in rows {
        *counts.entry(row.degeneracy_label.clone()).or_insert(0) += 1;
    }
    counts
}

fn write_jsonl<P: AsRef<Path>, T: Serialize>(path: P, rows: &[T]) -> std::io::Result<()> {
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);
    for row in rows {
        serde_json::to_writer(&mut writer, row)?;
        writer.write_all(b"\n")?;
    }
    Ok(())
}

fn write_json<P: AsRef<Path>, T: Serialize>(path: P, value: &T) -> std::io::Result<()> {
    let file = File::create(path)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, value)?;
    Ok(())
}

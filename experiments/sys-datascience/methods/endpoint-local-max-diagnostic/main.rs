//! Endpoint first-order local-maximum diagnostic on a numerical quotient slice.

use exp_sys_landscape::{
    apply_dual_step, ascent_direction, compute_active_sys_state, compute_step_bound_detailed,
    AscentMode, SysLandscapePolytopeCache, MAX_STEP_SIZE,
};
use good_lp::{constraint, default_solver, variable, variables, Expression, Solution, SolverModel};
use nalgebra::{DMatrix, Matrix4, Vector4};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::env;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};
use symplectic::algorithms::{OrbitAdmissibility, OrbitKktData};
use symplectic::derivatives::{
    capacity_subgradients_a, systolic_ratio_gradient_a, volume_derivatives_a,
};

const ACTIVE_ORBIT_RTOL: f64 = 1.0e-9;
const RANK_RTOL: f64 = 1.0e-9;
const ASCENT_TOL: f64 = 1.0e-9;
const STEP_FRACTIONS: &[f64] = &[1.0e-6, 1.0e-5, 1.0e-4, 1.0e-3];
const OPTIMIZER_STEP_FRACTIONS: &[f64] = &[0.1, 0.25, 0.5, 0.75, 0.95];
const OPTIMIZER_OVERSHOOT_MULTIPLIERS: &[f64] = &[1.5, 2.0, 3.0];

#[derive(Debug)]
struct Cli {
    polytope_table: PathBuf,
    provenance_table: PathBuf,
    diagnostic_out: PathBuf,
    summary_out: PathBuf,
}

#[derive(Clone, Debug, Deserialize)]
struct PolytopeRow {
    poly_id: String,
    facet_count: usize,
    capacity: f64,
    volume: f64,
    sys: f64,
    dual_vertices_f64: Vec<[f64; 4]>,
    orbit_best_beta_margin: f64,
    orbit_best_q_error_bound: f64,
    orbit_result_returned_orbit_count: f64,
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
struct SampleRow {
    polytope: PolytopeRow,
    provenance: Vec<ProvenanceRow>,
    selection_buckets: BTreeSet<String>,
}

#[derive(Debug)]
struct ProbeOutcome {
    best_delta: Option<f64>,
    best_label: Option<String>,
    increased: Option<bool>,
}

#[derive(Debug, Serialize)]
struct DiagnosticRow {
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
    recomputed_capacity: Option<f64>,
    recomputed_volume: Option<f64>,
    recomputed_sys: Option<f64>,
    recomputed_sys_delta: Option<f64>,
    active_orbit_rtol: f64,
    active_orbit_count: Option<usize>,
    active_sigma_lengths: Vec<usize>,
    active_action_min: Option<f64>,
    active_action_max: Option<f64>,
    active_action_spread: Option<f64>,
    beta_margin_min: Option<f64>,
    q_error_bound_max: Option<f64>,
    table_best_beta_margin: f64,
    table_best_q_error_bound: f64,
    table_returned_orbit_count: f64,
    gradient_row_count: Option<usize>,
    gradient_norm_min: Option<f64>,
    gradient_norm_max: Option<f64>,
    gradient_norm_mean: Option<f64>,
    symmetry_rank: Option<usize>,
    ambient_dimension: usize,
    slice_dimension: Option<usize>,
    symmetry_condition_proxy: Option<f64>,
    max_abs_gradient_on_symmetry: Option<f64>,
    max_abs_direction_on_symmetry: Option<f64>,
    quotient_maximin: Option<f64>,
    quotient_ascent_found: bool,
    quotient_direction_norm: Option<f64>,
    quotient_model_status: String,
    step_probe_ran: bool,
    step_probe_best_sys: Option<f64>,
    step_probe_best_delta: Option<f64>,
    step_probe_best_fraction: Option<f64>,
    step_probe_increased: Option<bool>,
    optimizer_mode: String,
    optimizer_direction_status: String,
    optimizer_direction_norm: Option<f64>,
    optimizer_t_max: Option<f64>,
    optimizer_existing_line_search_best_delta: Option<f64>,
    optimizer_existing_line_search_best_label: Option<String>,
    optimizer_existing_line_search_increased: Option<bool>,
    optimizer_tiny_probe_best_delta: Option<f64>,
    optimizer_tiny_probe_best_label: Option<String>,
    optimizer_tiny_probe_increased: Option<bool>,
    failure: Option<String>,
}

#[derive(Debug, Serialize)]
struct Summary {
    method: String,
    polytope_table: String,
    provenance_table: String,
    active_orbit_rtol: f64,
    rank_rtol: f64,
    ascent_tol: f64,
    step_fractions: Vec<f64>,
    sampled_rows: usize,
    failures: usize,
    quotient_ascent_found: usize,
    step_probe_ran: usize,
    step_probe_increased: usize,
    sys_gt_1_after_probe: usize,
    selection_bucket_counts: BTreeMap<String, usize>,
    dataset_counts: BTreeMap<String, usize>,
    max_input_sys: Option<f64>,
    max_recomputed_sys: Option<f64>,
    max_step_probe_sys: Option<f64>,
    rows_with_ill_conditioned_or_empty_slice: usize,
    optimizer_mode_counts: BTreeMap<String, usize>,
    optimizer_existing_line_search_increased_counts: BTreeMap<String, usize>,
    optimizer_tiny_probe_increased_counts: BTreeMap<String, usize>,
}

fn default_method_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../sys-datascience/methods/endpoint-local-max-diagnostic")
}

fn default_tables_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../sys-datascience/tables")
}

fn parse_args() -> Cli {
    let tables_dir = default_tables_dir();
    let method_dir = default_method_dir();
    let mut cli = Cli {
        polytope_table: tables_dir.join("polytope-table.jsonl"),
        provenance_table: tables_dir.join("polytope-provenance-table.jsonl"),
        diagnostic_out: method_dir.join("diagnostic.jsonl"),
        summary_out: method_dir.join("summary.json"),
    };

    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        let next_path = |args: &mut std::iter::Skip<env::Args>, flag: &str| -> PathBuf {
            PathBuf::from(
                args.next()
                    .unwrap_or_else(|| panic!("{flag} requires a path")),
            )
        };
        match arg.as_str() {
            "--polytope-table" => cli.polytope_table = next_path(&mut args, "--polytope-table"),
            "--provenance-table" => {
                cli.provenance_table = next_path(&mut args, "--provenance-table")
            }
            "--diagnostic-out" => cli.diagnostic_out = next_path(&mut args, "--diagnostic-out"),
            "--summary-out" => cli.summary_out = next_path(&mut args, "--summary-out"),
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
        "Usage: sys-endpoint-local-max-diagnostic [--polytope-table PATH] [--provenance-table PATH] [--diagnostic-out PATH] [--summary-out PATH]"
    );
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

fn row_datasets(provenance: &[ProvenanceRow]) -> BTreeSet<String> {
    provenance.iter().map(|row| row.dataset.clone()).collect()
}

fn add_sample(
    selected: &mut BTreeMap<String, SampleRow>,
    polytope: &PolytopeRow,
    provenance: &[ProvenanceRow],
    bucket: &str,
) {
    selected
        .entry(polytope.poly_id.clone())
        .and_modify(|row| {
            row.selection_buckets.insert(bucket.to_string());
        })
        .or_insert_with(|| {
            let mut selection_buckets = BTreeSet::new();
            selection_buckets.insert(bucket.to_string());
            SampleRow {
                polytope: polytope.clone(),
                provenance: provenance.to_vec(),
                selection_buckets,
            }
        });
}

fn stable_hash_key(poly_id: &str, salt: &str) -> [u8; 32] {
    let mut hasher = blake3::Hasher::new();
    hasher.update(salt.as_bytes());
    hasher.update(poly_id.as_bytes());
    *hasher.finalize().as_bytes()
}

fn select_samples(
    polytopes: &[PolytopeRow],
    provenance_by_poly: &HashMap<String, Vec<ProvenanceRow>>,
) -> Vec<SampleRow> {
    let mut selected = BTreeMap::new();
    let mut by_sys = polytopes.to_vec();
    by_sys.sort_by(|a, b| {
        b.sys
            .total_cmp(&a.sys)
            .then_with(|| a.poly_id.cmp(&b.poly_id))
    });
    for row in by_sys.iter().take(30) {
        let provenance = provenance_by_poly
            .get(&row.poly_id)
            .cloned()
            .unwrap_or_default();
        add_sample(&mut selected, row, &provenance, "top_sys_30");
    }

    for (dataset, bucket, salt) in [
        (
            "gradient_ascent_general",
            "general_ascent_control_10",
            "endpoint-local-max-general-control",
        ),
        (
            "gradient_ascent_products",
            "product_ascent_control_10",
            "endpoint-local-max-product-control",
        ),
    ] {
        let mut rows: Vec<_> = polytopes
            .iter()
            .filter(|row| {
                provenance_by_poly
                    .get(&row.poly_id)
                    .map(|rows| row_datasets(rows).contains(dataset))
                    .unwrap_or(false)
            })
            .collect();
        rows.sort_by_key(|row| stable_hash_key(&row.poly_id, salt));
        for row in rows.into_iter().take(10) {
            let provenance = provenance_by_poly
                .get(&row.poly_id)
                .cloned()
                .unwrap_or_default();
            add_sample(&mut selected, row, &provenance, bucket);
        }
    }

    let mut non_ascent: Vec<_> = polytopes
        .iter()
        .filter(|row| {
            provenance_by_poly
                .get(&row.poly_id)
                .map(|rows| {
                    let datasets = row_datasets(rows);
                    datasets.contains("random_sample") || datasets.contains("random_product_sample")
                })
                .unwrap_or(false)
        })
        .collect();
    non_ascent
        .sort_by_key(|row| stable_hash_key(&row.poly_id, "endpoint-local-max-non-ascent-control"));
    for row in non_ascent.into_iter().take(10) {
        let provenance = provenance_by_poly
            .get(&row.poly_id)
            .cloned()
            .unwrap_or_default();
        add_sample(&mut selected, row, &provenance, "non_ascent_control_10");
    }

    selected.into_values().collect()
}

fn vec4_rows(data: &[[f64; 4]]) -> Vec<Vector4<f64>> {
    data.iter()
        .map(|row| Vector4::new(row[0], row[1], row[2], row[3]))
        .collect()
}

fn flatten(data: &[Vector4<f64>]) -> Vec<f64> {
    data.iter().flat_map(|v| [v[0], v[1], v[2], v[3]]).collect()
}

fn unflatten(data: &[f64]) -> Vec<Vector4<f64>> {
    data.chunks_exact(4)
        .map(|chunk| Vector4::new(chunk[0], chunk[1], chunk[2], chunk[3]))
        .collect()
}

fn active_orbits(result: &symplectic::algorithms::OrbitSearchResult) -> Vec<OrbitKktData> {
    let tol = ACTIVE_ORBIT_RTOL * result.min_action.abs().max(1.0);
    let active: Vec<OrbitKktData> = result
        .orbits
        .iter()
        .filter(|orbit| {
            matches!(
                orbit.admissibility,
                OrbitAdmissibility::AdmissibleF64 | OrbitAdmissibility::AdmissibleExact
            )
        })
        .filter(|orbit| (orbit.action - result.min_action).abs() <= tol)
        .cloned()
        .collect();

    if active.is_empty() {
        vec![result.best_orbit().clone()]
    } else {
        active
    }
}

fn gradient_norms(rows: &[Vec<Vector4<f64>>]) -> Vec<f64> {
    rows.iter()
        .map(|row| row.iter().map(|v| v.norm_squared()).sum::<f64>().sqrt())
        .collect()
}

fn min_max(values: &[f64]) -> (Option<f64>, Option<f64>) {
    (
        values.iter().copied().reduce(f64::min),
        values.iter().copied().reduce(f64::max),
    )
}

fn mean(values: &[f64]) -> Option<f64> {
    (!values.is_empty()).then(|| values.iter().sum::<f64>() / values.len() as f64)
}

fn sp4_generators() -> Vec<Matrix4<f64>> {
    fn mat(rows: [[f64; 4]; 4]) -> Matrix4<f64> {
        Matrix4::from_fn(|row, col| rows[row][col])
    }
    vec![
        mat([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, -1.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
        ]),
        mat([
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, -1.0, 0.0],
        ]),
        mat([
            [0.0, 0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, -1.0],
            [0.0, 0.0, 0.0, 0.0],
        ]),
        mat([
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, -1.0],
        ]),
        mat([
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
        ]),
        mat([
            [0.0, 0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
        ]),
        mat([
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
        ]),
        mat([
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
        ]),
        mat([
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0, 0.0],
        ]),
        mat([
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
        ]),
    ]
}

fn symmetry_tangent_matrix(dual_vertices: &[Vector4<f64>]) -> DMatrix<f64> {
    let dim = dual_vertices.len() * 4;
    let mut columns: Vec<Vec<f64>> = Vec::new();

    for coord in 0..4 {
        let mut column = Vec::with_capacity(dim);
        for dual in dual_vertices {
            let scalar = dual[coord];
            for idx in 0..4 {
                column.push(-scalar * dual[idx]);
            }
        }
        columns.push(column);
    }

    columns.push(flatten(
        &dual_vertices.iter().map(|dual| -dual).collect::<Vec<_>>(),
    ));

    for generator in sp4_generators() {
        let mut column = Vec::with_capacity(dim);
        for dual in dual_vertices {
            for out_coord in 0..4 {
                let mut value = 0.0;
                for in_coord in 0..4 {
                    value += generator[(in_coord, out_coord)] * dual[in_coord];
                }
                column.push(-value);
            }
        }
        columns.push(column);
    }

    DMatrix::from_fn(dim, columns.len(), |row, col| columns[col][row])
}

fn dot(lhs: &[f64], rhs: &[f64]) -> f64 {
    lhs.iter().zip(rhs).map(|(a, b)| a * b).sum()
}

fn norm(values: &[f64]) -> f64 {
    dot(values, values).sqrt()
}

fn orthogonalize_against(vector: &mut [f64], basis: &[Vec<f64>]) {
    for basis_vector in basis {
        let coeff = dot(vector, basis_vector);
        for (value, basis_value) in vector.iter_mut().zip(basis_vector) {
            *value -= coeff * basis_value;
        }
    }
}

fn numerical_slice_basis(symmetry: &DMatrix<f64>) -> (usize, DMatrix<f64>, Option<f64>) {
    let dim = symmetry.nrows();
    let svd = symmetry.clone().svd(false, false);
    let singular_values = svd.singular_values.as_slice().to_vec();
    let scale = singular_values.first().copied().unwrap_or(0.0).max(1.0);
    let threshold = RANK_RTOL * scale;
    let mut tangent_basis: Vec<Vec<f64>> = Vec::new();
    for col in 0..symmetry.ncols() {
        let mut vector: Vec<f64> = (0..dim).map(|row| symmetry[(row, col)]).collect();
        orthogonalize_against(&mut vector, &tangent_basis);
        let vector_norm = norm(&vector);
        if vector_norm > threshold {
            for value in &mut vector {
                *value /= vector_norm;
            }
            tangent_basis.push(vector);
        }
    }
    let rank = tangent_basis.len();
    let condition_proxy = match (
        singular_values.first().copied(),
        singular_values.get(rank.saturating_sub(1)).copied(),
    ) {
        (Some(max), Some(min_kept)) if min_kept > 0.0 => Some(max / min_kept),
        _ => None,
    };

    let mut slice_vectors: Vec<Vec<f64>> = Vec::new();
    let mut full_basis = tangent_basis.clone();
    for coord in 0..dim {
        let mut vector = vec![0.0; dim];
        vector[coord] = 1.0;
        orthogonalize_against(&mut vector, &full_basis);
        let vector_norm = norm(&vector);
        if vector_norm > threshold {
            for value in &mut vector {
                *value /= vector_norm;
            }
            full_basis.push(vector.clone());
            slice_vectors.push(vector);
        }
    }

    let basis = DMatrix::from_fn(dim, slice_vectors.len(), |row, col| slice_vectors[col][row]);
    (rank, basis, condition_proxy)
}

fn solve_quotient_maximin(
    gradients: &[Vec<Vector4<f64>>],
    slice_basis: &DMatrix<f64>,
) -> Option<(f64, Vec<Vector4<f64>>, f64)> {
    let slice_dim = slice_basis.ncols();
    if slice_dim == 0 || gradients.is_empty() {
        return None;
    }

    let mut vars = variables!();
    let y_vars: Vec<_> = (0..slice_dim)
        .map(|_| vars.add(variable().min(-1.0).max(1.0)))
        .collect();
    let t_var = vars.add(variable().min(f64::NEG_INFINITY));
    let mut model = vars.maximise(Expression::from(t_var)).using(default_solver);

    for gradient in gradients {
        let flat = flatten(gradient);
        let mut lhs = Expression::from(0.0);
        for col in 0..slice_dim {
            let coeff: f64 = flat
                .iter()
                .enumerate()
                .map(|(row, value)| value * slice_basis[(row, col)])
                .sum();
            if coeff != 0.0 {
                lhs += coeff * y_vars[col];
            }
        }
        model = model.with(constraint!(lhs >= t_var));
    }

    let solution = model.solve().ok()?;
    let maximin = solution.value(t_var);
    let y: Vec<f64> = y_vars.iter().map(|var| solution.value(*var)).collect();
    let flat_direction: Vec<f64> = (0..slice_basis.nrows())
        .map(|row| {
            (0..slice_dim)
                .map(|col| slice_basis[(row, col)] * y[col])
                .sum()
        })
        .collect();
    let direction_norm = flat_direction
        .iter()
        .map(|value| value * value)
        .sum::<f64>()
        .sqrt();
    Some((maximin, unflatten(&flat_direction), direction_norm))
}

fn max_abs_row_on_columns(row: &[f64], matrix: &DMatrix<f64>) -> f64 {
    (0..matrix.ncols())
        .map(|col| {
            (0..matrix.nrows())
                .map(|matrix_row| row[matrix_row] * matrix[(matrix_row, col)])
                .sum::<f64>()
                .abs()
        })
        .fold(0.0, f64::max)
}

fn probe_steps(
    polytope: &SysLandscapePolytopeCache,
    direction: &[Vector4<f64>],
    base_sys: f64,
    steps: &[(String, f64)],
) -> ProbeOutcome {
    let mut best: Option<(String, f64)> = None;
    for (label, t) in steps {
        if *t <= 0.0 || !t.is_finite() {
            continue;
        }
        if let Some((_stepped, stepped_sys)) =
            apply_dual_step(&polytope.dual_vertices_f64, direction, *t)
        {
            let delta = stepped_sys - base_sys;
            if best
                .as_ref()
                .is_none_or(|(_, best_delta)| delta > *best_delta)
            {
                best = Some((label.clone(), delta));
            }
        }
    }

    match best {
        Some((label, delta)) => ProbeOutcome {
            best_delta: Some(delta),
            best_label: Some(label),
            increased: Some(delta > ASCENT_TOL),
        },
        None => ProbeOutcome {
            best_delta: None,
            best_label: None,
            increased: Some(false),
        },
    }
}

fn diagnostic_for_sample(sample: &SampleRow) -> DiagnosticRow {
    let datasets = row_datasets(&sample.provenance)
        .into_iter()
        .collect::<Vec<_>>();
    let roles = sample
        .provenance
        .iter()
        .map(|row| row.role.clone())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    let source_names = sample
        .provenance
        .iter()
        .map(|row| row.source_name.clone())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    let seed_indices = sample
        .provenance
        .iter()
        .filter_map(|row| row.seed_index)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    let best_strategies = sample
        .provenance
        .iter()
        .filter_map(|row| row.best_strategy.clone())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    let ambient_dimension = sample.polytope.facet_count * 4;

    let failure_row = |failure: String| DiagnosticRow {
        poly_id: sample.polytope.poly_id.clone(),
        selection_buckets: sample.selection_buckets.iter().cloned().collect(),
        datasets: datasets.clone(),
        roles: roles.clone(),
        source_names: source_names.clone(),
        seed_indices: seed_indices.clone(),
        best_strategies: best_strategies.clone(),
        input_facet_count: sample.polytope.facet_count,
        input_capacity: sample.polytope.capacity,
        input_volume: sample.polytope.volume,
        input_sys: sample.polytope.sys,
        recomputed_capacity: None,
        recomputed_volume: None,
        recomputed_sys: None,
        recomputed_sys_delta: None,
        active_orbit_rtol: ACTIVE_ORBIT_RTOL,
        active_orbit_count: None,
        active_sigma_lengths: Vec::new(),
        active_action_min: None,
        active_action_max: None,
        active_action_spread: None,
        beta_margin_min: None,
        q_error_bound_max: None,
        table_best_beta_margin: sample.polytope.orbit_best_beta_margin,
        table_best_q_error_bound: sample.polytope.orbit_best_q_error_bound,
        table_returned_orbit_count: sample.polytope.orbit_result_returned_orbit_count,
        gradient_row_count: None,
        gradient_norm_min: None,
        gradient_norm_max: None,
        gradient_norm_mean: None,
        symmetry_rank: None,
        ambient_dimension,
        slice_dimension: None,
        symmetry_condition_proxy: None,
        max_abs_gradient_on_symmetry: None,
        max_abs_direction_on_symmetry: None,
        quotient_maximin: None,
        quotient_ascent_found: false,
        quotient_direction_norm: None,
        quotient_model_status: "failed_before_quotient_lp".to_string(),
        step_probe_ran: false,
        step_probe_best_sys: None,
        step_probe_best_delta: None,
        step_probe_best_fraction: None,
        step_probe_increased: None,
        optimizer_mode: "not_evaluated".to_string(),
        optimizer_direction_status: "failed_before_direction".to_string(),
        optimizer_direction_norm: None,
        optimizer_t_max: None,
        optimizer_existing_line_search_best_delta: None,
        optimizer_existing_line_search_best_label: None,
        optimizer_existing_line_search_increased: None,
        optimizer_tiny_probe_best_delta: None,
        optimizer_tiny_probe_best_label: None,
        optimizer_tiny_probe_increased: None,
        failure: Some(failure),
    };

    let dual_vertices = vec4_rows(&sample.polytope.dual_vertices_f64);
    let Some(polytope) = SysLandscapePolytopeCache::from_f64_dual_vertices(dual_vertices) else {
        return failure_row("failed_to_reconstruct_polytope".to_string());
    };
    let Some(state) = compute_active_sys_state(&polytope) else {
        return failure_row("failed_to_compute_active_sys_state".to_string());
    };
    let active_orbits = active_orbits(&state.capacity);
    let d_volume_da = match volume_derivatives_a(
        &polytope.dual_vertices_f64,
        &polytope.vertices_f64,
        &polytope.vertex_facet_incidence,
    ) {
        Ok(value) => value,
        Err(err) => return failure_row(format!("failed_volume_derivatives:{err:?}")),
    };
    let d_capacity_da = match capacity_subgradients_a(&polytope.dual_vertices_f64, &active_orbits) {
        Ok(value) => value,
        Err(err) => return failure_row(format!("failed_capacity_subgradients:{err:?}")),
    };
    let gradients: Vec<Vec<Vector4<f64>>> = d_capacity_da
        .iter()
        .map(|capacity_gradient| {
            systolic_ratio_gradient_a(
                state.capacity.capacity(),
                state.vol,
                capacity_gradient,
                &d_volume_da,
            )
        })
        .collect();

    let gradient_norms = gradient_norms(&gradients);
    let (gradient_norm_min, gradient_norm_max) = min_max(&gradient_norms);
    let actions: Vec<f64> = active_orbits.iter().map(|orbit| orbit.action).collect();
    let (active_action_min, active_action_max) = min_max(&actions);
    let symmetry = symmetry_tangent_matrix(&polytope.dual_vertices_f64);
    let (symmetry_rank, slice_basis, symmetry_condition_proxy) = numerical_slice_basis(&symmetry);
    let max_abs_gradient_on_symmetry = gradients
        .iter()
        .map(|gradient| max_abs_row_on_columns(&flatten(gradient), &symmetry))
        .reduce(f64::max);

    let quotient = solve_quotient_maximin(&gradients, &slice_basis);
    let (quotient_maximin, quotient_direction, quotient_direction_norm, quotient_model_status) =
        match quotient {
            Some((maximin, direction, norm)) => (
                Some(maximin),
                Some(direction),
                Some(norm),
                "solved".to_string(),
            ),
            None => (None, None, None, "no_slice_or_lp_failure".to_string()),
        };
    let quotient_ascent_found = quotient_maximin.is_some_and(|value| value > ASCENT_TOL);
    let max_abs_direction_on_symmetry = quotient_direction.as_ref().map(|direction| {
        let flat_direction = flatten(direction);
        max_abs_row_on_columns(&flat_direction, &symmetry)
    });

    let mut step_probe_ran = false;
    let mut step_probe_best_sys = None;
    let mut step_probe_best_delta = None;
    let mut step_probe_best_fraction = None;
    let mut step_probe_increased = None;

    if quotient_ascent_found {
        if let Some(direction) = quotient_direction.as_ref() {
            step_probe_ran = true;
            let boundary = compute_step_bound_detailed(&polytope, direction);
            let base_step = boundary.t_max.min(1.0);
            let mut best: Option<(f64, f64, f64)> = None;
            if base_step.is_finite() && base_step > 0.0 {
                for fraction in STEP_FRACTIONS {
                    let t = fraction * base_step;
                    if let Some((_stepped, stepped_sys)) =
                        apply_dual_step(&polytope.dual_vertices_f64, direction, t)
                    {
                        let delta = stepped_sys - state.sys;
                        if best
                            .as_ref()
                            .is_none_or(|(_, best_sys, _)| stepped_sys > *best_sys)
                        {
                            best = Some((*fraction, stepped_sys, delta));
                        }
                    }
                }
            }
            if let Some((fraction, best_sys, delta)) = best {
                step_probe_best_fraction = Some(fraction);
                step_probe_best_sys = Some(best_sys);
                step_probe_best_delta = Some(delta);
                step_probe_increased = Some(delta > ASCENT_TOL);
            } else {
                step_probe_increased = Some(false);
            }
        }
    }

    let dataset_set = datasets.iter().cloned().collect::<BTreeSet<_>>();
    let mut optimizer_mode = "not_ascent_endpoint".to_string();
    let mut optimizer_direction_status = "not_applicable".to_string();
    let mut optimizer_direction_norm = None;
    let mut optimizer_t_max = None;
    let mut optimizer_existing_line_search_best_delta = None;
    let mut optimizer_existing_line_search_best_label = None;
    let mut optimizer_existing_line_search_increased = None;
    let mut optimizer_tiny_probe_best_delta = None;
    let mut optimizer_tiny_probe_best_label = None;
    let mut optimizer_tiny_probe_increased = None;

    if dataset_set.contains("gradient_ascent_general") {
        optimizer_mode = "general".to_string();
        if let Some(direction) = ascent_direction(&polytope, &state, AscentMode::General) {
            optimizer_direction_status = "solved".to_string();
            optimizer_direction_norm = Some(
                direction
                    .iter()
                    .map(|v| v.norm_squared())
                    .sum::<f64>()
                    .sqrt(),
            );
            let boundary = compute_step_bound_detailed(&polytope, &direction);
            optimizer_t_max = Some(boundary.t_max);
            let mut existing_steps: Vec<(String, f64)> = OPTIMIZER_STEP_FRACTIONS
                .iter()
                .map(|fraction| (format!("within_{fraction}"), fraction * boundary.t_max))
                .collect();
            if boundary.t_max < MAX_STEP_SIZE {
                existing_steps.extend(OPTIMIZER_OVERSHOOT_MULTIPLIERS.iter().map(|multiplier| {
                    (
                        format!("overshoot_{multiplier}"),
                        multiplier * boundary.t_max,
                    )
                }));
            }
            let existing = probe_steps(&polytope, &direction, state.sys, &existing_steps);
            optimizer_existing_line_search_best_delta = existing.best_delta;
            optimizer_existing_line_search_best_label = existing.best_label;
            optimizer_existing_line_search_increased = existing.increased;

            let tiny_steps: Vec<(String, f64)> = STEP_FRACTIONS
                .iter()
                .map(|fraction| (fraction.to_string(), fraction * boundary.t_max.min(1.0)))
                .collect();
            let tiny = probe_steps(&polytope, &direction, state.sys, &tiny_steps);
            optimizer_tiny_probe_best_delta = tiny.best_delta;
            optimizer_tiny_probe_best_label = tiny.best_label;
            optimizer_tiny_probe_increased = tiny.increased;
        } else {
            optimizer_direction_status = "ascent_direction_failed".to_string();
        }
    } else if dataset_set.contains("gradient_ascent_products") {
        optimizer_mode = "lagrangian_product".to_string();
        match symplectic::classify_facets_from_dual_vertices(&polytope.dual_vertices_f64) {
            Ok(classification) => {
                if let Some(direction) = ascent_direction(
                    &polytope,
                    &state,
                    AscentMode::LagrangianProduct {
                        classification: &classification,
                    },
                ) {
                    optimizer_direction_status = "solved".to_string();
                    optimizer_direction_norm = Some(
                        direction
                            .iter()
                            .map(|v| v.norm_squared())
                            .sum::<f64>()
                            .sqrt(),
                    );
                    let boundary = compute_step_bound_detailed(&polytope, &direction);
                    optimizer_t_max = Some(boundary.t_max);
                    let mut existing_steps: Vec<(String, f64)> = OPTIMIZER_STEP_FRACTIONS
                        .iter()
                        .map(|fraction| (format!("within_{fraction}"), fraction * boundary.t_max))
                        .collect();
                    if boundary.t_max < MAX_STEP_SIZE {
                        existing_steps.extend(OPTIMIZER_OVERSHOOT_MULTIPLIERS.iter().map(
                            |multiplier| {
                                (
                                    format!("overshoot_{multiplier}"),
                                    multiplier * boundary.t_max,
                                )
                            },
                        ));
                    }
                    let existing = probe_steps(&polytope, &direction, state.sys, &existing_steps);
                    optimizer_existing_line_search_best_delta = existing.best_delta;
                    optimizer_existing_line_search_best_label = existing.best_label;
                    optimizer_existing_line_search_increased = existing.increased;

                    let tiny_steps: Vec<(String, f64)> = STEP_FRACTIONS
                        .iter()
                        .map(|fraction| (fraction.to_string(), fraction * boundary.t_max.min(1.0)))
                        .collect();
                    let tiny = probe_steps(&polytope, &direction, state.sys, &tiny_steps);
                    optimizer_tiny_probe_best_delta = tiny.best_delta;
                    optimizer_tiny_probe_best_label = tiny.best_label;
                    optimizer_tiny_probe_increased = tiny.increased;
                } else {
                    optimizer_direction_status = "ascent_direction_failed".to_string();
                }
            }
            Err(err) => {
                optimizer_direction_status = format!("classification_failed:{err:?}");
            }
        }
    }

    DiagnosticRow {
        poly_id: sample.polytope.poly_id.clone(),
        selection_buckets: sample.selection_buckets.iter().cloned().collect(),
        datasets,
        roles,
        source_names,
        seed_indices,
        best_strategies,
        input_facet_count: sample.polytope.facet_count,
        input_capacity: sample.polytope.capacity,
        input_volume: sample.polytope.volume,
        input_sys: sample.polytope.sys,
        recomputed_capacity: Some(state.capacity.capacity()),
        recomputed_volume: Some(state.vol),
        recomputed_sys: Some(state.sys),
        recomputed_sys_delta: Some(state.sys - sample.polytope.sys),
        active_orbit_rtol: ACTIVE_ORBIT_RTOL,
        active_orbit_count: Some(active_orbits.len()),
        active_sigma_lengths: active_orbits
            .iter()
            .map(|orbit| orbit.sigma.len())
            .collect(),
        active_action_min,
        active_action_max,
        active_action_spread: active_action_min
            .zip(active_action_max)
            .map(|(min, max)| max - min),
        beta_margin_min: active_orbits
            .iter()
            .map(|orbit| orbit.beta_margin)
            .reduce(f64::min),
        q_error_bound_max: active_orbits
            .iter()
            .map(|orbit| orbit.q_error_bound)
            .reduce(f64::max),
        table_best_beta_margin: sample.polytope.orbit_best_beta_margin,
        table_best_q_error_bound: sample.polytope.orbit_best_q_error_bound,
        table_returned_orbit_count: sample.polytope.orbit_result_returned_orbit_count,
        gradient_row_count: Some(gradients.len()),
        gradient_norm_min,
        gradient_norm_max,
        gradient_norm_mean: mean(&gradient_norms),
        symmetry_rank: Some(symmetry_rank),
        ambient_dimension,
        slice_dimension: Some(slice_basis.ncols()),
        symmetry_condition_proxy,
        max_abs_gradient_on_symmetry,
        max_abs_direction_on_symmetry,
        quotient_maximin,
        quotient_ascent_found,
        quotient_direction_norm,
        quotient_model_status,
        step_probe_ran,
        step_probe_best_sys,
        step_probe_best_delta,
        step_probe_best_fraction,
        step_probe_increased,
        optimizer_mode,
        optimizer_direction_status,
        optimizer_direction_norm,
        optimizer_t_max,
        optimizer_existing_line_search_best_delta,
        optimizer_existing_line_search_best_label,
        optimizer_existing_line_search_increased,
        optimizer_tiny_probe_best_delta,
        optimizer_tiny_probe_best_label,
        optimizer_tiny_probe_increased,
        failure: None,
    }
}

fn summarize(rows: &[DiagnosticRow], cli: &Cli) -> Summary {
    let mut selection_bucket_counts = BTreeMap::new();
    let mut dataset_counts = BTreeMap::new();
    let mut optimizer_mode_counts = BTreeMap::new();
    let mut optimizer_existing_line_search_increased_counts = BTreeMap::new();
    let mut optimizer_tiny_probe_increased_counts = BTreeMap::new();
    for row in rows {
        for bucket in &row.selection_buckets {
            *selection_bucket_counts.entry(bucket.clone()).or_insert(0) += 1;
        }
        for dataset in &row.datasets {
            *dataset_counts.entry(dataset.clone()).or_insert(0) += 1;
        }
        *optimizer_mode_counts
            .entry(row.optimizer_mode.clone())
            .or_insert(0) += 1;
        let existing_key = format!(
            "{}:{}",
            row.optimizer_mode,
            row.optimizer_existing_line_search_increased
                .map(|value| value.to_string())
                .unwrap_or_else(|| "none".to_string())
        );
        *optimizer_existing_line_search_increased_counts
            .entry(existing_key)
            .or_insert(0) += 1;
        let tiny_key = format!(
            "{}:{}",
            row.optimizer_mode,
            row.optimizer_tiny_probe_increased
                .map(|value| value.to_string())
                .unwrap_or_else(|| "none".to_string())
        );
        *optimizer_tiny_probe_increased_counts
            .entry(tiny_key)
            .or_insert(0) += 1;
    }

    Summary {
        method: "endpoint-local-max-diagnostic".to_string(),
        polytope_table: repo_relative_path(&cli.polytope_table),
        provenance_table: repo_relative_path(&cli.provenance_table),
        active_orbit_rtol: ACTIVE_ORBIT_RTOL,
        rank_rtol: RANK_RTOL,
        ascent_tol: ASCENT_TOL,
        step_fractions: STEP_FRACTIONS.to_vec(),
        sampled_rows: rows.len(),
        failures: rows.iter().filter(|row| row.failure.is_some()).count(),
        quotient_ascent_found: rows.iter().filter(|row| row.quotient_ascent_found).count(),
        step_probe_ran: rows.iter().filter(|row| row.step_probe_ran).count(),
        step_probe_increased: rows
            .iter()
            .filter(|row| row.step_probe_increased == Some(true))
            .count(),
        sys_gt_1_after_probe: rows
            .iter()
            .filter(|row| row.step_probe_best_sys.is_some_and(|sys| sys > 1.0))
            .count(),
        selection_bucket_counts,
        dataset_counts,
        max_input_sys: rows.iter().map(|row| row.input_sys).reduce(f64::max),
        max_recomputed_sys: rows
            .iter()
            .filter_map(|row| row.recomputed_sys)
            .reduce(f64::max),
        max_step_probe_sys: rows
            .iter()
            .filter_map(|row| row.step_probe_best_sys)
            .reduce(f64::max),
        rows_with_ill_conditioned_or_empty_slice: rows
            .iter()
            .filter(|row| {
                row.slice_dimension == Some(0)
                    || row
                        .symmetry_condition_proxy
                        .is_some_and(|condition| condition > 1.0e10)
            })
            .count(),
        optimizer_mode_counts,
        optimizer_existing_line_search_increased_counts,
        optimizer_tiny_probe_increased_counts,
    }
}

fn write_outputs(rows: &[DiagnosticRow], summary: &Summary, cli: &Cli) {
    if let Some(parent) = cli.diagnostic_out.parent() {
        fs::create_dir_all(parent).expect("failed to create diagnostic output directory");
    }
    if let Some(parent) = cli.summary_out.parent() {
        fs::create_dir_all(parent).expect("failed to create summary output directory");
    }

    let diagnostic_file = File::create(&cli.diagnostic_out)
        .unwrap_or_else(|err| panic!("failed to create {}: {err}", cli.diagnostic_out.display()));
    let mut diagnostic = BufWriter::new(diagnostic_file);
    for row in rows {
        serde_json::to_writer(&mut diagnostic, row).expect("failed to serialize diagnostic row");
        diagnostic
            .write_all(b"\n")
            .expect("failed to write diagnostic newline");
    }

    let summary_file = File::create(&cli.summary_out)
        .unwrap_or_else(|err| panic!("failed to create {}: {err}", cli.summary_out.display()));
    serde_json::to_writer_pretty(BufWriter::new(summary_file), summary)
        .expect("failed to serialize summary");
}

fn repo_relative_path(path: &Path) -> String {
    let package_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let repo_root = package_root
        .parent()
        .and_then(Path::parent)
        .unwrap_or(&package_root);
    path.strip_prefix(repo_root)
        .unwrap_or(path)
        .display()
        .to_string()
}

fn main() {
    let cli = parse_args();
    let polytope_rows: Vec<PolytopeRow> = load_jsonl(&cli.polytope_table);
    let provenance_rows: Vec<ProvenanceRow> = load_jsonl(&cli.provenance_table);
    let provenance = provenance_by_poly_id(provenance_rows);
    let samples = select_samples(&polytope_rows, &provenance);

    println!(
        "endpoint-local-max-diagnostic: selected {} rows from {} retained polytopes",
        samples.len(),
        polytope_rows.len()
    );

    let mut rows = Vec::with_capacity(samples.len());
    for (idx, sample) in samples.iter().enumerate() {
        println!(
            "[{}/{}] {} sys={:.12} buckets={:?}",
            idx + 1,
            samples.len(),
            sample.polytope.poly_id,
            sample.polytope.sys,
            sample.selection_buckets
        );
        rows.push(diagnostic_for_sample(sample));
    }

    let summary = summarize(&rows, &cli);
    write_outputs(&rows, &summary, &cli);

    println!(
        "wrote {} and {}",
        cli.diagnostic_out.display(),
        cli.summary_out.display()
    );
    println!(
        "quotient_ascent_found={} step_probe_increased={} sys_gt_1_after_probe={}",
        summary.quotient_ascent_found, summary.step_probe_increased, summary.sys_gt_1_after_probe
    );
}

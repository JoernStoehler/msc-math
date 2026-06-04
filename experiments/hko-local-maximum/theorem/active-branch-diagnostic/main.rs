//! Diagnostic for HKO active branches and symmetry-quotient coverage.
//!
//! Goal: print/export the currently visible positive active sigma branches,
//! their `D_a sys` rows, the 15-dimensional symmetry tangent surface, and a
//! numerical slice/cone summary.
//!
//! This is a theorem-route diagnostic, not the final Sage certificate. Rust is
//! used here to generate candidates and high-VoI numerical summaries. Sage
//! remains the intended independent exact verifier for final theorem claims.
//!
//! Modes:
//! 1. `cargo run -p exp-hko-local-maximum --release --bin hko-active-branch-diagnostic`
//!    writes `theorem/active-branch-diagnostic/smoke-active-branch-diagnostic.json`.
//! 2. Add `--exact-limit N` to exact-check the first `N` active-at-a0 branches.
//! 3. Add `--canonical` only when intentionally refreshing
//!    `theorem/active-branch-diagnostic/active-branch-diagnostic.json`.

use algebraic_numbers::{kernel_basis, rank, solve_linear_system, LinearSystemSolution};
use exp_hko_local_maximum::exact_bank::PentagonField;
use exp_hko_local_maximum::{
    ehz_capacity_instrumented, euclidean_volume_f64, exact_hko_dual_vertices, HkoExactScalar,
};
use good_lp::{constraint, default_solver, variable, variables, Expression, Solution, SolverModel};
use nalgebra::{DMatrix, DVector, Matrix4, Vector4};
use num_traits::{One, Zero};
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use symplectic::algorithms::OrbitKktData;
use symplectic::derivatives::{
    capacity_derivatives_a, capacity_derivatives_a_from_orbit, systolic_ratio_gradient_a,
    volume_derivatives_a,
};
use symplectic::exact::{capacity_derivatives_a_exact_from_orbit, omega0, ExactOrbitKktData};
use symplectic::geom::known_polytopes;

const ACTIVE_ACTION_RTOL: f64 = 1.0e-8;
const NUMERICAL_RANK_RTOL: f64 = 1.0e-9;
const LAMBDA_POSITIVE_TOL: f64 = 1.0e-8;
const CONVEX_HULL_RESIDUAL_TOL: f64 = 1.0e-9;
const PADDED_BETA_ZERO_TOL: f64 = 1.0e-8;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct CliOptions {
    canonical: bool,
    exact_limit: Option<usize>,
}

fn print_usage() {
    eprintln!(
        r#"Usage: hko-active-branch-diagnostic [options]

Optional flags:
  --help, -h          Show this help message and exit.
  --smoke             Write the ignored smoke artifact. This is the default.
  --canonical         Refresh the tracked canonical artifact.
  --exact-limit <N>   Exact-check at most N active branches.
  --all-exact         Exact-check every active branch."#
    );
}

impl CliOptions {
    fn parse_from<I, S>(args: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let mut canonical = false;
        let mut exact_limit = Some(0usize);
        let mut args = args.into_iter();
        while let Some(arg) = args.next() {
            match arg.as_ref() {
                "--help" | "-h" => {
                    print_usage();
                    std::process::exit(0);
                }
                "--canonical" => canonical = true,
                "--smoke" => {}
                "--all-exact" => exact_limit = None,
                "--exact-limit" => {
                    let Some(value) = args.next() else {
                        panic!("--exact-limit requires an integer value");
                    };
                    exact_limit = Some(
                        value
                            .as_ref()
                            .parse()
                            .expect("--exact-limit requires an integer value"),
                    );
                }
                other => panic!("unsupported argument: {other}"),
            }
        }
        Self {
            canonical,
            exact_limit,
        }
    }
}

#[derive(Clone, Debug, Serialize)]
struct CanonicalElement {
    coeffs: Vec<String>,
}

impl CanonicalElement {
    fn from_field<F: HkoExactScalar>(value: &F) -> Self {
        Self {
            coeffs: value
                .canonical_coeffs()
                .into_iter()
                .map(|coeff| format!("{}/{}", coeff.numer(), coeff.denom()))
                .collect(),
        }
    }
}

#[derive(Debug)]
struct ExactBranch {
    sigma: Vec<usize>,
    nullity: Option<usize>,
    exact_status: &'static str,
    orbit: Option<ExactOrbitKktData<PentagonField>>,
}

#[derive(Debug, Serialize)]
struct BranchRow {
    category: String,
    sigma: Vec<usize>,
    sigma_len: usize,
    exact_status: String,
    kkt_nullity: Option<usize>,
    action_equals_min_exact_candidate: Option<bool>,
    action_diff_vs_known_hko_f64: Option<f64>,
    action_power_basis: Option<CanonicalElement>,
    action_f64: Option<f64>,
    q_power_basis: Option<CanonicalElement>,
    q_f64: Option<f64>,
    beta_power_basis: Option<Vec<CanonicalElement>>,
    beta_f64: Option<Vec<f64>>,
    beta_min_f64: Option<f64>,
    d_action_flat_power_basis: Option<Vec<CanonicalElement>>,
    d_action_flat_f64: Option<Vec<f64>>,
    d_volume_flat_f64: Vec<f64>,
    d_sys_flat_f64: Option<Vec<f64>>,
    projected_d_sys_norm_f64: Option<f64>,
}

#[derive(Debug, Serialize)]
struct F64ActiveBranchRow {
    sigma: Vec<usize>,
    sigma_len: usize,
    action_f64: f64,
    action_diff_vs_generated_min_f64: f64,
    kkt_f64: KktMatrixSummary,
    beta_f64: Vec<f64>,
    beta_min_f64: f64,
    d_action_flat_f64: Vec<f64>,
    d_volume_flat_f64: Vec<f64>,
    d_sys_flat_f64: Vec<f64>,
    projected_d_sys_norm_f64: f64,
}

#[derive(Clone, Debug, Serialize)]
struct PaddedExtensionSource {
    parent_sigma: Vec<usize>,
    inserted_facet: usize,
    insert_position: usize,
}

#[derive(Debug)]
struct PaddedExtensionAccum {
    source_count: usize,
    source_examples: Vec<PaddedExtensionSource>,
}

#[derive(Debug)]
struct DirectKktSolution {
    beta: Vec<f64>,
    mu: Vec<f64>,
    q: f64,
    action: Option<f64>,
}

#[derive(Debug, Serialize)]
struct PaddedExtensionRow {
    sigma: Vec<usize>,
    sigma_len: usize,
    source_count: usize,
    source_examples: Vec<PaddedExtensionSource>,
    kkt_f64: KktMatrixSummary,
    solve_status: String,
    drop_reason: String,
    beta_f64: Option<Vec<f64>>,
    beta_min_f64: Option<f64>,
    beta_max_abs_zero_f64: Option<f64>,
    beta_positive_except_zero_min_f64: Option<f64>,
    zero_beta_indices: Vec<usize>,
    zero_beta_facets: Vec<usize>,
    negative_beta_count: usize,
    negative_beta_min_f64: Option<f64>,
    q_f64: Option<f64>,
    action_f64: Option<f64>,
    action_diff_vs_generated_min_f64: Option<f64>,
    is_min_action_f64: bool,
    is_nonsingular_min_action_padded_once: bool,
    d_sys_flat_f64: Option<Vec<f64>>,
    projected_d_sys_norm_f64: Option<f64>,
}

#[derive(Debug, Serialize)]
struct KktMatrixSummary {
    size: usize,
    rank: usize,
    nullity: usize,
    singular: bool,
    singular_values: Vec<f64>,
    min_singular_value: Option<f64>,
    condition_number: Option<f64>,
}

#[derive(Debug, Serialize)]
struct NullityCount {
    nullity: usize,
    count: usize,
}

#[derive(Debug, Serialize)]
struct SymmetryGeneratorRow {
    label: String,
    tangent_flat_power_basis: Vec<CanonicalElement>,
    tangent_flat_f64: Vec<f64>,
}

#[derive(Debug, Serialize)]
struct MatrixRankSummary {
    row_count: usize,
    column_count: usize,
    rank: usize,
    singular_values: Vec<f64>,
}

#[derive(Debug, Serialize)]
struct ConvexHullZeroSummary {
    row_count: usize,
    dimension: usize,
    feasible: bool,
    max_abs_residual: Option<f64>,
    positive_lambda_count: Option<usize>,
    positive_lambda_projected_rank: Option<usize>,
    lambda_max: Option<f64>,
    lambda_min: Option<f64>,
    solver_message: String,
}

#[derive(Debug, Serialize)]
struct DiagnosticSummary {
    f64_active_branch_count: usize,
    exact_checked_branch_count: usize,
    exact_positive_branch_count: usize,
    smooth_active_unique_count: usize,
    active_nonunique_selected_count: usize,
    positive_above_min_count: usize,
    unresolved_or_rejected_count: usize,
    known_hko_capacity_f64: f64,
    generated_min_action_f64: f64,
    volume_f64: f64,
    sys_f64: f64,
    symmetry_rank_exact: usize,
    symmetry_generator_count: usize,
    symmetry_ambient_dimension: usize,
    slice_dimension_exact: usize,
    f64_active_kkt_singular_count: usize,
    f64_active_kkt_nullity_histogram: Vec<NullityCount>,
    f64_all_active_projected_rank: MatrixRankSummary,
    f64_all_active_convex_hull_zero: ConvexHullZeroSummary,
    f64_nonsingular_active_projected_rank: MatrixRankSummary,
    f64_nonsingular_active_convex_hull_zero: ConvexHullZeroSummary,
    padded_extension_unique_count: usize,
    padded_extension_source_count: usize,
    padded_extension_nonsingular_count: usize,
    padded_extension_min_action_count: usize,
    padded_extension_nonsingular_min_action_padded_once_count: usize,
    padded_extension_nonsingular_min_action_padded_once_projected_rank: MatrixRankSummary,
    padded_extension_nonsingular_min_action_padded_once_convex_hull_zero: ConvexHullZeroSummary,
    exact_smooth_projected_rank: MatrixRankSummary,
    exact_smooth_convex_hull_zero: ConvexHullZeroSummary,
    exact_checked_min_projected_rank: MatrixRankSummary,
    exact_checked_min_convex_hull_zero: ConvexHullZeroSummary,
    caveats: Vec<String>,
}

#[derive(Debug, Serialize)]
struct DiagnosticOutput {
    diagnostic_version: u32,
    theorem_use: String,
    output_mode: String,
    summary: DiagnosticSummary,
    symmetry_generators: Vec<SymmetryGeneratorRow>,
    f64_active_branches: Vec<F64ActiveBranchRow>,
    padded_extensions: Vec<PaddedExtensionRow>,
    exact_checked_branches: Vec<BranchRow>,
}

fn experiment_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("theorem/active-branch-diagnostic")
}

fn output_path(canonical: bool) -> PathBuf {
    let filename = if canonical {
        "active-branch-diagnostic.json"
    } else {
        "smoke-active-branch-diagnostic.json"
    };
    experiment_dir().join(filename)
}

fn write_json<T: Serialize>(path: &Path, payload: &T) {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .unwrap_or_else(|error| panic!("create {}: {error}", parent.display()));
    }
    let file =
        File::create(path).unwrap_or_else(|error| panic!("create {}: {error}", path.display()));
    let mut writer = BufWriter::new(file);
    serde_json::to_writer_pretty(&mut writer, payload).expect("serialize diagnostic JSON");
    writer.write_all(b"\n").expect("write trailing newline");
    writer.flush().expect("flush diagnostic JSON");
}

fn flatten_exact(values: &[Vector4<PentagonField>]) -> Vec<PentagonField> {
    values
        .iter()
        .flat_map(|value| {
            [
                value[0].clone(),
                value[1].clone(),
                value[2].clone(),
                value[3].clone(),
            ]
        })
        .collect()
}

fn flatten_f64(values: &[Vector4<f64>]) -> Vec<f64> {
    values
        .iter()
        .flat_map(|value| [value[0], value[1], value[2], value[3]])
        .collect()
}

fn exact_flat_to_f64(values: &[PentagonField]) -> Vec<f64> {
    values.iter().map(HkoExactScalar::to_f64).collect()
}

fn canonical_flat(values: &[PentagonField]) -> Vec<CanonicalElement> {
    values.iter().map(CanonicalElement::from_field).collect()
}

fn pf(n: i64) -> PentagonField {
    PentagonField::from(n)
}

fn build_kkt_matrix(
    dual_vertices: &[Vector4<PentagonField>],
    sigma: &[usize],
) -> (DMatrix<PentagonField>, DVector<PentagonField>) {
    let m = sigma.len();
    let size = m + 5;
    let mut matrix = DMatrix::from_element(size, size, PentagonField::zero());
    let mut rhs = DVector::from_element(size, PentagonField::zero());

    for i in 0..m {
        for j in (i + 1)..m {
            let value = omega0(&dual_vertices[sigma[i]], &dual_vertices[sigma[j]]);
            matrix[(i, j)] = value.clone();
            matrix[(j, i)] = value;
        }
    }

    for i in 0..m {
        for dim in 0..4 {
            let value = dual_vertices[sigma[i]][dim].clone();
            matrix[(i, m + dim)] = value.clone();
            matrix[(m + dim, i)] = value;
        }
    }

    for row in 0..m {
        matrix[(row, m + 4)] = PentagonField::one();
    }
    for col in 0..m {
        matrix[(m + 4, col)] = PentagonField::one();
    }
    rhs[m + 4] = PentagonField::one();

    (matrix, rhs)
}

fn omega0_f64(u: &Vector4<f64>, v: &Vector4<f64>) -> f64 {
    u[0] * v[2] - u[2] * v[0] + u[1] * v[3] - u[3] * v[1]
}

fn build_kkt_matrix_f64(dual_vertices: &[Vector4<f64>], sigma: &[usize]) -> DMatrix<f64> {
    let m = sigma.len();
    let size = m + 5;
    let mut matrix = DMatrix::zeros(size, size);

    for i in 0..m {
        for j in (i + 1)..m {
            let value = omega0_f64(&dual_vertices[sigma[i]], &dual_vertices[sigma[j]]);
            matrix[(i, j)] = value;
            matrix[(j, i)] = value;
        }
    }

    for i in 0..m {
        for dim in 0..4 {
            let value = dual_vertices[sigma[i]][dim];
            matrix[(i, m + dim)] = value;
            matrix[(m + dim, i)] = value;
        }
    }

    for row in 0..m {
        matrix[(row, m + 4)] = 1.0;
    }
    for col in 0..m {
        matrix[(m + 4, col)] = 1.0;
    }

    matrix
}

fn kkt_matrix_summary_f64(dual_vertices: &[Vector4<f64>], sigma: &[usize]) -> KktMatrixSummary {
    let matrix = build_kkt_matrix_f64(dual_vertices, sigma);
    let size = matrix.ncols();
    if matrix.nrows() == 0 || matrix.ncols() == 0 {
        return KktMatrixSummary {
            size: matrix.nrows(),
            rank: 0,
            nullity: 0,
            singular: false,
            singular_values: Vec::new(),
            min_singular_value: None,
            condition_number: None,
        };
    }

    let svd = matrix.svd(false, false);
    let singular_values = svd.singular_values.as_slice().to_vec();
    let max_singular_value = singular_values.iter().copied().fold(0.0, f64::max);
    let min_singular_value = singular_values.iter().copied().reduce(f64::min);
    let threshold = NUMERICAL_RANK_RTOL * max_singular_value.max(1.0);
    let rank = singular_values
        .iter()
        .filter(|value| **value > threshold)
        .count();
    let nullity = size.saturating_sub(rank);
    let condition_number = min_singular_value.and_then(|min_value| {
        if min_value > 0.0 {
            Some(max_singular_value / min_value)
        } else {
            None
        }
    });

    KktMatrixSummary {
        size,
        rank,
        nullity,
        singular: nullity > 0,
        singular_values,
        min_singular_value,
        condition_number,
    }
}

fn solve_exact_branch(dual_vertices: &[Vector4<PentagonField>], sigma: &[usize]) -> ExactBranch {
    let (matrix, rhs) = build_kkt_matrix(dual_vertices, sigma);
    let nullity = match solve_linear_system(&matrix, &rhs) {
        LinearSystemSolution::Inconsistent => {
            return ExactBranch {
                sigma: sigma.to_vec(),
                nullity: None,
                exact_status: "inconsistent",
                orbit: None,
            };
        }
        LinearSystemSolution::Consistent { kernel_basis, .. } => kernel_basis.ncols(),
    };

    let orbit = symplectic::exact::solve_orbit_sigma_exact(dual_vertices, sigma);
    let exact_status = match (&orbit, nullity) {
        (Some(_), 0) => "positive_unique",
        (Some(_), _) => "positive_nonunique_selected",
        (None, 0) => "unique_nonpositive_or_noncompetitive",
        (None, _) => "no_positive_solution_found",
    };

    ExactBranch {
        sigma: sigma.to_vec(),
        nullity: Some(nullity),
        exact_status,
        orbit,
    }
}

fn collect_active_candidate_orbits() -> (Vec<OrbitKktData>, f64) {
    let known = known_polytopes::hko_pentagon();
    let instrumented = ehz_capacity_instrumented(
        &known.dual_vertices_f64,
        &known.facet_intersection_is_nonempty,
        &known.omega_signs,
    )
    .expect("HKO active branch diagnostic requires valid generated orbits");

    let best_action = instrumented.capacity;
    let tol = ACTIVE_ACTION_RTOL * best_action.abs().max(1.0);
    let mut seen = BTreeSet::new();
    let mut orbits = Vec::new();
    for orbit in &instrumented.orbits {
        if (orbit.action - best_action).abs() <= tol && seen.insert(orbit.sigma.clone()) {
            orbits.push(orbit.clone());
        }
    }

    (orbits, best_action)
}

fn sp4_generators() -> Vec<(String, Matrix4<PentagonField>)> {
    fn mat(rows: [[i64; 4]; 4]) -> Matrix4<PentagonField> {
        Matrix4::from_fn(|row, col| pf(rows[row][col]))
    }

    vec![
        (
            "sp_a11".to_string(),
            mat([[1, 0, 0, 0], [0, 0, 0, 0], [0, 0, -1, 0], [0, 0, 0, 0]]),
        ),
        (
            "sp_a12".to_string(),
            mat([[0, 1, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, -1, 0]]),
        ),
        (
            "sp_a21".to_string(),
            mat([[0, 0, 0, 0], [1, 0, 0, 0], [0, 0, 0, -1], [0, 0, 0, 0]]),
        ),
        (
            "sp_a22".to_string(),
            mat([[0, 0, 0, 0], [0, 1, 0, 0], [0, 0, 0, 0], [0, 0, 0, -1]]),
        ),
        (
            "sp_b11".to_string(),
            mat([[0, 0, 1, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]]),
        ),
        (
            "sp_b12".to_string(),
            mat([[0, 0, 0, 1], [0, 0, 1, 0], [0, 0, 0, 0], [0, 0, 0, 0]]),
        ),
        (
            "sp_b22".to_string(),
            mat([[0, 0, 0, 0], [0, 0, 0, 1], [0, 0, 0, 0], [0, 0, 0, 0]]),
        ),
        (
            "sp_c11".to_string(),
            mat([[0, 0, 0, 0], [0, 0, 0, 0], [1, 0, 0, 0], [0, 0, 0, 0]]),
        ),
        (
            "sp_c12".to_string(),
            mat([[0, 0, 0, 0], [0, 0, 0, 0], [0, 1, 0, 0], [1, 0, 0, 0]]),
        ),
        (
            "sp_c22".to_string(),
            mat([[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 1, 0, 0]]),
        ),
    ]
}

fn translation_tangent_column(
    dual_vertices: &[Vector4<PentagonField>],
    coord: usize,
) -> Vec<PentagonField> {
    dual_vertices
        .iter()
        .flat_map(|dual| {
            let scalar = dual[coord].clone();
            (0..4).map(move |idx| -scalar.clone() * dual[idx].clone())
        })
        .collect()
}

fn scaling_tangent_column(dual_vertices: &[Vector4<PentagonField>]) -> Vec<PentagonField> {
    dual_vertices
        .iter()
        .flat_map(|dual| (0..4).map(move |idx| -dual[idx].clone()))
        .collect()
}

fn sp4_tangent_column(
    dual_vertices: &[Vector4<PentagonField>],
    generator: &Matrix4<PentagonField>,
) -> Vec<PentagonField> {
    let mut entries = Vec::with_capacity(dual_vertices.len() * 4);
    for dual in dual_vertices {
        for out_coord in 0..4 {
            let mut value = PentagonField::zero();
            for in_coord in 0..4 {
                value += generator[(in_coord, out_coord)].clone() * dual[in_coord].clone();
            }
            entries.push(-value);
        }
    }
    entries
}

fn symmetry_columns(dual_vertices: &[Vector4<PentagonField>]) -> Vec<(String, Vec<PentagonField>)> {
    let mut columns = Vec::new();
    for coord in 0..4 {
        columns.push((
            format!("translation_e{coord}"),
            translation_tangent_column(dual_vertices, coord),
        ));
    }
    columns.push(("scaling".to_string(), scaling_tangent_column(dual_vertices)));
    for (label, generator) in sp4_generators() {
        columns.push((label, sp4_tangent_column(dual_vertices, &generator)));
    }
    columns
}

fn columns_to_matrix(columns: &[(String, Vec<PentagonField>)]) -> DMatrix<PentagonField> {
    let rows = columns
        .first()
        .map(|(_, column)| column.len())
        .unwrap_or_default();
    DMatrix::from_fn(rows, columns.len(), |row, col| columns[col].1[row].clone())
}

fn transpose_exact(matrix: &DMatrix<PentagonField>) -> DMatrix<PentagonField> {
    DMatrix::from_fn(matrix.ncols(), matrix.nrows(), |row, col| {
        matrix[(col, row)].clone()
    })
}

fn matrix_to_f64(matrix: &DMatrix<PentagonField>) -> DMatrix<f64> {
    DMatrix::from_fn(matrix.nrows(), matrix.ncols(), |row, col| {
        matrix[(row, col)].to_f64()
    })
}

fn numerical_rank_summary(matrix: &DMatrix<f64>) -> MatrixRankSummary {
    if matrix.nrows() == 0 || matrix.ncols() == 0 {
        return MatrixRankSummary {
            row_count: matrix.nrows(),
            column_count: matrix.ncols(),
            rank: 0,
            singular_values: Vec::new(),
        };
    }

    let svd = matrix.clone().svd(false, false);
    let singular_values = svd.singular_values.as_slice().to_vec();
    let scale = singular_values.first().copied().unwrap_or(0.0).max(1.0);
    let threshold = NUMERICAL_RANK_RTOL * scale;
    let rank = singular_values
        .iter()
        .filter(|value| **value > threshold)
        .count();

    MatrixRankSummary {
        row_count: matrix.nrows(),
        column_count: matrix.ncols(),
        rank,
        singular_values,
    }
}

fn project_rows_to_slice(rows: &[Vec<f64>], slice_basis: &DMatrix<f64>) -> DMatrix<f64> {
    DMatrix::from_fn(rows.len(), slice_basis.ncols(), |row, col| {
        rows[row]
            .iter()
            .enumerate()
            .map(|(idx, value)| value * slice_basis[(idx, col)])
            .sum()
    })
}

fn row_norm(row: &[f64]) -> f64 {
    row.iter().map(|value| value * value).sum::<f64>().sqrt()
}

fn convex_hull_zero_summary(rows: &DMatrix<f64>) -> ConvexHullZeroSummary {
    if rows.nrows() == 0 || rows.ncols() == 0 {
        return ConvexHullZeroSummary {
            row_count: rows.nrows(),
            dimension: rows.ncols(),
            feasible: false,
            max_abs_residual: None,
            positive_lambda_count: None,
            positive_lambda_projected_rank: None,
            lambda_max: None,
            lambda_min: None,
            solver_message: "empty row matrix".to_string(),
        };
    }

    let mut vars = variables!();
    let lambdas: Vec<_> = (0..rows.nrows())
        .map(|_| vars.add(variable().min(0.0)))
        .collect();

    let mut model = vars.minimise(Expression::from(0.0)).using(default_solver);

    let sum_lambda = lambdas
        .iter()
        .fold(Expression::from(0.0), |expr, lambda| expr + *lambda);
    model = model.with(constraint!(sum_lambda <= 1.0 + CONVEX_HULL_RESIDUAL_TOL));

    let sum_lambda = lambdas
        .iter()
        .fold(Expression::from(0.0), |expr, lambda| expr + *lambda);
    model = model.with(constraint!(sum_lambda >= 1.0 - CONVEX_HULL_RESIDUAL_TOL));

    for col in 0..rows.ncols() {
        let expr = lambdas
            .iter()
            .enumerate()
            .fold(Expression::from(0.0), |acc, (row, lambda)| {
                acc + rows[(row, col)] * *lambda
            });
        model = model.with(constraint!(expr <= CONVEX_HULL_RESIDUAL_TOL));

        let expr = lambdas
            .iter()
            .enumerate()
            .fold(Expression::from(0.0), |acc, (row, lambda)| {
                acc + rows[(row, col)] * *lambda
            });
        model = model.with(constraint!(expr >= -CONVEX_HULL_RESIDUAL_TOL));
    }

    let Ok(solution) = model.solve() else {
        return ConvexHullZeroSummary {
            row_count: rows.nrows(),
            dimension: rows.ncols(),
            feasible: false,
            max_abs_residual: None,
            positive_lambda_count: None,
            positive_lambda_projected_rank: None,
            lambda_max: None,
            lambda_min: None,
            solver_message: "LP solver reported infeasible or failed".to_string(),
        };
    };

    let lambda_values: Vec<f64> = lambdas
        .iter()
        .map(|lambda| solution.value(*lambda))
        .collect();
    let mut residuals = vec![0.0; rows.ncols()];
    for row in 0..rows.nrows() {
        for col in 0..rows.ncols() {
            residuals[col] += lambda_values[row] * rows[(row, col)];
        }
    }
    let max_abs_residual = residuals
        .iter()
        .map(|value| value.abs())
        .fold(0.0, f64::max);

    let positive_rows: Vec<Vec<f64>> = (0..rows.nrows())
        .filter(|&row| lambda_values[row] > LAMBDA_POSITIVE_TOL)
        .map(|row| (0..rows.ncols()).map(|col| rows[(row, col)]).collect())
        .collect();
    let positive_matrix = DMatrix::from_fn(positive_rows.len(), rows.ncols(), |row, col| {
        positive_rows[row][col]
    });
    let positive_rank = numerical_rank_summary(&positive_matrix).rank;

    ConvexHullZeroSummary {
        row_count: rows.nrows(),
        dimension: rows.ncols(),
        feasible: true,
        max_abs_residual: Some(max_abs_residual),
        positive_lambda_count: Some(positive_rows.len()),
        positive_lambda_projected_rank: Some(positive_rank),
        lambda_max: lambda_values.iter().copied().reduce(f64::max),
        lambda_min: lambda_values.iter().copied().reduce(f64::min),
        solver_message: "LP solver found a numerical convex-hull witness".to_string(),
    }
}

fn category_for_branch(branch: &ExactBranch, min_exact_action: &Option<PentagonField>) -> String {
    let Some(orbit) = &branch.orbit else {
        return "unresolved_or_rejected".to_string();
    };
    let action_equals_min = min_exact_action
        .as_ref()
        .is_some_and(|min_action| orbit.action() == *min_action);
    if !action_equals_min {
        return "positive_above_min".to_string();
    }
    match branch.nullity {
        Some(0) => "smooth_active_unique".to_string(),
        Some(_) => "active_nonunique_selected".to_string(),
        None => "unresolved_or_rejected".to_string(),
    }
}

fn build_branch_rows(
    exact_branches: &[ExactBranch],
    min_exact_action: &Option<PentagonField>,
    known_capacity: f64,
    volume: f64,
    d_volume: &[Vector4<f64>],
    slice_basis_f64: &DMatrix<f64>,
    exact_dual_vertices: &[Vector4<PentagonField>],
) -> Vec<BranchRow> {
    let d_volume_flat = flatten_f64(d_volume);

    exact_branches
        .iter()
        .map(|branch| {
            let category = category_for_branch(branch, min_exact_action);
            let Some(orbit) = &branch.orbit else {
                return BranchRow {
                    category,
                    sigma: branch.sigma.clone(),
                    sigma_len: branch.sigma.len(),
                    exact_status: branch.exact_status.to_string(),
                    kkt_nullity: branch.nullity,
                    action_equals_min_exact_candidate: None,
                    action_diff_vs_known_hko_f64: None,
                    action_power_basis: None,
                    action_f64: None,
                    q_power_basis: None,
                    q_f64: None,
                    beta_power_basis: None,
                    beta_f64: None,
                    beta_min_f64: None,
                    d_action_flat_power_basis: None,
                    d_action_flat_f64: None,
                    d_volume_flat_f64: d_volume_flat.clone(),
                    d_sys_flat_f64: None,
                    projected_d_sys_norm_f64: None,
                };
            };

            let action = orbit.action();
            let action_f64 = action.to_f64();
            let d_action_exact =
                capacity_derivatives_a_exact_from_orbit(exact_dual_vertices, orbit);
            let d_action_flat_exact = flatten_exact(&d_action_exact);
            let d_action_flat_f64 = exact_flat_to_f64(&d_action_flat_exact);
            let d_action_f64_vectors: Vec<Vector4<f64>> = d_action_flat_f64
                .chunks_exact(4)
                .map(|chunk| Vector4::new(chunk[0], chunk[1], chunk[2], chunk[3]))
                .collect();
            let d_sys =
                systolic_ratio_gradient_a(action_f64, volume, &d_action_f64_vectors, d_volume);
            let d_sys_flat = flatten_f64(&d_sys);
            let projected =
                project_rows_to_slice(std::slice::from_ref(&d_sys_flat), slice_basis_f64);
            let projected_row: Vec<f64> = (0..projected.ncols())
                .map(|col| projected[(0, col)])
                .collect();

            BranchRow {
                category,
                sigma: branch.sigma.clone(),
                sigma_len: branch.sigma.len(),
                exact_status: branch.exact_status.to_string(),
                kkt_nullity: branch.nullity,
                action_equals_min_exact_candidate: min_exact_action
                    .as_ref()
                    .map(|min_action| action == *min_action),
                action_diff_vs_known_hko_f64: Some(action_f64 - known_capacity),
                action_power_basis: Some(CanonicalElement::from_field(&action)),
                action_f64: Some(action_f64),
                q_power_basis: Some(CanonicalElement::from_field(&orbit.q)),
                q_f64: Some(orbit.q.to_f64()),
                beta_power_basis: Some(
                    orbit
                        .beta
                        .iter()
                        .map(CanonicalElement::from_field)
                        .collect(),
                ),
                beta_f64: Some(orbit.beta.iter().map(HkoExactScalar::to_f64).collect()),
                beta_min_f64: orbit
                    .beta
                    .iter()
                    .map(HkoExactScalar::to_f64)
                    .reduce(f64::min),
                d_action_flat_power_basis: Some(canonical_flat(&d_action_flat_exact)),
                d_action_flat_f64: Some(d_action_flat_f64),
                d_volume_flat_f64: d_volume_flat.clone(),
                d_sys_flat_f64: Some(d_sys_flat),
                projected_d_sys_norm_f64: Some(row_norm(&projected_row)),
            }
        })
        .collect()
}

fn build_f64_active_branch_rows(
    active_orbits: &[OrbitKktData],
    generated_min_action: f64,
    volume: f64,
    d_volume: &[Vector4<f64>],
    slice_basis_f64: &DMatrix<f64>,
    dual_vertices_f64: &[Vector4<f64>],
) -> Vec<F64ActiveBranchRow> {
    let d_volume_flat = flatten_f64(d_volume);

    active_orbits
        .iter()
        .map(|orbit| {
            let d_action = capacity_derivatives_a_from_orbit(dual_vertices_f64, orbit)
                .expect("instrumented HKO orbits carry closure multipliers");
            let d_sys = systolic_ratio_gradient_a(orbit.action, volume, &d_action, d_volume);
            let d_sys_flat = flatten_f64(&d_sys);
            let projected =
                project_rows_to_slice(std::slice::from_ref(&d_sys_flat), slice_basis_f64);
            let projected_row: Vec<f64> = (0..projected.ncols())
                .map(|col| projected[(0, col)])
                .collect();

            F64ActiveBranchRow {
                sigma: orbit.sigma.clone(),
                sigma_len: orbit.sigma.len(),
                action_f64: orbit.action,
                action_diff_vs_generated_min_f64: orbit.action - generated_min_action,
                kkt_f64: kkt_matrix_summary_f64(dual_vertices_f64, &orbit.sigma),
                beta_f64: orbit.beta.clone(),
                beta_min_f64: orbit.beta_margin,
                d_action_flat_f64: flatten_f64(&d_action),
                d_volume_flat_f64: d_volume_flat.clone(),
                d_sys_flat_f64: d_sys_flat,
                projected_d_sys_norm_f64: row_norm(&projected_row),
            }
        })
        .collect()
}

fn canonical_cyclic_sigma(sigma: &[usize]) -> Vec<usize> {
    let mut best = sigma.to_vec();
    for shift in 1..sigma.len() {
        let rotated: Vec<usize> = (0..sigma.len())
            .map(|idx| sigma[(idx + shift) % sigma.len()])
            .collect();
        if rotated < best {
            best = rotated;
        }
    }
    best
}

fn padded_extension_accumulations(
    active_rows: &[F64ActiveBranchRow],
    facet_count: usize,
) -> BTreeMap<Vec<usize>, PaddedExtensionAccum> {
    let mut accumulations: BTreeMap<Vec<usize>, PaddedExtensionAccum> = BTreeMap::new();

    for parent in active_rows.iter().filter(|row| !row.kkt_f64.singular) {
        for inserted_facet in 0..facet_count {
            if parent.sigma.contains(&inserted_facet) {
                continue;
            }
            for insert_position in 0..=parent.sigma.len() {
                let mut padded = parent.sigma.clone();
                padded.insert(insert_position, inserted_facet);
                let canonical = canonical_cyclic_sigma(&padded);
                let entry =
                    accumulations
                        .entry(canonical)
                        .or_insert_with(|| PaddedExtensionAccum {
                            source_count: 0,
                            source_examples: Vec::new(),
                        });
                entry.source_count += 1;
                if entry.source_examples.len() < 8 {
                    entry.source_examples.push(PaddedExtensionSource {
                        parent_sigma: parent.sigma.clone(),
                        inserted_facet,
                        insert_position,
                    });
                }
            }
        }
    }

    accumulations
}

fn action_from_q(q: f64) -> Option<f64> {
    (q > 0.0).then_some(1.0 / (2.0 * q))
}

fn q_from_beta_f64(dual_vertices: &[Vector4<f64>], sigma: &[usize], beta: &[f64]) -> f64 {
    let m = beta.len();
    (1..m)
        .flat_map(|i| (0..i).map(move |j| (i, j)))
        .map(|(i, j)| {
            beta[i] * beta[j] * omega0_f64(&dual_vertices[sigma[j]], &dual_vertices[sigma[i]])
        })
        .sum()
}

fn solve_direct_kkt_f64(
    dual_vertices: &[Vector4<f64>],
    sigma: &[usize],
) -> Option<DirectKktSolution> {
    let matrix = build_kkt_matrix_f64(dual_vertices, sigma);
    let m = sigma.len();
    let size = m + 5;
    let mut rhs = DVector::zeros(size);
    rhs[m + 4] = 1.0;

    let solution = matrix.lu().solve(&rhs)?;
    let beta: Vec<f64> = (0..m).map(|idx| solution[idx]).collect();
    let mu: Vec<f64> = (m..m + 4).map(|idx| solution[idx]).collect();
    let q = q_from_beta_f64(dual_vertices, sigma, &beta);
    let action = action_from_q(q);

    Some(DirectKktSolution {
        beta,
        mu,
        q,
        action,
    })
}

fn is_active_action(action: f64, generated_min_action: f64) -> bool {
    let tol = ACTIVE_ACTION_RTOL * generated_min_action.abs().max(1.0);
    (action - generated_min_action).abs() <= tol
}

fn build_padded_extension_rows(
    active_rows: &[F64ActiveBranchRow],
    generated_min_action: f64,
    volume: f64,
    d_volume: &[Vector4<f64>],
    slice_basis_f64: &DMatrix<f64>,
    dual_vertices_f64: &[Vector4<f64>],
) -> Vec<PaddedExtensionRow> {
    padded_extension_accumulations(active_rows, dual_vertices_f64.len())
        .into_iter()
        .map(|(sigma, accumulation)| {
            let kkt_f64 = kkt_matrix_summary_f64(dual_vertices_f64, &sigma);
            if kkt_f64.singular {
                return PaddedExtensionRow {
                    sigma: sigma.clone(),
                    sigma_len: sigma.len(),
                    source_count: accumulation.source_count,
                    source_examples: accumulation.source_examples,
                    kkt_f64,
                    solve_status: "not_solved_singular_kkt".to_string(),
                    drop_reason: "singular_kkt".to_string(),
                    beta_f64: None,
                    beta_min_f64: None,
                    beta_max_abs_zero_f64: None,
                    beta_positive_except_zero_min_f64: None,
                    zero_beta_indices: Vec::new(),
                    zero_beta_facets: Vec::new(),
                    negative_beta_count: 0,
                    negative_beta_min_f64: None,
                    q_f64: None,
                    action_f64: None,
                    action_diff_vs_generated_min_f64: None,
                    is_min_action_f64: false,
                    is_nonsingular_min_action_padded_once: false,
                    d_sys_flat_f64: None,
                    projected_d_sys_norm_f64: None,
                };
            }

            let Some(kkt) = solve_direct_kkt_f64(dual_vertices_f64, &sigma) else {
                return PaddedExtensionRow {
                    sigma: sigma.clone(),
                    sigma_len: sigma.len(),
                    source_count: accumulation.source_count,
                    source_examples: accumulation.source_examples,
                    kkt_f64,
                    solve_status: "direct_solve_failed".to_string(),
                    drop_reason: "direct_solve_failed".to_string(),
                    beta_f64: None,
                    beta_min_f64: None,
                    beta_max_abs_zero_f64: None,
                    beta_positive_except_zero_min_f64: None,
                    zero_beta_indices: Vec::new(),
                    zero_beta_facets: Vec::new(),
                    negative_beta_count: 0,
                    negative_beta_min_f64: None,
                    q_f64: None,
                    action_f64: None,
                    action_diff_vs_generated_min_f64: None,
                    is_min_action_f64: false,
                    is_nonsingular_min_action_padded_once: false,
                    d_sys_flat_f64: None,
                    projected_d_sys_norm_f64: None,
                };
            };

            let beta = kkt.beta.clone();
            let zero_beta_indices: Vec<usize> = beta
                .iter()
                .enumerate()
                .filter_map(|(idx, value)| (value.abs() <= PADDED_BETA_ZERO_TOL).then_some(idx))
                .collect();
            let zero_beta_facets: Vec<usize> =
                zero_beta_indices.iter().map(|&idx| sigma[idx]).collect();
            let negative_beta_count = beta
                .iter()
                .filter(|value| **value < -PADDED_BETA_ZERO_TOL)
                .count();
            let negative_beta_min = beta
                .iter()
                .copied()
                .filter(|value| *value < -PADDED_BETA_ZERO_TOL)
                .reduce(f64::min);
            let beta_min = beta.iter().copied().reduce(f64::min);
            let beta_max_abs_zero = zero_beta_indices
                .iter()
                .map(|&idx| beta[idx].abs())
                .reduce(f64::max);
            let beta_positive_except_zero_min = beta
                .iter()
                .enumerate()
                .filter(|(idx, _)| !zero_beta_indices.contains(idx))
                .map(|(_, value)| *value)
                .reduce(f64::min);
            let action = kkt.action;
            let action_diff = action.map(|value| value - generated_min_action);
            let is_min_action =
                action.is_some_and(|value| is_active_action(value, generated_min_action));
            let is_padded_once = zero_beta_indices.len() == 1
                && negative_beta_count == 0
                && beta_positive_except_zero_min.is_some_and(|value| value > PADDED_BETA_ZERO_TOL);
            let is_nonsingular_min_action_padded_once = is_min_action && is_padded_once;

            let (d_sys_flat_f64, projected_d_sys_norm_f64) =
                if is_nonsingular_min_action_padded_once {
                    let d_action = capacity_derivatives_a(
                        &kkt.beta,
                        kkt.q,
                        &kkt.mu,
                        &sigma,
                        dual_vertices_f64,
                    );
                    let d_sys = systolic_ratio_gradient_a(
                        action.expect("minimum-action padded row has action"),
                        volume,
                        &d_action,
                        d_volume,
                    );
                    let d_sys_flat = flatten_f64(&d_sys);
                    let projected =
                        project_rows_to_slice(std::slice::from_ref(&d_sys_flat), slice_basis_f64);
                    let projected_row: Vec<f64> = (0..projected.ncols())
                        .map(|col| projected[(0, col)])
                        .collect();
                    (Some(d_sys_flat), Some(row_norm(&projected_row)))
                } else {
                    (None, None)
                };

            let drop_reason = if is_nonsingular_min_action_padded_once {
                "kept_nonsingular_min_action_padded_once"
            } else if negative_beta_count > 0 {
                "negative_beta"
            } else if zero_beta_indices.len() != 1 {
                "not_one_zero_beta"
            } else if !is_min_action {
                "not_min_action"
            } else {
                "unclassified_drop"
            }
            .to_string();

            PaddedExtensionRow {
                sigma: sigma.clone(),
                sigma_len: sigma.len(),
                source_count: accumulation.source_count,
                source_examples: accumulation.source_examples,
                kkt_f64,
                solve_status: "solved_direct_nonsingular_kkt".to_string(),
                drop_reason,
                beta_f64: Some(beta),
                beta_min_f64: beta_min,
                beta_max_abs_zero_f64: beta_max_abs_zero,
                beta_positive_except_zero_min_f64: beta_positive_except_zero_min,
                zero_beta_indices,
                zero_beta_facets,
                negative_beta_count,
                negative_beta_min_f64: negative_beta_min,
                q_f64: Some(kkt.q),
                action_f64: action,
                action_diff_vs_generated_min_f64: action_diff,
                is_min_action_f64: is_min_action,
                is_nonsingular_min_action_padded_once,
                d_sys_flat_f64,
                projected_d_sys_norm_f64,
            }
        })
        .collect()
}

fn rows_for_category(branch_rows: &[BranchRow], categories: &[&str]) -> Vec<Vec<f64>> {
    branch_rows
        .iter()
        .filter(|row| categories.contains(&row.category.as_str()))
        .filter_map(|row| row.d_sys_flat_f64.clone())
        .collect()
}

fn nullity_histogram(rows: &[F64ActiveBranchRow]) -> Vec<NullityCount> {
    let mut counts = BTreeMap::new();
    for row in rows {
        *counts.entry(row.kkt_f64.nullity).or_insert(0usize) += 1;
    }
    counts
        .into_iter()
        .map(|(nullity, count)| NullityCount { nullity, count })
        .collect()
}

fn main() {
    let options = CliOptions::parse_from(std::env::args().skip(1));
    let output_path = output_path(options.canonical);

    let known = known_polytopes::hko_pentagon();
    let volume = euclidean_volume_f64(&known.vertices, &known.vertex_facet_incidence);
    let sys = symplectic::systolic_ratio(known.capacity, volume);
    let d_volume = volume_derivatives_a(
        &known.dual_vertices_f64,
        &known.vertices_f64,
        &known.vertex_facet_incidence,
    )
    .expect("HKO geometry should have valid f64 volume derivatives");

    let exact_dual_vertices = exact_hko_dual_vertices();
    let (active_orbits, generated_min_action) = collect_active_candidate_orbits();
    let symmetry_columns = symmetry_columns(&exact_dual_vertices);
    let symmetry_matrix = columns_to_matrix(&symmetry_columns);
    let symmetry_rank_exact = rank(&symmetry_matrix);
    let symmetry_transpose = transpose_exact(&symmetry_matrix);
    let slice_basis_exact = kernel_basis(&symmetry_transpose);
    let slice_basis_f64 = matrix_to_f64(&slice_basis_exact);

    let exact_sigmas: Vec<Vec<usize>> = match options.exact_limit {
        Some(limit) => active_orbits
            .iter()
            .take(limit)
            .map(|orbit| orbit.sigma.clone())
            .collect(),
        None => active_orbits
            .iter()
            .map(|orbit| orbit.sigma.clone())
            .collect(),
    };
    let exact_branches: Vec<ExactBranch> = exact_sigmas
        .iter()
        .map(|sigma| solve_exact_branch(&exact_dual_vertices, sigma))
        .collect();
    let min_exact_action = exact_branches
        .iter()
        .filter_map(|branch| branch.orbit.as_ref().map(ExactOrbitKktData::action))
        .min();

    let f64_active_branch_rows = build_f64_active_branch_rows(
        &active_orbits,
        generated_min_action,
        volume,
        &d_volume,
        &slice_basis_f64,
        &known.dual_vertices_f64,
    );
    let padded_extension_rows = build_padded_extension_rows(
        &f64_active_branch_rows,
        generated_min_action,
        volume,
        &d_volume,
        &slice_basis_f64,
        &known.dual_vertices_f64,
    );
    let exact_branch_rows = build_branch_rows(
        &exact_branches,
        &min_exact_action,
        known.capacity,
        volume,
        &d_volume,
        &slice_basis_f64,
        &exact_dual_vertices,
    );

    let f64_all_active_rows: Vec<Vec<f64>> = f64_active_branch_rows
        .iter()
        .map(|row| row.d_sys_flat_f64.clone())
        .collect();
    let f64_nonsingular_active_rows: Vec<Vec<f64>> = f64_active_branch_rows
        .iter()
        .filter(|row| !row.kkt_f64.singular)
        .map(|row| row.d_sys_flat_f64.clone())
        .collect();
    let padded_nonsingular_min_action_padded_once_rows: Vec<Vec<f64>> = padded_extension_rows
        .iter()
        .filter(|row| row.is_nonsingular_min_action_padded_once)
        .filter_map(|row| row.d_sys_flat_f64.clone())
        .collect();
    let exact_smooth_rows = rows_for_category(&exact_branch_rows, &["smooth_active_unique"]);
    let exact_checked_min_rows = rows_for_category(
        &exact_branch_rows,
        &["smooth_active_unique", "active_nonunique_selected"],
    );
    let f64_all_active_projected = project_rows_to_slice(&f64_all_active_rows, &slice_basis_f64);
    let f64_nonsingular_active_projected =
        project_rows_to_slice(&f64_nonsingular_active_rows, &slice_basis_f64);
    let padded_nonsingular_min_action_padded_once_projected = project_rows_to_slice(
        &padded_nonsingular_min_action_padded_once_rows,
        &slice_basis_f64,
    );
    let exact_smooth_projected = project_rows_to_slice(&exact_smooth_rows, &slice_basis_f64);
    let exact_checked_min_projected =
        project_rows_to_slice(&exact_checked_min_rows, &slice_basis_f64);

    let f64_all_active_projected_rank = numerical_rank_summary(&f64_all_active_projected);
    let f64_nonsingular_active_projected_rank =
        numerical_rank_summary(&f64_nonsingular_active_projected);
    let padded_extension_nonsingular_min_action_padded_once_projected_rank =
        numerical_rank_summary(&padded_nonsingular_min_action_padded_once_projected);
    let exact_smooth_projected_rank = numerical_rank_summary(&exact_smooth_projected);
    let exact_checked_min_projected_rank = numerical_rank_summary(&exact_checked_min_projected);
    let f64_all_active_convex_hull_zero = convex_hull_zero_summary(&f64_all_active_projected);
    let f64_nonsingular_active_convex_hull_zero =
        convex_hull_zero_summary(&f64_nonsingular_active_projected);
    let padded_extension_nonsingular_min_action_padded_once_convex_hull_zero =
        convex_hull_zero_summary(&padded_nonsingular_min_action_padded_once_projected);
    let exact_smooth_convex_hull_zero = convex_hull_zero_summary(&exact_smooth_projected);
    let exact_checked_min_convex_hull_zero = convex_hull_zero_summary(&exact_checked_min_projected);

    let symmetry_generators: Vec<SymmetryGeneratorRow> = symmetry_columns
        .iter()
        .map(|(label, column)| SymmetryGeneratorRow {
            label: label.clone(),
            tangent_flat_power_basis: canonical_flat(column),
            tangent_flat_f64: exact_flat_to_f64(column),
        })
        .collect();

    let smooth_active_unique_count = exact_branch_rows
        .iter()
        .filter(|row| row.category == "smooth_active_unique")
        .count();
    let active_nonunique_selected_count = exact_branch_rows
        .iter()
        .filter(|row| row.category == "active_nonunique_selected")
        .count();
    let positive_above_min_count = exact_branch_rows
        .iter()
        .filter(|row| row.category == "positive_above_min")
        .count();
    let unresolved_or_rejected_count = exact_branch_rows
        .iter()
        .filter(|row| row.category == "unresolved_or_rejected")
        .count();
    let exact_positive_branch_count = exact_branch_rows
        .iter()
        .filter(|row| row.action_f64.is_some())
        .count();
    let f64_active_kkt_singular_count = f64_active_branch_rows
        .iter()
        .filter(|row| row.kkt_f64.singular)
        .count();
    let padded_extension_source_count = padded_extension_rows
        .iter()
        .map(|row| row.source_count)
        .sum();
    let padded_extension_nonsingular_count = padded_extension_rows
        .iter()
        .filter(|row| !row.kkt_f64.singular)
        .count();
    let padded_extension_min_action_count = padded_extension_rows
        .iter()
        .filter(|row| row.is_min_action_f64)
        .count();
    let padded_extension_nonsingular_min_action_padded_once_count = padded_extension_rows
        .iter()
        .filter(|row| row.is_nonsingular_min_action_padded_once)
        .count();

    let summary = DiagnosticSummary {
        f64_active_branch_count: active_orbits.len(),
        exact_checked_branch_count: exact_branch_rows.len(),
        exact_positive_branch_count,
        smooth_active_unique_count,
        active_nonunique_selected_count,
        positive_above_min_count,
        unresolved_or_rejected_count,
        known_hko_capacity_f64: known.capacity,
        generated_min_action_f64: generated_min_action,
        volume_f64: volume,
        sys_f64: sys,
        symmetry_rank_exact,
        symmetry_generator_count: symmetry_generators.len(),
        symmetry_ambient_dimension: symmetry_matrix.nrows(),
        slice_dimension_exact: slice_basis_exact.ncols(),
        f64_active_kkt_singular_count,
        f64_active_kkt_nullity_histogram: nullity_histogram(&f64_active_branch_rows),
        f64_all_active_projected_rank,
        f64_all_active_convex_hull_zero,
        f64_nonsingular_active_projected_rank,
        f64_nonsingular_active_convex_hull_zero,
        padded_extension_unique_count: padded_extension_rows.len(),
        padded_extension_source_count,
        padded_extension_nonsingular_count,
        padded_extension_min_action_count,
        padded_extension_nonsingular_min_action_padded_once_count,
        padded_extension_nonsingular_min_action_padded_once_projected_rank,
        padded_extension_nonsingular_min_action_padded_once_convex_hull_zero,
        exact_smooth_projected_rank,
        exact_smooth_convex_hull_zero,
        exact_checked_min_projected_rank,
        exact_checked_min_convex_hull_zero,
        caveats: vec![
            format!("Candidate sigmas are active at a0 in the current f64 instrumented positive-beta search: action is within relative tolerance {ACTIVE_ACTION_RTOL:e} of the generated HKO minimum. beta=0 active branches are intentionally outside this first diagnostic."),
            "The default f64 active rows use f64 D_a action, D_a volume, and D_a sys. Exact-checked branch rows additionally include exact D_a action over Q(tan(pi/5)).".to_string(),
            "When exact branch rows are requested, their exact minimum-action comparison is only against the minimum exact action among exact-checked active candidates, not against an independently trusted Sage/HKO2024 c0 certificate.".to_string(),
            "The default smoke path exact-checks no branches because the current Rust exact KKT sidecar is slow. Pass --exact-limit N or --all-exact only when intentionally running that sidecar.".to_string(),
            format!("f64 KKT singularity uses SVD rank threshold {NUMERICAL_RANK_RTOL:e} times the largest singular value, with a floor at scale 1."),
            format!("Padded-extension rows insert one missing facet into nonsingular six-facet active rows and quotient only by cyclic rotation. A kept padded row has nonsingular f64 KKT, one beta coordinate with |beta| <= {PADDED_BETA_ZERO_TOL:e}, no beta < -{PADDED_BETA_ZERO_TOL:e}, and minimum action within the active tolerance."),
            "Padded-extension gradients are equality-branch gradients. They do not by themselves give full halfplanes for theorem use; a final certificate must also impose the one-sided beta-zero activation condition for the inserted coordinate.".to_string(),
            "Convex-hull and projected-rank checks are numerical triage signals; final theorem use still needs Sage verification. Singular-KKT rows should not be used as smooth-gradient theorem witnesses without a separate nonsmooth/subfamily argument.".to_string(),
        ],
    };

    let output = DiagnosticOutput {
        diagnostic_version: 1,
        theorem_use: "High-VoI Rust diagnostic for the HKO M_10 local-maximum route: generate active branch candidates, D_a sys rows, KKT singularity flags, symmetry directions, and numerical quotient-cone checks before building the final Sage certificate.".to_string(),
        output_mode: if options.canonical { "canonical" } else { "smoke" }.to_string(),
        summary,
        symmetry_generators,
        f64_active_branches: f64_active_branch_rows,
        padded_extensions: padded_extension_rows,
        exact_checked_branches: exact_branch_rows,
    };

    println!("HKO active branch diagnostic");
    println!(
        "  f64 active branches: {}",
        output.summary.f64_active_branch_count
    );
    println!(
        "  exact checked branches: {}",
        output.summary.exact_checked_branch_count
    );
    println!(
        "  exact positive branches: {}",
        output.summary.exact_positive_branch_count
    );
    println!(
        "  smooth active unique: {}",
        output.summary.smooth_active_unique_count
    );
    println!(
        "  active nonunique selected: {}",
        output.summary.active_nonunique_selected_count
    );
    println!(
        "  symmetry rank: {} / {}",
        output.summary.symmetry_rank_exact, output.summary.symmetry_generator_count
    );
    println!(
        "  slice dimension: {}",
        output.summary.slice_dimension_exact
    );
    println!(
        "  f64 active singular KKT: {} / {}",
        output.summary.f64_active_kkt_singular_count, output.summary.f64_active_branch_count
    );
    println!(
        "  f64 all-active projected D_sys rank: {} / {}",
        output.summary.f64_all_active_projected_rank.rank, output.summary.slice_dimension_exact
    );
    println!(
        "  f64 nonsingular-active projected D_sys rank: {} / {}",
        output.summary.f64_nonsingular_active_projected_rank.rank,
        output.summary.slice_dimension_exact
    );
    println!(
        "  exact smooth projected D_sys rank: {} / {}",
        output.summary.exact_smooth_projected_rank.rank, output.summary.slice_dimension_exact
    );
    println!(
        "  exact checked-min projected D_sys rank: {} / {}",
        output.summary.exact_checked_min_projected_rank.rank, output.summary.slice_dimension_exact
    );
    println!(
        "  f64 all-active zero-in-conv feasible: {}",
        output.summary.f64_all_active_convex_hull_zero.feasible
    );
    println!(
        "  f64 nonsingular-active zero-in-conv feasible: {}",
        output
            .summary
            .f64_nonsingular_active_convex_hull_zero
            .feasible
    );
    println!(
        "  padded extensions: {} unique from {} insertions",
        output.summary.padded_extension_unique_count, output.summary.padded_extension_source_count
    );
    println!(
        "  padded nonsingular/min-action/one-zero rows: {}",
        output
            .summary
            .padded_extension_nonsingular_min_action_padded_once_count
    );
    println!(
        "  padded kept projected D_sys rank: {} / {}",
        output
            .summary
            .padded_extension_nonsingular_min_action_padded_once_projected_rank
            .rank,
        output.summary.slice_dimension_exact
    );
    println!(
        "  padded kept zero-in-conv feasible: {}",
        output
            .summary
            .padded_extension_nonsingular_min_action_padded_once_convex_hull_zero
            .feasible
    );
    println!(
        "  exact smooth zero-in-conv feasible: {}",
        output.summary.exact_smooth_convex_hull_zero.feasible
    );
    println!(
        "  exact checked-min zero-in-conv feasible: {}",
        output.summary.exact_checked_min_convex_hull_zero.feasible
    );

    write_json(&output_path, &output);
    println!("  wrote {}", output_path.display());
}

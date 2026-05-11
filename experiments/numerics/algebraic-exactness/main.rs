//! Algebraic exactness spike for HKO-style polytopes.
//!
//! Goal: construct selected algebraic polytopes and exact KKT rows over an
//! algebraic extension of `Q`, then compare them against the current dyadic
//! library path without changing the library core.
//!
//! Input Artifacts: None.
//! Output Artifacts: experiments/numerics/algebraic-exactness/{smoke-exact-polytopes.jsonl,smoke-exact-kkt-comparison.jsonl,exact-polytopes.jsonl,exact-kkt-comparison.jsonl}

#[path = "../src/flat_polytope.rs"]
mod flat_polytope;

use crate::flat_polytope::NumericsPolytopeCache;
use dev_numerical_analysis::algebraic::catalog::{
    ElementRecord, ExactKktComparisonRow, ExactPolytopeCatalogRow,
};
use dev_numerical_analysis::algebraic::field::{CatalogField, ExperimentScalar};
use dev_numerical_analysis::algebraic::fixtures::{
    exact_hko_pentagon, exact_hypercube, exact_simplex, hko_capacity_formula_f64,
    HKO_RANK_DEFICIENT_SIGMA, HKO_WINNING_SIGMA,
};
use dev_numerical_analysis::algebraic::geom::exact_polytope_fields;
use dev_numerical_analysis::algebraic::kkt::solve_kkt_exact;
use dev_numerical_analysis::capacity_pruned_hk2017;
use nalgebra::DMatrix;
use serde::Serialize;
use std::env;
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use symplectic::geom::known_polytopes;
use symplectic::kkt::rational_solver as library_rational_solver;

fn polytope_row<F: CatalogField>(
    name: &str,
    dual_vertices: &[[F; 4]],
    vertices: &[[F; 4]],
    omega_signs: &[Vec<i8>],
) -> ExactPolytopeCatalogRow {
    let field = F::field_tag();
    ExactPolytopeCatalogRow {
        name: name.to_string(),
        field,
        field_description: field.description().to_string(),
        basis: field
            .basis_labels()
            .iter()
            .map(|label| (*label).to_string())
            .collect(),
        facet_count: dual_vertices.len(),
        vertex_count: vertices.len(),
        dual_vertices: dual_vertices
            .iter()
            .map(|dual| std::array::from_fn(|i| ElementRecord::from_field(&dual[i])))
            .collect(),
        vertices: vertices
            .iter()
            .map(|vertex| std::array::from_fn(|i| ElementRecord::from_field(&vertex[i])))
            .collect(),
        has_zero_omega: has_off_diagonal_zero_omega(omega_signs),
    }
}

fn exact_kkt_row<F: CatalogField + 'static>(
    name: &str,
    sigma_label: &str,
    sigma: &[usize],
    dual_vertices: &[[F; 4]],
    reference_source: &str,
    reference_q_f64: Option<f64>,
) -> ExactKktComparisonRow {
    let exact = solve_kkt_exact(dual_vertices, sigma).expect("selected sigma should solve exactly");
    let action_exact_f64 = 0.5 / exact.q_exact_f64;
    ExactKktComparisonRow {
        name: name.to_string(),
        field: F::field_tag(),
        sigma_label: sigma_label.to_string(),
        sigma: sigma.to_vec(),
        q_exact: ElementRecord::from_field(&exact.q_exact),
        q_exact_f64: exact.q_exact_f64,
        action_exact_f64,
        beta_f64: exact.beta.iter().map(ExperimentScalar::to_f64).collect(),
        reference_source: reference_source.to_string(),
        reference_q_f64,
        abs_diff_vs_reference: reference_q_f64.map(|value| (exact.q_exact_f64 - value).abs()),
    }
}

fn write_jsonl<T: Serialize>(path: &Path, rows: &[T]) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .unwrap_or_else(|err| panic!("failed to create {}: {err}", parent.display()));
    }
    let file = File::create(path)
        .unwrap_or_else(|err| panic!("failed to create {}: {err}", path.display()));
    let mut writer = BufWriter::new(file);
    for row in rows {
        serde_json::to_writer(&mut writer, row).expect("serialize jsonl row");
        writeln!(&mut writer).expect("write jsonl newline");
    }
    writer.flush().expect("flush jsonl writer");
}

fn has_off_diagonal_zero_omega(omega_signs: &[Vec<i8>]) -> bool {
    omega_signs.iter().enumerate().any(|(row_idx, row)| {
        row.iter()
            .enumerate()
            .any(|(col_idx, &sign)| row_idx != col_idx && sign == 0)
    })
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum RunMode {
    Smoke,
    Canonical,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum RunModeArgError {
    Help,
    Unknown,
}

fn parse_run_mode<I>(args: I) -> Result<RunMode, RunModeArgError>
where
    I: IntoIterator,
    I::Item: Into<String>,
{
    let mut mode = RunMode::Smoke;
    for arg in args {
        match arg.into().as_str() {
            "--canonical" => mode = RunMode::Canonical,
            "--help" | "-h" => return Err(RunModeArgError::Help),
            _ => return Err(RunModeArgError::Unknown),
        }
    }
    Ok(mode)
}

fn output_path(manifest_dir: &Path, filename: &str, mode: RunMode) -> PathBuf {
    let output_dir = manifest_dir.join("algebraic-exactness");
    match mode {
        RunMode::Smoke => output_dir.join(format!("smoke-{filename}")),
        RunMode::Canonical => output_dir.join(filename),
    }
}

fn print_help_and_exit(code: i32) -> ! {
    eprintln!(
        "Usage: cargo run -p dev-numerical-analysis --release --bin num-algebraic-exactness [--canonical]"
    );
    eprintln!(
        "  default: write untracked smoke outputs under experiments/numerics/algebraic-exactness/"
    );
    eprintln!("  --canonical: refresh the tracked exact-polytopes.jsonl and exact-kkt-comparison.jsonl outputs");
    std::process::exit(code);
}

fn normalized_incidence_rows(rows: &[Vec<bool>]) -> Vec<Vec<bool>> {
    let mut rows = rows.to_vec();
    rows.sort();
    rows
}

fn matrix_rows<T: Copy>(matrix: &DMatrix<T>) -> Vec<Vec<T>> {
    (0..matrix.nrows())
        .map(|row| (0..matrix.ncols()).map(|col| matrix[(row, col)]).collect())
        .collect()
}

fn same_incidence(lhs: &[Vec<bool>], rhs: &DMatrix<bool>) -> bool {
    lhs.len() == rhs.nrows()
        && normalized_incidence_rows(lhs) == normalized_incidence_rows(&matrix_rows(rhs))
}

fn same_bool_matrix(lhs: &[Vec<bool>], rhs: &DMatrix<bool>) -> bool {
    lhs.len() == rhs.nrows()
        && lhs.iter().enumerate().all(|(row_idx, row)| {
            row.len() == rhs.ncols()
                && row
                    .iter()
                    .enumerate()
                    .all(|(col_idx, val)| *val == rhs[(row_idx, col_idx)])
        })
}

fn same_i8_matrix(lhs: &[Vec<i8>], rhs: &DMatrix<i8>) -> bool {
    lhs.len() == rhs.nrows()
        && lhs.iter().enumerate().all(|(row_idx, row)| {
            row.len() == rhs.ncols()
                && row
                    .iter()
                    .enumerate()
                    .all(|(col_idx, val)| *val == rhs[(row_idx, col_idx)])
        })
}

fn omega_mismatch_positions(lhs: &[Vec<i8>], rhs: &DMatrix<i8>) -> Vec<(usize, usize, i8, i8)> {
    let mut mismatches = Vec::new();
    for row in 0..lhs.len() {
        for col in 0..lhs[row].len() {
            let rhs_val = rhs[(row, col)];
            if lhs[row][col] != rhs_val {
                mismatches.push((row, col, lhs[row][col], rhs_val));
            }
        }
    }
    mismatches
}

fn main() {
    let mode = match parse_run_mode(env::args().skip(1)) {
        Ok(mode) => mode,
        Err(RunModeArgError::Help) => print_help_and_exit(0),
        Err(RunModeArgError::Unknown) => {
            eprintln!("unknown argument");
            print_help_and_exit(2);
        }
    };

    let exact_simplex = exact_simplex();
    let (simplex_vertices, _simplex_incidence, _simplex_adjacency, simplex_omega_signs) =
        exact_polytope_fields(&exact_simplex).expect("exact simplex");
    let exact_hypercube = exact_hypercube();
    let (hypercube_vertices, _hypercube_incidence, _hypercube_adjacency, hypercube_omega_signs) =
        exact_polytope_fields(&exact_hypercube).expect("exact hypercube");
    let exact_hko = exact_hko_pentagon();
    let (hko_vertices, hko_incidence, hko_adjacency, hko_omega_signs) =
        exact_polytope_fields(&exact_hko).expect("exact hko");

    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let polytope_output_path = output_path(manifest_dir, "exact-polytopes.jsonl", mode);
    let kkt_output_path = output_path(manifest_dir, "exact-kkt-comparison.jsonl", mode);

    let polytope_rows = vec![
        polytope_row(
            "simplex_exact_q",
            &exact_simplex,
            &simplex_vertices,
            &simplex_omega_signs,
        ),
        polytope_row(
            "hypercube_exact_q",
            &exact_hypercube,
            &hypercube_vertices,
            &hypercube_omega_signs,
        ),
        polytope_row(
            "hko_pentagon_exact_pentagon_field",
            &exact_hko,
            &hko_vertices,
            &hko_omega_signs,
        ),
    ];
    write_jsonl(&polytope_output_path, &polytope_rows);

    let library_simplex = known_polytopes::simplex();
    let simplex_cache = NumericsPolytopeCache::from_rational_parts(
        library_simplex.dual_vertices.clone(),
        library_simplex.vertices.clone(),
    )
    .expect("simplex cache");
    let simplex_best = capacity_pruned_hk2017(
        &simplex_cache.dual_vertices,
        &simplex_cache.dual_vertices_f64,
        &simplex_cache.facet_intersection_is_nonempty,
        &simplex_cache.omega_signs,
    )
    .expect("library simplex capacity");
    let simplex_reference = library_rational_solver::solve_kkt_exact(
        &library_simplex.dual_vertices,
        simplex_best.best_sigma(),
    )
    .expect("library simplex rational sigma");

    let library_hypercube = known_polytopes::hypercube();
    let hypercube_cache = NumericsPolytopeCache::from_rational_parts(
        library_hypercube.dual_vertices.clone(),
        library_hypercube.vertices.clone(),
    )
    .expect("hypercube cache");
    let hypercube_best = capacity_pruned_hk2017(
        &hypercube_cache.dual_vertices,
        &hypercube_cache.dual_vertices_f64,
        &hypercube_cache.facet_intersection_is_nonempty,
        &hypercube_cache.omega_signs,
    )
    .expect("library hypercube capacity");
    let hypercube_reference = library_rational_solver::solve_kkt_exact(
        &library_hypercube.dual_vertices,
        hypercube_best.best_sigma(),
    )
    .expect("library hypercube rational sigma");

    let library_hko = known_polytopes::hko_pentagon();
    let hko_winning_reference =
        library_rational_solver::solve_kkt_exact(&library_hko.dual_vertices, HKO_WINNING_SIGMA)
            .expect("library hko winning sigma");
    let hko_rank_deficient_reference = library_rational_solver::solve_kkt_exact(
        &library_hko.dual_vertices,
        HKO_RANK_DEFICIENT_SIGMA,
    )
    .expect("library hko rank-deficient sigma");

    let kkt_rows = vec![
        exact_kkt_row(
            "simplex_exact_q",
            "best_sigma",
            simplex_best.best_sigma(),
            &exact_simplex,
            "library_rational_solver",
            Some(simplex_reference.q_exact_f64),
        ),
        exact_kkt_row(
            "hypercube_exact_q",
            "best_sigma",
            hypercube_best.best_sigma(),
            &exact_hypercube,
            "library_rational_solver",
            Some(hypercube_reference.q_exact_f64),
        ),
        exact_kkt_row(
            "hko_pentagon_exact_pentagon_field",
            "winning_sigma",
            HKO_WINNING_SIGMA,
            &exact_hko,
            "library_rational_solver",
            Some(hko_winning_reference.q_exact_f64),
        ),
        exact_kkt_row(
            "hko_pentagon_exact_pentagon_field",
            "rank_deficient_sigma",
            HKO_RANK_DEFICIENT_SIGMA,
            &exact_hko,
            "library_rational_solver",
            Some(hko_rank_deficient_reference.q_exact_f64),
        ),
    ];
    write_jsonl(&kkt_output_path, &kkt_rows);

    let winning_action = kkt_rows
        .iter()
        .find(|row| {
            row.name == "hko_pentagon_exact_pentagon_field" && row.sigma_label == "winning_sigma"
        })
        .expect("winning hko row")
        .action_exact_f64;
    let expected_capacity = hko_capacity_formula_f64();
    let hko_incidence_match = same_incidence(&hko_incidence, &library_hko.vertex_facet_incidence);
    let hko_adjacency_match =
        same_bool_matrix(&hko_adjacency, &library_hko.facet_intersection_is_nonempty);
    let hko_omega_match = same_i8_matrix(&hko_omega_signs, &library_hko.omega_signs);
    let hko_omega_mismatches = omega_mismatch_positions(&hko_omega_signs, &library_hko.omega_signs);

    println!("exact-polytopes rows: {}", polytope_rows.len());
    println!("exact-kkt rows: {}", kkt_rows.len());
    println!(
        "Output mode: {:?}\npolytope output: {}\nkkt output: {}",
        mode,
        polytope_output_path.display(),
        kkt_output_path.display()
    );
    println!(
        "HKO combinatorics vs dyadic library: incidence_match={}, adjacency_match={}, omega_match={}",
        hko_incidence_match, hko_adjacency_match, hko_omega_match
    );
    if !hko_omega_mismatches.is_empty() {
        println!(
            "HKO omega mismatches (facet_i, facet_j, exact, dyadic): {:?}",
            hko_omega_mismatches
        );
    }
    println!(
        "HKO winning action: {:.15} (analytic capacity {:.15}, diff {:.3e})",
        winning_action,
        expected_capacity,
        (winning_action - expected_capacity).abs()
    );
}

#[cfg(test)]
mod tests {
    use super::{has_off_diagonal_zero_omega, parse_run_mode, RunMode, RunModeArgError};

    #[test]
    fn diagonal_zero_omega_does_not_trigger_catalog_flag() {
        let omega = vec![vec![0, 1], vec![-1, 0]];
        assert!(!has_off_diagonal_zero_omega(&omega));
    }

    #[test]
    fn off_diagonal_zero_omega_triggers_catalog_flag() {
        let omega = vec![vec![0, 0], vec![0, 0]];
        assert!(has_off_diagonal_zero_omega(&omega));
    }

    #[test]
    fn run_mode_defaults_to_smoke() {
        assert_eq!(parse_run_mode(Vec::<String>::new()), Ok(RunMode::Smoke));
    }

    #[test]
    fn canonical_flag_switches_run_mode() {
        assert_eq!(
            parse_run_mode(vec!["--canonical".to_string()]),
            Ok(RunMode::Canonical)
        );
    }

    #[test]
    fn unknown_argument_is_rejected() {
        assert_eq!(
            parse_run_mode(vec!["--unexpected".to_string()]),
            Err(RunModeArgError::Unknown)
        );
    }
}

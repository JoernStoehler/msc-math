//! Export the selected exact-bank rows for independent Sage validation.
//!
//! Goal: persist a canonical algebraic record of the selected HKO/control exact
//! one-sigma calculations so Sage can recompute them independently and compare
//! exact values.
//! Input Artifacts: None (starts from the hardcoded exact HKO/control fixtures).
//! Output Artifacts: experiments/hko-local-maximum/theorem/row-bank-validation/row-bank-validation-input.jsonl
//!
//! Modes:
//! 1. `cargo run -p exp-hko-local-maximum --release --bin hko-row-bank-validation`
//!    writes `smoke-row-bank-validation-input.jsonl`.
//! 2. `cargo run -p exp-hko-local-maximum --release --bin hko-row-bank-validation -- --smoke`
//!    also writes `smoke-row-bank-validation-input.jsonl`.
//! 3. `cargo run -p exp-hko-local-maximum --release --bin hko-row-bank-validation -- --canonical`
//!    refreshes `row-bank-validation-input.jsonl`.

use exp_hko_local_maximum::{
    exact_hko_dual_vertices, exact_simplex_dual_vertices, ExactBankEntry, ExactBankTarget,
    HkoExactScalar, EXACT_BANK_ENTRIES,
};
use nalgebra::Vector4;
use serde::Serialize;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use symplectic::exact::{capacity_derivatives_a_exact_from_orbit, solve_orbit_sigma_exact};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct CliOptions {
    canonical: bool,
}

impl CliOptions {
    fn parse_from<I, S>(args: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let mut canonical = false;
        for arg in args {
            match arg.as_ref() {
                "--canonical" => canonical = true,
                "--smoke" => {}
                other => panic!("unsupported argument: {other}"),
            }
        }
        Self { canonical }
    }
}

#[derive(Debug, Serialize)]
struct RowBankValidationInputRow {
    row_name: String,
    polytope: String,
    exact_field: String,
    sigma_label: String,
    sigma: Vec<usize>,
    dual_vertices: Vec<Vec<CanonicalElement>>,
    rust_status: String,
    rust_q: CanonicalElement,
    rust_action: CanonicalElement,
    rust_beta: Vec<CanonicalElement>,
    rust_capacity_gradient_a: Vec<Vec<CanonicalElement>>,
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

fn experiment_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("theorem/row-bank-validation")
}

fn output_path(canonical: bool) -> PathBuf {
    let base = experiment_dir();
    let filename = if canonical {
        "row-bank-validation-input.jsonl"
    } else {
        "smoke-row-bank-validation-input.jsonl"
    };
    base.join(filename)
}

fn canonical_vec4<F: HkoExactScalar>(vector: &Vector4<F>) -> Vec<CanonicalElement> {
    vector.iter().map(CanonicalElement::from_field).collect()
}

fn build_row<F: HkoExactScalar + 'static>(
    entry: &ExactBankEntry,
    dual_vertices: &[Vector4<F>],
) -> RowBankValidationInputRow {
    let orbit = solve_orbit_sigma_exact(dual_vertices, entry.sigma)
        .expect("selected row-bank validation sigma must solve exactly");
    let gradient = capacity_derivatives_a_exact_from_orbit(dual_vertices, &orbit);

    RowBankValidationInputRow {
        row_name: entry.row_name.to_string(),
        polytope: entry.target.polytope_name().to_string(),
        exact_field: entry.target.exact_field().to_string(),
        sigma_label: entry.sigma_label.to_string(),
        sigma: entry.sigma.to_vec(),
        dual_vertices: dual_vertices.iter().map(canonical_vec4).collect(),
        rust_status: "solved".to_string(),
        rust_q: CanonicalElement::from_field(&orbit.q),
        rust_action: CanonicalElement::from_field(&orbit.action()),
        rust_beta: orbit
            .beta
            .iter()
            .map(CanonicalElement::from_field)
            .collect(),
        rust_capacity_gradient_a: gradient.iter().map(canonical_vec4).collect(),
    }
}

fn build_rows() -> Vec<RowBankValidationInputRow> {
    let exact_hko = exact_hko_dual_vertices();
    let exact_simplex = exact_simplex_dual_vertices();

    EXACT_BANK_ENTRIES
        .iter()
        .map(|entry| match entry.target {
            ExactBankTarget::HkoPentagon => build_row(entry, &exact_hko),
            ExactBankTarget::SimplexControl => build_row(entry, &exact_simplex),
        })
        .collect()
}

fn write_jsonl_rows<T: Serialize>(path: &Path, rows: &[T]) {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .unwrap_or_else(|error| panic!("create {}: {error}", parent.display()));
    }
    let file =
        File::create(path).unwrap_or_else(|error| panic!("create {}: {error}", path.display()));
    let mut writer = BufWriter::new(file);
    for row in rows {
        serde_json::to_writer(&mut writer, row).expect("serialize jsonl row");
        writer.write_all(b"\n").expect("write newline");
    }
    writer.flush().expect("flush writer");
}

fn main() {
    let options = CliOptions::parse_from(std::env::args().skip(1));
    let output = output_path(options.canonical);
    let rows = build_rows();
    write_jsonl_rows(&output, &rows);

    println!(
        "wrote {} row-bank validation input rows to {}",
        rows.len(),
        output.display()
    );
}

#[cfg(test)]
mod tests {
    use super::{build_rows, experiment_dir, output_path, CliOptions};
    use std::path::{Path, PathBuf};

    #[test]
    fn cli_options_accept_smoke_and_canonical() {
        assert_eq!(
            CliOptions::parse_from(["--smoke"]),
            CliOptions { canonical: false }
        );
        assert_eq!(
            CliOptions::parse_from(["--canonical"]),
            CliOptions { canonical: true }
        );
    }

    #[test]
    fn output_paths_match_contract() {
        assert_eq!(
            output_path(false),
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("theorem/row-bank-validation")
                .join("smoke-row-bank-validation-input.jsonl")
        );
        assert_eq!(
            output_path(true),
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("theorem/row-bank-validation")
                .join("row-bank-validation-input.jsonl")
        );
    }

    #[test]
    fn experiment_dir_is_manifest_relative() {
        assert_eq!(
            experiment_dir(),
            Path::new(env!("CARGO_MANIFEST_DIR")).join("theorem/row-bank-validation")
        );
    }

    #[test]
    fn build_rows_covers_selected_exact_bank() {
        let rows = build_rows();
        assert_eq!(rows.len(), 6);
        assert!(rows.iter().all(|row| row.rust_status == "solved"));
        assert!(rows.iter().any(|row| row.polytope == "hko_pentagon"));
        assert!(rows.iter().any(|row| row.polytope == "simplex_control"));
    }
}

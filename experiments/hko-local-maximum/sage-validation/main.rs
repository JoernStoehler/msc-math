//! Export the selected exact-bank rows for independent Sage validation.
//!
//! Goal: persist a canonical algebraic record of the selected HKO/control exact
//! one-sigma calculations so Sage can recompute them independently and compare
//! exact values plus kernel timings.
//! Input Artifacts: None (starts from the hardcoded exact HKO/control fixtures).
//! Output Artifacts: experiments/hko-local-maximum/sage-validation/sage-validation-input.jsonl
//!
//! Modes:
//! 1. `cargo run -p exp-hko-local-maximum --release --bin hko-sage-validation`
//!    writes `smoke-sage-validation-input.jsonl`.
//! 2. `cargo run -p exp-hko-local-maximum --release --bin hko-sage-validation -- --smoke`
//!    also writes `smoke-sage-validation-input.jsonl`.
//! 3. `cargo run -p exp-hko-local-maximum --release --bin hko-sage-validation -- --canonical`
//!    refreshes `sage-validation-input.jsonl`.

use exp_hko_local_maximum::{
    exact_hko_polytope, exact_simplex_polytope, ExactBankEntry, ExactBankTarget, EXACT_BANK_ENTRIES,
};
use real_algebraic::{canonical_element, CanonicalElement, OrderedField};
use serde::Serialize;
use std::fs::File;
use std::hint::black_box;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use std::time::Instant;
use symplectic::exact::{capacity_derivatives_a_exact, solve_orbit_sigma_exact, ExactPolytope4D};

const TIMING_REPETITIONS: usize = 7;

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
struct SageValidationInputRow {
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
    rust_solve_ns_median: u64,
    rust_gradient_ns_median: u64,
    rust_timing_repetitions: usize,
}

fn output_path(canonical: bool) -> PathBuf {
    let base = Path::new("experiments/hko-local-maximum/sage-validation");
    let filename = if canonical {
        "sage-validation-input.jsonl"
    } else {
        "smoke-sage-validation-input.jsonl"
    };
    base.join(filename)
}

fn median_duration_ns<F>(repetitions: usize, mut run: F) -> u64
where
    F: FnMut(),
{
    let mut samples = Vec::with_capacity(repetitions);
    for _ in 0..repetitions {
        let start = Instant::now();
        run();
        samples.push(start.elapsed().as_nanos() as u64);
    }
    samples.sort_unstable();
    samples[repetitions / 2]
}

fn canonical_vec4<F: OrderedField>(vector: &[F; 4]) -> Vec<CanonicalElement> {
    vector.iter().map(canonical_element).collect()
}

fn build_row<F: OrderedField>(
    entry: &ExactBankEntry,
    polytope: &ExactPolytope4D<F>,
) -> SageValidationInputRow {
    let orbit = solve_orbit_sigma_exact(polytope, entry.sigma)
        .expect("selected Sage-validation sigma must solve exactly");
    let gradient = capacity_derivatives_a_exact(polytope, &orbit);

    let solve_ns = median_duration_ns(TIMING_REPETITIONS, || {
        let solved = solve_orbit_sigma_exact(black_box(polytope), black_box(entry.sigma))
            .expect("timed exact solve must succeed");
        black_box(solved);
    });
    let gradient_ns = median_duration_ns(TIMING_REPETITIONS, || {
        let derived = capacity_derivatives_a_exact(black_box(polytope), black_box(&orbit));
        black_box(derived);
    });

    SageValidationInputRow {
        row_name: entry.row_name.to_string(),
        polytope: entry.target.polytope_name().to_string(),
        exact_field: entry.target.exact_field().to_string(),
        sigma_label: entry.sigma_label.to_string(),
        sigma: entry.sigma.to_vec(),
        dual_vertices: polytope
            .dual_vertices()
            .iter()
            .map(canonical_vec4)
            .collect(),
        rust_status: "solved".to_string(),
        rust_q: canonical_element(&orbit.q),
        rust_action: canonical_element(&orbit.action()),
        rust_beta: orbit.beta.iter().map(canonical_element).collect(),
        rust_capacity_gradient_a: gradient.iter().map(canonical_vec4).collect(),
        rust_solve_ns_median: solve_ns,
        rust_gradient_ns_median: gradient_ns,
        rust_timing_repetitions: TIMING_REPETITIONS,
    }
}

fn build_rows() -> Vec<SageValidationInputRow> {
    let exact_hko = exact_hko_polytope();
    let exact_simplex = exact_simplex_polytope();

    EXACT_BANK_ENTRIES
        .iter()
        .map(|entry| match entry.target {
            ExactBankTarget::HkoPentagon => build_row(entry, &exact_hko),
            ExactBankTarget::SimplexControl => build_row(entry, &exact_simplex),
        })
        .collect()
}

fn write_jsonl_rows<T: Serialize>(path: &Path, rows: &[T]) {
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
        "wrote {} Sage-validation input rows to {}",
        rows.len(),
        output.display()
    );
}

#[cfg(test)]
mod tests {
    use super::{build_rows, output_path, CliOptions};

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
            output_path(false).to_string_lossy(),
            "experiments/hko-local-maximum/sage-validation/smoke-sage-validation-input.jsonl"
        );
        assert_eq!(
            output_path(true).to_string_lossy(),
            "experiments/hko-local-maximum/sage-validation/sage-validation-input.jsonl"
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

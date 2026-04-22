//! Export a deterministic benchmark bank for Sage end-to-end capacity runs.
//!
//! Goal: measure whether Sage can carry an HK2017-style unpruned search loop
//! end-to-end on a small bank of rational and algebraic test polytopes, and
//! compare that against the existing Rust `f64` path on the same inputs.
//! Input Artifacts: None (starts from hardcoded rational controls and the exact HKO fixture).
//! Output Artifacts: experiments/numerics/sage-feasibility/{sage-feasibility-input.jsonl,smoke-sage-feasibility-input.jsonl}
//!
//! Modes:
//! 1. `cargo run -p dev-numerical-analysis --release --bin num-sage-feasibility`
//!    writes `smoke-sage-feasibility-input.jsonl`.
//! 2. `cargo run -p dev-numerical-analysis --release --bin num-sage-feasibility -- --smoke`
//!    also writes `smoke-sage-feasibility-input.jsonl`.
//! 3. `cargo run -p dev-numerical-analysis --release --bin num-sage-feasibility -- --canonical`
//!    refreshes `sage-feasibility-input.jsonl`.

use algebraic_numbers::{canonical_element, CanonicalElement, OrderedField};
use dev_numerical_analysis::algebraic::field::ExactOrderedField;
use dev_numerical_analysis::algebraic::fixtures::exact_hko_pentagon;
use nalgebra::Vector4;
use num_bigint::BigInt;
use num_rational::BigRational;
use serde::Serialize;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use std::time::Instant;
use symplectic::ehz_capacity_unpruned;
use symplectic::geom::polytope::Polytope4D;

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
        let mut smoke = false;
        for arg in args {
            match arg.as_ref() {
                "--canonical" => canonical = true,
                "--smoke" => smoke = true,
                other => panic!("unsupported argument: {other}"),
            }
        }
        assert!(
            !(canonical && smoke),
            "`--canonical` and `--smoke` are separate output modes"
        );
        Self { canonical }
    }
}

#[derive(Clone)]
struct RationalFixture {
    name: &'static str,
    dual_vertices: Vec<[BigRational; 4]>,
}

#[derive(Debug, Serialize)]
struct SageFeasibilityInputRow {
    polytope: String,
    family: String,
    facet_count: usize,
    exact_field: String,
    dual_vertices: Vec<Vec<CanonicalElement>>,
    rust_f64_capacity: f64,
    rust_f64_iterations: u64,
    rust_f64_returned_orbit_count: usize,
    rust_f64_best_sigma: Vec<usize>,
    rust_f64_wall_time_ms: f64,
}

fn experiment_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("sage-feasibility")
}

fn output_path(canonical: bool) -> PathBuf {
    let base = experiment_dir();
    let filename = if canonical {
        "sage-feasibility-input.jsonl"
    } else {
        "smoke-sage-feasibility-input.jsonl"
    };
    base.join(filename)
}

fn rat(n: i64) -> BigRational {
    BigRational::from_integer(BigInt::from(n))
}

fn canonical_vec4<F: OrderedField>(vector: &[F; 4]) -> Vec<CanonicalElement> {
    vector.iter().map(canonical_element).collect()
}

fn simplex_f5() -> RationalFixture {
    let z = rat(0);
    RationalFixture {
        name: "simplex_f5",
        dual_vertices: vec![
            [rat(-5), z.clone(), z.clone(), z.clone()],
            [z.clone(), rat(-5), z.clone(), z.clone()],
            [z.clone(), z.clone(), rat(-5), z.clone()],
            [z.clone(), z.clone(), z.clone(), rat(-5)],
            [rat(5), rat(5), rat(5), rat(5)],
        ],
    }
}

fn cut_simplex_f6() -> RationalFixture {
    let mut dual_vertices = simplex_f5().dual_vertices;
    dual_vertices.push([rat(6), rat(0), rat(0), rat(0)]);
    RationalFixture {
        name: "cut_simplex_f6",
        dual_vertices,
    }
}

fn double_cut_simplex_f7() -> RationalFixture {
    let mut dual_vertices = cut_simplex_f6().dual_vertices;
    dual_vertices.push([rat(0), rat(6), rat(0), rat(0)]);
    RationalFixture {
        name: "double_cut_simplex_f7",
        dual_vertices,
    }
}

fn hypercube_f8() -> RationalFixture {
    let z = rat(0);
    RationalFixture {
        name: "hypercube_f8",
        dual_vertices: vec![
            [rat(1), z.clone(), z.clone(), z.clone()],
            [rat(-1), z.clone(), z.clone(), z.clone()],
            [z.clone(), rat(1), z.clone(), z.clone()],
            [z.clone(), rat(-1), z.clone(), z.clone()],
            [z.clone(), z.clone(), rat(1), z.clone()],
            [z.clone(), z.clone(), rat(-1), z.clone()],
            [z.clone(), z.clone(), z.clone(), rat(1)],
            [z.clone(), z.clone(), z.clone(), rat(-1)],
        ],
    }
}

fn cut_hypercube_f9() -> RationalFixture {
    let mut dual_vertices = hypercube_f8().dual_vertices;
    dual_vertices.push([rat(1), rat(1), rat(1), rat(1)]);
    RationalFixture {
        name: "cut_hypercube_f9",
        dual_vertices,
    }
}

fn double_cut_hypercube_f10() -> RationalFixture {
    let mut dual_vertices = cut_hypercube_f9().dual_vertices;
    dual_vertices.push([rat(1), rat(1), rat(-1), rat(-1)]);
    RationalFixture {
        name: "double_cut_hypercube_f10",
        dual_vertices,
    }
}

fn canonical_rational_bank(canonical: bool) -> Vec<RationalFixture> {
    let mut fixtures = vec![
        simplex_f5(),
        cut_simplex_f6(),
        double_cut_simplex_f7(),
        hypercube_f8(),
        cut_hypercube_f9(),
        double_cut_hypercube_f10(),
    ];
    if !canonical {
        fixtures = vec![simplex_f5(), hypercube_f8(), cut_hypercube_f9()];
    }
    fixtures
}

fn rust_unpruned_baseline(polytope: &Polytope4D) -> (f64, u64, usize, Vec<usize>, f64) {
    let start = Instant::now();
    let result = ehz_capacity_unpruned(polytope).expect("Rust unpruned baseline must succeed");
    let wall_time_ms = start.elapsed().as_secs_f64() * 1000.0;
    (
        result.capacity(),
        result.iterations,
        result.orbits.len(),
        result.best_sigma().to_vec(),
        wall_time_ms,
    )
}

fn rational_row(fixture: RationalFixture) -> SageFeasibilityInputRow {
    let polytope = Polytope4D::new(fixture.dual_vertices.clone())
        .unwrap_or_else(|error| panic!("{} construction failed: {error:?}", fixture.name));
    let (capacity, iterations, returned_orbit_count, best_sigma, wall_time_ms) =
        rust_unpruned_baseline(&polytope);

    SageFeasibilityInputRow {
        polytope: fixture.name.to_string(),
        family: "rational_control".to_string(),
        facet_count: polytope.facet_count(),
        exact_field: "rational".to_string(),
        dual_vertices: fixture.dual_vertices.iter().map(canonical_vec4).collect(),
        rust_f64_capacity: capacity,
        rust_f64_iterations: iterations,
        rust_f64_returned_orbit_count: returned_orbit_count,
        rust_f64_best_sigma: best_sigma,
        rust_f64_wall_time_ms: wall_time_ms,
    }
}

fn hko_row() -> SageFeasibilityInputRow {
    let exact_hko = exact_hko_pentagon().expect("exact HKO fixture");
    let f64_dual_vertices: Vec<_> = exact_hko
        .dual_vertices()
        .iter()
        .map(|dual| {
            Vector4::new(
                dual[0].to_f64(),
                dual[1].to_f64(),
                dual[2].to_f64(),
                dual[3].to_f64(),
            )
        })
        .collect();
    let rust_polytope =
        Polytope4D::from_f64(f64_dual_vertices).expect("Rust HKO f64 benchmark polytope");
    let (capacity, iterations, returned_orbit_count, best_sigma, wall_time_ms) =
        rust_unpruned_baseline(&rust_polytope);

    SageFeasibilityInputRow {
        polytope: "hko_pentagon_exact_f10".to_string(),
        family: "algebraic_hko".to_string(),
        facet_count: exact_hko.facet_count(),
        exact_field: "q_tan_pi_fifth".to_string(),
        dual_vertices: exact_hko
            .dual_vertices()
            .iter()
            .map(canonical_vec4)
            .collect(),
        rust_f64_capacity: capacity,
        rust_f64_iterations: iterations,
        rust_f64_returned_orbit_count: returned_orbit_count,
        rust_f64_best_sigma: best_sigma,
        rust_f64_wall_time_ms: wall_time_ms,
    }
}

fn build_rows(canonical: bool) -> Vec<SageFeasibilityInputRow> {
    let mut rows: Vec<_> = canonical_rational_bank(canonical)
        .into_iter()
        .map(rational_row)
        .collect();
    if canonical {
        rows.push(hko_row());
    }
    rows
}

fn bank_size(canonical: bool) -> usize {
    canonical_rational_bank(canonical).len() + usize::from(canonical)
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
    let rows = build_rows(options.canonical);
    write_jsonl_rows(&output, &rows);

    println!(
        "wrote {} Sage-feasibility input rows to {}",
        rows.len(),
        output.display()
    );
}

#[cfg(test)]
mod tests {
    use super::{
        bank_size, cut_hypercube_f9, cut_simplex_f6, double_cut_hypercube_f10,
        double_cut_simplex_f7, experiment_dir, hypercube_f8, output_path, simplex_f5, CliOptions,
    };
    use dev_numerical_analysis::algebraic::fixtures::exact_hko_pentagon;
    use std::path::{Path, PathBuf};
    use symplectic::geom::polytope::Polytope4D;

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
    #[should_panic(expected = "`--canonical` and `--smoke` are separate output modes")]
    fn cli_options_reject_conflicting_modes() {
        let _ = CliOptions::parse_from(["--canonical", "--smoke"]);
    }

    #[test]
    fn output_paths_match_contract() {
        assert_eq!(
            output_path(false),
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("sage-feasibility")
                .join("smoke-sage-feasibility-input.jsonl")
        );
        assert_eq!(
            output_path(true),
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("sage-feasibility")
                .join("sage-feasibility-input.jsonl")
        );
    }

    #[test]
    fn experiment_dir_is_manifest_relative() {
        assert_eq!(
            experiment_dir(),
            Path::new(env!("CARGO_MANIFEST_DIR")).join("sage-feasibility")
        );
    }

    #[test]
    fn rational_bank_fixture_facet_counts_are_stable() {
        let cases = vec![
            (simplex_f5(), 5usize),
            (cut_simplex_f6(), 6usize),
            (double_cut_simplex_f7(), 7usize),
            (hypercube_f8(), 8usize),
            (cut_hypercube_f9(), 9usize),
            (double_cut_hypercube_f10(), 10usize),
        ];

        for (fixture, expected_facets) in cases {
            let polytope = Polytope4D::new(fixture.dual_vertices)
                .unwrap_or_else(|error| panic!("{} should construct: {error:?}", fixture.name));
            assert_eq!(polytope.facet_count(), expected_facets, "{}", fixture.name);
        }
    }

    #[test]
    fn smoke_and_canonical_bank_sizes_are_intentional() {
        assert_eq!(bank_size(false), 3);
        assert_eq!(bank_size(true), 7);
    }

    #[test]
    fn hko_row_is_algebraic_f10() {
        let exact_hko = exact_hko_pentagon().expect("exact HKO fixture");
        assert_eq!(exact_hko.facet_count(), 10);
    }
}

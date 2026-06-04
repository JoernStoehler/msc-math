//! Candidate exporter for the HKO feasible-section certificate.
//!
//! This binary promotes a small 26-row candidate from the numerical
//! active-branch diagnostic into a dedicated candidate JSON file. The output is
//! a generator artifact, not a proof. The Sage verifier must later check exact
//! equations for the witness values derived from this candidate.

use serde_json::{json, Value};
use std::env;
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};

const SELECTED_ROWS: &[(usize, f64)] = &[
    (0, 0.0021861717243028163),
    (2, 0.02442653803666701),
    (8, 0.03272773526018294),
    (11, 0.039557185837217165),
    (17, 0.05183640645104748),
    (18, 0.0119328217297219),
    (21, 0.006776833414730591),
    (25, 0.05842052670554996),
    (29, 0.08029271605107965),
    (30, 0.009383420327956495),
    (32, 0.05633573557294202),
    (34, 0.06318790771040984),
    (36, 0.01844465622921349),
    (37, 0.006853420867865483),
    (40, 0.02344430454939738),
    (43, 0.012562003163),
    (46, 0.06396295003469424),
    (48, 0.05308168129237081),
    (54, 0.018579157983470047),
    (55, 0.06795276359113317),
    (57, 0.08011364621431648),
    (58, 0.06456749179311543),
    (62, 0.05490984799339861),
    (63, 0.0674751215942331),
    (66, 0.01799266935292128),
    (67, 0.012996286518897171),
];

#[derive(Clone, Debug)]
struct Options {
    input_path: PathBuf,
    output_path: PathBuf,
    output_mode: &'static str,
}

fn print_usage() {
    eprintln!(
        r#"Usage: hko-feasible-section-certificate [options]

Optional flags:
  --help, -h          Show this help message and exit.
  --smoke             Write smoke-candidate-certificate.json. This is the default.
  --canonical         Refresh candidate-certificate.json.
  --input <PATH>      Source active-branch diagnostic JSON."#
    );
}

impl Options {
    fn parse() -> Self {
        let packet_dir =
            PathBuf::from("experiments/hko-local-maximum/theorem/feasible-section-certificate");
        let mut input_path = PathBuf::from(
            "experiments/hko-local-maximum/theorem/active-branch-diagnostic/smoke-active-branch-diagnostic.json",
        );
        let mut output_path = packet_dir.join("smoke-candidate-certificate.json");
        let mut output_mode = "smoke";

        let mut args = env::args().skip(1);
        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--help" | "-h" => {
                    print_usage();
                    std::process::exit(0);
                }
                "--smoke" => {
                    output_path = packet_dir.join("smoke-candidate-certificate.json");
                    output_mode = "smoke";
                }
                "--canonical" => {
                    output_path = packet_dir.join("candidate-certificate.json");
                    output_mode = "canonical";
                }
                "--input" => {
                    let Some(path) = args.next() else {
                        panic!("--input requires a path");
                    };
                    input_path = PathBuf::from(path);
                }
                other => panic!("unsupported argument: {other}"),
            }
        }

        Self {
            input_path,
            output_path,
            output_mode,
        }
    }
}

fn read_json(path: &Path) -> Value {
    let file = File::open(path).unwrap_or_else(|err| {
        panic!(
            "failed to open {}: {err}. Run hko-active-branch-diagnostic first.",
            path.display()
        )
    });
    serde_json::from_reader(BufReader::new(file))
        .unwrap_or_else(|err| panic!("failed to parse {}: {err}", path.display()))
}

fn required<'a>(value: &'a Value, key: &str) -> &'a Value {
    value
        .get(key)
        .unwrap_or_else(|| panic!("missing key `{key}` in diagnostic row"))
}

fn selected_candidate_rows(diagnostic: &Value) -> Vec<Value> {
    let rows = diagnostic["feasible_section_rows"]
        .as_array()
        .expect("diagnostic must contain feasible_section_rows array");

    SELECTED_ROWS
        .iter()
        .enumerate()
        .map(|(certificate_index, &(source_index, lambda_hint))| {
            let row = rows.get(source_index).unwrap_or_else(|| {
                panic!(
                    "selected source index {source_index} outside feasible_section_rows length {}",
                    rows.len()
                )
            });
            json!({
                "certificate_index": certificate_index,
                "source_feasible_section_row_index": source_index,
                "lambda_hint_f64": lambda_hint,
                "sigma": required(row, "sigma"),
                "sigma_len": required(row, "sigma_len"),
                "source_kkt_singular_f64": required(row, "source_kkt_singular_f64"),
                "minor_columns_exact": required(row, "minor_columns_exact"),
                "fixed_beta_indices": required(row, "fixed_beta_indices"),
                "fixed_beta_values_f64": required(row, "fixed_beta_values_f64"),
                "beta_f64": required(row, "beta_f64"),
                "q_f64": required(row, "q_f64"),
                "action_f64": required(row, "action_f64"),
                "d_sys_flat_f64": required(row, "d_sys_flat_f64"),
            })
        })
        .collect()
}

fn write_json(path: &Path, payload: &Value) {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .unwrap_or_else(|err| panic!("failed to create {}: {err}", parent.display()));
    }
    let file = File::create(path)
        .unwrap_or_else(|err| panic!("failed to create {}: {err}", path.display()));
    let mut writer = BufWriter::new(file);
    serde_json::to_writer_pretty(&mut writer, payload)
        .unwrap_or_else(|err| panic!("failed to write {}: {err}", path.display()));
    writer
        .write_all(b"\n")
        .unwrap_or_else(|err| panic!("failed to finish {}: {err}", path.display()));
}

fn main() {
    let options = Options::parse();
    let diagnostic = read_json(&options.input_path);
    let candidate_rows = selected_candidate_rows(&diagnostic);

    let payload = json!({
        "packet": "hko-feasible-section-certificate",
        "candidate_version": 1,
        "output_mode": options.output_mode,
        "source_diagnostic_path": options.input_path,
        "source_diagnostic_version": diagnostic["diagnostic_version"],
        "source_summary": {
            "feasible_section_candidate_count": diagnostic["summary"]["feasible_section_candidate_count"],
            "feasible_section_projected_rank": diagnostic["summary"]["feasible_section_projected_rank"],
            "feasible_section_convex_hull_zero": diagnostic["summary"]["feasible_section_convex_hull_zero"],
        },
        "certificate_goal": {
            "selected_row_count": candidate_rows.len(),
            "ambient_dimension": 40,
            "symmetry_dimension": 15,
            "quotient_dimension": 25,
            "proof_use": "candidate choices for exact Sage feasible-section witness construction; Rust is not trusted for theorem correctness"
        },
        "rows": candidate_rows,
    });

    write_json(&options.output_path, &payload);
    println!("Wrote {}", options.output_path.display());
}

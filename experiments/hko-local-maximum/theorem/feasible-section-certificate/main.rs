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

#[derive(Clone, Copy, Debug)]
struct SelectedRow {
    source_index: usize,
    lambda_hint: f64,
    sigma: &'static [usize],
    minor_columns: &'static [usize],
}

const SELECTED_ROWS: &[SelectedRow] = &[
    SelectedRow {
        source_index: 0,
        lambda_hint: 0.0021861717243028163,
        sigma: &[0, 1, 7, 3, 9, 5],
        minor_columns: &[0, 1, 2, 3, 4],
    },
    SelectedRow {
        source_index: 2,
        lambda_hint: 0.02442653803666701,
        sigma: &[0, 1, 7, 6, 3, 9],
        minor_columns: &[0, 1, 2, 3, 4],
    },
    SelectedRow {
        source_index: 8,
        lambda_hint: 0.03272773526018294,
        sigma: &[1, 2, 7, 8, 4, 3, 5],
        minor_columns: &[0, 1, 2, 3, 4],
    },
    SelectedRow {
        source_index: 11,
        lambda_hint: 0.039557185837217165,
        sigma: &[0, 7, 6, 2, 3, 9],
        minor_columns: &[0, 1, 2, 3, 4],
    },
    SelectedRow {
        source_index: 17,
        lambda_hint: 0.05183640645104748,
        sigma: &[1, 8, 7, 4, 3, 5],
        minor_columns: &[0, 1, 2, 3, 4],
    },
    SelectedRow {
        source_index: 18,
        lambda_hint: 0.0119328217297219,
        sigma: &[1, 7, 3, 4, 5, 9],
        minor_columns: &[0, 1, 2, 3, 4],
    },
    SelectedRow {
        source_index: 21,
        lambda_hint: 0.006776833414730591,
        sigma: &[0, 5, 1, 7, 4, 3, 9],
        minor_columns: &[0, 1, 2, 3, 4],
    },
    SelectedRow {
        source_index: 25,
        lambda_hint: 0.05842052670554996,
        sigma: &[0, 6, 7, 3, 2, 8, 9],
        minor_columns: &[0, 1, 2, 3, 4],
    },
    SelectedRow {
        source_index: 29,
        lambda_hint: 0.08029271605107965,
        sigma: &[0, 4, 6, 5, 2, 9, 8],
        minor_columns: &[0, 1, 2, 3, 4],
    },
    SelectedRow {
        source_index: 30,
        lambda_hint: 0.009383420327956495,
        sigma: &[1, 7, 3, 2, 8, 4, 5],
        minor_columns: &[0, 1, 2, 3, 4],
    },
    SelectedRow {
        source_index: 32,
        lambda_hint: 0.05633573557294202,
        sigma: &[1, 2, 8, 7, 3, 4, 5],
        minor_columns: &[0, 1, 2, 3, 4],
    },
    SelectedRow {
        source_index: 34,
        lambda_hint: 0.06318790771040984,
        sigma: &[1, 7, 8, 4, 6, 5, 2],
        minor_columns: &[0, 1, 2, 3, 4],
    },
    SelectedRow {
        source_index: 36,
        lambda_hint: 0.01844465622921349,
        sigma: &[0, 6, 7, 3, 9, 1],
        minor_columns: &[0, 1, 2, 3, 4],
    },
    SelectedRow {
        source_index: 37,
        lambda_hint: 0.006853420867865483,
        sigma: &[0, 6, 3, 2, 8, 9],
        minor_columns: &[0, 1, 2, 3, 4],
    },
    SelectedRow {
        source_index: 40,
        lambda_hint: 0.02344430454939738,
        sigma: &[1, 8, 4, 5, 6, 2],
        minor_columns: &[0, 1, 2, 3, 4],
    },
    SelectedRow {
        source_index: 43,
        lambda_hint: 0.012562003163,
        sigma: &[1, 7, 2, 8, 4, 5],
        minor_columns: &[0, 1, 2, 3, 4],
    },
    SelectedRow {
        source_index: 46,
        lambda_hint: 0.06396295003469424,
        sigma: &[1, 7, 3, 4, 9, 5],
        minor_columns: &[0, 1, 2, 3, 4],
    },
    SelectedRow {
        source_index: 48,
        lambda_hint: 0.05308168129237081,
        sigma: &[0, 6, 2, 1, 7, 3, 9],
        minor_columns: &[0, 1, 2, 3, 4],
    },
    SelectedRow {
        source_index: 54,
        lambda_hint: 0.018579157983470047,
        sigma: &[0, 4, 5, 6, 2, 1, 8],
        minor_columns: &[0, 1, 2, 3, 4],
    },
    SelectedRow {
        source_index: 55,
        lambda_hint: 0.06795276359113317,
        sigma: &[0, 6, 2, 8, 4, 5, 1],
        minor_columns: &[0, 1, 2, 3, 4],
    },
    SelectedRow {
        source_index: 57,
        lambda_hint: 0.08011364621431648,
        sigma: &[0, 7, 4, 3, 5, 9, 1],
        minor_columns: &[0, 1, 2, 3, 4],
    },
    SelectedRow {
        source_index: 58,
        lambda_hint: 0.06456749179311543,
        sigma: &[0, 5, 1, 7, 6, 3, 9],
        minor_columns: &[0, 1, 2, 3, 4],
    },
    SelectedRow {
        source_index: 62,
        lambda_hint: 0.05490984799339861,
        sigma: &[0, 4, 6, 3, 2, 8, 9],
        minor_columns: &[0, 1, 2, 3, 5],
    },
    SelectedRow {
        source_index: 63,
        lambda_hint: 0.0674751215942331,
        sigma: &[0, 6, 2, 3, 9, 8, 4],
        minor_columns: &[0, 1, 2, 3, 4],
    },
    SelectedRow {
        source_index: 66,
        lambda_hint: 0.01799266935292128,
        sigma: &[0, 6, 5, 2, 9, 8, 4],
        minor_columns: &[0, 1, 2, 3, 4],
    },
    SelectedRow {
        source_index: 67,
        lambda_hint: 0.012996286518897171,
        sigma: &[1, 7, 8, 4, 3, 5, 2],
        minor_columns: &[0, 1, 2, 3, 4],
    },
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
        let package_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let packet_dir = package_root.join("theorem/feasible-section-certificate");
        let mut input_path = package_root
            .join("theorem/active-branch-diagnostic/smoke-active-branch-diagnostic.json");
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

fn path_for_json(path: &Path) -> String {
    let package_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    match path.strip_prefix(&package_root) {
        Ok(stripped) => Path::new("experiments/hko-local-maximum")
            .join(stripped)
            .display()
            .to_string(),
        Err(_) => path.display().to_string(),
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

fn required_usize_vec(value: &Value, key: &str) -> Vec<usize> {
    required(value, key)
        .as_array()
        .unwrap_or_else(|| panic!("diagnostic key `{key}` must be an array"))
        .iter()
        .map(|entry| {
            entry.as_u64().unwrap_or_else(|| {
                panic!("diagnostic key `{key}` must contain nonnegative integers")
            }) as usize
        })
        .collect()
}

fn selected_candidate_rows(diagnostic: &Value) -> Vec<Value> {
    let rows = diagnostic["feasible_section_rows"]
        .as_array()
        .expect("diagnostic must contain feasible_section_rows array");

    SELECTED_ROWS
        .iter()
        .enumerate()
        .map(|(certificate_index, selected)| {
            let row = rows.get(selected.source_index).unwrap_or_else(|| {
                panic!(
                    "selected source index {} outside feasible_section_rows length {}",
                    selected.source_index,
                    rows.len()
                )
            });
            let actual_sigma = required_usize_vec(row, "sigma");
            let actual_minor_columns = required_usize_vec(row, "minor_columns_exact");
            if actual_sigma.as_slice() != selected.sigma {
                panic!(
                    "selected source index {} has sigma {:?}, expected {:?}",
                    selected.source_index, actual_sigma, selected.sigma
                );
            }
            if actual_minor_columns.as_slice() != selected.minor_columns {
                panic!(
                    "selected source index {} has minor columns {:?}, expected {:?}",
                    selected.source_index, actual_minor_columns, selected.minor_columns
                );
            }
            json!({
                "certificate_index": certificate_index,
                "source_feasible_section_row_index": selected.source_index,
                "lambda_hint_f64": selected.lambda_hint,
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
        "candidate_role": "standalone finite row choices for exact Sage witness construction; diagnostic provenance is debug context only",
        "debug_source_diagnostic_path": path_for_json(&options.input_path),
        "debug_source_diagnostic_version": diagnostic["diagnostic_version"],
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

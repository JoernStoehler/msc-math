//! Generate finite data for the HKO feasible-section certificate.
//!
//! This binary promotes a small 26-entry selection from the numerical
//! active-branch diagnostic into `witness.json`. The output is finite verifier
//! input, not a proof by itself. SageMath reads this file, computes exact data,
//! and verifies the exact certificate predicate.

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
    input_was_explicit: bool,
    output_path: PathBuf,
    output_mode: &'static str,
}

fn print_usage() {
    eprintln!(
        r#"Usage: hko-feasible-section-generate [options]

Optional flags:
  --help, -h          Show this help message and exit.
  --smoke             Write smoke-witness.json. This is the default and may use the default smoke input.
  --canonical         Refresh witness.json. Requires an explicit --input.
  --input <PATH>      Source active-branch diagnostic JSON."#
    );
}

impl Options {
    fn parse() -> Self {
        let package_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let packet_dir = package_root.join("theorem/feasible-section-certificate");
        let default_input_path = package_root
            .join("theorem/active-branch-diagnostic/smoke-active-branch-diagnostic.json");
        let mut input_path = None;
        let mut output_path = packet_dir.join("smoke-witness.json");
        let mut output_mode = "smoke";

        let mut args = env::args().skip(1);
        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--help" | "-h" => {
                    print_usage();
                    std::process::exit(0);
                }
                "--smoke" => {
                    output_path = packet_dir.join("smoke-witness.json");
                    output_mode = "smoke";
                }
                "--canonical" => {
                    output_path = packet_dir.join("witness.json");
                    output_mode = "canonical";
                }
                "--input" => {
                    let Some(path) = args.next() else {
                        panic!("--input requires a path");
                    };
                    input_path = Some(PathBuf::from(path));
                }
                other => panic!("unsupported argument: {other}"),
            }
        }

        if output_mode == "canonical" && input_path.is_none() {
            panic!("--canonical requires an explicit --input path");
        }
        let input_was_explicit = input_path.is_some();
        let input_path = input_path.unwrap_or(default_input_path);

        Self {
            input_path,
            input_was_explicit,
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

fn required_f64_vec(value: &Value, key: &str) -> Vec<f64> {
    required(value, key)
        .as_array()
        .unwrap_or_else(|| panic!("diagnostic key `{key}` must be an array"))
        .iter()
        .map(|entry| {
            entry
                .as_f64()
                .unwrap_or_else(|| panic!("diagnostic key `{key}` must contain numbers"))
        })
        .collect()
}

fn complement_indices(len: usize, selected: &[usize]) -> Vec<usize> {
    (0..len).filter(|idx| !selected.contains(idx)).collect()
}

fn expected_fixed_beta_values(source_index: usize) -> &'static [f64] {
    match source_index {
        0 => &[0.13819660112489568],
        2 => &[0.22360679774997907],
        8 => &[0.06909830056250513, 0.22360679774997885],
        11 => &[0.22360679774997871],
        17 => &[0.22360679774997894],
        18 => &[0.13819660112501053],
        21 => &[0.180901699437495, 0.13819660112501064],
        25 => &[0.0690983005625053, 0.18090169943749473],
        29 => &[0.06909830056250515, 0.18090169943749462],
        30 => &[0.1809016994374947, 0.223606797749979],
        32 => &[0.18090169943749512, 0.22360679774997916],
        34 => &[0.18090169943749473, 0.13819660112501053],
        36 => &[0.1381966011250107],
        37 => &[0.1381966011250105],
        40 => &[0.1381966011250105],
        43 => &[0.2236067977499791],
        46 => &[0.13819660112501053],
        48 => &[0.18090169943749507, 0.22360679774997883],
        54 => &[0.06909830056250547, 0.22360679774997916],
        55 => &[0.13819660112501048, 0.06909830056250552],
        57 => &[0.1381966011250106, 0.14926352961534042],
        58 => &[0.22360679774997896, 0.1809016994374947],
        62 => &[0.18090169943749482, 0.13819660112501087],
        63 => &[0.13819660112501067, 0.06909830056250518],
        66 => &[0.1809016994374948, 0.1381966011250105],
        67 => &[0.22360679774997913, 0.12028993467659169],
        other => panic!("no expected fixed beta values for selected source index {other}"),
    }
}

fn check_f64_slice_close(source_index: usize, label: &str, actual: &[f64], expected: &[f64]) {
    if actual.len() != expected.len() {
        panic!(
            "selected source index {source_index} has {label} length {}, expected {}",
            actual.len(),
            expected.len()
        );
    }
    for (idx, (&actual, &expected)) in actual.iter().zip(expected.iter()).enumerate() {
        if (actual - expected).abs() > 1.0e-12 {
            panic!(
                "selected source index {source_index} has {label}[{idx}] {actual}, expected {expected}"
            );
        }
    }
}

fn selected_witness_entries(diagnostic: &Value) -> Vec<Value> {
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
            let actual_fixed_indices = required_usize_vec(row, "fixed_beta_indices");
            let actual_fixed_beta_values = required_f64_vec(row, "fixed_beta_values_f64");
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
            let expected_fixed_indices =
                complement_indices(actual_sigma.len(), selected.minor_columns);
            if actual_fixed_indices != expected_fixed_indices {
                panic!(
                    "selected source index {} has fixed beta indices {:?}, expected {:?}",
                    selected.source_index, actual_fixed_indices, expected_fixed_indices
                );
            }
            check_f64_slice_close(
                selected.source_index,
                "fixed_beta_values_f64",
                &actual_fixed_beta_values,
                expected_fixed_beta_values(selected.source_index),
            );
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
    let witness_entries = selected_witness_entries(&diagnostic);

    let payload = json!({
        "packet": "hko-feasible-section-certificate",
        "witness_version": 1,
        "output_mode": options.output_mode,
        "witness_role": "finite verifier input generated from the active-branch diagnostic; SageMath computes exact data and verifies it",
        "debug_source_diagnostic_path": path_for_json(&options.input_path),
        "debug_source_diagnostic_version": diagnostic["diagnostic_version"],
        "debug_source_input_was_explicit": options.input_was_explicit,
        "certificate_goal": {
            "selected_entry_count": witness_entries.len(),
            "ambient_dimension": 40,
            "symmetry_dimension": 15,
            "quotient_dimension": 25,
            "proof_use": "input for SageMath exact verification; Rust generation is not trusted for theorem correctness"
        },
        "entries": witness_entries,
    });

    write_json(&options.output_path, &payload);
    println!("Wrote {}", options.output_path.display());
}

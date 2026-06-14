//! Summarize the f64 rank defect in the smooth-only HKO route attempt.
//!
//! Input Artifacts: experiments/hko-local-maximum/theorem/smoke-active-branch-diagnostic.json
//! Output Artifacts: experiments/hko-local-maximum/smooth-only-rank-defect/summary.json

use serde_json::{json, Value};
use std::fs;
use std::path::{Path, PathBuf};

fn print_usage() {
    eprintln!(
        r#"Usage: hko-smooth-only-rank-defect [options]

Optional flags:
  --help, -h       Show this help message and exit.
  --input <PATH>   Source active-branch diagnostic JSON.
  --output <PATH>  Output summary JSON."#
    );
}

#[derive(Debug)]
struct CliOptions {
    input_path: PathBuf,
    output_path: PathBuf,
}

impl CliOptions {
    fn parse_from<I, S>(args: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let experiment_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
        let mut input_path = experiment_dir.join("theorem/smoke-active-branch-diagnostic.json");
        let mut output_path = experiment_dir.join("smooth-only-rank-defect/summary.json");

        let mut args = args.into_iter();
        while let Some(arg) = args.next() {
            match arg.as_ref() {
                "--help" | "-h" => {
                    print_usage();
                    std::process::exit(0);
                }
                "--input" => {
                    let Some(value) = args.next() else {
                        panic!("--input requires a path");
                    };
                    input_path = PathBuf::from(value.as_ref());
                }
                "--output" => {
                    let Some(value) = args.next() else {
                        panic!("--output requires a path");
                    };
                    output_path = PathBuf::from(value.as_ref());
                }
                other => panic!("unsupported argument: {other}"),
            }
        }

        Self {
            input_path,
            output_path,
        }
    }
}

fn required_usize<'a>(value: &'a Value, path: &[&str]) -> usize {
    let mut cursor = value;
    for key in path {
        cursor = cursor
            .get(*key)
            .unwrap_or_else(|| panic!("missing JSON key {}", path.join(".")));
    }
    cursor
        .as_u64()
        .unwrap_or_else(|| panic!("JSON key {} must be an unsigned integer", path.join(".")))
        as usize
}

fn required_bool<'a>(value: &'a Value, path: &[&str]) -> bool {
    let mut cursor = value;
    for key in path {
        cursor = cursor
            .get(*key)
            .unwrap_or_else(|| panic!("missing JSON key {}", path.join(".")));
    }
    cursor
        .as_bool()
        .unwrap_or_else(|| panic!("JSON key {} must be a boolean", path.join(".")))
}

fn required_value<'a>(value: &'a Value, path: &[&str]) -> &'a Value {
    let mut cursor = value;
    for key in path {
        cursor = cursor
            .get(*key)
            .unwrap_or_else(|| panic!("missing JSON key {}", path.join(".")));
    }
    cursor
}

fn optional_value<'a>(value: &'a Value, path: &[&str]) -> Option<&'a Value> {
    let mut cursor = value;
    for key in path {
        cursor = cursor.get(*key)?;
    }
    Some(cursor)
}

fn display_path(path: &Path, package_root: &Path) -> String {
    path.strip_prefix(package_root)
        .unwrap_or(path)
        .display()
        .to_string()
}

fn main() {
    let options = CliOptions::parse_from(std::env::args().skip(1));
    let package_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let diagnostic_text = fs::read_to_string(&options.input_path).unwrap_or_else(|err| {
        panic!(
            "failed to read {}: {err}. Run hko-active-branch-diagnostic first.",
            options.input_path.display()
        )
    });
    let diagnostic: Value =
        serde_json::from_str(&diagnostic_text).expect("input diagnostic must be valid JSON");

    let summary = required_value(&diagnostic, &["summary"]);
    let quotient_dimension = required_usize(summary, &["slice_dimension_exact"]);
    let nonsingular_rank =
        required_usize(summary, &["f64_nonsingular_active_projected_rank", "rank"]);
    let nonsingular_row_count = required_usize(
        summary,
        &["f64_nonsingular_active_projected_rank", "row_count"],
    );
    let nonsingular_dimension = required_usize(
        summary,
        &["f64_nonsingular_active_projected_rank", "column_count"],
    );
    let nonsingular_positive_lambda_rank = optional_value(
        summary,
        &[
            "f64_nonsingular_active_convex_hull_zero",
            "positive_lambda_projected_rank",
        ],
    )
    .and_then(Value::as_u64)
    .map(|value| value as usize);
    let feasible_section_rank =
        required_usize(summary, &["feasible_section_projected_rank", "rank"]);
    let feasible_section_row_count =
        required_usize(summary, &["feasible_section_projected_rank", "row_count"]);
    let feasible_section_positive_lambda_rank = optional_value(
        summary,
        &[
            "feasible_section_convex_hull_zero",
            "positive_lambda_projected_rank",
        ],
    )
    .and_then(Value::as_u64)
    .map(|value| value as usize);
    let padded_nonsingular_count = required_usize(
        summary,
        &["padded_extension_nonsingular_min_action_padded_once_count"],
    );

    let rank_defect = quotient_dimension.saturating_sub(nonsingular_rank);
    let attempt_closes = nonsingular_rank == quotient_dimension;

    let output = json!({
        "summary_version": 1,
        "source": {
            "diagnostic_path": display_path(&options.input_path, package_root),
            "diagnostic_version": diagnostic.get("diagnostic_version").cloned().unwrap_or(Value::Null),
            "diagnostic_output_mode": diagnostic.get("output_mode").cloned().unwrap_or(Value::Null)
        },
        "checked_attempt": {
            "description": "Use only positive-beta active branches whose f64 KKT matrix is nonsingular at the HKO point.",
            "evidence_status": "f64 numerical diagnostic, not theorem proof",
            "active_branch_count": required_usize(summary, &["f64_active_branch_count"]),
            "nonsingular_branch_count": nonsingular_row_count,
            "singular_branch_count": required_usize(summary, &["f64_active_kkt_singular_count"]),
            "quotient_dimension": quotient_dimension,
            "projected_matrix_column_count": nonsingular_dimension,
            "f64_projected_rank": nonsingular_rank,
            "rank_defect": rank_defect,
            "zero_in_convex_hull_feasible_f64": required_bool(summary, &["f64_nonsingular_active_convex_hull_zero", "feasible"]),
            "positive_lambda_projected_rank_f64": nonsingular_positive_lambda_rank,
            "currently_closes": attempt_closes
        },
        "padded_nonsingular_workaround": {
            "description": "Insert one missing facet into nonsingular six-facet active branches and keep nonsingular minimum-action rows with one zero beta coordinate.",
            "kept_row_count": padded_nonsingular_count,
            "f64_projected_rank": required_usize(summary, &["padded_extension_nonsingular_min_action_padded_once_projected_rank", "rank"]),
            "currently_closes": padded_nonsingular_count > 0
                && required_usize(summary, &["padded_extension_nonsingular_min_action_padded_once_projected_rank", "rank"]) == quotient_dimension
        },
        "current_feasible_section_comparison": {
            "description": "Current theorem route using feasible beta sections, including singular positive-beta rows.",
            "row_count": feasible_section_row_count,
            "f64_projected_rank": feasible_section_rank,
            "positive_lambda_projected_rank_f64": feasible_section_positive_lambda_rank,
            "zero_in_convex_hull_feasible_f64": required_bool(summary, &["feasible_section_convex_hull_zero", "feasible"]),
            "currently_closes_numerically": feasible_section_rank == quotient_dimension
        },
        "conclusion": {
            "claim": "The nonsingular positive-beta active branches do not currently close the first-order rank check in the f64 diagnostic.",
            "reason": format!("their projected D_sys matrix has numerical rank {nonsingular_rank} in quotient dimension {quotient_dimension}"),
            "not_claimed": "This does not show that no repaired smooth-only or nonsingular-only route can exist."
        }
    });

    if let Some(parent) = options.output_path.parent() {
        fs::create_dir_all(parent).expect("failed to create output directory");
    }
    fs::write(
        &options.output_path,
        serde_json::to_string_pretty(&output).expect("summary must serialize") + "\n",
    )
    .unwrap_or_else(|err| panic!("failed to write {}: {err}", options.output_path.display()));

    println!("HKO smooth-only rank defect");
    println!(
        "  f64 nonsingular projected rank: {} / {}",
        nonsingular_rank, quotient_dimension
    );
    println!("  f64 rank defect: {}", rank_defect);
    println!(
        "  feasible-section comparison rank: {} / {}",
        feasible_section_rank, quotient_dimension
    );
    println!("  wrote {}", options.output_path.display());
}

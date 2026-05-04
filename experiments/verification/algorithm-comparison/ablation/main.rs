//! Ablation study: compare HK2017 algorithm variants on a fixed dataset.
//!
//! Goal: Compare HK2017 pruning variants on a shared regression and random
//! dataset with one JSONL row per (polytope, variant).
//! Input Artifacts: None (generates all ablation fixtures internally).
//! Output Artifacts: experiments/verification/algorithm-comparison/ablation/ablation.jsonl
//!
//! Usage:
//! - `cargo run -p dev-algorithm-comparison --release --bin cmp-ablation -- --smoke`
//!   runs a small smoke subset and writes `ablation/ablation-smoke.jsonl`.
//! - `cargo run -p dev-algorithm-comparison --release --bin cmp-ablation`
//!   runs the full dataset and writes `ablation/ablation.jsonl`.
//!
//! A-axis variants: A0 (unpruned), A1 (vertex adjacency),
//! A2 (directed omega0), A3 (Reeb-flow feasibility).
//!
//! Convention: The library (`crates/symplectic/`) is stable. New variants are implemented as
//! self-contained code in this experiment. Library internals needed by the new variants
//! are copied here (marked with source references). If a variant is later promoted
//! to production, it enters the library then.
//!
//! KKT solver note: the copied `solve_kkt_svd_path` uses the old gap-ratio approach
//! (`SVD_GAP_THRESHOLD = 100.0`), not the library's current condition-number approach
//! (`SVD_CONDITION_TAU = 1e-3`). This is intentional: all variants use the same solver
//! for apples-to-apples comparison. Correctness is validated by agreement with A0.

mod fixtures;
mod kkt;
mod models;
mod variants;

use crate::fixtures::build_ablation_polytopes;
use crate::models::AblationEntry;
use crate::variants::VARIANTS;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use std::time::Instant;

#[derive(Debug, Clone, Copy)]
struct Args {
    smoke: bool,
}

fn print_usage() {
    eprintln!(
        r#"Usage: cmp-ablation [options]

Optional flags:
  --help, -h          Show this help message and exit.
  --smoke              Run a small smoke subset and write smoke output."#
    );
}

fn usage_error(message: String) -> ! {
    eprintln!("error: {message}\n");
    print_usage();
    std::process::exit(2);
}

fn parse_args() -> Args {
    let mut args = Args { smoke: false };
    for arg in std::env::args().skip(1) {
        match arg.as_str() {
            "--help" | "-h" => {
                print_usage();
                std::process::exit(0);
            }
            "--smoke" => args.smoke = true,
            other => usage_error(format!("unknown argument: {other}")),
        }
    }
    args
}

fn output_path(manifest_dir: &Path, smoke: bool) -> PathBuf {
    if smoke {
        manifest_dir.join("ablation/ablation-smoke.jsonl")
    } else {
        manifest_dir.join("ablation/ablation.jsonl")
    }
}

fn main() {
    let args = parse_args();
    let t0 = Instant::now();
    let output_path = output_path(std::path::Path::new(env!("CARGO_MANIFEST_DIR")), args.smoke);

    println!("Ablation study — A-axis (adjacency pruning)\n");
    println!("Variants: A0 (unpruned), A1 (vertex adj), A2 (directed ω₀), A3 (Reeb feasibility)");
    println!(
        "Seed: {}, h ∈ [{}, {}]\n",
        crate::models::SEED,
        crate::models::H_MIN,
        crate::models::H_MAX
    );

    let mut polytopes = build_ablation_polytopes();
    if args.smoke {
        polytopes.truncate(2);
    }
    let n_polytopes = polytopes.len();
    let n_entries = n_polytopes * VARIANTS.len();
    println!(
        "\nTotal: {n_polytopes} polytopes × {} variants = {n_entries} entries\n",
        VARIANTS.len()
    );

    let mut entries: Vec<AblationEntry> = Vec::with_capacity(n_entries);
    let mut n_disagreements = 0usize;
    let mut n_failures = 0usize;

    for fixture in &polytopes {
        let duals_raw: Vec<[f64; 4]> = fixture
            .polytope
            .dual_vertices_f64()
            .iter()
            .map(|a| [a[0], a[1], a[2], a[3]])
            .collect();
        let facet_count = fixture.polytope.facet_count();
        let mut capacities: Vec<(String, f64)> = Vec::new();

        for variant in VARIANTS {
            let t_start = Instant::now();
            let result = (variant.run)(&fixture.polytope);
            let time_ms = t_start.elapsed().as_secs_f64() * 1000.0;

            match result {
                None => {
                    eprintln!(
                        "  FAILURE: {} / {} returned None",
                        fixture.name, variant.name
                    );
                    n_failures += 1;
                }
                Some(result) => {
                    capacities.push((variant.name.to_string(), result.result.capacity));

                    if let Some(expected) = fixture.expected_capacity {
                        if (result.result.capacity - expected).abs() > 1e-5 {
                            eprintln!(
                                "  WRONG: {} / {}: got {:.8}, expected {:.8} (diff={:.2e})",
                                fixture.name,
                                variant.name,
                                result.result.capacity,
                                expected,
                                (result.result.capacity - expected).abs()
                            );
                            n_disagreements += 1;
                        }
                    }

                    entries.push(AblationEntry {
                        polytope_name: fixture.name.clone(),
                        variant: variant.name.to_string(),
                        group: fixture.group.clone(),
                        facet_count,
                        dual_vertices: duals_raw.clone(),
                        capacity: result.result.capacity,
                        capacity_uncertain: result.result.capacity_uncertain,
                        iterations: result.result.iterations,
                        time_ms,
                    });
                }
            }
        }

        for i in 0..capacities.len() {
            for j in (i + 1)..capacities.len() {
                let (ref name_i, c_i) = capacities[i];
                let (ref name_j, c_j) = capacities[j];
                if (c_i - c_j).abs() > 1e-5 {
                    eprintln!(
                        "  DISAGREE: {} {}={:.8} {}={:.8} (diff={:.2e})",
                        fixture.name,
                        name_i,
                        c_i,
                        name_j,
                        c_j,
                        (c_i - c_j).abs()
                    );
                    n_disagreements += 1;
                }
            }
        }
    }

    let file = File::create(&output_path)
        .unwrap_or_else(|err| panic!("failed to create {}: {err}", output_path.display()));
    let mut writer = BufWriter::new(file);
    for entry in &entries {
        serde_json::to_writer(&mut writer, entry).expect("failed to serialize entry");
        writeln!(writer).expect("failed to write newline");
    }
    writer.flush().expect("failed to flush output");

    let total_time = t0.elapsed().as_secs_f64();
    println!("Results:");
    println!("  Entries written:  {}", entries.len());
    println!("  Disagreements:    {n_disagreements}");
    println!("  Failures (None):  {n_failures}");
    println!("  Total time:       {total_time:.1}s");
    println!();
    println!("Output: {}", output_path.display());

    if n_disagreements > 0 || n_failures > 0 {
        eprintln!(
            "\nABLATION ISSUES FOUND: {n_disagreements} disagreements, {n_failures} failures"
        );
        std::process::exit(1);
    }

    println!("\nAll variants agree. Ready for Python analysis.");
}

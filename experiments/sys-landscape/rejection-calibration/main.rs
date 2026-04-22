//! Acceptance rate sweep: measure rejection sampling statistics across parameter configs.
//!
//! Goal: Measure rejection sampling acceptance rates across a grid of facet counts
//!   and height ranges, to characterize the efficiency of random polytope generation.
//! Input Artifacts: None (generates candidates from hardcoded parameter grid and seed).
//! Output Artifacts: experiments/sys-landscape/rejection-calibration/acceptance.jsonl (acceptance rates per config).

use exp_sys_landscape::experiment_path;
use symplectic::dataset::AcceptanceRow;
use symplectic::random::sample_random_polytope;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::time::Instant;

/// Hardcoded sweep grid.
const FACET_COUNTS: &[usize] = &[5, 6, 7, 8, 9, 10];
const HEIGHT_RANGES: &[(f64, f64)] = &[(0.5, 2.0), (0.1, 5.0), (0.8, 1.2)];

/// Run the acceptance sweep over the hardcoded parameter grid.
pub fn run_sweep(n_attempts: usize, seed: u64) -> Vec<AcceptanceRow> {
    let mut rows = Vec::new();

    for &f in FACET_COUNTS {
        for &(h_min, h_max) in HEIGHT_RANGES {
            let mut rng = ChaCha8Rng::seed_from_u64(seed);
            let mut n_accepted = 0usize;
            let mut total_accepted_us = 0u128;
            let mut total_rejected_us = 0u128;

            for _ in 0..n_attempts {
                let start = Instant::now();
                let accepted = sample_random_polytope(f, h_min, h_max, &mut rng).is_ok();
                let elapsed_us = start.elapsed().as_micros();

                if accepted {
                    n_accepted += 1;
                    total_accepted_us += elapsed_us;
                } else {
                    total_rejected_us += elapsed_us;
                }
            }

            let n_rejected = n_attempts - n_accepted;
            let avg_accepted_ms = if n_accepted > 0 {
                total_accepted_us as f64 / n_accepted as f64 / 1000.0 // μs → ms
            } else {
                0.0
            };
            let avg_rejected_ms = if n_rejected > 0 {
                total_rejected_us as f64 / n_rejected as f64 / 1000.0 // μs → ms
            } else {
                0.0
            };

            rows.push(AcceptanceRow {
                facet_count: f,
                h_min,
                h_max,
                n_total: n_attempts,
                n_accepted,
                acceptance_ratio: n_accepted as f64 / n_attempts as f64,
                avg_time_accepted_ms: avg_accepted_ms,
                avg_time_rejected_ms: avg_rejected_ms,
            });
        }
    }

    rows
}

fn main() {
    const N_ATTEMPTS: usize = 1000;
    const SEED: u64 = 42;

    let output_path = experiment_path("rejection-calibration", "acceptance.jsonl");

    println!("Running acceptance rate sweep...");
    println!("  n_attempts = {N_ATTEMPTS}");
    println!("  seed = {SEED}");
    println!();

    let rows = run_sweep(N_ATTEMPTS, SEED);

    let file = File::create(&output_path).expect("failed to create output file");
    let mut writer = BufWriter::new(file);

    for row in &rows {
        serde_json::to_writer(&mut writer, row).expect("failed to serialize row");
        writeln!(writer).expect("failed to write newline");
        println!(
            "F={:2} h∈[{:.1},{:.1}]: acceptance={:.1}% (accepted={:4}/{:4}, avg_time={:.2}ms)",
            row.facet_count,
            row.h_min,
            row.h_max,
            row.acceptance_ratio * 100.0,
            row.n_accepted,
            row.n_total,
            row.avg_time_accepted_ms
        );
    }

    writer.flush().expect("failed to flush output");
    println!();
    println!("Wrote {} rows to {}", rows.len(), output_path.display());
}

#[cfg(test)]
#[path = "acceptance_sweep_test.rs"]
mod acceptance_sweep_test;

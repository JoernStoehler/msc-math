/// Acceptance rate sweep: measure rejection sampling statistics across parameter configs.
use crate::dataset::AcceptanceRow;
use crate::random::sample_random_polytope;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
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
                let ok = sample_random_polytope(f, h_min, h_max, &mut rng).is_ok();
                let elapsed_us = start.elapsed().as_micros();

                if ok {
                    n_accepted += 1;
                    total_accepted_us += elapsed_us;
                } else {
                    total_rejected_us += elapsed_us;
                }
            }

            let n_rejected = n_attempts - n_accepted;
            let avg_accepted_ms = if n_accepted > 0 {
                total_accepted_us as f64 / n_accepted as f64 / 1000.0
            } else {
                0.0
            };
            let avg_rejected_ms = if n_rejected > 0 {
                total_rejected_us as f64 / n_rejected as f64 / 1000.0
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

#[cfg(test)]
#[path = "acceptance_sweep_test.rs"]
mod acceptance_sweep_test;

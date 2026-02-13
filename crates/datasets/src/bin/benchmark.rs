/// Benchmark EHZ capacity computation across facet counts.
///
/// Generates random polytopes at each facet count and times
/// ehz_capacity_pruned in release mode. Outputs CSV to stdout.
///
/// Usage: benchmark
use datasets::random::generate_random_polytopes;
use hk2017::ehz_capacity_pruned;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use std::time::Instant;

const SEED: u64 = 42;
const H_MIN: f64 = 0.5;
const H_MAX: f64 = 2.0;

/// (facet_count, n_samples)
const PLAN: &[(usize, usize)] = &[
    (5, 20),
    (6, 20),
    (7, 15),
    (8, 10),
    (9, 5),
    (10, 3),
    (11, 2),
    (12, 1),
];

/// Per-sample timeout in seconds. Skip remaining samples for this F
/// and all higher F values if any sample exceeds this.
const TIMEOUT_SECS: f64 = 180.0;

fn main() {
    let mut rng = ChaCha8Rng::seed_from_u64(SEED);
    let start = Instant::now();

    println!("facets,sample,time_ms,capacity,iterations");

    let mut hit_timeout = false;

    for &(f, n) in PLAN {
        if hit_timeout {
            eprintln!("Skipping F={f} (previous sample exceeded timeout)");
            continue;
        }

        eprintln!("F={f}: generating {n} random polytopes...");
        let polytopes = generate_random_polytopes(n, f, H_MIN, H_MAX, &mut rng);

        for (i, p) in polytopes.iter().enumerate() {
            let t0 = Instant::now();
            let result = ehz_capacity_pruned(p);
            let elapsed_ms = t0.elapsed().as_secs_f64() * 1000.0;

            if let Some(r) = result {
                println!("{f},{i},{elapsed_ms:.3},{:.6},{}", r.capacity, r.iterations);
            } else {
                println!("{f},{i},{elapsed_ms:.3},NA,0");
            }

            if t0.elapsed().as_secs_f64() > TIMEOUT_SECS {
                eprintln!(
                    "  Sample {i} took {:.1}s > {TIMEOUT_SECS}s timeout, stopping",
                    t0.elapsed().as_secs_f64()
                );
                hit_timeout = true;
                break;
            }
        }

        eprintln!(
            "  F={f} done ({n} samples, total elapsed {:.1}s)",
            start.elapsed().as_secs_f64()
        );
    }

    eprintln!("Benchmark complete in {:.1}s", start.elapsed().as_secs_f64());
}

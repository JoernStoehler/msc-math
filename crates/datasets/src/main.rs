use datasets::acceptance_sweep;
use datasets::dataset::PolytopeRow;
use datasets::known_polytopes;
use datasets::random::generate_random_polytopes;
use geom::volume::volume;
use hk2017::ehz_capacity_pruned;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use std::time::Instant;

// ---- Hardcoded parameters (KISS: refactor into CLI args later) ----

const N_RANDOM: usize = 50;
const RANDOM_FACET_COUNT: usize = 8;
const RANDOM_H_MIN: f64 = 0.5;
const RANDOM_H_MAX: f64 = 2.0;
const SWEEP_N_ATTEMPTS: usize = 1000;
const SEED: u64 = 42;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: datasets <subcommand> <output_path>");
        eprintln!("  subcommands: dataset, sweep");
        std::process::exit(1);
    }

    let subcommand = &args[1];
    let output_path = PathBuf::from(&args[2]);

    match subcommand.as_str() {
        "dataset" => cmd_dataset(&output_path),
        "sweep" => cmd_sweep(&output_path),
        other => {
            eprintln!("Unknown subcommand: {other}");
            eprintln!("  subcommands: dataset, sweep");
            std::process::exit(1);
        }
    }
}

fn cmd_dataset(output: &PathBuf) {
    let file = File::create(output).expect("cannot create output file");
    let mut writer = BufWriter::new(file);

    // 1. Known polytopes
    let known = known_polytopes::all_known();
    eprintln!("Writing {} known polytopes...", known.len());
    const MAX_FACETS_BRUTEFORCE: usize = 10;
    for kp in &known {
        let start_vol = Instant::now();
        let vol = volume(&kp.polytope).expect("volume computation failed");
        let vol_time = start_vol.elapsed();

        let (cap, cap_time) = if kp.polytope.facet_count() <= MAX_FACETS_BRUTEFORCE {
            let start_cap = Instant::now();
            let cap_result =
                ehz_capacity_pruned(&kp.polytope).expect("capacity computation failed");
            (cap_result.capacity, start_cap.elapsed())
        } else {
            eprintln!(
                "  Skipping capacity for {} ({} facets > {MAX_FACETS_BRUTEFORCE})",
                kp.name,
                kp.polytope.facet_count()
            );
            (f64::NAN, std::time::Duration::ZERO)
        };

        let row = PolytopeRow::from_polytope(
            &kp.polytope,
            kp.name.to_string(),
            vol,
            cap,
            vol_time.as_secs_f64() * 1000.0,
            cap_time.as_secs_f64() * 1000.0,
            0.0, // creation time negligible for hardcoded polytopes
        );
        let line = serde_json::to_string(&row).expect("serialize row");
        writeln!(writer, "{line}").expect("write line");
    }

    // 2. Random polytopes
    eprintln!(
        "Generating {} random polytopes (F={}, h in [{}, {}])...",
        N_RANDOM, RANDOM_FACET_COUNT, RANDOM_H_MIN, RANDOM_H_MAX
    );
    let mut rng = ChaCha8Rng::seed_from_u64(SEED);
    let mut count = 0;
    while count < N_RANDOM {
        let start = Instant::now();
        let polytopes =
            generate_random_polytopes(1, RANDOM_FACET_COUNT, RANDOM_H_MIN, RANDOM_H_MAX, &mut rng);
        let creation_time = start.elapsed();

        for p in &polytopes {
            let start_vol = Instant::now();
            let vol = volume(p).expect("volume computation failed");
            let vol_time = start_vol.elapsed();

            let start_cap = Instant::now();
            let cap_result = ehz_capacity_pruned(p).expect("capacity computation failed");
            let cap_time = start_cap.elapsed();
            let cap = cap_result.capacity;

            let row = PolytopeRow::from_polytope(
                p,
                "random".to_string(),
                vol,
                cap,
                vol_time.as_secs_f64() * 1000.0,
                cap_time.as_secs_f64() * 1000.0,
                creation_time.as_secs_f64() * 1000.0,
            );
            let line = serde_json::to_string(&row).expect("serialize row");
            writeln!(writer, "{line}").expect("write line");
        }

        count += 1;
        if count % 10 == 0 {
            eprintln!("  {count}/{N_RANDOM}");
        }
    }

    writer.flush().expect("flush output");
    eprintln!(
        "Done. Wrote {} rows to {}",
        known.len() + N_RANDOM,
        output.display()
    );
}

fn cmd_sweep(output: &PathBuf) {
    eprintln!(
        "Running acceptance sweep ({} attempts per config)...",
        SWEEP_N_ATTEMPTS
    );
    let rows = acceptance_sweep::run_sweep(SWEEP_N_ATTEMPTS, SEED);

    let file = File::create(output).expect("cannot create output file");
    let mut writer = BufWriter::new(file);

    for row in &rows {
        let line = serde_json::to_string(&row).expect("serialize row");
        writeln!(writer, "{line}").expect("write line");
    }

    writer.flush().expect("flush output");
    eprintln!("Done. Wrote {} rows to {}", rows.len(), output.display());
}

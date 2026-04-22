//! Dataset producer: random systolic-ratio sample over random 4D polytopes.
//!
//! Goal: Compute systolic ratios for random 4D polytopes across facet counts F=5..12,
//!   to probe whether random generic polytopes approach the Viterbo threshold.
//! Input Artifacts: experiments/sys-landscape/datascience/produce/shared-cache.jsonl
//! Output Artifacts: experiments/sys-landscape/datascience/produce/random.jsonl,
//!         experiments/sys-landscape/datascience/produce/shared-cache.jsonl
//!
//! Architecture:
//! 1. Bare `cargo run -p exp-sys-landscape --release --bin sys-dataset-random`
//!    is a smoke/default run: it writes temp output + temp cache under `/tmp`.
//! 2. Canonical refreshes pass explicit repo-owned paths, e.g.
//!    `--out experiments/sys-landscape/datascience/produce/random.jsonl`
//!    and `--cache experiments/sys-landscape/datascience/produce/shared-cache.jsonl`.
//! 3. Polytopes are generated via `generate_polytope` (blake3 per-attempt seeding)
//!    and cached in the sys-landscape family cache. Re-runs skip generation + capacity.
//! 4. Canonical runs write to `datascience/produce/random.jsonl`.
//!
//! Dataset design:
//! - Random polytopes with facet counts F=5..12
//! - Height range h in [0.8, 1.2]
//! - Default root capacity wrapper (`symplectic::ehz_capacity`), which
//!   auto-routes Lagrangian products to billiard and other inputs to pruned HK2017
//!
//! CLI (all optional):
//! - `--seed <u64>`            RNG seed                               (default: 42)
//! - `--samples-per-f <usize>` samples for each included facet count  (default: plan value)
//! - `--max-f <usize>`         cap facet counts included in the run   (default: 12)
//! - `--out <path>`            output JSONL path                      (default: untracked temp)
//! - `--cache <path>`          cache JSONL path                       (default: untracked temp)

use exp_sys_landscape::{orbit_scalars_from_result, smoke_output_path};
use num_rational::BigRational;
use serde::Serialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use std::time::Instant;
use symplectic::database::{load_many, save, DualVerticesKey, PolytopeRecord, SigmaAction, Source};
use symplectic::ehz_capacity;
use symplectic::geom::volume::volume;
use symplectic::random::generate_polytope;

const SEED: u64 = 42;
const H_MIN: f64 = 0.8;
const H_MAX: f64 = 1.2;

/// (facet_count, n_samples)
const RANDOM_PLAN: &[(usize, usize)] = &[
    (5, 10),
    (6, 10),
    (7, 10),
    (8, 10),
    (9, 10),
    (10, 10),
    (11, 5),
    (12, 5),
];

struct Args {
    seed: u64,
    samples_per_f: Option<usize>,
    max_f: usize,
    out: PathBuf,
    cache: PathBuf,
}

fn default_smoke_cache_path() -> PathBuf {
    smoke_output_path("sys-dataset-random", "smoke-cache.jsonl")
}

fn default_smoke_output_path() -> PathBuf {
    smoke_output_path("sys-dataset-random", "smoke-random.jsonl")
}

fn parse_args() -> Args {
    parse_args_from(std::env::args())
}

fn parse_args_from(
    argv: impl IntoIterator<Item = impl Into<String>>,
) -> Args {
    let argv: Vec<String> = argv.into_iter().map(Into::into).collect();

    let mut seed = SEED;
    let mut samples_per_f = None;
    let mut max_f = RANDOM_PLAN
        .iter()
        .map(|(f, _)| *f)
        .max()
        .expect("random plan should be non-empty");
    let mut out = None;
    let mut cache = None;

    let mut i = 1;
    while i < argv.len() {
        let arg = argv[i].as_str();
        let need_value = |flag: &str| -> &str {
            argv.get(i + 1)
                .map(|s| s.as_str())
                .unwrap_or_else(|| panic!("{flag} requires a value"))
        };
        match arg {
            "--seed" => {
                seed = need_value("--seed")
                    .parse()
                    .expect("--seed must be a u64");
                i += 2;
            }
            "--samples-per-f" => {
                samples_per_f = Some(
                    need_value("--samples-per-f")
                        .parse()
                        .expect("--samples-per-f must be a non-negative integer"),
                );
                i += 2;
            }
            "--max-f" => {
                max_f = need_value("--max-f")
                    .parse()
                    .expect("--max-f must be a positive integer");
                assert!(max_f >= 5, "--max-f must be at least 5");
                i += 2;
            }
            "--out" => {
                out = Some(PathBuf::from(need_value("--out")));
                i += 2;
            }
            "--cache" => {
                cache = Some(PathBuf::from(need_value("--cache")));
                i += 2;
            }
            other => panic!("unknown argument: {other}"),
        }
    }

    Args {
        seed,
        samples_per_f,
        max_f,
        out: out.unwrap_or_else(default_smoke_output_path),
        cache: cache.unwrap_or_else(default_smoke_cache_path),
    }
}

fn sweep_plan(samples_per_f: Option<usize>, max_f: usize) -> Vec<(usize, usize)> {
    RANDOM_PLAN
        .iter()
        .copied()
        .filter(|(facet_count, _)| *facet_count <= max_f)
        .map(|(facet_count, default_samples)| {
            (facet_count, samples_per_f.unwrap_or(default_samples))
        })
        .collect()
}

#[derive(Debug, Serialize)]
struct RandomSweepRow {
    name: String,
    facet_count: usize,
    dual_vertices: Vec<[f64; 4]>,
    dual_vertices_rational: Vec<[String; 4]>,
    vertices_rational: Vec<[String; 4]>,
    h_min: f64,
    h_max: f64,
    volume: f64,
    capacity: f64,
    sys: f64,
    iterations: u64,
    time_volume_ms: f64,
    time_capacity_ms: f64,
}

/// Find a cached record by Source metadata. Linear scan, negligible for <1000 records.
fn find_by_source<'a>(
    db: &'a HashMap<DualVerticesKey, PolytopeRecord>,
    source: &Source,
) -> Option<(&'a DualVerticesKey, &'a PolytopeRecord)> {
    db.iter().find(|(_, r)| r.source.as_ref() == Some(source))
}

fn rational_vec4_to_strings(data: &[[BigRational; 4]]) -> Vec<[String; 4]> {
    data.iter()
        .map(|row| std::array::from_fn(|i| format!("{}/{}", row[i].numer(), row[i].denom())))
        .collect()
}

fn main() {
    let args = parse_args();
    let t0 = Instant::now();
    let plan = sweep_plan(args.samples_per_f, args.max_f);
    assert!(
        !plan.is_empty(),
        "no facet counts selected; increase --max-f to at least 5"
    );

    if let Some(parent) = args.cache.parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent).expect("create cache directory");
        }
    }
    if let Some(parent) = args.out.parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent).expect("create output directory");
        }
    }

    let mut db = load_many(&[args.cache.as_path()])
        .expect("failed to load sys-landscape family cache");
    println!("Loaded family cache: {} entries\n", db.len());

    let file = File::create(&args.out).expect("failed to create output file");
    let mut writer = BufWriter::new(file);

    let mut total = 0usize;
    let mut cache_hits = 0usize;
    let mut attempt: u64 = 0;

    for (facet_count, n_samples) in plan {
        println!("F={facet_count:2}: generating {n_samples:2} samples (h in [{H_MIN}, {H_MAX}])");
        let mut accepted = 0usize;

        while accepted < n_samples {
            let source = Source::Random {
                master_seed: args.seed,
                attempt,
                facet_count_target: facet_count,
                h_min: H_MIN,
                h_max: H_MAX,
            };

            // Try Source-based lookup first
            if let Some(key) = find_by_source(&db, &source).map(|(key, _)| key.clone()) {
                let record = db
                    .get_mut(&key)
                    .expect("source lookup key should remain valid in the cache");
                // Cache hit: reconstruct polytope from rational data (skip vertex enumeration)
                let p = record
                    .to_polytope()
                    .expect("failed to reconstruct polytope from database");
                if record.orbit_scalars.is_none() {
                    let ehz = ehz_capacity(&p).expect("capacity recomputation failed on cache hit");
                    record.orbit_scalars = Some(orbit_scalars_from_result(&ehz));
                }
                let vol = record.volume.expect("cached record missing volume");
                let cap = record.capacity.expect("cached record missing capacity");
                let sys = cap * cap / (2.0 * vol);

                let row = RandomSweepRow {
                    name: format!("random_F{facet_count}_{accepted}"),
                    facet_count,
                    dual_vertices: p
                        .dual_vertices_f64()
                        .iter()
                        .map(|a| [a[0], a[1], a[2], a[3]])
                        .collect(),
                    dual_vertices_rational: rational_vec4_to_strings(
                        &record.dual_vertices_rational,
                    ),
                    vertices_rational: rational_vec4_to_strings(&record.vertices_rational),
                    h_min: H_MIN,
                    h_max: H_MAX,
                    volume: vol,
                    capacity: cap,
                    sys,
                    iterations: 0, // not stored in database
                    time_volume_ms: 0.0,
                    time_capacity_ms: 0.0,
                };

                let line = serde_json::to_string(&row).expect("serialize row");
                writeln!(writer, "{line}").expect("write line");
                total += 1;
                accepted += 1;
                cache_hits += 1;
                attempt += 1;
                continue;
            }

            // Cache miss: generate polytope
            let p = match generate_polytope(facet_count, H_MIN, H_MAX, args.seed, attempt) {
                Ok(p) => p,
                Err(_) => {
                    // Rejection: this (seed, attempt) doesn't produce a valid polytope
                    attempt += 1;
                    continue;
                }
            };

            let start_vol = Instant::now();
            let vol = volume(&p);
            let time_volume_ms = start_vol.elapsed().as_secs_f64() * 1000.0;

            let start_cap = Instant::now();
            let ehz = ehz_capacity(&p).expect("capacity computation failed");
            let time_capacity_ms = start_cap.elapsed().as_secs_f64() * 1000.0;

            let cap = ehz.capacity();
            let sys = cap * cap / (2.0 * vol);

            // Insert into database
            let mut record = PolytopeRecord::from_polytope(&p);
            record.source = Some(source);
            record = record.with_computed_fields(vol, 0.0, cap, 0.0);
            record = record.with_sigmas(
                vec![SigmaAction {
                    perm: ehz.best_sigma().to_vec(),
                    action: cap,
                }],
                0.0, // gap_cutoff: only storing the best sigma
            );
            record = record.with_orbit_scalars(orbit_scalars_from_result(&ehz));
            db.insert(record.key(), record);

            let row = RandomSweepRow {
                name: format!("random_F{facet_count}_{accepted}"),
                facet_count,
                dual_vertices: p
                    .dual_vertices_f64()
                    .iter()
                    .map(|a| [a[0], a[1], a[2], a[3]])
                    .collect(),
                dual_vertices_rational: rational_vec4_to_strings(p.dual_vertices()),
                vertices_rational: rational_vec4_to_strings(p.vertices()),
                h_min: H_MIN,
                h_max: H_MAX,
                volume: vol,
                capacity: cap,
                sys,
                iterations: ehz.iterations,
                time_volume_ms,
                time_capacity_ms,
            };

            let line = serde_json::to_string(&row).expect("serialize row");
            writeln!(writer, "{line}").expect("write line");
            total += 1;
            accepted += 1;
            attempt += 1;
        }
    }

    writer.flush().expect("flush output");
    save(&args.cache, &db).expect("failed to save sys-landscape family cache");

    println!("\nWrote {total} entries to {}", args.out.display());
    println!("Cache: {} entries (saved to {})", db.len(), args.cache.display());
    println!("Cache hits: {cache_hits}/{total}");
    println!("Total time: {:.1}s", t0.elapsed().as_secs_f64());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_args_defaults_to_temp_paths() {
        let args = parse_args_from(["sys-dataset-random"]);
        assert_eq!(args.seed, SEED);
        assert_eq!(args.samples_per_f, None);
        assert!(
            args.out.to_string_lossy().contains("sys-dataset-random"),
            "default output path should use smoke temp dir: {:?}",
            args.out
        );
        assert!(
            args.cache.to_string_lossy().contains("sys-dataset-random"),
            "default cache path should use smoke temp dir: {:?}",
            args.cache
        );
    }

    #[test]
    fn parse_args_overrides_paths_and_limits() {
        let args = parse_args_from([
            "sys-dataset-random",
            "--seed",
            "7",
            "--samples-per-f",
            "2",
            "--max-f",
            "6",
            "--out",
            "tmp/out.jsonl",
            "--cache",
            "tmp/cache.jsonl",
        ]);

        assert_eq!(args.seed, 7);
        assert_eq!(args.samples_per_f, Some(2));
        assert_eq!(args.max_f, 6);
        assert_eq!(args.out, PathBuf::from("tmp/out.jsonl"));
        assert_eq!(args.cache, PathBuf::from("tmp/cache.jsonl"));
    }

    #[test]
    fn sweep_plan_respects_override_and_limit() {
        let plan = sweep_plan(Some(1), 6);
        assert_eq!(plan, vec![(5, 1), (6, 1)]);
    }
}

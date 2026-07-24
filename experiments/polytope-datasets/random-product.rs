//! Dataset producer: random Lagrangian-product sample over polygon pairs.
//!
//! Goal: Sample random Lagrangian products across polygon-pair buckets and
//! record their systolic ratios.
//! Input Artifacts: experiments/polytope-datasets/shared-cache.jsonl
//! Output Artifacts: experiments/polytope-datasets/random-product.jsonl,
//!         experiments/polytope-datasets/shared-cache.jsonl
//!
//! Architecture:
//! 1. Bare `cargo run -p exp-polytope-datasets --release --bin sys-dataset-random-product`
//!    is a smoke/default run: it writes temp output + temp cache under `/tmp`.
//! 2. Canonical refreshes pass explicit repo-owned paths, e.g.
//!    `--out experiments/polytope-datasets/random-product.jsonl`
//!    and `--cache experiments/polytope-datasets/shared-cache.jsonl`.
//! 3. Polytopes are cached in the sys-landscape family cache. Re-runs skip capacity.
//! 4. Canonical runs write to `experiments/polytope-datasets/random-product.jsonl`.
//!
//! Dataset design:
//! - Random 2D polygons with k, m in {3,4,5,6}
//! - Pairs with 3 <= k <= m <= 6 (10 buckets)
//! - 10 samples per bucket
//! - Height range h in [0.8, 1.2]
//! - Explicit billiard reporting: this dataset writes billiard-native
//!   `iterations` and `bounces`, so the auto-routed capacity helper
//!   wrapper would drop required output fields.
//!
//! Note: Uses shared RNG (no blake3 per-attempt seeding) because there is no
//! source-tagged single-attempt generator for Lagrangian products. Database lookup is
//! key-based (BigRational dual vertices), not Source-based.
//!
//! CLI (all optional):
//! - `--seed <u64>`                 RNG seed                                (default: 42)
//! - `--samples-per-bucket <usize>` samples for each included pair bucket   (default: 10)
//! - `--max-sides <usize>`          cap polygon sizes included in the run   (default: 6)
//! - `--h-min <f64>`                minimum support height                  (default: 0.8)
//! - `--h-max <f64>`                maximum support height                  (default: 1.2)
//! - `--out <path>`                 output JSONL path                       (default: untracked temp)
//! - `--cache <path>`               cache JSONL path                        (default: untracked temp)

use exp_sys_landscape::{orbit_scalars_from_result, smoke_output_path};
use num_rational::BigRational;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
mod rows;
use exp_sys_landscape::capacity_billiard;
use exp_sys_landscape::exact_volume_from_incidence_as_f64;
use exp_sys_landscape::SysLandscapePolytopeCache;
use rows::RandomProductRow;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use std::time::Instant;
use symplectic::algorithms::billiard::bounce_count_from_sigma_for_facets;
use symplectic::classify_facets_from_dual_vertices;
use symplectic::database::{load_many, save, DualVerticesKey, PolytopeRecord, SigmaAction, Source};
use symplectic::geom::polygon::random_polygon_2d;

const SEED: u64 = 42;
const H_MIN: f64 = 0.8;
const H_MAX: f64 = 1.2;
const SAMPLES_PER_BUCKET: usize = 10;

const PAIRS: &[(usize, usize)] = &[
    (3, 3),
    (3, 4),
    (3, 5),
    (3, 6),
    (4, 4),
    (4, 5),
    (4, 6),
    (5, 5),
    (5, 6),
    (6, 6),
];

fn bounce_count(polytope: &SysLandscapePolytopeCache, sigma: &[usize]) -> Option<usize> {
    let classification = classify_facets_from_dual_vertices(&polytope.dual_vertices_f64).ok()?;
    bounce_count_from_sigma_for_facets(&classification.q_indices, &classification.p_indices, sigma)
}

struct Args {
    seed: u64,
    samples_per_bucket: usize,
    max_sides: usize,
    h_min: f64,
    h_max: f64,
    out: PathBuf,
    cache: PathBuf,
}

fn default_smoke_cache_path() -> PathBuf {
    smoke_output_path("sys-dataset-random-product", "smoke-cache.jsonl")
}

fn default_smoke_output_path() -> PathBuf {
    smoke_output_path("sys-dataset-random-product", "smoke-random-product.jsonl")
}

fn parse_args() -> Args {
    parse_args_from(std::env::args())
}

fn parse_args_from(argv: impl IntoIterator<Item = impl Into<String>>) -> Args {
    let argv: Vec<String> = argv.into_iter().map(Into::into).collect();

    let mut seed = SEED;
    let mut samples_per_bucket = SAMPLES_PER_BUCKET;
    let mut max_sides = PAIRS
        .iter()
        .map(|(_, m)| *m)
        .max()
        .expect("pair list non-empty");
    let mut h_min = H_MIN;
    let mut h_max = H_MAX;
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
                seed = need_value("--seed").parse().expect("--seed must be a u64");
                i += 2;
            }
            "--samples-per-bucket" => {
                samples_per_bucket = need_value("--samples-per-bucket")
                    .parse()
                    .expect("--samples-per-bucket must be a non-negative integer");
                i += 2;
            }
            "--max-sides" => {
                max_sides = need_value("--max-sides")
                    .parse()
                    .expect("--max-sides must be a positive integer");
                assert!(max_sides >= 3, "--max-sides must be at least 3");
                i += 2;
            }
            "--h-min" => {
                h_min = need_value("--h-min")
                    .parse()
                    .expect("--h-min must be a finite f64");
                i += 2;
            }
            "--h-max" => {
                h_max = need_value("--h-max")
                    .parse()
                    .expect("--h-max must be a finite f64");
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
        samples_per_bucket,
        max_sides,
        h_min,
        h_max,
        out: out.unwrap_or_else(default_smoke_output_path),
        cache: cache.unwrap_or_else(default_smoke_cache_path),
    }
}

fn included_pairs(max_sides: usize) -> Vec<(usize, usize)> {
    PAIRS
        .iter()
        .copied()
        .filter(|(k, m)| *k <= max_sides && *m <= max_sides)
        .collect()
}

fn f64_dual_vertices(polytope: &SysLandscapePolytopeCache) -> Vec<[f64; 4]> {
    polytope
        .dual_vertices_f64
        .iter()
        .map(|a| [a[0], a[1], a[2], a[3]])
        .collect()
}

fn rational_vec4_to_strings(data: &[[BigRational; 4]]) -> Vec<[String; 4]> {
    data.iter()
        .map(|row| std::array::from_fn(|i| format!("{}/{}", row[i].numer(), row[i].denom())))
        .collect()
}

fn dual_vertices_rational_strings(polytope: &SysLandscapePolytopeCache) -> Vec<[String; 4]> {
    rational_vec4_to_strings(&polytope.dual_vertices)
}

fn vertices_rational_strings(polytope: &SysLandscapePolytopeCache) -> Vec<[String; 4]> {
    rational_vec4_to_strings(&polytope.vertices)
}

fn main() {
    let args = parse_args();
    assert!(
        args.h_min.is_finite()
            && args.h_max.is_finite()
            && 0.0 < args.h_min
            && args.h_min < args.h_max,
        "height range must satisfy finite 0 < h_min < h_max, got h_min={}, h_max={}",
        args.h_min,
        args.h_max
    );
    let t0 = Instant::now();
    let mut rng = ChaCha8Rng::seed_from_u64(args.seed);
    let pairs = included_pairs(args.max_sides);
    assert!(
        !pairs.is_empty(),
        "no polygon pair buckets selected; increase --max-sides to at least 3"
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

    let mut db: HashMap<DualVerticesKey, PolytopeRecord> =
        load_many(&[args.cache.as_path()]).expect("failed to load sys-landscape family cache");
    println!("Loaded family cache: {} entries\n", db.len());

    let file = File::create(&args.out).expect("failed to create output file");
    let mut writer = BufWriter::new(file);

    let mut total = 0usize;
    let mut cache_hits = 0usize;
    let mut attempt: u64 = 0;

    for (k, m) in pairs {
        println!("Bucket ({k},{m}) with {} samples", args.samples_per_bucket);

        let mut accepted = 0usize;
        while accepted < args.samples_per_bucket {
            let attempt_for_row = attempt;
            attempt += 1;
            // Generate polygon pair using shared RNG (advances RNG regardless of acceptance)
            let (qn, qh) = random_polygon_2d(k, args.h_min, args.h_max, &mut rng);
            let (pn, ph) = random_polygon_2d(m, args.h_min, args.h_max, &mut rng);

            let polytope =
                match SysLandscapePolytopeCache::from_lagrangian_product(&qn, &qh, &pn, &ph) {
                    Some(p) => p,
                    None => continue,
                };

            let key: DualVerticesKey = polytope.dual_vertices.to_vec();

            // Key-based lookup: check if this exact polytope is already cached
            if let Some(record) = db.get_mut(&key) {
                // Backfill source for records that predate source tracking
                if record.source.is_none() {
                    record.source = Some(Source::LagrangianProduct {
                        n1: k,
                        n2: m,
                        circumradius_q: 0.0,
                        circumradius_p: 0.0,
                        rotation_p_rad: 0.0,
                    });
                }
                if record.orbit_scalars.is_none() {
                    let result = capacity_billiard(
                        &polytope.dual_vertices_f64,
                        &polytope.dual_vertices,
                        &polytope.facet_intersection_is_nonempty,
                        &polytope.omega_signs,
                    )
                    .expect("billiard should accept cached Lagrangian product");
                    record.orbit_scalars = Some(orbit_scalars_from_result(&result));
                }
                if let (Some(vol), Some(cap)) = (record.volume, record.capacity) {
                    let sys = cap * cap / (2.0 * vol);

                    let row = RandomProductRow {
                        name: format!("random_{k}x{m}_{accepted}"),
                        k,
                        m,
                        facet_count: k + m,
                        seed: Some(args.seed),
                        attempt: Some(attempt_for_row),
                        dual_vertices: f64_dual_vertices(&polytope),
                        dual_vertices_rational: dual_vertices_rational_strings(&polytope),
                        vertices_rational: vertices_rational_strings(&polytope),
                        h_min: args.h_min,
                        h_max: args.h_max,
                        volume: vol,
                        capacity: cap,
                        sys,
                        iterations: 0,
                        bounces: 0,
                        time_volume_ms: 0.0,
                        time_capacity_ms: 0.0,
                    };

                    let line = serde_json::to_string(&row).expect("serialize row");
                    writeln!(writer, "{line}").expect("write line");
                    accepted += 1;
                    total += 1;
                    cache_hits += 1;
                    continue;
                }
            }

            // Cache miss: compute the specialized billiard result because this
            // dataset records billiard-native iterations and bounce counts.
            let start_vol = Instant::now();
            let vol = exact_volume_from_incidence_as_f64(
                &polytope.vertices,
                &polytope.vertex_facet_incidence,
            );
            let time_volume_ms = start_vol.elapsed().as_secs_f64() * 1000.0;

            let start_cap = Instant::now();
            let result = capacity_billiard(
                &polytope.dual_vertices_f64,
                &polytope.dual_vertices,
                &polytope.facet_intersection_is_nonempty,
                &polytope.omega_signs,
            )
            .expect("billiard should accept Lagrangian product");
            let time_capacity_ms = start_cap.elapsed().as_secs_f64() * 1000.0;

            let cap = result.min_action;
            let sys = cap * cap / (2.0 * vol);
            let Some(bounces) = bounce_count(&polytope, result.best_sigma()) else {
                continue;
            };

            // Insert into database
            let mut record = polytope.to_record();
            record.source = Some(Source::LagrangianProduct {
                n1: k,
                n2: m,
                // Random polygon pair — no fixed circumradius or rotation.
                circumradius_q: 0.0,
                circumradius_p: 0.0,
                rotation_p_rad: 0.0,
            });
            record = record.with_computed_fields(vol, 0.0, cap, 0.0);
            record = record.with_sigmas(
                vec![SigmaAction {
                    perm: result.best_sigma().to_vec(),
                    action: cap,
                }],
                0.0,
            );
            record = record.with_orbit_scalars(orbit_scalars_from_result(&result));
            db.insert(key, record);

            let row = RandomProductRow {
                name: format!("random_{k}x{m}_{accepted}"),
                k,
                m,
                facet_count: k + m,
                seed: Some(args.seed),
                attempt: Some(attempt_for_row),
                dual_vertices: f64_dual_vertices(&polytope),
                dual_vertices_rational: dual_vertices_rational_strings(&polytope),
                vertices_rational: vertices_rational_strings(&polytope),
                h_min: args.h_min,
                h_max: args.h_max,
                volume: vol,
                capacity: cap,
                sys,
                iterations: result.iterations,
                bounces,
                time_volume_ms,
                time_capacity_ms,
            };

            let line = serde_json::to_string(&row).expect("serialize row");
            writeln!(writer, "{line}").expect("write line");
            accepted += 1;
            total += 1;
        }
    }

    writer.flush().expect("flush output");
    save(&args.cache, &db).expect("failed to save sys-landscape family cache");

    println!("\nWrote {total} entries to {}", args.out.display());
    println!(
        "Cache: {} entries (saved to {})",
        db.len(),
        args.cache.display()
    );
    println!("Cache hits: {cache_hits}/{total}");
    println!("Total time: {:.1}s", t0.elapsed().as_secs_f64());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_args_defaults_to_temp_paths() {
        let args = parse_args_from(["sys-dataset-random-product"]);
        assert_eq!(args.seed, SEED);
        assert_eq!(args.samples_per_bucket, SAMPLES_PER_BUCKET);
        assert_eq!(args.h_min, H_MIN);
        assert_eq!(args.h_max, H_MAX);
        assert!(
            args.out
                .to_string_lossy()
                .contains("sys-dataset-random-product"),
            "default output path should use smoke temp dir: {:?}",
            args.out
        );
        assert!(
            args.cache
                .to_string_lossy()
                .contains("sys-dataset-random-product"),
            "default cache path should use smoke temp dir: {:?}",
            args.cache
        );
    }

    #[test]
    fn parse_args_overrides_paths_and_limits() {
        let args = parse_args_from([
            "sys-dataset-random-product",
            "--seed",
            "11",
            "--samples-per-bucket",
            "2",
            "--max-sides",
            "4",
            "--h-min",
            "0.6",
            "--h-max",
            "1.8",
            "--out",
            "tmp/out.jsonl",
            "--cache",
            "tmp/cache.jsonl",
        ]);

        assert_eq!(args.seed, 11);
        assert_eq!(args.samples_per_bucket, 2);
        assert_eq!(args.max_sides, 4);
        assert_eq!(args.h_min, 0.6);
        assert_eq!(args.h_max, 1.8);
        assert_eq!(args.out, PathBuf::from("tmp/out.jsonl"));
        assert_eq!(args.cache, PathBuf::from("tmp/cache.jsonl"));
    }

    #[test]
    fn included_pairs_respects_max_sides() {
        assert_eq!(included_pairs(3), vec![(3, 3)]);
        assert_eq!(included_pairs(4), vec![(3, 3), (3, 4), (4, 4)]);
    }
}

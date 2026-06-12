//! Unified datascience producer for run-local outputs.
//!
//! This binary owns producer metadata and delegates expensive polytope payloads
//! to the shared computed-polytope cache. It does not promote outputs to
//! canonical producer filenames.

use exp_sys_landscape::{
    poly_id, CapacityBackend, ComputedPolytopeCache, ComputedPolytopePayloadRow,
    SysLandscapePolytopeCache,
};
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use rayon::prelude::*;
use serde::Serialize;
use std::collections::BTreeSet;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use std::sync::Mutex;
use symplectic::algorithms::billiard::bounce_count_from_sigma_for_facets;
use symplectic::classify_facets_from_dual_vertices;
use symplectic::geom::polygon::random_polygon_2d;

mod rows;
use rows::{DatascienceRandomProductSampleRow, DatascienceRandomSampleRow};

const SEED: u64 = 42;
const H_MIN: f64 = 0.8;
const H_MAX: f64 = 1.2;
const GENERIC_FACETS: &[usize] = &[5, 6, 7, 8, 9, 10, 11, 12];
const PRODUCT_PAIRS: &[(usize, usize)] = &[
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Mode {
    Smoke,
    Production,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
enum Producer {
    Random,
    RandomProduct,
}

struct Args {
    mode: Mode,
    producers: BTreeSet<Producer>,
    output_dir: PathBuf,
    parallelism: usize,
    base_cache: PathBuf,
    seed: u64,
}

#[derive(Clone)]
enum WorkUnit {
    Random {
        name: String,
        facet_count: usize,
        attempt: u64,
        polytope: SysLandscapePolytopeCache,
    },
    RandomProduct {
        name: String,
        k: usize,
        m: usize,
        attempt: u64,
        polytope: SysLandscapePolytopeCache,
    },
}

struct ComputedWorkUnit {
    producer: Producer,
    random: Option<DatascienceRandomSampleRow>,
    random_product: Option<DatascienceRandomProductSampleRow>,
}

#[derive(Serialize)]
struct ProduceStatsRow {
    mode: String,
    producers: Vec<String>,
    seed: u64,
    parallelism: usize,
    random_rows: usize,
    random_product_rows: usize,
    computed_payload_rows: usize,
    cache_hits: usize,
    cache_misses: usize,
    failures: usize,
    max_sys: Option<f64>,
    cache_miss_volume_ms: f64,
    cache_miss_capacity_ms: f64,
    wall_time_ms: f64,
}

fn parse_args() -> Args {
    parse_args_from(std::env::args())
}

fn parse_args_from(argv: impl IntoIterator<Item = impl Into<String>>) -> Args {
    let argv: Vec<String> = argv.into_iter().map(Into::into).collect();
    let mut mode = None;
    let mut producers = None;
    let mut output_dir = None;
    let mut parallelism = None;
    let mut base_cache = None;
    let mut seed = SEED;

    let mut i = 1usize;
    while i < argv.len() {
        let flag = argv[i].as_str();
        let value = || -> &str {
            argv.get(i + 1)
                .map(String::as_str)
                .unwrap_or_else(|| panic!("{flag} requires a value"))
        };
        match flag {
            "--mode" => {
                mode = Some(match value() {
                    "smoke" => Mode::Smoke,
                    "production" => Mode::Production,
                    other => panic!("unknown --mode {other:?}; use smoke or production"),
                });
                i += 2;
            }
            "--producers" => {
                producers = Some(parse_producers(value()));
                i += 2;
            }
            "--output-dir" => {
                output_dir = Some(PathBuf::from(value()));
                i += 2;
            }
            "--parallelism" => {
                let n = value()
                    .parse()
                    .expect("--parallelism must be a positive integer");
                assert!(n > 0, "--parallelism must be positive");
                parallelism = Some(n);
                i += 2;
            }
            "--base-cache" => {
                base_cache = Some(PathBuf::from(value()));
                i += 2;
            }
            "--seed" => {
                seed = value().parse().expect("--seed must be a u64");
                i += 2;
            }
            "--help" | "-h" => {
                print_help(
                    argv.first()
                        .map(String::as_str)
                        .unwrap_or("sys-datascience-produce"),
                );
                std::process::exit(0);
            }
            other => panic!("unknown argument: {other}"),
        }
    }

    Args {
        mode: mode.expect("--mode is required"),
        producers: producers.expect("--producers is required"),
        output_dir: output_dir.expect("--output-dir is required"),
        parallelism: parallelism.expect("--parallelism is required"),
        base_cache: base_cache.expect("--base-cache is required"),
        seed,
    }
}

fn print_help(program: &str) {
    println!(
        "\
Run datascience producers into a reviewable output directory.

Usage:
  {program} --mode smoke|production --producers <list> --output-dir <dir> \\
    --parallelism <n> --base-cache <computed-polytopes.jsonl> [--seed <u64>]

Producers:
  random,random-product

Outputs:
  computed-polytopes.jsonl
  random-samples.jsonl
  random-product-samples.jsonl
"
    );
}

fn parse_producers(raw: &str) -> BTreeSet<Producer> {
    let mut producers = BTreeSet::new();
    for item in raw.split(',') {
        match item.trim() {
            "random" => {
                producers.insert(Producer::Random);
            }
            "random-product" => {
                producers.insert(Producer::RandomProduct);
            }
            "" => {}
            other => panic!("unknown producer {other:?}"),
        }
    }
    assert!(!producers.is_empty(), "--producers must not be empty");
    producers
}

fn mode_name(mode: Mode) -> &'static str {
    match mode {
        Mode::Smoke => "smoke",
        Mode::Production => "production",
    }
}

fn producer_name(producer: Producer) -> &'static str {
    match producer {
        Producer::Random => "random",
        Producer::RandomProduct => "random-product",
    }
}

fn generic_samples_per_f(mode: Mode) -> usize {
    match mode {
        Mode::Smoke => 1,
        Mode::Production => 512,
    }
}

fn product_samples_per_bucket(mode: Mode) -> usize {
    match mode {
        Mode::Smoke => 1,
        Mode::Production => 1024,
    }
}

fn random_work(seed: u64, mode: Mode) -> Vec<WorkUnit> {
    let samples_per_f = generic_samples_per_f(mode);
    let mut work = Vec::new();
    for &facet_count in GENERIC_FACETS {
        let mut accepted = 0usize;
        let mut attempt = 0u64;
        while accepted < samples_per_f {
            if let Some(polytope) =
                SysLandscapePolytopeCache::generate_random(facet_count, H_MIN, H_MAX, seed, attempt)
            {
                work.push(WorkUnit::Random {
                    name: format!("random_F{facet_count}_{accepted}"),
                    facet_count,
                    attempt,
                    polytope,
                });
                accepted += 1;
            }
            attempt += 1;
        }
    }
    work
}

fn product_seed(seed: u64, k: usize, m: usize, attempt: u64) -> [u8; 32] {
    let mut material = Vec::new();
    material.extend_from_slice(&seed.to_le_bytes());
    material.extend_from_slice(&(k as u64).to_le_bytes());
    material.extend_from_slice(&(m as u64).to_le_bytes());
    material.extend_from_slice(&attempt.to_le_bytes());
    blake3::derive_key("datascience-random-product", &material)
}

fn random_product_work(seed: u64, mode: Mode) -> Vec<WorkUnit> {
    let samples_per_bucket = product_samples_per_bucket(mode);
    let mut work = Vec::new();
    for &(k, m) in PRODUCT_PAIRS {
        let mut accepted = 0usize;
        let mut attempt = 0u64;
        while accepted < samples_per_bucket {
            let mut rng = ChaCha8Rng::from_seed(product_seed(seed, k, m, attempt));
            let (qn, qh) = random_polygon_2d(k, H_MIN, H_MAX, &mut rng);
            let (pn, ph) = random_polygon_2d(m, H_MIN, H_MAX, &mut rng);
            if let Some(polytope) =
                SysLandscapePolytopeCache::from_lagrangian_product(&qn, &qh, &pn, &ph)
            {
                work.push(WorkUnit::RandomProduct {
                    name: format!("random_{k}x{m}_{accepted}"),
                    k,
                    m,
                    attempt,
                    polytope,
                });
                accepted += 1;
            }
            attempt += 1;
        }
    }
    work
}

fn compute_work_unit(
    unit: WorkUnit,
    seed: u64,
    cache: &ComputedPolytopeCache,
) -> Option<ComputedWorkUnit> {
    match unit {
        WorkUnit::Random {
            name,
            facet_count,
            attempt,
            polytope,
        } => {
            let payload = cache.compute(&polytope, CapacityBackend::Auto)?;
            Some(ComputedWorkUnit {
                producer: Producer::Random,
                random: Some(DatascienceRandomSampleRow {
                    name,
                    poly_id: poly_id(&polytope),
                    facet_count,
                    seed,
                    attempt,
                    h_min: H_MIN,
                    h_max: H_MAX,
                    sys: payload.sys,
                }),
                random_product: None,
            })
        }
        WorkUnit::RandomProduct {
            name,
            k,
            m,
            attempt,
            polytope,
        } => {
            let payload = cache.compute(&polytope, CapacityBackend::Billiard)?;
            let classification = classify_facets_from_dual_vertices(&polytope.dual_vertices_f64)
                .expect("generated Lagrangian product should classify");
            let bounces = bounce_count_from_sigma_for_facets(
                &classification.q_indices,
                &classification.p_indices,
                &payload.sigmas[0].perm,
            )?;
            Some(ComputedWorkUnit {
                producer: Producer::RandomProduct,
                random: None,
                random_product: Some(DatascienceRandomProductSampleRow {
                    name,
                    poly_id: poly_id(&polytope),
                    k,
                    m,
                    facet_count: polytope.facet_count(),
                    seed,
                    attempt,
                    h_min: H_MIN,
                    h_max: H_MAX,
                    sys: payload.sys,
                    bounces,
                }),
            })
        }
    }
}

fn write_jsonl<T: Serialize>(path: PathBuf, rows: &[T]) {
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent).expect("create output parent");
        }
    }
    let file = File::create(&path).unwrap_or_else(|e| panic!("create {}: {e}", path.display()));
    let mut writer = BufWriter::new(file);
    for row in rows {
        let line = serde_json::to_string(row).expect("serialize JSON row");
        writeln!(writer, "{line}").unwrap_or_else(|e| panic!("write {}: {e}", path.display()));
    }
    writer
        .flush()
        .unwrap_or_else(|e| panic!("flush {}: {e}", path.display()));
}

fn write_json<T: Serialize>(path: PathBuf, value: &T) {
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent).expect("create output parent");
        }
    }
    let file = File::create(&path).unwrap_or_else(|e| panic!("create {}: {e}", path.display()));
    serde_json::to_writer_pretty(BufWriter::new(file), value)
        .unwrap_or_else(|e| panic!("write {}: {e}", path.display()));
}

fn main() {
    let total_started = std::time::Instant::now();
    let args = parse_args();
    rayon::ThreadPoolBuilder::new()
        .num_threads(args.parallelism)
        .build_global()
        .expect("initialize rayon global thread pool");

    std::fs::create_dir_all(&args.output_dir).expect("create output dir");

    let payload_path = args.output_dir.join("computed-polytopes.jsonl");
    let cache = ComputedPolytopeCache::load_with_wal(
        &[args.base_cache.clone()],
        Some(payload_path.clone()),
    );
    let mut work = Vec::new();
    if args.producers.contains(&Producer::Random) {
        work.extend(random_work(args.seed, args.mode));
    }
    if args.producers.contains(&Producer::RandomProduct) {
        work.extend(random_product_work(args.seed, args.mode));
    }

    println!(
        "datascience produce: mode={:?} producers={:?} work_units={} parallelism={} base_cache={}",
        args.mode,
        args.producers,
        work.len(),
        args.parallelism,
        args.base_cache.display()
    );

    let failures = Mutex::new(0usize);
    let mut computed: Vec<_> = work
        .into_par_iter()
        .filter_map(|unit| {
            let result = compute_work_unit(unit, args.seed, &cache);
            if result.is_none() {
                let mut failures = failures.lock().expect("failure mutex poisoned");
                *failures += 1;
            }
            result
        })
        .collect();
    computed.sort_by(|a, b| {
        let left = match a.producer {
            Producer::Random => &a.random.as_ref().expect("random row").name,
            Producer::RandomProduct => &a.random_product.as_ref().expect("product row").name,
        };
        let right = match b.producer {
            Producer::Random => &b.random.as_ref().expect("random row").name,
            Producer::RandomProduct => &b.random_product.as_ref().expect("product row").name,
        };
        a.producer.cmp(&b.producer).then_with(|| left.cmp(right))
    });

    let random_rows: Vec<_> = computed
        .iter()
        .filter_map(|row| row.random.as_ref())
        .collect();
    let random_product_rows: Vec<_> = computed
        .iter()
        .filter_map(|row| row.random_product.as_ref())
        .collect();
    let payload_rows: Vec<ComputedPolytopePayloadRow> = cache.used_rows();

    if args.producers.contains(&Producer::Random) {
        write_jsonl(args.output_dir.join("random-samples.jsonl"), &random_rows);
    }
    if args.producers.contains(&Producer::RandomProduct) {
        write_jsonl(
            args.output_dir.join("random-product-samples.jsonl"),
            &random_product_rows,
        );
    }
    write_jsonl(payload_path, &payload_rows);

    let stats = cache.stats();
    let failure_count = *failures.lock().expect("failure mutex poisoned");
    let produce_stats = ProduceStatsRow {
        mode: mode_name(args.mode).to_string(),
        producers: args
            .producers
            .iter()
            .map(|producer| producer_name(*producer).to_string())
            .collect(),
        seed: args.seed,
        parallelism: args.parallelism,
        random_rows: random_rows.len(),
        random_product_rows: random_product_rows.len(),
        computed_payload_rows: payload_rows.len(),
        cache_hits: stats.hits,
        cache_misses: stats.misses,
        failures: failure_count,
        max_sys: payload_rows.iter().map(|row| row.sys).reduce(f64::max),
        cache_miss_volume_ms: stats.miss_volume_ms,
        cache_miss_capacity_ms: stats.miss_capacity_ms,
        wall_time_ms: total_started.elapsed().as_secs_f64() * 1000.0,
    };
    write_json(args.output_dir.join("produce-stats.json"), &produce_stats);
    println!(
        "wrote random={} random_product={} computed_payloads={} cache_hits={} cache_misses={} failures={}",
        random_rows.len(),
        random_product_rows.len(),
        payload_rows.len(),
        stats.hits,
        stats.misses,
        failure_count
    );
    assert_eq!(failure_count, 0, "producer work units failed");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_requires_explicit_operational_paths() {
        let args = parse_args_from([
            "sys-datascience-produce",
            "--mode",
            "smoke",
            "--producers",
            "random,random-product",
            "--output-dir",
            "/tmp/out",
            "--parallelism",
            "1",
            "--base-cache",
            "/tmp/base.jsonl",
        ]);
        assert_eq!(args.mode, Mode::Smoke);
        assert!(args.producers.contains(&Producer::Random));
        assert!(args.producers.contains(&Producer::RandomProduct));
        assert_eq!(args.parallelism, 1);
    }

    #[test]
    fn production_counts_match_documented_targets() {
        assert_eq!(
            generic_samples_per_f(Mode::Production) * GENERIC_FACETS.len(),
            4096
        );
        assert_eq!(
            product_samples_per_bucket(Mode::Production) * PRODUCT_PAIRS.len(),
            10240
        );
    }
}

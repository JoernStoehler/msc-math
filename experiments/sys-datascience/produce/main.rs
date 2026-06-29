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
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
use symplectic::algorithms::billiard::bounce_count_from_sigma_for_facets;
use symplectic::classify_facets_from_dual_vertices;
use symplectic::geom::polygon::random_polygon_2d;

mod rows;
use rows::{
    DatascienceRandomProductSampleRow, DatascienceRandomSampleRow, DatascienceSampleSource,
};

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
    plan_only: bool,
    plan_file: Option<PathBuf>,
}

#[derive(Clone)]
enum WorkSpec {
    Random {
        name: String,
        facet_count: usize,
        h_min: f64,
        h_max: f64,
        sample_index: usize,
    },
    RandomProduct {
        name: String,
        k: usize,
        m: usize,
        h_min: f64,
        h_max: f64,
        sample_index: usize,
    },
}

impl WorkSpec {
    fn label(&self) -> String {
        match self {
            Self::Random {
                name, facet_count, ..
            } => {
                format!("{name} F={facet_count}")
            }
            Self::RandomProduct { name, k, m, .. } => {
                format!("{name} pair={k}x{m}")
            }
        }
    }
}

struct ComputedWorkUnit {
    producer: Producer,
    label: String,
    poly_id: String,
    random: Option<DatascienceRandomSampleRow>,
    random_product: Option<DatascienceRandomProductSampleRow>,
}

#[derive(Debug, Deserialize)]
struct ProducePlan {
    #[serde(default)]
    buckets: Vec<PlanBucket>,
    #[serde(default)]
    random: Vec<RandomPlanBucket>,
    #[serde(default)]
    random_product: Vec<RandomProductPlanBucket>,
}

#[derive(Debug, Deserialize)]
struct RandomPlanBucket {
    facet_count: usize,
    #[serde(default = "default_h_min")]
    h_min: f64,
    #[serde(default = "default_h_max")]
    h_max: f64,
    rows: usize,
}

#[derive(Debug, Deserialize)]
struct RandomProductPlanBucket {
    k: usize,
    m: usize,
    #[serde(default = "default_h_min")]
    h_min: f64,
    #[serde(default = "default_h_max")]
    h_max: f64,
    rows: usize,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "producer", rename_all = "kebab-case")]
enum PlanBucket {
    Random {
        facet_count: usize,
        #[serde(default = "default_h_min")]
        h_min: f64,
        #[serde(default = "default_h_max")]
        h_max: f64,
        rows: usize,
    },
    RandomProduct {
        k: usize,
        m: usize,
        #[serde(default = "default_h_min")]
        h_min: f64,
        #[serde(default = "default_h_max")]
        h_max: f64,
        rows: usize,
    },
}

fn default_h_min() -> f64 {
    H_MIN
}

fn default_h_max() -> f64 {
    H_MAX
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
    let mut plan_only = false;
    let mut plan_file = None;

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
            "--plan-only" => {
                plan_only = true;
                i += 1;
            }
            "--plan-file" => {
                plan_file = Some(PathBuf::from(value()));
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
        plan_only,
        plan_file,
    }
}

fn print_help(program: &str) {
    println!(
        "\
Run datascience producers into a reviewable output directory.

Usage:
  {program} --mode smoke|production --producers <list> --output-dir <dir> \\
    --parallelism <n> --base-cache <computed-polytopes.jsonl> [--seed <u64>] \\
    [--plan-file <json>] [--plan-only]

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

fn work_from_plan_buckets<'a>(
    buckets: impl IntoIterator<Item = &'a PlanBucket>,
    producers: &BTreeSet<Producer>,
) -> Vec<WorkSpec> {
    let mut work = Vec::new();
    let mut random_counts = Vec::new();
    let mut product_counts = Vec::new();
    for bucket in buckets {
        match bucket {
            PlanBucket::Random {
                facet_count,
                h_min,
                h_max,
                rows,
            } => {
                if producers.contains(&Producer::Random) {
                    random_counts.push((*facet_count, *h_min, *h_max, *rows));
                }
            }
            PlanBucket::RandomProduct {
                k,
                m,
                h_min,
                h_max,
                rows,
            } => {
                if producers.contains(&Producer::RandomProduct) {
                    product_counts.push((*k, *m, *h_min, *h_max, *rows));
                }
            }
        }
    }
    work.extend(random_work_from_counts(random_counts));
    work.extend(random_product_work_from_counts(product_counts));
    work
}

fn should_log_plan_progress(accepted: usize, target: usize) -> bool {
    accepted == 1 || accepted == target || accepted % 128 == 0
}

fn load_plan(path: &PathBuf) -> ProducePlan {
    let handle =
        File::open(path).unwrap_or_else(|e| panic!("open plan file {}: {e}", path.display()));
    serde_json::from_reader(handle)
        .unwrap_or_else(|e| panic!("parse plan file {}: {e}", path.display()))
}

fn validate_height_interval(h_min: f64, h_max: f64) {
    assert!(
        h_min.is_finite() && h_max.is_finite() && 0.0 < h_min && h_min < h_max,
        "height interval must satisfy finite 0 < h_min < h_max, got [{h_min}, {h_max}]"
    );
}

fn height_key(h_min: f64, h_max: f64) -> String {
    format!("{h_min}_{h_max}").replace('.', "p")
}

fn random_work_from_counts(
    counts: impl IntoIterator<Item = (usize, f64, f64, usize)>,
) -> Vec<WorkSpec> {
    let started = std::time::Instant::now();
    let mut work = Vec::new();
    for (facet_count, h_min, h_max, samples_per_f) in counts {
        validate_height_interval(h_min, h_max);
        println!("planning random F={facet_count} h=[{h_min},{h_max}]: target={samples_per_f}");
        flush_stdout();
        for sample_index in 0..samples_per_f {
            work.push(WorkSpec::Random {
                name: format!(
                    "random_F{facet_count}_h{}_{sample_index}",
                    height_key(h_min, h_max)
                ),
                facet_count,
                h_min,
                h_max,
                sample_index,
            });
            let planned = sample_index + 1;
            if should_log_plan_progress(planned, samples_per_f) {
                println!(
                    "planning random F={facet_count} h=[{h_min},{h_max}]: planned={planned}/{samples_per_f}"
                );
                flush_stdout();
            }
        }
    }
    println!(
        "planned random work units={} elapsed_ms={:.1}",
        work.len(),
        started.elapsed().as_secs_f64() * 1000.0
    );
    flush_stdout();
    work
}

fn random_work(mode: Mode) -> Vec<WorkSpec> {
    let samples_per_f = generic_samples_per_f(mode);
    random_work_from_counts(
        GENERIC_FACETS
            .iter()
            .copied()
            .map(|facet_count| (facet_count, H_MIN, H_MAX, samples_per_f)),
    )
}

fn random_seed(
    seed: u64,
    facet_count: usize,
    h_min: f64,
    h_max: f64,
    sample_index: usize,
    attempt: u64,
) -> [u8; 32] {
    let mut material = Vec::new();
    material.extend_from_slice(&seed.to_le_bytes());
    material.extend_from_slice(&(facet_count as u64).to_le_bytes());
    material.extend_from_slice(&h_min.to_le_bytes());
    material.extend_from_slice(&h_max.to_le_bytes());
    material.extend_from_slice(&(sample_index as u64).to_le_bytes());
    material.extend_from_slice(&attempt.to_le_bytes());
    blake3::derive_key("datascience-random-generic", &material)
}

fn product_seed(
    seed: u64,
    k: usize,
    m: usize,
    h_min: f64,
    h_max: f64,
    sample_index: usize,
    attempt: u64,
) -> [u8; 32] {
    let mut material = Vec::new();
    material.extend_from_slice(&seed.to_le_bytes());
    material.extend_from_slice(&(k as u64).to_le_bytes());
    material.extend_from_slice(&(m as u64).to_le_bytes());
    material.extend_from_slice(&h_min.to_le_bytes());
    material.extend_from_slice(&h_max.to_le_bytes());
    material.extend_from_slice(&(sample_index as u64).to_le_bytes());
    material.extend_from_slice(&attempt.to_le_bytes());
    blake3::derive_key("datascience-random-product", &material)
}

fn random_product_work_from_counts(
    counts: impl IntoIterator<Item = (usize, usize, f64, f64, usize)>,
) -> Vec<WorkSpec> {
    let started = std::time::Instant::now();
    let mut work = Vec::new();
    for (k, m, h_min, h_max, samples_per_bucket) in counts {
        validate_height_interval(h_min, h_max);
        println!(
            "planning random-product {k}x{m} h=[{h_min},{h_max}]: target={samples_per_bucket}"
        );
        flush_stdout();
        for sample_index in 0..samples_per_bucket {
            work.push(WorkSpec::RandomProduct {
                name: format!(
                    "random_{k}x{m}_h{}_{sample_index}",
                    height_key(h_min, h_max)
                ),
                k,
                m,
                h_min,
                h_max,
                sample_index,
            });
            let planned = sample_index + 1;
            if should_log_plan_progress(planned, samples_per_bucket) {
                println!(
                    "planning random-product {k}x{m} h=[{h_min},{h_max}]: planned={planned}/{samples_per_bucket}"
                );
                flush_stdout();
            }
        }
    }
    println!(
        "planned random-product work units={} elapsed_ms={:.1}",
        work.len(),
        started.elapsed().as_secs_f64() * 1000.0
    );
    flush_stdout();
    work
}

fn random_product_work(mode: Mode) -> Vec<WorkSpec> {
    let samples_per_bucket = product_samples_per_bucket(mode);
    random_product_work_from_counts(
        PRODUCT_PAIRS
            .iter()
            .copied()
            .map(|(k, m)| (k, m, H_MIN, H_MAX, samples_per_bucket)),
    )
}

fn generate_random_polytope(
    seed: u64,
    facet_count: usize,
    h_min: f64,
    h_max: f64,
    sample_index: usize,
) -> Option<(SysLandscapePolytopeCache, u64)> {
    for attempt in 0.. {
        let mut rng = ChaCha8Rng::from_seed(random_seed(
            seed,
            facet_count,
            h_min,
            h_max,
            sample_index,
            attempt,
        ));
        if let Some(polytope) =
            SysLandscapePolytopeCache::sample_random(facet_count, h_min, h_max, &mut rng)
        {
            return Some((polytope, attempt));
        }
    }
    None
}

fn generate_product_polytope(
    seed: u64,
    k: usize,
    m: usize,
    h_min: f64,
    h_max: f64,
    sample_index: usize,
) -> Option<(SysLandscapePolytopeCache, u64)> {
    for attempt in 0.. {
        let mut rng = ChaCha8Rng::from_seed(product_seed(
            seed,
            k,
            m,
            h_min,
            h_max,
            sample_index,
            attempt,
        ));
        let (qn, qh) = random_polygon_2d(k, h_min, h_max, &mut rng);
        let (pn, ph) = random_polygon_2d(m, h_min, h_max, &mut rng);
        if let Some(polytope) =
            SysLandscapePolytopeCache::from_lagrangian_product(&qn, &qh, &pn, &ph)
        {
            return Some((polytope, attempt));
        }
    }
    None
}

fn compute_work_unit(
    unit: WorkSpec,
    seed: u64,
    cache: &ComputedPolytopeCache,
) -> Option<ComputedWorkUnit> {
    match unit {
        WorkSpec::Random {
            name,
            facet_count,
            h_min,
            h_max,
            sample_index,
        } => {
            let (polytope, attempt) =
                generate_random_polytope(seed, facet_count, h_min, h_max, sample_index)?;
            let poly_id = poly_id(&polytope);
            let payload = cache.compute(&polytope, CapacityBackend::Auto)?;
            Some(ComputedWorkUnit {
                producer: Producer::Random,
                label: format!("{name} F={facet_count}"),
                poly_id: poly_id.clone(),
                random: Some(DatascienceRandomSampleRow {
                    name,
                    poly_id,
                    source: DatascienceSampleSource::Random {
                        facet_count,
                        h_min,
                        h_max,
                        seed: Some(seed),
                        sample_index: Some(sample_index),
                        attempt: Some(attempt),
                    },
                    sys: payload.sys,
                }),
                random_product: None,
            })
        }
        WorkSpec::RandomProduct {
            name,
            k,
            m,
            h_min,
            h_max,
            sample_index,
        } => {
            let (polytope, attempt) =
                generate_product_polytope(seed, k, m, h_min, h_max, sample_index)?;
            let poly_id = poly_id(&polytope);
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
                label: format!("{name} pair={k}x{m}"),
                poly_id: poly_id.clone(),
                random: None,
                random_product: Some(DatascienceRandomProductSampleRow {
                    name,
                    poly_id,
                    facet_count: polytope.facet_count(),
                    source: DatascienceSampleSource::RandomProduct {
                        k,
                        m,
                        h_min,
                        h_max,
                        seed: Some(seed),
                        sample_index: Some(sample_index),
                        attempt: Some(attempt),
                        bounces,
                    },
                    sys: payload.sys,
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

fn flush_stdout() {
    std::io::stdout().flush().expect("flush stdout");
}

fn report_work_plan(work: &[WorkSpec]) {
    println!(
        "work plan: units={} random_units={} random_product_units={}",
        work.len(),
        work.iter()
            .filter(|unit| matches!(unit, WorkSpec::Random { .. }))
            .count(),
        work.iter()
            .filter(|unit| matches!(unit, WorkSpec::RandomProduct { .. }))
            .count()
    );
    flush_stdout();
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
    let plan = args.plan_file.as_ref().map(load_plan);
    let mut work = Vec::new();
    if let Some(plan) = &plan {
        if !plan.buckets.is_empty() {
            work.extend(work_from_plan_buckets(&plan.buckets, &args.producers));
        } else {
            if args.producers.contains(&Producer::Random) {
                work.extend(random_work_from_counts(plan.random.iter().map(|bucket| {
                    (bucket.facet_count, bucket.h_min, bucket.h_max, bucket.rows)
                })));
            }
            if args.producers.contains(&Producer::RandomProduct) {
                work.extend(random_product_work_from_counts(
                    plan.random_product.iter().map(|bucket| {
                        (bucket.k, bucket.m, bucket.h_min, bucket.h_max, bucket.rows)
                    }),
                ));
            }
        }
    } else {
        if args.producers.contains(&Producer::Random) {
            work.extend(random_work(args.mode));
        }
        if args.producers.contains(&Producer::RandomProduct) {
            work.extend(random_product_work(args.mode));
        }
    }

    println!(
        "datascience produce: mode={:?} producers={:?} work_units={} parallelism={} base_cache={} plan_file={} plan_only={}",
        args.mode,
        args.producers,
        work.len(),
        args.parallelism,
        args.base_cache.display(),
        args.plan_file
            .as_ref()
            .map(|path| path.display().to_string())
            .unwrap_or_else(|| "<none>".to_string()),
        args.plan_only
    );
    report_work_plan(&work);
    if args.plan_only {
        println!(
            "plan-only complete: work_units={} elapsed_ms={:.1}",
            work.len(),
            total_started.elapsed().as_secs_f64() * 1000.0
        );
        flush_stdout();
        return;
    }

    let failures = Mutex::new(0usize);
    let started = AtomicUsize::new(0);
    let completed = AtomicUsize::new(0);
    let total_work = work.len();
    let mut computed: Vec<_> = work
        .into_par_iter()
        .filter_map(|unit| {
            let label = unit.label();
            let started_now = started.fetch_add(1, Ordering::Relaxed) + 1;
            if started_now <= args.parallelism || started_now % 512 == 0 {
                println!("started {started_now}/{total_work}: {label}");
                flush_stdout();
            }
            let unit_started = std::time::Instant::now();
            let result = compute_work_unit(unit, args.seed, &cache);
            if result.is_none() {
                let mut failures = failures.lock().expect("failure mutex poisoned");
                *failures += 1;
            }
            let completed_now = completed.fetch_add(1, Ordering::Relaxed) + 1;
            let elapsed_ms = unit_started.elapsed().as_secs_f64() * 1000.0;
            if completed_now <= args.parallelism || completed_now % 128 == 0 || result.is_none() {
                let (label, poly_id) = result
                    .as_ref()
                    .map(|row| (row.label.as_str(), row.poly_id.as_str()))
                    .unwrap_or((label.as_str(), "<failed-before-poly-id>"));
                println!(
                    "completed {completed_now}/{total_work}: {label} poly_id={poly_id} ok={} elapsed_ms={elapsed_ms:.1}",
                    result.is_some()
                );
                flush_stdout();
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
            "--plan-only",
            "--plan-file",
            "/tmp/plan.json",
        ]);
        assert_eq!(args.mode, Mode::Smoke);
        assert!(args.producers.contains(&Producer::Random));
        assert!(args.producers.contains(&Producer::RandomProduct));
        assert_eq!(args.parallelism, 1);
        assert!(args.plan_only);
        assert_eq!(args.plan_file, Some(PathBuf::from("/tmp/plan.json")));
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

    #[test]
    fn explicit_counts_build_targeted_work() {
        let random = random_work_from_counts([(10, 0.8, 1.2, 2), (12, 0.6, 1.4, 1)]);
        assert_eq!(random.len(), 3);
        assert!(matches!(
            &random[0],
            WorkSpec::Random {
                facet_count: 10,
                h_min: 0.8,
                h_max: 1.2,
                sample_index: 0,
                ..
            }
        ));
        assert!(matches!(
            &random[2],
            WorkSpec::Random {
                facet_count: 12,
                h_min: 0.6,
                h_max: 1.4,
                sample_index: 0,
                ..
            }
        ));

        let product = random_product_work_from_counts([(4, 6, 0.7, 1.3, 2)]);
        assert_eq!(product.len(), 2);
        assert!(matches!(
            &product[1],
            WorkSpec::RandomProduct {
                k: 4,
                m: 6,
                h_min: 0.7,
                h_max: 1.3,
                sample_index: 1,
                ..
            }
        ));
    }

    #[test]
    fn unnamed_plan_buckets_build_targeted_work() {
        let producers = parse_producers("random,random-product");
        let buckets = [
            PlanBucket::Random {
                facet_count: 8,
                h_min: 0.8,
                h_max: 1.2,
                rows: 1,
            },
            PlanBucket::RandomProduct {
                k: 3,
                m: 5,
                h_min: 0.6,
                h_max: 1.4,
                rows: 2,
            },
        ];
        let work = work_from_plan_buckets(&buckets, &producers);
        assert_eq!(work.len(), 3);
        assert!(matches!(
            &work[0],
            WorkSpec::Random {
                facet_count: 8,
                h_min: 0.8,
                h_max: 1.2,
                sample_index: 0,
                ..
            }
        ));
        assert!(matches!(
            &work[2],
            WorkSpec::RandomProduct {
                k: 3,
                m: 5,
                h_min: 0.6,
                h_max: 1.4,
                sample_index: 1,
                ..
            }
        ));
    }
}

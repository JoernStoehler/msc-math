//! Target-free generic F=10 ridge-tail stage-one producer.
//!
//! This binary deliberately has no capacity call. It uses rational-arithmetic Euclidean
//! volume only as a reference for the f64 ridge-proxy screen.

#[path = "../../../prepare/features_face_symplectic.rs"]
mod features_face_symplectic;
#[path = "../../../prepare/features_helpers.rs"]
mod features_helpers;

use euclidean_polytopes::{
    two_faces_from_vertex_facet_incidence, volume_from_incidence_exact, volume_from_incidence_f64,
};
use exp_sys_landscape::{poly_id, SysLandscapePolytopeCache};
use nalgebra::Vector4 as NVector4;
use num_rational::BigRational;
use num_traits::ToPrimitive;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use rayon::prelude::*;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fs::{self, File};
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::path::{Path, PathBuf};
use std::time::Instant;

const FACET_COUNT: usize = 10;
const H_MIN: f64 = 0.8;
const H_MAX: f64 = 1.2;
const DEFAULT_COUNT: usize = 10_000;
const DEFAULT_SEED: u64 = 20_260_714;
const SELECTED_FRACTION: f64 = 0.01;
const DEFAULT_SUBSET_COUNT: usize = 64;
const PROXY_NAME: &str = "ridge_symp_area_mean_over_volume_sqrt";
const SELECTION_ID: &str = "generic-f10-low-ridge-mean-stage1-v1";
const BASELINE_ID: &str = "generic-f10-low-ridge-mean-stage1-v1__baseline_rep_0";

#[derive(Clone, Debug)]
struct Args {
    command: String,
    values: BTreeMap<String, String>,
}

impl Args {
    fn parse() -> Self {
        let argv = std::env::args().collect::<Vec<_>>();
        assert!(
            argv.len() >= 2,
            "usage: generic-ridge-tail-stage1 <smoke|produce|validate> [--key value ...]"
        );
        let command = argv[1].clone();
        let mut values = BTreeMap::new();
        let mut index = 2;
        while index < argv.len() {
            let key = argv[index]
                .strip_prefix("--")
                .unwrap_or_else(|| panic!("expected --flag, got {}", argv[index]));
            assert!(index + 1 < argv.len(), "missing value for --{key}");
            assert!(
                values
                    .insert(key.to_string(), argv[index + 1].clone())
                    .is_none(),
                "duplicate --{key}"
            );
            index += 2;
        }
        Self { command, values }
    }

    fn required_path(&self, key: &str) -> PathBuf {
        PathBuf::from(
            self.values
                .get(key)
                .unwrap_or_else(|| panic!("missing --{key}")),
        )
    }

    fn optional_path(&self, key: &str, default: impl Into<PathBuf>) -> PathBuf {
        self.values
            .get(key)
            .map(PathBuf::from)
            .unwrap_or_else(|| default.into())
    }

    fn usize(&self, key: &str, default: usize) -> usize {
        self.values
            .get(key)
            .map(|value| {
                value
                    .parse()
                    .unwrap_or_else(|_| panic!("invalid --{key} {value}"))
            })
            .unwrap_or(default)
    }

    fn u64(&self, key: &str, default: u64) -> u64 {
        self.values
            .get(key)
            .map(|value| {
                value
                    .parse()
                    .unwrap_or_else(|_| panic!("invalid --{key} {value}"))
            })
            .unwrap_or(default)
    }

    fn f64(&self, key: &str, default: f64) -> f64 {
        self.values
            .get(key)
            .map(|value| {
                value
                    .parse()
                    .unwrap_or_else(|_| panic!("invalid --{key} {value}"))
            })
            .unwrap_or(default)
    }

    fn deny_unknown(&self, allowed: &[&str]) {
        for key in self.values.keys() {
            assert!(allowed.contains(&key.as_str()), "unknown flag --{key}");
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
struct RetainedRandomRow {
    name: String,
    facet_count: usize,
    dual_vertices_rational: Vec<[String; 4]>,
    vertices_rational: Vec<[String; 4]>,
    volume: f64,
}

#[derive(Clone, Debug, Deserialize)]
struct RetainedTableRow {
    poly_id: String,
    facet_count: usize,
    capacity_source: String,
    sys: f64,
    ridge_symp_area_mean_over_volume_sqrt: f64,
}

#[derive(Clone, Debug, Serialize)]
struct SmokeRow {
    poly_id: String,
    retained_name: String,
    rational_volume_f64: f64,
    f64_volume: f64,
    relative_volume_error: f64,
    rational_reference_proxy: f64,
    f64_proxy: f64,
    retained_sys: f64,
    sys_using_f64_volume: f64,
    rational_volume_ms: f64,
    f64_volume_ms: f64,
}

#[derive(Clone, Debug, Serialize)]
struct MembershipCheck {
    fraction: f64,
    retained_count: usize,
    rational_reference_bottom_count: usize,
    overlap_count: usize,
    recall: f64,
    precision: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct SourceHash {
    path: String,
    blake3: String,
}

#[derive(Clone, Debug, Serialize)]
struct SmokeSummary {
    schema: &'static str,
    role: &'static str,
    retained_population: &'static str,
    row_count: usize,
    rational_volume_reference: &'static str,
    proxy: &'static str,
    max_absolute_relative_volume_error: f64,
    mean_absolute_relative_volume_error: f64,
    max_absolute_sys_change_from_f64_volume: f64,
    max_relative_sys_change_from_f64_volume: f64,
    nonfinite_or_invalid_count: usize,
    rational_reference_rank_equal_count: usize,
    spearman_rank_correlation: f64,
    bottom_one_percent: MembershipCheck,
    f64_bottom_two_percent_screen: MembershipCheck,
    f64_bottom_three_percent_screen: MembershipCheck,
    rational_volume_worker_time_ms: f64,
    f64_volume_worker_time_ms: f64,
    smoke_wall_time_ms: f64,
    measured_rational_over_f64_worker_time_ratio: f64,
    pass_for_stage_one: bool,
    frozen_high_sys_threshold: HighSysThreshold,
    source_hashes: Vec<SourceHash>,
    target_boundary: &'static str,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct HighSysThreshold {
    definition: String,
    retained_population_count: usize,
    exceedance_tail_fraction: f64,
    nearest_rank: usize,
    value: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct CandidateRow {
    candidate_id: String,
    poly_id: String,
    sample_index: usize,
    rejection_attempt: u64,
    ridge_count: usize,
    ridge_symp_area_mean: f64,
    f64_volume: f64,
    f64_proxy: f64,
}

#[derive(Clone, Debug)]
struct GeneratedCandidate {
    row: CandidateRow,
    polytope: SysLandscapePolytopeCache,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct SelectionRow {
    candidate_id: String,
    poly_id: String,
    sample_index: usize,
    rejection_attempt: u64,
    ridge_count: usize,
    ridge_symp_area_mean: f64,
    f64_volume: f64,
    f64_proxy: f64,
    f64_rank: usize,
    selected: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct PanelGeometryRow {
    schema: String,
    candidate_id: String,
    poly_id: String,
    sample_index: usize,
    rejection_attempt: u64,
    facet_count: usize,
    height_min: f64,
    height_max: f64,
    selection_ids: Vec<String>,
    baseline_ids: Vec<String>,
    evaluation_roles: Vec<String>,
    future_band: String,
    proxy: String,
    proxy_value_f64: f64,
    f64_rank: usize,
    dual_vertices_rational: Vec<[String; 4]>,
    vertices_rational: Vec<[String; 4]>,
    stage_order: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct SelectionBoundary {
    selected_count: usize,
    last_selected_candidate_id: String,
    last_selected_f64_rank: usize,
    last_selected_f64_proxy: f64,
    first_excluded_candidate_id: String,
    first_excluded_f64_rank: usize,
    first_excluded_f64_proxy: f64,
    f64_proxy_gap: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct ManifestCounts {
    accepted_candidates: usize,
    production_rational_volume_evaluations: usize,
    selected: usize,
    baseline: usize,
    panel_union: usize,
    future_band_zero_to_point_one_percent: usize,
    future_band_point_one_to_one_percent: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct TimingSummary {
    generation_and_f64_ranking_wall_ms: f64,
    panel_geometry_wall_ms: f64,
    total_wall_ms: f64,
    process_user_cpu_seconds: f64,
    process_system_cpu_seconds: f64,
    max_rss_kib: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Manifest {
    schema: String,
    status: String,
    generator: String,
    seed: u64,
    facet_count: usize,
    height_min: f64,
    height_max: f64,
    workers: usize,
    proxy: String,
    selection_rule: String,
    production_volume_definition: String,
    future_sys_volume_definition: String,
    baseline_rule: String,
    counts: ManifestCounts,
    selection_boundary: SelectionBoundary,
    candidate_population_hash: String,
    deterministic_subset_count: usize,
    deterministic_subset_hash: String,
    selected_hash: String,
    baseline_hash: String,
    panel_hash: String,
    smoke_contract: String,
    frozen_high_sys_threshold: f64,
    frozen_high_sys_threshold_definition: String,
    source_hashes: Vec<SourceHash>,
    timing: TimingSummary,
    target_exposure: TargetExposure,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct TargetExposure {
    capacity_computed_for_new_population: bool,
    sys_computed_for_new_population: bool,
    target_fields_present_in_stage_one_artifacts: bool,
    statement: String,
}

#[derive(Clone, Debug, Serialize)]
struct ValidationSummary {
    schema: &'static str,
    valid: bool,
    checks: BTreeMap<String, bool>,
    artifact_hashes: Vec<SourceHash>,
    artifact_bytes: BTreeMap<String, u64>,
    deterministic_subset_hash: String,
    target_field_tokens_absent: bool,
}

#[derive(Clone, Copy, Debug)]
struct Usage {
    user_seconds: f64,
    system_seconds: f64,
    max_rss_kib: i64,
}

fn usage() -> Usage {
    unsafe {
        let mut value: libc::rusage = std::mem::zeroed();
        assert_eq!(
            libc::getrusage(libc::RUSAGE_SELF, &mut value),
            0,
            "getrusage failed"
        );
        let seconds = |time: libc::timeval| time.tv_sec as f64 + time.tv_usec as f64 / 1_000_000.0;
        Usage {
            user_seconds: seconds(value.ru_utime),
            system_seconds: seconds(value.ru_stime),
            max_rss_kib: value.ru_maxrss,
        }
    }
}

fn parse_rational(value: &str) -> BigRational {
    value
        .parse()
        .unwrap_or_else(|error| panic!("parse rational {value}: {error}"))
}

fn parse_rational_vectors(rows: &[[String; 4]]) -> Vec<[BigRational; 4]> {
    rows.iter()
        .map(|row| std::array::from_fn(|index| parse_rational(&row[index])))
        .collect()
}

fn rational_vectors_to_strings(rows: &[[BigRational; 4]]) -> Vec<[String; 4]> {
    rows.iter()
        .map(|row| {
            std::array::from_fn(|index| format!("{}/{}", row[index].numer(), row[index].denom()))
        })
        .collect()
}

fn rational_volume_f64(polytope: &SysLandscapePolytopeCache) -> f64 {
    let vertices = polytope
        .vertices
        .iter()
        .map(|row| {
            NVector4::new(
                row[0].clone(),
                row[1].clone(),
                row[2].clone(),
                row[3].clone(),
            )
        })
        .collect::<Vec<_>>();
    volume_from_incidence_exact(&vertices, &polytope.vertex_facet_incidence)
        .to_f64()
        .expect("rational-arithmetic volume converts to f64")
}

fn ridge_mean(polytope: &SysLandscapePolytopeCache) -> (usize, f64) {
    let two_faces = two_faces_from_vertex_facet_incidence(&polytope.vertex_facet_incidence);
    let fields = features_face_symplectic::compute_face_symplectic_fields(
        &two_faces,
        &polytope.vertices_f64,
        &polytope.vertex_facet_incidence,
        1.0,
    );
    assert_eq!(
        fields.ridge_symp_area_ordering_failure_count, 0,
        "two-face ordering failure"
    );
    assert_eq!(
        fields.ridge_symp_area_ordered_face_count,
        two_faces.len(),
        "not all ridges ordered"
    );
    assert!(
        fields.ridge_symp_area_mean.is_finite() && fields.ridge_symp_area_mean > 0.0,
        "invalid ridge mean"
    );
    (two_faces.len(), fields.ridge_symp_area_mean)
}

fn random_seed(seed: u64, sample_index: usize, attempt: u64) -> [u8; 32] {
    let mut material = Vec::new();
    material.extend_from_slice(&seed.to_le_bytes());
    material.extend_from_slice(&(FACET_COUNT as u64).to_le_bytes());
    material.extend_from_slice(&H_MIN.to_le_bytes());
    material.extend_from_slice(&H_MAX.to_le_bytes());
    material.extend_from_slice(&(sample_index as u64).to_le_bytes());
    material.extend_from_slice(&attempt.to_le_bytes());
    blake3::derive_key("datascience-random-generic", &material)
}

fn candidate_id(seed: u64, sample_index: usize) -> String {
    let mut hasher = blake3::Hasher::new();
    hasher.update(b"generic-ridge-tail-stage1-candidate-v1");
    hasher.update(&seed.to_le_bytes());
    hasher.update(&(FACET_COUNT as u64).to_le_bytes());
    hasher.update(&H_MIN.to_le_bytes());
    hasher.update(&H_MAX.to_le_bytes());
    hasher.update(&(sample_index as u64).to_le_bytes());
    hasher.finalize().to_hex().to_string()
}

fn generate_candidate(seed: u64, sample_index: usize) -> GeneratedCandidate {
    for attempt in 0.. {
        let mut rng = ChaCha8Rng::from_seed(random_seed(seed, sample_index, attempt));
        if let Some(polytope) =
            SysLandscapePolytopeCache::sample_random(FACET_COUNT, H_MIN, H_MAX, &mut rng)
        {
            let f64_volume =
                volume_from_incidence_f64(&polytope.vertices_f64, &polytope.vertex_facet_incidence)
                    .expect("finite generated geometry");
            assert!(
                f64_volume.is_finite() && f64_volume > 0.0,
                "invalid f64 volume"
            );
            let (ridge_count, ridge_symp_area_mean) = ridge_mean(&polytope);
            let row = CandidateRow {
                candidate_id: candidate_id(seed, sample_index),
                poly_id: poly_id(&polytope),
                sample_index,
                rejection_attempt: attempt,
                ridge_count,
                ridge_symp_area_mean,
                f64_volume,
                f64_proxy: ridge_symp_area_mean / f64_volume.sqrt(),
            };
            assert!(
                row.f64_proxy.is_finite() && row.f64_proxy > 0.0,
                "invalid f64 proxy"
            );
            return GeneratedCandidate { row, polytope };
        }
    }
    unreachable!()
}

fn compare_proxy_id(left_proxy: f64, left_id: &str, right_proxy: f64, right_id: &str) -> Ordering {
    left_proxy
        .total_cmp(&right_proxy)
        .then_with(|| left_id.cmp(right_id))
}

fn ceil_fraction(count: usize, fraction: f64) -> usize {
    ((count as f64 * fraction).ceil() as usize)
        .max(1)
        .min(count)
}

fn hash_file(path: &Path) -> String {
    let mut reader = BufReader::new(
        File::open(path).unwrap_or_else(|error| panic!("open {}: {error}", path.display())),
    );
    let mut hasher = blake3::Hasher::new();
    let mut buffer = vec![0u8; 1024 * 1024];
    loop {
        let count = reader.read(&mut buffer).expect("read file for hash");
        if count == 0 {
            break;
        }
        hasher.update(&buffer[..count]);
    }
    hasher.finalize().to_hex().to_string()
}

fn source_hash(path: &Path) -> SourceHash {
    SourceHash {
        path: path.display().to_string(),
        blake3: hash_file(path),
    }
}

fn read_jsonl<T: DeserializeOwned>(path: &Path) -> Vec<T> {
    BufReader::new(
        File::open(path).unwrap_or_else(|error| panic!("open {}: {error}", path.display())),
    )
    .lines()
    .enumerate()
    .map(|(index, line)| {
        serde_json::from_str(&line.expect("read JSONL line"))
            .unwrap_or_else(|error| panic!("parse {} line {}: {error}", path.display(), index + 1))
    })
    .collect()
}

fn write_json(path: &Path, value: &impl Serialize) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("create output directory");
    }
    let file =
        File::create(path).unwrap_or_else(|error| panic!("create {}: {error}", path.display()));
    serde_json::to_writer_pretty(BufWriter::new(file), value).expect("write JSON");
}

fn write_jsonl(path: &Path, values: &[impl Serialize]) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("create output directory");
    }
    let mut writer = BufWriter::new(
        File::create(path).unwrap_or_else(|error| panic!("create {}: {error}", path.display())),
    );
    for value in values {
        serde_json::to_writer(&mut writer, value).expect("write JSONL row");
        writeln!(writer).expect("write JSONL newline");
    }
    writer.flush().expect("flush JSONL");
}

fn rank_map(rows: &[SmokeRow], use_rational_reference: bool) -> HashMap<String, usize> {
    let mut indices = (0..rows.len()).collect::<Vec<_>>();
    indices.sort_by(|&left, &right| {
        let left_proxy = if use_rational_reference {
            rows[left].rational_reference_proxy
        } else {
            rows[left].f64_proxy
        };
        let right_proxy = if use_rational_reference {
            rows[right].rational_reference_proxy
        } else {
            rows[right].f64_proxy
        };
        compare_proxy_id(
            left_proxy,
            &rows[left].poly_id,
            right_proxy,
            &rows[right].poly_id,
        )
    });
    indices
        .into_iter()
        .enumerate()
        .map(|(rank, index)| (rows[index].poly_id.clone(), rank + 1))
        .collect()
}

fn membership_check(
    rows: &[SmokeRow],
    reference_bottom_fraction: f64,
    retained_fraction: f64,
) -> MembershipCheck {
    let mut rational_reference = rows.iter().collect::<Vec<_>>();
    rational_reference.sort_by(|left, right| {
        compare_proxy_id(
            left.rational_reference_proxy,
            &left.poly_id,
            right.rational_reference_proxy,
            &right.poly_id,
        )
    });
    let reference_count = ceil_fraction(rows.len(), reference_bottom_fraction);
    let reference_ids = rational_reference
        .iter()
        .take(reference_count)
        .map(|row| row.poly_id.as_str())
        .collect::<BTreeSet<_>>();
    let mut approximate = rows.iter().collect::<Vec<_>>();
    approximate.sort_by(|left, right| {
        compare_proxy_id(
            left.f64_proxy,
            &left.poly_id,
            right.f64_proxy,
            &right.poly_id,
        )
    });
    let retained_count = ceil_fraction(rows.len(), retained_fraction);
    let retained_ids = approximate
        .iter()
        .take(retained_count)
        .map(|row| row.poly_id.as_str())
        .collect::<BTreeSet<_>>();
    let overlap_count = reference_ids.intersection(&retained_ids).count();
    MembershipCheck {
        fraction: retained_fraction,
        retained_count,
        rational_reference_bottom_count: reference_count,
        overlap_count,
        recall: overlap_count as f64 / reference_count as f64,
        precision: overlap_count as f64 / retained_count as f64,
    }
}

fn smoke(args: &Args) {
    args.deny_unknown(&["random-path", "table-path", "out", "workers"]);
    let started = Instant::now();
    let random_path = args.required_path("random-path");
    let table_path = args.required_path("table-path");
    let out = args.required_path("out");
    let workers = args.usize("workers", 12);
    assert!((1..=12).contains(&workers), "workers must be 1..=12");
    rayon::ThreadPoolBuilder::new()
        .num_threads(workers)
        .build_global()
        .expect("build rayon pool");

    let table_rows = read_jsonl::<RetainedTableRow>(&table_path)
        .into_iter()
        .filter(|row| row.facet_count == FACET_COUNT && row.capacity_source == "random_sample")
        .collect::<Vec<_>>();
    assert_eq!(
        table_rows.len(),
        512,
        "expected retained generic F=10 population"
    );
    let table_by_id = table_rows
        .iter()
        .map(|row| (row.poly_id.as_str(), row))
        .collect::<HashMap<_, _>>();
    let mut sys_values = table_rows.iter().map(|row| row.sys).collect::<Vec<_>>();
    assert!(
        sys_values
            .iter()
            .all(|value| value.is_finite() && *value <= 1.0),
        "trusted retained generic F=10 sys > 1 or invalid"
    );
    sys_values.sort_by(f64::total_cmp);
    let high_rank = (0.90 * sys_values.len() as f64).ceil() as usize;
    let threshold = HighSysThreshold {
        definition: "retained generic F=10 nearest-rank empirical 90th percentile; future exceedance is sys >= threshold".to_string(),
        retained_population_count: sys_values.len(),
        exceedance_tail_fraction: 0.10,
        nearest_rank: high_rank,
        value: sys_values[high_rank - 1],
    };

    let retained = read_jsonl::<RetainedRandomRow>(&random_path)
        .into_iter()
        .filter(|row| row.facet_count == FACET_COUNT)
        .collect::<Vec<_>>();
    assert_eq!(
        retained.len(),
        table_rows.len(),
        "retained geometry/table count mismatch"
    );
    let smoke_rows = retained
        .par_iter()
        .map(|row| {
            let dual = parse_rational_vectors(&row.dual_vertices_rational);
            let vertices = parse_rational_vectors(&row.vertices_rational);
            let polytope = SysLandscapePolytopeCache::from_rational_parts(dual, vertices)
                .expect("reconstruct retained geometry");
            let id = poly_id(&polytope);
            let table = table_by_id
                .get(id.as_str())
                .unwrap_or_else(|| panic!("retained poly_id {id} missing table row"));
            let (.., numerator) = ridge_mean(&polytope);
            let rational_started = Instant::now();
            let rational_volume = rational_volume_f64(&polytope);
            let rational_volume_ms = rational_started.elapsed().as_secs_f64() * 1000.0;
            let f64_started = Instant::now();
            let approximate =
                volume_from_incidence_f64(&polytope.vertices_f64, &polytope.vertex_facet_incidence)
                    .expect("retained f64 volume");
            let f64_volume_ms = f64_started.elapsed().as_secs_f64() * 1000.0;
            let rational_reference_proxy = numerator / rational_volume.sqrt();
            let f64_proxy = numerator / approximate.sqrt();
            let sys_using_f64_volume = table.sys * rational_volume / approximate;
            assert!(
                (rational_volume - row.volume).abs() <= 1e-12 * rational_volume.abs().max(1.0),
                "retained stored/rational-arithmetic volume mismatch"
            );
            assert!(
                (rational_reference_proxy - table.ridge_symp_area_mean_over_volume_sqrt).abs()
                    <= 1e-10 * rational_reference_proxy.abs().max(1.0),
                "prepared/rational-reference proxy mismatch"
            );
            SmokeRow {
                poly_id: id,
                retained_name: row.name.clone(),
                rational_volume_f64: rational_volume,
                f64_volume: approximate,
                relative_volume_error: (approximate - rational_volume) / rational_volume,
                rational_reference_proxy,
                f64_proxy,
                retained_sys: table.sys,
                sys_using_f64_volume,
                rational_volume_ms,
                f64_volume_ms,
            }
        })
        .collect::<Vec<_>>();

    let rational_reference_ranks = rank_map(&smoke_rows, true);
    let approximate_ranks = rank_map(&smoke_rows, false);
    let rational_reference_rank_equal_count = rational_reference_ranks
        .iter()
        .filter(|(id, rank)| approximate_ranks.get(*id) == Some(*rank))
        .count();
    let squared_rank_delta = rational_reference_ranks
        .iter()
        .map(|(id, rational_reference_rank)| {
            let delta =
                *rational_reference_rank as f64 - *approximate_ranks.get(id).unwrap() as f64;
            delta * delta
        })
        .sum::<f64>();
    let n = smoke_rows.len() as f64;
    let spearman = 1.0 - 6.0 * squared_rank_delta / (n * (n * n - 1.0));
    let errors = smoke_rows
        .iter()
        .map(|row| row.relative_volume_error.abs())
        .collect::<Vec<_>>();
    let sys_absolute_changes = smoke_rows
        .iter()
        .map(|row| (row.sys_using_f64_volume - row.retained_sys).abs())
        .collect::<Vec<_>>();
    let sys_relative_changes = smoke_rows
        .iter()
        .map(|row| ((row.sys_using_f64_volume - row.retained_sys) / row.retained_sys).abs())
        .collect::<Vec<_>>();
    let bottom_one = membership_check(&smoke_rows, 0.01, 0.01);
    let bottom_two = membership_check(&smoke_rows, 0.01, 0.02);
    let bottom_three = membership_check(&smoke_rows, 0.01, 0.03);
    let nonfinite = smoke_rows
        .iter()
        .filter(|row| {
            !row.rational_volume_f64.is_finite()
                || !row.f64_volume.is_finite()
                || row.rational_volume_f64 <= 0.0
                || row.f64_volume <= 0.0
                || !row.rational_reference_proxy.is_finite()
                || !row.f64_proxy.is_finite()
        })
        .count();
    let pass_for_stage_one = nonfinite == 0 && bottom_one.recall == 1.0 && bottom_two.recall == 1.0;
    let rational_volume_ms = smoke_rows
        .iter()
        .map(|row| row.rational_volume_ms)
        .sum::<f64>();
    let f64_ms = smoke_rows.iter().map(|row| row.f64_volume_ms).sum::<f64>();
    let summary = SmokeSummary {
        schema: "sys-datascience.generic-ridge-tail-stage1.smoke.v1",
        role: "engineering smoke; not a scientific stage",
        retained_population: "retained seed-42 generic F=10 random_sample rows",
        row_count: smoke_rows.len(),
        rational_volume_reference: "euclidean_polytopes::volume_from_incidence_exact returns BigRational; this packet converts that result to f64 for proxy ranking and reporting",
        proxy: PROXY_NAME,
        max_absolute_relative_volume_error: errors.iter().copied().fold(0.0, f64::max),
        mean_absolute_relative_volume_error: errors.iter().sum::<f64>() / errors.len() as f64,
        max_absolute_sys_change_from_f64_volume: sys_absolute_changes
            .iter()
            .copied()
            .fold(0.0, f64::max),
        max_relative_sys_change_from_f64_volume: sys_relative_changes
            .iter()
            .copied()
            .fold(0.0, f64::max),
        nonfinite_or_invalid_count: nonfinite,
        rational_reference_rank_equal_count,
        spearman_rank_correlation: spearman,
        bottom_one_percent: bottom_one,
        f64_bottom_two_percent_screen: bottom_two,
        f64_bottom_three_percent_screen: bottom_three,
        rational_volume_worker_time_ms: rational_volume_ms,
        f64_volume_worker_time_ms: f64_ms,
        smoke_wall_time_ms: started.elapsed().as_secs_f64() * 1000.0,
        measured_rational_over_f64_worker_time_ratio: rational_volume_ms / f64_ms,
        pass_for_stage_one,
        frozen_high_sys_threshold: threshold,
        source_hashes: vec![source_hash(&random_path), source_hash(&table_path)],
        target_boundary: "Existing retained sys was read only to freeze the predeclared high-sys threshold and check for trusted sys > 1; no new candidate target was computed.",
    };
    assert!(summary.pass_for_stage_one, "smoke gate failed");
    write_json(&out, &summary);
    println!("smoke passed: rows={} max_rel_error={:.3e} bottom2_recall={:.3} rational_volume_ms={:.1} f64_ms={:.1}", summary.row_count, summary.max_absolute_relative_volume_error, summary.f64_bottom_two_percent_screen.recall, rational_volume_ms, f64_ms);
}

fn hash_candidate_rows(rows: &[CandidateRow]) -> String {
    let mut hasher = blake3::Hasher::new();
    for row in rows {
        hasher.update(
            serde_json::to_string(row)
                .expect("serialize candidate hash row")
                .as_bytes(),
        );
        hasher.update(b"\n");
    }
    hasher.finalize().to_hex().to_string()
}

fn hash_ids<'a>(domain: &[u8], ids: impl IntoIterator<Item = &'a str>) -> String {
    let mut hasher = blake3::Hasher::new();
    hasher.update(domain);
    for id in ids {
        hasher.update(id.as_bytes());
        hasher.update(b"\n");
    }
    hasher.finalize().to_hex().to_string()
}

fn baseline_hash(candidate_id: &str) -> [u8; 32] {
    let mut hasher = blake3::Hasher::new();
    hasher.update(b"generic-ridge-tail-stage1-baseline-v1");
    hasher.update(SELECTION_ID.as_bytes());
    hasher.update(&0u64.to_le_bytes());
    hasher.update(candidate_id.as_bytes());
    *hasher.finalize().as_bytes()
}

fn source_files() -> Vec<PathBuf> {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    vec![manifest.join("Cargo.toml"), manifest.join("src/main.rs")]
}

fn produce(args: &Args) {
    args.deny_unknown(&[
        "out-dir",
        "seed",
        "count",
        "workers",
        "high-sys-threshold",
        "threshold-definition",
        "smoke-summary",
    ]);
    let total_started = Instant::now();
    let usage_started = usage();
    let out_dir = args.required_path("out-dir");
    let seed = args.u64("seed", DEFAULT_SEED);
    let count = args.usize("count", DEFAULT_COUNT);
    let workers = args.usize("workers", 12);
    let high_sys_threshold = args.f64("high-sys-threshold", f64::NAN);
    let threshold_definition = args
        .values
        .get("threshold-definition")
        .cloned()
        .unwrap_or_default();
    let smoke_summary_path = args.required_path("smoke-summary");
    assert_eq!(
        count, DEFAULT_COUNT,
        "scientific stage is exactly 10,000 accepted candidates"
    );
    assert!((1..=12).contains(&workers), "workers must be 1..=12");
    assert!(
        high_sys_threshold.is_finite() && high_sys_threshold > 0.0,
        "high-sys threshold must be frozen before production"
    );
    assert!(
        !threshold_definition.is_empty(),
        "threshold definition is required"
    );
    let smoke_value: serde_json::Value =
        serde_json::from_reader(File::open(&smoke_summary_path).expect("open smoke summary"))
            .expect("parse smoke summary");
    assert_eq!(
        smoke_value["pass_for_stage_one"], true,
        "smoke summary does not pass stage-one gate"
    );
    assert_eq!(
        smoke_value["bottom_one_percent"]["recall"], 1.0,
        "smoke f64/rational-reference bottom-1% membership is unstable"
    );
    rayon::ThreadPoolBuilder::new()
        .num_threads(workers)
        .build_global()
        .expect("build rayon pool");

    let generation_started = Instant::now();
    let candidates = (0..count)
        .into_par_iter()
        .map(|index| generate_candidate(seed, index).row)
        .collect::<Vec<_>>();
    assert_eq!(candidates.len(), count);
    assert_eq!(
        candidates
            .iter()
            .map(|row| row.candidate_id.as_str())
            .collect::<BTreeSet<_>>()
            .len(),
        count,
        "duplicate candidate id"
    );
    assert_eq!(
        candidates
            .iter()
            .map(|row| row.poly_id.as_str())
            .collect::<BTreeSet<_>>()
            .len(),
        count,
        "duplicate polytope id"
    );

    let mut f64_order = (0..count).collect::<Vec<_>>();
    f64_order.sort_by(|&left, &right| {
        compare_proxy_id(
            candidates[left].f64_proxy,
            &candidates[left].candidate_id,
            candidates[right].f64_proxy,
            &candidates[right].candidate_id,
        )
    });
    let f64_rank = f64_order
        .iter()
        .enumerate()
        .map(|(rank, &index)| (index, rank + 1))
        .collect::<HashMap<_, _>>();
    let generation_and_f64_ranking_wall_ms = generation_started.elapsed().as_secs_f64() * 1000.0;
    let selected_count = ceil_fraction(count, SELECTED_FRACTION);
    let selected_indices = f64_order
        .iter()
        .take(selected_count)
        .copied()
        .collect::<Vec<_>>();
    let selected_set = selected_indices.iter().copied().collect::<BTreeSet<_>>();

    let mut baseline_pool = (0..count)
        .filter(|index| !selected_set.contains(index))
        .collect::<Vec<_>>();
    baseline_pool.sort_by(|&left, &right| {
        baseline_hash(&candidates[left].candidate_id)
            .cmp(&baseline_hash(&candidates[right].candidate_id))
    });
    let baseline_indices = baseline_pool
        .into_iter()
        .take(selected_count)
        .collect::<Vec<_>>();
    assert_eq!(baseline_indices.len(), selected_count);
    assert!(baseline_indices
        .iter()
        .all(|index| !selected_set.contains(index)));

    let panel_started = Instant::now();
    let panel_indices = selected_indices
        .iter()
        .chain(&baseline_indices)
        .copied()
        .collect::<Vec<_>>();
    let mut panel_rows = panel_indices
        .into_par_iter()
        .map(|index| {
            let generated = generate_candidate(seed, candidates[index].sample_index);
            assert_eq!(
                generated.row.poly_id, candidates[index].poly_id,
                "regenerated panel geometry changed"
            );
            let is_selected = selected_set.contains(&index);
            let rank = f64_rank[&index];
            let future_band = if is_selected && rank <= 10 {
                "0-.1%"
            } else if is_selected {
                ".1-1%"
            } else {
                "matched-baseline"
            };
            PanelGeometryRow {
                schema: "sys-datascience.generic-ridge-tail-stage1.panel-geometry.v1".to_string(),
                candidate_id: candidates[index].candidate_id.clone(),
                poly_id: candidates[index].poly_id.clone(),
                sample_index: candidates[index].sample_index,
                rejection_attempt: candidates[index].rejection_attempt,
                facet_count: FACET_COUNT,
                height_min: H_MIN,
                height_max: H_MAX,
                selection_ids: if is_selected {
                    vec![SELECTION_ID.to_string()]
                } else {
                    vec![]
                },
                baseline_ids: if is_selected {
                    vec![]
                } else {
                    vec![BASELINE_ID.to_string()]
                },
                evaluation_roles: vec![if is_selected {
                    "selected".to_string()
                } else {
                    "baseline".to_string()
                }],
                future_band: future_band.to_string(),
                proxy: PROXY_NAME.to_string(),
                proxy_value_f64: candidates[index].f64_proxy,
                f64_rank: rank,
                dual_vertices_rational: rational_vectors_to_strings(
                    &generated.polytope.dual_vertices,
                ),
                vertices_rational: rational_vectors_to_strings(&generated.polytope.vertices),
                stage_order: "selected_before_target_evaluation".to_string(),
            }
        })
        .collect::<Vec<_>>();
    panel_rows.sort_by(|left, right| {
        left.evaluation_roles
            .cmp(&right.evaluation_roles)
            .then_with(|| left.f64_rank.cmp(&right.f64_rank))
            .then_with(|| left.candidate_id.cmp(&right.candidate_id))
    });
    let panel_wall_ms = panel_started.elapsed().as_secs_f64() * 1000.0;

    let selection_rows = selected_indices
        .iter()
        .map(|&index| SelectionRow {
            candidate_id: candidates[index].candidate_id.clone(),
            poly_id: candidates[index].poly_id.clone(),
            sample_index: candidates[index].sample_index,
            rejection_attempt: candidates[index].rejection_attempt,
            ridge_count: candidates[index].ridge_count,
            ridge_symp_area_mean: candidates[index].ridge_symp_area_mean,
            f64_volume: candidates[index].f64_volume,
            f64_proxy: candidates[index].f64_proxy,
            f64_rank: f64_rank[&index],
            selected: true,
        })
        .collect::<Vec<_>>();

    let last_selected_index = f64_order[selected_count - 1];
    let first_excluded_index = f64_order[selected_count];
    let boundary = SelectionBoundary {
        selected_count,
        last_selected_candidate_id: candidates[last_selected_index].candidate_id.clone(),
        last_selected_f64_rank: selected_count,
        last_selected_f64_proxy: candidates[last_selected_index].f64_proxy,
        first_excluded_candidate_id: candidates[first_excluded_index].candidate_id.clone(),
        first_excluded_f64_rank: selected_count + 1,
        first_excluded_f64_proxy: candidates[first_excluded_index].f64_proxy,
        f64_proxy_gap: candidates[first_excluded_index].f64_proxy
            - candidates[last_selected_index].f64_proxy,
    };
    assert!(
        boundary.f64_proxy_gap >= 0.0,
        "invalid f64 selection boundary"
    );

    fs::create_dir_all(&out_dir).expect("create stage-one output directory");
    let selection_path = out_dir.join("selection.jsonl");
    let panel_path = out_dir.join("panel-geometries.jsonl");
    write_jsonl(&selection_path, &selection_rows);
    write_jsonl(&panel_path, &panel_rows);

    let candidate_population_hash = hash_candidate_rows(&candidates);
    let subset_count = DEFAULT_SUBSET_COUNT.min(count);
    let deterministic_subset_hash = hash_candidate_rows(&candidates[..subset_count]);
    let selected_ids = selected_indices
        .iter()
        .map(|index| candidates[*index].candidate_id.as_str())
        .collect::<Vec<_>>();
    let baseline_ids = baseline_indices
        .iter()
        .map(|index| candidates[*index].candidate_id.as_str())
        .collect::<Vec<_>>();
    let mut panel_ids = selected_ids
        .iter()
        .chain(&baseline_ids)
        .copied()
        .collect::<Vec<_>>();
    panel_ids.sort_unstable();
    let source_hashes = source_files()
        .iter()
        .map(|path| source_hash(path))
        .chain(std::iter::once(source_hash(&smoke_summary_path)))
        .collect::<Vec<_>>();
    let usage_ended = usage();
    let manifest = Manifest {
        schema: "sys-datascience.generic-ridge-tail-stage1.manifest.v2".to_string(),
        status: "frozen-target-free-stage-one".to_string(),
        generator: "SysLandscapePolytopeCache::sample_random with current datascience-random-generic per-sample ChaCha8 seed derivation".to_string(),
        seed,
        facet_count: FACET_COUNT,
        height_min: H_MIN,
        height_max: H_MAX,
        workers,
        proxy: PROXY_NAME.to_string(),
        selection_rule: "lowest 1% by f64 ridge-area mean divided by sqrt(volume_from_incidence_f64); ascending proxy, candidate_id tie-break".to_string(),
        production_volume_definition: "euclidean_polytopes::volume_from_incidence_f64 on exact-derived incidence; no rational-arithmetic production volume".to_string(),
        future_sys_volume_definition: "target evaluation computes capacity and sys together; sys uses this same f64 incidence volume".to_string(),
        baseline_rule: "100 disjoint candidates with smallest BLAKE3 hashes under generic-ridge-tail-stage1-baseline-v1 and selection id; single F=10 population is exactly bucket matched".to_string(),
        counts: ManifestCounts {
            accepted_candidates: count,
            production_rational_volume_evaluations: 0,
            selected: selected_count,
            baseline: baseline_indices.len(),
            panel_union: selected_count + baseline_indices.len(),
            future_band_zero_to_point_one_percent: 10,
            future_band_point_one_to_one_percent: selected_count - 10,
        },
        selection_boundary: boundary,
        candidate_population_hash,
        deterministic_subset_count: subset_count,
        deterministic_subset_hash,
        selected_hash: hash_ids(b"f64-selected-v1", selected_ids),
        baseline_hash: hash_ids(b"baseline-v1", baseline_ids),
        panel_hash: hash_ids(b"panel-v1", panel_ids),
        smoke_contract: "one-time retained generic F=10 audit measured rational-volume versus f64 relative error, sys impact, rank agreement, and cutoff membership; production and future sys use f64 incidence volume".to_string(),
        frozen_high_sys_threshold: high_sys_threshold,
        frozen_high_sys_threshold_definition: threshold_definition,
        source_hashes,
        timing: TimingSummary {
            generation_and_f64_ranking_wall_ms,
            panel_geometry_wall_ms: panel_wall_ms,
            total_wall_ms: total_started.elapsed().as_secs_f64() * 1000.0,
            process_user_cpu_seconds: usage_ended.user_seconds - usage_started.user_seconds,
            process_system_cpu_seconds: usage_ended.system_seconds - usage_started.system_seconds,
            max_rss_kib: usage_ended.max_rss_kib,
        },
        target_exposure: TargetExposure {
            capacity_computed_for_new_population: false,
            sys_computed_for_new_population: false,
            target_fields_present_in_stage_one_artifacts: false,
            statement: "The producer computed geometry, ridge features, and f64 incidence volume only. It did not read or compute capacity or sys for any new candidate; target evaluation remains the later joint capacity-plus-sys step.".to_string(),
        },
    };
    write_json(&out_dir.join("manifest.json"), &manifest);
    println!(
        "production complete: candidates={} selected={} baseline={} wall_s={:.3} cpu_s={:.3}",
        count,
        selected_count,
        baseline_indices.len(),
        manifest.timing.total_wall_ms / 1000.0,
        manifest.timing.process_user_cpu_seconds + manifest.timing.process_system_cpu_seconds
    );
}

fn validate(args: &Args) {
    args.deny_unknown(&["out-dir", "out"]);
    let out_dir = args.required_path("out-dir");
    let out = args.optional_path("out", out_dir.join("validation.json"));
    let manifest_path = out_dir.join("manifest.json");
    let selection_path = out_dir.join("selection.jsonl");
    let panel_path = out_dir.join("panel-geometries.jsonl");
    let manifest: Manifest =
        serde_json::from_reader(File::open(&manifest_path).expect("open manifest"))
            .expect("parse manifest");
    let selection = read_jsonl::<SelectionRow>(&selection_path);
    let panel = read_jsonl::<PanelGeometryRow>(&panel_path);
    let selected = panel
        .iter()
        .filter(|row| row.evaluation_roles == ["selected"])
        .collect::<Vec<_>>();
    let baseline = panel
        .iter()
        .filter(|row| row.evaluation_roles == ["baseline"])
        .collect::<Vec<_>>();
    let selected_ids = selected
        .iter()
        .map(|row| row.candidate_id.as_str())
        .collect::<BTreeSet<_>>();
    let selection_ids = selection
        .iter()
        .map(|row| row.candidate_id.as_str())
        .collect::<BTreeSet<_>>();
    let baseline_ids = baseline
        .iter()
        .map(|row| row.candidate_id.as_str())
        .collect::<BTreeSet<_>>();
    let subset = (0..manifest.deterministic_subset_count)
        .into_par_iter()
        .map(|index| generate_candidate(manifest.seed, index).row)
        .collect::<Vec<_>>();
    let subset_hash = hash_candidate_rows(&subset);

    let mut checks = BTreeMap::new();
    checks.insert(
        "candidate_count_exactly_10000".to_string(),
        manifest.counts.accepted_candidates == DEFAULT_COUNT,
    );
    checks.insert(
        "selection_count".to_string(),
        selection.len() == 100 && selected.len() == 100,
    );
    checks.insert(
        "selection_rank_contiguous".to_string(),
        selection
            .iter()
            .enumerate()
            .all(|(index, row)| row.f64_rank == index + 1 && row.selected),
    );
    checks.insert(
        "selection_proxy_ordered".to_string(),
        selection.windows(2).all(|pair| {
            compare_proxy_id(
                pair[0].f64_proxy,
                &pair[0].candidate_id,
                pair[1].f64_proxy,
                &pair[1].candidate_id,
            ) != Ordering::Greater
        }),
    );
    checks.insert(
        "selection_matches_panel".to_string(),
        selection_ids == selected_ids,
    );
    checks.insert("baseline_count".to_string(), baseline.len() == 100);
    checks.insert(
        "panel_disjoint".to_string(),
        selected_ids.is_disjoint(&baseline_ids),
    );
    checks.insert(
        "future_bands_disjoint_and_complete".to_string(),
        selected
            .iter()
            .filter(|row| row.future_band == "0-.1%")
            .count()
            == 10
            && selected
                .iter()
                .filter(|row| row.future_band == ".1-1%")
                .count()
                == 90,
    );
    checks.insert(
        "production_no_rational_volume".to_string(),
        manifest.counts.production_rational_volume_evaluations == 0,
    );
    checks.insert(
        "deterministic_subset".to_string(),
        subset_hash == manifest.deterministic_subset_hash,
    );
    checks.insert(
        "target_flags_false".to_string(),
        !manifest
            .target_exposure
            .capacity_computed_for_new_population
            && !manifest.target_exposure.sys_computed_for_new_population
            && !manifest
                .target_exposure
                .target_fields_present_in_stage_one_artifacts,
    );
    let forbidden = [
        "\"sys\"",
        "\"capacity\"",
        "time_capacity",
        "sigma",
        "orbit_scalars",
    ];
    let target_tokens_absent = [&manifest_path, &selection_path, &panel_path]
        .iter()
        .all(|path| {
            let text = fs::read_to_string(path).expect("read artifact for target-token audit");
            forbidden.iter().all(|token| !text.contains(token))
        });
    checks.insert(
        "target_field_tokens_absent".to_string(),
        target_tokens_absent,
    );
    let artifact_paths = [&manifest_path, &selection_path, &panel_path];
    let artifact_hashes = artifact_paths
        .iter()
        .map(|path| source_hash(path))
        .collect::<Vec<_>>();
    let artifact_bytes = artifact_paths
        .iter()
        .map(|path| {
            (
                path.file_name().unwrap().to_string_lossy().to_string(),
                fs::metadata(path).expect("artifact metadata").len(),
            )
        })
        .collect::<BTreeMap<_, _>>();
    let valid = checks.values().all(|value| *value);
    let summary = ValidationSummary {
        schema: "sys-datascience.generic-ridge-tail-stage1.validation.v2",
        valid,
        checks,
        artifact_hashes,
        artifact_bytes,
        deterministic_subset_hash: subset_hash,
        target_field_tokens_absent: target_tokens_absent,
    };
    assert!(valid, "stage-one validation failed: {:?}", summary.checks);
    write_json(&out, &summary);
    println!("validation passed: {} checks", summary.checks.len());
}

fn main() {
    let args = Args::parse();
    match args.command.as_str() {
        "smoke" => smoke(&args),
        "produce" => produce(&args),
        "validate" => validate(&args),
        command => panic!("unknown command {command}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ceil_fraction_contract() {
        assert_eq!(ceil_fraction(512, 0.01), 6);
        assert_eq!(ceil_fraction(10_000, 0.01), 100);
        assert_eq!(ceil_fraction(10_000, 0.02), 200);
    }

    #[test]
    fn candidate_generation_is_deterministic() {
        let first = generate_candidate(12345, 7).row;
        let second = generate_candidate(12345, 7).row;
        assert_eq!(
            serde_json::to_string(&first).unwrap(),
            serde_json::to_string(&second).unwrap()
        );
    }

    #[test]
    fn baseline_order_is_deterministic_and_selection_specific() {
        let mut ids = ["a", "b", "c", "d"];
        ids.sort_by_key(|id| baseline_hash(id));
        let first = ids;
        ids.sort_by_key(|id| baseline_hash(id));
        assert_eq!(first, ids);
        assert_eq!(first.len(), 4);
    }

    #[test]
    fn proxy_tie_breaks_by_candidate_id() {
        assert_eq!(compare_proxy_id(1.0, "a", 1.0, "b"), Ordering::Less);
        assert_eq!(compare_proxy_id(1.0, "b", 1.0, "a"), Ordering::Greater);
    }
}

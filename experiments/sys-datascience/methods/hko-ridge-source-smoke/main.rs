//! Source-reproducible HKO ridge-area smoke packet.
//!
//! This binary owns a narrow sys-datascience method packet. It generates HKO
//! plus deterministic f64 dual-vertex perturbations from tracked code, computes
//! volume/capacity/sys through the sys-landscape cache API, and computes
//! ridge-area feature columns through the shared sys-datascience feature helper.

#[path = "../../../polytope-invariant-table/features_face_symplectic.rs"]
mod features_face_symplectic;
#[path = "../../../polytope-invariant-table/features_helpers.rs"]
mod features_helpers;

use euclidean_polytopes::two_faces_from_vertex_facet_incidence;
use exp_sys_landscape::{
    poly_id, CapacityBackend, ComputedPolytopeCache, ComputedPolytopePayloadRow,
    SysLandscapePolytopeCache,
};
use nalgebra::Vector4;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use serde::Serialize;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use symplectic::geom::known_polytopes;

const DEFAULT_OUT_DIR: &str =
    "experiments/sys-datascience/methods/hko-ridge-source-smoke/artifacts";
const DEFAULT_SEED: u64 = 42;
const DEFAULT_EPSILON: f64 = 0.01;
const DEFAULT_PERTURBED_COUNT: usize = 8;
const MAX_ATTEMPTS_PER_ROW: usize = 10_000;

const SCALAR_SIGNAL_FEATURES: [&str; 6] = [
    "ridge_symp_area_sum_over_volume_sqrt",
    "ridge_symp_area_max_over_volume_sqrt",
    "ridge_symp_area_std_over_volume_sqrt",
    "ridge_symp_area_q95_over_volume_sqrt",
    "ridge_symp_area_q90_over_volume_sqrt",
    "ridge_symp_area_mean_over_volume_sqrt",
];

struct Args {
    out_dir: PathBuf,
    seed: u64,
    epsilon: f64,
    perturbed_count: usize,
}

#[derive(Clone)]
struct SourceSample {
    name: String,
    sample_index: usize,
    is_base: bool,
    attempt: usize,
    epsilon: f64,
    dual_vertices: Vec<Vector4<f64>>,
    delta_dual_vertices: Vec<[f64; 4]>,
    polytope: SysLandscapePolytopeCache,
    payload: ComputedPolytopePayloadRow,
}

#[derive(Clone, Serialize)]
struct RidgeAreaFeatureRow {
    name: String,
    sample_index: usize,
    is_base: bool,
    attempt: usize,
    epsilon: f64,
    poly_id: String,
    facet_count: usize,
    source: String,
    dual_vertices: Vec<[f64; 4]>,
    delta_dual_vertices: Vec<[f64; 4]>,
    volume: f64,
    volume_sqrt: f64,
    capacity: f64,
    sys: f64,
    delta_sys_from_hko: f64,
    backend: String,
    ridge_symp_area_ordered_face_count: usize,
    ridge_symp_area_ordering_failure_count: usize,
    ridge_symp_area_ordered_fraction: f64,
    ridge_symp_area_mean_over_volume_sqrt: f64,
    ridge_symp_area_std_over_volume_sqrt: f64,
    ridge_symp_area_min_over_volume_sqrt: f64,
    ridge_symp_area_max_over_volume_sqrt: f64,
    ridge_symp_area_q25_over_volume_sqrt: f64,
    ridge_symp_area_median_over_volume_sqrt: f64,
    ridge_symp_area_q75_over_volume_sqrt: f64,
    ridge_symp_area_q90_over_volume_sqrt: f64,
    ridge_symp_area_q95_over_volume_sqrt: f64,
    ridge_symp_area_sum_over_volume_sqrt: f64,
    ridge_symp_area_max_share: f64,
    ridge_symp_area_top3_share: f64,
    ridge_symp_area_le_1em3_over_volume_sqrt_fraction: f64,
    ridge_symp_area_le_1em2_over_volume_sqrt_fraction: f64,
    ridge_symp_area_le_1em1_over_volume_sqrt_fraction: f64,
    ridge_symp_area_entropy: f64,
    ridge_symp_area_effective_face_count: f64,
    ridge_symp_area_normalized_entropy: f64,
    ridge_signal_all_magnitude_features_above_hko: bool,
}

#[derive(Serialize)]
struct FeatureSummary {
    feature: String,
    hko_value: f64,
    perturbed_mean: f64,
    perturbed_min: f64,
    perturbed_max: f64,
    perturbed_increase_fraction: f64,
    perturbed_decrease_fraction: f64,
}

#[derive(Serialize)]
struct Summary {
    source_status: &'static str,
    row_count: usize,
    perturbed_count: usize,
    seed: u64,
    epsilon: f64,
    max_attempts_per_row: usize,
    scalar_signal_features: Vec<&'static str>,
    hko: BTreeMap<String, f64>,
    sys: BTreeMap<String, f64>,
    ridge_area_features: Vec<FeatureSummary>,
    joint_signal: BTreeMap<String, f64>,
    provenance: Vec<&'static str>,
    caveats: Vec<&'static str>,
}

fn parse_args() -> Args {
    let mut out_dir = PathBuf::from(DEFAULT_OUT_DIR);
    let mut seed = DEFAULT_SEED;
    let mut epsilon = DEFAULT_EPSILON;
    let mut perturbed_count = DEFAULT_PERTURBED_COUNT;
    let argv = std::env::args().collect::<Vec<_>>();
    let mut index = 1usize;
    while index < argv.len() {
        let value = |index: usize, flag: &str| -> &str {
            argv.get(index + 1)
                .map(String::as_str)
                .unwrap_or_else(|| panic!("{flag} requires a value"))
        };
        match argv[index].as_str() {
            "--out-dir" => {
                out_dir = PathBuf::from(value(index, "--out-dir"));
                index += 2;
            }
            "--seed" => {
                seed = value(index, "--seed").parse().expect("--seed must be u64");
                index += 2;
            }
            "--epsilon" => {
                epsilon = value(index, "--epsilon")
                    .parse()
                    .expect("--epsilon must be f64");
                index += 2;
            }
            "--perturbed-count" => {
                perturbed_count = value(index, "--perturbed-count")
                    .parse()
                    .expect("--perturbed-count must be usize");
                index += 2;
            }
            "--help" | "-h" => {
                print_usage();
                std::process::exit(0);
            }
            other => panic!("unknown argument {other}"),
        }
    }
    assert!(
        epsilon > 0.0 && epsilon.is_finite(),
        "epsilon must be positive"
    );
    assert!(perturbed_count > 0, "perturbed-count must be positive");
    Args {
        out_dir,
        seed,
        epsilon,
        perturbed_count,
    }
}

fn print_usage() {
    println!(
        "Usage: sys-datascience-hko-ridge-source-smoke [--out-dir PATH] [--seed U64] [--epsilon F64] [--perturbed-count N]"
    );
}

fn arrays_from_vectors(vertices: &[Vector4<f64>]) -> Vec<[f64; 4]> {
    vertices
        .iter()
        .map(|vertex| [vertex[0], vertex[1], vertex[2], vertex[3]])
        .collect()
}

fn zero_deltas(len: usize) -> Vec<[f64; 4]> {
    vec![[0.0, 0.0, 0.0, 0.0]; len]
}

fn perturb(base: &[Vector4<f64>], rng: &mut ChaCha8Rng, epsilon: f64) -> Vec<Vector4<f64>> {
    base.iter()
        .map(|vertex| {
            Vector4::new(
                vertex[0] + rng.gen_range(-epsilon..=epsilon),
                vertex[1] + rng.gen_range(-epsilon..=epsilon),
                vertex[2] + rng.gen_range(-epsilon..=epsilon),
                vertex[3] + rng.gen_range(-epsilon..=epsilon),
            )
        })
        .collect()
}

fn deltas(base: &[Vector4<f64>], perturbed: &[Vector4<f64>]) -> Vec<[f64; 4]> {
    base.iter()
        .zip(perturbed)
        .map(|(left, right)| {
            [
                right[0] - left[0],
                right[1] - left[1],
                right[2] - left[2],
                right[3] - left[3],
            ]
        })
        .collect()
}

fn compute_payload(
    polytope: &SysLandscapePolytopeCache,
    cache: &ComputedPolytopeCache,
) -> ComputedPolytopePayloadRow {
    cache
        .compute(polytope, CapacityBackend::Auto)
        .expect("volume/capacity/sys computation should succeed")
}

fn source_samples(args: &Args) -> Vec<SourceSample> {
    let fixture = known_polytopes::hko_pentagon();
    let base_polytope = SysLandscapePolytopeCache::from_rational_parts(
        fixture.dual_vertices.clone(),
        fixture.vertices.clone(),
    )
    .expect("HKO fixture should build sys-landscape cache");
    let base_duals = base_polytope.dual_vertices_f64.clone();
    let cache = ComputedPolytopeCache::load(&[]);
    let base_payload = compute_payload(&base_polytope, &cache);

    let mut rows = vec![SourceSample {
        name: "hko_pentagon_base".to_string(),
        sample_index: 0,
        is_base: true,
        attempt: 0,
        epsilon: args.epsilon,
        dual_vertices: base_duals.clone(),
        delta_dual_vertices: zero_deltas(base_duals.len()),
        polytope: base_polytope,
        payload: base_payload,
    }];

    let mut rng = ChaCha8Rng::seed_from_u64(args.seed);
    for sample_index in 1..=args.perturbed_count {
        let mut accepted = None;
        for attempt in 1..=MAX_ATTEMPTS_PER_ROW {
            let perturbed_duals = perturb(&base_duals, &mut rng, args.epsilon);
            let Some(polytope) =
                SysLandscapePolytopeCache::from_f64_dual_vertices(perturbed_duals.clone())
            else {
                continue;
            };
            let payload = compute_payload(&polytope, &cache);
            accepted = Some(SourceSample {
                name: format!("hko_pentagon_perturbed_{}", sample_index - 1),
                sample_index,
                is_base: false,
                attempt,
                epsilon: args.epsilon,
                delta_dual_vertices: deltas(&base_duals, &perturbed_duals),
                dual_vertices: perturbed_duals,
                polytope,
                payload,
            });
            break;
        }
        rows.push(accepted.unwrap_or_else(|| {
            panic!(
                "failed to generate accepted perturbation {sample_index} within {MAX_ATTEMPTS_PER_ROW} attempts"
            )
        }));
    }
    rows
}

fn feature_value(row: &RidgeAreaFeatureRow, feature: &str) -> f64 {
    match feature {
        "ridge_symp_area_sum_over_volume_sqrt" => row.ridge_symp_area_sum_over_volume_sqrt,
        "ridge_symp_area_max_over_volume_sqrt" => row.ridge_symp_area_max_over_volume_sqrt,
        "ridge_symp_area_std_over_volume_sqrt" => row.ridge_symp_area_std_over_volume_sqrt,
        "ridge_symp_area_q95_over_volume_sqrt" => row.ridge_symp_area_q95_over_volume_sqrt,
        "ridge_symp_area_q90_over_volume_sqrt" => row.ridge_symp_area_q90_over_volume_sqrt,
        "ridge_symp_area_mean_over_volume_sqrt" => row.ridge_symp_area_mean_over_volume_sqrt,
        _ => panic!("unknown ridge-area feature {feature}"),
    }
}

fn row_from_sample(
    sample: &SourceSample,
    base: Option<&RidgeAreaFeatureRow>,
) -> RidgeAreaFeatureRow {
    let volume_sqrt = sample.payload.volume.sqrt();
    let two_faces = two_faces_from_vertex_facet_incidence(&sample.polytope.vertex_facet_incidence);
    let fields = features_face_symplectic::compute_face_symplectic_fields(
        &two_faces,
        &sample.polytope.vertices_f64,
        &sample.polytope.vertex_facet_incidence,
        volume_sqrt,
    );
    let mut row = RidgeAreaFeatureRow {
        name: sample.name.clone(),
        sample_index: sample.sample_index,
        is_base: sample.is_base,
        attempt: sample.attempt,
        epsilon: sample.epsilon,
        poly_id: poly_id(&sample.polytope),
        facet_count: sample.payload.facet_count,
        source: if sample.is_base {
            "known_polytopes::hko_pentagon".to_string()
        } else {
            "deterministic_chacha8_uniform_dual_vertex_perturbation".to_string()
        },
        dual_vertices: arrays_from_vectors(&sample.dual_vertices),
        delta_dual_vertices: sample.delta_dual_vertices.clone(),
        volume: sample.payload.volume,
        volume_sqrt,
        capacity: sample.payload.capacity,
        sys: sample.payload.sys,
        delta_sys_from_hko: 0.0,
        backend: sample.payload.backend.clone(),
        ridge_symp_area_ordered_face_count: fields.ridge_symp_area_ordered_face_count,
        ridge_symp_area_ordering_failure_count: fields.ridge_symp_area_ordering_failure_count,
        ridge_symp_area_ordered_fraction: fields.ridge_symp_area_ordered_fraction,
        ridge_symp_area_mean_over_volume_sqrt: fields.ridge_symp_area_mean / volume_sqrt,
        ridge_symp_area_std_over_volume_sqrt: fields.ridge_symp_area_std / volume_sqrt,
        ridge_symp_area_min_over_volume_sqrt: fields.ridge_symp_area_min / volume_sqrt,
        ridge_symp_area_max_over_volume_sqrt: fields.ridge_symp_area_max / volume_sqrt,
        ridge_symp_area_q25_over_volume_sqrt: fields.ridge_symp_area_q25 / volume_sqrt,
        ridge_symp_area_median_over_volume_sqrt: fields.ridge_symp_area_median / volume_sqrt,
        ridge_symp_area_q75_over_volume_sqrt: fields.ridge_symp_area_q75 / volume_sqrt,
        ridge_symp_area_q90_over_volume_sqrt: fields.ridge_symp_area_q90 / volume_sqrt,
        ridge_symp_area_q95_over_volume_sqrt: fields.ridge_symp_area_q95 / volume_sqrt,
        ridge_symp_area_sum_over_volume_sqrt: fields.ridge_symp_area_sum / volume_sqrt,
        ridge_symp_area_max_share: fields.ridge_symp_area_max_share,
        ridge_symp_area_top3_share: fields.ridge_symp_area_top3_share,
        ridge_symp_area_le_1em3_over_volume_sqrt_fraction: fields
            .ridge_symp_area_le_1em3_over_volume_sqrt_fraction,
        ridge_symp_area_le_1em2_over_volume_sqrt_fraction: fields
            .ridge_symp_area_le_1em2_over_volume_sqrt_fraction,
        ridge_symp_area_le_1em1_over_volume_sqrt_fraction: fields
            .ridge_symp_area_le_1em1_over_volume_sqrt_fraction,
        ridge_symp_area_entropy: fields.ridge_symp_area_entropy,
        ridge_symp_area_effective_face_count: fields.ridge_symp_area_effective_face_count,
        ridge_symp_area_normalized_entropy: fields.ridge_symp_area_normalized_entropy,
        ridge_signal_all_magnitude_features_above_hko: false,
    };

    if let Some(base) = base {
        row.delta_sys_from_hko = row.sys - base.sys;
        row.ridge_signal_all_magnitude_features_above_hko = SCALAR_SIGNAL_FEATURES
            .iter()
            .all(|feature| feature_value(&row, feature) > feature_value(base, feature));
    }
    row
}

fn feature_rows(samples: &[SourceSample]) -> Vec<RidgeAreaFeatureRow> {
    let base = row_from_sample(&samples[0], None);
    let mut rows = vec![base.clone()];
    rows.extend(
        samples[1..]
            .iter()
            .map(|sample| row_from_sample(sample, Some(&base))),
    );
    rows
}

fn mean(values: &[f64]) -> f64 {
    if values.is_empty() {
        0.0
    } else {
        values.iter().sum::<f64>() / values.len() as f64
    }
}

fn min(values: &[f64]) -> f64 {
    values.iter().copied().fold(f64::INFINITY, f64::min)
}

fn max(values: &[f64]) -> f64 {
    values.iter().copied().fold(f64::NEG_INFINITY, f64::max)
}

fn fraction(values: &[bool]) -> f64 {
    if values.is_empty() {
        0.0
    } else {
        values.iter().filter(|&&value| value).count() as f64 / values.len() as f64
    }
}

fn summarize(args: &Args, rows: &[RidgeAreaFeatureRow]) -> Summary {
    let base = rows.iter().find(|row| row.is_base).expect("base row");
    let perturbed = rows.iter().filter(|row| !row.is_base).collect::<Vec<_>>();
    let sys_values = perturbed.iter().map(|row| row.sys).collect::<Vec<_>>();
    let sys_decreased = perturbed
        .iter()
        .map(|row| row.sys < base.sys)
        .collect::<Vec<_>>();
    let all_signal_features_above_hko = perturbed
        .iter()
        .map(|row| row.ridge_signal_all_magnitude_features_above_hko)
        .collect::<Vec<_>>();
    let joint_signal_rows = perturbed
        .iter()
        .map(|row| row.sys < base.sys && row.ridge_signal_all_magnitude_features_above_hko)
        .collect::<Vec<_>>();
    let mut hko = BTreeMap::new();
    hko.insert("sys".to_string(), base.sys);
    hko.insert(
        "ridge_symp_area_sum_over_volume_sqrt".to_string(),
        base.ridge_symp_area_sum_over_volume_sqrt,
    );
    hko.insert("volume".to_string(), base.volume);
    hko.insert("capacity".to_string(), base.capacity);

    let mut sys = BTreeMap::new();
    sys.insert("hko_sys".to_string(), base.sys);
    sys.insert("perturbed_mean".to_string(), mean(&sys_values));
    sys.insert("perturbed_min".to_string(), min(&sys_values));
    sys.insert("perturbed_max".to_string(), max(&sys_values));
    sys.insert(
        "perturbed_decrease_fraction".to_string(),
        fraction(&sys_decreased),
    );
    sys.insert(
        "perturbed_increase_fraction".to_string(),
        fraction(
            &perturbed
                .iter()
                .map(|row| row.sys > base.sys)
                .collect::<Vec<_>>(),
        ),
    );

    let ridge_area_features = SCALAR_SIGNAL_FEATURES
        .iter()
        .map(|feature| {
            let base_value = feature_value(base, feature);
            let values = perturbed
                .iter()
                .map(|row| feature_value(row, feature))
                .collect::<Vec<_>>();
            FeatureSummary {
                feature: feature.to_string(),
                hko_value: base_value,
                perturbed_mean: mean(&values),
                perturbed_min: min(&values),
                perturbed_max: max(&values),
                perturbed_increase_fraction: fraction(
                    &values
                        .iter()
                        .map(|value| *value > base_value)
                        .collect::<Vec<_>>(),
                ),
                perturbed_decrease_fraction: fraction(
                    &values
                        .iter()
                        .map(|value| *value < base_value)
                        .collect::<Vec<_>>(),
                ),
            }
        })
        .collect::<Vec<_>>();

    let mut joint_signal = BTreeMap::new();
    joint_signal.insert(
        "sys_decreased_fraction".to_string(),
        fraction(&sys_decreased),
    );
    joint_signal.insert(
        "all_scalar_signal_features_above_hko_fraction".to_string(),
        fraction(&all_signal_features_above_hko),
    );
    joint_signal.insert(
        "sys_decreased_and_all_scalar_signal_features_above_hko_fraction".to_string(),
        fraction(&joint_signal_rows),
    );

    Summary {
        source_status: "source-rewritten",
        row_count: rows.len(),
        perturbed_count: perturbed.len(),
        seed: args.seed,
        epsilon: args.epsilon,
        max_attempts_per_row: MAX_ATTEMPTS_PER_ROW,
        scalar_signal_features: SCALAR_SIGNAL_FEATURES.to_vec(),
        hko,
        sys,
        ridge_area_features,
        joint_signal,
        provenance: vec![
            "Base row is generated from symplectic::geom::known_polytopes::hko_pentagon.",
            "Perturbations are generated by tracked code with ChaCha8 seed, epsilon, and accepted sample count recorded in this summary.",
            "Volume, capacity, and sys are recomputed by exp-sys-landscape; no generated ridge rows from hko-ridge-area-packet are inputs.",
            "Ridge-area columns call experiments/polytope-invariant-table/features_face_symplectic.rs.",
        ],
        caveats: vec![
            "Empirical smoke sample only; not a proof of a local maximum or minimum.",
            "The perturbation model is direct f64 normalized-dual coordinate noise with rejection for invalid polytopes.",
            "No quotient by finite HKO symmetries or perturbation-direction equivalence is applied.",
        ],
    }
}

fn write_jsonl<T: Serialize>(path: &Path, rows: &[T]) {
    let file = File::create(path).unwrap_or_else(|err| panic!("create {}: {err}", path.display()));
    let mut writer = BufWriter::new(file);
    for row in rows {
        serde_json::to_writer(&mut writer, row).expect("serialize row");
        writeln!(&mut writer).expect("write newline");
    }
}

fn write_summary(path: &Path, summary: &Summary) {
    let file = File::create(path).unwrap_or_else(|err| panic!("create {}: {err}", path.display()));
    serde_json::to_writer_pretty(file, summary).expect("write summary");
}

fn write_report(path: &Path, summary: &Summary) {
    let mut file =
        File::create(path).unwrap_or_else(|err| panic!("create {}: {err}", path.display()));
    writeln!(file, "# HKO ridge source smoke\n").unwrap();
    writeln!(
        file,
        "Status: `{}`. This packet regenerates HKO and deterministic perturbation rows from tracked source code.\n",
        summary.source_status
    )
    .unwrap();
    writeln!(file, "## Inputs\n").unwrap();
    writeln!(file, "- seed: `{}`", summary.seed).unwrap();
    writeln!(file, "- epsilon: `{}`", summary.epsilon).unwrap();
    writeln!(file, "- rows: `{}`", summary.row_count).unwrap();
    writeln!(file, "- perturbed rows: `{}`\n", summary.perturbed_count).unwrap();
    writeln!(file, "## Result\n").unwrap();
    writeln!(file, "- HKO `sys`: `{}`", summary.sys["hko_sys"]).unwrap();
    writeln!(
        file,
        "- perturbed `sys` mean/min/max: `{}` / `{}` / `{}`",
        summary.sys["perturbed_mean"], summary.sys["perturbed_min"], summary.sys["perturbed_max"]
    )
    .unwrap();
    writeln!(
        file,
        "- fraction of perturbed rows with lower `sys` than HKO: `{}`",
        summary.sys["perturbed_decrease_fraction"]
    )
    .unwrap();
    writeln!(
        file,
        "- fraction with all selected ridge-area magnitude features above HKO: `{}`",
        summary.joint_signal["all_scalar_signal_features_above_hko_fraction"]
    )
    .unwrap();
    writeln!(
        file,
        "- fraction satisfying both conditions: `{}`\n",
        summary.joint_signal["sys_decreased_and_all_scalar_signal_features_above_hko_fraction"]
    )
    .unwrap();
    writeln!(file, "## Ridge-Area Features\n").unwrap();
    writeln!(
        file,
        "| feature | HKO | perturbed mean | min | max | increase fraction |"
    )
    .unwrap();
    writeln!(file, "| --- | ---: | ---: | ---: | ---: | ---: |").unwrap();
    for feature in &summary.ridge_area_features {
        writeln!(
            file,
            "| `{}` | {} | {} | {} | {} | {} |",
            feature.feature,
            feature.hko_value,
            feature.perturbed_mean,
            feature.perturbed_min,
            feature.perturbed_max,
            feature.perturbed_increase_fraction
        )
        .unwrap();
    }
    writeln!(file, "\n## Provenance\n").unwrap();
    for item in &summary.provenance {
        writeln!(file, "- {item}").unwrap();
    }
    writeln!(file, "\n## Caveats\n").unwrap();
    for caveat in &summary.caveats {
        writeln!(file, "- {caveat}").unwrap();
    }
}

fn main() {
    let args = parse_args();
    std::fs::create_dir_all(&args.out_dir)
        .unwrap_or_else(|err| panic!("create {}: {err}", args.out_dir.display()));
    let samples = source_samples(&args);
    let rows = feature_rows(&samples);
    let summary = summarize(&args, &rows);

    let rows_path = args.out_dir.join("ridge-area-rows.jsonl");
    let summary_path = args.out_dir.join("summary.json");
    let report_path = args.out_dir.join("report.md");
    write_jsonl(&rows_path, &rows);
    write_summary(&summary_path, &summary);
    write_report(&report_path, &summary);

    let lower_fraction = summary.sys["perturbed_decrease_fraction"];
    let joint_fraction =
        summary.joint_signal["sys_decreased_and_all_scalar_signal_features_above_hko_fraction"];
    println!("# hko-ridge-source-smoke");
    println!("- status: {}", summary.source_status);
    println!("- rows: {}", summary.row_count);
    println!("- sys lower than HKO fraction: {lower_fraction}");
    println!("- joint signal fraction: {joint_fraction}");
    println!("Wrote {}", args.out_dir.display());
}

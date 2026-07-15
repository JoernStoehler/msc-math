use exp_generator_sys_orbit_view::{
    canonicalize, standard_symplectic_matrix, unordered_row_assignment_rms, SectionDiagnostics,
    SectionOutput,
};
use nalgebra::{Matrix4, Vector4};
use rand::seq::SliceRandom;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::BTreeMap;
use std::env;
use std::fs::{create_dir_all, read, File};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Instant;

const ROW_SCHEMA: &str = "generator-sys-orbit-view-row-v1";
const REPORT_SCHEMA: &str = "generator-sys-orbit-view-report-v1";
const INPUT_SCHEMA: &str = "generator-orientation-smoke-row-v2";
const CONTROL_SEED: u64 = 20_260_715;
const SECTION_TOLERANCE: f64 = 1e-5;
const IDENTITY_TOLERANCE: f64 = 1e-12;
const SO4_NONZERO_THRESHOLD: f64 = 1e-5;

const FORMAL_PATH: &str = "formal/generic-coordinate-canonization.tex";
const FROZEN_CANDIDATE_PATH: &str =
    "experiments/dev-canonization-t-search/src/candidates/volume_one_omega_labeled_symplectic_frame.rs";
const FROZEN_README_PATH: &str = "experiments/dev-canonization-t-search/README.md";
const ORIENTATION_REPORT_PATH: &str =
    "experiments/sys-datascience/methods/generator-orientation-smoke/artifacts/panel-2-per-bucket/report.json";
const OWN_LIB_PATH: &str =
    "experiments/sys-datascience/methods/generator-sys-orbit-view/src/lib.rs";
const OWN_MAIN_PATH: &str =
    "experiments/sys-datascience/methods/generator-sys-orbit-view/src/main.rs";
const OWN_MANIFEST_PATH: &str =
    "experiments/sys-datascience/methods/generator-sys-orbit-view/Cargo.toml";
const OWN_README_PATH: &str =
    "experiments/sys-datascience/methods/generator-sys-orbit-view/README.md";

#[derive(Clone, Debug)]
struct Args {
    input: PathBuf,
    out_dir: PathBuf,
}

#[derive(Clone, Debug, Deserialize)]
struct OrientationRow {
    schema: String,
    sample_id: String,
    base_id: String,
    bucket: String,
    map_variant: String,
    map_family: String,
    map_mode: String,
    reconstruction_status: String,
    transformed_dual_vertices_f64: Vec<[f64; 4]>,
    reconstruction_ms: f64,
}

impl OrientationRow {
    fn duals(&self) -> Vec<Vector4<f64>> {
        self.transformed_dual_vertices_f64
            .iter()
            .map(|row| Vector4::from_row_slice(row))
            .collect()
    }
}

#[derive(Clone, Debug, Serialize)]
struct SectionRecord {
    status: String,
    diagnostics: SectionDiagnostics,
}

impl From<&SectionOutput> for SectionRecord {
    fn from(output: &SectionOutput) -> Self {
        Self {
            status: output.status.to_string(),
            diagnostics: output.diagnostics.clone(),
        }
    }
}

#[derive(Clone, Debug, Serialize)]
struct AdapterRow {
    schema: &'static str,
    comparison_id: String,
    source_kind: &'static str,
    base_id: String,
    transformed_id: String,
    bucket: Option<String>,
    map_variant: String,
    map_family: String,
    map_mode: String,
    seed: Option<u64>,
    attempt: Option<usize>,
    expectation: &'static str,
    base_section: SectionRecord,
    transformed_section: SectionRecord,
    raw_coordinate_unordered_row_assignment_rms: Option<f64>,
    generic_sys_section_unordered_row_assignment_rms: Option<f64>,
    section_distance_tolerance: Option<f64>,
    control_passed: Option<bool>,
    transform_parameters: serde_json::Value,
    interpretation: &'static str,
}

#[derive(Clone, Debug, Default, Serialize)]
struct DistanceStats {
    count: usize,
    mean: Option<f64>,
    max: Option<f64>,
    positive_above_so4_threshold: usize,
}

#[derive(Clone, Debug, Default, Serialize)]
struct PairSummary {
    pair_count: usize,
    generic_success_pair_count: usize,
    base_non_success_count: usize,
    transformed_non_success_count: usize,
    evaluated_control_count: usize,
    passed_control_count: usize,
    raw_coordinate_distance: DistanceStats,
    generic_sys_section_distance: DistanceStats,
}

#[derive(Clone, Debug, Default, Serialize)]
struct ResidualSummary {
    evaluation_count: usize,
    success_count: usize,
    status_counts: BTreeMap<String, usize>,
    max_volume_one_algebraic_residual: Option<f64>,
    max_analytic_center_gradient_norm: Option<f64>,
    max_analytic_center_newton_decrement: Option<f64>,
    max_frame_symplectic_defect_frobenius: Option<f64>,
    max_frame_solve_relative_residual: Option<f64>,
    minimum_quantized_signature_linf_gap_on_success: Option<u128>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let started = Instant::now();
    let args = parse_args()?;
    let repo_root = git_repo_root()?;
    let input = read_orientation_rows(&args.input)?;
    validate_input(&input)?;
    create_dir_all(&args.out_dir)?;

    let mut sections = BTreeMap::new();
    for row in &input {
        sections.insert(row.sample_id.clone(), canonicalize(&row.duals()));
    }

    let identities = input
        .iter()
        .filter(|row| row.map_variant == "identity")
        .map(|row| (row.base_id.clone(), row))
        .collect::<BTreeMap<_, _>>();
    if identities.len() * 5 != input.len() {
        return Err(format!(
            "expected one identity and five total variants per base; got {} identities and {} rows",
            identities.len(),
            input.len()
        )
        .into());
    }

    let mut output_rows = Vec::new();
    for transformed in &input {
        let base = identities
            .get(&transformed.base_id)
            .ok_or_else(|| format!("missing identity row for {}", transformed.base_id))?;
        let base_section = &sections[&base.sample_id];
        let transformed_section = &sections[&transformed.sample_id];
        let expectation = match transformed.map_family.as_str() {
            "identity" => "identity_self_numerical_zero",
            "u2" => "section_zero_within_tolerance_on_generic_success",
            "so4" => "section_not_expected_to_vanish",
            other => return Err(format!("unexpected map family {other}").into()),
        };
        let (section_distance, control_passed, tolerance) = pair_section_result(
            base_section,
            transformed_section,
            transformed.map_family.as_str(),
        );
        output_rows.push(AdapterRow {
            schema: ROW_SCHEMA,
            comparison_id: format!("panel/{}", transformed.sample_id),
            source_kind: "retained_orientation_panel",
            base_id: base.base_id.clone(),
            transformed_id: transformed.sample_id.clone(),
            bucket: Some(transformed.bucket.clone()),
            map_variant: transformed.map_variant.clone(),
            map_family: transformed.map_family.clone(),
            map_mode: transformed.map_mode.clone(),
            seed: None,
            attempt: None,
            expectation,
            base_section: base_section.into(),
            transformed_section: transformed_section.into(),
            raw_coordinate_unordered_row_assignment_rms: unordered_row_assignment_rms(
                &base.duals(),
                &transformed.duals(),
            ),
            generic_sys_section_unordered_row_assignment_rms: section_distance,
            section_distance_tolerance: tolerance,
            control_passed,
            transform_parameters: json!({
                "source": "retained_orientation_row",
                "orientation_reconstruction_ms_observation": transformed.reconstruction_ms,
            }),
            interpretation: panel_interpretation(&transformed.map_family),
        });
    }

    let stochastic_start = output_rows.len();
    let mut rng = ChaCha8Rng::seed_from_u64(CONTROL_SEED);
    for (base_index, base) in identities.values().enumerate() {
        let base_duals = base.duals();
        let base_section = &sections[&base.sample_id];
        for family in ["scale", "translation", "facet_permutation", "sp4"] {
            let (transformed, parameters) = stochastic_transform(&base_duals, family, &mut rng)?;
            let transformed_section = canonicalize(&transformed);
            let section_distance = section_distance(base_section, &transformed_section);
            let control_passed = section_distance.map(|distance| distance <= SECTION_TOLERANCE);
            output_rows.push(AdapterRow {
                schema: ROW_SCHEMA,
                comparison_id: format!(
                    "stochastic/seed={CONTROL_SEED}/base={base_index}/family={family}/attempt=0"
                ),
                source_kind: "seeded_stochastic_invariance_control",
                base_id: base.base_id.clone(),
                transformed_id: format!("{}/stochastic/{family}", base.base_id),
                bucket: Some(base.bucket.clone()),
                map_variant: family.to_string(),
                map_family: family.to_string(),
                map_mode: "seeded_stochastic".to_string(),
                seed: Some(CONTROL_SEED),
                attempt: Some(0),
                expectation: "section_zero_within_tolerance_on_generic_success",
                base_section: base_section.into(),
                transformed_section: (&transformed_section).into(),
                raw_coordinate_unordered_row_assignment_rms: unordered_row_assignment_rms(
                    &base_duals,
                    &transformed,
                ),
                generic_sys_section_unordered_row_assignment_rms: section_distance,
                section_distance_tolerance: Some(SECTION_TOLERANCE),
                control_passed,
                transform_parameters: parameters,
                interpretation: "Seeded implementation control for the combined volume/center/generic-section adapter; it is not population evidence.",
            });
        }
    }

    let tied = symmetric_cube_duals();
    let tied_section = canonicalize(&tied);
    let tied_passed =
        tied_section.status == "nonunique_omega_signature" && tied_section.coordinates.is_none();
    output_rows.push(AdapterRow {
        schema: ROW_SCHEMA,
        comparison_id: "boundary/symmetric-cube".to_string(),
        source_kind: "generic_domain_boundary_control",
        base_id: "synthetic/symmetric-cube".to_string(),
        transformed_id: "synthetic/symmetric-cube".to_string(),
        bucket: None,
        map_variant: "identity".to_string(),
        map_family: "boundary".to_string(),
        map_mode: "deterministic".to_string(),
        seed: None,
        attempt: Some(0),
        expectation: "observable_nonunique_signature_non_success",
        base_section: (&tied_section).into(),
        transformed_section: (&tied_section).into(),
        raw_coordinate_unordered_row_assignment_rms: Some(0.0),
        generic_sys_section_unordered_row_assignment_rms: None,
        section_distance_tolerance: None,
        control_passed: Some(tied_passed),
        transform_parameters: json!({"dual_rows": "plus/minus standard basis in (q1,q2,p1,p2)"}),
        interpretation: "The symmetric cube has tied omega row signatures; the adapter emits non-success and no canonical coordinate rows.",
    });

    let panel_rows = &output_rows[..stochastic_start];
    let stochastic_rows = &output_rows[stochastic_start..output_rows.len() - 1];
    let panel_summary = summarize_by_variant(panel_rows);
    let stochastic_summary = summarize_by_variant(stochastic_rows);
    let panel_residuals = summarize_unique_sections(&sections);

    let identity_ok = required_group_passed(&panel_summary, "identity", true);
    let u2_deterministic_ok = required_group_passed(&panel_summary, "u2-deterministic", true);
    let u2_haar_ok = required_group_passed(&panel_summary, "u2-haar", true);
    let so4_haar_positive = panel_summary
        .get("so4-haar")
        .map(|summary| {
            summary.generic_sys_section_distance.count > 0
                && summary
                    .generic_sys_section_distance
                    .positive_above_so4_threshold
                    > 0
        })
        .unwrap_or(false);
    let stochastic_ok = ["scale", "translation", "facet_permutation", "sp4"]
        .iter()
        .all(|family| required_group_passed(&stochastic_summary, family, true));
    let all_required_controls_passed = identity_ok
        && u2_deterministic_ok
        && u2_haar_ok
        && so4_haar_positive
        && stochastic_ok
        && tied_passed;

    let source_revision = git(&repo_root, &["rev-parse", "HEAD"])?;
    let source_tree = git(&repo_root, &["rev-parse", "HEAD^{tree}"])?;
    let tracked_clean = git_tracked_clean(&repo_root)?;
    let input_hash = hash_file(&args.input)?;
    let source_hashes = hash_named_files(
        &repo_root,
        &[
            OWN_LIB_PATH,
            OWN_MAIN_PATH,
            OWN_MANIFEST_PATH,
            OWN_README_PATH,
            FORMAL_PATH,
            FROZEN_CANDIDATE_PATH,
            FROZEN_README_PATH,
            ORIENTATION_REPORT_PATH,
        ],
    )?;
    let orientation_geometry_ms = input.iter().map(|row| row.reconstruction_ms).sum::<f64>();

    let report = json!({
        "schema": REPORT_SCHEMA,
        "command": format!(
            "cargo run -p exp-generator-sys-orbit-view --release -- --input {} --out-dir {}",
            args.input.display(), args.out_dir.display()
        ),
        "provenance": {
            "source_revision": source_revision,
            "source_tree": source_tree,
            "tracked_clean_at_execution": tracked_clean,
            "producer_and_source_file_blake3": source_hashes,
        },
        "input": {
            "path": repo_relative_or_display(&repo_root, &args.input),
            "blake3": input_hash,
            "schema": INPUT_SCHEMA,
            "row_count": input.len(),
            "base_independence_units": identities.len(),
            "selection": "all retained orientation panel rows; paired by base_id against the identity row",
        },
        "formula_correspondence": {
            "formal_source": FORMAL_PATH,
            "formal_labels": [
                "def:omega-row-signatures",
                "def:symplectic-gram-schmidt-quadruple",
                "def:generic-coordinate-section",
                "prop:generic-coordinate-canonization",
                "rem:generic-coordinate-canonization-not-universal"
            ],
            "frozen_candidate": FROZEN_CANDIDATE_PATH,
            "local_steps": [
                "volume-one scaling reconstructed from normalized dual inequalities",
                "analytic-center translation",
                "descending quantized sorted omega-row signatures and explicit tie non-success",
                "lexicographic ordered-quadruple scan",
                "symplectic Gram-Schmidt frame (q1,q2,p1,p2)",
                "frame-inverse coordinate rows"
            ],
            "theorem_boundary": "The formal note is agent-written and unreviewed; it proves the generic Sp(4) and facet-permutation section after scale fixing and centering. The Rust adapter is an f64 prototype of the frozen candidate."
        },
        "views": {
            "raw_coordinate_unordered_row_assignment_rms": {
                "definition": "sqrt(minimum over row bijections of mean squared Euclidean row distance)",
                "quotient": "facet-row permutation only",
                "boundary": "order-insensitive raw coordinate diagnostic; not translation, scale, O(4), GL(4), or Sp(4) invariant"
            },
            "generic_sys_section_unordered_row_assignment_rms": {
                "definition": "the same exact assignment RMS after successful generic section construction on both inputs",
                "quotient": "generic f64 representative for positive scale, analytic-center translation, facet permutation, and Sp(4)",
                "boundary": "one partial section view, not an optimized quotient metric theorem; absent on any non-success"
            }
        },
        "tolerances": {
            "identity_numerical_zero": IDENTITY_TOLERANCE,
            "generic_section_invariance": SECTION_TOLERANCE,
            "so4_observed_nonzero": SO4_NONZERO_THRESHOLD,
            "justification": "The frozen 256-case candidate artifact used 1e-5 as its residual-failure threshold and observed maxima below 1e-6 for ordinary random inputs. Identity uses a stricter direct self-comparison tolerance."
        },
        "panel": {
            "pair_rows": panel_rows.len(),
            "section_evaluations": panel_residuals,
            "by_map_variant": panel_summary,
        },
        "stochastic_invariance_controls": {
            "seed": CONTROL_SEED,
            "attempt_per_base_family": 0,
            "base_count": identities.len(),
            "pair_rows": stochastic_rows.len(),
            "by_family": stochastic_summary,
        },
        "generic_domain_boundary": {
            "input": "symmetric dual cube with rows plus/minus e_i",
            "status": tied_section.status,
            "coordinates_emitted": tied_section.coordinates.is_some(),
            "control_passed": tied_passed,
        },
        "cost": {
            "retained_orientation_geometry_reconstruction_ms_observation_total": orientation_geometry_ms,
            "retained_orientation_geometry_reconstruction_observation_count": input.len(),
            "panel_section_canonicalization_count": sections.len(),
            "stochastic_transformed_section_canonicalization_count": stochastic_rows.len(),
            "boundary_section_canonicalization_count": 1,
            "adapter_wall_clock_retained": "separate cost-observation.json",
            "adapter_wall_clock_policy": "cost-observation.json is explicitly nondeterministic; rows.jsonl and report.json remain byte-replayable",
            "dominant_adapter_work": "f64 vertex enumeration and volume reconstruction, followed by O(F^4) ordered frame scan in the worst case and O(F^3) assignment distance"
        },
        "completion": {
            "identity_controls_passed": identity_ok,
            "u2_deterministic_generic_controls_passed": u2_deterministic_ok,
            "u2_haar_generic_controls_passed": u2_haar_ok,
            "haar_so4_has_observed_nonzero_section_view": so4_haar_positive,
            "stochastic_invariance_controls_passed": stochastic_ok,
            "tied_boundary_control_passed": tied_passed,
            "all_required_controls_passed": all_required_controls_passed,
        },
        "interpretation": {
            "allowed": [
                "the adapter implements the frozen generic construction with observable statuses",
                "the retained finite panel and seeded controls test numerical section behavior",
                "Haar U(2) panel pairs are Sp(4)-inside controls on generic successes",
                "nonzero Haar SO(4) values are section-view evidence on this finite panel"
            ],
            "prohibited": [
                "a universal canonical form",
                "a theorem that the section distance is an optimized quotient metric",
                "population support, law ranking, mechanism, target transfer, or any sys/capacity claim",
                "interpreting section Euclidean geometry as intrinsic body geometry"
            ],
            "section_warning": "The section is generic, discontinuous near omega-signature ties, and chooses an arbitrary-but-canonical facet-derived symplectic frame. It complements direct invariants and optimized quotient distances.",
            "deferred": {
                "O(4)_representatives": "not created; direct invariant or optimized comparisons may be preferable",
                "GL(4)_representatives": "not created; direct invariant or optimized comparisons may be preferable"
            }
        }
    });

    write_rows(&args.out_dir.join("rows.jsonl"), &output_rows)?;
    write_json(&args.out_dir.join("report.json"), &report)?;
    let elapsed_ms = started.elapsed().as_secs_f64() * 1000.0;
    let cost_observation = json!({
        "schema": "generator-sys-orbit-view-cost-observation-v1",
        "status": "nondeterministic_wall_clock_observation",
        "source_revision": source_revision,
        "source_tree": source_tree,
        "input_blake3": input_hash,
        "adapter_elapsed_ms": elapsed_ms,
        "adapter_section_canonicalization_count": sections.len() + stochastic_rows.len() + 1,
        "orientation_input_geometry_reconstruction_ms_observation_total": orientation_geometry_ms,
        "orientation_input_geometry_reconstruction_observation_count": input.len(),
        "orientation_to_adapter_elapsed_ratio": orientation_geometry_ms / elapsed_ms,
        "comparison_boundary": "The orientation timings measure its exact reconstruction boundary row by row; the adapter timing measures one release-process pass including f64 volume reconstruction, comparisons, provenance, and deterministic artifact writes. This is a cost-scale observation, not a controlled benchmark."
    });
    write_json(
        &args.out_dir.join("cost-observation.json"),
        &cost_observation,
    )?;
    eprintln!(
        "generator-sys-orbit-view: {} panel rows, {} stochastic rows, elapsed_ms={:.3} (transient observation)",
        panel_rows.len(),
        stochastic_rows.len(),
        elapsed_ms
    );
    if !all_required_controls_passed {
        eprintln!("required control failed; artifacts were written fail-closed");
        std::process::exit(2);
    }
    Ok(())
}

fn parse_args() -> Result<Args, Box<dyn std::error::Error>> {
    let mut input = None;
    let mut out_dir = None;
    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--input" => input = args.next().map(PathBuf::from),
            "--out-dir" => out_dir = args.next().map(PathBuf::from),
            "--help" | "-h" => {
                println!("usage: exp-generator-sys-orbit-view --input ROWS.jsonl --out-dir DIR");
                std::process::exit(0);
            }
            _ => return Err(format!("unknown argument {arg}").into()),
        }
    }
    Ok(Args {
        input: input.ok_or("missing --input")?,
        out_dir: out_dir.ok_or("missing --out-dir")?,
    })
}

fn read_orientation_rows(path: &Path) -> Result<Vec<OrientationRow>, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let mut rows = Vec::new();
    for (index, line) in BufReader::new(file).lines().enumerate() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }
        let row: OrientationRow = serde_json::from_str(&line)
            .map_err(|error| format!("{} line {}: {error}", path.display(), index + 1))?;
        rows.push(row);
    }
    Ok(rows)
}

fn validate_input(rows: &[OrientationRow]) -> Result<(), Box<dyn std::error::Error>> {
    if rows.is_empty() {
        return Err("input has no rows".into());
    }
    for row in rows {
        if row.schema != INPUT_SCHEMA {
            return Err(format!("{} has unexpected schema {}", row.sample_id, row.schema).into());
        }
        if row.reconstruction_status != "reconstructed" {
            return Err(format!(
                "{} is not reconstructed: {}",
                row.sample_id, row.reconstruction_status
            )
            .into());
        }
        if row.transformed_dual_vertices_f64.len() < 5 {
            return Err(format!("{} has fewer than five dual rows", row.sample_id).into());
        }
    }
    Ok(())
}

fn pair_section_result(
    base: &SectionOutput,
    transformed: &SectionOutput,
    family: &str,
) -> (Option<f64>, Option<bool>, Option<f64>) {
    let distance = section_distance(base, transformed);
    match family {
        "identity" => (
            distance,
            distance.map(|value| value <= IDENTITY_TOLERANCE),
            Some(IDENTITY_TOLERANCE),
        ),
        "u2" => (
            distance,
            distance.map(|value| value <= SECTION_TOLERANCE),
            Some(SECTION_TOLERANCE),
        ),
        "so4" => (distance, None, None),
        _ => (distance, None, None),
    }
}

fn section_distance(base: &SectionOutput, transformed: &SectionOutput) -> Option<f64> {
    match (&base.coordinates, &transformed.coordinates) {
        (Some(left), Some(right)) if base.status == "ok" && transformed.status == "ok" => {
            unordered_row_assignment_rms(left, right)
        }
        _ => None,
    }
}

fn panel_interpretation(family: &str) -> &'static str {
    match family {
        "identity" => "Identity is a numerical self-distance control.",
        "u2" => "U(2) lies inside the sys symmetry group Sp(4); zero is expected only when both partial-section evaluations succeed.",
        "so4" => "SO(4) is not generally symplectic. A nonzero value is only evidence in this section view, not a quotient-metric theorem or target result.",
        _ => "Unknown family.",
    }
}

fn stochastic_transform(
    base: &[Vector4<f64>],
    family: &str,
    rng: &mut ChaCha8Rng,
) -> Result<(Vec<Vector4<f64>>, serde_json::Value), Box<dyn std::error::Error>> {
    match family {
        "scale" => {
            let scale = rng.gen_range(-1.0_f64..1.0).exp();
            Ok((
                base.iter().map(|dual| dual * scale).collect(),
                json!({"positive_dual_scale": scale}),
            ))
        }
        "translation" => {
            let mut direction = Vector4::new(
                rng.gen_range(-1.0..1.0),
                rng.gen_range(-1.0..1.0),
                rng.gen_range(-1.0..1.0),
                rng.gen_range(-1.0..1.0),
            );
            direction /= direction.norm();
            let max_dot = base
                .iter()
                .map(|dual| dual.dot(&direction))
                .fold(f64::NEG_INFINITY, f64::max);
            let radius = if max_dot <= 1e-12 {
                0.15
            } else {
                0.15_f64.min(0.35 / max_dot)
            };
            let translation = radius * direction;
            let transformed = base
                .iter()
                .map(|dual| {
                    let denominator = 1.0 - dual.dot(&translation);
                    if denominator <= 1e-10 {
                        Err("translation left normalized-inequality domain")
                    } else {
                        Ok(dual / denominator)
                    }
                })
                .collect::<Result<Vec<_>, _>>()?;
            Ok((
                transformed,
                json!({"primal_translation": vector_to_array(&translation)}),
            ))
        }
        "facet_permutation" => {
            let mut permutation = (0..base.len()).collect::<Vec<_>>();
            permutation.shuffle(rng);
            let transformed = permutation.iter().map(|&index| base[index]).collect();
            Ok((transformed, json!({"source_row_indices": permutation})))
        }
        "sp4" => {
            let mut h = Matrix4::zeros();
            for row in 0..4 {
                for col in row..4 {
                    let value = rng.gen_range(-0.55..0.55);
                    h[(row, col)] = value;
                    h[(col, row)] = value;
                }
            }
            let primal = (standard_symplectic_matrix() * h).exp();
            let defect = (primal.transpose() * standard_symplectic_matrix() * primal
                - standard_symplectic_matrix())
            .norm();
            let inverse_transpose = primal
                .try_inverse()
                .ok_or("sampled Sp(4) matrix was singular")?
                .transpose();
            let transformed = base.iter().map(|dual| inverse_transpose * dual).collect();
            Ok((
                transformed,
                json!({
                    "primal_matrix_row_major": matrix_to_rows(&primal),
                    "symplectic_defect_frobenius": defect,
                }),
            ))
        }
        _ => Err(format!("unknown stochastic transform {family}").into()),
    }
}

fn symmetric_cube_duals() -> Vec<Vector4<f64>> {
    vec![
        Vector4::new(1.0, 0.0, 0.0, 0.0),
        Vector4::new(-1.0, 0.0, 0.0, 0.0),
        Vector4::new(0.0, 1.0, 0.0, 0.0),
        Vector4::new(0.0, -1.0, 0.0, 0.0),
        Vector4::new(0.0, 0.0, 1.0, 0.0),
        Vector4::new(0.0, 0.0, -1.0, 0.0),
        Vector4::new(0.0, 0.0, 0.0, 1.0),
        Vector4::new(0.0, 0.0, 0.0, -1.0),
    ]
}

fn summarize_by_variant(rows: &[AdapterRow]) -> BTreeMap<String, PairSummary> {
    let mut grouped = BTreeMap::<String, Vec<&AdapterRow>>::new();
    for row in rows {
        grouped
            .entry(row.map_variant.clone())
            .or_default()
            .push(row);
    }
    grouped
        .into_iter()
        .map(|(variant, rows)| {
            let raw = rows
                .iter()
                .filter_map(|row| row.raw_coordinate_unordered_row_assignment_rms)
                .collect::<Vec<_>>();
            let section = rows
                .iter()
                .filter_map(|row| row.generic_sys_section_unordered_row_assignment_rms)
                .collect::<Vec<_>>();
            let summary = PairSummary {
                pair_count: rows.len(),
                generic_success_pair_count: section.len(),
                base_non_success_count: rows
                    .iter()
                    .filter(|row| row.base_section.status != "ok")
                    .count(),
                transformed_non_success_count: rows
                    .iter()
                    .filter(|row| row.transformed_section.status != "ok")
                    .count(),
                evaluated_control_count: rows
                    .iter()
                    .filter(|row| row.control_passed.is_some())
                    .count(),
                passed_control_count: rows
                    .iter()
                    .filter(|row| row.control_passed == Some(true))
                    .count(),
                raw_coordinate_distance: distance_stats(&raw),
                generic_sys_section_distance: distance_stats(&section),
            };
            (variant, summary)
        })
        .collect()
}

fn distance_stats(values: &[f64]) -> DistanceStats {
    DistanceStats {
        count: values.len(),
        mean: (!values.is_empty()).then(|| values.iter().sum::<f64>() / values.len() as f64),
        max: values.iter().copied().reduce(f64::max),
        positive_above_so4_threshold: values
            .iter()
            .filter(|&&value| value > SO4_NONZERO_THRESHOLD)
            .count(),
    }
}

fn summarize_unique_sections(sections: &BTreeMap<String, SectionOutput>) -> ResidualSummary {
    let mut out = ResidualSummary::default();
    out.evaluation_count = sections.len();
    for section in sections.values() {
        *out.status_counts
            .entry(section.status.to_string())
            .or_default() += 1;
        if section.status == "ok" {
            out.success_count += 1;
        }
        max_option(
            &mut out.max_volume_one_algebraic_residual,
            section.diagnostics.volume_one_algebraic_residual,
        );
        max_option(
            &mut out.max_analytic_center_gradient_norm,
            section.diagnostics.analytic_center_gradient_norm,
        );
        max_option(
            &mut out.max_analytic_center_newton_decrement,
            section.diagnostics.analytic_center_newton_decrement,
        );
        max_option(
            &mut out.max_frame_symplectic_defect_frobenius,
            section.diagnostics.frame_symplectic_defect_frobenius,
        );
        max_option(
            &mut out.max_frame_solve_relative_residual,
            section.diagnostics.frame_solve_max_relative_residual,
        );
        if section.status == "ok" {
            if let Some(gap) = section.diagnostics.minimum_quantized_signature_linf_gap {
                out.minimum_quantized_signature_linf_gap_on_success = Some(
                    out.minimum_quantized_signature_linf_gap_on_success
                        .map_or(gap, |known| known.min(gap)),
                );
            }
        }
    }
    out
}

fn max_option(target: &mut Option<f64>, value: Option<f64>) {
    if let Some(value) = value {
        *target = Some(target.map_or(value, |known| known.max(value)));
    }
}

fn required_group_passed(
    groups: &BTreeMap<String, PairSummary>,
    name: &str,
    require_generic_success: bool,
) -> bool {
    groups
        .get(name)
        .map(|summary| {
            (!require_generic_success || summary.generic_success_pair_count > 0)
                && summary.evaluated_control_count == summary.pair_count
                && summary.passed_control_count == summary.pair_count
        })
        .unwrap_or(false)
}

fn write_rows(path: &Path, rows: &[AdapterRow]) -> Result<(), Box<dyn std::error::Error>> {
    let mut writer = BufWriter::new(File::create(path)?);
    for row in rows {
        serde_json::to_writer(&mut writer, row)?;
        writer.write_all(b"\n")?;
    }
    writer.flush()?;
    Ok(())
}

fn write_json(path: &Path, value: &serde_json::Value) -> Result<(), Box<dyn std::error::Error>> {
    let mut writer = BufWriter::new(File::create(path)?);
    serde_json::to_writer_pretty(&mut writer, value)?;
    writer.write_all(b"\n")?;
    writer.flush()?;
    Ok(())
}

fn git_repo_root() -> Result<PathBuf, Box<dyn std::error::Error>> {
    Ok(PathBuf::from(git(
        Path::new("."),
        &["rev-parse", "--show-toplevel"],
    )?))
}

fn git(root: &Path, args: &[&str]) -> Result<String, Box<dyn std::error::Error>> {
    let output = Command::new("git").args(args).current_dir(root).output()?;
    if !output.status.success() {
        return Err(format!(
            "git {} failed: {}",
            args.join(" "),
            String::from_utf8_lossy(&output.stderr)
        )
        .into());
    }
    Ok(String::from_utf8(output.stdout)?.trim().to_string())
}

fn git_tracked_clean(root: &Path) -> Result<bool, Box<dyn std::error::Error>> {
    let unstaged = Command::new("git")
        .args(["diff", "--quiet", "HEAD", "--"])
        .current_dir(root)
        .status()?;
    let staged = Command::new("git")
        .args(["diff", "--cached", "--quiet", "HEAD", "--"])
        .current_dir(root)
        .status()?;
    Ok(unstaged.success() && staged.success())
}

fn hash_file(path: &Path) -> Result<String, Box<dyn std::error::Error>> {
    Ok(blake3::hash(&read(path)?).to_hex().to_string())
}

fn hash_named_files(
    root: &Path,
    paths: &[&str],
) -> Result<BTreeMap<String, String>, Box<dyn std::error::Error>> {
    paths
        .iter()
        .map(|path| Ok(((*path).to_string(), hash_file(&root.join(path))?)))
        .collect()
}

fn repo_relative_or_display(root: &Path, path: &Path) -> String {
    path.canonicalize()
        .ok()
        .and_then(|canonical| {
            root.canonicalize()
                .ok()
                .and_then(|root| canonical.strip_prefix(root).ok().map(Path::to_path_buf))
        })
        .unwrap_or_else(|| path.to_path_buf())
        .display()
        .to_string()
}

fn vector_to_array(vector: &Vector4<f64>) -> [f64; 4] {
    [vector[0], vector[1], vector[2], vector[3]]
}

fn matrix_to_rows(matrix: &Matrix4<f64>) -> [[f64; 4]; 4] {
    std::array::from_fn(|row| std::array::from_fn(|col| matrix[(row, col)]))
}

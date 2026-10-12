use exp_sys_landscape::{exact_volume_from_incidence_as_f64, poly_id, SysLandscapePolytopeCache};
use nalgebra::Vector2;
use num_rational::BigRational;
use num_traits::{One, Signed, ToPrimitive, Zero};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Instant;
use symplectic::algorithms::billiard::bounce_count_from_sigma_for_facets;
use symplectic::algorithms::facet_adjacency::build_transition_matrix_from_facet_intersections_and_omega;
use symplectic::{
    aggregate_certified_orbits_with_dual_vertices_exact, aggregate_orbits_with_dual_vertices_exact,
    classify_facets_from_dual_vertices, solve_billiard_candidates, CertifiedOrbitSetMode,
    OrbitAdmissibility, OrbitGuaranteeMode, OrbitKktData,
};
use symplectic::kkt::rational_solver::solve_kkt_exact;
use symplectic::systolic_ratio;

const ARTIFACT_DIR: &str = "artifacts";
const ENDPOINT_TOLERANCE: f64 = 1e-8;
const ARITHMETIC_TOLERANCE: f64 = 1e-12;
const EXPECTED_IDS: [&str; 8] = [
    "ridge-endpoint-3x6-q01", "ridge-endpoint-3x6-q001", "ridge-endpoint-3x6-q0001",
    "ridge-endpoint-3x6-endpoint", "ridge-endpoint-4x4-q01", "ridge-endpoint-4x4-q001",
    "ridge-endpoint-4x4-q0001", "ridge-endpoint-4x4-endpoint",
];

#[derive(Deserialize)]
struct HrepRow { normal: [f64; 2], height: f64 }

#[derive(Deserialize)]
struct Candidate {
    candidate_id: String,
    bucket: String,
    path_label: String,
    q_hrep: Vec<HrepRow>,
    p_hrep: Vec<HrepRow>,
}

#[derive(Serialize)]
struct ImplementationFile { path: String, sha256: String }

#[derive(Serialize)]
struct CapacityManifest {
    schema: &'static str,
    repo_commit: String,
    cargo_lock_sha256: String,
    implementation_files: Vec<ImplementationFile>,
    implementation_closure_sha256: String,
    ordinary_contract: &'static str,
    certificate_contract: &'static str,
}

#[derive(Serialize)]
struct TargetRow {
    schema: &'static str,
    candidate_id: String,
    bucket: String,
    path_label: String,
    poly_id: String,
    volume: f64,
    capacity: f64,
    sys: f64,
    bounces: Option<usize>,
    best_sigma: Vec<usize>,
    time_capacity_ms: f64,
    result_is_finite: bool,
    arithmetic_absolute_error: f64,
    iterations: u64,
    min_action_lower: f64,
    min_action_upper: f64,
    returned_orbit_count: usize,
    best_orbit_admissibility: String,
    best_orbit_beta_margin: f64,
    f64_indeterminate_candidate_count_before_aggregation: usize,
    /// Number of exact KKT fallback attempts made by the local line-for-line
    /// MinimaSafe resolver instrumentation before zero-gap trimming.
    exact_fallback_resolution_count: usize,
    /// Of those exact fallback attempts, the number rejected by exact KKT.
    exact_fallback_rejected_count: usize,
    candidates_sha256: String,
    api_verification_sha256: String,
    evaluator_source_sha256: String,
    capacity_manifest_sha256: String,
}

#[derive(Serialize)]
struct CertificateRecord {
    schema: &'static str,
    scope: &'static str,
    candidate_id: String,
    poly_id: String,
    candidates_sha256: String,
    capacity_manifest_sha256: String,
    ordinary_capacity: f64,
    certified_capacity: f64,
    certified_capacity_exact: String,
    agreement_absolute_error: f64,
    agreement_tolerance: f64,
    minimizer_sigmas: Vec<Vec<usize>>,
    candidate_iterations: u64,
    exact_resolutions: usize,
    enumerated_stream_scope: &'static str,
}

#[derive(Serialize)]
struct TargetSummary {
    schema: &'static str,
    status: &'static str,
    expected_candidate_ids: Vec<&'static str>,
    observed_candidate_ids: Vec<String>,
    expected_id_set_matches: bool,
    unique_ids: bool,
    row_count: usize,
    all_results_finite: bool,
    no_sys_above_one: bool,
    max_arithmetic_absolute_error: f64,
    arithmetic_tolerance: f64,
    endpoint_3x6_sys: f64,
    endpoint_4x4_sys: f64,
    endpoint_tolerance: f64,
    q01_3x6_capacity: f64,
    q01_3x6_sys: f64,
    target_evaluation_sha256: String,
    certificate_sha256: String,
    candidates_sha256: String,
    api_verification_sha256: String,
    cdf_placement_json_sha256: String,
    cdf_placement_tsv_sha256: String,
    capacity_manifest_sha256: String,
    evaluator_source_sha256: String,
}

fn sha256(path: impl AsRef<Path>) -> String {
    let bytes = std::fs::read(path).expect("read hashed input");
    format!("{:x}", Sha256::digest(bytes))
}

fn rational_string(value: &BigRational) -> String {
    format!("{}/{}", value.numer(), value.denom())
}

fn artifact_path(name: &str) -> PathBuf {
    Path::new(ARTIFACT_DIR).join(name)
}

fn repo_root() -> PathBuf {
    let output = Command::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .output()
        .expect("run git rev-parse --show-toplevel");
    assert!(output.status.success(), "must run inside a git checkout");
    PathBuf::from(String::from_utf8(output.stdout).expect("git path utf8").trim())
}

fn implementation_manifest() -> CapacityManifest {
    let relative_paths = [
        "experiments/sys-landscape/src/lib.rs",
        "crates/symplectic/src/lib.rs",
        "crates/symplectic/src/algorithms/mod.rs",
        "crates/symplectic/src/algorithms/billiard/mod.rs",
        "crates/symplectic/src/algorithms/billiard/block_enumeration.rs",
        "crates/symplectic/src/algorithms/billiard/facet_classification.rs",
        "crates/symplectic/src/algorithms/facet_adjacency.rs",
        "crates/symplectic/src/algorithms/orbit_search.rs",
        "crates/symplectic/src/kkt/rational_solver.rs",
        "crates/symplectic/src/kkt/mod.rs",
        "crates/symplectic/src/kkt/saddle_point_solver.rs",
        "crates/symplectic/src/kkt/qp_assembly.rs",
        "crates/symplectic/src/kkt/beta_feasibility.rs",
        "crates/symplectic/src/geom/mod.rs",
        "crates/symplectic/src/geom/symplectic_form.rs",
        "crates/symplectic/src/geom/rational_arithmetic.rs",
    ];
    let repo_root = repo_root();
    let implementation_files = relative_paths.iter().map(|relative| ImplementationFile {
        path: (*relative).to_string(),
        sha256: sha256(repo_root.join(relative)),
    }).collect::<Vec<_>>();
    let mut digest_input = String::new();
    for file in &implementation_files {
        digest_input.push_str(&file.path);
        digest_input.push('\n');
        digest_input.push_str(&file.sha256);
        digest_input.push('\n');
    }
    let repo_commit = String::from_utf8(Command::new("git").args(["rev-parse", "HEAD"])
        .output().expect("run git rev-parse").stdout).expect("git hash utf8").trim().to_string();
    CapacityManifest {
        schema: "ridge-endpoint-smoke.capacity-implementation-manifest.v1",
        repo_commit,
        cargo_lock_sha256: sha256("Cargo.lock"),
        implementation_files,
        implementation_closure_sha256: format!("{:x}", Sha256::digest(digest_input.as_bytes())),
        ordinary_contract: "classify -> transition matrix -> solve_billiard_candidates -> aggregate_orbits_with_dual_vertices_exact(MinimaSafe)",
        certificate_contract: "aggregate_certified_orbits_with_dual_vertices_exact(MinimizersOnly) over the same f64-enumerated billiard candidate stream",
    }
}

/// Scratch-local instrumentation of the current private MinimaSafe resolver.
/// It preserves the production selection/replacement logic exactly, while
/// exposing the otherwise-private pre-trim count of exact KKT attempts.
fn instrumented_minimasafe_exact_fallback_counts(
    dual_vertices: &[[BigRational; 4]],
    mut orbits: Vec<OrbitKktData>,
) -> (usize, usize, Vec<OrbitKktData>) {
    fn exact_replace(
        dual_vertices: &[[BigRational; 4]],
        orbit: &OrbitKktData,
    ) -> Option<OrbitKktData> {
        let exact = solve_kkt_exact(dual_vertices, &orbit.sigma)?;
        if !exact.beta.iter().all(|beta| beta.is_positive()) || exact.q_exact <= BigRational::zero() {
            return None;
        }
        let beta_sum = exact.beta.iter().cloned().fold(BigRational::zero(), |sum, beta| sum + beta);
        if beta_sum != BigRational::one() || !(0..4).all(|d| {
            orbit.sigma.iter().zip(exact.beta.iter()).map(|(&facet, beta)| beta * &dual_vertices[facet][d])
                .fold(BigRational::zero(), |sum, entry| sum + entry).is_zero()
        }) {
            return None;
        }
        let q = exact.q_exact.to_f64().expect("exact q converts to f64");
        let action = 0.5 / q;
        let beta = exact.beta.iter().map(|value| value.to_f64().expect("exact beta converts to f64")).collect::<Vec<_>>();
        let beta_margin = beta.iter().copied().fold(f64::INFINITY, f64::min);
        Some(OrbitKktData {
            sigma: orbit.sigma.clone(), beta, beta_margin, action, action_lower: action, action_upper: action,
            q, q_error_bound: 0.0, mu: orbit.mu, xi: orbit.xi,
            admissibility: OrbitAdmissibility::AdmissibleExact,
        })
    }
    fn resolve_indices(
        dual_vertices: &[[BigRational; 4]], orbits: &mut Vec<OrbitKktData>, mut indices: Vec<usize>,
        resolutions: &mut usize, rejected: &mut usize,
    ) {
        indices.sort_unstable(); indices.dedup();
        *resolutions += indices.len();
        for index in indices.into_iter().rev() {
            match exact_replace(dual_vertices, &orbits[index]) {
                Some(exact) => orbits[index] = exact,
                None => { orbits.remove(index); *rejected += 1; }
            }
        }
        assert!(!orbits.is_empty(), "MinimaSafe exact fallback removed every orbit");
    }
    let mut resolutions = 0usize;
    let mut rejected = 0usize;
    loop {
        loop {
            let lower_index = orbits.iter().enumerate().min_by(|(_, a), (_, b)| a.action_lower.total_cmp(&b.action_lower)).map(|(i, _)| i).unwrap();
            let upper_index = orbits.iter().enumerate().min_by(|(_, a), (_, b)| a.action_upper.total_cmp(&b.action_upper)).map(|(i, _)| i).unwrap();
            let mut needs = Vec::new();
            if orbits[lower_index].admissibility == OrbitAdmissibility::IndeterminateF64 { needs.push(lower_index); }
            if orbits[upper_index].admissibility == OrbitAdmissibility::IndeterminateF64 { needs.push(upper_index); }
            if needs.is_empty() { break; }
            resolve_indices(dual_vertices, &mut orbits, needs, &mut resolutions, &mut rejected);
        }
        let lower = orbits.iter().map(|orbit| orbit.action_lower).fold(f64::INFINITY, f64::min);
        let upper = orbits.iter().map(|orbit| orbit.action_upper).fold(f64::INFINITY, f64::min);
        let needs = orbits.iter().enumerate().filter_map(|(i, orbit)| {
            (orbit.admissibility == OrbitAdmissibility::IndeterminateF64
                && orbit.action_lower <= upper && lower <= orbit.action_upper).then_some(i)
        }).collect::<Vec<_>>();
        if needs.is_empty() { break; }
        resolve_indices(dual_vertices, &mut orbits, needs, &mut resolutions, &mut rejected);
    }
    (resolutions, rejected, orbits)
}

fn main() {
    let candidates_sha256 = sha256(artifact_path("candidates.jsonl"));
    let api_verification_sha256 = sha256(artifact_path("api-verification.jsonl"));
    let evaluator_source_sha256 = sha256("src/main.rs");
    let manifest = implementation_manifest();
    let mut manifest_out = BufWriter::new(File::create(artifact_path("capacity-implementation-manifest.json")).expect("create manifest"));
    writeln!(manifest_out, "{}", serde_json::to_string_pretty(&manifest).unwrap()).unwrap();
    manifest_out.flush().unwrap();
    let capacity_manifest_sha256 = sha256(artifact_path("capacity-implementation-manifest.json"));

    let input = BufReader::new(File::open(artifact_path("candidates.jsonl")).expect("open candidates"));
    let mut output = BufWriter::new(File::create(artifact_path("target-evaluation.jsonl")).expect("create target output"));
    let mut endpoint_3x6_sys = None;
    let mut endpoint_4x4_sys = None;
    let mut q01_3x6 = None;
    let mut all_results_finite = true;
    let mut no_sys_above_one = true;
    let mut max_arithmetic_absolute_error = 0.0f64;
    let mut observed_ids = BTreeSet::new();
    let mut certificate = None;

    for line in input.lines() {
        let row: Candidate = serde_json::from_str(&line.expect("read line")).expect("parse row");
        let qn = row.q_hrep.iter().map(|x| Vector2::from(x.normal)).collect::<Vec<_>>();
        let qh = row.q_hrep.iter().map(|x| x.height).collect::<Vec<_>>();
        let pn = row.p_hrep.iter().map(|x| Vector2::from(x.normal)).collect::<Vec<_>>();
        let ph = row.p_hrep.iter().map(|x| x.height).collect::<Vec<_>>();
        let poly = SysLandscapePolytopeCache::from_lagrangian_product(&qn, &qh, &pn, &ph)
            .expect("canonical product must construct");
        let poly_id = poly_id(&poly);
        let volume = exact_volume_from_incidence_as_f64(&poly.vertices, &poly.vertex_facet_incidence);
        let classification = classify_facets_from_dual_vertices(&poly.dual_vertices_f64)
            .expect("frozen candidates are Lagrangian products");
        let transition_is_allowed = build_transition_matrix_from_facet_intersections_and_omega(
            &poly.facet_intersection_is_nonempty, &poly.omega_signs,
        );
        let start = Instant::now();
        let (candidates, iterations) = solve_billiard_candidates(
            &poly.dual_vertices_f64, &classification.q_indices, &classification.p_indices,
            &poly.facet_intersection_is_nonempty, &transition_is_allowed,
        ).expect("enumerate frozen billiard stream");
        let f64_indeterminate_candidate_count_before_aggregation = candidates.iter()
            .filter(|orbit| orbit.admissibility == OrbitAdmissibility::IndeterminateF64).count();
        let (exact_fallback_resolution_count, exact_fallback_rejected_count, instrumented_orbits) =
            instrumented_minimasafe_exact_fallback_counts(&poly.dual_vertices, candidates.clone());
        let ordinary = aggregate_orbits_with_dual_vertices_exact(
            &poly.dual_vertices, candidates.clone(), iterations, 0.0, OrbitGuaranteeMode::MinimaSafe,
        ).expect("ordinary MinimaSafe capacity");
        let instrumented_min_action = instrumented_orbits.iter()
            .filter(|orbit| matches!(orbit.admissibility, OrbitAdmissibility::AdmissibleF64 | OrbitAdmissibility::AdmissibleExact))
            .map(|orbit| orbit.action).min_by(|a, b| a.total_cmp(b)).expect("instrumented admissible orbit");
        assert!((instrumented_min_action - ordinary.min_action).abs() <= ARITHMETIC_TOLERANCE,
            "instrumented MinimaSafe must match the public ordinary capacity");
        let time_capacity_ms = start.elapsed().as_secs_f64() * 1000.0;
        let capacity = ordinary.min_action;
        let sys = systolic_ratio(capacity, volume);
        let best = ordinary.best_orbit();
        let best_sigma = best.sigma.clone();
        let bounces = bounce_count_from_sigma_for_facets(&classification.q_indices, &classification.p_indices, &best_sigma);
        let arithmetic_absolute_error = ((capacity * capacity) / (2.0 * volume) - sys).abs();
        let result_is_finite = volume.is_finite() && capacity.is_finite() && sys.is_finite()
            && time_capacity_ms.is_finite() && ordinary.min_action_lower.is_finite() && ordinary.min_action_upper.is_finite();
        all_results_finite &= result_is_finite;
        no_sys_above_one &= sys <= 1.0;
        max_arithmetic_absolute_error = max_arithmetic_absolute_error.max(arithmetic_absolute_error);
        let target = TargetRow {
            schema: "ridge-endpoint-smoke.target-evaluation.v2",
            candidate_id: row.candidate_id.clone(), bucket: row.bucket.clone(), path_label: row.path_label.clone(),
            poly_id: poly_id.clone(), volume, capacity, sys, bounces, best_sigma: best_sigma.clone(), time_capacity_ms,
            result_is_finite, arithmetic_absolute_error, iterations: ordinary.iterations,
            min_action_lower: ordinary.min_action_lower, min_action_upper: ordinary.min_action_upper,
            returned_orbit_count: ordinary.orbits.len(), best_orbit_admissibility: format!("{:?}", best.admissibility),
            best_orbit_beta_margin: best.beta_margin,
            f64_indeterminate_candidate_count_before_aggregation,
            exact_fallback_resolution_count, exact_fallback_rejected_count,
            candidates_sha256: candidates_sha256.clone(), api_verification_sha256: api_verification_sha256.clone(),
            evaluator_source_sha256: evaluator_source_sha256.clone(), capacity_manifest_sha256: capacity_manifest_sha256.clone(),
        };
        writeln!(output, "{}", serde_json::to_string(&target).unwrap()).unwrap();
        output.flush().unwrap();
        assert!(sys <= 1.0, "trusted target result sys > 1 for {}; stopping immediately", row.candidate_id);
        if row.path_label == "endpoint" && row.bucket == "3x6" {
            assert!((sys - 0.75).abs() <= ENDPOINT_TOLERANCE, "3x6 endpoint sys mismatch: {sys}");
            endpoint_3x6_sys = Some(sys);
        }
        if row.path_label == "endpoint" && row.bucket == "4x4" {
            assert!((sys - 0.5).abs() <= ENDPOINT_TOLERANCE, "4x4 endpoint sys mismatch: {sys}");
            endpoint_4x4_sys = Some(sys);
        }
        if row.candidate_id == "ridge-endpoint-3x6-q01" {
            let certified = aggregate_certified_orbits_with_dual_vertices_exact(
                &poly.dual_vertices, candidates, iterations, BigRational::from_integer(0.into()),
                CertifiedOrbitSetMode::MinimizersOnly,
            ).expect("exact q01 certificate over enumerated stream");
            let agreement_absolute_error = (certified.capacity - capacity).abs();
            assert!(agreement_absolute_error <= ARITHMETIC_TOLERANCE, "q01 ordinary/certified capacity mismatch");
            certificate = Some(CertificateRecord {
                schema: "ridge-endpoint-smoke.certified-minimizers.v1",
                scope: "f64-enumerated billiard stream for the frozen q01 candidate; not an all-sigma symbolic enumeration",
                candidate_id: row.candidate_id.clone(), poly_id, candidates_sha256: candidates_sha256.clone(),
                capacity_manifest_sha256: capacity_manifest_sha256.clone(), ordinary_capacity: capacity,
                certified_capacity: certified.capacity, certified_capacity_exact: rational_string(&certified.capacity_exact),
                agreement_absolute_error, agreement_tolerance: ARITHMETIC_TOLERANCE,
                minimizer_sigmas: certified.minimizers.iter().map(|orbit| orbit.sigma.clone()).collect(),
                candidate_iterations: certified.iterations, exact_resolutions: certified.exact_resolutions,
                enumerated_stream_scope: "solve_billiard_candidates from current facet classification and omega-aware transition matrix",
            });
            q01_3x6 = Some((capacity, sys));
        }
        assert!(observed_ids.insert(row.candidate_id), "duplicate candidate id");
    }
    let certificate = certificate.expect("q01 certificate must be present");
    let mut certificate_out = BufWriter::new(File::create(artifact_path("q01-certified-minimizers.json")).expect("create certificate"));
    writeln!(certificate_out, "{}", serde_json::to_string_pretty(&certificate).unwrap()).unwrap();
    certificate_out.flush().unwrap();
    output.flush().unwrap();
    let expected_ids = EXPECTED_IDS.into_iter().collect::<BTreeSet<_>>();
    let expected_id_set_matches = observed_ids.iter().map(String::as_str).collect::<BTreeSet<_>>() == expected_ids;
    assert!(expected_id_set_matches, "expected exactly the frozen eight candidate ids");
    assert!(max_arithmetic_absolute_error <= ARITHMETIC_TOLERANCE, "sys arithmetic mismatch");
    let (q01_3x6_capacity, q01_3x6_sys) = q01_3x6.expect("q01 row must be present");
    let summary = TargetSummary {
        schema: "ridge-endpoint-smoke.target-summary.v2", status: "completed",
        expected_candidate_ids: EXPECTED_IDS.to_vec(), observed_candidate_ids: observed_ids.into_iter().collect(),
        expected_id_set_matches, unique_ids: true, row_count: 8, all_results_finite, no_sys_above_one,
        max_arithmetic_absolute_error, arithmetic_tolerance: ARITHMETIC_TOLERANCE,
        endpoint_3x6_sys: endpoint_3x6_sys.expect("3x6 endpoint must be present"),
        endpoint_4x4_sys: endpoint_4x4_sys.expect("4x4 endpoint must be present"), endpoint_tolerance: ENDPOINT_TOLERANCE,
        q01_3x6_capacity, q01_3x6_sys,
        target_evaluation_sha256: sha256(artifact_path("target-evaluation.jsonl")), certificate_sha256: sha256(artifact_path("q01-certified-minimizers.json")),
        candidates_sha256, api_verification_sha256, cdf_placement_json_sha256: sha256(artifact_path("cdf-placement.json")),
        cdf_placement_tsv_sha256: sha256(artifact_path("cdf-placement.tsv")), capacity_manifest_sha256, evaluator_source_sha256,
    };
    let mut summary_out = BufWriter::new(File::create(artifact_path("target-summary.json")).expect("create summary"));
    writeln!(summary_out, "{}", serde_json::to_string_pretty(&summary).unwrap()).unwrap();
}

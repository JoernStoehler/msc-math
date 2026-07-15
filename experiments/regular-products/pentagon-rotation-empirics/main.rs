//! Pentagon-rotation empirical producers for `P_5 x_L R(theta) P_5`.
//!
//! Goal: record an owned per-theta dataset with all tied admissible minimal
//! orbits, so the branch structure behind the conjectured explicit formula can
//! be classified without depending on older experiment outputs.
//! Input Artifacts: None
//! Output Artifacts:
//!   - default smoke run: experiments/regular-products/pentagon-rotation-empirics/smoke-theta-sweep.jsonl
//!   - canonical refresh with `--canonical`: experiments/regular-products/pentagon-rotation-empirics/theta-sweep.jsonl
//!   - branch smoke run with `--three-bounce-branches`: experiments/regular-products/pentagon-rotation-empirics/smoke-three-bounce-branches.jsonl
//!   - canonical branch refresh with `--three-bounce-branches --canonical`: experiments/regular-products/pentagon-rotation-empirics/three-bounce-branches.jsonl
//!   - bounded KKT-landscape spike with `--branch-landscape --spike`: experiments/regular-products/pentagon-rotation-empirics/smoke-kkt-branch-landscape.jsonl
//!   - canonical KKT landscape with `--branch-landscape --canonical`: experiments/regular-products/pentagon-rotation-empirics/kkt-branch-landscape.jsonl
//!
//! Architecture:
//! 1. Build the regular pentagon product family over the fundamental domain.
//! 2. Run the billiard capacity frontend at each angle.
//! 3. Retain all admissible orbits tied at the minimum action within a relative
//!    tolerance aligned with the `sys-landscape` active-orbit surface.
//! 4. Write one JSONL row per angle with the tied orbit payloads.
//! 5. Optionally dump all admissible 3-bounce sigma solves for legacy branch plots.
//! 6. In KKT-landscape mode, freeze the raw two-/three-block sigma universe at
//!    one generic interior angle and retain one four-way solve status for every
//!    raw sigma at every sampled angle.
//!
//! Formal correspondence:
//! - The owned sweep is the computational sanity-check companion to
//!   [lem:pentagon-rotation-empirical-branch].
//! - The 3-bounce branch dump is the empirical surface for the open proof
//!   obligation [lem:pentagon-rotation-three-bounce].

use exp_regular_products::{euclidean_volume_f64, ProductPolytopeCache};
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::time::{Instant, SystemTime, UNIX_EPOCH};
use symplectic::algorithms::billiard::facet_classification::FacetClassification;
use symplectic::algorithms::billiard::{
    bounce_count_from_sigma_for_facets, for_each_sigma_from_facets,
};
use symplectic::algorithms::{
    aggregate_orbits_with_dual_vertices_exact, solve_orbit_sigma_saddle_point, OrbitGuaranteeMode,
    OrbitSearchError, OrbitSolveError,
};
use symplectic::geom::polygon::{polygon_area, regular_polygon_2d, rotate_polygon_2d};
use symplectic::{
    classify_facets_from_dual_vertices, solve_billiard_candidates, OrbitAdmissibility, OrbitKktData,
};

const START_DEG: f64 = 0.0;
const END_DEG: f64 = 36.0;
const STEP_DEG: f64 = 0.25;
const ACTIVE_ORBIT_RTOL: f64 = 1e-9;
const LANDSCAPE_END_DEG: f64 = 18.0;
const LANDSCAPE_STEP_DEG: f64 = 0.25;
const LANDSCAPE_FREEZE_DEG: f64 = 9.0;
const EXPECTED_OPEN_SIGMA_COUNT: usize = 3340;
// The analyzer applies this fixed display censoring threshold. All numerical
// outcomes remain in the raw artifact even when their action exceeds it.
const LANDSCAPE_ACTION_CUTOFF: f64 = 6.0;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum SweepMode {
    Minima,
    ThreeBounceBranches,
    BranchLandscape,
}

#[derive(Debug, Serialize)]
struct AffineBlockDump {
    kind: String,
    facets_rel: Vec<usize>,
}

#[derive(Debug, Serialize)]
struct OrbitDump {
    sigma: Vec<usize>,
    subset: Vec<usize>,
    beta: Vec<f64>,
    beta_margin: f64,
    action: f64,
    action_lower: f64,
    action_upper: f64,
    q: f64,
    q_error_bound: f64,
    admissibility: String,
    bounces: Option<usize>,
    q_blocks: Vec<AffineBlockDump>,
    p_blocks: Vec<AffineBlockDump>,
}

#[derive(Debug, Serialize)]
struct ThetaRow {
    angle_deg: f64,
    angle_rad: f64,
    volume: f64,
    area_q: f64,
    area_p: f64,
    capacity: f64,
    sys: f64,
    iterations: u64,
    n_returned_orbits: usize,
    n_tied_orbits: usize,
    tied_orbits: Vec<OrbitDump>,
}

#[derive(Debug, Serialize)]
struct ThreeBounceThetaRow {
    angle_deg: f64,
    angle_rad: f64,
    volume: f64,
    area_q: f64,
    area_p: f64,
    n_three_bounce_sigmas_examined: usize,
    n_admissible_three_bounce_orbits: usize,
    admissible_three_bounce_orbits: Vec<OrbitDump>,
}

fn main() {
    let cli = parse_cli();
    if cli.mode == SweepMode::BranchLandscape {
        run_branch_landscape(&cli);
        return;
    }

    let output_path = output_path(cli.mode, cli.canonical);
    let file = File::create(&output_path).expect("cannot create theta sweep output");
    let mut writer = BufWriter::new(file);

    let (qn, qh) = regular_polygon_2d(5, 1.0);
    let (pn_base, ph_base) = regular_polygon_2d(5, 1.0);
    let area_q = polygon_area(&qn, &qh).expect("pentagon area_q");
    let area_p = polygon_area(&pn_base, &ph_base).expect("pentagon area_p");

    for angle_deg in sweep_angles(START_DEG, END_DEG, STEP_DEG) {
        let theta = angle_deg.to_radians();
        let (pn, ph) = rotate_polygon_2d(&pn_base, &ph_base, theta);
        let polytope = ProductPolytopeCache::from_lagrangian_product(&qn, &qh, &pn, &ph)
            .expect("pentagon product construction failed");
        let classification = classify_facets_from_dual_vertices(&polytope.dual_vertices_f64)
            .expect("pentagon product should classify as a product");
        let vol = euclidean_volume_f64(&polytope.vertices, &polytope.vertex_facet_incidence);
        match cli.mode {
            SweepMode::Minima => {
                let result = collect_minima_safe_billiard_result(&polytope)
                    .expect("minima-safe billiard aggregation should succeed");
                let capacity = result.min_action;
                let sys = capacity * capacity / (2.0 * vol);
                let tied_orbits = active_orbits(&result.orbits, result.min_action)
                    .into_iter()
                    .map(|orbit| orbit_dump(&classification, &polytope, orbit))
                    .collect::<Vec<_>>();

                let row = ThetaRow {
                    angle_deg,
                    angle_rad: theta,
                    volume: vol,
                    area_q,
                    area_p,
                    capacity,
                    sys,
                    iterations: result.iterations,
                    n_returned_orbits: result.orbits.len(),
                    n_tied_orbits: tied_orbits.len(),
                    tied_orbits,
                };
                writeln!(
                    writer,
                    "{}",
                    serde_json::to_string(&row).expect("theta row serialization should succeed")
                )
                .expect("theta row write should succeed");
            }
            SweepMode::ThreeBounceBranches => {
                let (sigmas_examined, admissible_orbits) =
                    collect_admissible_three_bounce_orbits(&polytope)
                        .expect("3-bounce branch collection should succeed");
                let row = ThreeBounceThetaRow {
                    angle_deg,
                    angle_rad: theta,
                    volume: vol,
                    area_q,
                    area_p,
                    n_three_bounce_sigmas_examined: sigmas_examined,
                    n_admissible_three_bounce_orbits: admissible_orbits.len(),
                    admissible_three_bounce_orbits: admissible_orbits
                        .iter()
                        .map(|orbit| orbit_dump(&classification, &polytope, orbit))
                        .collect(),
                };
                writeln!(
                    writer,
                    "{}",
                    serde_json::to_string(&row).expect("3-bounce row serialization should succeed")
                )
                .expect("3-bounce row write should succeed");
            }
            SweepMode::BranchLandscape => unreachable!("landscape mode returns before minima loop"),
        }
    }

    writer.flush().expect("theta sweep flush should succeed");
    eprintln!("Wrote {}", output_path.display());
}

fn output_file_name(mode: SweepMode, canonical: bool) -> &'static str {
    match (mode, canonical) {
        (SweepMode::Minima, false) => "smoke-theta-sweep.jsonl",
        (SweepMode::Minima, true) => "theta-sweep.jsonl",
        (SweepMode::ThreeBounceBranches, false) => "smoke-three-bounce-branches.jsonl",
        (SweepMode::ThreeBounceBranches, true) => "three-bounce-branches.jsonl",
        (SweepMode::BranchLandscape, false) => "smoke-kkt-branch-landscape.jsonl",
        (SweepMode::BranchLandscape, true) => "kkt-branch-landscape.jsonl",
    }
}

fn output_path(mode: SweepMode, canonical: bool) -> PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("pentagon-rotation-empirics")
        .join(output_file_name(mode, canonical))
}

struct Cli {
    canonical: bool,
    mode: SweepMode,
    spike: bool,
    command_args: Vec<String>,
}

fn parse_cli() -> Cli {
    let command_args = env::args().collect::<Vec<_>>();
    let mut args = command_args.iter().skip(1);
    let mut canonical = false;
    let mut mode = SweepMode::Minima;
    let mut spike = false;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-h" | "--help" => {
                eprintln!(
                    "Usage: cargo run -p exp-regular-products --release --bin regular-pentagon-rotation-empirics -- [--canonical] [--three-bounce-branches | --branch-landscape [--spike]]"
                );
                eprintln!(
                    "  --canonical             Write canonical file names instead of smoke names."
                );
                eprintln!("  --three-bounce-branches Enable three-bounce branch dataset mode.");
                eprintln!(
                    "  --branch-landscape      Enable the frozen two-/three-block KKT landscape."
                );
                eprintln!("  --spike                 Use only 0, 9, and 18 degrees (branch landscape only).");
                eprintln!("Mode default: minima dataset over 0.0..36.0° with step 0.25°.");
                eprintln!("Defaults are smoke-style outputs when --canonical is not supplied:");
                eprintln!("  minima: smoke-theta-sweep.jsonl");
                eprintln!("  branches: smoke-three-bounce-branches.jsonl");
                eprintln!("  landscape spike: smoke-kkt-branch-landscape.jsonl");
                std::process::exit(0);
            }
            "--canonical" => canonical = true,
            "--three-bounce-branches" => mode = SweepMode::ThreeBounceBranches,
            "--branch-landscape" => mode = SweepMode::BranchLandscape,
            "--spike" => spike = true,
            _ => panic!("unknown argument: {arg}"),
        }
    }

    assert!(
        !spike || mode == SweepMode::BranchLandscape,
        "--spike is valid only with --branch-landscape"
    );
    assert!(
        !(spike && canonical),
        "--spike and --canonical are mutually exclusive"
    );
    if mode == SweepMode::BranchLandscape && !canonical {
        spike = true;
    }

    Cli {
        canonical,
        mode,
        spike,
        command_args,
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
enum LandscapeSolveStatus {
    Admissible,
    NumericallyInadmissible,
    Indeterminate,
    SolveFailure,
}

impl LandscapeSolveStatus {
    fn as_str(self) -> &'static str {
        match self {
            Self::Admissible => "admissible",
            Self::NumericallyInadmissible => "numerically_inadmissible",
            Self::Indeterminate => "indeterminate",
            Self::SolveFailure => "solve_failure",
        }
    }
}

#[derive(Debug, Serialize)]
struct LandscapeSampleOutcome {
    status: LandscapeSolveStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    action: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    action_lower: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    action_upper: Option<f64>,
    action_upper_unbounded: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    beta_margin: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    q: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    q_error_bound: Option<f64>,
}

#[derive(Debug, Serialize)]
struct LandscapeBranchRow {
    record_type: &'static str,
    raw_sigma_id: usize,
    sigma: Vec<usize>,
    block_count: usize,
    samples: Vec<LandscapeSampleOutcome>,
}

#[derive(Debug, Serialize)]
struct LandscapeNumericalContract {
    saddle_beta_feasibility_threshold: f64,
    saddle_beta_feasibility_rule: &'static str,
    beta_positive_threshold: f64,
    beta_indeterminate_band: [f64; 2],
    q_positive_threshold: f64,
    kkt_residual_threshold: f64,
    eigen_condition_ratio: f64,
    action_bound_fields: &'static str,
    status_rule: &'static str,
}

#[derive(Debug, Serialize)]
struct LandscapeMetadata {
    record_type: &'static str,
    schema_version: &'static str,
    epistemic_role: &'static str,
    run_kind: &'static str,
    generated_unix_millis: u128,
    command_args: Vec<String>,
    producer_source: &'static str,
    producer_source_sha256: String,
    source_sha256: BTreeMap<String, String>,
    git_revision: String,
    git_status_porcelain: Vec<String>,
    rustc_version: String,
    theta_domain_deg: [f64; 2],
    sample_angles_deg: Vec<f64>,
    canonical_step_deg: Option<f64>,
    frozen_universe_angle_deg: f64,
    frozen_universe_count: usize,
    frozen_universe_sha256: String,
    frozen_block_counts: BTreeMap<usize, usize>,
    generic_angle_set_checks_deg: Vec<f64>,
    generic_angle_set_checks_equal: bool,
    exact_family_expected_count: usize,
    exact_family_comparison: &'static str,
    display_action_cutoff: f64,
    numerical_contract: LandscapeNumericalContract,
}

#[derive(Debug, Serialize)]
struct LandscapeRunSummary {
    record_type: &'static str,
    completed: bool,
    branch_rows: usize,
    sample_angles: usize,
    expected_outcomes: usize,
    retained_outcomes: usize,
    status_counts: BTreeMap<&'static str, usize>,
    status_counts_by_block: BTreeMap<usize, BTreeMap<&'static str, usize>>,
    elapsed_seconds: f64,
    bytes_before_summary: u64,
}

fn run_branch_landscape(cli: &Cli) {
    let started = Instant::now();
    let sample_angles_deg = if cli.spike {
        vec![0.0, LANDSCAPE_FREEZE_DEG, LANDSCAPE_END_DEG]
    } else {
        sweep_angles(START_DEG, LANDSCAPE_END_DEG, LANDSCAPE_STEP_DEG)
    };

    let frozen_universe = landscape_universe_at(LANDSCAPE_FREEZE_DEG);
    let frozen_set = frozen_universe.iter().cloned().collect::<BTreeSet<_>>();
    assert_eq!(
        frozen_universe.len(),
        EXPECTED_OPEN_SIGMA_COUNT,
        "generic Rust universe must match the exact certificate's open-domain raw count"
    );
    assert_eq!(
        frozen_set.len(),
        EXPECTED_OPEN_SIGMA_COUNT,
        "generic Rust universe must contain unique raw sigmas"
    );

    let generic_angle_set_checks_deg = vec![4.5, LANDSCAPE_FREEZE_DEG, 13.5];
    let generic_angle_set_checks_equal = generic_angle_set_checks_deg.iter().all(|&angle| {
        landscape_universe_at(angle)
            .into_iter()
            .collect::<BTreeSet<_>>()
            == frozen_set
    });
    assert!(
        generic_angle_set_checks_equal,
        "Rust generator raw universe changed across generic open-domain angles"
    );

    let mut frozen_block_counts = BTreeMap::new();
    for sigma in &frozen_universe {
        *frozen_block_counts
            .entry(block_count_for_sigma(sigma))
            .or_insert(0usize) += 1;
    }
    assert_eq!(
        frozen_block_counts.values().sum::<usize>(),
        EXPECTED_OPEN_SIGMA_COUNT
    );
    assert!(
        frozen_block_counts.keys().copied().eq([2usize, 3usize]),
        "landscape universe must contain exactly the two- and three-block families"
    );

    let mut branches = frozen_universe
        .into_iter()
        .enumerate()
        .map(|(raw_sigma_id, sigma)| LandscapeBranchRow {
            record_type: "branch",
            raw_sigma_id,
            block_count: block_count_for_sigma(&sigma),
            sigma,
            samples: Vec::with_capacity(sample_angles_deg.len()),
        })
        .collect::<Vec<_>>();
    let status_names = [
        "admissible",
        "numerically_inadmissible",
        "indeterminate",
        "solve_failure",
    ];
    let mut status_counts = status_names
        .into_iter()
        .map(|status| (status, 0usize))
        .collect::<BTreeMap<_, _>>();
    let mut status_counts_by_block = [2usize, 3usize]
        .into_iter()
        .map(|block_count| {
            (
                block_count,
                status_names
                    .into_iter()
                    .map(|status| (status, 0usize))
                    .collect::<BTreeMap<_, _>>(),
            )
        })
        .collect::<BTreeMap<_, _>>();

    for &angle_deg in &sample_angles_deg {
        let polytope = pentagon_product_at(angle_deg);
        let classification = classify_facets_from_dual_vertices(&polytope.dual_vertices_f64)
            .expect("pentagon product should classify as a product");
        assert_eq!(classification.q_indices, vec![0, 1, 2, 3, 4]);
        assert_eq!(classification.p_indices, vec![5, 6, 7, 8, 9]);

        for branch in &mut branches {
            let outcome = landscape_outcome(&polytope.dual_vertices_f64, &branch.sigma);
            *status_counts
                .get_mut(outcome.status.as_str())
                .expect("all solve statuses must be predeclared") += 1;
            *status_counts_by_block
                .get_mut(&branch.block_count)
                .expect("two-/three-block status map must exist")
                .get_mut(outcome.status.as_str())
                .expect("all solve statuses must be predeclared") += 1;
            branch.samples.push(outcome);
        }
    }

    let output_path = output_path(SweepMode::BranchLandscape, cli.canonical);
    let file = File::create(&output_path).expect("cannot create KKT landscape output");
    let mut writer = BufWriter::new(file);
    let repo_root = repo_root();
    let producer_relative = "experiments/regular-products/pentagon-rotation-empirics/main.rs";
    let source_paths = [
        producer_relative,
        "experiments/regular-products/Cargo.toml",
        "crates/symplectic/src/algorithms/billiard/mod.rs",
        "crates/symplectic/src/algorithms/billiard/block_enumeration.rs",
        "crates/symplectic/src/algorithms/orbit_search.rs",
        "crates/symplectic/src/kkt/mod.rs",
        "crates/symplectic/src/kkt/saddle_point_solver.rs",
        "Cargo.lock",
    ];
    let source_sha256 = source_paths
        .iter()
        .map(|path| ((*path).to_string(), sha256_file(&repo_root.join(path))))
        .collect::<BTreeMap<_, _>>();
    let frozen_universe_payload = branches
        .iter()
        .map(|branch| &branch.sigma)
        .collect::<Vec<_>>();
    let frozen_universe_sha256 = sha256_bytes(
        &serde_json::to_vec(&frozen_universe_payload)
            .expect("frozen universe should serialize for identity hash"),
    );
    let metadata = LandscapeMetadata {
        record_type: "metadata",
        schema_version: "pentagon-kkt-branch-landscape-v1",
        epistemic_role: "sampled numerical explanation only; not a proof input",
        run_kind: if cli.spike { "three-angle-spike" } else { "canonical" },
        generated_unix_millis: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time should follow Unix epoch")
            .as_millis(),
        command_args: cli.command_args.clone(),
        producer_source: producer_relative,
        producer_source_sha256: source_sha256[producer_relative].clone(),
        source_sha256,
        git_revision: command_stdout(&repo_root, "git", &["rev-parse", "HEAD"]),
        git_status_porcelain: command_stdout(&repo_root, "git", &["status", "--porcelain=v1"])
            .lines()
            .map(str::to_string)
            .collect(),
        rustc_version: command_stdout(&repo_root, "rustc", &["--version"]),
        theta_domain_deg: [START_DEG, LANDSCAPE_END_DEG],
        sample_angles_deg: sample_angles_deg.clone(),
        canonical_step_deg: (!cli.spike).then_some(LANDSCAPE_STEP_DEG),
        frozen_universe_angle_deg: LANDSCAPE_FREEZE_DEG,
        frozen_universe_count: branches.len(),
        frozen_universe_sha256,
        frozen_block_counts,
        generic_angle_set_checks_deg,
        generic_angle_set_checks_equal,
        exact_family_expected_count: EXPECTED_OPEN_SIGMA_COUNT,
        exact_family_comparison: "count comparison only against the exact certificate's 3340-word open-domain family; no set comparison because the exact family is not retained as a non-Sage artifact and this packet is forbidden to invoke Sage",
        display_action_cutoff: LANDSCAPE_ACTION_CUTOFF,
        numerical_contract: LandscapeNumericalContract {
            saddle_beta_feasibility_threshold: 1e-12,
            saddle_beta_feasibility_rule: "the saddle solver rejects a unique solution with any beta <= -1e-12 before the later margin classifier; rank-deficient paths apply the same scale while searching or validating a feasible representative, so not every beta in the later +/-1e-9 band reaches indeterminate status",
            beta_positive_threshold: 1e-9,
            beta_indeterminate_band: [-1e-9, 1e-9],
            q_positive_threshold: 1e-15,
            kkt_residual_threshold: 1e-6,
            eigen_condition_ratio: 1e-3,
            action_bound_fields: "action_lower/action_upper are copied from the shared OrbitKktData residual diagnostic; the shared saddle-point source marks the underlying q_error_bound lemma as needing replacement before thesis-facing use, so this packet does not use these fields for tie, ordering, or lower-envelope claims; an unbounded upper diagnostic is represented by action_upper=null and action_upper_unbounded=true",
            status_rule: "Ok + AdmissibleF64/AdmissibleExact -> admissible; Ok + IndeterminateF64 -> indeterminate; OrbitSolveError::Inadmissible -> numerically_inadmissible; OrbitSolveError::NumericalFailure -> solve_failure",
        },
    };
    write_jsonl(&mut writer, &metadata);
    for branch in &branches {
        assert_eq!(branch.samples.len(), sample_angles_deg.len());
        write_jsonl(&mut writer, branch);
    }
    writer.flush().expect("KKT landscape flush should succeed");
    let bytes_before_summary = fs::metadata(&output_path)
        .expect("stat KKT landscape output before summary")
        .len();
    let retained_outcomes = branches.iter().map(|branch| branch.samples.len()).sum();
    let expected_outcomes = branches.len() * sample_angles_deg.len();
    assert_eq!(retained_outcomes, expected_outcomes);
    let summary = LandscapeRunSummary {
        record_type: "run_summary",
        completed: true,
        branch_rows: branches.len(),
        sample_angles: sample_angles_deg.len(),
        expected_outcomes,
        retained_outcomes,
        status_counts,
        status_counts_by_block,
        elapsed_seconds: started.elapsed().as_secs_f64(),
        bytes_before_summary,
    };
    write_jsonl(&mut writer, &summary);
    writer
        .flush()
        .expect("KKT landscape summary flush should succeed");

    let final_bytes = fs::metadata(&output_path)
        .expect("stat completed KKT landscape output")
        .len();
    eprintln!(
        "Wrote {} branches x {} angles = {} outcomes to {} ({} bytes) in {:.3}s; statuses={:?}",
        branches.len(),
        sample_angles_deg.len(),
        expected_outcomes,
        output_path.display(),
        final_bytes,
        started.elapsed().as_secs_f64(),
        summary.status_counts
    );
}

fn pentagon_product_at(angle_deg: f64) -> ProductPolytopeCache {
    let (qn, qh) = regular_polygon_2d(5, 1.0);
    let (pn_base, ph_base) = regular_polygon_2d(5, 1.0);
    let (pn, ph) = rotate_polygon_2d(&pn_base, &ph_base, angle_deg.to_radians());
    ProductPolytopeCache::from_lagrangian_product(&qn, &qh, &pn, &ph)
        .expect("pentagon product construction failed")
}

fn landscape_universe_at(angle_deg: f64) -> Vec<Vec<usize>> {
    let polytope = pentagon_product_at(angle_deg);
    let classification = classify_facets_from_dual_vertices(&polytope.dual_vertices_f64)
        .expect("pentagon product should classify as a product");
    let transition_is_allowed = transition_matrix(&polytope);
    let mut sigmas = Vec::new();
    for_each_sigma_from_facets(
        &classification.q_indices,
        &classification.p_indices,
        &polytope.facet_intersection_is_nonempty,
        &transition_is_allowed,
        |sigma| sigmas.push(sigma.to_vec()),
    );
    sigmas.sort();
    sigmas
}

fn block_count_for_sigma(sigma: &[usize]) -> usize {
    bounce_count_from_sigma_for_facets(&[0, 1, 2, 3, 4], &[5, 6, 7, 8, 9], sigma)
        .expect("frozen sigma must have the billiard block structure")
}

fn landscape_outcome(
    dual_vertices: &[nalgebra::Vector4<f64>],
    sigma: &[usize],
) -> LandscapeSampleOutcome {
    match solve_orbit_sigma_saddle_point(dual_vertices, sigma) {
        Ok(orbit) => {
            let status = match orbit.admissibility {
                OrbitAdmissibility::AdmissibleF64 | OrbitAdmissibility::AdmissibleExact => {
                    LandscapeSolveStatus::Admissible
                }
                OrbitAdmissibility::IndeterminateF64 => LandscapeSolveStatus::Indeterminate,
            };
            LandscapeSampleOutcome {
                status,
                action: finite(orbit.action),
                action_lower: finite(orbit.action_lower),
                action_upper: finite(orbit.action_upper),
                action_upper_unbounded: !orbit.action_upper.is_finite(),
                beta_margin: finite(orbit.beta_margin),
                q: finite(orbit.q),
                q_error_bound: finite(orbit.q_error_bound),
            }
        }
        Err(OrbitSolveError::Inadmissible) => {
            empty_landscape_outcome(LandscapeSolveStatus::NumericallyInadmissible)
        }
        Err(OrbitSolveError::NumericalFailure) => {
            empty_landscape_outcome(LandscapeSolveStatus::SolveFailure)
        }
    }
}

fn empty_landscape_outcome(status: LandscapeSolveStatus) -> LandscapeSampleOutcome {
    LandscapeSampleOutcome {
        status,
        action: None,
        action_lower: None,
        action_upper: None,
        action_upper_unbounded: false,
        beta_margin: None,
        q: None,
        q_error_bound: None,
    }
}

fn finite(value: f64) -> Option<f64> {
    value.is_finite().then_some(value)
}

fn write_jsonl(writer: &mut impl Write, value: &impl Serialize) {
    writeln!(
        writer,
        "{}",
        serde_json::to_string(value).expect("KKT landscape record should serialize")
    )
    .expect("KKT landscape record should write");
}

fn repo_root() -> PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(std::path::Path::parent)
        .expect("regular-products manifest should live below the repo root")
        .to_path_buf()
}

fn command_stdout(cwd: &std::path::Path, program: &str, args: &[&str]) -> String {
    let output = Command::new(program)
        .args(args)
        .current_dir(cwd)
        .output()
        .unwrap_or_else(|error| panic!("run {program}: {error}"));
    assert!(
        output.status.success(),
        "{program} {:?} failed: {}",
        args,
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout)
        .expect("command output should be UTF-8")
        .trim()
        .to_string()
}

fn sha256_file(path: &std::path::Path) -> String {
    command_stdout(
        path.parent().expect("hashed path should have a parent"),
        "sha256sum",
        &[path
            .file_name()
            .expect("hashed path should have a file name")
            .to_str()
            .expect("hashed file name should be UTF-8")],
    )
    .split_whitespace()
    .next()
    .expect("sha256sum should print a digest")
    .to_string()
}

fn sha256_bytes(bytes: &[u8]) -> String {
    let mut child = Command::new("sha256sum")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("spawn sha256sum for bytes");
    child
        .stdin
        .take()
        .expect("sha256sum stdin should be piped")
        .write_all(bytes)
        .expect("write bytes to sha256sum");
    let output = child.wait_with_output().expect("wait for sha256sum");
    assert!(
        output.status.success(),
        "sha256sum for bytes should succeed"
    );
    String::from_utf8(output.stdout)
        .expect("sha256sum output should be UTF-8")
        .split_whitespace()
        .next()
        .expect("sha256sum should print a digest")
        .to_string()
}

fn sweep_angles(start_deg: f64, end_deg: f64, step_deg: f64) -> Vec<f64> {
    const SNAP_TOL: f64 = 1e-9;
    let mut angles = Vec::new();
    let mut angle = start_deg;
    while angle <= end_deg + SNAP_TOL {
        angles.push(angle);
        angle += step_deg;
    }
    if (angles.last().unwrap_or(&start_deg) - end_deg).abs() > SNAP_TOL {
        angles.push(end_deg);
    }
    angles
}

fn active_orbits(orbits: &[OrbitKktData], min_action: f64) -> Vec<&OrbitKktData> {
    let tol = ACTIVE_ORBIT_RTOL * min_action.abs().max(1.0);
    orbits
        .iter()
        .filter(|orbit| {
            matches!(
                orbit.admissibility,
                OrbitAdmissibility::AdmissibleF64 | OrbitAdmissibility::AdmissibleExact
            )
        })
        .filter(|orbit| (orbit.action - min_action).abs() <= tol)
        .collect()
}

fn admissibility_name(admissibility: OrbitAdmissibility) -> &'static str {
    match admissibility {
        OrbitAdmissibility::AdmissibleF64 => "AdmissibleF64",
        OrbitAdmissibility::IndeterminateF64 => "IndeterminateF64",
        OrbitAdmissibility::AdmissibleExact => "AdmissibleExact",
    }
}

fn transition_matrix(polytope: &ProductPolytopeCache) -> nalgebra::DMatrix<bool> {
    symplectic::algorithms::facet_adjacency::build_transition_matrix_from_facet_intersections_and_omega(
        &polytope.facet_intersection_is_nonempty,
        &polytope.omega_signs,
    )
}

fn collect_minima_safe_billiard_result(
    polytope: &ProductPolytopeCache,
) -> Result<symplectic::OrbitSearchResult, OrbitSearchError> {
    let classification = classify_facets_from_dual_vertices(&polytope.dual_vertices_f64)
        .expect("valid pentagon family should classify as Lagrangian product");
    let dual_vertices_exact = &polytope.dual_vertices;
    let transition_is_allowed = transition_matrix(polytope);
    let (orbits, iterations) = solve_billiard_candidates(
        &polytope.dual_vertices_f64,
        &classification.q_indices,
        &classification.p_indices,
        &polytope.facet_intersection_is_nonempty,
        &transition_is_allowed,
    )?;
    aggregate_orbits_with_dual_vertices_exact(
        dual_vertices_exact,
        orbits,
        iterations,
        0.0,
        OrbitGuaranteeMode::MinimaSafe,
    )
}

fn collect_admissible_three_bounce_orbits(
    polytope: &ProductPolytopeCache,
) -> Result<(usize, Vec<OrbitKktData>), OrbitSearchError> {
    let dual_vertices = &polytope.dual_vertices_f64;
    let classification = classify_facets_from_dual_vertices(dual_vertices)
        .expect("valid pentagon family should classify as Lagrangian product");
    let transition_is_allowed = transition_matrix(polytope);
    let mut sigmas_examined = 0usize;
    let mut orbits = Vec::new();
    let mut fatal_error: Option<OrbitSearchError> = None;

    for_each_sigma_from_facets(
        &classification.q_indices,
        &classification.p_indices,
        &polytope.facet_intersection_is_nonempty,
        &transition_is_allowed,
        |sigma| {
            if fatal_error.is_some() {
                return;
            }

            let bounce_count = bounce_count_from_sigma_for_facets(
                &classification.q_indices,
                &classification.p_indices,
                sigma,
            );
            if bounce_count != Some(3) {
                return;
            }

            sigmas_examined += 1;
            match solve_orbit_sigma_saddle_point(dual_vertices, sigma) {
                Ok(orbit) => {
                    if matches!(
                        orbit.admissibility,
                        OrbitAdmissibility::AdmissibleF64 | OrbitAdmissibility::AdmissibleExact
                    ) {
                        orbits.push(orbit);
                    }
                }
                Err(OrbitSolveError::Inadmissible) => {}
                Err(OrbitSolveError::NumericalFailure) => {
                    fatal_error = Some(OrbitSearchError::NumericalFailure);
                }
            }
        },
    );

    if let Some(err) = fatal_error {
        return Err(err);
    }
    Ok((sigmas_examined, orbits))
}

fn orbit_dump(
    classification: &FacetClassification,
    polytope: &exp_regular_products::ProductPolytopeCache,
    orbit: &OrbitKktData,
) -> OrbitDump {
    let (q_blocks, p_blocks) = parse_sigma_blocks(classification, &orbit.sigma);
    OrbitDump {
        sigma: orbit.sigma.clone(),
        subset: orbit.best_subset(),
        beta: orbit.beta.clone(),
        beta_margin: orbit.beta_margin,
        action: orbit.action,
        action_lower: orbit.action_lower,
        action_upper: orbit.action_upper,
        q: orbit.q,
        q_error_bound: orbit.q_error_bound,
        admissibility: admissibility_name(orbit.admissibility).to_string(),
        bounces: bounce_count_from_sigma_for_facets(
            &classification.q_indices,
            &classification.p_indices,
            &orbit.sigma,
        ),
        q_blocks,
        p_blocks,
    }
}

fn parse_sigma_blocks(
    classification: &FacetClassification,
    sigma: &[usize],
) -> (Vec<AffineBlockDump>, Vec<AffineBlockDump>) {
    let mut q_blocks = Vec::new();
    let mut p_blocks = Vec::new();
    let mut i = 0usize;
    let mut expect_q = true;

    while i < sigma.len() {
        let current_is_q = classification.q_indices.contains(&sigma[i]);
        assert_eq!(current_is_q, expect_q, "sigma must alternate q/p blocks");

        let mut block = vec![relative_facet_index(classification, sigma[i], expect_q)];
        if i + 1 < sigma.len() {
            let next_is_q = classification.q_indices.contains(&sigma[i + 1]);
            if next_is_q == expect_q {
                block.push(relative_facet_index(classification, sigma[i + 1], expect_q));
                i += 1;
            }
        }

        if block.len() == 2 {
            block.sort_unstable();
        }

        let dump = AffineBlockDump {
            kind: if block.len() == 1 {
                "edge".to_string()
            } else {
                "vertex".to_string()
            },
            facets_rel: block,
        };
        if expect_q {
            q_blocks.push(dump);
        } else {
            p_blocks.push(dump);
        }

        expect_q = !expect_q;
        i += 1;
    }

    (q_blocks, p_blocks)
}

fn relative_facet_index(classification: &FacetClassification, facet: usize, q_type: bool) -> usize {
    let indices = if q_type {
        &classification.q_indices
    } else {
        &classification.p_indices
    };
    indices
        .iter()
        .position(|&idx| idx == facet)
        .expect("facet should belong to the requested type")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_landscape_grid_includes_both_endpoints() {
        let angles = sweep_angles(START_DEG, LANDSCAPE_END_DEG, LANDSCAPE_STEP_DEG);
        assert_eq!(angles.len(), 73);
        assert_eq!(angles.first(), Some(&0.0));
        assert_eq!(angles.last(), Some(&18.0));
    }

    #[test]
    fn generic_landscape_universe_is_stable_and_has_both_block_counts() {
        let frozen = landscape_universe_at(LANDSCAPE_FREEZE_DEG);
        assert_eq!(frozen.len(), EXPECTED_OPEN_SIGMA_COUNT);
        assert_eq!(
            frozen.iter().cloned().collect::<BTreeSet<_>>().len(),
            EXPECTED_OPEN_SIGMA_COUNT
        );
        for angle in [4.5, 13.5] {
            assert_eq!(landscape_universe_at(angle), frozen);
        }
        assert!(frozen.iter().any(|sigma| block_count_for_sigma(sigma) == 2));
        assert!(frozen.iter().any(|sigma| block_count_for_sigma(sigma) == 3));
    }

    #[test]
    fn landscape_status_names_are_the_schema_values() {
        assert_eq!(LandscapeSolveStatus::Admissible.as_str(), "admissible");
        assert_eq!(
            LandscapeSolveStatus::NumericallyInadmissible.as_str(),
            "numerically_inadmissible"
        );
        assert_eq!(
            LandscapeSolveStatus::Indeterminate.as_str(),
            "indeterminate"
        );
        assert_eq!(LandscapeSolveStatus::SolveFailure.as_str(), "solve_failure");
    }
}

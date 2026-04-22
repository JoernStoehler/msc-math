//! Pentagon-rotation formula sweep for `P_5 x_L R(theta) P_5`.
//!
//! Goal: record an owned per-theta dataset with all tied admissible minimal
//! orbits, so the branch structure behind the conjectured explicit formula can
//! be classified without depending on older experiment outputs.
//! Input Artifacts: None
//! Output Artifacts:
//!   - default smoke run: experiments/sys-landscape/pentagon-rotation-formula/smoke-theta-sweep.jsonl
//!   - canonical refresh with `--canonical`: experiments/sys-landscape/pentagon-rotation-formula/theta-sweep.jsonl
//!   - branch smoke run with `--three-bounce-branches`: experiments/sys-landscape/pentagon-rotation-formula/smoke-three-bounce-branches.jsonl
//!   - canonical branch refresh with `--three-bounce-branches --canonical`: experiments/sys-landscape/pentagon-rotation-formula/three-bounce-branches.jsonl
//!
//! Architecture:
//! 1. Build the regular pentagon product family over the fundamental domain.
//! 2. Run the billiard capacity frontend at each angle.
//! 3. Retain all admissible orbits tied at the minimum action within a relative
//!    tolerance aligned with the `sys-landscape` active-orbit surface.
//! 4. Write one JSONL row per angle with the tied orbit payloads.
//! 5. Optionally dump all admissible 3-bounce sigma solves for branch plots.
//!
//! Formal correspondence:
//! - The owned sweep is the computational sanity-check companion to
//!   [lem:pentagon-rotation-empirical-branch].
//! - The 3-bounce branch dump is the empirical surface for the open proof
//!   obligation [lem:pentagon-rotation-three-bounce].

use serde::Serialize;
use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use symplectic::algorithms::billiard::facet_classification::{
    classify_facets, FacetClassification,
};
use symplectic::algorithms::billiard::{bounce_count_from_sigma, for_each_sigma};
use symplectic::algorithms::{
    aggregate_orbits, solve_orbit_sigma, OrbitGuaranteeMode, OrbitSearchError, OrbitSolveBackend,
    OrbitSolveError,
};
use symplectic::geom::lagrangian_product::lagrangian_product;
use symplectic::geom::polygon::{polygon_area, regular_polygon_2d, rotate_polygon_2d};
use symplectic::geom::volume::volume;
use symplectic::{OrbitAdmissibility, OrbitKktData};

const START_DEG: f64 = 0.0;
const END_DEG: f64 = 36.0;
const STEP_DEG: f64 = 0.25;
const ACTIVE_ORBIT_RTOL: f64 = 1e-9;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum SweepMode {
    Minima,
    ThreeBounceBranches,
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
        let polytope =
            lagrangian_product(&qn, &qh, &pn, &ph).expect("pentagon product construction failed");
        let classification =
            classify_facets(&polytope).expect("pentagon product should classify as a product");
        let vol = volume(&polytope);
        match cli.mode {
            SweepMode::Minima => {
                let result = collect_minima_safe_billiard_result(&polytope)
                    .expect("minima-safe billiard aggregation should succeed");
                let capacity = result.capacity();
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
    }
}

fn output_path(mode: SweepMode, canonical: bool) -> PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("pentagon-rotation-formula")
        .join(output_file_name(mode, canonical))
}

struct Cli {
    canonical: bool,
    mode: SweepMode,
}

fn parse_cli() -> Cli {
    let mut args = env::args().skip(1);
    let mut canonical = false;
    let mut mode = SweepMode::Minima;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-h" | "--help" => {
                eprintln!(
                    "Usage: cargo run -p exp-sys-landscape --release --bin sys-pentagon-rotation-formula -- [--canonical] [--three-bounce-branches]"
                );
                eprintln!(
                    "  --canonical             Write canonical file names instead of smoke names."
                );
                eprintln!("  --three-bounce-branches Enable three-bounce branch dataset mode.");
                eprintln!("Mode default: minima dataset over 0.0..36.0° with step 0.25°.");
                eprintln!("Defaults are smoke-style outputs when --canonical is not supplied:");
                eprintln!("  minima: smoke-theta-sweep.jsonl");
                eprintln!("  branches: smoke-three-bounce-branches.jsonl");
                std::process::exit(0);
            }
            "--canonical" => canonical = true,
            "--three-bounce-branches" => mode = SweepMode::ThreeBounceBranches,
            _ => panic!("unknown argument: {arg}"),
        }
    }

    Cli { canonical, mode }
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

fn collect_minima_safe_billiard_result(
    polytope: &symplectic::Polytope4D,
) -> Result<symplectic::OrbitSearchResult, OrbitSearchError> {
    let mut orbits = Vec::new();
    let mut iterations = 0u64;
    let mut fatal_error: Option<OrbitSearchError> = None;

    for_each_sigma(polytope, |sigma| {
        if fatal_error.is_some() {
            return;
        }
        iterations += 1;
        match solve_orbit_sigma(polytope, sigma, OrbitSolveBackend::SaddlePoint) {
            Ok(orbit) => orbits.push(orbit),
            Err(OrbitSolveError::Inadmissible) => {}
            Err(OrbitSolveError::UnsupportedBackend) => {
                fatal_error = Some(OrbitSearchError::UnsupportedBackend);
            }
            Err(OrbitSolveError::NumericalFailure) => {
                fatal_error = Some(OrbitSearchError::NumericalFailure);
            }
        }
    })
    .expect("valid pentagon family should enumerate billiard sigmas");

    if let Some(err) = fatal_error {
        return Err(err);
    }
    aggregate_orbits(
        polytope,
        orbits,
        iterations,
        0.0,
        OrbitGuaranteeMode::MinimaSafe,
    )
}

fn collect_admissible_three_bounce_orbits(
    polytope: &symplectic::Polytope4D,
) -> Result<(usize, Vec<OrbitKktData>), OrbitSearchError> {
    let mut sigmas_examined = 0usize;
    let mut orbits = Vec::new();
    let mut fatal_error: Option<OrbitSearchError> = None;

    for_each_sigma(polytope, |sigma| {
        if fatal_error.is_some() {
            return;
        }

        let bounce_count = bounce_count_from_sigma(polytope, sigma)
            .expect("enumerated sigma should carry a valid bounce count");
        if bounce_count != Some(3) {
            return;
        }

        sigmas_examined += 1;
        match solve_orbit_sigma(polytope, sigma, OrbitSolveBackend::SaddlePoint) {
            Ok(orbit) => {
                if matches!(
                    orbit.admissibility,
                    OrbitAdmissibility::AdmissibleF64 | OrbitAdmissibility::AdmissibleExact
                ) {
                    orbits.push(orbit);
                }
            }
            Err(OrbitSolveError::Inadmissible) => {}
            Err(OrbitSolveError::UnsupportedBackend) => {
                fatal_error = Some(OrbitSearchError::UnsupportedBackend);
            }
            Err(OrbitSolveError::NumericalFailure) => {
                fatal_error = Some(OrbitSearchError::NumericalFailure);
            }
        }
    })
    .expect("valid pentagon family should enumerate billiard sigmas");

    if let Some(err) = fatal_error {
        return Err(err);
    }
    Ok((sigmas_examined, orbits))
}

fn orbit_dump(
    classification: &FacetClassification,
    polytope: &symplectic::Polytope4D,
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
        bounces: bounce_count_from_sigma(polytope, &orbit.sigma)
            .expect("bounce-count classification should succeed"),
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

//! All-minimum orbit validation on a diverse local-first target pool.
//!
//! Goal: validate the sigma/KKT side of the shared result layer: which minimum
//! simple orbits are returned for each selected polytope, and does the reported
//! minimum action agree with the root scalar route?
//!
//! Input Artifacts: None (builds the target pool internally).
//! Output Artifacts: experiments/verification/all-minimum/all-minimum.jsonl,
//!                   experiments/verification/all-minimum/all-minimum-orbits.jsonl,
//!                   experiments/verification/all-minimum/smoke-all-minimum.jsonl,
//!                   experiments/verification/all-minimum/smoke-all-minimum-orbits.jsonl

use dev_capacity_validation::{
    build_target_pool, RunMode, Target, MINIMUM_ACTION_GAP_TOL, SCALAR_TOL,
};
use serde::Serialize;
use std::collections::BTreeMap;
use std::env;
use std::fs::{create_dir_all, File};
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use std::time::Instant;
use symplectic::algorithms::hk2017::for_each_sigma_pruned;
use symplectic::algorithms::{
    aggregate_orbits, solve_orbit_sigma, OrbitAdmissibility, OrbitGuaranteeMode, OrbitKktData,
    OrbitSearchError, OrbitSolveBackend, OrbitSolveError,
};
use symplectic::ehz_capacity;

struct MinimumSetResult {
    orbits: Vec<OrbitKktData>,
    iterations: u64,
    min_action: f64,
    min_action_lower: f64,
    min_action_upper: f64,
    observed_action_max: f64,
    observed_action_spread: f64,
}

#[derive(Debug, Serialize)]
struct AllMinimumSummaryRow {
    name: String,
    family: String,
    source_kind: String,
    facet_count: usize,
    status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    failure_stage: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    failure_reasons: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expected_min_orbit_count: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_orbit_count: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    admissible_f64_count: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    admissible_exact_count: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hk_iterations: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_action: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_action_lower: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_action_upper: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    interval_width: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    observed_action_max: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    observed_action_spread: Option<f64>,
    minimum_gap_tolerance: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    scalar_capacity: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scalar_capacity_error: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scalar_matches: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    count_matches_expected: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    time_minimum_set_ms: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    time_scalar_check_ms: Option<f64>,
    passes_validation: bool,
}

#[derive(Debug, Serialize)]
struct AllMinimumOrbitRow {
    polytope_name: String,
    family: String,
    source_kind: String,
    orbit_index: usize,
    admissibility: String,
    sigma: Vec<usize>,
    subset: Vec<usize>,
    beta_margin: f64,
    action: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    action_lower: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    action_upper: Option<f64>,
    total_segments: usize,
}

struct RunPaths {
    mode: RunMode,
    summary_path: PathBuf,
    detail_path: PathBuf,
}

fn main() {
    let t0 = Instant::now();
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let run_paths = parse_run_paths(manifest_dir);
    let targets = build_target_pool(manifest_dir, run_paths.mode);

    create_dir_all(
        run_paths
            .summary_path
            .parent()
            .expect("summary output must have a parent"),
    )
    .expect("failed to create all-minimum output directory");

    let mut summary_writer = BufWriter::new(
        File::create(&run_paths.summary_path).expect("failed to create summary output"),
    );
    let mut detail_writer = BufWriter::new(
        File::create(&run_paths.detail_path).expect("failed to create detail output"),
    );

    eprintln!(
        "Mode: {}",
        match run_paths.mode {
            RunMode::Smoke => "smoke",
            RunMode::Full => "full",
        }
    );
    eprintln!("Target pool: {} polytopes", targets.len());

    let mut families = BTreeMap::<String, usize>::new();
    let mut failures = 0usize;

    for target in &targets {
        *families.entry(target.family.clone()).or_insert(0) += 1;
        let (summary, detail_rows) = validate_target(target);
        log_summary(&summary);
        if !summary.passes_validation {
            failures += 1;
        }
        write_json_line(&mut summary_writer, &summary);
        for row in &detail_rows {
            write_json_line(&mut detail_writer, row);
        }
    }

    summary_writer.flush().expect("flush summary output");
    detail_writer.flush().expect("flush detail output");

    eprintln!("\nFamilies:");
    for (family, count) in families {
        eprintln!("  {family}: {count}");
    }
    eprintln!(
        "\nDone: {} targets, {} failures, {:.1}s total",
        targets.len(),
        failures,
        t0.elapsed().as_secs_f64()
    );
    eprintln!(
        "Summary: {}\nDetails: {}",
        run_paths.summary_path.display(),
        run_paths.detail_path.display()
    );

    if failures > 0 {
        std::process::exit(1);
    }
}

fn parse_run_paths(manifest_dir: &Path) -> RunPaths {
    let mut args = env::args().skip(1);
    let mut full = false;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--full" => full = true,
            "--help" | "-h" => print_help_and_exit(),
            other => {
                eprintln!("unknown argument: {other}");
                print_help_and_exit();
            }
        }
    }

    let output_dir = manifest_dir.join("all-minimum");
    if full {
        RunPaths {
            mode: RunMode::Full,
            summary_path: output_dir.join("all-minimum.jsonl"),
            detail_path: output_dir.join("all-minimum-orbits.jsonl"),
        }
    } else {
        RunPaths {
            mode: RunMode::Smoke,
            summary_path: output_dir.join("smoke-all-minimum.jsonl"),
            detail_path: output_dir.join("smoke-all-minimum-orbits.jsonl"),
        }
    }
}

fn print_help_and_exit() -> ! {
    eprintln!("Usage: cargo run -p dev-capacity-validation --release --bin axioms-all-minimum [--full]");
    eprintln!("  default: write untracked smoke outputs for infrastructure checks");
    eprintln!("  --full: refresh the canonical local-first all-minimum dataset");
    std::process::exit(2);
}

fn validate_target(target: &Target) -> (AllMinimumSummaryRow, Vec<AllMinimumOrbitRow>) {
    let mut summary = AllMinimumSummaryRow {
        name: target.name.clone(),
        family: target.family.clone(),
        source_kind: target.source_kind.clone(),
        facet_count: target.polytope.facet_count(),
        status: "failed".to_string(),
        failure_stage: None,
        failure_reasons: Vec::new(),
        expected_min_orbit_count: target.expected_min_orbit_count,
        min_orbit_count: None,
        admissible_f64_count: None,
        admissible_exact_count: None,
        hk_iterations: None,
        min_action: None,
        min_action_lower: None,
        min_action_upper: None,
        interval_width: None,
        observed_action_max: None,
        observed_action_spread: None,
        minimum_gap_tolerance: MINIMUM_ACTION_GAP_TOL,
        scalar_capacity: None,
        scalar_capacity_error: None,
        scalar_matches: None,
        count_matches_expected: None,
        time_minimum_set_ms: None,
        time_scalar_check_ms: None,
        passes_validation: false,
    };

    let t_minimum = Instant::now();
    let minimum_result = match compute_minimum_orbits(&target.polytope) {
        Ok(result) => {
            summary.time_minimum_set_ms = Some(t_minimum.elapsed().as_secs_f64() * 1000.0);
            result
        }
        Err(err) => {
            summary.failure_stage = Some("minimum_set".to_string());
            summary
                .failure_reasons
                .push(format!("minimum-set computation failed: {err}"));
            return (summary, Vec::new());
        }
    };

    summary.min_orbit_count = Some(minimum_result.orbits.len());
    summary.hk_iterations = Some(minimum_result.iterations);
    summary.min_action = Some(minimum_result.min_action);
    summary.min_action_lower = Some(minimum_result.min_action_lower);
    summary.min_action_upper = Some(minimum_result.min_action_upper);
    summary.interval_width = Some(minimum_result.min_action_upper - minimum_result.min_action_lower);
    summary.observed_action_max = Some(minimum_result.observed_action_max);
    summary.observed_action_spread = Some(minimum_result.observed_action_spread);
    summary.admissible_f64_count = Some(
        minimum_result
            .orbits
            .iter()
            .filter(|orbit| orbit.admissibility == OrbitAdmissibility::AdmissibleF64)
            .count(),
    );
    summary.admissible_exact_count = Some(
        minimum_result
            .orbits
            .iter()
            .filter(|orbit| orbit.admissibility == OrbitAdmissibility::AdmissibleExact)
            .count(),
    );
    summary.count_matches_expected = target
        .expected_min_orbit_count
        .map(|expected| expected == minimum_result.orbits.len());

    let detail_rows = minimum_result
        .orbits
        .iter()
        .enumerate()
        .map(|(orbit_index, orbit)| AllMinimumOrbitRow {
            polytope_name: target.name.clone(),
            family: target.family.clone(),
            source_kind: target.source_kind.clone(),
            orbit_index,
            admissibility: admissibility_label(orbit).to_string(),
            sigma: orbit.sigma.clone(),
            subset: orbit.best_subset(),
            beta_margin: orbit.beta_margin,
            action: orbit.action,
            action_lower: finite_or_none(orbit.action_lower),
            action_upper: finite_or_none(orbit.action_upper),
            total_segments: orbit.sigma.len(),
        })
        .collect::<Vec<_>>();

    let t_scalar = Instant::now();
    match ehz_capacity(&target.polytope) {
        Ok(result) => {
            let scalar_capacity = result.capacity();
            let scalar_error = (minimum_result.min_action - scalar_capacity).abs();
            summary.scalar_capacity = Some(scalar_capacity);
            summary.scalar_capacity_error = Some(scalar_error);
            summary.scalar_matches = Some(scalar_error <= SCALAR_TOL);
            summary.time_scalar_check_ms = Some(t_scalar.elapsed().as_secs_f64() * 1000.0);
        }
        Err(err) => {
            summary.failure_stage = Some("scalar".to_string());
            summary
                .failure_reasons
                .push(format!("scalar cross-check failed: {err:?}"));
            summary.time_scalar_check_ms = Some(t_scalar.elapsed().as_secs_f64() * 1000.0);
        }
    }

    if summary.scalar_matches == Some(false) {
        summary.failure_stage.get_or_insert_with(|| "scalar".to_string());
        summary.failure_reasons.push(format!(
            "scalar capacity mismatch: |{} - {}| = {:.2e}",
            summary.min_action.unwrap_or(f64::NAN),
            summary.scalar_capacity.unwrap_or(f64::NAN),
            summary.scalar_capacity_error.unwrap_or(f64::NAN),
        ));
    }
    if summary.count_matches_expected == Some(false) {
        summary.failure_stage.get_or_insert_with(|| "validation".to_string());
        summary.failure_reasons.push(format!(
            "expected {} minimum orbits, got {}",
            target.expected_min_orbit_count.unwrap_or(0),
            summary.min_orbit_count.unwrap_or(0),
        ));
    }

    summary.passes_validation =
        summary.failure_stage.is_none() && summary.scalar_matches == Some(true);
    summary.status = if summary.passes_validation {
        "ok".to_string()
    } else {
        "failed".to_string()
    };

    (summary, detail_rows)
}

fn compute_minimum_orbits(polytope: &symplectic::Polytope4D) -> Result<MinimumSetResult, String> {
    let mut orbits = Vec::<OrbitKktData>::new();
    let mut iterations = 0u64;
    let mut fatal_error = None::<String>;

    for_each_sigma_pruned(polytope, |sigma| {
        if fatal_error.is_some() {
            return;
        }
        iterations += 1;
        match solve_orbit_sigma(polytope, sigma, OrbitSolveBackend::SaddlePoint) {
            Ok(orbit) => orbits.push(orbit),
            Err(OrbitSolveError::Inadmissible) => {}
            Err(OrbitSolveError::UnsupportedBackend) => {
                fatal_error = Some("solve_orbit_sigma returned UnsupportedBackend".to_string())
            }
            Err(OrbitSolveError::NumericalFailure) => {
                fatal_error = Some(format!("solve_orbit_sigma failed on sigma {:?}", sigma))
            }
        }
    });

    if let Some(err) = fatal_error {
        return Err(err);
    }

    let result = aggregate_orbits(
        polytope,
        orbits,
        iterations,
        MINIMUM_ACTION_GAP_TOL,
        OrbitGuaranteeMode::MinimaSafe,
    )
    .map_err(|err| match err {
        OrbitSearchError::NoAdmissibleOrbit => "no admissible orbit remained".to_string(),
        OrbitSearchError::UnsupportedBackend => {
            "aggregate_orbits reported UnsupportedBackend".to_string()
        }
        OrbitSearchError::NumericalFailure => {
            "aggregate_orbits reported NumericalFailure".to_string()
        }
        OrbitSearchError::ExactFallbackFailure => {
            "aggregate_orbits reported ExactFallbackFailure".to_string()
        }
    })?;

    let minimum_orbits = result
        .orbits
        .iter()
        .filter(|orbit| orbit.action <= result.min_action + MINIMUM_ACTION_GAP_TOL)
        .cloned()
        .collect::<Vec<_>>();
    let observed_action_max = minimum_orbits
        .iter()
        .map(|orbit| orbit.action)
        .max_by(f64::total_cmp)
        .expect("aggregate_orbits should return at least one minimum orbit");

    Ok(MinimumSetResult {
        orbits: minimum_orbits,
        iterations: result.iterations,
        min_action: result.min_action,
        min_action_lower: result.min_action_lower,
        min_action_upper: result.min_action_upper,
        observed_action_max,
        observed_action_spread: observed_action_max - result.min_action,
    })
}

fn admissibility_label(orbit: &OrbitKktData) -> &'static str {
    match orbit.admissibility {
        OrbitAdmissibility::AdmissibleF64 => "AdmissibleF64",
        OrbitAdmissibility::IndeterminateF64 => "IndeterminateF64",
        OrbitAdmissibility::AdmissibleExact => "AdmissibleExact",
    }
}

fn finite_or_none(value: f64) -> Option<f64> {
    value.is_finite().then_some(value)
}

fn log_summary(summary: &AllMinimumSummaryRow) {
    if summary.passes_validation {
        eprintln!(
            "  {} [{}] minima={} spread={:.2e} exact={} OK",
            summary.name,
            summary.family,
            summary.min_orbit_count.unwrap_or(0),
            summary.observed_action_spread.unwrap_or(0.0),
            summary.admissible_exact_count.unwrap_or(0),
        );
        return;
    }

    eprintln!(
        "  FAIL {} [{}] stage={} {}",
        summary.name,
        summary.family,
        summary.failure_stage.as_deref().unwrap_or("unknown"),
        summary.failure_reasons.join("; "),
    );
}

fn write_json_line<T: Serialize>(writer: &mut BufWriter<File>, row: &T) {
    serde_json::to_writer(&mut *writer, row).expect("serialize all-minimum row");
    writeln!(&mut *writer).expect("write all-minimum newline");
}

//! Finite local-ascent diagnostics for the currently known sys=1 panel.
//!
//! The output is empirical fixed-facet evidence. A finite miss is not a
//! local-maximality theorem; see the owner-local README for the claim boundary.

mod directions;
mod seeds;

use directions::{
    l2_norm, orientation_perturbation, perturb_linearly, quotient_basis, random_quotient_directions,
};
use exp_sys_landscape::{compute_sys_computation, SysComputation, SysLandscapePolytopeCache};
use nalgebra::Vector4;
use rayon::prelude::*;
use seeds::{known_equality_seeds, product_dual_vertices, Seed};
use serde::Serialize;
use std::collections::BTreeMap;
use std::env;
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

const SCHEMA_VERSION: u32 = 1;
const MASTER_SEED: u64 = 0x51_51_2026;
const ROW_RELATIVE_RADII: &[f64] = &[1.0e-3, 1.0e-4, 1.0e-5];
const ANGULAR_RADII: &[f64] = &[1.0e-2, 1.0e-3, 1.0e-4];
const RANDOM_ANTIPODAL_PAIRS: usize = 32;
const ORIENTATION_DIRECTIONS: usize = 16;
const BASE_EQUALITY_TOLERANCE: f64 = 2.0e-9;
const MATERIAL_DELTA_SYS: f64 = 1.0e-12;

#[derive(Debug)]
struct Cli {
    canonical: bool,
    smoke: bool,
    out_dir: PathBuf,
    threads: usize,
    command_args: Vec<String>,
}

#[derive(Clone)]
struct BaseEvaluation {
    seed: Seed,
    polytope: SysLandscapePolytopeCache,
    computation: SysComputation,
    incidence_signature: Vec<String>,
    sys_lower: f64,
    sys_upper: f64,
    dual_norm: f64,
}

#[derive(Debug)]
struct ProbeTask {
    seed_index: usize,
    perturbation: &'static str,
    radius_kind: &'static str,
    radius: f64,
    direction_index: usize,
    sign: i8,
    parameters: BTreeMap<String, f64>,
    perturbed_dual_vertices: Vec<Vector4<f64>>,
}

#[derive(Debug, Serialize)]
struct BaseRow {
    schema_version: u32,
    seed_id: String,
    role: String,
    source: String,
    expected_sys: f64,
    recomputed_sys: f64,
    recomputed_minus_expected: f64,
    equality_tolerance: f64,
    equality_check_passed: bool,
    facet_count: usize,
    vertex_count: usize,
    product_q_sides: Option<usize>,
    product_p_sides: Option<usize>,
    product_theta_rad: Option<f64>,
    product_theta_deg: Option<f64>,
    volume: f64,
    min_action: f64,
    min_action_lower: f64,
    min_action_upper: f64,
    sys_lower: f64,
    sys_upper: f64,
    returned_orbit_count: usize,
    orbit_iterations: u64,
    best_sigma: Vec<usize>,
    capacity_interval_width: f64,
    dual_norm: f64,
    ambient_dimension: usize,
    orbit_generator_count: usize,
    orbit_rank: usize,
    quotient_dimension: usize,
    max_orbit_orthonormal_error: f64,
    max_slice_orthonormal_error: f64,
    max_orbit_slice_inner_product: f64,
    incidence_signature: Vec<String>,
    dual_vertices: Vec<[f64; 4]>,
}

#[derive(Debug, Serialize)]
struct ProbeRow {
    schema_version: u32,
    seed_id: String,
    role: String,
    perturbation: String,
    radius_kind: String,
    radius: f64,
    direction_index: usize,
    sign: i8,
    parameters: BTreeMap<String, f64>,
    step_norm: f64,
    relative_step_norm: f64,
    base_sys: f64,
    base_sys_lower: f64,
    base_sys_upper: f64,
    state_valid: bool,
    failure: Option<String>,
    perturbed_sys: Option<f64>,
    perturbed_sys_lower: Option<f64>,
    perturbed_sys_upper: Option<f64>,
    delta_sys: Option<f64>,
    delta_sys_per_step: Option<f64>,
    raw_positive_delta: bool,
    nominal_improvement: bool,
    lower_bound_above_one: bool,
    lower_bound_above_base_upper: bool,
    facet_count: Option<usize>,
    vertex_count: Option<usize>,
    all_facets_defining: bool,
    same_incidence_signature: bool,
    volume: Option<f64>,
    min_action: Option<f64>,
    min_action_lower: Option<f64>,
    min_action_upper: Option<f64>,
    returned_orbit_count: Option<usize>,
    orbit_iterations: Option<u64>,
    best_sigma: Option<Vec<usize>>,
    perturbed_dual_vertices: Vec<[f64; 4]>,
    delta_dual_vertices: Vec<[f64; 4]>,
    wall_seconds: f64,
}

#[derive(Debug, Serialize)]
struct RadiusSummary {
    schema_version: u32,
    seed_id: String,
    role: String,
    perturbation: String,
    radius_kind: String,
    radius: f64,
    total_probes: usize,
    valid_probes: usize,
    invalid_probes: usize,
    incidence_change_probes: usize,
    nominal_improving_probes: usize,
    lower_bound_above_one_probes: usize,
    lower_bound_above_base_upper_probes: usize,
    max_delta_sys: Option<f64>,
    min_delta_sys: Option<f64>,
    best_direction_index: Option<usize>,
    best_sign: Option<i8>,
    finite_poll_status: String,
}

#[derive(Debug, Serialize)]
struct Provenance {
    schema_version: u32,
    experiment_id: &'static str,
    command_args: Vec<String>,
    canonical: bool,
    smoke: bool,
    git_revision: String,
    git_status_porcelain: Vec<String>,
    master_seed: u64,
    row_relative_radii: Vec<f64>,
    angular_radii: Vec<f64>,
    random_antipodal_pairs: usize,
    orientation_directions: usize,
    material_delta_sys: f64,
    threads: usize,
    base_count: usize,
    probe_count: usize,
    total_wall_seconds: f64,
    source_paths: Vec<&'static str>,
    claim_boundary: &'static str,
}

fn main() {
    let cli = parse_args(env::args().collect());
    let started = Instant::now();
    fs::create_dir_all(&cli.out_dir)
        .unwrap_or_else(|err| panic!("create output directory {}: {err}", cli.out_dir.display()));

    let selected_seeds = if cli.smoke {
        known_equality_seeds()
            .into_iter()
            .take(1)
            .collect::<Vec<_>>()
    } else {
        known_equality_seeds()
    };
    let bases = selected_seeds
        .into_iter()
        .map(evaluate_base)
        .collect::<Vec<_>>();

    let (base_rows, tasks) = build_rows_and_tasks(&bases, cli.smoke);
    eprintln!(
        "Evaluating {} probes for {} bases with {} threads",
        tasks.len(),
        bases.len(),
        cli.threads
    );
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(cli.threads)
        .build()
        .expect("build Rayon thread pool");
    let mut probe_rows = pool.install(|| {
        tasks
            .par_iter()
            .map(|task| evaluate_probe(task, &bases[task.seed_index]))
            .collect::<Vec<_>>()
    });
    probe_rows.sort_by(|left, right| probe_identity(left).cmp(&probe_identity(right)));
    let radius_summaries = summarize(&probe_rows);

    write_jsonl_atomic(cli.out_dir.join("bases.jsonl"), &base_rows);
    write_jsonl_atomic(cli.out_dir.join("probes.jsonl"), &probe_rows);
    write_jsonl_atomic(
        cli.out_dir.join("radius-summaries.jsonl"),
        &radius_summaries,
    );
    let provenance = Provenance {
        schema_version: SCHEMA_VERSION,
        experiment_id: "sys1-known-equality-local-maxima-v1",
        command_args: cli.command_args.clone(),
        canonical: cli.canonical,
        smoke: cli.smoke,
        git_revision: git_output(&["rev-parse", "HEAD"]).unwrap_or_else(|| "unknown".into()),
        git_status_porcelain: git_output(&[
            "status",
            "--porcelain",
            "--untracked-files=all",
        ])
        .unwrap_or_default()
        .lines()
        .map(str::to_owned)
        .collect(),
        master_seed: MASTER_SEED,
        row_relative_radii: active_row_radii(cli.smoke).to_vec(),
        angular_radii: active_angular_radii(cli.smoke).to_vec(),
        random_antipodal_pairs: if cli.smoke { 0 } else { RANDOM_ANTIPODAL_PAIRS },
        orientation_directions: if cli.smoke { 0 } else { ORIENTATION_DIRECTIONS },
        material_delta_sys: MATERIAL_DELTA_SYS,
        threads: cli.threads,
        base_count: bases.len(),
        probe_count: probe_rows.len(),
        total_wall_seconds: started.elapsed().as_secs_f64(),
        source_paths: vec![
            "experiments/local-maxima-check/README.md",
            "thesis/09-rotated-regular-polygons-pentagon-profile-theorem.tex",
            "experiments/regular-products/rotated-regular-products/lagrangian-products-3x6-6deg.jsonl",
            "experiments/regular-products/rotated-regular-products/lagrangian-products-4x4-6deg.jsonl",
            "papers/ch2021/s1_introduction_and_main_results.tex",
            "experiments/verification/ch2021-six-vertex/report.json",
        ],
        claim_boundary: "Finite fixed-facet probes can find improving points but cannot prove local maximality from a miss.",
    };
    write_json_atomic(cli.out_dir.join("run-provenance.json"), &provenance);
    eprintln!(
        "Wrote {} bases and {} probes to {} in {:.2}s",
        base_rows.len(),
        probe_rows.len(),
        cli.out_dir.display(),
        started.elapsed().as_secs_f64()
    );
}

fn evaluate_base(seed: Seed) -> BaseEvaluation {
    let (polytope, computation) = compute_state(&seed.dual_vertices)
        .unwrap_or_else(|failure| panic!("base {} failed: {failure}", seed.id));
    assert_eq!(
        polytope.facet_count(),
        seed.dual_vertices.len(),
        "base {} lost a facet",
        seed.id
    );
    let difference = (computation.sys - seed.expected_sys).abs();
    assert!(
        difference <= BASE_EQUALITY_TOLERANCE,
        "base {} recomputed sys {} differs from expected {} by {}",
        seed.id,
        computation.sys,
        seed.expected_sys,
        difference
    );
    let (sys_lower, sys_upper) = sys_interval(&computation);
    BaseEvaluation {
        incidence_signature: incidence_signature(&polytope.vertex_facet_incidence),
        dual_norm: l2_norm(&seed.dual_vertices),
        seed,
        polytope,
        computation,
        sys_lower,
        sys_upper,
    }
}

fn build_rows_and_tasks(bases: &[BaseEvaluation], smoke: bool) -> (Vec<BaseRow>, Vec<ProbeTask>) {
    let mut rows = Vec::with_capacity(bases.len());
    let mut tasks = Vec::new();
    for (seed_index, base) in bases.iter().enumerate() {
        let quotient = quotient_basis(&base.seed.dual_vertices);
        let product = base.seed.product.as_ref();
        rows.push(BaseRow {
            schema_version: SCHEMA_VERSION,
            seed_id: base.seed.id.to_owned(),
            role: base.seed.role.to_owned(),
            source: base.seed.source.to_owned(),
            expected_sys: base.seed.expected_sys,
            recomputed_sys: base.computation.sys,
            recomputed_minus_expected: base.computation.sys - base.seed.expected_sys,
            equality_tolerance: BASE_EQUALITY_TOLERANCE,
            equality_check_passed: true,
            facet_count: base.polytope.facet_count(),
            vertex_count: base.polytope.vertices.len(),
            product_q_sides: product.map(|spec| spec.q_sides),
            product_p_sides: product.map(|spec| spec.p_sides),
            product_theta_rad: product.map(|spec| spec.theta_rad),
            product_theta_deg: product.map(|spec| spec.theta_rad.to_degrees()),
            volume: base.computation.vol,
            min_action: base.computation.capacity.min_action,
            min_action_lower: base.computation.capacity.min_action_lower,
            min_action_upper: base.computation.capacity.min_action_upper,
            sys_lower: base.sys_lower,
            sys_upper: base.sys_upper,
            returned_orbit_count: base.computation.capacity.orbits.len(),
            orbit_iterations: base.computation.capacity.iterations,
            best_sigma: base.computation.capacity.best_sigma().to_vec(),
            capacity_interval_width: base.computation.capacity.min_action_upper
                - base.computation.capacity.min_action_lower,
            dual_norm: base.dual_norm,
            ambient_dimension: 4 * base.polytope.facet_count(),
            orbit_generator_count: quotient.orbit_generator_count,
            orbit_rank: quotient.orbit_basis.len(),
            quotient_dimension: quotient.slice_basis.len(),
            max_orbit_orthonormal_error: quotient.max_orbit_orthonormal_error,
            max_slice_orthonormal_error: quotient.max_slice_orthonormal_error,
            max_orbit_slice_inner_product: quotient.max_cross_inner_product,
            incidence_signature: base.incidence_signature.clone(),
            dual_vertices: vectors_to_arrays(&base.seed.dual_vertices),
        });

        let row_radii = active_row_radii(smoke);
        let basis_limit = if smoke { 1 } else { quotient.slice_basis.len() };
        for &radius in row_radii {
            for (direction_index, direction) in
                quotient.slice_basis.iter().take(basis_limit).enumerate()
            {
                for sign in [-1_i8, 1_i8] {
                    let signed = direction * f64::from(sign);
                    tasks.push(ProbeTask {
                        seed_index,
                        perturbation: "quotient_basis",
                        radius_kind: "relative_row_l2",
                        radius,
                        direction_index,
                        sign,
                        parameters: BTreeMap::new(),
                        perturbed_dual_vertices: perturb_linearly(
                            &base.seed.dual_vertices,
                            &signed,
                            radius * base.dual_norm,
                        ),
                    });
                }
            }
        }

        if !smoke {
            let random = random_quotient_directions(
                &quotient,
                RANDOM_ANTIPODAL_PAIRS,
                seed_for(base.seed.id),
            );
            for &radius in row_radii {
                for (direction_index, direction) in random.iter().enumerate() {
                    for sign in [-1_i8, 1_i8] {
                        let signed = direction * f64::from(sign);
                        tasks.push(ProbeTask {
                            seed_index,
                            perturbation: "quotient_random_antipodal",
                            radius_kind: "relative_row_l2",
                            radius,
                            direction_index,
                            sign,
                            parameters: BTreeMap::new(),
                            perturbed_dual_vertices: perturb_linearly(
                                &base.seed.dual_vertices,
                                &signed,
                                radius * base.dual_norm,
                            ),
                        });
                    }
                }
            }
            for &radius in active_angular_radii(smoke) {
                for direction_index in 0..ORIENTATION_DIRECTIONS {
                    let phi = std::f64::consts::TAU * direction_index as f64
                        / ORIENTATION_DIRECTIONS as f64;
                    tasks.push(ProbeTask {
                        seed_index,
                        perturbation: "so4_mod_u2_orientation",
                        radius_kind: "angle_rad",
                        radius,
                        direction_index,
                        sign: 1,
                        parameters: BTreeMap::from([("phi_rad".to_owned(), phi)]),
                        perturbed_dual_vertices: orientation_perturbation(
                            &base.seed.dual_vertices,
                            radius,
                            phi,
                        ),
                    });
                }
            }
        }

        if let Some(spec) = &base.seed.product {
            for &radius in active_angular_radii(smoke) {
                for sign in [-1_i8, 1_i8] {
                    let mut perturbed_spec = spec.clone();
                    perturbed_spec.theta_rad += f64::from(sign) * radius;
                    tasks.push(ProbeTask {
                        seed_index,
                        perturbation: "product_relative_rotation",
                        radius_kind: "angle_rad",
                        radius,
                        direction_index: 0,
                        sign,
                        parameters: BTreeMap::from([(
                            "theta_rad".to_owned(),
                            perturbed_spec.theta_rad,
                        )]),
                        perturbed_dual_vertices: product_dual_vertices(&perturbed_spec),
                    });
                }
            }
        }
    }
    (rows, tasks)
}

fn evaluate_probe(task: &ProbeTask, base: &BaseEvaluation) -> ProbeRow {
    let started = Instant::now();
    let step_norm = l2_distance(&task.perturbed_dual_vertices, &base.seed.dual_vertices);
    let delta_duals = task
        .perturbed_dual_vertices
        .iter()
        .zip(&base.seed.dual_vertices)
        .map(|(perturbed, original)| perturbed - original)
        .collect::<Vec<_>>();
    let common = |state_valid: bool,
                  failure: Option<String>,
                  perturbed_sys: Option<f64>,
                  perturbed_sys_lower: Option<f64>,
                  perturbed_sys_upper: Option<f64>,
                  polytope: Option<&SysLandscapePolytopeCache>,
                  computation: Option<&SysComputation>| {
        let delta_sys = perturbed_sys.map(|sys| sys - base.computation.sys);
        let signature = polytope.map(|poly| incidence_signature(&poly.vertex_facet_incidence));
        ProbeRow {
            schema_version: SCHEMA_VERSION,
            seed_id: base.seed.id.to_owned(),
            role: base.seed.role.to_owned(),
            perturbation: task.perturbation.to_owned(),
            radius_kind: task.radius_kind.to_owned(),
            radius: task.radius,
            direction_index: task.direction_index,
            sign: task.sign,
            parameters: task.parameters.clone(),
            step_norm,
            relative_step_norm: step_norm / base.dual_norm,
            base_sys: base.computation.sys,
            base_sys_lower: base.sys_lower,
            base_sys_upper: base.sys_upper,
            state_valid,
            failure,
            perturbed_sys,
            perturbed_sys_lower,
            perturbed_sys_upper,
            delta_sys,
            delta_sys_per_step: delta_sys.map(|delta| delta / step_norm),
            raw_positive_delta: delta_sys.is_some_and(|delta| delta > 0.0),
            nominal_improvement: delta_sys.is_some_and(|delta| delta > MATERIAL_DELTA_SYS),
            lower_bound_above_one: perturbed_sys_lower.is_some_and(|value| value > 1.0),
            lower_bound_above_base_upper: perturbed_sys_lower
                .is_some_and(|value| value > base.sys_upper),
            facet_count: polytope.map(SysLandscapePolytopeCache::facet_count),
            vertex_count: polytope.map(|poly| poly.vertices.len()),
            all_facets_defining: polytope
                .is_some_and(|poly| poly.facet_count() == base.seed.dual_vertices.len()),
            same_incidence_signature: signature
                .is_some_and(|value| value == base.incidence_signature),
            volume: computation.map(|value| value.vol),
            min_action: computation.map(|value| value.capacity.min_action),
            min_action_lower: computation.map(|value| value.capacity.min_action_lower),
            min_action_upper: computation.map(|value| value.capacity.min_action_upper),
            returned_orbit_count: computation.map(|value| value.capacity.orbits.len()),
            orbit_iterations: computation.map(|value| value.capacity.iterations),
            best_sigma: computation.map(|value| value.capacity.best_sigma().to_vec()),
            perturbed_dual_vertices: vectors_to_arrays(&task.perturbed_dual_vertices),
            delta_dual_vertices: vectors_to_arrays(&delta_duals),
            wall_seconds: started.elapsed().as_secs_f64(),
        }
    };
    match compute_state(&task.perturbed_dual_vertices) {
        Ok((polytope, computation)) => {
            let (lower, upper) = sys_interval(&computation);
            common(
                true,
                None,
                Some(computation.sys),
                Some(lower),
                Some(upper),
                Some(&polytope),
                Some(&computation),
            )
        }
        Err(failure) => common(false, Some(failure), None, None, None, None, None),
    }
}

fn summarize(rows: &[ProbeRow]) -> Vec<RadiusSummary> {
    let mut groups: BTreeMap<(String, String, String, String, u64), Vec<&ProbeRow>> =
        BTreeMap::new();
    for row in rows {
        groups
            .entry((
                row.seed_id.clone(),
                row.role.clone(),
                row.perturbation.clone(),
                row.radius_kind.clone(),
                row.radius.to_bits(),
            ))
            .or_default()
            .push(row);
    }
    groups
        .into_iter()
        .map(
            |((seed_id, role, perturbation, radius_kind, radius_bits), selected)| {
                let valid = selected
                    .iter()
                    .copied()
                    .filter(|row| row.state_valid)
                    .collect::<Vec<_>>();
                let best = valid.iter().copied().max_by(|left, right| {
                    left.delta_sys.unwrap().total_cmp(&right.delta_sys.unwrap())
                });
                let invalid = selected.len() - valid.len();
                let incidence_changes = valid
                    .iter()
                    .filter(|row| !row.same_incidence_signature)
                    .count();
                let nominal = valid.iter().filter(|row| row.nominal_improvement).count();
                let above_one = valid.iter().filter(|row| row.lower_bound_above_one).count();
                let above_base = valid
                    .iter()
                    .filter(|row| row.lower_bound_above_base_upper)
                    .count();
                let status = if invalid > 0 {
                    "incomplete_invalid_probe"
                } else if incidence_changes > 0 {
                    "combinatorial_change_observed"
                } else if above_base > 0 {
                    "interval_separated_improvement"
                } else if nominal > 0 {
                    "nominal_improvement_only"
                } else {
                    "no_improvement_observed"
                };
                RadiusSummary {
                    schema_version: SCHEMA_VERSION,
                    seed_id,
                    role,
                    perturbation,
                    radius_kind,
                    radius: f64::from_bits(radius_bits),
                    total_probes: selected.len(),
                    valid_probes: valid.len(),
                    invalid_probes: invalid,
                    incidence_change_probes: incidence_changes,
                    nominal_improving_probes: nominal,
                    lower_bound_above_one_probes: above_one,
                    lower_bound_above_base_upper_probes: above_base,
                    max_delta_sys: valid
                        .iter()
                        .map(|row| row.delta_sys.unwrap())
                        .max_by(f64::total_cmp),
                    min_delta_sys: valid
                        .iter()
                        .map(|row| row.delta_sys.unwrap())
                        .min_by(f64::total_cmp),
                    best_direction_index: best.map(|row| row.direction_index),
                    best_sign: best.map(|row| row.sign),
                    finite_poll_status: status.to_owned(),
                }
            },
        )
        .collect()
}

fn compute_state(
    dual_vertices: &[Vector4<f64>],
) -> Result<(SysLandscapePolytopeCache, SysComputation), String> {
    let polytope = SysLandscapePolytopeCache::from_f64_dual_vertices(dual_vertices.to_vec())
        .ok_or_else(|| "invalid_geometry_or_redundant_facet".to_owned())?;
    let computation = compute_sys_computation(&polytope)
        .ok_or_else(|| "full_sys_computation_failed".to_owned())?;
    Ok((polytope, computation))
}

fn sys_interval(computation: &SysComputation) -> (f64, f64) {
    let denominator = 2.0 * computation.vol;
    (
        computation.capacity.min_action_lower.powi(2) / denominator,
        computation.capacity.min_action_upper.powi(2) / denominator,
    )
}

fn incidence_signature(matrix: &nalgebra::DMatrix<bool>) -> Vec<String> {
    (0..matrix.nrows())
        .map(|row| {
            (0..matrix.ncols())
                .map(|column| if matrix[(row, column)] { '1' } else { '0' })
                .collect()
        })
        .collect()
}

fn vectors_to_arrays(vectors: &[Vector4<f64>]) -> Vec<[f64; 4]> {
    vectors.iter().map(|v| [v[0], v[1], v[2], v[3]]).collect()
}

fn l2_distance(left: &[Vector4<f64>], right: &[Vector4<f64>]) -> f64 {
    left.iter()
        .zip(right)
        .map(|(a, b)| (a - b).norm_squared())
        .sum::<f64>()
        .sqrt()
}

fn seed_for(seed_id: &str) -> u64 {
    seed_id.bytes().fold(MASTER_SEED, |state, byte| {
        state.rotate_left(7) ^ u64::from(byte)
    })
}

fn active_row_radii(smoke: bool) -> &'static [f64] {
    if smoke {
        &ROW_RELATIVE_RADII[1..2]
    } else {
        ROW_RELATIVE_RADII
    }
}

fn active_angular_radii(smoke: bool) -> &'static [f64] {
    if smoke {
        &ANGULAR_RADII[1..2]
    } else {
        ANGULAR_RADII
    }
}

fn probe_identity(row: &ProbeRow) -> (String, String, u64, usize, i8) {
    (
        row.seed_id.clone(),
        row.perturbation.clone(),
        row.radius.to_bits(),
        row.direction_index,
        row.sign,
    )
}

fn write_jsonl_atomic<T: Serialize>(path: PathBuf, rows: &[T]) {
    let temporary = path.with_extension("jsonl.new");
    let file = File::create(&temporary)
        .unwrap_or_else(|err| panic!("create {}: {err}", temporary.display()));
    let mut writer = BufWriter::new(file);
    for row in rows {
        serde_json::to_writer(&mut writer, row).expect("serialize JSONL row");
        writer.write_all(b"\n").expect("write JSONL newline");
    }
    writer.flush().expect("flush JSONL");
    fs::rename(&temporary, &path).unwrap_or_else(|err| panic!("replace {}: {err}", path.display()));
}

fn write_json_atomic<T: Serialize>(path: PathBuf, value: &T) {
    let temporary = path.with_extension("json.new");
    let file = File::create(&temporary)
        .unwrap_or_else(|err| panic!("create {}: {err}", temporary.display()));
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, value).expect("serialize JSON");
    fs::rename(&temporary, &path).unwrap_or_else(|err| panic!("replace {}: {err}", path.display()));
}

fn canonical_output_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("sys-landscape manifest must be below experiments")
        .join("local-maxima-check")
        .join("artifacts")
}

fn smoke_output_dir() -> PathBuf {
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock before UNIX epoch")
        .as_millis();
    env::temp_dir().join(format!(
        "sys1-local-maxima-smoke-{}-{stamp}",
        std::process::id()
    ))
}

fn parse_args(command_args: Vec<String>) -> Cli {
    let mut canonical = false;
    let mut smoke = false;
    let mut out_dir = None;
    let mut threads = std::thread::available_parallelism()
        .map(usize::from)
        .unwrap_or(1)
        .min(8);
    let mut args = command_args.iter().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--canonical" => canonical = true,
            "--smoke" => smoke = true,
            "--out-dir" => {
                out_dir = Some(PathBuf::from(
                    args.next()
                        .unwrap_or_else(|| usage("--out-dir needs a path")),
                ));
            }
            "--threads" => {
                threads = args
                    .next()
                    .unwrap_or_else(|| usage("--threads needs a value"))
                    .parse()
                    .unwrap_or_else(|_| usage("--threads must be a positive integer"));
                if threads == 0 {
                    usage("--threads must be positive");
                }
            }
            "-h" | "--help" => usage(""),
            other => usage(&format!("unknown argument: {other}")),
        }
    }
    if canonical && smoke {
        usage("--canonical and --smoke are mutually exclusive");
    }
    if !canonical && !smoke {
        smoke = true;
    }
    let out_dir = out_dir.unwrap_or_else(|| {
        if canonical {
            canonical_output_dir()
        } else {
            smoke_output_dir()
        }
    });
    Cli {
        canonical,
        smoke,
        out_dir,
        threads,
        command_args,
    }
}

fn usage(message: &str) -> ! {
    if !message.is_empty() {
        eprintln!("error: {message}\n");
    }
    eprintln!("Usage: sys1-local-maxima [--smoke | --canonical] [--out-dir PATH] [--threads N]");
    std::process::exit(if message.is_empty() { 0 } else { 2 });
}

fn git_output(args: &[&str]) -> Option<String> {
    let output = Command::new("git").args(args).output().ok()?;
    output.status.success().then(|| {
        String::from_utf8_lossy(&output.stdout)
            .trim_end()
            .to_owned()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn panel_has_control_and_three_targets() {
        let seeds = known_equality_seeds();
        assert_eq!(seeds.len(), 4);
        assert_eq!(
            seeds
                .iter()
                .filter(|seed| seed.role == "expected_positive_control")
                .count(),
            1
        );
        assert_eq!(seeds.iter().filter(|seed| seed.role == "target").count(), 3);
        assert_eq!(
            seeds.iter().map(|seed| seed.id).collect::<Vec<_>>(),
            vec![
                "pentagon_threshold_control",
                "triangle_hexagon_theta0",
                "square_square_pi_over_4",
                "ch2021_six_vertex"
            ]
        );
    }
}

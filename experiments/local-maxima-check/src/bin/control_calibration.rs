//! Small matched-control calibration for fixed-facet local-maxima falsifiers.
//!
//! This producer deliberately uses only finite recomputed `sys` evaluations.
//! A positive probe is a finite improvement; a miss is not a local-maximality
//! certificate. The rotated-pentagon theorem, not this producer, supplies the
//! exact non-local-maximality conclusion for its structured family.

#[path = "../directions.rs"]
mod directions;
#[path = "../seeds.rs"]
mod seeds;

use directions::{l2_norm, perturb_linearly, quotient_basis};
use exp_sys_landscape::{compute_sys_computation, SysComputation, SysLandscapePolytopeCache};
use nalgebra::{DVector, Vector4};
use seeds::{product_dual_vertices, ProductSpec};
use serde::Serialize;
use std::collections::BTreeMap;
use std::env;
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{Instant, SystemTime, UNIX_EPOCH};
use symplectic::geom::known_polytopes;

const SCHEMA_VERSION: u32 = 1;
const MATERIAL_DELTA_SYS: f64 = 1.0e-12;
const QUOTIENT_RADIUS: f64 = 1.0e-4;
const HKO_PERTURBATION_RADIUS: f64 = 1.0e-3;
const HKO_RETURN_FRACTIONS: &[f64] = &[1.0e-1, 1.0e-2, 1.0e-3];
const CROSSING_RADII: &[f64] = &[1.0e-2, 1.0e-3, 1.0e-4];
const IMPROVING_SIDE_OFFSET: f64 = 1.0e-2;
const IMPROVING_SIDE_STEPS: &[f64] = &[1.0e-3, 1.0e-4, 1.0e-5];
const RANDOM_MASTER_SEED: u64 = 20_260_729;
const RANDOM_FACETS: usize = 6;
const RANDOM_H_MIN: f64 = 0.8;
const RANDOM_H_MAX: f64 = 1.2;

#[derive(Debug)]
struct Cli {
    canonical: bool,
    smoke: bool,
    out_dir: PathBuf,
    command_args: Vec<String>,
}

#[derive(Clone)]
struct EvaluatedState {
    id: String,
    role: &'static str,
    source: String,
    duals: Vec<Vector4<f64>>,
    polytope: SysLandscapePolytopeCache,
    computation: SysComputation,
    sys_lower: f64,
    sys_upper: f64,
    incidence_signature: Vec<String>,
}

#[derive(Debug)]
struct Probe {
    case_id: String,
    case_role: &'static str,
    source: String,
    family: &'static str,
    direction_index: usize,
    sign: i8,
    nominal_radius: f64,
    parameters: BTreeMap<String, f64>,
    analytic_expectation: &'static str,
    direction: DVector<f64>,
    perturbed_duals: Vec<Vector4<f64>>,
}

#[derive(Debug, Serialize)]
struct ProbeRow {
    schema_version: u32,
    case_id: String,
    case_role: String,
    source: String,
    family: String,
    direction_index: usize,
    sign: i8,
    nominal_radius: f64,
    parameters: BTreeMap<String, f64>,
    analytic_expectation: String,
    base_facet_count: usize,
    base_vertex_count: usize,
    base_sys: f64,
    base_sys_lower: f64,
    base_sys_upper: f64,
    base_best_sigma: Vec<usize>,
    base_returned_orbit_count: usize,
    direction_flat: Vec<f64>,
    step_norm: f64,
    relative_step_norm: f64,
    state_valid: bool,
    failure: Option<String>,
    perturbed_facet_count: Option<usize>,
    perturbed_vertex_count: Option<usize>,
    all_facets_defining: bool,
    same_incidence_signature: bool,
    perturbed_sys: Option<f64>,
    perturbed_sys_lower: Option<f64>,
    perturbed_sys_upper: Option<f64>,
    delta_sys: Option<f64>,
    delta_sys_per_step: Option<f64>,
    nominal_improvement: bool,
    interval_separated_improvement: bool,
    perturbed_best_sigma: Option<Vec<usize>>,
    perturbed_returned_orbit_count: Option<usize>,
    base_dual_vertices: Vec<[f64; 4]>,
    perturbed_dual_vertices: Vec<[f64; 4]>,
    wall_seconds: f64,
    active_beta_domain_status: &'static str,
}

#[derive(Debug, Serialize)]
struct CaseSummary {
    case_id: String,
    case_role: String,
    source: String,
    probe_count: usize,
    valid_probe_count: usize,
    incidence_change_count: usize,
    nominal_improvement_count: usize,
    interval_separated_improvement_count: usize,
    max_delta_sys: Option<f64>,
    finite_classification: String,
    claim_boundary: String,
}

#[derive(Debug, Serialize)]
struct Summary {
    schema_version: u32,
    experiment_id: &'static str,
    smoke: bool,
    all_probes_valid: bool,
    case_count: usize,
    probe_count: usize,
    cases: Vec<CaseSummary>,
    exact_control_source: &'static str,
    overall_claim_boundary: &'static str,
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
    material_delta_sys: f64,
    quotient_radius: f64,
    hko_perturbation_radius: f64,
    hko_return_fractions: Vec<f64>,
    crossing_radii: Vec<f64>,
    improving_side_offset: f64,
    improving_side_steps: Vec<f64>,
    random_master_seed: u64,
    random_attempt: u64,
    random_facet_count: usize,
    probe_count: usize,
    total_wall_seconds: f64,
    source_paths: Vec<&'static str>,
}

fn main() {
    let cli = parse_args(env::args().collect());
    let started = Instant::now();
    fs::create_dir_all(&cli.out_dir)
        .unwrap_or_else(|err| panic!("create {}: {err}", cli.out_dir.display()));

    let hko = evaluated_state(
        "hko_reference",
        "proved_fixed_f_positive_control",
        "symplectic::known_polytopes::hko_pentagon; theorem authority: experiments/hko-local-maximum/theorem/README.md",
        known_polytopes::hko_pentagon().dual_vertices_f64.clone(),
    );
    let hko_quotient = quotient_basis(&hko.duals);
    let crossing_theta = pentagon_crossing_theta();
    let crossing = evaluated_state(
        "pentagon_equality_crossing",
        "exact_nonmaximum_control",
        "thesis/09-rotated-regular-polygons-pentagon-profile-theorem.tex",
        pentagon_product(crossing_theta),
    );
    let improving_side = evaluated_state(
        "pentagon_improving_side",
        "exact_increasing_family_control",
        "thesis/09-rotated-regular-polygons-pentagon-profile-theorem.tex",
        pentagon_product(crossing_theta + IMPROVING_SIDE_OFFSET),
    );
    let (random_attempt, random_duals) = first_random_state();
    let random = evaluated_state(
        "ordinary_random_f6",
        "ordinary_random_negative_control",
        &format!(
            "SysLandscapePolytopeCache::generate_random(F=6, h=[0.8,1.2], master_seed={RANDOM_MASTER_SEED}, attempt={random_attempt})"
        ),
        random_duals,
    );

    let mut probes = Vec::new();
    add_signed_quotient_probes(
        &mut probes,
        &hko,
        &hko_quotient.slice_basis,
        if cli.smoke { 1 } else { hko_quotient.slice_basis.len() },
        QUOTIENT_RADIUS,
        "signed_quotient_basis",
        "no nominal improvement expected from the HKO positive control; this f64 expectation is not theorem evidence",
    );
    add_hko_return_probes(
        &mut probes,
        &hko,
        &hko_quotient.slice_basis,
        if cli.smoke { 1 } else { 2 },
        cli.smoke,
    );
    add_pentagon_crossing_probes(&mut probes, &crossing, cli.smoke);
    add_pentagon_improving_side_probes(&mut probes, &improving_side, cli.smoke);
    let random_quotient = quotient_basis(&random.duals);
    add_signed_quotient_probes(
        &mut probes,
        &random,
        &random_quotient.slice_basis,
        if cli.smoke {
            1
        } else {
            random_quotient.slice_basis.len()
        },
        QUOTIENT_RADIUS,
        "signed_quotient_basis",
        "at least one improvement is expected for an ordinary nonstationary random state, but no theorem assumption is made",
    );

    let mut rows = probes
        .iter()
        .map(|probe| evaluate_probe(probe, [&hko, &crossing, &improving_side, &random]))
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| {
        (
            &left.case_id,
            &left.family,
            left.nominal_radius.to_bits(),
            left.direction_index,
            left.sign,
        )
            .cmp(&(
                &right.case_id,
                &right.family,
                right.nominal_radius.to_bits(),
                right.direction_index,
                right.sign,
            ))
    });
    let cases = summarize(&rows);
    let summary = Summary {
        schema_version: SCHEMA_VERSION,
        experiment_id: "local-maxima-control-calibration-v1",
        smoke: cli.smoke,
        all_probes_valid: rows.iter().all(|row| row.state_valid),
        case_count: cases.len(),
        probe_count: rows.len(),
        cases,
        exact_control_source:
            "thesis/09-rotated-regular-polygons-pentagon-profile-theorem.tex",
        overall_claim_boundary: "Finite recomputed probes can find improving points or record a declared miss. Only the cited exact pentagon profile proves a non-local-maximum germ; finite misses and f64 HKO behavior are not local-maximality certificates.",
    };
    let provenance = Provenance {
        schema_version: SCHEMA_VERSION,
        experiment_id: "local-maxima-control-calibration-v1",
        command_args: cli.command_args,
        canonical: cli.canonical,
        smoke: cli.smoke,
        git_revision: git_output(&["rev-parse", "HEAD"]).unwrap_or_else(|| "unknown".into()),
        git_status_porcelain: git_output(&["status", "--porcelain", "--untracked-files=all"])
            .unwrap_or_default()
            .lines()
            .map(str::to_owned)
            .collect(),
        material_delta_sys: MATERIAL_DELTA_SYS,
        quotient_radius: QUOTIENT_RADIUS,
        hko_perturbation_radius: HKO_PERTURBATION_RADIUS,
        hko_return_fractions: HKO_RETURN_FRACTIONS.to_vec(),
        crossing_radii: CROSSING_RADII.to_vec(),
        improving_side_offset: IMPROVING_SIDE_OFFSET,
        improving_side_steps: IMPROVING_SIDE_STEPS.to_vec(),
        random_master_seed: RANDOM_MASTER_SEED,
        random_attempt,
        random_facet_count: RANDOM_FACETS,
        probe_count: rows.len(),
        total_wall_seconds: started.elapsed().as_secs_f64(),
        source_paths: vec![
            "experiments/local-maxima-check/control-calibration/README.md",
            "experiments/local-maxima-check/src/bin/control_calibration.rs",
            "experiments/local-maxima-check/src/directions.rs",
            "experiments/hko-local-maximum/theorem/README.md",
            "thesis/09-rotated-regular-polygons-pentagon-profile-theorem.tex",
            "experiments/dev-gradient-ascent/quotient-endpoint-diagnostic/artifacts/DISCUSSION.md",
        ],
    };
    write_jsonl_atomic(cli.out_dir.join("rows.jsonl"), &rows);
    write_json_atomic(cli.out_dir.join("summary.json"), &summary);
    write_json_atomic(cli.out_dir.join("run-provenance.json"), &provenance);
    eprintln!(
        "Wrote {} probes across {} cases to {} in {:.2}s",
        rows.len(),
        summary.case_count,
        cli.out_dir.display(),
        started.elapsed().as_secs_f64()
    );
}

fn add_signed_quotient_probes(
    probes: &mut Vec<Probe>,
    base: &EvaluatedState,
    basis: &[DVector<f64>],
    limit: usize,
    radius: f64,
    family: &'static str,
    expectation: &'static str,
) {
    for (direction_index, direction) in basis.iter().take(limit).enumerate() {
        for sign in [-1_i8, 1_i8] {
            let signed = direction * f64::from(sign);
            probes.push(Probe {
                case_id: base.id.clone(),
                case_role: base.role,
                source: base.source.clone(),
                family,
                direction_index,
                sign,
                nominal_radius: radius,
                parameters: BTreeMap::new(),
                analytic_expectation: expectation,
                direction: signed.clone(),
                perturbed_duals: perturb_linearly(
                    &base.duals,
                    &signed,
                    radius * l2_norm(&base.duals),
                ),
            });
        }
    }
}

fn add_hko_return_probes(
    probes: &mut Vec<Probe>,
    hko: &EvaluatedState,
    basis: &[DVector<f64>],
    direction_count: usize,
    smoke: bool,
) {
    let return_fractions = if smoke {
        &HKO_RETURN_FRACTIONS[..1]
    } else {
        HKO_RETURN_FRACTIONS
    };
    for (direction_index, direction) in basis.iter().take(direction_count).enumerate() {
        let base_duals = perturb_linearly(
            &hko.duals,
            direction,
            HKO_PERTURBATION_RADIUS * l2_norm(&hko.duals),
        );
        let base = evaluated_state(
            &format!("hko_perturb_q{direction_index}_r1e-3"),
            "controlled_hko_perturbation",
            "constructed from the HKO reference along a deterministic quotient-basis direction",
            base_duals,
        );
        let displacement = flatten_difference(&hko.duals, &base.duals);
        let displacement_norm = displacement.norm();
        let return_direction = &displacement / displacement_norm;
        for &fraction in return_fractions {
            let absolute_step = fraction * displacement_norm;
            probes.push(Probe {
                case_id: base.id.clone(),
                case_role: base.role,
                source: base.source.clone(),
                family: "return_toward_hko",
                direction_index,
                sign: 1,
                nominal_radius: fraction,
                parameters: BTreeMap::from([
                    ("fraction_of_hko_distance".to_owned(), fraction),
                    ("hko_distance".to_owned(), displacement_norm),
                ]),
                analytic_expectation: "nominal improvement is expected near the proved HKO maximum, but a finite return sequence does not prove that the perturbed base is not a local maximum",
                direction: return_direction.clone(),
                perturbed_duals: perturb_linearly(&base.duals, &return_direction, absolute_step),
            });
        }
    }
}

fn add_pentagon_crossing_probes(probes: &mut Vec<Probe>, crossing: &EvaluatedState, smoke: bool) {
    let radii = if smoke {
        &CROSSING_RADII[1..2]
    } else {
        CROSSING_RADII
    };
    let theta = pentagon_crossing_theta();
    for &radius in radii {
        for sign in [-1_i8, 1_i8] {
            let perturbed = pentagon_product(theta + f64::from(sign) * radius);
            probes.push(Probe {
                case_id: crossing.id.clone(),
                case_role: crossing.role,
                source: crossing.source.clone(),
                family: "exact_profile_relative_rotation",
                direction_index: 0,
                sign,
                nominal_radius: radius,
                parameters: BTreeMap::from([
                    ("base_theta_rad".to_owned(), theta),
                    (
                        "perturbed_theta_rad".to_owned(),
                        theta + f64::from(sign) * radius,
                    ),
                ]),
                analytic_expectation: if sign > 0 {
                    "the exact pentagon profile proves improvement toward larger theta"
                } else {
                    "the exact pentagon profile proves decrease toward smaller theta"
                },
                direction: flatten_difference(&perturbed, &crossing.duals),
                perturbed_duals: perturbed,
            });
        }
    }
}

fn add_pentagon_improving_side_probes(probes: &mut Vec<Probe>, base: &EvaluatedState, smoke: bool) {
    let steps = if smoke {
        &IMPROVING_SIDE_STEPS[1..2]
    } else {
        IMPROVING_SIDE_STEPS
    };
    let theta = pentagon_crossing_theta() + IMPROVING_SIDE_OFFSET;
    for &step in steps {
        let perturbed = pentagon_product(theta + step);
        probes.push(Probe {
            case_id: base.id.clone(),
            case_role: base.role,
            source: base.source.clone(),
            family: "exact_profile_continue_improving_side",
            direction_index: 0,
            sign: 1,
            nominal_radius: step,
            parameters: BTreeMap::from([
                ("base_theta_rad".to_owned(), theta),
                ("perturbed_theta_rad".to_owned(), theta + step),
            ]),
            analytic_expectation:
                "the exact pentagon profile proves improvement at arbitrarily smaller positive theta steps while theta remains below pi/10",
            direction: flatten_difference(&perturbed, &base.duals),
            perturbed_duals: perturbed,
        });
    }
}

fn evaluate_probe(probe: &Probe, fixed_bases: [&EvaluatedState; 4]) -> ProbeRow {
    let started = Instant::now();
    let owned_base;
    let base = if let Some(base) = fixed_bases.iter().find(|base| base.id == probe.case_id) {
        *base
    } else {
        // Controlled HKO perturbations are cheap enough to reconstruct here;
        // their exact dual coordinates remain in every raw row.
        owned_base = evaluated_state(
            &probe.case_id,
            probe.case_role,
            &probe.source,
            infer_hko_perturbation_base(probe),
        );
        &owned_base
    };
    let step_norm = l2_distance(&probe.perturbed_duals, &base.duals);
    let common = |state_valid: bool,
                  failure: Option<String>,
                  polytope: Option<&SysLandscapePolytopeCache>,
                  computation: Option<&SysComputation>| {
        let perturbed_sys = computation.map(|value| value.sys);
        let (perturbed_sys_lower, perturbed_sys_upper) = computation
            .map(sys_interval)
            .map_or((None, None), |(lower, upper)| (Some(lower), Some(upper)));
        let delta_sys = perturbed_sys.map(|value| value - base.computation.sys);
        let signature = polytope.map(|value| incidence_signature(&value.vertex_facet_incidence));
        ProbeRow {
            schema_version: SCHEMA_VERSION,
            case_id: probe.case_id.clone(),
            case_role: probe.case_role.to_owned(),
            source: probe.source.clone(),
            family: probe.family.to_owned(),
            direction_index: probe.direction_index,
            sign: probe.sign,
            nominal_radius: probe.nominal_radius,
            parameters: probe.parameters.clone(),
            analytic_expectation: probe.analytic_expectation.to_owned(),
            base_facet_count: base.polytope.facet_count(),
            base_vertex_count: base.polytope.vertices.len(),
            base_sys: base.computation.sys,
            base_sys_lower: base.sys_lower,
            base_sys_upper: base.sys_upper,
            base_best_sigma: base.computation.capacity.best_sigma().to_vec(),
            base_returned_orbit_count: base.computation.capacity.orbits.len(),
            direction_flat: probe.direction.iter().copied().collect(),
            step_norm,
            relative_step_norm: step_norm / l2_norm(&base.duals),
            state_valid,
            failure,
            perturbed_facet_count: polytope.map(SysLandscapePolytopeCache::facet_count),
            perturbed_vertex_count: polytope.map(|value| value.vertices.len()),
            all_facets_defining: polytope
                .is_some_and(|value| value.facet_count() == base.duals.len()),
            same_incidence_signature: signature
                .is_some_and(|value| value == base.incidence_signature),
            perturbed_sys,
            perturbed_sys_lower,
            perturbed_sys_upper,
            delta_sys,
            delta_sys_per_step: delta_sys.map(|value| value / step_norm),
            nominal_improvement: delta_sys.is_some_and(|value| value > MATERIAL_DELTA_SYS),
            interval_separated_improvement: perturbed_sys_lower
                .is_some_and(|value| value > base.sys_upper),
            perturbed_best_sigma: computation
                .map(|value| value.capacity.best_sigma().to_vec()),
            perturbed_returned_orbit_count: computation
                .map(|value| value.capacity.orbits.len()),
            base_dual_vertices: vectors_to_arrays(&base.duals),
            perturbed_dual_vertices: vectors_to_arrays(&probe.perturbed_duals),
            wall_seconds: started.elapsed().as_secs_f64(),
            active_beta_domain_status:
                "not exposed by this full-scalar producer; no numerical active-set completeness claim",
        }
    };
    match compute_state(&probe.perturbed_duals) {
        Ok((polytope, computation)) => common(true, None, Some(&polytope), Some(&computation)),
        Err(failure) => common(false, Some(failure), None, None),
    }
}

fn infer_hko_perturbation_base(probe: &Probe) -> Vec<Vector4<f64>> {
    let hko = known_polytopes::hko_pentagon().dual_vertices_f64.clone();
    let quotient = quotient_basis(&hko);
    perturb_linearly(
        &hko,
        &quotient.slice_basis[probe.direction_index],
        HKO_PERTURBATION_RADIUS * l2_norm(&hko),
    )
}

fn summarize(rows: &[ProbeRow]) -> Vec<CaseSummary> {
    let mut groups: BTreeMap<(String, String, String), Vec<&ProbeRow>> = BTreeMap::new();
    for row in rows {
        groups
            .entry((
                row.case_id.clone(),
                row.case_role.clone(),
                row.source.clone(),
            ))
            .or_default()
            .push(row);
    }
    groups
        .into_iter()
        .map(|((case_id, case_role, source), selected)| {
            let valid = selected.iter().filter(|row| row.state_valid).count();
            let incidence_changes = selected
                .iter()
                .filter(|row| row.state_valid && !row.same_incidence_signature)
                .count();
            let nominal = selected
                .iter()
                .filter(|row| row.nominal_improvement)
                .count();
            let separated = selected
                .iter()
                .filter(|row| row.interval_separated_improvement)
                .count();
            let max_delta_sys = selected
                .iter()
                .filter_map(|row| row.delta_sys)
                .max_by(f64::total_cmp);
            let (finite_classification, claim_boundary) = match case_role.as_str() {
                "exact_nonmaximum_control" => (
                    "exactly_not_local_maximum_in_named_rotation_family",
                    "The exact theorem supplies the germ; the rows only check that the numerical detector recovers it.",
                ),
                "exact_increasing_family_control" => (
                    "exactly_not_local_maximum_in_named_rotation_family",
                    "The exact profile is strictly increasing toward pi/10; the finite rows are calibration.",
                ),
                "proved_fixed_f_positive_control" if nominal == 0 => (
                    "survives_declared_finite_signed_basis_suite",
                    "The separate exact HKO packet proves fixed-F local maximality; this finite miss is not proof evidence.",
                ),
                "controlled_hko_perturbation" if nominal > 0 => (
                    "finite_return_improvement_observed",
                    "The finite return steps are empirical germ evidence, not a proof of non-local-maximality.",
                ),
                "ordinary_random_negative_control" if nominal > 0 => (
                    "finite_quotient_improvement_observed",
                    "The found finite point rejects this state under the declared poll but does not prove an arbitrarily small improving germ.",
                ),
                _ if nominal > 0 => (
                    "finite_improvement_observed",
                    "A finite improvement was found; no theorem-strength germ is claimed.",
                ),
                _ => (
                    "survives_declared_finite_suite",
                    "No improvement was found in this finite suite; no local-maximality claim follows.",
                ),
            };
            CaseSummary {
                case_id,
                case_role,
                source,
                probe_count: selected.len(),
                valid_probe_count: valid,
                incidence_change_count: incidence_changes,
                nominal_improvement_count: nominal,
                interval_separated_improvement_count: separated,
                max_delta_sys,
                finite_classification: finite_classification.to_owned(),
                claim_boundary: claim_boundary.to_owned(),
            }
        })
        .collect()
}

fn evaluated_state(
    id: &str,
    role: &'static str,
    source: &str,
    duals: Vec<Vector4<f64>>,
) -> EvaluatedState {
    let (polytope, computation) =
        compute_state(&duals).unwrap_or_else(|failure| panic!("base {id} failed: {failure}"));
    assert_eq!(
        polytope.facet_count(),
        duals.len(),
        "base {id} has a redundant facet"
    );
    let (sys_lower, sys_upper) = sys_interval(&computation);
    EvaluatedState {
        id: id.to_owned(),
        role,
        source: source.to_owned(),
        incidence_signature: incidence_signature(&polytope.vertex_facet_incidence),
        duals,
        polytope,
        computation,
        sys_lower,
        sys_upper,
    }
}

fn first_random_state() -> (u64, Vec<Vector4<f64>>) {
    for attempt in 0..10_000 {
        if let Some(polytope) = SysLandscapePolytopeCache::generate_random(
            RANDOM_FACETS,
            RANDOM_H_MIN,
            RANDOM_H_MAX,
            RANDOM_MASTER_SEED,
            attempt,
        ) {
            return (attempt, polytope.dual_vertices_f64);
        }
    }
    panic!("no valid deterministic random F=6 state in first 10,000 attempts");
}

fn pentagon_crossing_theta() -> f64 {
    let c0 = (5.0 + 2.0 * 5.0_f64.sqrt()) / 10.0;
    c0.sqrt().acos()
}

fn pentagon_product(theta_rad: f64) -> Vec<Vector4<f64>> {
    product_dual_vertices(&ProductSpec {
        q_sides: 5,
        p_sides: 5,
        theta_rad,
    })
}

fn compute_state(
    duals: &[Vector4<f64>],
) -> Result<(SysLandscapePolytopeCache, SysComputation), String> {
    let polytope = SysLandscapePolytopeCache::from_f64_dual_vertices(duals.to_vec())
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

fn flatten_difference(left: &[Vector4<f64>], right: &[Vector4<f64>]) -> DVector<f64> {
    DVector::from_iterator(
        left.len() * 4,
        left.iter()
            .zip(right)
            .flat_map(|(a, b)| (a - b).iter().copied().collect::<Vec<_>>()),
    )
}

fn l2_distance(left: &[Vector4<f64>], right: &[Vector4<f64>]) -> f64 {
    left.iter()
        .zip(right)
        .map(|(a, b)| (a - b).norm_squared())
        .sum::<f64>()
        .sqrt()
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
    vectors
        .iter()
        .map(|value| [value[0], value[1], value[2], value[3]])
        .collect()
}

fn write_jsonl_atomic<T: Serialize>(path: PathBuf, rows: &[T]) {
    let temporary = path.with_extension("jsonl.new");
    let file = File::create(&temporary)
        .unwrap_or_else(|err| panic!("create {}: {err}", temporary.display()));
    let mut writer = BufWriter::new(file);
    for row in rows {
        serde_json::to_writer(&mut writer, row).expect("serialize JSONL");
        writer.write_all(b"\n").expect("write newline");
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
        .join("control-calibration")
        .join("artifacts")
}

fn smoke_output_dir() -> PathBuf {
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock before UNIX epoch")
        .as_millis();
    env::temp_dir().join(format!(
        "local-maxima-control-calibration-smoke-{}-{stamp}",
        std::process::id()
    ))
}

fn parse_args(command_args: Vec<String>) -> Cli {
    let mut canonical = false;
    let mut smoke = false;
    let mut out_dir = None;
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
        command_args,
    }
}

fn usage(message: &str) -> ! {
    if !message.is_empty() {
        eprintln!("{message}");
    }
    eprintln!("Usage: local-maxima-control-calibration [--smoke | --canonical] [--out-dir PATH]");
    std::process::exit(if message.is_empty() { 0 } else { 2 });
}

fn git_output(args: &[&str]) -> Option<String> {
    let output = Command::new("git").args(args).output().ok()?;
    if !output.status.success() {
        return None;
    }
    Some(String::from_utf8_lossy(&output.stdout).trim().to_owned())
}

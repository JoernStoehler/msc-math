//! Conditional inactive-facet resampling smoke for retained random 5x5 products.
//!
//! The active dual vertices of one exact winning word are copied bit-for-bit.
//! Only inactive factor-normal angles and support heights are resampled.  This
//! executable is a bounded feasibility probe, not a class-effect estimator.

use exp_sys_landscape::{exact_volume_from_incidence_as_f64, SysLandscapePolytopeCache};
use nalgebra::Vector4;
use num_rational::BigRational;
use num_traits::Zero;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::f64::consts::TAU;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};
use std::time::Instant;
use symplectic::algorithms::billiard::bounce_count_from_sigma_for_facets;
use symplectic::algorithms::facet_adjacency::build_transition_matrix_from_facet_intersections_and_omega;
use symplectic::algorithms::hk2017::orbit_recovery::recover_and_verify;
use symplectic::{
    aggregate_certified_orbits_with_dual_vertices_exact, classify_facets_from_dual_vertices,
    solve_billiard_candidates, solve_orbit_sigma_saddle_point, CertifiedOrbitSetMode,
};

const RAW_SHA256: &str = "66bf82010e92e0f26b0df226f4e6c0eef05d21eb22a0967c7f669530f6545736";
const CLASS_SHA256: &str = "187089804bd17fdac76bdaf51a8d8202e67fb2b14779fe9a418cc8da47c7b4c4";
const MASTER_SEED: u64 = 2_026_071_401;
const DEFAULT_ACCEPTED_PER_BASE: usize = 16;
const DEFAULT_MAX_ATTEMPTS: usize = 160;
const TARGET_EVALUATION_CAP: usize = 128;
const H_MIN: f64 = 0.8;
const H_MAX: f64 = 1.2;
const GEOMETRIC_TOL: f64 = 1e-8;

#[derive(Clone, Deserialize)]
struct RawRow {
    name: String,
    k: usize,
    m: usize,
    bounces: usize,
    dual_vertices: Vec<[f64; 4]>,
    volume: f64,
    sys: f64,
}

#[derive(Clone, Deserialize)]
struct ClassMinimum {
    action: f64,
    action_exact: String,
    minimizer_sigmas: Vec<Vec<usize>>,
}

#[derive(Clone, Deserialize)]
struct ClassRow {
    name: String,
    producer_bounces: usize,
    class_minima: BTreeMap<String, Option<ClassMinimum>>,
}

#[derive(Deserialize)]
struct BaseSelection {
    schema: String,
    bases: Vec<BaseSpec>,
}

#[derive(Clone, Deserialize)]
struct BaseSpec {
    name: String,
    pair_id: String,
    producer_bounces: usize,
    sigma: Vec<usize>,
    active_support: Vec<usize>,
    winner_action_exact: String,
    winner_action: f64,
}

struct Args {
    raw: PathBuf,
    classes: PathBuf,
    bases: PathBuf,
    out: PathBuf,
    accepted_per_base: usize,
    max_attempts_per_base_law: usize,
}

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
enum Law {
    FixedRanks,
    UnlabeledSupport,
}

impl Law {
    fn label(self) -> &'static str {
        match self {
            Self::FixedRanks => "fixed_ranks",
            Self::UnlabeledSupport => "unlabeled_support",
        }
    }
}

#[derive(Clone, Debug)]
struct FactorFacet {
    old_rank: Option<usize>,
    angle: f64,
    height: f64,
    dual: [f64; 2],
}

#[derive(Clone, Debug)]
struct FactorSample {
    facets: Vec<FactorFacet>,
    old_to_new: BTreeMap<usize, usize>,
}

#[derive(Serialize)]
struct ProposalRow {
    schema: &'static str,
    base_name: String,
    pair_id: String,
    producer_bounces: usize,
    law: Law,
    master_seed: u64,
    proposal_index: usize,
    target_evaluation_index: Option<usize>,
    status: &'static str,
    rejection_reason: Option<String>,
    q_inactive_heights: Vec<f64>,
    p_inactive_heights: Vec<f64>,
    q_active_new_ranks: Vec<usize>,
    p_active_new_ranks: Vec<usize>,
    metrics: Option<AcceptedMetrics>,
}

#[derive(Serialize)]
struct AcceptedMetrics {
    fixed_sigma: Vec<usize>,
    fixed_active_support: Vec<usize>,
    fixed_action: f64,
    fixed_action_exact: String,
    fixed_action_exact_agrees_with_base: bool,
    fixed_action_relative_error_from_base_f64: f64,
    fixed_kkt_beta_margin: f64,
    fixed_constraint_rank_exact: usize,
    fixed_constraint_kernel_dimension: usize,
    fixed_recovery_solution_dimension: usize,
    fixed_recovery_max_halfspace_violation: f64,
    fixed_inactive_clearance_min: f64,
    fixed_geometrically_feasible_at_1e_8: bool,
    fixed_candidate_stream_present: bool,
    fixed_global_minimal_exact: bool,
    global_action: f64,
    global_action_exact: String,
    global_minimizer_sigmas: Vec<Vec<usize>>,
    global_bounce_labels: Vec<usize>,
    takeover_sigmas: Vec<Vec<usize>>,
    volume: f64,
    global_sys: f64,
    fixed_branch_sys: f64,
    log_volume_ratio_from_base: f64,
    log_global_sys_ratio_from_base: f64,
    log_fixed_branch_sys_ratio_from_base: f64,
    candidate_iterations: u64,
    candidate_orbit_count: usize,
    evaluation_wall_seconds: f64,
}

fn parse_args_from(argv: impl IntoIterator<Item = impl Into<String>>) -> Args {
    let argv: Vec<String> = argv.into_iter().map(Into::into).collect();
    let mut raw = None;
    let mut classes = None;
    let mut bases = None;
    let mut out = None;
    let mut accepted_per_base = DEFAULT_ACCEPTED_PER_BASE;
    let mut max_attempts_per_base_law = DEFAULT_MAX_ATTEMPTS;
    let mut i = 1;
    while i < argv.len() {
        let value = || {
            argv.get(i + 1)
                .unwrap_or_else(|| panic!("{} requires a value", argv[i]))
        };
        match argv[i].as_str() {
            "--raw" => raw = Some(PathBuf::from(value())),
            "--classes" => classes = Some(PathBuf::from(value())),
            "--bases" => bases = Some(PathBuf::from(value())),
            "--out" => out = Some(PathBuf::from(value())),
            "--accepted-per-base" => {
                accepted_per_base = value().parse().expect("--accepted-per-base must be usize")
            }
            "--max-attempts-per-base-law" => {
                max_attempts_per_base_law = value()
                    .parse()
                    .expect("--max-attempts-per-base-law must be usize")
            }
            other => panic!("unknown argument: {other}"),
        }
        i += 2;
    }
    assert!(accepted_per_base > 0);
    assert!(max_attempts_per_base_law >= accepted_per_base);
    Args {
        raw: raw.expect("--raw is required"),
        classes: classes.expect("--classes is required"),
        bases: bases.expect("--bases is required"),
        out: out.expect("--out is required"),
        accepted_per_base,
        max_attempts_per_base_law,
    }
}

fn sha256(path: &Path) -> String {
    let mut hasher = Sha256::new();
    let mut reader = BufReader::new(File::open(path).expect("open hash input"));
    std::io::copy(&mut reader, &mut hasher).expect("hash input");
    format!("{:x}", hasher.finalize())
}

fn read_jsonl<T: for<'de> Deserialize<'de>>(path: &Path) -> Vec<T> {
    BufReader::new(File::open(path).expect("open JSONL"))
        .lines()
        .filter_map(|line| {
            let line = line.expect("read JSONL");
            (!line.trim().is_empty()).then(|| serde_json::from_str(&line).expect("parse JSONL"))
        })
        .collect()
}

fn factor_facets(raw: &RawRow, q_factor: bool) -> Vec<FactorFacet> {
    let (start, coords) = if q_factor {
        (0, (0, 1))
    } else {
        (raw.k, (2, 3))
    };
    let count = if q_factor { raw.k } else { raw.m };
    raw.dual_vertices[start..start + count]
        .iter()
        .enumerate()
        .map(|(rank, a)| {
            let norm = a[coords.0].hypot(a[coords.1]);
            FactorFacet {
                old_rank: Some(rank),
                angle: a[coords.1].atan2(a[coords.0]).rem_euclid(TAU),
                height: 1.0 / norm,
                dual: [a[coords.0], a[coords.1]],
            }
        })
        .collect()
}

fn inactive_facet(angle: f64, rng: &mut ChaCha8Rng) -> FactorFacet {
    let height = rng.gen_range(H_MIN..H_MAX);
    FactorFacet {
        old_rank: None,
        angle,
        height,
        dual: [angle.cos() / height, angle.sin() / height],
    }
}

fn ordered_uniforms(count: usize, lo: f64, hi: f64, rng: &mut ChaCha8Rng) -> Vec<f64> {
    let mut out: Vec<f64> = (0..count).map(|_| rng.gen_range(lo..hi)).collect();
    out.sort_by(f64::total_cmp);
    out
}

fn resample_factor(
    source: &[FactorFacet],
    active_old_ranks: &[usize],
    law: Law,
    rng: &mut ChaCha8Rng,
) -> FactorSample {
    assert_eq!(source.len(), 5);
    assert_eq!(active_old_ranks.len(), 3);
    let active_set: BTreeSet<usize> = active_old_ranks.iter().copied().collect();
    let mut facets = Vec::with_capacity(source.len());
    match law {
        Law::FixedRanks => {
            let mut previous_rank = 0usize;
            let mut previous_angle = 0.0;
            for &rank in active_old_ranks {
                assert!(rank >= previous_rank);
                for angle in ordered_uniforms(
                    rank - previous_rank,
                    previous_angle,
                    source[rank].angle,
                    rng,
                ) {
                    facets.push(inactive_facet(angle, rng));
                }
                facets.push(source[rank].clone());
                previous_rank = rank + 1;
                previous_angle = source[rank].angle;
            }
            for angle in ordered_uniforms(source.len() - previous_rank, previous_angle, TAU, rng) {
                facets.push(inactive_facet(angle, rng));
            }
        }
        Law::UnlabeledSupport => {
            facets.extend(active_old_ranks.iter().map(|&rank| source[rank].clone()));
            for _ in 0..source.len() - active_old_ranks.len() {
                facets.push(inactive_facet(rng.gen_range(0.0..TAU), rng));
            }
            facets.sort_by(|a, b| a.angle.total_cmp(&b.angle));
        }
    }
    assert_eq!(facets.len(), source.len());
    let old_to_new: BTreeMap<usize, usize> = facets
        .iter()
        .enumerate()
        .filter_map(|(new, facet)| facet.old_rank.map(|old| (old, new)))
        .collect();
    assert_eq!(old_to_new.len(), active_set.len());
    if matches!(law, Law::FixedRanks) {
        for &rank in active_old_ranks {
            assert_eq!(
                old_to_new[&rank], rank,
                "fixed-rank law changed an active rank"
            );
        }
    }
    FactorSample { facets, old_to_new }
}

fn remap_sigma(
    sigma: &[usize],
    q_map: &BTreeMap<usize, usize>,
    p_map: &BTreeMap<usize, usize>,
) -> Vec<usize> {
    sigma
        .iter()
        .map(|&index| {
            if index < 5 {
                q_map[&index]
            } else {
                5 + p_map[&(index - 5)]
            }
        })
        .collect()
}

fn assembled_duals(q: &FactorSample, p: &FactorSample) -> Vec<Vector4<f64>> {
    q.facets
        .iter()
        .map(|f| Vector4::new(f.dual[0], f.dual[1], 0.0, 0.0))
        .chain(
            p.facets
                .iter()
                .map(|f| Vector4::new(0.0, 0.0, f.dual[0], f.dual[1])),
        )
        .collect()
}

fn canonical_cycle(sigma: &[usize]) -> Vec<usize> {
    (0..sigma.len())
        .map(|i| {
            sigma[i..]
                .iter()
                .chain(&sigma[..i])
                .copied()
                .collect::<Vec<_>>()
        })
        .min()
        .expect("sigma nonempty")
}

fn cyclic_equal(a: &[usize], b: &[usize]) -> bool {
    a.len() == b.len() && canonical_cycle(a) == canonical_cycle(b)
}

fn exact_constraint_rank(duals: &[[BigRational; 4]], sigma: &[usize]) -> usize {
    let rows = 5;
    let cols = sigma.len();
    let mut matrix = vec![vec![BigRational::zero(); cols]; rows];
    for (col, &facet) in sigma.iter().enumerate() {
        for row in 0..4 {
            matrix[row][col] = duals[facet][row].clone();
        }
        matrix[4][col] = BigRational::from_integer(1.into());
    }
    let mut rank = 0usize;
    for col in 0..cols {
        let Some(pivot) = (rank..rows).find(|&row| !matrix[row][col].is_zero()) else {
            continue;
        };
        matrix.swap(rank, pivot);
        let pivot_value = matrix[rank][col].clone();
        for value in matrix[rank].iter_mut().skip(col) {
            *value /= pivot_value.clone();
        }
        let normalized_pivot_row = matrix[rank].clone();
        for (row_index, row) in matrix.iter_mut().enumerate().take(rows) {
            if row_index == rank || row[col].is_zero() {
                continue;
            }
            let factor = row[col].clone();
            for (value, pivot_value) in row.iter_mut().zip(&normalized_pivot_row).skip(col) {
                *value -= factor.clone() * pivot_value.clone();
            }
        }
        rank += 1;
        if rank == rows {
            break;
        }
    }
    rank
}

fn evaluate(
    poly: &SysLandscapePolytopeCache,
    fixed_sigma: &[usize],
    base: &BaseSpec,
    raw: &RawRow,
) -> Result<AcceptedMetrics, String> {
    let started = Instant::now();
    let classification = classify_facets_from_dual_vertices(&poly.dual_vertices_f64)
        .map_err(|error| format!("classification:{error:?}"))?;
    let transitions = build_transition_matrix_from_facet_intersections_and_omega(
        &poly.facet_intersection_is_nonempty,
        &poly.omega_signs,
    );
    let (candidates, iterations) = solve_billiard_candidates(
        &poly.dual_vertices_f64,
        &classification.q_indices,
        &classification.p_indices,
        &poly.facet_intersection_is_nonempty,
        &transitions,
    )
    .map_err(|error| format!("candidate_stream:{error:?}"))?;
    let fixed_candidate_stream_present = candidates
        .iter()
        .any(|orbit| cyclic_equal(&orbit.sigma, fixed_sigma));
    let candidate_orbit_count = candidates.len();
    let global = aggregate_certified_orbits_with_dual_vertices_exact(
        &poly.dual_vertices,
        candidates,
        iterations,
        BigRational::zero(),
        CertifiedOrbitSetMode::MinimizersOnly,
    )
    .map_err(|error| format!("global_exact_aggregation:{error:?}"))?;

    let fixed_f64 = solve_orbit_sigma_saddle_point(&poly.dual_vertices_f64, fixed_sigma)
        .map_err(|error| format!("fixed_sigma_solve:{error:?}"))?;
    let fixed = aggregate_certified_orbits_with_dual_vertices_exact(
        &poly.dual_vertices,
        vec![fixed_f64.clone()],
        1,
        BigRational::zero(),
        CertifiedOrbitSetMode::MinimizersOnly,
    )
    .map_err(|error| format!("fixed_exact_aggregation:{error:?}"))?;
    let fixed_exact = fixed.capacity_exact.to_string();
    let fixed_action = fixed.capacity;

    let geometric = recover_and_verify(&poly.dual_vertices_f64, &fixed_f64)
        .ok_or_else(|| "fixed_geometric_recovery_unavailable".to_string())?;
    let active: BTreeSet<usize> = fixed_sigma.iter().copied().collect();
    let fixed_inactive_clearance_min = geometric
        .breakpoints
        .iter()
        .flat_map(|point| {
            poly.dual_vertices_f64
                .iter()
                .enumerate()
                .filter(|(facet, _)| !active.contains(facet))
                .map(move |(_, dual)| 1.0 - dual.dot(point))
        })
        .fold(f64::INFINITY, f64::min);

    let mut global_bounce_labels = BTreeSet::new();
    let mut global_minimizer_sigmas = Vec::new();
    for orbit in &global.minimizers {
        global_minimizer_sigmas.push(canonical_cycle(&orbit.sigma));
        let label = bounce_count_from_sigma_for_facets(
            &classification.q_indices,
            &classification.p_indices,
            &orbit.sigma,
        )
        .ok_or_else(|| "global_bounce_label_unavailable".to_string())?;
        global_bounce_labels.insert(label);
    }
    global_minimizer_sigmas.sort();
    global_minimizer_sigmas.dedup();
    let fixed_global_minimal_exact = global.capacity_exact == fixed.capacity_exact;
    let takeover_sigmas = if fixed_global_minimal_exact {
        Vec::new()
    } else {
        global_minimizer_sigmas.clone()
    };
    let volume = exact_volume_from_incidence_as_f64(&poly.vertices, &poly.vertex_facet_incidence);
    if !volume.is_finite() || volume <= 0.0 {
        return Err("nonpositive_or_nonfinite_volume".to_string());
    }
    let global_sys = global.capacity * global.capacity / (2.0 * volume);
    let fixed_branch_sys = fixed_action * fixed_action / (2.0 * volume);
    let rank = exact_constraint_rank(&poly.dual_vertices, fixed_sigma);

    Ok(AcceptedMetrics {
        fixed_sigma: canonical_cycle(fixed_sigma),
        fixed_active_support: active.into_iter().collect(),
        fixed_action,
        fixed_action_exact: fixed_exact.clone(),
        fixed_action_exact_agrees_with_base: fixed_exact == base.winner_action_exact,
        fixed_action_relative_error_from_base_f64: (fixed_action - base.winner_action).abs()
            / base.winner_action,
        fixed_kkt_beta_margin: fixed_f64.beta_margin,
        fixed_constraint_rank_exact: rank,
        fixed_constraint_kernel_dimension: fixed_sigma.len() - rank,
        fixed_recovery_solution_dimension: geometric.solution_dim,
        fixed_recovery_max_halfspace_violation: geometric.max_violation,
        fixed_inactive_clearance_min,
        fixed_geometrically_feasible_at_1e_8: geometric.max_violation <= GEOMETRIC_TOL,
        fixed_candidate_stream_present,
        fixed_global_minimal_exact,
        global_action: global.capacity,
        global_action_exact: global.capacity_exact.to_string(),
        global_minimizer_sigmas,
        global_bounce_labels: global_bounce_labels.into_iter().collect(),
        takeover_sigmas,
        volume,
        global_sys,
        fixed_branch_sys,
        log_volume_ratio_from_base: (volume / raw.volume).ln(),
        log_global_sys_ratio_from_base: (global_sys / raw.sys).ln(),
        log_fixed_branch_sys_ratio_from_base: (fixed_branch_sys / raw.sys).ln(),
        candidate_iterations: iterations,
        candidate_orbit_count,
        evaluation_wall_seconds: started.elapsed().as_secs_f64(),
    })
}

fn stream_rng(base_name: &str, law: Law) -> ChaCha8Rng {
    let mut hasher = Sha256::new();
    hasher.update(MASTER_SEED.to_le_bytes());
    hasher.update(base_name.as_bytes());
    hasher.update(law.label().as_bytes());
    ChaCha8Rng::from_seed(hasher.finalize().into())
}

fn main() {
    let args = parse_args_from(std::env::args());
    assert_eq!(sha256(&args.raw), RAW_SHA256, "raw input identity mismatch");
    assert_eq!(
        sha256(&args.classes),
        CLASS_SHA256,
        "class input identity mismatch"
    );
    let raw_rows: Vec<RawRow> = read_jsonl(&args.raw);
    let class_rows: Vec<ClassRow> = read_jsonl(&args.classes);
    let raw_by_name: HashMap<_, _> = raw_rows.into_iter().map(|r| (r.name.clone(), r)).collect();
    let class_by_name: HashMap<_, _> = class_rows
        .into_iter()
        .map(|r| (r.name.clone(), r))
        .collect();
    let selection: BaseSelection =
        serde_json::from_reader(File::open(&args.bases).expect("open bases")).expect("parse bases");
    assert_eq!(
        selection.schema,
        "product-bounce-active-resampling/base-selection/v1"
    );
    assert_eq!(
        selection.bases.len(),
        4,
        "selection must contain four bases"
    );
    assert!(args.accepted_per_base * selection.bases.len() * 2 <= TARGET_EVALUATION_CAP);
    if let Some(parent) = args.out.parent() {
        std::fs::create_dir_all(parent).expect("create output parent");
    }
    let mut writer = BufWriter::new(File::create(&args.out).expect("create output"));
    let mut target_evaluations = 0usize;
    let run_started = Instant::now();
    let mut stop = false;

    for base in &selection.bases {
        if stop {
            break;
        }
        let raw = raw_by_name
            .get(&base.name)
            .expect("selected raw base exists");
        let class = class_by_name
            .get(&base.name)
            .expect("selected class base exists");
        assert_eq!((raw.k, raw.m), (5, 5));
        assert_eq!(raw.bounces, base.producer_bounces);
        assert_eq!(class.producer_bounces, base.producer_bounces);
        let winner = class.class_minima[&base.producer_bounces.to_string()]
            .as_ref()
            .expect("selected winner exists");
        assert_eq!(winner.action_exact, base.winner_action_exact);
        assert!((winner.action - base.winner_action).abs() <= 1e-12 * winner.action.abs().max(1.0));
        assert!(winner
            .minimizer_sigmas
            .iter()
            .any(|s| cyclic_equal(s, &base.sigma)));
        let selected_support: Vec<usize> = {
            let mut s = base.sigma.clone();
            s.sort_unstable();
            s.dedup();
            s
        };
        assert_eq!(selected_support, base.active_support);
        let q_active: Vec<usize> = base
            .active_support
            .iter()
            .copied()
            .filter(|&i| i < 5)
            .collect();
        let p_active: Vec<usize> = base
            .active_support
            .iter()
            .copied()
            .filter(|&i| i >= 5)
            .map(|i| i - 5)
            .collect();
        assert_eq!((q_active.len(), p_active.len()), (3, 3));
        let q_source = factor_facets(raw, true);
        let p_source = factor_facets(raw, false);

        for law in [Law::FixedRanks, Law::UnlabeledSupport] {
            let mut rng = stream_rng(&base.name, law);
            let mut accepted = 0usize;
            for proposal_index in 0..args.max_attempts_per_base_law {
                if accepted >= args.accepted_per_base {
                    break;
                }
                if target_evaluations >= TARGET_EVALUATION_CAP {
                    stop = true;
                    break;
                }
                let q = resample_factor(&q_source, &q_active, law, &mut rng);
                let p = resample_factor(&p_source, &p_active, law, &mut rng);
                let fixed_sigma = remap_sigma(&base.sigma, &q.old_to_new, &p.old_to_new);
                let duals = assembled_duals(&q, &p);
                let q_inactive_heights = q
                    .facets
                    .iter()
                    .filter(|f| f.old_rank.is_none())
                    .map(|f| f.height)
                    .collect();
                let p_inactive_heights = p
                    .facets
                    .iter()
                    .filter(|f| f.old_rank.is_none())
                    .map(|f| f.height)
                    .collect();
                let q_active_new_ranks = q.old_to_new.values().copied().collect();
                let p_active_new_ranks = p.old_to_new.values().copied().collect();

                let Some(poly) = SysLandscapePolytopeCache::from_f64_dual_vertices(duals) else {
                    let row = ProposalRow {
                        schema: "product-bounce-active-resampling/proposal/v1",
                        base_name: base.name.clone(),
                        pair_id: base.pair_id.clone(),
                        producer_bounces: base.producer_bounces,
                        law,
                        master_seed: MASTER_SEED,
                        proposal_index,
                        target_evaluation_index: None,
                        status: "rejected",
                        rejection_reason: Some("accepted_generator_validity_failed".into()),
                        q_inactive_heights,
                        p_inactive_heights,
                        q_active_new_ranks,
                        p_active_new_ranks,
                        metrics: None,
                    };
                    writeln!(writer, "{}", serde_json::to_string(&row).unwrap()).unwrap();
                    continue;
                };

                target_evaluations += 1;
                let evaluation = evaluate(&poly, &fixed_sigma, base, raw);
                let (status, rejection_reason, metrics) = match evaluation {
                    Ok(metrics) => {
                        assert!(metrics.fixed_action_exact_agrees_with_base);
                        accepted += 1;
                        ("accepted", None, Some(metrics))
                    }
                    Err(reason) => ("rejected", Some(reason), None),
                };
                let row = ProposalRow {
                    schema: "product-bounce-active-resampling/proposal/v1",
                    base_name: base.name.clone(),
                    pair_id: base.pair_id.clone(),
                    producer_bounces: base.producer_bounces,
                    law,
                    master_seed: MASTER_SEED,
                    proposal_index,
                    target_evaluation_index: Some(target_evaluations - 1),
                    status,
                    rejection_reason,
                    q_inactive_heights,
                    p_inactive_heights,
                    q_active_new_ranks,
                    p_active_new_ranks,
                    metrics,
                };
                writeln!(writer, "{}", serde_json::to_string(&row).unwrap()).unwrap();
            }
            if accepted < args.accepted_per_base {
                eprintln!(
                    "STOP base={} law={}: accepted {accepted}/{} within {} attempts; target evaluations={target_evaluations}",
                    base.name,
                    law.label(),
                    args.accepted_per_base,
                    args.max_attempts_per_base_law
                );
                stop = true;
                break;
            }
        }
    }
    writer.flush().expect("flush output");
    eprintln!(
        "wrote {} after {:.3}s with {} target evaluations",
        args.out.display(),
        run_started.elapsed().as_secs_f64(),
        target_evaluations
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    fn source() -> Vec<FactorFacet> {
        [0.2, 1.2, 2.4, 4.0, 5.1]
            .into_iter()
            .enumerate()
            .map(|(rank, angle): (usize, f64)| FactorFacet {
                old_rank: Some(rank),
                angle,
                height: 1.0,
                dual: [angle.cos(), angle.sin()],
            })
            .collect()
    }

    #[test]
    fn fixed_rank_law_keeps_active_rows_and_slots() {
        let mut rng = ChaCha8Rng::seed_from_u64(7);
        let src = source();
        let sample = resample_factor(&src, &[0, 2, 4], Law::FixedRanks, &mut rng);
        assert_eq!(sample.old_to_new, BTreeMap::from([(0, 0), (2, 2), (4, 4)]));
        for rank in [0, 2, 4] {
            assert_eq!(sample.facets[rank].dual, src[rank].dual);
        }
    }

    #[test]
    fn inactive_heights_remain_in_open_generator_support() {
        let mut rng = ChaCha8Rng::seed_from_u64(8);
        for law in [Law::FixedRanks, Law::UnlabeledSupport] {
            let sample = resample_factor(&source(), &[0, 2, 4], law, &mut rng);
            let inactive: Vec<_> = sample
                .facets
                .iter()
                .filter(|f| f.old_rank.is_none())
                .collect();
            assert_eq!(inactive.len(), 2);
            assert!(inactive
                .iter()
                .all(|f| H_MIN <= f.height && f.height < H_MAX));
        }
    }

    #[test]
    fn remapping_tracks_unlabeled_active_geometry_after_sorting() {
        let mut rng = ChaCha8Rng::seed_from_u64(19);
        let q = resample_factor(&source(), &[0, 2, 4], Law::UnlabeledSupport, &mut rng);
        let p = resample_factor(&source(), &[0, 2, 4], Law::UnlabeledSupport, &mut rng);
        let old = vec![0, 5, 2, 7, 4, 9];
        let new = remap_sigma(&old, &q.old_to_new, &p.old_to_new);
        for (&old_index, &new_index) in old.iter().zip(&new) {
            if old_index < 5 {
                assert_eq!(q.facets[new_index].old_rank, Some(old_index));
            } else {
                assert_eq!(p.facets[new_index - 5].old_rank, Some(old_index - 5));
            }
        }
    }

    #[test]
    fn accepted_generator_rejects_semicircle_factor() {
        let q: Vec<_> = [0.0_f64, 0.1, 0.2, 0.3, 0.4]
            .into_iter()
            .map(|a| Vector4::new(a.cos(), a.sin(), 0.0, 0.0))
            .collect();
        let p: Vec<_> = [0.0_f64, 1.3, 2.6, 3.9, 5.2]
            .into_iter()
            .map(|a| Vector4::new(0.0, 0.0, a.cos(), a.sin()))
            .collect();
        assert!(SysLandscapePolytopeCache::from_f64_dual_vertices(
            q.into_iter().chain(p).collect()
        )
        .is_none());
    }

    #[test]
    fn copied_active_duals_preserve_fixed_sigma_action_exactly() {
        let q_angles = [0.1_f64, 1.4, 2.7, 3.8, 5.0];
        let p_angles = [0.4_f64, 1.7, 2.9, 4.2, 5.5];
        let duals: Vec<_> = q_angles
            .into_iter()
            .map(|a| Vector4::new(a.cos(), a.sin(), 0.0, 0.0))
            .chain(
                p_angles
                    .into_iter()
                    .map(|a| Vector4::new(0.0, 0.0, a.cos(), a.sin())),
            )
            .collect();
        let base = SysLandscapePolytopeCache::from_f64_dual_vertices(duals).unwrap();
        let classification = classify_facets_from_dual_vertices(&base.dual_vertices_f64).unwrap();
        let transitions = build_transition_matrix_from_facet_intersections_and_omega(
            &base.facet_intersection_is_nonempty,
            &base.omega_signs,
        );
        let (orbits, _) = solve_billiard_candidates(
            &base.dual_vertices_f64,
            &classification.q_indices,
            &classification.p_indices,
            &base.facet_intersection_is_nonempty,
            &transitions,
        )
        .unwrap();
        let sigma = orbits[0].sigma.clone();
        let active: BTreeSet<_> = sigma.iter().copied().collect();
        let mut changed = base.dual_vertices_f64.clone();
        for (i, dual) in changed.iter_mut().enumerate() {
            if !active.contains(&i) {
                *dual *= 0.93;
            }
        }
        let changed = SysLandscapePolytopeCache::from_f64_dual_vertices(changed).unwrap();
        let solve_exact = |poly: &SysLandscapePolytopeCache| {
            let orbit = solve_orbit_sigma_saddle_point(&poly.dual_vertices_f64, &sigma).unwrap();
            aggregate_certified_orbits_with_dual_vertices_exact(
                &poly.dual_vertices,
                vec![orbit],
                1,
                BigRational::zero(),
                CertifiedOrbitSetMode::MinimizersOnly,
            )
            .unwrap()
            .capacity_exact
        };
        assert_eq!(solve_exact(&base), solve_exact(&changed));
    }
}

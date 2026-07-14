//! Focused evaluation of exact-rechecking the candidates retained by the f64
//! capacity routes.  This is an experiment consumer; it deliberately does not
//! change the production route or claim candidate recall.

use exp_dev_quadratic_program::{
    exact_binary64_dual_vertex_arrays, generated_f64_cases_with_source_filter,
    solve_exact_capacity_for_transition_pruned_sigmas,
};
use nalgebra::{DMatrix, Vector4};
use num_rational::BigRational;
use num_traits::{One, Signed};
use serde::Serialize;
use std::{
    collections::{BTreeMap, BTreeSet},
    env,
    fs::{create_dir_all, File},
    io::{BufWriter, Write},
    path::PathBuf,
    time::Instant,
};
use symplectic::algorithms::billiard::{for_each_sigma_from_facets, solve_billiard_candidates};
use symplectic::algorithms::facet_adjacency::build_transition_matrix_from_facet_intersections_and_omega;
use symplectic::algorithms::hk2017::solve_pruned_hk2017_candidates;
use symplectic::exact::{
    exact_vertices_with_incidence, facet_intersection_is_nonempty_exact, omega_signs_exact,
};
use symplectic::{
    aggregate_certified_orbits_with_dual_vertices_exact, aggregate_orbits_with_dual_vertices_exact,
    classify_facets_from_dual_vertices, known_polytopes, CertifiedOrbitSetMode, OrbitAdmissibility,
    OrbitGuaranteeMode, OrbitKktData,
};

const RUN_ID: &str = "retained-exact-v1";
const SCHEMA: &str = "qp-retained-exact-v1";
const FIVE_PERCENT_NUM: i64 = 21;
const FIVE_PERCENT_DEN: i64 = 20;

#[derive(Clone)]
struct Fixture {
    case_id: &'static str,
    route: &'static str,
    completeness: &'static str,
    target_kind: &'static str,
    source_id: String,
    dual_f64: Vec<Vector4<f64>>,
    dual_exact: Vec<[BigRational; 4]>,
    transition: DMatrix<bool>,
    facet_intersection: DMatrix<bool>,
    product: Option<(Vec<usize>, Vec<usize>)>,
}

#[derive(Serialize)]
struct CandidateRow {
    sigma: Vec<usize>,
    f64_status: String,
    current_minimasafe_status: String,
    retained_exact_status: String,
    exact_resolution_reason: String,
    f64_action: f64,
    f64_action_lower: f64,
    f64_action_upper: f64,
    exact_q: Option<String>,
    exact_action: Option<String>,
}

#[derive(Serialize)]
struct CaseRow {
    run_id: String,
    schema_version: String,
    case_id: String,
    route: String,
    completeness_contract: String,
    target_input_kind: String,
    intended_algebraic_target: String,
    source_id: String,
    target_dual_vertices_exact: Vec<Vec<String>>,
    sigma_stream_count: usize,
    f64_true_count: usize,
    f64_indeterminate_count: usize,
    f64_rejected_count: usize,
    candidate_generation_ms: f64,
    current_minimasafe_ms: f64,
    retained_exact_ms: f64,
    exact_all_reference_ms: Option<f64>,
    current_min_action_f64: Option<f64>,
    current_scalar_rule: String,
    current_minimizer_rule: String,
    current_window_rule: String,
    current_minimizer_sigmas: Vec<Vec<usize>>,
    current_window_cutoff_f64: Option<f64>,
    current_window_sigmas: Vec<Vec<usize>>,
    retained_exact_resolution_count: usize,
    retained_exact_accept_count: usize,
    retained_exact_reject_count: usize,
    retained_exact_min_action: Option<String>,
    retained_exact_minimizer_sigmas: Vec<Vec<usize>>,
    retained_exact_window_cutoff: Option<String>,
    retained_exact_window_sigmas: Vec<Vec<usize>>,
    exact_all_stream_count: Option<usize>,
    exact_all_accept_count: Option<usize>,
    exact_all_min_action: Option<String>,
    exact_all_minimizer_sigmas: Vec<Vec<usize>>,
    exact_all_window_cutoff: Option<String>,
    exact_all_window_sigmas: Vec<Vec<usize>>,
    scalar_agreement_current_vs_retained: bool,
    minimizer_agreement_current_vs_retained: bool,
    window_agreement_current_vs_retained: bool,
    agreement_rules: String,
    scalar_agreement_retained_vs_all: Option<bool>,
    minimizer_agreement_retained_vs_all: Option<bool>,
    window_agreement_retained_vs_all: Option<bool>,
    candidates: Vec<CandidateRow>,
}

fn rat(x: &BigRational) -> String {
    format!("{}/{}", x.numer(), x.denom())
}
fn exact_action(q: &BigRational) -> BigRational {
    BigRational::one() / (q.clone() + q.clone())
}
fn exact_vecs(xs: &[[BigRational; 4]]) -> Vec<Vector4<BigRational>> {
    xs.iter()
        .map(|x| Vector4::new(x[0].clone(), x[1].clone(), x[2].clone(), x[3].clone()))
        .collect()
}
fn sigma_key(s: &[usize]) -> Vec<usize> {
    s.to_vec()
}
fn exact_target(xs: &[[BigRational; 4]]) -> Vec<Vec<String>> {
    xs.iter().map(|x| x.iter().map(rat).collect()).collect()
}

fn fixture_generated(
    case_id: &'static str,
    route: &'static str,
    source: &str,
    product: bool,
    completeness: &'static str,
) -> Fixture {
    let c = generated_f64_cases_with_source_filter(1, 99_540_836, &[source.to_string()])
        .pop()
        .expect("generated fixture");
    let dual_exact = exact_binary64_dual_vertex_arrays(&c.dual_vertices);
    let ex = exact_vecs(&dual_exact);
    let ev = exact_vertices_with_incidence(&ex).expect("exact fixture geometry");
    let fi = facet_intersection_is_nonempty_exact(&ev.vertex_facet_incidence);
    let signs = omega_signs_exact(&ex);
    let transition = build_transition_matrix_from_facet_intersections_and_omega(&fi, &signs);
    let product_indices = product.then(|| {
        let cl =
            classify_facets_from_dual_vertices(&c.dual_vertices).expect("product classification");
        (cl.q_indices, cl.p_indices)
    });
    Fixture {
        case_id,
        route,
        completeness,
        target_kind: "stored_binary64_rational",
        source_id: c.source_id,
        dual_f64: c.dual_vertices,
        dual_exact,
        transition,
        facet_intersection: fi,
        product: product_indices,
    }
}

fn fixture_triangle_square() -> Fixture {
    let kp = known_polytopes::lagrangian_triangle_square();
    let dual_exact = exact_binary64_dual_vertex_arrays(&kp.dual_vertices_f64);
    let signs = omega_signs_exact(&exact_vecs(&dual_exact));
    let transition = build_transition_matrix_from_facet_intersections_and_omega(
        &kp.facet_intersection_is_nonempty,
        &signs,
    );
    let cl =
        classify_facets_from_dual_vertices(&kp.dual_vertices_f64).expect("product classification");
    Fixture {
        case_id: "triangle_square_tie",
        route: "product_block",
        completeness: "transition_complete_product_block",
        target_kind: "stored_binary64_rational (algebraic-original unavailable)",
        source_id: kp.name.to_string(),
        dual_f64: kp.dual_vertices_f64.clone(),
        dual_exact,
        transition,
        facet_intersection: kp.facet_intersection_is_nonempty.clone(),
        product: Some((cl.q_indices, cl.p_indices)),
    }
}

fn pruning_fixture() -> Fixture {
    let dual_f64 = vec![
        Vector4::new(
            -0.7609176562997226,
            -0.5842245470076217,
            -0.6093220693528425,
            0.07216780853507296,
        ),
        Vector4::new(
            0.784069284213464,
            -0.5531443877418841,
            0.18211913477611671,
            -0.36079445513926356,
        ),
        Vector4::new(
            -0.043547885416314415,
            0.8556529705333096,
            0.8361784175796745,
            0.2857765173406991,
        ),
        Vector4::new(
            -0.2753007640820361,
            -0.48381690655215637,
            -0.8235951274500787,
            0.35426171198575546,
        ),
        Vector4::new(
            -0.12602783596581424,
            0.6516682410783413,
            0.1098373351502524,
            -0.5152232850628169,
        ),
    ];
    let dual_exact = exact_binary64_dual_vertex_arrays(&dual_f64);
    let ex = exact_vecs(&dual_exact);
    let ev = exact_vertices_with_incidence(&ex).expect("exact pruning geometry");
    let fi = facet_intersection_is_nonempty_exact(&ev.vertex_facet_incidence);
    let transition =
        build_transition_matrix_from_facet_intersections_and_omega(&fi, &omega_signs_exact(&ex));
    Fixture {
        case_id: "pruning_roundoff",
        route: "hk_transition_pruned",
        completeness: "transition_complete",
        target_kind: "stored_binary64_rational",
        source_id: "embedded:literal_f64_pruning_fixture".into(),
        dual_f64,
        dual_exact,
        transition,
        facet_intersection: fi,
        product: None,
    }
}

fn solve_candidates(f: &Fixture) -> (Vec<OrbitKktData>, u64) {
    if let Some((q, p)) = &f.product {
        solve_billiard_candidates(&f.dual_f64, q, p, &f.facet_intersection, &f.transition)
            .expect("billiard candidate solve")
    } else {
        solve_pruned_hk2017_candidates(&f.dual_f64, &f.transition).expect("HK candidate solve")
    }
}

fn exact_all(
    f: &Fixture,
    _gap: BigRational,
) -> Option<(usize, usize, BigRational, Vec<Vec<usize>>, Vec<Vec<usize>>)> {
    if f.product.is_none() {
        let r = solve_exact_capacity_for_transition_pruned_sigmas(
            &f.dual_exact,
            &f.transition,
            BigRational::from_integer(10_000_000_000i64.into()),
        )
        .ok()?;
        let cutoff = r.capacity_exact.clone() * BigRational::from_integer(FIVE_PERCENT_NUM.into())
            / BigRational::from_integer(FIVE_PERCENT_DEN.into());
        let window = r
            .orbits
            .iter()
            .filter(|o| o.action_exact <= cutoff)
            .map(|o| o.sigma.clone())
            .collect();
        return Some((
            r.iterations as usize,
            r.exact_admissible_count,
            r.capacity_exact,
            r.minimizers.iter().map(|o| o.sigma.clone()).collect(),
            window,
        ));
    }
    let (q, p) = f.product.as_ref().unwrap();
    let mut stream = Vec::new();
    for_each_sigma_from_facets(q, p, &f.facet_intersection, &f.transition, |s| {
        stream.push(s.to_vec())
    });
    let mut rows = Vec::new();
    for sigma in &stream {
        if let Some(sol) = symplectic::kkt::rational_solver::solve_kkt_exact(&f.dual_exact, sigma) {
            if sol.beta.iter().all(|b| b.is_positive()) && sol.q_exact.is_positive() {
                rows.push((exact_action(&sol.q_exact), sigma.clone()));
            }
        }
    }
    rows.sort_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.cmp(&b.1)));
    let min = rows.first()?.0.clone();
    let cutoff = min.clone() * BigRational::from_integer(FIVE_PERCENT_NUM.into())
        / BigRational::from_integer(FIVE_PERCENT_DEN.into());
    let mins = rows
        .iter()
        .filter(|x| x.0 == min)
        .map(|x| x.1.clone())
        .collect();
    let accepted_count = rows.len();
    let window = rows
        .into_iter()
        .filter(|x| x.0 <= cutoff)
        .map(|x| x.1)
        .collect();
    Some((stream.len(), accepted_count, min, mins, window))
}

fn main() {
    let out = env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("artifacts/retained-exact"));
    create_dir_all(&out).expect("create output directory");
    let gap = BigRational::from_integer(FIVE_PERCENT_NUM.into())
        / BigRational::from_integer(FIVE_PERCENT_DEN.into())
        - BigRational::one();
    let huge_gap = BigRational::from_integer(10_000_000_000i64.into());
    let fixtures = vec![
        fixture_generated(
            "ordinary_generated_F5",
            "hk_transition_pruned",
            "seed99540836:F5:sample0:attempt5000000008",
            false,
            "transition_complete",
        ),
        fixture_generated(
            "pinned_q4_p5",
            "product_block",
            "seed99540836:q4:p5:attempt405000000000",
            true,
            "transition_complete_product_block",
        ),
        fixture_triangle_square(),
        pruning_fixture(),
    ];
    let path = out.join("raw_rows.jsonl");
    let mut writer = BufWriter::new(File::create(&path).expect("raw output"));
    for f in fixtures {
        let gen_start = Instant::now();
        let (candidates, iterations) = solve_candidates(&f);
        let generation_ms = gen_start.elapsed().as_secs_f64() * 1000.0;
        let f64_true = candidates
            .iter()
            .filter(|o| o.admissibility == OrbitAdmissibility::AdmissibleF64)
            .count();
        let f64_ind = candidates
            .iter()
            .filter(|o| o.admissibility == OrbitAdmissibility::IndeterminateF64)
            .count();
        let f64_rejected = iterations as usize - candidates.len();
        let cur_start = Instant::now();
        let current = aggregate_orbits_with_dual_vertices_exact(
            &f.dual_exact,
            candidates.clone(),
            iterations,
            0.0,
            OrbitGuaranteeMode::MinimaSafe,
        )
        .expect("MinimaSafe aggregation");
        let current_ms = cur_start.elapsed().as_secs_f64() * 1000.0;
        let current_min = current.min_action;
        let current_mins = current
            .orbits
            .iter()
            .filter(|o| (o.action - current_min).abs() <= 1e-12)
            .map(|o| o.sigma.clone())
            .collect::<Vec<_>>();
        let retained_start = Instant::now();
        let retained = aggregate_certified_orbits_with_dual_vertices_exact(
            &f.dual_exact,
            candidates.clone(),
            iterations,
            huge_gap.clone(),
            CertifiedOrbitSetMode::GapWindow,
        )
        .expect("retained exact aggregation");
        let retained_ms = retained_start.elapsed().as_secs_f64() * 1000.0;
        let retained_set = retained
            .orbits
            .iter()
            .map(|o| o.sigma.clone())
            .collect::<BTreeSet<_>>();
        let retained_min = retained.capacity_exact.clone();
        let retained_cutoff = retained_min.clone()
            * BigRational::from_integer(FIVE_PERCENT_NUM.into())
            / BigRational::from_integer(FIVE_PERCENT_DEN.into());
        let retained_window = retained
            .orbits
            .iter()
            .filter(|o| o.action_exact <= retained_cutoff)
            .map(|o| o.sigma.clone())
            .collect::<Vec<_>>();
        let current_cutoff = current_min * (FIVE_PERCENT_NUM as f64) / (FIVE_PERCENT_DEN as f64);
        let current_window = current
            .orbits
            .iter()
            .filter(|o| o.action <= current_cutoff)
            .map(|o| o.sigma.clone())
            .collect::<Vec<_>>();
        let current_status: BTreeMap<Vec<usize>, String> = current
            .orbits
            .iter()
            .map(|o| (sigma_key(&o.sigma), format!("{:?}", o.admissibility)))
            .collect();
        let exact_status: BTreeMap<Vec<usize>, (String, String, String)> = candidates
            .iter()
            .map(|o| {
                let key = sigma_key(&o.sigma);
                if let Some(e) = retained.orbits.iter().find(|e| e.sigma == o.sigma) {
                    (
                        key,
                        ("accepted".into(), rat(&e.q_exact), rat(&e.action_exact)),
                    )
                } else {
                    (key, ("rejected_exact".into(), "".into(), "".into()))
                }
            })
            .collect();
        let mut candidates_rows = Vec::new();
        for o in &candidates {
            let key = sigma_key(&o.sigma);
            let e = exact_status.get(&key).unwrap();
            candidates_rows.push(CandidateRow {
                sigma: key.clone(),
                f64_status: format!("{:?}", o.admissibility),
                current_minimasafe_status: current_status
                    .get(&key)
                    .cloned()
                    .unwrap_or_else(|| "rejected_by_minimasafe".into()),
                retained_exact_status: e.0.clone(),
                exact_resolution_reason: if e.0 == "accepted" {
                    "exact solver returned an admissible positive-Q witness".into()
                } else {
                    "unavailable: exact solver returned no admissible positive-Q witness; this conflates singular/inconsistent systems and nonpositive beta or Q".into()
                },
                f64_action: o.action,
                f64_action_lower: o.action_lower,
                f64_action_upper: o.action_upper,
                exact_q: (e.1.is_empty())
                    .then_some(String::new())
                    .or_else(|| Some(e.1.clone()))
                    .filter(|x| !x.is_empty()),
                exact_action: (e.2.is_empty())
                    .then_some(String::new())
                    .or_else(|| Some(e.2.clone()))
                    .filter(|x| !x.is_empty()),
            });
        }
        let ref_start = Instant::now();
        let reference = exact_all(&f, gap.clone());
        let ref_ms = ref_start.elapsed().as_secs_f64() * 1000.0;
        let (ref_stream, ref_accept, ref_min, ref_mins, ref_window) = reference
            .as_ref()
            .map(|r| {
                (
                    Some(r.0),
                    Some(r.1),
                    Some(r.2.clone()),
                    r.3.clone(),
                    r.4.clone(),
                )
            })
            .unwrap_or((None, None, None, Vec::new(), Vec::new()));
        let row = CaseRow {
            run_id: RUN_ID.into(),
            schema_version: SCHEMA.into(),
            case_id: f.case_id.into(),
            route: f.route.into(),
            completeness_contract: f.completeness.into(),
            target_input_kind: f.target_kind.into(),
            intended_algebraic_target:
                "unavailable: no algebraic oracle; all exact values are over stored rational target"
                    .into(),
            source_id: f.source_id,
            target_dual_vertices_exact: exact_target(&f.dual_exact),
            sigma_stream_count: iterations as usize,
            f64_true_count: f64_true,
            f64_indeterminate_count: f64_ind,
            f64_rejected_count: f64_rejected,
            candidate_generation_ms: generation_ms,
            current_minimasafe_ms: current_ms,
            retained_exact_ms: retained_ms,
            exact_all_reference_ms: reference.as_ref().map(|_| ref_ms),
            current_min_action_f64: Some(current_min),
            current_scalar_rule: "MinimaSafe returned f64 min_action (production scalar; no tolerance)".into(),
            current_minimizer_rule: "diagnostic grouping of MinimaSafe returned actions with abs(action - min_action) <= 1e-12; not an API minimizer set".into(),
            current_window_rule: "MinimaSafe returned actions with f64 action <= (21/20) * MinimaSafe f64 min_action; independent of retained/exact reference".into(),
            current_minimizer_sigmas: current_mins.clone(),
            current_window_cutoff_f64: Some(current_cutoff),
            current_window_sigmas: current_window.clone(),
            retained_exact_resolution_count: retained.exact_resolutions,
            retained_exact_accept_count: retained_set.len(),
            retained_exact_reject_count: candidates.len() - retained_set.len(),
            retained_exact_min_action: Some(rat(&retained_min)),
            retained_exact_minimizer_sigmas: retained
                .minimizers
                .iter()
                .map(|o| o.sigma.clone())
                .collect(),
            retained_exact_window_cutoff: Some(rat(&retained_cutoff)),
            retained_exact_window_sigmas: retained_window.clone(),
            exact_all_stream_count: ref_stream,
            exact_all_accept_count: ref_accept,
            exact_all_min_action: ref_min.as_ref().map(rat),
            exact_all_minimizer_sigmas: ref_mins.clone(),
            exact_all_window_cutoff: ref_min.as_ref().map(|m| {
                rat(
                    &(m.clone() * BigRational::from_integer(FIVE_PERCENT_NUM.into())
                        / BigRational::from_integer(FIVE_PERCENT_DEN.into())),
                )
            }),
            exact_all_window_sigmas: ref_window.clone(),
            scalar_agreement_current_vs_retained: (current_min - retained.capacity).abs() <= 1e-12,
            minimizer_agreement_current_vs_retained: current_mins
                == retained
                    .minimizers
                    .iter()
                    .map(|o| o.sigma.clone())
                    .collect::<Vec<_>>(),
            window_agreement_current_vs_retained: current_window == retained_window,
            agreement_rules: "current scalar vs retained uses abs(f64_min - f64_exact) <= 1e-12; current minimizer/window comparisons use ordered sigma-vector equality; retained vs exact-all scalar/minimizer/window comparisons use exact rational/vector equality".into(),
            scalar_agreement_retained_vs_all: ref_min.as_ref().map(|m| *m == retained_min),
            minimizer_agreement_retained_vs_all: reference.as_ref().map(|r| {
                r.3 == retained
                    .minimizers
                    .iter()
                    .map(|o| o.sigma.clone())
                    .collect::<Vec<_>>()
            }),
            window_agreement_retained_vs_all: reference.as_ref().map(|r| r.4 == retained_window),
            candidates: candidates_rows,
        };
        serde_json::to_writer(&mut writer, &row).expect("serialize row");
        writeln!(&mut writer).expect("newline");
    }
}

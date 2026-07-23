//! Wide-row QP numerical evidence producer.
//!
//! The producer intentionally has one observation boundary:
//! `observe(polytope, sigma) -> RawRow`.  It calls the production Rust KKT,
//! geometry, derivative, recovery, exact-KKT, capacity, and volume APIs
//! directly.  Nullable fields carry narrow reasons when a route or oracle does
//! not apply.  Python is only used for offline arithmetic and presentation.

use algebraic_numbers::{rank as exact_matrix_rank, solve_linear_system, LinearSystemSolution};
use euclidean_polytopes::{volume_from_incidence_exact, volume_from_incidence_f64};
use exp_dev_quadratic_program::{
    exact_binary64_dual_vertex_arrays, exact_binary64_transition_matrix_assuming_origin_interior,
    generated_f64_cases_with_source_filter, F64ValidationPolicy,
};
use exp_sys_landscape::SysLandscapePolytopeCache;
use nalgebra::{DMatrix, DVector, Vector4};
use num_rational::BigRational;
use num_traits::{One, ToPrimitive, Zero};
use serde::Serialize;
use std::fs::{create_dir_all, File};
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use std::time::Instant;
use symplectic::algorithms::hk2017::orbit_recovery::recover_and_verify_sigma_beta_action;
use symplectic::algorithms::hk2017::SimpleDirectedCyclesCanonical;
use symplectic::derivatives::capacity_derivatives_a_from_kkt_result;
use symplectic::geom::known_polytopes::{self, KnownPolytope};
use symplectic::geom::symplectic_form::omega0;
use symplectic::kkt::q_value;
use symplectic::kkt::qp_assembly::{
    build_augmented_system_from_dual_vertices, build_qp_from_dual_vertices,
};
use symplectic::kkt::saddle_point_solver::{solve_kkt_for_dual_vertices, KktOutcome};

const PRODUCER_VERSION: &str = "wide-row-rust-v1";
const SCHEMA_VERSION: &str = "qp-wide-row-v1";
const MAX_SIGMAS_PER_CASE: usize = 256;

#[derive(Clone)]
struct Case {
    case_id: String,
    cohort: String,
    family: String,
    source_id: String,
    dual_f64: Vec<Vector4<f64>>,
    dual_exact: Vec<[BigRational; 4]>,
    vertices_exact: Option<Vec<[BigRational; 4]>>,
    vertices_f64: Option<Vec<Vector4<f64>>>,
    original_exact: bool,
    incidence: Option<DMatrix<bool>>,
    transition: Option<DMatrix<bool>>,
    sigmas: Vec<Vec<usize>>,
    universe_contract: String,
}

#[derive(Serialize, Clone)]
struct RawRow {
    run_id: String,
    source_revision: String,
    producer_version: String,
    schema_version: String,
    case_id: String,
    cohort: String,
    source_family: String,
    source_id: String,
    target_id: String,
    target_coordinate_kind: String,
    target_dual_vertices_exact: Vec<Vec<String>>,
    original_rational_dual_vertices_exact: Option<Vec<Vec<String>>>,
    stored_dyadic_dual_vertices_exact: Option<Vec<Vec<String>>>,
    target_preprocessing_status: String,
    universe_contract: String,
    sigma: Vec<usize>,
    lifecycle_stage: String,
    lifecycle_reason: String,
    lifecycle_events: Vec<String>,
    geometry_status: String,
    predictor_status: String,
    f64_solver_status: String,
    exact_solver_status: String,
    f64_solver_elapsed_us: f64,
    exact_solver_elapsed_us: f64,
    exact_algebra_status: String,
    exact_algebra_reason: String,
    exact_consistency_status: String,
    exact_rank: Option<usize>,
    exact_nullity: Option<usize>,
    exact_beta_witness: Option<Vec<String>>,
    exact_beta_particular_predicate: String,
    exact_beta_selection_status: String,
    intended_algebraic_status: String,
    proposal_status: String,
    proposal_beta_f64: Option<Vec<f64>>,
    proposal_mu_f64: Option<Vec<f64>>,
    proposal_xi_f64: Option<f64>,
    proposal_q_f64: Option<f64>,
    proposal_q_raw_f64: Option<f64>,
    proposal_q_correction_f64: Option<f64>,
    proposal_q_corrected_f64: Option<f64>,
    proposal_q_error_bound_f64: Option<f64>,
    proposal_q_beta_radius_bound: Option<f64>,
    proposal_action_f64: Option<f64>,
    proposal_residual_norm: Option<f64>,
    proposal_rank: Option<usize>,
    proposal_nullity: Option<usize>,
    proposal_singular_values_f64: Vec<f64>,
    proposal_eigenvalues_f64: Vec<f64>,
    proposal_inertia_positive: Option<usize>,
    proposal_inertia_negative: Option<usize>,
    proposal_inertia_zero: Option<usize>,
    beta_f64: Option<Vec<f64>>,
    beta_exact: Option<Vec<String>>,
    mu_f64: Option<Vec<f64>>,
    xi_f64: Option<f64>,
    q_raw_f64: Option<f64>,
    q_corrected_f64: Option<f64>,
    q_correction_f64: Option<f64>,
    /// Explicit accepted-solver projections.  The legacy q_* fields above
    /// remain for formula-inventory compatibility; aggregate filters must use
    /// these names or the proposal_* names, never infer population semantics.
    accepted_q_raw_f64: Option<f64>,
    accepted_q_corrected_f64: Option<f64>,
    accepted_q_correction_f64: Option<f64>,
    q_exact: Option<String>,
    action_f64: Option<f64>,
    accepted_action_f64: Option<f64>,
    action_exact: Option<String>,
    action_exact_defined: Option<String>,
    beta_error_linf: Option<f64>,
    proposal_beta_error_linf: Option<f64>,
    q_raw_error: Option<f64>,
    q_corrected_error: Option<f64>,
    action_error: Option<f64>,
    action_error_bound: Option<f64>,
    q_error_bound: Option<f64>,
    q_beta_radius_bound: Option<f64>,
    kkt_residual_norm: Option<f64>,
    beta_margin_f64: Option<f64>,
    proposal_beta_margin_f64: Option<f64>,
    f64_beta_predicate: String,
    exact_beta_predicate: String,
    f64_q_predicate: String,
    exact_q_predicate: String,
    predicate_category: String,
    q_predicate_category: String,
    qp_c_f64: Vec<Vec<f64>>,
    qp_d_f64: Vec<f64>,
    qp_h_f64: Vec<Vec<f64>>,
    qp_c_exact: Vec<Vec<String>>,
    qp_d_exact: Vec<String>,
    qp_h_exact: Vec<Vec<String>>,
    kkt_matrix_f64: Vec<Vec<f64>>,
    kkt_rhs_f64: Vec<f64>,
    kkt_matrix_exact: Vec<Vec<String>>,
    kkt_rhs_exact: Vec<String>,
    kkt_residual_vector_f64: Option<Vec<f64>>,
    geometry_vertex_facet_incidence: Option<Vec<Vec<bool>>>,
    geometry_facet_intersection: Option<Vec<Vec<bool>>>,
    geometry_transition_matrix: Option<Vec<Vec<bool>>>,
    omega_matrix_f64: Vec<Vec<f64>>,
    omega_matrix_exact: Vec<Vec<String>>,
    capacity_f64: Option<f64>,
    volume_f64: Option<f64>,
    volume_exact: Option<String>,
    sys_f64: Option<f64>,
    derivative_linf: Option<f64>,
    derivative_components: Option<usize>,
    derivative_f64: Option<Vec<Vec<f64>>>,
    recovery_closure_error: Option<f64>,
    recovery_max_violation: Option<f64>,
    recovery_action_error: Option<f64>,
    recovery_action_f64: Option<f64>,
    recovery_dwell_times: Option<Vec<f64>>,
    recovery_valid: Option<bool>,
    route_count_scope: String,
    route_population_sigma_count: usize,
    route_population_admissible_count: usize,
    route_population_indeterminate_count: usize,
    route_population_failure_count: usize,
    route_eligibility_status: String,
    route_attempt_status: String,
    route_state: String,
    route_retained: Option<bool>,
    route_pruned: Option<bool>,
    route_candidate_order_f64: Option<usize>,
    route_candidate_order_exact: Option<usize>,
    route_q_rank_desc: Option<usize>,
    route_action_rank_asc: Option<usize>,
    route_exact_action_rank_asc: Option<usize>,
    route_maximum_q_member: Option<bool>,
    route_minimum_action_member: Option<bool>,
    route_low_action_window_member: Option<bool>,
    unconditional_q_rank_desc: Option<usize>,
    unconditional_action_rank_asc: Option<usize>,
    unconditional_exact_action_rank_asc: Option<usize>,
    unconditional_maximum_q_member: bool,
    unconditional_minimum_action_member: bool,
    unconditional_low_action_window_member: bool,
    unavailable_reason: Option<String>,
}

#[derive(Serialize)]
struct AggregateRow {
    run_id: String,
    case_id: String,
    universe_contract: String,
    row_count: usize,
    exact_resolution_count: usize,
    f64_min_action: Option<f64>,
    exact_min_action: Option<String>,
    f64_runner_up_action: Option<f64>,
    exact_runner_up_action: Option<String>,
    f64_minimizer_sigma: Option<Vec<usize>>,
    exact_minimizer_count: usize,
    f64_minimizer_count: usize,
    f64_low_action_window_count: usize,
    f64_low_action_window_cutoff: Option<f64>,
    exact_low_action_window_count: Option<usize>,
    low_action_window_definition: String,
    candidate_completeness: String,
    allowed_claim: String,
    prohibited_claim: String,
    population_all_count: usize,
    population_maximum_q_count: usize,
    population_minimum_action_count: usize,
    population_production_visited_count: Option<usize>,
    population_production_retained_count: Option<usize>,
    population_exact_resolved_count: usize,
    population_low_action_window_count: usize,
    population_filter_definitions: Vec<String>,
    f64_max_q: Option<f64>,
    f64_max_q_sigma: Option<Vec<usize>>,
    set_preservation_status: String,
    /// Aggregates over every finite mathematically defined proposal atom.
    proposal_min_action: Option<f64>,
    proposal_runner_up_action: Option<f64>,
    proposal_minimizer_sigma: Option<Vec<usize>>,
    proposal_minimizer_count: usize,
    proposal_low_action_window_count: usize,
    proposal_low_action_window_cutoff: Option<f64>,
    proposal_max_q: Option<f64>,
    proposal_max_q_sigma: Option<Vec<usize>>,
    /// Aggregates over accepted production solver outputs only.
    accepted_min_action: Option<f64>,
    accepted_runner_up_action: Option<f64>,
    accepted_minimizer_sigma: Option<Vec<usize>>,
    accepted_minimizer_count: usize,
    accepted_low_action_window_count: usize,
    accepted_low_action_window_cutoff: Option<f64>,
    accepted_max_q: Option<f64>,
    accepted_max_q_sigma: Option<Vec<usize>>,
}

fn source_revision() -> String {
    option_env!("QP_SOURCE_REVISION")
        .unwrap_or("working-tree")
        .to_string()
}

fn source_content_id() -> String {
    option_env!("QP_SOURCE_CONTENT_ID")
        .unwrap_or("working-tree")
        .to_string()
}

fn rational_text(x: &BigRational) -> String {
    format!("{}/{}", x.numer(), x.denom())
}

fn rational_f64(x: &BigRational) -> f64 {
    x.to_f64().unwrap_or(f64::NAN)
}

fn parse_rational(text: &str) -> Option<BigRational> {
    let (n, d) = text.split_once('/')?;
    Some(BigRational::new(n.parse().ok()?, d.parse().ok()?))
}

fn vectors_from_arrays(rows: &[[BigRational; 4]]) -> Vec<Vector4<BigRational>> {
    rows.iter()
        .map(|r| Vector4::new(r[0].clone(), r[1].clone(), r[2].clone(), r[3].clone()))
        .collect()
}

fn case_from_known(kp: &KnownPolytope, case_id: &str, cohort: &str) -> Case {
    let exact = kp.dual_vertices.clone();
    let transition = symplectic::algorithms::facet_adjacency::build_transition_matrix_from_facet_intersections_and_omega(&kp.facet_intersection_is_nonempty, &kp.omega_signs);
    let sigmas = SimpleDirectedCyclesCanonical::new(&transition)
        .take(MAX_SIGMAS_PER_CASE)
        .collect();
    Case {
        case_id: case_id.into(),
        cohort: cohort.into(),
        family: "known_polytope".into(),
        source_id: kp.name.into(),
        dual_f64: kp.dual_vertices_f64.clone(),
        dual_exact: exact,
        vertices_exact: Some(kp.vertices.clone()),
        vertices_f64: Some(kp.vertices_f64.clone()),
        original_exact: true,
        incidence: Some(kp.vertex_facet_incidence.clone()),
        transition: Some(transition),
        sigmas,
        universe_contract: "transition_complete_small_capped".into(),
    }
}

fn case_from_known_sigmas(
    kp: &KnownPolytope,
    case_id: &str,
    cohort: &str,
    sigmas: Vec<Vec<usize>>,
    universe_contract: &str,
) -> Case {
    let mut case = case_from_known(kp, case_id, cohort);
    if kp.name == "hko_pentagon" {
        case.family = "stored_dyadic_hko_like".into();
        case.original_exact = false;
    }
    case.sigmas = sigmas;
    case.universe_contract = universe_contract.into();
    case
}

fn case_from_f64(
    case_id: &str,
    cohort: &str,
    family: &str,
    source_id: &str,
    dual_f64: Vec<Vector4<f64>>,
    sigmas: Option<Vec<Vec<usize>>>,
) -> Option<Case> {
    let dual_exact = exact_binary64_dual_vertex_arrays(&dual_f64);
    let cache = SysLandscapePolytopeCache::from_f64_dual_vertices(dual_f64.clone())?;
    let transition = exact_binary64_transition_matrix_assuming_origin_interior(&dual_exact);
    let generated: Vec<Vec<usize>> = SimpleDirectedCyclesCanonical::new(&transition)
        .take(MAX_SIGMAS_PER_CASE)
        .collect();
    Some(Case {
        case_id: case_id.into(),
        cohort: cohort.into(),
        family: family.into(),
        source_id: source_id.into(),
        dual_f64,
        dual_exact: cache.dual_vertices.clone(),
        vertices_exact: Some(cache.vertices.clone()),
        vertices_f64: Some(cache.vertices_f64.clone()),
        original_exact: false,
        incidence: Some(cache.vertex_facet_incidence.clone()),
        transition: Some(transition),
        sigmas: sigmas.unwrap_or(generated),
        universe_contract: "stored_dyadic_transition_complete_small_capped".into(),
    })
}

fn parse_dual_arrays(values: &[[&str; 4]]) -> Vec<[BigRational; 4]> {
    values
        .iter()
        .map(|row| row.map(|value| parse_rational(value).expect("embedded rational dual vertex")))
        .collect()
}

fn case_from_embedded_original(
    case_id: &str,
    cohort: &str,
    source_id: &str,
    dual_f64: &[[f64; 4]],
    dual_exact: &[[&str; 4]],
    sigmas: Option<Vec<Vec<usize>>>,
    universe_contract: &str,
) -> Case {
    let dual_f64 = dual_f64
        .iter()
        .map(|row| Vector4::new(row[0], row[1], row[2], row[3]))
        .collect::<Vec<_>>();
    let dual_exact = parse_dual_arrays(dual_exact);
    let transition = exact_binary64_transition_matrix_assuming_origin_interior(&dual_exact);
    let generated = SimpleDirectedCyclesCanonical::new(&transition).collect();
    Case {
        case_id: case_id.into(),
        cohort: cohort.into(),
        family: "source_record_original_rational".into(),
        source_id: source_id.into(),
        dual_f64,
        dual_exact,
        vertices_exact: None,
        vertices_f64: None,
        original_exact: true,
        incidence: None,
        transition: Some(transition),
        sigmas: sigmas.unwrap_or(generated),
        universe_contract: universe_contract.into(),
    }
}

const RANDOM_F5_S0_0_DUAL_F64: [[f64; 4]; 5] = [
    [
        0.3058295412454651,
        0.3581182064486669,
        -0.5057391493514737,
        0.673952717487378,
    ],
    [
        0.14082959874930542,
        -0.3846666593533085,
        -0.7813848829272452,
        -0.040905403181863274,
    ],
    [
        -0.005709129791842338,
        0.4345647770756493,
        -0.4577765276459585,
        0.8257015959891256,
    ],
    [
        0.2696713712229452,
        -0.6284061588732212,
        0.7911325125965956,
        0.6426883863832571,
    ],
    [
        -0.3453549099816208,
        0.573928902375191,
        -0.1269653937544231,
        -0.8770702474620782,
    ],
];

const RANDOM_F5_S0_0_DUAL_RATIONAL: [[&str; 4]; 5] = [
    [
        "5509335231967865/18014398509481984",
        "3225642042233613/9007199254740992",
        "-4555293289131937/9007199254740992",
        "6070426414682977/9007199254740992",
    ],
    [
        "5073921027600867/36028797018963968",
        "-6929538494901655/18014398509481984",
        "-219940291724005/281474976710656",
        "-5895089872872939/144115188075855872",
    ],
    [
        "-6582178509606643/1152921504606846976",
        "7828423072424947/18014398509481984",
        "-8246568797301193/18014398509481984",
        "1859314700007925/2251799813685248",
    ],
    [
        "4857967547808587/18014398509481984",
        "-5660179485877527/9007199254740992",
        "445368011116339/562949953421312",
        "1447205588715491/2251799813685248",
    ],
    [
        "-3110680487807597/9007199254740992",
        "646186497718517/1125899906842624",
        "-4574410400010945/36028797018963968",
        "-987493309911991/1125899906842624",
    ],
];

const RANDOM_3X5_S0_0_DUAL_F64: [[f64; 4]; 8] = [
    [0.971921193582736, 0.11023238268070087, 0.0, 0.0],
    [-0.9531208458069649, 0.30073641880924634, 0.0, 0.0],
    [0.5406889100134046, -0.6937138494994952, 0.0, 0.0],
    [0.0, 0.0, 0.8943409860531801, 0.2583871944393862],
    [0.0, 0.0, 0.8573977179828585, 0.3859819324012109],
    [0.0, 0.0, -0.8937644378209634, -0.09135037366206142],
    [0.0, 0.0, -1.0065689521652772, -0.5860891298435376],
    [0.0, 0.0, 0.32729353333347366, -0.8733303165187348],
];

const RANDOM_3X5_S0_0_DUAL_RATIONAL: [[&str; 4]; 8] = [
    [
        "8754287850505395/9007199254740992",
        "3971540140519731/36028797018963968",
        "0/1",
        "0/1",
    ],
    [
        "-4292474686015299/4503599627370496",
        "5417585694744237/18014398509481984",
        "0/1",
        "0/1",
    ],
    [
        "4870092747319457/9007199254740992",
        "-3124209434107679/4503599627370496",
        "0/1",
        "0/1",
    ],
    [
        "0/1",
        "0/1",
        "7866706506897/8796093022208",
        "2327344945189055/9007199254740992",
    ],
    [
        "0/1",
        "0/1",
        "3861376043215915/4503599627370496",
        "3476616173867675/9007199254740992",
    ],
    [
        "0/1",
        "0/1",
        "-8050314378254983/9007199254740992",
        "-3291244070276923/36028797018963968",
    ],
    [
        "0/1",
        "0/1",
        "-4533183557894253/4503599627370496",
        "-1319755393384627/2251799813685248",
    ],
    [
        "0/1",
        "0/1",
        "1473999034761405/4503599627370496",
        "-7866260176090263/9007199254740992",
    ],
];

fn pruning_fixture() -> Vec<Vector4<f64>> {
    vec![
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
    ]
}

fn cases() -> Vec<Case> {
    let mut out = vec![case_from_known(
        known_polytopes::simplex(),
        "simplex_f5",
        "complete_small",
    )];
    out.push(case_from_known(
        known_polytopes::hypercube(),
        "hypercube_f8",
        "complete_small",
    ));
    let hko = known_polytopes::hko_pentagon();
    out.push(case_from_known_sigmas(
        hko,
        "hko_beta_boundary",
        "regression_beta_boundary",
        vec![vec![0, 1, 6, 7, 3, 4, 5, 9]],
        "matched_named_context_only",
    ));
    out.push(case_from_known_sigmas(
        hko,
        "hko_near_singular_false_acceptance",
        "regression_near_singular_false_acceptance",
        vec![vec![1, 8, 7, 3, 4, 5, 9]],
        "matched_named_context_only",
    ));
    out.push(case_from_known_sigmas(
        hko,
        "hko_residual_q_failure",
        "regression_residual_q_failure",
        vec![vec![0, 1, 7, 3, 9, 5]],
        "matched_named_context_only",
    ));
    out.push(case_from_known_sigmas(
        hko,
        "hko_rank_deficient",
        "regression_rank_deficient",
        vec![vec![1, 7, 2, 8, 4, 6, 5]],
        "matched_named_context_only",
    ));
    // This exact-zero boundary is retained as a named original-rational row,
    // separate from the complete-small hypercube population.
    out.push(case_from_known_sigmas(
        known_polytopes::hypercube(),
        "hypercube_exact_zero_beta_boundary",
        "regression_exact_zero_beta_boundary",
        vec![vec![0, 2, 1, 5, 6]],
        "matched_named_context_only",
    ));

    out.push(case_from_embedded_original(
        "random_F5_s0_0",
        "ordinary_seeded_pre_retention",
        "experiments/sys-datascience/produce/random.jsonl#random_F5_s0_0",
        &RANDOM_F5_S0_0_DUAL_F64,
        &RANDOM_F5_S0_0_DUAL_RATIONAL,
        None,
        "source_transition_complete",
    ));

    let product_tie_sigmas = vec![
        vec![1, 5, 6, 0, 2, 3],
        vec![1, 5, 6, 2, 0, 3],
        vec![1, 6, 5, 0, 2, 3],
        vec![1, 6, 5, 2, 0, 3],
    ];
    out.push(case_from_embedded_original(
        "random_3x5_s0_0",
        "minimum_window_tie",
        "experiments/sys-datascience/produce/random-product.jsonl#random_3x5_s0_0",
        &RANDOM_3X5_S0_0_DUAL_F64,
        &RANDOM_3X5_S0_0_DUAL_RATIONAL,
        Some(product_tie_sigmas),
        "exact_within_declared_stream_tie",
    ));

    let generated_dual = generated_f64_cases_with_source_filter(
        1,
        99540836,
        &["seed99540836:q4:p5:attempt405000000000".into()],
    )
    .pop()
    .expect("required generated product tie fixture is missing")
    .dual_vertices;
    out.push(
        case_from_f64(
            "seed99540836_q4_p5_attempt405000000000",
            "pinned_complete_transition",
            "generated_product_f64",
            "seed99540836:q4:p5:attempt405000000000",
            generated_dual,
            None,
        )
        .expect("required generated product tie geometry cache is unavailable"),
    );
    if let Some(case) = out.last_mut() {
        let transition = case.transition.as_ref().expect("generated transition");
        case.sigmas = SimpleDirectedCyclesCanonical::new(transition).collect();
        case.universe_contract = "transition_complete_1294".into();
    }
    out.push(
        case_from_f64(
            "pruning_roundoff",
            "regression_adversarial",
            "stored_dyadic_pruning",
            "route_demonstrations::pruning_roundoff_fixture",
            pruning_fixture(),
            None,
        )
        .expect("required pruning fixture geometry cache is unavailable"),
    );
    out
}

fn predicate_category(f64_label: &str, exact_label: &str) -> String {
    if exact_label == "unavailable" {
        return "exact_unavailable".into();
    }
    if f64_label == "indeterminate" {
        return format!("indeterminate|{}", exact_label);
    }
    if f64_label == exact_label {
        format!("{}|{}_sound", f64_label, exact_label)
    } else {
        format!("{}|{}_unsound", f64_label, exact_label)
    }
}

fn f64_result_is_feasible(status: &str) -> bool {
    status == "feasible"
}

fn matrix_rows(matrix: &DMatrix<f64>) -> Vec<Vec<f64>> {
    (0..matrix.nrows())
        .map(|row| (0..matrix.ncols()).map(|col| matrix[(row, col)]).collect())
        .collect()
}

fn vector_values(vector: &DVector<f64>) -> Vec<f64> {
    vector.iter().copied().collect()
}

fn omega_rows(dual_vertices: &[Vector4<f64>], sigma: &[usize]) -> Vec<Vec<f64>> {
    sigma
        .iter()
        .map(|&i| {
            sigma
                .iter()
                .map(|&j| omega0(&dual_vertices[i], &dual_vertices[j]))
                .collect()
        })
        .collect()
}

fn omega_exact(u: &[BigRational; 4], v: &[BigRational; 4]) -> BigRational {
    &u[0] * &v[2] - &u[2] * &v[0] + &u[1] * &v[3] - &u[3] * &v[1]
}

fn omega_exact_rows(dual_vertices: &[[BigRational; 4]], sigma: &[usize]) -> Vec<Vec<String>> {
    sigma
        .iter()
        .map(|&i| {
            sigma
                .iter()
                .map(|&j| rational_text(&omega_exact(&dual_vertices[i], &dual_vertices[j])))
                .collect()
        })
        .collect()
}

fn exact_q_from_beta(
    dual_vertices: &[[BigRational; 4]],
    sigma: &[usize],
    beta: &[BigRational],
) -> BigRational {
    (1..beta.len())
        .flat_map(|i| (0..i).map(move |j| (i, j)))
        .map(|(i, j)| {
            &beta[i] * &beta[j] * omega_exact(&dual_vertices[sigma[j]], &dual_vertices[sigma[i]])
        })
        .sum()
}

fn exact_qp_rows(
    dual_vertices: &[[BigRational; 4]],
    sigma: &[usize],
) -> (
    Vec<Vec<String>>,
    Vec<String>,
    Vec<Vec<String>>,
    Vec<Vec<String>>,
    Vec<String>,
) {
    let m = sigma.len();
    let mut c = vec![vec![String::from("0/1"); m]; 5];
    for (column, &index) in sigma.iter().enumerate() {
        for coordinate in 0..4 {
            c[coordinate][column] = rational_text(&dual_vertices[index][coordinate]);
        }
        c[4][column] = String::from("1/1");
    }
    let d = vec![String::from("0/1"); 4]
        .into_iter()
        .chain(std::iter::once(String::from("1/1")))
        .collect::<Vec<_>>();
    let mut h = vec![vec![String::from("0/1"); m]; m];
    for i in 0..m {
        for j in (i + 1)..m {
            let value = rational_text(&omega_exact(
                &dual_vertices[sigma[i]],
                &dual_vertices[sigma[j]],
            ));
            h[i][j] = value.clone();
            h[j][i] = value;
        }
    }
    let size = m + 5;
    let mut kkt = vec![vec![String::from("0/1"); size]; size];
    for i in 0..m {
        for j in 0..m {
            kkt[i][j] = h[i][j].clone();
        }
        for coordinate in 0..4 {
            let value = rational_text(&dual_vertices[sigma[i]][coordinate]);
            kkt[i][m + coordinate] = value.clone();
            kkt[m + coordinate][i] = value;
        }
        kkt[i][m + 4] = String::from("1/1");
        kkt[m + 4][i] = String::from("1/1");
    }
    let mut rhs = vec![String::from("0/1"); size];
    rhs[m + 4] = String::from("1/1");
    (c, d, h, kkt, rhs)
}

struct KktProposal {
    solution: Option<DVector<f64>>,
    residual: Option<DVector<f64>>,
    residual_norm: Option<f64>,
    singular_values: Vec<f64>,
    eigenvalues: Vec<f64>,
    rank: Option<usize>,
    inertia: Option<(usize, usize, usize)>,
}

fn proposal_q_atoms(
    proposal: &KktProposal,
    q_raw: Option<f64>,
    m: usize,
) -> (Option<f64>, Option<f64>, Option<f64>, Option<f64>) {
    let Some(solution) = proposal.solution.as_ref() else {
        return (None, None, None, None);
    };
    let Some(residual) = proposal.residual.as_ref() else {
        return (None, None, None, None);
    };
    let correction = (0..4)
        .map(|index| residual[m + index] * solution[m + index])
        .sum::<f64>()
        + residual[m + 4] * solution[m + 4];
    let corrected = q_raw.map(|q| q + correction);
    let min_eigen = proposal
        .eigenvalues
        .iter()
        .map(|value| value.abs())
        .filter(|value| *value > 1e-14)
        .fold(f64::INFINITY, f64::min);
    let error_bound = if min_eigen.is_finite() {
        proposal
            .residual_norm
            .map(|norm| 4.5 * norm * norm / min_eigen)
    } else {
        None
    };
    let action = corrected.filter(|q| *q > 0.0).map(|q| 0.5 / q);
    (Some(correction), corrected, error_bound, action)
}

struct ExactLinearDiagnostics {
    consistency_status: String,
    rank: Option<usize>,
    nullity: Option<usize>,
    beta_particular: Option<Vec<BigRational>>,
}

fn exact_linear_diagnostics(
    dual_vertices: &[[BigRational; 4]],
    sigma: &[usize],
) -> ExactLinearDiagnostics {
    let m = sigma.len();
    let size = m + 5;
    let (_, _, _h, kkt, rhs) = exact_qp_rows(dual_vertices, sigma);
    let mut matrix = DMatrix::from_element(size, size, BigRational::zero());
    let mut rhs_exact = DVector::from_element(size, BigRational::zero());
    for i in 0..size {
        rhs_exact[i] = parse_rational(&rhs[i]).unwrap_or_else(BigRational::zero);
        for j in 0..size {
            matrix[(i, j)] = parse_rational(&kkt[i][j]).unwrap_or_else(BigRational::zero);
        }
    }
    match solve_linear_system(&matrix, &rhs_exact) {
        LinearSystemSolution::Inconsistent => ExactLinearDiagnostics {
            consistency_status: "inconsistent".into(),
            rank: Some(exact_matrix_rank(&matrix)),
            nullity: Some(size.saturating_sub(exact_matrix_rank(&matrix))),
            beta_particular: None,
        },
        LinearSystemSolution::Consistent {
            particular,
            kernel_basis,
        } => {
            let rank = size.saturating_sub(kernel_basis.ncols());
            ExactLinearDiagnostics {
                consistency_status: if kernel_basis.ncols() == 0 {
                    "consistent_unique".into()
                } else {
                    "consistent_rank_deficient".into()
                },
                rank: Some(rank),
                nullity: Some(kernel_basis.ncols()),
                beta_particular: Some(particular.iter().take(m).cloned().collect()),
            }
        }
    }
}

/// Produce an unconditional least-squares proposal and matrix diagnostics.
/// This is deliberately separate from the production `KktOutcome`: a proposal
/// remains an observation even when positivity or the production residual gate
/// rejects the candidate.
fn kkt_proposal(kkt: &DMatrix<f64>, rhs: &DVector<f64>) -> KktProposal {
    let svd = kkt.clone().svd(true, true);
    let singular_values: Vec<f64> = svd.singular_values.iter().copied().collect();
    let max_sv = singular_values.iter().copied().fold(0.0, f64::max);
    let tolerance = if max_sv.is_finite() {
        (max_sv * 1e-12).max(1e-14)
    } else {
        f64::INFINITY
    };
    let rank = max_sv.is_finite().then(|| {
        singular_values
            .iter()
            .filter(|&&value| value > tolerance)
            .count()
    });
    let solution = match (svd.u.as_ref(), svd.v_t.as_ref(), rank) {
        (Some(u), Some(v_t), Some(rank)) if rank > 0 => {
            let mut x = DVector::zeros(kkt.ncols());
            for index in 0..rank {
                let coefficient = u.column(index).dot(rhs) / svd.singular_values[index];
                x += v_t.row(index).transpose() * coefficient;
            }
            Some(x)
        }
        _ => None,
    };
    let residual = solution.as_ref().map(|x| kkt * x - rhs);
    let residual_norm = residual.as_ref().map(DVector::norm);
    let eigenvalues = if kkt.nrows() == kkt.ncols() {
        kkt.clone()
            .symmetric_eigen()
            .eigenvalues
            .iter()
            .copied()
            .collect::<Vec<_>>()
    } else {
        Vec::new()
    };
    let inertia = if eigenvalues.is_empty() {
        None
    } else {
        let eigen_tolerance = eigenvalues
            .iter()
            .copied()
            .map(f64::abs)
            .fold(0.0, f64::max)
            * 1e-12;
        Some((
            eigenvalues
                .iter()
                .filter(|&&value| value > eigen_tolerance)
                .count(),
            eigenvalues
                .iter()
                .filter(|&&value| value < -eigen_tolerance)
                .count(),
            eigenvalues
                .iter()
                .filter(|&&value| value.abs() <= eigen_tolerance)
                .count(),
        ))
    };
    KktProposal {
        solution,
        residual,
        residual_norm,
        singular_values,
        eigenvalues,
        rank,
        inertia,
    }
}

fn bool_matrix_rows(matrix: &DMatrix<bool>) -> Vec<Vec<bool>> {
    (0..matrix.nrows())
        .map(|row| (0..matrix.ncols()).map(|col| matrix[(row, col)]).collect())
        .collect()
}

fn facet_intersection_from_incidence(incidence: &DMatrix<bool>) -> DMatrix<bool> {
    let facets = incidence.ncols();
    let mut intersection = DMatrix::from_element(facets, facets, false);
    for i in 0..facets {
        for j in 0..facets {
            if i != j {
                intersection[(i, j)] = (0..incidence.nrows())
                    .any(|vertex| incidence[(vertex, i)] && incidence[(vertex, j)]);
            }
        }
    }
    intersection
}

fn observe(
    case: &Case,
    sigma: &[usize],
    run_id: &str,
    capacity: Option<f64>,
    volume: Option<f64>,
    volume_exact: Option<String>,
    sys: Option<f64>,
    route_counts: (usize, usize, usize, usize),
) -> RawRow {
    let f64_started = Instant::now();
    let f64_result = solve_kkt_for_dual_vertices(&case.dual_f64, sigma);
    let f64_solver_elapsed_us = f64_started.elapsed().as_secs_f64() * 1e6;
    let qp = build_qp_from_dual_vertices(&case.dual_f64, sigma);
    let qp_c_f64 = matrix_rows(&qp.c);
    let qp_d_f64 = vector_values(&qp.d);
    let qp_h_f64 = matrix_rows(&qp.h);
    let (kkt, rhs) = build_augmented_system_from_dual_vertices(&case.dual_f64, sigma);
    let kkt_matrix_f64 = matrix_rows(&kkt);
    let kkt_rhs_f64 = vector_values(&rhs);
    let proposal = kkt_proposal(&kkt, &rhs);
    let proposal_beta = proposal.solution.as_ref().map(|solution| {
        solution
            .rows(0, sigma.len())
            .iter()
            .copied()
            .collect::<Vec<_>>()
    });
    let proposal_mu = proposal.solution.as_ref().map(|solution| {
        solution
            .rows(sigma.len(), 4)
            .iter()
            .copied()
            .collect::<Vec<_>>()
    });
    let proposal_xi = proposal
        .solution
        .as_ref()
        .map(|solution| solution[sigma.len() + 4]);
    let proposal_q = proposal_beta.as_ref().map(|beta| q_value(&qp.h, beta));
    let proposal_residual_vector = proposal.residual.as_ref().map(vector_values);
    // Q(beta) is mathematically defined for the unconditional least-squares
    // proposal even when production rejects beta for positivity or residual
    // reasons.  Keep the accepted solver's corrected Q separate below.
    let q_raw = proposal_q;
    let accepted_q_raw = match &f64_result {
        KktOutcome::Feasible(k) => Some(q_value(&qp.h, &k.beta)),
        _ => None,
    };
    let mut beta_f64 = None;
    let mut mu = None;
    let mut xi = None;
    let mut q_corr = None;
    let mut q_bound = None;
    let f64_status;
    let mut derivative = None;
    let mut recovery = None;
    let (f64_pred, action_f64): (String, Option<f64>) = match &f64_result {
        KktOutcome::Feasible(k) => {
            beta_f64 = Some(k.beta.clone());
            mu = Some(k.mu.clone());
            xi = Some(k.xi);
            q_corr = Some(k.q_corrected);
            q_bound = Some(k.q_error_bound);
            f64_status = "feasible".into();
            let action = (k.q_corrected > 0.0).then(|| 0.5 / k.q_corrected);
            derivative =
                action.map(|_| capacity_derivatives_a_from_kkt_result(&case.dual_f64, sigma, k));
            recovery = action.and_then(|a| {
                recover_and_verify_sigma_beta_action(&case.dual_f64, sigma, &k.beta, a)
            });
            (
                if k.beta.iter().all(|b| *b > 1e-9) {
                    "true"
                } else if k.beta.iter().any(|b| *b < -1e-9) {
                    "false"
                } else {
                    "indeterminate"
                }
                .into(),
                action,
            )
        }
        KktOutcome::Infeasible => {
            f64_status = "infeasible".into();
            ("indeterminate".into(), None)
        }
        KktOutcome::SingularMatrix => {
            f64_status = "singular_matrix".into();
            ("indeterminate".into(), None)
        }
        other => {
            f64_status = format!("{other:?}");
            ("indeterminate".into(), None)
        }
    };
    let (proposal_correction, proposal_corrected, proposal_q_bound, proposal_action_corrected) =
        proposal_q_atoms(&proposal, q_raw, sigma.len());
    // Proposal atoms remain separate from production-result centers.  Offline
    // consumers may compare either center explicitly without mixing them.
    let exact_started = Instant::now();
    let exact = symplectic::kkt::rational_solver::solve_kkt_exact(&case.dual_exact, sigma);
    let exact_diag = exact_linear_diagnostics(&case.dual_exact, sigma);
    let exact_solver_elapsed_us = exact_started.elapsed().as_secs_f64() * 1e6;
    let (qp_c_exact, qp_d_exact, qp_h_exact, kkt_matrix_exact, kkt_rhs_exact) =
        exact_qp_rows(&case.dual_exact, sigma);
    let target_dual_vertices_exact = case
        .dual_exact
        .iter()
        .map(|vertex| vertex.iter().map(rational_text).collect())
        .collect::<Vec<Vec<_>>>();
    let stored_dyadic_dual_vertices_exact = (!case.original_exact).then(|| {
        case.dual_f64
            .iter()
            .map(|vertex| {
                vertex
                    .iter()
                    .map(|coordinate| {
                        rational_text(
                            &BigRational::from_float(*coordinate)
                                .expect("finite stored f64 coordinate"),
                        )
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<Vec<_>>>()
    });
    // `exact_beta_predicate` is a feasibility statement about the whole exact
    // KKT system, not a classification of an arbitrary row-reduction vector:
    // the rational solver's `Some` result certifies existence of beta > 0;
    // consistency without such a witness certifies false; inconsistency is
    // unavailable.  Particular-vector sign information is retained under a
    // separate field and is never used as a beta-error reference.
    let exact_beta_pred = if exact.is_some() {
        "true"
    } else if exact_diag.consistency_status == "inconsistent" {
        "unavailable"
    } else {
        "false"
    }
    .to_string();
    let exact_beta_particular_predicate = exact_diag
        .beta_particular
        .as_ref()
        .map(|beta| {
            if beta.iter().all(|b| b > &BigRational::zero()) {
                "true"
            } else if beta.iter().any(|b| b < &BigRational::zero()) {
                "false"
            } else {
                "indeterminate"
            }
            .to_string()
        })
        .unwrap_or_else(|| "unavailable".into());
    let beta_exact_rational = (exact_diag.consistency_status == "consistent_unique")
        .then(|| exact.as_ref().map(|e| e.beta.clone()))
        .flatten();
    let q_reference_beta = exact
        .as_ref()
        .map(|e| e.beta.clone())
        .or_else(|| exact_diag.beta_particular.clone());
    let exact_beta_selection_status =
        match (exact.is_some(), exact_diag.consistency_status.as_str()) {
            (true, "consistent_unique") => "unique_exact_solution",
            (true, _) => "positive_witness_not_error_reference",
            (false, "inconsistent") => "unavailable_inconsistent",
            (false, _) => "particular_row_reduction_vector",
        }
        .to_string();
    let q_exact_rational = exact.as_ref().map(|e| e.q_exact.clone()).or_else(|| {
        q_reference_beta
            .as_ref()
            .map(|beta| exact_q_from_beta(&case.dual_exact, sigma, beta))
    });
    let q_exact = q_exact_rational.as_ref().map(rational_text);
    let action_exact_value = exact
        .as_ref()
        .filter(|e| e.q_exact > BigRational::zero())
        .map(|e| BigRational::one() / (&e.q_exact + &e.q_exact));
    let action_exact = action_exact_value.as_ref().map(rational_text);
    let action_exact_defined = q_exact_rational
        .as_ref()
        .filter(|q| !q.is_zero())
        .map(|q| rational_text(&(BigRational::one() / (q + q))));
    let beta_exact = beta_exact_rational
        .as_ref()
        .map(|beta| beta.iter().map(rational_text).collect());
    let beta_error = match (&beta_f64, &beta_exact_rational) {
        (Some(b), Some(e)) => b
            .iter()
            .zip(e)
            .map(|(x, y)| {
                (BigRational::from_float(*x).unwrap() - y)
                    .to_f64()
                    .unwrap_or(f64::NAN)
                    .abs()
            })
            .reduce(f64::max),
        _ => None,
    };
    let proposal_beta_error = match (&proposal_beta, &beta_exact_rational) {
        (Some(b), Some(e)) => b
            .iter()
            .zip(e)
            .map(|(x, y)| {
                (BigRational::from_float(*x).unwrap() - y)
                    .to_f64()
                    .unwrap_or(f64::NAN)
                    .abs()
            })
            .reduce(f64::max),
        _ => None,
    };
    let q_raw_error = match (q_raw, &q_exact_rational) {
        (Some(q), Some(e)) => Some(
            (BigRational::from_float(q).unwrap() - e)
                .to_f64()
                .unwrap_or(f64::NAN)
                .abs(),
        ),
        _ => None,
    };
    let q_corr_error = match (q_corr, &q_exact_rational) {
        (Some(q), Some(e)) => Some(
            (BigRational::from_float(q).unwrap() - e)
                .to_f64()
                .unwrap_or(f64::NAN)
                .abs(),
        ),
        _ => None,
    };
    let action_error = match (action_f64, &action_exact_value) {
        (Some(a), Some(e)) => Some(
            (BigRational::from_float(a).unwrap() - e)
                .to_f64()
                .unwrap_or(f64::NAN)
                .abs(),
        ),
        _ => None,
    };
    let action_error_bound = match (q_corr, q_bound, action_f64) {
        (Some(q), Some(bound), Some(a)) if q > bound && q > 0.0 => {
            let lower = 0.5 / (q - bound);
            let upper = 0.5 / (q + bound);
            Some((a - lower).abs().max((a - upper).abs()))
        }
        _ => None,
    };
    let beta_margin = beta_f64
        .as_ref()
        .map(|b| b.iter().copied().fold(f64::INFINITY, f64::min));
    let proposal_beta_margin = proposal_beta
        .as_ref()
        .map(|b| b.iter().copied().fold(f64::INFINITY, f64::min));
    let q_pred = q_corr
        .map(|q| {
            if q > 1e-15 {
                "true"
            } else if q < -1e-15 {
                "false"
            } else {
                "indeterminate"
            }
        })
        .unwrap_or("indeterminate");
    let exact_q_pred = q_exact_rational
        .as_ref()
        .map(|q| {
            if q > &BigRational::zero() {
                "true"
            } else {
                "false"
            }
        })
        .unwrap_or("unavailable");
    let derivative_linf = derivative.as_ref().map(|g| {
        g.iter()
            .flat_map(|v| v.iter())
            .map(|x| x.abs())
            .fold(0.0, f64::max)
    });
    let q_beta_radius_bound = proposal.solution.as_ref().and_then(|solution| {
        let residual = proposal.residual.as_ref()?;
        let inverse = kkt.clone().try_inverse()?;
        let rho = residual.iter().map(|v| v.abs()).fold(0.0, f64::max);
        let radius = inverse
            .abs()
            .row_iter()
            .map(|row| row.iter().copied().sum::<f64>())
            .fold(0.0, f64::max)
            * rho;
        let h = kkt.view((0, 0), (sigma.len(), sigma.len()));
        let beta = solution.rows(0, sigma.len());
        let hnorm = h.iter().map(|v| v.abs()).sum::<f64>();
        let hbeta = (h * beta).iter().map(|v| v.abs()).sum::<f64>();
        Some(radius * hbeta + 0.5 * radius * radius * hnorm)
    });
    let accepted_residual = match (&beta_f64, &mu, xi) {
        (Some(beta), Some(mu), Some(xi)) => {
            let mut x = DVector::zeros(sigma.len() + 5);
            for (i, value) in beta.iter().enumerate() {
                x[i] = *value;
            }
            for (i, value) in mu.iter().enumerate() {
                x[sigma.len() + i] = *value;
            }
            x[sigma.len() + 4] = xi;
            Some(&kkt * x - &rhs)
        }
        _ => None,
    };
    let residual_for_center = accepted_residual
        .as_ref()
        .map(vector_values)
        .or(proposal_residual_vector);
    let kkt_residual_norm = accepted_residual
        .as_ref()
        .map(DVector::norm)
        .or(proposal.residual_norm);
    let (closure, violation, rec_action, valid) = recovery
        .as_ref()
        .map(|r| {
            (
                Some(r.closure_error),
                Some(r.max_violation),
                Some((r.action - action_f64.unwrap_or(r.action)).abs()),
                Some(r.closure_error < 1e-8 && r.max_violation < 1e-8),
            )
        })
        .unwrap_or((None, None, None, None));
    let recovery_action_f64 = recovery.as_ref().map(|orbit| orbit.action);
    let recovery_dwell_times = recovery.as_ref().map(|orbit| orbit.dwell_times.clone());
    let kkt_residual_vector_f64 = residual_for_center;
    let geometry_vertex_facet_incidence = case.incidence.as_ref().map(bool_matrix_rows);
    let geometry_facet_intersection = case
        .incidence
        .as_ref()
        .map(facet_intersection_from_incidence)
        .as_ref()
        .map(bool_matrix_rows);
    let geometry_transition_matrix = case.transition.as_ref().map(bool_matrix_rows);
    let (stage, reason) = if f64_status == "feasible" {
        ("visited", "production_kkt")
    } else {
        ("visited", "production_outcome")
    };
    let exact_solver_status = if exact.is_some() {
        "feasible"
    } else {
        "infeasible_or_singular"
    };
    let (exact_algebra_status, exact_algebra_reason) = match exact.as_ref() {
        Some(_) => (
            "rational_oracle_feasible",
            "exact rational KKT returned a strictly positive witness",
        ),
        None if exact_diag.consistency_status == "inconsistent" => (
            "rational_system_inconsistent",
            "exact rational KKT row reduction found an inconsistent system",
        ),
        None if exact_diag.beta_particular.is_some() => (
            "consistent_no_positive_beta",
            "exact rational KKT system is consistent but its positive-beta oracle returned no witness",
        ),
        None => (
            "rational_system_unresolved",
            "exact rational row reduction did not produce a usable witness",
        ),
    };
    let proposal_status = match proposal.solution.as_ref() {
        Some(_) if proposal.rank == Some(kkt.nrows()) => "least_squares_full_rank",
        Some(_) => "least_squares_rank_deficient",
        None => "unavailable_svd_proposal",
    };
    RawRow {
        run_id: run_id.into(),
        source_revision: source_revision(),
        producer_version: PRODUCER_VERSION.into(),
        schema_version: SCHEMA_VERSION.into(),
        case_id: case.case_id.clone(),
        cohort: case.cohort.clone(),
        source_family: case.family.clone(),
        source_id: case.source_id.clone(),
        target_id: if case.original_exact {
            "original_rational"
        } else {
            "stored_dyadic"
        }
        .into(),
        target_coordinate_kind: if case.original_exact {
            "original_rational_coordinates"
        } else {
            "exact_binary64_dyadic_coordinates"
        }
        .into(),
        target_dual_vertices_exact,
        original_rational_dual_vertices_exact: case.original_exact.then(|| {
            case.dual_exact
                .iter()
                .map(|vertex| vertex.iter().map(rational_text).collect())
                .collect()
        }),
        stored_dyadic_dual_vertices_exact,
        target_preprocessing_status:
            "none_observed_target_is_not_a_transformed_or_preprocessed_object".into(),
        universe_contract: case.universe_contract.clone(),
        sigma: sigma.to_vec(),
        lifecycle_stage: stage.into(),
        lifecycle_reason: reason.into(),
        lifecycle_events: vec![
            "declared".into(),
            "route_eligible".into(),
            "attempted".into(),
            stage.into(),
            f64_status.clone(),
        ],
        geometry_status: if case.incidence.is_some() {
            "exact_incidence_available".into()
        } else {
            "unavailable:geometry oracle not retained for this case".into()
        },
        predictor_status: "unavailable:no local predictor API in production crate".into(),
        f64_solver_status: f64_status.clone(),
        exact_solver_status: exact_solver_status.into(),
        f64_solver_elapsed_us,
        exact_solver_elapsed_us,
        exact_algebra_status: exact_algebra_status.into(),
        exact_algebra_reason: exact_algebra_reason.into(),
        exact_consistency_status: exact_diag.consistency_status,
        exact_rank: exact_diag.rank,
        exact_nullity: exact_diag.nullity,
        exact_beta_witness: exact
            .as_ref()
            .map(|e| e.beta.iter().map(rational_text).collect())
            .or_else(|| {
                exact_diag
                    .beta_particular
                    .map(|beta| beta.iter().map(rational_text).collect())
            }),
        exact_beta_particular_predicate,
        exact_beta_selection_status,
        intended_algebraic_status: if case.original_exact {
            "not_applicable_original_rational_source"
        } else {
            "unavailable_no_genuine_algebraic_oracle"
        }
        .into(),
        proposal_status: proposal_status.into(),
        proposal_beta_f64: proposal_beta,
        proposal_mu_f64: proposal_mu,
        proposal_xi_f64: proposal_xi,
        proposal_q_f64: proposal_q,
        proposal_q_raw_f64: q_raw,
        proposal_q_correction_f64: proposal_correction,
        proposal_q_corrected_f64: proposal_corrected,
        proposal_q_error_bound_f64: proposal_q_bound,
        proposal_q_beta_radius_bound: q_beta_radius_bound,
        proposal_action_f64: proposal_action_corrected,
        proposal_residual_norm: proposal.residual_norm,
        proposal_rank: proposal.rank,
        proposal_nullity: proposal.rank.map(|rank| kkt.nrows().saturating_sub(rank)),
        proposal_singular_values_f64: proposal.singular_values,
        proposal_eigenvalues_f64: proposal.eigenvalues,
        proposal_inertia_positive: proposal.inertia.as_ref().map(|inertia| inertia.0),
        proposal_inertia_negative: proposal.inertia.as_ref().map(|inertia| inertia.1),
        proposal_inertia_zero: proposal.inertia.as_ref().map(|inertia| inertia.2),
        beta_f64,
        beta_exact,
        mu_f64: mu,
        xi_f64: xi,
        q_raw_f64: q_raw,
        q_corrected_f64: q_corr,
        q_correction_f64: match &f64_result {
            KktOutcome::Feasible(k) => Some(k.q_correction),
            _ => None,
        },
        accepted_q_raw_f64: accepted_q_raw,
        accepted_q_corrected_f64: q_corr,
        accepted_q_correction_f64: match &f64_result {
            KktOutcome::Feasible(k) => Some(k.q_correction),
            _ => None,
        },
        q_exact,
        action_f64,
        accepted_action_f64: action_f64,
        action_exact,
        action_exact_defined,
        beta_error_linf: beta_error,
        proposal_beta_error_linf: proposal_beta_error,
        q_raw_error,
        q_corrected_error: q_corr_error,
        action_error,
        action_error_bound,
        q_error_bound: q_bound,
        q_beta_radius_bound: q_beta_radius_bound.filter(|_| f64_status == "feasible"),
        kkt_residual_norm,
        beta_margin_f64: beta_margin,
        proposal_beta_margin_f64: proposal_beta_margin,
        f64_beta_predicate: f64_pred.clone(),
        exact_beta_predicate: exact_beta_pred.clone(),
        f64_q_predicate: q_pred.into(),
        exact_q_predicate: exact_q_pred.into(),
        predicate_category: predicate_category(&f64_pred, &exact_beta_pred),
        q_predicate_category: predicate_category(
            if q_pred == "indeterminate" {
                "indeterminate"
            } else {
                q_pred
            },
            &exact_q_pred,
        ),
        qp_c_f64,
        qp_d_f64,
        qp_h_f64,
        qp_c_exact,
        qp_d_exact,
        qp_h_exact,
        kkt_matrix_f64,
        kkt_rhs_f64,
        kkt_matrix_exact,
        kkt_rhs_exact,
        kkt_residual_vector_f64,
        geometry_vertex_facet_incidence,
        geometry_facet_intersection,
        geometry_transition_matrix,
        omega_matrix_f64: omega_rows(&case.dual_f64, sigma),
        omega_matrix_exact: omega_exact_rows(&case.dual_exact, sigma),
        capacity_f64: capacity,
        volume_f64: volume,
        volume_exact,
        sys_f64: sys,
        derivative_linf,
        derivative_components: derivative.as_ref().map(|g| g.len()),
        derivative_f64: derivative.as_ref().map(|gradient| {
            gradient
                .iter()
                .map(|vector| vector.iter().copied().collect())
                .collect()
        }),
        recovery_closure_error: closure,
        recovery_max_violation: violation,
        recovery_action_error: rec_action,
        recovery_action_f64,
        recovery_dwell_times,
        recovery_valid: valid,
        route_count_scope: "case_population_summary_repeated_on_each_sigma_row".into(),
        route_population_sigma_count: route_counts.0,
        route_population_admissible_count: route_counts.1,
        route_population_indeterminate_count: route_counts.2,
        route_population_failure_count: route_counts.3,
        route_eligibility_status: "declared_sigma;production_route_status_unavailable".into(),
        route_attempt_status: "unavailable:aggregate_route_only".into(),
        route_state: "unavailable:aggregate_route_only".into(),
        route_retained: None,
        route_pruned: None,
        route_candidate_order_f64: None,
        route_candidate_order_exact: None,
        route_q_rank_desc: None,
        route_action_rank_asc: None,
        route_exact_action_rank_asc: None,
        route_maximum_q_member: None,
        route_minimum_action_member: None,
        route_low_action_window_member: None,
        unconditional_q_rank_desc: None,
        unconditional_action_rank_asc: None,
        unconditional_exact_action_rank_asc: None,
        unconditional_maximum_q_member: false,
        unconditional_minimum_action_member: false,
        unconditional_low_action_window_member: false,
        unavailable_reason: if !case.original_exact
            && case.family == "stored_dyadic_hko_like"
            && case.source_id == "hko_pentagon"
        {
            Some("intended algebraic source unavailable; stored dyadic oracle retained".into())
        } else {
            None
        },
    }
}

fn write_jsonl<T: Serialize>(path: &PathBuf, rows: &[T]) -> std::io::Result<()> {
    let mut w = BufWriter::new(File::create(path)?);
    for row in rows {
        serde_json::to_writer(&mut w, row)?;
        writeln!(w)?;
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("artifacts/broad"));
    create_dir_all(&out)?;
    let revision = source_revision();
    let run_id = format!("wide-{revision}");
    let mut rows = Vec::new();
    let mut aggregates = Vec::new();
    let started = Instant::now();
    for case in cases() {
        let cache = case
            .vertices_exact
            .as_ref()
            .filter(|v| v.len() >= 5)
            .and_then(|v| {
                SysLandscapePolytopeCache::from_rational_parts(case.dual_exact.clone(), v.clone())
            });
        let (capacity, route_counts) =
            match exp_dev_quadratic_program::capacity_f64_only_with_policy_profiled(
                &case.dual_f64,
                F64ValidationPolicy::LpOriginVertex,
            ) {
                (report, _) => {
                    let cap = match report.outcome {
                        exp_dev_quadratic_program::F64CapacityOutcome::Success {
                            capacity, ..
                        } => Some(capacity),
                        _ => None,
                    };
                    (
                        cap,
                        (
                            report.sigma_count as usize,
                            report.admissible_f64_count,
                            report.indeterminate_f64_count,
                            report.numerical_failure_count,
                        ),
                    )
                }
            };
        let volume = case.vertices_f64.as_ref().and_then(|vertices| {
            volume_from_incidence_f64(vertices, case.incidence.as_ref()?).ok()
        });
        let volume_exact = cache.as_ref().map(|c| {
            rational_text(&volume_from_incidence_exact(
                &vectors_from_arrays(&c.vertices),
                &c.vertex_facet_incidence,
            ))
        });
        let sys = capacity
            .zip(volume)
            .map(|(c, v)| symplectic::systolic_ratio(c, v));
        let mut case_rows = Vec::new();
        for sigma in &case.sigmas {
            case_rows.push(observe(
                &case,
                sigma,
                &run_id,
                capacity,
                volume,
                volume_exact.clone(),
                sys,
                route_counts,
            ));
        }
        // Add consumer-facing population annotations after the unconditional
        // rows exist. These are observational memberships, not recall claims
        // about the production route's hidden candidate enumeration.
        let mut q_order: Vec<(usize, f64)> = case_rows
            .iter()
            .enumerate()
            .filter_map(|(index, row)| {
                row.proposal_q_raw_f64
                    .filter(|q| q.is_finite())
                    .map(|q| (index, q))
            })
            .collect();
        q_order.sort_by(|left, right| right.1.total_cmp(&left.1));
        let max_q = q_order.first().map(|item| item.1);
        let q_tie_tolerance = 1e-12;
        let max_q_indices: Vec<usize> = max_q
            .map(|maximum| {
                q_order
                    .iter()
                    .filter(|(_, q)| (*q - maximum).abs() <= q_tie_tolerance)
                    .map(|(index, _)| *index)
                    .collect()
            })
            .unwrap_or_default();
        for (rank, (index, _)) in q_order.iter().enumerate() {
            case_rows[*index].unconditional_q_rank_desc = Some(rank + 1);
        }
        let mut proposal_actions: Vec<(f64, Vec<usize>)> = case_rows
            .iter()
            .filter_map(|r| r.proposal_action_f64.map(|a| (a, r.sigma.clone())))
            .collect();
        proposal_actions.sort_by(|a, b| a.0.total_cmp(&b.0));
        let mut proposal_action_order: Vec<(usize, f64)> = case_rows
            .iter()
            .enumerate()
            .filter_map(|(index, row)| row.proposal_action_f64.map(|action| (index, action)))
            .collect();
        proposal_action_order.sort_by(|left, right| left.1.total_cmp(&right.1));
        for (rank, (index, _)) in proposal_action_order.iter().enumerate() {
            case_rows[*index].unconditional_action_rank_asc = Some(rank + 1);
        }
        let mut accepted_actions: Vec<(usize, f64)> = case_rows
            .iter()
            .enumerate()
            .filter_map(|(index, row)| row.accepted_action_f64.map(|action| (index, action)))
            .collect();
        accepted_actions.sort_by(|left, right| left.1.total_cmp(&right.1));
        let mut exact_actions: Vec<(String, Vec<usize>)> = case_rows
            .iter()
            .filter_map(|r| r.action_exact.clone().map(|a| (a, r.sigma.clone())))
            .collect();
        exact_actions.sort_by(|a, b| parse_rational(&a.0).cmp(&parse_rational(&b.0)));
        let mut exact_order: Vec<(usize, BigRational)> = case_rows
            .iter()
            .enumerate()
            .filter_map(|(index, row)| {
                row.action_exact
                    .as_ref()
                    .and_then(|action| parse_rational(action))
                    .map(|action| (index, action))
            })
            .collect();
        exact_order.sort_by(|left, right| left.1.cmp(&right.1));
        for (rank, (index, _)) in exact_order.iter().enumerate() {
            case_rows[*index].unconditional_exact_action_rank_asc = Some(rank + 1);
        }
        let exact_min = exact_actions.first().map(|x| x.0.clone());
        let exact_minimizer_count = exact_actions
            .first()
            .map(|x| exact_actions.iter().filter(|item| item.0 == x.0).count())
            .unwrap_or(0);
        let proposal_min = proposal_actions.first().map(|x| x.0);
        let proposal_low_cutoff = proposal_min.map(|minimum| minimum * 1.05);
        let proposal_minimizer_count = proposal_min
            .map(|minimum| {
                proposal_actions
                    .iter()
                    .filter(|(action, _)| *action == minimum)
                    .count()
            })
            .unwrap_or(0);
        let proposal_low_action_window_count = proposal_low_cutoff
            .map(|cutoff| {
                proposal_actions
                    .iter()
                    .filter(|(action, _)| *action <= cutoff)
                    .count()
            })
            .unwrap_or(0);
        let accepted_min = accepted_actions.first().map(|x| x.1);
        let accepted_low_cutoff = accepted_min.map(|minimum| minimum * 1.05);
        let accepted_minimizer_count = accepted_min
            .map(|minimum| {
                accepted_actions
                    .iter()
                    .filter(|(_, a)| (*a - minimum).abs() <= 1e-12)
                    .count()
            })
            .unwrap_or(0);
        let accepted_low_action_window_count = accepted_low_cutoff
            .map(|cutoff| {
                accepted_actions
                    .iter()
                    .filter(|(_, a)| *a <= cutoff)
                    .count()
            })
            .unwrap_or(0);
        let exact_low_action_window_count = exact_actions
            .first()
            .and_then(|(minimum, _)| parse_rational(minimum))
            .map(|minimum| {
                let cutoff = minimum * BigRational::new(21.into(), 20.into());
                exact_actions
                    .iter()
                    .filter_map(|(action, _)| parse_rational(action))
                    .filter(|action| *action <= cutoff)
                    .count()
            });
        let f64_low_cutoff_for_rows = proposal_low_cutoff;
        let minimum_action_indices: Vec<usize> = proposal_min
            .map(|minimum| {
                proposal_action_order
                    .iter()
                    .filter(|(_, action)| (*action - minimum).abs() <= 1e-12)
                    .map(|(index, _)| *index)
                    .collect()
            })
            .unwrap_or_default();
        let low_action_indices: Vec<usize> = f64_low_cutoff_for_rows
            .map(|cutoff| {
                proposal_action_order
                    .iter()
                    .filter(|(_, action)| *action <= cutoff)
                    .map(|(index, _)| *index)
                    .collect()
            })
            .unwrap_or_default();
        for (index, row) in case_rows.iter_mut().enumerate() {
            row.unconditional_maximum_q_member = max_q_indices.contains(&index);
            row.unconditional_minimum_action_member = minimum_action_indices.contains(&index);
            row.unconditional_low_action_window_member = low_action_indices.contains(&index);
        }
        let production_visited_count = None;
        let production_retained_count = None;
        aggregates.push(AggregateRow {
            run_id: run_id.clone(),
            case_id: case.case_id.clone(),
            universe_contract: case.universe_contract.clone(),
            row_count: case_rows.len(),
            exact_resolution_count: exact_actions.len(),
            f64_min_action: proposal_min,
            exact_min_action: exact_min,
            f64_runner_up_action: proposal_actions.get(1).map(|x| x.0),
            exact_runner_up_action: exact_actions.get(1).map(|x| x.0.clone()),
            f64_minimizer_sigma: proposal_actions.first().map(|x| x.1.clone()),
            exact_minimizer_count,
            f64_minimizer_count: proposal_minimizer_count,
            f64_low_action_window_count: proposal_low_action_window_count,
            f64_low_action_window_cutoff: proposal_low_cutoff,
            exact_low_action_window_count,
            low_action_window_definition: "f64 count uses A <= 1.05 * finite f64 minimum; exact count uses A <= 1.05 * exact minimum when exact minimum exists".into(),
            candidate_completeness: case.universe_contract.clone(),
            allowed_claim: "declared row/aggregate contract only".into(),
            prohibited_claim:
                "global HK capacity unless full candidate family contract is supplied".into(),
            population_all_count: case_rows.len(),
            population_maximum_q_count: max_q_indices.len(),
            population_minimum_action_count: minimum_action_indices.len(),
            population_production_visited_count: production_visited_count,
            population_production_retained_count: production_retained_count,
            population_exact_resolved_count: exact_actions.len(),
            population_low_action_window_count: low_action_indices.len(),
            population_filter_definitions: vec![
                "all = every declared (case,sigma) unconditional row".into(),
                "maximum_q = rows tied within 1e-12 of maximum unconditional proposal q_raw_f64".into(),
                "minimum_action = rows tied within 1e-12 of minimum proposal_action_f64 (unconditional)".into(),
                "production_visited = unavailable: route API exposes only aggregate totals".into(),
                "production_retained = unavailable: route API exposes only aggregate totals".into(),
                "exact_resolved = rows with exact rational positive witness".into(),
                "low_action_window = proposal_action_f64 <= 1.05 times finite proposal minimum".into(),
            ],
            f64_max_q: max_q,
            f64_max_q_sigma: max_q_indices.first().map(|index| case_rows[*index].sigma.clone()),
            set_preservation_status: "unconditional_rows_preserved; filtered_views_are_annotations".into(),
            proposal_min_action: proposal_min,
            proposal_runner_up_action: proposal_actions.get(1).map(|x| x.0),
            proposal_minimizer_sigma: proposal_actions.first().map(|x| x.1.clone()),
            proposal_minimizer_count,
            proposal_low_action_window_count,
            proposal_low_action_window_cutoff: proposal_low_cutoff,
            proposal_max_q: max_q,
            proposal_max_q_sigma: max_q_indices.first().map(|index| case_rows[*index].sigma.clone()),
            accepted_min_action: accepted_min,
            accepted_runner_up_action: accepted_actions.get(1).map(|x| x.1),
            accepted_minimizer_sigma: accepted_actions.first().map(|(index, _)| case_rows[*index].sigma.clone()),
            accepted_minimizer_count,
            accepted_low_action_window_count,
            accepted_low_action_window_cutoff: accepted_low_cutoff,
            accepted_max_q: case_rows.iter().filter_map(|r| r.accepted_q_raw_f64).max_by(f64::total_cmp),
            accepted_max_q_sigma: case_rows.iter().enumerate().filter_map(|(index, r)| r.accepted_q_raw_f64.map(|q| (index, q))).max_by(|a,b| a.1.total_cmp(&b.1)).map(|(index, _)| case_rows[index].sigma.clone()),
        });
        rows.extend(case_rows);
    }
    let serialization_started = Instant::now();
    write_jsonl(&out.join("raw_rows.jsonl"), &rows)?;
    write_jsonl(&out.join("aggregates.jsonl"), &aggregates)?;
    let serialization_elapsed_seconds = serialization_started.elapsed().as_secs_f64();
    let expected_case_ids: Vec<&str> = aggregates
        .iter()
        .map(|aggregate| aggregate.case_id.as_str())
        .collect();
    let producer_elapsed_seconds = started.elapsed().as_secs_f64();
    let manifest = serde_json::json!({
        "run_id":run_id,
        "source_revision":revision,
        "source_content_id":source_content_id(),
        "producer_version":PRODUCER_VERSION,
        "schema_version":SCHEMA_VERSION,
        "command":"bash experiments/qp-error-bounds/run.sh",
        "rows":rows.len(),
        "aggregates":aggregates.len(),
        "expected_case_ids":expected_case_ids,
        "elapsed_seconds":producer_elapsed_seconds,
        "producer_elapsed_seconds":producer_elapsed_seconds,
        "serialization_elapsed_seconds":serialization_elapsed_seconds,
        "observation_boundary":"one unconditional RawRow for every declared (case,sigma)",
        "target_coordinate_kinds":["original_rational_coordinates","exact_binary64_dyadic_coordinates"],
        "population_contract":"all rows retained; maximum-Q/minimum-action/visited/retained/minimizer/low-window are annotated projections",
        "formula_scope":"production KKT/Q/action/beta/derivative/recovery/volume/sys atoms; offline bounds/categories"
    });
    std::fs::write(
        out.join("manifest.json"),
        serde_json::to_vec_pretty(&manifest)?,
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{parse_rational, predicate_category, rational_text};
    use num_rational::BigRational;

    #[test]
    fn predicate_categories_keep_indeterminate_distinct() {
        assert_eq!(
            predicate_category("indeterminate", "indeterminate"),
            "indeterminate|indeterminate"
        );
        assert_eq!(
            predicate_category("true", "unavailable"),
            "exact_unavailable"
        );
    }

    #[test]
    fn predicate_truth_table_has_six_explicit_cells() {
        let expected = [
            ("true", "true", "true|true_sound"),
            ("true", "false", "true|false_unsound"),
            ("false", "true", "false|true_unsound"),
            ("false", "false", "false|false_sound"),
            ("indeterminate", "true", "indeterminate|true"),
            ("indeterminate", "false", "indeterminate|false"),
        ];
        for (f64_label, exact_label, category) in expected {
            assert_eq!(predicate_category(f64_label, exact_label), category);
        }
    }

    #[test]
    fn rational_encoding_round_trips() {
        let value = BigRational::new(7.into(), 12.into());
        assert_eq!(parse_rational(&rational_text(&value)), Some(value));
    }
}

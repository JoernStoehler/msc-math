//! Retained four-dimensional QP route experiment.
//!
//! The comparison contains scalar-interval, batched, normwise, and staged
//! inverse-defect enclosures plus an empirical control. The verified variants
//! implement Lemmas `lem:kkt-verified-inverse-defect`,
//! `lem:kkt-batched-defect-enclosure`, and
//! `lem:kkt-normwise-defect-enclosure` from
//! `formal/hk2017-qp-precision.tex`.
//! Curvature discovery and cyclic inheritance implement
//! `lem:kkt-certified-curvature-direction` and
//! `lem:kkt-cyclic-obstruction-inheritance` from the same file.
//! The cheap residual/inverse-norm control uses unverified error estimates;
//! exact fallback on its indeterminate cases does not make its determinate
//! decisions sound.
//!
//! nalgebra 0.35 is intentionally an experiment-only second dependency: it
//! supplies the pivoted Bunch--Kaufman LBL^T factorization missing from the
//! repository's current nalgebra 0.33 dependency.

use algebraic_numbers::{solve_linear_system, LinearSystemSolution};
use exp_dev_quadratic_program::{
    capacity_f64_only_with_policy_and_method_profiled, edge_fixture_cases,
    exact_binary64_dual_vertex_arrays, generated_f64_cases,
    selected_route::general::solve_selected_general,
    try_exact_binary64_transition_matrix_assuming_origin_interior, validate_f64_polytope_input,
    F64CapacityMethod, F64CapacityOutcome, F64ValidationPolicy,
};
use nalgebra::{DMatrix, DVector, Vector4};
use nalgebra035::{DMatrix as DMatrix35, DVector as DVector35};
use num_rational::BigRational;
use num_traits::{One, Signed, ToPrimitive, Zero};
use std::collections::{BTreeMap, BTreeSet};
use std::hint::black_box;
use std::time::{Duration, Instant};
use symplectic::algorithms::billiard::{
    facet_classification::classify_facets_from_dual_vertices, for_each_sigma_from_facets,
};
use symplectic::algorithms::capacity_4d::CapacityInput4d;
use symplectic::algorithms::hk2017::SimpleDirectedCyclesCanonical;
use symplectic::geom::known_polytopes;
use symplectic::geom::known_polytopes::hko_pentagon;
use symplectic::geom::rational_arithmetic::f64_to_rational;
use symplectic::kkt::projection_solver::solve_projected;
use symplectic::kkt::qp_assembly::{
    build_augmented_system_from_dual_vertices, build_qp_from_dual_vertices,
};
use symplectic::kkt::rational_solver::solve_kkt_exact;
use symplectic::kkt::saddle_point_solver::{solve_saddle_point, KktOutcome};
use symplectic::kkt::Verdict;
use symplectic::solve_pruned_hk2017_candidates;

const DEFAULT_SEED: u64 = 99_599_604;
const INERTIA_RELATIVE_FLOOR: f64 = 1e-12;

// ── Shared route data ────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DecisionKind {
    Accept,
    Reject,
}

#[derive(Clone, Copy, Debug)]
struct Decision {
    kind: DecisionKind,
    // These legacy experiment field names have different status by guard:
    // certified radii for OutwardCertified and BatchedAnalyticEnvelope,
    // unverified estimates for EmpiricalThenExact, and None for exact
    // decisions. Do not expose them through a shared production result type.
    action: Option<f64>,
    beta_radius: Option<f64>,
    q_radius: Option<f64>,
    q_lower: Option<f64>,
    q_upper: Option<f64>,
    exact_fallback: bool,
}

#[derive(Clone, Copy, Debug)]
enum FactorKind {
    Lu,
    Lblt,
}

#[derive(Clone, Copy, Debug)]
enum GuardKind {
    OutwardCertified,
    BatchedAnalyticEnvelope,
    NormwiseAnalyticEnvelope,
    HybridAnalyticEnvelope,
    EmpiricalThenExact,
}

#[derive(Clone, Debug)]
struct FactorData {
    solution: Vec<f64>,
    inverse: DMatrix<f64>,
    positive_inertia: Option<usize>,
}

#[derive(Clone, Debug)]
struct CurvatureProposal {
    direction: Vec<f64>,
}

#[derive(Clone, Debug)]
struct Obstruction {
    labels: Vec<usize>,
    mask: u16,
}

#[derive(Clone, Debug, Default)]
struct RouteStats {
    words: usize,
    inherited_rejections: usize,
    direct_obstructions: usize,
    obstruction_proposals: usize,
    obstruction_unknown: usize,
    lblt_factorizations: usize,
    lu_factorizations: usize,
    guarded_decisions: usize,
    exact_fallbacks: usize,
    short_exact_solves: usize,
    short_interval_rejections: usize,
    accepted: usize,
    rejected: usize,
    max_beta_radius: f64,
    max_q_radius: f64,
    best_action: Option<f64>,
    best_action_lower: Option<f64>,
    best_action_upper: Option<f64>,
    elapsed: Duration,
    direct_by_length: BTreeMap<usize, usize>,
    inherited_by_length: BTreeMap<usize, usize>,
    fallback_by_length: BTreeMap<usize, usize>,
    lookup_time: Duration,
    factor_time: Duration,
    obstruction_time: Duration,
    guard_time: Duration,
    guard_phases: GuardPhaseStats,
    exact_time: Duration,
    short_exact_time: Duration,
}

#[derive(Clone, Debug, Default)]
struct GuardPhaseStats {
    entries_time: Duration,
    residual_time: Duration,
    defect_time: Duration,
    decision_time: Duration,
}

#[derive(Clone, Debug)]
struct RouteResult {
    cutoff: Option<usize>,
    long_factor: FactorKind,
    stats: RouteStats,
    decisions: Vec<DecisionKind>,
    full_decisions: Vec<Decision>,
}

#[derive(Clone, Copy, Debug)]
struct Interval {
    lo: f64,
    hi: f64,
}

impl Interval {
    fn point(value: f64) -> Self {
        Self {
            lo: value,
            hi: value,
        }
    }

    fn add(self, rhs: Self) -> Self {
        Self {
            lo: next_down(self.lo + rhs.lo),
            hi: next_up(self.hi + rhs.hi),
        }
    }

    fn neg(self) -> Self {
        Self {
            lo: -self.hi,
            hi: -self.lo,
        }
    }

    fn sub(self, rhs: Self) -> Self {
        self.add(rhs.neg())
    }

    fn mul(self, rhs: Self) -> Self {
        let products = [
            self.lo * rhs.lo,
            self.lo * rhs.hi,
            self.hi * rhs.lo,
            self.hi * rhs.hi,
        ];
        let lo = products.iter().copied().fold(f64::INFINITY, f64::min);
        let hi = products.iter().copied().fold(f64::NEG_INFINITY, f64::max);
        Self {
            lo: next_down(lo),
            hi: next_up(hi),
        }
    }

    fn abs_upper(self) -> f64 {
        next_up(self.lo.abs().max(self.hi.abs()))
    }

    fn is_valid_finite(self) -> bool {
        self.lo.is_finite() && self.hi.is_finite() && self.lo <= self.hi
    }
}

// ── Command-line packets and cohorts ─────────────────────────────────────

fn main() {
    let arguments = std::env::args().collect::<Vec<_>>();
    if arguments.len() == 1
        || arguments
            .iter()
            .any(|argument| matches!(argument.as_str(), "--help" | "-h"))
    {
        print_usage();
        return;
    }
    let seed = std::env::args()
        .find_map(|argument| {
            argument
                .strip_prefix("--seed=")
                .and_then(|value| value.parse().ok())
        })
        .unwrap_or(DEFAULT_SEED);
    let cases = generated_f64_cases(1, seed)
        .into_iter()
        .filter(|case| case.family == "generated_random_f64")
        .collect::<Vec<_>>();
    let input_cases = cases
        .iter()
        .map(|case| (case.source_id.clone(), case.dual_vertices.clone()))
        .collect::<Vec<_>>();
    if arguments
        .iter()
        .any(|argument| argument == "--end-to-end-profile")
    {
        run_end_to_end_profile(&input_cases);
        return;
    }
    let long_words_only = std::env::args().any(|argument| argument == "--long-words-only");
    let mut case_words = Vec::new();
    for case in cases {
        let exact = exact_binary64_dual_vertex_arrays(&case.dual_vertices);
        let transition = try_exact_binary64_transition_matrix_assuming_origin_interior(&exact)
            .expect("generated case has exact transition graph");
        let mut words = SimpleDirectedCyclesCanonical::new(&transition).collect::<Vec<_>>();
        if long_words_only {
            words.retain(|word| word.len() >= 5);
        }
        case_words.push((case.source_id, case.dual_vertices, words));
    }

    // Warm both library versions and exact conversion outside reported time.
    for (_, duals, words) in &case_words {
        if let Some(word) = words.iter().find(|word| word.len() >= 5) {
            let (matrix, rhs) = build_augmented_system_from_dual_vertices(duals, word);
            std::hint::black_box(factor_system(&matrix, &rhs, FactorKind::Lu, false));
            std::hint::black_box(factor_system(&matrix, &rhs, FactorKind::Lblt, true));
        }
    }
    if std::env::args().any(|argument| argument == "--predicate-audit-only") {
        run_empirical_predicate_audit(&case_words);
        return;
    }
    if std::env::args().any(|argument| argument == "--batched-envelope-audit") {
        run_batched_envelope_audit(&case_words);
        return;
    }
    if std::env::args().any(|argument| argument == "--normwise-envelope-audit") {
        run_normwise_envelope_audit(&case_words);
        return;
    }
    if std::env::args().any(|argument| argument == "--hybrid-envelope-audit") {
        run_hybrid_envelope_audit(&case_words);
        return;
    }
    if std::env::args().any(|argument| argument == "--adversarial-predicate-search") {
        run_adversarial_predicate_search();
        return;
    }
    if std::env::args().any(|argument| argument == "--beta-boundary-search") {
        run_beta_boundary_search();
        return;
    }
    if std::env::args().any(|argument| argument == "--known-negative-controls") {
        run_known_negative_control_audit("hybrid", GuardKind::HybridAnalyticEnvelope);
        run_known_negative_control_audit("empirical", GuardKind::EmpiricalThenExact);
        return;
    }
    if std::env::args().any(|argument| argument == "--verification-packet") {
        let products = product_case_words();
        run_selected_production_correspondence(&case_words);
        run_exact_route_agreement_audit(
            "general_hybrid",
            &case_words,
            Some(7),
            GuardKind::HybridAnalyticEnvelope,
        );
        run_exact_route_agreement_audit(
            "product_billiard_hybrid_control",
            &products,
            None,
            GuardKind::HybridAnalyticEnvelope,
        );
        run_product_enumeration_agreement_audit(&products);
        run_existing_product_solver_agreement_audit(&products);
        run_known_negative_control_audit("hybrid", GuardKind::HybridAnalyticEnvelope);
        return;
    }
    if std::env::args().any(|argument| argument == "--numerics-packet") {
        let products = product_case_words();
        run_hybrid_envelope_audit(&case_words);
        run_empirical_predicate_audit(&case_words);
        run_product_predicate_audit("hybrid", &products, GuardKind::HybridAnalyticEnvelope);
        run_product_predicate_audit("empirical", &products, GuardKind::EmpiricalThenExact);
        run_known_negative_control_audit("hybrid", GuardKind::HybridAnalyticEnvelope);
        run_known_negative_control_audit("empirical", GuardKind::EmpiricalThenExact);
        return;
    }
    if std::env::args().any(|argument| argument == "--profile-packet") {
        let products = product_case_words();
        run_slowdown_ablation(&case_words);
        run_best_route_benchmark(
            "general",
            "hybrid_envelope",
            &case_words,
            GuardKind::HybridAnalyticEnvelope,
        );
        run_best_route_benchmark(
            "general",
            "empirical_control",
            &case_words,
            GuardKind::EmpiricalThenExact,
        );
        run_best_route_benchmark(
            "product_billiard",
            "hybrid_general_control",
            &products,
            GuardKind::HybridAnalyticEnvelope,
        );
        run_existing_product_route_benchmark(&products);
        return;
    }
    if std::env::args().any(|argument| argument == "--slowdown-ablation") {
        run_slowdown_ablation(&case_words);
        return;
    }
    if std::env::args().any(|argument| argument == "--aggregation-only") {
        run_exact_aggregation_audit(&case_words);
        return;
    }
    if std::env::args().any(|argument| argument == "--rich-output-spike") {
        run_rich_output_spike(&case_words);
        return;
    }

    let variants = [
        (None, FactorKind::Lu),
        (Some(7), FactorKind::Lu),
        (Some(8), FactorKind::Lu),
        // Same obstruction policy as the previous row, but retain LBL^T as a
        // plain solve/inverse factorization above the cutoff. No inertia is
        // inspected and no new obstruction is learned there.
        (Some(8), FactorKind::Lblt),
        (Some(9), FactorKind::Lu),
        (Some(9), FactorKind::Lblt),
        (Some(12), FactorKind::Lblt),
    ];
    if std::env::args().any(|argument| argument == "--normwise-routes") {
        run_interleaved_guard_benchmark(
            &case_words,
            &variants,
            GuardKind::NormwiseAnalyticEnvelope,
        );
        return;
    }
    if std::env::args().any(|argument| argument == "--benchmark-only") {
        run_interleaved_benchmark(&case_words, &variants);
        return;
    }
    let mut results = Vec::new();
    for (cutoff, long_factor) in variants {
        let result = run_route(&case_words, cutoff, long_factor);
        print_route(&result);
        results.push(result);
    }
    compare_routes(&results);
    if std::env::args().any(|argument| argument == "--routes-only") {
        return;
    }
    run_uncertified_baselines(&case_words);
    run_exact_audit(&case_words);
    run_exact_aggregation_audit(&case_words);
    run_inertia_equivalence_audit(&case_words);
    run_scaled_audit(&case_words);
    run_near_singular_audit();
    run_factor_microbenchmark(&case_words);
}

fn run_selected_production_correspondence(cases: &[(String, Vec<Vector4<f64>>, Vec<Vec<usize>>)]) {
    let mut compared = 0usize;
    for (source_id, duals, words) in cases {
        let existing = run_route_with_guard(
            &[(source_id.clone(), duals.clone(), words.clone())],
            Some(usize::MAX),
            FactorKind::Lblt,
            GuardKind::HybridAnalyticEnvelope,
        );
        let existing_bounds = existing
            .stats
            .best_action_lower
            .zip(existing.stats.best_action_upper)
            .expect("existing selected route returns capacity bounds");
        let readable = solve_selected_general(duals, words.clone())
            .expect("readable selected route returns capacity bounds");
        let input = CapacityInput4d::try_from_dual_vertices(duals)
            .expect("retained general case passes production validation");
        let production = input
            .general_capacity()
            .expect("production selected route returns capacity bounds");
        let production_bounds = (production.bounds().lower(), production.bounds().upper());
        assert_eq!(existing_bounds, readable, "{source_id}: existing/readable");
        assert_eq!(
            readable, production_bounds,
            "{source_id}: readable/production"
        );
        compared += 1;
    }
    println!("production_correspondence.cases={compared}");
    println!("production_correspondence.bound_mismatches=0");
}

fn print_usage() {
    println!("Four-dimensional QP route evidence");
    println!("retained packets:");
    println!("  --verification-packet");
    println!("  --numerics-packet");
    println!("  --profile-packet");
    println!("  --end-to-end-profile");
    println!("optional falsification:");
    println!("  --batched-envelope-audit");
    println!("  --normwise-envelope-audit");
    println!("  --hybrid-envelope-audit");
    println!("  --known-negative-controls");
    println!("  --adversarial-predicate-search");
    println!("  --beta-boundary-search");
    println!("legacy ablations:");
    println!("  --normwise-routes");
    println!("  --predicate-audit-only");
    println!("  --slowdown-ablation");
    println!("  --aggregation-only");
    println!("  --rich-output-spike");
    println!("  --routes-only");
    println!("  --benchmark-only [--long-words-only] [--seed=U64]");
}

#[derive(Clone, Copy)]
struct EndToEndSample {
    total: Duration,
    validation: Duration,
    candidate_stream: Duration,
    route: Duration,
}

fn build_validated_case_words(
    inputs: &[(String, Vec<Vector4<f64>>)],
) -> (
    Vec<(String, Vec<Vector4<f64>>, Vec<Vec<usize>>)>,
    Duration,
    Duration,
) {
    let validation_started = Instant::now();
    for (_, duals) in inputs {
        let report = validate_f64_polytope_input(duals);
        assert!(
            report.status.capacity_may_run(),
            "profile input must pass the intended validation boundary"
        );
        std::hint::black_box(report);
    }
    let validation = validation_started.elapsed();

    let candidate_started = Instant::now();
    let case_words = inputs
        .iter()
        .map(|(source_id, duals)| {
            let exact = exact_binary64_dual_vertex_arrays(duals);
            let transition = try_exact_binary64_transition_matrix_assuming_origin_interior(&exact)
                .expect("profile input has an exact transition graph");
            let words = SimpleDirectedCyclesCanonical::new(&transition).collect::<Vec<_>>();
            (source_id.clone(), duals.clone(), words)
        })
        .collect();
    (case_words, validation, candidate_started.elapsed())
}

fn run_end_to_end_profile(inputs: &[(String, Vec<Vector4<f64>>)]) {
    const ROUNDS: usize = 9;
    let run = |guard_kind: Option<GuardKind>| {
        let started = Instant::now();
        let (cases, validation, candidate_stream) = build_validated_case_words(inputs);
        let route_started = Instant::now();
        if let Some(guard_kind) = guard_kind {
            std::hint::black_box(run_route_with_guard(
                &cases,
                Some(usize::MAX),
                FactorKind::Lblt,
                guard_kind,
            ));
        } else {
            std::hint::black_box(run_empirical_inverse_guard(&cases));
        }
        EndToEndSample {
            total: started.elapsed(),
            validation,
            candidate_stream,
            route: route_started.elapsed(),
        }
    };

    std::hint::black_box(run(None));
    std::hint::black_box(run(Some(GuardKind::BatchedAnalyticEnvelope)));
    std::hint::black_box(run(Some(GuardKind::HybridAnalyticEnvelope)));
    let mut empirical = Vec::with_capacity(ROUNDS);
    let mut batched = Vec::with_capacity(ROUNDS);
    let mut hybrid = Vec::with_capacity(ROUNDS);
    for round in 0..ROUNDS {
        for offset in 0..3 {
            match (round + offset) % 3 {
                0 => empirical.push(run(None)),
                1 => batched.push(run(Some(GuardKind::BatchedAnalyticEnvelope))),
                2 => hybrid.push(run(Some(GuardKind::HybridAnalyticEnvelope))),
                _ => unreachable!(),
            }
        }
    }
    print_end_to_end_samples("yesterday_empirical_inverse_guard", &empirical);
    print_end_to_end_samples("certified_batched_route", &batched);
    print_end_to_end_samples("certified_hybrid_route", &hybrid);
}

fn print_end_to_end_samples(label: &str, samples: &[EndToEndSample]) {
    println!("end_to_end.route={label}");
    println!("end_to_end.rounds={}", samples.len());
    println!(
        "end_to_end.total_ms={:.6}",
        median_duration_ms(samples.iter().map(|sample| sample.total))
    );
    println!(
        "end_to_end.validation_ms={:.6}",
        median_duration_ms(samples.iter().map(|sample| sample.validation))
    );
    println!(
        "end_to_end.candidate_stream_ms={:.6}",
        median_duration_ms(samples.iter().map(|sample| sample.candidate_stream))
    );
    println!(
        "end_to_end.route_ms={:.6}",
        median_duration_ms(samples.iter().map(|sample| sample.route))
    );
}

fn product_case_words() -> Vec<(String, Vec<Vector4<f64>>, Vec<Vec<usize>>)> {
    [
        known_polytopes::lagrangian_triangle_product(),
        known_polytopes::lagrangian_triangle_square(),
        known_polytopes::hypercube(),
    ]
    .into_iter()
    .map(|fixture| {
        let duals = fixture.dual_vertices_f64.clone();
        let classification = classify_facets_from_dual_vertices(&duals)
            .expect("known product fixture has exact q/p zero blocks");
        let exact_duals = exact_binary64_dual_vertex_arrays(&duals);
        let transition =
            try_exact_binary64_transition_matrix_assuming_origin_interior(&exact_duals)
                .expect("known product fixture has an exact binary64 transition graph");
        let mut words = Vec::new();
        for_each_sigma_from_facets(
            &classification.q_indices,
            &classification.p_indices,
            &fixture.facet_intersection_is_nonempty,
            &transition,
            |word| words.push(word.to_vec()),
        );
        (fixture.name.to_string(), duals, words)
    })
    .collect()
}

// ── Historical heuristic controls ────────────────────────────────────────

fn run_uncertified_baselines(cases: &[(String, Vec<Vector4<f64>>, Vec<Vec<usize>>)]) {
    let eig = benchmark_baseline(cases, BaselineKind::CurrentEigen);
    let raw = benchmark_baseline(cases, BaselineKind::RawDirect);
    print_baseline("current_symmetric_eigen", &eig);
    print_baseline("raw_direct_unchecked", &raw);
}

#[derive(Clone, Debug, Default)]
struct EmpiricalGuardStats {
    words: usize,
    accepted: usize,
    rejected: usize,
    indeterminate: usize,
    best_action: Option<f64>,
    elapsed: Duration,
    assembly_time: Duration,
    factor_time: Duration,
    guard_time: Duration,
}

/// Reproduction of the previous empirical direct-solve route. Its radius uses
/// ordinary rounded matrix operations, so this is a timing control rather than
/// a correctness certificate.
fn run_empirical_inverse_guard(
    cases: &[(String, Vec<Vector4<f64>>, Vec<Vec<usize>>)],
) -> EmpiricalGuardStats {
    let started = Instant::now();
    let mut stats = EmpiricalGuardStats::default();
    for (_, duals, words) in cases {
        for word in words.iter().filter(|word| word.len() >= 5) {
            stats.words += 1;
            let phase_started = Instant::now();
            let qp = build_qp_from_dual_vertices(duals, word);
            let (matrix, rhs) = build_augmented_system_from_dual_vertices(duals, word);
            stats.assembly_time += phase_started.elapsed();

            let phase_started = Instant::now();
            let inverse = matrix.clone().try_inverse();
            let factor = inverse.and_then(|inverse| {
                let solution = &inverse * &rhs;
                finite_factor_data(solution.as_slice(), inverse, None)
            });
            stats.factor_time += phase_started.elapsed();

            let phase_started = Instant::now();
            let decision = factor.as_ref().and_then(|factor| {
                empirical_inverse_radius_decision(&qp.h, &matrix, &rhs, word, factor)
            });
            stats.guard_time += phase_started.elapsed();
            match decision {
                Some(Decision {
                    kind: DecisionKind::Accept,
                    action: Some(action),
                    ..
                }) => {
                    stats.accepted += 1;
                    stats.best_action = Some(
                        stats
                            .best_action
                            .map_or(action, |current| current.min(action)),
                    );
                }
                Some(Decision {
                    kind: DecisionKind::Reject,
                    ..
                }) => stats.rejected += 1,
                _ => stats.indeterminate += 1,
            }
        }
    }
    stats.elapsed = started.elapsed();
    stats
}

fn empirical_inverse_radius_decision(
    h: &DMatrix<f64>,
    matrix: &DMatrix<f64>,
    rhs: &DVector<f64>,
    word: &[usize],
    factor: &FactorData,
) -> Option<Decision> {
    // This is an empirical classifier, not a certificate. Ordinary rounded
    // residuals and an unchecked approximate inverse norm do not establish a
    // forward-error bound, even when the returned estimate is small.
    let solution = DVector::from_column_slice(&factor.solution);
    let residual = (matrix * &solution - rhs).norm();
    let beta_radius = factor.inverse.norm() * residual;
    let beta = solution.rows(0, word.len()).into_owned();
    let beta_min = beta.iter().copied().fold(f64::INFINITY, f64::min);
    if beta_min < -beta_radius {
        return Some(Decision {
            kind: DecisionKind::Reject,
            action: None,
            beta_radius: Some(beta_radius),
            q_radius: None,
            q_lower: None,
            q_upper: None,
            exact_fallback: false,
        });
    }
    if !(beta_min > beta_radius) {
        return None;
    }
    let h_beta = h * &beta;
    let q = 0.5 * beta.dot(&h_beta);
    let q_radius = h_beta.norm() * beta_radius + 0.5 * h.norm() * beta_radius.powi(2);
    if q - q_radius > 0.0 {
        Some(Decision {
            kind: DecisionKind::Accept,
            action: Some(0.5 / q),
            beta_radius: Some(beta_radius),
            q_radius: Some(q_radius),
            q_lower: Some(q - q_radius),
            q_upper: Some(q + q_radius),
            exact_fallback: false,
        })
    } else if q + q_radius <= 0.0 {
        Some(Decision {
            kind: DecisionKind::Reject,
            action: None,
            beta_radius: Some(beta_radius),
            q_radius: Some(q_radius),
            q_lower: Some(q - q_radius),
            q_upper: Some(q + q_radius),
            exact_fallback: false,
        })
    } else {
        None
    }
}

fn run_slowdown_ablation(cases: &[(String, Vec<Vector4<f64>>, Vec<Vec<usize>>)]) {
    const ROUNDS: usize = 9;
    let long_cases = cases
        .iter()
        .map(|(name, duals, words)| {
            (
                name.clone(),
                duals.clone(),
                words
                    .iter()
                    .filter(|word| word.len() >= 5)
                    .cloned()
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>();

    std::hint::black_box(run_empirical_inverse_guard(&long_cases));
    std::hint::black_box(run_route(&long_cases, None, FactorKind::Lu));
    std::hint::black_box(run_route(&long_cases, Some(12), FactorKind::Lblt));
    std::hint::black_box(run_route_with_guard(
        &long_cases,
        Some(12),
        FactorKind::Lblt,
        GuardKind::BatchedAnalyticEnvelope,
    ));
    std::hint::black_box(run_route_with_guard(
        &long_cases,
        Some(12),
        FactorKind::Lblt,
        GuardKind::NormwiseAnalyticEnvelope,
    ));
    std::hint::black_box(run_route_with_guard(
        &long_cases,
        Some(12),
        FactorKind::Lblt,
        GuardKind::HybridAnalyticEnvelope,
    ));
    std::hint::black_box(run_route_with_guard(
        &long_cases,
        Some(12),
        FactorKind::Lblt,
        GuardKind::EmpiricalThenExact,
    ));

    let mut empirical = Vec::with_capacity(ROUNDS);
    let mut outward_all = Vec::with_capacity(ROUNDS);
    let mut selected = Vec::with_capacity(ROUNDS);
    let mut batched = Vec::with_capacity(ROUNDS);
    let mut normwise = Vec::with_capacity(ROUNDS);
    let mut hybrid = Vec::with_capacity(ROUNDS);
    let mut lazy_exact = Vec::with_capacity(ROUNDS);
    for round in 0..ROUNDS {
        for offset in 0..7 {
            match (round + offset) % 7 {
                0 => empirical.push(run_empirical_inverse_guard(&long_cases)),
                1 => outward_all.push(run_route(&long_cases, None, FactorKind::Lu).stats),
                2 => selected.push(run_route(&long_cases, Some(12), FactorKind::Lblt).stats),
                3 => batched.push(
                    run_route_with_guard(
                        &long_cases,
                        Some(12),
                        FactorKind::Lblt,
                        GuardKind::BatchedAnalyticEnvelope,
                    )
                    .stats,
                ),
                4 => normwise.push(
                    run_route_with_guard(
                        &long_cases,
                        Some(12),
                        FactorKind::Lblt,
                        GuardKind::NormwiseAnalyticEnvelope,
                    )
                    .stats,
                ),
                5 => hybrid.push(
                    run_route_with_guard(
                        &long_cases,
                        Some(12),
                        FactorKind::Lblt,
                        GuardKind::HybridAnalyticEnvelope,
                    )
                    .stats,
                ),
                6 => lazy_exact.push(
                    run_route_with_guard(
                        &long_cases,
                        Some(12),
                        FactorKind::Lblt,
                        GuardKind::EmpiricalThenExact,
                    )
                    .stats,
                ),
                _ => unreachable!(),
            }
        }
    }

    print_empirical_benchmark(&empirical);
    print_slowdown_route_benchmark("outward_guard_every_word_lu", &outward_all);
    print_slowdown_route_benchmark("all_length_lblt_outward_guard", &selected);
    print_slowdown_route_benchmark("all_length_lblt_batched_envelope", &batched);
    print_slowdown_route_benchmark("all_length_lblt_normwise_envelope", &normwise);
    print_slowdown_route_benchmark("all_length_lblt_hybrid_envelope", &hybrid);
    print_slowdown_route_benchmark("all_length_lblt_empirical_then_exact", &lazy_exact);
    let empirical_ms = median_duration_ms(empirical.iter().map(|stats| stats.elapsed));
    let outward_ms = median_duration_ms(outward_all.iter().map(|stats| stats.elapsed));
    let selected_ms = median_duration_ms(selected.iter().map(|stats| stats.elapsed));
    let batched_ms = median_duration_ms(batched.iter().map(|stats| stats.elapsed));
    let normwise_ms = median_duration_ms(normwise.iter().map(|stats| stats.elapsed));
    let hybrid_ms = median_duration_ms(hybrid.iter().map(|stats| stats.elapsed));
    let lazy_exact_ms = median_duration_ms(lazy_exact.iter().map(|stats| stats.elapsed));
    println!(
        "slowdown.outward_all_over_empirical={:.6}",
        outward_ms / empirical_ms
    );
    println!(
        "slowdown.selected_over_empirical={:.6}",
        selected_ms / empirical_ms
    );
    println!(
        "slowdown.pruning_speedup_with_outward_guard={:.6}",
        outward_ms / selected_ms
    );
    println!(
        "slowdown.batched_speedup_vs_scalar_interval={:.6}",
        selected_ms / batched_ms
    );
    println!(
        "slowdown.normwise_speedup_vs_batched={:.6}",
        batched_ms / normwise_ms
    );
    println!(
        "slowdown.normwise_speedup_vs_yesterday={:.6}",
        empirical_ms / normwise_ms
    );
    println!(
        "slowdown.hybrid_speedup_vs_yesterday={:.6}",
        empirical_ms / hybrid_ms
    );
    println!(
        "slowdown.lazy_exact_speedup_vs_yesterday={:.6}",
        empirical_ms / lazy_exact_ms
    );
    println!(
        "slowdown.lazy_exact_speedup_vs_outward={:.6}",
        selected_ms / lazy_exact_ms
    );
}

// ── Exact-oracle numerical audits ────────────────────────────────────────

#[derive(Clone, Debug, Default)]
struct PredicateAudit {
    compared: usize,
    true_false: usize,
    false_true: usize,
    indeterminate: usize,
    beta_radius_compared: usize,
    beta_radius_violations: usize,
    q_radius_compared: usize,
    q_radius_violations: usize,
    max_beta_error_over_radius: f64,
    max_q_error_over_radius: f64,
    max_action_error: f64,
    exact_fallback_time: Duration,
}

impl PredicateAudit {
    fn observe(&mut self, decision: Option<Decision>, exact: Decision) {
        self.compared += 1;
        match decision {
            None => self.indeterminate += 1,
            Some(decision) => {
                self.true_false += usize::from(
                    decision.kind == DecisionKind::Accept && exact.kind == DecisionKind::Reject,
                );
                self.false_true += usize::from(
                    decision.kind == DecisionKind::Reject && exact.kind == DecisionKind::Accept,
                );
                if let (Some(action), Some(exact_action)) = (decision.action, exact.action) {
                    self.max_action_error =
                        self.max_action_error.max((action - exact_action).abs());
                }
            }
        }
    }

    fn add(&mut self, other: &Self) {
        self.compared += other.compared;
        self.true_false += other.true_false;
        self.false_true += other.false_true;
        self.indeterminate += other.indeterminate;
        self.beta_radius_compared += other.beta_radius_compared;
        self.beta_radius_violations += other.beta_radius_violations;
        self.q_radius_compared += other.q_radius_compared;
        self.q_radius_violations += other.q_radius_violations;
        self.max_beta_error_over_radius = self
            .max_beta_error_over_radius
            .max(other.max_beta_error_over_radius);
        self.max_q_error_over_radius = self
            .max_q_error_over_radius
            .max(other.max_q_error_over_radius);
        self.max_action_error = self.max_action_error.max(other.max_action_error);
        self.exact_fallback_time += other.exact_fallback_time;
    }
}

fn audit_empirical_case(
    audit: &mut PredicateAudit,
    duals: &[Vector4<f64>],
    exact_duals: &[[BigRational; 4]],
    word: &[usize],
) {
    audit_guard_case(
        audit,
        duals,
        exact_duals,
        word,
        GuardKind::EmpiricalThenExact,
    );
}

fn audit_guard_case(
    audit: &mut PredicateAudit,
    duals: &[Vector4<f64>],
    exact_duals: &[[BigRational; 4]],
    word: &[usize],
    guard_kind: GuardKind,
) {
    let (matrix, rhs) = build_augmented_system_from_dual_vertices(duals, word);
    let factor = factor_system(&matrix, &rhs, FactorKind::Lblt, false);
    let h = matrix.view((0, 0), (word.len(), word.len())).into_owned();
    let decision = factor.as_ref().and_then(|factor| match guard_kind {
        GuardKind::OutwardCertified => certify_direct_solution(duals, word, &matrix, factor),
        GuardKind::BatchedAnalyticEnvelope => {
            certify_direct_solution_batched(duals, word, &matrix, factor)
        }
        GuardKind::NormwiseAnalyticEnvelope => {
            certify_direct_solution_normwise(duals, word, &matrix, factor)
        }
        GuardKind::HybridAnalyticEnvelope => {
            certify_direct_solution_hybrid(duals, word, &matrix, factor)
        }
        GuardKind::EmpiricalThenExact => {
            empirical_inverse_radius_decision(&h, &matrix, &rhs, word, factor)
        }
    });
    let started = Instant::now();
    let exact_result = solve_kkt_exact(exact_duals, word);
    let exact = match exact_result.as_ref() {
        Some(result) if result.q_exact.is_positive() => {
            exact_positive_decision_from_q(&result.q_exact, true)
        }
        _ => rejected_exact_decision(),
    };
    if decision.is_none() {
        audit.exact_fallback_time += started.elapsed();
    }
    if let (Some(decision), Some(factor), Some(exact_result)) =
        (decision, factor.as_ref(), exact_result.as_ref())
    {
        if let Some(beta_radius) = decision.beta_radius {
            audit.beta_radius_compared += 1;
            let beta_error = exact_result
                .beta
                .iter()
                .zip(&factor.solution[..word.len()])
                .map(|(exact, &approximate)| (exact - f64_to_rational(approximate)).abs())
                .max()
                .unwrap_or_else(BigRational::zero);
            let radius = f64_to_rational(beta_radius);
            audit.beta_radius_violations += usize::from(beta_error > radius);
            let beta_error_f64 = beta_error.to_f64().unwrap_or(f64::INFINITY);
            let ratio = if beta_radius == 0.0 {
                if beta_error.is_zero() {
                    0.0
                } else {
                    f64::INFINITY
                }
            } else {
                beta_error_f64 / beta_radius
            };
            audit.max_beta_error_over_radius = audit.max_beta_error_over_radius.max(ratio);
        }
        if let Some(q_radius) = decision.q_radius {
            audit.q_radius_compared += 1;
            let q = match guard_kind {
                GuardKind::OutwardCertified
                | GuardKind::BatchedAnalyticEnvelope
                | GuardKind::NormwiseAnalyticEnvelope
                | GuardKind::HybridAnalyticEnvelope => -0.5 * factor.solution[word.len() + 4],
                GuardKind::EmpiricalThenExact => {
                    let solution = DVector::from_column_slice(&factor.solution);
                    let beta = solution.rows(0, word.len()).into_owned();
                    0.5 * beta.dot(&(&h * &beta))
                }
            };
            let q_error = (&exact_result.q_exact - f64_to_rational(q)).abs();
            audit.q_radius_violations += usize::from(q_error > f64_to_rational(q_radius));
            let ratio = if q_radius == 0.0 {
                if q_error.is_zero() {
                    0.0
                } else {
                    f64::INFINITY
                }
            } else {
                q_error.to_f64().unwrap_or(f64::INFINITY) / q_radius
            };
            audit.max_q_error_over_radius = audit.max_q_error_over_radius.max(ratio);
        }
    }
    audit.observe(decision, exact);
}

fn empirical_predicate_decision(duals: &[Vector4<f64>], word: &[usize]) -> Option<Decision> {
    let (matrix, rhs) = build_augmented_system_from_dual_vertices(duals, word);
    let factor = factor_system(&matrix, &rhs, FactorKind::Lblt, false)?;
    let h = matrix.view((0, 0), (word.len(), word.len())).into_owned();
    empirical_inverse_radius_decision(&h, &matrix, &rhs, word, &factor)
}

fn run_empirical_predicate_audit(cases: &[(String, Vec<Vector4<f64>>, Vec<Vec<usize>>)]) {
    let mut total = PredicateAudit::default();
    let mut generic = PredicateAudit::default();
    for (_, duals, words) in cases {
        let exact_duals = exact_binary64_dual_vertex_arrays(duals);
        for (index, word) in words.iter().enumerate() {
            if word.len() < 5 || !(duals.len() <= 7 || index % 257 == 0) {
                continue;
            }
            audit_empirical_case(&mut generic, duals, &exact_duals, word);
        }
    }
    print_predicate_audit("generic", &generic);
    total.add(&generic);

    for scale in [1e-2, 1.0, 1e2, 1e3] {
        let mut scaled_audit = PredicateAudit::default();
        for (_, duals, words) in cases
            .iter()
            .filter(|(_, duals, _)| matches!(duals.len(), 6 | 8 | 10 | 12))
        {
            let scaled = duals.iter().map(|dual| dual * scale).collect::<Vec<_>>();
            let exact_duals = exact_binary64_dual_vertex_arrays(&scaled);
            for (index, word) in words.iter().enumerate() {
                if word.len() < 5 || index % 401 != 0 {
                    continue;
                }
                audit_empirical_case(&mut scaled_audit, &scaled, &exact_duals, word);
            }
        }
        print_predicate_audit(&format!("scale_{scale:.0e}"), &scaled_audit);
        total.add(&scaled_audit);
    }

    let base = hko_pentagon().dual_vertices_f64.clone();
    let words = [
        vec![0, 1, 6, 7, 3, 4, 5, 9],
        vec![1, 8, 7, 3, 4, 5, 9],
        vec![0, 1, 7, 3, 9, 5],
        vec![1, 7, 2, 8, 4, 6, 5],
    ];
    let mut near_singular = PredicateAudit::default();
    for epsilon in [
        1e-3, 1e-6, 1e-9, 1e-10, 1e-11, 1e-12, 1e-13, 1e-14, 1e-15, 1e-16, 0.0,
    ] {
        let duals = perturb(&base, epsilon);
        let exact_duals = exact_binary64_dual_vertex_arrays(&duals);
        let mut epsilon_audit = PredicateAudit::default();
        for word in &words {
            audit_empirical_case(&mut epsilon_audit, &duals, &exact_duals, word);
        }
        print_predicate_audit(&format!("near_singular_{epsilon:.0e}"), &epsilon_audit);
        near_singular.add(&epsilon_audit);
    }
    print_predicate_audit("near_singular", &near_singular);
    total.add(&near_singular);
    print_predicate_audit("total", &total);
    assert_predicate_audit_has_no_wrong_decisions_or_radii("general_total", &total);
}

fn run_batched_envelope_audit(cases: &[(String, Vec<Vector4<f64>>, Vec<Vec<usize>>)]) {
    run_certified_envelope_audit(cases, GuardKind::BatchedAnalyticEnvelope, "batched");
}

fn run_normwise_envelope_audit(cases: &[(String, Vec<Vector4<f64>>, Vec<Vec<usize>>)]) {
    run_certified_envelope_audit(cases, GuardKind::NormwiseAnalyticEnvelope, "normwise");
}

fn run_hybrid_envelope_audit(cases: &[(String, Vec<Vector4<f64>>, Vec<Vec<usize>>)]) {
    run_certified_envelope_audit(cases, GuardKind::HybridAnalyticEnvelope, "hybrid");
}

fn run_certified_envelope_audit(
    cases: &[(String, Vec<Vector4<f64>>, Vec<Vec<usize>>)],
    guard_kind: GuardKind,
    label: &str,
) {
    let mut total = PredicateAudit::default();
    let mut generic = PredicateAudit::default();
    for (_, duals, words) in cases {
        let exact_duals = exact_binary64_dual_vertex_arrays(duals);
        for (index, word) in words.iter().enumerate() {
            if word.len() < 5 || !(duals.len() <= 7 || index % 257 == 0) {
                continue;
            }
            audit_guard_case(&mut generic, duals, &exact_duals, word, guard_kind);
        }
    }
    print_predicate_audit(&format!("{label}_generic"), &generic);
    total.add(&generic);

    for scale in [1e-2, 1.0, 1e2, 1e3] {
        let mut scaled_audit = PredicateAudit::default();
        for (_, duals, words) in cases
            .iter()
            .filter(|(_, duals, _)| matches!(duals.len(), 6 | 8 | 10 | 12))
        {
            let scaled = duals.iter().map(|dual| dual * scale).collect::<Vec<_>>();
            let exact_duals = exact_binary64_dual_vertex_arrays(&scaled);
            for (index, word) in words.iter().enumerate() {
                if word.len() < 5 || index % 401 != 0 {
                    continue;
                }
                audit_guard_case(&mut scaled_audit, &scaled, &exact_duals, word, guard_kind);
            }
        }
        print_predicate_audit(&format!("{label}_scale_{scale:.0e}"), &scaled_audit);
        total.add(&scaled_audit);
    }

    let base = hko_pentagon().dual_vertices_f64.clone();
    let words = [
        vec![0, 1, 6, 7, 3, 4, 5, 9],
        vec![1, 8, 7, 3, 4, 5, 9],
        vec![0, 1, 7, 3, 9, 5],
        vec![1, 7, 2, 8, 4, 6, 5],
    ];
    let mut near_singular = PredicateAudit::default();
    for epsilon in [
        1e-3, 1e-6, 1e-9, 1e-10, 1e-11, 1e-12, 1e-13, 1e-14, 1e-15, 1e-16, 0.0,
    ] {
        let duals = perturb(&base, epsilon);
        let exact_duals = exact_binary64_dual_vertex_arrays(&duals);
        for word in &words {
            audit_guard_case(&mut near_singular, &duals, &exact_duals, word, guard_kind);
        }
    }
    print_predicate_audit(&format!("{label}_near_singular"), &near_singular);
    total.add(&near_singular);
    print_predicate_audit(&format!("{label}_total"), &total);
    assert_predicate_audit_has_no_wrong_decisions_or_radii(&format!("{label}_total"), &total);

    let scalar = run_route_with_guard(
        cases,
        Some(12),
        FactorKind::Lblt,
        GuardKind::OutwardCertified,
    );
    let candidate = run_route_with_guard(cases, Some(12), FactorKind::Lblt, guard_kind);
    let decision_mismatches = scalar
        .decisions
        .iter()
        .zip(&candidate.decisions)
        .filter(|(left, right)| left != right)
        .count();
    println!("{label}_control.words={}", scalar.decisions.len());
    println!("{label}_control.decision_mismatches={decision_mismatches}");
    println!(
        "{label}_control.scalar_exact_fallbacks={}",
        scalar.stats.exact_fallbacks
    );
    println!(
        "{label}_control.candidate_exact_fallbacks={}",
        candidate.stats.exact_fallbacks
    );
    assert_eq!(
        decision_mismatches, 0,
        "{label} and scalar-interval routes disagree"
    );
}

fn run_known_negative_control_audit(label: &str, guard_kind: GuardKind) {
    let mut total = PredicateAudit::default();
    let mut accepted_cases = 0usize;
    let mut rejected_cases = 0usize;
    let mut candidate_sigmas = 0usize;
    for case in edge_fixture_cases() {
        let validation = validate_f64_polytope_input(&case.dual_vertices);
        if !matches!(
            validation.status.label(),
            "accepted_decisive" | "accepted_ambiguous"
        ) {
            rejected_cases += 1;
            println!("negative_control.case={}", case.source_id);
            println!("negative_control.validation={}", validation.status.label());
            continue;
        }
        let exact_duals = exact_binary64_dual_vertex_arrays(&case.dual_vertices);
        let Ok(transition) =
            try_exact_binary64_transition_matrix_assuming_origin_interior(&exact_duals)
        else {
            rejected_cases += 1;
            println!("negative_control.case={}", case.source_id);
            println!("negative_control.validation=exact_geometry_failed");
            continue;
        };
        let Ok((candidates, _)) = solve_pruned_hk2017_candidates(&case.dual_vertices, &transition)
        else {
            rejected_cases += 1;
            println!("negative_control.case={}", case.source_id);
            println!("negative_control.validation=f64_candidate_solve_failed");
            continue;
        };
        accepted_cases += 1;
        let mut audit = PredicateAudit::default();
        for candidate in candidates {
            candidate_sigmas += 1;
            audit_guard_case(
                &mut audit,
                &case.dual_vertices,
                &exact_duals,
                &candidate.sigma,
                guard_kind,
            );
        }
        print_predicate_audit(&format!("{label}_{}", case.source_id), &audit);
        total.add(&audit);
    }
    println!("negative_control.accepted_cases={accepted_cases}");
    println!("negative_control.rejected_cases={rejected_cases}");
    println!("negative_control.candidate_sigmas={candidate_sigmas}");
    print_predicate_audit(&format!("{label}_known_negative_controls_total"), &total);
    assert_predicate_audit_has_no_wrong_decisions_or_radii(
        &format!("{label}_known_negative_controls_total"),
        &total,
    );
}

fn run_product_predicate_audit(
    label: &str,
    cases: &[(String, Vec<Vector4<f64>>, Vec<Vec<usize>>)],
    guard_kind: GuardKind,
) {
    let mut total = PredicateAudit::default();
    for (source_id, duals, words) in cases {
        let exact_duals = exact_binary64_dual_vertex_arrays(duals);
        let mut audit = PredicateAudit::default();
        for (index, word) in words.iter().enumerate() {
            if duals.len() >= 8 && index % 17 != 0 {
                continue;
            }
            audit_guard_case(&mut audit, duals, &exact_duals, word, guard_kind);
        }
        print_predicate_audit(&format!("{label}_product_{source_id}"), &audit);
        total.add(&audit);
    }
    print_predicate_audit(&format!("{label}_product_total"), &total);
    assert_predicate_audit_has_no_wrong_decisions_or_radii(
        &format!("{label}_product_total"),
        &total,
    );
}

fn assert_predicate_audit_has_no_wrong_decisions_or_radii(label: &str, audit: &PredicateAudit) {
    assert!(
        audit.compared > 0,
        "{label}: empty predicate cohort cannot support the audit"
    );
    assert_eq!(audit.true_false, 0, "{label}: wrong accept decisions");
    assert_eq!(audit.false_true, 0, "{label}: wrong reject decisions");
    assert_eq!(
        audit.beta_radius_violations, 0,
        "{label}: beta radius failed to cover exact error"
    );
    assert_eq!(
        audit.q_radius_violations, 0,
        "{label}: Q radius failed to cover exact error"
    );
}

fn print_predicate_audit(label: &str, audit: &PredicateAudit) {
    println!("predicate_audit.group={label}");
    println!("predicate_audit.compared={}", audit.compared);
    println!("predicate_audit.true_false={}", audit.true_false);
    println!("predicate_audit.false_true={}", audit.false_true);
    println!("predicate_audit.indeterminate={}", audit.indeterminate);
    println!(
        "predicate_audit.beta_radius_compared={}",
        audit.beta_radius_compared
    );
    println!(
        "predicate_audit.beta_radius_violations={}",
        audit.beta_radius_violations
    );
    println!(
        "predicate_audit.max_beta_error_over_radius={:.6e}",
        audit.max_beta_error_over_radius
    );
    println!(
        "predicate_audit.q_radius_compared={}",
        audit.q_radius_compared
    );
    println!(
        "predicate_audit.q_radius_violations={}",
        audit.q_radius_violations
    );
    println!(
        "predicate_audit.max_q_error_over_radius={:.6e}",
        audit.max_q_error_over_radius
    );
    println!(
        "predicate_audit.max_action_error={:.6e}",
        audit.max_action_error
    );
    println!(
        "predicate_audit.exact_fallback_ms={:.6}",
        audit.exact_fallback_time.as_secs_f64() * 1e3
    );
}

// ── Benchmarks and ablations ─────────────────────────────────────────────

fn print_empirical_benchmark(samples: &[EmpiricalGuardStats]) {
    let representative = &samples[0];
    println!("slowdown.route=yesterday_empirical_inverse_guard");
    println!("slowdown.rounds={}", samples.len());
    println!("slowdown.words={}", representative.words);
    println!("slowdown.accepted={}", representative.accepted);
    println!("slowdown.rejected={}", representative.rejected);
    println!("slowdown.indeterminate={}", representative.indeterminate);
    println!("slowdown.best_action={:?}", representative.best_action);
    println!(
        "slowdown.median_ms={:.6}",
        median_duration_ms(samples.iter().map(|stats| stats.elapsed))
    );
    println!(
        "slowdown.assembly_ms={:.6}",
        median_duration_ms(samples.iter().map(|stats| stats.assembly_time))
    );
    println!(
        "slowdown.factor_ms={:.6}",
        median_duration_ms(samples.iter().map(|stats| stats.factor_time))
    );
    println!(
        "slowdown.guard_ms={:.6}",
        median_duration_ms(samples.iter().map(|stats| stats.guard_time))
    );
}

fn print_slowdown_route_benchmark(label: &str, samples: &[RouteStats]) {
    let representative = &samples[0];
    println!("slowdown.route={label}");
    println!("slowdown.rounds={}", samples.len());
    println!("slowdown.words={}", representative.words);
    println!("slowdown.accepted={}", representative.accepted);
    println!("slowdown.rejected={}", representative.rejected);
    println!(
        "slowdown.exact_fallbacks={}",
        representative.exact_fallbacks
    );
    println!("slowdown.best_action={:?}", representative.best_action);
    println!(
        "slowdown.best_action_lower={:?}",
        representative.best_action_lower
    );
    println!(
        "slowdown.best_action_upper={:?}",
        representative.best_action_upper
    );
    println!(
        "slowdown.median_ms={:.6}",
        median_duration_ms(samples.iter().map(|stats| stats.elapsed))
    );
    println!(
        "slowdown.lookup_ms={:.6}",
        median_duration_ms(samples.iter().map(|stats| stats.lookup_time))
    );
    println!(
        "slowdown.factor_ms={:.6}",
        median_duration_ms(samples.iter().map(|stats| stats.factor_time))
    );
    println!(
        "slowdown.obstruction_ms={:.6}",
        median_duration_ms(samples.iter().map(|stats| stats.obstruction_time))
    );
    println!(
        "slowdown.guard_ms={:.6}",
        median_duration_ms(samples.iter().map(|stats| stats.guard_time))
    );
    println!(
        "slowdown.guard_entries_ms={:.6}",
        median_duration_ms(samples.iter().map(|stats| stats.guard_phases.entries_time))
    );
    println!(
        "slowdown.guard_residual_ms={:.6}",
        median_duration_ms(samples.iter().map(|stats| stats.guard_phases.residual_time))
    );
    println!(
        "slowdown.guard_defect_ms={:.6}",
        median_duration_ms(samples.iter().map(|stats| stats.guard_phases.defect_time))
    );
    println!(
        "slowdown.guard_decision_ms={:.6}",
        median_duration_ms(samples.iter().map(|stats| stats.guard_phases.decision_time))
    );
    let overhead = samples.iter().map(|stats| {
        let accounted = stats.lookup_time
            + stats.factor_time
            + stats.obstruction_time
            + stats.guard_time
            + stats.exact_time
            + stats.short_exact_time;
        stats.elapsed.saturating_sub(accounted)
    });
    println!(
        "slowdown.unaccounted_ms={:.6}",
        median_duration_ms(overhead)
    );
}

fn median_duration_ms(values: impl IntoIterator<Item = Duration>) -> f64 {
    let mut values = values.into_iter().collect::<Vec<_>>();
    values.sort_unstable();
    values[values.len() / 2].as_secs_f64() * 1e3
}

#[derive(Clone, Copy)]
enum BaselineKind {
    CurrentEigen,
    RawDirect,
}

#[derive(Clone, Copy, Debug, Default)]
struct BaselineResult {
    words: usize,
    accepted: usize,
    rejected: usize,
    indeterminate: usize,
    best_action: Option<f64>,
    elapsed: Duration,
}

fn benchmark_baseline(
    cases: &[(String, Vec<Vector4<f64>>, Vec<Vec<usize>>)],
    kind: BaselineKind,
) -> BaselineResult {
    std::hint::black_box(run_baseline_once(cases, kind));
    let mut samples = (0..7)
        .map(|_| run_baseline_once(cases, kind))
        .collect::<Vec<_>>();
    samples.sort_by_key(|sample| sample.elapsed);
    samples[samples.len() / 2]
}

fn run_baseline_once(
    cases: &[(String, Vec<Vector4<f64>>, Vec<Vec<usize>>)],
    kind: BaselineKind,
) -> BaselineResult {
    let started = Instant::now();
    let mut result = BaselineResult::default();
    for (_, duals, words) in cases {
        for word in words {
            result.words += 1;
            let outcome = match kind {
                BaselineKind::CurrentEigen => {
                    let (matrix, rhs) = build_augmented_system_from_dual_vertices(duals, word);
                    match solve_saddle_point(&matrix, &rhs) {
                        KktOutcome::Feasible(value) => {
                            classify_unchecked(&value.beta, value.q_corrected)
                        }
                        KktOutcome::Infeasible => None,
                        KktOutcome::SingularMatrix
                        | KktOutcome::TypeCViolation
                        | KktOutcome::ConstraintViolation => Some(Err(())),
                    }
                }
                BaselineKind::RawDirect if word.len() < 5 => {
                    let qp = build_qp_from_dual_vertices(duals, word);
                    let solution = solve_projected(&qp);
                    match solution.verdict {
                        Verdict::True => classify_unchecked(&solution.beta, solution.q),
                        Verdict::False => None,
                        Verdict::Indeterminate => Some(Err(())),
                    }
                }
                BaselineKind::RawDirect => {
                    let qp = build_qp_from_dual_vertices(duals, word);
                    let (matrix, rhs) = build_augmented_system_from_dual_vertices(duals, word);
                    matrix
                        .clone()
                        .lu()
                        .solve(&rhs)
                        .map_or(Some(Err(())), |solution| {
                            let beta = solution.rows(0, word.len());
                            let q = 0.5 * beta.dot(&(&qp.h * beta));
                            classify_unchecked(beta.as_slice(), q)
                        })
                }
            };
            match outcome {
                Some(Ok(action)) => {
                    result.accepted += 1;
                    result.best_action = Some(
                        result
                            .best_action
                            .map_or(action, |current| current.min(action)),
                    );
                }
                None => result.rejected += 1,
                Some(Err(())) => result.indeterminate += 1,
            }
        }
    }
    result.elapsed = started.elapsed();
    result
}

fn classify_unchecked(beta: &[f64], q: f64) -> Option<Result<f64, ()>> {
    let margin = beta.iter().copied().fold(f64::INFINITY, f64::min);
    if margin > 1e-9 && q > 1e-15 {
        Some(Ok(0.5 / q))
    } else if margin < -1e-9 || q <= 1e-15 {
        None
    } else {
        Some(Err(()))
    }
}

fn print_baseline(label: &str, result: &BaselineResult) {
    println!("baseline={label}");
    println!("baseline.words={}", result.words);
    println!(
        "baseline.elapsed_ms={:.6}",
        result.elapsed.as_secs_f64() * 1e3
    );
    println!("baseline.accepted={}", result.accepted);
    println!("baseline.rejected={}", result.rejected);
    println!("baseline.indeterminate={}", result.indeterminate);
    println!("baseline.best_action={:?}", result.best_action);
}

fn run_interleaved_benchmark(
    cases: &[(String, Vec<Vector4<f64>>, Vec<Vec<usize>>)],
    variants: &[(Option<usize>, FactorKind)],
) {
    run_interleaved_guard_benchmark(cases, variants, GuardKind::OutwardCertified);
}

fn run_interleaved_guard_benchmark(
    cases: &[(String, Vec<Vector4<f64>>, Vec<Vec<usize>>)],
    variants: &[(Option<usize>, FactorKind)],
    guard_kind: GuardKind,
) {
    const ROUNDS: usize = 9;
    for &(cutoff, factor) in variants {
        std::hint::black_box(run_route_with_guard(cases, cutoff, factor, guard_kind));
    }
    let mut samples = vec![Vec::<f64>::with_capacity(ROUNDS); variants.len()];
    for round in 0..ROUNDS {
        for offset in 0..variants.len() {
            let index = (round + offset) % variants.len();
            let (cutoff, factor) = variants[index];
            let result = run_route_with_guard(cases, cutoff, factor, guard_kind);
            samples[index].push(result.stats.elapsed.as_secs_f64() * 1e3);
        }
    }
    for ((cutoff, factor), values) in variants.iter().copied().zip(&mut samples) {
        values.sort_by(f64::total_cmp);
        let median = values[values.len() / 2];
        println!("benchmark.route={}", route_label(cutoff, factor));
        println!("benchmark.rounds={ROUNDS}");
        println!("benchmark.median_ms={median:.6}");
        println!("benchmark.min_ms={:.6}", values[0]);
        println!("benchmark.max_ms={:.6}", values[values.len() - 1]);
        println!("benchmark.samples_ms={values:?}");
    }
}

fn run_best_route_benchmark(
    cohort: &str,
    guard_label: &str,
    cases: &[(String, Vec<Vector4<f64>>, Vec<Vec<usize>>)],
    guard_kind: GuardKind,
) {
    const ROUNDS: usize = 9;
    std::hint::black_box(run_route_with_guard(
        cases,
        Some(usize::MAX),
        FactorKind::Lblt,
        guard_kind,
    ));
    let mut samples = (0..ROUNDS)
        .map(|_| run_route_with_guard(cases, Some(usize::MAX), FactorKind::Lblt, guard_kind))
        .collect::<Vec<_>>();
    samples.sort_by_key(|sample| sample.stats.elapsed);
    let median = &samples[ROUNDS / 2];
    println!("best_profile.cohort={cohort}");
    println!("best_profile.guard={guard_label}");
    println!("best_profile.rounds={ROUNDS}");
    println!("best_profile.cases={}", cases.len());
    println!("best_profile.words={}", median.stats.words);
    println!(
        "best_profile.median_ms={:.6}",
        median.stats.elapsed.as_secs_f64() * 1e3
    );
    println!(
        "best_profile.lookup_ms={:.6}",
        median.stats.lookup_time.as_secs_f64() * 1e3
    );
    println!(
        "best_profile.factor_ms={:.6}",
        median.stats.factor_time.as_secs_f64() * 1e3
    );
    println!(
        "best_profile.obstruction_ms={:.6}",
        median.stats.obstruction_time.as_secs_f64() * 1e3
    );
    println!(
        "best_profile.guard_ms={:.6}",
        median.stats.guard_time.as_secs_f64() * 1e3
    );
    println!(
        "best_profile.exact_ms={:.6}",
        median.stats.exact_time.as_secs_f64() * 1e3
    );
    println!(
        "best_profile.exact_fallbacks={}",
        median.stats.exact_fallbacks
    );
    println!("best_profile.accepted={}", median.stats.accepted);
    println!("best_profile.rejected={}", median.stats.rejected);
    println!("best_profile.best_action={:?}", median.stats.best_action);
    println!(
        "best_profile.best_action_lower={:?}",
        median.stats.best_action_lower
    );
    println!(
        "best_profile.best_action_upper={:?}",
        median.stats.best_action_upper
    );
}

fn run_existing_product_route_benchmark(cases: &[(String, Vec<Vector4<f64>>, Vec<Vec<usize>>)]) {
    const ROUNDS: usize = 9;
    for (source_id, duals, _) in cases {
        std::hint::black_box(capacity_f64_only_with_policy_and_method_profiled(
            duals,
            F64ValidationPolicy::LpOriginVertex,
            F64CapacityMethod::ProductBilliardOrHk,
        ));
        let mut samples = (0..ROUNDS)
            .map(|_| {
                let started = Instant::now();
                let (report, timing) = capacity_f64_only_with_policy_and_method_profiled(
                    duals,
                    F64ValidationPolicy::LpOriginVertex,
                    F64CapacityMethod::ProductBilliardOrHk,
                );
                (started.elapsed(), report, timing)
            })
            .collect::<Vec<_>>();
        samples.sort_by_key(|sample| sample.0);
        let (elapsed, report, timing) = &samples[ROUNDS / 2];
        let capacity = match &report.outcome {
            F64CapacityOutcome::Success { capacity, .. } => Some(*capacity),
            F64CapacityOutcome::Failure { .. } => None,
        };
        println!("existing_product_profile.source_id={source_id}");
        println!("existing_product_profile.rounds={ROUNDS}");
        println!(
            "existing_product_profile.median_ms={:.6}",
            elapsed.as_secs_f64() * 1e3
        );
        println!("existing_product_profile.words={}", report.sigma_count);
        println!(
            "existing_product_profile.kkt_ms={:.6}",
            timing.candidate_kkt_solve_ms
        );
        println!(
            "existing_product_profile.indeterminate={}",
            report.indeterminate_f64_count
        );
        println!("existing_product_profile.capacity={capacity:?}");
    }
}

// ── Complete general route ───────────────────────────────────────────────

/// `None` is guarded LU on every word. `Some(k)` discovers and caches
/// certified obstructions through length k with one LBL^T factorization per
/// uncovered source candidate, then switches to guarded LU above k.
fn run_route(
    cases: &[(String, Vec<Vector4<f64>>, Vec<Vec<usize>>)],
    cutoff: Option<usize>,
    long_factor: FactorKind,
) -> RouteResult {
    run_route_with_guard(cases, cutoff, long_factor, GuardKind::OutwardCertified)
}

fn run_route_with_guard(
    cases: &[(String, Vec<Vector4<f64>>, Vec<Vec<usize>>)],
    cutoff: Option<usize>,
    long_factor: FactorKind,
    guard_kind: GuardKind,
) -> RouteResult {
    let started = Instant::now();
    let gradual_underflow = gradual_underflow_available();
    let mut stats = RouteStats::default();
    let mut decisions = Vec::new();
    let mut full_decisions = Vec::new();
    for (_, duals, words) in cases {
        let exact_duals = exact_binary64_dual_vertex_arrays(duals);
        let mut cache = Vec::<Obstruction>::new();
        let mut order = (0..words.len()).collect::<Vec<_>>();
        order.sort_by_key(|&index| (words[index].len(), index));
        let mut case_decisions = vec![rejected_exact_decision(); words.len()];
        for index in order {
            let word = &words[index];
            stats.words += 1;
            // Every f64 certificate in the batched route, including short-word
            // rejection and curvature pruning, assumes gradual underflow. If
            // the arithmetic environment lacks it, bypass the entire f64
            // route and exact-resolve the original candidate stream.
            if matches!(
                guard_kind,
                GuardKind::BatchedAnalyticEnvelope
                    | GuardKind::NormwiseAnalyticEnvelope
                    | GuardKind::HybridAnalyticEnvelope
            ) && !gradual_underflow
            {
                let phase_started = Instant::now();
                let decision = exact_decision(&exact_duals, word);
                stats.exact_time += phase_started.elapsed();
                stats.exact_fallbacks += 1;
                *stats.fallback_by_length.entry(word.len()).or_default() += 1;
                record_decision(&mut stats, decision);
                case_decisions[index] = decision;
                continue;
            }
            if word.len() < 5 {
                let phase_started = Instant::now();
                let decision = if certified_short_inconsistent(duals, word) {
                    stats.short_interval_rejections += 1;
                    rejected_exact_decision()
                } else {
                    stats.short_exact_solves += 1;
                    short_exact_decision(&exact_duals, word)
                };
                stats.short_exact_time += phase_started.elapsed();
                record_decision(&mut stats, decision);
                case_decisions[index] = decision;
                continue;
            }
            let phase_started = Instant::now();
            let inherited = contains_certified_subword(word, &cache);
            stats.lookup_time += phase_started.elapsed();
            if inherited {
                stats.inherited_rejections += 1;
                *stats.inherited_by_length.entry(word.len()).or_default() += 1;
                stats.rejected += 1;
                case_decisions[index] = rejected_exact_decision();
                continue;
            }

            let discover = cutoff.is_some_and(|value| word.len() >= 6 && word.len() <= value);
            let factor_kind = if discover {
                FactorKind::Lblt
            } else {
                long_factor
            };
            match factor_kind {
                FactorKind::Lu => stats.lu_factorizations += 1,
                FactorKind::Lblt => stats.lblt_factorizations += 1,
            }
            let (matrix, rhs) = build_augmented_system_from_dual_vertices(duals, word);
            let factor = if discover {
                // Inertia needs only the Bunch--Kaufman factorization. Try the
                // certified curvature rejection before solving for beta and a
                // full inverse; direct obstructions never use either result.
                let matrix35 =
                    DMatrix35::from_column_slice(matrix.nrows(), matrix.ncols(), matrix.as_slice());
                let rhs35 = DVector35::from_column_slice(rhs.as_slice());
                let phase_started = Instant::now();
                let decomposition = matrix35.lblt();
                let positive = positive_inertia(&decomposition.d());
                stats.factor_time += phase_started.elapsed();

                let obstruction_started = Instant::now();
                if positive > 5 && has_certified_rank_five_constraints(duals, word) {
                    stats.obstruction_proposals += 1;
                    if let Some(proposal) = reduced_curvature_proposal(duals, word) {
                        if certify_curvature(duals, word, &proposal.direction) {
                            stats.direct_obstructions += 1;
                            *stats.direct_by_length.entry(word.len()).or_default() += 1;
                            cache.push(Obstruction {
                                labels: word.clone(),
                                mask: label_mask(word),
                            });
                            stats.rejected += 1;
                            case_decisions[index] = rejected_exact_decision();
                            stats.obstruction_time += obstruction_started.elapsed();
                            continue;
                        }
                    }
                    stats.obstruction_unknown += 1;
                }
                stats.obstruction_time += obstruction_started.elapsed();

                let phase_started = Instant::now();
                let factor = decomposition
                    .solve(&rhs35)
                    .zip(decomposition.solve(&DMatrix35::identity(matrix.nrows(), matrix.ncols())))
                    .and_then(|(solution, inverse35)| {
                        let inverse = DMatrix::from_column_slice(
                            inverse35.nrows(),
                            inverse35.ncols(),
                            inverse35.as_slice(),
                        );
                        finite_factor_data(solution.as_slice(), inverse, Some(positive))
                    });
                stats.factor_time += phase_started.elapsed();
                factor
            } else {
                let phase_started = Instant::now();
                let factor = factor_system(&matrix, &rhs, factor_kind, false);
                stats.factor_time += phase_started.elapsed();
                factor
            };

            let phase_started = Instant::now();
            let guarded = factor.as_ref().and_then(|data| match guard_kind {
                GuardKind::OutwardCertified => certify_direct_solution_profiled(
                    duals,
                    word,
                    &matrix,
                    data,
                    &mut stats.guard_phases,
                ),
                GuardKind::BatchedAnalyticEnvelope => certify_direct_solution_batched_profiled(
                    duals,
                    word,
                    &matrix,
                    data,
                    gradual_underflow,
                    &mut stats.guard_phases,
                ),
                GuardKind::NormwiseAnalyticEnvelope => certify_direct_solution_normwise_profiled(
                    duals,
                    word,
                    &matrix,
                    data,
                    gradual_underflow,
                    &mut stats.guard_phases,
                ),
                GuardKind::HybridAnalyticEnvelope => certify_direct_solution_hybrid_profiled(
                    duals,
                    word,
                    &matrix,
                    data,
                    gradual_underflow,
                    &mut stats.guard_phases,
                ),
                GuardKind::EmpiricalThenExact => {
                    let h = matrix.view((0, 0), (word.len(), word.len())).into_owned();
                    empirical_inverse_radius_decision(&h, &matrix, &rhs, word, data)
                }
            });
            stats.guard_time += phase_started.elapsed();
            let decision = if let Some(decision) = guarded {
                decision
            } else {
                let phase_started = Instant::now();
                let decision = exact_decision(&exact_duals, word);
                stats.exact_time += phase_started.elapsed();
                decision
            };
            if decision.exact_fallback {
                stats.exact_fallbacks += 1;
                *stats.fallback_by_length.entry(word.len()).or_default() += 1;
            } else {
                stats.guarded_decisions += 1;
            }
            stats.max_beta_radius = stats
                .max_beta_radius
                .max(decision.beta_radius.unwrap_or(0.0));
            stats.max_q_radius = stats.max_q_radius.max(decision.q_radius.unwrap_or(0.0));
            record_decision(&mut stats, decision);
            case_decisions[index] = decision;
        }
        record_case_capacity_interval(&mut stats, &case_decisions, guard_kind);
        decisions.extend(case_decisions.iter().map(|decision| decision.kind));
        full_decisions.extend(case_decisions);
    }
    stats.elapsed = started.elapsed();
    RouteResult {
        cutoff,
        long_factor,
        stats,
        decisions,
        full_decisions,
    }
}

fn record_decision(stats: &mut RouteStats, decision: Decision) {
    match decision.kind {
        DecisionKind::Accept => stats.accepted += 1,
        DecisionKind::Reject => stats.rejected += 1,
    }
}

fn record_case_capacity_interval(
    stats: &mut RouteStats,
    decisions: &[Decision],
    guard_kind: GuardKind,
) {
    let accepted = decisions
        .iter()
        .filter(|decision| decision.kind == DecisionKind::Accept)
        .collect::<Vec<_>>();
    let Some(case_action) = accepted
        .iter()
        .filter_map(|decision| decision.action)
        .min_by(f64::total_cmp)
    else {
        return;
    };
    stats.best_action = Some(
        stats
            .best_action
            .map_or(case_action, |current| current.min(case_action)),
    );

    if matches!(guard_kind, GuardKind::EmpiricalThenExact) {
        return;
    }
    let Some(q_max_lower) = accepted
        .iter()
        .filter_map(|decision| decision.q_lower)
        .max_by(f64::total_cmp)
    else {
        return;
    };
    let Some(q_max_upper) = accepted
        .iter()
        .filter_map(|decision| decision.q_upper)
        .max_by(f64::total_cmp)
    else {
        return;
    };
    if !(q_max_upper > 0.0) {
        return;
    }
    let action_lower = next_down(0.5 / q_max_upper);
    let action_upper = if q_max_lower > 0.0 {
        next_up(0.5 / q_max_lower)
    } else {
        f64::INFINITY
    };
    stats.best_action_lower = Some(
        stats
            .best_action_lower
            .map_or(action_lower, |current| current.min(action_lower)),
    );
    stats.best_action_upper = Some(
        stats
            .best_action_upper
            .map_or(action_upper, |current| current.min(action_upper)),
    );
}

fn factor_system(
    matrix: &DMatrix<f64>,
    rhs: &DVector<f64>,
    kind: FactorKind,
    need_inertia: bool,
) -> Option<FactorData> {
    match kind {
        FactorKind::Lu => {
            let lu = matrix.clone().lu();
            let solution = lu.solve(rhs)?;
            let inverse = lu.try_inverse()?;
            finite_factor_data(solution.as_slice(), inverse, None)
        }
        FactorKind::Lblt => {
            let matrix35 =
                DMatrix35::from_column_slice(matrix.nrows(), matrix.ncols(), matrix.as_slice());
            let rhs35 = DVector35::from_column_slice(rhs.as_slice());
            let factor = matrix35.lblt();
            let positive = need_inertia.then(|| positive_inertia(&factor.d()));
            let solution = factor.solve(&rhs35)?;
            let inverse35 = factor.solve(&DMatrix35::identity(matrix.nrows(), matrix.ncols()))?;
            let inverse = DMatrix::from_column_slice(
                inverse35.nrows(),
                inverse35.ncols(),
                inverse35.as_slice(),
            );
            finite_factor_data(solution.as_slice(), inverse, positive)
        }
    }
}

fn finite_factor_data(
    solution: &[f64],
    inverse: DMatrix<f64>,
    positive_inertia: Option<usize>,
) -> Option<FactorData> {
    if solution.iter().any(|value| !value.is_finite())
        || inverse.iter().any(|value| !value.is_finite())
    {
        return None;
    }
    Some(FactorData {
        solution: solution.to_vec(),
        inverse,
        positive_inertia,
    })
}

// ── Certified direct predicates ──────────────────────────────────────────

fn positive_inertia(d: &DMatrix35<f64>) -> usize {
    let scale = d.iter().copied().map(f64::abs).fold(0.0, f64::max);
    let floor = scale * INERTIA_RELATIVE_FLOOR;
    let mut positive = 0;
    let mut index = 0;
    while index < d.nrows() {
        if index + 1 < d.nrows() && d[(index + 1, index)] != 0.0 {
            let a = d[(index, index)];
            let b = d[(index + 1, index)];
            let c = d[(index + 1, index + 1)];
            let centre = 0.5 * (a + c);
            let spread = 0.5 * (a - c).hypot(2.0 * b);
            positive += usize::from(centre + spread > floor);
            positive += usize::from(centre - spread > floor);
            index += 2;
        } else {
            positive += usize::from(d[(index, index)] > floor);
            index += 1;
        }
    }
    positive
}

fn certify_direct_solution(
    duals: &[Vector4<f64>],
    word: &[usize],
    matrix: &DMatrix<f64>,
    factor: &FactorData,
) -> Option<Decision> {
    certify_direct_solution_profiled(duals, word, matrix, factor, &mut GuardPhaseStats::default())
}

fn certify_direct_solution_batched(
    duals: &[Vector4<f64>],
    word: &[usize],
    matrix: &DMatrix<f64>,
    factor: &FactorData,
) -> Option<Decision> {
    certify_direct_solution_batched_profiled(
        duals,
        word,
        matrix,
        factor,
        gradual_underflow_available(),
        &mut GuardPhaseStats::default(),
    )
}

fn certify_direct_solution_normwise(
    duals: &[Vector4<f64>],
    word: &[usize],
    matrix: &DMatrix<f64>,
    factor: &FactorData,
) -> Option<Decision> {
    certify_direct_solution_normwise_profiled(
        duals,
        word,
        matrix,
        factor,
        gradual_underflow_available(),
        &mut GuardPhaseStats::default(),
    )
}

fn certify_direct_solution_hybrid(
    duals: &[Vector4<f64>],
    word: &[usize],
    matrix: &DMatrix<f64>,
    factor: &FactorData,
) -> Option<Decision> {
    certify_direct_solution_hybrid_profiled(
        duals,
        word,
        matrix,
        factor,
        gradual_underflow_available(),
        &mut GuardPhaseStats::default(),
    )
}

fn certify_direct_solution_hybrid_profiled(
    duals: &[Vector4<f64>],
    word: &[usize],
    matrix: &DMatrix<f64>,
    factor: &FactorData,
    gradual_underflow: bool,
    phases: &mut GuardPhaseStats,
) -> Option<Decision> {
    certify_direct_solution_normwise_profiled(
        duals,
        word,
        matrix,
        factor,
        gradual_underflow,
        phases,
    )
    .or_else(|| {
        certify_direct_solution_batched_profiled(
            duals,
            word,
            matrix,
            factor,
            gradual_underflow,
            phases,
        )
    })
}

fn certify_direct_solution_profiled(
    duals: &[Vector4<f64>],
    word: &[usize],
    matrix: &DMatrix<f64>,
    factor: &FactorData,
    phases: &mut GuardPhaseStats,
) -> Option<Decision> {
    let size = matrix.nrows();
    let phase_started = Instant::now();
    let exact_entries = exact_kkt_intervals(duals, word);
    phases.entries_time += phase_started.elapsed();

    let phase_started = Instant::now();
    let mut residual_norm = 0.0;
    for row in 0..size {
        let mut residual = Interval::point(if row + 1 == size { -1.0 } else { 0.0 });
        for col in 0..size {
            residual = residual
                .add(exact_entries[row * size + col].mul(Interval::point(factor.solution[col])));
        }
        if !residual.is_valid_finite() {
            return None;
        }
        residual_norm = add_up(residual_norm, residual.abs_upper());
    }
    phases.residual_time += phase_started.elapsed();

    let phase_started = Instant::now();
    let mut defect_norm = 0.0_f64;
    for row in 0..size {
        let mut row_sum = 0.0;
        for col in 0..size {
            let mut product = Interval::point(0.0);
            for mid in 0..size {
                product = product.add(
                    exact_entries[row * size + mid]
                        .mul(Interval::point(factor.inverse[(mid, col)])),
                );
            }
            let defect = Interval::point(usize::from(row == col) as f64).sub(product);
            row_sum = add_up(row_sum, defect.abs_upper());
        }
        defect_norm = defect_norm.max(row_sum);
    }
    phases.defect_time += phase_started.elapsed();
    if !(defect_norm < 1.0) {
        return None;
    }

    let phase_started = Instant::now();
    let decision = decision_from_certified_norms(word, factor, residual_norm, defect_norm);
    phases.decision_time += phase_started.elapsed();
    decision
}

/// Uses the same inverse-defect theorem as the entrywise batched enclosure,
/// but bounds the four auxiliary positive products by induced infinity norms.
///
/// The central residual and defect are still evaluated as ordinary matrix
/// products. For nonnegative matrices, submultiplicativity gives
/// `|| |A| |B| ||_inf <= ||A||_inf ||B||_inf`; outward operations turn this
/// into a valid upper bound for both rounding magnitudes and input-interval
/// propagation. This is looser than forming those four products entrywise but
/// avoids their runtime cost.
fn certify_direct_solution_normwise_profiled(
    duals: &[Vector4<f64>],
    word: &[usize],
    matrix: &DMatrix<f64>,
    factor: &FactorData,
    gradual_underflow: bool,
    phases: &mut GuardPhaseStats,
) -> Option<Decision> {
    if !gradual_underflow {
        return None;
    }
    let size = matrix.nrows();
    let phase_started = Instant::now();
    let entry_radius_norm = exact_kkt_entry_radius_inf_norm(duals, word)?;
    let matrix_norm = matrix_inf_norm_up(matrix);
    let inverse_norm = matrix_inf_norm_up(&factor.inverse);
    let solution_norm = factor
        .solution
        .iter()
        .copied()
        .map(|value| next_up(value.abs()))
        .fold(0.0, f64::max);
    if !matrix_norm.is_finite()
        || !entry_radius_norm.is_finite()
        || !inverse_norm.is_finite()
        || !solution_norm.is_finite()
    {
        return None;
    }
    let (gamma, underflow) = dot_product_error_parameters(size)?;
    phases.entries_time += phase_started.elapsed();

    let phase_started = Instant::now();
    let solution = DMatrix::from_column_slice(size, 1, &factor.solution);
    let mut residual_centre = matrix * &solution;
    residual_centre[(size - 1, 0)] -= 1.0;
    let residual_centre_norm = residual_centre
        .iter()
        .copied()
        .map(|value| next_up(value.abs()))
        .fold(0.0, f64::max);
    // The final subtraction of b contributes one to the augmented dot-product
    // magnitude in its single nonzero row.
    let residual_rounding = add_up(
        mul_up(gamma, add_up(mul_up(matrix_norm, solution_norm), 1.0)),
        underflow,
    );
    let residual_input = mul_up(entry_radius_norm, solution_norm);
    let residual_norm = add_up(
        residual_centre_norm,
        add_up(residual_rounding, residual_input),
    );
    phases.residual_time += phase_started.elapsed();
    if !residual_norm.is_finite() {
        return None;
    }

    let phase_started = Instant::now();
    let defect_centre = DMatrix::identity(size, size) - matrix * &factor.inverse;
    let defect_centre_norm = matrix_inf_norm_up(&defect_centre);
    // Summing the per-entry dot-product bounds across one row contributes
    // ||K||_inf ||R||_inf. The identity contributes one in that row, and the
    // per-entry underflow allowance occurs `size` times.
    let defect_rounding = add_up(
        mul_up(gamma, add_up(mul_up(matrix_norm, inverse_norm), 1.0)),
        mul_up(size as f64, underflow),
    );
    let defect_input = mul_up(entry_radius_norm, inverse_norm);
    let defect_norm = add_up(defect_centre_norm, add_up(defect_rounding, defect_input));
    phases.defect_time += phase_started.elapsed();
    if !(defect_norm < 1.0) {
        return None;
    }

    let phase_started = Instant::now();
    let decision = decision_from_certified_norms_with_inverse_norm(
        word,
        factor,
        residual_norm,
        defect_norm,
        inverse_norm,
    );
    phases.decision_time += phase_started.elapsed();
    decision
}

/// Experimental batched enclosure for the same inverse-defect theorem used by
/// `certify_direct_solution_profiled`.
///
/// Matrix products are ordinary f64 operations. Their exact real values are
/// enclosed afterward using a conservative dot-product rounding factor, an
/// explicit gradual-underflow allowance, and outward f64 reductions. Exact
/// rational arithmetic is not used here.
fn certify_direct_solution_batched_profiled(
    duals: &[Vector4<f64>],
    word: &[usize],
    matrix: &DMatrix<f64>,
    factor: &FactorData,
    gradual_underflow: bool,
    phases: &mut GuardPhaseStats,
) -> Option<Decision> {
    if !gradual_underflow {
        return None;
    }
    let size = matrix.nrows();
    let phase_started = Instant::now();
    let exact_entries = exact_kkt_intervals(duals, word);
    let entry_radii = DMatrix::from_fn(size, size, |row, col| {
        interval_radius_around(exact_entries[row * size + col], matrix[(row, col)])
    });
    if entry_radii.iter().any(|value| !value.is_finite()) {
        return None;
    }
    phases.entries_time += phase_started.elapsed();

    let abs_matrix = matrix.map(|value| value.abs());
    let abs_inverse = factor.inverse.map(|value| value.abs());
    let (gamma, underflow) = dot_product_error_parameters(size)?;

    let phase_started = Instant::now();
    let solution = DMatrix::from_column_slice(size, 1, &factor.solution);
    let abs_solution = solution.map(|value| value.abs());
    let mut residual_centre = matrix * &solution;
    residual_centre[(size - 1, 0)] -= 1.0;
    let residual_magnitude = positive_product_upper(&abs_matrix, &abs_solution, gamma, underflow)?;
    let residual_input = positive_product_upper(&entry_radii, &abs_solution, gamma, underflow)?;
    let mut residual_norm = 0.0_f64;
    for row in 0..size {
        // Treat the final subtraction of b as the last operation of an
        // augmented dot product. Only the final row has |b_i| = 1.
        let augmented_magnitude = add_up(
            residual_magnitude[(row, 0)],
            usize::from(row == size - 1) as f64,
        );
        let arithmetic_error = add_up(mul_up(gamma, augmented_magnitude), underflow);
        let residual_upper = add_up(
            next_up(residual_centre[(row, 0)].abs()),
            add_up(arithmetic_error, residual_input[(row, 0)]),
        );
        if !residual_upper.is_finite() {
            return None;
        }
        residual_norm = residual_norm.max(residual_upper);
    }
    phases.residual_time += phase_started.elapsed();

    let phase_started = Instant::now();
    let defect_centre = DMatrix::identity(size, size) - matrix * &factor.inverse;
    let defect_magnitude = positive_product_upper(&abs_matrix, &abs_inverse, gamma, underflow)?;
    let defect_input = positive_product_upper(&entry_radii, &abs_inverse, gamma, underflow)?;
    let mut defect_norm = 0.0_f64;
    for row in 0..size {
        let mut row_sum = 0.0;
        for col in 0..size {
            // The subtraction from I is likewise the last operation of an
            // augmented dot product. Its additional magnitude is one on the
            // diagonal and zero elsewhere.
            let augmented_magnitude =
                add_up(defect_magnitude[(row, col)], usize::from(row == col) as f64);
            let arithmetic_error = add_up(mul_up(gamma, augmented_magnitude), underflow);
            let defect_upper = add_up(
                next_up(defect_centre[(row, col)].abs()),
                add_up(arithmetic_error, defect_input[(row, col)]),
            );
            if !defect_upper.is_finite() {
                return None;
            }
            row_sum = add_up(row_sum, defect_upper);
        }
        defect_norm = defect_norm.max(row_sum);
    }
    phases.defect_time += phase_started.elapsed();
    if !(defect_norm < 1.0) {
        return None;
    }

    let phase_started = Instant::now();
    let decision = decision_from_certified_norms(word, factor, residual_norm, defect_norm);
    phases.decision_time += phase_started.elapsed();
    decision
}

/// Checks the two runtime modes that invalidate the subnormal error allowance:
/// flush-to-zero for subnormal outputs and denormals-are-zero for inputs.
///
/// `black_box` is essential: this is an arithmetic-environment check, not a
/// constant identity for LLVM to fold during a release build.
#[inline(never)]
fn gradual_underflow_available() -> bool {
    let minimum_normal = black_box(f64::MIN_POSITIVE);
    let half = black_box(0.5_f64);
    let expected_half_normal = f64::from_bits(1_u64 << 51);
    let half_normal = black_box(minimum_normal * half);

    let minimum_subnormal = black_box(f64::from_bits(1));
    let one = black_box(1.0_f64);
    let preserved_subnormal = black_box(minimum_subnormal * one);

    half_normal == expected_half_normal && preserved_subnormal == f64::from_bits(1)
}

fn interval_radius_around(interval: Interval, centre: f64) -> f64 {
    if interval.lo == centre && interval.hi == centre {
        0.0
    } else {
        next_up(
            (interval.lo - centre)
                .abs()
                .max((interval.hi - centre).abs()),
        )
    }
}

fn dot_product_error_parameters(term_count: usize) -> Option<(f64, f64)> {
    // A full machine epsilon, rather than half an epsilon, deliberately
    // overestimates the unit roundoff. The operation count also treats every
    // multiply and add separately, so fused operations only improve the bound.
    let operation_count = 2 * term_count;
    let scaled = mul_up(operation_count as f64, f64::EPSILON);
    if !(scaled < 1.0) {
        return None;
    }
    let gamma = next_up(scaled / next_down(1.0 - scaled));
    let underflow = mul_up((2 * operation_count) as f64, f64::from_bits(1));
    Some((gamma, underflow))
}

/// Upper bound for an exact nonnegative matrix product from its ordinary f64
/// evaluation. For each dot product, `fl(s) >= (1-gamma)s-underflow`.
fn positive_product_upper(
    left: &DMatrix<f64>,
    right: &DMatrix<f64>,
    gamma: f64,
    underflow: f64,
) -> Option<DMatrix<f64>> {
    let computed = left * right;
    let denominator = next_down(1.0 - gamma);
    if !(denominator > 0.0) {
        return None;
    }
    let upper = computed.map(|value| next_up(add_up(value, underflow) / denominator));
    upper.iter().all(|value| value.is_finite()).then_some(upper)
}

fn decision_from_certified_norms(
    word: &[usize],
    factor: &FactorData,
    residual_norm: f64,
    defect_norm: f64,
) -> Option<Decision> {
    let inverse_norm = matrix_inf_norm_up(&factor.inverse);
    decision_from_certified_norms_with_inverse_norm(
        word,
        factor,
        residual_norm,
        defect_norm,
        inverse_norm,
    )
}

fn decision_from_certified_norms_with_inverse_norm(
    word: &[usize],
    factor: &FactorData,
    residual_norm: f64,
    defect_norm: f64,
    inverse_norm: f64,
) -> Option<Decision> {
    let inverse_bound = next_up(inverse_norm / next_down(1.0 - defect_norm));
    let beta_radius = mul_up(inverse_bound, residual_norm);
    let beta = &factor.solution[..word.len()];
    let beta_min = beta.iter().copied().fold(f64::INFINITY, f64::min);

    if beta_min < -beta_radius {
        return Some(Decision {
            kind: DecisionKind::Reject,
            action: None,
            beta_radius: Some(beta_radius),
            q_radius: None,
            q_lower: None,
            q_upper: None,
            exact_fallback: false,
        });
    }
    if !(beta_min > beta_radius) {
        return None;
    }

    // The last KKT component is the normalization multiplier xi. At an exact
    // solution, stationarity and sum(beta)=1 give 2Q + xi = 0. The same
    // componentwise solution radius therefore encloses Q without another
    // quadratic-form evaluation or a separate perturbation formula.
    let xi = factor.solution[word.len() + 4];
    let xi_interval = Interval {
        lo: next_down(xi - beta_radius),
        hi: next_up(xi + beta_radius),
    };
    let q = Interval::point(-0.5).mul(xi_interval);
    if !q.is_valid_finite() {
        return None;
    }
    let q_lower = q.lo;
    let q_upper = q.hi;
    let q_centre = -0.5 * xi;
    let q_radius = next_up((q_centre - q_lower).abs().max((q_upper - q_centre).abs()));
    if q_lower > 0.0 {
        Some(Decision {
            kind: DecisionKind::Accept,
            action: Some(0.5 / q_centre),
            beta_radius: Some(beta_radius),
            q_radius: Some(q_radius),
            q_lower: Some(q_lower),
            q_upper: Some(q_upper),
            exact_fallback: false,
        })
    } else if q_upper <= 0.0 {
        Some(Decision {
            kind: DecisionKind::Reject,
            action: None,
            beta_radius: Some(beta_radius),
            q_radius: Some(q_radius),
            q_lower: Some(q_lower),
            q_upper: Some(q_upper),
            exact_fallback: false,
        })
    } else {
        None
    }
}

fn exact_decision(exact_duals: &[[num_rational::BigRational; 4]], word: &[usize]) -> Decision {
    match solve_kkt_exact(exact_duals, word) {
        Some(result) if result.q_exact.is_positive() => {
            exact_positive_decision_from_q(&result.q_exact, true)
        }
        _ => Decision {
            kind: DecisionKind::Reject,
            action: None,
            beta_radius: None,
            q_radius: None,
            q_lower: None,
            q_upper: None,
            exact_fallback: true,
        },
    }
}

fn exact_positive_decision_from_q(q: &BigRational, exact_fallback: bool) -> Decision {
    debug_assert!(q.is_positive());
    let action_exact = BigRational::one() / (q.clone() + q.clone());
    let (q_lower, q_upper) = exact_rational_to_f64_interval(q);
    Decision {
        kind: DecisionKind::Accept,
        action: Some(action_exact.to_f64().unwrap_or(f64::INFINITY)),
        beta_radius: None,
        q_radius: None,
        q_lower: Some(q_lower),
        q_upper: Some(q_upper),
        exact_fallback,
    }
}

fn exact_rational_to_f64_interval(value: &BigRational) -> (f64, f64) {
    let rounded = value.to_f64().unwrap_or_else(|| {
        if value.is_positive() {
            f64::INFINITY
        } else {
            f64::NEG_INFINITY
        }
    });
    if rounded == f64::INFINITY {
        return (0.0, f64::INFINITY);
    }
    if rounded == f64::NEG_INFINITY {
        return (f64::NEG_INFINITY, 0.0);
    }
    let rounded_exact =
        BigRational::from_float(rounded).expect("every finite f64 is an exact rational");
    match rounded_exact.cmp(value) {
        std::cmp::Ordering::Less => (rounded, next_up(rounded)),
        std::cmp::Ordering::Equal => (rounded, rounded),
        std::cmp::Ordering::Greater => (next_down(rounded), rounded),
    }
}

// ── Short-word exact route ───────────────────────────────────────────────

/// A nonzero (m+1)-minor of [C | d] proves rank([C | d]) > rank(C), hence
/// exact inconsistency. The interval contains the determinant of the exact
/// binary64 input, so this is a one-sided certificate, not a tolerance test.
fn certified_short_inconsistent(duals: &[Vector4<f64>], word: &[usize]) -> bool {
    let columns = word.len() + 1;
    if columns > 5 {
        return false;
    }
    let omissions = if columns == 5 {
        vec![None]
    } else {
        (0..5).map(Some).collect::<Vec<_>>()
    };
    for omitted_row in omissions {
        let rows = if let Some(omitted_row) = omitted_row {
            // For m=3 this enumerates the five 4-row subsets by their omitted
            // row. Other short lengths are not emitted by the cycle iterator.
            (0..5)
                .filter(|&row| row != omitted_row)
                .take(columns)
                .collect::<Vec<_>>()
        } else {
            (0..5).collect::<Vec<_>>()
        };
        if rows.len() != columns {
            continue;
        }
        let matrix = rows
            .iter()
            .map(|&row| {
                (0..columns)
                    .map(|col| {
                        if col == word.len() {
                            Interval::point(usize::from(row == 4) as f64)
                        } else {
                            Interval::point(constraint_entry(duals, word, row, col))
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let determinant = interval_determinant(&matrix);
        if determinant.lo > 0.0 || determinant.hi < 0.0 {
            return true;
        }
    }
    false
}

fn interval_determinant(matrix: &[Vec<Interval>]) -> Interval {
    match matrix.len() {
        0 => Interval::point(1.0),
        1 => matrix[0][0],
        size => (0..size).fold(Interval::point(0.0), |sum, col| {
            let minor = matrix[1..]
                .iter()
                .map(|row| {
                    row.iter()
                        .enumerate()
                        .filter_map(|(index, &value)| (index != col).then_some(value))
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();
            let term = matrix[0][col].mul(interval_determinant(&minor));
            if col % 2 == 0 {
                sum.add(term)
            } else {
                sum.sub(term)
            }
        }),
    }
}

/// For m < 5, full-column-rank C makes the affine feasible set a point.
/// Solving only C beta = d is therefore sufficient: stationarity multipliers
/// exist because C^T is onto, and positivity/Q can be decided exactly.
fn short_exact_decision(exact_duals: &[[BigRational; 4]], word: &[usize]) -> Decision {
    let m = word.len();
    let matrix = DMatrix::from_fn(5, m, |row, col| {
        if row == 4 {
            BigRational::one()
        } else {
            exact_duals[word[col]][row].clone()
        }
    });
    let mut rhs = DVector::from_element(5, BigRational::zero());
    rhs[4] = BigRational::one();
    let beta = match solve_linear_system(&matrix, &rhs) {
        LinearSystemSolution::Inconsistent => {
            return rejected_exact_decision();
        }
        LinearSystemSolution::Consistent {
            particular,
            kernel_basis,
        } if kernel_basis.ncols() == 0 => particular,
        // Rank-deficient short supports are outside this shortcut's premise;
        // the general exact solver remains the complete fallback.
        LinearSystemSolution::Consistent { .. } => return exact_decision(exact_duals, word),
    };
    if !beta.iter().all(BigRational::is_positive) {
        return rejected_exact_decision();
    }
    let mut q = BigRational::zero();
    for i in 0..m {
        for j in i + 1..m {
            q += beta[i].clone()
                * beta[j].clone()
                * omega_exact(&exact_duals[word[i]], &exact_duals[word[j]]);
        }
    }
    if !q.is_positive() {
        return rejected_exact_decision();
    }
    exact_positive_decision_from_q(&q, false)
}

fn rejected_exact_decision() -> Decision {
    Decision {
        kind: DecisionKind::Reject,
        action: None,
        beta_radius: None,
        q_radius: None,
        q_lower: None,
        q_upper: None,
        exact_fallback: false,
    }
}

fn omega_exact(left: &[BigRational; 4], right: &[BigRational; 4]) -> BigRational {
    left[0].clone() * right[2].clone() - left[2].clone() * right[0].clone()
        + left[1].clone() * right[3].clone()
        - left[3].clone() * right[1].clone()
}

// ── Exact KKT-entry enclosures ───────────────────────────────────────────

fn exact_kkt_intervals(duals: &[Vector4<f64>], word: &[usize]) -> Vec<Interval> {
    let m = word.len();
    let size = m + 5;
    let mut entries = vec![Interval::point(0.0); size * size];
    for i in 0..m {
        for j in i + 1..m {
            let value = omega_interval(&duals[word[i]], &duals[word[j]]);
            entries[i * size + j] = value;
            entries[j * size + i] = value;
        }
        for dim in 0..4 {
            let value = Interval::point(duals[word[i]][dim]);
            entries[i * size + m + dim] = value;
            entries[(m + dim) * size + i] = value;
        }
        entries[i * size + m + 4] = Interval::point(1.0);
        entries[(m + 4) * size + i] = Interval::point(1.0);
    }
    entries
}

/// Infinity norm of the entrywise distance between the exact binary64-input
/// KKT matrix and its ordinary f64 assembly.
///
/// Constraint, identity, and zero entries are copied exactly. Only the omega
/// block incurs roundoff. A single bound from the largest coordinate and word
/// length avoids allocating or scanning a dense interval matrix.
fn exact_kkt_entry_radius_inf_norm(duals: &[Vector4<f64>], word: &[usize]) -> Option<f64> {
    let coordinate_bound = word
        .iter()
        .flat_map(|&label| duals[label].iter())
        .copied()
        .map(|value| next_up(value.abs()))
        .fold(0.0, f64::max);
    let pair_magnitude = mul_up(4.0, mul_up(coordinate_bound, coordinate_bound));
    let (gamma, underflow) = dot_product_error_parameters(4)?;
    let per_entry = add_up(mul_up(gamma, pair_magnitude), underflow);
    let row_entries = word.len().saturating_sub(1) as f64;
    let norm = mul_up(row_entries, per_entry);
    norm.is_finite().then_some(norm)
}

/// Roundoff bound for the seven-operation coordinate formula used by
/// `omega0`. The eight-operation gamma deliberately overcounts by one; fused
/// multiply-add contraction can only reduce the error. The input coordinates
/// themselves are exact binary64 values.
#[cfg(test)]
fn omega_roundoff_radius(left: &Vector4<f64>, right: &Vector4<f64>) -> Option<f64> {
    let magnitude = [(0, 2), (2, 0), (1, 3), (3, 1)].into_iter().try_fold(
        0.0,
        |sum, (left_index, right_index)| {
            let product = mul_up(left[left_index].abs(), right[right_index].abs());
            product
                .is_finite()
                .then(|| add_up(sum, product))
                .filter(|value| value.is_finite())
        },
    )?;
    let (gamma, underflow) = dot_product_error_parameters(4)?;
    let radius = add_up(mul_up(gamma, magnitude), underflow);
    radius.is_finite().then_some(radius)
}

fn beta_dot_h_beta(duals: &[Vector4<f64>], word: &[usize], beta: &[f64]) -> f64 {
    let qp = build_qp_from_dual_vertices(duals, word);
    let beta = DVector::from_column_slice(beta);
    beta.dot(&(&qp.h * &beta))
}

fn q_interval_and_h_norm(duals: &[Vector4<f64>], word: &[usize], beta: &[f64]) -> (Interval, f64) {
    let mut q = Interval::point(0.0);
    let mut row_sums = vec![0.0; word.len()];
    for i in 0..word.len() {
        for j in i + 1..word.len() {
            let omega = omega_interval(&duals[word[i]], &duals[word[j]]);
            q = q.add(
                Interval::point(beta[i])
                    .mul(Interval::point(beta[j]))
                    .mul(omega),
            );
            let magnitude = omega.abs_upper();
            row_sums[i] = add_up(row_sums[i], magnitude);
            row_sums[j] = add_up(row_sums[j], magnitude);
        }
    }
    (q, row_sums.into_iter().fold(0.0, f64::max))
}

// ── Certified curvature obstructions and inheritance ─────────────────────

fn reduced_curvature_proposal(duals: &[Vector4<f64>], word: &[usize]) -> Option<CurvatureProposal> {
    let qp = build_qp_from_dual_vertices(duals, word);
    // nalgebra returns the thin right factor for a 5 x m matrix. Padding to a
    // square-or-tall matrix exposes every right-null direction when m > 5.
    let mut padded = DMatrix::zeros(word.len().max(5), word.len());
    padded.view_mut((0, 0), (5, word.len())).copy_from(&qp.c);
    let svd = padded.svd(true, true);
    let max_sv = svd.singular_values.iter().copied().fold(0.0, f64::max);
    let floor = (max_sv * 1e-12).max(1e-14);
    let rank = svd
        .singular_values
        .iter()
        .filter(|&&value| value > floor)
        .count();
    let nullity = word.len().saturating_sub(rank);
    if nullity == 0 {
        return None;
    }
    let vt = svd.v_t?;
    let mut basis = DMatrix::zeros(word.len(), nullity);
    for col in 0..nullity {
        for row in 0..word.len() {
            basis[(row, col)] = vt[(rank + col, row)];
        }
    }
    let reduced = basis.transpose() * qp.h * &basis;
    let eigen = reduced.symmetric_eigen();
    let (index, value) = eigen
        .eigenvalues
        .iter()
        .copied()
        .enumerate()
        .max_by(|(_, left), (_, right)| left.total_cmp(right))?;
    if value <= 0.0 {
        return None;
    }
    let direction = (&basis * eigen.eigenvectors.column(index))
        .iter()
        .copied()
        .collect();
    Some(CurvatureProposal { direction })
}

fn certify_curvature(duals: &[Vector4<f64>], word: &[usize], direction: &[f64]) -> bool {
    // Implements lem:kkt-certified-curvature-direction: project the numerical
    // proposal into ker(C) through a verified right-inverse bound, then prove
    // the exact projected direction retains positive H-curvature.
    if direction.len() != word.len() || direction.iter().any(|value| !value.is_finite()) {
        return false;
    }
    let mut residual_norm = 0.0_f64;
    for row in 0..5 {
        let mut residual = Interval::point(0.0);
        for col in 0..word.len() {
            residual = residual.add(
                Interval::point(constraint_entry(duals, word, row, col))
                    .mul(Interval::point(direction[col])),
            );
        }
        residual_norm = residual_norm.max(residual.abs_upper());
    }
    let Some(inverse_bound) = constraint_right_inverse_bound(duals, word) else {
        return false;
    };
    let correction = mul_up(inverse_bound, residual_norm);
    let (half_quadratic, h_norm) = q_interval_and_h_norm(duals, word, direction);
    let quadratic = Interval::point(2.0).mul(half_quadratic);
    let direction_norm = direction.iter().copied().map(f64::abs).fold(0.0, f64::max);
    let error = mul_up(
        word.len() as f64,
        mul_up(
            h_norm,
            add_up(
                mul_up(2.0, mul_up(direction_norm, correction)),
                mul_up(correction, correction),
            ),
        ),
    );
    quadratic.lo > error
}

fn has_certified_rank_five_constraints(duals: &[Vector4<f64>], word: &[usize]) -> bool {
    constraint_right_inverse_bound(duals, word).is_some()
}

fn constraint_right_inverse_bound(duals: &[Vector4<f64>], word: &[usize]) -> Option<f64> {
    if word.len() < 5 {
        return None;
    }
    for a in 0..word.len() - 4 {
        for b in a + 1..word.len() - 3 {
            for c in b + 1..word.len() - 2 {
                for d in c + 1..word.len() - 1 {
                    for e in d + 1..word.len() {
                        if let Some(bound) = inverse_bound_for_pivots(duals, word, [a, b, c, d, e])
                        {
                            return Some(bound);
                        }
                    }
                }
            }
        }
    }
    None
}

fn inverse_bound_for_pivots(
    duals: &[Vector4<f64>],
    word: &[usize],
    pivots: [usize; 5],
) -> Option<f64> {
    let matrix = DMatrix::from_fn(5, 5, |row, col| {
        constraint_entry(duals, word, row, pivots[col])
    });
    let inverse = matrix.clone().try_inverse()?;
    if inverse.iter().any(|value| !value.is_finite()) {
        return None;
    }
    let inverse_norm = matrix_inf_norm_up(&inverse);
    let mut defect_norm = 0.0_f64;
    for row in 0..5 {
        let mut row_sum = 0.0;
        for col in 0..5 {
            let mut product = Interval::point(0.0);
            for mid in 0..5 {
                product = product.add(
                    Interval::point(matrix[(row, mid)]).mul(Interval::point(inverse[(mid, col)])),
                );
            }
            let defect = Interval::point(usize::from(row == col) as f64).sub(product);
            row_sum = add_up(row_sum, defect.abs_upper());
        }
        defect_norm = defect_norm.max(row_sum);
    }
    if !(defect_norm < 1.0) {
        return None;
    }
    Some(next_up(inverse_norm / next_down(1.0 - defect_norm)))
}

fn constraint_entry(duals: &[Vector4<f64>], word: &[usize], row: usize, col: usize) -> f64 {
    if row == 4 {
        1.0
    } else {
        duals[word[col]][row]
    }
}

fn omega_interval(left: &Vector4<f64>, right: &Vector4<f64>) -> Interval {
    Interval::point(left[0])
        .mul(Interval::point(right[2]))
        .sub(Interval::point(left[2]).mul(Interval::point(right[0])))
        .add(Interval::point(left[1]).mul(Interval::point(right[3])))
        .sub(Interval::point(left[3]).mul(Interval::point(right[1])))
}

fn contains_certified_subword(word: &[usize], cache: &[Obstruction]) -> bool {
    // lem:kkt-cyclic-obstruction-inheritance permits exactly the
    // cyclic-order-preserving embeddings recognized below.
    let word_mask = label_mask(word);
    let mut positions = [usize::MAX; 16];
    for (position, &label) in word.iter().enumerate() {
        assert!(label < 16, "bit-mask lookup requires F <= 16");
        positions[label] = position;
    }
    cache.iter().any(|obstruction| {
        obstruction.labels.len() < word.len()
            && word_mask & obstruction.mask == obstruction.mask
            && cyclic_order_is_preserved(&obstruction.labels, &positions)
    })
}

fn label_mask(word: &[usize]) -> u16 {
    word.iter().fold(0u16, |mask, &label| {
        assert!(label < 16, "bit-mask lookup requires F <= 16");
        mask | (1u16 << label)
    })
}

/// For distinct labels on a circle, one cyclic order embeds in another iff
/// their positions have exactly one cyclic descent.
fn cyclic_order_is_preserved(labels: &[usize], positions: &[usize; 16]) -> bool {
    labels
        .iter()
        .zip(labels.iter().cycle().skip(1))
        .take(labels.len())
        .filter(|(left, right)| positions[**left] > positions[**right])
        .count()
        == 1
}

// ── Outward binary64 helpers and diagnostics ─────────────────────────────

fn matrix_inf_norm_up(matrix: &DMatrix<f64>) -> f64 {
    (0..matrix.nrows())
        .map(|row| {
            (0..matrix.ncols()).fold(0.0, |sum, col| {
                add_up(sum, next_up(matrix[(row, col)].abs()))
            })
        })
        .fold(0.0, f64::max)
}

fn next_up(value: f64) -> f64 {
    if value.is_nan() || value == f64::INFINITY {
        return value;
    }
    if value == 0.0 {
        return f64::from_bits(1);
    }
    let bits = value.to_bits();
    f64::from_bits(if value > 0.0 { bits + 1 } else { bits - 1 })
}

fn next_down(value: f64) -> f64 {
    if value.is_nan() || value == f64::NEG_INFINITY {
        return value;
    }
    if value == 0.0 {
        return -f64::from_bits(1);
    }
    let bits = value.to_bits();
    f64::from_bits(if value > 0.0 { bits - 1 } else { bits + 1 })
}

fn add_up(left: f64, right: f64) -> f64 {
    next_up(left + right)
}

fn mul_up(left: f64, right: f64) -> f64 {
    next_up(left * right)
}

fn route_label(cutoff: Option<usize>, long_factor: FactorKind) -> String {
    match (cutoff, long_factor) {
        (None, FactorKind::Lu) => "lu_only".to_string(),
        (None, FactorKind::Lblt) => "lblt_solve_only".to_string(),
        (Some(value), FactorKind::Lu) => format!("obstructions_through_{value}_then_lu"),
        (Some(value), FactorKind::Lblt) if value >= 12 => {
            "obstructions_at_all_lengths_with_lblt".to_string()
        }
        (Some(value), FactorKind::Lblt) => {
            format!("obstructions_through_{value}_then_lblt_solve")
        }
    }
}

fn print_route(result: &RouteResult) {
    let label = route_label(result.cutoff, result.long_factor);
    let stats = &result.stats;
    println!("route={label}");
    println!("route.words={}", stats.words);
    println!("route.elapsed_ms={:.6}", stats.elapsed.as_secs_f64() * 1e3);
    println!("route.lblt_factorizations={}", stats.lblt_factorizations);
    println!("route.lu_factorizations={}", stats.lu_factorizations);
    println!(
        "route.obstruction_proposals={}",
        stats.obstruction_proposals
    );
    println!("route.direct_obstructions={}", stats.direct_obstructions);
    println!("route.inherited_rejections={}", stats.inherited_rejections);
    println!("route.obstruction_unknown={}", stats.obstruction_unknown);
    println!("route.guarded_decisions={}", stats.guarded_decisions);
    println!("route.exact_fallbacks={}", stats.exact_fallbacks);
    println!("route.short_exact_solves={}", stats.short_exact_solves);
    println!(
        "route.short_interval_rejections={}",
        stats.short_interval_rejections
    );
    println!("route.accepted={}", stats.accepted);
    println!("route.rejected={}", stats.rejected);
    println!("route.best_action={:?}", stats.best_action);
    println!("route.best_action_lower={:?}", stats.best_action_lower);
    println!("route.best_action_upper={:?}", stats.best_action_upper);
    println!("route.max_beta_radius={:.6e}", stats.max_beta_radius);
    println!("route.max_q_radius={:.6e}", stats.max_q_radius);
    println!(
        "route.lookup_ms={:.6}",
        stats.lookup_time.as_secs_f64() * 1e3
    );
    println!(
        "route.factor_ms={:.6}",
        stats.factor_time.as_secs_f64() * 1e3
    );
    println!(
        "route.obstruction_ms={:.6}",
        stats.obstruction_time.as_secs_f64() * 1e3
    );
    println!("route.guard_ms={:.6}", stats.guard_time.as_secs_f64() * 1e3);
    println!(
        "route.guard_entries_ms={:.6}",
        stats.guard_phases.entries_time.as_secs_f64() * 1e3
    );
    println!(
        "route.guard_residual_ms={:.6}",
        stats.guard_phases.residual_time.as_secs_f64() * 1e3
    );
    println!(
        "route.guard_defect_ms={:.6}",
        stats.guard_phases.defect_time.as_secs_f64() * 1e3
    );
    println!(
        "route.guard_decision_ms={:.6}",
        stats.guard_phases.decision_time.as_secs_f64() * 1e3
    );
    println!("route.exact_ms={:.6}", stats.exact_time.as_secs_f64() * 1e3);
    println!(
        "route.short_exact_ms={:.6}",
        stats.short_exact_time.as_secs_f64() * 1e3
    );
    println!("route.direct_by_length={:?}", stats.direct_by_length);
    println!("route.inherited_by_length={:?}", stats.inherited_by_length);
    println!("route.fallback_by_length={:?}", stats.fallback_by_length);
}

fn compare_routes(results: &[RouteResult]) {
    let baseline = &results[0];
    for result in &results[1..] {
        let mismatches = baseline
            .decisions
            .iter()
            .zip(&result.decisions)
            .filter(|(left, right)| left != right)
            .count();
        println!(
            "comparison.route={}",
            route_label(result.cutoff, result.long_factor)
        );
        println!("comparison.decision_mismatches={mismatches}");
        println!(
            "comparison.speedup_vs_lu_only={:.6}",
            baseline.stats.elapsed.as_secs_f64() / result.stats.elapsed.as_secs_f64()
        );
    }
}

// ── End-to-end exact agreement ───────────────────────────────────────────

fn run_exact_audit(cases: &[(String, Vec<Vector4<f64>>, Vec<Vec<usize>>)]) {
    let mut compared = 0;
    let mut guarded_decisive = 0;
    let mut guarded_mismatches = 0;
    let mut short_compared = 0;
    let mut short_mismatches = 0;
    let mut short_interval_rejections = 0;
    let mut short_interval_false_rejections = 0;
    let mut max_action_error = 0.0_f64;
    for (_, duals, words) in cases {
        let exact_duals = exact_binary64_dual_vertex_arrays(duals);
        for (local, word) in words.iter().enumerate() {
            // Complete exact check for F5--F7 and a deterministic sample of
            // longer streams. A curvature-pruned word is not compared as a
            // KKT-feasibility decision: the theorem rejects it as a maximum
            // even when an exact stationary point exists.
            let selected = duals.len() <= 7 || local % 257 == 0;
            if !selected {
                continue;
            }
            let exact = exact_decision(&exact_duals, word);
            compared += 1;
            if word.len() < 5 {
                short_compared += 1;
                short_mismatches +=
                    usize::from(short_exact_decision(&exact_duals, word).kind != exact.kind);
                if certified_short_inconsistent(duals, word) {
                    short_interval_rejections += 1;
                    short_interval_false_rejections +=
                        usize::from(exact.kind != DecisionKind::Reject);
                }
            }
            let (matrix, rhs) = build_augmented_system_from_dual_vertices(duals, word);
            if let Some(factor) = factor_system(&matrix, &rhs, FactorKind::Lu, false) {
                if let Some(decision) = certify_direct_solution(duals, word, &matrix, &factor) {
                    guarded_decisive += 1;
                    guarded_mismatches += usize::from(decision.kind != exact.kind);
                    if let Some(exact_action) = exact.action {
                        if let Some(action) = decision.action {
                            max_action_error = max_action_error.max((action - exact_action).abs());
                        }
                    }
                }
            }
        }
    }
    println!("exact_audit.compared={compared}");
    println!("exact_audit.guarded_decisive={guarded_decisive}");
    println!("exact_audit.guarded_mismatches={guarded_mismatches}");
    println!("exact_audit.short_compared={short_compared}");
    println!("exact_audit.short_mismatches={short_mismatches}");
    println!("exact_audit.short_interval_rejections={short_interval_rejections}");
    println!("exact_audit.short_interval_false_rejections={short_interval_false_rejections}");
    println!("exact_audit.max_guarded_action_error={max_action_error:.6e}");
}

fn run_exact_aggregation_audit(cases: &[(String, Vec<Vector4<f64>>, Vec<Vec<usize>>)]) {
    run_exact_route_agreement_audit(
        "general_batched",
        cases,
        Some(7),
        GuardKind::BatchedAnalyticEnvelope,
    );
}

#[derive(Clone, Copy, Debug)]
enum RichOutputSelection {
    MinimizersOnly,
    RelativeWindowElevenTenths,
    AllAdmissible,
}

impl RichOutputSelection {
    fn label(self) -> &'static str {
        match self {
            Self::MinimizersOnly => "minimizers",
            Self::RelativeWindowElevenTenths => "relative_window_11_10",
            Self::AllAdmissible => "all_admissible",
        }
    }
}

#[derive(Debug)]
struct RichOutputResolution {
    records: Vec<(usize, BigRational)>,
    contenders: usize,
    exact_solves: usize,
    exact_time: Duration,
}

/// Cheap architecture spike for a lean `(sigma, exact action)` search result.
///
/// The complete exact solve over every word is the reference only. The route
/// under test first applies the selected certified f64/exact pruning, then
/// exact-solves only candidates whose certified action intervals can meet the
/// requested minimum/window. This packet deliberately starts with the retained
/// F <= 7 cohort: it is large enough to exercise pruning and exact resolution,
/// but cheap enough to reject a bad architecture before broader runs.
fn run_rich_output_spike(cases: &[(String, Vec<Vector4<f64>>, Vec<Vec<usize>>)]) {
    let selected_cases = cases
        .iter()
        .filter(|(_, duals, _)| duals.len() <= 7)
        .collect::<Vec<_>>();
    assert!(
        !selected_cases.is_empty(),
        "rich-output spike requires at least one F <= 7 case"
    );

    let mut cases_compared = 0usize;
    let mut record_mismatches = 0usize;
    let mut total_words = 0usize;
    let mut total_accepted = 0usize;
    let mut total_reference_exact_time = Duration::ZERO;
    let mut total_route_time = Duration::ZERO;
    let mut total_resolution_time = [Duration::ZERO; 3];
    let mut total_resolution_solves = [0usize; 3];
    let mut total_returned = [0usize; 3];

    for (source_id, duals, words) in selected_cases {
        let exact_duals = exact_binary64_dual_vertex_arrays(duals);
        let single_case = vec![(source_id.clone(), duals.clone(), words.clone())];
        let route = run_route_with_guard(
            &single_case,
            Some(usize::MAX),
            FactorKind::Lblt,
            GuardKind::HybridAnalyticEnvelope,
        );
        assert_eq!(route.full_decisions.len(), words.len());

        let reference_started = Instant::now();
        let reference_actions = words
            .iter()
            .map(|word| exact_action_for_word(&exact_duals, word))
            .collect::<Vec<_>>();
        let reference_exact_time = reference_started.elapsed();
        let exact_positive = reference_actions
            .iter()
            .filter(|value| value.is_some())
            .count();
        let accepted = route
            .full_decisions
            .iter()
            .filter(|decision| decision.kind == DecisionKind::Accept)
            .count();

        println!("rich_output.source_id={source_id}");
        println!("rich_output.words={}", words.len());
        println!("rich_output.route_accepted={accepted}");
        println!("rich_output.reference_exact_positive={exact_positive}");
        println!(
            "rich_output.route_ms={:.6}",
            route.stats.elapsed.as_secs_f64() * 1e3
        );
        println!(
            "rich_output.reference_exact_all_ms={:.6}",
            reference_exact_time.as_secs_f64() * 1e3
        );

        for (selection_index, selection) in [
            RichOutputSelection::MinimizersOnly,
            RichOutputSelection::RelativeWindowElevenTenths,
            RichOutputSelection::AllAdmissible,
        ]
        .into_iter()
        .enumerate()
        {
            let reference = selected_exact_records(words, &reference_actions, selection, None);
            let resolution =
                resolve_rich_output(&exact_duals, words, &route.full_decisions, selection);
            let observed = resolution
                .records
                .iter()
                .map(|(index, action)| (words[*index].clone(), action.clone()))
                .collect::<Vec<_>>();
            let matches = observed == reference;
            record_mismatches += usize::from(!matches);
            total_resolution_time[selection_index] += resolution.exact_time;
            total_resolution_solves[selection_index] += resolution.exact_solves;
            total_returned[selection_index] += resolution.records.len();

            println!("rich_output.mode={}", selection.label());
            println!("rich_output.contenders={}", resolution.contenders);
            println!("rich_output.exact_solves={}", resolution.exact_solves);
            println!("rich_output.returned={}", resolution.records.len());
            println!(
                "rich_output.exact_resolution_ms={:.6}",
                resolution.exact_time.as_secs_f64() * 1e3
            );
            println!("rich_output.records_match={matches}");
        }

        cases_compared += 1;
        total_words += words.len();
        total_accepted += accepted;
        total_reference_exact_time += reference_exact_time;
        total_route_time += route.stats.elapsed;
    }

    println!("rich_output.summary_cases={cases_compared}");
    println!("rich_output.summary_words={total_words}");
    println!("rich_output.summary_route_accepted={total_accepted}");
    println!("rich_output.summary_record_mismatches={record_mismatches}");
    println!(
        "rich_output.summary_route_ms={:.6}",
        total_route_time.as_secs_f64() * 1e3
    );
    println!(
        "rich_output.summary_reference_exact_all_ms={:.6}",
        total_reference_exact_time.as_secs_f64() * 1e3
    );
    for (index, selection) in [
        RichOutputSelection::MinimizersOnly,
        RichOutputSelection::RelativeWindowElevenTenths,
        RichOutputSelection::AllAdmissible,
    ]
    .into_iter()
    .enumerate()
    {
        println!("rich_output.summary_mode={}", selection.label());
        println!(
            "rich_output.summary_exact_solves={}",
            total_resolution_solves[index]
        );
        println!("rich_output.summary_returned={}", total_returned[index]);
        println!(
            "rich_output.summary_exact_resolution_ms={:.6}",
            total_resolution_time[index].as_secs_f64() * 1e3
        );
    }
    assert_eq!(
        record_mismatches, 0,
        "certified rich-output records differ from exact-all reference"
    );
}

fn resolve_rich_output(
    exact_duals: &[[BigRational; 4]],
    words: &[Vec<usize>],
    decisions: &[Decision],
    selection: RichOutputSelection,
) -> RichOutputResolution {
    let minimum_contenders = minimum_contender_indices(decisions);
    let mut exact_actions = vec![None; words.len()];
    let mut exact_solves = 0usize;
    let mut exact_time = Duration::ZERO;
    resolve_exact_indices(
        exact_duals,
        words,
        &minimum_contenders,
        &mut exact_actions,
        &mut exact_solves,
        &mut exact_time,
    );
    let minimum = minimum_contenders
        .iter()
        .filter_map(|&index| exact_actions[index].as_ref())
        .min()
        .cloned()
        .expect("accepted minimum contenders contain an exact-positive word");

    let selected_indices = match selection {
        RichOutputSelection::MinimizersOnly => minimum_contenders,
        RichOutputSelection::RelativeWindowElevenTenths => {
            let threshold = &minimum * BigRational::new(11.into(), 10.into());
            decisions
                .iter()
                .enumerate()
                .filter(|(_, decision)| decision.kind == DecisionKind::Accept)
                .filter(|(_, decision)| {
                    action_interval(**decision).is_none_or(|(lower, _)| {
                        !lower.is_finite() || f64_to_rational(lower) <= threshold
                    })
                })
                .map(|(index, _)| index)
                .collect::<Vec<_>>()
        }
        RichOutputSelection::AllAdmissible => decisions
            .iter()
            .enumerate()
            .filter(|(_, decision)| decision.kind == DecisionKind::Accept)
            .map(|(index, _)| index)
            .collect::<Vec<_>>(),
    };
    resolve_exact_indices(
        exact_duals,
        words,
        &selected_indices,
        &mut exact_actions,
        &mut exact_solves,
        &mut exact_time,
    );

    let mut records = selected_exact_records(words, &exact_actions, selection, Some(&minimum))
        .into_iter()
        .map(|(word, action)| {
            let index = words
                .iter()
                .position(|candidate| candidate == &word)
                .expect("selected word belongs to candidate stream");
            (index, action)
        })
        .collect::<Vec<_>>();
    records.sort_by_key(|(index, _)| *index);

    RichOutputResolution {
        records,
        contenders: selected_indices.len(),
        exact_solves,
        exact_time,
    }
}

fn resolve_exact_indices(
    exact_duals: &[[BigRational; 4]],
    words: &[Vec<usize>],
    indices: &[usize],
    exact_actions: &mut [Option<BigRational>],
    exact_solves: &mut usize,
    exact_time: &mut Duration,
) {
    for &index in indices {
        if exact_actions[index].is_some() {
            continue;
        }
        let started = Instant::now();
        let action = exact_action_for_word(exact_duals, &words[index])
            .expect("certified accepted candidate must be exact-positive");
        *exact_time += started.elapsed();
        *exact_solves += 1;
        exact_actions[index] = Some(action);
    }
}

fn minimum_contender_indices(decisions: &[Decision]) -> Vec<usize> {
    let accepted = decisions
        .iter()
        .enumerate()
        .filter(|(_, decision)| decision.kind == DecisionKind::Accept)
        .collect::<Vec<_>>();
    let minimum_upper = accepted
        .iter()
        .filter_map(|(_, decision)| action_interval(**decision).map(|(_, upper)| upper))
        .min_by(f64::total_cmp)
        .unwrap_or(f64::INFINITY);
    accepted
        .into_iter()
        .filter(|(_, decision)| {
            action_interval(**decision).is_none_or(|(lower, _)| lower <= minimum_upper)
        })
        .map(|(index, _)| index)
        .collect()
}

fn action_interval(decision: Decision) -> Option<(f64, f64)> {
    let (q_lower, q_upper) = decision.q_lower.zip(decision.q_upper)?;
    if !(q_lower > 0.0 && q_upper >= q_lower) {
        return None;
    }
    Some((next_down(0.5 / q_upper), next_up(0.5 / q_lower)))
}

fn exact_action_for_word(exact_duals: &[[BigRational; 4]], word: &[usize]) -> Option<BigRational> {
    solve_kkt_exact(exact_duals, word).and_then(|result| {
        result
            .q_exact
            .is_positive()
            .then(|| BigRational::one() / (result.q_exact.clone() + result.q_exact))
    })
}

fn selected_exact_records(
    words: &[Vec<usize>],
    actions: &[Option<BigRational>],
    selection: RichOutputSelection,
    known_minimum: Option<&BigRational>,
) -> Vec<(Vec<usize>, BigRational)> {
    let minimum = known_minimum
        .cloned()
        .or_else(|| actions.iter().filter_map(Option::as_ref).min().cloned());
    let Some(minimum) = minimum else {
        return Vec::new();
    };
    let relative_threshold = &minimum * BigRational::new(11.into(), 10.into());
    words
        .iter()
        .zip(actions)
        .filter_map(|(word, action)| {
            let action = action.as_ref()?;
            let selected = match selection {
                RichOutputSelection::MinimizersOnly => action == &minimum,
                RichOutputSelection::RelativeWindowElevenTenths => action <= &relative_threshold,
                RichOutputSelection::AllAdmissible => true,
            };
            selected.then(|| (word.clone(), action.clone()))
        })
        .collect()
}

fn run_exact_route_agreement_audit(
    cohort: &str,
    cases: &[(String, Vec<Vector4<f64>>, Vec<Vec<usize>>)],
    maximum_facet_count: Option<usize>,
    guard_kind: GuardKind,
) {
    let mut cases_compared = 0;
    let mut scalar_mismatches = 0;
    let mut minimizer_class_mismatches = 0;
    let mut exact_positive_pruned = 0usize;
    let mut fast_scalar_mismatches = 0usize;
    let mut capacity_interval_mismatches = 0usize;
    let mut maximum_fast_scalar_error = 0.0_f64;
    let mut exact_fallbacks = 0usize;
    for (source_id, duals, words) in cases
        .iter()
        .filter(|(_, duals, _)| maximum_facet_count.is_none_or(|maximum| duals.len() <= maximum))
    {
        let exact_duals = exact_binary64_dual_vertex_arrays(duals);
        let exact_actions = words
            .iter()
            .map(|word| {
                solve_kkt_exact(&exact_duals, word).and_then(|result| {
                    result
                        .q_exact
                        .is_positive()
                        .then(|| BigRational::one() / (result.q_exact.clone() + result.q_exact))
                })
            })
            .collect::<Vec<_>>();

        let single_case = vec![(source_id.clone(), duals.clone(), words.clone())];
        let route =
            run_route_with_guard(&single_case, Some(usize::MAX), FactorKind::Lblt, guard_kind);
        exact_fallbacks += route.stats.exact_fallbacks;
        let retained = route
            .decisions
            .iter()
            .map(|decision| *decision == DecisionKind::Accept)
            .collect::<Vec<_>>();
        let case_exact_positive_pruned = route
            .decisions
            .iter()
            .zip(&exact_actions)
            .filter(|(decision, exact_action)| {
                **decision == DecisionKind::Reject && exact_action.is_some()
            })
            .count();
        exact_positive_pruned += case_exact_positive_pruned;

        let reference = exact_minimizer_classes(words, &exact_actions, None);
        let pruned = exact_minimizer_classes(words, &exact_actions, Some(&retained));
        let scalar_match =
            reference.as_ref().map(|value| &value.0) == pruned.as_ref().map(|value| &value.0);
        let classes_match =
            reference.as_ref().map(|value| &value.1) == pruned.as_ref().map(|value| &value.1);
        cases_compared += 1;
        scalar_mismatches += usize::from(!scalar_match);
        minimizer_class_mismatches += usize::from(!classes_match);
        let exact_scalar = reference.as_ref().and_then(|value| value.0.to_f64());
        let fast_scalar_error = route
            .stats
            .best_action
            .zip(exact_scalar)
            .map_or(f64::INFINITY, |(fast, exact)| (fast - exact).abs());
        let fast_scalar_match =
            fast_scalar_error <= 1e-10 * exact_scalar.unwrap_or(1.0).abs().max(1.0);
        fast_scalar_mismatches += usize::from(!fast_scalar_match);
        let capacity_interval_match = exact_scalar.is_some_and(|exact| {
            route
                .stats
                .best_action_lower
                .zip(route.stats.best_action_upper)
                .is_some_and(|(lower, upper)| lower <= exact && exact <= upper)
        });
        capacity_interval_mismatches += usize::from(!capacity_interval_match);
        maximum_fast_scalar_error = maximum_fast_scalar_error.max(fast_scalar_error);
        println!("aggregation.cohort={cohort}");
        println!("aggregation.source_id={source_id}");
        println!("aggregation.words={}", words.len());
        println!(
            "aggregation.retained={}",
            retained.iter().filter(|&&value| value).count()
        );
        println!("aggregation.scalar_match={scalar_match}");
        println!("aggregation.minimizer_classes_match={classes_match}");
        println!("aggregation.exact_positive_pruned={case_exact_positive_pruned}");
        println!("aggregation.fast_scalar={:?}", route.stats.best_action);
        println!("aggregation.exact_scalar={exact_scalar:?}");
        println!("aggregation.fast_scalar_error={fast_scalar_error:.6e}");
        println!(
            "aggregation.capacity_interval={:?}",
            route
                .stats
                .best_action_lower
                .zip(route.stats.best_action_upper)
        );
        println!("aggregation.capacity_interval_match={capacity_interval_match}");
        println!(
            "aggregation.exact_fallbacks={}",
            route.stats.exact_fallbacks
        );
        println!(
            "aggregation.reference_minimizers={:?}",
            reference.as_ref().map(|value| &value.1)
        );
        println!(
            "aggregation.pruned_minimizers={:?}",
            pruned.as_ref().map(|value| &value.1)
        );
    }
    println!("aggregation.summary_cohort={cohort}");
    println!("aggregation.cases_compared={cases_compared}");
    println!("aggregation.exact_positive_pruned={exact_positive_pruned}");
    println!("aggregation.scalar_mismatches={scalar_mismatches}");
    println!("aggregation.minimizer_class_mismatches={minimizer_class_mismatches}");
    println!("aggregation.fast_scalar_mismatches={fast_scalar_mismatches}");
    println!("aggregation.capacity_interval_mismatches={capacity_interval_mismatches}");
    println!("aggregation.maximum_fast_scalar_error={maximum_fast_scalar_error:.6e}");
    println!("aggregation.exact_fallbacks={exact_fallbacks}");
    assert!(
        cases_compared > 0,
        "{cohort}: empty route-agreement cohort cannot support the audit"
    );
    assert_eq!(scalar_mismatches, 0, "{cohort}: exact scalar changed");
    assert_eq!(
        minimizer_class_mismatches, 0,
        "{cohort}: exact minimizing cyclic classes changed"
    );
    assert_eq!(
        fast_scalar_mismatches, 0,
        "{cohort}: reported f64 scalar exceeded the audit tolerance"
    );
    assert_eq!(
        capacity_interval_mismatches, 0,
        "{cohort}: certified capacity interval missed the exact scalar"
    );
}

fn run_product_enumeration_agreement_audit(
    product_cases: &[(String, Vec<Vector4<f64>>, Vec<Vec<usize>>)],
) {
    let mut compared = 0usize;
    let mut scalar_mismatches = 0usize;
    for (source_id, duals, billiard_words) in product_cases
        .iter()
        .filter(|(_, duals, _)| duals.len() <= 7)
    {
        let exact_duals = exact_binary64_dual_vertex_arrays(duals);
        let transition =
            try_exact_binary64_transition_matrix_assuming_origin_interior(&exact_duals)
                .expect("known clean product has an exact transition graph");
        let general_words = SimpleDirectedCyclesCanonical::new(&transition).collect::<Vec<_>>();
        let exact_minimum = |words: &[Vec<usize>]| {
            words
                .iter()
                .filter_map(|word| solve_kkt_exact(&exact_duals, word))
                .filter(|result| result.q_exact.is_positive())
                .map(|result| BigRational::one() / (result.q_exact.clone() + result.q_exact))
                .min()
        };
        let general_minimum = exact_minimum(&general_words);
        let billiard_minimum = exact_minimum(billiard_words);
        let scalar_match = general_minimum == billiard_minimum;
        compared += 1;
        scalar_mismatches += usize::from(!scalar_match);
        println!("product_agreement.source_id={source_id}");
        println!("product_agreement.general_words={}", general_words.len());
        println!("product_agreement.billiard_words={}", billiard_words.len());
        println!("product_agreement.scalar_match={scalar_match}");
        println!(
            "product_agreement.general_scalar={:?}",
            general_minimum.as_ref().and_then(ToPrimitive::to_f64)
        );
        println!(
            "product_agreement.billiard_scalar={:?}",
            billiard_minimum.as_ref().and_then(ToPrimitive::to_f64)
        );
    }
    println!("product_agreement.cases_compared={compared}");
    println!("product_agreement.scalar_mismatches={scalar_mismatches}");
    assert!(
        compared > 0,
        "empty product-agreement cohort cannot support the audit"
    );
    assert_eq!(
        scalar_mismatches, 0,
        "general and billiard exact scalar disagree"
    );
}

fn run_existing_product_solver_agreement_audit(
    product_cases: &[(String, Vec<Vector4<f64>>, Vec<Vec<usize>>)],
) {
    let mut compared = 0usize;
    let mut mismatches = 0usize;
    let mut maximum_error = 0.0_f64;
    for (source_id, duals, words) in product_cases {
        let exact_duals = exact_binary64_dual_vertex_arrays(duals);
        let exact_capacity = words
            .iter()
            .filter_map(|word| solve_kkt_exact(&exact_duals, word))
            .filter(|result| result.q_exact.is_positive())
            .map(|result| BigRational::one() / (result.q_exact.clone() + result.q_exact))
            .min()
            .and_then(|value| value.to_f64())
            .expect("known product has an exact positive candidate");
        let report = capacity_f64_only_with_policy_and_method_profiled(
            duals,
            F64ValidationPolicy::LpOriginVertex,
            F64CapacityMethod::ProductBilliardOrHk,
        )
        .0;
        let observed = match report.outcome {
            F64CapacityOutcome::Success { capacity, .. } => capacity,
            F64CapacityOutcome::Failure { ref reason } => {
                panic!("existing product solver failed for {source_id}: {reason:?}")
            }
        };
        let error = (observed - exact_capacity).abs();
        let matches = error <= 1e-10 * exact_capacity.abs().max(1.0);
        compared += 1;
        mismatches += usize::from(!matches);
        maximum_error = maximum_error.max(error);
        println!("existing_product_verification.source_id={source_id}");
        println!("existing_product_verification.capacity={observed:.16e}");
        println!("existing_product_verification.exact_capacity={exact_capacity:.16e}");
        println!("existing_product_verification.error={error:.6e}");
        println!("existing_product_verification.matches={matches}");
    }
    println!("existing_product_verification.compared={compared}");
    println!("existing_product_verification.mismatches={mismatches}");
    println!("existing_product_verification.maximum_error={maximum_error:.6e}");
    assert_eq!(
        mismatches, 0,
        "existing product solver disagrees with exact billiard aggregation"
    );
}

fn exact_minimizer_classes(
    words: &[Vec<usize>],
    actions: &[Option<BigRational>],
    retained: Option<&[bool]>,
) -> Option<(BigRational, Vec<Vec<usize>>)> {
    let minimum = actions
        .iter()
        .enumerate()
        .filter(|(index, action)| action.is_some() && retained.is_none_or(|mask| mask[*index]))
        .filter_map(|(_, action)| action.as_ref())
        .min()
        .cloned()?;
    let classes = actions
        .iter()
        .enumerate()
        .filter(|(index, action)| {
            retained.is_none_or(|mask| mask[*index]) && action.as_ref() == Some(&minimum)
        })
        .map(|(index, _)| words[index].clone())
        .collect();
    Some((minimum, classes))
}

fn run_inertia_equivalence_audit(cases: &[(String, Vec<Vector4<f64>>, Vec<Vec<usize>>)]) {
    let mut compared = 0;
    let mut certified_curvature = 0;
    let mut missed_by_inertia = 0;
    let mut inertia_proposals = 0;
    let mut uncertified_proposals = 0;
    for (_, duals, words) in cases {
        for word in words.iter().filter(|word| (6..=10).contains(&word.len())) {
            if !has_certified_rank_five_constraints(duals, word) {
                continue;
            }
            let (matrix, _) = build_augmented_system_from_dual_vertices(duals, word);
            let proposed = lblt_positive_inertia_only(&matrix) > 5;
            let certified = reduced_curvature_proposal(duals, word)
                .is_some_and(|proposal| certify_curvature(duals, word, &proposal.direction));
            compared += 1;
            inertia_proposals += usize::from(proposed);
            certified_curvature += usize::from(certified);
            missed_by_inertia += usize::from(certified && !proposed);
            uncertified_proposals += usize::from(proposed && !certified);
        }
    }
    println!("inertia_audit.compared={compared}");
    println!("inertia_audit.inertia_proposals={inertia_proposals}");
    println!("inertia_audit.certified_curvature={certified_curvature}");
    println!("inertia_audit.missed_by_inertia={missed_by_inertia}");
    println!("inertia_audit.uncertified_proposals={uncertified_proposals}");
}

fn lblt_positive_inertia_only(matrix: &DMatrix<f64>) -> usize {
    let matrix35 = DMatrix35::from_column_slice(matrix.nrows(), matrix.ncols(), matrix.as_slice());
    positive_inertia(&matrix35.lblt().d())
}

fn run_scaled_audit(cases: &[(String, Vec<Vector4<f64>>, Vec<Vec<usize>>)]) {
    let selected = cases
        .iter()
        .filter(|(_, duals, _)| matches!(duals.len(), 6 | 8 | 10 | 12))
        .collect::<Vec<_>>();
    for scale in [1e-2, 1.0, 1e2, 1e3] {
        let mut words = 0;
        let mut exact_fallbacks = 0;
        let mut exact_compared = 0;
        let mut mismatches = 0;
        let mut max_beta_radius = 0.0_f64;
        let mut max_q_radius = 0.0_f64;
        let mut validation = BTreeMap::<String, usize>::new();
        for (_, duals, source_words) in &selected {
            let scaled = duals.iter().map(|dual| dual * scale).collect::<Vec<_>>();
            *validation
                .entry(
                    validate_f64_polytope_input(&scaled)
                        .status
                        .label()
                        .to_string(),
                )
                .or_default() += 1;
            let exact_duals = exact_binary64_dual_vertex_arrays(&scaled);
            for (index, word) in source_words.iter().enumerate() {
                words += 1;
                let (matrix, rhs) = build_augmented_system_from_dual_vertices(&scaled, word);
                let decision = factor_system(&matrix, &rhs, FactorKind::Lu, false)
                    .as_ref()
                    .and_then(|factor| certify_direct_solution(&scaled, word, &matrix, factor));
                exact_fallbacks += usize::from(decision.is_none());
                if index % 401 == 0 || decision.is_none() {
                    exact_compared += 1;
                    let exact = exact_decision(&exact_duals, word);
                    if let Some(decision) = decision {
                        mismatches += usize::from(decision.kind != exact.kind);
                        max_beta_radius = max_beta_radius.max(decision.beta_radius.unwrap_or(0.0));
                        max_q_radius = max_q_radius.max(decision.q_radius.unwrap_or(0.0));
                    }
                }
            }
        }
        println!("scale.value={scale:.1e}");
        println!("scale.validation={validation:?}");
        println!("scale.words={words}");
        println!("scale.exact_fallbacks={exact_fallbacks}");
        println!("scale.exact_compared={exact_compared}");
        println!("scale.wrong_guarded_decisions={mismatches}");
        println!("scale.max_beta_radius={max_beta_radius:.6e}");
        println!("scale.max_q_radius={max_q_radius:.6e}");
    }
}

fn run_near_singular_audit() {
    let base = hko_pentagon().dual_vertices_f64.clone();
    let words = [
        vec![0, 1, 6, 7, 3, 4, 5, 9],
        vec![1, 8, 7, 3, 4, 5, 9],
        vec![0, 1, 7, 3, 9, 5],
        vec![1, 7, 2, 8, 4, 6, 5],
    ];
    for epsilon in [1e-3, 1e-6, 1e-9, 1e-12] {
        let duals = perturb(&base, epsilon);
        let exact_duals = exact_binary64_dual_vertex_arrays(&duals);
        let mut lu_decisive = 0;
        let mut lblt_decisive = 0;
        let mut mismatches = 0;
        for word in &words {
            let exact = exact_decision(&exact_duals, word);
            let (matrix, rhs) = build_augmented_system_from_dual_vertices(&duals, word);
            for (kind, decisive) in [
                (FactorKind::Lu, &mut lu_decisive),
                (FactorKind::Lblt, &mut lblt_decisive),
            ] {
                if let Some(decision) = factor_system(&matrix, &rhs, kind, false)
                    .as_ref()
                    .and_then(|factor| certify_direct_solution(&duals, word, &matrix, factor))
                {
                    *decisive += 1;
                    mismatches += usize::from(decision.kind != exact.kind);
                }
            }
        }
        println!("near_singular.epsilon={epsilon:.1e}");
        println!(
            "near_singular.validation={}",
            validate_f64_polytope_input(&duals).status.label()
        );
        println!("near_singular.lu_guard_decisive={lu_decisive}");
        println!("near_singular.lblt_guard_decisive={lblt_decisive}");
        println!("near_singular.wrong_guarded_decisions={mismatches}");
    }
}

// ── Optional adversarial and legacy experiments ──────────────────────────

fn perturb(base: &[Vector4<f64>], epsilon: f64) -> Vec<Vector4<f64>> {
    base.iter()
        .enumerate()
        .map(|(index, dual)| {
            let t = index as f64 + 1.0;
            Vector4::new(
                dual[0] + epsilon * (0.17 * t).sin(),
                dual[1] + epsilon * (0.31 * t).cos(),
                dual[2] + epsilon * (0.43 * t).sin(),
                dual[3] + epsilon * (0.59 * t).cos(),
            )
        })
        .collect()
}

#[derive(Clone, Debug)]
struct BetaBoundarySnapshot {
    beta: Vec<f64>,
    beta_radius: f64,
    condition_proxy: f64,
    decision: Option<Decision>,
}

#[derive(Clone, Debug)]
struct BetaBoundaryCandidate {
    source: String,
    duals: Vec<Vector4<f64>>,
    word: Vec<usize>,
    direction: usize,
    lambda: f64,
    target: usize,
    target_beta: f64,
    beta_radius: f64,
    condition_proxy: f64,
    cheap: Decision,
}

fn run_beta_boundary_search() {
    const EXACT_CAP: usize = 128;
    let mut bases = Vec::new();
    for seed in [DEFAULT_SEED, 314_159, 271_828] {
        for case in generated_f64_cases(1, seed)
            .into_iter()
            .filter(|case| case.family == "generated_random_f64")
            .filter(|case| (5..=8).contains(&case.dual_vertices.len()))
        {
            let exact = exact_binary64_dual_vertex_arrays(&case.dual_vertices);
            let Some(transition) =
                try_exact_binary64_transition_matrix_assuming_origin_interior(&exact).ok()
            else {
                continue;
            };
            let words = SimpleDirectedCyclesCanonical::new(&transition)
                .filter(|word| word.len() >= 5)
                .collect::<Vec<_>>();
            bases.push((
                format!("seed_{seed}_{}", case.source_id),
                case.dual_vertices,
                words,
            ));
        }
    }
    let hko = hko_pentagon().dual_vertices_f64.clone();
    bases.push((
        "hko_known_risky".to_string(),
        hko,
        vec![
            vec![0, 1, 6, 7, 3, 4, 5, 9],
            vec![1, 8, 7, 3, 4, 5, 9],
            vec![0, 1, 7, 3, 9, 5],
            vec![1, 7, 2, 8, 4, 6, 5],
        ],
    ));

    let mut base_words = 0usize;
    let mut paths = 0usize;
    let mut endpoint_rejected = 0usize;
    let mut crossings = 0usize;
    let mut boundary_samples = 0usize;
    let mut indeterminate_samples = 0usize;
    let mut decisive_samples = 0usize;
    let mut candidates = Vec::new();
    let levels = [1e-3, 1e-2, 5e-2, 2e-1, 5e-1, 1.0];

    for (source, base, words) in &bases {
        for word in words {
            let Some(base_snapshot) = beta_boundary_snapshot(base, word) else {
                continue;
            };
            if base_snapshot.beta.iter().any(|&beta| beta <= 0.0) {
                continue;
            }
            base_words += 1;
            for direction in 0..16 {
                for sign in [-1.0, 1.0] {
                    paths += 1;
                    for level in levels {
                        let endpoint_lambda = sign * level;
                        let endpoint = beta_path_transform(base, endpoint_lambda, direction);
                        let validation = validate_f64_polytope_input(&endpoint);
                        if !matches!(
                            validation.status.label(),
                            "accepted_decisive" | "accepted_ambiguous"
                        ) {
                            endpoint_rejected += 1;
                            continue;
                        }
                        let Some(endpoint_snapshot) = beta_boundary_snapshot(&endpoint, word)
                        else {
                            continue;
                        };
                        let Some(target) = endpoint_snapshot
                            .beta
                            .iter()
                            .enumerate()
                            .filter(|(_, beta)| **beta <= 0.0)
                            .min_by(|(_, left), (_, right)| left.total_cmp(right))
                            .map(|(index, _)| index)
                        else {
                            continue;
                        };
                        crossings += 1;
                        let mut negative_lambda = endpoint_lambda;
                        let mut positive_lambda = 0.0;
                        let mut negative_snapshot = endpoint_snapshot;
                        let mut positive_snapshot = base_snapshot.clone();
                        let mut closest_negative = None;
                        let mut closest_positive = None;

                        for _ in 0..72 {
                            let midpoint = 0.5 * (negative_lambda + positive_lambda);
                            if midpoint == negative_lambda || midpoint == positive_lambda {
                                break;
                            }
                            let midpoint_duals = beta_path_transform(base, midpoint, direction);
                            let Some(midpoint_snapshot) =
                                beta_boundary_snapshot(&midpoint_duals, word)
                            else {
                                break;
                            };
                            if midpoint_snapshot
                                .beta
                                .iter()
                                .enumerate()
                                .any(|(index, &beta)| index != target && beta <= 0.0)
                            {
                                negative_lambda = midpoint;
                                negative_snapshot = midpoint_snapshot;
                                continue;
                            }
                            boundary_samples += 1;
                            if let Some(cheap) = midpoint_snapshot.decision {
                                decisive_samples += 1;
                                let candidate = BetaBoundaryCandidate {
                                    source: source.clone(),
                                    duals: midpoint_duals,
                                    word: word.clone(),
                                    direction,
                                    lambda: midpoint,
                                    target,
                                    target_beta: midpoint_snapshot.beta[target],
                                    beta_radius: midpoint_snapshot.beta_radius,
                                    condition_proxy: midpoint_snapshot.condition_proxy,
                                    cheap,
                                };
                                if candidate.target_beta > 0.0 {
                                    retain_closest_decisive(&mut closest_positive, candidate);
                                } else {
                                    retain_closest_decisive(&mut closest_negative, candidate);
                                }
                            } else {
                                indeterminate_samples += 1;
                            }
                            if midpoint_snapshot.beta[target] > 0.0 {
                                positive_lambda = midpoint;
                                positive_snapshot = midpoint_snapshot;
                            } else {
                                negative_lambda = midpoint;
                                negative_snapshot = midpoint_snapshot;
                            }
                        }

                        for (lambda, snapshot) in [
                            (negative_lambda, negative_snapshot),
                            (positive_lambda, positive_snapshot),
                        ] {
                            boundary_samples += 1;
                            let duals = beta_path_transform(base, lambda, direction);
                            if let Some(cheap) = snapshot.decision {
                                decisive_samples += 1;
                                let candidate = BetaBoundaryCandidate {
                                    source: source.clone(),
                                    duals,
                                    word: word.clone(),
                                    direction,
                                    lambda,
                                    target,
                                    target_beta: snapshot.beta[target],
                                    beta_radius: snapshot.beta_radius,
                                    condition_proxy: snapshot.condition_proxy,
                                    cheap,
                                };
                                if candidate.target_beta > 0.0 {
                                    retain_closest_decisive(&mut closest_positive, candidate);
                                } else {
                                    retain_closest_decisive(&mut closest_negative, candidate);
                                }
                            } else {
                                indeterminate_samples += 1;
                            }
                        }
                        candidates.extend(closest_negative);
                        candidates.extend(closest_positive);
                        break;
                    }
                }
            }
        }
    }

    candidates.sort_by(|left, right| {
        let left_zero = usize::from(left.beta_radius == 0.0);
        let right_zero = usize::from(right.beta_radius == 0.0);
        let left_confidence = left.target_beta.abs() / left.beta_radius;
        let right_confidence = right.target_beta.abs() / right.beta_radius;
        right_zero
            .cmp(&left_zero)
            .then_with(|| left_confidence.total_cmp(&right_confidence))
            .then_with(|| right.condition_proxy.total_cmp(&left.condition_proxy))
    });

    let mut route_relevant = 0usize;
    let mut exact_compared = 0usize;
    let mut true_false = 0usize;
    let mut false_true = 0usize;
    let mut exact_radius_compared = 0usize;
    let mut exact_radius_violations = 0usize;
    let mut worst_radius_ratio = 0.0_f64;
    let mut exact_time = Duration::ZERO;
    let mut first_failure = None;
    let mut first_radius_failure = None;
    let mut exact_boundary_paths = BTreeSet::new();
    for candidate in &candidates {
        if exact_compared == EXACT_CAP {
            break;
        }
        let exact_duals = exact_binary64_dual_vertex_arrays(&candidate.duals);
        let Ok(transition) =
            try_exact_binary64_transition_matrix_assuming_origin_interior(&exact_duals)
        else {
            continue;
        };
        if !word_is_emitted(&transition, &candidate.word) {
            continue;
        }
        route_relevant += 1;
        exact_boundary_paths.insert((
            candidate.source.clone(),
            candidate.word.clone(),
            candidate.direction,
            candidate.lambda.is_sign_positive(),
        ));
        let started = Instant::now();
        let exact_result = solve_kkt_exact(&exact_duals, &candidate.word);
        exact_time += started.elapsed();
        exact_compared += 1;
        let exact_kind = if exact_result
            .as_ref()
            .is_some_and(|result| result.q_exact.is_positive())
        {
            DecisionKind::Accept
        } else {
            DecisionKind::Reject
        };
        let is_true_false =
            candidate.cheap.kind == DecisionKind::Accept && exact_kind == DecisionKind::Reject;
        let is_false_true =
            candidate.cheap.kind == DecisionKind::Reject && exact_kind == DecisionKind::Accept;
        true_false += usize::from(is_true_false);
        false_true += usize::from(is_false_true);
        if is_true_false || is_false_true {
            first_failure = Some(candidate.clone());
            break;
        }
        if let Some(exact_result) = exact_result {
            exact_radius_compared += 1;
            let exact_component_error = (&exact_result.beta[candidate.target]
                - f64_to_rational(candidate.target_beta))
            .abs();
            let exact_radius = f64_to_rational(candidate.beta_radius);
            let ratio = if candidate.beta_radius == 0.0 {
                if exact_component_error.is_zero() {
                    0.0
                } else {
                    f64::INFINITY
                }
            } else {
                exact_component_error.to_f64().unwrap_or(f64::INFINITY) / candidate.beta_radius
            };
            worst_radius_ratio = worst_radius_ratio.max(ratio);
            if exact_component_error > exact_radius {
                exact_radius_violations += 1;
                first_radius_failure.get_or_insert_with(|| candidate.clone());
            }
        }
    }

    println!("beta_boundary.bases={}", bases.len());
    println!("beta_boundary.base_positive_words={base_words}");
    println!("beta_boundary.paths={paths}");
    println!("beta_boundary.endpoint_rejected={endpoint_rejected}");
    println!("beta_boundary.crossings={crossings}");
    println!("beta_boundary.samples={boundary_samples}");
    println!("beta_boundary.indeterminate_samples={indeterminate_samples}");
    println!("beta_boundary.decisive_samples={decisive_samples}");
    println!("beta_boundary.decisive_candidates={}", candidates.len());
    println!("beta_boundary.route_relevant={route_relevant}");
    println!("beta_boundary.exact_compared={exact_compared}");
    println!(
        "beta_boundary.exact_boundary_paths={}",
        exact_boundary_paths.len()
    );
    println!("beta_boundary.true_false={true_false}");
    println!("beta_boundary.false_true={false_true}");
    println!("beta_boundary.exact_radius_compared={exact_radius_compared}");
    println!("beta_boundary.exact_radius_violations={exact_radius_violations}");
    println!("beta_boundary.worst_radius_ratio={worst_radius_ratio:.17e}");
    println!(
        "beta_boundary.exact_time_ms={:.6}",
        exact_time.as_secs_f64() * 1e3
    );
    if let Some(failure) = first_failure {
        println!("beta_boundary.failure_found=true");
        println!("beta_boundary.failure_source={}", failure.source);
        println!("beta_boundary.failure_direction={}", failure.direction);
        println!("beta_boundary.failure_lambda={:.17e}", failure.lambda);
        println!("beta_boundary.failure_target={}", failure.target);
        println!(
            "beta_boundary.failure_target_beta={:.17e}",
            failure.target_beta
        );
        println!(
            "beta_boundary.failure_beta_radius={:.17e}",
            failure.beta_radius
        );
        println!(
            "beta_boundary.failure_condition={:.17e}",
            failure.condition_proxy
        );
        println!("beta_boundary.failure_word={:?}", failure.word);
        println!("beta_boundary.failure_duals={:?}", failure.duals);
    } else {
        println!("beta_boundary.failure_found=false");
    }
    if let Some(failure) = first_radius_failure {
        println!("beta_boundary.radius_failure_found=true");
        println!("beta_boundary.radius_failure_source={}", failure.source);
        println!(
            "beta_boundary.radius_failure_direction={}",
            failure.direction
        );
        println!(
            "beta_boundary.radius_failure_lambda={:.17e}",
            failure.lambda
        );
        println!("beta_boundary.radius_failure_target={}", failure.target);
        println!(
            "beta_boundary.radius_failure_target_beta={:.17e}",
            failure.target_beta
        );
        println!(
            "beta_boundary.radius_failure_beta_radius={:.17e}",
            failure.beta_radius
        );
        println!("beta_boundary.radius_failure_word={:?}", failure.word);
        println!("beta_boundary.radius_failure_duals={:?}", failure.duals);
    } else {
        println!("beta_boundary.radius_failure_found=false");
    }
}

fn retain_closest_decisive(
    slot: &mut Option<BetaBoundaryCandidate>,
    candidate: BetaBoundaryCandidate,
) {
    let replace = slot.as_ref().is_none_or(|current| {
        match (candidate.beta_radius == 0.0, current.beta_radius == 0.0) {
            (true, false) => true,
            (false, true) => false,
            (true, true) => candidate.target_beta.abs() < current.target_beta.abs(),
            (false, false) => {
                candidate.target_beta.abs() / candidate.beta_radius
                    < current.target_beta.abs() / current.beta_radius
            }
        }
    });
    if replace {
        *slot = Some(candidate);
    }
}

fn beta_boundary_snapshot(duals: &[Vector4<f64>], word: &[usize]) -> Option<BetaBoundarySnapshot> {
    let (matrix, rhs) = build_augmented_system_from_dual_vertices(duals, word);
    let factor = factor_system(&matrix, &rhs, FactorKind::Lblt, false)?;
    let h = matrix.view((0, 0), (word.len(), word.len())).into_owned();
    let solution = DVector::from_column_slice(&factor.solution);
    let residual = (&matrix * &solution - &rhs).norm();
    let beta_radius = factor.inverse.norm() * residual;
    let decision = empirical_inverse_radius_decision(&h, &matrix, &rhs, word, &factor);
    Some(BetaBoundarySnapshot {
        beta: factor.solution[..word.len()].to_vec(),
        beta_radius,
        condition_proxy: matrix.norm() * factor.inverse.norm(),
        decision,
    })
}

fn beta_path_transform(base: &[Vector4<f64>], lambda: f64, direction: usize) -> Vec<Vector4<f64>> {
    base.iter()
        .enumerate()
        .map(|(index, dual)| {
            let mut transformed = *dual;
            for coordinate in 0..4 {
                let phase = (1 + direction * 97 + index * 17 + coordinate * 31) as f64;
                let signed = (phase * 0.754_877_666).sin();
                let scale = dual[coordinate].abs().max(0.1);
                transformed[coordinate] += lambda * scale * signed;
            }
            transformed
        })
        .collect()
}

fn word_is_emitted(transition: &DMatrix<bool>, word: &[usize]) -> bool {
    word.iter()
        .zip(word.iter().cycle().skip(1))
        .all(|(&left, &right)| transition[(left, right)])
}

#[derive(Clone, Debug)]
struct RiskyPredicateCase {
    configuration_id: usize,
    duals: Vec<Vector4<f64>>,
    word: Vec<usize>,
    epsilon: f64,
    direction: usize,
    anisotropy: f64,
    scale: f64,
    condition_proxy: f64,
    cheap: Decision,
    outward: Option<Decision>,
    validation: String,
}

fn run_adversarial_predicate_search() {
    const EXACT_CAP: usize = 128;
    let base = hko_pentagon().dual_vertices_f64.clone();
    let exact_base = exact_binary64_dual_vertex_arrays(&base);
    let transition = try_exact_binary64_transition_matrix_assuming_origin_interior(&exact_base)
        .expect("HKO binary64 input has an exact transition graph");
    let emitted = SimpleDirectedCyclesCanonical::new(&transition).collect::<Vec<_>>();

    let mut ranked_words = emitted
        .iter()
        .filter(|word| word.len() >= 5)
        .filter_map(|word| {
            let (matrix, rhs) = build_augmented_system_from_dual_vertices(&base, word);
            factor_system(&matrix, &rhs, FactorKind::Lblt, false)
                .map(|factor| (matrix.norm() * factor.inverse.norm(), word.clone()))
        })
        .collect::<Vec<_>>();
    ranked_words.sort_by(|left, right| right.0.total_cmp(&left.0));
    let mut words = ranked_words
        .into_iter()
        .take(16)
        .map(|(_, word)| word)
        .collect::<Vec<_>>();
    for word in [
        vec![0, 1, 6, 7, 3, 4, 5, 9],
        vec![1, 8, 7, 3, 4, 5, 9],
        vec![0, 1, 7, 3, 9, 5],
        vec![1, 7, 2, 8, 4, 6, 5],
    ] {
        if emitted.contains(&word) && !words.contains(&word) {
            words.push(word);
        }
    }

    let mut configurations = 0usize;
    let mut validation_rejected = 0usize;
    let mut systems = 0usize;
    let mut factor_failures = 0usize;
    let mut cheap_indeterminate = 0usize;
    let mut outward_agreements = 0usize;
    let mut outward_opposites = 0usize;
    let mut risky = Vec::<RiskyPredicateCase>::new();
    let epsilons = [1e-8, 1e-10, 1e-12, 1e-13, 1e-14, 1e-15, 1e-16, 0.0];
    let anisotropies = [1e-3, 1e-2, 1.0, 1e2, 1e3];
    let scales = [1e-2, 1.0, 1e2];

    for epsilon in epsilons {
        let direction_count = if epsilon == 0.0 { 1 } else { 8 };
        for direction in 0..direction_count {
            for anisotropy in anisotropies {
                for scale in scales {
                    configurations += 1;
                    let duals = adversarial_transform(&base, epsilon, direction, anisotropy, scale);
                    let validation = validate_f64_polytope_input(&duals);
                    let validation_label = validation.status.label().to_string();
                    if !matches!(
                        validation_label.as_str(),
                        "accepted_decisive" | "accepted_ambiguous"
                    ) {
                        validation_rejected += 1;
                        continue;
                    }
                    for word in &words {
                        systems += 1;
                        let (matrix, rhs) = build_augmented_system_from_dual_vertices(&duals, word);
                        let Some(factor) = factor_system(&matrix, &rhs, FactorKind::Lblt, false)
                        else {
                            factor_failures += 1;
                            continue;
                        };
                        let h = matrix.view((0, 0), (word.len(), word.len())).into_owned();
                        let cheap =
                            empirical_inverse_radius_decision(&h, &matrix, &rhs, word, &factor);
                        let outward = certify_direct_solution(&duals, word, &matrix, &factor);
                        let Some(cheap) = cheap else {
                            cheap_indeterminate += 1;
                            continue;
                        };
                        match outward {
                            Some(outward) if outward.kind == cheap.kind => {
                                outward_agreements += 1;
                            }
                            Some(_) => {
                                outward_opposites += 1;
                                risky.push(RiskyPredicateCase {
                                    configuration_id: configurations - 1,
                                    duals: duals.clone(),
                                    word: word.clone(),
                                    epsilon,
                                    direction,
                                    anisotropy,
                                    scale,
                                    condition_proxy: matrix.norm() * factor.inverse.norm(),
                                    cheap,
                                    outward,
                                    validation: validation_label.clone(),
                                });
                            }
                            None => risky.push(RiskyPredicateCase {
                                configuration_id: configurations - 1,
                                duals: duals.clone(),
                                word: word.clone(),
                                epsilon,
                                direction,
                                anisotropy,
                                scale,
                                condition_proxy: matrix.norm() * factor.inverse.norm(),
                                cheap,
                                outward,
                                validation: validation_label.clone(),
                            }),
                        }
                    }
                }
            }
        }
    }

    risky.sort_by(|left, right| {
        let left_opposite = usize::from(left.outward.is_some());
        let right_opposite = usize::from(right.outward.is_some());
        right_opposite
            .cmp(&left_opposite)
            .then_with(|| right.condition_proxy.total_cmp(&left.condition_proxy))
    });

    let mut exact_compared = 0usize;
    let mut transition_present = 0usize;
    let mut route_candidates_scanned = 0usize;
    let mut true_false = 0usize;
    let mut false_true = 0usize;
    let mut exact_time = Duration::ZERO;
    let mut first_failure = None::<RiskyPredicateCase>;
    let mut exact_configurations = BTreeMap::new();
    for candidate in &risky {
        if exact_compared == EXACT_CAP {
            break;
        }
        route_candidates_scanned += 1;
        let exact_configuration = exact_configurations
            .entry(candidate.configuration_id)
            .or_insert_with(|| {
                let exact_duals = exact_binary64_dual_vertex_arrays(&candidate.duals);
                let transition =
                    try_exact_binary64_transition_matrix_assuming_origin_interior(&exact_duals)
                        .ok()?;
                Some((exact_duals, transition))
            });
        let Some((exact_duals, transition)) = exact_configuration.as_ref() else {
            continue;
        };
        let emitted_here = candidate
            .word
            .iter()
            .zip(candidate.word.iter().cycle().skip(1))
            .all(|(&left, &right)| transition[(left, right)]);
        if !emitted_here {
            continue;
        }
        transition_present += 1;
        let started = Instant::now();
        let exact = exact_decision(exact_duals, &candidate.word);
        exact_time += started.elapsed();
        exact_compared += 1;
        true_false += usize::from(
            candidate.cheap.kind == DecisionKind::Accept && exact.kind == DecisionKind::Reject,
        );
        false_true += usize::from(
            candidate.cheap.kind == DecisionKind::Reject && exact.kind == DecisionKind::Accept,
        );
        if candidate.cheap.kind != exact.kind {
            first_failure = Some(candidate.clone());
            break;
        }
    }

    println!("adversarial.base_words={}", words.len());
    println!("adversarial.configurations={configurations}");
    println!("adversarial.validation_rejected={validation_rejected}");
    println!("adversarial.systems={systems}");
    println!("adversarial.factor_failures={factor_failures}");
    println!("adversarial.cheap_indeterminate={cheap_indeterminate}");
    println!("adversarial.outward_agreements={outward_agreements}");
    println!("adversarial.outward_opposites={outward_opposites}");
    println!("adversarial.risky={}", risky.len());
    println!("adversarial.exact_cap={EXACT_CAP}");
    println!("adversarial.route_candidates_scanned={route_candidates_scanned}");
    println!("adversarial.exact_compared={exact_compared}");
    println!("adversarial.transition_present={transition_present}");
    println!("adversarial.true_false={true_false}");
    println!("adversarial.false_true={false_true}");
    println!(
        "adversarial.exact_time_ms={:.6}",
        exact_time.as_secs_f64() * 1e3
    );
    if let Some(failure) = first_failure {
        println!("adversarial.failure_found=true");
        println!("adversarial.failure_epsilon={:.17e}", failure.epsilon);
        println!("adversarial.failure_direction={}", failure.direction);
        println!("adversarial.failure_anisotropy={:.17e}", failure.anisotropy);
        println!("adversarial.failure_scale={:.17e}", failure.scale);
        println!(
            "adversarial.failure_condition={:.17e}",
            failure.condition_proxy
        );
        println!("adversarial.failure_validation={}", failure.validation);
        println!("adversarial.failure_word={:?}", failure.word);
        println!("adversarial.failure_duals={:?}", failure.duals);
    } else {
        println!("adversarial.failure_found=false");
    }
}

fn adversarial_transform(
    base: &[Vector4<f64>],
    epsilon: f64,
    direction: usize,
    anisotropy: f64,
    scale: f64,
) -> Vec<Vector4<f64>> {
    base.iter()
        .enumerate()
        .map(|(index, dual)| {
            let mut transformed = *dual;
            for coordinate in 0..4 {
                let phase = (1 + direction * 97 + index * 17 + coordinate * 31) as f64;
                let noise = (phase * 0.754_877_666).sin() + 0.5 * (phase * 1.324_717_957).cos();
                transformed[coordinate] += epsilon * noise;
            }
            transformed[0] *= anisotropy;
            transformed[1] *= anisotropy;
            transformed[2] /= anisotropy;
            transformed[3] /= anisotropy;
            transformed * scale
        })
        .collect()
}

fn run_factor_microbenchmark(cases: &[(String, Vec<Vector4<f64>>, Vec<Vec<usize>>)]) {
    let systems = cases
        .iter()
        .flat_map(|(_, duals, words)| {
            words
                .iter()
                .filter(|word| word.len() >= 6)
                .map(|word| {
                    let (matrix, rhs) = build_augmented_system_from_dual_vertices(duals, word);
                    (matrix, rhs, word.len())
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    for minimum_length in [6, 9, 10] {
        let selected = systems
            .iter()
            .filter(|(_, _, length)| *length >= minimum_length)
            .collect::<Vec<_>>();
        let repetitions = 5;
        let lu = benchmark(repetitions, || {
            for (matrix, rhs, _) in &selected {
                std::hint::black_box(factor_system(matrix, rhs, FactorKind::Lu, false));
            }
        });
        let lblt = benchmark(repetitions, || {
            for (matrix, rhs, _) in &selected {
                std::hint::black_box(factor_system(matrix, rhs, FactorKind::Lblt, false));
            }
        });
        println!("factor.minimum_length={minimum_length}");
        println!("factor.systems={}", selected.len());
        println!(
            "factor.lu_ms={:.6}",
            lu.as_secs_f64() * 1e3 / repetitions as f64
        );
        println!(
            "factor.lblt_ms={:.6}",
            lblt.as_secs_f64() * 1e3 / repetitions as f64
        );
        println!(
            "factor.lblt_over_lu={:.6}",
            lblt.as_secs_f64() / lu.as_secs_f64()
        );
    }
}

fn benchmark(repetitions: usize, mut action: impl FnMut()) -> Duration {
    let started = Instant::now();
    for _ in 0..repetitions {
        action();
    }
    started.elapsed()
}

// ── Focused theorem-to-code regression tests ─────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cyclic_embedding_accepts_rotations_and_rejects_reversal() {
        let source = Obstruction {
            labels: vec![0, 2, 1],
            mask: label_mask(&[0, 2, 1]),
        };
        assert!(contains_certified_subword(
            &[3, 0, 4, 2, 5, 1],
            std::slice::from_ref(&source)
        ));
        assert!(contains_certified_subword(
            &[2, 5, 1, 3, 0, 4],
            std::slice::from_ref(&source)
        ));
        assert!(!contains_certified_subword(
            &[3, 0, 4, 1, 5, 2],
            std::slice::from_ref(&source)
        ));
    }

    #[test]
    fn nextafter_helpers_enclose_zero() {
        assert!(next_down(0.0) < 0.0);
        assert!(next_up(0.0) > 0.0);
        let cancellation = Interval::point(0.1).sub(Interval::point(0.1));
        assert!(cancellation.lo <= 0.0 && cancellation.hi >= 0.0);
    }

    #[test]
    fn interval_determinant_has_expected_controls() {
        let identity = (0..3)
            .map(|row| {
                (0..3)
                    .map(|col| Interval::point(usize::from(row == col) as f64))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let determinant = interval_determinant(&identity);
        assert!(determinant.lo <= 1.0 && determinant.hi >= 1.0);

        let singular = vec![
            vec![Interval::point(1.0), Interval::point(2.0)],
            vec![Interval::point(1.0), Interval::point(2.0)],
        ];
        let determinant = interval_determinant(&singular);
        assert!(determinant.lo <= 0.0 && determinant.hi >= 0.0);
    }

    #[test]
    fn batched_rounding_envelope_contains_exact_matrix_products() {
        assert!(
            gradual_underflow_available(),
            "the supported development target must preserve subnormals"
        );
        let left = DMatrix::from_fn(6, 6, |row, col| {
            let sign = if (row + col) % 2 == 0 { 1.0 } else { -1.0 };
            sign * ((row + 1) as f64) * 10.0_f64.powi(col as i32 - 3)
        });
        let right = DMatrix::from_fn(6, 3, |row, col| {
            let sign = if (2 * row + col) % 3 == 0 { -1.0 } else { 1.0 };
            sign * ((col + 2) as f64) * 10.0_f64.powi(2 - row as i32)
        });
        assert_product_rounding_enclosed(&left, &right);
        assert_product_then_subtraction_enclosed(&left, &right, |row, col| {
            usize::from(row == col) as f64
        });

        // One-column output exercises nalgebra's non-GEMM matrix-vector path.
        let vector = DMatrix::from_fn(6, 1, |row, _| {
            (-1.0_f64).powi(row as i32) * 10.0_f64.powi(row as i32 - 3)
        });
        assert_product_rounding_enclosed(&left, &vector);
        assert_product_then_subtraction_enclosed(&left, &vector, |row, _| {
            usize::from(row == 5) as f64
        });

        let minimum_subnormal = f64::from_bits(1);
        let underflow_left =
            DMatrix::from_row_slice(2, 2, &[minimum_subnormal, 0.0, 0.0, minimum_subnormal]);
        let underflow_right = DMatrix::from_element(2, 1, 0.5);
        assert_product_rounding_enclosed(&underflow_left, &underflow_right);
    }

    #[test]
    fn capacity_interval_uses_maximum_q_endpoints() {
        let candidate = |q_lower, q_upper| Decision {
            kind: DecisionKind::Accept,
            action: Some(1.0),
            beta_radius: Some(0.0),
            q_radius: Some(0.0),
            q_lower: Some(q_lower),
            q_upper: Some(q_upper),
            exact_fallback: false,
        };
        let decisions = [candidate(2.0, 3.0), candidate(4.0, 5.0)];
        let mut stats = RouteStats::default();
        record_case_capacity_interval(&mut stats, &decisions, GuardKind::BatchedAnalyticEnvelope);
        let (lower, upper) = stats
            .best_action_lower
            .zip(stats.best_action_upper)
            .expect("positive Q intervals produce an action interval");
        assert!(lower <= 0.1 && 0.1 <= upper);
        assert!(lower <= 0.125 && 0.125 <= upper);
        assert!(upper < 0.13, "the lower-Q candidate must not set Q_max");
    }

    #[test]
    fn exact_rational_interval_handles_rounding_and_underflow() {
        for value in [
            BigRational::new(1.into(), 10.into()),
            BigRational::new((-1).into(), 10.into()),
            BigRational::new(1.into(), num_bigint::BigInt::from(1_u8) << 1100_u32),
        ] {
            let (lower, upper) = exact_rational_to_f64_interval(&value);
            assert!(
                f64_to_rational(lower) <= value,
                "lower endpoint exceeds exact rational"
            );
            assert!(
                value <= f64_to_rational(upper),
                "upper endpoint is below exact rational"
            );
        }
    }

    #[test]
    fn fused_kkt_entry_radius_norm_encloses_exact_assembly_error() {
        let duals = hko_pentagon().dual_vertices_f64.clone();
        let word = vec![0, 1, 6, 7, 3, 4, 5, 9];
        let (matrix, _) = build_augmented_system_from_dual_vertices(&duals, &word);
        let bound = exact_kkt_entry_radius_inf_norm(&duals, &word)
            .expect("finite fixture has a finite assembly bound");
        let exact_duals = exact_binary64_dual_vertex_arrays(&duals);

        let exact_norm = (0..word.len())
            .map(|row| {
                (0..word.len())
                    .filter(|&col| col != row)
                    .map(|col| {
                        let exact = if row < col {
                            omega_exact(&exact_duals[word[row]], &exact_duals[word[col]])
                        } else {
                            omega_exact(&exact_duals[word[col]], &exact_duals[word[row]])
                        };
                        (exact - f64_to_rational(matrix[(row, col)])).abs()
                    })
                    .fold(BigRational::zero(), |sum, error| sum + error)
            })
            .max()
            .expect("nonempty word");
        assert!(
            exact_norm <= f64_to_rational(bound),
            "fused norm missed exact KKT assembly error"
        );
    }

    #[test]
    fn analytic_omega_roundoff_radius_encloses_exact_formula_error() {
        let minimum_subnormal = f64::from_bits(1);
        let pairs = [
            (
                Vector4::new(1e3, 1e-3, -1e3, -1e-3),
                Vector4::new(-1e-3, 1e3, 1e-3, -1e3),
            ),
            (
                Vector4::new(0.1, -0.2, 0.3, -0.4),
                Vector4::new(-0.5, 0.6, -0.7, 0.8),
            ),
            (
                Vector4::new(minimum_subnormal, 0.0, 0.0, 0.0),
                Vector4::new(0.0, 0.0, 0.5, 0.0),
            ),
        ];
        for (left, right) in pairs {
            let left_exact = std::array::from_fn(|index| f64_to_rational(left[index]));
            let right_exact = std::array::from_fn(|index| f64_to_rational(right[index]));
            let exact = omega_exact(&left_exact, &right_exact);
            let computed =
                left[0] * right[2] - left[2] * right[0] + left[1] * right[3] - left[3] * right[1];
            let error = (exact - f64_to_rational(computed)).abs();
            let radius =
                omega_roundoff_radius(&left, &right).expect("finite inputs have finite bound");
            assert!(
                error <= f64_to_rational(radius),
                "analytic omega roundoff bound missed exact error"
            );
        }
    }

    fn assert_product_rounding_enclosed(left: &DMatrix<f64>, right: &DMatrix<f64>) {
        let (gamma, underflow) =
            dot_product_error_parameters(left.ncols()).expect("small test dot product");
        let computed = left * right;
        let magnitude_upper = positive_product_upper(
            &left.map(|value| value.abs()),
            &right.map(|value| value.abs()),
            gamma,
            underflow,
        )
        .expect("finite positive product");

        for row in 0..left.nrows() {
            for col in 0..right.ncols() {
                let exact = (0..left.ncols())
                    .map(|mid| {
                        f64_to_rational(left[(row, mid)]) * f64_to_rational(right[(mid, col)])
                    })
                    .fold(BigRational::zero(), |sum, term| sum + term);
                let error = (exact - f64_to_rational(computed[(row, col)])).abs();
                let error_upper = add_up(mul_up(gamma, magnitude_upper[(row, col)]), underflow);
                assert!(
                    error <= f64_to_rational(error_upper),
                    "entry ({row},{col}) escaped the rounding envelope"
                );
            }
        }
    }

    fn assert_product_then_subtraction_enclosed(
        left: &DMatrix<f64>,
        right: &DMatrix<f64>,
        target: impl Fn(usize, usize) -> f64,
    ) {
        let (gamma, underflow) =
            dot_product_error_parameters(left.ncols()).expect("small test dot product");
        let computed = left * right;
        let magnitude_upper = positive_product_upper(
            &left.map(|value| value.abs()),
            &right.map(|value| value.abs()),
            gamma,
            underflow,
        )
        .expect("finite positive product");

        for row in 0..left.nrows() {
            for col in 0..right.ncols() {
                let target = target(row, col);
                let exact = (0..left.ncols())
                    .map(|mid| {
                        f64_to_rational(left[(row, mid)]) * f64_to_rational(right[(mid, col)])
                    })
                    .fold(BigRational::zero(), |sum, term| sum + term)
                    - f64_to_rational(target);
                let computed_residual = computed[(row, col)] - target;
                let error = (exact - f64_to_rational(computed_residual)).abs();
                let augmented_magnitude = add_up(magnitude_upper[(row, col)], target.abs());
                let error_upper = add_up(mul_up(gamma, augmented_magnitude), underflow);
                assert!(
                    error <= f64_to_rational(error_upper),
                    "augmented entry ({row},{col}) escaped the rounding envelope"
                );
            }
        }
    }
}

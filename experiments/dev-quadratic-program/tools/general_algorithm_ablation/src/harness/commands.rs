// ── Command-line packets and cohorts ─────────────────────────────────────

#[derive(Clone, Copy)]
pub enum NamedVariant {
    LegacySymmetricEigen,
    UncheckedLu,
    EmpiricalInverse,
    VerifiedScalarLuEveryWord,
    VerifiedScalarLbltPruned,
    VerifiedBatchedLbltPruned,
    VerifiedNormwiseLbltPruned,
    VerifiedHybridLbltPruned,
    EmpiricalLbltPruned,
}

impl NamedVariant {
    fn id(self) -> &'static str {
        match self {
            Self::LegacySymmetricEigen => "legacy_symmetric_eigen",
            Self::UncheckedLu => "unchecked_lu",
            Self::EmpiricalInverse => "empirical_inverse",
            Self::VerifiedScalarLuEveryWord => "verified_scalar_lu_every_word",
            Self::VerifiedScalarLbltPruned => "verified_scalar_lblt_pruned",
            Self::VerifiedBatchedLbltPruned => "verified_batched_lblt_pruned",
            Self::VerifiedNormwiseLbltPruned => "verified_normwise_lblt_pruned",
            Self::VerifiedHybridLbltPruned => "verified_hybrid_lblt_pruned",
            Self::EmpiricalLbltPruned => "empirical_lblt_pruned",
        }
    }

    fn numerical_status(self) -> &'static str {
        match self {
            Self::LegacySymmetricEigen => "known_unsound_scale_dependent",
            Self::UncheckedLu => "unchecked",
            Self::EmpiricalInverse | Self::EmpiricalLbltPruned => {
                "heuristic_wrong_determinate_decisions_possible"
            }
            Self::VerifiedScalarLuEveryWord
            | Self::VerifiedScalarLbltPruned
            | Self::VerifiedBatchedLbltPruned
            | Self::VerifiedNormwiseLbltPruned
            | Self::VerifiedHybridLbltPruned => "certified_with_exact_fallback",
        }
    }

    fn curvature_status(self) -> &'static str {
        match self {
            Self::LegacySymmetricEigen
            | Self::UncheckedLu
            | Self::EmpiricalInverse
            | Self::VerifiedScalarLuEveryWord => "not_checked",
            Self::VerifiedScalarLbltPruned
            | Self::VerifiedBatchedLbltPruned
            | Self::VerifiedNormwiseLbltPruned
            | Self::VerifiedHybridLbltPruned
            | Self::EmpiricalLbltPruned => "certified_obstruction_and_cyclic_inheritance",
        }
    }

    fn production(self) -> bool {
        matches!(self, Self::VerifiedHybridLbltPruned)
    }
}

fn standard_case_words(long_words_only: bool) -> Vec<(String, Vec<Vector4<f64>>, Vec<Vec<usize>>)> {
    generated_f64_cases(1, DEFAULT_SEED)
        .into_iter()
        .filter(|case| case.family == "generated_random_f64")
        .map(|case| {
            let exact = exact_binary64_dual_vertex_arrays(&case.dual_vertices);
            let transition = try_exact_binary64_transition_matrix_assuming_origin_interior(&exact)
                .expect("generated case has exact transition graph");
            let mut words = SimpleDirectedCyclesCanonical::new(&transition).collect::<Vec<_>>();
            if long_words_only {
                words.retain(|word| word.len() >= 5);
            }
            (case.source_id, case.dual_vertices, words)
        })
        .collect()
}

/// Run one stable general-QP algorithm on the standard matched long-word
/// cohort. This intentionally accepts no tuning flags: the executable name
/// identifies the algorithm, and every variant uses the same cases and timing
/// protocol.
pub fn run_named_variant(variant: NamedVariant) {
    const ROUNDS: usize = 9;
    let cases = standard_case_words(true);

    println!("algorithm.id={}", variant.id());
    println!("algorithm.numerics={}", variant.numerical_status());
    println!("algorithm.curvature={}", variant.curvature_status());
    println!("algorithm.production={}", variant.production());
    println!("algorithm.cohort=generated_random_f64_F5_F12_long_words");
    println!("algorithm.cases={}", cases.len());

    match variant {
        NamedVariant::LegacySymmetricEigen => {
            let result = benchmark_baseline(&cases, BaselineKind::CurrentEigen);
            print_baseline(variant.id(), &result);
        }
        NamedVariant::UncheckedLu => {
            let result = benchmark_baseline(&cases, BaselineKind::RawDirect);
            print_baseline(variant.id(), &result);
        }
        NamedVariant::EmpiricalInverse => {
            std::hint::black_box(run_empirical_inverse_guard(&cases));
            let samples = (0..ROUNDS)
                .map(|_| run_empirical_inverse_guard(&cases))
                .collect::<Vec<_>>();
            print_empirical_benchmark(variant.id(), &samples);
        }
        _ => {
            let (cutoff, factor, guard) = match variant {
                NamedVariant::VerifiedScalarLuEveryWord => {
                    (None, FactorKind::Lu, GuardKind::OutwardCertified)
                }
                NamedVariant::VerifiedScalarLbltPruned => (
                    Some(usize::MAX),
                    FactorKind::Lblt,
                    GuardKind::OutwardCertified,
                ),
                NamedVariant::VerifiedBatchedLbltPruned => (
                    Some(usize::MAX),
                    FactorKind::Lblt,
                    GuardKind::BatchedAnalyticEnvelope,
                ),
                NamedVariant::VerifiedNormwiseLbltPruned => (
                    Some(usize::MAX),
                    FactorKind::Lblt,
                    GuardKind::NormwiseAnalyticEnvelope,
                ),
                NamedVariant::VerifiedHybridLbltPruned => (
                    Some(usize::MAX),
                    FactorKind::Lblt,
                    GuardKind::HybridAnalyticEnvelope,
                ),
                NamedVariant::EmpiricalLbltPruned => (
                    Some(usize::MAX),
                    FactorKind::Lblt,
                    GuardKind::EmpiricalThenExact,
                ),
                NamedVariant::LegacySymmetricEigen
                | NamedVariant::UncheckedLu
                | NamedVariant::EmpiricalInverse => unreachable!(),
            };
            std::hint::black_box(run_route_with_guard(&cases, cutoff, factor, guard));
            let samples = (0..ROUNDS)
                .map(|_| run_route_with_guard(&cases, cutoff, factor, guard).stats)
                .collect::<Vec<_>>();
            print_slowdown_route_benchmark(variant.id(), &samples);
        }
    }
}

pub fn run_algorithm_comparison() {
    run_uncertified_baselines(&standard_case_words(true));
    run_slowdown_ablation(&standard_case_words(false));
}

pub fn run_selected_verification_packet() {
    let cases = standard_case_words(false);
    let products = product_case_words();
    run_selected_production_correspondence(&cases);
    run_exact_route_agreement_audit(
        "general_hybrid",
        &cases,
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
}

pub fn run_selected_numerics_packet() {
    let cases = standard_case_words(false);
    let products = product_case_words();
    run_hybrid_envelope_audit(&cases);
    run_empirical_predicate_audit(&cases);
    run_product_predicate_audit("hybrid", &products, GuardKind::HybridAnalyticEnvelope);
    run_product_predicate_audit("empirical", &products, GuardKind::EmpiricalThenExact);
    run_known_negative_control_audit("hybrid", GuardKind::HybridAnalyticEnvelope);
    run_known_negative_control_audit("empirical", GuardKind::EmpiricalThenExact);
}

pub fn run_general_end_to_end_profile() {
    let inputs = generated_f64_cases(1, DEFAULT_SEED)
        .into_iter()
        .filter(|case| case.family == "generated_random_f64")
        .map(|case| (case.source_id, case.dual_vertices))
        .collect::<Vec<_>>();
    run_end_to_end_profile(&inputs);
}

pub fn run_legacy_product_billiard_kkt_profile() {
    println!("algorithm.id=product_legacy_billiard_kkt");
    println!("algorithm.numerics=known_unsound_scale_dependent");
    println!("algorithm.production=false");
    run_existing_product_route_benchmark(&product_case_words());
}

pub fn run_cli() {
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
    if arguments
        .iter()
        .any(|argument| argument == "--end-to-end-profile")
    {
        run_general_end_to_end_profile();
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
        run_selected_verification_packet();
        return;
    }
    if std::env::args().any(|argument| argument == "--numerics-packet") {
        run_selected_numerics_packet();
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
        run_algorithm_comparison();
        return;
    }
    if std::env::args().any(|argument| argument == "--aggregation-only") {
        run_exact_aggregation_audit(&case_words);
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
        let geometry = checked_production_geometry(duals);
        let production =
            general_capacity(&geometry).expect("production selected route returns capacity bounds");
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
    print_end_to_end_samples("empirical_inverse", &empirical);
    print_end_to_end_samples("verified_batched_lblt_pruned", &batched);
    print_end_to_end_samples("verified_hybrid_lblt_pruned", &hybrid);
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

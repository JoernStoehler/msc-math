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

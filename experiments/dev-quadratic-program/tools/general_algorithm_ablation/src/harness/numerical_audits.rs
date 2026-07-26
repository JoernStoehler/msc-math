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

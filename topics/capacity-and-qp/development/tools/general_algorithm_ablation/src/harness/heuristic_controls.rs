// ── Historical heuristic controls ────────────────────────────────────────

fn run_uncertified_baselines(cases: &[(String, Vec<Vector4<f64>>, Vec<Vec<usize>>)]) {
    let eig = benchmark_baseline(cases, BaselineKind::CurrentEigen);
    let raw = benchmark_baseline(cases, BaselineKind::RawDirect);
    print_baseline("legacy_symmetric_eigen", &eig);
    print_baseline("unchecked_lu", &raw);
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
    std::hint::black_box(run_route(&long_cases, Some(usize::MAX), FactorKind::Lblt));
    std::hint::black_box(run_route_with_guard(
        &long_cases,
        Some(usize::MAX),
        FactorKind::Lblt,
        GuardKind::BatchedAnalyticEnvelope,
    ));
    std::hint::black_box(run_route_with_guard(
        &long_cases,
        Some(usize::MAX),
        FactorKind::Lblt,
        GuardKind::NormwiseAnalyticEnvelope,
    ));
    std::hint::black_box(run_route_with_guard(
        &long_cases,
        Some(usize::MAX),
        FactorKind::Lblt,
        GuardKind::HybridAnalyticEnvelope,
    ));
    std::hint::black_box(run_route_with_guard(
        &long_cases,
        Some(usize::MAX),
        FactorKind::Lblt,
        GuardKind::EmpiricalThenExact,
    ));

    let mut empirical = Vec::with_capacity(ROUNDS);
    let mut scalar_lu = Vec::with_capacity(ROUNDS);
    let mut scalar_lblt = Vec::with_capacity(ROUNDS);
    let mut batched = Vec::with_capacity(ROUNDS);
    let mut normwise = Vec::with_capacity(ROUNDS);
    let mut hybrid = Vec::with_capacity(ROUNDS);
    let mut pruned_empirical = Vec::with_capacity(ROUNDS);
    for round in 0..ROUNDS {
        for offset in 0..7 {
            match (round + offset) % 7 {
                0 => empirical.push(run_empirical_inverse_guard(&long_cases)),
                1 => scalar_lu.push(run_route(&long_cases, None, FactorKind::Lu).stats),
                2 => scalar_lblt
                    .push(run_route(&long_cases, Some(usize::MAX), FactorKind::Lblt).stats),
                3 => batched.push(
                    run_route_with_guard(
                        &long_cases,
                        Some(usize::MAX),
                        FactorKind::Lblt,
                        GuardKind::BatchedAnalyticEnvelope,
                    )
                    .stats,
                ),
                4 => normwise.push(
                    run_route_with_guard(
                        &long_cases,
                        Some(usize::MAX),
                        FactorKind::Lblt,
                        GuardKind::NormwiseAnalyticEnvelope,
                    )
                    .stats,
                ),
                5 => hybrid.push(
                    run_route_with_guard(
                        &long_cases,
                        Some(usize::MAX),
                        FactorKind::Lblt,
                        GuardKind::HybridAnalyticEnvelope,
                    )
                    .stats,
                ),
                6 => pruned_empirical.push(
                    run_route_with_guard(
                        &long_cases,
                        Some(usize::MAX),
                        FactorKind::Lblt,
                        GuardKind::EmpiricalThenExact,
                    )
                    .stats,
                ),
                _ => unreachable!(),
            }
        }
    }

    print_empirical_benchmark("empirical_inverse", &empirical);
    print_slowdown_route_benchmark("verified_scalar_lu_every_word", &scalar_lu);
    print_slowdown_route_benchmark("verified_scalar_lblt_pruned", &scalar_lblt);
    print_slowdown_route_benchmark("verified_batched_lblt_pruned", &batched);
    print_slowdown_route_benchmark("verified_normwise_lblt_pruned", &normwise);
    print_slowdown_route_benchmark("verified_hybrid_lblt_pruned", &hybrid);
    print_slowdown_route_benchmark("empirical_lblt_pruned", &pruned_empirical);
    let empirical_ms = median_duration_ms(empirical.iter().map(|stats| stats.elapsed));
    let scalar_lu_ms = median_duration_ms(scalar_lu.iter().map(|stats| stats.elapsed));
    let scalar_lblt_ms = median_duration_ms(scalar_lblt.iter().map(|stats| stats.elapsed));
    let batched_ms = median_duration_ms(batched.iter().map(|stats| stats.elapsed));
    let normwise_ms = median_duration_ms(normwise.iter().map(|stats| stats.elapsed));
    let hybrid_ms = median_duration_ms(hybrid.iter().map(|stats| stats.elapsed));
    let pruned_empirical_ms =
        median_duration_ms(pruned_empirical.iter().map(|stats| stats.elapsed));
    println!(
        "slowdown.verified_scalar_lu_over_empirical_inverse={:.6}",
        scalar_lu_ms / empirical_ms
    );
    println!(
        "slowdown.verified_scalar_lblt_over_empirical_inverse={:.6}",
        scalar_lblt_ms / empirical_ms
    );
    println!(
        "slowdown.curvature_pruning_speedup_scalar={:.6}",
        scalar_lu_ms / scalar_lblt_ms
    );
    println!(
        "slowdown.verified_batched_speedup_vs_verified_scalar={:.6}",
        scalar_lblt_ms / batched_ms
    );
    println!(
        "slowdown.verified_normwise_speedup_vs_verified_batched={:.6}",
        batched_ms / normwise_ms
    );
    println!(
        "slowdown.verified_normwise_speedup_vs_empirical_inverse={:.6}",
        empirical_ms / normwise_ms
    );
    println!(
        "slowdown.verified_hybrid_speedup_vs_empirical_inverse={:.6}",
        empirical_ms / hybrid_ms
    );
    println!(
        "slowdown.pruned_empirical_speedup_vs_empirical_inverse={:.6}",
        empirical_ms / pruned_empirical_ms
    );
    println!(
        "slowdown.pruned_empirical_speedup_vs_verified_scalar_lblt={:.6}",
        scalar_lblt_ms / pruned_empirical_ms
    );
}

// ── Benchmarks and ablations ─────────────────────────────────────────────

fn print_empirical_benchmark(label: &str, samples: &[EmpiricalGuardStats]) {
    let representative = &samples[0];
    let median_ms = median_duration_ms(samples.iter().map(|stats| stats.elapsed));
    println!("slowdown.route={label}");
    println!("slowdown.rounds={}", samples.len());
    println!("slowdown.words={}", representative.words);
    println!("slowdown.accepted={}", representative.accepted);
    println!("slowdown.rejected={}", representative.rejected);
    println!("slowdown.indeterminate={}", representative.indeterminate);
    println!("slowdown.best_action={:?}", representative.best_action);
    println!("slowdown.median_ms={median_ms:.6}");
    println!(
        "slowdown.per_enumerated_word_us={:.6}",
        median_ms * 1e3 / representative.words as f64
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
    let median_ms = median_duration_ms(samples.iter().map(|stats| stats.elapsed));
    println!("slowdown.route={label}");
    println!("slowdown.rounds={}", samples.len());
    println!("slowdown.words={}", representative.words);
    println!("slowdown.accepted={}", representative.accepted);
    println!("slowdown.rejected={}", representative.rejected);
    println!(
        "slowdown.inherited_rejections={}",
        representative.inherited_rejections
    );
    println!(
        "slowdown.direct_obstructions={}",
        representative.direct_obstructions
    );
    println!(
        "slowdown.lblt_factorizations={}",
        representative.lblt_factorizations
    );
    println!(
        "slowdown.lu_factorizations={}",
        representative.lu_factorizations
    );
    println!(
        "slowdown.guarded_decisions={}",
        representative.guarded_decisions
    );
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
    println!("slowdown.median_ms={median_ms:.6}");
    println!(
        "slowdown.per_enumerated_word_us={:.6}",
        median_ms * 1e3 / representative.words as f64
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
    println!("baseline.rounds=7");
    println!("baseline.words={}", result.words);
    println!(
        "baseline.elapsed_ms={:.6}",
        result.elapsed.as_secs_f64() * 1e3
    );
    println!(
        "baseline.per_enumerated_word_us={:.6}",
        result.elapsed.as_secs_f64() * 1e6 / result.words as f64
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

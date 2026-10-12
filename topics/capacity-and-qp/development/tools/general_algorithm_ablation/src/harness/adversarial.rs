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

use equal_budget_product_search::cem::{
    complete_generation, completed_generation_record, construct_generation, CemConstructed,
    CemDistribution, CemScoredCandidate,
};
use equal_budget_product_search::chart::{iid_base_candidate_attempt, ProductChart};
use equal_budget_product_search::evaluator::{
    ArmEvaluator, CacheExportRow, QueryOutcome, SysComputationOracle, TargetEvaluationRow,
};
use equal_budget_product_search::local::{
    multistart_branch_local_phase0, ChargedEvaluation, EvaluationResult, LocalProposal,
    LocalSearchEngine, ScoredState, TargetBudgetExhausted, TrajectoryStop,
};
use equal_budget_product_search::model::{
    candidate_id, Arm, CandidateIdentity, ProposalMeta, ProposalRole, PACKET_VERSION,
};
use exp_sys_landscape::{
    ascent_direction, compute_step_bound, ActiveSysState, AscentMode, ExpensiveComputationCache,
    SysLandscapePolytopeCache,
};
use nalgebra::Vector4;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use serde::Serialize;
use std::env;
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Instant;
use symplectic::classify_facets_from_dual_vertices;
use symplectic::geom::known_polytopes;

const MASTER_SEEDS: [u64; 3] = [202607110001, 202607110002, 202607110003];

#[derive(Serialize)]
struct LineageRow {
    candidate_id: String,
    parent_kind: &'static str,
    parent_candidate_id: Option<String>,
    elite_set_id: Option<String>,
}

#[derive(Clone, Debug, Serialize)]
struct ConstructionRejectionRow {
    candidate_id: String,
    arm: Arm,
    replicate: usize,
    generation: Option<usize>,
    trajectory: Option<usize>,
    iteration: Option<usize>,
    proposal_index: usize,
    construction_attempt: usize,
    construction_sequence_index: usize,
    role: ProposalRole,
    reason: String,
}

#[derive(Clone, Debug, Serialize)]
struct ArmRunRow {
    arm: Arm,
    replicate: usize,
    target_attempts: usize,
    successful_new_computations: usize,
    cache_hits: usize,
    failed_new_computations: usize,
    construction_attempts: usize,
    construction_rejections: usize,
    target_wall_time_ms: f64,
    total_wall_time_ms: f64,
    status: String,
}

#[derive(Clone, Debug, Serialize)]
struct LocalTrajectoryRow {
    arm: Arm,
    replicate: usize,
    trajectory: usize,
    start_candidate_id: String,
    final_candidate_id: String,
    start_sys: f64,
    final_sys: f64,
    accepted_iterations: usize,
    stop: &'static str,
    complete: bool,
}

#[derive(Clone, Debug)]
struct StopEvent {
    classification: &'static str,
    message: String,
}

struct ArmOutput<O = ExpensiveComputationCache> {
    evaluator: ArmEvaluator<O>,
    rejections: Vec<ConstructionRejectionRow>,
    total_wall_time_ms: f64,
    stop: Option<StopEvent>,
}

struct CemOutput<O = ExpensiveComputationCache> {
    arm: ArmOutput<O>,
    records: Vec<serde_json::Value>,
}

struct LocalOutput<O = ExpensiveComputationCache> {
    arm: ArmOutput<O>,
    trajectories: Vec<LocalTrajectoryRow>,
}

#[derive(Serialize)]
struct RunStatus<'a> {
    packet_version: &'a str,
    complete: bool,
    charged_target_attempts: usize,
    overall_wall_time_ms: f64,
    stop: Option<RunStatusStop<'a>>,
}

#[derive(Serialize)]
struct RunStatusStop<'a> {
    classification: &'a str,
    message: &'a str,
}

#[derive(Default)]
struct PacketArtifacts {
    targets: Vec<TargetEvaluationRow>,
    cache: Vec<CacheExportRow>,
    cem: Vec<serde_json::Value>,
    rejections: Vec<ConstructionRejectionRow>,
    arm_runs: Vec<ArmRunRow>,
    local_trajectories: Vec<LocalTrajectoryRow>,
}

impl PacketArtifacts {
    fn absorb<O: SysComputationOracle>(&mut self, evaluator: &mut ArmEvaluator<O>) {
        self.targets.extend(evaluator.drain_target_rows());
        self.cache.extend(evaluator.drain_cache_rows());
    }

    fn lineages(&self) -> Vec<LineageRow> {
        self.targets
            .iter()
            .map(|row| LineageRow {
                candidate_id: row.candidate_id.clone(),
                parent_kind: if row.parent_candidate_id.is_some() {
                    "candidate"
                } else if row.elite_set_id.is_some() {
                    "distribution"
                } else {
                    "none"
                },
                parent_candidate_id: row.parent_candidate_id.clone(),
                elite_set_id: row.elite_set_id.clone(),
            })
            .collect()
    }

    fn write(&self, directory: &Path) -> Result<(), String> {
        fs::create_dir_all(directory).map_err(|error| error.to_string())?;
        write_jsonl(directory.join("target-evaluations.jsonl"), &self.targets)?;
        write_jsonl(
            directory.join("expensive-computation-cache.jsonl"),
            &self.cache,
        )?;
        write_jsonl(directory.join("cem-generations.jsonl"), &self.cem)?;
        write_jsonl(directory.join("lineages.jsonl"), &self.lineages())?;
        write_jsonl(
            directory.join("construction-rejections.jsonl"),
            &self.rejections,
        )?;
        write_jsonl(directory.join("arm-runs.jsonl"), &self.arm_runs)?;
        write_jsonl(
            directory.join("local-trajectories.jsonl"),
            &self.local_trajectories,
        )?;
        Ok(())
    }
}

fn arm_run_row<O: SysComputationOracle>(
    evaluator: &ArmEvaluator<O>,
    rejection_count: usize,
    total_wall_time_ms: f64,
    status: impl Into<String>,
) -> ArmRunRow {
    use equal_budget_product_search::model::CacheStatus;
    let rows = evaluator.target_rows();
    ArmRunRow {
        arm: evaluator.arm(),
        replicate: evaluator.replicate(),
        target_attempts: rows.len(),
        successful_new_computations: rows
            .iter()
            .filter(|row| row.cache_status == CacheStatus::Miss)
            .count(),
        cache_hits: rows
            .iter()
            .filter(|row| row.cache_status == CacheStatus::Hit)
            .count(),
        failed_new_computations: rows
            .iter()
            .filter(|row| row.cache_status == CacheStatus::FailedMiss)
            .count(),
        construction_attempts: rows.len() + rejection_count,
        construction_rejections: rejection_count,
        // Match Python 3.12's accurate `sum`, which the artifact verifier uses
        // to reconcile this field with serialized target rows.
        target_wall_time_ms: accurate_sum(rows.iter().map(|row| row.wall_time_ms)),
        total_wall_time_ms,
        status: status.into(),
    }
}

/// Faithful finite-input compensated summation (the partials algorithm used by
/// `math.fsum`). Timing rows are nonnegative and finite by construction.
fn accurate_sum(values: impl IntoIterator<Item = f64>) -> f64 {
    let mut partials = Vec::<f64>::new();
    for mut value in values {
        let mut write = 0;
        for index in 0..partials.len() {
            let mut other = partials[index];
            if value.abs() < other.abs() {
                std::mem::swap(&mut value, &mut other);
            }
            let high = value + other;
            let low = other - (high - value);
            if low != 0.0 {
                partials[write] = low;
                write += 1;
            }
            value = high;
        }
        partials.truncate(write);
        partials.push(value);
    }
    partials.into_iter().rev().sum()
}

fn write_jsonl<T: Serialize>(path: PathBuf, rows: &[T]) -> Result<(), String> {
    let file = File::create(&path).map_err(|error| format!("create {path:?}: {error}"))?;
    let mut writer = BufWriter::new(file);
    for row in rows {
        serde_json::to_writer(&mut writer, row).map_err(|error| error.to_string())?;
        writer.write_all(b"\n").map_err(|error| error.to_string())?;
    }
    writer.flush().map_err(|error| error.to_string())
}

fn proposal_meta(
    identity: CandidateIdentity<'_>,
    role: ProposalRole,
    parent_candidate_id: Option<String>,
    construction_rejections_before: usize,
    construction_sequence_index: usize,
) -> ProposalMeta {
    ProposalMeta {
        candidate_id: candidate_id(&identity),
        arm: identity.arm,
        replicate: identity.replicate,
        generation: identity.generation,
        trajectory: identity.trajectory,
        iteration: identity.iteration,
        proposal_index: identity.proposal_index,
        construction_attempt: identity.construction_attempt,
        construction_sequence_index,
        construction_rejections_before,
        role,
        parent_candidate_id,
        elite_set_id: None,
    }
}

fn check_unexpected_positive<O: SysComputationOracle>(
    outcome: &QueryOutcome,
    evaluator: &ArmEvaluator<O>,
    hko_key: &str,
) -> Option<(usize, String, f64)> {
    let QueryOutcome::Success {
        row_index,
        computation,
    } = outcome
    else {
        return None;
    };
    if computation.sys <= 1.0 {
        return None;
    }
    let row = &evaluator.target_rows()[*row_index];
    let key = row.polytope_key.clone().expect("successful row key");
    (key != hko_key).then_some((*row_index, key, computation.sys))
}

fn hko_polytope_key() -> String {
    let fixture = known_polytopes::hko_pentagon();
    let polytope = SysLandscapePolytopeCache::from_rational_parts(
        fixture.dual_vertices.clone(),
        fixture.vertices.clone(),
    )
    .expect("known HKO fixture reconstructs");
    exp_sys_landscape::polytope_key(&polytope)
}

fn run_iid(seed: u64, replicate: usize, hko_key: &str) -> ArmOutput {
    run_iid_with_oracle(seed, replicate, hko_key, ExpensiveComputationCache::empty())
}

fn run_iid_with_oracle<O: SysComputationOracle>(
    seed: u64,
    replicate: usize,
    hko_key: &str,
    oracle: O,
) -> ArmOutput<O> {
    let started = Instant::now();
    let mut evaluator = ArmEvaluator::new(Arm::Iid, replicate, oracle);
    let mut rejections = Vec::new();
    let mut construction_sequence_index = 0usize;
    for base_index in 0..256 {
        let mut rejected = 0usize;
        let (candidate, construction_attempt) = loop {
            if rejected == 10_000 {
                return ArmOutput {
                    evaluator,
                    rejections,
                    total_wall_time_ms: started.elapsed().as_secs_f64() * 1_000.0,
                    stop: Some(StopEvent {
                        classification: "construction_incomplete",
                        message: format!("IID construction did not accept base index {base_index}"),
                    }),
                };
            }
            match iid_base_candidate_attempt(seed, replicate, base_index, rejected) {
                Ok(candidate) => break (candidate, rejected),
                Err(reason) => {
                    let identity = CandidateIdentity {
                        packet_version: PACKET_VERSION,
                        master_seed: seed,
                        replicate,
                        arm: Arm::Iid,
                        generation: None,
                        trajectory: None,
                        iteration: None,
                        proposal_index: base_index,
                        construction_attempt: rejected,
                    };
                    rejections.push(ConstructionRejectionRow {
                        candidate_id: candidate_id(&identity),
                        arm: Arm::Iid,
                        replicate,
                        generation: None,
                        trajectory: None,
                        iteration: None,
                        proposal_index: base_index,
                        construction_attempt: rejected,
                        construction_sequence_index,
                        role: ProposalRole::Iid,
                        reason: format!("{reason:?}"),
                    });
                    construction_sequence_index += 1;
                    rejected += 1;
                    evaluator.record_construction_rejection();
                }
            }
        };
        let meta = proposal_meta(
            CandidateIdentity {
                packet_version: PACKET_VERSION,
                master_seed: seed,
                replicate,
                arm: Arm::Iid,
                generation: None,
                trajectory: None,
                iteration: None,
                proposal_index: base_index,
                construction_attempt,
            },
            ProposalRole::Iid,
            None,
            rejected,
            construction_sequence_index,
        );
        construction_sequence_index += 1;
        let outcome = evaluator.evaluate(meta, &candidate.polytope);
        if let Some((_, key, sys)) = check_unexpected_positive(&outcome, &evaluator, hko_key) {
            return ArmOutput {
                evaluator,
                rejections,
                total_wall_time_ms: started.elapsed().as_secs_f64() * 1_000.0,
                stop: Some(StopEvent {
                    classification: "new_or_materially_uncertain_relative_to_exact_HKO_control",
                    message: format!("new_or_uncertain sys={sys:.17} key={key}"),
                }),
            };
        }
    }
    ArmOutput {
        evaluator,
        rejections,
        total_wall_time_ms: started.elapsed().as_secs_f64() * 1_000.0,
        stop: None,
    }
}

fn cem_rng(seed: u64, replicate: usize) -> ChaCha8Rng {
    let mut material = [0u8; 16];
    material[..8].copy_from_slice(&seed.to_le_bytes());
    material[8..].copy_from_slice(&(replicate as u64).to_le_bytes());
    ChaCha8Rng::from_seed(blake3::derive_key("s0-diagonal-cem-stream-v1", &material))
}

fn run_cem(seed: u64, replicate: usize, hko_key: &str) -> CemOutput {
    run_cem_with_oracle(seed, replicate, hko_key, ExpensiveComputationCache::empty())
}

fn run_cem_with_oracle<O: SysComputationOracle>(
    seed: u64,
    replicate: usize,
    hko_key: &str,
    oracle: O,
) -> CemOutput<O> {
    let started = Instant::now();
    let mut evaluator = ArmEvaluator::new(Arm::DiagonalCem, replicate, oracle);
    let mut rng = cem_rng(seed, replicate);
    let mut records = Vec::new();
    let mut rejections = Vec::new();
    let mut sampling_distribution: Option<CemDistribution> = None;
    let mut parent_elite_set_id: Option<String> = None;
    let mut construction_sequence_offset = 0usize;

    for generation in 0..4 {
        let mut retry_for_base_index = [0usize; 64];
        let batch = construct_generation(
            seed,
            replicate,
            generation,
            sampling_distribution.as_ref(),
            parent_elite_set_id.as_deref(),
            &mut rng,
            |request| {
                let candidate = if generation == 0 {
                    let retry = retry_for_base_index[request.proposal_index];
                    retry_for_base_index[request.proposal_index] += 1;
                    iid_base_candidate_attempt(seed, replicate, request.proposal_index, retry)?
                } else {
                    ProductChart::from_continuous_coordinates(
                        request
                            .proposed_coordinates
                            .expect("later CEM generation has sampled coordinates"),
                        false,
                    )
                    .reconstruct_candidate()?
                };
                let chart = ProductChart::from_factors(
                    &candidate.factors.q_normals,
                    &candidate.factors.q_heights,
                    &candidate.factors.p_normals,
                    &candidate.factors.p_heights,
                )
                .expect("constructed product has a chart");
                Ok::<_, equal_budget_product_search::chart::ConstructionRejection>(CemConstructed {
                    coordinates: chart.continuous_coordinates(),
                    payload: candidate,
                })
            },
        );
        // `construct_generation` deliberately has no artifact dependency.  Its
        // rejection vector is ordered by construction attempt, so replay the
        // failed identities here without changing the CEM state machine.
        let mut rejection_cursor = 0usize;
        for construction_attempt in 0..batch.construction_attempts {
            let successful = batch
                .proposals
                .iter()
                .any(|proposal| proposal.meta.construction_attempt == construction_attempt);
            if successful {
                continue;
            }
            let reason = &batch.rejections[rejection_cursor];
            rejection_cursor += 1;
            let proposal_index = batch
                .proposals
                .iter()
                .filter(|proposal| proposal.meta.construction_attempt < construction_attempt)
                .count();
            let identity = CandidateIdentity {
                packet_version: PACKET_VERSION,
                master_seed: seed,
                replicate,
                arm: Arm::DiagonalCem,
                generation: Some(generation),
                trajectory: None,
                iteration: None,
                proposal_index,
                construction_attempt,
            };
            rejections.push(ConstructionRejectionRow {
                candidate_id: candidate_id(&identity),
                arm: Arm::DiagonalCem,
                replicate,
                generation: Some(generation),
                trajectory: None,
                iteration: None,
                proposal_index,
                construction_attempt,
                construction_sequence_index: construction_sequence_offset + construction_attempt,
                role: ProposalRole::CemPopulation,
                reason: format!("{reason:?}"),
            });
        }
        debug_assert_eq!(rejection_cursor, batch.rejections.len());
        for _ in 0..batch.construction_rejections {
            evaluator.record_construction_rejection();
        }
        if !batch.complete {
            records.push(serde_json::json!({
                "replicate": replicate,
                "generation": generation,
                "complete": false,
                "reason": "construction_attempt_cap",
                "message": format!(
                    "CEM construction cap reached with {} valid candidates",
                    batch.proposals.len()
                ),
                "member_candidate_ids": batch.proposals.iter().map(|proposal| &proposal.meta.candidate_id).collect::<Vec<_>>(),
                "construction_attempts": batch.construction_attempts,
                "construction_rejections": batch.construction_rejections,
            }));
            return CemOutput {
                arm: ArmOutput {
                    evaluator,
                    rejections,
                    total_wall_time_ms: started.elapsed().as_secs_f64() * 1_000.0,
                    stop: Some(StopEvent {
                        classification: "cem_construction_attempt_cap",
                        message: format!(
                            "CEM replicate {replicate} generation {generation} hit 640-attempt construction cap"
                        ),
                    }),
                },
                records,
            };
        }

        let mut scored = Vec::with_capacity(64);
        for proposal in &batch.proposals {
            let mut meta = proposal.meta.clone();
            meta.construction_sequence_index += construction_sequence_offset;
            let outcome = evaluator.evaluate(meta, &proposal.payload.polytope);
            match &outcome {
                QueryOutcome::Success { computation, .. } => scored.push(CemScoredCandidate {
                    candidate_id: proposal.meta.candidate_id.clone(),
                    coordinates: proposal.coordinates,
                    sys: computation.sys,
                }),
                QueryOutcome::Failure { .. } => {
                    records.push(incomplete_cem_record(
                        replicate,
                        generation,
                        &batch,
                        "charged_target_failure",
                    ));
                    return CemOutput {
                        arm: ArmOutput {
                            evaluator,
                            rejections,
                            total_wall_time_ms: started.elapsed().as_secs_f64() * 1_000.0,
                            stop: Some(StopEvent {
                                classification: "cem_charged_target_failure",
                                message: format!(
                                    "CEM target failure in replicate {replicate} generation {generation}"
                                ),
                            }),
                        },
                        records,
                    };
                }
                QueryOutcome::Exhausted => {
                    records.push(incomplete_cem_record(
                        replicate,
                        generation,
                        &batch,
                        "target_budget_exhausted_before_generation_barrier",
                    ));
                    return CemOutput {
                        arm: ArmOutput {
                            evaluator,
                            rejections,
                            total_wall_time_ms: started.elapsed().as_secs_f64() * 1_000.0,
                            stop: Some(StopEvent {
                                classification: "cem_target_budget_exhausted",
                                message: format!(
                                    "CEM budget exhausted in replicate {replicate} generation {generation}"
                                ),
                            }),
                        },
                        records,
                    };
                }
            }
            if let Some((_, key, sys)) = check_unexpected_positive(&outcome, &evaluator, hko_key) {
                records.push(incomplete_cem_record(
                    replicate,
                    generation,
                    &batch,
                    "new_or_materially_uncertain_relative_to_exact_HKO_control",
                ));
                return CemOutput {
                    arm: ArmOutput {
                        evaluator,
                        rejections,
                        total_wall_time_ms: started.elapsed().as_secs_f64() * 1_000.0,
                        stop: Some(StopEvent {
                            classification:
                                "new_or_materially_uncertain_relative_to_exact_HKO_control",
                            message: format!("new_or_uncertain sys={sys:.17} key={key}"),
                        }),
                    },
                    records,
                };
            }
        }
        let completed = match complete_generation(sampling_distribution.as_ref(), &batch, &scored) {
            Ok(completed) => completed,
            Err(error) => {
                records.push(incomplete_cem_record(
                    replicate,
                    generation,
                    &batch,
                    "generation_barrier_error",
                ));
                return CemOutput {
                    arm: ArmOutput {
                        evaluator,
                        rejections,
                        total_wall_time_ms: started.elapsed().as_secs_f64() * 1_000.0,
                        stop: Some(StopEvent {
                            classification: "cem_generation_barrier_error",
                            message: format!("CEM generation barrier: {error:?}"),
                        }),
                    },
                    records,
                };
            }
        };
        let (record, _, elites) = match completed_generation_record(
            replicate,
            generation,
            &batch,
            &scored,
            completed.recorded_distribution,
            parent_elite_set_id.clone(),
        ) {
            Ok(record) => record,
            Err(error) => {
                records.push(incomplete_cem_record(
                    replicate,
                    generation,
                    &batch,
                    "generation_record_error",
                ));
                return CemOutput {
                    arm: ArmOutput {
                        evaluator,
                        rejections,
                        total_wall_time_ms: started.elapsed().as_secs_f64() * 1_000.0,
                        stop: Some(StopEvent {
                            classification: "cem_generation_record_error",
                            message: format!("CEM generation record: {error:?}"),
                        }),
                    },
                    records,
                };
            }
        };
        debug_assert_eq!(elites.len(), completed.elites.len());
        parent_elite_set_id = Some(record.elite_set_id.clone());
        sampling_distribution = Some(completed.next_distribution);
        records.push(serde_json::to_value(record).expect("CEM record serializes"));
        construction_sequence_offset += batch.construction_attempts;
    }
    CemOutput {
        arm: ArmOutput {
            evaluator,
            rejections,
            total_wall_time_ms: started.elapsed().as_secs_f64() * 1_000.0,
            stop: None,
        },
        records,
    }
}

fn incomplete_cem_record<T, E>(
    replicate: usize,
    generation: usize,
    batch: &equal_budget_product_search::cem::CemConstructionBatch<T, E>,
    reason: &'static str,
) -> serde_json::Value {
    serde_json::json!({
        "replicate": replicate,
        "generation": generation,
        "complete": false,
        "reason": reason,
        "member_candidate_ids": batch.proposals.iter().map(|proposal| &proposal.meta.candidate_id).collect::<Vec<_>>(),
        "construction_attempts": batch.construction_attempts,
        "construction_rejections": batch.construction_rejections,
    })
}

#[derive(Clone)]
struct LocalState {
    candidate_id: String,
    polytope: SysLandscapePolytopeCache,
    active: ActiveSysState,
    row_index: usize,
}

struct LocalEngine<'a, O = ExpensiveComputationCache> {
    seed: u64,
    replicate: usize,
    evaluator: ArmEvaluator<O>,
    hko_key: &'a str,
    stop: Option<StopEvent>,
    rejections_since_target: usize,
    rejections: Vec<ConstructionRejectionRow>,
    next_construction_sequence_index: usize,
}

impl<O: SysComputationOracle> LocalEngine<'_, O> {
    fn evaluate_candidate(
        &mut self,
        meta: ProposalMeta,
        polytope: SysLandscapePolytopeCache,
    ) -> EvaluationResult<LocalState, usize> {
        if self.stop.is_some() {
            return Err(TargetBudgetExhausted);
        }
        let outcome = self.evaluator.evaluate(meta.clone(), &polytope);
        if let Some((_, key, sys)) =
            check_unexpected_positive(&outcome, &self.evaluator, self.hko_key)
        {
            self.stop = Some(StopEvent {
                classification: "new_or_materially_uncertain_relative_to_exact_HKO_control",
                message: format!("new_or_uncertain sys={sys:.17} key={key}"),
            });
        }
        match outcome {
            QueryOutcome::Success {
                row_index,
                computation,
            } => Ok(ChargedEvaluation::Success {
                scored: ScoredState {
                    sys: computation.sys,
                    state: LocalState {
                        candidate_id: meta.candidate_id,
                        polytope,
                        active: ActiveSysState {
                            capacity: computation.capacity,
                            vol: computation.vol,
                            sys: computation.sys,
                        },
                        row_index,
                    },
                },
                observation: row_index,
            }),
            QueryOutcome::Failure { row_index } => Ok(ChargedEvaluation::Failure {
                observation: row_index,
            }),
            QueryOutcome::Exhausted => Err(TargetBudgetExhausted),
        }
    }
}

impl<O: SysComputationOracle> LocalSearchEngine for LocalEngine<'_, O> {
    type State = LocalState;
    type Direction = Vec<Vector4<f64>>;
    type Observation = usize;

    fn evaluate_start(
        &mut self,
        proposal: LocalProposal,
    ) -> EvaluationResult<Self::State, Self::Observation> {
        if self.stop.is_some() || self.evaluator.is_exhausted() {
            return Err(TargetBudgetExhausted);
        }
        let mut rejected = 0usize;
        let candidate = loop {
            if rejected == 10_000 {
                self.stop = Some(StopEvent {
                    classification: "local_construction_incomplete",
                    message: format!(
                        "local construction did not accept trajectory {}",
                        proposal.trajectory
                    ),
                });
                return Err(TargetBudgetExhausted);
            }
            match iid_base_candidate_attempt(
                self.seed,
                self.replicate,
                proposal.trajectory,
                rejected,
            ) {
                Ok(candidate) => break candidate,
                Err(reason) => {
                    let identity = CandidateIdentity {
                        packet_version: PACKET_VERSION,
                        master_seed: self.seed,
                        replicate: self.replicate,
                        arm: Arm::MultistartBranchLocalPhase0,
                        generation: None,
                        trajectory: Some(proposal.trajectory),
                        iteration: None,
                        proposal_index: proposal.proposal_index,
                        construction_attempt: rejected,
                    };
                    self.rejections.push(ConstructionRejectionRow {
                        candidate_id: candidate_id(&identity),
                        arm: Arm::MultistartBranchLocalPhase0,
                        replicate: self.replicate,
                        generation: None,
                        trajectory: Some(proposal.trajectory),
                        iteration: None,
                        proposal_index: proposal.proposal_index,
                        construction_attempt: rejected,
                        construction_sequence_index: self.next_construction_sequence_index,
                        role: proposal.role,
                        reason: format!("{reason:?}"),
                    });
                    self.next_construction_sequence_index += 1;
                    rejected += 1;
                    self.evaluator.record_construction_rejection();
                }
            }
        };
        let identity = CandidateIdentity {
            packet_version: PACKET_VERSION,
            master_seed: self.seed,
            replicate: self.replicate,
            arm: Arm::MultistartBranchLocalPhase0,
            generation: None,
            trajectory: Some(proposal.trajectory),
            iteration: None,
            proposal_index: proposal.proposal_index,
            construction_attempt: rejected,
        };
        let rejected = rejected + std::mem::take(&mut self.rejections_since_target);
        let meta = proposal_meta(
            identity,
            proposal.role,
            None,
            rejected,
            self.next_construction_sequence_index,
        );
        self.next_construction_sequence_index += 1;
        self.evaluate_candidate(meta, candidate.polytope)
    }

    fn ascent_direction(&mut self, current: &ScoredState<Self::State>) -> Option<Self::Direction> {
        let classification =
            classify_facets_from_dual_vertices(&current.state.polytope.dual_vertices_f64).ok()?;
        ascent_direction(
            &current.state.polytope,
            &current.state.active,
            AscentMode::LagrangianProduct {
                classification: &classification,
            },
        )
    }

    fn step_bound(
        &mut self,
        current: &ScoredState<Self::State>,
        direction: &Self::Direction,
    ) -> f64 {
        compute_step_bound(&current.state.polytope, direction)
    }

    fn evaluate_step(
        &mut self,
        current: &ScoredState<Self::State>,
        direction: &Self::Direction,
        proposal: LocalProposal,
    ) -> EvaluationResult<Self::State, Self::Observation> {
        if self.stop.is_some() || self.evaluator.is_exhausted() {
            return Err(TargetBudgetExhausted);
        }
        let duals = current
            .state
            .polytope
            .dual_vertices_f64
            .iter()
            .zip(direction)
            .map(|(dual, delta)| dual + proposal.step_size.expect("step proposal has size") * delta)
            .collect();
        let Some(polytope) = SysLandscapePolytopeCache::from_f64_dual_vertices(duals) else {
            let identity = CandidateIdentity {
                packet_version: PACKET_VERSION,
                master_seed: self.seed,
                replicate: self.replicate,
                arm: Arm::MultistartBranchLocalPhase0,
                generation: None,
                trajectory: Some(proposal.trajectory),
                iteration: proposal.iteration,
                proposal_index: proposal.proposal_index,
                construction_attempt: 0,
            };
            self.rejections.push(ConstructionRejectionRow {
                candidate_id: candidate_id(&identity),
                arm: Arm::MultistartBranchLocalPhase0,
                replicate: self.replicate,
                generation: None,
                trajectory: Some(proposal.trajectory),
                iteration: proposal.iteration,
                proposal_index: proposal.proposal_index,
                construction_attempt: 0,
                construction_sequence_index: self.next_construction_sequence_index,
                role: proposal.role,
                reason: "polytope_constructor_rejected".into(),
            });
            self.next_construction_sequence_index += 1;
            self.evaluator.record_construction_rejection();
            self.rejections_since_target += 1;
            return Ok(ChargedEvaluation::UnchargedConstructionRejection);
        };
        let identity = CandidateIdentity {
            packet_version: PACKET_VERSION,
            master_seed: self.seed,
            replicate: self.replicate,
            arm: Arm::MultistartBranchLocalPhase0,
            generation: None,
            trajectory: Some(proposal.trajectory),
            iteration: proposal.iteration,
            proposal_index: proposal.proposal_index,
            construction_attempt: 0,
        };
        let meta = proposal_meta(
            identity,
            proposal.role,
            Some(current.state.candidate_id.clone()),
            std::mem::take(&mut self.rejections_since_target),
            self.next_construction_sequence_index,
        );
        self.next_construction_sequence_index += 1;
        self.evaluate_candidate(meta, polytope)
    }

    fn accept_next_state(&mut self, next: &ScoredState<Self::State>) {
        self.evaluator
            .target_row_mut(next.state.row_index)
            .expect("accepted target row exists")
            .became_next_state = true;
    }
}

fn run_local(seed: u64, replicate: usize, hko_key: &str) -> LocalOutput {
    run_local_with_oracle(seed, replicate, hko_key, ExpensiveComputationCache::empty())
}

fn run_local_with_oracle<O: SysComputationOracle>(
    seed: u64,
    replicate: usize,
    hko_key: &str,
    oracle: O,
) -> LocalOutput<O> {
    let started = Instant::now();
    let mut engine = LocalEngine {
        seed,
        replicate,
        evaluator: ArmEvaluator::new(Arm::MultistartBranchLocalPhase0, replicate, oracle),
        hko_key,
        stop: None,
        rejections_since_target: 0,
        rejections: Vec::new(),
        next_construction_sequence_index: 0,
    };
    let result = multistart_branch_local_phase0(&mut engine);
    if engine.stop.is_none() && engine.evaluator.attempts_used() != 256 {
        engine.stop = Some(StopEvent {
            classification: "local_search_incomplete",
            message: format!(
                "local replicate {replicate} stopped at {} attempts ({:?})",
                engine.evaluator.attempts_used(),
                result.trajectories.last().map(|row| row.stop)
            ),
        });
    }
    let trajectories = result
        .trajectories
        .iter()
        .map(|trajectory| LocalTrajectoryRow {
            arm: Arm::MultistartBranchLocalPhase0,
            replicate,
            trajectory: trajectory.trajectory,
            start_candidate_id: trajectory.start.state.candidate_id.clone(),
            final_candidate_id: trajectory.final_state.state.candidate_id.clone(),
            start_sys: trajectory.start.sys,
            final_sys: trajectory.final_state.sys,
            accepted_iterations: trajectory.accepted_iterations,
            stop: trajectory_stop_name(trajectory.stop),
            complete: trajectory.stop != TrajectoryStop::IncompleteGrid,
        })
        .collect();
    LocalOutput {
        arm: ArmOutput {
            evaluator: engine.evaluator,
            rejections: engine.rejections,
            total_wall_time_ms: started.elapsed().as_secs_f64() * 1_000.0,
            stop: engine.stop,
        },
        trajectories,
    }
}

fn trajectory_stop_name(stop: TrajectoryStop) -> &'static str {
    match stop {
        TrajectoryStop::NoDirection => "no_direction",
        TrajectoryStop::InvalidStepBound => "invalid_step_bound",
        TrajectoryStop::NoImprovement => "no_improvement",
        TrajectoryStop::ImprovementBelowThreshold => "improvement_below_threshold",
        TrajectoryStop::IncompleteGrid => "incomplete_grid",
    }
}

fn write_stop(directory: &Path, stop: &StopEvent) -> Result<(), String> {
    let path = directory.join("stop-event.json");
    let value = serde_json::json!({
        "classification": stop.classification,
        "message": stop.message,
        "action": "fixed run stopped after row/cache flush"
    });
    fs::write(path, serde_json::to_vec_pretty(&value).unwrap()).map_err(|error| error.to_string())
}

fn write_run_status(
    directory: &Path,
    complete: bool,
    artifacts: &PacketArtifacts,
    overall_started: Instant,
    stop: Option<&StopEvent>,
) -> Result<(), String> {
    let status = RunStatus {
        packet_version: PACKET_VERSION,
        complete,
        charged_target_attempts: artifacts.targets.len(),
        overall_wall_time_ms: overall_started.elapsed().as_secs_f64() * 1_000.0,
        stop: stop.map(|stop| RunStatusStop {
            classification: stop.classification,
            message: &stop.message,
        }),
    };
    fs::write(
        directory.join("run-status.json"),
        serde_json::to_vec_pretty(&status).expect("run status serializes"),
    )
    .map_err(|error| error.to_string())
}

fn flush_incomplete(
    artifacts: &PacketArtifacts,
    directory: &Path,
    overall_started: Instant,
    stop: &StopEvent,
) -> Result<(), String> {
    artifacts.write(directory)?;
    write_run_status(directory, false, artifacts, overall_started, Some(stop))?;
    write_stop(directory, stop)
}

fn copy_frozen_config(directory: &Path) -> Result<PathBuf, String> {
    let source = Path::new(env!("CARGO_MANIFEST_DIR")).join("resolved-config.json");
    let destination = directory.join("resolved-config.json");
    fs::copy(&source, &destination)
        .map_err(|error| format!("copy frozen config {source:?} to {destination:?}: {error}"))?;
    Ok(source)
}

fn invoke_analyzer(directory: &Path, config: &Path) -> Result<(), String> {
    let script = Path::new(env!("CARGO_MANIFEST_DIR")).join("analyze.py");
    let help = Command::new("python3")
        .arg(&script)
        .arg("--help")
        .output()
        .map_err(|error| format!("launch analyzer help: {error}"))?;
    if !help.status.success() {
        return Err(format!(
            "analyzer help failed: {}",
            String::from_utf8_lossy(&help.stderr)
        ));
    }
    let mut command = Command::new("python3");
    command.arg(&script).arg("--artifacts").arg(directory);
    if String::from_utf8_lossy(&help.stdout).contains("--config") {
        command.arg("--config").arg(config);
    }
    let output = command
        .output()
        .map_err(|error| format!("launch analyzer: {error}"))?;
    if output.status.success() {
        Ok(())
    } else {
        Err(format!(
            "analyzer failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ))
    }
}

fn retain_arm<O: SysComputationOracle>(
    artifacts: &mut PacketArtifacts,
    mut output: ArmOutput<O>,
) -> Option<StopEvent> {
    let status = if output.stop.is_some() {
        "stopped"
    } else {
        "complete"
    };
    artifacts.arm_runs.push(arm_run_row(
        &output.evaluator,
        output.rejections.len(),
        output.total_wall_time_ms,
        status,
    ));
    artifacts.rejections.append(&mut output.rejections);
    artifacts.absorb(&mut output.evaluator);
    output.stop
}

fn main() -> Result<(), String> {
    let overall_started = Instant::now();
    let args = env::args().skip(1).collect::<Vec<_>>();
    let artifacts_dir = match args.as_slice() {
        [] => PathBuf::from("artifacts"),
        [flag, path] if flag == "--artifacts" => PathBuf::from(path),
        _ => return Err("usage: equal-budget-product-search [--artifacts PATH]".into()),
    };
    fs::create_dir_all(&artifacts_dir).map_err(|error| error.to_string())?;
    let config = copy_frozen_config(&artifacts_dir)?;
    let hko_key = hko_polytope_key();
    let mut artifacts = PacketArtifacts::default();

    for (replicate, seed) in MASTER_SEEDS.into_iter().enumerate() {
        if let Some(stop) = retain_arm(&mut artifacts, run_iid(seed, replicate, &hko_key)) {
            flush_incomplete(&artifacts, &artifacts_dir, overall_started, &stop)?;
            return Err(stop.message);
        }

        let mut local = run_local(seed, replicate, &hko_key);
        artifacts.local_trajectories.append(&mut local.trajectories);
        if let Some(stop) = retain_arm(&mut artifacts, local.arm) {
            flush_incomplete(&artifacts, &artifacts_dir, overall_started, &stop)?;
            return Err(stop.message);
        }

        let cem = run_cem(seed, replicate, &hko_key);
        artifacts.cem.extend(cem.records);
        if let Some(stop) = retain_arm(&mut artifacts, cem.arm) {
            flush_incomplete(&artifacts, &artifacts_dir, overall_started, &stop)?;
            return Err(stop.message);
        }
    }
    artifacts.write(&artifacts_dir)?;
    write_run_status(&artifacts_dir, true, &artifacts, overall_started, None)?;
    invoke_analyzer(&artifacts_dir, &config)?;
    Ok(())
}

#[cfg(test)]
mod smoke_tests {
    use super::*;
    use equal_budget_product_search::cem::{
        elite_set_id, generation_zero_distribution, ranked_elites, update_distribution,
    };
    use equal_budget_product_search::evaluator::SyntheticOracle;
    use exp_sys_landscape::{polytope_key, SysComputation};
    use nalgebra::Vector2;
    use symplectic::{OrbitAdmissibility, OrbitKktData, OrbitSearchResult};

    fn synthetic_computation() -> SysComputation {
        SysComputation {
            capacity: OrbitSearchResult {
                orbits: vec![OrbitKktData {
                    sigma: vec![0, 1, 2],
                    beta: vec![1.0; 3],
                    beta_margin: 1.0,
                    action: 2.0,
                    action_lower: 2.0,
                    action_upper: 2.0,
                    q: 0.25,
                    q_error_bound: 0.0,
                    mu: None,
                    xi: None,
                    admissibility: OrbitAdmissibility::AdmissibleExact,
                }],
                min_action: 2.0,
                min_action_lower: 2.0,
                min_action_upper: 2.0,
                iterations: 1,
            },
            vol: 4.0,
            sys: 0.5,
        }
    }

    fn smoke_polytopes() -> Vec<SysLandscapePolytopeCache> {
        let normals = vec![
            Vector2::new(1.0, 0.0),
            Vector2::new(0.0, 1.0),
            Vector2::new(-1.0, 0.0),
            Vector2::new(0.0, -1.0),
            Vector2::new(0.8, 0.6),
        ];
        (0..8)
            .map(|index| {
                let heights = vec![1.0 + 0.01 * index as f64; 5];
                SysLandscapePolytopeCache::from_lagrangian_product(
                    &normals, &heights, &normals, &heights,
                )
                .expect("synthetic product constructs")
            })
            .collect()
    }

    fn smoke_directory(label: &str) -> PathBuf {
        let unique = format!(
            "s0-runner-smoke-{label}-{}-{}",
            std::process::id(),
            Instant::now().elapsed().as_nanos()
        );
        std::env::temp_dir().join(unique)
    }

    #[test]
    fn synthetic_objective_exercises_production_arm_drivers() {
        let seed = MASTER_SEEDS[0];
        let hko_key = hko_polytope_key();
        let positive = || {
            let mut computation = synthetic_computation();
            computation.sys = 1.1;
            computation
        };

        let iid = run_iid_with_oracle(seed, 0, &hko_key, |_: &SysLandscapePolytopeCache| {
            Some(positive())
        });
        assert_eq!(iid.evaluator.attempts_used(), 1);
        assert!(iid.stop.is_some());

        let local = run_local_with_oracle(seed, 0, &hko_key, |_: &SysLandscapePolytopeCache| {
            Some(positive())
        });
        assert_eq!(local.arm.evaluator.attempts_used(), 1);
        assert!(local.arm.stop.is_some());

        let cem = run_cem_with_oracle(seed, 0, &hko_key, |_: &SysLandscapePolytopeCache| {
            Some(positive())
        });
        assert_eq!(cem.arm.evaluator.attempts_used(), 1);
        assert_eq!(cem.records.len(), 1);
        assert_eq!(cem.records[0]["complete"], serde_json::json!(false));
        assert_eq!(
            cem.records[0]["reason"],
            "new_or_materially_uncertain_relative_to_exact_HKO_control"
        );
        assert_eq!(
            cem.arm.stop.as_ref().map(|stop| stop.classification),
            Some("new_or_materially_uncertain_relative_to_exact_HKO_control")
        );
    }

    fn complete_generation_record(
        replicate: usize,
        generation: usize,
        members: Vec<String>,
        elite_candidate_ids: Vec<String>,
        distribution: CemDistribution,
        parent_elite_set_id: Option<String>,
    ) -> serde_json::Value {
        let elite_set_id = elite_set_id(&elite_candidate_ids);
        serde_json::json!({
            "replicate": replicate,
            "generation": generation,
            "elite_set_id": elite_set_id,
            "parent_elite_set_id": parent_elite_set_id,
            "member_candidate_ids": members,
            "elite_candidate_ids": elite_candidate_ids,
            "distribution": distribution,
            "complete": true,
            "construction_attempts": 64,
            "construction_rejections": 0,
        })
    }

    /// This remains target-free while exercising the actual Rust JSONL writers
    /// and the Python reconciliation boundary.  The fixture intentionally has
    /// a charged failed computation, cache hits, construction rejection
    /// provenance, CEM lineage, and separately rejected corrupt variants.
    #[test]
    fn synthetic_runner_artifacts_reconcile_and_corruption_is_rejected() {
        let directory = smoke_directory("complete");
        let polytopes = smoke_polytopes();
        let computation = synthetic_computation();
        let mut artifacts = PacketArtifacts::default();

        for arm in [Arm::Iid, Arm::MultistartBranchLocalPhase0, Arm::DiagonalCem].into_iter() {
            for (replicate, &master_seed) in MASTER_SEEDS.iter().enumerate() {
                let mut oracle = SyntheticOracle::default();
                for polytope in &polytopes {
                    oracle =
                        oracle.with_response(polytope_key(polytope), Some(computation.clone()));
                }
                let mut evaluator = ArmEvaluator::new(arm, replicate, oracle);
                let has_rejection = arm == Arm::Iid && replicate == 0;
                if has_rejection {
                    let identity = CandidateIdentity {
                        packet_version: PACKET_VERSION,
                        master_seed,
                        replicate,
                        arm,
                        generation: None,
                        trajectory: None,
                        iteration: None,
                        proposal_index: 0,
                        construction_attempt: 0,
                    };
                    evaluator.record_construction_rejection();
                    artifacts.rejections.push(ConstructionRejectionRow {
                        candidate_id: candidate_id(&identity),
                        arm,
                        replicate,
                        generation: None,
                        trajectory: None,
                        iteration: None,
                        proposal_index: 0,
                        construction_attempt: 0,
                        construction_sequence_index: 0,
                        role: ProposalRole::Iid,
                        reason: "synthetic_construction_rejection".into(),
                    });
                }
                for proposal_index in 0..256 {
                    let failure = arm == Arm::Iid && replicate == 0 && proposal_index == 255;
                    let polytope = if failure {
                        // It is a constructed, charged candidate but its key is
                        // deliberately absent from this arm-private oracle.
                        &polytopes[7]
                    } else {
                        &polytopes[proposal_index % 8]
                    };
                    let identity = CandidateIdentity {
                        packet_version: PACKET_VERSION,
                        master_seed,
                        replicate,
                        arm,
                        generation: (arm == Arm::DiagonalCem).then_some(proposal_index / 64),
                        trajectory: (arm == Arm::MultistartBranchLocalPhase0)
                            .then_some(proposal_index),
                        iteration: None,
                        proposal_index: if arm == Arm::DiagonalCem {
                            proposal_index % 64
                        } else {
                            proposal_index
                        },
                        construction_attempt: if arm == Arm::DiagonalCem {
                            proposal_index % 64
                        } else {
                            usize::from(has_rejection && proposal_index == 0)
                        },
                    };
                    let mut meta = proposal_meta(
                        identity,
                        match arm {
                            Arm::Iid => ProposalRole::Iid,
                            Arm::MultistartBranchLocalPhase0 => ProposalRole::LocalStart,
                            Arm::DiagonalCem => ProposalRole::CemPopulation,
                        },
                        None,
                        usize::from(has_rejection && proposal_index == 0),
                        proposal_index + usize::from(has_rejection),
                    );
                    if arm == Arm::DiagonalCem && proposal_index >= 64 {
                        meta.elite_set_id = Some(format!(
                            "s0v1-elite-{replicate:02x}{generation:022x}",
                            generation = proposal_index / 64 - 1
                        ));
                    }
                    if failure {
                        // The seventh successful key has already been cached;
                        // make the final request use a fresh missing key.
                        let failure_polytope = SysLandscapePolytopeCache::from_lagrangian_product(
                            &[
                                Vector2::new(1.0, 0.0),
                                Vector2::new(0.0, 1.0),
                                Vector2::new(-1.0, 0.0),
                                Vector2::new(0.0, -1.0),
                                Vector2::new(0.6, 0.8),
                            ],
                            &[1.3; 5],
                            &[
                                Vector2::new(1.0, 0.0),
                                Vector2::new(0.0, 1.0),
                                Vector2::new(-1.0, 0.0),
                                Vector2::new(0.0, -1.0),
                                Vector2::new(0.6, 0.8),
                            ],
                            &[1.3; 5],
                        )
                        .expect("failure fixture constructs");
                        evaluator.evaluate(meta, &failure_polytope);
                    } else {
                        evaluator.evaluate(meta, polytope);
                    }
                }
                artifacts.arm_runs.push(arm_run_row(
                    &evaluator,
                    artifacts
                        .rejections
                        .iter()
                        .filter(|row| row.arm == arm && row.replicate == replicate)
                        .count(),
                    accurate_sum(evaluator.target_rows().iter().map(|row| row.wall_time_ms)),
                    "complete",
                ));
                if arm == Arm::MultistartBranchLocalPhase0 {
                    artifacts
                        .local_trajectories
                        .extend(
                            evaluator
                                .target_rows()
                                .iter()
                                .map(|row| LocalTrajectoryRow {
                                    arm,
                                    replicate,
                                    trajectory: row.trajectory.expect("synthetic local trajectory"),
                                    start_candidate_id: row.candidate_id.clone(),
                                    final_candidate_id: row.candidate_id.clone(),
                                    start_sys: row.sys.expect("synthetic local success"),
                                    final_sys: row.sys.expect("synthetic local success"),
                                    accepted_iterations: 0,
                                    stop: "no_direction",
                                    complete: true,
                                }),
                        );
                }
                artifacts.absorb(&mut evaluator);
            }
        }

        for replicate in 0..3 {
            let cem_population = artifacts
                .targets
                .iter()
                .filter(|row| row.arm == Arm::DiagonalCem && row.replicate == replicate)
                .cloned()
                .collect::<Vec<_>>();
            let mut sampling_distribution = None;
            let mut parent_elite_set_id = None;
            for generation in 0..4 {
                let population = &cem_population[generation * 64..(generation + 1) * 64];
                let scored = population
                    .iter()
                    .map(|row| CemScoredCandidate {
                        candidate_id: row.candidate_id.clone(),
                        coordinates: row
                            .product_chart
                            .as_ref()
                            .expect("CEM smoke chart")
                            .continuous_coordinates(),
                        sys: row.sys.expect("CEM smoke sys"),
                    })
                    .collect::<Vec<_>>();
                let distribution = sampling_distribution.clone().unwrap_or_else(|| {
                    generation_zero_distribution(&scored).expect("generation-zero distribution")
                });
                let elites = ranked_elites(&scored, 16).expect("ranked smoke elites");
                let elite_candidate_ids = elites
                    .iter()
                    .map(|elite| elite.candidate_id.clone())
                    .collect::<Vec<_>>();
                let next_distribution =
                    update_distribution(&distribution, &elites).expect("smoke distribution update");
                let member_ids = population
                    .iter()
                    .map(|row| row.candidate_id.clone())
                    .collect::<Vec<_>>();
                for row in artifacts.targets.iter_mut().filter(|row| {
                    row.arm == Arm::DiagonalCem
                        && row.replicate == replicate
                        && row.generation == Some(generation)
                }) {
                    row.elite_set_id = parent_elite_set_id.clone();
                }
                artifacts.cem.push(complete_generation_record(
                    replicate,
                    generation,
                    member_ids,
                    elite_candidate_ids.clone(),
                    distribution,
                    parent_elite_set_id.clone(),
                ));
                parent_elite_set_id = Some(elite_set_id(&elite_candidate_ids));
                sampling_distribution = Some(next_distribution);
            }
        }

        artifacts
            .write(&directory)
            .expect("Rust serializers write smoke packet");
        let synthetic_overall_wall_time_ms = artifacts
            .arm_runs
            .iter()
            .map(|row| row.total_wall_time_ms)
            .sum::<f64>();
        fs::write(
            directory.join("run-status.json"),
            serde_json::to_vec_pretty(&serde_json::json!({
                "packet_version": PACKET_VERSION,
                "complete": true,
                "charged_target_attempts": artifacts.targets.len(),
                "overall_wall_time_ms": synthetic_overall_wall_time_ms,
                "stop": null,
            }))
            .expect("synthetic run status serializes"),
        )
        .expect("synthetic run status writes");
        let config = copy_frozen_config(&directory).expect("frozen config copied");
        invoke_analyzer(&directory, &config).expect("Python analyzer accepts Rust packet");
        assert!(directory.join("comparison-summary.json").is_file());

        let corrupt_cem = smoke_directory("corrupt-cem");
        fs::create_dir_all(&corrupt_cem).expect("corrupt directory");
        for entry in fs::read_dir(&directory).expect("complete rows") {
            let entry = entry.expect("directory entry");
            if entry.file_type().expect("entry type").is_file() {
                fs::copy(entry.path(), corrupt_cem.join(entry.file_name())).expect("copy fixture");
            }
        }
        let cem_path = corrupt_cem.join("cem-generations.jsonl");
        let mut cem = read_values(&cem_path);
        cem[0] = serde_json::json!({
            "replicate": 0,
            "generation": 0,
            "complete": false,
            "reason": "synthetic_incomplete_generation",
            "member_candidate_ids": [],
            "construction_attempts": 640,
            "construction_rejections": 640,
        });
        write_jsonl(cem_path, &cem).expect("Rust serializes incomplete CEM fixture");
        assert!(invoke_analyzer(&corrupt_cem, &config).is_err());

        let corrupt_cache = smoke_directory("corrupt-cache");
        fs::create_dir_all(&corrupt_cache).expect("corrupt directory");
        for entry in fs::read_dir(&directory).expect("complete rows") {
            let entry = entry.expect("directory entry");
            if entry.file_type().expect("entry type").is_file() {
                fs::copy(entry.path(), corrupt_cache.join(entry.file_name()))
                    .expect("copy fixture");
            }
        }
        let cache_path = corrupt_cache.join("expensive-computation-cache.jsonl");
        let mut cache = read_values(&cache_path);
        cache.remove(0);
        write_jsonl(cache_path, &cache).expect("Rust serializes corrupt cache fixture");
        assert!(invoke_analyzer(&corrupt_cache, &config).is_err());

        let corrupt_rejection = smoke_directory("corrupt-rejection");
        fs::create_dir_all(&corrupt_rejection).expect("corrupt rejection directory");
        for entry in fs::read_dir(&directory).expect("complete rows") {
            let entry = entry.expect("directory entry");
            if entry.file_type().expect("entry type").is_file() {
                fs::copy(entry.path(), corrupt_rejection.join(entry.file_name()))
                    .expect("copy rejection fixture");
            }
        }
        let rejection_path = corrupt_rejection.join("construction-rejections.jsonl");
        let mut rejections = read_values(&rejection_path);
        rejections[0]["proposal_index"] = serde_json::json!(999);
        rejections[0]["candidate_id"] = serde_json::json!(candidate_id(&CandidateIdentity {
            packet_version: PACKET_VERSION,
            master_seed: MASTER_SEEDS[0],
            replicate: 0,
            arm: Arm::Iid,
            generation: None,
            trajectory: None,
            iteration: None,
            proposal_index: 999,
            construction_attempt: 0,
        }));
        write_jsonl(rejection_path, &rejections)
            .expect("Rust serializes corrupt rejection fixture");
        assert!(invoke_analyzer(&corrupt_rejection, &config).is_err());

        let _ = fs::remove_dir_all(&directory);
        let _ = fs::remove_dir_all(&corrupt_cem);
        let _ = fs::remove_dir_all(&corrupt_cache);
        let _ = fs::remove_dir_all(&corrupt_rejection);
    }

    fn read_values(path: &Path) -> Vec<serde_json::Value> {
        fs::read_to_string(path)
            .expect("JSONL fixture")
            .lines()
            .map(|line| serde_json::from_str(line).expect("JSON row"))
            .collect()
    }
}

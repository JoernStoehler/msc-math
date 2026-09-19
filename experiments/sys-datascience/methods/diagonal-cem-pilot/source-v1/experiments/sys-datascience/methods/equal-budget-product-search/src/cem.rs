//! Fixed-constant diagonal cross-entropy-method state for the S0 packet.
//!
//! This module deliberately owns only chart statistics, proposal accounting,
//! and distribution genealogy.  The runner constructs geometry and evaluates
//! each proposal itself, so it can persist a target row (and honour the
//! packet's immediate-stop rule) before asking CEM for another proposal.

use rand::Rng;
use rand_distr::{Distribution, StandardNormal};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

pub use crate::chart::wrap_phase;
use crate::model::{
    candidate_id, Arm, CandidateIdentity, ProposalMeta, ProposalRole, PACKET_VERSION,
};

pub const CEM_DIMENSIONS: usize = 17;
pub const PHASE_COORDINATE: usize = 16;
pub const CEM_GENERATIONS: usize = 4;
pub const CEM_POPULATION: usize = 64;
pub const CEM_ELITES: usize = 16;
pub const CEM_SMOOTHING: f64 = 0.5;
pub const CEM_VARIANCE_FLOOR_FRACTION: f64 = 0.05;
pub const CEM_CONSTRUCTION_ATTEMPT_CAP: usize = 640;

pub type Coordinates = [f64; CEM_DIMENSIONS];

/// Wraps a phase difference to `[-pi, pi)` before squaring it.
fn wrapped_deviation(angle: f64) -> f64 {
    (angle + std::f64::consts::PI).rem_euclid(std::f64::consts::TAU) - std::f64::consts::PI
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CemDistribution {
    /// Means of the sixteen linear coordinates and the circular phase mean.
    pub mean: Coordinates,
    /// Diagonal variances.  The final entry is variance of wrapped deviations.
    pub variance: Coordinates,
    /// Immutable empirical generation-0 variances used by the variance floor.
    pub generation_zero_variance: Coordinates,
}

impl CemDistribution {
    pub fn sample_coordinates<R: Rng + ?Sized>(&self, rng: &mut R) -> Coordinates {
        let mut coordinates = self.mean;
        for (index, coordinate) in coordinates.iter_mut().enumerate() {
            let z: f64 = StandardNormal.sample(rng);
            *coordinate += self.variance[index].max(0.0).sqrt() * z;
        }
        coordinates[PHASE_COORDINATE] = wrap_phase(coordinates[PHASE_COORDINATE]);
        coordinates
    }

    pub fn variance_floor(&self, index: usize) -> f64 {
        CEM_VARIANCE_FLOOR_FRACTION * self.generation_zero_variance[index]
    }
}

#[derive(Clone, Debug)]
pub struct CemScoredCandidate {
    pub candidate_id: String,
    pub coordinates: Coordinates,
    pub sys: f64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CemError {
    EmptyPopulation,
    NonFiniteCoordinate {
        candidate_id: String,
        coordinate: usize,
    },
    NonFiniteSys {
        candidate_id: String,
    },
    WrongPopulationSize {
        expected: usize,
        actual: usize,
    },
    WrongEliteCount {
        expected: usize,
        actual: usize,
    },
    IncompleteGeneration {
        generated: usize,
    },
}

fn validate_candidates(candidates: &[CemScoredCandidate]) -> Result<(), CemError> {
    if candidates.is_empty() {
        return Err(CemError::EmptyPopulation);
    }
    for candidate in candidates {
        if !candidate.sys.is_finite() {
            return Err(CemError::NonFiniteSys {
                candidate_id: candidate.candidate_id.clone(),
            });
        }
        for (coordinate, value) in candidate.coordinates.iter().enumerate() {
            if !value.is_finite() {
                return Err(CemError::NonFiniteCoordinate {
                    candidate_id: candidate.candidate_id.clone(),
                    coordinate,
                });
            }
        }
    }
    Ok(())
}

/// Exact packet elite order: decreasing `sys`, then increasing candidate ID.
pub fn ranked_elites(
    candidates: &[CemScoredCandidate],
    elite_count: usize,
) -> Result<Vec<CemScoredCandidate>, CemError> {
    validate_candidates(candidates)?;
    if candidates.len() < elite_count {
        return Err(CemError::WrongEliteCount {
            expected: elite_count,
            actual: candidates.len(),
        });
    }
    let mut ranked = candidates.to_vec();
    ranked.sort_by(|left, right| {
        right
            .sys
            .partial_cmp(&left.sys)
            .expect("finite scores were checked")
            .then_with(|| left.candidate_id.cmp(&right.candidate_id))
    });
    ranked.truncate(elite_count);
    Ok(ranked)
}

fn linear_moments(candidates: &[CemScoredCandidate], index: usize) -> (f64, f64) {
    let count = candidates.len() as f64;
    let mean = candidates
        .iter()
        .map(|candidate| candidate.coordinates[index])
        .sum::<f64>()
        / count;
    let variance = candidates
        .iter()
        .map(|candidate| (candidate.coordinates[index] - mean).powi(2))
        .sum::<f64>()
        / count;
    (mean, variance)
}

/// Circular moments, using `fallback_phase` exactly when the sample resultant
/// is zero (up to ordinary f64 zero).  Variance uses wrapped deviations.
fn phase_moments(candidates: &[CemScoredCandidate], fallback_phase: f64) -> (f64, f64) {
    let count = candidates.len() as f64;
    let (cosine, sine) = candidates
        .iter()
        .fold((0.0, 0.0), |(cosine, sine), candidate| {
            let phase = candidate.coordinates[PHASE_COORDINATE];
            (cosine + phase.cos(), sine + phase.sin())
        });
    let mean = if resultant_is_zero(cosine, sine, candidates.len()) {
        wrap_phase(fallback_phase)
    } else {
        wrap_phase(sine.atan2(cosine))
    };
    let variance = candidates
        .iter()
        .map(|candidate| wrapped_deviation(candidate.coordinates[PHASE_COORDINATE] - mean).powi(2))
        .sum::<f64>()
        / count;
    (mean, variance)
}

fn resultant_is_zero(cosine: f64, sine: f64, count: usize) -> bool {
    // Trigonometric evaluations do not make an antipodal finite sample bitwise
    // zero (for example, `sin(PI)`).  This scale-aware threshold implements
    // the frozen *zero-resultant* rule rather than making it platform-noise
    // dependent.
    cosine.hypot(sine) <= 1e-12 * count as f64
}

fn moments(
    candidates: &[CemScoredCandidate],
    phase_fallback: f64,
) -> Result<(Coordinates, Coordinates), CemError> {
    validate_candidates(candidates)?;
    let mut mean = [0.0; CEM_DIMENSIONS];
    let mut variance = [0.0; CEM_DIMENSIONS];
    for index in 0..PHASE_COORDINATE {
        (mean[index], variance[index]) = linear_moments(candidates, index);
    }
    (mean[PHASE_COORDINATE], variance[PHASE_COORDINATE]) =
        phase_moments(candidates, phase_fallback);
    Ok((mean, variance))
}

/// Fits the generation-0 distribution from all 64 evaluated population rows.
/// If its phase resultant vanishes, choose the wrapped phase of the
/// lexicographically least candidate ID, as frozen in `resolved-config.json`.
pub fn generation_zero_distribution(
    population: &[CemScoredCandidate],
) -> Result<CemDistribution, CemError> {
    if population.len() != CEM_POPULATION {
        return Err(CemError::WrongPopulationSize {
            expected: CEM_POPULATION,
            actual: population.len(),
        });
    }
    validate_candidates(population)?;
    let fallback = population
        .iter()
        .min_by(|left, right| left.candidate_id.cmp(&right.candidate_id))
        .expect("nonempty after exact population-size check")
        .coordinates[PHASE_COORDINATE];
    let (mean, variance) = moments(population, fallback)?;
    Ok(CemDistribution {
        mean,
        variance,
        generation_zero_variance: variance,
    })
}

/// Applies the frozen 0.5 previous/0.5 elite update.  For the relative phase,
/// means are blended as circular unit moments and elite wrapped deviations are
/// used for the variance.  A zero elite resultant falls back to the previous
/// circular mean.
pub fn update_distribution(
    previous: &CemDistribution,
    elites: &[CemScoredCandidate],
) -> Result<CemDistribution, CemError> {
    if elites.len() != CEM_ELITES {
        return Err(CemError::WrongEliteCount {
            expected: CEM_ELITES,
            actual: elites.len(),
        });
    }
    let (elite_mean, elite_variance) = moments(elites, previous.mean[PHASE_COORDINATE])?;
    let mut mean = [0.0; CEM_DIMENSIONS];
    let mut variance = [0.0; CEM_DIMENSIONS];
    for index in 0..PHASE_COORDINATE {
        mean[index] =
            CEM_SMOOTHING * previous.mean[index] + (1.0 - CEM_SMOOTHING) * elite_mean[index];
        variance[index] = (CEM_SMOOTHING * previous.variance[index]
            + (1.0 - CEM_SMOOTHING) * elite_variance[index])
            .max(previous.variance_floor(index));
    }

    let cosine = CEM_SMOOTHING * previous.mean[PHASE_COORDINATE].cos()
        + (1.0 - CEM_SMOOTHING) * elite_mean[PHASE_COORDINATE].cos();
    let sine = CEM_SMOOTHING * previous.mean[PHASE_COORDINATE].sin()
        + (1.0 - CEM_SMOOTHING) * elite_mean[PHASE_COORDINATE].sin();
    mean[PHASE_COORDINATE] = if resultant_is_zero(cosine, sine, 2) {
        wrap_phase(previous.mean[PHASE_COORDINATE])
    } else {
        wrap_phase(sine.atan2(cosine))
    };
    variance[PHASE_COORDINATE] = (CEM_SMOOTHING * previous.variance[PHASE_COORDINATE]
        + (1.0 - CEM_SMOOTHING) * elite_variance[PHASE_COORDINATE])
        .max(previous.variance_floor(PHASE_COORDINATE));

    Ok(CemDistribution {
        mean,
        variance,
        generation_zero_variance: previous.generation_zero_variance,
    })
}

/// The only state that may cross a CEM generation boundary.  It separates the
/// distribution recorded for the just-evaluated population from the one that
/// may sample its successor.  `next_distribution` is intentionally absent
/// when a construction or evaluation barrier has not been passed.
#[derive(Clone, Debug)]
pub struct CompletedCemGeneration {
    pub recorded_distribution: CemDistribution,
    pub next_distribution: CemDistribution,
    pub elites: Vec<CemScoredCandidate>,
}

/// Fits/updates a completed generation and returns its next sampling state.
///
/// For generation 0, `previous_sampling_distribution` is `None`, the
/// recorded state is the empirical population distribution, and the successor
/// is its 0.5-smoothed elite update.  For later generations the recorded state
/// is the distribution that sampled the population.  This is the guarded API
/// an integration runner should use between generations; it rejects every
/// incomplete batch before any child distribution can be made.
pub fn complete_generation<T, E>(
    previous_sampling_distribution: Option<&CemDistribution>,
    batch: &CemConstructionBatch<T, E>,
    scored: &[CemScoredCandidate],
) -> Result<CompletedCemGeneration, CemError> {
    batch.require_complete()?;
    if scored.len() != CEM_POPULATION {
        return Err(CemError::WrongPopulationSize {
            expected: CEM_POPULATION,
            actual: scored.len(),
        });
    }
    let elites = ranked_elites(scored, CEM_ELITES)?;
    let recorded_distribution = match previous_sampling_distribution {
        Some(distribution) => distribution.clone(),
        None => generation_zero_distribution(scored)?,
    };
    let next_distribution = update_distribution(&recorded_distribution, &elites)?;
    Ok(CompletedCemGeneration {
        recorded_distribution,
        next_distribution,
        elites,
    })
}

pub fn elite_set_id(member_ids: &[String]) -> String {
    let mut ids = member_ids.to_vec();
    ids.sort();
    let digest = format!("{:x}", Sha256::digest(ids.join("\n").as_bytes()));
    format!("s0v1-elite-{}", &digest[..24])
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CemParentKind {
    None,
    Distribution,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CemLineage {
    pub candidate_id: String,
    pub parent_kind: CemParentKind,
    pub elite_set_id: Option<String>,
}

/// A valid constructed proposal.  `payload` is intentionally opaque to CEM;
/// it is normally the reconstructed product candidate owned by `chart.rs`.
#[derive(Clone, Debug)]
pub struct CemProposal<T> {
    pub meta: ProposalMeta,
    pub coordinates: Coordinates,
    pub payload: T,
    pub lineage: CemLineage,
}

#[derive(Clone, Debug)]
pub struct CemConstructionRequest<'a> {
    pub generation: usize,
    pub proposal_index: usize,
    pub construction_attempt: usize,
    pub candidate_id: String,
    /// `None` for the IID generation-0 population; later generations receive
    /// an independently sampled diagonal-CEM point.
    pub proposed_coordinates: Option<Coordinates>,
    pub distribution: Option<&'a CemDistribution>,
    pub parent_elite_set_id: Option<&'a str>,
}

#[derive(Clone, Debug)]
pub struct CemConstructed<T> {
    pub coordinates: Coordinates,
    pub payload: T,
}

#[derive(Clone, Debug)]
pub struct CemConstructionBatch<T, E> {
    pub proposals: Vec<CemProposal<T>>,
    pub rejections: Vec<E>,
    pub construction_attempts: usize,
    pub construction_rejections: usize,
    pub complete: bool,
}

impl<T, E> CemConstructionBatch<T, E> {
    pub fn require_complete(&self) -> Result<(), CemError> {
        if self.complete {
            Ok(())
        } else {
            Err(CemError::IncompleteGeneration {
                generated: self.proposals.len(),
            })
        }
    }
}

/// Constructs at most one 64-member population.  Rejected constructions never
/// become IID fill candidates.  In particular, a failed generation is a hard
/// generation barrier: callers must not call `update_distribution` from it.
pub fn construct_generation<T, E, R, F>(
    master_seed: u64,
    replicate: usize,
    generation: usize,
    distribution: Option<&CemDistribution>,
    parent_elite_set_id: Option<&str>,
    rng: &mut R,
    mut construct: F,
) -> CemConstructionBatch<T, E>
where
    R: Rng + ?Sized,
    F: FnMut(CemConstructionRequest<'_>) -> Result<CemConstructed<T>, E>,
{
    let mut proposals = Vec::with_capacity(CEM_POPULATION);
    let mut rejections = Vec::new();
    let mut rejections_since_previous_candidate = 0usize;
    for construction_attempt in 0..CEM_CONSTRUCTION_ATTEMPT_CAP {
        if proposals.len() == CEM_POPULATION {
            break;
        }
        let proposal_index = proposals.len();
        let candidate_id = candidate_id(&CandidateIdentity {
            packet_version: PACKET_VERSION,
            master_seed,
            replicate,
            arm: Arm::DiagonalCem,
            generation: Some(generation),
            trajectory: None,
            iteration: None,
            proposal_index,
            construction_attempt,
        });
        let proposed_coordinates = distribution.map(|state| state.sample_coordinates(rng));
        let request = CemConstructionRequest {
            generation,
            proposal_index,
            construction_attempt,
            candidate_id: candidate_id.clone(),
            proposed_coordinates,
            distribution,
            parent_elite_set_id,
        };
        match construct(request) {
            Ok(constructed) => {
                let meta = ProposalMeta {
                    candidate_id: candidate_id.clone(),
                    arm: Arm::DiagonalCem,
                    replicate,
                    generation: Some(generation),
                    trajectory: None,
                    iteration: None,
                    proposal_index,
                    construction_attempt,
                    construction_sequence_index: construction_attempt,
                    construction_rejections_before: rejections_since_previous_candidate,
                    role: ProposalRole::CemPopulation,
                    parent_candidate_id: None,
                    elite_set_id: parent_elite_set_id.map(str::to_owned),
                };
                proposals.push(CemProposal {
                    meta,
                    coordinates: constructed.coordinates,
                    payload: constructed.payload,
                    lineage: CemLineage {
                        candidate_id,
                        parent_kind: if parent_elite_set_id.is_some() {
                            CemParentKind::Distribution
                        } else {
                            CemParentKind::None
                        },
                        elite_set_id: parent_elite_set_id.map(str::to_owned),
                    },
                });
                rejections_since_previous_candidate = 0;
            }
            Err(error) => {
                rejections.push(error);
                rejections_since_previous_candidate += 1;
            }
        }
    }
    let construction_attempts = proposals.len() + rejections.len();
    CemConstructionBatch {
        complete: proposals.len() == CEM_POPULATION,
        construction_rejections: rejections.len(),
        proposals,
        rejections,
        construction_attempts,
    }
}

/// Calls the evaluator in proposal order.  The closure is invoked immediately
/// for each proposal, so artifact writers can flush a target row before the
/// next request.  Returning `false` implements an external stop (for example
/// a newly trusted `sys > 1`) without creating another target proposal.
pub fn evaluate_sequential<T, F>(proposals: &[CemProposal<T>], mut evaluate: F) -> usize
where
    F: FnMut(&CemProposal<T>) -> bool,
{
    let mut evaluated = 0;
    for proposal in proposals {
        evaluated += 1;
        if !evaluate(proposal) {
            break;
        }
    }
    evaluated
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CemGenerationRecord {
    pub replicate: usize,
    pub generation: usize,
    pub elite_set_id: String,
    pub parent_elite_set_id: Option<String>,
    pub member_candidate_ids: Vec<String>,
    pub elite_candidate_ids: Vec<String>,
    /// The distribution used to propose this population; generation 0 records
    /// the empirical population distribution once it has been evaluated.
    pub distribution: CemDistribution,
    pub complete: bool,
    pub construction_attempts: usize,
    pub construction_rejections: usize,
}

/// Builds the schema-facing generation and genealogy facts only after the
/// generation barrier has been passed and exactly 16 elites are available.
pub fn completed_generation_record<T, E>(
    replicate: usize,
    generation: usize,
    batch: &CemConstructionBatch<T, E>,
    scored: &[CemScoredCandidate],
    distribution: CemDistribution,
    parent_elite_set_id: Option<String>,
) -> Result<
    (
        CemGenerationRecord,
        Vec<CemLineage>,
        Vec<CemScoredCandidate>,
    ),
    CemError,
> {
    batch.require_complete()?;
    if scored.len() != CEM_POPULATION {
        return Err(CemError::WrongPopulationSize {
            expected: CEM_POPULATION,
            actual: scored.len(),
        });
    }
    let elites = ranked_elites(scored, CEM_ELITES)?;
    let member_candidate_ids = batch
        .proposals
        .iter()
        .map(|proposal| proposal.meta.candidate_id.clone())
        .collect::<Vec<_>>();
    let elite_candidate_ids = elites
        .iter()
        .map(|candidate| candidate.candidate_id.clone())
        .collect::<Vec<_>>();
    let elite_set_id = elite_set_id(&elite_candidate_ids);
    let record = CemGenerationRecord {
        replicate,
        generation,
        elite_set_id,
        parent_elite_set_id,
        member_candidate_ids,
        elite_candidate_ids,
        distribution,
        complete: true,
        construction_attempts: batch.construction_attempts,
        construction_rejections: batch.construction_rejections,
    };
    let lineages = batch
        .proposals
        .iter()
        .map(|proposal| proposal.lineage.clone())
        .collect();
    Ok((record, lineages, elites))
}

#[cfg(test)]
mod tests {
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    use super::*;

    fn scored(id: &str, phase: f64, sys: f64) -> CemScoredCandidate {
        let mut coordinates = [0.0; CEM_DIMENSIONS];
        coordinates[PHASE_COORDINATE] = phase;
        CemScoredCandidate {
            candidate_id: id.to_owned(),
            coordinates,
            sys,
        }
    }

    fn population() -> Vec<CemScoredCandidate> {
        (0..CEM_POPULATION)
            .map(|index| {
                let mut candidate =
                    scored(&format!("c{index:03}"), 0.1 * index as f64, index as f64);
                candidate.coordinates[0] = index as f64;
                candidate
            })
            .collect()
    }

    #[test]
    fn elite_ties_are_broken_by_candidate_id() {
        let candidates = vec![
            scored("z", 0.0, 1.0),
            scored("a", 0.0, 1.0),
            scored("b", 0.0, 0.5),
        ];
        let elites = ranked_elites(&candidates, 2).unwrap();
        assert_eq!(
            elites
                .iter()
                .map(|candidate| candidate.candidate_id.as_str())
                .collect::<Vec<_>>(),
            ["a", "z"]
        );
    }

    #[test]
    fn phase_moments_use_wrapped_deviations() {
        let near_cut = vec![
            scored("a", std::f64::consts::PI - 0.01, 0.0),
            scored("b", -std::f64::consts::PI + 0.01, 0.0),
        ];
        let (mean, variance) = moments(&near_cut, 0.0).unwrap();
        assert!(mean[PHASE_COORDINATE].abs() > 3.0);
        assert!(variance[PHASE_COORDINATE] < 0.001);
        assert_eq!(wrap_phase(std::f64::consts::PI), std::f64::consts::PI);
    }

    #[test]
    fn generation_zero_zero_resultant_uses_smallest_id_phase() {
        let mut candidates = population();
        candidates[0].candidate_id = "z".into();
        candidates[0].coordinates[PHASE_COORDINATE] = 0.0;
        candidates[1].candidate_id = "a".into();
        candidates[1].coordinates[PHASE_COORDINATE] = std::f64::consts::PI;
        for candidate in candidates.iter_mut().skip(2) {
            candidate.coordinates[PHASE_COORDINATE] = 0.0;
        }
        // 63 zero phases plus pi is not zero; make 32 antipodal pairs and use
        // the smallest ID on one side as the frozen fallback witness.
        for (index, candidate) in candidates.iter_mut().enumerate() {
            candidate.coordinates[PHASE_COORDINATE] = if index % 2 == 0 {
                0.25
            } else {
                0.25 + std::f64::consts::PI
            };
            candidate.candidate_id = format!("z{index:03}");
        }
        candidates[1].candidate_id = "a".into();
        let distribution = generation_zero_distribution(&candidates).unwrap();
        assert!(
            (distribution.mean[PHASE_COORDINATE]
                - wrap_phase(candidates[1].coordinates[PHASE_COORDINATE]))
            .abs()
                < 1e-12
        );
    }

    #[test]
    fn zero_elite_resultant_falls_back_to_previous_phase() {
        let previous = CemDistribution {
            mean: [0.0; CEM_DIMENSIONS],
            variance: [1.0; CEM_DIMENSIONS],
            generation_zero_variance: [1.0; CEM_DIMENSIONS],
        };
        let elites = (0..CEM_ELITES)
            .map(|index| {
                scored(
                    &format!("e{index}"),
                    if index % 2 == 0 {
                        1.2
                    } else {
                        1.2 + std::f64::consts::PI
                    },
                    1.0,
                )
            })
            .collect::<Vec<_>>();
        let updated = update_distribution(&previous, &elites).unwrap();
        assert!((updated.mean[PHASE_COORDINATE] - 0.0).abs() < 1e-12);
    }

    #[test]
    fn update_applies_generation_zero_variance_floor() {
        let mut previous = CemDistribution {
            mean: [0.0; CEM_DIMENSIONS],
            variance: [0.001; CEM_DIMENSIONS],
            generation_zero_variance: [4.0; CEM_DIMENSIONS],
        };
        previous.variance[0] = 0.0;
        let elites = (0..CEM_ELITES)
            .map(|index| scored(&format!("e{index}"), 0.0, 1.0))
            .collect::<Vec<_>>();
        let updated = update_distribution(&previous, &elites).unwrap();
        assert_eq!(updated.variance[0], 0.2);
        assert_eq!(updated.variance[PHASE_COORDINATE], 0.2);
    }

    #[test]
    fn construction_cap_stops_without_iid_fill() {
        let mut rng = ChaCha8Rng::seed_from_u64(7);
        let batch = construct_generation(
            1,
            0,
            1,
            Some(&CemDistribution {
                mean: [0.0; CEM_DIMENSIONS],
                variance: [1.0; CEM_DIMENSIONS],
                generation_zero_variance: [1.0; CEM_DIMENSIONS],
            }),
            Some("elite-0"),
            &mut rng,
            |_request| Err::<CemConstructed<()>, _>("invalid"),
        );
        assert!(!batch.complete);
        assert_eq!(batch.proposals.len(), 0);
        assert_eq!(batch.construction_attempts, CEM_CONSTRUCTION_ATTEMPT_CAP);
        assert_eq!(batch.construction_rejections, CEM_CONSTRUCTION_ATTEMPT_CAP);
        assert_eq!(
            batch.require_complete(),
            Err(CemError::IncompleteGeneration { generated: 0 })
        );
        assert_eq!(
            complete_generation::<(), _>(None, &batch, &[])
                .expect_err("an incomplete generation cannot make a child distribution"),
            CemError::IncompleteGeneration { generated: 0 }
        );
    }

    #[test]
    fn complete_generation_is_a_barrier_before_update_and_records_distribution_parentage() {
        let mut rng = ChaCha8Rng::seed_from_u64(11);
        let distribution = CemDistribution {
            mean: [0.0; CEM_DIMENSIONS],
            variance: [1.0; CEM_DIMENSIONS],
            generation_zero_variance: [1.0; CEM_DIMENSIONS],
        };
        let incomplete = construct_generation(
            1,
            0,
            1,
            Some(&distribution),
            Some("elite-0"),
            &mut rng,
            |request| {
                if request.construction_attempt == 2 {
                    Err(())
                } else {
                    Ok(CemConstructed {
                        coordinates: request.proposed_coordinates.unwrap(),
                        payload: (),
                    })
                }
            },
        );
        // One rejection does not cross the cap: the population is still a
        // proper barrier-complete generation and the retry has a distinct ID.
        assert!(incomplete.complete);
        assert_eq!(incomplete.proposals[2].meta.construction_attempt, 3);
        assert_eq!(
            incomplete.proposals[2].lineage.parent_kind,
            CemParentKind::Distribution
        );
        assert_eq!(
            incomplete.proposals[2].lineage.elite_set_id.as_deref(),
            Some("elite-0")
        );

        let scored_rows = incomplete
            .proposals
            .iter()
            .enumerate()
            .map(|(index, proposal)| CemScoredCandidate {
                candidate_id: proposal.meta.candidate_id.clone(),
                coordinates: proposal.coordinates,
                sys: index as f64,
            })
            .collect::<Vec<_>>();
        let (record, lineages, _) = completed_generation_record(
            0,
            1,
            &incomplete,
            &scored_rows,
            distribution,
            Some("elite-0".into()),
        )
        .unwrap();
        assert_eq!(record.parent_elite_set_id.as_deref(), Some("elite-0"));
        assert_eq!(record.member_candidate_ids.len(), CEM_POPULATION);
        assert!(lineages
            .iter()
            .all(|lineage| lineage.parent_kind == CemParentKind::Distribution));
    }

    #[test]
    fn evaluation_driver_stops_before_another_proposal_is_evaluated() {
        let mut rng = ChaCha8Rng::seed_from_u64(3);
        let batch = construct_generation(1, 0, 0, None, None, &mut rng, |_request| {
            Ok::<_, ()>(CemConstructed {
                coordinates: [0.0; CEM_DIMENSIONS],
                payload: (),
            })
        });
        assert_eq!(
            evaluate_sequential(&batch.proposals, |proposal| proposal.meta.proposal_index
                < 6),
            7
        );
    }
}

//! Minimal fixed-population adaptive multilevel splitting (AMS).
//!
//! The engine is generic over particles, scores, and proposal streams.  The
//! product adapter uses the package's IID stream and trusted `sys` evaluator;
//! unit tests use a deterministic synthetic stream and never evaluate `sys`.

use crate::chart::{iid_base_candidate_attempt, ProductCandidate};
use exp_sys_landscape::compute_sys;
use rand::Rng;

#[derive(Clone, Debug, PartialEq)]
pub struct Scored<T> {
    pub value: T,
    pub score: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Report<T> {
    pub particles: Vec<Scored<T>>,
    pub estimate: f64,
    pub splitting_count: usize,
    pub target_evaluations: usize,
    pub target_reached: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub enum AmsError {
    InvalidPopulation,
    InvalidKillCount,
    NonFiniteScore,
    BoundaryTie { level: f64 },
}

/// Run fixed-population AMS with kill count `k` and at most `cap` splits.
///
/// Initial scores count as one target evaluation per particle. `draw_and_score`
/// draws a fresh independent proposal and evaluates it once; the proposal is
/// accepted only when its finite score is strictly above the current level.
/// Otherwise the selected survivor clone is retained. Initial scores, proposal
/// scores, and `target` must be finite.
pub fn run_ams<T, R, F>(
    mut particles: Vec<Scored<T>>,
    target: f64,
    k: usize,
    cap: usize,
    rng: &mut R,
    mut draw_and_score: F,
) -> Result<Report<T>, AmsError>
where
    T: Clone,
    R: Rng + ?Sized,
    F: FnMut(&mut R) -> (T, f64),
{
    let n = particles.len();
    if n == 0 {
        return Err(AmsError::InvalidPopulation);
    }
    if k == 0 || k >= n {
        return Err(AmsError::InvalidKillCount);
    }
    if !target.is_finite() || particles.iter().any(|particle| !particle.score.is_finite()) {
        return Err(AmsError::NonFiniteScore);
    }
    let mut target_evaluations = n;
    let mut splitting_count = 0;
    let mut target_reached = false;

    while splitting_count < cap {
        particles.sort_by(|a, b| a.score.total_cmp(&b.score));
        let level = particles[k - 1].score;
        if particles[k].score == level {
            return Err(AmsError::BoundaryTie { level });
        }
        if level >= target {
            target_reached = true;
            break;
        }

        // The survivor set is a stable slice; killed slots are replaced only
        // after all parent choices have been made from this actual set.
        let survivors: Vec<Scored<T>> = particles[k..].to_vec();
        for killed_particle in particles.iter_mut().take(k) {
            let parent = &survivors[rng.gen_range(0..survivors.len())];
            let (proposal, score) = draw_and_score(rng);
            target_evaluations += 1;
            if !score.is_finite() {
                return Err(AmsError::NonFiniteScore);
            }
            *killed_particle = if score > level {
                Scored {
                    value: proposal,
                    score,
                }
            } else {
                parent.clone()
            };
        }
        splitting_count += 1;
    }

    particles.sort_by(|a, b| a.score.total_cmp(&b.score));
    if !target_reached {
        let level = particles[k - 1].score;
        if particles[k].score == level {
            return Err(AmsError::BoundaryTie { level });
        }
        // A population produced by the final permitted split may already
        // exceed the target, but the target was not observed before the cap.
        // With cap zero, however, an initially reached target is immediate.
        target_reached = level >= target && (splitting_count < cap || cap == 0);
    }
    let above = particles.iter().filter(|p| p.score > target).count();
    let survival = (n - k) as f64 / n as f64;
    let estimate = survival.powi(splitting_count as i32) * above as f64 / n as f64;
    Ok(Report {
        particles,
        estimate,
        splitting_count,
        target_evaluations,
        target_reached,
    })
}

#[derive(Clone, Debug, PartialEq)]
pub enum IidPopulationError {
    ConstructionExhausted { base_index: usize },
    TargetFailed { base_index: usize },
}

/// Construct and score an IID initial population with a bounded number of
/// uncharged construction attempts per particle.
///
/// This calls the trusted `compute_sys` once for every successfully constructed
/// particle. Unit tests do not call this adapter.
pub fn iid_scored_population(
    master_seed: u64,
    replicate: usize,
    n: usize,
    construction_attempt_limit: usize,
) -> Result<Vec<Scored<ProductCandidate>>, IidPopulationError> {
    let mut population = Vec::with_capacity(n);
    for base_index in 0..n {
        let mut candidate = None;
        for attempt in 0..construction_attempt_limit {
            if let Ok(value) =
                iid_base_candidate_attempt(master_seed, replicate, base_index, attempt)
            {
                candidate = Some(value);
                break;
            }
        }
        let candidate =
            candidate.ok_or(IidPopulationError::ConstructionExhausted { base_index })?;
        let score = compute_sys(&candidate.polytope)
            .ok_or(IidPopulationError::TargetFailed { base_index })?;
        population.push(Scored {
            value: candidate,
            score,
        });
    }
    Ok(population)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    fn run(initial: &[f64], target: f64, k: usize, cap: usize, proposals: &[f64]) -> Report<f64> {
        let mut rng = ChaCha8Rng::seed_from_u64(7);
        let mut next = proposals.iter().copied();
        run_ams(
            initial
                .iter()
                .copied()
                .map(|x| Scored { value: x, score: x })
                .collect(),
            target,
            k,
            cap,
            &mut rng,
            |_| {
                let x = next.next().expect("synthetic proposal");
                (x, x)
            },
        )
        .expect("synthetic AMS run")
    }

    #[test]
    fn rejection_retains_clone_and_acceptance_replaces() {
        let report = run(&[0.1, 0.2, 0.8, 0.9], 2.0, 1, 1, &[0.3]);
        assert!(report.particles.iter().any(|p| p.score == 0.8));
        let accepted = run(&[0.1, 0.2, 0.8, 0.9], 2.0, 1, 1, &[0.95]);
        assert!(accepted.particles.iter().any(|p| p.score == 0.95));
    }

    #[test]
    fn survivors_only_and_exact_slots() {
        let report = run(&[0.1, 0.2, 0.8, 0.9, 1.0], 2.0, 2, 1, &[0.95, 0.96]);
        assert_eq!(report.particles.len(), 5);
        assert!(report.particles.iter().all(|p| p.score >= 0.8));
        assert_eq!(report.target_evaluations, 7);
    }

    #[test]
    fn boundary_tie_is_explicit() {
        let mut rng = ChaCha8Rng::seed_from_u64(1);
        let result = run_ams(
            vec![0.0, 1.0, 1.0, 2.0]
                .into_iter()
                .map(|x| Scored { value: x, score: x })
                .collect(),
            3.0,
            2,
            1,
            &mut rng,
            |_| (0.0, 0.0),
        );
        assert_eq!(result, Err(AmsError::BoundaryTie { level: 1.0 }));
    }

    #[test]
    fn boundary_tie_at_cap_is_explicit() {
        let mut rng = ChaCha8Rng::seed_from_u64(1);
        let mut proposals = [0.8, 0.8].into_iter();
        let result = run_ams(
            vec![0.1, 0.2, 0.8, 0.9]
                .into_iter()
                .map(|x| Scored { value: x, score: x })
                .collect(),
            0.7,
            2,
            1,
            &mut rng,
            |_| {
                let x = proposals.next().expect("proposal");
                (x, x)
            },
        );
        assert_eq!(result, Err(AmsError::BoundaryTie { level: 0.8 }));
    }

    #[test]
    fn target_and_cap_accounting_and_estimator() {
        let reached = run(&[0.1, 0.9, 0.95, 0.99], 0.85, 1, 4, &[0.95]);
        assert!(reached.target_reached);
        assert_eq!(reached.splitting_count, 1);
        assert_eq!(reached.target_evaluations, 5);
        assert!((reached.estimate - 0.75f64.powi(1) * 4.0 / 4.0).abs() < 1e-12);

        let capped = run(&[0.1, 0.2, 0.8, 0.9], 2.0, 1, 1, &[0.3]);
        assert!(!capped.target_reached);
        assert_eq!(capped.splitting_count, 1);
        assert!((capped.estimate - 0.0).abs() < 1e-12);

        let reached_at_cap = run(&[0.1, 0.9, 0.95, 0.99], 0.85, 1, 1, &[0.95]);
        assert!(!reached_at_cap.target_reached);
        assert_eq!(reached_at_cap.splitting_count, 1);
    }
}

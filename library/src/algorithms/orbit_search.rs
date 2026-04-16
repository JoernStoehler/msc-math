//! Shared orbit-search result types for capacity algorithms.
//!
//! This module is the common result-layer scaffold for the `hk2017`,
//! `hk2017_unpruned`, and `billiard` frontends. It deliberately separates:
//!
//! - orbit payload data (`OrbitKktData`)
//! - search-level guarantees and backend choice
//! - search/recovery error classification
//!
//! The current implementation still uses older algorithm-specific result types
//! (`EhzResult`, `BilliardResult`). This module exists so later refactor
//! packets can migrate those frontends onto one shared surface without further
//! renaming churn.

use crate::algorithms::capacity_accumulator::{CapacityAccumulator, CapacityResult};
use crate::geom::polytope::Polytope4D;
use crate::geom::rational_arithmetic::rational_to_f64;
use crate::kkt::rational_solver::solve_kkt_exact;
use crate::kkt::saddle_point_solver::{solve_kkt_for, KktOutcome, KktResult, EPS_Q_POSITIVE};
use crate::kkt::{classify_margin, Solution, Verdict};

/// Admissibility status of a numerically solved orbit candidate.
///
/// Known-inadmissible candidates are discarded before they become
/// `OrbitKktData`. This enum therefore describes only the surviving states.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OrbitAdmissibility {
    /// Admissible according to the f64 solve/classification path.
    AdmissibleF64,
    /// Still unresolved after the f64 path.
    IndeterminateF64,
    /// Admissibility was certified by the exact fallback path.
    AdmissibleExact,
}

/// Strength of the admissibility guarantee applied before returning a search
/// result.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OrbitGuaranteeMode {
    /// Resolve enough indeterminate candidates that the reported minimum-action
    /// interval endpoints are justified by admissible orbits.
    BoundSafe,
    /// Resolve every indeterminate candidate whose action interval intersects
    /// the exact-minimum window `[min_action_lower, min_action_upper]`.
    MinimaSafe,
    /// Resolve every indeterminate candidate that remains in the returned
    /// orbit list.
    AllSafe,
}

/// Primitive numerical backend used to solve one sigma.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OrbitSolveBackend {
    /// Constraint-projection/eigendecomposition path.
    Projected,
    /// Augmented saddle-point KKT path.
    SaddlePoint,
}

/// Solved orbit payload used by all capacity frontends.
#[derive(Clone, Debug, PartialEq)]
pub struct OrbitKktData {
    /// Cyclic facet sequence σ. Entries are distinct facet indices, not a full
    /// permutation of `0..F`.
    pub sigma: Vec<usize>,
    /// β aligned with σ: `beta[i]` belongs to `sigma[i]`.
    pub beta: Vec<f64>,
    /// Convenience scalar `min(beta)`.
    pub beta_margin: f64,
    /// Producer-chosen scalar action summary for ordinary consumers.
    pub action: f64,
    /// Lower endpoint of the action interval.
    pub action_lower: f64,
    /// Upper endpoint of the action interval.
    pub action_upper: f64,
    /// Public name for the corrected Q value used internally today as
    /// `q_corrected`.
    pub q: f64,
    /// Absolute error bound for `q`.
    pub q_error_bound: f64,
    /// Closure multipliers when the chosen backend/path provides them.
    pub mu: Option<[f64; 4]>,
    /// Normalization multiplier when the chosen backend/path provides it.
    pub xi: Option<f64>,
    /// Admissibility state after any exact fallback requested by the active
    /// guarantee mode.
    pub admissibility: OrbitAdmissibility,
}

/// Shared result of collecting near-minimum solved orbits.
#[derive(Clone, Debug, PartialEq)]
pub struct OrbitSearchResult {
    /// Returned orbits, sorted by lower action bound ascending.
    pub orbits: Vec<OrbitKktData>,
    /// Canonical single-f64 minimum among admissible returned orbits.
    pub min_action: f64,
    /// Lower bound for the minimum action across retained candidates.
    pub min_action_lower: f64,
    /// Upper bound for the minimum action across retained candidates.
    pub min_action_upper: f64,
    /// Number of sigma candidates examined by the search frontend.
    pub iterations: u64,
}

/// Search-level failure classification for the shared orbit collectors.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OrbitSearchError {
    /// No admissible orbit remained after filtering and requested fallback.
    NoAdmissibleOrbit,
    /// The numerical backend failed before the requested guarantee could be
    /// established.
    NumericalFailure,
    /// Exact fallback was required by the active guarantee mode but failed.
    ExactFallbackFailure,
}

/// Failure classification for solving a single sigma into `OrbitKktData`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OrbitSolveError {
    /// The chosen backend is not yet wired into the guaranteed orbit payload
    /// surface.
    UnsupportedBackend,
    /// The sigma is certified non-admissible or has non-competitive `Q <= 0`.
    Inadmissible,
    /// The numerical backend failed to produce the payload required by
    /// `OrbitKktData`.
    NumericalFailure,
}

/// Failure classification for geometric orbit construction/verification.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GeometricOrbitError {
    /// The supplied sigma/beta data does not determine a meaningful geometric
    /// orbit to reconstruct.
    DegenerateOrbit,
    /// Linear algebra used during reconstruction failed.
    LinearSolveFailure,
    /// A geometric orbit was produced, but the verification checks failed.
    VerificationFailed,
}

/// Solve one sigma into the shared orbit payload.
///
/// The current implementation is only complete for the saddle-point backend.
/// The projected backend remains scaffold-only until the library projection
/// path exposes the same `Q`-bound contract required by `OrbitKktData`.
pub fn solve_orbit_sigma(
    polytope: &Polytope4D,
    sigma: &[usize],
    backend: OrbitSolveBackend,
) -> Result<OrbitKktData, OrbitSolveError> {
    match backend {
        // TODO(capacity-result-api): Replace this with a real projection-backed
        // orbit payload once `library/src/kkt/projection_solver.rs` exposes the
        // Q-bound contract needed by `OrbitKktData` (`q_error_bound`,
        // interval-aware action fields, and any multiplier reconstruction we
        // decide to support there).
        OrbitSolveBackend::Projected => Err(OrbitSolveError::UnsupportedBackend),
        OrbitSolveBackend::SaddlePoint => {
            let outcome = solve_kkt_for(polytope, sigma);
            solve_saddle_point_sigma(sigma, outcome)
        }
    }
}

fn solve_saddle_point_sigma(
    sigma: &[usize],
    outcome: KktOutcome,
) -> Result<OrbitKktData, OrbitSolveError> {
    let kkt = match outcome {
        KktOutcome::Feasible(kkt) => kkt,
        KktOutcome::Infeasible => return Err(OrbitSolveError::Inadmissible),
        KktOutcome::SingularMatrix
        | KktOutcome::TypeCViolation
        | KktOutcome::ConstraintViolation => return Err(OrbitSolveError::NumericalFailure),
    };
    orbit_from_saddle_point_result(sigma, kkt)
}

fn orbit_from_saddle_point_result(
    sigma: &[usize],
    result: KktResult,
) -> Result<OrbitKktData, OrbitSolveError> {
    if result.q_corrected <= EPS_Q_POSITIVE {
        return Err(OrbitSolveError::Inadmissible);
    }

    let beta_margin = result
        .beta
        .iter()
        .copied()
        .fold(f64::INFINITY, f64::min);
    let admissibility = match classify_margin(beta_margin) {
        crate::kkt::Verdict::True => OrbitAdmissibility::AdmissibleF64,
        crate::kkt::Verdict::Indeterminate => OrbitAdmissibility::IndeterminateF64,
        crate::kkt::Verdict::False => return Err(OrbitSolveError::Inadmissible),
    };
    let (action_lower, action_upper) =
        action_bounds_from_q(result.q_corrected, result.q_error_bound);

    let mu: [f64; 4] = result
        .mu
        .as_slice()
        .try_into()
        .map_err(|_| OrbitSolveError::NumericalFailure)?;

    Ok(OrbitKktData {
        sigma: sigma.to_vec(),
        beta: result.beta,
        beta_margin,
        action: 0.5 / result.q_corrected,
        action_lower,
        action_upper,
        q: result.q_corrected,
        q_error_bound: result.q_error_bound,
        mu: Some(mu),
        xi: Some(result.xi),
        admissibility,
    })
}

fn action_bounds_from_q(q: f64, q_error_bound: f64) -> (f64, f64) {
    let q_upper = q + q_error_bound;
    let action_lower = 0.5 / q_upper;
    let q_lower = q - q_error_bound;
    let action_upper = if q_lower > EPS_Q_POSITIVE {
        0.5 / q_lower
    } else {
        f64::INFINITY
    };
    (action_lower, action_upper)
}

fn exact_orbit_from_sigma(
    polytope: &Polytope4D,
    sigma: &[usize],
    old_mu: Option<[f64; 4]>,
    old_xi: Option<f64>,
) -> Option<OrbitKktData> {
    let exact = solve_kkt_exact(polytope.dual_vertices(), sigma)?;
    let beta: Vec<f64> = exact.beta.iter().map(rational_to_f64).collect();
    let beta_margin = beta.iter().copied().fold(f64::INFINITY, f64::min);
    let action = 0.5 / exact.q_exact_f64;

    Some(OrbitKktData {
        sigma: sigma.to_vec(),
        beta,
        beta_margin,
        action,
        action_lower: action,
        action_upper: action,
        q: exact.q_exact_f64,
        q_error_bound: 0.0,
        // TODO(capacity-result-api): If the exact fallback later computes exact
        // multipliers, upgrade these carried-over optional fields instead of
        // preserving the old numerical values.
        mu: old_mu,
        xi: old_xi,
        admissibility: OrbitAdmissibility::AdmissibleExact,
    })
}

fn resolve_orbit_exact(
    polytope: &Polytope4D,
    orbit: &OrbitKktData,
) -> Option<OrbitKktData> {
    exact_orbit_from_sigma(polytope, &orbit.sigma, orbit.mu, orbit.xi)
}

fn argmin_action_lower(orbits: &[OrbitKktData]) -> Option<usize> {
    orbits
        .iter()
        .enumerate()
        .min_by(|(_, a), (_, b)| a.action_lower.total_cmp(&b.action_lower))
        .map(|(idx, _)| idx)
}

fn argmin_action_upper(orbits: &[OrbitKktData]) -> Option<usize> {
    orbits
        .iter()
        .enumerate()
        .min_by(|(_, a), (_, b)| a.action_upper.total_cmp(&b.action_upper))
        .map(|(idx, _)| idx)
}

fn resolve_indices_exact(
    polytope: &Polytope4D,
    orbits: &mut Vec<OrbitKktData>,
    mut indices: Vec<usize>,
) -> Result<(), OrbitSearchError> {
    indices.sort_unstable();
    indices.dedup();

    for idx in indices.into_iter().rev() {
        let upgraded = resolve_orbit_exact(polytope, &orbits[idx]);
        match upgraded {
            Some(exact_orbit) => orbits[idx] = exact_orbit,
            None => {
                orbits.remove(idx);
            }
        }
    }

    if orbits.is_empty() {
        return Err(OrbitSearchError::NoAdmissibleOrbit);
    }
    Ok(())
}

fn resolve_boundsafe(
    polytope: &Polytope4D,
    orbits: &mut Vec<OrbitKktData>,
) -> Result<(), OrbitSearchError> {
    loop {
        let lower_idx = argmin_action_lower(orbits).ok_or(OrbitSearchError::NoAdmissibleOrbit)?;
        let upper_idx = argmin_action_upper(orbits).ok_or(OrbitSearchError::NoAdmissibleOrbit)?;

        let mut needs_exact = Vec::new();
        if orbits[lower_idx].admissibility == OrbitAdmissibility::IndeterminateF64 {
            needs_exact.push(lower_idx);
        }
        if orbits[upper_idx].admissibility == OrbitAdmissibility::IndeterminateF64 {
            needs_exact.push(upper_idx);
        }

        if needs_exact.is_empty() {
            return Ok(());
        }

        resolve_indices_exact(polytope, orbits, needs_exact)?;
    }
}

fn resolve_minimasafe(
    polytope: &Polytope4D,
    orbits: &mut Vec<OrbitKktData>,
) -> Result<(), OrbitSearchError> {
    loop {
        resolve_boundsafe(polytope, orbits)?;
        let lower = orbits
            .iter()
            .map(|orbit| orbit.action_lower)
            .fold(f64::INFINITY, f64::min);
        let upper = orbits
            .iter()
            .map(|orbit| orbit.action_upper)
            .fold(f64::INFINITY, f64::min);

        let needs_exact: Vec<usize> = orbits
            .iter()
            .enumerate()
            .filter_map(|(idx, orbit)| {
                let intersects_minimum_window =
                    orbit.action_lower <= upper && lower <= orbit.action_upper;
                (orbit.admissibility == OrbitAdmissibility::IndeterminateF64
                    && intersects_minimum_window)
                    .then_some(idx)
            })
            .collect();

        if needs_exact.is_empty() {
            return Ok(());
        }

        resolve_indices_exact(polytope, orbits, needs_exact)?;
    }
}

fn resolve_allsafe(
    polytope: &Polytope4D,
    orbits: &mut Vec<OrbitKktData>,
) -> Result<(), OrbitSearchError> {
    let needs_exact: Vec<usize> = orbits
        .iter()
        .enumerate()
        .filter_map(|(idx, orbit)| {
            (orbit.admissibility == OrbitAdmissibility::IndeterminateF64).then_some(idx)
        })
        .collect();
    resolve_indices_exact(polytope, orbits, needs_exact)
}

pub(crate) fn resolve_orbits_for_guarantee(
    polytope: &Polytope4D,
    orbits: &mut Vec<OrbitKktData>,
    mode: OrbitGuaranteeMode,
) -> Result<(), OrbitSearchError> {
    if orbits.is_empty() {
        return Err(OrbitSearchError::NoAdmissibleOrbit);
    }

    match mode {
        OrbitGuaranteeMode::BoundSafe => resolve_boundsafe(polytope, orbits),
        OrbitGuaranteeMode::MinimaSafe => resolve_minimasafe(polytope, orbits),
        OrbitGuaranteeMode::AllSafe => resolve_allsafe(polytope, orbits),
    }
}

fn legacy_solution_from_orbit(orbit: OrbitKktData) -> Solution {
    let verdict = match orbit.admissibility {
        OrbitAdmissibility::AdmissibleF64 | OrbitAdmissibility::AdmissibleExact => Verdict::True,
        OrbitAdmissibility::IndeterminateF64 => Verdict::Indeterminate,
    };

    Solution {
        verdict,
        q: orbit.q,
        beta: orbit.beta,
        margin: orbit.beta_margin,
    }
}

/// Shared collector seam for the current legacy capacity frontends.
///
/// This helper deliberately sits below frontend-specific sigma generation and
/// above frontend-specific metadata such as HK2017 subsets or billiard bounce
/// counts. It lets Packet 2 share the solve/classify/track/finalize loop
/// without prematurely forcing all candidate generators into one abstraction.
pub(crate) fn collect_legacy_capacity<M: Clone>(
    polytope: &Polytope4D,
    backend: OrbitSolveBackend,
    mut emit_sigma: impl FnMut(&mut dyn FnMut(&[usize], M)),
    fallback_metadata: impl FnOnce(&CapacityResult) -> M,
) -> Option<(CapacityResult, M)> {
    let mut acc = CapacityAccumulator::new();
    let mut best_certified: Option<(f64, M)> = None;

    let mut visit = |sigma: &[usize], metadata: M| {
        let orbit = match solve_orbit_sigma(polytope, sigma, backend) {
            Ok(orbit) => orbit,
            Err(_) => return,
        };

        if matches!(
            orbit.admissibility,
            OrbitAdmissibility::AdmissibleF64 | OrbitAdmissibility::AdmissibleExact
        ) && orbit.q > EPS_Q_POSITIVE
        {
            let update = best_certified
                .as_ref()
                .is_none_or(|(best, _)| orbit.action < *best);
            if update {
                best_certified = Some((orbit.action, metadata.clone()));
            }
        }

        let solution = legacy_solution_from_orbit(orbit);
        acc.submit(sigma, &solution);
    };

    emit_sigma(&mut visit);

    let result = acc.finalize()?;
    let metadata = best_certified
        .map(|(_, metadata)| metadata)
        .unwrap_or_else(|| fallback_metadata(&result));
    Some((result, metadata))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algorithms::hk2017::ehz_capacity;
    use crate::geom::known_polytopes;

    #[test]
    fn exact_resolution_upgrades_known_winner() {
        let kp = known_polytopes::simplex();
        let result = ehz_capacity(&kp.polytope).expect("ehz_capacity should succeed");
        let orbit = solve_orbit_sigma(
            &kp.polytope,
            &result.result.best_permutation,
            OrbitSolveBackend::SaddlePoint,
        )
        .expect("saddle-point solve should succeed");

        let exact = resolve_orbit_exact(&kp.polytope, &orbit)
            .expect("exact fallback should certify the known winner");

        assert_eq!(exact.admissibility, OrbitAdmissibility::AdmissibleExact);
        assert_eq!(exact.sigma, orbit.sigma);
        assert_eq!(exact.q_error_bound, 0.0);
        assert_eq!(exact.action_lower, exact.action_upper);
    }

    #[test]
    fn boundsafe_resolves_indeterminate_argmin() {
        let kp = known_polytopes::simplex();
        let result = ehz_capacity(&kp.polytope).expect("ehz_capacity should succeed");
        let mut orbit = solve_orbit_sigma(
            &kp.polytope,
            &result.result.best_permutation,
            OrbitSolveBackend::SaddlePoint,
        )
        .expect("saddle-point solve should succeed");
        orbit.admissibility = OrbitAdmissibility::IndeterminateF64;

        let mut orbits = vec![orbit];
        resolve_orbits_for_guarantee(&kp.polytope, &mut orbits, OrbitGuaranteeMode::BoundSafe)
            .expect("boundsafe resolution should succeed");

        assert_eq!(orbits.len(), 1);
        assert_eq!(orbits[0].admissibility, OrbitAdmissibility::AdmissibleExact);
        assert_eq!(orbits[0].action_lower, orbits[0].action_upper);
    }
}

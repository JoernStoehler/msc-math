//! Shared orbit-search result types for capacity algorithms.
//!
//! This module is the common result-layer scaffold for the `hk2017`,
//! `hk2017_unpruned`, and `billiard` frontends. It deliberately separates:
//!
//! - orbit payload data (`OrbitKktData`)
//! - search-level guarantees and backend choice
//! - search/recovery error classification

use crate::geom::polytope::Polytope4D;
use crate::geom::rational_arithmetic::rational_to_f64;
use crate::kkt::classify_margin;
use crate::kkt::rational_solver::solve_kkt_exact;
use crate::kkt::saddle_point_solver::{solve_kkt_for, KktOutcome, KktResult, EPS_Q_POSITIVE};

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
    ///
    /// This choice is currently scaffold-only at the shared
    /// `solve_orbit_sigma` surface and returns `UnsupportedBackend`.
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

impl OrbitKktData {
    /// Unordered participating facet set derived from `sigma`.
    pub fn best_subset(&self) -> Vec<usize> {
        let mut subset = self.sigma.clone();
        subset.sort_unstable();
        subset
    }
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

impl OrbitSearchResult {
    /// Canonical best/minimum orbit used by scalar-style consumers.
    ///
    /// The constructor guarantees `orbits` is nonempty.
    pub fn best_orbit(&self) -> &OrbitKktData {
        self.orbits
            .iter()
            .filter(|orbit| {
                matches!(
                    orbit.admissibility,
                    OrbitAdmissibility::AdmissibleF64 | OrbitAdmissibility::AdmissibleExact
                )
            })
            .min_by(|a, b| a.action.total_cmp(&b.action))
            .unwrap_or(&self.orbits[0])
    }

    /// Convenience scalar alias for ordinary callers that still think in terms
    /// of one returned capacity value.
    pub fn capacity(&self) -> f64 {
        self.min_action
    }

    /// Convenience access to the best orbit's sigma.
    pub fn best_sigma(&self) -> &[usize] {
        &self.best_orbit().sigma
    }

    /// Convenience access to the best orbit's beta vector.
    pub fn best_beta(&self) -> &[f64] {
        &self.best_orbit().beta
    }

    /// Unordered participating facet set of the best orbit.
    pub fn best_subset(&self) -> Vec<usize> {
        self.best_orbit().best_subset()
    }
}

/// Search-level failure classification for the shared orbit collectors.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OrbitSearchError {
    /// No admissible orbit remained after filtering and requested fallback.
    NoAdmissibleOrbit,
    /// The requested backend is not yet supported at the shared collector
    /// surface.
    UnsupportedBackend,
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

    let beta_margin = result.beta.iter().copied().fold(f64::INFINITY, f64::min);
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
    if exact.q_exact_f64 <= EPS_Q_POSITIVE {
        return None;
    }
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

fn resolve_orbit_exact(polytope: &Polytope4D, orbit: &OrbitKktData) -> Option<OrbitKktData> {
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

fn sort_orbits_by_lower_action(orbits: &mut [OrbitKktData]) {
    orbits.sort_by(|a, b| {
        a.action_lower
            .total_cmp(&b.action_lower)
            .then_with(|| a.action_upper.total_cmp(&b.action_upper))
            .then_with(|| a.action.total_cmp(&b.action))
    });
}

fn trim_orbits_to_gap(orbits: &mut Vec<OrbitKktData>, gap: f64) -> Result<(), OrbitSearchError> {
    let min_action_upper = orbits
        .iter()
        .map(|orbit| orbit.action_upper)
        .fold(f64::INFINITY, f64::min);
    let cutoff = min_action_upper + gap;
    orbits.retain(|orbit| orbit.action_lower <= cutoff);
    if orbits.is_empty() {
        return Err(OrbitSearchError::NoAdmissibleOrbit);
    }
    Ok(())
}

fn summarize_orbits(
    orbits: Vec<OrbitKktData>,
    iterations: u64,
) -> Result<OrbitSearchResult, OrbitSearchError> {
    let min_action = orbits
        .iter()
        .filter(|orbit| {
            matches!(
                orbit.admissibility,
                OrbitAdmissibility::AdmissibleF64 | OrbitAdmissibility::AdmissibleExact
            )
        })
        .map(|orbit| orbit.action)
        .min_by(|a, b| a.total_cmp(b))
        .ok_or(OrbitSearchError::NoAdmissibleOrbit)?;
    let min_action_lower = orbits
        .iter()
        .map(|orbit| orbit.action_lower)
        .fold(f64::INFINITY, f64::min);
    let min_action_upper = orbits
        .iter()
        .map(|orbit| orbit.action_upper)
        .fold(f64::INFINITY, f64::min);

    Ok(OrbitSearchResult {
        orbits,
        min_action,
        min_action_lower,
        min_action_upper,
        iterations,
    })
}

pub(crate) fn solve_sigma_stream(
    polytope: &Polytope4D,
    backend: OrbitSolveBackend,
    mut emit_sigma: impl FnMut(&mut dyn FnMut(&[usize])),
) -> Result<(Vec<OrbitKktData>, u64), OrbitSearchError> {
    let mut orbits = Vec::new();
    let mut iterations = 0u64;
    let mut fatal_error: Option<OrbitSearchError> = None;

    let mut visit = |sigma: &[usize]| {
        if fatal_error.is_some() {
            return;
        }
        iterations += 1;
        match solve_orbit_sigma(polytope, sigma, backend) {
            Ok(orbit) => orbits.push(orbit),
            Err(OrbitSolveError::Inadmissible) => {}
            Err(OrbitSolveError::UnsupportedBackend) => {
                fatal_error = Some(OrbitSearchError::UnsupportedBackend);
            }
            Err(OrbitSolveError::NumericalFailure) => {
                fatal_error = Some(OrbitSearchError::NumericalFailure);
            }
        }
    };

    emit_sigma(&mut visit);

    if let Some(err) = fatal_error {
        return Err(err);
    }
    if orbits.is_empty() {
        return Err(OrbitSearchError::NoAdmissibleOrbit);
    }

    Ok((orbits, iterations))
}

/// Aggregate solved orbit candidates with explicit admissibility guarantees.
///
/// This is the non-default postprocessing building block. Callers that need a
/// stronger guarantee than the ordinary `ehz_capacity*` routers should:
///
/// 1. enumerate sigma candidates with the algorithm-specific traversal helper,
/// 2. solve them with [`solve_orbit_sigma`],
/// 3. call this function with the chosen `gap` and `mode`.
pub fn aggregate_orbits(
    polytope: &Polytope4D,
    mut orbits: Vec<OrbitKktData>,
    iterations: u64,
    gap: f64,
    mode: OrbitGuaranteeMode,
) -> Result<OrbitSearchResult, OrbitSearchError> {
    if orbits.is_empty() {
        return Err(OrbitSearchError::NoAdmissibleOrbit);
    }

    resolve_orbits_for_guarantee(polytope, &mut orbits, mode)?;
    trim_orbits_to_gap(&mut orbits, gap)?;
    if mode == OrbitGuaranteeMode::AllSafe {
        resolve_orbits_for_guarantee(polytope, &mut orbits, mode)?;
    }
    sort_orbits_by_lower_action(&mut orbits);
    summarize_orbits(orbits, iterations)
}

/// Shared unresolved-f64 aggregation seam for the root scalar wrappers.
///
/// This keeps the public `ehz_capacity*` family on the f64 search path and
/// surfaces unresolved intervals instead of forcing exact fallback on
/// approximate geometries. Non-default callers that want stronger guarantees
/// use [`aggregate_orbits`] explicitly.
pub(crate) fn aggregate_orbits_f64_only(
    gap: f64,
    mut orbits: Vec<OrbitKktData>,
    iterations: u64,
) -> Result<OrbitSearchResult, OrbitSearchError> {
    if orbits.is_empty() {
        return Err(OrbitSearchError::NoAdmissibleOrbit);
    }

    trim_orbits_to_gap(&mut orbits, gap)?;
    sort_orbits_by_lower_action(&mut orbits);
    summarize_orbits(orbits, iterations)
}

#[cfg(test)]
#[path = "test_orbit_search.rs"]
mod test_orbit_search;

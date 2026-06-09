//! Shared orbit-search result types for capacity algorithms.
//!
//! This module is the common result-layer scaffold for the `hk2017`,
//! `hk2017_unpruned`, and `billiard` frontends. It deliberately separates:
//!
//! - orbit payload data (`OrbitKktData`)
//! - search-level guarantees
//! - search/recovery error classification

use crate::geom::rational_arithmetic::rational_to_f64;
use crate::kkt::classify_margin;
use crate::kkt::rational_solver::{solve_kkt_exact, ExactKktResult};
use crate::kkt::saddle_point_solver::{
    solve_kkt_for_dual_vertices, KktOutcome, KktResult, EPS_Q_POSITIVE,
};
use nalgebra::Vector4;
use num_rational::BigRational;
use num_traits::{One, Signed, Zero};
use serde::{Deserialize, Serialize};

/// Admissibility status of a numerically solved orbit candidate.
///
/// Known-inadmissible candidates are discarded before they become
/// `OrbitKktData`. This enum therefore describes only the surviving states.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
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

/// Exact orbit-set contract for certified aggregation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CertifiedOrbitSetMode {
    /// Return the exact capacity and all exact minimizers.
    MinimizersOnly,
    /// Return the exact capacity, all exact minimizers, and all exact orbits
    /// whose action lies in `capacity_exact + action_gap_exact`.
    GapWindow,
}

/// Solved orbit payload used by all capacity frontends.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

/// Exact rational certificate for one admissible sigma.
#[derive(Clone, Debug, PartialEq)]
pub struct CertifiedOrbitKktData {
    /// Cyclic facet sequence σ. Entries are distinct facet indices, not a full
    /// permutation of `0..F`.
    pub sigma: Vec<usize>,
    /// Exact β aligned with σ: `beta_exact[i]` belongs to `sigma[i]`.
    pub beta_exact: Vec<BigRational>,
    /// Exact Q value from the rational KKT system.
    pub q_exact: BigRational,
    /// Exact action `1 / (2Q)`.
    pub action_exact: BigRational,
    /// f64 convenience copy of `action_exact`.
    pub action: f64,
}

/// Exact rational result for callers that need a certified orbit set, not only
/// an interval-safe scalar capacity.
#[derive(Clone, Debug, PartialEq)]
pub struct CertifiedOrbitSearchResult {
    /// Exact capacity, i.e. the minimum exact action among certified candidates.
    pub capacity_exact: BigRational,
    /// f64 convenience copy of `capacity_exact`.
    pub capacity: f64,
    /// Exact action gap requested by the caller.
    pub action_gap_exact: BigRational,
    /// Exact minimizers, sorted by sigma.
    pub minimizers: Vec<CertifiedOrbitKktData>,
    /// Returned exact orbit set. In `MinimizersOnly` mode this equals
    /// `minimizers`; in `GapWindow` mode it contains all certified orbits with
    /// action at most `capacity_exact + action_gap_exact`.
    pub orbits: Vec<CertifiedOrbitKktData>,
    /// Number of sigma candidates examined by the search frontend.
    pub iterations: u64,
    /// Number of f64 candidates that required exact KKT resolution while
    /// certifying this result.
    pub exact_resolutions: usize,
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
    /// The numerical backend failed before the requested guarantee could be
    /// established.
    NumericalFailure,
    /// Exact fallback was required by the active guarantee mode but failed.
    ExactFallbackFailure,
    /// The caller supplied a negative action gap for a certified orbit set.
    InvalidGap,
}

/// Failure classification for solving a single sigma into `OrbitKktData`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OrbitSolveError {
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

/// Solve one sigma from flat f64 dual vertices into the shared orbit payload.
///
/// Input contract: `sigma` indexes the same ordered facet set as
/// `dual_vertices`.
///
pub fn solve_orbit_sigma_saddle_point(
    dual_vertices: &[Vector4<f64>],
    sigma: &[usize],
) -> Result<OrbitKktData, OrbitSolveError> {
    let outcome = solve_kkt_for_dual_vertices(dual_vertices, sigma);
    solve_saddle_point_sigma(sigma, outcome)
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

fn exact_orbit_from_sigma_with_dual_vertices_exact(
    dual_vertices_exact: &[[BigRational; 4]],
    sigma: &[usize],
    old_mu: Option<[f64; 4]>,
    old_xi: Option<f64>,
) -> Option<OrbitKktData> {
    let exact = solve_kkt_exact(dual_vertices_exact, sigma)?;
    if !exact_kkt_result_satisfies_constraints_with_dual_vertices_exact(
        dual_vertices_exact,
        sigma,
        &exact,
    ) {
        return None;
    }
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

fn exact_action_from_q(q_exact: &BigRational) -> BigRational {
    BigRational::one() / (q_exact.clone() + q_exact.clone())
}

fn certified_orbit_from_sigma_with_dual_vertices_exact(
    dual_vertices_exact: &[[BigRational; 4]],
    sigma: &[usize],
) -> Option<CertifiedOrbitKktData> {
    let exact = solve_kkt_exact(dual_vertices_exact, sigma)?;
    if !exact_kkt_result_satisfies_constraints_with_dual_vertices_exact(
        dual_vertices_exact,
        sigma,
        &exact,
    ) {
        return None;
    }
    let action_exact = exact_action_from_q(&exact.q_exact);
    let action = rational_to_f64(&action_exact);

    Some(CertifiedOrbitKktData {
        sigma: sigma.to_vec(),
        beta_exact: exact.beta,
        q_exact: exact.q_exact,
        action_exact,
        action,
    })
}

fn exact_kkt_result_satisfies_constraints_with_dual_vertices_exact(
    dual_vertices_exact: &[[BigRational; 4]],
    sigma: &[usize],
    exact: &ExactKktResult,
) -> bool {
    if exact.beta.len() != sigma.len()
        || !exact.beta.iter().all(|beta| beta.is_positive())
        || !exact.q_exact.is_positive()
    {
        return false;
    }

    let beta_sum = exact
        .beta
        .iter()
        .cloned()
        .fold(num_rational::BigRational::zero(), |acc, beta| acc + beta);
    if beta_sum != num_rational::BigRational::one() {
        return false;
    }

    (0..4).all(|d| {
        sigma
            .iter()
            .zip(exact.beta.iter())
            .map(|(&facet, beta)| beta * &dual_vertices_exact[facet][d])
            .fold(num_rational::BigRational::zero(), |acc, entry| acc + entry)
            .is_zero()
    })
}

fn resolve_orbit_exact_with_dual_vertices_exact(
    dual_vertices_exact: &[[BigRational; 4]],
    orbit: &OrbitKktData,
) -> Option<OrbitKktData> {
    exact_orbit_from_sigma_with_dual_vertices_exact(
        dual_vertices_exact,
        &orbit.sigma,
        orbit.mu,
        orbit.xi,
    )
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

fn resolve_indices_exact_with_dual_vertices_exact(
    dual_vertices_exact: &[[BigRational; 4]],
    orbits: &mut Vec<OrbitKktData>,
    mut indices: Vec<usize>,
) -> Result<(), OrbitSearchError> {
    indices.sort_unstable();
    indices.dedup();

    for idx in indices.into_iter().rev() {
        let upgraded =
            resolve_orbit_exact_with_dual_vertices_exact(dual_vertices_exact, &orbits[idx]);
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

fn resolve_boundsafe_with_dual_vertices_exact(
    dual_vertices_exact: &[[BigRational; 4]],
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

        resolve_indices_exact_with_dual_vertices_exact(dual_vertices_exact, orbits, needs_exact)?;
    }
}

fn resolve_minimasafe_with_dual_vertices_exact(
    dual_vertices_exact: &[[BigRational; 4]],
    orbits: &mut Vec<OrbitKktData>,
) -> Result<(), OrbitSearchError> {
    loop {
        resolve_boundsafe_with_dual_vertices_exact(dual_vertices_exact, orbits)?;
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

        resolve_indices_exact_with_dual_vertices_exact(dual_vertices_exact, orbits, needs_exact)?;
    }
}

fn resolve_allsafe_with_dual_vertices_exact(
    dual_vertices_exact: &[[BigRational; 4]],
    orbits: &mut Vec<OrbitKktData>,
) -> Result<(), OrbitSearchError> {
    let needs_exact: Vec<usize> = orbits
        .iter()
        .enumerate()
        .filter_map(|(idx, orbit)| {
            (orbit.admissibility == OrbitAdmissibility::IndeterminateF64).then_some(idx)
        })
        .collect();
    resolve_indices_exact_with_dual_vertices_exact(dual_vertices_exact, orbits, needs_exact)
}

pub(crate) fn resolve_orbits_for_guarantee_with_dual_vertices_exact(
    dual_vertices_exact: &[[BigRational; 4]],
    orbits: &mut Vec<OrbitKktData>,
    mode: OrbitGuaranteeMode,
) -> Result<(), OrbitSearchError> {
    if orbits.is_empty() {
        return Err(OrbitSearchError::NoAdmissibleOrbit);
    }

    match mode {
        OrbitGuaranteeMode::BoundSafe => {
            resolve_boundsafe_with_dual_vertices_exact(dual_vertices_exact, orbits)
        }
        OrbitGuaranteeMode::MinimaSafe => {
            resolve_minimasafe_with_dual_vertices_exact(dual_vertices_exact, orbits)
        }
        OrbitGuaranteeMode::AllSafe => {
            resolve_allsafe_with_dual_vertices_exact(dual_vertices_exact, orbits)
        }
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

fn conservative_f64_upper_bound(exact: &BigRational) -> f64 {
    let mut value = rational_to_f64(exact);
    for _ in 0..8 {
        value = if value.is_finite() && value >= 0.0 {
            f64::from_bits(value.to_bits() + 1)
        } else {
            value
        };
    }
    value
}

fn sort_certified_orbits_by_action(orbits: &mut [CertifiedOrbitKktData]) {
    orbits.sort_by(|a, b| {
        a.action_exact
            .cmp(&b.action_exact)
            .then_with(|| a.sigma.cmp(&b.sigma))
    });
}

fn sort_certified_orbits_by_sigma(orbits: &mut [CertifiedOrbitKktData]) {
    orbits.sort_by(|a, b| a.sigma.cmp(&b.sigma));
}

/// Aggregate solved orbit candidates into an exact rational orbit-set result
/// using flat exact dual vertices for fallback certification.
///
/// Input contract: every candidate sigma must index the same ordered facet set
/// as `dual_vertices_exact`. In the standard f64-then-exact path, candidates
/// are produced from the matching f64 projection of these exact dual vertices.
pub fn aggregate_certified_orbits_with_dual_vertices_exact(
    dual_vertices_exact: &[[BigRational; 4]],
    mut candidates: Vec<OrbitKktData>,
    iterations: u64,
    action_gap_exact: BigRational,
    mode: CertifiedOrbitSetMode,
) -> Result<CertifiedOrbitSearchResult, OrbitSearchError> {
    if candidates.is_empty() {
        return Err(OrbitSearchError::NoAdmissibleOrbit);
    }
    if action_gap_exact.is_negative() {
        return Err(OrbitSearchError::InvalidGap);
    }

    sort_orbits_by_lower_action(&mut candidates);
    let mut certified: Vec<Option<CertifiedOrbitKktData>> = vec![None; candidates.len()];
    let mut rejected = vec![false; candidates.len()];
    let mut exact_resolutions = 0usize;

    let mut capacity_exact = None;
    for (idx, candidate) in candidates.iter().enumerate() {
        exact_resolutions += 1;
        match certified_orbit_from_sigma_with_dual_vertices_exact(
            dual_vertices_exact,
            &candidate.sigma,
        ) {
            Some(exact_orbit) => {
                capacity_exact = Some(exact_orbit.action_exact.clone());
                certified[idx] = Some(exact_orbit);
                break;
            }
            None => rejected[idx] = true,
        }
    }
    let mut capacity_exact = capacity_exact.ok_or(OrbitSearchError::NoAdmissibleOrbit)?;

    let resolution_gap = match mode {
        CertifiedOrbitSetMode::MinimizersOnly => BigRational::zero(),
        CertifiedOrbitSetMode::GapWindow => action_gap_exact.clone(),
    };

    loop {
        let threshold_exact = capacity_exact.clone() + resolution_gap.clone();
        let threshold_f64 = conservative_f64_upper_bound(&threshold_exact);
        let needs_resolution: Vec<usize> = candidates
            .iter()
            .enumerate()
            .filter_map(|(idx, candidate)| {
                (certified[idx].is_none()
                    && !rejected[idx]
                    && candidate.action_lower <= threshold_f64)
                    .then_some(idx)
            })
            .collect();

        if needs_resolution.is_empty() {
            break;
        }

        for idx in needs_resolution {
            exact_resolutions += 1;
            match certified_orbit_from_sigma_with_dual_vertices_exact(
                dual_vertices_exact,
                &candidates[idx].sigma,
            ) {
                Some(exact_orbit) => {
                    if exact_orbit.action_exact < capacity_exact {
                        capacity_exact = exact_orbit.action_exact.clone();
                    }
                    certified[idx] = Some(exact_orbit);
                }
                None => rejected[idx] = true,
            }
        }
    }

    let window_cutoff = capacity_exact.clone() + action_gap_exact.clone();
    let mut minimizers: Vec<CertifiedOrbitKktData> = certified
        .iter()
        .filter_map(|orbit| orbit.as_ref())
        .filter(|orbit| orbit.action_exact == capacity_exact)
        .cloned()
        .collect();
    if minimizers.is_empty() {
        return Err(OrbitSearchError::NoAdmissibleOrbit);
    }
    sort_certified_orbits_by_sigma(&mut minimizers);

    let mut orbits: Vec<CertifiedOrbitKktData> = match mode {
        CertifiedOrbitSetMode::MinimizersOnly => minimizers.clone(),
        CertifiedOrbitSetMode::GapWindow => certified
            .into_iter()
            .flatten()
            .filter(|orbit| orbit.action_exact <= window_cutoff)
            .collect(),
    };
    sort_certified_orbits_by_action(&mut orbits);

    Ok(CertifiedOrbitSearchResult {
        capacity: rational_to_f64(&capacity_exact),
        capacity_exact,
        action_gap_exact,
        minimizers,
        orbits,
        iterations,
        exact_resolutions,
    })
}

pub(crate) fn solve_sigma_stream_with_dual_vertices(
    dual_vertices: &[Vector4<f64>],
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
        match solve_orbit_sigma_saddle_point(dual_vertices, sigma) {
            Ok(orbit) => orbits.push(orbit),
            Err(OrbitSolveError::Inadmissible) => {}
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
/// 2. solve them with [`solve_orbit_sigma_saddle_point`],
/// 3. call this function with flat exact dual vertices for the exact fallback
///    required by `mode`.
///
/// Input contract: every orbit sigma must index the same ordered facet set as
/// `dual_vertices_exact`. In the standard f64-then-exact path, the orbits are
/// solved from the matching f64 projection of these exact dual vertices.
pub fn aggregate_orbits_with_dual_vertices_exact(
    dual_vertices_exact: &[[BigRational; 4]],
    mut orbits: Vec<OrbitKktData>,
    iterations: u64,
    gap: f64,
    mode: OrbitGuaranteeMode,
) -> Result<OrbitSearchResult, OrbitSearchError> {
    if orbits.is_empty() {
        return Err(OrbitSearchError::NoAdmissibleOrbit);
    }

    resolve_orbits_for_guarantee_with_dual_vertices_exact(dual_vertices_exact, &mut orbits, mode)?;
    trim_orbits_to_gap(&mut orbits, gap)?;
    if mode == OrbitGuaranteeMode::AllSafe {
        resolve_orbits_for_guarantee_with_dual_vertices_exact(
            dual_vertices_exact,
            &mut orbits,
            mode,
        )?;
    }
    sort_orbits_by_lower_action(&mut orbits);
    summarize_orbits(orbits, iterations)
}

#[cfg(test)]
#[path = "test_orbit_search.rs"]
mod test_orbit_search;

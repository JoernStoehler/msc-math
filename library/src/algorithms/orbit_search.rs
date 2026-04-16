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

use crate::geom::polytope::Polytope4D;
use crate::kkt::saddle_point_solver::{solve_kkt_for, KktOutcome, KktResult, EPS_Q_POSITIVE};
use crate::kkt::classify_margin;

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

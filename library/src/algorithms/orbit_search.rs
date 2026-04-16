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

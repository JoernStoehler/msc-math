<!--
Purpose: durable design note for the shared capacity/orbit result surface used
by hk2017, hk2017_unpruned, and billiard.
Context: created during the repo-maintainability program after Jörn/top-level
discussion of the current result layering. This note records what is already
settled, what remains open, and what later implementation sessions should
change. It is a design handoff; later status bullets record which pieces of the
target API have already landed in code.
-->

# Capacity/Orbit Result API Plan

## Status

- State: first design pass from chat discussion on 2026-04-16, then Packet 1
  scaffold work on `capacity-result-api-exec`.
- Scope: shared capacity/orbit result layering for `hk2017`,
  `hk2017_unpruned`, and `billiard`; orbit/KKT payloads; orbit recovery;
  derivatives; and Clarke-subdifferential support.
- Out of scope here: broad data-flow cleanup, thesis prose, and the full
  execution DAG for all maintainability work.
- Landed code scaffold:
  - `library/src/algorithms/orbit_search.rs` now defines the shared public
    types `OrbitAdmissibility`, `OrbitGuaranteeMode`, `OrbitSolveBackend`,
    `OrbitKktData`, `OrbitSearchResult`, `OrbitSearchError`, and
    `GeometricOrbitError`.
  - These types are re-exported from `library/src/algorithms/mod.rs` and
    `library/src/lib.rs`.
  - Packet 2 first slice: `solve_orbit_sigma(...)` now exists on the shared
    result-layer module and the current HK2017/billiard solver bridges route
    through it for the saddle-point backend.
  - Packet 2 later slices: the current HK2017/billiard frontends now also
    share `collect_legacy_capacity(...)`, and `orbit_search.rs` now contains
    the first exact-fallback helpers that upgrade or drop `IndeterminateF64`
    orbits under `BoundSafe`, `MinimaSafe`, and `AllSafe`.
  - Packet 2 collector slice: public `hk2017_minimum_orbits(...)`,
    `hk2017_minimum_orbits_unpruned(...)`, and
    `billiard_minimum_orbits(...)` now exist on top of the shared collector
    seam. `OrbitSearchError` now includes an explicit
    `UnsupportedBackend` variant, and billiard wraps shared search failures in
    `BilliardOrbitSearchError`.
  - Packet 3 first slice: `library/src/derivatives.rs` now defines
    `OrbitGradientA`, `ClarkeSubdiffA`, and `DerivativeError`, plus helper
    functions on both the current `KktResult` seam and the new
    `OrbitKktData` seam. The first migrated consumer packages are
    `exp-combinatorial-cells` and `exp-hko-local-maximum`
    (`hko-second-order`).
  - Packet 3 second slice: the same `KktResult`-level helper now backs the
    buildable ascent binaries in `exp-sys-landscape` and
    `exp-hko-local-maximum` (`hko-cut-and-ascent`) as well.

## Goal

- Give `hk2017`, `hk2017_unpruned`, and `billiard` one shared orbit/result
  layer in `library/`, while keeping their search frontends separate.
- Make the richer HK2017-family algorithm output available from `library/`
  without forcing every caller onto a heavy report surface.
- Remove repeated experiment-local instrumentation for "collect all minimum or
  near-minimum orbits" and "re-solve one sigma to recover beta/mu".
- Keep the hot default API cheap and predictable.
- Expose enough orbit/KKT data that downstream consumers can recover geometric
  orbits, compute derivatives, and build Clarke-subdifferential data without
  re-solving when the data is already known.

## Current Code State

- `ehz_capacity` / `ehz_capacity_unpruned` still return `Option<EhzResult>`.
- The richer public collector entrypoints now exist:
  - `hk2017_minimum_orbits(...)`
  - `hk2017_minimum_orbits_unpruned(...)`
  - `billiard_minimum_orbits(...)`
- `EhzResult` currently stores:
  - `result: CapacityResult`
  - `best_subset: Vec<usize>`
- `CapacityResult` currently stores:
  - `capacity`
  - `capacity_uncertain`
  - `best_permutation`
  - `best_beta`
  - `iterations`
- `recover_and_verify(polytope, &ehz_result)` is a separate pass that turns the
  best orbit summary into geometric-orbit data (currently named
  `OrbitRecovery` in code).
- `capacity_derivatives_a(...)` is a lower-level derivative routine on
  orbit/KKT inputs (`beta`, `q`, `mu`, `sigma`, `dual_vertices`), not on
  `EhzResult`.
- Several experiments duplicate "collect all certified orbits" or
  "collect minimum/tied orbits" logic locally.

## User-Specified Correct Algorithm Policy

Jörn stated a stronger algorithm policy than the current production
implementation. This section records that policy as the target semantics for
later refactor/design work.

Target algorithm sketch:

1. Enumerate all candidate `sigma` values except those rejected by the exact
   adjacency argument.
2. For each surviving `sigma`, compute the numerical critical point of the
   projected problem on the affine constraint space.
   - Jörn prefers the "solve constraints -> project to affine space ->
     eigendecompose reduced Hessian" route because the error-bound proofs are
     cleaner there.
   - The saddle-point route may still be acceptable if it supports the same
     guarantees.
3. If the critical point is known inadmissible, discard it.
   - Rationale: then the maximum for that combinatorics lies on the boundary
     and is handled by a shorter `sigma`.
4. If `Q < 0`, discard it.
   - Rationale: this is either nonsense or a numerically tiny positive `Q`
     whose action `1/(2Q)` would be huge anyway.
5. Keep the candidate list sorted by the lower action bound
   `(action - error_bound)` and note the first orbit that is already known
   admissible, if any.
6. Discard items whose best possible action is already outside the requested
   final window, e.g. items with
   `action - error_bound > known_admissible.action - known_admissible.error_bound + gap`.
7. Resolve indeterminate admissibility lazily with the rational solver when the
   caller needs stronger guarantees.
   - If the rational solver certifies inadmissible, discard the orbit.
   - If it certifies admissible, keep it and upgrade its error bound to zero.
8. Aggregate the minimum action as an interval:
   - upper bound = lowest orbit upper bound
   - lower bound = highest orbit lower bound among the retained list

This policy is stronger than the current production `CapacityAccumulator`
story. In particular, the current code tracks indeterminate candidates only via
the scalar `capacity_uncertain`; it does not yet retain them as orbit payloads
or lazily resolve them with the rational solver in the hot HK2017 path.

## Settled In Discussion

These points were explicitly agreed in chat and should not be re-opened unless
new repo evidence contradicts them.

- Keep one thin scalar/default API in addition to the richer collectors; do not
  force every caller to pay for eager all-orbit collection or eager geometric
  orbit recovery.
- The current `ehz_capacity` / `ehz_capacity_unpruned` wrappers are not
  protected as names or exact result shapes. They may be deleted or replaced
  during migration as long as the repo still has one cheap scalar/default
  entrypoint.
- The richer HK2017 surface should return a sorted list of solved orbit/KKT
  payloads, not only one best orbit.
- Returning only one best orbit is the wrong boundary for the richer
  API. If the minimum-orbit list is nonempty, simple consumers can take
  `orbits[0]` themselves.
- The richer collector should use an explicit action-gap parameter rather than a
  magic default such as `1e-3`.
- Gap semantics should be asymmetric:
  - completeness guarantee: every orbit with `action <= min_action + gap` is
    returned
  - no exclusivity guarantee: near-cutoff extras may also be returned because
    of numerical tolerance
- `False` / known-inadmissible candidates should be discarded.
- Indeterminate admissibility should not be collapsed away into a scalar-only
  side channel. Orbit-level admissibility status matters for the richer API.
- The stored orbit sequence should be named `sigma`, not `permutation`.
  Rationale: it matches the math and avoids implying a full permutation of
  `0..F`.
- The richer orbit payload should store `beta`; downstream consumers use it
  often enough that repeated one-sigma re-solves are avoidable noise.
- The richer orbit payload should also store `beta_margin = min(beta)` as a
  convenience scalar for admissibility/debugging/logging consumers.
- The richer search API should take an explicit guarantee-mode parameter so the
  search layer knows which indeterminate candidates require exact resolution.
- `subset` should not be stored in the richer orbit payload. It is derived from
  `sigma` by sorting.
- Clarke-subdifferential support should move into `library/`; a primitive data
  type such as an ordered list of per-orbit gradients is the intended first
  surface.
- Orbit-level derivative/subdifferential helpers are worth adding when they
  eliminate repeated glue that many consumers would otherwise rebuild around the
  low-level derivative primitive.
- The library search result should stay small. Experiment-specific search
  diagnostics should remain experiment-local by copying or lightly adapting the
  search loop rather than expanding the library result type with ad hoc
  metrics.

## Proposed Core Types

The current leading design is:

```rust
pub enum OrbitAdmissibility {
    AdmissibleF64,
    IndeterminateF64,
    AdmissibleExact,
}

pub enum OrbitGuaranteeMode {
    BoundSafe,
    MinimaSafe,
    AllSafe,
}

pub enum OrbitSolveBackend {
    Projected,
    SaddlePoint,
}

pub struct OrbitKktData {
    /// Cyclic facet sequence σ. Distinct facet indices, not a full
    /// permutation of 0..F.
    pub sigma: Vec<usize>,
    /// β aligned with σ: beta[i] belongs to sigma[i].
    pub beta: Vec<f64>,
    /// Convenience scalar: min(beta).
    pub beta_margin: f64,
    /// Opinionated scalar action summary chosen by the producer.
    /// Current rule: min(action[k] for admissible returned k).
    pub action: f64,
    /// Lower endpoint of the action interval, guaranteed up to minor rounding.
    pub action_lower: f64,
    /// Upper endpoint of the action interval, guaranteed up to minor rounding.
    pub action_upper: f64,
    /// Public API name for the corrected Q value used by current code as
    /// `q_corrected`.
    pub q: f64,
    /// Closure multiplier when the chosen backend/path provides it.
    pub mu: Option<[f64; 4]>,
    /// Normalization multiplier when the chosen backend/path provides it.
    pub xi: Option<f64>,
    pub q_error_bound: f64,
    pub admissibility: OrbitAdmissibility,
}

pub struct OrbitSearchResult {
    /// Nonempty and sorted by lower action bound ascending.
    pub orbits: Vec<OrbitKktData>,
    /// Canonical single-f64 summary chosen from admissible returned orbits.
    pub min_action: f64,
    /// Lower bound for the minimum action across retained candidates.
    pub min_action_lower: f64,
    /// Upper bound for the minimum action across retained candidates.
    pub min_action_upper: f64,
    pub iterations: u64,
}

pub enum OrbitSearchError {
    NoAdmissibleOrbit,
    UnsupportedBackend,
    NumericalFailure,
    ExactFallbackFailure,
}

pub enum GeometricOrbitError {
    DegenerateOrbit,
    LinearSolveFailure,
    VerificationFailed,
}
```

Notes:

- `sigma.len() == beta.len()`.
- `beta_margin = min(beta)` is stored explicitly as a convenience field, not
  because the full beta-side error geometry is settled.
- When present, `mu` has fixed shape `[f64; 4]`, unlike `beta`.
- The payload should expose all three views explicitly:
  - `action`: opinionated producer-chosen scalar
  - `action_lower`: lower endpoint, guaranteed up to minor rounding
  - `action_upper`: upper endpoint, guaranteed up to minor rounding
- The user-specified algorithm sorts and filters by lower action bound, so the
  result surface uses explicit action intervals.
- `min_action_lower` and `min_action_upper` are mandatory fields, not optional
  conveniences; the aggregate minimum-action interval is part of the contract.
- `min_action` should also exist so ordinary consumers do not have to choose
  their own scalar representative.
  Current rule: `min_action = min(action[k] for admissible returned k)`.
- `subset` is intentionally omitted; derive it from `sigma` when needed.

## Field Evidence From Current Consumers

Read-only scan on 2026-04-16 across the current library and experiment
consumers found this concrete demand surface:

- Strong evidence for storing:
  - `sigma`
    - orbit recovery uses the facet order directly
      ([orbit_recovery.rs](</workspaces/msc-math/library/src/algorithms/hk2017/orbit_recovery.rs:95>))
    - derivative assembly re-solves from `best_permutation`
      ([derivatives.rs](</workspaces/msc-math/library/src/derivatives.rs:290>))
    - cached minimum-sigma reload in orbit-recovery verification
      ([orbit-recovery/main.rs](</workspaces/msc-math/experiments/verification/orbit-recovery/main.rs:603>))
  - `beta`
    - orbit recovery needs dwell-time coefficients
      ([orbit_recovery.rs](</workspaces/msc-math/library/src/algorithms/hk2017/orbit_recovery.rs:96>))
    - analytical derivatives and subdifferential experiments use it directly
      ([derivatives.rs](</workspaces/msc-math/library/src/derivatives.rs:39>),
      [gradient-analysis/main.rs](</workspaces/msc-math/experiments/hko-local-maximum/gradient-analysis/main.rs:253>),
      [numerics-subdifferential/main.rs](</workspaces/msc-math/experiments/numerics/gradient/numerics-subdifferential/main.rs:305>))
  - `q` as the public field name
    - live code currently uses the internal name `q_corrected` for the same `Q`
      scalar
      ([saddle_point_solver.rs](</workspaces/msc-math/library/src/kkt/saddle_point_solver.rs:207>),
      [billiard/mod.rs](</workspaces/msc-math/library/src/algorithms/billiard/mod.rs:179>),
      [tests_literature.rs](</workspaces/msc-math/library/src/algorithms/hk2017/tests_literature.rs:127>))
  - `action`
    - already the canonical scalar summary in result/caching code
      ([capacity_accumulator.rs](</workspaces/msc-math/library/src/algorithms/capacity_accumulator.rs:41>),
      [orbit-recovery/main.rs](</workspaces/msc-math/experiments/verification/orbit-recovery/main.rs:561>))
  - `mu` when available
    - analytical derivatives and system-gradient work need it directly
      ([derivatives.rs](</workspaces/msc-math/library/src/derivatives.rs:34>),
      [gradient-analysis/main.rs](</workspaces/msc-math/experiments/hko-local-maximum/gradient-analysis/main.rs:257>),
      [numerics-subdifferential/main.rs](</workspaces/msc-math/experiments/numerics/gradient/numerics-subdifferential/main.rs:312>))
  - `xi` when available
    - real but narrower dependency: convention conversion and KKT auditing
      ([gradient-analysis/main.rs](</workspaces/msc-math/experiments/hko-local-maximum/gradient-analysis/main.rs:196>),
      [derivatives.rs](</workspaces/msc-math/library/src/derivatives.rs:294>))
- Lower evidence for storing as first-class payload fields:
  - `beta_margin`
    - current code computes `min(beta)` locally where needed
      ([billiard/mod.rs](</workspaces/msc-math/library/src/algorithms/billiard/mod.rs:181>),
      [orbit_search.rs](</workspaces/msc-math/library/src/algorithms/orbit_search.rs:167>),
      [beta_feasibility.rs](</workspaces/msc-math/library/src/kkt/beta_feasibility.rs:49>))
  - `q_error_bound`
    - strong evidence on the producer/proof side, but no current downstream
      runtime consumer yet
      ([saddle_point_solver.rs](</workspaces/msc-math/library/src/kkt/saddle_point_solver.rs:561>))
  - `action_lower`, `action_upper`, `admissibility`
    - planned public-surface fields, but there is not yet live code that reads
      them because the richer API does not exist yet
- Current code consistently uses the internal name `q_corrected`; the richer
  public API renames that field to `q`.

Current design consequence:

- Keep `q` and `q_error_bound` as the fundamental `Q`-side fields in the
  richer public payload, with an explicit migration from today's internal
  `q_corrected` naming.
- Derive `q_lower` / `q_upper` from `q_error_bound`; do not add them as
  separate stored fields.
- Keep `action`, `action_lower`, and `action_upper` in the richer public
  payload/result even though current consumers cannot yet read them directly;
  that is part of making the interval contract explicit once the richer API
  exists.
- `beta_margin` stays as a convenience field. The reason to keep it is
  ergonomics/debugging, not strong existing consumer demand.

## Proposed Function Surface

Current leading signatures:

```rust
pub fn hk2017_minimum_orbits(
    polytope: &Polytope4D,
    gap: f64,
    mode: OrbitGuaranteeMode,
    backend: OrbitSolveBackend,
) -> Result<OrbitSearchResult, OrbitSearchError>;

pub fn hk2017_minimum_orbits_unpruned(
    polytope: &Polytope4D,
    gap: f64,
    mode: OrbitGuaranteeMode,
    backend: OrbitSolveBackend,
) -> Result<OrbitSearchResult, OrbitSearchError>;
```

```rust
pub fn billiard_minimum_orbits(
    polytope: &Polytope4D,
    gap: f64,
    mode: OrbitGuaranteeMode,
    backend: OrbitSolveBackend,
) -> Result<OrbitSearchResult, BilliardOrbitSearchError>;
```

Notes on the frontend split:

- `hk2017_minimum_orbits`: general search frontend.
- `hk2017_minimum_orbits_unpruned`: same family, different pruning policy.
- `billiard_minimum_orbits`: same result surface, but with Lagrangian-product
  validation plus the specialized bounded-length/block-structured `sigma`
  enumeration.
- The shared surface is the point of the refactor. The frontends diverge in
  input validation and candidate enumeration, not in the returned orbit/result
  model.
- The search frontend should also be able to toggle between the projected
  solver and the saddle-point solver so the repo can compare correctness,
  stability, and runtime on the same result surface.

```rust
pub enum OrbitSolveError {
    UnsupportedBackend,
    Inadmissible,
    NumericalFailure,
}

pub fn solve_orbit_sigma(
    polytope: &Polytope4D,
    sigma: &[usize],
    backend: OrbitSolveBackend,
) -> Result<OrbitKktData, OrbitSolveError>;
```

Notes on solver backends:

- Both primitive solvers should target the same returned orbit payload type so
  the search layer and downstream consumers can compare them directly.
- Do not encode missing backend-specific quantities with `NaN` sentinels.
- `Option` is acceptable here because `None` has one clear meaning: the chosen
  backend/path did not produce that multiplier data for this orbit payload.
- If later the projected path learns to reconstruct `mu` / `xi`, it can
  upgrade those fields from `None` to `Some(...)` without changing the payload
  shape.
- Current implementation status differs slightly from the target shape:
  `solve_orbit_sigma(..., OrbitSolveBackend::Projected)` currently returns
  `OrbitSolveError::UnsupportedBackend`, because the library projection solver
  does not yet expose the `q_error_bound` contract required by
  `OrbitKktData`.
- The public shared collectors currently surface that same limitation as
  `OrbitSearchError::UnsupportedBackend`.
- The exact-fallback helpers are now implemented against the current rational
  solver. They currently preserve any pre-existing numerical `mu` / `xi`
  values because the exact fallback path does not yet compute exact
  multipliers.

```rust
pub enum DerivativeError {
    MissingClosureMultiplier,
    EmptySubdifferential,
}

pub fn recover_and_verify_orbit(
    polytope: &Polytope4D,
    orbit: &OrbitKktData,
) -> Result<GeometricOrbit, GeometricOrbitError>;
```

```rust
pub fn capacity_derivatives_a_from_kkt_result(
    polytope: &Polytope4D,
    sigma: &[usize],
    kkt: &KktResult,
) -> OrbitGradientA;

pub fn capacity_derivatives_a_from_orbit(
    polytope: &Polytope4D,
    orbit: &OrbitKktData,
) -> Result<OrbitGradientA, DerivativeError>;
```

```rust
pub type OrbitGradientA = Vec<Vector4<f64>>;
pub type ClarkeSubdiffA = Vec<OrbitGradientA>;

pub fn capacity_subgradients_a(
    polytope: &Polytope4D,
    orbits: &[OrbitKktData],
) -> Result<ClarkeSubdiffA, DerivativeError>;

pub fn directional_derivative_a(
    grad: &[Vector4<f64>],
    direction: &[Vector4<f64>],
) -> f64;

pub fn clarke_directional_derivative_a(
    subdiff: &ClarkeSubdiffA,
    direction: &[Vector4<f64>],
) -> Result<f64, DerivativeError>;
```

The current codebase has these thin wrappers:

```rust
pub fn ehz_capacity(polytope: &Polytope4D) -> Option<EhzResult>;
pub fn ehz_capacity_unpruned(polytope: &Polytope4D) -> Option<EhzResult>;
```

They are not treated as protected names or result shapes. Jörn's current
preference is to mark them for deletion once the richer API lands, because that
makes migration explicit: remove the wrappers, let the build fail, then patch
callers until green. A short transitional wrapper phase is still allowed if it
materially simplifies a packet, but the real design goal is one cheap
scalar/default entrypoint plus the richer collectors, not preservation of these
particular wrapper functions.

## Shape/Contract Notes

- `hk2017_minimum_orbits(..., gap, mode)` returns all minimum-action orbits when
  `gap == 0.0`, subject to the "complete but not exact-only" tolerance contract.
- Orbit-level admissibility should be part of the returned payload:
  `AdmissibleF64`, `IndeterminateF64`, or `AdmissibleExact`. The current design
  no longer assumes a separate query-time `OrbitClass` filter is the right
  primary split.
- `min_action` should be the producer-chosen scalar summary, not a caller-chosen
  heuristic. Current rule: `min_action = min(action[k] for admissible returned k)`.
- The aggregate minimum-action interval is computed orbitwise, not by coupling
  intervals as if they estimated the same latent variable:
  - `min_action_lower = min_k action_lower[k]`
  - `min_action_upper = min_k action_upper[k]`
  If admissibility must be enforced exactly for these bounds, resolve the
  corresponding argmin orbit(s) when they are still `IndeterminateF64`.
- `Err(...)` should distinguish mathematical/numerical outcomes that were
  previously blurred together by `Option::None`.
- Current public collector behavior already reflects this:
  unsupported projected-backend calls fail explicitly rather than silently
  degrading to the saddle-point path.
- If the API later offers cheaper/weaker guarantee modes, ambiguity itself
  should normally not become an error in those modes. The cheaper mode should
  return the weaker result rather than fail just because exact resolution was
  skipped.
- `solve_orbit_sigma(...)` is currently expected to stay f64-only. Lazy exact
  admissibility resolution fits better one layer up, once the search code knows
  whether a given indeterminate orbit actually matters for the requested
  guarantee.
- The same rule applies to recovery: `recover_and_verify_orbit(...)` should use
  `Result<..., GeometricOrbitError>` rather than `Option<_>`, because callers
  may need to distinguish degenerate orbit data, linear-solve failure, and a
  recovered orbit that fails verification.
- Orbit gradients are indexed by facet `k = 0..F-1`, not by orbit step.
  Direction vectors for derivative helpers must have the same facet-indexed
  shape.
- The orbit-level derivative helper is part of the intended ergonomic surface:
  many downstream consumers want Clarke-subdifferential computations rather than
  repeated manual glue around the low-level `capacity_derivatives_a(...)`
  inputs.
- Current implementation nuance: the orbit-level helper returns
  `Result<..., DerivativeError>` rather than a bare gradient because
  `OrbitKktData.mu` is optional across backends. The repo now also exposes a
  `capacity_derivatives_a_from_kkt_result(...)` helper because many current
  experiment consumers sit on that seam today.
- `recover_and_verify_orbit` should stay a separate transform. Eager geometric
  recovery is not justified on the hot path by the current profiling evidence.

## Why This Shape

- Profiling says the expensive part is enumeration count, not per-orbit KKT
  payload storage. Single KKT solves are microsecond-scale; the overall capacity
  path is dominated by the number of candidate sigmas.
- Orbit recovery is a different stage and not on the hot default path.
- Several real consumers want `beta`, and some want `mu`, `q`, or
  `q_error_bound`; storing the solved payload once is cleaner than forcing
  repeated one-sigma adapters.
- Keeping the thin default API preserves the current simple entry point for
  callers that only need one best certified result.
- Current preferred downstream pipeline is:
  - cached rows store `dual_vertices`, provenance, and `sigma`-level orbit
    summaries (plus action/capacity convenience fields)
  - `dual_vertices -> Polytope4D`
  - `(polytope, sigma) -> OrbitKktData`
  - `(polytope, OrbitKktData) -> geometric orbit`
  - `(polytope, OrbitKktData) -> OrbitGradientA`
  - `(polytope, [OrbitKktData]) -> Clarke subdifferential` as a convenience
  So the cache does not need to persist the full KKT payload by default.

## Open Decisions

These points are not yet fully settled.

- How the implementation stages the two primitive solvers. Current direction:
  support both projected/eigendecompose and saddle-point behind a backend
  toggle, so the repo can compare which is faster and which behaves better on
  real cases while keeping one shared result surface.
  Current code fact after the Packet 2 collector slice: only the saddle-point
  backend is wired through the public collector entrypoints; projected remains
  blocked on exposing a compatible `Q`-bound/payload surface from
  `library/src/kkt/projection_solver.rs`.
- How beta-side uncertainty should be represented is deferred for now.
  Current state:
  - `beta` itself is still worth storing
  - a convenience scalar such as `beta_margin = min(beta)` may still be useful
  - but the shape/storage of any beta error bound is not settled and should not
    be faked as if a production-ready axis-aligned bound already exists
- Whether `GeometricOrbitError` should distinguish more specific verification
  failures (closure / on-facet / inside / action mismatch) or keep one coarse
  `VerificationFailed` variant plus metrics elsewhere.
- Whether the first implementation packet should expose only `BoundSafe` and
  `MinimaSafe`, or also ship `AllSafe` immediately.
- How much of the richer API should be `pub` now versus `pub(crate)` during an
  intermediate migration.
- How the API should talk about "exact" admissibility/certification long-term.
  Current code means exact = rational arithmetic, but later work may need
  broader exact number domains such as `Q`, `Q[sqrt(5)]`, or symbolic/trigonometric
  extensions like `Q[sqrt(5), cos(theta), sin(theta)]`. So the public API should
  avoid over-baking "exact == rationals only" into names or semantics if that
  would make future generalization awkward.
- Whether any cache should ever persist fuller `OrbitKktData` payloads remains
  open, but the current preferred default is lighter `sigma`-level storage plus
  a one-sigma re-solve when downstream orbit/KKT data is needed.

## Migration Plan

1. Introduce the shared orbit/KKT payload type and one-sigma solve helper.
2. Introduce the shared sorted minimum-orbit collector surface, with frontend
   entrypoints for `hk2017`, `hk2017_unpruned`, and `billiard`, each taking
   `(gap, OrbitGuaranteeMode)`.
3. Re-implement or adapt existing experiment-local collectors to use the new
   library API.
4. Add library-level Clarke-subdifferential helpers on lists of
   `OrbitKktData`.
5. Delete or aggressively de-emphasize `EhzResult` / thin `ehz_capacity`
   wrappers once callers can migrate to the richer result surface. A short
   wrapper phase is acceptable only as a staging tactic.
6. Update the durable repo docs after the code shape settles:
   - `TASKS.md` for the tracker state and remaining follow-ups
   - `ARCHITECTURE.md` if the public/result boundary described there changes
   - this note or its successor if implementation choices differ from the
     current design draft
7. Keep search diagnostics out of `OrbitSearchResult` unless a stable,
   shared metric surface emerges. Experiments that need diagnostics should own
   their local loop/result types.

## Working Guarantee-Mode Mapping

Current discussion lean: the consumer should choose the guarantee mode. The
interesting question is which modes are actually needed by current repo
consumers.

Definitions below use these per-orbit fields:

- `q_upper`: upper endpoint of the candidate's Q interval
- `q_lower`: lower endpoint of the candidate's Q interval
- `action_lower`, `action_upper`: derived action interval endpoints
- `admissibility`: `AdmissibleF64`, `IndeterminateF64`, or `AdmissibleExact`

The current algorithm sketch sorts by `action_lower` ascending and uses
exact-resolution only for indeterminate candidates that matter for the chosen
guarantee.

Provisional mode table:

| Mode | Concrete return condition | Current likely consumers |
| --- | --- | --- |
| `BoundSafe` | Ensure that the argmin orbit for `min_action_lower` and the argmin orbit for `min_action_upper` are admissible (`AdmissibleF64` or `AdmissibleExact`). If either argmin orbit is `IndeterminateF64`, resolve it exactly before return. After this pass, the reported bounds `min_action_lower` and `min_action_upper` are trustworthy. | ordinary capacity-value consumers such as correctness/conformality/invariance tests, random-sample/sys-landscape value collection, many `ehz_capacity`-style callers |
| `MinimaSafe` | Before returning, exact-resolve every `IndeterminateF64` candidate whose action interval intersects the minimum-action bound interval `[min_action_lower, min_action_upper]`. After this pass, every orbit that might be an exact minimum is admissible (`AdmissibleF64` or `AdmissibleExact`). | gradient / derivative / Clarke-subdifferential / geometric-orbit consumers |
| `AllSafe` | Before returning, exact-resolve every `IndeterminateF64` candidate that would otherwise remain in `orbits`. After this pass, every listed orbit is admissible (`AdmissibleF64` or `AdmissibleExact`). The list itself may still include non-minimizing orbits up to the caller's gap policy. | niche verification/reporting use; no strong current evidence that most callers need this by default |

Current repo evidence behind this mapping:

- Capacity-value consumers dominate the current simple `ehz_capacity` surface:
  correctness validation, random-sample sweeps, sys-landscape value collection,
  conformality/invariance tests, and many library tests only read the scalar
  capacity.
- Gradient/subdifferential consumers already do more work because they need
  near-minimum orbit sets and per-orbit derivatives:
  - `experiments/hko-local-maximum/gradient-analysis/main.rs`
  - `experiments/numerics/gradient/numerics-subdifferential/main.rs`
  - second-order / combinatorial-cells gradient uses
- Orbit-recovery validation and visualization also care about actual admissible
  near-minimum orbit combinatorics rather than only the scalar minimum:
  - `experiments/verification/orbit-recovery/main.rs`
  - `experiments/visualization/main/main.rs`
- `experiments/numerics/unknown-predicates/main.rs` is evidence that ambiguity
  detection matters in practice, but it is itself diagnostic tooling rather than
  the final consumer surface.

Current design lean from discussion:

- Many existing capacity-only consumers want something like `BoundSafe`.
- Gradient / Clarke-subdifferential / recovered-orbit consumers want something
  like `MinimaSafe`.
- `AllSafe` may still be useful, but there is not yet strong
  evidence that it should be the default for most production callers.
- Do not add a public `Heuristic` mode now. Current preference is to start with
  the safe modes only and add a weaker mode later only if a real consumer
  complains about the cost.
- The mode definitions should stay operational and candidate-based, not vague.
  A later implementation packet should be able to translate them directly into
  "which `IndeterminateF64` candidates must be sent through exact fallback
  before return?".

## Billiard Relation

- The billiard algorithm is structurally close to the intended HK2017 richer
  result surface: it also uses enumerate -> solve -> track via
  `CapacityAccumulator`
  ([billiard/mod.rs](</workspaces/msc-math/library/src/algorithms/billiard/mod.rs:1>)).
- The main difference is not result shape but enumeration policy:
  billiard exploits Lagrangian-product structure and the known bounce bound, so
  it only enumerates block-structured `sigma` with bounded length rather than
  the general HK2017 search space.
- Design implication: billiard should be refactored as part of the same
  result-layering family, not left on a separate bespoke result story. The
  shared surface should diverge mainly in search policy: billiard asserts
  Lagrangian-product structure and uses the proved bound on `|sigma|`, while
  HK2017 uses the general search.

## Verification Targets For Implementation Sessions

- `cargo test -p symplectic --release --lib`
- targeted experiment build(s) that migrate off copied instrumentation
- a regression/validation check that `gap = 0.0` returns all minimum-action
  certified orbits on known tied/symmetric examples
- derivative/subdifferential tests or experiment smoke checks on migrated
  consumers
- rerun the affected verification surface after each refactor packet rather than
  only unit-testing the touched helper:
  - library tests
  - migrated experiment smoke/build checks
  - any all-minimum-orbit validation pass that the refactor was meant to enable

## Non-goals

- Do not eagerly attach `GeometricOrbit` data to every returned orbit.
- Do not commit to a heavyweight abstract "subdifferential object" unless the
  primitive gradient-list surface proves insufficient.
- Do not redesign the KKT solver semantics here.
- Do not solve the generalized exact-number-domain problem in this refactor.
  Record the pressure so naming and layering stay compatible with future
  generalization, but keep the current implementation target grounded in the
  repo's existing exact rational machinery.
- Do not broaden this note into a generic data-flow or dataset-policy document.

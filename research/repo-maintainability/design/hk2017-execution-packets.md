<!--
Purpose: execution-kickoff note for the shared capacity/orbit result refactor.
Context: this lives on the integration worktree branch so later subagent
sessions can branch from the same parent state and report back into one packet
queue. It is intentionally incremental: only the first packets are defined.
-->

# Capacity/Orbit Execution Packets

## Status

- Parent branch: `capacity-result-api-exec`
- Parent worktree: `/workspaces/msc-math/.codex/worktrees/capacity-result-api-exec`
- Source design note:
  `research/repo-maintainability/design/hk2017-result-api-plan.md`
- Planning rule: do not freeze the whole DAG up front. Add packets as earlier
  packets land and expose the next dependency surface.
- Current progress:
  - Packet 1 scaffold landed: shared `algorithms::orbit_search` module plus
    public re-exports from `algorithms/mod.rs` and `lib.rs`.
  - Verification so far: `cargo build -p symplectic --release` passes on this
    branch after the scaffold landed.
  - Packet 2 slice 1 landed:
    - seam report at
      `research/repo-maintainability/design/packet-2-search-frontend-seam-report.md`
    - shared `solve_orbit_sigma(...)` primitive for the saddle-point path
    - current HK2017 and billiard solver bridges now route through that
      primitive
  - Packet 2 slice 2 landed:
    - shared `collect_legacy_capacity(...)` seam for the current
      solve/classify/track/finalize loop
    - HK2017 and billiard now share that collector while still owning their
      own sigma generation and winner-side metadata
  - Packet 2 slice 3 landed:
    - exact-fallback helpers for single-orbit upgrade/drop
    - first internal guarantee-mode resolution helpers for `BoundSafe`,
      `MinimaSafe`, and `AllSafe`
  - Packet 2 slice 4 landed:
    - shared `OrbitSearchResult`-returning collector entrypoints for
      `hk2017`, `hk2017_unpruned`, and `billiard`
    - shared internal `collect_orbits(...)` collector/finalization seam above
      frontend-specific `sigma` generation
    - explicit `OrbitSearchError::UnsupportedBackend` and
      `BilliardOrbitSearchError`
  - Packet 3 slice 1 landed:
    - `library/src/derivatives.rs` now defines `OrbitGradientA`,
      `ClarkeSubdiffA`, and `DerivativeError`
    - added derivative helpers on both clean seams:
      - `(polytope, sigma, KktResult) -> OrbitGradientA`
      - `(polytope, OrbitKktData) -> Result<OrbitGradientA, DerivativeError>`
    - added primitive Clarke directional-derivative helpers
    - first consumer migrations landed in `exp-combinatorial-cells` and
      `exp-hko-local-maximum` (`hko-second-order`)
  - Packet 3 slice 2 landed:
    - widened the same `KktResult`-level derivative-helper migration across the
      remaining buildable ascent binaries in `exp-sys-landscape` and
      `exp-hko-local-maximum` (`hko-cut-and-ascent`)
  - Packet 3 slice 3 landed:
    - migrated the first truly Clarke/subdifferential-heavy consumer,
      `dev_numerics_subdifferential`, onto the library helper surface
    - that binary now uses `capacity_derivatives_a_from_kkt_result(...)`,
      `directional_derivative_a(...)`, and
      `clarke_directional_derivative_a(...)` instead of local glue for the
      all-orbit directional-derivative path
  - Verification after Packet 2 slice 1:
    - `cargo build -p symplectic --release`
    - `cargo test -p symplectic --release --lib`
  - Verification after Packet 2 slice 3:
    - `cargo test -p symplectic --release algorithms::orbit_search::tests -- --nocapture`
    - `cargo test -p symplectic --release --lib`
  - Verification after Packet 2 slice 4:
    - `cargo build -p symplectic --release`
    - `cargo test -p symplectic --release minimum_orbits -- --nocapture`
    - `cargo test -p symplectic --release --lib`
  - Verification after Packet 3 slice 1:
    - `cargo test -p symplectic --release derivatives::tests -- --nocapture`
    - `cargo build -p exp-combinatorial-cells --release`
    - `cargo build -p exp-hko-local-maximum --release`
    - `cargo test -p symplectic --release --lib`
  - Verification after Packet 3 slice 2:
    - `cargo build -p exp-sys-landscape --release`
    - `cargo build -p exp-hko-local-maximum --release`
  - Verification after Packet 3 slice 3:
    - `cargo build -p dev-gradient --release --bin dev_numerics_subdifferential`

## Integration Model

- This worktree is the integration trunk for the refactor.
- Later subagent worktrees should branch from `capacity-result-api-exec`, not
  from root `main`.
- Each packet should end with:
  - code integrated back into this worktree
  - packet-local verification run here
  - tracker/doc update here if the landed code changes the approved shape

## First Packets

1. **Core types and naming scaffold**
   - Scope:
     - add the shared enums/types for guarantee mode, backend, admissibility,
       orbit payload, and search result
     - add/co-locate the shared error enums
     - no broad caller migration yet
   - Landed so far:
     - `OrbitAdmissibility`
     - `OrbitGuaranteeMode`
     - `OrbitSolveBackend`
     - `OrbitKktData`
     - `OrbitSearchResult`
     - `OrbitSearchError`
     - `GeometricOrbitError`
   - Why first:
     - every later packet depends on the names and field shapes existing in
       code
   - Verification:
     - `cargo build -p symplectic --release`
     - targeted library tests for touched modules if any compile-time glue needs
       updates
   - Stop condition:
     - if `mu`/`xi` optionality or public-module placement becomes unclear
  - Status:
    - complete for the current packet goal; later packets may still rename or
      extend these types if consumer migration exposes a missing field

2. **Shared search frontend surface**
   - Scope:
     - introduce shared collector entrypoints for `hk2017`,
       `hk2017_unpruned`, and `billiard`
     - wire guarantee/backend parameters through the search layer
     - keep old wrappers only as staging aids if needed
   - Planning artifact:
     - `research/repo-maintainability/design/packet-2-search-frontend-seam-report.md`
  - Landed so far:
     - shared seam identified: extract below frontend sigma generation and
       above frontend-local certified-winner metadata
     - `solve_orbit_sigma(...)` is the first shared primitive on that seam
     - `collect_legacy_capacity(...)` now owns the current shared
       solve/classify/track/finalize loop
     - existing HK2017/billiard frontends now depend on both shared seams
    - the shared module now contains the first internal exact-fallback /
      guarantee-mode machinery
    - the public `OrbitSearchResult` collectors now exist:
      - `hk2017_minimum_orbits(...)`
      - `hk2017_minimum_orbits_unpruned(...)`
      - `billiard_minimum_orbits(...)`
   - Known blocker discovered in this packet:
     - `OrbitSolveBackend::Projected` is still unsupported at the shared
       payload boundary because `library/src/kkt/projection_solver.rs` does not
       yet expose `q_error_bound`
   - Why second:
     - this establishes the real architectural seam before consumer migration
   - Verification:
     - `cargo build -p symplectic --release`
     - `cargo test -p symplectic --release --lib`
  - Stop condition:
    - if backend plumbing forces a solver-semantics change instead of an API
      refactor
  - Status:
    - complete for the saddle-point-backed shared collector goal
    - projected backend support remains a later packet because the projection
      solver does not yet expose the payload/error-bound contract this packet
      needs

3. **Derivative/subdifferential library helpers**
   - Scope:
     - add `OrbitGradientA`
     - add orbit-level derivative helper(s)
     - add primitive Clarke-subdifferential helpers on orbit lists
  - Why third:
    - this gives immediate leverage for the duplicated experiment logic
  - Verification:
    - derivative-related library tests
    - build at least one derivative-heavy experiment package
  - Stop condition:
    - if helper shape depends on unresolved consumer ergonomics not covered by
      the design note
  - Status:
    - in progress
    - first slices landed: helper aliases/errors plus migrated
      `KktResult`-level consumers across the main buildable ascent packages
      and the first truly subdifferential-heavy binary

4. **First consumer migrations**
   - Scope:
     - migrate one geometry/orbit consumer
     - migrate one derivative/subdifferential consumer
     - prefer consumers with clear existing verification surfaces
   - Why fourth:
     - proves the API is actually usable before broader migration
   - Verification:
     - targeted experiment build(s)
     - packet-specific smoke checks
   - Stop condition:
     - if migration reveals a missing core type/field rather than a local bug

## Immediate Next Action

- Decide whether the next highest-value seam is:
  - orbit-payload consumers via `capacity_derivatives_a_from_orbit(...)`, or
  - finishing the gradient/numerics package migration so sibling binaries use
    the same helper surface consistently.
- Keep projected-backend support out of Packet 3 unless the derivative packet
  directly needs it; it is a separate solver-contract follow-up.
- Branch subagent worktrees from this branch only if Packet 3 splits into
  disjoint write scopes.

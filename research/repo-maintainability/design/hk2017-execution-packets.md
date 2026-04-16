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
  - Packet 2 slice 5 landed:
    - the root scalar API is now an explicit family:
      - `ehz_capacity` = auto wrapper
      - `ehz_capacity_pruned`
      - `ehz_capacity_unpruned`
      - `ehz_capacity_billiard`
    - the auto wrapper dispatches to billiard on inputs that pass the
      Lagrangian-product structure test and otherwise falls back to pruned
      HK2017
    - policy: ordinary consumers use the auto wrapper; explicit HK2017 on
      Lagrangian products is now mainly for verification/debugging rather than
      normal consumption
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
  - Packet 3 slice 4 landed:
    - finished the sibling `dev-gradient` package migration so
      `dev_numerics` and `dev_numerics_edge_cases` now use the same
      derivative/directional-derivative helper seam
  - Packet 3 slice 5 landed:
    - migrated `experiments/hko-local-maximum/gradient-analysis/main.rs`
      off its local `ValidOrbit` payload and onto `OrbitKktData`
    - that experiment still owns its stricter `beta > EPS_BETA_POSITIVE`
      “valid orbit” threshold and local instrumented search loop, but it no
      longer owns a second orbit/KKT data shape or a local derivative adapter
  - Packet 3 slice 6 landed:
    - migrated `experiments/hko-local-maximum/second-order/main.rs`
      off its local `ValidOrbit` payload and onto `OrbitKktData`
    - that binary now uses `capacity_derivatives_a_from_orbit(...)` directly,
      so it no longer re-solves KKT just to recover `mu` for each stored orbit
  - Packet 3 slice 7 landed:
    - migrated `experiments/combinatorial-cells/omega-hypothesis/main.rs`
      off the raw `capacity_derivatives_a(...)` call and onto the
      `capacity_derivatives_a_from_kkt_result(...)` seam
    - this is intentionally a local-helper cleanup, not a collector migration:
      the remaining all-valid-orbit experiment binaries still need a separate
      decision about whether the library should expose their count/report
      semantics
  - Packet 3 slice 8 landed:
    - extracted the repeated all-valid-orbit HK2017 summary helper into
      `experiments/combinatorial-cells/src/lib.rs`
    - `cell-boundary-characterization`, `cell-widths`, `cell-convexity`, and
      `cell-multiple-crossings` now share that experiment-local helper instead
      of each owning the same `ehz_capacity_instrumented(...)` copy
  - Packet 3 slice 9 landed:
    - extracted the repeated stricter `beta > EPS_BETA_POSITIVE` orbit-search
      helper into `experiments/hko-local-maximum/src/lib.rs`
    - `hko-gradient-analysis` and `hko-second-order` now share that
      experiment-local collector instead of each owning nearly the same
      `OrbitKktData`-producing loop
  - Packet 3 slice 10 landed:
    - extracted the repeated strict-orbit enumeration and safe wrapper helpers
      into `experiments/numerics/gradient/src/lib.rs`
    - `dev_numerics`, `dev_numerics_edge_cases`, and
      `dev_numerics_subdifferential` now share `random_direction(...)`,
      `ehz_capacity_safe(...)`, and `solve_kkt_safe(...)`; the strict
      `enumerate_all_orbits(...)` helper is also shared where its semantics
      match, while the subdifferential binary keeps its inclusive/boundary
      enumeration logic local
  - Packet 3 slice 11 landed:
    - ordinary scalar/best-permutation experiment consumers now route through
      the root auto wrapper `symplectic::ehz_capacity(...)`
    - migrated surfaces:
      - `exp-sys-landscape`: `gradient-ascent-general`,
        `variable-f-ascent`, `random-sample`, `gradient-ascent-products`
      - `exp-hko-local-maximum`: `cut-and-ascent`, `gradient-analysis`,
        `second-order`, `perturbation-neighborhood`, `facet-splitting`
      - `exp-combinatorial-cells`: `boundary-characterization`,
        `multiple-crossings`
    - synced `ARCHITECTURE.md` and the maintainability discovery notes with the
      new root scalar API family:
      - `ehz_capacity` = auto
      - `ehz_capacity_pruned`
      - `ehz_capacity_unpruned`
      - `ehz_capacity_billiard`
    - product-reporting and verification surfaces that still need
      `billiard_capacity` or explicit HK2017 variants were left explicit on
      purpose because they consume native outputs such as `bounce_count` or
      compare algorithms directly
  - Packet 3 slice 12 landed:
    - documented the remaining explicit algorithm imports in verification and
      numerics surfaces so they no longer look like stale migrations
    - clarified intent in:
      - `experiments/numerics/unknown-predicates/main.rs`
      - `experiments/numerics/q-error/main.rs`
      - `experiments/verification/algorithm-comparison/benchmark/profile.rs`
      - `experiments/verification/correctness/main.rs`
    - those files stay explicit because they validate or profile specific
      algorithm paths rather than the root auto wrapper
  - Parallel follow-up now in flight on sub-worktrees branched from this
    integration trunk:
    - `capacity-orbit-recovery-refactor`:
      `experiments/verification/orbit-recovery/**`
    - `capacity-lagrangian-boundary-refactor`:
      `experiments/hko-local-maximum/lagrangian-boundary/**`
    - `capacity-product-reporting-refactor`:
      `experiments/sys-landscape/random-product-sample/**` and
      `experiments/sys-landscape/rotated-regular-products/**`
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
  - Verification after Packet 2 slice 5:
    - `cargo build -p symplectic --release`
    - `cargo test -p symplectic --release auto_dispatch_tests -- --nocapture`
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
  - Verification after Packet 3 slice 4:
    - `cargo build -p dev-gradient --release`
  - Verification after Packet 3 slice 5:
    - `cargo build -p exp-hko-local-maximum --release`
  - Verification after Packet 3 slice 6:
    - `cargo build -p exp-hko-local-maximum --release`
  - Verification after Packet 3 slice 7:
    - `cargo build -p exp-combinatorial-cells --release`
  - Verification after Packet 3 slice 8:
    - `cargo build -p exp-combinatorial-cells --release`
  - Verification after Packet 3 slice 9:
    - `cargo build -p exp-hko-local-maximum --release`
  - Verification after Packet 3 slice 10:
    - `cargo build -p dev-gradient --release`
  - Verification after Packet 3 slice 11:
    - `cargo build -p exp-sys-landscape --release`
    - `cargo build -p exp-hko-local-maximum --release`
    - `cargo build -p exp-combinatorial-cells --release`
    - `git diff --check`
  - Verification after Packet 3 slice 12:
    - `cargo build -p dev-numerical-analysis --release --bin num-q-error --bin num-unknown-predicates`
    - `cargo build -p dev-capacity-validation --release --bin axioms-correctness`
    - `cargo build -p dev-algorithm-comparison --release --bin cmp-benchmark-profile`
    - `git diff --check`

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
      and the full `dev-gradient` package
    - first `OrbitKktData` consumer migration landed in
      `experiments/hko-local-maximum/gradient-analysis/main.rs`
      while intentionally preserving that binary's local stricter
      admissibility threshold
    - a second `OrbitKktData` consumer migration landed in
      `experiments/hko-local-maximum/second-order/main.rs`, removing a
      per-orbit KKT re-solve that had existed only because the old local
      payload dropped multiplier data
    - one remaining raw derivative-glue site,
      `experiments/combinatorial-cells/omega-hypothesis/main.rs`, now uses the
      helper seam too, leaving the remaining Packet 3 work mostly on the
      collector/report side rather than the derivative-helper side
    - the repeated all-valid-orbit summary helper in the
      `exp-combinatorial-cells` package is now centralized in `src/lib.rs`,
      which reduces four copies of the same instrumentation without forcing the
      semantics into `library/`
    - the repeated stricter-orbit collector in `exp-hko-local-maximum` is now
      centralized in `src/lib.rs`, which removes another duplicated
      experiment-local search loop while preserving the package's stricter
      validity semantics
    - the `dev-gradient` package now centralizes its strict-orbit enumeration
      and safe-wrapper helpers in `src/lib.rs`, leaving the remaining package
      differences concentrated in the subdifferential-only inclusive/boundary
      logic rather than in copied boilerplate

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
  - collector/report migration for binaries whose semantics already match
    `hk2017_minimum_orbits(...)`, or
  - intentionally keeping experiment-local all-valid-orbit loops where the
    binary still reports counts/distributions outside the near-minimum window.
- Current caution from the latest migration sweep:
  `hko-gradient-analysis` and `hko-second-order` are not yet clean
  `hk2017_minimum_orbits(...)` migrations because they still report local
  all-valid-orbit metrics, not only near-minimum orbit sets.
- Keep projected-backend support out of Packet 3 unless the derivative packet
  directly needs it; it is a separate solver-contract follow-up.
- Branch subagent worktrees from this branch only if Packet 3 splits into
  disjoint write scopes.

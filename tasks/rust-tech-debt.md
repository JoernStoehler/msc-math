<!--
Purpose: Rust-facing tech-debt cleanup roadmap for final thesis closeout.
Context: this bundle routes cleanup work that affects agent velocity,
experiment safety, validation trust, or durable crate maintainability.
-->

# Rust Tech Debt Roadmap

## Status

- State: active.
- Last updated: 2026-05-11.
- Source surfaces: `crates/`, `experiments/`, `crates/MAP.md`,
  `experiments/MAP.md`, `research/verification.md`, relevant topic bundles,
  and tracked reports under `tasks/references/`.
- Refresh when: a Rust cleanup packet changes API support levels, experiment
  command safety, generated-output ownership, validation commands, or the
  interpretation of a retained thesis claim.

## Steering Cache

- [agent synthesis 2026-05-04] The strongest current pattern is unclear
  operating contracts, not one chosen architecture. The repeated agent-cost
  questions are: which command is safe, which output is canonical, which API is
  supported, which duplicate owns current data, which blocked code can be
  ignored, and which validation command protects a refactor.
  Source: summarized from a scratch exploration report and spot-checked against
  `crates/MAP.md`, `experiments/MAP.md`, `research/verification.md`, and
  `tasks/*.md`.
  Why it matters: cleanup should proceed by independent packets unless a packet
  proves that an architecture decision is now worth Jörn's time.
- [accepted 2026-05-04] Consult Jörn for high-risk architecture/API/data-shape
  decisions. Do not consult him for low-risk, easily reversible mechanical
  cleanup where more evidence is unlikely to change the choice.
  Source: Jörn chat instruction.
  Why it matters: keeps scarce decision time for choices that are expensive to
  unwind.
- [accepted 2026-05-09] Branch `delete-algebraic-crate` is the exact arithmetic
  replacement branch, not literal directory deletion. The merged reference
  record is `tasks/references/exact-arithmetic-replacement-2026-05-10.md`.
  Source: Jörn chat clarification and branch DoD session on 2026-05-09.
  Why it matters: future agents must judge the branch by thesis usefulness,
  consumer adoption, generic/domain separation, and bounded verification cost,
  not by branch name, crate-local tests, or old API continuity.

## Work Map

| item | state | value class | owner/gate | next action | source |
| --- | --- | --- | --- | --- | --- |
| Broad Rust lint gate | `[done]` | map input | agents | Keep `cargo clippy --workspace --all-targets -- -D warnings` green; use it as a cheap first-pass regression gate for future cleanup packets. | 2026-05-11 Euclidean-polytopes merge checks |
| Tracing/instrumentation surface | `[future]` | future/follow-up | agents, Jörn only for architecture if the first concrete setup is invasive | Future setup should prefer `tracing` spans/events in production code plus profile-binary subscribers over returning instrumentation through function signatures. Start with coarse release-build spans for random fixture generation, exact geometry/polar vertices, transition construction, sigma enumeration, KKT solving, and aggregation; add counters only where branch counts answer a concrete profiling question. | 2026-05-11 profiling discussion, `crates/symplectic/DEVELOPMENT.md`, `crates/symplectic/src/bin/profile_pruned_hk2017.rs` |
| Safe experiment command contracts | `[active]` | mainline thesis | agents, Jörn only for retained-output policy | Continue with finer per-binary smoke repairs after package-level contracts for HKO, verification/algorithm-comparison, combinatorial-cells, numerics, and sys-landscape; verification/numerics help exits are being normalized to status 0. | `experiments/MAP.md`, `tasks/reproducibility.md` |
| Verification trust chain | `[active]` | mainline thesis | retained claims | `experiments/verification/README.md` now records the top-level Rust command contract. Decide later which full verification commands are required before broad Rust cleanup; keep path/row diagnostics in verification plumbing. | `research/verification.md`, `experiments/verification/README.md` |
| `symplectic` API support levels | `[map-input]` | contingent during writing | Jörn for public API/architecture choices | Audit only the paths needed by retained thesis experiments before hiding, promoting, or redesigning public modules. | `crates/MAP.md`, `crates/symplectic/src/lib.rs` |
| Euclidean polytope crate migration | `[done; follow-ups routed]` | mainline thesis | agents, Jörn for API close calls | The architecture branch merged into `main` at `b90e92b2`. Use the dedicated roadmap for follow-ups, especially polar vertex enumeration performance. | `tasks/euclidean-polytopes.md`, `crates/euclidean-polytopes/README.md`, `crates/euclidean-polytopes/DEVELOPMENT.md` |
| Capacity result semantics | `[active]` | mainline thesis | retained claims, Jörn for thesis-facing contract | Root `ehz_capacity*` wrappers now use `OrbitGuaranteeMode::MinimaSafe`. `ehz_capacity_pruned_certified` adds an exact rational result path for capacity, minimizers, and an optional action-gap window while reusing f64 search intervals as the prefilter. Next thesis-facing decision: which callers need the ordinary `OrbitSearchResult` contract versus the certified rational contract. | `tasks/numerics.md`, `crates/symplectic/src/lib.rs`, `crates/symplectic/src/algorithms/orbit_search.rs`, `crates/symplectic/src/kkt/rational_solver.rs` |
| Unsupported projected backend | `[retired from current API]` | contingent during writing | Jörn if a projected route is reintroduced | The old backend strategy surface was deleted during flat orbit migration. Reintroduce a projected route only as a real flat solver function with current callers and Q-bound contracts. | `tasks/numerics.md`, `crates/symplectic/src/algorithms/orbit_search.rs` |
| Hidden hard failures in fallible APIs | `[active]` | map input | agents | Non-finite f64 dual vertices and invalid random-sampling parameters now fail before panic/nontermination boundaries through the flat vertex-enumeration/random APIs. Continue with minimal reproducers before changing capacity-wrapper error semantics. | `crates/symplectic/src/lib.rs`, `crates/symplectic/src/geom/vertex_enumeration/mod.rs`, `crates/symplectic/src/random.rs` |
| Runtime invariant checks | `[active]` | mainline thesis | agents | Add exact/runtime validation at trust-boundary handoffs when complexity and compute cost are small. Start with places that turn internal payloads into certified/public results, then broaden only when a concrete failure mode or thesis-facing claim needs it. | `crates/symplectic/src/algorithms/orbit_search.rs`, `crates/symplectic/src/kkt/rational_solver.rs` |
| Exact arithmetic replacement | `[done]` | mainline thesis if exact validation is cited or exact-code ambiguity blocks main-branch task sessions | agents, Jörn for math/proof acceptance | The former `delete-algebraic-crate` branch was merged into main. It migrated thesis-relevant Rust consumers off the old public API and keeps domain geometry/KKT/workflow code outside the generic crate. | `tasks/references/exact-arithmetic-replacement-2026-05-10.md`, `crates/algebraic-numbers/README.md`, `crates/algebraic-numbers/DEVELOPMENT.md`, `research/numerics.md`, `crates/MAP.md` |
| Incomplete lie-audit remediation | `[active]` | mainline thesis | agents, Jörn for mathematical/source-of-truth calls | Rework exact/certified/ground-truth validation paths with a code-first pass, then route every confirmed finding to a fix, caveat, cut, or Jörn decision before using any repo-promise verification gate. | `crates/symplectic/src/kkt/rational_solver.rs`, `tasks/verify-thesis-done.md` |
| Duplicate producer ownership | `[map-input]` | map input | agents, Jörn only for deleting provenance | Label current, historical, frozen-baseline, exploratory, or delete only after checking research/task truth for that package. | `experiments/sys-landscape/`, `experiments/numerics/`, `experiments/verification/algorithm-comparison/` |
| Solver and exact-arithmetic copies | `[active]` | map input | agents, Jörn only for deletion/provenance decisions | Route reusable generic exact arithmetic and exact linear algebra through `crates/algebraic-numbers/`. Keep KKT, symplectic geometry, capacity/orbit logic, and experiment workflows in their domain crates or experiments. Classify experiment-local and historical copies in their owning topic before deleting them. | `tasks/references/exact-arithmetic-replacement-2026-05-10.md`, `experiments/verification/algorithm-comparison/ablation/kkt.rs`, `experiments/crosspolytope/main/kkt.rs`, `experiments/numerics/error-bounds/saddle_point_solver.rs`, `experiments/numerics/src/algebraic/`, `crates/symplectic/src/kkt/rational_solver.rs`, `crates/symplectic/src/geom/vertex_enumeration/exact_linalg.rs` |
| Blocked/stale/provenance code that looks live | `[active]` | map input | agents | Fix sampled stale headers and add grep-able status markers where source truth is already clear; `gradient-ascent-dev` now has a local stub README. Avoid broad deletion without provenance review. | topic research notes |
| Local diagnostic text | `[active]` | map input | agents | Improve path/row/error context opportunistically while touching nearby experiment or verification code; `num-unknown-predicates` billiard failures name the failed row and `axioms-correctness` output failures name the path. | `experiments/numerics/unknown-predicates/main.rs`, `experiments/verification/correctness/main.rs` |
| Large mixed-purpose files | `[future]` | future/follow-up by default | architecture decision if reopened | Split only when a concrete retained task is blocked by the mixed purpose. | `crates/MAP.md`, `experiments/MAP.md` |

## Agent Cache

- [fresh 2026-05-10] Scratch reports are not durable source surfaces for this
  roadmap. If a past scratch report matters, use only the claim summarized in a
  tracked task/reference file and refresh it against live source before editing.
- [fresh 2026-05-04] `cargo clippy --workspace --all-targets -- -D warnings`
  passed on branch `rust-tech-debt-cleanup` after mechanical lint fixes in
  Rust tests, benches, and one numerics-gradient doc comment. Refresh by
  rerunning the command before stacking larger cleanup.
- [fresh 2026-05-04] `cargo clippy --workspace --all-targets -- -D warnings`
  passed again after the command-contract, provenance, and diagnostics packets
  through commit `281c38b1`.
- [fresh 2026-05-04] Verification target-pool shared-cache paths are optional
  candidate inputs: missing paths load as empty through `database::load`.
  Conflicts and parse errors still fail loudly. `target_pool.rs` and `io.rs`
  now include path/row context for the local failure points touched in this
  branch.
- [fresh 2026-05-04] `hko-facet-splitting` now has `--help` and `--smoke`.
  Full mode still writes `facet-splitting/hko-neighborhood-splitting.jsonl`;
  smoke mode writes the separate
  `facet-splitting/hko-neighborhood-splitting-smoke.jsonl`.
- [fresh 2026-05-04] `experiments/hko-local-maximum/README.md` now records the
  HKO Rust command contract in one place. It distinguishes smoke/default/full
  and canonical output modes for all eight HKO binaries.
- [fresh 2026-05-04] `hko-lagrangian-probe` now rejects unknown arguments and
  supports `--help`; its `--smoke` mode still writes
  `lagrangian-boundary/lagrangian-probe-smoke.jsonl`.
- [fresh 2026-05-04]
  `experiments/verification/algorithm-comparison/README.md` records command
  safety for `cmp-ablation`, `cmp-benchmark`, and `cmp-benchmark-profile`.
  `cmp-ablation --smoke` and `cmp-benchmark --smoke` write separate smoke
  JSONL files; full mode keeps the tracked evidence paths.
- [fresh 2026-05-04] `experiments/combinatorial-cells/README.md` records that
  the current binaries are full-output producers without smoke modes. Do not
  run them as quick command checks unless intentionally refreshing tracked
  artifacts.
- [fresh 2026-05-04] `experiments/numerics/README.md` records the numerics
  command contract. `num-unknown-predicates --smoke` now writes a separate
  smoke JSONL file; full mode keeps the tracked evidence path.
- [fresh 2026-05-04] `experiments/sys-landscape/README.md` records the root
  sys-landscape command contract, including which binaries default to temp
  smoke outputs, which require explicit tracked-output refreshes, and which
  remain full-output producers without smoke modes.
- [fresh 2026-05-04] `crates/algebraic-numbers/README.md` records the exact
  scalar public surface, `canonical_element` serialization contract,
  field-spec validation/invariant-panic boundary, and the remaining formal
  reference gaps.
- [fresh 2026-05-04] A previous scratch lie-audit was incomplete and weakly
  validated after it missed a high-severity
  `crates/symplectic/src/kkt/rational_solver.rs` contract mismatch. Refresh by
  doing a code-first audit of exact/certified/ground-truth paths, not by
  trusting old scratch report coverage.
- [updated 2026-05-11] `rationalize_f64_dual_vertices` rejects non-finite f64
  coordinates with `ConstructionError::F64Conversion` before calling
  `f64_to_rational`; regression coverage lives in
  `crates/symplectic/src/geom/vertex_enumeration/tests.rs` and
  `crates/symplectic/src/random.rs`.
- [fresh 2026-05-04, updated 2026-05-11] `sample_random_dual_vertices`
  validates `facet_count` and the height interval before constructing a
  `Uniform` distribution. `generate_random_dual_vertices` checks the same
  preconditions before entering its fill loop.
- [fresh 2026-05-04] `num-unknown-predicates` no longer reports bare
  `billiard error` panics for lagrangian-product rows; the panic names the
  pentagon or polygon-product row and includes the `BilliardError`.
- [fresh 2026-05-04] Attempting to switch public root
  `symplectic::ehz_capacity*` wrappers to `OrbitGuaranteeMode::MinimaSafe`
  exposed a false exact-fallback certificate. The square product triggered
  sigma `[0, 3, 4, 2, 6]` with action `1.904761904761905` and converted
  beta margin about `3.4e-17`. The cause was not an exact rational
  admissibility proof: `solve_kkt_exact` used a floating relative pivot
  threshold (`1e-12`) and treated tiny nonzero rational pivots as null-space
  directions. Strict exact-zero pivoting rejects the sigma. Regression coverage:
  `f64_square_product_bad_sigma_rejected_by_exact_rank` at the solver level,
  `minimasafe_does_not_accept_spurious_square_product_minimum` at the result
  aggregation level, and `minimasafe_accepts_exact_rational_scaled_cube` as an
  exact rational cube contrast. The old `OrbitSolveBackend` surface has since
  been deleted from the current flat API.
- [fresh 2026-05-04] Root `symplectic::ehz_capacity*` wrappers now use
  `OrbitGuaranteeMode::MinimaSafe` instead of f64-only aggregation. Exact
  fallback output is revalidated at the result boundary: beta length,
  `beta_i > 0`, `Q > 0`, exact normalization `sum beta_i = 1`, and exact
  closure `sum beta_i a_{sigma_i} = 0` must all hold before an orbit can become
  `AdmissibleExact`. `BilliardError` has an `OrbitSearch` variant so the
  billiard wrapper returns aggregation failures instead of panicking after
  successful Lagrangian-product classification.
- [fresh 2026-05-04] `ehz_capacity_pruned_certified` is the exact rational
  output path for callers that need certified capacity/minimizers instead of a
  scalar-style f64 result. It uses the existing f64 saddle-point HK2017 stream
  for search, exact-resolves the first admissible candidate, then exact-resolves
  every remaining candidate whose f64 action lower bound can still lie in the
  requested exact window. `CertifiedOrbitSetMode::MinimizersOnly` returns exact
  capacity plus all exact minimizers; `GapWindow` also returns exact orbits with
  action at most `capacity_exact + action_gap_exact`. Criterion smoke profile
  with `cargo bench -p symplectic --bench profiling capacity -- --warm-up-time
  0.5 --measurement-time 1.0 --sample-size 10`: ordinary pruned capacity
  measured about 30 us, 114 us, 438 us, 1.31 ms, 4.77 ms, 15.5 ms, 108 ms for
  F=5..11; certified minimizers measured about 10.0 ms, 11.9 ms, 15.5 ms, 12.7
  ms, 28.8 ms, 28.0 ms, 126 ms for F=5..11.
- [fresh 2026-05-04] Sampled duplicate KKT/projection solver surfaces already
  carry provenance labels: algorithm-comparison ablation keeps a historical KKT
  helper copy for A0..A3 comparability, crosspolytope keeps a historical
  normalized-normal solver for that search, and numerics error-bounds labels
  its saddle-point solver as dead reference code.
- [fresh 2026-05-04] `experiments/numerics/src/algebraic/` no longer points
  future migration at stale `library/` paths. Its headers now distinguish the
  experiment-scoped exact spike from `crates/algebraic-numbers` and
  `crates/symplectic/src/exact`.
- [fresh 2026-05-04] `experiments/sys-landscape/gradient-ascent-dev/README.md`
  records that `step-calibration` and `strategy-comparison` are current stubs,
  explains the root-package and local-incubator binary names, and says they are
  not evidence producers.
- [fresh 2026-05-04] `axioms-all-minimum --help` and
  `axioms-orbit-recovery --help` now exit with status 0. Unknown arguments
  still print usage and exit with status 2.
- [fresh 2026-05-04] `num-algebraic-exactness --help` now exits with status
  0. Unknown arguments still print usage and exit with status 2.
- [fresh 2026-05-04] `dev-gradient` binaries
  `dev_numerics`, `dev_numerics_edge_cases`, and
  `dev_numerics_subdifferential` now exit with status 0 for `--help`.
  Unknown arguments still print usage and exit with status 2.
- [fresh 2026-05-04] `experiments/verification/README.md` records the command
  contract for `axioms-correctness`, `axioms-all-minimum`, and
  `axioms-orbit-recovery`, including which modes refresh tracked JSONL evidence.
- [fresh 2026-05-04] `axioms-correctness --help` now exits before generating
  `correctness/correctness.jsonl`; unknown arguments fail before writes.
- [fresh 2026-05-04] `axioms-correctness` output creation, row serialization,
  newline writes, and flush failures now include the target
  `correctness/correctness.jsonl` path.
- [fresh 2026-05-04] `experiments/crosspolytope/main/main.rs` no longer says
  it fills a placeholder capacity. The current source truth is
  `research/crosspolytope.md`: capacity `4.0` is recorded, with explicit
  caveat that search is complete only through `m = 13`.

## Pruned / Stale

- None yet. Add entries here when a tempting cleanup route is rejected after
  source-grounded review, so future agents do not rediscover it.

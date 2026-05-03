<!--
Purpose: historical discovery, decision, and execution-planning note for the
repo maintainability / architecture program.
Context: created before a broad refactor. Verify all facts, decisions, and
packets against current maps, tasks, and code before reuse.
-->

# Repo Maintainability / Architecture Program

> Historical snapshot. Do not treat this note as current instruction or live
> architecture state; verify facts against current maps, tasks, and code before
> reuse.

## Status

- Phase: main execution packet landed on `capacity-result-api-exec`; this note
  now serves mainly as durable history/index for the discovery and design pass.
- Last updated: 2026-04-17.
- Planning rule: write execution packets incrementally from the approved design
  notes; do not treat the full DAG as fixed before early packets are tested in
  code.
- Historical tracker anchor: legacy `TASKS.md` entry "Repo maintainability /
  architecture program". Current routing lives in `ROADMAP.md` and the relevant
  `tasks/*.md` bundle.
- Deliverable of this phase: a reviewed program note that survives chat
  compactification and can seed later worktree sessions.

## Goal

- Enable future research by making `crates/symplectic/`, `experiments/`, datasets, and
  top-level docs clearer, more predictable, and easier for agents to extend.
- Reduce avoidable agent failure modes: guessing import paths, duplicating
  helpers, depending on accidental internals, and misreading data ownership.
- Convert the current scattered evidence into a refactor program with explicit
  dependencies, verification checks, and stop conditions.
- Use a facts-first pipeline: collect current-state facts in one source note,
  then derive architecture docs from that note in a separate pass.

## Non-goals

- Do not start new research branches here.
- Do not approve broad library promotion just because code is reused.
- Do not change `.jsonl` contents or regenerate thesis-facing data as part of
  the planning pass.
- Do not write `ARCHITECTURE.md` as if the API/data decisions were already
  settled.

## Seeded Repo Facts

These are observed facts as of 2026-04-16, not design decisions.

- Before this session, no top-level `ARCHITECTURE.md` existed.
  Evidence from the initial discovery pass: `rg --files -g 'ARCHITECTURE.md' -g 'README.md'`.
- Repo orientation in this snapshot was split across `AGENTS.md`, legacy
  `TASKS.md`,
  `crates/symplectic/src/lib.rs`, `crates/symplectic/src/database.rs`, and per-package
  `experiments/<topic>/src/lib.rs` headers.
- The simple public library surface in `crates/symplectic/src/lib.rs` re-exports the
  settled router family and shared result types: `ehz_capacity` (auto),
  `ehz_capacity_pruned`, `ehz_capacity_unpruned`,
  `ehz_capacity_billiard`, `OrbitSearchResult`, `OrbitSearchError`,
  `OrbitKktData`, `OrbitSolveBackend`, `volume`, `omega0`,
  `lagrangian_product`, polygon builders, known polytopes, and test utils.
- Experiments already depend on deep library paths beyond that simple surface.
  Evidence: `rg -n "use symplectic::" experiments`.
  Examples seen on 2026-04-16:
  - `symplectic::algorithms::hk2017::permutations`
  - `symplectic::algorithms::hk2017::orbit_recovery`
  - `symplectic::kkt::saddle_point_solver`
  - `symplectic::algorithms::facet_adjacency`
- Topic packages already have helper-crate entry points:
  - `experiments/combinatorial-cells/src/lib.rs`
  - `experiments/hko-local-maximum/src/lib.rs`
  - `experiments/numerics/gradient/src/lib.rs`
  - `experiments/sys-landscape/src/lib.rs`
  Some are still nearly empty while shared logic remains copied across binaries.
- Repeated local helper patterns still exist across experiments.
  Evidence: `rg -n "fn ehz_capacity_instrumented|fn enumerate_all_orbits|fn compute_step_bound_detailed|fn sys_derivatives_a" experiments`.
- The 170-row polytope cache is mirrored at least three times and is
  byte-identical on 2026-04-15.
  Evidence:
  `sha256sum experiments/sys-landscape/cache.jsonl experiments/combinatorial-cells/polytopes.jsonl experiments/verification/orbit-recovery/polytopes.jsonl`
- `experiments/sys-landscape/variable-f-ascent/cache.jsonl` is intentionally
  local search history rather than a shared canonical catalog.
  Evidence at the time: legacy `TASKS.md` data-flow packet. Refresh against
  current `tasks/*.md` bundles before reuse.
- The library docs audit already concluded that module headers + per-module
  formal files mostly cover library-internal architecture, but not repo-level
  navigation for agents.
  Evidence at the time: legacy `TASKS.md` item "Library architecture docs
  audit".

## Open Decisions

- Which library paths are the intended stable/simple surface for routine use?
- Which deep paths are expert-but-allowed dependencies during the thesis push?
- Which deep paths should be treated as accidental internals and not gain new
  callers?
- Which repeated helpers should move to `crates/symplectic/`, to
  `experiments/<topic>/src/lib.rs`, or stay per-binary?
- Which dataset is the canonical shared polytope catalog, and which paths are
  mirrors or transient outputs?
- What should `ARCHITECTURE.md` explain directly, and what should it link to
  instead of duplicating?
- What is the minimal safe migration path from the current repo to the desired
  state without destabilizing thesis-facing evidence?

## Phase Structure

1. Discovery packets: collect and record facts in a form later agents can trust.
2. Shared repo-state discussion: align Jörn and the top-level session on the
   current repo shape and which details matter versus which can be glossed over.
3. Architecture-scope discussion: align on what the target architecture must
   define, describe, or leave intentionally local.
4. Architecture decision surface: present Jörn with compact choices and
   concrete tradeoffs one conceptual unit at a time.
5. Execution DAG synthesis: split approved work into PR-sized, verifiable
   packets with worktree boundaries.
6. Implementation/verification sessions: execute the approved packets and
   verify each one locally.
7. Consolidation: update `ARCHITECTURE.md`, `ROADMAP.md`, relevant
   `tasks/*.md`, and future-work buckets after the implemented changes settle.

## Discussion Plan

Use one conceptual unit per chat message to avoid mixing design layers.

1. Current repo state:
   agree on the present architecture picture and the level of detail that
   matters for the refactor.
2. Architecture scope:
   agree on what the architecture needs to define or describe, and what should
   remain implementation detail.
3. Individual decision units:
   discuss one decision family at a time, compare approaches, and converge on a
   target state.
   Start with the library API/result layering around capacity, orbit recovery,
   derivatives, and Clarke-subdifferential support.
4. Execution planning:
   only after the target state is chosen, break the work into a DAG of
   Codex-managed sessions, subagents, worktrees, and verification gates.

## Current Drafting Step

- Current-state fact note now exists at
  `.codex/reference/repo-maintainability/design/repo-facts.md`.
- Current merged top-level doc pass now exists:
  - `ARCHITECTURE.md` for component/code architecture plus the current
    persisted-data architecture
- Review result: Jörn read the file and accepted the section shape as useful.
- Current follow-up unit: review and trim any remaining accidental complexity
  after the landed router/building-block refactor, then use this note as
  history rather than as a live design queue.
- Dedicated design note for that unit now exists at
  `.codex/reference/repo-maintainability/design/hk2017-result-api-plan.md`.
- Evidence source for later fill:
  - consolidated current-state fact note
    `.codex/reference/repo-maintainability/design/repo-facts.md`
  - D1 import surface inventory
  - D2 shared-helper inventory
  - D3 data-flow inventory
  - D4 docs/navigation inventory
  - D5 execution constraints inventory
- Delegation plan for the fill phase:
  - use bounded read-only subagents to gather section-specific evidence and
    citations after the skeleton is approved
  - keep integration and wording consistency in the top-level session

## Discovery Artifacts

- D0: `.codex/reference/repo-maintainability/design/repo-facts.md`
- D1: `.codex/reference/repo-maintainability/design/import-surface-inventory.md`
- D2: `.codex/reference/repo-maintainability/design/shared-helper-inventory.md`
- D3: `.codex/reference/repo-maintainability/design/data-flow-inventory.md`
- D4: `.codex/reference/repo-maintainability/design/docs-navigation-inventory.md`
- D5: `.codex/reference/repo-maintainability/design/execution-constraints-inventory.md`

## Discovery Packet Queue

These packets are discovery-first. They should produce notes or matrices, not
refactor patches.

### D1. Import-Surface Inventory

- Status: written.
- Note: `.codex/reference/repo-maintainability/design/import-surface-inventory.md`
- Objective: list the library paths used by experiments and classify them as
  `simple public`, `expert public`, `accidental internal`, or `unclear`.
- Scope: `crates/symplectic/src/lib.rs`, `crates/symplectic/src/**/mod.rs`, and all Rust imports
  under `experiments/`.
- Expected output: a matrix of import path -> current callers -> candidate tier
  -> why later agents would pick it.
- Verification: rerun `rg -n "use symplectic::" experiments`.
- Stop condition: if classification depends on choosing the future API boundary,
  record the ambiguity instead of deciding it.

### D2. Shared-Helper Inventory

- Status: written.
- Note: `.codex/reference/repo-maintainability/design/shared-helper-inventory.md`
- Objective: group repeated helper logic by the right home:
  `library`, `topic-local helper crate`, or `per-binary local`.
- Scope: repeated orbit enumeration wrappers, `sys` helpers, step-bound logic,
  solver instrumentation helpers, and similar duplicated code.
- Expected output: grouped list of helper families with suggested home and
  reason.
- Verification: rerun the repeated-helper `rg` scan and inspect the current
  `experiments/<topic>/src/lib.rs` files.
- Stop condition: if moving a helper would force a new mathematical commitment
  or public API promise, mark it as a Jörn decision point.

### D3. Data-Flow And Cache Inventory

- Status: written.
- Note: `.codex/reference/repo-maintainability/design/data-flow-inventory.md`
- Objective: record the canonical shared datasets, the mirrors, and the
  intentionally local/transient datasets.
- Scope: `crates/symplectic/src/database.rs`, mirrored polytope caches, topic-local
  search traces, and experiment outputs that are read by analyzers.
- Expected output: table with `dataset`, `producer`, `consumers`, `trusted
  fields`, `canonical or mirror`, `regenerate or preserve`, and `drift risk`.
- Verification: rerun the mirror `sha256sum` command and inspect consumers via
  `rg -n "cache\\.jsonl|polytopes\\.jsonl|orbit-recovery\\.jsonl"`.
- Stop condition: if a proposed cleanup would change committed data values,
  stop and keep the note descriptive only.

### D4. Docs And Navigation Inventory

- Status: written.
- Note: `.codex/reference/repo-maintainability/design/docs-navigation-inventory.md`
- Objective: identify which frequently needed questions are already answered by
  existing docs and which require a new repo-level guide.
- Scope: `AGENTS.md`, legacy `TASKS.md`, `crates/symplectic/src/lib.rs`, `crates/symplectic/src/database.rs`,
  `experiments/<topic>/src/lib.rs`, and relevant research notes.
- Expected output: question -> current answer source -> gap -> eventual home
  (`ARCHITECTURE.md`, file header, task packet, or no new doc needed).
- Verification: check each listed question against a concrete existing file.
- Stop condition: if a gap is really an unsettled architecture decision, record
  it under the decision surface instead of writing around it.

### D5. Execution Constraints Inventory

- Status: written.
- Note: `.codex/reference/repo-maintainability/design/execution-constraints-inventory.md`
- Objective: record the operational constraints that the later execution DAG
  must obey.
- Scope: worktree usage, LFS safety, allowed verification commands, and what
  kinds of packets are safe for worker/reviewer subagents.
- Expected output: one short section that later packet writers can reuse
  instead of re-deriving the constraints.
- Verification: confirm each constraint against `AGENTS.md`, relevant skills,
  or existing task packets.
- Stop condition: if a constraint depends on Jörn policy rather than repo
  evidence, leave it as a question for the decision surface.

## Decision Families To Prepare

After the shared repo-state and architecture-scope discussions, the later Jörn
decision surface should cover these questions explicitly:

1. Library boundary: what must be stable before thesis submission, and what may
   remain expert-only?
2. Helper-location policy: when does repeated experiment logic belong in
   `crates/symplectic/` versus `experiments/<topic>/src/lib.rs`?
3. Data policy: what is the canonical shared catalog and what counts as a
   mirror or transient analysis artifact?
4. Architecture-doc scope: what belongs in `AGENTS.md`,
   `ARCHITECTURE.md`, or local file headers?
5. Execution shape: which packets are safe for agent-only execution and which
   packets require Jörn review before implementation?

## Draft Execution Packet Families

These are placeholders only. Do not treat them as approved work packets yet.

- E1. Library API tiering note and import-path classification.
- E2. Rich HK2017 report/orbit API design and possible minimal implementation.
- E3. Topic-local helper extraction passes where shared experiment logic is
  clearly not library-ready.
- E4. Data-flow policy note plus any non-destructive mirror-refresh tooling.
- E5. Deep-import cleanup or documentation patches after the intended tiers are
  explicit.
- E6. Top-level architecture docs (`ARCHITECTURE.md`) plus targeted local doc
  fixes.
- E7. Residual future bucket for work that is maintainability-positive but not
  worth doing before thesis submission.

## Packet Template For Later Sessions

Use this shape when the execution DAG is ready to be written into work packets:

1. **Unit of work**
   - Worktree: short branch name.
   - Tracked at: `ROADMAP.md`, the relevant `tasks/*.md` entry, and this
     program note section when still useful.
   - Scope: exact files, directories, or question.
   - Context: why this packet exists and which discovery evidence triggered it.
   - Why shallow: what makes the packet agent-doable.
   - Decision points: choices reserved for Jörn or the top-level session.
   - Expected output: patch, note, report, or finding.
   - Verification: exact command, scan, build, or review check.
   - Stop condition: when to hand back instead of guessing.

## Progress Log

- 2026-04-16: created this durable planning note.
- 2026-04-16: seeded the note with the then-current legacy `TASKS.md` packets, library
  surface scan, deep-import scan, repeated-helper scan, and cache-mirror check.
- 2026-04-16: wrote D1-D5 as separate file-backed discovery artifacts, using
  subagents for D1-D4 and local integration for D5.
- 2026-04-16: integration note for D1: added missing deep-path families
  (`billiard::facet_classification`, `kkt::qp_assembly`,
  `geom::reeb_trajectory`, `geom::facet_volume`, `kkt::rational_solver`) after
  local verification.
- 2026-04-16: discussion order set with Jörn: first shared repo-state picture,
  then architecture scope, then one decision family per message, then the
  execution DAG.
- 2026-04-16: decided to prefer a single `ARCHITECTURE.md` over ADR-style
  notes for the high-level architecture surface; decision history stays in
  commit messages and local consequences stay in file/module comments.
- 2026-04-16: started `ARCHITECTURE.md` with skeleton + conventions only; stop
  for Jörn readability review before filling sections.
- 2026-04-16: next safe resume point is review of the `ARCHITECTURE.md`
  skeleton, then delegated evidence-gathering for section fill.

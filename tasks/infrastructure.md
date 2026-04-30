<!--
Purpose: infrastructure, harness, documentation, and future SWE polish roadmap.
Context: non-thesis-spine work should stay bounded during finish mode.
-->

# Infrastructure Roadmap

## Status

- State: future by default.
- Last updated: 2026-04-30.
- Source surfaces: `AGENTS.md`, `.agents/skills/`, `.codex/`,
  `crates/MAP.md`, `experiments/MAP.md`, `ROADMAP.md`, `tasks/README.md`.
- Refresh when: agent routing, task-bundle conventions, architecture map, or
  repo-maintenance policy changes.

## Steering Cache

- [accepted 2026-04-24] Broad architecture, API, and code-polish programs are
  future/follow-up unless they fix a false thesis claim, cited reproducibility,
  or a direct writing blocker.
  Source: finish-mode reset.
  Why it matters: prevents maintainability work from expanding thesis closeout.
- [accepted 2026-04-24] `ROADMAP.md` + `tasks/*.md` are the task-navigation
  layer; the legacy `TASKS.md` pointer was deleted after stale references were
  migrated.
  Source: Jorn task-system refactor request.
  Why it matters: future agents should not edit the old mega tracker.
- [accepted 2026-04-24] Task bundles distinguish Steering Cache, Work Map,
  Agent Cache, and Pruned/Stale entries.
  Source: Jorn.
  Why it matters: preserves scarce human steering while allowing aggressive
  pruning of stale agent shortcuts.
- [accepted 2026-04-25] Agent-facing PM should mix common practices rather than
  invent a custom task method: topic roadmap bundles, research interpretation
  notes, thesis story index, final verification gates, and toolbox skills.
  Source: Jorn/Codex design-space discussion.
  Why it matters: keeps agents close to familiar formats while preserving the
  research-specific epistemic layer.
- [accepted 2026-04-25] Skills should be combinable protocols, not pure session
  identities. The old `*-focus` names have no compatibility value.
  Source: Jorn.
  Why it matters: real sessions mix roadmap, research, verification, and editing
  work.
- [accepted 2026-04-25] Research interpretations and proof-route state live as
  first-class artifacts under `research/` or proof-bearing sources, not inside
  task rows. `tasks/*.md` records the work and links to the result.
  Source: Jorn.
  Why it matters: expensive reasoning becomes a reusable source artifact instead
  of stale PM prose.
- [accepted 2026-04-25] Steering Cache entries should express epistemic status
  in precise prose, not terse hard/soft/suggestion labels.
  Source: Jorn.
  Why it matters: expert suggestions, dominating constraints, reversible value
  judgments, and agent syntheses differ in ways a three-state label hides.
- [accepted 2026-04-25] Top-level map/cache files should be short entrypoints
  that point outward instead of reexplaining detailed source state.
  Source: Jorn.
  Why it matters: agents can combine familiar map, claim-register, assurance,
  and runbook practices when the interaction between files stays low.
- [accepted 2026-04-25] `RESULTS.md` was a migration cache. Thesis story
  interpretation now lives in `research/INDEX.md` and `research/*.md`; thesis
  obligations live in `tasks/*.md`.
  Source: Jorn.
  Why it matters: desired claim type/strength is visible through proof,
  interpretation, writeup, verification, and cut/weaken obligations instead of
  a separate label register.
- [accepted 2026-04-25] `research/README.md` was renamed to
  `research/INDEX.md` because it is a thesis story index and retained synthesis
  surface, not a generic directory README.
  Source: Jorn.
  Why it matters: the filename now matches the map's purpose without implying
  that the index is purely regenerable.
- [accepted 2026-04-25] `DATAFLOW.md` and `scripts/dataflow.sh` were deleted.
  Use targeted grep/local inspection for provenance unless repeated dataflow
  tracing proves that a new cache is worth designing from scratch.
  Source: Jorn.
  Why it matters: stale or overlarge generated cache files become decoys, and a
  future frequent workflow can rebuild a smaller cache around observed needs.
- [accepted 2026-04-25] `cached-map-maintenance` owns the generic workflow for
  creating, refreshing, pruning, splitting, deleting, and reviewing cached map
  files.
  Source: Jorn/Codex cached-map discussion.
  Why it matters: map files can share one familiar update loop while keeping
  map-type-specific authority and purpose clear.

## Work Map

| item | state | value class | owner/gate | next action | source |
| --- | --- | --- | --- | --- | --- |
| Task-system migration | `[done]` | map input | current session | Legacy TASKS/FINISH were removed; use ROADMAP and topic bundles. | `tasks/README.md`, `ROADMAP.md` |
| PM/research convention refactor | `[done]` | map input | current session | Added next-map layer, research index, task outcome conventions, and protocol-style skills. | `AGENTS.md`, `research/INDEX.md`, `.agents/skills/` |
| Results migration | `[done]` | map input | current session | `RESULTS.md` was deleted after distributing story interpretation and obligations to research/tasks/final verification surfaces. | `research/INDEX.md`, `tasks/*.md`, `tasks/verify-thesis-done.md` |
| Cache-surface audit | `[done]` | map input | current session | Deleted stale root/generated maps, split subtree maps, renamed the research story index, and added the cached-map maintenance workflow. | `crates/MAP.md`, `experiments/MAP.md`, `tasks/README.md`, `research/INDEX.md` |
| Agent-facing architecture/navigation | `[done]` | map input | current session | Split the old root architecture map into subtree `MAP.md` files for crates and experiments. | `crates/MAP.md`, `experiments/MAP.md` |
| Cached-map workflow skill | `[done]` | map input | current session | Added the protocol skill and used it for the first `crates/MAP.md` / `experiments/MAP.md` refresh. | `.agents/skills/cached-map-maintenance/`, `crates/MAP.md`, `experiments/MAP.md` |
| Harness engineering skill revisit | `[future]` | future/follow-up | Jorn + agent review | Around 2026-05-07, reread `.agents/skills/harness-engineering/SKILL.md` after live use and add only harness-engineering practices Jörn remembered or agents encountered. Preserve the skill's objective/success-measurement structure; do not add proxy checks without a remembered/observed failure or official guidance. | `.agents/skills/harness-engineering/SKILL.md` |
| Capacity/orbit API architecture | `[future]` | future/follow-up by default | retained claim impact | `Polytope4D` boundary reduction is partly plausible only at the KKT/QP assembly layer; do not promote broad APIs unless retained thesis/reproducibility needs it. | `crates/MAP.md`, `crates/symplectic/src/kkt/qp_assembly.rs`, legacy rows |
| Experiment-to-library audit | `[future]` | future/follow-up by default | retained claim impact | Classify repeated helpers only when it unblocks validation, writeup, or agent navigation. | legacy library rows |
| Codex migration/orchestration tests | `[future]` | future/follow-up | Jorn/tooling | Keep separate from thesis closeout unless current agents are blocked. | `.codex/`, legacy rows |
| Rust convention label/proof wording | `[future]` | future/follow-up by default | harness discussion | Revisit only if invented labels or proof-in-doc-comment drift blocks current agents. The current mismatch packet reports that `rust-conventions` can be read as both "definitions, lemmas, and proofs live as doc comments" and "do not duplicate proofs inline". | `thesis/migration-findings.md`, `.agents/skills/rust-conventions/SKILL.md` |
| SWE polish | `[future]` | future/follow-up | post-thesis | Defer broad code cleanup until thesis is no longer at risk. | legacy polish rows |

## Agent Cache

- [fresh 2026-04-25] `crates/MAP.md` and `experiments/MAP.md` replace the old
  root `ARCHITECTURE.md`. They are subtree navigation caches, not task queues
  or public API promises.
  Refresh by: reading the headers and checking the relevant code/package
  surfaces.
- [fresh 2026-04-29] `Polytope4D` is still justified for geometry, volume,
  skeleton, root capacity routers, billiard classification, and dataset row
  construction because those paths use vertices, incidence, adjacency, or
  construction invariants together.  The low-risk boundary reduction is limited
  to `crates/symplectic/src/kkt/qp_assembly.rs`: add a helper that builds QP
  matrices from `&[Vector4<f64>]` plus `perm`, then keep existing
  `Polytope4D` wrappers.  Moving HK2017 enumeration off `Polytope4D` needs a
  concrete caller and likely Jörn/math judgment about the input contract.
- [fresh 2026-04-24] Harness edits should keep `AGENTS.md` short and put
  detailed rules in skills.
  Refresh by: reading `.agents/skills/harness-engineering/SKILL.md`.

## Pruned / Stale

- [stale 2026-04-24] Historical infrastructure migrations are preserved in git
  history and should not be expanded into live task rows unless they block
  current closeout.

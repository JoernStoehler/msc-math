<!--
Purpose: infrastructure, harness, documentation, and future SWE polish roadmap.
Context: non-thesis-spine work should stay bounded during finish mode.
-->

# Infrastructure Roadmap

## Status

- State: future by default.
- Last updated: 2026-04-25.
- Source surfaces: `AGENTS.md`, `.agents/skills/`, `.codex/`,
  `ARCHITECTURE.md`, `ROADMAP.md`, `tasks/README.md`.
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
  interpretation now lives in `research/README.md` and `research/*.md`; thesis
  obligations live in `tasks/*.md`.
  Source: Jorn.
  Why it matters: desired claim type/strength is visible through proof,
  interpretation, writeup, verification, and cut/weaken obligations instead of
  a separate label register.
- [accepted 2026-04-25] `DATAFLOW.md` and `scripts/dataflow.sh` were deleted.
  Use targeted grep/local inspection for provenance unless repeated dataflow
  tracing proves that a new cache is worth designing from scratch.
  Source: Jorn.
  Why it matters: stale or overlarge generated cache files become decoys, and a
  future frequent workflow can rebuild a smaller cache around observed needs.

## Work Map

| item | state | value class | owner/gate | next action | source |
| --- | --- | --- | --- | --- | --- |
| Task-system migration | `[done]` | map input | current session | Legacy TASKS/FINISH were removed; use ROADMAP and topic bundles. | `tasks/README.md`, `ROADMAP.md` |
| PM/research convention refactor | `[active]` | map input | current session | Add next-map layer, research index, task outcome conventions, and protocol-style skills. | `AGENTS.md`, `research/README.md`, `.agents/skills/` |
| Results migration | `[done]` | map input | current session | `RESULTS.md` was deleted after distributing story interpretation and obligations to research/tasks/final verification surfaces. | `research/README.md`, `tasks/*.md`, `tasks/verify-thesis-done.md` |
| Cache-surface audit | `[active]` | map input | current session or next roadmap session | Finish deciding how `ARCHITECTURE.md` and other top-level cached maps should be maintained; `DATAFLOW.md` was deleted. | `ARCHITECTURE.md`, `tasks/README.md`, `research/README.md` |
| Agent-facing architecture/navigation | `[map-input]` | contingent during writing | archive clarity | Keep `ARCHITECTURE.md` current only where it saves agent friction for retained work. | `ARCHITECTURE.md` |
| Capacity/orbit API architecture | `[future]` | future/follow-up by default | retained claim impact | Do not promote broad APIs unless retained thesis/reproducibility needs it. | `ARCHITECTURE.md`, legacy rows |
| Experiment-to-library audit | `[future]` | future/follow-up by default | retained claim impact | Classify repeated helpers only when it unblocks validation, writeup, or agent navigation. | legacy library rows |
| Codex migration/orchestration tests | `[future]` | future/follow-up | Jorn/tooling | Keep separate from thesis closeout unless current agents are blocked. | `.codex/`, legacy rows |
| SWE polish | `[future]` | future/follow-up | post-thesis | Defer broad code cleanup until thesis is no longer at risk. | legacy polish rows |

## Agent Cache

- [fresh 2026-04-25] Use `.agents/skills/roadmap-maintenance` when editing
  `ROADMAP.md`, `tasks/*.md`, or migrating old tracker state.
  Refresh by: reading `tasks/README.md`.
- [fresh 2026-04-24] `ARCHITECTURE.md` is the model for cached map style:
  current-state description, update rules, and freshness rules, not a task
  queue.
  Refresh by: reading the header of `ARCHITECTURE.md`.
- [fresh 2026-04-24] Harness edits should keep `AGENTS.md` short and put
  detailed rules in skills.
  Refresh by: reading `.agents/skills/harness-engineering/SKILL.md`.

## Pruned / Stale

- [stale 2026-04-24] Historical infrastructure migrations are preserved in git
  history and should not be expanded into live task rows unless they block
  current closeout.

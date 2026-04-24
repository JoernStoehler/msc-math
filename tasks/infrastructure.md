<!--
Purpose: infrastructure, harness, documentation, and future SWE polish roadmap.
Context: non-thesis-spine work should stay bounded during finish mode.
-->

# Infrastructure Roadmap

## Status

- State: future by default.
- Last updated: 2026-04-24.
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
  layer; `TASKS.md` is legacy.
  Source: Jorn task-system refactor request.
  Why it matters: future agents should not edit the old mega tracker.
- [accepted 2026-04-24] Task bundles distinguish Steering Cache, Work Map,
  Agent Cache, and Pruned/Stale entries.
  Source: Jorn.
  Why it matters: preserves scarce human steering while allowing aggressive
  pruning of stale agent shortcuts.

## Work Map

| item | state | value class | owner/gate | next action | source |
| --- | --- | --- | --- | --- | --- |
| Task-system migration | `[active]` | map input | current session | Create ROADMAP/task bundles, shrink legacy TASKS/FINISH, update routing. | `tasks/README.md` |
| Agent-facing architecture/navigation | `[map-input]` | contingent during writing | archive clarity | Keep `ARCHITECTURE.md` current only where it saves agent friction for retained work. | `ARCHITECTURE.md` |
| Capacity/orbit API architecture | `[future]` | future/follow-up by default | retained claim impact | Do not promote broad APIs unless retained thesis/reproducibility needs it. | `ARCHITECTURE.md`, legacy rows |
| Experiment-to-library audit | `[future]` | future/follow-up by default | retained claim impact | Classify repeated helpers only when it unblocks validation, writeup, or agent navigation. | legacy library rows |
| Codex migration/orchestration tests | `[future]` | future/follow-up | Jorn/tooling | Keep separate from thesis closeout unless current agents are blocked. | `.codex/`, legacy rows |
| SWE polish | `[future]` | future/follow-up | post-thesis | Defer broad code cleanup until thesis is no longer at risk. | legacy polish rows |

## Agent Cache

- [fresh 2026-04-24] Use `.agents/skills/task-roadmap-conventions` when editing
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

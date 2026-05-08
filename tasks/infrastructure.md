<!--
Purpose: infrastructure, harness, documentation, and future SWE polish roadmap.
Context: non-thesis-spine work should stay bounded during finish mode.
-->

# Infrastructure Roadmap

## Status

- State: future by default.
- Last updated: 2026-05-03.
- Source surfaces: `AGENTS.md`, `.codex/config.toml`, `.agents/skills/`,
  `.codex/agents/`, `crates/MAP.md`, `experiments/MAP.md`, `tasks/MAP.md`,
  `tasks/README.md`.
- Refresh when: agent routing, task-bundle conventions, architecture map, or
  repo-maintenance policy changes.

## Steering Cache

- [accepted 2026-04-24] Broad architecture, API, and code-polish programs are
  future/follow-up unless they fix a false thesis claim, cited reproducibility,
  or a direct writing blocker.
  Source: finish-mode reset.
  Why it matters: prevents maintainability work from expanding thesis closeout.
- [accepted 2026-04-24] `tasks/MAP.md` + `tasks/*.md` are the task-navigation
  layer; the legacy `TASKS.md` pointer was deleted after stale references were
  migrated.
  Source: Jorn task-system refactor request.
  Why it matters: future agents should not edit the old mega tracker.
- [accepted 2026-04-24] Task bundles distinguish Steering Cache, Work Map,
  Agent Cache, and Pruned/Stale entries.
  Source: Jorn.
  Why it matters: preserves scarce human steering while allowing aggressive
  pruning of stale agent shortcuts.
- [accepted 2026-04-25; updated 2026-05-03] Agent-facing PM should mix common
  practices rather than invent a custom task method: root instructions, topic
  roadmap bundles, research interpretation notes, thesis story index, and final
  verification gates. Repo-local skills are optional helpers, not a required
  layer of the current surface.
  Source: Jorn/Codex design-space discussion.
  Why it matters: keeps agents close to familiar formats while preserving the
  research-specific epistemic layer.
- [accepted 2026-04-25; updated 2026-05-03] If repo-local skills are
  reintroduced, they should be combinable protocols, not pure session
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
- [accepted 2026-04-25; updated 2026-05-03] Cached-map maintenance now lives in
  file-local map headers and ordinary task routing, not in a live
  `cached-map-maintenance` skill. Recreate a skill only if repeated map-refresh
  work proves that the header workflow is too easy to miss.
  Source: Jorn/Codex cached-map discussion and GPT-5.5 surface shrink.
  Why it matters: map files can share one familiar update loop without keeping
  stale helper code or another source surface alive.
- [accepted 2026-05-03] The low-quality cached rerunnable verification helpers
  were deleted instead of preserved as live skills or task surfaces. Rebuild
  verification packets only when repeated checks prove valuable enough, and
  anchor them in current source truth rather than stale helper prose.
  Source: Jorn in chat after AGENTS.md and skill-surface review; this is not a
  quote from `tasks/*.md`.
  Why it matters: stale verification helpers are worse than no helper when they
  make agents trust outdated checklists or skip source-grounded review.
- [accepted 2026-05-03] The GPT-5.5 harness modernization intentionally shrank
  the live surface to root `AGENTS.md`, map/task headers, `.codex/config.toml`,
  and empty placeholder directories for future repo-local skills/agent
  overrides. Deleted draft skills and low-quality helper packets are historical
  evidence, not current instructions.
  Source: Jorn after AGENTS.md rewrite and skill-surface review.
  Why it matters: future agents should not search for or obey deleted helper
  protocols when the current repo surface is intentionally smaller.
- [accepted 2026-05-02] Several needed harness behavior changes are already
  salient enough to preserve before the full style cleanup: difficult
  interactive chat, error recovery after wrong task interpretation, explicit
  goal-clarification mode, flat data-science packet orchestration, and current
  GPT-5.5 prompt guidance.
  Source: Jorn.
  Why it matters: these are observed or strongly suspected failure modes, not
  speculative polish. A broad cleanup should not lose them while separating
  domain artifacts from procedure artifacts.
- [accepted 2026-05-02] Jörn has concrete ideas about chat communication
  failure categories and harness style, but those should be combined with
  external evidence before becoming durable repo style.
  Source: Jorn.
  Why it matters: the cleanup needs both local failure taxonomy and outside
  practice from OpenAI, Anthropic, Google, and related research-engineering
  sources, especially where default agent behavior differs from the desired
  repo behavior.
- [accepted 2026-05-02] Harness modernization must treat GPT-5.5 and current
  Claude Opus 4.6/4.7 behavior as materially different from earlier GPT and
  Claude model families.
  Source: Jorn.
  Why it matters: prompt-engineering habits that were useful for older models
  can become misleading defaults; keep model-current guidance and observed
  behavior separate from inherited prompt folklore.

## Work Map

| item | state | value class | owner/gate | next action | source |
| --- | --- | --- | --- | --- | --- |
| Task-system migration | `[done]` | map input | current session | Legacy TASKS/FINISH were removed; use `tasks/MAP.md` and topic bundles. | `tasks/README.md`, `tasks/MAP.md` |
| PM/research convention refactor | `[done]` | map input | current session | Added the map/task/research layer and task outcome conventions. Old protocol-skill drafts were deleted during the GPT-5.5 harness shrink; use the live root/map/task surfaces instead. | `AGENTS.md`, `research/INDEX.md`, `tasks/README.md` |
| Results migration | `[done]` | map input | current session | `RESULTS.md` was deleted after distributing story interpretation and obligations to research/tasks/final verification surfaces. | `research/INDEX.md`, `tasks/*.md`, `tasks/verify-thesis-done.md` |
| Cache-surface audit | `[done]` | map input | current session | Deleted stale root/generated maps, split subtree maps, renamed the research story index, and added the cached-map maintenance workflow. | `crates/MAP.md`, `experiments/MAP.md`, `tasks/README.md`, `research/INDEX.md` |
| Agent-facing architecture/navigation | `[done]` | map input | current session | Split the old root architecture map into subtree `MAP.md` files for crates and experiments. | `crates/MAP.md`, `experiments/MAP.md` |
| Cached-map workflow | `[done]` | map input | current session | The durable workflow lives in map headers plus task routing; no live skill owns cached-map maintenance. Recreate a helper only after repeated map-refresh failures. | `crates/MAP.md`, `experiments/MAP.md`, `tasks/README.md` |
| GPT-5.5 harness surface shrink | `[done]` | map input | current session | Current live harness surface is the compact root `AGENTS.md`, map/task headers, `.codex/config.toml`, and placeholder `.agents/skills/` / `.codex/agents/` directories. Deleted draft skills and helper packets are historical evidence only. | `AGENTS.md`, `.codex/config.toml`, `.agents/skills/.gitkeep`, `.codex/agents/.gitkeep` |
| Harness style baseline | `[done]` | map input | Jorn + harness discussion | Use the smaller GPT-5.5 surface as the current baseline. Future style changes should patch observed failures in that surface, not revive the old draft-skill split by default. | `AGENTS.md`, `tasks/README.md` |
| Harness style external survey | `[done]` | map input | agent research + local review | Created a concise source-backed survey; use it as evidence input for the harness style baseline, chat taxonomy, recovery, goal-clarification, packet orchestration, and current-model modernization work. | old harness extraction: external-survey candidate |
| Chat communication style taxonomy | `[future]` | future/follow-up | Jorn + external evidence | Reopen only if live chat failures recur under the smaller GPT-5.5 surface; then patch `AGENTS.md` directly or create a focused skill if repeated use justifies it. | `AGENTS.md` |
| Difficult-interaction recovery | `[future]` | future/follow-up | Jorn + harness discussion | Reopen only after an observed failure shows the compact root guidance is insufficient. Prefer a small root-instruction patch before adding a new helper surface. | `AGENTS.md` |
| Goal-clarification mode | `[future]` | future/follow-up | Jorn + harness discussion | Reopen only if agents still start work before the objective/artifact is settled. Keep any fix in the smallest live surface that changes behavior. | `AGENTS.md` |
| Data-science packet orchestration | `[map-input]` | map input | Jorn + agent review | Extend the existing data-science subexperiment workflow so a lead agent can prepare and manage many flat, similar, independent packets without losing packet boundaries, output contracts, review gates, or integration ownership. | old harness extraction: data-science and delegation candidates, `tasks/landscape.md` |
| Current-model harness prompt modernization | `[done]` | map input | current session | GPT-5.5 modernization now means a smaller live surface, not a broad prompt-library migration. Reopen only for current official-doc conflicts or observed model-specific failures. | `AGENTS.md`, `.codex/config.toml` |
| Harness engineering skill revisit | `[pruned]` | future/follow-up | Jorn + agent review | Do not perform a scheduled reread of deleted skill drafts. Recreate a repo-local skill only from a current repeated workflow or observed failure. | deleted harness migration drafts in git history |
| Capacity/orbit API architecture | `[future]` | future/follow-up by default | retained claim impact | `Polytope4D` boundary reduction is partly plausible only at the KKT/QP assembly layer; do not promote broad APIs unless retained thesis/reproducibility needs it. | `crates/MAP.md`, `crates/symplectic/src/kkt/qp_assembly.rs`, legacy rows |
| Experiment-to-library audit | `[future]` | future/follow-up by default | retained claim impact | Classify repeated helpers only when it unblocks validation, writeup, or agent navigation. | legacy library rows |
| Codex migration/orchestration tests | `[future]` | future/follow-up | Jorn/tooling | Keep separate from thesis closeout unless current agents are blocked. | `.codex/`, legacy rows |
| Rust convention label/proof wording | `[future]` | future/follow-up by default | harness discussion | Revisit only if invented labels or proof-in-doc-comment drift blocks current agents. The current mismatch packet reports that `rust-conventions` can be read as both "definitions, lemmas, and proofs live as doc comments" and "do not duplicate proofs inline". | `thesis/migration-findings.md`, old harness extraction: Rust convention candidate |
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
- [fresh 2026-05-03] Current harness surface is deliberately compact:
  `AGENTS.md` for durable root instructions, map/task headers for local
  refresh rules, `.codex/config.toml` for shared Codex config, and empty
  `.agents/skills/` / `.codex/agents/` placeholders for future justified
  helpers. Deleted draft skills and rerunnable verification helpers are
  historical evidence only.
  Refresh by: reading `AGENTS.md`, `.codex/config.toml`, and the relevant
  map/task header before adding a new harness surface.

## Pruned / Stale

- [stale 2026-04-24] Historical infrastructure migrations are preserved in git
  history and should not be expanded into live task rows unless they block
  current closeout.

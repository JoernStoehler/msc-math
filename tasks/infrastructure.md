<!--
Purpose: infrastructure, harness, documentation, and future SWE polish roadmap.
Context: non-thesis-spine work should stay bounded during finish mode.
-->

# Infrastructure Roadmap

## Status

- State: future by default.
- Last updated: 2026-05-02.
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
- [accepted 2026-05-02] The current harness has broad style problems and does
  not yet specify a canonical harness style.
  Source: Jorn.
  Why it matters: without an explicit style baseline, agents can only patch
  local failures and may keep mixing domain knowledge, procedure knowledge,
  temporary session state, and durable instructions in inconsistent ways.
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
| Task-system migration | `[done]` | map input | current session | Legacy TASKS/FINISH were removed; use ROADMAP and topic bundles. | `tasks/README.md`, `ROADMAP.md` |
| PM/research convention refactor | `[done]` | map input | current session | Added next-map layer, research index, task outcome conventions, and protocol-style skills. | `AGENTS.md`, `research/INDEX.md`, `.agents/skills/` |
| Results migration | `[done]` | map input | current session | `RESULTS.md` was deleted after distributing story interpretation and obligations to research/tasks/final verification surfaces. | `research/INDEX.md`, `tasks/*.md`, `tasks/verify-thesis-done.md` |
| Cache-surface audit | `[done]` | map input | current session | Deleted stale root/generated maps, split subtree maps, renamed the research story index, and added the cached-map maintenance workflow. | `crates/MAP.md`, `experiments/MAP.md`, `tasks/README.md`, `research/INDEX.md` |
| Agent-facing architecture/navigation | `[done]` | map input | current session | Split the old root architecture map into subtree `MAP.md` files for crates and experiments. | `crates/MAP.md`, `experiments/MAP.md` |
| Cached-map workflow skill | `[done]` | map input | current session | Added the protocol skill and used it for the first `crates/MAP.md` / `experiments/MAP.md` refresh. | `.agents/skills/cached-map-maintenance/`, `crates/MAP.md`, `experiments/MAP.md` |
| Harness style baseline | `[map-input]` | map input | Jorn + harness discussion | Define the harness style convention before broad cleanup: which artifacts hold domain knowledge, procedure knowledge, temporary session state, durable instructions, and TODO workflow placeholders; then use that convention to decide what to extract, delete, rewrite, or leave alone. | `AGENTS.md`, `.agents/skills/`, `.codex/reference/`, `tasks/README.md` |
| Harness style external survey | `[done]` | map input | agent research + local review | Created a concise source-backed survey; use it as evidence input for the harness style baseline, chat taxonomy, recovery, goal-clarification, packet orchestration, and current-model modernization work. | `.agents/skills/harness-engineering/references/harness-style-external-survey.md` |
| Chat communication style taxonomy | `[map-input]` | map input | Jorn + external evidence | Capture Jörn's concrete examples of chat-style failures and candidate repairs, then compare them against outside guidance before writing a durable chat-communication convention. | `AGENTS.md`, `.agents/skills/harness-engineering/`, `.codex/reference/` |
| Difficult-interaction recovery | `[map-input]` | map input | Jorn + harness discussion | Define how agents should detect that frequent back-and-forth, accumulated errors, or unclear goals have pushed the session outside ordinary non-interactive coding mode; include an abort/restart path that carries forward only useful state when repair is cheaper than continuing. Use the chat-style taxonomy as input. | `AGENTS.md`, `.agents/skills/harness-engineering/`, `.codex/reference/` |
| Goal-clarification mode | `[map-input]` | map input | Jorn + harness discussion | Define an explicit mode for complex goal clarification where agents do not start implementation, planning-by-inertia, or cleanup until the objective and intended artifact are settled. | `AGENTS.md`, `.agents/skills/harness-engineering/`, `.codex/reference/` |
| Data-science packet orchestration | `[map-input]` | map input | Jorn + agent review | Extend the existing data-science subexperiment workflow so a lead agent can prepare and manage many flat, similar, independent packets without losing packet boundaries, output contracts, review gates, or integration ownership. | `.agents/skills/data-science-subexperiment/`, `.agents/skills/subagent-delegation/`, `tasks/landscape.md` |
| Current-model harness prompt modernization | `[map-input]` | map input | Jorn + official-doc check | Audit prompt-engineering assumptions embedded in harness material; replace stale GPT-4 through GPT-5.3 and older-Claude-style advice only where it conflicts with current GPT-5.5, Claude Opus 4.6/4.7, or observed repo failures. | `.agents/skills/harness-engineering/`, `.agents/skills/*/SKILL.md`, `.codex/reference/` |
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

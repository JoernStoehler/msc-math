<!--
Purpose: agent-facing project roadmap for the master-thesis closeout.
Context: this is the navigation layer for current mini-roadmaps under tasks/.
It is not the literal done truth-spec, not the thesis story index, and not a
complete history of old task rows.

Writing/update rules:
- keep this file short enough to skim before starting a session
- point to stable task-bundle sections instead of duplicating task detail
- preserve Jorn/Kai/external decisions that affect future work
- move execution detail into tasks/*.md and proof/data truth into source files
- delete stale planning prose rather than carrying it here

Freshness rules:
- update this file when a task bundle changes status or priority
- update task bundles when source truth changes
- if ROADMAP.md and a task bundle disagree, trust the bundle and refresh this
  file
-->

# ROADMAP.md

## Status

- State: finish-mode roadmap scaffold.
- Last updated: 2026-04-29.
- Target: finish the scoped master-thesis project by 2026-05-14. Finishing by
  2026-05-07 is plausible but not assumed.
- Phase 1 done-state basis was accepted by Jorn on 2026-04-24, modulo inline
  external TODOs.
- Phase 2 is current-state and Jorn-knowledge migration before selecting more
  content or presentation labor.

## Source Surfaces

| Surface | Role |
| --- | --- |
| `tasks/verify-thesis-done.md` | once-run final thesis-done gate |
| `ROADMAP.md` | overview and routing surface for humans and agents |
| `tasks/*.md` | topic mini-roadmaps and cached decision context |
| `research/INDEX.md` and `research/*.md` | thesis story index, detailed interpretation, proof-route state, negative results, and research caches |
| `.agents/skills/verification/` | reusable quality-measurement packets and operational definitions |
| `thesis/submission/README.md` | submission/admin forms, source links, and external-clock TODOs |
| `crates/MAP.md` and `experiments/MAP.md` | subtree navigation caches for durable code and experiment packages |

The old `TASKS.md` mega-tracker was deleted after migration. Use this roadmap
and the topic bundles instead.

## Current Closeout Rule

Do not select content or presentation labor directly from old open tasks. First
fill the topic maps enough to distinguish:

- `mainline thesis`
- `contingent during writing`
- `external clock`
- `map input`
- `future/follow-up`
- `cut/weaken`

Cool-to-have work is future/follow-up unless it improves the thesis enough to
justify calendar delay and Jorn-time cost.

## Topic Bundles

| Bundle | Current role | Start here |
| --- | --- | --- |
| Verify thesis done | compact final gate run after topic, writing, reproducibility, and repeated verification passes stop finding blockers | `tasks/verify-thesis-done.md` |
| Submission and archive | external-clock actions, forms, Zenodo/arXiv/outreach, final archive | `tasks/submit-thesis.md` |
| Writing | thesis structure, writer-ready boundary, figures, final prose gates | `tasks/writing.md` |
| HKO | HKO2024 theorem/evidence/blocker split and exact-Clarke route | `tasks/hko.md` |
| Sys first-order | arbitrary-polytope first-order theorem/evaluator gap for `sys` | `tasks/sys-first-order.md` |
| Hostile landscape | negative sys-search story and data-science evidence | `tasks/landscape.md` |
| Numerics | numerical appendix, solver/projection/beta-LP state | `tasks/numerics.md` |
| Reproducibility | data freshness, artifact truth, repo promises | `tasks/reproducibility.md` |
| Infrastructure | agent/harness/repo-maintenance work and future SWE polish | `tasks/infrastructure.md` |

## Immediate Phase-2 Agenda

1. Fill repo-state rows inside the task bundles from source files and current
   evidence.
2. Ask Jorn focused questions only after agents have reduced repo evidence to
   concrete decision surfaces.
3. Update bundle `Work Map` rows to classify each live item by value class.
4. Backchain the final three-week execution plan from the filled bundles and
   `tasks/verify-thesis-done.md`.

## Final Done Surface

The thesis-done declaration is in `tasks/verify-thesis-done.md`. The
submission/archive follow-through is in `tasks/submit-thesis.md`; no direct
repo-related master-thesis work remains after the final GitHub archive/read-only
action.

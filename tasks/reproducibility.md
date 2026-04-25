<!--
Purpose: data freshness, artifact truth, and repo-promise roadmap.
Context: supports final verification gates for data, builds, and cited outputs.
-->

# Reproducibility Roadmap

## Status

- State: map-input.
- Last updated: 2026-04-25.
- Source surfaces: `experiments/**`, `research/INDEX.md`, `research/*.md`,
  `tasks/verify-thesis-done.md`,
  `.agents/skills/verification/references/data-and-figures-are-traceable.md`.
- Refresh when: retained thesis claims cite artifacts, experiment outputs move,
  or repo promises change.

## Steering Cache

- [accepted 2026-04-24] Verify only cited or promised artifacts; do not chase
  every historical orphaned dataset unless it affects a retained claim.
  Source: finish-mode reset.
  Why it matters: keeps data hygiene bounded to thesis truth.
- [accepted 2026-04-24] Experiment code must not make thesis correctness depend
  on runtime links into `experiments/`, `formal/`, or `crates/`; thesis owns its
  publication assets.
  Source: `AGENTS.md`.
  Why it matters: final verification checks thesis promises, not every repo
  artifact.
- [accepted 2026-04-24] LFS-tracked `.jsonl` artifacts in the final state can be
  intentional preserved artifacts, historical records, or future/follow-up
  material; they must not be silently used as thesis support.
  Source: `tasks/verify-thesis-done.md`.
  Why it matters: final archive can contain historical data if claims are clear.

## Work Map

| item | state | value class | owner/gate | next action | source |
| --- | --- | --- | --- | --- | --- |
| Data freshness matrix | `[map-input]` | map input | agents | Reclassify remaining rows by retained thesis claim impact. | legacy data-freshness rows |
| Thesis-facing artifact truth | `[blocked]` | mainline thesis | retained claims | Verify only artifacts cited or promised by thesis text. | `data-and-figures-are-traceable.md`, `repo-promises-are-truthful.md` |
| Dataflow/cache policy | `[done]` | map input | current session | Deleted the generated dataflow map and script; use targeted grep/local inspection unless repeated provenance work justifies a new cache design. | `data-and-figures-are-traceable.md`, git history |
| Repo promises | `[blocked]` | mainline thesis | final README/thesis wording | Check fresh-clone/build/repro promises only after wording stabilizes. | `repo-promises-are-truthful.md` |
| LICCA returned outputs | `[future]` | future/follow-up | external compute | Keep pending/future unless thesis cites returned data. | legacy LICCA rows |

## Agent Cache

- [fresh 2026-04-25] The generated `DATAFLOW.md` / `scripts/dataflow.sh`
  surface was deleted because its custom parser and global report were more
  complex than the current thesis-facing provenance workload needs.
  Refresh by: using targeted `rg` over `Input Artifacts:`, `Output Artifacts:`,
  filenames, thesis sources, and nearby research notes.
- [fresh 2026-04-24] Existing rerun matrix signals: perturbation neighborhood
  needs LICCA only if thesis keeps large-N HKO falsification; convexity,
  crosspolytope timing, numerics notes, and cut-and-ascent timing should not be
  rerun by default.
  Refresh by: checking retained story obligations in `tasks/*.md`, research
  notes, and thesis text.
- [fresh 2026-04-24] Smoke outputs should stay untracked/temp unless an
  experiment documents an analyzer path.
  Refresh by: checking `job-smoke.sh` scripts and `.jsonl` status.

## Pruned / Stale

- [stale 2026-04-24] Broad repo-wide data cleanup is future unless it protects
  retained claims, final archive clarity, or a promised command.

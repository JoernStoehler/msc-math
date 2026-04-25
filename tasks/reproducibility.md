<!--
Purpose: data freshness, artifact truth, and repo-promise roadmap.
Context: supports final verification gates for data, builds, and cited outputs.
-->

# Reproducibility Roadmap

## Status

- State: map-input.
- Last updated: 2026-04-25.
- Source surfaces: `DATAFLOW.md`, `experiments/**`, `RESULTS.md`,
  `FINAL-VERIFICATION.md:T4/T5`, `scripts/dataflow.sh`.
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
  Source: `FINAL-VERIFICATION.md:T9.5`.
  Why it matters: final archive can contain historical data if claims are clear.

## Work Map

| item | state | value class | owner/gate | next action | source |
| --- | --- | --- | --- | --- | --- |
| Data freshness matrix | `[map-input]` | map input | agents | Reclassify remaining rows by retained thesis claim impact. | legacy data-freshness rows |
| Thesis-facing artifact truth | `[blocked]` | mainline thesis | retained claims | Verify only artifacts cited or promised by thesis text. | `FINAL-VERIFICATION.md:T4/T5` |
| Dataflow/cache policy | `[map-input]` | contingent during writing | retained claims | Keep or extend generated dataflow only if cited artifact truth, archive clarity, or repeated audits need it; targeted grep/local inspection is fine otherwise. | `DATAFLOW.md`, `scripts/dataflow.sh` |
| Repo promises | `[blocked]` | mainline thesis | final README/thesis wording | Check fresh-clone/build/repro promises only after wording stabilizes. | `FINAL-VERIFICATION.md:T5` |
| LICCA returned outputs | `[future]` | future/follow-up | external compute | Keep pending/future unless thesis cites returned data. | legacy LICCA rows |

## Agent Cache

- [fresh 2026-04-25] `scripts/dataflow.sh` regenerates `DATAFLOW.md` for
  declared experiment artifact-flow audits, but it is not mandatory for small
  artifact questions.
  Refresh by: running the script only when changing declared artifact headers,
  dataflow documentation, or a repeated audit that benefits from the generated
  view.
- [fresh 2026-04-24] Existing rerun matrix signals: perturbation neighborhood
  needs LICCA only if thesis keeps large-N HKO falsification; convexity,
  crosspolytope timing, numerics notes, and cut-and-ascent timing should not be
  rerun by default.
  Refresh by: checking retained claim wording in `RESULTS.md` and thesis text.
- [fresh 2026-04-24] Smoke outputs should stay untracked/temp unless an
  experiment documents an analyzer path.
  Refresh by: checking `job-smoke.sh` scripts and `.jsonl` status.

## Pruned / Stale

- [stale 2026-04-24] Broad repo-wide data cleanup is future unless it protects
  retained claims, final archive clarity, or a promised command.

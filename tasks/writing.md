<!--
Purpose: thesis writing, structure, figures, and reader-facing closeout roadmap.
Context: current thesis sources are stale; value decisions should wait for the
phase-2 repo/Jorn state maps.
-->

# Writing Roadmap

## Status

- State: map-input.
- Last updated: 2026-04-24.
- Source surfaces: `thesis/`, `RESULTS.md`, `FINAL-VERIFICATION.md:T1/T3/T8`,
  `ROADMAP.md`.
- Refresh when: thesis structure, retained claim set, or advisor feedback
  changes.

## Steering Cache

- [accepted 2026-04-24] Content and presentation labor should not be selected
  until repo-state and Jorn-knowledge maps exist.
  Source: Jorn.
  Why it matters: prevents old open writing tasks from driving scope before
  value assessment.
- [accepted 2026-04-15] Kai accepted the two main `RESULTS.md` result blocks as
  sufficient for thesis completion.
  Source: Kai meeting state recorded in legacy tracker.
  Why it matters: standalone/polish results do not create default obligations.
- [accepted 2026-04-24] Jorn is the final clarity/usefulness judge if Kai or
  Elizabeth do not perform final clarity reads; blocker feedback from them still
  means not done unless resolved or explicitly accepted outside submitted scope.
  Source: `FINAL-VERIFICATION.md:T3`.
  Why it matters: keeps final readability gates actionable.

## Work Map

| item | state | value class | owner/gate | next action | source |
| --- | --- | --- | --- | --- | --- |
| Thesis structure | `[Jorn]` | map input | Jorn | Pick retained chapter structure after repo-state map gives concrete content options. | `thesis/`, `FINAL-VERIFICATION.md:T3.4` |
| Writer-ready boundary | `[Jorn]` | map input | Jorn + agent prep | Classify thesis-external packets as must-finish-before-writing, contingent during writing, or future. | legacy `TASKS.md` writer-ready rows |
| HKO writeup compression | `[blocked]` | mainline thesis | HKO map | Wait for `tasks/hko.md` theorem/evidence/blocker state. | `RESULTS.md`, `FINAL-VERIFICATION.md:T2.1` |
| Hostile landscape compression | `[blocked]` | mainline thesis | landscape map | Wait for `tasks/landscape.md` retained-claim state. | `RESULTS.md`, `FINAL-VERIFICATION.md:T2.2` |
| Numerical appendix route | `[blocked]` | contingent during writing | numerics map | Wait for `tasks/numerics.md` proof-vs-validation-vs-caveat state. | `FINAL-VERIFICATION.md:T2.4` |
| Figures | `[future]` | contingent during writing | thesis structure | Decide after chapter structure names what needs illustration. | `FINAL-VERIFICATION.md:T3.6/T4.4` |
| Final assembly | `[blocked]` | mainline thesis | thesis complete | Build PDF, bibliography/cross-reference/proofread checks, print/USB/forms/upload, final tag/archive. | `FINAL-VERIFICATION.md:T7` |

## Agent Cache

- [fresh 2026-04-24] Before asking Jorn for writing decisions, agents should
  summarize actual `thesis/` chapter/source state and obvious build blockers
  without rewriting prose.
  Refresh by: reading `thesis/` and running thesis build checks only if cheap
  and non-disruptive.
- [fresh 2026-04-24] Figure inventory should wait until thesis structure names
  concepts and chapters; before then it is likely churn.
  Refresh by: checking `thesis/**/*.tex` for `\includegraphics` and `TODO`.

## Pruned / Stale

- [stale 2026-04-24] Old April writing schedule and pre-writing packet queue
  are superseded by finish mode and should not drive task selection.

<!--
Purpose: thesis writing, structure, figures, and reader-facing closeout roadmap.
Context: current thesis sources are stale; value decisions should wait for the
phase-2 repo/Jorn state maps.
-->

# Writing Roadmap

## Status

- State: map-input.
- Last updated: 2026-04-25.
- Source surfaces: `thesis/`, `research/INDEX.md`, `research/*.md`,
  `tasks/*.md`, `tasks/verify-thesis-done.md`, `ROADMAP.md`.
- Refresh when: thesis structure, retained claim set, or advisor feedback
  changes.

## Steering Cache

- [accepted 2026-04-24] Content and presentation labor should not be selected
  until repo-state and Jorn-knowledge maps exist.
  Source: Jorn.
  Why it matters: prevents old open writing tasks from driving scope before
  value assessment.
- [accepted 2026-04-15] Kai accepted the two main thesis story blocks, HKO local
  maximality and hostile sys-search landscape, as sufficient for thesis
  completion.
  Source: Kai meeting state recorded in legacy tracker.
  Why it matters: standalone/polish results do not create default obligations.
- [accepted 2026-04-24] Jorn is the final clarity/usefulness judge if Kai or
  Elizabeth do not perform final clarity reads; blocker feedback from them still
  means not done unless resolved or explicitly accepted outside submitted scope.
  Source: `tasks/verify-thesis-done.md`.
  Why it matters: keeps final readability gates actionable.

## Work Map

| item | state | value class | owner/gate | next action | source |
| --- | --- | --- | --- | --- | --- |
| Thesis structure | `[Jorn]` | map input | Jorn | Pick retained chapter structure after repo-state map gives concrete content options. | `thesis/`, `tasks/verify-thesis-done.md` |
| Writer-ready boundary | `[Jorn]` | map input | Jorn + agent prep | Classify thesis-external packets as must-finish-before-writing, contingent during writing, or future. | migrated tracker history |
| HKO writeup compression | `[blocked]` | mainline thesis | HKO map | Wait for `tasks/hko.md` theorem/evidence/blocker state. | `research/hko-local-maximum.md`, `thesis-stories-are-supported.md` |
| Hostile landscape compression | `[blocked]` | mainline thesis | landscape map | Wait for `tasks/landscape.md` retained-story state. | `research/sys-landscape.md`, `thesis-stories-are-supported.md` |
| Numerical appendix route | `[blocked]` | contingent during writing | numerics map | Wait for `tasks/numerics.md` proof-vs-validation-vs-caveat state. | `thesis-stories-are-supported.md` |
| Figures | `[future]` | contingent during writing | thesis structure | Decide after chapter structure names what needs illustration. | `data-and-figures-are-traceable.md` |
| AI process reflection | `[future]` | contingent during writing | thesis structure | Include only if the final thesis structure has a reader-facing reason to discuss agent contribution, counterfactual impact, or failure modes. | `research/INDEX.md`, `tasks/infrastructure.md` |
| Final assembly | `[blocked]` | mainline thesis | thesis complete | Build PDF, bibliography/cross-reference/proofread checks, print/USB/forms/upload, final tag/archive. | `tasks/verify-thesis-done.md`, `tasks/submit-thesis.md` |

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

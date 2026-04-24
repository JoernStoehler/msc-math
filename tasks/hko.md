<!--
Purpose: HKO2024 local-maximality roadmap.
Context: main thesis result and potential publication-grade follow-up surface.
-->

# HKO Roadmap

## Status

- State: map-input.
- Last updated: 2026-04-24.
- Source surfaces: `RESULTS.md`, `research/hko-local-maximum.md`,
  `research/hko-local-maximum-status.md`,
  `research/hko-local-maximum-exact-clarke.md`,
  `experiments/hko-local-maximum/`, `FINAL-VERIFICATION.md:T2.1`.
- Refresh when: exact-Clarke route, HKO theorem wording, or LICCA evidence
  changes.

## Steering Cache

- [accepted 2026-04-15] HKO2024 local maximality is part of the thesis spine.
  Source: Kai/Jorn state in `RESULTS.md` and legacy tracker.
  Why it matters: HKO compression is mainline thesis work.
- [accepted 2026-04-24] LICCA large HKO runs are optional publication-grade
  polish, not required for thesis sufficiency unless results already exist with
  low integration cost or Jorn chooses the external action.
  Source: Jorn finish-mode reset.
  Why it matters: prevents compute work from delaying thesis writing by default.
- [accepted 2026-04-24] Exact first-order certificate is the preferred stronger
  route if it becomes trusted; otherwise wording can fall back to supported
  numerical/conditional evidence.
  Source: legacy exact-Clarke row and `FINAL-VERIFICATION.md:T2.1`.
  Why it matters: theorem strength depends on certification status.

## Work Map

| item | state | value class | owner/gate | next action | source |
| --- | --- | --- | --- | --- | --- |
| HKO theorem/evidence/blocker split | `[map-input]` | mainline thesis | agent prep then Jorn | Summarize current proof, empirical evidence, and blockers into thesis-safe wording options. | `research/hko-local-maximum*.md` |
| h-space proof check | `[Jorn]` | mainline thesis | Jorn | Verify Danskin/symmetry/Euler argument if retained. | `research/hko-local-maximum.md` |
| second-order proposition | `[Jorn]` | contingent during writing | Jorn | Decide whether second-order note is proof route, support, or future evidence after exact route state is known. | `formal/hko-local-maximum/second-order.tex` |
| exact Clarke checker | `[active]` | mainline thesis or contingent | dedicated sessions | Continue exactifying widened active-row representative surface and witness contract only if it remains the best route to retained claim strength. | `research/hko-local-maximum-exact-clarke.md` |
| higher-F perturbation | `[future]` | future/follow-up | Jorn/external compute | Leave F=12/F=13 validation as pending/future unless cheap results already exist. | `RESULTS.md` |
| LICCA F=10 neighborhood | `[future]` | future/follow-up | Jorn/external compute | Reopen only if Jorn chooses LICCA action or results already returned. | `experiments/hko-local-maximum/perturbation-neighborhood/` |

## Agent Cache

- [fresh 2026-04-24] Current local HKO evidence includes first-order
  positive-span signal, second-order negative curvature samples, facet-splitting
  checks, cut-and-ascent checks, and a perturbation-neighborhood artifact.
  Refresh by: reading `research/hko-local-maximum.md` and the linked experiment
  directories.
- [fresh 2026-04-24] Exact-Clarke route state is nuanced: current Sage
  representative-first route weakened the old SymPy cost objection, but active
  row multiplicity remains the obstruction.
  Refresh by: reading `research/hko-local-maximum-exact-clarke.md` and current
  exact-Clarke artifacts.
- [fresh 2026-04-24] Before LICCA submission, the remote repo layout must match
  the current `experiments/...` package layout, not old `crates/exp-*` paths.
  Refresh by: checking `tasks/submission.md` and current LICCA scripts.

## Pruned / Stale

- [stale 2026-04-24] Treat old "all HKO polish before April cutoff" scheduling
  as superseded. Retain only thesis-spine proof/evidence choices.

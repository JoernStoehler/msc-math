<!--
Purpose: thesis writing, structure, figures, and reader-facing closeout roadmap.
Context: current thesis sources are stale; value decisions should wait for the
phase-2 repo/Jorn state maps.
-->

# Writing Roadmap

## Status

- State: map-input.
- Last updated: 2026-05-01.
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
- [accepted 2026-04-25] The next thesis-structure artifact should be
  `thesis/planned-toc.md`, not a task-bundle file. It should be a working
  thesis table of contents with one-sentence leaf obligations, dependencies,
  and gaps; task bundles can be updated from it after the content shape
  stabilizes.
  Source: Jorn Phase-2 TOC guidance.
  Why it matters: thesis content knowledge belongs with thesis planning first,
  while `tasks/*.md` should keep routing and work obligations.
- [accepted 2026-05-01] For sys first-order material, write the generic case
  first. Introduce concrete finite open dense assumptions only when a lemma uses
  them, and put the non-generic/boundary cases in a later discussion chapter.
  Why it matters: this makes the thesis readable while still preserving the
  HKO2024 and gradient-ascent motivations for zero dwell times, active ties,
  rank loss, semidefinite Hessians, and branch changes.

## Work Map

| item | state | value class | owner/gate | next action | source |
| --- | --- | --- | --- | --- | --- |
| Planned thesis TOC | `[Jorn]` | map input | Jorn with agent state answers | Draft `thesis/planned-toc.md` interactively before selecting broad prose work; use it to expose unnatural divisions, missing dependencies, and sections whose content is not yet supported. | `thesis/`, `research/INDEX.md`, `tasks/*.md` |
| Thesis structure | `[Jorn]` | map input | Jorn | Pick retained chapter structure after repo-state map gives concrete content options. | `thesis/`, `tasks/verify-thesis-done.md` |
| Writer-ready boundary | `[Jorn]` | map input | Jorn + agent prep | Classify thesis-external packets as must-finish-before-writing, contingent during writing, or future. | migrated tracker history |
| Thesis/code mismatch packet routing | `[map-input]` | map input | agents | Before using existing algorithm/numerics/tube prose as TOC leaves, route the 15 rows in `thesis/migration-findings.md` to thesis-side fix, code/comment fix, future/cut, or Jörn decision. | `thesis/migration-findings.md`, `tasks/numerics.md`, `tasks/infrastructure.md` |
| Generic sys first-order section | `[active]` | mainline thesis | agents then Jorn math | Expand the committed unapproved draft into a readable generic-case section: list the concrete open dense row-chart assumptions, prove smooth local behavior on those chambers, and defer non-generic active-germ/cell-decomposition behavior to a later boundary chapter. | `thesis/sys-first-order-regular-case.tex`, `research/sys-first-order-local-behavior.md`, `tasks/sys-first-order.md` |
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
- [fresh 2026-04-25] Current thesis source is a stale skeleton relative to the
  closeout maps: `main.tex` has a deferred abstract and TODO introduction,
  `experiments.tex` is a placeholder, current inputs are algorithms,
  sys-first-order regular case, proofs, experiments, and numerical appendix.
  The old tube section is quarantined and not input while it is re-imported from
  Jörn's 2026-05-04 raw source note. A targeted scan found no current
  figure/table environments.
  Refresh by: `rg -n -e '^\\section' -e '^\\subsection' -e '^\\input' -e '^\\appendix' thesis -g '*.tex'`
  plus targeted reads of `thesis/main.tex` and `thesis/experiments.tex`.
- [fresh 2026-04-25] `thesis/migration-findings.md` is an unrouted decision
  packet for thesis/code mismatches. Rows 3-11 mostly affect algorithm/numerics
  exposition, rows 1 and 12-14 affect tube/code-vs-thesis alignment, row 2 is a
  label-cross-reference choice, row 15 is a label inventory, and the convention
  gap belongs under infrastructure/harness follow-up. Route these before thesis
  prose relies on the affected statements.
  Refresh by: reading `thesis/migration-findings.md`.
- [fresh 2026-05-01] `thesis/sys-first-order-regular-case.tex` is a committed
  unapproved draft for "Generic feasible HK gradients". It already separates
  HK-generic nonvanishing conditions from feasible generic support maxima. The
  next writer should preserve that split and add only the concrete genericity
  assumptions that are actually used by each lemma.
  Refresh by: `latexmk && ./check-build.sh` in `thesis/` and a reviewer pass
  against `research/sys-first-order-local-behavior.md`.

## Pruned / Stale

- [stale 2026-04-24] Old April writing schedule and pre-writing packet queue
  are superseded by finish mode and should not drive task selection.

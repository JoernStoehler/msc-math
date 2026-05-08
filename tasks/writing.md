<!--
Purpose: thesis writing, structure, figures, and reader-facing closeout roadmap.
Context: current thesis sources are stale; value decisions should wait for the
phase-2 repo/Jorn state maps.
-->

# Writing Roadmap

## Status

- State: map-input.
- Last updated: 2026-05-05.
- Source surfaces: `thesis/`, `research/INDEX.md`, `research/*.md`,
  `tasks/*.md`, `tasks/verify-thesis-done.md`, `tasks/MAP.md`.
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
- [accepted 2026-05-05] Kai and Jorn discussed the working thesis outline now
  recorded in `thesis/planned-toc.md`.
  Source: Jorn after Kai meeting.
  Why it matters: `thesis/planned-toc.md` is now the current thesis-structure
  surface. Earlier ToC drafts should not drive writing.
- [accepted 2026-05-05] Many remaining ToC questions are intentionally
  chapter-local writer-session questions, not global ToC blockers.
  Source: Jorn.
  Why it matters: writers should resolve inline questions in
  `thesis/planned-toc.md` when starting the relevant chapter instead of trying
  to settle every detail before prose begins.
- [accepted 2026-05-05] Numerics is interesting to Kai for about one high-level
  paragraph in the main text; detailed proofs and intermediate bounds belong in
  the appendix.
  Source: Jorn after Kai meeting.
  Why it matters: prevents the numerics appendix from expanding into a main
  chapter.

## Work Map

| item | state | value class | owner/gate | next action | source |
| --- | --- | --- | --- | --- | --- |
| Planned thesis TOC | `[done]` | map input | Jorn with Kai | Use `thesis/planned-toc.md` as the current structure. Resolve its inline `OUTLINE GAP`, `TOC DECISION`, and `WRITER-SESSION QUESTION` comments during the relevant chapter writer sessions. | `thesis/planned-toc.md` |
| Thesis structure | `[done]` | map input | Jorn/Kai | Keep the meeting-derived structure unless advisor feedback or chapter drafting exposes a real conflict. | `thesis/planned-toc.md`, `tasks/verify-thesis-done.md` |
| Writer-ready boundary | `[map-input]` | map input | Jorn + chapter writers | For each chapter, start from `thesis/planned-toc.md`; turn its inline questions into the chapter's local write plan before drafting prose. | `thesis/planned-toc.md`, migrated tracker history |
| Thesis/code mismatch packet routing | `[map-input]` | map input | agents | Before using existing algorithm/numerics/tube prose as TOC leaves, route the 15 rows in `thesis/migration-findings.md` to thesis-side fix, code/comment fix, future/cut, or Jörn decision. | `thesis/migration-findings.md`, `tasks/numerics.md`, `tasks/infrastructure.md` |
| Generic sys first-order section | `[active]` | mainline thesis | agents then Jorn math | Expand the committed unapproved draft into a readable generic-case section: list the concrete open dense row-chart assumptions, prove smooth local behavior on those chambers, and defer non-generic active-germ/cell-decomposition behavior to a later boundary chapter. | `thesis/sys-first-order-regular-case.tex`, `research/sys-first-order-local-behavior.md`, `tasks/sys-first-order.md` |
| HKO local maximum chapter | `[map-input]` | mainline thesis | chapter writer + Jorn math | Define the decision problem, explain the SageMath computation as chained checked subroutines, decide selected code/data excerpts for main text, and keep full code/data in the repo. | `thesis/planned-toc.md`, `research/hko-local-maximum.md`, `research/hko-local-maximum-exact-clarke.md` |
| Black-box data-science chapter | `[map-input]` | mainline thesis | landscape branch + chapter writer | When the data-science branch stabilizes, finalize table rows, feature columns, method families, and any positive-result escalation. | `thesis/planned-toc.md`, `tasks/landscape.md`, `research/sys-landscape-toolbox-audit.md` |
| Numerics main-text paragraph and appendix | `[map-input]` | contingent during writing | numerics map + chapter writer | Keep main text to a high-level paragraph; put exact algebraic fallback, empirical error measurements, proven error bounds, and intermediate constants in the appendix. | `thesis/planned-toc.md`, `tasks/numerics.md` |
| Figures | `[map-input]` | contingent during writing | chapter writers | Choose figures during the relevant chapter writer sessions; visualization likely gets a short top-level chapter with a small figure spread. | `thesis/planned-toc.md`, `data-and-figures-are-traceable.md` |
| AI process reflection | `[future]` | contingent during writing | agent-log analysis + Jorn | Decide section length after designing the agent-log analysis; current range is short factual section with figures to a longer discussion of prompts and AI changes over six months. | `thesis/planned-toc.md`, `research/INDEX.md`, `tasks/infrastructure.md` |
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
  The old tube section was deleted from the active tree while the tube
  algorithm is re-imported from Jörn's 2026-05-04 raw source note. A targeted
  scan found no current figure/table environments.
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
- [fresh 2026-05-05] `thesis/planned-toc.md` now contains the Kai/Jorn
  meeting-derived outline plus inline comments for settled decisions, outline
  gaps, and chapter-local writer-session questions. The old
  `thesis/planned-toc.draft.1.md` was deleted because git history can recover
  it if needed.
  Refresh by: reading `thesis/planned-toc.md`.

## Pruned / Stale

- [stale 2026-04-24] Old April writing schedule and pre-writing packet queue
  are superseded by finish mode and should not drive task selection.
- [stale 2026-05-05] Earlier planned-ToC drafts and pre-meeting outline spikes
  are superseded by `thesis/planned-toc.md`.

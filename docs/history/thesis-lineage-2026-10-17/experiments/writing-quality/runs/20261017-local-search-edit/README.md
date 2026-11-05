# A bounded local-search prose alternative

17 October 2026; native worker `/root/local_search_prose`; PM task
`f5a1d421-01e9-4936-a423-02eea3646dbc`, parent prose goal `56f90800`.

The concrete end state for this step is an applicable alternative to the current
Chapter 8 local-search subsection, with its empirical statements and references
preserved, so the owner can inspect and compile an actual edit. The whole-thesis
success story remains the one in [OWNER-PLAN](../../OWNER-PLAN.md): an exact PDF
Jörn can follow and endorse, with Kai finding only minor problems that Jörn can readily repair. This packet
supplies no human acceptance observation and no new mathematical result.

The author recommends the modest restructuring in [after.tex](after.tex), rather
than applying a new global writing workflow. The current manuscript was not
edited. [proposed.patch](proposed.patch) applies to the selected source. The exact
original is [before.tex](before.tex); [verification.json](verification.json) binds
it to the inspected source. Separate root verification/integration owns any
manuscript edit, build, page inspection, commit and cleanup.

## Why this edit, and what could be skipped wrongly

1. Read the current subsection, its Chapter 6 branch model and Appendix local
   optimizer account: without those, a comfortable explanation might silently
   turn an implementation ranking into evidence that retaining history caused
   the advantage, or treat a chosen branch window as complete.
2. Recover actual human feedback and current-source dispositions: otherwise a
   source audit could repeat the earlier mistake of treating accurate prose as
   accepted prose or disturb accepted correlation/area passages.
3. Produce exact before/after and a patch: an abstract recommendation would leave
   the next owner to perform the authoring step again.
4. Check table/reference preservation and applicability: these verify the small
   proposed source change. A separate build and rendered-page check remain
   necessary before any source integration is delivered.

## Real alternatives and selection

The alternatives are actual texts, not proposed reviewer architectures:

| Option | Text | Recoverable result/argument | Scope and effort | Decision |
| --- | --- | --- | --- | --- |
| Keep current text | [before.tex](before.tex) | Correct explanation and fixed-policy result already exist; first paragraph supplies motivation, and finite-gap/history definitions recur after the table. | No edit risk or integration effort. | A reasonable baseline, not a failure label. |
| Replace only the vague opening | [minimal-option.tex](minimal-option.tex) | Opens directly with the branch-switch obstruction: “Increasing one currently minimizing branch can expose another as the new minimum”. | Smallest edit; table precedes the heuristic-objective explanation and model definitions still recur. | Better opening, weaker response to the known result-first preference. |
| Lead with the finite result; consolidate its explanation | [after.tex](after.tex) | Names the two highest-median implementations first, states the heuristic status before values, then explains branch competition once after the table. | Same table, model limitation, references, allocation and interpretation; some local order changes. | **Recommended local candidate**, subject to root checks and human judgment. |

Rubrics considered and their failure modes:

- Shortness alone would reward deleting numerical scope or branch-window limits;
  it does not justify this choice. The observed reduction is incidental.
- Source completeness alone would reward repeating every implementation detail;
  it checks fidelity but cannot establish useful exposition.
- Result-first topic sentences alone could conceal the mechanism or imply causal
  superiority. Here the opening names fixed implementations; the following
  model paragraph explains motivation without claiming this comparison isolated
  the value of history.
- Recoverable result, support and limits, considered together with existing human
  preferences, favor the third option. This is an editorial judgment from one
  shared project history, not an experimentally measured quality score.

The human basis is [annotation 10](../../human-review/responses/20260918-root-contemporary-ds-reader.md)
(qualified preference for results visible in paragraph openings), annotation 3
(phenomenon before operational detail), and annotations 5/8 (stable explicit
quantity names). The same record rejected that older sample overall. It did not
judge this subsection, and current-source changes already addressed much of the
older feedback; [the disposition record](../../../../docs/history/thesis-restart-2026-10-04/ds-comment-disposition.md)
prevents treating all historical objections as current defects.

## Meaning and evidence comparison

| Content | Original / proposed treatment | Inspected support and boundary |
| --- | --- | --- |
| Comparison result | Original “Retaining branches from earlier points gave the largest median…” becomes an opening identifying the two branch-history implementations and the two highest medians. | The seven-row current table and canonical comparison README rank these policies .985 and .982 above .977. This is descriptive; no isolated causal benefit of history is claimed. |
| Population/allocation | Same 64 generic ten-facet starts, seven implementations, 128 full-call cap and nominal one-second accumulated evaluator-plus-optimizer budget. | [Appendix](../../../../thesis/appendices/data-science.tex), `app:ds-local-optimization`; [canonical comparison README](../../../dev-gradient-ascent/optimizer-comparison/README.md). Equal allocation does not mean identical realized time; work already begun may overrun. Tuning independence is not established; this is not newly labeled held out. |
| Table | Byte-identical, including all seven medians and 10–90% ranges. | Exact original table retained; no figure/table numbers or labels are introduced or changed. This worker did not recompute the scientific table. |
| Objective | Moved before the numerical table, retaining heuristic floating-point objective, floating-point geometry/volume and distinction from the certified implementation. | Same appendix; [runner contract](../../../dev-gradient-ascent/optimizer-runs/README.md). No claim of certified mathematical ratios or completeness of the historical candidate search. |
| Branch-switch motivation | Two explanations around the table become one paragraph after it. It states the lower-envelope obstruction, near-minimum finite-gap branches, earlier history branches and transition prediction. | [Chapter 6](../../../../thesis/chapters/06-variation.tex), `subsec:first-order-search-model`; implemented-step appendix table. The model is predictive and conditional, not a certificate of capacity ascent. |
| Missing branches | Retains explicitly that a numerical window need not include every relevant branch. | Same Chapter 6 discussion and original subsection. No completeness theorem is inferred. |
| Other methods | Preserves CMA-ES population and coordinate-poll direct-search roles and the CMA-ES citation. | Current source and implemented-step appendix table. |
| Interpretation | Preserves fixed-implementation rather than tuned-family ranking, variation across starts, 448 best-so-far values below one, and the appendix pointer for traces and budget interpretation. | Same source/README. No convergence, local-maximality or global-maximality assertion is added. |
| Neighboring material | Entire before/after span ends immediately before “A failed probe does not establish a local maximum”. No accepted correlation/absolute-area paragraph or pending selection slice is touched. | Patch changes only this subsection. No material outside the designated span is replaced. |

## Actual operations and checks

Only ordinary local reads, drafting and a Python/diff check were performed; no
cloud task, new research run, optimizer producer, questionnaire or human-review
request was launched. Actual operations are retained in [prepare.py](prepare.py).
This worker began at 11:47:18 UTC and completed the retained packet during the
same bounded local session, within the assigned 15-minute cap; this is wall-clock
context, not an active-time or token-cost measurement.

Checks passed: exact seven-row table identity; equal label/reference/citation
multisets; patch applicability with `git apply --check`; source unchanged by the
worker. TeX whitespace-word counts (including table/commands) are recorded only
as a mechanical observation, not a prose-quality metric. The parent owns the
isolated build and visual verification; this packet alone does not establish
layout or reader acceptance.

No human verdict is inferred. The pending selection-passage verdict and parent
prose goal remain open. This different natural passage provides actual editing
material for a transfer test; it is not an independent holdout from the earlier
project feedback and establishes no reliable general authoring method.

## Separate root verification

Root recomputed the seven table medians and 10–90% ranges from the 448
retained run rows, checked the same 64 starts for each implementation and
all values below one, and confirmed the common nominal allocation with
310 realized cutoff overruns. [Numerical receipt](independent-numeric-check.json).
This checks the recorded descriptive statistics, not the evaluator's mathematical
correctness or a new algorithm-family ranking.

The patch changed only the designated subsection in a temporary copy; its table,
labels, references and citation were preserved. A clean isolated whole-thesis
build passed at 105 pages with the same 174 labels and no undefined references
or citations. [Verification receipt](independent-verification.json) records the
scope and hashes. Root inspected [before page64](before-page-064.png),
[after page64](after-page-064.png), [before page65](before-page-065.png) and
[after page65](after-page-065.png): the table, text and adjacent section/figure
are legible, with no overlap or clipping. The selected manuscript and Kai packet
remain unchanged; this is a verified actual alternative, not an integration or
human acceptance claim.

[review: /root/batch_triage]

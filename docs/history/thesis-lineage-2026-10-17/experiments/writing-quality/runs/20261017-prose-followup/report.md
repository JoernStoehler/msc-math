# Bounded response to the current selection passage

17 October 2026 · worker `/root/prose_followup` · task `f4d732a5`.

**Recommendation: preserve the displayed passage pending Jörn's judgment.**
Its empirical comparisons match the retained summaries and their sampling
boundaries. The earlier objection to narration is relevant guidance, but it
was made about a different version. No answer to current question `9fb6ee9a`
is recorded at this check. The task therefore remains open; this is an agent
assessment and a verified handoff, not a human prose verdict.

The bounded end state was to identify whether existing evidence already
requires a local repair, preserve the content any later repair must carry,
and hand back a concrete response. Recovering the exact passage avoids applying
old labels to new prose; checking its sources separates a factual repair from
style preference; comparing actual changes prevents another broad rewrite from
silently dropping results.

## Exact object and existing human evidence

The reviewed object is the first three paragraphs of “Coarse selection
succeeds; stronger selection need not”: selected source lines 149–177,
266 words under the existing receipt's convention, physical PDF page 59.
The [frozen TeX](../20261017-owner-calibration/source-excerpt.tex),
[current display](../20261017-owner-calibration/current-selection.html) and
[existing receipt](../20261017-owner-calibration/verification.json) identify it.
Fresh SHA-256 checks of the chapter, frozen excerpt, display and 105-page
candidate all agree with that receipt. The excerpt is still a verbatim source
substring; this continuation did not rerender or alter it.

In [September's selection-paragraph feedback](../../../../docs/history/ds-result-trial/human-feedback.md),
Jörn called a related account of 0.626/0.316 and 0.034 “mostly still just
narration instead of getting to the point” and rejected the request for one
fatal defect. That is not a rejection of these exact three paragraphs.
The broader [contemporary DS sample](../../human-review/responses/20260918-root-contemporary-ds-reader.md)
received “Needs revision before acceptable”; its local qualifications and
withdrawals stay attached to that version.
The [current-source disposition](../../../../docs/history/thesis-restart-2026-10-04/ds-comment-disposition.md)
records the separate accepted correlation paragraph and accepted absolute-area
explanation. Neither is inside this selection slice, and neither label transfers
to it. They must remain intact in any larger edit.

## What the source check established

| Passage content | Direct support and preservation requirement |
| --- | --- |
| Small $R$ gives coarse mean enrichment, without reliable further ordering | Paragraph 1's contrast is resolved by paragraph 3; retain both sides. These are finite numerical observations, not a universal selector theorem. |
| 100,000 products; ten lowest-$R$ per side-count group; means 0.626 and 0.316 | Initial selection summary gives 100 selected and 100 comparison bodies, ten per group, means 0.626116682098113 and 0.315503943368181. Resolved config identifies seed 271828 and 10,000 candidates per group. |
| Geometry-only selection; equal group contributions; disjoint same-pool controls chosen without capacities | Appendix A.5 and initial packet specify fixed hash-order comparisons excluding each selected set. Preserve “comparison”; do not upgrade it to random sampling or ten independent replications. |
| $C(K)=\max_F A_\omega(F)/\sum_F A_\omega(F)$ when the denominator is positive | The formula measures largest-ridge share. Translation and linear symplectic maps preserve each ridge area; common dilation scales numerator and denominator alike. Preserve the domain condition and the distinction from $R$. |
| Lower-$C$ half inside the lowest 1% by $R$ improves the mean by 0.034, in eight of ten groups | Concentration table has 50 lower-$C$ and 50 complementary bodies in each group; overall difference 0.03399246962136249, eight positive groups. Config identifies seed 1618033. This is a second pool, separate from paragraph 1. |
| Within that 1,000-body low-$R$ panel, correlations span −0.315 to +0.301; pooled within-group ranks give +0.032; extreme tenth improves in two groups | Tail-dependence table gives endpoints −0.31464746474647465 and +0.3014701470147015, and two positive tenth contrasts. Its summary gives +0.03196999699969997 and 1,000 rows. This analysis reuses paragraph 2's seed-1618033 panel; it is not a third independent validation. |

Checked numerical sources are the initial [selection summary](../../../../topics/empirical-studies/sys-datascience/methods/extreme-scalar-rejection-proposer/artifacts/100k-promising-scalars/selection-summary.tsv),
the concentration [incremental table](../../../../topics/empirical-studies/sys-datascience/methods/extreme-scalar-rejection-proposer/artifacts/100k-ridge-concentration-validation/incremental-validation.tsv)
and [verdict](../../../../topics/empirical-studies/sys-datascience/methods/extreme-scalar-rejection-proposer/artifacts/100k-ridge-concentration-validation/validation-verdict.json),
and the [within-tail table](../../../../topics/empirical-studies/sys-datascience/methods/tail-dependence-feasibility/artifacts/current/frozen-100k-bucket-trends.tsv)
and [summary](../../../../topics/empirical-studies/sys-datascience/methods/tail-dependence-feasibility/artifacts/current/summary.json).
Their packet READMEs and both resolved configs establish sample identity and
scope. Fresh arithmetic checks reproduce the rounded contrasts/counts from
these compact tables; no geometry, capacity or method producer was rerun.

## Actual alternatives and decision

A fluency-only rubric would repeat the earlier AI approvals of rejected prose.
A completeness-only rubric would push selection bookkeeping back into the main
argument. A shortest-text rubric would reward losing the conditional failure.
The useful comparison is recoverable result/inference, source fidelity,
version-specific human judgment and integration cost.

| Response | Passage-specific benefit and risk | Disposition |
| --- | --- | --- |
| Keep the current three paragraphs | The opening states coarse success versus failed further restriction; the numerical comparison follows immediately. The largest-ridge-share paragraph defines a genuinely different filter, and the final paragraph states the weak within-tail ordering explicitly. No supported result or current human label is lost. | Selected provisional response. This does not certify that Jörn finds the explanation efficient. |
| Tiny sample-identity clarification | Current “panels themselves” suggests reuse, but paragraph 3 does not explicitly identify paragraph 2's pool. The retained evidence permits “in that sample's lowest one percent” in place of “in the lowest one percent.” This makes the dependence explicit without adding operational detail or changing claims. | Actual unintegrated candidate in [sample-identity-only.patch](sample-identity-only.patch). It addresses a concrete possible ambiguity, not a demonstrated human objection; it is not applied to the pending judged version. |
| Reorder or replace the whole account | Moving the within-$R$ failure before $C$ would place the two $R$ comparisons together, but then the current shared-sample reference needs new wording. A broader old 916-word replacement also loses integrated results and live references, as the [placement note](../../../../meta-topics/repository-organization/inventory/pilots/2026-10-11/writing/placement-note.md) demonstrates. More text change has no new human diagnosis to target. | Do not select on this evidence. |

The existing shorthand “total ridge area is small” is read in the established
normalized-$R$ context, not as a scale-dependent raw-area claim. Any future
rewrite must retain that normalized meaning. Neither this check nor the tiny
sample-identity candidate establishes a geometric cause, independence,
asymptotic behavior or reliable upper-tail prediction.

## Integration boundary and receiving owner

The slice contains no `\label`, `\ref` or `\eqref`; its displayed $C$
formula is unnumbered. Its required context is the preceding definitions of
$A_\omega$, $R$ and numerical systolic ratio. Immediately after it, the generic
ten-facet comparison and area/alignment factorization remain, including the
pointer to `app:ds-selection`. The chapter's accepted correlation paragraph,
three population/conditional figures, and later exact equations
`eq:ds-pentagon-ridge-capacity`, `eq:ds-triangle-profile` and
`eq:ds-product-ridge-minimum` are outside this replacement boundary.
The patch changes only the sample-identifying phrase and preserves every
number, formula, label, reference and adjacent paragraph.

Root owns retention, review, PM state, integration and commits. The unresolved
dependency is exactly Jörn's keep/rewrite/context judgment on question
`9fb6ee9a`. Preserve its eventual label with the unchanged excerpt hash. If he
accepts, retain that local acceptance; if he reports a defect, make one repair
that addresses it while using the content map above; if context is missing,
restore the necessary context instead of recording a prose failure. No answer,
chapter acceptance or whole-PDF acceptance was inferred here.

[Verification receipt](verification.json) records hashes and fresh bounded
checks, including the proposed patch's strictly local effect.

[review: /root/delivery_review]

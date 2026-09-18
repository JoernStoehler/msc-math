# Prospective comprehension review

Read as a mathematical reader familiar with symplectic capacity and systolic ratio, but unfamiliar with these experiments. I read only `baseline.md`; this is a comprehension review, not a source-fidelity or mathematical verification audit. This review is frozen at its first written version.

## Reconstruction

The section reports a strong inverse rank association between a scale-invariant sum of absolute symplectic two-face areas and a historical numerical estimate of systolic ratio in a particular mixed sample. It explicitly separates those estimates from certified mathematical values and distinguishes pooled correlation from a universal relationship. For Lagrangian polygon products the descriptor is a sum of absolute edge pairings. A supplied capacity formula for rotated regular pentagon products makes the inverse relationship exact on that family, including ratios above one. Other product paths reverse the direction, so minimizing the descriptor is not a general improvement method. Subsequent experiments support using ridge geometry to allocate capacity evaluations to promising candidates; an additional concentration feature improves one prescribed comparison. Local optimization is a separate subject handled elsewhere.

## Concrete comprehension or inference obstacles

1. **Which target is used in the opposing-path comparison?**

   Span: “decreasing $R$ also decreased the numerical systolic ratio at every retained step, ending at ratios $3/4$ and $1/2$, respectively.”

   The introduction carefully distinguishes the historical target $\widehat{\operatorname{sys}}$ from mathematical $\operatorname{sys}$. Here “numerical systolic ratio” and exact-looking fractional endpoints leave me unsure whether the reversal concerns the same historical estimator, a certified calculation, or known exact endpoint values combined with numerical intermediate values. I understand the intended warning against a universal ascent rule, but cannot tell how strongly these paths establish that warning about mathematical systolic ratio. Confidence that the wording leaves this distinction unresolved: high. The surrounding thesis could supply the missing status.

2. **What result does the first large search test establish about selection?**

   Span: “A more direct search test generated 100,000 product candidates, selected bodies using geometric features before evaluating their targets, and then evaluated a selected/control union of 1,675 bodies.” Followed by: “The largest stored target was approximately $0.867546$, and none exceeded one.”

   I can reconstruct the evaluation-saving procedure, but these outcome numbers do not tell me whether selected candidates outperformed controls: the maximum is stated for the union, with no attribution to either group. The initial sample maximum is lower, but that is not itself a comparison against this test's controls. Consequently I cannot infer whether this particular test supports enrichment, merely demonstrates the search procedure, or had mixed results. The later refinement comparison does provide an interpretable benefit, so this is a local gap rather than an obstacle to understanding the section's overall conclusion. Confidence: high about the unavailable comparison; moderate about whether the author intends this test itself to demonstrate enrichment.

## Conditional context dependencies

1. Span: “The capacity profile proved in the pentagon analysis gives”.

   This is a legitimate dependency for a section whose surrounding thesis covers the capacity proof. The displayed formula lets me follow the exact relationship without reading that proof. In the assembled thesis, however, I need a resolvable reference to know which result supplies it, especially because the ensuing value exceeds one. This is not a request to reproduce the proof here. I cannot tell from this isolated text whether that reference is already supplied by its eventual context.

2. Span: “Predictive models and feature ablations found that ridge-area features carried much of the held-out signal.”

   I read this as background motivation, not a numerical claim I can assess here. “Ridge-area features” might include the descriptor, concentration measures, or several other summaries; “much” supplies neither a comparison nor a magnitude. If this sentence is meant to substantiate the usefulness of $R$ specifically, it needs a short account of which features and what comparison, or a reference to that account. If it is just a bridge to the direct selection evidence that follows, the present level of detail is sufficient for comprehension. Confidence in the intended role: moderate.

3. Span: “A transfer study on a separately area-normalized source also found that a frozen ridge selector increased mean target against disjoint controls”.

   The direction of the result is clear. What changed between sources, and what exactly remained frozen, are not specified enough for me to understand the scope of “transfer.” A study reference could carry that detail. If the intended takeaway is only another finite enrichment observation, this omission does not block it; it matters if the sentence is supposed to establish robustness across substantially different generation procedures.

## Mere preferences

- Span: “The selected tail was split into halves by that largest-ridge share.” I would find a displayed definition of largest-ridge share convenient, but infer it to mean the largest absolute symplectic face area divided by their sum. The preceding definition and wording give me a workable interpretation, so I do not count this as a comprehension defect.
- Span: “A separate fresh 100,000-candidate experiment tested a refinement”. A paragraph break here would make the two search experiments easier to locate on rereading. Their distinction is already explicit; this is a presentation preference.

The geometric calculation and the distinction between choosing a starting body and improving that body are understandable from the section itself. My principal unresolved inference concerns the evidential status of the opposing paths; the remaining issues mainly limit how much I can conclude about particular selection experiments.

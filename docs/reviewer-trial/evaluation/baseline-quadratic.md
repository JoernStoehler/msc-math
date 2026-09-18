# Review of the quadratic-program excerpt

The mathematical development is generally well signposted for the stated MSc reader. I found two writing concerns worth raising. I have ignored PDF extraction artifacts and assumed the preceding capacity theory and pure-facet minimizer result are available.

## Concerns

1. **The implementation overview introduces too much numerical terminology at once.**

   Exact quotation: “On the remaining words a floating-point LDLT factorization proposes curvature and a KKT solution. Outward residual bounds certify the decisions; an exact rational solve is used only when the floating-point certificate is indeterminate. Outward objective intervals are aggregated to a certified capacity interval.”

   KKT has been explained, but “proposes curvature,” “outward residual bounds,” and “outward objective intervals” have not. The reader must infer both what decisions are being certified and how numerical approximations become rigorous bounds. Section 11 is explicitly offered for details, which helps, but this overview still asks the reader to decode several specialist terms before understanding its main claim. The later “exact capacity of the supplied binary64 input” adds another unexplained numerical distinction. This is a local accessibility problem in Section 4.5, rather than a request for a numerical-analysis digression throughout the chapter.

   Confidence: **high**.

2. **“Block” temporarily changes scope during the twelve-facet proof.**

   Exact quotations: “We call a single facet a one-facet block, and an ordered adjacent pair of facets of the same type a two-facet block.” And: “If the source block is a singleton, deleting it coalesces its two neighboring opposite-type blocks and reduces both block counts by one.”

   The initial definition and theorem describe blocks of one or two facets. During merging, however, the coalesced run can have more than two facets; the proof establishes the final length bound only after reconstruction on the boundary. Consequently, the reader tracking the claimed invariant must recognize an unstated intermediate use of “block” as an arbitrary maximal same-type run. The later sentence “Hence every maximal block in the final word has length one or two” makes the intended argument recoverable, but the terminology can interrupt the proof at precisely the point where preserving the block count matters.

   Confidence: **medium**.

## Passages worth retaining

1. **The four-facet example makes the local/global distinction concrete.**

   “The corresponding capacity upper bound is 4. Identifying the capacity still requires a global comparison or another argument giving the matching lower bound.”

   This explains what the elementary calculation has actually established and prevents the reader from mistaking a solved word for a solved capacity problem.

2. **The warning before the billiard surgery anticipates a natural geometric misunderstanding.**

   “In particular, splitting a normal-cone direction into pure facet directions does not assert that the corresponding broken path, drawn directly in Kq or Kp, would remain inside that polygon. Boundary placement is recovered only after the surgery has been shown to preserve dual minimality.”

   This gives the reader a specific distinction to track through the proof and explains why reconstruction is deferred.

3. **The opening of the six-facet reduction supplies the mechanism before the notation.**

   “After separating the total weight in each factor from its normalized distribution, the objective is therefore linear in either distribution when the other is fixed. Maximizing these two linear functions at vertices is the mechanism behind both the six-facet bound and the following algorithm.”

   This prepares the reader for the successive vertex choices and makes the subsequent proof much easier to organize mentally.

## Context that could change the judgments

An earlier definition of a block as an arbitrary maximal same-type run would reduce the second concern. Prior explanations of rigorous interval arithmetic, outward bounds, and binary64 input would reduce the first. I have treated the referenced dual reconstruction and splitting–merging results as established context, rather than requiring their proofs to be repeated here.

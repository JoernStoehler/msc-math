# Review

The excerpt makes its main distinction clear: a descriptor that helps select candidates need not tell us how to deform a particular body. The following three concerns are about access to the evidence, rather than objections to that distinction.

## Concerns

1. **The transfer experiment introduces several unexplained methods at the point where the reader needs to assess its result.**

   Exact quotation: “A separate transfer experiment used frozen rho and ridge selectors on a different, separately factor-area-normalized source, called `factorial-both`. Selected means exceeded disjoint-control means in both its \(4\times6\) and \(6\times6\) buckets, across 91 distinct evaluated bodies.”

   “Rho” has no definition in the excerpt, and neither the selection rules nor the meaning of “frozen” is given. The source name does not explain what changed in the generation procedure. Although a reader can infer that the selections performed better than controls, they cannot tell what was selected, what was held fixed, or what comparison makes this a transfer experiment. This is a substantial obstacle for the stated reader, who is not assumed to know machine-learning terminology.

   Confidence: **high** for the excerpt in isolation; lower if immediately preceding text defines these methods.

2. **The original sampling description leaves the population partly unspecified.**

   Exact quotation: “The exploratory population comprised 4,096 generic polytopes, with 512 at each facet count from five to twelve, and 10,240 Lagrangian polygon products, with 1,024 in each bucket (3\leq k\leq m\leq6). Generation used seed 42 and support heights in \([0.8,1.2]\), subject to acceptance conditions.”

   The text never explicitly identifies \(k,m\) as the factors’ edge counts, and “acceptance conditions” supplies no information about which generated bodies were retained. The subsequent warning that source composition matters makes these omissions consequential: the reader is asked to interpret an association for a particular sampling procedure without enough information to identify that procedure. A nearby methods reference could supply the missing information without expanding this passage substantially.

   Confidence: **medium**, since the edge-count interpretation is readily inferred and the acceptance conditions may already be documented in surrounding text.

3. **The capacity argument is presented as a proof summary without an identifiable proof location.**

   Exact quotation: “The pentagon capacity proof uses the variational formula: feasible facet weights remain unchanged under rotation, while each fixed-weight quadratic expression has the form (A\cos\theta+B\sin\theta). Endpoint bounds and an explicit attaining six-facet word determine the capacity throughout the reduced interval.”

   The endpoint bounds and attaining word are the substantial steps supporting the capacity profile, but neither is stated or linked to a theorem here. Familiarity with EHZ capacity does not necessarily include this particular facet-word argument. If this is meant as a summary of a proof elsewhere, the reader needs a precise pointer; if it is the only proof offered, it leaves too much to reconstruct. The issue is the reader’s ability to follow or locate the justification, not an assessment that the formula is incorrect.

   Confidence: **medium**, strongly dependent on the surrounding chapter’s proof and cross-references.

## Passages worth retaining

1. “It might identify promising bodies among newly generated candidates, or indicate how to improve a body already chosen. These uses require different evidence.”

   This gives the reader two concrete uses to distinguish before any experimental or mathematical detail appears. It makes the later adverse paths relevant rather than surprising digressions.

2. “The absolute value is taken separately on each face, so different faces cannot cancel through their orientations. This measures symplectic area rather than Euclidean area. The normalization makes (R) unchanged under common dilation: both a face integral and the square root of four-volume scale quadratically.”

   These sentences anticipate the natural questions raised by the definition and answer them directly, including the reason for the normalization.

3. “A population association cannot by itself determine the effect of deforming an individual body.”

   This states the statistical limitation in ordinary mathematical language. Its placement after the adverse paths gives the limitation a concrete basis, and it avoids requiring statistical vocabulary.

## Missing context and predictions

The nearby definitions of the selectors and source, the sampling-methods reference, and the location of the pentagon capacity proof would change the severity of the concerns. If those are already supplied locally, concerns 2 and 3 may need no action, and concern 1 may reduce to a reminder or cross-reference.

Before any human feedback, I predict that the unexplained transfer-experiment terminology is the most likely concern to survive review of this excerpt alone. The sampling and proof concerns are more likely to be resolved by surrounding context. I expect the distinction between candidate selection and individual deformation to remain worth retaining regardless of those contextual answers. These are predictions about reader difficulty, not evidence of human acceptance. No human request is needed to complete this excerpt-only review.

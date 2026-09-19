# Review of “Ridge area and systolic ratio”

The excerpt generally gives its intended reader a clear route from an empirical observation to an exact example and then to a selection experiment. I identify two concerns that merit attention in the supplied context.

## Concerns

1. **The opening describes a different quantity from the one subsequently analysed.**

   Exact quotation: “In the sampled polytopes, a smaller total symplectic area of the two-dimensional faces usually accompanied a larger systolic ratio.”

   The plotted quantity and correlations concern the volume-normalized sum defining $R$, whereas this sentence describes the unnormalized total. Those quantities need not order bodies in the same way when their volumes differ. Because the sentence introduces the central empirical claim, a reader could carry the wrong interpretation into the subsequent explanation. The definition makes the normalization visible, but does not say whether the opening is shorthand or a separate observation.

   **Confidence: high.** If all sampled bodies were normalized to a common volume before these measurements, that context would substantially reduce the concern; the supplied historical-sample description does not establish this.

2. **The control population depends on an unexplained selection method.**

   Exact quotations: “Sixteen controls were chosen by a fixed hash ordering from bodies outside both geometric selection sets in this experiment, before capacity evaluation.” And: “The experiment also tested a covariance-based selector, not discussed here. Its selected bodies were excluded from the shared controls as well.”

   The reader needs to understand which bodies supply the comparison means in the table. The footnote identifies the second exclusion but leaves its geometric meaning and extent unspecified. For this audience, “covariance-based selector” supplies little usable information, and “fixed hash ordering” also introduces a computational term without explaining what property of the control choice it ensures. Consequently, the reader can see that selection preceded capacity evaluation but cannot judge how the controls differ from the eligible pool beyond failing the discussed rule. This matters to the interpretation of the reported enrichment, even with the appropriately finite-pool conclusion.

   **Confidence: medium.** A nearby methodological account explaining the second selector, the number of excluded bodies, and the control-ordering procedure could resolve much of this concern. The concern is about interpretability, not evidence that the control procedure is invalid.

## Effective passages worth retaining

1. **The distinction between symplectic and Euclidean area is explicit.**

   “The absolute value is taken separately on each face: $R$ records symplectic area without cancellation between faces, rather than their Euclidean areas.”

   This sentence answers two natural questions about the definition immediately, without requiring the reader to reconstruct its conventions.

2. **The correlation discussion translates an unfamiliar statistic into the mathematical observation it supports.**

   “Rank correlation compares the orderings of the bodies by the two quantities; a value of $-1$ would mean exactly reversed orderings.” And: “Thus the principal observation concerns which bodies have larger systolic ratio, rather than how much the systolic ratio changes per unit of ridge area.”

   Together these passages explain the statistic and its interpretive consequence. They make the contrast with ordinary linear correlation useful for a reader without statistical training.

3. **The concluding distinction connects the experiments to the geometric question.**

   “The pentagon identity supplies an exact example of the inverse association, while the opposing deformation paths show why successful selection from a population need not give a rule for improving each individual body.”

   This gives the exact calculation, counterexamples, and selection experiment distinct roles in one coherent conclusion. It states clearly what the reader should and should not infer from their combination.

## Missing context

I have not seen Figure 1 or its underlying visual asset, so I make no judgment about its visual clarity or whether its panels support the caption's description. The accompanying source notes could clarify volume normalization and the control construction. The pentagon chapter is also absent; this review treats the capacity profile as an explicitly cited prior result, rather than judging the adequacy of its proof. Details of numerical accuracy for the fresh selection experiments would affect how strongly their reported differences can be interpreted; the explicit provenance limitation in the historical-sample discussion does not by itself settle that question.

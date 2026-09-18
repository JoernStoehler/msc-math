# Baseline review: topic transfer

Reviewed the supplied excerpt for an MSc mathematical audience with the stated background, including graduate linear algebra, compactness, the inverse function theorem, basic symplectic geometry, and earlier capacity variational formulas. PDF extraction artifacts are excluded from this assessment.

## Concern

1. **The computational proof component lacks an identifiable reference and a clear verification report.**

   Exact quotations: “The exact fixed choices recovered by the existing verifier are β6 = r, β7 = z”; “The finite predicates are:”; “These exact arithmetic predicates, followed by the positive-convex-margin and symmetry-slice arguments, prove the existing theorem.”

   The text supplies the data and formulas needed to specify the calculation, which is useful. However, the rank and strict-sign assertions carry the central conclusion, and the reader is referred to an “existing verifier” and an “accompanying computational witness” without being told where to find them. The transition from listing predicates to declaring the theorem proved also leaves the status of their verification less explicit than the reader needs: are these recorded successful checks, instructions for checks, or assertions supported elsewhere? This obstructs following the evidence for the proof, independently of whether the assertions are mathematically correct.

   **Confidence: medium.** A preceding computational-methods section or a clearly identified accompanying artifact could resolve this concern.

## Effective passages worth retaining

1. **“For every nonzero direction h, at least one rj (h) must be negative. Indeed, if all were nonnegative, their positively weighted sum could vanish only if they were all zero. The spanning assumption would then force h = 0.”**

   This gives the crucial linear-algebra argument directly. The subsequent compactness and remainder estimates make clear why directional decrease implies a neighborhood statement.

2. **“The exact calculation checks that these fifteen vectors are independent; group dimension alone would not exclude a stabilizer at this body.”**

   This concisely explains why the independence check is needed, rather than merely presenting a dimension count. It prepares the reader for the slice and inverse-function-theorem argument.

3. **“Nevertheless the five-by-five closure minor is invertible. No assertion that a nearby stationary or optimizing branch continues is needed: the displayed section remains feasible, and therefore supplies a smooth upper bound.”**

   The passage makes the role of the seven-facet example explicit and distinguishes the property the proof needs from the stronger property that singularity would make problematic.

## Missing context

The location, description, and verification results of the referenced computational witness would change the concern above. Earlier capacity-formula background is assumed as instructed; the excerpt need not reintroduce that theory. This is a writing review, not an independent verification of the exact arithmetic.

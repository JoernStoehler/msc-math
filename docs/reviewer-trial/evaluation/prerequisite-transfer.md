# Prerequisite reconstruction: transfer prediction

Scope: the supplied preliminary-mathematics excerpt and figure captions only. No images, surrounding manuscript, external sources, labels, or other reviews were consulted. Extraction artifacts are excluded. These predictions are saved before scoring.

## Consequential finding

### 1. The admissible curves and the meaning of their weak equation need one analytic bridge

**Anchor:** Definition 2.1 introduces `W^{1,2}([0,T], R^4)` without explanation; Section 2.6 later uses a “measurable L² choice of p,” a “distributional equation,” and integration by parts to obtain equation (10).

**Reader's unresolved question:** “What regularity do these curves actually have, and in what sense can I differentiate and integrate by parts when neither the boundary nor the conjugate is smooth?”

**Prerequisite mismatch:** Graduate real analysis and basic differential geometry do not necessarily include Sobolev spaces or distributional derivatives. This is a new analytical framework, not just notation for familiar smooth curves. The reader needs it to understand what Definition 2.1 permits and how the dual minimizer yields a characteristic. The earlier action formula is explicitly for absolutely continuous curves, but the text does not connect that class to the new notation.

**Why context does not resolve it:** The excerpt carefully identifies the nonsmooth multiplier theorem as an imported result and explains its signs and constants. That settles the source of the assertion, but not the meaning of its weak formulation. The subsequent computations presuppose the missing vocabulary. This is a localized prerequisite gap; the excerpt already explains why the dual problem and reconstruction are useful.

**Confidence:** Moderate (0.75). Some graduate real-analysis courses cover this material; the stated background does not guarantee it.

**Smallest useful improvement:** At the first occurrence, identify these one-dimensional Sobolev curves with absolutely continuous representatives having square-integrable derivatives. At the weak variational equation, add a sentence explaining that a distributional identity is tested against smooth periodic variations, so the integration-by-parts step does not assume classical differentiability. Keep the existence of the subgradient selection and multiplier explicitly with the cited theorem.

## Passages that spend explanation effort well

1. **Sections 2.1–2.2, characteristic direction through equation (3).** The progression distinguishes the geometric curve from its parametrization, fixes the positive direction, and then uses homogeneity to show why the nonsmooth Hamiltonian inclusion has exactly the same action–period normalization. The reader can now interpret the minimization in Section 2.3 without treating “period” and “action” as unexplained synonyms.

2. **Section 2.5, the coordinate-pairing display and `J₀(u,v) = (−v,u)`.** The passage uses familiar linear algebra to explain the operational consequence of the Lagrangian product: a supporting row in one factor drives motion in the other. This supplies a reason to care about the coordinate convention and a clear dependency for the later facet dynamics.

3. **Section 2.6, its opening paragraph and the final feasibility/minimality distinction.** The opening identifies the obstruction to editing boundary curves and the purpose of a velocity-only functional before presenting conjugate machinery. The closing explains precisely when a modified dual curve can be returned to the boundary. Together they make the theorem's role in the promised facet-velocity rearrangement intelligible, rather than leaving a formally complete but purposeless chain of definitions.

No additional consequential findings are asserted. In particular, the contact terminology is accompanied by operational equations, the capacity properties are explicitly marked as background, and the nonsmooth variational theorem is explicitly identified as imported; those are not treated as missing derivations.

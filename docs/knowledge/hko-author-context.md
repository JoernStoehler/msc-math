> Retained source-interpretation knowledge, not a current assignment or fresh audit.
> Original text: `ee282e065004adc1ccd64e51140edccd075845de:docs/history/memories/hko-author-context.md`. Original source routes below describe the September 5 source state; current selected source is `thesis/chapters/07-hko.tex` (HKO) or `thesis/chapters/08-data-science.tex` (DS).

# HKO author context — pilot

This pilot preserves source-derived understanding for writing and tracing the
HKO local-maximality exposition. HKO local maximality is an established theorem
in this project (user-confirmed context); theorem existence is not an open task.
The account below comes from reading the active thesis and theorem README, not
an independent proof audit or a new certificate run. Inspected working-tree
source state on 2026-09-05 had HEAD
`3a00abcc6ada31341b7064c98bf861076857c5da`; this records the baseline commit,
not a claim that every inspected file was identical to that commit.

The theorem concerns a Hausdorff neighborhood among exactly ten-facet polytopes.
Equality occurs along translations, positive scalings, and linear symplectic
changes of coordinates. The proof connects the geometric neighborhood to a
finite certificate: polarity and local labelling give a 40-dimensional chart
of ten dual vertices; the symmetry orbit has 15 tangent dimensions; a
transverse slice therefore has dimension 25. Twenty-six smooth feasible upper
functions touch the systolic ratio at HKO. Their derivative rows annihilate the
symmetry directions, have rank 25, and admit a strictly positive convex
relation summing to zero. Consequently every nonzero slice direction has a
negative upper slope; the uniform Taylor argument yields strict local decrease.
The local slice returns this conclusion to the Hausdorff theorem and its
equality statement. Origin interior, the simple HKO combinatorics, and
sufficiently small perturbations underlie the chart reduction.

For writing, preserve the distinction between a **feasible upper function** and
a nearby optimizing branch. A cyclic facet word and smooth positive feasible
weights give an admissible action bound; with the thesis normalization,
`U_sigma = A_sigma^2 / (2 vol_4(K))` bounds the systolic ratio from above.
The certificate uses singular seven-facet rows as well as nonsingular six-facet
rows, because the latter alone do not span the quotient. Fixing beta coordinates
and using an invertible closure/normalization minor produces feasible sections
without needing nearby optimizing KKT branches. The name “active-branch
diagnostic” for the numerical input can otherwise send an author into optimizer
code to recover an explanation already supplied by this feasible-section route.

Rust selects finite candidate data. Sage reconstructs exact algebraic data over
the ordered field `Q(t)`, where `t = tan(pi/5)` is the root of
`t^4 - 10t^2 + 5` in `(0,1)`, and checks the finite acceptance predicate.
The hand argument owns the geometric identification of supplied formulas, the
Hausdorff chart, the symmetry slice, and the upper-function/Taylor implication.
Numerical perturbation and facet-addition tests are supporting evidence outside
the proof; they do not extend the theorem to changing facet count.

## Source route

- `thesis/07-hko-local-maximum.tex`,
  `thm:hko-ten-facet-local-maximum`: theorem scope and equality set. The opening
  paragraphs explain the HKO representative and symplectic convention; the
  proof-spine box connects geometry, slice, and finite certificate.
- `thesis/07-hko-local-maximum-chart-reduction.tex`,
  `lem:hko-hausdorff-local-ten-facet-chart`: assumptions and geometric chart.
  The subsequent symmetry-action and complement paragraphs supply the slice.
- `thesis/07-hko-local-maximum-exact-certificate.tex`,
  `subsec:hko-local-maximum-computation-with-sagemath`: upper-function formulas,
  singular-row rationale, first-order criterion, and proof of the theorem.
- `thesis/07-hko-local-maximum-sage-verifier.tex`,
  `tab:hko-verification-interface`: each exact acceptance obligation and its
  use in the hand proof; the final paragraph states the trust boundary.
- `thesis/07-hko-local-maximum-empirical-tests.tex`,
  `subsec:hko-local-maximum-empirical-tests`: numerical evidence and its scope.
- `experiments/hko-local-maximum/theorem/README.md`, “Explainability Contract,”
  “Theorem Target,” and “Verifier Predicate”: maintained pipeline explanation,
  witness-field roles, and routes to executable and explained verifier sources.

These sources own the detailed argument and implementation explanation. This
entry is a retrieval aid: a reviewer can start at an active TeX claim, select
its mathematical obligation above, and follow the corresponding support.

For motivation, transitions, and summaries, reuse this architecture and
terminology without crawling the generator. Reopen the geometry and formulas
if the representative, symplectic convention, or normalization changes. Reopen
the certificate interface and exact verification evidence if selected rows,
feasible sections, witness data, or verifier behavior changes. Reopen the chart
and slice argument if the local model or symmetry claim changes. A proposed
extension to changing facet count needs additional support. Current source
changes or explicit corrections override this pilot; this entry does not
certify a later source state.

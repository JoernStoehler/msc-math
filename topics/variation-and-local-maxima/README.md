# Variation and local maxima

This topic owns the retained local-maximality work around the HKO pentagon
product and a second, inequivalent ten-facet body. It asks how the systolic
ratio changes near a body, which nearby variations are controlled, and what
is needed to pass from a finite certificate to a neighborhood theorem.
The normalization throughout is
`sys(K) = c_EHZ(K)^2 / (2 vol_4(K))`.

[HKO author context](hko-author-context.md) preserves dated interpretation and
writing guidance, with its September source routes explicitly historical.
[Optimizer review route](optimizer-review-route.md) preserves the separate
seven-policy finite-budget comparison interpretation: historical evaluator
ranking on its matched starts is not a certified local-maximality result.

## Two local results

**HKO:** for the regular pentagon `P` of circumradius one,
`K0 = P × R(-π/2)P` has ratio `(3 + sqrt(5))/5 > 1`. The counterexample and
its capacity value are due to Haim–Kislev and Ostrover. The selected manuscript proves
local maximality among convex four-polytopes with exactly ten facets, with
local equality precisely along translation, positive dilation and linear
symplectic images. Strictness is on a transverse slice, not in the raw
40-dimensional labelled dual-vertex space where these symmetries preserve
ratio. The statement and proof appear in
[the selected thesis chapter](../../thesis/chapters/07-hko.tex), theorem
`thm:hko-ten-facet-local-maximum`, and its three `hko-lemma-*.tex` inputs.

The later [accepted F≤11 research theorem](hko-local-maximum/f11-acceptance-20261005.md)
extends local maximality to all convex four-polytopes with **at most eleven
genuine facets** in an existential Hausdorff neighborhood, including arbitrary
moving normals and coupled approach sequences. Its 4 October acceptance
combines a complete seven-stage finite replay with independent analytic and
crossed implication reviews, taking the sharp ten-facet theorem and local
symmetry slice as inputs. It does not classify eleven-facet equality or give
an explicit radius. The selected manuscript and PDF still state ten facets;
the F≤11 result has no selected thesis location yet.

The supporting mathematical route for the ten-facet input is
[the feasible-section argument](../../formal/hko-feasible-section-upper-branches.tex).
For each selected HK word, solve closure and normalization with an invertible
minor, hold the complementary beta coordinates fixed, and obtain a positive
feasible section near HKO. Its action bounds the capacity from above; it need
not be a nearby optimizing KKT branch. The exact verifier checks 26 such
sections whose ratio derivatives annihilate the 15-dimensional symmetry
tangent, have rank 25, and admit strictly positive coefficients summing to
one with weighted sum zero. Those predicates force at least one strict
negative upper-branch derivative in each nonzero transverse direction.
Continuity and the symmetry-slice argument supply the neighborhood conclusion;
exact arithmetic alone does not supply that implication. The formal argument
also records the local reduction from Hausdorff-near ten-facet bodies to the
labelled chart.

**Non-HKO:** the rational hexagon–quadrilateral product `P*` in
[hko-local-maximum/non-hko/candidates.json](hko-local-maximum/non-hko/candidates.json)
has ten genuine facets, 24 vertices, volume `1/2`, capacity `1`, and ratio `1`.
It locally maximizes the ratio among all nearby convex four-polytopes with
exactly ten genuine facets, including nonproduct and nonsymmetric variations.
It is inequivalent to HKO even under general invertible affine maps because
HKO has 25 vertices. This local maximum is non-strict: an explicit family has
two independent equality parameters modulo the same symmetries.

The [result account](hko-local-maximum/non-hko/RESULT.md) and
[detailed proof](hko-local-maximum/non-hko/PROOF.md) explain how 24 positive
feasible sections, gradient rank 22, their positive relation, two equality
parameters, and the remaining negative second-order direction combine.
A full rank-40 chart check covers arbitrary nearby row matrices. Complete
product enumeration establishes equality on the product family separately;
feasible sections only provide upper bounds for general perturbations.
The selected thesis chapter states theorem
`thm:non-hko-ten-facet-local-maximum` and supplies the analytic argument.
The retained original and independent exact checks are in
[hko-local-maximum/non-hko/verification/](hko-local-maximum/non-hko/verification/).
They support the finite predicates and rational-function identities; the
independent implementation is not human refereeing or proof-assistant
formalization.

[Variation interface audit](variation-interface-check/README.md) retains the
18 September source/formula check and an optional proposed prose patch. Its
recorded source routes and tests are historical; it does not establish current
integration status or generic branch coverage.

## Evidence and unsuccessful routes

[hko-local-maximum/](hko-local-maximum/README.md) remains one coherent Cargo
package: theorem selection and numerical support share its explicit local
helpers and HKO fixtures. Its proof packets, producers, analyses, retained
data and figures stay together. The empirical children have different
questions, sampling measures and trust roles:

| Packet | Finding and boundary |
| --- | --- |
| [Accepted F≤11 research](hko-local-maximum/f11-acceptance-20261005.md) | Immutable proof return, exact finite replay and analytic acceptance; manuscript integration pending. Finite checks alone do not imply the neighborhood theorem. |
| [Theorem certificate](hko-local-maximum/theorem/README.md) | Exact 26-section first-order predicate over the ordered field `Q(t)`, `t = tan(π/5)`. Rust proposes finite choices; Sage reconstructs and accepts exact objects. |
| [Smooth-only rank defect](hko-local-maximum/smooth-only-rank-defect/README.md) | In the retained f64 diagnostic, 44 nonsingular positive-beta branches have projected rank 23 in the 25-dimensional quotient. This route did not close; it does not prove that every nonsingular-only repair is impossible. |
| [First order](hko-local-maximum/empirical/first-order/README.md) | Near-optimal orbit bookkeeping and a rejected/no-change ascent step. A single branch gradient is not a nonsmooth critical-point test. |
| [Second order](hko-local-maximum/empirical/second-order/README.md) | Negative retained median curvature on the 15 numerical basis directions and 100 sampled flat directions; small-scale deltas include near-zero positive values. No unrestricted negative-definiteness claim. |
| [Neighborhood sampling](hko-local-maximum/empirical/neighborhood-sampling/README.md) | General ten-facet perturbations, eleven-facet additions, product-preserving sweeps/probes and a frozen quotient-ray panel. Counts and distributions belong to the named sampler and gauge. |
| [Quotient rays](hko-local-maximum/empirical/neighborhood-sampling/m10-quotient-ray/README.md) | All 32 retained seed-44 rays have a route-resolved nominal above-to-below transition. No positivity inradius, first mathematical exit, star-shapedness or exclusion of exceptional thin sets follows. |
| [Eleven-facet ascent](hko-local-maximum/empirical/m11-ascent/README.md) | None of 20 retained cut-and-ascent placements improves the HKO value under the producer threshold. This is evidence about its search policy, not an eleven-facet theorem. |
| [Row-bank validation](hko-local-maximum/row-bank-validation/README.md) | Independent Sage cross-checks of selected Rust exact rows; this is validation machinery rather than the 26-row theorem predicate. |

The older [smooth-envelope sketch](../../formal/hko-local-maximality-conditions.tex)
is unresolved; it is not the local theorem’s proof. The singular-KKT-gradient
interpretation is superseded by explicit feasible sections. Singular seven-facet rows are accepted as feasible upper branches;
the certificate does not justify treating them as differentiable optimizing
KKT branches. The non-HKO discovery archive also records an aborted full
scalar capacity poll; that heuristic failure is not a premise of the exact
proof.

## Scope, reproduction and remaining limits

The selected manuscript states both local theorems for ten genuine facets.
The accepted HKO research theorem covers at most eleven; the non-HKO theorem
remains fixed-ten-facet. Neither proves uniqueness of ten-facet maxima, a global
optimum, or local maximality for unrestricted facet count.
The non-HKO example has ratio one and supplies no new violation of Viterbo's
inequality. No explicit numerical Hausdorff neighborhood radius is claimed.
The HKO finite certificate is not a complete catalogue of nearby capacity
branches. Sampling remains supporting evidence even where it studies
variations outside the theorem's facet-count domain.

Use each packet README for exact commands and write effects. Full Rust modes
can replace or append retained data; the exact theorem verifier rewrites its
summary. The HKO numerical diagnostic used to propose the witness is ignored
local data and may be absent from a clean checkout. The tracked witness is
sufficient input for the exact verifier. The non-HKO verifier should write to
a fresh temporary directory and both Python implementations require assertions
enabled. No producer or research run was performed during this relocation.

Retained witness provenance, frozen launch packet and bundle files continue
to record their original `experiments/hko-local-maximum/` paths and source
hashes. They are historical identities, not current output defaults. Preserve
the frozen panel's recorded commit for literal reproduction; a run from the
migrated sources will report source drift and is a new run. Its original
launch packet is not silently rebound to the new tree. The current package
path is `topics/variation-and-local-maxima/hko-local-maximum/`; executable
paths and local depth assumptions are updated there.

The migration establishes file routing and package compatibility, not a new
mathematical audit or a renewed acceptance of the integrated thesis PDF.
Current certificate checks, analytic arguments, matching retained reports,
and release-facing prose remain distinct review obligations.

[Dated eleven-facet extension hypotheses](eleven-facet-extension-hypotheses.md)
retain conditional cuts-additivity and representation/uniformity clues from an
unlaunched October proposal. That proposal is historical: the accepted F≤11
packet closes the extension through its own proof and evidence, without
asserting that each earlier suggested route was verified.

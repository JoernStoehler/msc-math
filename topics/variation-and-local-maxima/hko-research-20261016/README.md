# HKO specialness: research on 16 October 2026

This packet collects research on the assigned branch
`claude/hko-research-2026-10-16`. It changes no thesis source and selects no
new manuscript claims. The [source and question map](source-map.md) inventories
the relevant prior results, original paper questions, unresolved quantifiers and
failed shortcuts; it distinguishes the accepted F≤11 research frontier from
the selected manuscript's ten-facet statement.

The run produced three complementary results:

| Question | Outcome | Scope and evidence |
| --- | --- | --- |
| What asymmetry is necessary to beat HKO with a regular pentagon factor? | For any convex planar partner K, `sys(K×Q)≤κ(1+s(K))/2`, with `κ=(10+6sqrt(5))/25`. Thus `s(K)≤sqrt(5)−1` implies the sharp HKO bound; equality is exactly the HKO-angle regular pentagon. | [Full proof and rigidity](../../products/pentagon-arbitrary-partner/research-20261016/asymmetry.md), including a necessary `s(K)>(15sqrt(5)−29)/4` for any violation, plus 20 finite floating-point capacity/asymmetry controls. No bound for every more-asymmetric partner. |
| Can arbitrarily many small cuts away from the harder polar strata improve HKO? | No, under compact vertex/edge exposure separation: a deepest-cut quadratic capacity drop dominates a count-independent cubic union-volume loss, including moving old facets. | [Multicut theorem](../hko-local-maximum/multicut-research-20261016/PROOF.md), conditional on the stated accepted F10 and vertex-insertion inputs; 12 numerical multiple-cut cases and exact geometry checks. Does not settle joint splitting near polar edges/vertices. |
| Must a counterexample retain a product splitting or special dependencies among normals? | No: a concrete rational ten-facet body has every four facet normals independent and admits no nontrivial affine Cartesian product decomposition. An inclusion proof gives ratio `>1.0463761596`; production capacity plus exact volume gives `[1.047191884343,1.047191884346]`. | [Construction and exact certificate](generic-counterexample/README.md); all 210 four-row subsets checked, 25 simple vertices, ten genuine facets. Quantitative constructive corollary of HKO, not a new record ratio or a claim that openness/smoothing is new. |

All three arguments received separate native-subagent mathematical review;
their local review records distinguish accepted inputs, newly derived analytic
steps, exact finite predicates and numerical controls. No human proof acceptance
or external novelty determination is asserted.

The main remaining extremality problems are global arbitrary asymmetric
partners above the asymmetry threshold; arbitrary non-affinely-regular pentagon
pairs; unrestricted facet-birth local maximality; and complete equality for the
accepted eleven-facet theorem. The multicut result narrows a potential improving
sequence with ten convergent core rows to extra normals approaching the polar
one-skeleton. It does not establish that every Hausdorff-near body admits such a
core representation. The [source map](source-map.md) records these quantifiers
and the larger capacity/dynamics questions from HKO's paper.

Verification commands and write effects are supplied in each packet. All new
computation began in `/tmp/msc-math/hko-20261016/`; the small source-bound outputs
were deliberately retained with their scientific owner. This run makes no
external publication, unrestricted uniqueness or sharp global constant claim.

[Verification record](verification.md) collects executed checks and integration review.

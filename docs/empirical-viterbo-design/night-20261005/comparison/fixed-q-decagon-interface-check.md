# Fixed-Q criterion and decagon separating interface check

Owner: `scientific_compare_initial`; receiving owner root. Checked
2026-10-04 23:14 UTC. This is a bounded integrating check of the independently
reviewed theorem and an exact example interface, not a second full proof audit
or a producer run. External novelty is unassessed.

## Criterion boundary

The [independent proof](../geometry-discovery/fixed-q-rank/independent-proof-check.md)
establishes an iff theorem in an **open fixed-normal partner support chamber**:
the normalized area gradient lies in the interior of the complete active
normalized fitting-dual hull, measured in the full affine closure hyperplane
of dimension `N−3`, exactly when the partner is a local ratio maximum in that
chart. A hull's own relative interior would be insufficient when its affine
span is deficient. Sufficiency gives strictness modulo translations and positive
dilation. Boundary separation gives a one-sided capacity-constant direction
with zero first area derivative and strictly negative second derivative.

This is stronger than the previously accepted rank necessity and first-order
convex balance. It does not address changing normals, adding edges, arbitrary
partner perturbations or global HKO optimality. For triangles the shape quotient
is zero dimensional, so strictness there is vacuous.

## Actual decagon separates balance/coverage from maximality

Inspected [exact verifier](../geometry-discovery/fixed-q-rank/verify_decagon_separator.py)
and [retained certificate](../geometry-discovery/fixed-q-rank/decagon-separator-certificate.json).
The verifier uses standard-library rational field arithmetic for
`a=cos(pi/10)`, isolated by exact signs of `16a^4−20a²+5`; all accepted signs
come from strict rational interval enclosures or an exact zero coefficient
vector. Its area and capacity comparisons do not rely on displayed decimals.

I ran an unchanged scratch copy under
`/tmp/msc-math/scientific-compare-decagon-review/`; its own scratch certificate
was written there, leaving the source owner's certificate unchanged. The run
passed in approximately 0.21 s. It verifies all ten triangle translations and
feasible dual witnesses with matching objective1, genuine decagon facets,
the exact area-decrease identity, five opposite-pair duals and positive apex
slacks. This is bounded replay of a small new example, not F11 or global search.

Write `R=1/[2(1+cos(pi/5))]`, `r=R cos(pi/10)`. Normals are at
`pi/10+i*pi/5`; the perturbation changes supports by
`r ε(e0−e5)`, with `ε=1/100` in the replay. Capacity remains exactly1 since
**each** template has a feasible translated copy and a feasible dual objective1.
Area is

`A(ε)=A0−2 cot(pi/5) r² ε²`,

with a strictly positive decrease. In physical support displacement `t=r ε`,
this is `A(t)=A0−2 cot(pi/5)t²`. Confusing `t` and normalized `ε` would lose
the factor `r²`. Ratio therefore increases, from approximately0.9366563 to
0.9367357 at the verified endpoint; it remains below the HKO benchmark.

For the complete optimal-dual interface, each template's primal placement has
only four tight support labels, comprising two opposite pairs; its apex is
strictly inside. Complementary slackness forces every optimal dual onto those
four labels. Closure in their two independent directions forces equal weights
on each opposite pair, so its optimal face is the segment between the two
displayed pair vertices. Across all ten templates the complete union is exactly
five independent opposite-pair vectors, rank5 rather than the required8.
Their equal mixture matches the positive area target and covers all ten facets.
Thus this is an actual geometry passing coverage and first-order balance while
failing the new necessary rank condition, not an abstract active-graph model.

The displayed uniform `|ε|≤1/100` claim follows from checking the positive
endpoint, reflection exchanging the negative endpoint with the same full ten-
template list, and linearity of polygon/placement slacks in ε. The single run
is an endpoint check; that reflection/linearity implication remains mathematical
reasoning. The exact formula gives arbitrarily small improving deformations.

## Stronger next consequence to review

The geometry lead now proposes a global **prescribed-fan** finite vertex
reduction: capacity≥1 and nonnegative edge lengths define a polyhedron modulo
translations; a contained nondegenerate template supplies area-sublevel
coercivity, and mixed-area concavity moves a minimum to a vertex, including
facet-loss boundaries. This is not accepted by the present example check.
Its next proof must handle recession directions, closed type-cone boundary,
translation quotient and lower-edge polygon capacities before exact finite
classification is claimed. If accepted, it provides a stronger positive
research route than collecting further balanced rank-defect examples.

A proposed regular-five-normal fan must first be checked against the supplied
sharp prescribed-ten-direction-fan theorem assumed in the earlier local-cover
rigidity note. A class already covered there is a control, not a new global
classification. Other orientations or perturbed fans need exact domain checks.

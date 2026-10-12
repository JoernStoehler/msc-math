# Orientation-wide preflight: verified pruning and measured active complexity

Owner: `direct_cover_lead`; receiving comparator: `scientific_compare_initial`.
Root-selected preflight began 2026-10-05 00:04 UTC, cap 00:20.
Exclusive output is this directory; all completed earlier classes are preserved.
No full joint arrangement, CAD, body enumeration or bivariate positivity run
was executed. This is a complete bounded preflight, not a global theorem.

## Actual return and recommendation

The proposed shortcut survives primary-source and independent review:
**every** canonical three-/four-edge partner has ratio≤1, so it cannot attain
or exceed HKO's threshold >1. The only benchmark-relevant vertices are the
**ten nonsingular triples of the five complete fit rows**. Equality capture
requires the eventual global upper bound first; it is not asserted prematurely.

The complete-family covector normalization and angle symmetries are checked.
Exact dominance screening reduces 121 bilinear candidates per circuit to
**5,6,8,7,6**, including certificates for removed candidates and retained raw
boundary ties. Circuit 0's whole orientation strip is extracted: **no interior
orientation events**, five x branches in its open strip, four rational-in-u
break curves. Independent screening/event review passes.

Recommend root select a bounded **complete joint-denominator-cell stage** next,
with no global positivity work before its actual counts and coverage review.
[next-stage-proposal.md](next-stage-proposal.md) gives a 20-minute cap and
explicit cross-circuit crossing obligations; root has not selected it here.

## Source-backed pruning and full implication chain

The primary input is [Balitskiy–Mitrofanov–Polyanskii, arXiv:2603.12495v1,
Theorem 1.2](https://arxiv.org/html/2603.12495v1#S1), published 12 March 2026.
It proves `area(K)area(Q)≥c(K×Q)²/2` for any compact planar convex body K
with interior and any quadrilateral Q. The source explicitly gives capacity
factor-exchange symmetry; this also follows from antisymplectic factor swap
and reversing time in the characteristic-action characterization. Hence a
quadrilateral partner against fixed regular Q has sys≤1.

For a triangle T, add a point just outside the relative interior of one side.
The hull is a genuine outer quadrilateral T_ε with area tending to area(T).
Capacity monotonicity gives `c(T×Q)≤c(T_ε×Q)`. Applying the quadrilateral
bound and passing only its areas to the limit proves sys(T×Q)≤1. No assertion
about capacity continuity is required.

Canonical supports of the five-normal fan have edge numerators
`e_i=h_(i−1)+h_(i+1)−2cos(2π/5)h_i≥0`. The cone has only a point as a
zero-area object, since its five edge directions have no opposite pair.
Capacity≥1 excludes that point. Thus any zero-edge body has three or four
genuine sides. Invertible normal transforms preserve this side count, so
**every** canonical zero-edge stratum, including parameter endpoints where
edges disappear, has sys≤1. In scaled support notation this gives
`P≥x d(u)/5`, strictly stronger than the HKO target
`P≥x d(u)/(3+√5)`.

The accepted fixed-parameter global-area theorem says every canonical minimum
modulo translation is a vertex and has capacity exactly one. A hypothetical
ratio≥HKO can therefore be tested by minimizing area at its fixed parameters;
a benchmark-relevant minimizing vertex cannot have zero edges. Its three
independent active quotient rows are solely fit rows. All ten triples among
five fit rows are nonsingular, verified independently by
[verify_pruning.py](verify_pruning.py). Extra active fit equalities can give
several representations of one body and must be deduplicated. Once a global
HKO bound is established, every equality body is a minimum and this same
argument captures all its classes; before then it captures a possible violation
through optimization, not all arbitrary bodies lying on a threshold.

[pruning-review.md](pruning-review.md) retains exact source/version/domain,
exchange/triangle arguments and the necessary distinction about equality.

## Complete-family normalization, symmetry and field

Set `z=√5`, `τ²=5−2z`, `u=tanθ/τ`. Computational points are
`(q_x,q_y/τ)` and covectors `N=(n_x,τ n_y)`. Correct scaled covector rotation is

```
C_u=[[1,−u],[τ²u,1]], d(u)=det C_u=1+τ²u².
```

Physical normals `G_xR_θn_i` have common positive cosθ times `G_x C_u N_i`;
arbitrary supports absorb it. For `M'=G_x R_θ/cosθ`, `Y=M'^T K` has the
regular normal fan. With
`P=Σh_i h_(i+1)−((z−1)/4)Σh_i²`,

```
area(K)=P/[x d(u) sin(2π/5)],
sys=x d(u)/(5P) at capacity one.
```

The complete ten templates remain fixed; their denominators are the maxima
of the original triangle supports under these covectors. The same five closure
rays are complete because the transform is invertible. Their weighted sums
are positive and translation-invariant. Thus capacity is exactly
`min_l (λ_l·h)/D_l(x,u)`, with the **maximum** denominator over all ten
triangles for each ray. The closed canonical support/vertex proof transfers
without assuming capacity is preserved by changing one factor alone.

Period π/5 follows from `R_(π/5)n_i=−n_(i+3)` and template permutation
`T_(j+5)=−T_j`, with corresponding partner reflection and support relabeling.
Horizontal reflection commutes with G_x, preserves Q, and permutes the
normal/template sets, identifying θ and −θ. Consequently the full optimized
family reduces to `0≤θ≤π/10`, or `0≤u≤1/z`. This does not leave a fixed raw
support vector unchanged. The independent review checks both transformations.

All coefficients remain in Q(√5). Individual quadratic event roots may have
degree up to four over Q; later quartic positivity roots may have degree up to
eight. These are individual-root bounds, not bounds on an entire compositum.
Exact embedding/order and zero tests remain obligations if such roots occur.

## Measured global screening and one-circuit active events

[active_events.py](active_events.py) constructs the original 1,350 raw
vertex-choice polynomials using copied exact source coordinates, not a rerun
of an older producer. Their 605 circuit-labeled distinct tuples have 121 per
circuit. Write a candidate as `m(u)x+b(u)`, with affine m,b.

If another candidate has both slope and intercept differences nonnegative
at u=0 and u=1/z, those differences are nonnegative throughout the strip.
It therefore dominates for every x>0. Distinct polynomial tuples give strict
domination throughout the open u strip, so dominance is acyclic and every
removed candidate chain reaches a survivor. All supplied witnesses and their
boundary ties are retained in [active-events.json](active-events.json).

The exact survivor counts are 5,6,8,7,6. A separate read-only audit independently
reconstructs all 605 tuples, checks all 17,514 supplied dominance witnesses,
verifies acyclic chains and verifies survivors are undominated.

For **circuit 0 only**, five survivors give ten pairs and ten triples. Its
complete kinetic test uses 20 linear slope/intercept polynomials and ten
quadratic triple-concurrency polynomials. The only in-strip root orientations
are u=0 and u=1/z; no event polynomial is identically zero. Nonsquare
quadratic discriminants occur only for roots outside the strip, excluded by
exact endpoint/vertex range signs. No extended in-strip field engine was
exercised; its future cost is unknown.

Throughout `0<u<1/z` the active x-line order is `[60,107,36,14,12]`, giving
five x cells and four exact rational-linear-over-linear break curves, all
retained in the JSON. Full raw-candidate ties at both u endpoints and at x
breaks are retained, including x=0 and limiting slope ties. This is complete
circuit-0 coverage, **not** a complete five-circuit/common capacity arrangement.

The no-missed-change implication is checked independently: between all event
roots, slope order and all nonzero triple orientations are fixed, so the
upper hull is fixed. Intercept events capture x=0 clipping; slope events
capture parallel/infinity changes; persistent collinearity or identical
polynomials would need tie treatment. A midpoint hull represents the whole
open patch only because these complete event predicates were checked.
Events of even multiplicity and isolated boundary winners cannot be dropped
just because open hull orders agree. The exact event root argument and all
raw midpoint/endpoint hulls/ties pass in
[screening-event-review.md](screening-event-review.md) and
[review_screen.py](review_screen.py).

## Next proof plan, unresolved work and resource boundary

[positivity-plan.md](positivity-plan.md) derives, rather than assumes, the
bidegree≤(2,2) gap for the ten fit triples. Bilinear feasible inequalities give
rational-linear x bounds after coefficient signs are partitioned in u.
Endpoint substitutions and stationary-minimum numerators have degree≤4;
stationary membership additionally uses degree≤3 sign predicates. Zero
quadratic coefficients, singleton fibers, shared boundaries, x tails and both
angle endpoints are explicit cases. A one-variable ordered-field Sturm route
is plausible, but has **not** been executed or priced.

The remaining individual circuit events, complete joint-cell ordering of
break curves from different circuits, actual joint cells and positivity-root
costs are unknown. Different circuits' x break curves can cross without any
individual hull change. The next-stage proposal therefore includes those
crossings before asking for a substantive positivity allocation. Favorable
circuit-0 topology does not prove there are no such crossings elsewhere.

Current measured script time is about 10 seconds, dominated by screening;
the retained final JSON records its observed runtime. This is not a theorem-
runtime forecast. Proposed next cap is 20 minutes for remaining individual
and **complete joint** denominator extraction/counts/review only, with a ten-
minute actual-cost return and a hard stop if the root engine or shared-boundary
coverage fails. No full positivity is included. Root must select any renewal;
the hard 02:59 UTC stop remains unchanged.

## Reproduce and write effects

```
python -B docs/empirical-viterbo-design/night-20261005/orientation-wide/preflight/verify_pruning.py
python -B docs/empirical-viterbo-design/night-20261005/orientation-wide/preflight/active_events.py
python -B docs/empirical-viterbo-design/night-20261005/orientation-wide/preflight/review_screen.py
```

All are exact, standard-library scripts. The producer overwrites only its
sibling active-events.json; the review scripts are read-only. Raw field
coefficients, source labels, dominance witnesses, event polynomials, roots and
parametric breaks are retained. Existing completed producers were neither
rerun nor overwritten. No paid API, broad population, manuscript/dashboard/
registry edit or unrestricted global result was introduced.

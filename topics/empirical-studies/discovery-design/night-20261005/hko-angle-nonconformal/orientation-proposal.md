# Orientation-wide family: active-envelope and ten-triple proposal

Prepared 2026-10-05 00:02 UTC for `scientific_compare_initial` and root.
**Unselected proposal only.** No orientation-wide extraction, body enumeration
or global positivity run has been executed. The HKO-angle theorem is complete.

## Target, symmetries and exact predicate

Keep Q fixed. Enlarge the partner fan to `G_x R_θ n_i`, `x>0`, arbitrary
supports. This is still a prescribed affine-normal-fan family, not all pentagons.
With `z=√5`, `τ²=5−2z`, use `u=tanθ/τ` and scaled computational covectors

```
G_x C_u N_i, C_u=[[1,−u],[τ²u,1]], d(u)=1+τ²u².
```

Original normals have common factor cosθ; scaled supports absorb it. Capacity
uses the reduced complete denominators. Area and ratio retain d(u), so at
capacity one `sys=x d(u)/(5P)`.

The orientation family has period π/5: `R_(π/5)n_i=−n_(i+3)`; central reflection
of the partner preserves complete fitting because `T_(j+5)=−T_j`. Horizontal
reflection commutes with G_x, preserves Q and identifies θ with −θ. Thus the
whole family reduces to `0≤θ≤π/10`, or `0≤u≤1/√5`. This reduces optimized
families with the appropriate support relabeling/reflection, not a fixed raw
support vector unchanged.

The benchmark predicate is exactly

```
P(H(x,u)) − x(1+τ²u²)/(3+z) ≥ 0,
x>0, 0≤u≤1/z, complete capacity≥1, canonical edges≥0.
```

On each complete denominator branch, candidate H is bilinear and this gap has
bidegree at most (2,2). The desired equality locus must be derived; the known
control is regular HKO at `(x,u)=(1,1/z)`, modulo family symmetries, translations
and positive dilation. No absence of interior equality is assumed.

All input coefficients are in Q(√5). New event boundaries need not lie in that
field: quadratic event roots can have degree up to four over Q. Later positivity
roots of quartics over Q(√5) can have degree up to eight. Retain exact root
polynomials and rational isolating intervals rather than silently rounding or
claiming all parameter endpoints remain quadratic-field elements.

## Strong geometric pruning: only ten all-fit triples matter

The primary input is [Balitskiy–Mitrofanov–Polyanskii, Theorem 1.2](https://arxiv.org/html/2603.12495v1#S1),
checked directly on 2026-10-05: for any planar convex K and any quadrilateral Q,
`area(K)area(Q)≥c(K×_L Q)²/2`. Factor-exchange symmetry of capacity, already
owned by the fixed-Q evaluator contract, lets the quadrilateral be the partner.
Thus any quadrilateral partner has sys≤1.

Triangles follow without an unproved uniformity assertion. Given a triangle T,
add a point at distance ε just outside the relative interior of one side;
its hull with T is a genuine outer quadrilateral T_ε, with `area(T_ε)→area(T)`.
Monotonicity gives `c(T×Q)≤c(T_ε×Q)`. Apply the quadrilateral theorem and pass
only the areas to the limit, obtaining sys(T×Q)≤1. Capacity continuity is not
needed for this argument.

Every canonical capacity≥1 body with a zero edge in the five-normal fan has
three or four genuine edges, and stays so under the invertible normal transform.
Its area therefore satisfies `P≥x d(u)/5`, **strictly stronger** than the HKO
target because `3+√5>5`. No such boundary vertex can violate or attain HKO.
This includes all shared parameter endpoints at which a previously genuine
five-edge body loses an edge; those endpoints must be assigned to this boundary
argument, not dropped as numerical degeneracies.

Use the accepted canonical global-vertex theorem at each fixed `(x,u)`.
A genuine five-edge vertex has no active edge inequality. Its three independent
active rows must therefore be among the five fit rows. Exactly the ten triples
`C(5,3)` remain, and **all ten are nonsingular** in the already executed constant
matrix calculation. This replaces 100 nonsingular candidate systems per complete
parameter cell with ten for the HKO question. A feasible all-fit triple has
capacity exactly one: its active fit equality gives c≤1 and all complete fit
inequalities give c≥1.

This is a pruning theorem for the benchmark question, not a full optimizer list
below one. Once a global HKO bound is proved, any body attaining it is a fixed-
parameter area minimum after normalization. The stronger every-minimizer-is-a-
vertex theorem then makes it one of the ten genuine-five-edge triples. Thus
pruning preserves all possible equality bodies, including extra-fit degeneracies
and duplicate representations by several triples. Nonvertex or redundant raw-
offset fibers do not create additional body equality classes.

## Actual existing counts and unknown costs

[The exact read-only preflight](../nonconformal-fan/angle-preflight.md) measured:

- 1,350 raw bilinear candidates; 121 distinct per circuit;
- 605 with circuit labels, 371 coefficient tuples across circuits;
- 7,260 pairs per circuit; 1,365 normalized equality polynomials per circuit;
- 4,420 distinct normalized pair-equality polynomials in their union;
- 4.72 seconds to count these exact input tuples.

The actual active envelope events, common cells, feasibility regions and root-
isolation costs are **unknown**. The two completed fixed-angle calculations
used five/six common cells and about a second of arithmetic; they do not price
an orientation-wide decomposition. Do not launch a naive arrangement of all
4,420 curves or the product of five 121-way choices.

## Proposed first selected operation: active fiber extraction

For circuit l, regard each reduced candidate as an x-line

```
f_r(x,u)=m_r(u)x+b_r(u),
```

where both slope m and intercept b are affine in u. First screen globally
inferior candidates: if another candidate dominates both slope and intercept
at the two u endpoints, it dominates for every x>0 throughout the strip. Keep
all equality cases rather than deleting coincident boundary-only winners.

For each fixed u, use the exact one-dimensional upper-hull algorithm already
certified in the two slices. Hull combinatorics can change only at:

1. equality of two slopes or x=0 intercepts (linear u events);
2. concurrency of three x-lines (quadratic u events);
3. the corresponding x=0 or infinity boundary event.

Extract events adaptively: for each current active edge, test inactive candidates
for the next possible triple-concurrency event instead of precomputing all
121-choose-3 triples. Certify a candidate stays inactive up to the next event,
and retain simultaneous events, identically equal polynomials and endpoint ties.
At u=0 tied events require a right-hand algebraic/lexicographic initialization;
using a floating ε is not a coverage certificate.

The first output should be a complete tested extraction on one circuit, with
actual surviving candidate/event counts, exact event polynomials/isolating costs,
and a proof that discarded candidates cannot enter unnoticed. Then report how
five circuits' active break curves would merge into common `(x,u)` cells.
No substantial global positivity run belongs before that actual-count review.

This stage also independently checks the ten-triple pruning above and compares
it with enumerating the full 100 systems. Its useful adverse outcome is an
identified missing event, expensive active branching, or invalid proposed
completeness argument, retained precisely enough to reform the route.

## Subsequent positivity route if root later selects it

On a certified common denominator cell, each of the ten candidates is bilinear.
Every capacity/edge/cell inequality is bilinear and hence linear in x at fixed u.
After partitioning signs of their affine x-coefficients, feasible x fibers have
bounds rational-linear-over-linear in u. Crossings of such bounds are at most
quadratic events. Preserve singleton fibers and all shared boundaries.

The HKO gap is quadratic in x with u-polynomial coefficients of degree≤2.
Its minimum over an x interval is controlled by endpoints, an interior stationary
minimum, and the unbounded leading-coefficient case. Substituting a rational-
linear endpoint and clearing a **known-sign squared denominator** gives a
u-polynomial of degree≤4. The stationary numerator `4q_0q_2−q_1²` also has
degree≤4; membership of its stationary x in the feasible interval needs its
own sign predicates. Exact Sturm isolation/sign checks can therefore reduce
positivity and equality on each certified cell to one-variable polynomials,
without a general two-variable CAD of all raw curves. Handle q_2=0 separately.

A failed exact gap on a feasible genuine-five-edge branch would nominate an
above-HKO body. Reconstruct all ten fits and an active dual, direct physical
shoelace/area product and actual side count before calling it a counterexample.
A passed cell gives a named restricted result; isolated slices or scalar trends
are not global coverage. A global pass must close every event boundary, every
x tail and every equality component.

## Alternatives, receiving point and proposed resource boundary

Preferred first step combines geometric ten-triple pruning with the active
fiber sweep. A full raw-curve CAD is the serious computational alternative,
but its 4,420 input-curve count gives no acceptable runtime estimate. Another
pure geometric alternative would need a sharp inequality for the five unequal
circuit requirements. The common-denominator inner-regular-core identity is
already lost under distortion, so replaying it does not supply that estimate.
No such direct replacement inequality is presently available.

Propose **one 20-minute preliminary allocation**, with a lead and at most two
exclusive mathematical/control/review children, using existing subscription
scope and no paid API. At its cap return the actual active counts, demonstrated
coverage argument, field-isolation timings, pruning review and concrete next
proof cost. Stop there unless root separately selects the substantive positivity
stage. No total theorem-runtime estimate is defensible before that count packet.
Any renewed cap must remain inside the hard 02:59 UTC stop. This proposal
requests a route selection; it is not authorization to perform the sweep now.

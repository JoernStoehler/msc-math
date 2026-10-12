# Exact ten-fit positivity route after active-envelope extraction

Lead derivation, 2026-10-05 UTC. This is an algebraic proof plan, **not an
executed orientation-wide positivity calculation**. The benchmark family and
pruning are reviewed separately in this preflight.

Let `F=Q(sqrt(5))`, `t=5−2sqrt(5)`, `0≤u≤1/sqrt(5)`, `x>0`.
Reduced covectors use `C_u=[[1,−u],[t u,1]]`; `d(u)=1+t u²`.
The complete capacity constraints are `λ_l·h≥D_l(x,u)`. Canonical zero-edge
bodies have ratio≤1 and cannot meet HKO, so the only necessary vertex systems
for the HKO predicate are the ten triples of the five fit rows. Their matrices
are constant and nonsingular in the translation gauge `h_3=h_4=0`.

## Coefficients and exact degree bounds

On a certified common complete-denominator cell each D_l is bilinear, so
solving a fit triple gives

```
h(x,u)=p(u)+x q(u),
```

where p and q are five-vectors affine in u. For `a=(sqrt(5)−1)/4`, define

```
P(h)=Σ_i h_i h_(i+1)−a Σ_i h_i²,
B(p,q)=Σ_i [p_i q_(i+1)+q_i p_(i+1)]−2a Σ_i p_i q_i.
```

Then `P(p+xq)=P(p)+x B(p,q)+x²P(q)`. The benchmark gap is

```
g(x,u)=g_0(u)+g_1(u)x+g_2(u)x²,
g_0=P(p), g_1=B(p,q)−(1+t u²)/(3+sqrt(5)), g_2=P(q).
```

Each g_i has degree at most two in u, with coefficients in F. Thus bidegree
(2,2) is proved directly; it is not inferred from samples or from the one-angle
certificate.

All remaining fit and canonical edge inequalities are bilinear. The chosen
complete-denominator branch conditions are bilinear too. At fixed u, each is
`a_k(u)x+b_k(u)≥0`, with affine a_k,b_k. Partition u where any a_k vanishes;
positive a_k gives lower x bound `−b_k/a_k`, negative a_k gives upper bound,
and zero a_k needs the separate condition b_k≥0. Include x>0. Bounds crossing
one another have numerator `a_i b_j−a_j b_i` of degree at most two. A fixed
branch's feasible x fiber is an interval, singleton or empty; no disconnected
fiber is silently merged. Shared parameter endpoints and u boundary ties need
all adjacent formulas, whose envelope values must agree exactly.

## Endpoint and stationary predicates

For a finite fiber endpoint `x=−b(u)/a(u)` with a≠0,

```
a(u)² g(−b/a,u) = a² g_0 −a b g_1+b² g_2.
```

This polynomial has degree at most four in u. The denominator a² is strictly
positive, irrespective of the sign of a. At a=0, return to the un-divided
feasibility inequality; do not use its rational formula or drop a boundary.

For g_2>0, an interior stationary minimum has

```
x_* = −g_1/(2g_2),
g(x_*,u) = (4g_0g_2−g_1²)/(4g_2).
```

The numerator again has degree at most four. Whether x_* lies in the fiber is
an additional sign predicate. For a bound `L=−b/a`,

```
x_*−L = (−g_1 a+2g_2 b)/(2g_2 a),
```

whose numerator has degree at most three. The signs of a and g_2 must already
be known on the relevant u interval. A stationary value outside the fiber is
irrelevant; a feasible negative one is a candidate witness.

For finite fibers with g_2≤0, endpoint minima suffice. For an unbounded x tail,
a negative leading coefficient fails positivity, as does g_2=0,g_1<0.
Otherwise test the finite endpoint and any feasible stationary minimum. Treat
g_2=0 separately as a linear/constant polynomial; no division by zero occurs.
The x=0 extension can certify one-sided nonnegativity, but x=0 itself remains
outside the body domain and cannot supply an equality body.

## Exact root ordering, positivity and equality

A complete proof can isolate roots of the affine/quadratic feasibility and
branch polynomials, g_2, stationary-membership cubics, endpoint quartics and
stationary quartics, then determine signs on each resulting u interval and
at every root. Sturm sequences over the ordered field F give a finite route.
Parameter endpoints from quadratic kinetic events need not belong to F;
quartic positivity roots can have degree up to eight over Q. Retain minimal
or norm polynomials, rational isolating intervals and the correct positive-
sqrt(5) embedding. Norm-polynomial conjugate roots must be filtered; an
interval isolating a root of a conjugate polynomial alone is insufficient.

A root shared by several predicates needs exact gcd/resultant or algebraic
zero checks; interval refinement without a zero test can fail to terminate.
The eventual certificate must list which branch/fiber contains each root and
which old support/edge incidences remain valid. This is essential for isolated
parameter equality points and simultaneous branch changes.

A global HKO pass must cover every certified denominator cell, all ten fit
triples, every singleton, x tail, both u boundaries and all equality components.
After the bound is established, the accepted every-minimizer-is-a-vertex theorem
preserves equality completeness. Positive five edges and all fit constraints
make a feasible fit-triple body capacity exactly one. A negative feasible gap
must be converted into explicit inputs, all-ten primal fits, active dual,
physical shoelace/area product and actual five sides before calling it an
above-HKO example. No such gap has been sought or found in this preflight.

## Independent review and resource uncertainty

The eventual reviewer should independently check: complete D envelope coverage
and all endpoint ties; field embeddings/event ordering; fit-triple coordinates
and full feasibility intervals; direct physical d(u) normalization; every
endpoint/stationary/tail predicate; all equality zeros; exact witness controls
if any. One consequential review may cover this assembled implication chain,
but a denominator count alone is not that review.

The actual active common cells and polynomial-root workload are not yet known.
The bounded one-circuit preflight can measure those inputs; it cannot price the
full proof merely by extrapolating a one-second fixed-angle calculation. Root
must select a subsequent execution stage and cap after the measured return.

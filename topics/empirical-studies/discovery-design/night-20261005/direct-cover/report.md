# Direct-cover hedge: a realized singly-contact stratum and escape

Owner: `direct_cover_lead`; receiving/comparative owner: `scientific_compare_initial`.
2026-10-04, first bounded research window; returned early after the discriminator.
Scope: fixed regular Q, ten independently translated normal triangles. No global
fixed-Q area bound, manuscript result or exhaustive contact classification is claimed.

The selected codimension-one contact **is realizable**, and a one-block tangential
slide extends the open-cell escape argument to this boundary. At a local area
minimum the slide has constant area and reaches an additional physical supporting
incidence in finite parameter distance. This remains true when the contacting
triangle supplies other genuine hull vertices. It avoids eliminating the coupled
contact equation and does not use a restricted zero-diagonal Hessian assertion.

The explicit realized point is nonstationary and has area above HKO throughout
its checked slide interval. It is a useful exact negative control, not a competitor.
A direct lower bound on the entire 18-dimensional stratum has not been proved.

## Exact realization and domain

Use precisely the ordered-field triangle labels and vertices `p[j,k]` from
[the fixture](../../../../products/pentagon-arbitrary-partner/local-cover/pentagon_fixture.py),
with field `Q(tau)`, `tau^4-10tau^2+5=0`, `7/10<tau<3/4`.
Write `v[j,k]=p[j,k]+t[j]`. Coordinates and areas are in that fixture's Euclidean
coordinate convention. `m=0.20081141588622728...` is the exact capacity-one HKO
partner area reconstructed from the same fixture.

The contact uses three distinct templates:

- `A=(0,0)`: triangle 0, vertex 0;
- `B=(5,0)`: triangle 5, vertex 0;
- `C=(9,0)`: triangle 9, vertex 0.

Before removing common translation, set `t[j]=r[j]/50` for `j!=0`, where

```
r[1]=(10,1), r[2]=(-3,10), r[3]=(0,0), r[4]=(6,1),
r[5]=(-5,0), r[6]=(1,4), r[7]=(4,-10), r[8]=(1,4), r[9]=(-10,1).
```

Set `t[0]=(v[B]+v[C])/2-p[0,0]`, then subtract `t[5]` from every translation.
Thus `t[5]=0`; the 18 remaining coordinates `x[j],y[j]`, `j!=5`, are free
variables, subject to the following domain. The exact certified hull cycle is

```
(5,0), (9,0), (7,0), (7,2), (4,1), (1,1),
(2,1), (8,1), (0,2), (9,1).
```

For its successive labels `h[i],h[i+1]`, define

```
D[i,j,k] = det(v[h[i+1]]-v[h[i]], v[j,k]-v[h[i]]).
g = det(v[C]-v[B], v[A]-v[B]).
```

The domain is `g=0`, `v[B]!=v[C]`,

```
0 < dot(v[A]-v[B],v[C]-v[B]) < dot(v[C]-v[B],v[C]-v[B]),
D[i,j,k] > 0
```

for all 279 label triples except the two endpoint labels of each edge and the
one prescribed incidence `(B,C,A)`. Endpoint determinants and `g` are zero.
Every inequality is given explicitly by its labels in
[stratum-certificate.json](stratum-certificate.json). Strict support inequalities
force the stated cycle to be the strictly convex hull and all other points to
be strictly inside its supporting halfplanes. The contact vertex is the midpoint
of `BC`. The gradient of `g` in `t[0]` is a nonzero quarter-turn of `v[C]-v[B]`,
so this is genuinely a smooth codimension-one stratum near the realization.

The hull has **10 actual sides**, not five. Triangle A also contributes hull
vertex `(0,2)`; the slide changes the hull rather than merely repositioning an
invisible interior triangle. This example therefore does not fall in the 5+5
product class. Inclusion of all ten translated templates gives capacity at least
one; exact capacity and the example's ratio have not been evaluated.

## Exact area polynomial and tangent

On the whole stated domain the exact polynomial is

```
F(t)=1/2 sum[i=0..9] det(p[h[i]]+t[template(h[i])],
                        p[h[i+1]]+t[template(h[i+1])]), h[10]=h[0].
```

This is a fully specified quadratic polynomial in the 18 variables. Its expanded
sparse coefficient list, and the expanded contact polynomial `g`, are retained
in the certificate. Coefficients are rational arrays in descending powers of
`tau`; they are checked by exact evaluation at the displayed realization.
The feasible tangent space at any domain point satisfies

```
dg(delta t)=det(delta t[C]-delta t[B], v[A]-v[B])
           +det(v[C]-v[B], delta t[A]-delta t[B])=0,
delta t[5]=0.
```

The equation couples A, B and C and is quadratic in all translations. Eliminating
one variable can produce rational coefficients and curvature; no PSD/Hessian
claim after elimination is used here.

Instead freeze all templates except A and put

```
d=v[C]-v[B],  t[0](s)=t[0]+s*d.
```

The contact equation then holds identically and the relative contact parameter
is `1/2+s`. Each `D` and `F` is affine in `s`: the only potentially quadratic
terms contain `det(d,d)=0`. The exact realized slide has

```
F(s) = (-637 tau^3/12000 -13 tau^2/480 +3061 tau/7200 +1871/12000)
     + s*(287 tau^3/18000 +17 tau^2/6000 -79 tau/1200 -119/30000).
```

Numerically `F(s)=0.43014354131017374-0.04418682388289331*s`.
The maximal strict interval for this fixed cycle is
`-0.21889843944572995... < s < 0.10811925762411777...`, with both endpoints
exactly retained and verified. At the lower endpoint `(0,2)` gains supporting
incidence on edge `(2,1)--(8,1)`; at the upper endpoint the supporting determinant
for `(8,1),(0,2),(9,1)` becomes zero. These are physical hull degeneracies,
not imposed search-box boundaries. Endpoint hull areas are respectively
`0.4398159681022024...` and `0.4253660947151877...`, both rigorously greater
than `m`. Because area is affine, the whole checked interval is above `m`.

## Escape theorem and its exact limits

For any configuration in the stated singly-contact domain, the same slide
exists in both signs for sufficiently small `s`, by strict inequalities and
relative-interior contact. Area is affine. Consequently either one direction
strictly decreases area, or the area is constant on its entire feasible interval.
At a local minimum the derivative must vanish, giving the latter alternative.

The interval lies in the finite contact interval `-lambda<s<1-lambda`, where
`v[A]=v[B]+lambda*d`; its endpoint is attained. If the endpoint is reached
because `lambda` reaches 0 or 1, A gains an endpoint coincidence. Otherwise
another support determinant vanishes. In either case there is another physical
supporting incidence or hull degeneracy. The closed endpoint configuration is
still a feasible cover and has the same area in the stationary case.

Thus a singly-contact configuration cannot be an isolated area minimum in
**placement space** modulo common translation. Body isolation is a different
notion: an interior template can move without changing the hull. A distant
constant-area endpoint of a local minimum need not itself remain locally
minimizing. A global minimum does remain globally minimizing at that endpoint.
There is no proof that repeatedly applying this operation terminates, retains
all earlier incidences, or reaches finitely many rigid contact patterns.

[Independent review](review.md) checks this one consequential implication,
including the case where A owns several hull vertices and the distinction
between a new supporting-face incidence and membership in the old finite edge.

## General block-rank necessary condition

For a strict geometric hull cycle, choose one representative label per extreme
point and test all input labels, including coincident duplicates, against every
supporting edge. Let `q_alpha=0` be all active support incidences at the point.
Freeze all translations except `t[j]`; every `q_alpha` is affine in this entire
two-coordinate block, including edges whose endpoints themselves move with j.
If their coefficient rows have rank less than two, choose a nonzero common
kernel vector `d[j]`. All active equalities remain exact on the whole line;
the other strict inequalities persist on a neighborhood. Area is affine along
that line, so local minimality makes it constant.

This constant-area interval is bounded for a physical reason, even without
an interior-edge contact trapping the parameter. Another fixed nondegenerate
template contains a disk of some radius `r>0` centered at `z`. For any point p
of the moving template the hull contains the triangle with apex p and the disk
diameter perpendicular to `p-z`. Therefore `area(hull)>=r*||p-z||`. Constant
area bounds p, hence the moving translation; a finite endpoint introduces a
new physical hull degeneracy or incidence. This is again one slide, not an
iterated classification algorithm.

Consequently active block rank two for **each** template is necessary for an
isolated labeled-placement local minimum modulo common translation: to test a
block, anchor a different template. It is not sufficient and does not exclude
isolated bodies with redundant template placements. The independent review
checks this extension and its duplicate-label convention.

## Scientific comparison and recommendation

The constant-area escape route has a clean bounded success: singly-contact
strata admit a physical escape from any stationary local minimum. The direct
area-inequality route has only the restricted one-dimensional negative control
above; its 18-dimensional estimate remains missing. This result extends the
known open-cell argument but does not establish the fixed-Q global inequality
or remove the need to control more heavily coupled strata.

The useful next crux is a configuration in which every template's active contact
constraints pin both independent translation directions. Rank below two in a
single block leaves a line on which active constraints and area are affine;
rank two can obstruct this operation. This contact rank is a necessary condition
for isolated placement minima, not an area bound or a sufficiency criterion.
No new group or substantial continuation is recommended merely to enumerate
contact labels. Return this early result to the comparator and preserve the
selected support-LP and P* proof allocations. No concrete realizable rank-two coupled stratum is nominated here. No decisive
direct global argument was found, and no extension of this hedge is requested.

At area `m`, covering would only imply ratio at least HKO's benchmark until
exact capacity one or an independent global upper bound is supplied. No equality
family or improved ratio was found. Nothing is promoted to thesis/formal sources.

## Reproduce and retained evidence

```
uv run --with sympy python topics/empirical-studies/discovery-design/night-20261005/direct-cover/verify_stratum.py
```

The checker reconstructs the original fixture, verifies all 279 strict signs
using the ordered real embedding, checks affine dependence by exact second
finite differences, computes the maximal interval from all support inequalities
and endpoint-contact constraints, verifies actual endpoint hull areas, and checks
all three areas against `m`. It overwrites only this directory's certificate.
The selected numerical construction used a bounded seed-501 probe and returned
on its first placement; the lasting checker uses only the exact retained input.

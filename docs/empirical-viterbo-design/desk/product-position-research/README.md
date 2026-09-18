# Product position: first research return

2026-09-18. Owner: `product_position_research`, empirical desk. No broad capacity
producer was launched. The work moved from proposed lifted-orbit inspection to
a general mass-splitting argument and an exact separating family.

The authorized [follow-up on lift persistence](lift-persistence.md) is complete:
translative containment measures orbit existence and a local insertion test,
but fails robustly as a global capacity-minimality criterion. A rational
feasible-action witness and strict geometric containment certify the failure.
The suggested first-loss investigation at the end of this initial report should
now be read together with that completed follow-up, not as an unstarted task.

## Scientific result and status

Two results answer what endpoint product descriptions preserve and lose.

1. **All planar convex factors have an explicit initial capacity regime** when
   their fixed Euclidean product is moved away from symplectic product position.
   A lower bound using both endpoint capacities proves this regime. The argument
   has passed an independent read-only mathematical audit by
   `product_position_research/mass_split_audit`; novelty is not assessed.
2. **The areas and both endpoint capacities do not determine the intermediate
   capacity profile.**
   An exact one-parameter family holds those data fixed while its capacity at
   the intermediate angle pi/4 changes. A complete rational HK enumeration
   certifies the coefficient inequalities for the whole parameter interval.
   This is a separating construction, not a claim that another triangle case
   is independently a significant research endpoint.

These are research derivations/certificates accepted in the
[coordinator's review](../root-review/README.md), not manuscript or formal-theorem
integration. The exact certificate's coverage
rests on the reduction explained below and HK Theorem 1.1. No novelty claim is
made. The same independent auditor checked the exact certificate,
its coverage argument and a read-only rerun, plus three interior rational values.

## Conventions and the general bound

Let A and B be arbitrary planar convex bodies with nonempty interiors, translated
to contain zero. Write a=area(A), b=area(B), and assume a<=b. Keep A x B fixed in
Euclidean R4. For 0<=theta<=pi/2, use the orthogonal complex structure

```
J_theta = [[cos(theta) J2, sin(theta) I],
           [-sin(theta) I, -cos(theta) J2]],
J2 = [[0,1],[-1,0]].
```

Direct block multiplication gives J_theta^2=-I. This path lies in one component
of orthogonal complex structures and is equivalent to rotating the body while
keeping the symplectic structure fixed. Denote its EHZ capacity by c_theta, and
put ell=c_(pi/2). At theta=0 the factor planes are symplectic, with opposite
orientation conventions; c_0=min(a,b). At pi/2 they are Lagrangian. The relative
identification of their coordinates matters to ell and must remain fixed.

For C=cos(theta), S=sin(theta), the derivation gives

```
1/c_theta <= max_{0<=t<=1} [ C(t^2/a + (1-t)^2/b)
                           + (4S/ell)t(1-t) ],
1/c_theta >= C/a.
```

Thus, in particular,

```
c_theta = a/cos(theta) whenever tan(theta) <= ell/(2a).
```

This is a sufficient initial interval; examples below show its endpoint need
not be the actual first transition. Capacity and a,b,ell have units length^2.

**Proof for polygons.** Use support-normalized facet covectors u_i=n_i/h_i.
By [Haim-Kislev, Theorem 1.1](https://arxiv.org/html/1712.03494v3), reciprocal
capacity is the maximum of the signed ordered pairing sum with factor 2, over
nonnegative weights lambda summing to one and satisfying sum lambda_i u_i=0,
and all facet orders. Reversing the global sign convention just reverses orders.

For a Cartesian product, closure holds separately in the two factor planes.
Let t be the total A weight. Normalizing those weights by t makes a feasible
planar HK configuration; its within-A contribution is at most t^2/a. Similarly,
the within-B contribution is at most (1-t)^2/b. The minus sign in the B block
causes no issue because planar ordering can be reversed.

For the mixed contribution, divide the A weights by 2t and the B weights by
2(1-t). These are feasible product weights, now with half the mass in each
factor, for the pure Lagrangian form. Therefore the original mixed contribution
is at most 4t(1-t)/ell. The three upper bounds need not be simultaneously
attainable; that lost compatibility is consequential below. Zero-mass endpoints
follow directly. Maximizing over all HK data proves the first inequality. All
weight in A, using a planar maximizer, proves the second.

Put U=C/a, V=C/b<=U, W=4S/ell. The displayed quadratic minus U factors as

```
(1-t)[V-U + t(W-U-V)].
```

Its bracket is nonpositive on [0,1] when W<=2U, proving equality in the stated
interval. Polygonal approximation and continuity of EHZ capacity extend both
inequalities and the equality to arbitrary convex planar factors.

**Consequence.** Whenever ell<=a, every sufficiently small positive tilt has
capacity strictly greater than *both* endpoint capacities. A universal
endpoint-range principle, or monotone interpolation between the two product
positions, is therefore false on an unrestricted-factor class. These hypotheses
are easy to realize; the rational triangle family below has ell=18/5<9/2=a.
Within that family, c_theta=(9/2)/cos(theta) at least until tan(theta)=2/5.

## What endpoint data miss: exact continuous separating family

Take

```
A = conv{(-1,-1),(2,-1),(-1,2)},
B_s = [[2,s],[0,1/2]] A,          0 <= s <= 1/2.
```

For every s in this interval,

```
area(A)=area(B_s)=9/2,
c_0=9/2,
c_(pi/2)=18/5,
c_(pi/4)=18 sqrt(2)/(7-2s).
```

Thus both endpoints and the volume 81/4 stay fixed, but the intermediate capacity
ranges from 18 sqrt(2)/7 to 3 sqrt(2). This is an exact fiber of the endpoint
description, not a nearest-neighbor collision. The Euclidean shape is fixed
along each theta path; it need not be the same for different s.

The first loss of the initial one-factor regime is also fixed throughout this
family: it occurs exactly at arctan(2/5). The family therefore separates the
later profile, not the first transition angle.

**Certificate and coverage.** A centered triangle has exactly one normalized
positive closure vector, here (1/3,1/3,1/3). Consequently all feasible HK weights
of A x B_s are obtained from one scalar t: the A weights equal t/3, the B weights
(1-t)/3. All six-facet orders modulo cyclic shifts are the 120 orders starting
with facet zero. Orders with zero weights are already included; there is no
missing lower-support search.

For each order the reciprocal action is

```
cos(theta) [x t^2 + y(1-t)^2] + sin(theta) z(s)t(1-t),
x,y in {+2/9,-2/9}.
```

All z(s) are affine because the factor matrix has determinant one and its dual
matrix is affine in s. The rational verifier checks at BOTH interval endpoints:

* Every order has z<=10/9, and one fixed order attains 10/9 throughout.
* In the x=y=+2/9 group, z<=(10-4s)/9, and one fixed order attains that
  bound throughout.

Endpoint checks extend these affine inequalities to the whole interval. The
first proves c_(pi/2)=18/5. At theta=pi/4, divide the ordered quadratic by sqrt(2).
The positive-positive group has maximum (7-2s)/18, reached at t=1/2. The
mixed-sign groups have maximum at most 29/90, and the negative-negative group
at most 1/6. Since (7-2s)/18>=1/3, none can overtake the first group. This proves
the intermediate formula.

To certify the first transition, the fixed word (0,3,1,4,2,5) has
x=2/9, y=-2/9, z=10/9 for every s. Its derivative with respect to t at t=1 is
(4 cos(theta)-10 sin(theta))/9. Immediately above tan(theta)=2/5, reducing t
therefore beats the pure-factor reciprocal action. The general initial-regime
bound gives the converse interval, proving the exact transition assertion.

**Interpretation.** Endpoint capacity remembers the largest mixed contribution.
It forgets whether that contribution can coexist, in the *same ordered closed
configuration*, with large positive within-factor contributions. In this family
the former stays fixed while the latter changes. A measurement of endpoint
capacity alone cannot encode this compatibility. This suggests retaining joint
within/cross action data, not adding another scalar endpoint summary.

## Evidence and reproduction

* [exact_endpoint_collision.py](exact_endpoint_collision.py): standard-library
  Fraction arithmetic, complete cyclic ordering and interval coefficient check.
* [exact-endpoint-collision.json](exact-endpoint-collision.json): rational input
  vertices, all coefficients, winners and whole-interval certificate metadata.
* [triangle_response.py](triangle_response.py): independent floating-point
  implementation of the triangle closure reduction; requires NumPy.
* [triangle-response.json](triangle-response.json): 12 deterministic affine
  triangle products, explicit geometries, 120 branches/body and full angle curves.

Run from the checkout root:

```
python3 docs/empirical-viterbo-design/desk/product-position-research/exact_endpoint_collision.py
python3 docs/empirical-viterbo-design/desk/product-position-research/triangle_response.py
```

Both runs take below one second in this environment. The 12-body numerical
control agrees with the guaranteed initial regime to relative error below 9e-16;
maximum violation of the general inverse-capacity bound is below 6e-16. These
are binary64 consistency checks, not interval certificates. They include cases
where the guaranteed initial threshold is conservative. The rational construction
provides stronger evidence for the particular separation, and the general
mass-splitting proof does not depend on the numerical sample.

## Next scientific decision

The initial question was whether a planar characteristic survives while its
lift fits into the other factor. A pure A characteristic has B-coordinate equal
to a translate of tan(theta) J2 times its A-coordinate (up to the global sign
convention). Fitting this lift is a condition for existence of that orbit;
existence alone does not establish that it still minimizes. I did not promote
that geometric containment condition to an exact capacity-transition criterion.

The useful next question is now: **can the within/cross compatibility be expressed
by a geometric condition on arbitrary factors, giving sharper transition bounds
or identifying first loss of the one-factor branch?** A bounded follow-up would
derive the pure-branch first-variation condition, compare it with the necessary
lift-containment condition, and test the resulting distinction on non-triangular
factors. Expected cost: roughly 1–2 hours including interpretation and retained
examples; general-capacity enumeration is a dependency to avoid until an actual
distinguishing example needs it. A sharper bound is plausible; a complete arbitrary-
factor classification is not presently justified. Stop if the proposed condition
only restates the HK maximum or if the distinction is already settled in the
literature. An independent literature check should precede a novelty claim.

The line should not spend its next window cataloguing more triangle formulas.
The separating family has already answered the representational question it
was chosen to answer.

Literature depth in this window: HK Theorem 1.1 and its normalization were read
directly. A targeted search also found
[Zediker's 2023 thesis on rotated 4-polytopes](https://egrove.olemiss.edu/etd/2602/),
whose abstract describes rotated-hypercube upper bounds and linearized cylindrical
capacity. Only its landing page/abstract was inspected; its full proofs and
references remain unchecked. This is a relevant novelty-check lead, not evidence
that the present arbitrary-factor deduction is new or already known.

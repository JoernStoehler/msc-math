# Exact covariance–volume collision with different capacity

Geometry-discovery owner, 2026-10-04 UTC. New elementary derivation; external
novelty is unassessed. Independent mathematical review is retained in
`product-review.md`. This concerns uniform-volume covariance, not vertex
covariance. Coordinates are `(q1,p1,q2,p2)`: both planar factors are symplectic
planes, rather than the Lagrangian factors used in the pentagon programme.

## Statement

There are two simple convex eight-facet four-polytopes with the same face
combinatorics, centered uniform-volume covariance `I4`, and volume `8748/65`,
but respective EHZ capacities

```
54 sqrt(3/65) = 11.6010609594…,
729/65        = 11.2153846154….
```

Consequently neither the full centered bulk covariance together with volume,
nor its Williamson spectrum together with volume, determines EHZ capacity in
this class. Both Williamson spectra are `{1,1}`. Their systolic ratios are
`1/2` and `243/520 = 0.4673076923…`.

This settles the reverse determinacy direction that the inherited Hamiltonian
shear did not settle. It supplies no classification of local maxima and no
counterexample to a one-sided covariance bound. It also supplies no empirical
novelty claim.

## Polygon family and exact moments

For `0<t<=1`, take the trapezoid

```
T_t = {(x,y): 0<=y<=1, |x|<=1-(1-t)y}.
```

Its four vertices are `(-1,0),(1,0),(t,1),(-t,1)`. Its uniform-volume centroid
and covariance are

```
area = 1+t,
mean = (0,(1+2t)/(3(1+t))),
Cov  = diag((1+t²)/6, (1+4t+t²)/(18(1+t)²)).
```

For example, writing `a(y)=1-(1-t)y`, direct integration gives
`E x²=(2/3) integral_0^1 a(y)^3 dy/(1+t)=(1+t²)/6`,
`E y²=(1+3t)/(6(1+t))`; subtraction of the mean square gives the displayed
y variance. Reflection in x makes the cross covariance zero.

Subtract the centroid and divide each coordinate by its positive standard
deviation. Call the resulting convex polygon `T_hat_t`. It has covariance
`I2` and area

```
r(t)=6 sqrt(3)(1+t)² / sqrt((1+t²)(1+4t+t²)).
```

This is a construction of new polygons by linear maps; those maps are not
asserted to preserve symplectic capacity or area. Area is recalculated by the
Jacobian. The square `S_hat=T_hat_1` has area12 and covarianceI2.

Set

```
u0 = -2 + 162 sqrt(3/2783),
t0 = (u0-sqrt(u0²-4))/2 = 0.335152879019….
```

Then `u0>2`, so `0<t0<1`. For `u=t+t^-1`,

```
r(t)² = 108 (u+2)² / (u(u+4)).
```

Here `u0(u0+4)=67600/2783`; therefore
`r(t0)²=108*(19683/16900)=(729/65)²` and `r(t0)=729/65`.
Also `r(1/2)=54 sqrt(3/65)`.

## Four-dimensional bodies and capacity

Define

```
K_bal = T_hat_(1/2) × T_hat_(1/2),
K_unbal = S_hat × T_hat_t0.
```

The uniform measure of a Cartesian product is the product of the uniform
factor measures. Both factors are centered with covarianceI2, so both bodies
have covarianceI4. Their volumes are respectively `r(1/2)²=8748/65` and
`12*r(t0)=8748/65`. A product of two genuine quadrilaterals is a simple
four-polytope with eight facets, sixteen vertices, and the same product face
lattice in both cases.

For convex planar polygons in two symplectic coordinate planes,

```
c_EHZ(A×B)=min(area(A),area(B)).
```

An elementary proof uses generalized characteristics. The normal cone of a
product splits: `N_(A×B)(a,b)=N_A(a)×N_B(b)`, and J preserves the two planes.
A nonconstant projected characteristic advances along its planar convex
boundary in the positive orientation; the accumulated action is a nonnegative
integer multiple of that factor's area. A closed nonconstant characteristic
therefore has action `m area(A)+n area(B)`, with integers `m,n>=0` not both
zero. Holding the other factor at an interior point realizes either factor
area. The minimal-action characterization of nonsmooth EHZ capacity then gives
the formula. See [Haim-Kislev, §2.2](https://arxiv.org/abs/1712.03494), the
primary-source input reviewed in `product-review.md`.

Applying this formula gives the capacities above; the inequality between them
is exact, because `(729/65)² < 8748/65`.

## What this calculation decides

The named claim falsified is: *there exists a function of full bulk covariance
and volume giving EHZ capacity for every simple eight-facet convex four-polytope*.
It fails even when combinatorics and the entire covariance matrix are fixed.
This is a constructive adverse example, rather than correlation or invariance
intuition. It closes this sufficiency proposal without any capacity producer.

A falsifier of our result would be an error in the polygon covariance
integrals, the algebraic parameter identity, or the nonsmooth product capacity
argument. Independent review was assigned precisely those interfaces.

The exact area split explains the missing information: covarianceI2 does not
fix a polygon's area, and covarianceI4 plus product volume fixes only the
product of those areas, whereas capacity reads their minimum. Balancing factors
maximizes capacity within this particular product geometry at fixed product
volume; that restricted fact does not address general local maximality.

Further descriptor collisions have no selected payoff. The worthwhile rival
is a non-tautological restriction on active trajectories that permits exact
first-order exclusion of local-max candidates. That route is retained in
`active-restriction.md` and compared in `report.md`.

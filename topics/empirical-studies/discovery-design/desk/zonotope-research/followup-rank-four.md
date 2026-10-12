# Rank-four follow-up: a failed proof route and a constrained extremal locus

18 September 2026. Bounded mathematical/prior-art follow-up requested by the
empirical root after review of the first packet. No additional random collection,
capacity evaluation, or sign-chamber optimization infrastructure was launched.

## Status and recommendation

The general conjecture `4 sum |Pf W_I| <= (sum |W_ij|)^2`, for rank-four real
skew matrices W, remains **unproved and unfalsified**. No applicable existing
theorem was found in the bounded follow-up. This is not a novelty claim.

The follow-up eliminated an attractive proof shortcut **using an equality
example**, rather than merely failing to make that shortcut work. It also
located a necessary degeneracy of extremizing symplectic forms. Both deductions
are proved below and were independently checked by the mathematical auditor.

Recommendation: pause the scalar inequality attack here. A further investment
would need a specific method that controls cancellation, or a justified exact
optimization argument on the constrained form locus. Additional product tests,
smooth unconstrained searches, and cancellation-free decompositions cannot
resolve the remaining question. Phi remains a well-defined measurement for
the broader zonotope research line; stopping this conjecture attempt does not
close that line.

## Why two simple bivectors do not give a quick proof

Any rank-four skew matrix can be expressed as a sum of two rank-two skew
matrices. If one could choose `W=A+B` with no entrywise cancellation, then

```
||W||_1 = ||A||_1 + ||B||_1
||W wedge W / 2||_1 = ||A wedge B||_1
                         <= ||A||_1 ||B||_1 <= ||W||_1^2 / 4
```

would prove the conjecture. Here the first norm counts only upper-triangular
entries; exterior-product coordinates use the corresponding increasing-index
basis. The missing no-cancellation hypothesis is false even at equality.

Take the six half-generators

```
(q1, p1, -q1+q2, p2, -q2, -p1-p2).
```

Their pairing matrix is

```
 0  1  0  0  0 -1
-1  0  1  0  0  0
 0 -1  0  1  0  0
 0  0 -1  0  1  0
 0  0  0 -1  0  1
 1  0  0  0 -1  0
```

It has rank four, Phi=6, Psi=9, and volume=144, so equality holds. Its
support is a six-cycle. A cancellation-free decomposition would force both
summands to have support contained in this cycle. A rank-two skew matrix
supported there cannot contain two disjoint nonzero edges: their four-index
Pfaffian has a nonzero product, and cancellation would require a four-cycle,
which the six-cycle does not contain. Thus each summand's support is a star
or a single edge. Two stars cover at most four of the six cycle edges.
Contradiction.

Consequently every two-simple decomposition of this equality matrix has
entrywise cancellations. A proof using decompositions must control those
cancellations quantitatively; the ordinary triangle inequality loses precisely
the needed information.

## Extremizing the symplectic form: necessary zero pairings

Fix spanning half-generators `v_i` in R4. Let Omega range over alternating
forms and define

```
q(Omega) = Pf(Omega),       F(Omega) = sum_{i<j} |Omega(v_i,v_j)|.
```

Choose the orientation so that the constraint is `q=1`. The opposite
Pfaffian sign is handled by replacing q with -q. In six form coordinates,

```
q = Omega12 Omega34 - Omega13 Omega24 + Omega14 Omega23,
```

which has signature `(3,3)`. Because the generators span R4, all their
pairwise wedges span the six-dimensional bivector space; hence F is a norm.

At a local minimum of F on `q=1`, let H be the intersection of the kernels
of all zero pairings. In a neighborhood within H, F is a single linear
functional L because the nonzero pairing signs remain unchanged. Necessarily

\[
\operatorname{ind}_{+}(q|_H)\le1.
\]

Otherwise `H intersect ker L` contains a direction delta with `q(delta)>0`.
For small epsilon, both `Omega +/- epsilon delta` preserve F and all active
zeros. Their average q-value is `1+epsilon^2 q(delta)>1`. Rescale the better
one to Pfaffian one; its F-value strictly decreases, a contradiction.

In particular at least two independent pairings must vanish. Exactly two
constraints from disjoint generator pairs whose four vectors span R4 cannot
suffice: in suitable coordinates they read `Omega12=Omega34=0`, leaving
`q=-Omega13 Omega24+Omega14 Omega23` of signature `(2,2)`.

This deduction prevents interpreting zero pairings at an optimized form as
an unexplained empirical phenomenon. It also rules out searching only smooth
interior sign chambers for the sharp bound.

## A stronger global-locus reduction

A global maximum of q on the compact F-unit ball can be chosen with at
least **three independent zero-pairing constraints**. The maximum is positive
because nondegenerate alternating forms exist.

The preceding argument handles fewer than two constraints and the disjoint
spanning-four case. If there are exactly two independent zero constraints
whose associated two-planes intersect, choose coordinates so they are
`Omega12=Omega13=0`. On H,

```
q = Omega14 Omega23,
```

with a two-dimensional radical. Its intersection with `ker L` contains a
nonzero delta. Moving along delta preserves both q and F while the remaining
signs remain fixed. Since the F-unit ball is compact, this movement must
reach another zero pairing in finite distance. That constraint is independent
of the old two, and the objective value is unchanged. Thus some maximizing
form lies on the claimed locus. This is a reduction for each fixed generator
configuration, not a bound on all configurations.

An exact finite approach could exploit these lower-dimensional intersections,
but no implementation or completeness certificate was attempted in this
window. Enumerating them numerically would itself not constitute a proof of
the universal inequality.

## Prior-art boundary and a probabilistic equivalent

The searches covered exterior l1/mass norms, rank/minor/Pfaffian inequalities,
Wirtinger-type arguments and random determinants. The previously checked
zonoid-algebra framework gives the functional, but its mixed-J-volume
inequalities concern complex determinants, not absolute symplectic pairings.
No theorem supplying the desired factor emerged. Euclidean mass/comass
Wirtinger bounds do not automatically control the coordinate l1 norms here.

An equivalent useful formulation is: for independent identically distributed
random vectors in R4 representing a zonoid,

\[
\mathbb E|\det(X_1,X_2,X_3,X_4)|
\le\frac32\bigl(\mathbb E|\omega(X_1,X_2)|\bigr)^2.
\]

For finite atomic distributions this follows by counting ordered distinct
indices; repeated indices contribute zero to the determinant. The Pfaffian
triangle inequality gives the weaker coefficient 3. Improving 3 to 3/2 is
the same unresolved question, not an independent discovery or solution.

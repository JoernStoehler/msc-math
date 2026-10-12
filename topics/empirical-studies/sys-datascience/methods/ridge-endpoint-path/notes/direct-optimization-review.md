# Independent mathematical review: direct ridge-sum optimization scout

## Verdict

**Pass with small but important repairs.** The product formula, mixed-area
constant, universal lower bound and equality classification, and triangular
`3x6` endpoint are correct. I found no factor-of-two or feature-coordinate
error. These results are strong enough to justify skipping a general optimizer
for now and running the proposed bounded endpoint-path smoke.

The main repair is empirical wording: the retained top-ten records already
show that each exact endpoint is below **all** 100,000 sampled rows in its
bucket, so the endpoint comparison is censored at the per-bucket sample
resolution, not merely certified past the tenth-order-statistic cutoff. This
does not estimate a population tail probability. The rotation and mixed-area
conventions should also be stated explicitly.

## 1. Product ridge-sum formula

Verified. Use coordinates `(q1,q2,p1,p2)` and

```text
omega_0((a,0),(0,b)) = a dot b.
```

The two-faces of `P x Q`, for a `k`-gon and an `m`-gon, are exactly:

- `m` copies of `P x {vertex}`;
- `k` copies of `{vertex} x Q`;
- `km` rectangles `edge_i(P) x edge_j(Q)`.

The first `k+m` have zero symplectic area. A cyclicly ordered mixed rectangle
has unsigned symplectic area `|a_i dot b_j|`. Euclidean four-volume is
`A_P A_Q`. Hence, when every product two-face is present and ordered,

```text
R(P,Q) = sum_{i,j} |a_i dot b_j| / sqrt(A_P A_Q).
```

This agrees with `omega0` and the polygonal symplectic-area implementation in
the current feature code. Reversing either polygon's cyclic orientation only
reorders/negates its edge vectors and does not change the absolute sum.

Exact wording repair: replace “includes many zero-area ridges” by “includes
exactly `k+m` zero-area product two-faces and `km` mixed rectangles, provided
all `km+k+m` faces order successfully.” For a valid product the expected ridge
count and ordered-face count are both `km+k+m`; the ordering-failure count must
be zero. This makes the implementation precondition part of the claimed
identity rather than a later caveat.

## 2. Difference-body and mixed-area identity

Verified, including the factor `4`, under the standard convention

```text
area(K+tL) = area(K) + 2t V(K,L) + t^2 area(L).
```

For a nonzero vector `b`, total variation of the projection of the boundary of
`P` gives

```text
sum_i |a_i dot b| = 2 |b| width_P(b/|b|) = 2 h_{P-P}(b).
```

Let `J(x,y)=(-y,x)` be counterclockwise rotation by 90 degrees, and orient
`Q` counterclockwise. The outward unit normal to the edge `J b_j` of `JQ` is
`b_j/|b_j|`. The polygon mixed-area formula therefore gives

```text
2 V(P-P,JQ) = sum_j |b_j| h_{P-P}(b_j/|b_j|),
sum_{i,j}|a_i dot b_j| = 4 V(P-P,JQ).
```

The identity is orientation-independent because `P-P` is centrally symmetric.
A unit-square pair gives sum `8`, `V(P-P,JQ)=2`, confirming the normalization.

Exact wording repair: define both `J` and the normalization of `V`. Otherwise
the displayed factor is ambiguous across the two common mixed-area
conventions.

## 3. Universal bound and equality

Verified for full-dimensional convex polygons:

```text
V(P-P,JQ)^2 >= area(P-P) area(Q),
area(P-P) >= 4 area(P),
R(P,Q) >= 8.
```

Equality in the difference-body inequality holds exactly when `P` is centrally
symmetric up to translation. Equality in planar Minkowski's inequality holds
exactly when `JQ` and `P-P` are positively homothetic. Thus simultaneous
equality is exactly

```text
P centrally symmetric,
JQ = lambda (P-P) + t,   lambda > 0.
```

Equivalently, after translations and independent positive rescalings, `Q` is
a quarter-turn of `P`. With the explicit counterclockwise `J` above, the
literal formula is `Q` homothetic to `-J(P-P)`; for centrally symmetric `P`,
the sign differs only by a half-turn. Squares and nondegenerate centrally
symmetric hexagons give genuine interior `4x4` and `6x6` examples, not
side-collapsed closure examples.

Repair: use the literal `-J(P-P)` formula once before the informal “a
90-degree rotation” formulation, so there is no clockwise/counterclockwise
ambiguity.

## 4. Triangle bound and the `3x6` construction

Verified. For every full-dimensional triangle,

```text
area(P-P) = 6 area(P),
R(P,Q) >= 4 sqrt(6).
```

Equality holds exactly when `JQ` is positively homothetic to `P-P`. A triangle
difference body is a genuine centrally symmetric hexagon, so this is attained
inside the `3x6` bucket.

An independent exact-coordinate check used

```text
P = conv{(0,0),(1,0),(0,1)},
Q = J(P-P).
```

It gives `A_P=1/2`, `A_Q=3`, numerator `12`, and
`R=12/sqrt(3/2)=4 sqrt(6)=9.797958971...`. (Using `-J` instead gives the same
absolute sum.) This also checks the six-side count and the factor in the mixed
area identity.

For a smoke intended to compare against the frozen `h in [0.8,1.2]` generator,
choose the equilateral-triangle/regular-hexagon representative (and a centered
square pair). After independent scaling their support heights can all be `1`,
so the endpoints lie in the closure/support of the frozen parameter family.
Keep perturbations small enough that, after normalization, all support heights
remain in `[0.8,1.2]`; otherwise the numerical CDF comparison is descriptive
but is no longer a within-generator rarity comparison.

The packet's `3x3` value `12` remains correctly labeled conjectural. Directly
rotating the standard triangle gives numerator `6`, denominator `1/2`, hence
`R=12`; this checks the claimed construction but not the unproved global
`3x3` minimum.

## 5. Million-row cutoffs and rarity wording

The stated tenth-order-statistic cutoffs exactly match the current 1M summary:

```text
3x3  12.235881326478664
3x6  10.714510249814834
4x4   9.329640964460278
6x6   8.781408639955293
```

There are 100,000 rows per bucket, so rank 10 corresponds to empirical mass
`10/100000 = 10^-4` and `-log2(10^-4)=13.2877` bits. Thus the packet's
13.3-bit cutoff statement is correct as a conservative comparison.

However, the source top-ten records also expose the first order statistic:

```text
bucket   observed minimum   exact endpoint
3x6      10.2933146430      4 sqrt(6) = 9.7979589711
4x4       8.6425623813      8
6x6       8.6123961910      8
```

Because these are the selected ten smallest values, each exact endpoint is
already verified below all 100,000 rows in its bucket. Repair the final two
rarity paragraphs as follows:

> The endpoint has empirical count zero among the 100,000 frozen rows in each
> of the `3x6`, `4x4`, and `6x6` buckets. Its empirical lower-tail rank is
> therefore right-censored beyond the sample resolution `1/100000`
> (`log2(100000)=16.6096` reciprocal-rank bits). This is not an estimate or a
> confidence bound for the generator's population tail probability, and no
> extrapolation beyond the observed resolution is justified.

Do not call the endpoint exactly “16.6 bits rare”: zero observations determine
neither its probability nor even a finite empirical reciprocal rank without a
chosen smoothing convention. Conversely, saying verification below all rows
is still missing is stale; the selected first-order statistics provide it.

## 6. Closure, feature ordering, and smoke decision

The endpoint families selected for the smoke avoid the fixed-side-count
closure problem: equilateral triangle/regular hexagon and square/square have
all edges nonzero and the advertised exact side counts. Surplus-edge collapse
remains a real issue only for later incompatible-bucket optimization.

The feature code skips any two-face whose incidence-derived cyclic ordering
fails. Such a skip can only lower the sum, so the proposed smoke must retain
the stated checks. Make the numerical acceptance contract explicit:

```text
facet_count = k+m
vertex_count = km
ridge_count = ordered_face_count = km+k+m
ordering_failure_count = 0
edge formula agrees with the 4D feature
```

Also compare fields by their names, not positional feature order; the producer
contains both raw counts and normalized scalar features and positional reuse
would be fragile.

With those repairs, the theorem-level result answers the optimization
feasibility question already. The eight-row endpoint-path smoke is the right
next discriminator for whether `sys` continues to improve toward the ridge
endpoint. A general optimizer would add cost without resolving that Goodhart
question first.

## Review evidence and limits

Review passes performed: direct coordinate/form check against the current
feature implementation; independent derivation of both mixed-area constants;
equality-case audit for both inequalities; exact-coordinate `3x6` and `3x3`
checks; side-count/interior audit; and comparison against the current 1M
summary plus its selected top-ten first-order statistics. No subagent was used.
No repository files were changed.

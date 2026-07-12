# Direct optimization scout: normalized ridge symplectic-area sum

## Scope and source truth

This is a read-only mathematical/algorithmic scout for
`ridge_symp_area_sum_over_volume_sqrt` on random Lagrangian products. The
feature is defined by
`experiments/sys-datascience/prepare/features_face_symplectic.rs`; the current
random-product source uses pairs of polygons with `3 <= k <= m <= 6`, uniformly
random normal angles, and support heights in `[0.8,1.2]`. The relevant current
tail diagnostic generated 100,000 candidates per bucket (one million total).
Its tenth-order-statistic cutoffs include

- `3x3`: 12.2358813;
- `3x6`: 10.7145102;
- `4x4`: 9.3296410;
- `6x6`: 8.7814086.

Thus crossing one of these cutoffs certifies only “better than the observed
lowest `10^-4` band” (about 13.3 bits) for that frozen generator. It does not
calibrate a rarer quantile without the full empirical order statistics or a
tail model.

## Theorem-level geometry

Let `P,Q` be full-dimensional convex polygons in the `q`- and `p`-planes. Write
their cyclic edge vectors as `a_i` and `b_j`, and their Euclidean areas as
`A_P,A_Q`. For the Lagrangian product `P x_L Q`, the feature is

```text
R(P,Q) = [sum_{i,j} |a_i dot b_j|] / sqrt(A_P A_Q).                 (1)
```

Reason: its two-faces are the `m` copies of `P x {vertex}`, the `k` copies of
`{vertex} x Q`, and the `km` rectangles `edge_i x edge_j`. The first `k+m`
faces lie in Lagrangian planes and have zero symplectic area. The mixed
rectangle has unsigned symplectic area `|a_i dot b_j|`. Formula (1) therefore
requires ridge count and ordered-face count `km+k+m` and zero ordering
failures.

Let `D P = P-P` be the difference body and let `J(x,y)=(-y,x)` be
counterclockwise rotation by 90 degrees. Normalize mixed area by
`area(K+tL)=area(K)+2t V(K,L)+t^2 area(L)`. Using total variation of a linear
functional along a convex polygon,

```text
sum_i |a_i dot b| = 2 |b| width_P(b/|b|).
```

Equivalently, in mixed-area notation `V`,

```text
sum_{i,j}|a_i dot b_j| = 4 V(D P, J Q).                            (2)
```

Minkowski's mixed-area inequality and the planar difference-body inequality
give

```text
V(DP,JQ)^2 >= area(DP) area(Q),
area(DP) >= 4 area(P),
therefore R(P,Q) >= 8.                                             (3)
```

Equality in (3) holds exactly when `P` is centrally symmetric and `JQ` is
positively homothetic to `DP`; literally, `Q` is homothetic to `-J(DP)`.
Equivalently, up to translations and independent positive scales, `Q` is a
90-degree rotation of `P`. Hence `R=8` is explicitly attained
in the `4x4` and `6x6` buckets (for example by a square and its rotated copy,
or a centrally symmetric hexagon and its rotated copy). This is not merely a
numerical extremum.

For a triangle, `area(DP)=6 area(P)`. Therefore

```text
R(P,Q) >= 4 sqrt(6) = 9.79795897...                                (4)
```

and equality is attained when `Q` is a rotated/scaled copy of the difference
body of `P`, which is a centrally symmetric hexagon. Thus (4) is an explicit
global minimum in the `3x6` bucket.

The functional is translation invariant, invariant under independent positive
rescaling of the factors, and invariant under the symplectic linear change
`(P,Q) -> (A P, A^{-T} Q)`. Consequently many apparent optimizer degrees of
freedom are gauges, and the exact equality families above are each much less
diverse symplectically than their coordinates suggest.

## Repo-specific derivation status

Equation (1) follows directly from the current coordinate convention and
feature code. Equations (2)--(4) use standard planar mixed-area and
difference-body results; I checked the constants against a unit square pair,
for which the code-level edge formula gives `R=8`.

The feature implementation silently omits two-faces whose incidence-derived
cyclic ordering fails. Any optimizer candidate must therefore require zero
ordering failures and the expected product face counts before its low value is
accepted. Otherwise numerical/combinatorial degeneration can counterfeit
improvement.

## Further derivation, not needed as a theorem for the smoke

After the symplectic affine normalization that sends one triangle to the
standard triangle, the `3x3` objective becomes the perimeter of the other
triangle in the hexagonal norm

```text
N(x,y) = |x| + |y| + |x-y|,
```

divided by its area normalization. A finite sign-cone calculation suggests
the exact `3x3` minimum is `R=12`, attained when the two triangles are related
by a 90-degree rotation. A 100,000-draw unconstrained numerical check found
`12.0391` as its best random value and approached that configuration. I have
not written a review-grade proof of the finite-cone inequality here, so `12`
should be treated as a strong derivation/conjecture, not as an established
result in this packet.

For buckets whose side counts do not match an equality family, `8` can often
be approached by shrinking surplus edges and moving to the boundary of the
nominal combinatorial class. This is an important conjectural bucket-by-bucket
statement rather than a proved classification here. In particular, a direct
optimizer is likely to discover “effective lower-side-count polygons” unless
it is constrained away from vanishing edges.

## Consequence for bits of rarity

Sample-and-filter spends approximately `2^b` independent candidates to obtain
`b` bits of lower-tail rarity; one additional bit doubles candidate cost. In
contrast, the explicit `3x6`, `4x4`, and `6x6` constructions reach the global
endpoint with constant algebraic work. Relative to the frozen one-million run,

- `3x6`: `4 sqrt(6) = 9.7980`, versus the observed lowest-`10^-4` cutoff
  `10.7145`;
- `4x4`: `8`, versus `9.3296`;
- `6x6`: `8`, versus `8.7814`.

The endpoint has empirical count zero among the 100,000 frozen rows in each of
the `3x6`, `4x4`, and `6x6` buckets. Its empirical lower-tail placement is
therefore censored beyond sample resolution `1/100000`, or 16.6096
reciprocal-rank bits. This is not an estimate or confidence bound for the
generator's population tail probability, and it does not justify calling the
endpoint exactly “16.6 bits rare” or extrapolating past the observed
resolution.

This changes the main question. For these buckets, “can optimization beat
sample-filter on `R`?” is already yes. The informative question is whether
`sys` enrichment persists along a path toward the explicit `R` endpoint, or
saturates/reverses (Goodhart behavior).

## If a general constrained optimizer is later needed

Use polygon edge variables rather than 4D facet/vertex enumeration:

```text
a_i = l_i (cos theta_i, sin theta_i),   l_i > 0,
sum_i a_i = 0,
and similarly for b_j.
```

Impose cyclic angle order with every exterior gap in `(0,pi)`, fix one global
angle, and normalize both polygon areas to one. Evaluate (1) in `O(km)` time.
This leaves only low-teens dimension for the current buckets. A practical
route is smooth continuation `|x| -> sqrt(x^2+epsilon^2)` with constrained SQP
or augmented Lagrangian, followed by exact nonsmooth evaluation; a small
multistart derivative-free method is a useful cross-check.

Guardrails are part of the mathematical question, not implementation polish:

- require a minimum edge-length/perimeter ratio (and report it) if the claim
  concerns a fixed `(k,m)` interior rather than its closure;
- separately allow boundary-seeking runs if the purpose is to expose effective
  lower-side-count optima;
- require positive area, strict convexity, expected face counts, no ordering
  failures, and agreement between edge formula (1) and the 4D feature;
- quotient or fix translation, scale, and affine/symplectic gauges, otherwise
  flat directions make optimizer progress and restart diversity misleading;
- do not infer rarity bits from objective decrease alone; map final values to a
  frozen per-bucket empirical CDF and label values beyond its resolution as
  censored.

The main failure modes are combinatorial collapse, nonsmooth stalls at
`a_i dot b_j=0`, optimizer exploitation of the face-ordering omission, and
confusing many affine copies of one symplectic shape with independent geometric
discoveries. Optimizing in the producer's normal/height coordinates also adds
irreducibility and boundedness discontinuities without adding mathematical
value.

## Cheapest discriminating smoke

Do not implement a general optimizer first. Use two exact endpoint families,
`3x6` and `4x4`, because they realize different equality mechanisms and both
sit well beyond the observed lowest-`10^-4` cutoffs.

For each bucket, make one canonical endpoint and three deterministic, strictly
convex perturbations in edge coordinates, chosen to give a short monotone
sequence from roughly the existing extreme-tail scale toward the endpoint.
This is eight candidates total. Before any target call, require:

1. edge formula/4D-feature agreement;
2. expected side, facet, ridge, and ordered-face counts;
3. zero ordering failures and no tiny-edge warning for the three interior
   perturbations;
4. frozen `R` values and frozen per-bucket empirical-CDF comparison.

Then evaluate `sys` for only those eight rows and plot/report `sys` against `R`
in path order. Interpret outcomes as follows:

- sustained rise toward both endpoints: direct proxy optimization remains
  promising and justifies a real multistart optimizer;
- flattening/reversal on both paths: cheap evidence for extreme-tail Goodhart
  behavior; do not spend on the optimizer;
- different behavior between `3x6` and `4x4`: bucket/phenotype mixture, so a
  pooled optimizer claim is inappropriate;
- endpoint-only anomaly: likely symmetry/equality-family behavior; add local
  perturbations before making a tail claim.

Eight product target calls and trivial `O(km)` pre-target calculations are
materially cheaper and more discriminating than another large sample-filter
run. The smoke should not claim a hit probability or asymptotic dependence.

## Conjectures exposed by the algebra

1. In side-count buckets compatible with the equality families, aggressive
   optimization rapidly converges to `R=8` or `4 sqrt(6)` and therefore gains
   more empirical rarity bits per feature evaluation than rejection sampling.
2. In incompatible buckets, unconstrained optimization mainly shrinks surplus
   edges and approaches the closure of a lower-complexity bucket; an interior
   edge barrier changes both the optimum and its meaning.
3. Because exact minimizers are rigid modulo symplectic affine gauges, proxy
   enrichment may saturate before the endpoint even if it is strong in the
   sampled tail. The proposed path smoke directly distinguishes this from a
   persistent-tail story at minimal cost.

## Jörn crux

No mathematical input from Jörn is needed before the smoke. The only later
stakeholder choice is semantic: whether the intended optimization domain is
the interior of each fixed `(k,m)` combinatorial bucket (requiring an explicit
minimum-edge/facet-slack convention) or its closure (where collapsing surplus
edges is legitimate). The eight-row smoke avoids this crux by using two
nondegenerate exact endpoint buckets.

# A uniform capacity plateau under fixed-width symmetrization

19 October 2026. New research-swarm derivation; the exact pentagon plateau
argument and the transfer/ratio consequences have separate independent agent
checks. The explicit trapezoid below proves the universal interval is sharp.
This is a fixed-factor capacity result, with no claim of external novelty,
human acceptance or thesis selection.

## Exact statement

Let `Q` be the regular circumradius-one pentagon with outward unit normals
`n_i=(cos(2πi/5),sin(2πi/5))`. Put

```
h=cos(π/5),  w=1+h,
B=(Q−Q)^polar/2=conv{±n_i/(2w)},
W=W_Q(K)=max{r≥0:r(Q−Q)^polar⊆K−K}.
```

For every planar convex body `K` with nonempty interior, let
`K_t=(1−t)K+t(−K)`, `0≤t≤1`. Then

```
c_EHZ(K_t ×_L Q)=W
for (3−sqrt(5))/2 ≤ t ≤ (sqrt(5)−1)/2.
```

Thus the width ceiling is reached on a guaranteed interval of length
`sqrt(5)−2≈0.236068`, centered at `t=1/2`, for every partner. The old
concavity/symmetry observation allowed the equality interval to degenerate to
its midpoint. The present conclusion gives a uniform nonzero interval,
without any bound on the partner's aspect ratio, smoothness or number of
facets. Some partners reach the ceiling earlier. The displayed endpoints are
optimal as a uniform guarantee over all partners: the four-vertex partner below
has exactly this capacity-saturation interval.

## Imported inputs and geometric margin

The [symmetric-partner proof](symmetric-partner-lemma.md) supplies the complete
ten normal-triangle family for `Q`, the segment upper bound `c(L×_L Q)≤W_Q(L)`,
and the following template geometry. Each unit template has longest side
length `2R`, with `R=1/(2w)`, parallel to some `n_i`. After translating the
side midpoint to zero its endpoints are `±d/2`, antipodal vertices of `B`;
its remaining vertex `v` is perpendicular to that side and has length
`R tan(π/5)`.

The incircle of `B` has radius `R cos(π/10)`. Consequently

```
v∈η B,    η=tan(π/5)/cos(π/10)=3−sqrt(5)<1.
```

The identity follows from `sin(π/5)=2sin(π/10)cos(π/10)`,
`sin(π/10)=(sqrt(5)−1)/4` and `cos(π/5)=(sqrt(5)+1)/4`.
For this pentagon the apex direction meets an edge midpoint of `B`, so its
`B`-gauge is exactly `η`; only the displayed containment is needed.
Both traversal orders of every normal triple have this property.

The capacity interpretation uses the imported BMP translation-cover criterion:
capacity is at least one exactly when all normalized normal triangles translate
into the partner. It is not inferred from numerical LP agreement. The source
and convention contract remain in the existing symmetric-partner proof and
[research charter](research-charter.md).

## Constructive proof

Set `S=(K−K)/2`. The maximum `W` is positive and attained, and `WB⊆S`.
Fix `0≤t≤1/2`, and write `s=1−2t`. The exact Minkowski identity is

```
K_t=sK+(1−s)S.
```

Scale one template by `W`, and write its centered vertices as `±d/2,v`, where
`d∈2WB⊆K−K` and `v∈η WB⊆η S`. Choose `a,b∈K` with `a−b=d`, and let
`m=(a+b)/2∈K`. The proposed translated template has vertices

```
s m+d/2=(1−t)a−t b,
s m−d/2=(1−t)b−t a,
s m+v.
```

The first two lie in `(1−t)K+t(−K)=K_t` directly. If `2t=1−s≥η`, then
`v∈(1−s)S`, since `0∈S` and `S` is convex. Hence the third vertex lies in
`sK+(1−s)S=K_t`. Convexity contains the whole translated triangle. The choices
of `a,b,m` may differ for the ten templates, as permitted by the covering
criterion; a single placement of all triangles is not required.

This constructs fits of all ten templates at scale `W` for `η/2≤t≤1/2`.
Therefore `c(K_t×_L Q)≥W`. On the other hand

```
K_t−K_t=K−K,
```

so the segment upper bound gives `c(K_t×_L Q)≤W` for every `t`.
For `1/2≤t≤1−η/2`, apply the same argument to `−K` at parameter `1−t`;
its interpolation body is exactly `K_t` and its difference body is unchanged.
This proves the stated closed interval, including its endpoints.

## Sharpness: one explicit trapezoid

Write `θ=π/5`, `c=cos(θ/2)`, `q=tan(θ/2)`,
`a=(3−sqrt(5))/2`, and let `C=B/R=conv{±n_i}` be the circumradius-one decagon.
Consider the convex trapezoid

```
K*=conv{(−1,0),(1,0),(a,−2c),(−a,−2c)}.
```

Its half-difference body is the hexagon

```
S*=(K*−K*)/2
  ={(x,y): |x+q y|≤1, |x−q y|≤1, |y|≤c}.
```

Indeed the inclined sides of `K*` have normals `(±1,−q)`, since
`2cq=1−a`; adding the reflected normal fan gives the four inclined
inequalities above, and the vertical width is `2c`. Their offsets follow
directly by evaluating supports on the four displayed vertices.

To check `C⊆S*`, use reflection symmetry across both coordinate axes. The
three first-quadrant vertices of `C` are

```
(1,0), (h,sin θ), (sin(θ/2),c).
```

Their sums `x+q y` are respectively `1`, `1`, and `2sin(θ/2)<1`, using
`q sin θ=1−h` and `q c=sin(θ/2)`. They also satisfy `y≤c`. This checks every
decagon vertex. The horizontal support of `S*` is exactly one, so `C` cannot
be enlarged inside it. Therefore `W_Q(K*)R=1`, or `W_Q(K*)=2(1+h)`.

The two triangles

```
T±=conv{(−1,0),(1,0),(0,±tan θ)}
```

are `W_Q(K*)`-scaled normal templates: their directed edges use `n_0,n_2,n_3`
in the two orders, with total support-function length
`h(2+2/h)=2(1+h)`. Both therefore have to translate into a width-saturated
partner.

For every `0≤t≤1`, the only points of `K*_t` with horizontal coordinate `1`
and `−1` are `(1,0)` and `(−1,0)`. This follows from the corresponding unique
exposed points of each summand `(1−t)K*` and `t(−K*)`. A translated `T±` has
a horizontal side of length two. Its endpoints are therefore forced to those
two points, fixing its translation completely. But

```
max{y:(x,y)∈K*_t}=2tc,
min{y:(x,y)∈K*_t}=−2(1−t)c.
```

The upward template requires `tan θ≤2tc`, and the downward template requires
`tan θ≤2(1−t)c`. Thus capacity saturation requires

```
a=tan θ/(2c) ≤ t ≤ 1−a.
```

The universal theorem supplies the converse. Consequently this single
trapezoid has **exactly** the saturation interval `[a,1−a]`; outside it one of
the two normal templates fails and capacity is strictly below `W_Q(K*)`.
This proves optimality of both universal endpoints, without optimizing over
sampled bodies. The sharpness partner is not claimed to optimize systolic
ratio or area among partners.

The [read-only exact check](verify-sharp-symmetrization-threshold-20261019.py)
reconstructs the actual half-difference hull, checks all ten decagon vertices
inside it, verifies the width/contact identities, and checks normal-template
membership using the existing ordered `QQ(tau)` helper. Its arithmetic PASS
corroborates the displayed proof; it does not independently verify that helper.

## Transferable geometric lemma

The construction does not depend on regular-pentagon symmetry after the
calibration has been supplied. Let `B=−B` be a convex body, and consider finitely
many translation templates. Suppose each template has a translation of one of
the following forms, for one common `0≤η<1`:

1. it lies in `η B`; or
2. it is a triangle with vertices `±d/2,v`, where `d∈2B` and `v∈η B`.

Then every `K` with `B⊆(K−K)/2` contains a translated copy of every template in
`K_t` whenever `η/2≤t≤1−η/2`. For the first type, choose any `m∈K` and use
`s m+η B⊆sK+(1−s)S`; for the second, use the endpoint construction above.
The lemma is a statement about translation covers. Turning it into a capacity
plateau additionally requires the relevant complete-template criterion and a
matching difference-body upper bound.

In particular, the strict interior and exact antipodal-side placements in the
[odd-regular calibration](odd-regular-symmetric-partner.md) give some uniform
nondegenerate plateau for each fixed odd regular `n≥5`: there are finitely many
templates. Take the maximum of the whole-template `B`-gauges for the strictly
interior non-extreme placements and the apex `B`-gauges for the extreme
placements; each is strictly below one. The antipodal endpoints of an extreme
template retain gauge one and are handled by the separate endpoint construction.
No explicit general-`n` constant or optimality is claimed here. The same
conditional implication applies to the finite template calibration in
[local calibration stability](local-calibration-stability.md).

## Ratio along the central plateau

The exact mixed-area identity already used in [the first window](first-window.md)
is

```
area(K_t)=area(K)+t(1−t)[area(K−K)−4area(K)].
```

Brunn–Minkowski and its equality condition give
`area(K−K)≥4area(K)`, with equality exactly when `K` is centrally symmetric up
to translation. Therefore for a partner that is not centrally symmetric, area
strictly increases as `t` runs from `η/2` to `1/2`, while capacity is constant.
Its systolic ratio strictly decreases on that interval and strictly increases
on the reflected interval from `1/2` to `1−η/2`. Thus no parameter strictly
inside the guaranteed central interval is a local ratio maximum along this
fixed-width path. The midpoint is a strict pathwise ratio minimum. This is a
selection statement about this path, not a global maximum theorem.

For centrally symmetric `K`, every `K_t` is a translate of `K`, and both area
and ratio stay constant. The mixed-area identity and equality condition are
standard inputs; their conjunction with the uniform capacity plateau is the
new consequence here.

## Polygonal paths: ratio maxima occur at capacity switches

If `K` is a polygon, the union normal fan of `K` and `−K` represents every
`K_t`, with redundant inequalities allowed at the endpoints. The support offsets
vary linearly in `t`. To see finiteness without assuming a nondegenerate LP
basis, translate each normal template so that zero is in its interior. Its
support coefficients `s_ij=h_Tj(u_i)` at all fixed facet directions are then
strictly positive. The dual feasible set

```
{y≥0: Σ_i y_i u_i=0, Σ_i y_i s_ij=1}
```

is a compact polytope: positivity bounds every `y_i`. It has finitely many
vertices, and the minimum of the affine objective `Σ_i y_i h_i(t)` is therefore
a finite minimum of affine functions. Taking the minimum over the complete ten
templates preserves this property. This is the existing parametric-LP fact
from the research charter, with its degeneracy interface supplied explicitly;
it is not a numerical reliability claim for the floating-point evaluator.

On any maximal interval where `c(t)=a+bt`, with `0≤t≤1/2`, concavity and
reflection symmetry give `b≥0`. Concavity also gives `a≥c(0)>0`, since the
active affine function is a supporting line. Put
`A0=area(K)` and `D=area(K−K)−4A0≥0`. After suppressing the constant positive
factor `1/[2area(Q)]`, the ratio is

```
f(t)=(a+bt)^2/[A0+D t(1−t)].
```

Its derivative has the sign of

```
L(t)=2bA0−aD+D(b+2a)t.
```

For a partner that is not centrally symmetric, `D>0`, so `L` is strictly
increasing. An interior critical point of this affine-capacity interval is
therefore a strict local minimum, and there is no interior local maximum.
Consequently, for a non-centrally-symmetric polygon, the maximum ratio on
`0≤t≤1/2` is attained at `t=0` or at a capacity breakpoint. The endpoint `1/2` is a strict local minimum by the
central plateau, so it need not be included among the candidates.

This is a finite pathwise reduction, not a global shape reduction. A
breakpoint can occur before width saturation, and nothing here forces a
maximizing breakpoint to be the plateau onset. Distinct templates can switch,
or a single template can change its translation-fit basis; no orbit label is
inferred. The argument does not exclude a local maximum for a smooth
non-polygonal capacity curve with nonzero curvature.

## What this resolves and what stays open

This resolves whether the symmetrization plateau can be only a point for the
fixed regular pentagon: it cannot. It does not determine a given partner's
first plateau parameter, prove the global HKO ratio bound for arbitrary
partners, classify equality partners of an area inequality, or establish
novelty relative to outside literature.

The [existing onset LP](first-window.md#onset-lp), implemented in
[`symmetrization.py`](symmetrization.py), already computes the first plateau
parameter of a polygon. It is reused for the bounded controls below; no new
onset algorithm is claimed. These floating-point controls corroborate the
construction and do not certify the theorem or an optimal interval.

## Bounded controls and review

The [control script](verify-symmetrization-plateau-20261019.py) reuses the existing
onset LP, explicitly constructs the proof's endpoint/apex placements, and checks
the capacity plateau and mixed-area formula at five central parameters. Its
eight deterministic controls include two regular triangles, a skinny triangle,
an irregular pentagon and its large translation, an HKO-angle pentagon, a
regular decagon, and an elongated irregular pentagon. The two triangle onsets
were approximately `0.2225624520` and `0.2276329200`; the other six controls
already attained the ceiling at `t=0`. These observations neither prove
optimality of the universal interval nor classify the first onset.

The script writes only the new path named by `--out` and refuses to replace an
existing file. For a fresh run from repository root:

```sh
uv run topics/products/pentagon-arbitrary-partner/verify-symmetrization-plateau-20261019.py \
  --out /tmp/msc-math/plateau-controls/NEW-RUN/results.json
```

The read-only exact sharpness check is:

```sh
uv run topics/products/pentagon-arbitrary-partner/verify-sharp-symmetrization-threshold-20261019.py
```

The original floating-point run's dependency versions, script hash, inputs and residuals are
recorded in the retained [control result](symmetrization-plateau-controls-20261019.json).
This is floating-point corroboration, not an exact arithmetic proof.

The [initial independent check](symmetrization-plateau-review-20261019.md)
passes the exact pentagon implication and directly checks the imported BMP v2
criterion. It predates the general transfer, mixed-area and polygonal
breakpoint paragraphs; those have a separate [extension audit](symmetrization-plateau-extension-review-20261019.md).
The later sharpness section has its own [independent check](symmetrization-plateau-sharpness-review-20261019.md). Both are
independent agent checks, not human acceptance. No existing thesis source,
retained producer output or global-cover claim was changed by this note.

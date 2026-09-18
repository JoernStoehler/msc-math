# Equal widths, area and fitting scales; different shape information

2026-09-18. Follow-up to the methods owner's
[third-harmonic adverse moment example](../relation-discovery/moment-adverse-example.md).
The question here is specific: **do the entire fitting vector and width field
already subsume the geometric information supplied by moments and polarity?**
We do not optimize or repair the falsified moment inequality.

The answer is no. The construction below has the same entire width function,
area, perimeter, capacity and all ten fitting scales, but different polar area
products. Moreover, the maximal-template *placements* do distinguish the chosen
modes. Thus the richer LP interface contains useful information which its ten
optimal values discard. This is a representation result, not a new extremal
capacity claim or a literature novelty claim.

## Exact smooth-body construction

Let m be an odd integer at least three and let the body's support function be

```
h(theta)=1+a cos(m theta),     a>0.
```

Use u(theta)=(cos(theta),sin(theta)) and v(theta)=(-sin(theta),cos(theta)).
If (m²-1)a<1, the positive curvature radius h+h'' makes this a smooth strictly
convex body, with boundary

```
x(theta)=h(theta)u(theta)+h'(theta)v(theta).
```

Its m-fold rotational symmetry fixes the area centroid at zero. Oddness gives
h(theta)+h(theta+pi)=2, so its difference body is exactly the radius-two disk.
Its perimeter is 2pi. Green's area formula gives

```
area(K)=(1/2)∫[h²-(h')²]dtheta
       =pi[1-(m²-1)a²/2].
```

Choose

```
a_m²=1/[200(m²-1)],  m=3,5,7.
a_3=1/40;  a_5=1/(40sqrt(3));  a_7=1/(40sqrt(6)).
```

All three then have area 399pi/400. The strict-convexity condition holds.
These are support-function harmonics, not the radial harmonics used by the
methods owner's moment counterexample; confusing the two changes the body.

## Why every fitting scale stays fixed

Write w=1+cos(pi/5), with the fixed Q=P5 convention of the evaluator.
Each unit-Q-perimeter template has longest edge length 1/w. Scaling it by
2w gives longest edge length two and perpendicular apex height tan(pi/5).

The support-boundary points with normals u(theta) and -u(theta) satisfy

```
x(theta)-x(theta+pi)=2u(theta).
```

Their midpoint is

```
p(theta)=a cos(mtheta)u(theta)-ma sin(mtheta)v(theta),
|p(theta)|<=ma.
```

Center the scaled template's longest edge at p(theta), in its prescribed
normal direction. Its endpoints are those boundary points. K contains the
disk of radius 1-a, since h>=1-a. The third vertex is also inside K whenever

```
ma+tan(pi/5) <= 1-a,
i.e. (m+1)a <= 1-tan(pi/5).
```

All three chosen amplitudes satisfy this inequality. Convexity therefore puts
the entire template in K. No larger scale can fit: every diameter of a
constant-width-two body has length at most two. Thus each of the ten scales is
exactly 2w, and capacity has that same value. This holds at every relative
orientation, not just the orientations sampled in an experiment.

The argument applies more generally to h=1+f with f(theta+pi)=-f(theta),
h+h''>0. Set epsilon=||f||∞ and M=sup|f u+f'v|. If

```
M+tan(pi/5) <= 1-epsilon,
```

the same proof fixes all ten scales at 2w. This is an infinite-dimensional
neighborhood of small odd support perturbations, not a coincidence of three
selected curves. Translation and representative conventions remain explicit.

## Exact geometric separation

The centroid is zero, so the centroid-polar body's radial function is 1/h.
Consequently

```
area(K^polar)=(1/2)∫[1+a cos(mtheta)]^-2 dtheta
            =pi/(1-a²)^(3/2).
```

The integral is independent of integer m: substitute mtheta and use
periodicity. Its evaluation follows, for example, by differentiating the
standard tangent-half-angle integral
∫[b+a cos(theta)]^-1 dtheta=2pi/sqrt(b²-a²) with respect to b, then setting
b=1. Therefore the exact polar area products are

```
(399pi²/400) [1-1/(200(m²-1))]^-3/2,  m=3,5,7.
```

They are strictly different despite equality of all the geometric/symplectic
quantities listed above. These are also distinct affine-invariant volume
products, so the distinction is not simply a rotation or affine change of
representative. This exact separation is stronger evidence than a small
numerical moment difference.

## What the full LP placements recover

Because the bodies are strictly convex, the diameter segment in each prescribed
direction has its unique pair of support endpoints. A maximum-scale placement
of a template must use that pair for its longest edge. Its midpoint is therefore
exactly p(theta), not an arbitrary solver choice.

Reading the midpoint from the optimal translation plus the template's known
longest-edge midpoint gives samples of the odd support data:

```
u(theta)·p(theta)=a cos(mtheta),
v(theta)·p(theta)=-ma sin(mtheta).
```

These distinguish our third-, fifth- and seventh-harmonic bodies even though
all fitting scales agree. They are useful representative-dependent geometric
responses, not additional symplectic capacities. This does not assert that
finitely many placements reconstruct an arbitrary support function: their
sampling directions are finite, and other odd perturbations can be invisible
there too.

## Polygon evidence and the smooth/polygon boundary

[Retained geometries and fits](harmonic-fibers-v1.jsonl) contain 360- and
720-vertex inscribed approximants of each of the three support-function bodies,
plus the methods owner's actual 96-vertex radial-third-harmonic example as a
separate control. No random shapes or additional optimization campaign ran.

[Summary](harmonic-fibers-v1-summary.json) retains source parameters, exact
amplitude squares, hashes, all scale errors and midpoint checks. The replay
uses seven bodies and takes about two seconds locally.

The approximating polygons do **not** have exactly constant width or exactly
the prescribed smooth area. At 720 vertices, sampled width errors are at most
1.91e-5 and area errors at most 4.47e-5; both improve roughly fourfold when
resolution doubles. All ten fitting values differ from 2w by less than 2.8e-14
for this sample grid. The grid includes the prescribed diameter endpoints,
which explains this much smaller fitting error; it is not evidence that the
polygons have exact constant width. Stored vertices are floating-point numbers.
The exact statements above concern the explicitly defined smooth bodies.

Fitted midpoint locations agree with the analytic p(theta) within 3.5e-12.
Numerical continuous fourth moments, covariance and polar products also separate
the modes. The degree-three moment norm is nonzero for m=3 and zero for the
smooth m=5,7 bodies by rotational symmetry; it cannot alone describe their
asymmetry. The radial control has widths ranging approximately 1.9997–2.0208
and c=A2, confirming that it is a different intervention.

## Judgment

This settles the immediate ambiguity: keep both support/asymmetry geometry and
fitting placements; ten fitting values, width data and area alone lose information
we have now exhibited exactly. The result justifies the existing representation
breadth. It does not justify enlarging this harmonic sample or starting a moment
extremal project. A future measurement should answer a new question, such as
which missing directional information matters in a strict c<A2 regime, rather
than record more copies of this demonstrated blind direction.

Reproduce without changing other owners' files:

```
PYTHONDONTWRITEBYTECODE=1 uv run --with numpy==2.5.3 --with scipy==1.18.1 python docs/empirical-viterbo-design/desk/pentagon-arbitrary-partner/harmonic_fibers.py
```

The cross-line control's source file hash and original stable ID are retained.
Its new ID follows this owner's canonical hull-order/binary-coordinate hash;
both IDs refer to the stored lineage rather than an inferred rounded match.

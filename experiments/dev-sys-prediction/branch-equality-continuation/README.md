# Branch-Equality Continuation

Status: retained method experiment; the run is empirical evidence
about the sampler and its pentagon-product control, not evidence that a new
high-`sys` region exists.

## Question and downstream decision

Can we cheaply generate local points satisfying

```text
action_sigma(a) = action_tau(a)
```

for two fixed admissible KKT branches, rather than hoping that an ambient
random sample lands near this codimension-one set?

If the equality correction is stable and the full capacity recomputation says
that both branches remain minimizing on some corrected points, the same method
is worth applying to near-active branch pairs at non-product basepoints. If the
correction works but another branch always lies below the selected pair, the
method is working but this pair does not expose a ridge of the `sys` lower
envelope. If correction or branch validity fails already in the control, the
general search should stop until the chart or continuation rule is repaired.

This packet develops the sampling method. Optimizer use of equality
continuation belongs to `experiments/dev-gradient-ascent/` and is deliberately
deferred.

## Positive control and measured object

The basepoint is

```text
P_5 x_L R(9 degrees) P_5.
```

The retained pentagon branch landscape records twenty symmetry-related
capacity-minimizing raw words at this point. Several raw words have the same
numerical first-order action jet at the basepoint. The producer groups equal
logarithmic action gradients, then selects two distinct groups with the
strongest sampled exposed-edge witness: a direction where the two linearized
actions agree and lie below every other gradient group. Raw-word duplicates
are not allowed to turn floating-point noise into a false exposed-edge margin.

### Product chart and the meaning of “slice”

Let `h_* = cos(pi/5)`, let `theta_* = pi/20`, and let `n_i,m_j` be the fixed
regular-pentagon unit normals. The eleven-dimensional parameter is

```text
z = (u_1,...,u_5, v_1,...,v_5, phi) in R^11.
```

It represents the Lagrangian product `K(z)=Q(u) x_L P(v,phi)` with

```text
Q(u)       = {q : <n_i,q> <= h_* exp(u_i) for every i},
P(v,phi)   = {p : <R(theta_*+phi)m_j,p> <= h_* exp(v_j) for every j}.
```

Thus `z=0` is `P_5 x_L R(9 degrees)P_5`. This chart varies all ten support
numbers and the relative factor angle, but it does not vary the five normal
angles of each factor independently. It is an eleven-dimensional product
family, not a chart on all ten-facet four-polytopes.

Six tangent directions at `z=0` change only the representative for `sys`:

```text
q translation by t:  u_i' = <n_i,t>/h_*,
p translation by s:  v_j' = <R(theta_*)m_j,s>/h_*,
common dilation:      (u',v',phi') = (1_5, 1_5,0),
reciprocal scaling:   (u',v',phi') = (1_5,-1_5,0).
```

The first four directions come from two translations of each planar factor.
Common dilation leaves `sys` invariant; reciprocal factor scaling is the
linear symplectic map `(q,p) -> (lambda q,lambda^-1 p)`. Let `E` be their
six-dimensional span in `R^11`. The code applies Gram--Schmidt and fixes

```text
S = E^perp,             dim S = 5,
z = B y,                y in R^5,
```

where the columns of `B` are the recorded orthonormal basis of `S`. This is
what this experiment calls the **local slice**. More precisely, it is a fixed
affine subspace transverse at the basepoint to the six known equivalence
directions inside this particular product chart. It is not a constructed
global quotient, it does not remove symmetries outside this chart, and no
unique-representative theorem is used. Euclidean orthogonality in the chosen
`(u,v,phi)` coordinates is also part of the sampling law rather than an
intrinsic metric on polytopes.

### Pairwise equality manifold and projection sampler

For a chosen branch pair, let

```text
f(y) = log(action_sigma(y)) - log(action_tau(y))
```

on the five-dimensional slice. While both fixed-word KKT optima remain smooth
and beta-positive, `f` is smooth. The selected pair satisfies
`grad f(0) != 0`; hence the implicit-function theorem makes `f^{-1}(0)` a
four-dimensional submanifold of `S` near the basepoint.

For a requested radius `r`, the implemented proposal law is:

```text
g ~ N(0,I_5),
n = grad f(0),
u = normalize(g - n <n,g>/|n|^2),
y_0 = r u.
```

Thus `u` is uniform on the unit three-sphere in `ker Df(0)` for the chosen
Euclidean coordinates. The code then applies normal Newton correction

```text
y <- y - f(y) grad(f)(y) / |grad(f)(y)|^2.
```

until the log-action residual is below tolerance. Geometry construction,
branch invalidity, loss of transversality, and failure to converge are explicit
rejection states. Numerical KKT nullity is recorded separately rather than
silently treating every feasible solve as a smooth branch. The corrected
images are therefore a specified local
projection sampler; they are not asserted to be uniform for intrinsic surface
measure on the curved equality manifold.

Equality is not enough for capacity relevance. At the regular basepoint, the
twenty tied raw words are first grouped into five numerical log-action-gradient
groups. For each pair of groups, the code samples tangent directions `u` with
equal first-order slope and scores

```text
margin(u) = min_{k outside the pair}
              (<g_k,u> - (<g_sigma,u>+<g_tau,u>)/2).
```

The selected pair/direction has positive sampled margin, so it is an
expected-positive lower-envelope witness to first order. Every corrected point
is nevertheless passed through the full minima-safe billiard capacity search.
That recomputation, not the equality equation, labels whether the pair is
actually jointly minimizing or is undercut by a third branch.

## Controls and recorded checks

Each corrected point records:

- equality residual, Newton iterations, and correction size;
- both fixed-word actions, beta margins, and numerical KKT nullities;
- the full minima-safe billiard capacity result;
- the pair's relative gap above the recomputed minimum;
- whether both exact words are retained in the returned action window;
- the closest returned third branch, volume, and `sys`.

One deterministic exposed-edge witness ray is included at every radius. Random
tangent directions test whether success is restricted to that selected cone.
The regular basepoint itself checks the twenty-way action tie and the rank of
the local slice.

## Predicted outcomes, value, and stopping rules

Before running the retained panel:

- `0.75` probability: at least 90% of proposals through radius `1e-3` correct
  to relative action residual below `1e-10` without branch invalidity. This
  establishes usable local equality-manifold plumbing.
- `0.15` probability: correction is numerically reliable but no sampled point
  has the selected pair jointly capacity-minimizing. The method survives, but
  pair selection or the basepoint is unsuitable for finding lower-envelope
  ridges.
- `0.10` probability: rank loss, invalid geometry, beta loss, or Newton
  failures prevent a reliable control. Stop rather than applying the method to
  general polytopes.

These events are not disjoint in every detail; the headline decision is
whether the correction works, followed by whether it reaches a
capacity-relevant part of the equality set.

Start with two random directions at radii `1e-5` and `1e-4`. Stop on a slice
rank error, a non-transverse selected pair, or more than half correction
failures. Only then run the small retained panel. Do not widen to arbitrary
polytopes or add an optimizer loop in this experiment.

The direct thesis value of a sampler-only result is low. Its option value is
that future searches can target measure-zero branch ridges and intersections
instead of relying on ambient random sampling. A dominated-pair result still
separates failure of the sampling method from failure of the chosen geometric
mechanism.

## Retained result

The retained panel contains 68 corrected points: one exposed-edge witness and
sixteen random tangent proposals at each of four radii from `1e-5` through
`1e-2`. All 68 geometry/branch evaluations and corrections succeeded. All 51
points through radius `1e-3` had positive beta margins and relative equality
residual below `1e-10`; their selected KKT systems also had zero recorded
numerical nullity. The largest residual in the entire panel was
`9.28e-13`. The largest correction was `0.00109` times its requested radius.
This passes the stated pre-run local reliability threshold.

The raw twenty-way tie collapsed to five distinct logarithmic action-gradient
groups. The selected pair had sampled exposed margin `0.618` per unit slice
step. All four exposed witness points remained nominal joint capacity
minimizers. Across the 64 random tangent proposals, only 27 remained joint
minimizers; the rest satisfied the selected equality but were undercut by a
third branch. Equality correction and lower-envelope relevance are therefore
separate tasks, and exposed-cone information is useful before sampling.

No point had `sys>1`. The maximum observed value was `0.973435`, versus base
`0.970975`, but its selected equal pair was dominated by relative action gap
`1.22e-4`. Among points where the selected pair was jointly minimizing, the
maximum was `0.970976`, at radius `1e-5`. This small panel supports the method,
not a new high-`sys` product mechanism.

Machine-readable aggregates and the explicit pass criterion are in
`analysis.json`.

## Claim boundary

Numerical equality of two fixed KKT branch actions does not prove either branch
computes the capacity. The full billiard recomputation supplies the empirical
capacity-relevance label for this product control. Neither calculation proves
that the sampled component is complete, that the local slice is global, or
that a finite random panel excludes thin jointly-minimizing regions.

## Commands

Smoke run:

```bash
cargo run -p exp-dev-sys-prediction --release \
  --bin dev-branch-equality-continuation -- \
  --samples-per-radius 2 --radii 1e-5,1e-4 \
  --output /tmp/branch-equality-smoke.jsonl
```

Retained local panel:

```bash
cargo run -p exp-dev-sys-prediction --release \
  --bin dev-branch-equality-continuation -- \
  --samples-per-radius 16 --radii 1e-5,1e-4,1e-3,1e-2 \
  --output experiments/dev-sys-prediction/branch-equality-continuation/pentagon-control.jsonl
```

Analyze the retained panel:

```bash
uv run --script experiments/dev-sys-prediction/branch-equality-continuation/analyze.py \
  --input experiments/dev-sys-prediction/branch-equality-continuation/pentagon-control.jsonl \
  --output experiments/dev-sys-prediction/branch-equality-continuation/analysis.json
```

# Branch-Equality Continuation

Status: approved method experiment; the retained run is empirical evidence
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

The local parameter chart has eleven coordinates:

- five logarithmic support-number changes for the first pentagon;
- five logarithmic support-number changes for the second pentagon;
- the relative rotation angle.

At the basepoint, the producer removes the six continuous equivalence
directions present in this chart: two translations of each factor, common
dilation, and reciprocal factor scaling. Fixing the orthogonal complement gives
a five-dimensional local affine slice. This is a local transversal at the
basepoint, not a global quotient or a claim that every nearby polytope has a
unique representative.

For a chosen branch pair, let

```text
f(y) = log(action_sigma(y)) - log(action_tau(y))
```

on the five-dimensional slice. A proposal direction is sampled from a standard
Gaussian, projected to `ker Df(0)`, normalized, and scaled to the requested
radius. Newton correction along the current normal repeatedly applies

```text
y <- y - f(y) grad(f)(y) / |grad(f)(y)|^2.
```

Thus the random proposals are uniform on the tangent three-sphere before
correction. Their corrected images are a specified local projection sampler;
they are not asserted to be uniform for intrinsic surface measure on the
curved equality manifold.

## Controls and recorded checks

Each corrected point records:

- equality residual, Newton iterations, and correction size;
- both fixed-word actions and beta margins;
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
residual below `1e-10`; the largest residual in the entire panel was
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

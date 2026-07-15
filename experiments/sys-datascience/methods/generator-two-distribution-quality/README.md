# Two-distribution quality comparison

This packet is a target-free breadth pass for deciding whether two sampleable
polytope distributions are redundant, or differ in a way worth transferring.
It reads only `factor-shape-row-v1` polygon rows and never evaluates `sys`,
capacity, a target cache, or a p-value. Every comparison is stratified by
fixed side count, so a change in the facet-count mixture cannot masquerade as
geometric separation.

## Reproduce

The retained smoke uses the small generator-zoo source already present on the
line branch. Population labels are exact (law plus knob):

```bash
cd experiments/sys-datascience/methods/generator-two-distribution-quality
uv run --script compare.py --calibrate --out-dir artifacts/calibration
uv run --script compare.py \
  --input ../generator-zoo-smoke/artifacts/factor-shapes.jsonl \
  --left-population 'current-baseline[delta=0.2]' \
  --right-population 'repulsive-gap[alpha=1]' \
  --out-dir artifacts/current-vs-gap
uv run --script compare.py \
  --input ../generator-zoo-smoke/artifacts/factor-shapes.jsonl \
  --left-population 'current-baseline[delta=0.2]' \
  --right-population 'zonogon[lengths=uniform(0.5,1.5)]' \
  --out-dir artifacts/current-vs-zonogon
uv run --script compare.py \
  --input ../generator-zoo-smoke/artifacts/factor-shapes.jsonl \
  --left-population 'current-baseline[delta=0.2]' \
  --right-population 'primal-hull-uniform-disk[points=n+4,origin=interior]' \
  --out-dir artifacts/current-vs-hull
uv run --script test_compare.py
```

The calibration is a deterministic 12-dimensional synthetic control with
same-law, location, scale, narrow/broad support, identical-support mixture
weight, disjoint-mode, outlier, and high-dimensional-noise cases. It is a
qualitative sanity check for ordering and failure modes, not a threshold test.
The retained generator reports apply the surviving subset to three small
existing pairs: baseline versus `repulsive-gap[alpha=1]` (strata `n=3,4,6`),
baseline versus zonogon, and baseline versus primal hull (both `n=4,6`). Each
has three views and a compact `comparison.tsv` plus method guide.

## Views and methods

For each fixed side count, the script compares:

* `raw_ordered`: area-normalized, centroid-centered vertex coordinates in the
  producer's cyclic index and frame. Separation here can be representation-
  induced.
* `canonicalized`: the sequence of normalized edge lengths and positive
  exterior turns is cyclically canonicalized by its lexicographically smallest
  start. This removes common rotation and cyclic start even for regular
  polygons, retains CCW orientation, and avoids a principal-axis eigenvalue
  degeneracy.
* `chord_multiset_quotient`: the sorted multiset of all pairwise chord lengths.
  This quotients translation, scale, rotation, reflection, and every vertex
  permutation, but discards adjacency. It is not an optimal cyclic/dihedral
  assignment distance and is not orientation-sensitive.

The implemented diagnostics are energy V-statistics, an explicit RBF-MMD
bandwidth ladder (`0.5, 1, 2, 4` times the cross-sample median), sliced
Wasserstein with 32 seeded projections, grouped five-fold nearest-centroid and
5-nearest-neighbor cross-validation, cross-nearest-neighbor mixing,
precision/recall-style local support coverage and density, 90% quantile-region
overlap, and smoothed one-dimensional occupancy Jensen--Shannon divergence.
Classifier results are diagnostic separability only; no classifier accuracy or
small p-value is treated as coverage evidence. The JS calculation uses
additive smoothing (`0.5`) and its one-dimensional projection is deliberately
reported as bin-sensitive.

Rows with the same `pair_bucket` are assigned to the same deterministic CV
group. If no bucket is present, the stable sample ID supplies a group. Missing
side-count strata are omitted from pairwise metrics and remain visible in the
report's input populations; no pooling is performed.
Every emitted comparison also carries `sample_size_status`. Ten rows per side
is only a declared descriptive floor, not an empirical calibration: retained
strata with at most 15 rows per side are marked `uncalibrated_descriptive`, and
all statuses carry disposition
`descriptive_only_do_not_treat_as_estimate_or_ranking`. This is a machine-
readable warning, not a claim that the metric is mathematically invalid.

## Dispositions and interpretation boundary

The surviving subset is broad enough to expose different failure modes while
remaining dependency-free. Full optimal-transport Wasserstein was deferred
because an exact solver would add substantial backend cost to this bounded
pass. A learned classifier was deferred in favour of grouped nearest-centroid
and kNN diagnostics, which preserve the honest held-out-group contract without
adding scikit-learn. High-dimensional occupancy grids were deferred because
their bins become sparse and unstable; the retained 1-D occupancy diagnostic
is an explicit negative-control view. No single metric is a justified ranking
score.

True cyclic/dihedral or optimal vertex-assignment distances were also deferred:
the retained chord-multiset view is deliberately a lossy, adjacency-free
surrogate and must not be presented as an assignment optimizer.

If raw or canonicalized distances are large but chord-multiset-quotient
distances collapse, report **representation-induced separation**. If all views separate,
the result is still only target-free geometry evidence at the observed sample
size and side-count strata; this packet has no empirical calibration for
metric estimates. The table says which kind of difference each
method detects; it does not establish natural-law coverage, transfer to `sys`,
causality, or a population-level generator ranking.

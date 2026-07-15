# Within-distribution quality diagnostics

This packet is a target-free, one-distribution assessment surface. It answers
“what does this generator explore at the observed sample size?” without
calling `sys`, pooling side counts, or ranking laws. The input adapter accepts
the existing `factor-shape-row-v1` contract and the local `shape-row-v1` alias;
a future generator can copy the two small validation functions in
`analyze.py` and emit cyclic CCW vertices plus a stable `sample_id`, population
(including knob settings), and side count.

## Two explicit geometry views

Every same-population/side-count stratum is analyzed separately.

* `raw_ordered` area-normalizes and centroid-centers vertices while preserving
  the producer's cyclic start index, orientation, and vertex order. It exposes
  representation redundancy and row-order artifacts.
* `frame_adjusted` minimizes RMS vertex distance over cyclic shifts and an
  orientation-preserving planar rotation. It removes start-index and frame
  choices but does not reflect or identify arbitrary affine maps.

Their disagreement is retained as information. It is not a reason to declare
one representation “correct”. Both distance matrices report pair and nearest-
neighbor distributions, exact/near duplicate rates, greedy k-center radius
curves, a distance-threshold cluster balance, subsampling saturation, and a
leave-one-influential-point outlier sensitivity check. Pair and frame costs are
`O(n^2 * side_count)`; no hidden target or shared pair-metric import is used.

The discrete view uses a producer-supplied `combinatorial_cell`/`f_vector` when
available, and otherwise bins side count and edge-length coefficient of
variation. Fixed-side planar polygons have one combinatorial type, so the
fallback is explicitly a geometric view-cell rather than an invented
combinatorial distinction. Plugin entropy, effective number, occupancy,
singleton Good--Turing unseen-mass diagnostics, and their explicit `n < 50`
warning describe finite-sample discovery only. They do not estimate support
probability or population mass.

`rare_region_discovery` freezes a target-free scalar tail rule (edge-length CV
at the calibration-half 90th percentile), then evaluates the second half of
the input-order stream as a split holdout. It records time-to-first-hit, block
hit probability, distinct tail signatures, and attempted/accepted/independent-
block costs. Attempted/accepted costs are non-null only for rows explicitly
declaring `cost_semantics=counts-v1` with numeric count fields; producer
attempt indices and Boolean `accepted` flags are not counts. A zero-hit bound
and independent-block cost/count are null unless every row declares
`independence_semantics=independent-block-v1` with an independence-unit ID.
Contiguous block counts remain plain non-inferential diagnostics. This scalar
event is deliberately separate from geometric support coverage; it is not a
`sys` proxy.

## Synthetic calibration

The retained fixture covers identical, concentrated, broad, multimodal,
duplicated, imbalanced, contaminated-outlier, rare-mixture, and dependent-
duplicate clouds. It calibrates the qualitative behavior expected from each
summary: transformed copies disagree in `raw_ordered` but collapse in
`frame_adjusted`; broad and multimodal clouds increase coverage and effective
number; duplicates increase duplicate/nearest-neighbor concentration; an
imbalanced mixture can have high support coverage but a dominant cluster; and
two dependent outliers can defeat a naive “largest nearest-neighbor” rule, so
the implementation uses leave-one-out influence instead. The rare-mixture and
dependent-duplicate cases make passive-search limits visible rather than
pretending independent draws.

Run the retained synthetic calibration from this directory:

```bash
uv run --script analyze.py \
  --write-synthetic-fixture fixtures/synthetic.jsonl \
  --input fixtures/synthetic.jsonl \
  --out-dir artifacts/synthetic
uv run --script test_analyze.py
```

`artifacts/synthetic/report.json` is generated source data; `summary.tsv` is a
compact investigation table. Reports include deterministic input and analyzer
source SHA-256 values, a source revision/dirty-state contract, command
template, and seed. Regenerate, do not hand-edit them.

## Real smoke

The small real smoke consumes the accepted generator-zoo factor population,
without target fields:

```bash
uv run --script analyze.py \
  --input ../generator-zoo-smoke/artifacts/factor-shapes.jsonl \
  --out-dir artifacts/real-smoke
```

The smoke remains stratified by the exact `population` label (law plus knob)
and `side_count`; it therefore does not pool facet counts or parameter levels.
Its rows are a plumbing/descriptive check, not a natural-law estimate or
population ranking. Missing attempts/acceptance metadata are reported with the
fallback cost of one accepted row per input row.

## Interpretation boundary and dispositions

Useful claims are conditional on the emitted rows, strata, and finite `n`:
support coverage, probability-mass concentration, finite-sample discovery, and
feature variance are different objects. No p-values, confidence claims,
population rankings, or downstream-`sys` mechanism claims are authorized.

Implemented: raw/frame pair and nearest-neighbor views; duplicate/near-
duplicate rates; heuristic k-center coverage; coarse occupancy,
entropy/effective number, and Good--Turing singleton diagnostics; saturation;
cluster/mode balance; outlier influence; and input-order split-holdout
rare-region discovery/cost curves.

Deferred: certified packing numbers (the k-center curve is a bounded heuristic),
population-level unseen-mass inference, formal model-selected clustering, and
inferential tests. Reopen those only with a named downstream decision and a
larger, independent sample contract.

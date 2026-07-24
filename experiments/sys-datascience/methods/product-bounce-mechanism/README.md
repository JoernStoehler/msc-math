# Product bounce mechanism: existing-row decomposition

## Question

This packet is the cheap precursor to any active-facet resampling study. It asks
how the retained within-`(k,m)` association between the producer's two-/three-
bounce winner label and `sys` splits into unscaled capacity and volume, how
sensitive that split is to existing generator/shape controls, and whether
active-support counts or two-/three-class balance support the proposed
mechanisms.

It performs no new capacity or geometry evaluation. The bounce label and class
minima are post-target and target-derived, so every result here is descriptive.

## Inputs and command

Hydrate the two reviewed LFS inputs and rebuild the reviewed current-schema
prepared table:

```bash
git lfs checkout -- \
  experiments/polytope-datasets/random-product.jsonl \
  experiments/sys-datascience/methods/product-bounce-distribution/artifacts/class-minima.jsonl

TABLES_DIR="$(mktemp -d /tmp/sys-ds-product-bounce-mechanism.XXXXXX)"
experiments/polytope-invariant-table/build-random-only-slice.sh full "$TABLES_DIR"

python3 experiments/sys-datascience/methods/product-bounce-mechanism/analyze.py \
  --raw experiments/polytope-datasets/random-product.jsonl \
  --classes experiments/sys-datascience/methods/product-bounce-distribution/artifacts/class-minima.jsonl \
  --table "$TABLES_DIR/polytope-table.jsonl" \
  --provenance "$TABLES_DIR/polytope-provenance-table.jsonl" \
  --out experiments/sys-datascience/methods/product-bounce-mechanism/artifacts/summary.json
```

The analyzer requires the four reviewed SHA-256 identities. It joins raw and
class rows one-to-one by source name, joins the prepared table through its
provenance `source_name`/`poly_id`, and aborts on duplicate or missing rows.

## Measured objects

For the same fixed-effects design, the analyzer fits the exact identity

```text
log(sys) = 2 log(capacity) - log(2 volume)
```

and reports the three-bounce coefficient for each term. Controls are
standardized within exact `(k,m)` buckets. The primary sensitivity sets are:

- no controls beyond `(k,m)`;
- original-generator coordinates recoverable from stored dual rows: q/p mean
  and standard deviation of log support height, plus q/p minimum angular gap;
- the volume-free ridge-distribution summaries normalized entropy and maximum
  share;
- both sets together.

The prior three-ridge adjustment and a ridge-magnitude sensitivity are retained
for continuity. The latter uses a feature normalized by `sqrt(volume)`, so it
must not be read as an independent volume-mechanism adjustment.

The class-balance check uses `abs(log(A3/A2))` only where both reviewed class
minima exist. Unlike `min(s2,s3)`, closeness of the action ratio to one is not
algebraically forced by high `sys`.

## Observations

- On all 10,240 retained rows, the within-bucket three-bounce log-`sys`
  coefficient is `0.8452`. Its exact component split is `0.2086` from
  `2 log(capacity)` and `0.6366` from `-log(2 volume)`: 24.7% versus 75.3%.
- Original-generator coordinate controls reduce the coefficient to `0.7426`,
  split 31.8% capacity and 68.2% volume. Adding the two volume-free ridge-
  distribution controls reduces it to `0.5705`, split 43.5% capacity and 56.5%
  volume.
- The complete-support overlap sensitivity uses 8,737 rows and removes `3x3`,
  whose 718 complete rows are all three-bounce-labelled. Its raw split is
  27.3% capacity and 72.7% volume. With generator plus ridge-distribution
  controls the remaining `0.4508` coefficient is approximately even: `0.2242`
  capacity and `0.2266` volume.
- Every retained global winner in both label classes has a six-facet support,
  split exactly `3q+3p`. Thus the number of inactive facets is `k+m-6` for both
  labels and is fixed by the bucket. The 321 rows with an eight-facet A3
  minimizer are all two-bounce global winners; the eight-facet word never wins.
- Among the 9,455 complete rows, the pooled within-bucket top decile has median
  `abs(log(A3/A2)) = 0.0813`, versus `0.0428` outside it. The near-tie rate
  `abs(log(A3/A2)) <= 0.01` is 11.3% versus 21.1%. The within-bucket Spearman
  association between gap and `sys` is positive in nine buckets and slightly
  negative only in `6x6`.
- The maximum stored `sys` remains the known `0.86258589584944`; this packet
  contains no `sys > 1` row.

Detailed per-bucket decompositions, every adjustment, row counts, and the input
identities are generated in `artifacts/summary.json`.

## Interpretation

The unscaled decomposition is meaningful for this generator: all rows in a
bucket share the same coordinate convention and support-height law, and the
log identity is scale-unit invariant. It is not causal mediation. The producer
label is the sign of lower-envelope competition, and the controls are observed
shape summaries rather than interventions.

The evidence rejects the simple count version of inactive-facet freedom: two-
bounce winners do not leave more facets inactive than three-bounce winners.
Volume remains a material part of the association, but it is not robustly the
unique dominant part after shape controls. Effective constraint rank or fibre
dimension was not measured and remains an explicit resampling-design check;
equal active-facet counts alone do not settle it.

The retained upper tail is not organized by near two-/three-class balance.
This weakens the proposed HKO-intersection explanation on this generator. The
persistent capacity term leaves a two-bounce width/difference-body shortcut and
generic lower-envelope selection live; this packet does not expose an
independent pre-target width statistic that could distinguish them.

## Allowed and prohibited use

Allowed: use this packet to design or stop a bounded conditional-resampling or
width-mechanism experiment, preserving its retained-generator and observational
scope.

Not allowed: causal inactive-facet freedom, mediation by ridge features, a
generic two-/three-bounce theorem, an independent proposer, or extrapolation to
another generator. A3-null rows are availability states under the existing
candidate-stream contract and are never assigned a numeric class gap.

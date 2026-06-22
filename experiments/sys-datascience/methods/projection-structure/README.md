# projection-structure

## Research Question

What low-dimensional projection, cluster, and anomaly structure is visible in
geometry-only features of the trusted random/product rows, and does any of it
merit a separate proposer experiment?

## Method

Use geometry-only retained features. Standardize features, run PCA, summarize
k-means clusters for several `k`, compare isolation-forest anomaly scores with
the top `sys` tail, and record source/facet/product metadata overlays for PCA
coordinates and cluster composition. The metadata overlays are diagnostics for
source stratification, not geometry features used to construct the projection.

## Inputs

- trusted random-only rows from `../trusted-random-dataset/`

## Command

```bash
uv run --script experiments/sys-datascience/methods/projection-structure/analyze.py
```

## Retained Artifacts

- `artifacts/summary.json`
- `artifacts/pca-sys.png`

## Observation

Feature-space closure branch note: the script now defaults to all eligible
geometry features instead of the old `80`-feature cap, and the shared feature
selector includes new `omega_*` geometry columns plus two-face
symplectic-area tail columns when all loaded rows have complete two-face
ordering. Two-face ordering diagnostics are excluded from geometry inputs. The
script now records metadata overlays for source/facet/product labels. The
current artifact below was regenerated with this branch's method code against
the full scoped random/product prepare output at `/tmp/sys-ds-random-only-full`,
built with `sys-dataset --random-only`.

Current full scoped random/product run:

- rows: `14336`;
- geometry features: `109`;
- first five PCA explained-variance ratios:
  `0.351133867446898`, `0.2097027842622585`,
  `0.17838138619847757`, `0.05255241762135098`,
  `0.03744208973362901`;
- PC1/`sys` correlation: `-0.4636669884808957`;
- PC2/`sys` correlation: `0.2283293325852428`;
- top 25 isolation-forest anomaly rows overlap with top 2% `sys` rows: `0`.
- metadata overlays are recorded for `capacity_source`, `dataset_label`,
  `dataset_label_by_facet_count`, `facet_count`, `product_bucket`, and
  `sample_height_range`.

K-means clusters separate some low/high-tail mass. The isolation-forest anomaly
rows do not overlap the top `sys` tail in this run. This is in-table
exploratory evidence only; no cluster or anomaly rule was validated as a
candidate-proposer.

## Validity Guards

- This packet uses in-table exploratory structure.
- Metadata overlays are source-stratification diagnostics. They are not used to
  build the geometry projection and are not candidate-proposer evidence.
- An anomaly score or cluster label is not a candidate-proposer unless it ranks
  held-out or newly generated unevaluated polytopes before `sys` is computed.

## Current Disposition

Exploratory structural evidence only. No validated candidate-proposer is
recorded by this packet.

## Remaining Worthwhile Questions

Only follow up if a cluster/projection rule cleanly separates high tail rows
using geometry-only columns after source/facet/product overlays are checked.

## Predicted Stability Under Rerun

High on unchanged retained tables.

## Thesis Use

Supports a statement about in-table projection, clustering, and anomaly checks
on geometry-only features. It does not support a generated-candidate or
held-out proposer claim.

## Reopen Triggers

- retained table columns change;
- a new random-only dataset is added;
- a projection rule is promoted into a generated-candidate experiment.

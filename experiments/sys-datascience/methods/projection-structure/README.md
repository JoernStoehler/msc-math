# projection-structure

## Research Question

What low-dimensional projection, cluster, and anomaly structure is visible in
geometry-only features of the trusted random/product rows, and does any of it
merit a separate proposer experiment?

## Method

Use geometry-only retained features. Standardize features, run PCA, summarize
k-means clusters for several `k`, and compare isolation-forest anomaly scores
with the top `sys` tail.

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

Current run on hydrated retained tables in this branch:

- rows: `14336`;
- geometry features: `80`;
- first five PCA explained-variance ratios:
  `0.3898206499072291`, `0.22221957682055385`,
  `0.15963593059038905`, `0.056606354072659314`,
  `0.04365248795758812`;
- PC1/`sys` correlation: `-0.46871789993085916`;
- PC2/`sys` correlation: `0.21779070089156527`;
- top 25 isolation-forest anomaly rows overlap with top 2% `sys` rows: `0`.

K-means clusters separate some low/high-tail mass. The isolation-forest anomaly
rows do not overlap the top `sys` tail in this run. This is in-table
exploratory evidence only; no cluster or anomaly rule was validated as a
candidate-proposer.

## Validity Guards

- This packet uses in-table exploratory structure.
- An anomaly score or cluster label is not a candidate-proposer unless it ranks
  held-out or newly generated unevaluated polytopes before `sys` is computed.

## Current Disposition

Exploratory structural evidence only. No validated candidate-proposer is
recorded by this packet.

## Remaining Worthwhile Questions

Only follow up if a cluster/projection rule cleanly separates high tail rows
using geometry-only columns.

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

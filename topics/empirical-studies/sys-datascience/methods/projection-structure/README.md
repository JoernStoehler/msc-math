# projection-structure

## Research Question

What low-dimensional projection, cluster, and anomaly structure is visible in
active invariant features of the trusted random/product rows, and does any of it
merit a separate proposer experiment?

## Method

Use active invariant retained features. Standardize features, run PCA, summarize
k-means clusters for several `k`, compare isolation-forest anomaly scores with
the top `sys` tail, and record source/facet/product metadata overlays for PCA
coordinates and cluster composition. The metadata overlays are diagnostics for
source stratification, not invariant features used to construct the projection.

## Inputs

- trusted random/product rows from `../trusted-random-dataset/`

## Command

```bash
uv run --script topics/empirical-studies/sys-datascience/methods/projection-structure/analyze.py
```

## Retained Artifacts

- `artifacts/summary.json`

The script can generate diagnostic plots during reruns, but only the compact
JSON summary is retained in git for the current packet.

## Schema and rerun boundary

The retained summary below used the registered 39-feature invariant snapshot.
Today's shared selector requires 45 numeric features, so the no-argument
command rejects that snapshot. Pass `--tables-dir` pointing to a deliberate
current-schema rebuild when rerunning, and use a fresh `--out-dir` to preserve
the frozen result. [P2](../standard-baseline-p2/README.md) owns the recorded
45-feature rebuild contract. Its six additional ridge columns can change the
result; the existing numbers are not evidence of an updated run.

## Observation

Recorded integrated full retained run (39-feature snapshot):

- input: retained invariant table under `topics/empirical-studies/polytope-invariant-table`, `14336` trusted
  random/product rows;
- invariant features: `39`;
- PC1/`sys` correlation: `0.4644434019289053`;
- top-25 isolation-forest anomaly rows overlapping the top `2%` `sys` rows:
  `0`.

The projection packet records selected invariant features, large PCA loadings,
source/facet/product metadata overlays, k-means cluster summaries, and
isolation-forest anomaly overlap with high `sys` tails.

Any cluster, projection, or anomaly pattern is in-table exploratory evidence
only unless promoted to a separate proposer experiment that ranks unevaluated
rows before `sys` is computed.

## Validity Guards

- This packet uses in-table exploratory structure.
- Metadata overlays are source-stratification diagnostics. They are not used to
  build the invariant-feature projection and are not candidate-proposer
  evidence.
- An anomaly score or cluster label is not a candidate-proposer unless it ranks
  held-out or newly generated unevaluated polytopes before `sys` is computed.

## Current Disposition

Exploratory structural evidence only. No validated candidate-proposer is
recorded by this packet.

## Remaining Worthwhile Questions

Only follow up if a cluster/projection rule cleanly separates high tail rows
using invariant-feature columns after source/facet/product overlays are checked.

## Predicted Stability Under Rerun

High on unchanged retained tables.

## Thesis Use

Supports a statement about in-table projection, clustering, and anomaly checks
on invariant features. It does not support a generated-candidate or
held-out proposer claim.

## Reopen Triggers

- retained table columns change;
- a new random/product dataset is added;
- a projection rule is promoted into a generated-candidate experiment.

# DS-I004 PCA / Clustering / Anomaly Source-Truth Repair

## Command / Provenance

- Script: `experiments/sys-landscape/datascience/methods/pca-cluster-spike/analyze.py`
- Dataset dir: `experiments/sys-landscape/datascience/dataset`
- Output dir: `experiments/sys-landscape/datascience/methods/pca-cluster-spike`
- Producer command recorded in packet: `experiments/sys-landscape/datascience/build-dataset.sh`
- Git commit: `af976dbf`
- Random state: `20260430`

## Dataset Snapshot And Guards

- Polytope rows: `282` expected `282`.
- Observation rows: `282` expected `282`.
- Max `sys`: `0.906316153431123` expected `0.906316153431123`.
- `sys > 1` count: `0` expected `0`.
- Guard status: `passed`.

## Feature Scope

- Fitted feature count: `99` nonconstant numeric columns.
- Candidate numeric polytope features before constant drop: `110`.
- Constant columns dropped: `10`.
- Excluded classes:
  - target and capacity columns: sys, capacity, capacity_source
  - raw vertex arrays and id columns
  - sigma/orbit-search witness columns tied to capacity search
  - all observation metadata, endpoint labels, dataset identity, and optimizer provenance
- The fitted matrix uses no observation-table columns.

## Observations

- PCA: component `1` has the largest absolute correlation with `sys`, `|corr| = 0.758`; PC1 explains `0.386` of standardized feature variance.
- PCA top absolute-score rows for that component have mean `sys = 0.213` and max `sys = 0.833`.
- KMeans: silhouette selects `k = 2` with silhouette `0.412`.
- KMeans best mean-`sys` cluster has `234` rows, mean `sys = 0.564`, max `sys = 0.906`, top-decile high-`sys` rate `0.124`, dominant dataset `variable_f_ascent` at share `0.385`, and regime counts `{'endpoint': 112, 'random': 122}`.
- Across `k = 2..8`, the highest mean-`sys` cluster has `104` rows, mean `sys = 0.737`, dominant dataset `variable_f_ascent` at share `0.721`, and regime counts `{'endpoint': 85, 'random': 19}`.
- Across `k = 2..8`, the highest top-decile high-`sys` rate is `0.267` in a `15`-row cluster with dominant dataset `variable_f_ascent` at share `1.000`.
- IsolationForest: `29` anomalies have mean `sys = 0.099` versus normal mean `sys = 0.529`; anomaly max `sys = 0.693`.

## Inference

The PCA/clustering/anomaly methods see structure in the retained dataset, but the fitted structure is not by itself a candidate-proposer. The strongest PCA diagnostic is a correlation with `sys`, which is an audit statistic rather than a sampling rule. The silhouette-selected cluster split is broad rather than a targeted high-`sys` rule; higher-k clusters with stronger high-`sys` concentration are endpoint/dataset-heavy when inspected after fitting. The anomaly rule does not enrich for high `sys` relative to the rest of the table. A positive follow-up would need to turn intrinsic feature loadings or cluster geometry into a sampling rule specified before inspecting `sys`, endpoint labels, dataset identity, or optimizer provenance.

## Verdict

- `verdict`: `no-search-output`
- `evidence_strength`: `medium`
- `implementation_trust`: `high`
- `thesis_use`: `supporting/caveat only`
- `caveat`: This is a 282-row retained-dataset scan over nonconstant intrinsic numeric polytope features; it excludes observation provenance and capacity/search witness columns, and it tests only PCA, KMeans k=2..8, and IsolationForest at 10 percent contamination.
- `reopen_trigger`: Reopen if a larger or fresher table adds sys > 1, changes the row guards, or a sampling rule is proposed that can sample a feature-space region before inspecting sys, endpoint labels, dataset identity, or optimizer provenance.

## Reproducibility

```bash
uv run --script experiments/sys-landscape/datascience/methods/pca-cluster-spike/analyze.py --dataset-dir experiments/sys-landscape/datascience/dataset --out-dir experiments/sys-landscape/datascience/methods/pca-cluster-spike
```

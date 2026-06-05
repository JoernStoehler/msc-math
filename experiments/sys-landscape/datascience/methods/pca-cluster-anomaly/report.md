# PCA / Clustering / Anomaly

Status: needs current retained-dataset rerun.

This report path is current as a status marker, not as evidence. The previous
report used the old pre-LICCA `282`-row dataset and was removed from current
source truth.

Run:

```bash
uv run --script experiments/sys-landscape/datascience/methods/pca-cluster-anomaly/analyze.py --dataset-dir experiments/sys-landscape/datascience/dataset --out-dir experiments/sys-landscape/datascience/methods/pca-cluster-anomaly
```

Current method-table role:

- PCA, clustering, and anomaly scan over retained table columns;
- checks whether an unsupervised or projection rule becomes a candidate-proposer;
- current thesis use is pending rerun or explicit abandonment from current data.

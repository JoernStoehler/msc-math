# Exact-vs-f64 Spot Check

Status: needs current retained-dataset rerun.

This report path is current as a status marker, not as evidence. The previous
report used the old pre-LICCA `282`-row dataset and was removed from current
source truth.

Run:

```bash
uv run --script experiments/sys-landscape/datascience/methods/exact-f64-spot-check/analyze.py --dataset-dir experiments/sys-landscape/datascience/dataset
```

Expected input:

- `experiments/sys-landscape/datascience/dataset/polytope-table.jsonl`
- `experiments/sys-landscape/datascience/dataset/observation-table.jsonl`

Current method-table role:

- sanity row for exact rational coordinates versus stored f64 geometry columns;
- supporting evidence only;
- not a candidate-proposer.

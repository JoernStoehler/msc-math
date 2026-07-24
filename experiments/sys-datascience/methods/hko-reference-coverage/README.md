# HKO Reference Coverage

Question: does the known HKO `sys > 1` pentagon sit inside, near, or outside
the retained random/product invariant-feature support?

This packet treats HKO as an `N=1` reference/holdout row. It loads retained
random/product rows through the shared trusted-random filter and loads HKO
through the reference-holdout filter. HKO is scored against the random/product
feature cloud; it is not used to fit thresholds, summaries, or training inputs.

Run after preparing a reference directory:

```bash
uv run --script experiments/sys-datascience/methods/hko-reference-coverage/analyze.py \
  --random-tables-dir experiments/polytope-invariant-table \
  --reference-tables-dir /tmp/ds-prepare-hko \
  --out-dir /tmp/hko-reference-coverage
```

Outputs:

- `summary.json`: machine-readable distances, ranks, nearest rows, high-tail
  rows, and ridge-area score.
- `report.md`: compact readable packet with the same values.

Interpretation rule:

- HKO is `outside` if its nearest standardized invariant-feature distance is
  beyond the random/product 99th percentile nearest-neighbor scale.
- HKO is `near the boundary` if it is beyond the 95th percentile scale.
- Otherwise, HKO is reported as inside the retained random/product cloud.

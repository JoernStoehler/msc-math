# Local Behavior Prediction

Draft method packet for local and semi-local `sys(a)` prediction diagnostics.

Status: exploratory. This packet reads run-local prepared outputs from
`experiments/sys-datascience/tables/prepare-local-behavior.py`; it does not
own capacity search or reusable joins.

Typical local flow from repo root:

```bash
cargo run --release -p exp-sys-landscape --bin sys-local-behavior-produce -- \
  --out-dir /tmp/sys-local-behavior-smoke \
  --max-top-basepoints 1 --max-hash-basepoints 0 \
  --random-directions 1 \
  --radii 1e-6,1e-3

uv run --script experiments/sys-datascience/tables/prepare-local-behavior.py \
  /tmp/sys-local-behavior-smoke

uv run --script experiments/sys-datascience/methods/local-behavior-prediction/analyze.py \
  /tmp/sys-local-behavior-smoke/prepared
```

The report and figures are written under
`/tmp/sys-local-behavior-smoke/prepared/local-behavior-prediction/` by default.

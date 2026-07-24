# high-sys-tail-diagnostic

Purpose: investigate the high-`sys` tail by fixed bucket, separately from the
body of the marginal distribution.

The method fits simple excess-tail models above empirical bucket thresholds.
For threshold `u`, the modeled variable is `x = sys - u | sys > u`.

Models:

- `gpd`: generalized Pareto distribution with `loc = 0`;
- `exponential`: the `gpd` special case with shape `c = 0`.

For GPD fits with negative shape, the fitted upper endpoint is
`u - scale / shape`. This is only an exploratory endpoint estimate, not a proof
that the real distribution has that endpoint.

Command:

```bash
uv run --script experiments/sys-datascience/methods/high-sys-tail-diagnostic/analyze.py \
  --tables-dir experiments/polytope-invariant-table \
  --out-dir /tmp/sys-ds-tail-20260625/high-sys-tail \
  --bootstrap-count 200
```

Outputs:

- `tail-pattern-summary.md`: compact human-readable pattern summary;
- `tail-fit-table.tsv`: all bucket/threshold tail fits;
- `tail-fit-readable.md`: human-readable table with endpoint and likelihood
  gaps;
- `bucket-tail-summary.tsv`: bucket-level top quantiles and maxima;
- `gpd-endpoint-bootstrap.tsv`: bootstrap endpoint diagnostics;
- `summary.json`: machine-readable summary;
- `figures/generic-survival.png`;
- `figures/product-survival.png`;
- `figures/endpoint-by-bucket.png`.

Current scratch run:

- artifact directory: `/tmp/sys-ds-tail-20260625/high-sys-tail`;
- status in `summary.json`: `high_sys_tail_diagnostic`;
- 18 fixed buckets: 8 generic buckets with 512 rows each and 10 product
  buckets with 1024 rows each;
- thresholds: empirical q80, q90, and q95 per bucket;
- tail-fit rows: 108 table rows, covering GPD and exponential fits for each
  bucket/threshold cell;
- endpoint bootstrap count: 200.

No compact artifact is currently retained in this packet directory. Rerun and
promote selected outputs deliberately before using this as thesis-facing
evidence.

Interpretation guard: with 512 generic rows and 1024 product rows per bucket,
the top 5% contains only 26 or 52 points. Use the table for patterns and
follow-up design, not for final tail-risk claims. The current pattern summary
reports that GPD excess-tail fits beat exponential fits in every fitted
bucket/threshold cell and that many fitted GPD shapes are negative. Treat this
as exploratory evidence about the sampled rows only, not as a proof of a
bounded real distribution or of `P(sys > 1) = 0`.

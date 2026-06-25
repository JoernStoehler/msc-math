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
  --tables-dir /workspaces/msc-math/experiments/sys-datascience/prepare \
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

Interpretation guard: with 512 generic rows and 1024 product rows per bucket,
the top 5% contains only 26 or 52 points. Use the table for patterns and
follow-up design, not for final tail-risk claims.

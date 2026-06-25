# sys-distribution-mle-likelihood-table

Purpose: for each fixed bucket and candidate transform/family model, report
approximate maximum-likelihood parameters and same-data likelihoods.

A model has the form `transform+distribution`. The likelihood is always scored
as a density on the original `sys` scale, including the Jacobian term for the
transform. Therefore `logit+norm`, `square+beta`, and `neglog1m+gamma` are
directly comparable when fitted to the same bucket rows.

This table answers a different question from held-out prediction. For two
models fitted on the same rows, the log-likelihood difference gives the maximum
possible likelihood-ratio evidence for the model with the larger MLE
likelihood, before any prior/integrated-parameter penalty. This is useful as an
exploratory upper bound on Bayesian updating between named transform/family
models.

Command:

```bash
uv run --script experiments/sys-datascience/methods/sys-distribution-mle-likelihood-table/analyze.py \
  --tables-dir /workspaces/msc-math/experiments/sys-datascience/prepare \
  --out-dir /tmp/sys-ds-shape-20260625/mle-transform-table \
  --max-fit-seconds 3
```

Outputs:

- `mle-likelihood-table.tsv`: all bucket/model rows;
- `bucket-best-summary.tsv`: best model per bucket and log2 gaps to selected
  reference models when present;
- `model-summary.tsv`: per-model bucket counts, non-ok buckets, best buckets,
  and worst/median/best log2 likelihood ratio versus the bucket winner;
- `model-bucket-log2-gap-matrix.tsv`: compact model-by-bucket matrix of log2
  likelihood ratios versus each bucket winner, with non-ok status labels;
- `model-readable-summary.md`: human-readable summary using wins and
  within-5/within-20-bit bucket counts;
- `model-bucket-readable-gap-matrix.md`: human-readable matrix with `best`,
  `~best`, numeric gaps, `no`, and `non-ok`;
- `bucket-best-readable-summary.md`: human-readable bucket winner table with
  selected reference models;
- `summary.json`: same information in JSON.

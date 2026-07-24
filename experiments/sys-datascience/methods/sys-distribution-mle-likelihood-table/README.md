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
  --tables-dir experiments/polytope-invariant-table \
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

Current scratch run:

- artifact directory: `/tmp/sys-ds-shape-20260625/mle-transform-table`;
- status in `summary.json`: `all_data_approximate_mle_transformed_likelihood_table`;
- 14336 rows across 18 fixed buckets;
- 8 transforms: `identity`, `logit`, `log`, `log1m`, `neglog1m`, `sqrt`,
  `square`, and `cloglog`;
- 80 SciPy distributions, giving 640 candidate transform/family models;
- 11520 model-table rows plus header in `mle-likelihood-table.tsv`;
- bucket winners are mixed across transforms/families, with `square+mielke`
  winning the most buckets in this run.

No compact artifact is currently retained in this packet directory. Rerun and
promote selected outputs deliberately before using this as thesis-facing
evidence.

Interpretation guard: this is an all-data approximate MLE likelihood table.
Use it as an exploratory upper bound on same-row model discrimination, not as
held-out predictive evidence and not as a claim that any winning family is the
true distribution.

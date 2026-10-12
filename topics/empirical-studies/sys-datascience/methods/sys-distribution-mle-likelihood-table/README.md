# sys-distribution-mle-likelihood-table

Purpose: for each fixed bucket and candidate transform/family model, report
approximate maximum-likelihood parameters and same-data likelihoods.

A model has the form `transform+distribution`. The score includes the Jacobian
term for the transform, so it is expressed on the original `sys` scale. The implementation fits the transformed
observations with unconstrained SciPy location/scale parameters. It does not
condition each family on the transform's image of `(0,1)`: for example, a
`square+beta` fit may place mass outside `(0,1)` after fitted location/scale,
and a `neglog1m+gamma` fit may place mass below zero. Such scores are not
normalized densities on the declared bounded `sys` domain. Cross-transform
comparisons are exploratory scores until that support contract is repaired.

This table answers a different question from held-out prediction. Differences
between same-row fitted scores compare the selected numerical fits. SciPy
optimization is approximate and does not guarantee global likelihood maxima.
A ratio of maximized likelihoods is neither a Bayes factor nor an upper bound
on a Bayes factor: marginal likelihoods integrate each model against its own
parameter prior, and their ratio need not obey the ratio of the two maxima.
No Bayesian update or posterior model probability is computed here.

Command:

```bash
uv run --script topics/empirical-studies/sys-datascience/methods/sys-distribution-mle-likelihood-table/analyze.py \
  --tables-dir topics/empirical-studies/polytope-invariant-table \
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
Use it as exploratory same-row score discrimination, subject to the support
normalization caveat above. It is not held-out predictive evidence, Bayesian
evidence, or a claim that any winning family is the true distribution.

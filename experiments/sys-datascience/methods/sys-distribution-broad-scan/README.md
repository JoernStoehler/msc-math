# sys-distribution-broad-scan

Purpose: broad exploratory distribution screen for fixed-bucket marginals of
`sys(a)`.

This replaces the too-small hand-picked candidate-family passes as a proposal
generator. It is exploratory and post-hoc. Do not use it to rule families out,
prove a fit, or make confirmatory statistical claims.

Main idea:

- `sys` lives in `(0,1)`;
- fit many SciPy continuous distributions to `logit(sys)`;
- transform each fitted distribution back to `(0,1)`;
- compare by held-out log likelihood and CDF residuals;
- record fit failures instead of silently shrinking the family set.

Command:

```bash
uv run --script experiments/sys-datascience/methods/sys-distribution-broad-scan/analyze.py \
  --tables-dir /workspaces/msc-math/experiments/sys-datascience/prepare \
  --out-dir /tmp/sys-ds-shape-20260625/broad-scan \
  --max-fit-seconds 2
```

Current local run:

- 14336 trusted random/product rows;
- 18 buckets;
- 80 logit-transformed SciPy candidate families;
- 0.25 test fraction with RNG seed `20260625`;
- up to 2 seconds per fit;
- fit/failure tables:
  - `/tmp/sys-ds-shape-20260625/broad-scan/tables/family-failure-summary.md`
  - `/tmp/sys-ds-shape-20260625/broad-scan/tables/family-failure-summary.tsv`
  - `/tmp/sys-ds-shape-20260625/broad-scan/tables/family-bucket-fit-matrix.tsv`
- ECDF overview plots:
  - `/tmp/sys-ds-shape-20260625/broad-scan/overview/generic-broad-scan-ecdf.png`
  - `/tmp/sys-ds-shape-20260625/broad-scan/overview/product-broad-scan-ecdf.png`

Interpretation: this scan is broad enough to avoid the earlier artificial
three/seven-family bottleneck. It is not a claim that the held-out winner is
mathematically meaningful; many best-vs-second gaps are tiny.

Screening-table convention:

- `technical_fail`: SciPy fitting/evaluation failed or timed out.
- `cdf_flag`: max absolute ECDF/CDF error exceeds the bucket's 95% DKW band in
  this exploratory all-data diagnostic.
- `weak`: the error is within the DKW band but above 75% of that band.
- `ok`: the error is at most 75% of the DKW band.

The DKW comparison here is a screening diagnostic, not a formal goodness-of-fit
test after model selection. Evaluation claims would need a separate held-out
packet; no such packet is active here.

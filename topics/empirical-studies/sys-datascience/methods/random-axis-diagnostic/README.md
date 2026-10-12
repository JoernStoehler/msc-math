# random-axis-diagnostic

Purpose: diagnose whether a candidate random-distribution axis is useful enough
to guide source-dataset producer design.

Current use: height interval `[h_min,h_max]`. The diagnostic compares its effect
on `sys`, invariant-feature distributions, and bucket/family structure. It also
reports matched-source top-tail overlap when provenance permits it, but seed is
only reproducibility metadata, not a random axis.

Uncertainty should come from the sampled rows: effect sizes, distribution
statistics, and bootstrap intervals. Do not report seed effects or ask whether
the PRNG seed is meaningful.

Interpretation boundary: this method is exploratory. It ranks axes and proposes
next tests; it does not certify distributional equivalence from non-significant
p-values.

## Current Method-Sized Check

The integrated branch ran a method-sized two-height check:

- input: `/tmp/ds-height-method-prepare`;
- rows: `1152`;
- buckets: all generic `F=5..12` and all product pairs `3<=k<=m<=6`;
- intervals: `[0.8,1.2]` and `[0.6,1.4]`;
- rows per producer bucket per interval: `32`;
- max `sys`: `0.8304105991003154`;
- `sys > 1`: `0`;
- height-interval effect on `sys`: eta-squared `0.003467165839432387`,
  Kruskal p-value `0.03207805986746921`;
- dataset-family effect on `sys`: eta-squared `0.022284083423394482`;
- bucket effect on `sys`: eta-squared `0.21317406426785865`;
- median height-interval effect across invariant features:
  `8.939809940815113e-05`.

Interpretation: height interval is visible in this sample, but the observed
effect is small compared with product/facet bucket structure. This is not a
final thesis-scale distribution-sensitivity result.

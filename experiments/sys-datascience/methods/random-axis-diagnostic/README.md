# random-axis-diagnostic

Purpose: diagnose whether a candidate random-distribution axis is useful enough
to guide `produce/` design.

Current use: height interval `[h_min,h_max]`. The diagnostic compares its effect
on `sys`, geometry-feature distributions, and bucket/family structure. It also
reports matched-source top-tail overlap when provenance permits it, but seed is
only reproducibility metadata, not a random axis.

Uncertainty should come from the sampled rows: effect sizes, distribution
statistics, and bootstrap intervals. Do not report seed effects or ask whether
the PRNG seed is meaningful.

Interpretation boundary: this method is exploratory. It ranks axes and proposes
next tests; it does not certify distributional equivalence from non-significant
p-values.

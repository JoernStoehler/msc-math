# distribution-sensitivity

Purpose: compare several random/product prepared-table variants to decide
whether the random generator parameters should be treated as a major
experimental variable.

Current comparison axis: support-height interval `[h_min,h_max]`.
For generic random rows, `|a_k| = 1/h_k`, so changing this interval changes the
spread of facet-dual norms. For random product rows, it changes the two polygon
height distributions before taking the Lagrangian product.

Seed is reproducibility metadata only. It is not a research parameter and should
not be reported as a meaningful axis. Use larger independent samples,
bootstrap/subsampling, and effect sizes for uncertainty.

The method reports tail summaries, pairwise distribution tests for `sys`, top
rows, and the overlap of strongest geometry-only scalar associations with
`sys`.

# Conditional-tail atlas

This exploratory packet gives future thesis authors three ready-to-inspect
figure options for how retained invariant scalars relate to the upper `sys`
tail. It reuses all 14,336 trusted generic/product rows and compares features
by percentile inside each source bucket, so differences between facet-count and
product buckets do not create the principal trend.

## What the current artifacts show

- `ridge sum` has within-bucket rank correlation `-0.944` with `sys`; all 18
  bucket correlations have the same sign (range `-0.997` to `-0.841`).
- The favorable ridge-sum decile has mean `sys = 0.588`, versus `0.292`
  outside it, and contains a bucket-relative top-5%-`sys` row at rate `0.355`,
  versus `0.017` outside it.
- More extreme conditioning does not monotonically improve the observed upper
  envelope: the lowest 1% ridge-sum band has maximum `sys = 0.790`, while the
  1--2% band reaches `0.854` and the 20--50% band reaches `0.863`.
- HKO's ridge sum is below every retained random/product row. The scalar is
  therefore useful for tail enrichment on these sources, but the present
  generators do not reach the known positive reference regime.

These are descriptive, in-sample observations. They make no causal,
generalization, or proposer claim. In particular, the plots do not show that
pushing a scalar ever farther in its favorable direction improves the best
attainable `sys`.

## Figure options

- `artifacts/conditional-tail-atlas.png`: `sys` quantiles, maxima, and
  bucket-relative top-tail rates across invariant percentiles. This is the
  strongest current thesis-figure candidate.
- `artifacts/bucket-correlation-heatmap.png`: stability of correlation signs
  and magnitudes across all 18 buckets; useful when the cross-bucket robustness
  matters more than visual simplicity.
- `artifacts/raw-clouds.png`: generic/product raw clouds for three ridge
  summaries; mainly an exploratory diagnostic.

The PNGs are intentionally retained without publication polish. A thesis
author should choose the claim first, then simplify labels, colors, and panel
count for that use rather than polishing all three pre-emptively.

## Reproduction

From the repository root, with the shared prepared table materialized:

```bash
scripts/artifacts.py materialize polytope-invariant-table
uv run --script experiments/sys-datascience/methods/conditional-tail-atlas/analyze.py
```

`analyze.py` regenerates every file under `artifacts/`. `analysis.md` and the
TSV files retain the exact plotted summaries so the images need not be read as
the numerical source of truth.

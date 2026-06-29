# random-tail-eda

## Research Question

What does standard EDA show for `sys` on trusted random/product rows and on
overlapping source/parameter subsets of those rows?

The packet is one EDA packet because the methodology is the same across these
questions: count rows, plot the marginal, inspect quantiles and maxima, compare
obvious slices, and record the generator contract. It answers different
research questions at different scope levels:

- What is the empirical marginal distribution of `sys(a)` on the retained
  finite random/product sample?
- Do any retained random/product rows, or obvious overlapping source/parameter
  subsets, contain `sys > 1`?
- Where is the high tail concentrated inside the retained sample?
- Are these statements universal across arbitrary random polytope
  distributions?
- At what sample size should one expect the first `sys > 1` row under a
  specified generator?

The first three are answered descriptively for the retained sample. The last
two are not answered by this finite retained run: the packet records exactly
which generator parameters were used and gives only zero-positive bounds and
unstable tail extrapolations.

## Method

Direct scan, quantiles, top-row inspection, histogram, and top-tail plot over
the trusted random-only input. The same EDA/tail summary is applied to
overlapping filters by source family, facet count, source-family/facet-count
pair, and random-product polygon-pair bucket. The packet also records the
source-backed generator contract for the retained random/product sample, so the
negative and descriptive claims are tied to the actual sampling distributions.

## Inputs

- `../trusted-random-dataset/` filter logic
- `../../prepare/polytope-table.jsonl`
- `../../prepare/polytope-provenance-table.jsonl`

## Command

```bash
uv run --script experiments/sys-datascience/methods/random-tail-eda/analyze.py
```

## Generated Artifacts After Rerun

- `artifacts/summary.json`
- `artifacts/sys-histogram.png`
- `artifacts/sys-tail.png`
- `artifacts/generic-by-f.png`
- `artifacts/product-by-bucket.png`

## Observation

The provenance schema exposes optional source/generator fields such as height
range and product `(k,m,bounces)` when the producer row provides them. This
script prefers explicit product parameters over reparsing `path`, and records
`source_parameter_availability` in its summary. The current integrated check was
rerun against `/tmp/ds-integrated-full`; a scratch artifact copy was written to
`/tmp/ds-integrated-tail-eda`.

Current full scoped random/product run:

- rows: `14336`;
- `sys > 1` rows: `0`;
- max `sys`: `0.86258589584944`;
- overall median `sys`: `0.3108413591329677`;
- overall 99th percentile `sys`: `0.7521020122453151`;
- random-product max `sys`: `0.86258589584944`;
- generic-random max `sys`: `0.8595596058028344`.

Filtered tail summaries are stored in
`summary.json["filtered_tail_summaries"]`. The top retained filtered slices by
p99 are product-heavy (`6x6`, `5x6`) plus facet-count `F=12` views. No filtered
slice contains a `sys > 1` row. The largest observed value is still about
`0.137` below the threshold `1`.

`summary.json["source_parameter_availability"]` records explicit source
parameters present in the scoped provenance rows. Retained rows expose height
range and product `(k,m,bounces)` where applicable; seed/attempt remain absent
for these older retained producer files.

The source-backed generator contract is stored in
`summary.json["generator_contract"]`. In production mode, generic random rows
use `F=5..12`, `512` accepted samples per `F`, seed `42`, and heights
`h in [0.8,1.2]`. Random-product rows use all `3 <= k <= m <= 6`, `1024`
accepted samples per bucket, seed `42`, and the same height range.

This supports a finite-sample marginal statement, not a universal random-model
statement. The retained data do not vary the height range, seed/independent
rerun, generic facet counts outside `5..12`, product side counts outside
`3..6`, or alternative random distributions.

With zero positives in `14336` rows, the distribution-free 95% upper bound on
the success probability per draw is about `2.09e-4`, corresponding to about
`4786` samples per hit. This is only an upper bound from absence of positives,
not a prediction. Crude exponential tail extrapolations fitted to the top
`50..1000` rows range from about `1.9e4` to `2.6e6` samples per hit, so the
current tail data do not support a stable sample-size forecast.

Current interpretation: neither the pooled retained data nor the obvious
source/parameter filters contain a positive row. The filtered tail scan shows
where the high tail is concentrated, but it does not give a stable
sample-size forecast.

## Validity Guards

- Absence of `sys > 1` is sample-scoped, not an exhaustive theorem.
- Tail plots are exploratory; they do not define a candidate-proposer by
  themselves.
- Filtered-slice summaries are overlapping views of the same retained rows, not
  independent confirmations.
- Claims about arbitrary random polytope distributions require new generator
  families or independent reruns, not only this packet.

## Current Disposition

Baseline random-only negative result if the run records no `sys > 1` row.

## Remaining Worthwhile Questions

- At what sample size should one expect the first `sys > 1` row under a
  specified generator? The current retained sample does not answer this
  stably; it only gives zero-positive upper bounds and unstable tail
  extrapolations.
- How stable are the high-tail buckets under independent reruns?
- How does the high tail change if generator parameters outside this retained
  production run are varied, including height range, facet/side-count ranges,
  seeds, and alternative random polytope distributions?

## Predicted Stability Under Rerun

High on unchanged retained tables.

## Thesis Use

Supports the narrow statement that the trusted random/product sample contained
no recorded positive row, including under the obvious source and generator
parameter filters, and that its top tail remained below the Viterbo threshold.

## Reopen Triggers

- retained tables are rebuilt;
- random/product generation parameters change;
- thesis wording asks for a broader sample class.

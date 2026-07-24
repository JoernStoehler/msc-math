# Tail Survival 1M Posterior

## Split Status

Clean extraction from parked branch `sys-ds-tail-method-hardening`, top commit
`ff45bf0a Park tail survival hardening packet`.

This packet keeps the zero-positive, empirical-survival, EVT/model-sensitivity,
and observed-tail backtest artifacts. It deliberately excludes the parked
branch's HKO-distance/flank and product-structure-HKO outputs because review
found a normalization mismatch in those interpretations.

## Question

Under the current retained random/product generator and selected strata, is
`1M` accepted samples a rational run for finding at least one `sys > 1` row?

This packet treats fixed thresholds as landmarks. The decision object is the
upper-tail survival curve `S(t) = P(sys > t)` and the predictive probability of
at least one `sys > 1` row by a finite accepted-sample budget.

## Input

The script reads the retained trusted random/product tables through the shared
filter in `../_shared/random_only.py`:

- `experiments/polytope-invariant-table/polytope-table.jsonl`
- `experiments/polytope-invariant-table/polytope-provenance-table.jsonl`

It does not produce new random rows.

Validated retained-table anchors from the run:

- rows: `14336`;
- `sys > 1` rows: `0`;
- max `sys`: `0.86258589584944`;
- p99 `sys`: `0.7521020122453151`.

## Command

```bash
uv run --script experiments/sys-datascience/methods/tail-survival-1m-posterior/analyze.py
```

The script validates the anchors above and exits on mismatch.

## Artifacts

- `artifacts/empirical-survival.tsv`: tabular survival summaries at fixed
  thresholds for pooled, dataset, generic `F`, and product-bucket strata.
- `artifacts/dense-empirical-survival.tsv`: dense CCDF rows over observed
  high-tail values and grid thresholds, with Wilson confidence bands.
- `artifacts/top-k-spacings.tsv`: top-order-statistic gaps and record-row
  spacing summaries for selected strata.
- `artifacts/support-limited-future-max.tsv`: empirical-resampling future-max
  diagnostics. These are explicitly support-limited and cannot produce unseen
  values above the observed maximum.
- `artifacts/zero-positive-posterior-predictive.tsv`: beta-binomial
  zero-positive-only predictive probabilities for `N = 10k, 100k, 1M`.
- `artifacts/generic-f-density-posterior.tsv`: zero-positive density posterior
  artifact for generic random `F=5..12`, with exact-zero-support visibility.
- `artifacts/tail-model-predictive.tsv`: exponential-excess and GPD-excess
  extrapolations for pooled, dataset, and selected high-tail fixed buckets.
- `artifacts/dense-threshold-tail-fit.tsv`: dense threshold-stability scan for
  exponential-excess and GPD-excess fits over high quantiles `0.75..0.99`.
- `artifacts/mean-residual-life.tsv`: mean and median residual-life rows over
  the same threshold grid.
- `artifacts/observed-tail-backtest.tsv`: within-observed-tail backtests; fits
  above lower thresholds predict exceedance counts above higher observed
  thresholds.
- `artifacts/model-class-ledger.tsv`: structured model-class evidence ledger.
- `artifacts/naturalistic-hypothesis-ledger.tsv`: mechanism-level hypotheses,
  predictions, observations, update direction, and current qualitative weight.
- `artifacts/decision-summary.tsv`: compact decision table and flags.
- `artifacts/summary.json`: run metadata and artifact pointers.

Excluded from this split:

- `product-structure-hko-metrics.tsv`;
- `product-structure-hko-summary.tsv`;
- the HKO/flank mechanism rows formerly present in
  `naturalistic-hypothesis-ledger.tsv`.

## Interpretation

Distribution-free retained-table fact: the current retained table contains no
`sys > 1` row, and its record row is still at `sys = 0.86258589584944`.

Zero-positive-only Bayesian predictions are prior-sensitive rather than
distribution-free. On the pooled table, the `1M` posterior predictive ranges
from about `0.0367` under `Beta(0.01, 10000)` to about `0.986` under
`Beta(1, 1)`. These numbers mostly measure the prior assigned to unseen rare
events after zero positives in `14336` rows.

Tail extrapolation is model-dominated. Dense threshold scans keep the split
visible. On the pooled table, GPD-excess fits are available across the dense
threshold grid and put the fitted endpoint below `1` at every checked
threshold; none of those pooled GPD rows passes the local threshold-stability
flag. Pooled exponential-excess fits are locally stable across many thresholds
and put substantial mass beyond `1`, but they fail most within-observed-tail
backtests against higher observed thresholds. The disagreement is not a small
calibration issue; it is the main result for the scale-up decision.

Observed-tail backtesting makes the model choice concrete. For the pooled
table, GPD-excess predictions for observed higher-threshold exceedance counts
fall inside Wilson intervals in all checked backtests, while pooled
exponential-excess predictions do so in only one of nine checked backtests.
This does not prove the GPD endpoint forecast beyond the observed support, but
it is a reason not to let exponential-like extrapolation control the `1M`
decision.

Strongest current conclusion: do not treat `1M` accepted samples as a
well-calibrated rational blind scale-up under the current retained generator.
A controlled large run is rational only as a deliberately model-dominated probe
or after focusing strata and generator-axis work.

## Validity Guards

- Claims are restricted to the current retained random/product generator
  contract: generic random `F=5..12`, random Lagrangian-product buckets
  `3 <= k <= m <= 6`, seed `42`, height range `[0.8, 1.2]`, and the current
  accepted-sample filters.
- Pooled fits are included only as diagnostics. The retained table is a mixture
  of generators and fixed buckets.
- The tail-model rows are not fully explicit Bayesian posteriors over
  `P(sys > 1)`. They are model-class sensitivity probes with bootstrap
  instability flags.
- Dense threshold scans and observed-tail backtests test finite-sample
  compatibility inside the observed high tail. They still do not validate
  behavior beyond the current record row.
- The generic `F` posterior artifact is zero-positive-only. With only `512`
  retained rows per `F` and zero positives, it cannot support narrow density
  claims by facet count.
- Current high-tail rows are sub-threshold high rows, not positive examples.

## Disposition

Use this packet to justify one of these next actions:

- focus independent pilots on high-tail strata before any blind large run;
- run generator-axis work if the thesis question needs a credible hit search;
- run `1M` only as an explicitly model-dominated probe with sequential stopping
  and a pre-registered interpretation boundary;
- avoid scale-up if the goal is a calibrated probability claim rather than a
  compute probe.

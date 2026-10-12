# Tail Survival 1M Belief Ledger

Date: 2026-06-30.

This ledger records the non-generated model judgment behind
`tail-survival-1m-posterior`. The generated artifacts remain the source truth
for numeric rows.

## Scope

This clean split keeps retained-table tail, zero-positive, EVT/model-sensitivity,
and observed-tail backtest judgments. It excludes the parked branch's
HKO-distance/flank and product-structure-HKO judgments because those artifacts
had a normalization mismatch.

## Hypotheses Considered

- `H1`: an exponential-like continuation of the observed sub-threshold high tail
  is a reasonable guide beyond the current record row.
- `H2`: the retained table is a mixture whose pooled upper tail is not a
  generator-level law; apparent high-tail behavior is mostly stratum selection
  and winner's curse.
- `H3`: important generator axes are missing from this retained table, so the
  right action is generator redesign rather than blind scale-up.
- `H4`: the retained record rows around `sys = 0.86` arise from a
  generator-specific or mixture mechanism that this pooled packet does not
  identify.

## Updates From This Packet

1. Zero positives in `14336` retained rows moves mass away from ordinary-rate
   `sys > 1` claims, but by itself it does not distinguish rare-positive
   mechanisms from exact-zero or support-limited mechanisms.
2. The gap between the record row and `1` is large enough that fixed-threshold
   language should not drive the decision. The relevant object is extrapolation
   beyond the observed support.
3. Exponential excess models and GPD excess models disagree at the decision
   scale. Dense threshold scans preserve that disagreement: pooled exponential
   rows are locally stable across many thresholds, while pooled GPD rows put
   endpoints below `1` across the checked grid but do not pass the local
   stability flag.
4. The generic `F=5..12` per-bucket evidence is weak for density estimation:
   each `F` has only `512` retained rows and zero positives. It supports wide
   zero-positive posterior sensitivity, not point-like density predictions.
5. Pooled extrapolations should be discounted because the table is a mixture of
   generic and product strata.
6. Observed-tail backtesting discounts pooled exponential extrapolation: it
   predicts higher observed-threshold exceedance counts outside Wilson
   intervals in most checked pooled backtests. Pooled GPD backtests fit the
   observed tail better, but this does not prove the endpoint forecast beyond
   the observed record.

## Working Credence State

These are decision-calibration weights, not artifact-derived posterior
probabilities:

- low mass should be assigned to `H1` as a literal forecast, because it is not
  stable against GPD endpoint fits, fails pooled observed-tail backtests, and
  the retained table is a mixture;
- substantial mass remains on `H2`, because the retained table is a mixture and
  pooled extrapolations are not generator-level probability models;
- enough mass remains on `H3` that generator-axis work is competitive with
  spending compute on a blind `1M` run.

Practical consequence: a `1M` run is reasonable only if it is framed as a
controlled information-gathering probe. It is not currently justified as a
calibrated high-probability hit search.

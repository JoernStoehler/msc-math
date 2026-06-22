# prediction-ranking

## Research Question

How much in-table `sys` signal do geometry-only supervised predictors recover
on grouped held-out trusted random/product rows, and is the signal strong
enough to motivate a separate generated-candidate follow-up?

## Method

Use geometry-only features, excluding direct `sys`, capacity, volume, and
capacity-derived fields from the proposer input. Evaluate ridge regression and
random-forest regression on a grouped holdout split. Compare random-forest top
decile enrichment to a small permutation-null sanity check.

## Inputs

- trusted random-only rows from `../trusted-random-dataset/`

## Command

```bash
uv run --script experiments/sys-datascience/methods/prediction-ranking/analyze.py
```

## Retained Artifacts

- `artifacts/summary.json`

## Observation

Current run on hydrated retained tables in this branch:

- rows: `14336`;
- geometry-only features: `80`;
- grouped split: `capacity_source:facet_count`;
- train rows: `8192`;
- test rows: `6144`;
- ridge: `R^2 = 0.5278029834784923`, MAE
  `0.08913307407594219`, top-decile enrichment
  `0.5593495934959349`;
- random forest: `R^2 = 0.9213458228927175`, MAE
  `0.04077231570456351`, top-decile enrichment
  `0.6325203252032521`;
- random-forest enrichment permutation p-value with `10` bounded permutations:
  `0.09090909090909091`.

This is a strong in-table prediction signal. It is not a validated
candidate-proposer because it did not rank unevaluated generated rows before
their `sys` values were computed.

## Validity Guards

- The held-out rows have already had `sys` computed; this is a candidate-ranking
  diagnostic, not a validated new-row proposer.
- Metadata/provenance labels are not used as predictor features.
- A validated proposer would need to rank unevaluated generated rows before
  computing their `sys`, followed by evaluation of those generated candidates.

## Current Disposition

Use as an in-table supervised diagnostic. Treat as no validated
candidate-proposer unless a follow-up generated-row experiment supports a
stronger claim.

## Remaining Worthwhile Questions

If held-out enrichment is strong enough to matter for thesis value, run a
generated-candidate follow-up. Otherwise record the in-table signal as
explanatory evidence only.

## Predicted Stability Under Rerun

Moderate to high on unchanged retained tables; random seeds are fixed.

## Thesis Use

Supports the statement that ordinary supervised predictors recover an in-table
geometry signal. It does not support a validated candidate-proposer claim on
the trusted random/product data.

## Reopen Triggers

- retained table columns change;
- a new random-only dataset is added;
- model output is promoted to a generated-candidate experiment.

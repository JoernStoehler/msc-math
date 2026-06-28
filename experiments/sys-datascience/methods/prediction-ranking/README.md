# prediction-ranking

## Research Question

How much in-table `sys` signal do geometry-only supervised predictors recover
on grouped held-out trusted random/product rows, and is the signal strong
enough to motivate a separate generated-candidate follow-up?

## Method

Use geometry-only prepared feature columns as proposer input. Exclude the target
`sys`, prepared evaluation columns such as `capacity` and `volume`,
and capacity-derived diagnostics. Evaluate ridge regression and random-forest
regression on a grouped holdout split. Compare random-forest top decile
enrichment to a small permutation-null sanity check.

Also run metadata-only ridge and random-forest baselines using source/facet and
available provenance labels. These are leakage/source diagnostics, not geometry
candidate-proposer inputs.

## Inputs

- trusted random-only rows from `../trusted-random-dataset/`

## Command

```bash
uv run --script experiments/sys-datascience/methods/prediction-ranking/analyze.py
```

## Generated Artifacts After Rerun

- `artifacts/summary.json`

## Observation

This packet now uses all eligible geometry features by default, records which
features were selected, and records the top random-forest feature importances.
It also reports metadata-only baselines for source/facet/product provenance
labels. Metadata baselines test source leakage and stratification; they are not
candidate-proposer inputs.

Current full scoped random/product run:

- rows: `14336`;
- geometry-only features: `121`;
- metadata-only one-hot features: `27`;
- grouped split: `capacity_source:facet_count`;
- train rows: `8192`;
- test rows: `6144`;
- ridge: `R^2 = 0.7735964249383396`, MAE
  `0.0693694046813973`, top-decile enrichment
  `0.6276422764227643`;
- random forest: `R^2 = 0.927284952889666`, MAE
  `0.039484291990222124`, top-decile enrichment
  `0.6650406504065041`;
- metadata-only ridge: `R^2 = 0.30334429073518265`, MAE
  `0.14266912157047226`, top-decile enrichment
  `0.2245475113122172`;
- metadata-only random forest: `R^2 = 0.0019535588595063214`, MAE
  `0.16758290080062851`, top-decile enrichment
  `0.14031357959075205`;
- random-forest enrichment permutation p-value with `10` permutations:
  `0.09090909090909091`.

This is a strong in-table prediction signal. It is not a validated
candidate-proposer because it did not rank unevaluated generated rows before
their `sys` values were computed. Metadata-only baselines are much weaker than
geometry-only models, so the geometry signal is not explained away by
source/facet/product labels alone.

## Validity Guards

- The held-out rows have already had `sys` computed; this is a candidate-ranking
  diagnostic, not a validated new-row proposer.
- Metadata/provenance labels are not used in the geometry predictor. They are
  used only in separately reported metadata-only baselines.
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

# prediction-ranking

## Research Question

How much in-table `sys` signal do geometry-only supervised predictors recover
on grouped held-out trusted random/product rows, and is the signal strong
enough to motivate a separate generated-candidate follow-up?

## Method

Use geometry-only features, excluding direct `sys`, capacity, volume,
post-capacity orbit fields, and capacity-derived fields from the proposer
input. Evaluate ridge regression and random-forest regression on a grouped
holdout split. Compare random-forest top decile enrichment to a small
permutation-null sanity check.

Also run metadata-only ridge and random-forest baselines using source/facet and
available provenance labels. These are leakage/source diagnostics, not geometry
candidate-proposer inputs.

## Inputs

- trusted random-only rows from `../trusted-random-dataset/`

## Command

```bash
uv run --script experiments/sys-datascience/methods/prediction-ranking/analyze.py
```

## Retained Artifacts

- `artifacts/summary.json`

## Observation

Feature-space closure branch note: the script now defaults to all eligible
geometry features instead of the old `80`-feature cap, and the shared feature
selector includes new `omega_*` geometry columns plus two-face
symplectic-area tail columns when all loaded rows have complete two-face
ordering. Two-face ordering diagnostics are excluded from geometry inputs.
This branch also adds metadata-only baselines for source/facet/product
provenance labels. The current artifact below was regenerated with this
branch's method code against the full scoped random/product prepare output at
`/tmp/sys-ds-random-only-full`, built with `sys-dataset --random-only`.

Current full scoped random/product run:

- rows: `14336`;
- geometry-only features: `88`;
- metadata-only one-hot features: `26`;
- grouped split: `capacity_source:facet_count`;
- train rows: `8192`;
- test rows: `6144`;
- ridge: `R^2 = 0.5278029834786313`, MAE
  `0.08913307407593929`, top-decile enrichment
  `0.5593495934959349`;
- random forest: `R^2 = 0.9219838259237741`, MAE
  `0.04059841487617869`, top-decile enrichment
  `0.6341463414634146`;
- metadata-only ridge: `R^2 = 0.12017539977683755`, MAE
  `0.16535248208223238`, top-decile enrichment `0.296875`;
- metadata-only random forest: `R^2 = -0.04953269595337528`, MAE
  `0.1800783042710761`, top-decile enrichment `0.296875`;
- random-forest enrichment permutation p-value with `10` bounded permutations:
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

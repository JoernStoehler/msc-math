# prediction-ranking

## Research Question

How much in-table `sys` signal do invariant-feature supervised predictors recover
on grouped held-out trusted random/product rows, and is the signal strong
enough to motivate a separate generated-candidate follow-up?

## Method

Use active invariant prepared feature columns as proposer input. Exclude the
target `sys`, legacy evaluation columns if present, and source/provenance
metadata. Evaluate ridge regression and random-forest regression on a grouped
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

## Generated Artifacts After Rerun

- `artifacts/summary.json`

## Observation

No current full retained-table interpretation is recorded here until the
invariant-only schema is rerun. This packet records selected invariant
features, top random-forest feature importances, and metadata-only baselines.
Metadata baselines test source leakage and stratification; they are not
candidate-proposer inputs.

After rerun, a strong held-out prediction signal is still only in-table
evidence. A validated proposer would have to rank unevaluated generated rows
before their `sys` values are computed.

## Validity Guards

- The held-out rows have already had `sys` computed; this is a candidate-ranking
  diagnostic, not a validated new-row proposer.
- Metadata/provenance labels are not used in the invariant-feature predictor. They are
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
invariant-feature signal. It does not support a validated candidate-proposer
claim on the trusted random/product data.

## Reopen Triggers

- retained table columns change;
- a new random-only dataset is added;
- model output is promoted to a generated-candidate experiment.

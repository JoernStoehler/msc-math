# Endpoint Residualized Regression Disposition

Date: 2026-05-31.

Purpose: record the pre-LICCA review result and later repair result for the
`endpoint-residualized-regression` row. The repaired report path is
`experiments/sys-landscape/datascience/methods/feature-pattern-search/feature_pattern_search_residual_summary.md`.

## Disposition

Pre-LICCA final status after 2026-06-03 repair: `no-search-output`.

Current thesis role after the LICCA dataset refresh:

- pre-LICCA source material only;
- redo or record a terminal state from the current retained dataset before
  thesis use.

## Pre-Repair Findings

- `research/sys-landscape-toolbox-audit.md` marked the row unresolved:
  no current-contract report had been reviewed, and current artifacts were not
  enough for a terminal no-search-output or thesis-facing claim.
- `research/sys-landscape-datascience/idea-ledger.md` marked the row
  `current-review`.
- `research/sys-landscape-datascience/method-ledger.md` marked thesis use
  `undecided`.
- `experiments/sys-landscape/datascience/methods/feature-pattern-search/analyze_residual.py`
  documented an endpoint-only residual packet, but its `main()` called
  `load_joined_rows(dataset_dir)` instead of
  `load_joined_rows(dataset_dir, endpoint_only=True)`.
- `experiments/sys-landscape/datascience/methods/feature-pattern-search/common.py`
  already supported endpoint filtering via the `endpoint_only` argument.
- `analyze_residual.py` defined `write_summary(...)`, but `main()` did not call
  it, and no durable residual markdown report was committed.

## Repair Result

The 2026-06-03 repair made `analyze_residual.py` enforce endpoint-only loading
via `load_joined_rows(dataset_dir, endpoint_only=True)` and write
`feature_pattern_search_residual_summary.md`.

The report was refreshed on 2026-06-04 from the old retained `282`-row dataset
at `experiments/sys-landscape/datascience/dataset`. It is not current LICCA
evidence.
Pre-LICCA rerun command:

```bash
uv run --script experiments/sys-landscape/datascience/methods/feature-pattern-search/analyze_residual.py --dataset-dir experiments/sys-landscape/datascience/dataset
```

The pre-LICCA report records:

- `112` endpoint rows;
- `5` grouped endpoint folds;
- endpoint datasets `gradient_ascent_general`, `gradient_ascent_products`, and
  `variable_f_ascent`.

Several blocks add endpoint-only grouped-CV association beyond metadata. The
strongest ridge block is `face_symplectic` with metadata `R^2 = -0.0159`,
combined `R^2 = 0.4065`, and residual `R^2 = 0.4158`. The strongest random
forest block is `all_non_metadata` with metadata `R^2 = -0.0079`, combined
`R^2 = 0.3281`, and residual `R^2 = 0.3334`.

Interpretation: this is an endpoint-side table association, not a
candidate-proposer. It does not produce a validated new `sys > 1` row and does
not give a rule for proposing fresh candidates before inspecting `sys`, endpoint
labels, producer identity, optimizer provenance, or HKO2024-derived status.

## Reopen Trigger

Reopen by rerunning or reclassifying this row on the current retained dataset,
or if someone derives a candidate-proposer that does not use forbidden inputs.

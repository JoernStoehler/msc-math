# scan-sys-gt-1

## Research Question

Does the retained random/product datascience table contain a recorded row with
`sys > 1`?

## Method

Directly scan prepared table rows for the target predicate and summarize source
counts. Positive evidence would be at least one retained table row with
`sys > 1`. Negative evidence is finite-table-scoped absence only.

## Inputs

- `../../prepare/polytope-table.jsonl`
- `../../prepare/polytope-provenance-table.jsonl`
- `../../prepare/computed-polytope-observation-table.jsonl`

The script can scan additional producer computed-polytope JSONL files if passed
explicitly with `--computed-polytopes`, but the active random/product slice does
not use deleted ascent producer files.

## Command

```bash
uv run --script experiments/sys-datascience/methods/scan-sys-gt-1/analyze.py \
  --random-only
```

For scratch prepared tables:

```bash
uv run --script experiments/sys-datascience/methods/scan-sys-gt-1/analyze.py \
  --polytope-table "$TABLES_DIR/polytope-table.jsonl" \
  --provenance-table "$TABLES_DIR/polytope-provenance-table.jsonl" \
  --computed-polytope-observation-table "$TABLES_DIR/computed-polytope-observation-table.jsonl" \
  --random-only
```

## Generated Artifacts After Rerun

This packet prints its scan report to stdout and does not retain a separate
artifact by default.

## Observation

Previous full scoped random/product runs found:

- trusted random/product rows: `14336`;
- `random_sample`: `4096` rows, `0` positives;
- `random_product_sample`: `10240` rows, `0` positives;
- max `sys`: below `1`.

These numbers must be refreshed after the current-schema prepare rerun.

## Validity Guards

- This does not validate capacity, volume, or `sys` computations.
- This is not an exhaustive-search claim.
- This does not close the method table by itself.
- Additional producer files are scanned only when passed explicitly.

## Current Disposition

Run-pending-rerun baseline method-table row.

## Remaining Worthwhile Questions

No follow-up is worthwhile for this exact table-scoped predicate scan unless a
reopen trigger fires.

## Predicted Stability Under Rerun

High if rerun against unchanged retained tables and unchanged `sys` schema.

## Thesis Use

Supports the narrow statement that the retained random/product table contains
no recorded positive `sys > 1` row, after the current-schema rerun confirms it.

## Reopen Triggers

- retained tables are rebuilt;
- a new random/product source family is added;
- `sys` schema or normalization changes;
- thesis wording asks about producer-stage or non-retained outputs.

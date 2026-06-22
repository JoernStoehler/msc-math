# scan-sys-gt-1

## Research Question

Does the current retained sys-landscape datascience dataset already contain a
recorded row with `sys > 1`?

Checklist anchor: `Baseline And EDA / target predicate scan`.

## Method

This is baseline EDA for the black-box search problem: directly scan the
retained table rows for the target predicate and summarize the top tail.

Positive evidence would be at least one retained table row with `sys > 1`.
Negative evidence is only table-scoped absence of a recorded positive row.

## Inputs

- `../../prepare/polytope-table.jsonl`
- `../../prepare/polytope-provenance-table.jsonl`
- `../../prepare/computed-polytope-observation-table.jsonl`
- `../../produce/ascent-general-computed-polytopes.jsonl`
- `../../produce/ascent-product-computed-polytopes.jsonl`

The script scans geometric `sys` values from `polytope-table.jsonl` and raw
producer computed-polytope rows from canonical producer files by default. Pass
`--computed-polytopes` to override those producer paths. This lets the method
check intermediate ascent polytopes without requiring tables to materialize full
feature rows for them.

## Command

```bash
uv run --script experiments/sys-datascience/methods/scan-sys-gt-1/analyze.py
```

Run locally after retained-table rebuilds or producer computed-polytope refreshes.

Random-only scoped run:

```bash
uv run --script experiments/sys-datascience/methods/scan-sys-gt-1/analyze.py \
  --random-only
```

Smoke or LICCA merge review should scan the prepare-stage computed-polytope
observation output and producer computed-polytope rows:

```bash
uv run --script experiments/sys-datascience/methods/scan-sys-gt-1/analyze.py \
  --polytope-table "$TABLES_DIR/polytope-table.jsonl" \
  --provenance-table "$TABLES_DIR/polytope-provenance-table.jsonl" \
  --computed-polytope-observation-table "$TABLES_DIR/computed-polytope-observation-table.jsonl" \
  --computed-polytopes "$PRODUCE_DIR/ascent-general-computed-polytopes.jsonl" \
  --computed-polytopes "$PRODUCE_DIR/ascent-product-computed-polytopes.jsonl"
```

## Observation

These counts are from current retained prepared tables on the datascience architecture
merge, plus the committed fixed-F ascent producer computed-polytopes files.

- polytope rows: `32610`
- provenance rows: `22611`
- table rows with `sys > 1`: `0`
- producer computed-polytope rows scanned: `879235`
- producer computed-polytope rows with `sys > 1`: `0`
- computed-polytope observation rows: `879235`

Source summary:

| dataset | rows | sys > 1 |
| --- | ---: | ---: |
| gradient_ascent_general | `9533` | `0` |
| gradient_ascent_products | `8651` | `0` |
| random_product_sample | `10240` | `0` |
| random_sample | `4096` | `0` |
| variable_f_ascent | `90` | `0` |

Random-only observation from `--random-only`:

- trusted random/product rows: `14336`;
- provenance rows: `14336`;
- table rows with `sys > 1`: `0`;
- `random_sample`: `4096` rows, `0` positive;
- `random_product_sample`: `10240` rows, `0` positive.

## Interpretation

The retained datascience table dataset and the fixed-F producer computed
polytopes contain no recorded row with `sys > 1`.

This is baseline EDA evidence only. It confirms that the direct target
predicate is absent from the current retained method-table input and from the
current fixed-F intermediate producer rows.

## Validity Guards

- Producer computed-polytope scanning checks raw recorded `sys` values only.
- Refresh this report after rebuilding retained tables with
  `computed-polytope-observation-table.jsonl`.
- This does not validate capacity, volume, or `sys` computations.
- This does not scan producer files, LICCA merged review targets, stale
  artifacts, or other experiment folders unless passed explicitly.
- This is not an exhaustive-search claim.
- This does not close the hostile-landscape method table by itself.
- With `--random-only`, the scan excludes ascent producer computed-polytopes and
  reports only trusted random/product retained rows.

## Jörn Feedback

No method-specific Jörn feedback is recorded for this packet.

## Related Method Folders

No related method folders are currently present in HEAD.

## Current Disposition

Use as a baseline datascience method-table row: the current retained table
dataset does not already contain a recorded positive sample.

Do not use it as evidence that no positive sample exists outside the retained
table dataset.

## Remaining Worthwhile Questions

No follow-up is worthwhile for this exact table-scoped predicate scan unless a
reopen trigger fires. Other EDA, validation, and search-method questions belong
in separate method packets.

## Predicted Stability Under Rerun

High if rerun against unchanged retained tables and unchanged `sys` schema. The
script is deterministic and only reads the retained table files named above.

## Thesis Use

This packet supports the narrow statement that the retained datascience table
contains no recorded positive `sys > 1` row.

With `--random-only`, this packet supports the narrower random-only baseline
scan used by the scoped random/product method table.

It does not support a claim about producer-stage outputs, non-retained rows, or
the nonexistence of positive examples.

## Reopen Triggers

- retained tables are rebuilt;
- a new retained source family is added;
- `sys` schema or normalization changes;
- thesis wording asks about producer-stage or non-retained outputs.

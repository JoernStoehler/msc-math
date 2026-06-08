# scan-sys-gt-1

Status: method packet report. Not approved method-row status; see
`../STATUS.md` for approved status.

## Question

Does the current retained sys-landscape datascience dataset already contain a
recorded row with `sys > 1`?

Checklist anchor: `Baseline And EDA / target predicate scan`.

## Inputs

- `../../tables/polytope-table.jsonl`
- `../../tables/polytope-provenance-table.jsonl`

The retained report below is table-scoped. The script can also scan ascent
producer `*-computed-polytopes.jsonl` files when the caller passes
`--computed-polytopes`. Those files preserve computed-polytope facts that are not
yet table rows; the smoke pipeline uses this path as the basic producer-output
consumer.

## Command

```bash
uv run --script experiments/sys-landscape/datascience/methods/scan-sys-gt-1/analyze.py
```

Run locally on 2026-06-08.

Smoke or LICCA merge review can include producer computed-polytope rows after
those files have been generated. Use the `produce:` directory printed by
`../../smoke-pipeline.sh`, or a reviewed LICCA merge output directory:

```bash
uv run --script experiments/sys-landscape/datascience/methods/scan-sys-gt-1/analyze.py \
  --polytope-table "$TABLES_DIR/polytope-table.jsonl" \
  --provenance-table "$TABLES_DIR/polytope-provenance-table.jsonl" \
  --computed-polytopes "$PRODUCE_DIR/ascent-general-computed-polytopes.jsonl" \
  --computed-polytopes "$PRODUCE_DIR/ascent-product-computed-polytopes.jsonl"
```

## Observation

- polytope rows: `8445`
- provenance rows: `8445`
- rows with `sys > 1`: `0`

Source summary:

| dataset | rows | sys > 1 |
| --- | ---: | ---: |
| gradient_ascent_general | `4096` | `0` |
| gradient_ascent_products | `4089` | `0` |
| random_product_sample | `100` | `0` |
| random_sample | `70` | `0` |
| variable_f_ascent | `90` | `0` |

## Interpretation

The retained datascience table dataset contains no recorded row with
`sys > 1`.

This is baseline EDA evidence only. It confirms that the direct target
predicate is absent from the retained method-table input.

## Caveats

- The retained observation scans recorded table values only.
- Producer computed-polytope scans are available but not included in the retained
  full-dataset observation above until those producer rows are intentionally
  converted into a table row entity.
- This does not validate capacity, volume, or `sys` computations.
- This does not scan producer files, LICCA merged review targets, stale
  artifacts, or other experiment folders.
- This is not an exhaustive-search claim.
- This does not close the hostile-landscape method table by itself.

## Thesis Use

Use as a baseline datascience method-table row: the current retained table
dataset does not already contain a recorded positive sample.

Do not use it as evidence that no positive sample exists outside the retained
table dataset.

## Reopen Triggers

- retained tables are rebuilt;
- a new retained source family is added;
- `sys` schema or normalization changes;
- thesis wording asks about producer-stage or non-retained outputs.

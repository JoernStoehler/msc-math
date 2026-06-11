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
- `../../tables/computed-polytope-observation-table.jsonl`
- `../../produce/ascent-general-computed-polytopes.jsonl`
- `../../produce/ascent-product-computed-polytopes.jsonl`

The script scans geometric `sys` values from `polytope-table.jsonl` and raw
producer computed-polytope rows from canonical producer files by default. Pass
`--computed-polytopes` to override those producer paths. This lets the method
check intermediate ascent polytopes without requiring tables to materialize full
feature rows for them.

## Command

```bash
uv run --script experiments/sys-landscape/datascience/methods/scan-sys-gt-1/analyze.py
```

Run locally on 2026-06-11 after copying the LICCA `9825413` table outputs and
fixed-F producer computed-polytopes files.

Smoke or LICCA merge review should scan the table-stage computed-polytope
observation output and producer computed-polytope rows:

```bash
uv run --script experiments/sys-landscape/datascience/methods/scan-sys-gt-1/analyze.py \
  --polytope-table "$TABLES_DIR/polytope-table.jsonl" \
  --provenance-table "$TABLES_DIR/polytope-provenance-table.jsonl" \
  --computed-polytope-observation-table "$TABLES_DIR/computed-polytope-observation-table.jsonl" \
  --computed-polytopes "$PRODUCE_DIR/ascent-general-computed-polytopes.jsonl" \
  --computed-polytopes "$PRODUCE_DIR/ascent-product-computed-polytopes.jsonl"
```

## Observation

These counts are from the retained tables built by LICCA job `9825413` from
commit `58a93537`, plus the producer computed-polytopes files from the same
fixed-F ascent wave.

- polytope rows: `16629`
- provenance rows: `8445`
- table rows with `sys > 1`: `0`
- producer computed-polytope rows scanned: `879235`
- producer computed-polytope rows with `sys > 1`: `0`
- computed-polytope observation rows: `879235`

Source summary:

| dataset | rows | sys > 1 |
| --- | ---: | ---: |
| gradient_ascent_general | `4096` | `0` |
| gradient_ascent_products | `4089` | `0` |
| random_product_sample | `100` | `0` |
| random_sample | `70` | `0` |
| variable_f_ascent | `90` | `0` |

## Interpretation

The retained datascience table dataset and the fixed-F producer computed
polytopes contain no recorded row with `sys > 1`.

This is baseline EDA evidence only. It confirms that the direct target
predicate is absent from the current retained method-table input and from the
current fixed-F intermediate producer rows.

## Caveats

- Producer computed-polytope scanning checks raw recorded `sys` values only.
- Refresh this report after rebuilding retained tables with
  `computed-polytope-observation-table.jsonl`.
- This does not validate capacity, volume, or `sys` computations.
- This does not scan producer files, LICCA merged review targets, stale
  artifacts, or other experiment folders unless passed explicitly.
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

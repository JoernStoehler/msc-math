# scan-sys-gt-1

Status: method packet report. Not approved method-row status; see
`../STATUS.md` for approved status.

## Question

Does the current retained sys-landscape datascience dataset already contain a
recorded row with `sys > 1`?

Checklist anchor: `Baseline And EDA / target predicate scan`.

## Inputs

- `../../tables/polytope-table.jsonl`
- `../../tables/computed-polytope-observation-table.jsonl`
- `../../tables/polytope-provenance-table.jsonl`

The retained report below is table-scoped. The script scans geometric `sys`
values from `polytope-table.jsonl` and counts
`computed-polytope-observation-table.jsonl` rows as ascent occurrence context.
The optional `--computed-polytopes` flag is for ad hoc review of extra producer
files.

## Command

```bash
uv run --script experiments/sys-landscape/datascience/methods/scan-sys-gt-1/analyze.py
```

Run locally on 2026-06-08.

Smoke or LICCA merge review should scan the table-stage computed-polytope
observation output:

```bash
uv run --script experiments/sys-landscape/datascience/methods/scan-sys-gt-1/analyze.py \
  --polytope-table "$TABLES_DIR/polytope-table.jsonl" \
  --provenance-table "$TABLES_DIR/polytope-provenance-table.jsonl" \
  --computed-polytope-observation-table "$TABLES_DIR/computed-polytope-observation-table.jsonl"
```

## Pre-Rebuild Observation

These counts are from the old retained tables before computed ascent polytopes
were integrated into `polytope-table.jsonl`.

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

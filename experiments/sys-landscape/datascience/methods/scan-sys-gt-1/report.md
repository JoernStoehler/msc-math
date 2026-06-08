# scan-sys-gt-1

Status: method packet report. Not approved method-row status; see
`../STATUS.md` for approved status.

## Question

Does the current retained sys-landscape datascience table dataset already
contain a recorded row with `sys > 1`?

Checklist anchor: `Baseline And EDA / target predicate scan`.

## Inputs

- `../../tables/polytope-table.jsonl`
- `../../tables/polytope-provenance-table.jsonl`

The scan is intentionally table-scoped. It does not scan producer files, LICCA
merged review targets, stale artifacts, or other experiment folders.

## Command

```bash
uv run --script experiments/sys-landscape/datascience/methods/scan-sys-gt-1/analyze.py
```

Run locally on 2026-06-08.

## Observation

- polytope rows: `8445`
- provenance rows: `8445`
- rows with `sys > 1`: `0`
- max `sys`: `0.9750768559799221`

Top rows by `sys`:

| rank | poly_id | sys | dataset |
| ---: | --- | ---: | --- |
| 1 | `f6be75d99a357735276fc4b6eb36b0549c823dd75faeedb4fc7506903da2f1b8` | `0.9750768559799221` | gradient_ascent_products |
| 2 | `07455e997d624c62193180fd92026e2aba426e9b5bd1c3be4e8fe303ca4ffe5b` | `0.9605700102775944` | gradient_ascent_products |
| 3 | `d2fb3ebe31cb3192143eb192ddeb27c6d70ca7e3cb484deb5fb3858d7b9f746d` | `0.956663551909748` | gradient_ascent_products |
| 4 | `ddb6ac601f1085fecc1d51776f1bd9058e796d8b25743a5cab14f26ab09db121` | `0.9553992412593918` | gradient_ascent_products |
| 5 | `97ae586fc85a513a051605e90adecb5ccb9a082a94c00fe2e06ef467be8df7eb` | `0.9547696696237373` | gradient_ascent_products |
| 6 | `b1d3d0bac68600c440fb6ba1837869b4218e84d0f19b85a8dfec9538dc329614` | `0.9537672243081698` | gradient_ascent_products |
| 7 | `38552872ecba96f726f65653466b50c6e12965237b9e32ab322b4f672a94209c` | `0.9537556658671064` | gradient_ascent_products |
| 8 | `58a58dcb1026465fa23ae81b24c15da071886bfd5b8d0a63adb3ccfc43bdc411` | `0.9535851812204081` | gradient_ascent_products |
| 9 | `d4f37cf94718937fb912204c8d230ddc28e1a2deb816dc0a598c9d395f56af47` | `0.9520898741903471` | gradient_ascent_products |
| 10 | `36367c1dec04ad7b6cbbc810f80052368f7c9b006c4f07233c1c8e021d78002d` | `0.9500365215689874` | gradient_ascent_products |

Source summary:

| dataset | rows | sys > 1 | max sys |
| --- | ---: | ---: | ---: |
| gradient_ascent_general | `4096` | `0` | `0.9202772093964651` |
| gradient_ascent_products | `4089` | `0` | `0.9750768559799221` |
| random_product_sample | `100` | `0` | `0.7943664315075561` |
| random_sample | `70` | `0` | `0.7389189350162976` |
| variable_f_ascent | `90` | `0` | `0.9063161368249018` |

## Interpretation

The retained datascience table dataset contains no recorded row with
`sys > 1`.

This is baseline EDA evidence only. It confirms that the direct target
predicate is absent from the retained method-table input, and it identifies the
current maximum and top rows for follow-up context.

## Caveats

- This scans recorded table values only.
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

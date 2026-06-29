# trusted-random-dataset

## Research Question

Which retained table rows are trusted for the random/product sys-datascience
method wave?

## Method

This packet applies the shared trusted input filter from `../_shared/`.
Included datasets are `random_sample` and `random_product_sample`. Rows with
non-random/product provenance are excluded.

## Inputs

- `../../prepare/polytope-table.jsonl`
- `../../prepare/polytope-provenance-table.jsonl`

Source-truth pointers for the retained random producer shapes:

- `../../produce/main.rs`, production mode;
- `../../produce/README.md`, section "Run-Local Produce Path";
- `../../produce/random.rs` and `../../produce/random-product.rs` are older
  standalone refresh binaries with smaller default plans; do not use their
  smoke defaults as retained-table row-count source truth.

## Command

```bash
uv run --script experiments/sys-datascience/methods/trusted-random-dataset/analyze.py
```

For consumers that need a filtered JSONL input table, write it to a temporary
or explicit output directory:

```bash
uv run --script experiments/sys-datascience/methods/trusted-random-dataset/analyze.py \
  --out-dir /tmp/sys-random-only-dataset \
  --write-filtered
```

## Generated Artifacts After Rerun

- `artifacts/summary.json`

Filtered JSONL tables are generated on demand and are not retained by default
because they duplicate the retained table data.

## Observation

Current run on hydrated retained tables:

- trusted polytope rows: `14336`;
- trusted provenance rows: `14336`;
- `random_sample`: `4096`;
- `random_product_sample`: `10240`;
- duplicate polytope rows: `0`;
- excluded label hits: `0`;
- max `sys`: `0.86258589584944`;
- `sys > 1` rows: `0`.

## Validity Guards

- This packet does not validate capacity or volume computations.
- This packet is scoped to random/product rows only.
- The retained random/product row count is table-scoped.

## Current Disposition

Use as the input contract for trusted random/product method packets.

## Remaining Worthwhile Questions

Rerun after retained tables are rebuilt or after a new random/product producer
family is promoted into the retained table.

## Predicted Stability Under Rerun

High on unchanged retained tables.

## Thesis Use

Supports the statement that the random/product method table used an explicit
trusted input filter.

## Reopen Triggers

- retained tables are rebuilt;
- new trusted random/product datasets are added;
- provenance schema changes.

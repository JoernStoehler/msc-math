# Rotated Regular Products

This folder owns broad empirical sweeps for Lagrangian products of rotated
regular polygon pairs.

Local filenames below are relative to this folder. Paths outside this folder
are repo-root relative unless they begin with `../`.

Read this file if you need broad empirical context across regular polygon
pairs.

The selected analytic pentagon proof is maintained in
[`formal/pentagon-affine-products/`](../../../formal/pentagon-affine-products/README.md).
The earlier computational proof is retained in
`../pentagon-rotation-formula-proof/README.md`. Neither proof requires refreshing
these empirical sweeps.

Do not open generated JSONL or PNG files by default. Open them only when you
are regenerating plots or checking a specific empirical claim.

The more focused pentagon formula packet is split into sibling folders:

```text
../pentagon-rotation-empirics/
../pentagon-rotation-formula-proof/
```

## Commands

Refresh the broad sweeps:

```bash
cargo run -p exp-regular-products --release --bin regular-rotated-products
```

The current producer records the exact rational product capacity, outward
binary64 bounds, a deterministic minimizing word, and an explicitly
approximate `sys` value because volume is converted to binary64. Existing
tracked JSONL remains historical until a deliberate refresh; older rows still
contain the retired legacy billiard `iterations` field.

Refresh the plots:

```bash
uv run --script experiments/regular-products/rotated-regular-products/analyze.py
```

The default plot command reads the retained JSONL files and replaces the PNGs
beside them. To reproduce only the selected thesis profile into a fresh directory,
without running the capacity producer or replacing retained figures:

```bash
rotation_plot_dir=$(mktemp -d)
MPLBACKEND=Agg uv run --script \
  experiments/regular-products/rotated-regular-products/analyze.py \
  --only-pentagon --out-dir "$rotation_plot_dir"
```

This reads `lagrangian-products-5x5.jsonl` and writes
`lagrangian_products_5x5.png`; `--data-dir` optionally selects another input
directory. The selected thesis copy is `thesis/figures/rotation-profile.png`.
Copying a regenerated plot into the thesis is a deliberate step after review;
the thesis build uses its retained copy. The curve overlays the analytic formula
and the historical finite-QP samples; this plotting command does not reevaluate
or certify those samples. Rendering can reproduce while file metadata differs.

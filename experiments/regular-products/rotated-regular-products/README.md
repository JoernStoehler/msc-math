# Rotated Regular Products

This folder owns broad empirical sweeps for Lagrangian products of rotated
regular polygon pairs.

Local filenames below are relative to this folder. Paths outside this folder
are repo-root relative unless they begin with `../`.

Read this file if you need broad empirical context across regular polygon
pairs.

Do not read this folder for the pentagon formula proof. For that proof, go to
`../pentagon-rotation-formula-proof/README.md`.

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

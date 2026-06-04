# Rotated Regular Products

This folder owns broad empirical sweeps for Lagrangian products of rotated
regular polygon pairs.

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

Refresh the plots:

```bash
uv run --script experiments/regular-products/rotated-regular-products/analyze.py
```

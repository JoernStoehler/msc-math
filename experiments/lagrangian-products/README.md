# Lagrangian Products of Rotated Polygons

Systematically search for Viterbo counterexamples (sys > 1) in the space of Lagrangian products of regular 2D polygons, since the only known 4D counterexample is a Lagrangian product of two pentagons.

## Status
Complete

## Run

```bash
cd experiments/
cargo run --bin lagrangian_sweep --release
python3 lagrangian-products/lagrangian_products.py
```

## Design

Three experiment families:

1. **Pentagon rotation curve** (Family 1): Fix P = Q = regular pentagon, vary rotation angle theta in [0, 36] degrees at 1-degree steps. Exploits the symmetry lemma (period = 360/F degrees) to reduce the fundamental domain.
   - Output: `lagrangian-products-5x5.jsonl` (37 points)

2. **Polygon pair grid** (Family 2): For all pairs (n, m) with 3 <= n <= m <= 6, sweep rotation angle at 6-degree steps.
   - Output: `lagrangian-products-NxM-6deg.jsonl` (one file per pair, 10 files total)

3. **Random Lagrangian products** (Family 3): Covered by the separate random-product-sweep experiment.

## Key findings

- Pentagon x pentagon at theta=18 degrees achieves sys ~= 1.047, confirming the HK-O counterexample
- No other polygon pair achieves sys > 1
- Sys is a smooth function of rotation angle with clear periodicity
- The counterexample is a local maximum in the rotation parameter space

## Files

| File | Description |
|------|-------------|
| lagrangian_sweep.rs | Rust binary — generates all datasets |
| lagrangian_products.py | Python script — plots figures |
| lagrangian-products-5x5.jsonl | Pentagon rotation curve data |
| lagrangian-products-NxM-6deg.jsonl | Polygon pair grid data (10 files) |
| lagrangian_products_5x5.png | Pentagon rotation curve figure |
| lagrangian_products_polygon_pairs.png | Polygon pair comparison figure |
| lagrangian-products.tex | Thesis writeup |
| triangle_square.md | Investigation notes on triangle x square capacity |

## Known limitations

- Only regular polygons tested; irregular polygon products not explored
- Rotation angle step size is 6 degrees for polygon pair grid (coarser than pentagon 1-degree sweep)
- Family 3 (random Lagrangian products) delegated to separate random-product-sweep experiment

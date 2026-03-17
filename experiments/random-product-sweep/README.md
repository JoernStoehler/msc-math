# Random Product Sweep (sys vs polygon pair)

Sample random Lagrangian products from random 2D polygons and summarize systolic ratios by polygon pair (k,m), probing whether random Lagrangian products approach the Viterbo threshold.

## Status
Complete

## Design

- Polygon pairs: all (k,m) with 3 <= k <= m <= 6 (10 buckets)
- 10 samples per bucket (100 total)
- Heights: uniform in [0.8, 1.2]
- Algorithm: billiard only (native for Lagrangian products)

## Key findings

- **No random Lagrangian product achieved sys > 1.** Maximum sys = 0.794 (6x6 pair)
- Higher polygon pairs (more facets) tend to reach higher sys: 6x6 median = 0.572, 3x3 median = 0.273
- Balanced pairs (k=m) do not consistently outperform asymmetric pairs at the same total facet count
- The HKO counterexample (5x5 at theta=18 degrees) is not reproduced by random rotation angles

## Files

| File | Purpose |
|------|---------|
| `random_product_sweep.rs` | Rust binary: generates random Lagrangian products and computes sys |
| `random_product_sweep.py` | Python: scatter plot with median and std error bars per pair |
| `random-product-sweep.jsonl` | Dataset (100 rows) |
| `random-product-sweep.tex` | Thesis writeup |
| `random_product_sweep_sys_vs_pair.png` | Figure: sys vs polygon pair |

## Run

```bash
cd experiments/ && cargo run --bin random_product_sweep --release
python experiments/random-product-sweep/random_product_sweep.py
```

## Known limitations

- Only 10 samples per pair; limited statistical power
- Only regular polygon pairs with 3-6 sides; higher polygon counts not tested
- Billiard algorithm only; no cross-validation with HK2017

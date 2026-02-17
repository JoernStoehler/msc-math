# Random product sweep (sys vs polygon pair)

Goal: sample random Lagrangian products from random 2D polygons and summarize systolic ratios by polygon pair (k,m).

## Dataset

- Generator: `experiments/random-product-sweep/random_product_sweep.rs`
- Output JSONL: `experiments/random-product-sweep/random-product-sweep.jsonl`
- Polygon pairs: all (k,m) with 3 <= k <= m <= 6 (10 buckets)
- Samples: 10 per bucket
- Heights: uniform in [0.8, 1.2]
- Algorithm: billiard only

## Plot

- Script: `experiments/random-product-sweep/random_product_sweep.py`
- Output: `experiments/random-product-sweep/random_product_sweep_sys_vs_pair.png`
- Plot details:
  - Scatter all samples
  - Median with standard deviation error bars per pair
  - Reference line at sys = 1

## Run

```bash
cd experiments/ && cargo run --bin random_product_sweep --release
python experiments/random-product-sweep/random_product_sweep.py
```

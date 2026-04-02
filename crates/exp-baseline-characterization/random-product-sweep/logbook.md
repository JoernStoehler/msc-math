# Random Product Sweep: Logbook

## Motivation

Can random Lagrangian products approach or exceed the Viterbo threshold sys = 1? The known counterexample is a Lagrangian product of two pentagons at a specific rotation angle. This experiment checks whether random orientations of random polygon pairs produce high systolic ratios, or whether the counterexample requires fine-tuned parameters.

## Status

**Complete.** No random Lagrangian product exceeded sys = 0.80.

## How to run

```bash
# Generate dataset
cd experiments/ && cargo run --bin random_product_sweep --release

# Plot
python3 experiments/random-product-sweep/analyze.py
```

### Files

| File | Role |
|------|------|
| `run.rs` | Rust binary: generates random Lagrangian products, computes sys |
| `analyze.py` | Python: scatter plot with median and std error bars per pair |
| `math.tex` | Formal writeup (figure, observations) |
| `random-product-sweep.jsonl` | Dataset (100 rows) |
| `random_product_sweep_sys_vs_pair.png` | Figure: sys vs polygon pair |

## Design

- **Polygon pairs:** All (k, m) with 3 <= k <= m <= 6 (10 buckets).
- **Samples:** 10 per bucket (100 total).
- **Heights:** Uniform in [0.8, 1.2].
- **Algorithm:** Billiard only (native for Lagrangian products).
- **Seed:** 42 (deterministic).

## Findings

1. **No random Lagrangian product achieved sys > 1.** Maximum sys = 0.794 (6x6 pair).
2. Higher polygon pairs (more facets) tend to reach higher sys: (6,6) median = 0.572, (3,3) median = 0.273.
3. Balanced pairs (k = m) do not consistently outperform asymmetric pairs at the same total facet count. For example, (4,6) has median 0.515 vs (5,5) median 0.273.
4. The HKO counterexample (5x5 at specific rotation angle) is not reproduced by random orientations.
5. Random Lagrangian products reach higher sys than random generic polytopes (max 0.794 vs 0.58 in random-sweep), suggesting products are closer to the violation boundary.

## Known limitations

- Only 10 samples per pair; limited statistical power.
- Only regular polygon pairs with 3-6 sides; higher polygon counts not tested.
- Billiard algorithm only; no cross-validation with HK2017.

## Related experiments

- `random-sweep`: Random generic polytopes (non-product). Max sys = 0.58.
- `lagrangian-products`: Targeted sweep of specific polygon pairs at varying rotation angles, including the HKO counterexample configuration.

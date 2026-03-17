# Random Sweep (sys vs F)

Compute systolic ratios for random 4D polytopes across facet counts F=5..12, probing whether random generic polytopes approach the Viterbo threshold sys=1.

## Status
Complete

## Design

- 70 random 4D polytopes with facet counts F=5..12
- Heights: uniform in [0.8, 1.2], seed 42, ChaCha8Rng
- Sample plan: 10 samples each for F=5..10, 5 samples each for F=11..12
- Algorithm: HK2017 pruned only (production algorithm)

## Key findings

- **No polytope achieved sys > 1.** Maximum sys = 0.578 (at F=11)
- Median sys ranges from 0.08 (F=5) to 0.48 (F=12), generally increasing with F
- All 70 random polytopes remain well below the Viterbo threshold

## Files

| File | Purpose |
|------|---------|
| `random_sweep.rs` | Rust binary: generates random polytopes and computes sys |
| `random_sweep.py` | Python: scatter plot with median and std error bars |
| `random-sweep.jsonl` | Dataset (70 rows: polytope geometry, capacity, volume, sys) |
| `random-sweep.tex` | Thesis writeup |
| `random_sweep_sys_vs_f.png` | Figure: sys vs F scatter with median trend |

## Run

```bash
cd experiments/ && cargo run --bin random_sweep --release
python experiments/random-sweep/random_sweep.py
```

## Known limitations

- Small sample sizes (5-10 per F) limit statistical power
- Fixed seed (42) for reproducibility; different seeds may shift distributions
- Height range [0.8, 1.2] is narrow; wider ranges may produce different behavior

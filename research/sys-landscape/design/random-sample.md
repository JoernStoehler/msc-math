# Random Sweep: Logbook

## Motivation

Do random generic 4D polytopes approach the Viterbo threshold sys=1? This is the simplest baseline experiment: sample random polytopes across facet counts F=5..12 and compute their systolic ratios. If random polytopes routinely approach sys=1, the HKO counterexample would be less surprising. If they stay far below, it suggests that special structure (like Lagrangian product geometry) is needed to violate the conjecture.

## Status

**Complete.** Data generated, figure produced, thesis section written.

## How to run

```bash
# Default smoke run (temp output/cache)
cargo run -p exp-sys-landscape --release --bin sys-random-sample

# Canonical dataset refresh
cargo run -p exp-sys-landscape --release --bin sys-random-sample -- \
  --out experiments/sys-landscape/random-sample/random-sweep.jsonl \
  --cache experiments/sys-landscape/cache.jsonl

uv run analyze.py
```

### Files

| File | Role |
|---|---|
| `main.rs` | Rust binary: generates random polytopes, computes sys for each |
| `analyze.py` | Python: scatter plot of sys vs F with median and std error bars |
| `formal/sys-landscape/random-sample.tex` | Formal writeup: sampling setup, figure, interpretation |
| `random-sweep.jsonl` | Dataset (70 rows: polytope geometry, capacity, volume, sys) |
| `random_sweep_sys_vs_f.png` | Figure: sys vs F scatter with median trend |

## Design

- 70 random 4D polytopes with facet counts F=5..12
- Sample plan: 10 polytopes each for F=5..10, 5 each for F=11..12
- Normals: uniform on S^3. Heights: uniform in [0.8, 1.2]
- Seed 42, ChaCha8Rng for reproducibility
- Algorithm: HK2017 pruned only (production algorithm)
- Polytopes accepted by the standard rejection sampler

## Findings

![Systolic ratio vs facet count](random_sweep_sys_vs_f.png)

1. **No polytope achieved sys > 1.** Maximum sys = 0.739 (random_F12_2, F=12). All 70 random polytopes remain well below the Viterbo threshold.

2. **Median sys increases with F.** From 0.068 (F=5) to 0.423 (F=12). Spearman rho = 0.53. But within-F variance is large — spread from min to max is comparable to the median at each F. Median is non-monotone between adjacent F values.

3. **Geometry matters as much as facet count.** The large within-F variance suggests that normal directions and height ratios matter as much as facet count for determining sys.

4. **Random generic polytopes stay far from the violation regime.** The only known counterexample (sys ~= 1.047, HKO2024) is a Lagrangian product of two pentagons at a specific relative rotation angle. Random polytopes, which have no product structure, don't come close.

## Known limitations

- Small sample sizes (5-10 per F) limit statistical power
- Fixed seed (42) for reproducibility; different seeds may shift distributions
- Height range [0.8, 1.2] is narrow; wider ranges may produce different behavior
- Only tests generic polytopes — no product structure, no special symmetry

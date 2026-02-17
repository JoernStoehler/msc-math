# Random sweep (sys vs F)

Goal: sample random 4D polytopes with facet counts $F=5..12$, compute systolic ratios using HK2017 pruned, and summarize sys vs F.

## Dataset

- Generator: `experiments/random-sweep/random_sweep.rs`
- Output JSONL: `experiments/random-sweep/random-sweep.jsonl`
- Heights: uniform in [0.8, 1.2]
- Sample plan:
  - F=5..10: 10 samples each
  - F=11: 5 samples
  - F=12: 5 samples

## Plot

- Script: `experiments/random-sweep/random_sweep.py`
- Output: `experiments/random-sweep/random_sweep_sys_vs_f.png`
- Plot details:
  - Scatter all samples
  - Median with standard deviation error bars per F
  - Reference line at sys = 1

## Run

```bash
cd experiments/ && cargo run --bin random_sweep --release
python experiments/random-sweep/random_sweep.py
```

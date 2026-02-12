# generate_figures.py — Dataset figures

Produces 3 figures from `experiments/data/polytopes.jsonl` and `experiments/data/acceptance.jsonl`.

## Figures

1. **sys_histogram.png** — Systolic ratio distribution by facet count, with Viterbo threshold (sys=1) and HK-O pentagon marked.
2. **facet_vs_capacity.png** — Two panels: sys vs facet count (left), computation time vs facet count on log scale (right).
3. **acceptance_rates.png** — Rejection sampling acceptance rates from sweep data.

## Key findings (277 polytopes: 7 known + 270 random, F=5–10)

- **0 out of 270** random polytopes violate Viterbo's conjecture (sys > 1).
- Systolic ratios for random polytopes: range [0.001, 0.680], mean 0.190, median 0.151.
- Only counterexample: HK-O pentagon (sys = 1.047), a specially constructed 10-facet polytope.
- Computational limits: F=9 avg 394ms, F=10 avg 1969ms per polytope.

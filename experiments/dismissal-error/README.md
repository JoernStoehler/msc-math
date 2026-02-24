# Dismissal Error Bound Experiment

## Goal

Empirically confirm that the value loss from near-singular system dismissal
(Algorithm A.4 in the thesis, `alg:near-singular-handling`) is negligible.

## What it measures

For each polytope in the test dataset (capacity_dataset.json, 33 polytopes),
enumerates all (S, σ) pairs with A2 adjacency pruning. For every dismissed
pair, computes the error bound from Remark A.6 (`lem:dismissal-error-bound`):

    error_bound = (α*)² · ‖δβ'‖ · (1 + ‖H‖/σ_C) · σ_j

Pairs where β₀ ≤ 0 are "trivial dismissals" (no admissible critical point,
dismissal is exact). Pairs where β₀ > 0 get a computed error bound, compared
against the production capacity.

## Results

- 5,662 total pairs evaluated across 33 polytopes
- 68 non-trivial dismissals with computed error bounds
- 1,675 trivial dismissals (β₀ ≤ 0)
- **Maximum relative error: 2.3 × 10⁻¹⁷** (≈ 0.1× machine epsilon)
- Conclusion: value loss from dismissal is negligible

## Outputs

- `dismissal-error.jsonl` — per-dismissal records + per-polytope summaries
- `dismissal-error.png` — distribution of error bounds
- `dismissal-error.tex` — thesis writeup (input'd from appendix-numerical.tex)

## Run

```bash
cd experiments/ && cargo run --bin dismissal_error --release
python3 experiments/dismissal-error/dismissal_error.py
```

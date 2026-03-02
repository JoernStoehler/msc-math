# Rejection Sampling Acceptance Rates

## Purpose

Measure how often random halfspace representations yield valid (bounded, irredundant) 4-polytopes, across facet counts F=5..10 and three height ranges.

## Pipeline

```
acceptance_sweep (Rust) → acceptance.jsonl
```

No Python script or figure — the .tex writeup uses a table directly.

## Design

- 1000 attempts per (F, height_range) configuration
- 3 height ranges: narrow [0.8, 1.2], medium [0.5, 2.0], wide [0.1, 5.0]
- 6 facet counts: F = 5, 6, 7, 8, 9, 10
- Total: 18 rows (6 x 3)
- Deterministic seed (42) for reproducibility

## Key findings

- Narrow height range gives highest acceptance (>70% at F=10)
- Wide height range penalized by irredundancy failures (~28% at F=10)
- Acceptance increases with facet count for narrow and medium height ranges; wide range peaks around F=8-9 then dips slightly
- Used to calibrate random-sweep and random-product-sweep experiments

## Files

| File | Description |
|------|-------------|
| acceptance_sweep.rs | Rust binary — generates dataset |
| acceptance_sweep_test.rs | Unit tests for the sweep function |
| acceptance.jsonl | Generated data (18 rows) |
| rejection-sampling.tex | Thesis writeup with table |

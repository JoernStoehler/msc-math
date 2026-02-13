# benchmark — EHZ capacity performance across facet counts

Benchmarks `ehz_capacity_pruned` (release mode) on random polytopes.

## Data source

`experiments/profiling/benchmark.csv`: 76 random polytopes across F=5-12,
generated with seed 42, h in [0.5, 2.0].

## Model (random polytopes)

T(F) = 3.68 x 10^-7 * 4.73^F seconds (R^2 = 0.996)

Growth rate: **4.73x per facet** (vs 5.74x for known polytopes in the old model).

Random polytopes are faster than known polytopes, likely because random adjacency
graphs are sparser than the highly symmetric known polytopes, giving pruning more
to cut.

## Summary statistics

| F  | N  | Median (ms) | Mean (ms) | Min (ms)  | Max (ms)  | Growth |
|----|----|-------------|-----------|-----------|-----------|--------|
| 5  | 20 | 1.0         | 1.0       | 0.9       | 1.1       | -      |
| 6  | 20 | 3.6         | 3.9       | 2.4       | 6.1       | 3.7x   |
| 7  | 15 | 18.5        | 18.2      | 12.4      | 34.0      | 5.1x   |
| 8  | 10 | 86.2        | 88.5      | 59.0      | 160.6     | 4.7x   |
| 9  | 5  | 359.4       | 401.5     | 136.5     | 649.4     | 4.2x   |
| 10 | 3  | 2524.4      | 2114.9    | 1183.5    | 2636.9    | 7.0x   |
| 11 | 2  | 14751.1     | 14751.1   | 12217.0   | 17285.1   | 5.8x   |
| 12 | 1  | 31865.9     | 31865.9   | 31865.9   | 31865.9   | 2.2x   |

Note: F=12 has only 1 sample, so the 2.2x growth is unreliable. The overall
exponential fit (4.73x) is a better predictor.

## Comparison with old model (known polytopes only)

| F  | Old model (known) | New model (random) | Measured median |
|----|--------------------|---------------------|-----------------|
| 5  | 0.5 ms             | 0.5 ms              | 1.0 ms          |
| 8  | 98.8 ms            | 48.8 ms             | 86.2 ms         |
| 10 | 3257 ms            | 1091 ms             | 2524 ms         |
| 12 | 98971 ms           | 24371 ms            | 31866 ms        |
| 14 | 3.3M ms (54 min)   | 544K ms (9 min)     | not measured     |
| 16 | 107M ms (30 hr)    | 12.2M ms (3.4 hr)   | not measured     |

The old model overestimates by 3-4x at high F. Random polytopes benefit more
from adjacency pruning than the symmetric known polytopes.

## Practical limits (updated)

- F <= 8: sub-100ms, suitable for large datasets (1000+)
- F = 9: ~0.4s median, feasible for 50-100 polytopes
- F = 10: ~2.5s median, feasible for 20-50 polytopes
- F = 11: ~15s median, feasible for 5-10 polytopes
- F = 12: ~32s (1 sample), feasible for 1-5 polytopes
- F = 13: ~3.6 min (projected), borderline single-run
- F = 14: ~17 min (projected), single-run only
- F >= 15: hours per polytope

## Implications for MAX_FACETS_BRUTEFORCE

The current cutoff of 10 was based on the old model. With the new data:
- F=11 takes ~15s: adding to datasets is feasible (2 samples took 30s)
- F=12 takes ~32s: feasible for small counts
- Raising cutoff to 12 would add ~2 minutes to dataset generation

Decision: for Jorn.

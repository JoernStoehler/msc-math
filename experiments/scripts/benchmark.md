# benchmark — EHZ capacity performance across facet counts

Benchmarks `ehz_capacity_pruned` (release mode) on random polytopes.

## Data source

`experiments/profiling/benchmark.csv`: 76 random polytopes across F=5-12,
generated with seed 42, h in [0.5, 2.0]. Total wall time: 72s.

## Raw data

| F | Sample | time_ms | capacity | iterations |
|---|--------|---------|----------|------------|
| 5 | 0 | 1.058 | 6.405 | 84 |
| 5 | 1 | 0.949 | 9.560 | 84 |
| 5 | 2 | 0.976 | 13.425 | 84 |
| 5 | 3 | 0.967 | 4.254 | 84 |
| 5 | 4 | 0.993 | 18.744 | 84 |
| 5 | 5 | 0.932 | 21.324 | 84 |
| 5 | 6 | 0.977 | 30.816 | 84 |
| 5 | 7 | 1.136 | 14.452 | 84 |
| 5 | 8 | 0.978 | 12.102 | 84 |
| 5 | 9 | 0.941 | 15.702 | 84 |
| 5 | 10 | 1.006 | 7.647 | 84 |
| 5 | 11 | 0.980 | 21.892 | 84 |
| 5 | 12 | 0.988 | 12.765 | 84 |
| 5 | 13 | 1.116 | 13.002 | 84 |
| 5 | 14 | 1.036 | 17.414 | 84 |
| 5 | 15 | 0.984 | 6.946 | 84 |
| 5 | 16 | 0.981 | 15.209 | 84 |
| 5 | 17 | 0.985 | 9.132 | 84 |
| 5 | 18 | 0.954 | 44.939 | 84 |
| 5 | 19 | 1.000 | 18.007 | 84 |
| 6 | 0 | 6.079 | 7.822 | 409 |
| 6 | 1 | 6.072 | 18.581 | 409 |
| 6 | 2 | 6.032 | 10.401 | 409 |
| 6 | 3 | 6.103 | 18.626 | 409 |
| 6 | 4 | 5.707 | 4.112 | 409 |
| 6 | 5 | 3.770 | 14.353 | 409 |
| 6 | 6 | 3.559 | 6.144 | 409 |
| 6 | 7 | 3.504 | 8.813 | 409 |
| 6 | 8 | 3.612 | 8.491 | 409 |
| 6 | 9 | 3.639 | 6.658 | 409 |
| 6 | 10 | 2.377 | 7.574 | 280 |
| 6 | 11 | 3.580 | 10.317 | 409 |
| 6 | 12 | 3.592 | 4.658 | 409 |
| 6 | 13 | 2.556 | 5.118 | 280 |
| 6 | 14 | 2.467 | 12.006 | 280 |
| 6 | 15 | 3.743 | 11.879 | 409 |
| 6 | 16 | 2.466 | 4.031 | 280 |
| 6 | 17 | 3.657 | 11.848 | 409 |
| 6 | 18 | 2.407 | 3.200 | 280 |
| 6 | 19 | 3.693 | 22.880 | 409 |
| 7 | 0 | 34.020 | 9.190 | 2365 |
| 7 | 1 | 18.874 | 3.711 | 1714 |
| 7 | 2 | 25.020 | 9.241 | 2365 |
| 7 | 3 | 18.165 | 11.903 | 1714 |
| 7 | 4 | 19.481 | 10.400 | 1714 |
| 7 | 5 | 12.447 | 12.704 | 1193 |
| 7 | 6 | 18.561 | 9.837 | 1714 |
| 7 | 7 | 18.679 | 10.602 | 1714 |
| 7 | 8 | 13.517 | 17.785 | 1259 |
| 7 | 9 | 13.271 | 17.186 | 1259 |
| 7 | 10 | 12.601 | 9.737 | 1193 |
| 7 | 11 | 18.392 | 11.938 | 1714 |
| 7 | 12 | 13.294 | 7.726 | 1259 |
| 7 | 13 | 18.483 | 9.913 | 1714 |
| 7 | 14 | 18.610 | 10.752 | 1714 |
| 8 | 0 | 89.718 | 13.425 | 6869 |
| 8 | 1 | 58.965 | 16.708 | 4652 |
| 8 | 2 | 67.988 | 6.238 | 5110 |
| 8 | 3 | 87.553 | 4.546 | 6543 |
| 8 | 4 | 84.757 | 5.635 | 6543 |
| 8 | 5 | 91.489 | 9.981 | 6869 |
| 8 | 6 | 114.947 | 9.835 | 8890 |
| 8 | 7 | 61.823 | 10.778 | 4880 |
| 8 | 8 | 66.961 | 7.420 | 5110 |
| 8 | 9 | 160.597 | 12.764 | 12151 |
| 9 | 0 | 136.483 | 14.384 | 8885 |
| 9 | 1 | 649.430 | 4.652 | 42424 |
| 9 | 2 | 359.382 | 10.153 | 23608 |
| 9 | 3 | 571.206 | 7.438 | 36797 |
| 9 | 4 | 290.829 | 6.696 | 18705 |
| 10 | 0 | 2636.913 | 7.345 | 141228 |
| 10 | 1 | 1183.470 | 16.322 | 63034 |
| 10 | 2 | 2524.425 | 8.598 | 137228 |
| 11 | 0 | 17285.102 | 9.169 | 784001 |
| 11 | 1 | 12217.028 | 6.611 | 531789 |
| 12 | 0 | 31865.866 | 9.495 | 673082 |

Observations on raw data:
- F=5: iteration count is always 84 = sum_{m=2}^{5} C(5,m)*(m-1)! — no pruning
  possible with 5 facets (all facets adjacent)
- F=6: iteration count is either 280 or 409 — two distinct pruning regimes
- F=9-12: large variance in iterations (e.g. F=9 ranges 8885-42424), showing
  that adjacency structure dominates runtime

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

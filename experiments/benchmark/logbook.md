# Benchmark: Logbook

## Motivation

Before committing to large-scale experiments, we need to know: how long does capacity computation take as a function of facet count? What is the practical upper limit? Which algorithm (HK2017 pruned/unpruned, billiard) to use in which regime? This experiment establishes the timing model that guides dataset design and algorithm selection for all subsequent experiments.

## Status

**Complete.** Timing models fitted, algorithm selection guidelines established.

## How to run

```bash
cd experiments/
cargo run --bin benchmark --release   # -> benchmark/benchmark.jsonl (~8 seconds)
python3 benchmark/analyze.py          # -> profiling/timing_model.json, benchmark_timing.png
```

After regeneration, manually update `math.tex` table if numbers changed significantly.

### Files

| File | Role |
|------|------|
| `run.rs` | Rust binary: dataset generator, runs all algorithm variants |
| `analyze.py` | Python: timing model fitting (exponential) + unified figure |
| `math.tex` | Formal writeup: timing tables, model, algorithm selection |
| `benchmark.jsonl` | Dataset: 95 polytopes with timing for each applicable algorithm |
| `benchmark_timing.png` | Figure: all algorithms, polytope classes, fitted models |
| `profiling/timing_model.json` | Fitted model parameters |
| `profiling/` | Profiling artifacts (left as-is) |

## Design

### Dataset structure

95 polytopes total (83 random + 12 Lagrangian), self-contained (does not reuse polytopes from other experiments):

| Group | F | N | Algorithms | Purpose |
|-------|---|---|------------|---------|
| Random | 5 | 10 | Pruned, Unpruned | Algorithm agreement |
| Random | 6 | 10 | Pruned, Unpruned | Algorithm agreement |
| Random | 7 | 10 | Pruned, Unpruned | Algorithm agreement |
| Random | 8 | 15 | Pruned only | Timing model |
| Random | 9 | 15 | Pruned only | Timing model |
| Random | 10 | 15 | Pruned only | Timing model |
| Random | 11 | 5 | Pruned only | Timing model (expensive) |
| Random | 12 | 3 | Pruned only | Timing model (very expensive) |
| Lagrangian | 6-10 | 2 each (12 total) | Pruned, Billiard | Algorithm comparison |

Total capacity computations: 137 (95 pruned + 30 unpruned + 12 billiard).

**DATA DISCREPANCY**: The README stated "~85 polytopes, ~100 capacity computations." Actual data: 95 polytopes, 137 capacity computations. The "~85" may have been an earlier dataset size that was not updated.

### Why unpruned only for F <= 7

Unpruned HK2017 has exponential cost without adjacency pruning. At F > 7 it becomes prohibitively expensive (hours per polytope).

### Why Lagrangian products for billiard

The billiard algorithm only works on Lagrangian products (K_q x K_p where K_q, K_p are 2D polygons).

### HK2017 implementation optimizations

Before benchmarking, two implementation-level optimizations were applied (no algorithm changes):

1. **Lazy permutation generation**: In-place callback via Heap's algorithm replaces eager collection of all (m-1)! cyclic permutations. For m=10: eliminates 362,880 vector allocations. Wall-clock speedup: 4-6x on F=10 polytopes. Memory: peak heap dropped from 573 KB to 17 KB.

2. **LU fast path + gap-based SVD rank detection**: Try FullPivLU first, fall back to SVD only when LU reports singularity. LU adds 6-12% wall-clock overhead (most KKT systems yield beta <= 0 even when invertible), but catches cases where gap-based SVD rank detection over-truncates. Retained for correctness despite overhead.

Two attempted optimizations were discarded:
- Pre-allocated matrix buffers: no measurable speedup (nalgebra handles small matrices well).
- Skipping residual check for LU: broke correctness on the hypercube.

Detailed profiling methodology (Valgrind callgrind, massif heap profiling, phase-by-phase wall-clock timing) and before/after measurements were documented in the original `hk2017_optimization.md`.

## Findings

1. **Timing models** (fitted exponential T(F) = a * b^F):
   - HK2017 pruned (random): T(F) = 2.3e-7 * 4.8^F seconds (R^2 = 0.998)
   - HK2017 pruned (Lagrangian): T(F) = 1.2e-7 * 5.3^F seconds (R^2 = 0.997)
   - HK2017 unpruned (random, F <= 7): T(F) = 1.4e-8 * 7.9^F seconds (R^2 = 1.000)
   - Billiard (Lagrangian): T(F) = 5.8e-8 * 5.3^F seconds (R^2 = 0.998)

2. **Growth rates**: ~5x per facet (pruned/billiard), ~8x per facet (unpruned).

3. **Practical limits**:
   - F <= 10: routine use (< 300ms median)
   - F = 11-12: acceptable for one-off computations (1-9 seconds)
   - F >= 13: prohibitively expensive for large datasets

4. **Algorithm selection**:
   - Lagrangian products: always use billiard (polynomial cost, validated against HK2017)
   - General polytopes: use HK2017 pruned (only general algorithm available)
   - Debug/testing: use unpruned on F <= 7 for exhaustive search

5. **Pruned vs unpruned speedup**: 8.6x at F=5, 17.5x at F=6, 42.5x at F=7.

6. **Billiard vs HK2017 pruned on Lagrangian products**: billiard is 2.4-6.8x slower, but used for its polynomial-time guarantee.

## Open questions

1. **Timing model coefficients disagree between this logbook and math.tex.** The logbook (inherited from README) has T(F) = 2.3e-7 * 4.8^F (R^2=0.998) for HK2017 pruned random; math.tex has T(F) = 2.1e-8 * 4.2^F (R^2=0.970). The growth rate is ~5x/facet here vs ~4x in math.tex. One source was independently updated. Ground truth: rerun `analyze.py` on `benchmark.jsonl`.
2. **"1-9 seconds" for F=11-12** — the math.tex table shows F=12 max of 1806ms (1.8s). The "9 seconds" figure is unsupported. Needs verification.

## Known limitations

- Sample size at high F: only 3-5 samples for F >= 11, limited statistical power for tail behavior.
- Fixed seed (42) for reproducibility. Different seeds may give slightly different timing characteristics.
- Billiard sample count: only 2 Lagrangian products per (n,m) pair.

## Related experiments

- **ablation**: Detailed analysis of pruning effectiveness at each level (A0-A3).
- **crosspolytope**: Uses the timing model to estimate feasibility of F=16 computation (~4 hours predicted).

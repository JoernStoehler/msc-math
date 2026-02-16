# Benchmark Experiment

**Purpose:** Establish practical performance limits for EHZ capacity computation and guide algorithm selection.

**Status:** Complete. Dataset generated, timing model fitted, .tex writeup updated.

**Files:**
- Rust: `crates/datasets/src/bin/benchmark.rs` (dataset generator)
- Dataset: `experiments/data/benchmark.jsonl` (~85 polytopes, ~100 capacity computations)
- Python: `experiments/scripts/benchmark.py` (timing model fitting + figure)
- Figure: `experiments/figures/benchmark_timing.png`
- Model: `experiments/profiling/timing_model.json`
- Writeup: `thesis/experiments/benchmarks.tex`

## Design

### Motivation

Before committing to large-scale experiments, we need to understand:
1. How long does capacity computation take as a function of facet count?
2. What's the practical upper limit for routine use?
3. Which algorithm (HK2017 pruned/unpruned vs billiard) to use in which situation?

### Strategy

Generate a curated benchmark dataset with:
- **Random polytopes** (F=5..12) for HK2017 timing model
- **Lagrangian products** (F=6..10) for billiard vs HK2017 comparison
- Smaller sample counts at high F (expensive to compute)

Key differences from other experiments:
- **Self-contained**: doesn't reuse polytopes.jsonl, generates its own data
- **Mixed algorithms**: times pruned, unpruned (F≤7), and billiard (Lagrangian products)
- **Timing-focused**: primary output is wall-clock time, not capacity values

### Dataset Structure

~85 polytopes total:

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
| Lagrangian | 6..10 | 2 each (12 total) | Pruned, Billiard | Algorithm comparison |

**Why unpruned only for F≤7?** Unpruned HK2017 has exponential cost without adjacency pruning. At F>7, it becomes prohibitively expensive (hours per polytope). We test pruned vs unpruned agreement on smaller polytopes, then rely on pruned for F>7.

**Why Lagrangian products for billiard?** The billiard algorithm only works on Lagrangian products (polytopes of the form K_q × K_p where K_q, K_p are 2D polygons). We generate random Lagrangian products to test pruned vs billiard agreement and billiard timing.

## Pipeline

```
benchmark.rs → benchmark.jsonl → benchmark.py → {timing_model.json, benchmark_timing.png}
```

### Step 1: Generate dataset

```bash
cd crates/
cargo run --bin benchmark --release
```

Outputs: `experiments/data/benchmark.jsonl` (~85 entries)

Each entry contains:
- Polytope geometry (normals, heights)
- Timing for pruned (always)
- Timing for unpruned (F≤7 only)
- Timing for billiard (Lagrangian products only)

**Runtime:** ~5-10 minutes (dominated by F=11,12 samples)

### Step 2: Fit timing model

```bash
cd experiments/
python3 scripts/benchmark.py
```

Reads `benchmark.jsonl`, fits exponential model T(F) = a · b^F via log-linear regression.

Outputs:
- `profiling/timing_model.json` (model parameters, R² fit quality)
- `figures/benchmark_timing.png` (scatter plot + fitted curve)

Prints summary table to stdout (for manual .tex update).

## HK2017 Implementation Optimization

Before benchmarking, we optimized the HK2017 implementation without changing the underlying algorithm:

### 1. Lazy permutation generation

**Optimization:** In-place callback via Heap's algorithm replaces eager collection of all (m-1)! cyclic permutations. For m=10, this eliminates 362,880 vector allocations.

**Speedup:** 1.2–2.2× on individual operations.

### 2. Gap-based SVD rank detection

**Optimization:** A gap-based rank determination replaces the fixed-tolerance SVD pseudoinverse (see Section on correctness in thesis).

### 3. LU fast path

**Optimization:** The solver first attempts full-pivoting LU, falling through to SVD with gap-based rank detection on failure.

**Performance characteristics:** Phase profiling showed the LU path adds 6–12% wall-clock overhead: most KKT systems yield β ≤ 0 even when LU reports the system as invertible, so LU decomposes and solves wastefully before SVD runs anyway.

**Why retained despite overhead:** LU catches cases where the gap-based SVD rank detection over-truncates. On the HK-O pentagon:
- LU reports 92% of systems invertible but only 3% produce valid β > 0
- On random polytopes (F=7-8), the valid rate rises to ~9%
- At F=7, LU finds 26 valid orbits (out of 23,650) that SVD rejects
- These don't affect the computed capacity (all are non-optimal), but the discrepancy indicates that the gap threshold (100×) is too aggressive for some well-conditioned systems
- This is discussed further in the correctness section of the thesis

### Overall improvement

**Wall-clock speedup from lazy permutation:** 4–6× on the HK-O pentagon (F=10) and random polytopes at F=10.

**Memory reduction:** Peak heap memory dropped from 573 KB to 17 KB.

**Reference:** See `experiments/hk2017_optimization.md` for full profiling methodology (Valgrind callgrind, massif heap profiling, phase-by-phase wall-clock timing).

## Results Summary

**Timing model** (fitted to F=5..12 random polytopes):
```
T(F) = 1.5×10⁻⁷ · 4.32^F seconds  (R² = 0.994)
```

**Growth rate:** ~4-5× per additional facet (exponential)

**Practical limits:**
- F≤10: routine use (<300ms median)
- F=11-12: acceptable for one-off computations (1-9 seconds)
- F≥13: prohibitively expensive for large datasets (tens of seconds to minutes per polytope)

**Algorithm selection:**
- Lagrangian products: always use billiard (polynomial cost, validated against HK2017)
- General polytopes: use HK2017 pruned (only general algorithm available)
- Debug/testing: use unpruned on F≤7 for exhaustive search (all code paths exercised)

## Data Flow to Thesis

The thesis section (`benchmarks.tex`) includes:
1. **Timing table** by facet count (median, mean, min, max)
2. **Exponential model** with fitted parameters and R²
3. **Figure** showing scatter plot + fitted curve
4. **Algorithm comparison** (billiard vs HK2017 on Lagrangian products)

The writeup focuses on performance characteristics and practical limits, not implementation details.

## Regeneration

To regenerate after algorithm changes:

```bash
cd crates/
cargo run --bin benchmark --release   # Generates experiments/data/benchmark.jsonl (~5-10 min)
cd ../experiments/
python3 scripts/benchmark.py          # Fits model, generates figure
```

Then manually update `benchmarks.tex` table if numbers changed significantly.

## Known Limitations

- **Sample size at high F:** Only 3-5 samples for F≥11 due to computational cost. Limited statistical power for tail behavior.
- **Random sampling:** Fixed seed (42) for reproducibility. Different seeds may give slightly different timing characteristics.
- **Billiard sample count:** Only 2 Lagrangian products per (n,m) pair. Enough to verify agreement, but not for detailed billiard timing analysis.

# Benchmark: Logbook

## Motivation

How long does the systolic ratio pipeline take, where does the time go, and which algorithm should we use? This experiment establishes timing models and phase breakdowns that guide dataset design and algorithm selection for all subsequent experiments.

## Status

**Complete.** Capacity timing models fitted. End-to-end phase breakdown measured via criterion + perf flamegraphs. Algorithm selection guidelines established.

## How to run

```bash
# Capacity timing dataset (95 polytopes, 137 capacity computations)
cd crates/exp-algorithm-comparison/benchmark/
cargo run --bin cmp-benchmark --release   # -> benchmark.jsonl (~8 seconds)
python3 analyze.py                        # -> profiling/timing_model.json, benchmark_timing.png

# End-to-end phase profiling (criterion benchmarks)
cd crates/
cargo bench --bench profiling         # -> target/criterion/ (JSON + HTML reports)
python3 ../crates/exp-algorithm-comparison/benchmark/profiling/analyze_profiling.py  # -> phase_breakdown.png, micro_benchmarks.png

# Flamegraph (requires sudo for perf)
cd crates/exp-algorithm-comparison/benchmark/
cargo build --release --bin cmp-benchmark-profile
sudo env "PATH=$PATH" flamegraph -o profiling/flamegraph_F9.svg \
  -- ./target/release/cmp-benchmark-profile 9 50
```

## Files

| File | Role |
|------|------|
| `run.rs` | Dataset generator: 95 polytopes, times all algorithm variants |
| `profile.rs` | Single-polytope profiling harness (for flamegraph) |
| `analyze.py` | Timing model fitting (exponential) + algorithm comparison figure |
| `benchmark.jsonl` | Dataset: 95 polytopes with timing per algorithm |
| `benchmark_timing.png` | Figure: all algorithms + fitted exponential models |
| `profiling/timing_model.json` | Fitted model parameters |
| `profiling/analyze_profiling.py` | Reads criterion JSON, produces phase breakdown figures |
| `profiling/phase_breakdown.png` | Figure: construction/capacity/volume breakdown vs F |
| `profiling/micro_benchmarks.png` | Figure: per-call micro-benchmarks (KKT, transition, pruning) |
| `profiling/flamegraph_F*.svg` | Flamegraphs at F=9 and F=11 |
| `profiling/crosspolytope_progress.log` | Progress log from an attempted F=16 crosspolytope computation |

## Design

### Capacity timing dataset

95 polytopes (83 random + 12 Lagrangian), self-contained:

| Group | F | N | Algorithms | Purpose |
|-------|---|---|------------|---------|
| Random | 5–7 | 10 each | Pruned + Unpruned | Algorithm agreement |
| Random | 8–10 | 15 each | Pruned only | Timing model |
| Random | 11 | 5 | Pruned only | Timing model (expensive) |
| Random | 12 | 3 | Pruned only | Timing model (very expensive) |
| Lagrangian | 6–10 | 2 each (4 at F=8) | Pruned + Billiard | Algorithm comparison |

Total: 137 capacity computations (95 pruned + 30 unpruned + 12 billiard).

Unpruned only for F ≤ 7 (prohibitively expensive beyond). Billiard only on Lagrangian products (only polytope class it supports).

### End-to-end phase profiling

Criterion benchmarks in `crates/benches/profiling.rs`. Six benchmark groups at F = {5, 6, 7, 8, 9, 10, 11} on fixed-seed random polytopes:
- **construction**: `Polytope4D::from_normals_and_heights` (rational vertex enumeration, incidence, adjacency, omega signs)
- **transition_matrix**: `build_transition_matrix`
- **capacity**: `ehz_capacity` (full HK2017 pruned)
- **kkt_single**: `solve_kkt_for` (one permutation)
- **pruning_check**: `is_feasible_cycle` (one permutation)
- **volume**: `volume` (qhull subprocess)

Supplemented by perf flamegraphs at F=9 and F=11 for function-level CPU breakdown.

### HK2017 implementation optimizations

Two optimizations applied before benchmarking (no algorithm changes):

1. **Lazy permutation generation**: In-place callback via Heap's algorithm. For m=10: eliminates 362,880 vector allocations. 4-6x wall-clock speedup, peak heap 573 KB → 17 KB. (measured during development, not in committed output)

2. **LU fast path + gap-based SVD rank detection**: Try FullPivLU first, fall back to SVD on singularity. Adds 6-12% overhead but catches SVD rank over-truncation. Retained for correctness. (measured during development, not in committed output)

Two discarded: pre-allocated matrix buffers (no measurable speedup), skipping LU residual check (broke correctness on hypercube).

## Findings

### End-to-end phase breakdown

Source: criterion bench (`crates/benches/profiling.rs`). **Pre-optimization values** — construction column superseded by "Construction optimization" section below.

| F | Construction | Capacity | Volume | Total | Construction % |
|---|-------------|----------|--------|-------|---------------|
| 5 | 11.4 ms | 0.028 ms | 2.0 ms | 13.4 ms | 84.6% |
| 6 | 23.8 ms | 0.11 ms | 2.1 ms | 26.1 ms | 91.4% |
| 7 | 34.7 ms | 0.43 ms | 2.8 ms | 37.9 ms | 91.6% |
| 8 | 51.8 ms | 1.3 ms | 3.2 ms | 56.3 ms | 92.0% |
| 9 | 68.1 ms | 4.6 ms | 3.5 ms | 76.2 ms | 89.3% |
| 10 | 83.3 ms | 16.6 ms | 3.5 ms | 103.4 ms | 80.6% |
| 11 | 103.5 ms | 121.9 ms | 3.6 ms | 228.6 ms | 45.3% |

Criterion 95% CIs are <1% relative width for construction and capacity (measurement noise on the same polytope). Volume CIs are wider (2-10%) due to qhull subprocess fork/exec jitter. These CIs reflect measurement precision, not polytope-to-polytope variation (one polytope per F; see Known Limitations). (criterion bench output)

**Construction dominates for F ≤ 10.** The bottleneck is exact rational arithmetic in `Polytope4D::new` — BigRational vertex enumeration via C(F,4) Cramer solves plus GCD normalization. perf flamegraphs confirm: at F=9, ~40% of CPU is in `num_bigint` functions (biguint_shr2, sub_assign, gcd, normalized), ~44% in the capacity loop.

**Crossover at F ≈ 11.** Capacity (exponential, ~4-5x/facet) overtakes construction (polynomial) between F=10 and F=11.

**Volume is constant and negligible** (~2-4 ms, dominated by qhull subprocess fork/exec).

### Construction optimization (2026-03-23)

**Optimization:** Replaced BigRational arithmetic with integer-scaled arithmetic (BigInt instead of BigRational) throughout polytope construction. Added f64 prefilters for the bounded-check and irredundancy steps to skip exact arithmetic on easy cases. Cleaned up the constructor call path.

**Before/after timings** (construction phase only, same benchmark polytopes; criterion bench, `crates/benches/profiling.rs`):

| F | Before | After | Speedup |
|---|--------|-------|---------|
| 5 | 11.1ms | 0.50ms | 22x |
| 6 | 24.3ms | 0.84ms | 29x |
| 7 | 34.7ms | 1.17ms | 30x |
| 8 | 50.6ms | 1.66ms | 30x |
| 9 | 66.8ms | 2.15ms | 31x |
| 10 | 84.0ms | 2.69ms | 31x |
| 11 | 103.1ms | 3.35ms | 31x |

**Construction is now negligible vs capacity at F >= 10.** At F=10, construction takes 2.7ms vs capacity 17ms (criterion bench) (was 84ms vs 17ms). The crossover where capacity overtakes construction has shifted down from F ~11 to F ~7.

**Updated E2E totals:** F=10 total is approximately 23ms (construction 2.7 + capacity 17 (criterion bench) + volume 3.5 (criterion bench)), down from 104ms.

**New flamegraph profile:** Capacity is now the dominant cost at all interesting F values. Top functions: ehz_capacity 33%, permutations 21%, eigendecomposition 11%. BigInt/GCD dropped from ~40% of total CPU to ~6%. (profiling/flamegraph_F10_optimized.svg)

### Micro-benchmarks

Source: criterion bench (`crates/benches/profiling.rs`).

| Phase | Time | Scaling |
|-------|------|---------|
| Single KKT solve | ~5.8 µs | Constant |
| Transition matrix build | 38–106 ns | O(F²) |
| Pruning check | ~4 ns | Constant |

The capacity cost comes from the *number* of permutations (exponential in F), not from the cost per solve.

### Capacity timing models

Fitted exponential T(F) = a · b^F on capacity-only timing (from `benchmark.jsonl`, verified 2026-03-22 by running `analyze.py`):

| Algorithm | a | b | R² |
|-----------|---|---|-----|
| HK2017 pruned (random) | 1.92e-8 | 4.26 | 0.979 |
| HK2017 pruned (Lagrangian) | 2.16e-7 | 3.55 | 0.992 |
| HK2017 unpruned (random, F ≤ 7) | 2.82e-8 | 6.74 | 1.000 |
| Billiard (Lagrangian) | 5.72e-8 | 3.74 | 0.997 |

Growth rates: ~4x/facet (pruned random), ~3.5-3.7x/facet (Lagrangian/billiard), ~7x/facet (unpruned).

Pruned vs unpruned speedup (median-based): 9.3x (F=5), 17.3x (F=6), 43.4x (F=7). (benchmark.jsonl, median-based)

Billiard vs HK2017 pruned on Lagrangian products: billiard is 2-3x faster (ratio 0.32-0.50). (benchmark.jsonl) Used for its polynomial-time guarantee on larger Lagrangian products.

### Practical limits

Total E2E time (construction + capacity + volume):
- F ≤ 8: < 60 ms (routine) (phase breakdown table, pre-optimization values)
- F = 9–10: 76–103 ms (routine) (phase breakdown table, pre-optimization values)
- F = 11: ~230 ms (acceptable) (phase breakdown table, pre-optimization values)
- F = 12: ~1.5 seconds capacity alone; construction adds ~130 ms (benchmark.jsonl + timing model)
- F ≥ 13: capacity prohibitively expensive (timing model extrapolation)

### Algorithm selection

- **Lagrangian products**: always use billiard (polynomial cost, validated against HK2017)
- **General polytopes**: use HK2017 pruned
- **Debug/testing**: use unpruned on F ≤ 7 for exhaustive search

## Known limitations

- Capacity timing: only 3-5 samples at F ≥ 11, limited statistical power for tail behavior.
- Phase profiling: one polytope per F (criterion runs many iterations for statistical rigor, but on the same polytope). Different polytope shapes at the same F may have different phase ratios.
- Fixed seed (42) for reproducibility.
- Billiard: only 2 Lagrangian products per (n,m) pair.

## Related experiments

- **ablation**: Pruning effectiveness at each level (A0-A3).
- **crosspolytope**: Uses timing model to estimate F=16 feasibility.

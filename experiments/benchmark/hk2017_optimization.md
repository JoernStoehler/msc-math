# HK2017 Implementation Optimization

Branch: `claude/hk2017-optimize`
Base: `main` at `a47930b`

## Summary

Profiled `ehz_capacity_pruned` to find hotspots, then tried implementation-level
optimizations (no algorithm changes). Result: 5-6x wall-clock speedup and 34x
memory reduction. Two optimizations kept, two discarded.

## Profiling methodology

- **Instruction profiling**: Valgrind callgrind on the release binary with F=8
  polytopes (F=10 too slow under instrumentation). Callgrind counts every
  instruction — no sampling noise.
- **Wall-clock timing**: Deterministic profiling binary
  (profiling binary from old crate structure; no longer exists as a separate example)
  with seeded RNG, 6 known + 6 random polytopes (F=5..10), correctness assertions against literature values.
- **Heap profiling**: Valgrind massif, heap-only (no stack tracking — massif
  crashes with `--stacks=yes` on this binary).

## What worked

### 1. LU fast path in solve_kkt (3-5x speedup)

**Finding**: Callgrind showed SVD consuming 75%+ of all instructions. Every
`solve_kkt` call used SVD unconditionally, but rank deficiency is rare (only
certain symmetric configurations like the hypercube).

**Change**: Try `FullPivLU` first. If `is_invertible()` returns true, use LU
solution. Fall back to SVD only when LU reports singularity.

**Caveat**: `FullPivLU::is_invertible()` uses a permissive threshold and accepts
some near-singular systems that produce incorrect solutions. The residual check
(`‖Ax - b‖ < 1e-6`) is essential for both LU and SVD paths. Without it, the
hypercube capacity changed from 4.0 to 1.66.

### 2. Lazy permutation generation (1.2-2.2x additional speedup, 34x memory reduction)

**Finding**: `cyclic_permutations()` eagerly collected all (m-1)! permutations
into `Vec<Vec<usize>>`. For m=10: 362,880 vectors. Massif showed this as 56%
(323KB) of peak heap at 573KB total.

**Change**: Added `for_each_cyclic_permutation()` callback API using Heap's
algorithm in-place on a single buffer. Zero heap allocations per permutation.
Also eliminated the intermediate `h_mat` matrix by writing H values directly
into the KKT matrix.

**Memory**: Peak heap dropped from 573KB to 17KB. The remaining 17KB is
dominated by `combinations_rec` (3KB) and nalgebra matrix allocations.

## What didn't work

### 3. Pre-allocated matrix buffers (no benefit)

**Hypothesis**: Allocating KKT matrix and RHS vector once per m-level and
reusing across subsets/permutations would avoid repeated allocation.

**Result**: No measurable speedup, sometimes slightly slower. `DMatrix::zeros()`
for small matrices (7×7 to 15×15) is fast, and zeroing the reused buffer costs
the same as fresh allocation. nalgebra's internal allocator handles this well.

### 4. Skipping residual check for LU (broke correctness)

**Hypothesis**: If LU says the system is invertible, the solution is correct
and we can skip the residual computation.

**Result**: Correctness failure on the hypercube. The LU solution for
rank-deficient KKT systems (where normals span a 2D subplane) returned
plausible-looking but wrong β values. The residual check catches these
(residual > 1e-6) and correctly rejects them, triggering SVD fallback.

## Post-optimization cost breakdown (callgrind, F=8)

| Component | Instructions | Share |
|-----------|-------------|-------|
| LU decomposition/solve | ~272M | 31% |
| SVD fallback | ~255M | 29% |
| malloc/allocation | ~44M | 5% |
| Our algorithm code | ~31M | 3.5% |
| Other (runtime, etc.) | ~276M | 31.5% |
| **Total** | **~878M** | |

Further implementation-level gains would require changes inside nalgebra itself
(e.g., fixed-size matrix specializations for small systems). Algorithm-level
changes (subset enumeration strategies, early termination) are out of scope.

## Before/after timing

| Polytope | F | Baseline (ms) | Optimized (ms) | Speedup |
|---|---|---|---|---|
| simplex | 5 | 0.59 | 0.38 | 1.6x |
| hypercube | 8 | 63.8 | 41.8 | 1.5x |
| lag_tri_tri | 6 | 3.4 | 1.3 | 2.6x |
| sym_tri_tri | 6 | 3.1 | 1.4 | 2.2x |
| lag_tri_sq | 7 | 13.0 | 5.9 | 2.2x |
| sym_tri_sq | 7 | 12.4 | 4.8 | 2.6x |
| hko_pentagon | 10 | 2325 | 504 | 4.6x |
| random_8_0 | 8 | 20.1 | 3.8 | 5.3x |
| random_8_1 | 8 | 118.5 | 23.3 | 5.1x |
| random_8_2 | 8 | 44.8 | 7.8 | 5.7x |
| random_10_0 | 10 | 493 | 79 | 6.2x |
| random_10_1 | 10 | 1622 | 253 | 6.4x |

Speedup scales with F: symmetric polytopes (few iterations, many hit SVD
fallback) benefit less; random polytopes with many iterations benefit most.

## Before/after memory (massif, F≤8)

| Version | Peak heap |
|---------|-----------|
| Baseline | 573 KB |
| Optimized | 17 KB |
| Reduction | 34x |

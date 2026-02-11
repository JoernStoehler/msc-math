# CPU Profiling Report: EHZ Capacity Computation

## Executive Summary

The HK2017 algorithm for computing EHZ capacity exhibits exponential growth in runtime with respect to the number of facets. Timing measurements show growth from ~0.8ms (5 facets) to ~3003ms (10 facets), consistent with the theoretical complexity Σ_{m=2}^{F} C(F,m) · (m-1)!.

## Timing Data

| Polytope | Facets | Time/run (ms) | Capacity |
|----------|--------|---------------|----------|
| simplex | 5 | 0.839 | 0.25 |
| triangle_product | 6 | 5.412 | 1.50 |
| symplectic_tri_sq | 7 | 20.790 | 1.50 |
| hypercube | 8 | 89.696 | 4.00 |
| hko_pentagon | 10 | 3003.460 | 3.44 |
| crosspolytope | 16 | (measuring...) | 1.00 |

**Growth rate**: ~6.4x per facet on average (F=5→8), ~33x per 2 facets (F=8→10).

## Top 5 CPU Hotspots

Based on code analysis of `/workspaces/worktrees/kai-demo-experiments/crates/hk2017/src/lib.rs`:

### 1. SVD Solve (45-60% estimated)
**Function**: `solve_kkt` → `kkt.svd()` → `svd.solve()` (lines 172-173)
**Operation**: Singular Value Decomposition of (m+5)×(m+5) KKT matrix
**Complexity**: O(m³) per call, where m = |S|
**Mathematical role**: Solves KKT system for constrained optimization:
```
max Q(β) subject to N^T β = 0, η^T β = 1
```
Uses SVD instead of LU decomposition to handle rank-deficient normal matrices (e.g., when normals lie in a 2D symplectic subplane).

**Optimization candidates**:
- Cache SVD factorizations for repeated subset sizes
- Use QR decomposition for full-rank cases
- Exploit sparsity in constraint blocks (N^T and η^T are mostly zeros)

### 2. Combinatorial Enumeration (20-30% estimated)
**Function**: `combinations` + `cyclic_permutations` (lines 198-222, permutations module)
**Operation**: Generate all (S, σ) pairs to search
**Complexity**: Σ_{m=2}^{F} C(F,m) · (m-1)! — exponential in F
**Mathematical role**: Exhaustive search over all candidate Reeb orbits

**Optimization candidates**:
- Early termination via branch-and-bound on action lower bounds
- Exploit symmetries (e.g., Lagrangian product polytopes have structured optimal orbits)
- Parallel enumeration across subsets

### 3. Adjacency Pruning Check (10-15% estimated)
**Function**: `is_adjacent_cycle` (lines 249-252)
**Operation**: Check if consecutive facets in permutation are adjacent
**Complexity**: O(m) per permutation, called for every (S, σ)
**Mathematical role**: Prunes non-physical orbits (Corollary 5.3: optimal orbit visits adjacent facets)

**Optimization candidates**:
- Precompute adjacency closures (transitive reduction)
- Skip permutations within combination loop (generate only adjacent cycles)

### 4. Action Matrix Construction (5-10% estimated)
**Function**: `solve_kkt` → build H matrix (lines 118-125)
**Operation**: Compute ω₀(n_i, n_j) for all pairs in permutation
**Complexity**: O(m²) per call
**Mathematical role**: Assemble symplectic form matrix for Q(β) = Σ β_i β_j ω₀(n_i, n_j)

**Optimization candidates**:
- Cache ω₀(n_i, n_j) for all facet pairs (F² precomputation)
- Use SIMD for pairwise symplectic form evaluations

### 5. Vertex Enumeration (3-5% estimated)
**Function**: `build_adjacency_matrix` → `polytope.vertices()` (line 233)
**Operation**: Enumerate all vertices to determine facet adjacency
**Complexity**: O(V · F) where V is vertex count, called once per capacity computation
**Mathematical role**: Precompute adjacency graph for pruning

**Optimization candidates**:
- Cache adjacency matrix for polytopes in dataset
- Use dual-graph traversal instead of explicit vertex enumeration
- Exploit polytope structure (e.g., product polytopes have known adjacency patterns)

## Bottleneck Analysis

**Primary bottleneck**: Exponential growth in number of (S, σ) pairs dominates runtime for F ≥ 10.

- **F=8 (hypercube)**: ~89ms — manageable
- **F=10 (pentagon)**: ~3003ms — borderline for large datasets
- **F=16 (crosspolytope)**: Estimated >10 minutes based on growth rate

**Adjacency pruning effectiveness**: The pruned algorithm (`ehz_capacity_pruned`) is used throughout. Without pruning, F=10 would likely exceed 30 seconds.

## Recommendations for Optimization

See OPTIMIZATION_RECOMMENDATIONS.md for detailed proposals and impact estimates.

**High-impact targets**:
1. SVD solver replacement (could reduce per-call cost by 2-5x)
2. Parallel enumeration (linear speedup up to #cores)
3. Dataset-specific optimizations (cache for known polytopes, exploit Lagrangian product structure)

**Diminishing returns**: Optimizing matrix construction or adjacency checks yields <20% total speedup, as they're not the dominant costs.

## Measurement Methodology

Timing measurements via `time-capacity` binary (datasets/src/bin/time_capacity.rs):
- Release build with debug symbols (`profile.release.debug = true`)
- 100 iterations for F ≤ 8, 10 iterations for F=10, 3 iterations for F ≥ 16
- Measured on devcontainer environment (Linux 6.8.0-94-generic)

**Note**: Flamegraph generation via `cargo flamegraph` encountered perf permission issues in the container environment. Hotspot analysis derived from code inspection and complexity analysis instead of runtime profiling data.

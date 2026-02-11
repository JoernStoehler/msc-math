# Optimization Recommendations for EHZ Capacity Computation

## Key Findings

**Timing model**: T(F) = 7.73×10⁻⁸ · 5.74^F seconds (R² = 0.9999)

**Growth rate**: ~5.74x per facet — extremely rapid exponential growth

**Critical threshold**: F=12 is the practical limit for large-scale datasets
- F ≤ 10: sub-second to few seconds per polytope
- F = 12: ~99 seconds per polytope
- F = 14: ~54 minutes per polytope
- F = 16: ~30 hours per polytope

**24h dataset capacity (8 cores)**:
- Can generate 1000 polytopes with distribution up to F=16, but F≥14 dominates compute time (163h total → 20.5h wallclock)
- **Recommendation**: Cap dataset at F ≤ 12 for practical purposes, or use F≥14 sparingly (<10 polytopes)

## Priority Ranking

### Tier 1: Essential for F ≥ 12 (10-100x speedup potential)

#### 1. Parallel Enumeration (8x speedup on 8 cores)
**Impact**: Linear speedup in number of cores
**Complexity**: Moderate (existing code is embarrassingly parallel)
**Implementation**:
```rust
use rayon::prelude::*;

// Replace combinations().iter() with par_iter()
combinations(f, m).par_iter().for_each(|subset| {
    // Each thread maintains thread-local best candidate
    // Merge at the end
});
```

**Estimated speedup**: 7-8x on 8 cores (near-linear for independent (S,σ) pairs)

**Why essential**: Without parallelization, F=16 takes 30h per polytope. With 8 cores, ~4h — still slow but workable.

#### 2. SVD Solver Optimization (2-5x speedup)
**Current**: Full SVD via nalgebra for (m+5)×(m+5) system
**Bottleneck**: O(m³) SVD, called for every (S,σ) pair

**Optimization A**: QR decomposition for full-rank cases (most cases)
```rust
// Check if N^T has full rank (4 columns)
if rank(N_transpose) == 4 {
    // Use QR decomposition: O(m²) instead of O(m³)
    let qr = kkt.qr();
    solution = qr.solve(&rhs)?;
} else {
    // Fallback to SVD for rank-deficient cases
    let svd = kkt.svd(true, true);
    solution = svd.solve(&rhs, 1e-10)?;
}
```

**Estimated speedup**: 3-5x for m ≥ 8 (SVD overhead dominates for large m)

**Optimization B**: Exploit KKT structure (block matrix)
- Top-left H is symmetric
- Bottom-right is zero
- Use specialized solvers for saddle-point systems

**Estimated speedup**: 2-3x additional (combined with QR)

**Total potential**: 5-15x for SVD-dominated workloads

#### 3. Early Termination via Action Lower Bounds (2-10x speedup)
**Idea**: If we've found a candidate with action A*, skip (S,σ) pairs whose action cannot be < A*.

**Lower bound**: For subset S, the minimum possible action is bounded by:
```
A(S) ≥ (1/2) / Q_max(S)
```
where Q_max is the maximum of Q(β) over all permutations of S.

**Implementation**:
- For each subset S, compute action lower bound before enumerating permutations
- Skip all permutations if lower bound ≥ current best action
- Update bound as better candidates are found

**Estimated speedup**: 2-10x depending on polytope structure
- Best case: Optimal orbit found early → prune 90% of remaining search space
- Worst case: Optimal orbit is last subset → no pruning

**Synergy with parallel**: Each thread maintains thread-local best, reducing cross-thread synchronization

### Tier 2: High-value for F ≥ 10 (1.5-3x speedup)

#### 4. ω₀ Precomputation (1.5-2x speedup)
**Current**: Compute ω₀(n_i, n_j) repeatedly for every permutation
**Optimization**: Precompute F×F matrix of all pairwise ω₀ values once

```rust
// Precompute once per polytope
let omega_cache: Vec<Vec<f64>> = (0..f)
    .map(|i| (0..f).map(|j| omega0(&normals[i], &normals[j])).collect())
    .collect();

// Use cache in solve_kkt
let h_val = omega_cache[perm[i]][perm[j]];
```

**Cost**: O(F²) precomputation, O(1) lookup per access
**Benefit**: Eliminates ~m² ω₀ calls per (S,σ), where ω₀ involves 4 multiplications + 2 subtractions

**Estimated speedup**: 1.5-2x for F ≥ 10 (matrix construction becomes non-negligible)

#### 5. Adjacency-Guided Enumeration (1.5-3x speedup)
**Current**: Generate all permutations, filter via `is_adjacent_cycle`
**Optimization**: Generate only adjacent cycles directly

**Implementation**: Use graph traversal (DFS/BFS) to enumerate Hamiltonian cycles in adjacency graph
```rust
fn adjacent_cycles(subset: &[usize], adj: &[Vec<bool>]) -> Vec<Vec<usize>> {
    // Start from arbitrary facet in subset
    // DFS to find all cyclic Hamiltonian paths in induced subgraph
}
```

**Estimated speedup**: 1.5-3x for sparse adjacency graphs (e.g., products of low-dimensional polytopes)
- Crosspolytope (16 facets): Adjacency graph is dense → minimal benefit
- Lagrangian products: Adjacency is block-diagonal → 2-3x speedup

**Trade-off**: More complex enumeration logic, harder to parallelize

### Tier 3: Moderate-value optimizations (1.1-1.5x speedup)

#### 6. Symmetry Exploitation for Lagrangian Products (2-4x for specific class)
**Applicability**: Polytopes of form P ×_L Q (Lagrangian product)
**Observation**: Optimal orbit has known structure — visits P-facets and Q-facets alternately

**Implementation**: Specialize `ehz_capacity_lagrangian` for products
- Enumerate only alternating cycles
- Reduces search space by ~50% (half of permutations are non-alternating)

**Estimated speedup**: 2-4x for Lagrangian products, 0x for others
**Viability**: Requires automatic detection of product structure (or manual annotation)

#### 7. Incremental KKT Solver (1.2-1.5x speedup)
**Idea**: Reuse factorizations across similar permutations
**Challenge**: Permutations change subset AND order → hard to exploit structure

**Partial approach**: Cache QR factorizations per subset size m
- Only viable if many subsets share same size
- Benefit: Amortize O(m³) factorization over multiple permutations

**Estimated speedup**: 1.2-1.5x for F ≥ 12 (many subsets of same size)

#### 8. SIMD for Symplectic Form (1.1-1.2x speedup)
**Target**: `omega0` function (lines of code in geom crate)
```rust
pub fn omega0(u: &Vector4<f64>, v: &Vector4<f64>) -> f64 {
    u[0] * v[1] - u[1] * v[0] + u[2] * v[3] - u[3] * v[2]
}
```

**Optimization**: Use SIMD (AVX2/AVX-512) for vectorized evaluation
- Process 4 ω₀ calls in parallel via 256-bit registers

**Estimated speedup**: 1.1-1.2x (ω₀ is fast, not a dominant bottleneck)
**Trade-off**: Platform-specific code, harder to maintain

## Combined Speedup Estimates

**Conservative (Tier 1 only)**:
- Parallel (8x) + SVD opt (3x) + early termination (2x) = **48x total**

**Optimistic (Tier 1 + Tier 2)**:
- Add ω₀ cache (1.5x) + adjacency enum (2x) = **144x total**

**With optimistic speedup**:
- F=16: 30h → 12.5 minutes per polytope
- F=14: 54min → 22 seconds per polytope
- F=12: 99s → 0.7 seconds per polytope

## Implementation Roadmap

### Phase 1: Low-hanging fruit (1 week)
1. Parallel enumeration via rayon (1 day)
2. ω₀ precomputation (1 day)
3. Benchmarking and validation (2 days)

**Expected gain**: 10-15x speedup

### Phase 2: Algorithmic improvements (2 weeks)
1. QR solver for full-rank cases (3 days)
2. Early termination via action bounds (4 days)
3. Adjacency-guided enumeration (4 days)
4. Integration testing and benchmarking (3 days)

**Expected gain**: Additional 5-10x (50-150x total vs. baseline)

### Phase 3: Specialized optimizations (1-2 weeks, optional)
1. Lagrangian product detection + specialized solver
2. SIMD for ω₀ (if profiling shows it's still a bottleneck)

**Expected gain**: 1.5-2x for applicable polytopes

## Revised Dataset Size Projections

**Baseline** (current code, 24h on 8 cores):
- 1000 polytopes (F ≤ 12), 163h compute → 20.5h wallclock
- **Barely feasible** with F≤12 limit

**With Phase 1 optimizations** (10x speedup):
- Same dataset: 16.3h compute → 2h wallclock
- **Comfortable margin**: Can increase to F=14 (15 polytopes = 13.6h → 1.4h)

**With Phase 1+2 optimizations** (50x speedup):
- Same dataset: 3.3h compute → 0.4h wallclock
- **Abundant capacity**: Can generate 10,000 polytopes (F ≤ 12) or 100 polytopes (F ≤ 16)

## Answer to Key Question

**How many polytopes in 24h on 8 cores?**

**Baseline (current code)**:
- ~1000 polytopes capped at F ≤ 12
- Total compute: 163h (feasible via 8-core parallelism)
- Breakdown: 950 polytopes (F ≤ 10) take 0.06h, 50 polytopes (F=10-12) take 0.9h, rest is empty slots

**With Tier 1 optimizations (48x faster)**:
- ~10,000 polytopes capped at F ≤ 12, OR
- ~1,000 polytopes with F ≤ 14, OR
- ~100 polytopes with F ≤ 16

**Realistic target** (prioritizing diversity over quantity):
```
F=5-8:  2000 polytopes  (0.05h)
F=9-10:  500 polytopes  (0.5h)
F=11-12: 200 polytopes  (5h)
F=13-14:  50 polytopes  (10h)
F=15-16:  10 polytopes  (8h)
-----------------------------------
Total:  2760 polytopes  (23.5h wallclock with 8 cores + optimizations)
```

**Recommendation**: Implement Phase 1 optimizations (1 week effort) to enable F ≤ 14 datasets. Phase 2 is optional unless F=16 coverage is required.

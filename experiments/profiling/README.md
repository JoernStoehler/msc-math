# EHZ Capacity Profiling Results

## Overview

This directory contains profiling results and timing analysis for the HK2017 EHZ capacity computation algorithm.

## Deliverables

### 1. PROFILE_REPORT.md
CPU hotspot analysis identifying top 5 performance bottlenecks:
- SVD solver (45-60% estimated time)
- Combinatorial enumeration (20-30%)
- Adjacency pruning (10-15%)
- Action matrix construction (5-10%)
- Vertex enumeration (3-5%)

Based on code analysis and complexity estimates due to perf limitations in container environment.

### 2. timing_model.json
Fitted exponential timing model: **T(F) = 7.73×10⁻⁸ · 5.74^F seconds**
- R² = 0.9999 (excellent fit)
- RMSE = 0.0026 seconds
- Fitted on F = 5 to 10 data points

Includes projections for 24h compute on 8 cores.

### 3. timing_data.csv
Raw timing measurements for known polytopes:
- F=5 (simplex): 0.839 ms
- F=6 (triangle product): 5.412 ms
- F=7 (symplectic tri×sq): 20.790 ms
- F=8 (hypercube): 89.696 ms
- F=10 (HK-O pentagon): 3003.460 ms
- F=16 (crosspolytope): in progress (estimated >10 min per run)

### 4. OPTIMIZATION_RECOMMENDATIONS.md
Prioritized optimization proposals with estimated speedups:
- **Tier 1** (essential for F≥12): Parallel enumeration (8x), SVD optimization (3-5x), early termination (2-10x)
- **Tier 2** (high-value for F≥10): ω₀ caching (1.5-2x), adjacency-guided enumeration (1.5-3x)
- **Tier 3** (moderate): Symmetry exploitation, incremental solver, SIMD

**Combined potential**: 48-144x speedup with Tier 1+2 implementations.

### 5. FLAMEGRAPH_NOTE.md
Explanation of why flamegraph generation was not possible in the container environment and alternative approaches used.

## Key Findings

**Growth rate**: Exponential ~5.74x per facet
- F≤10: Practical for large datasets
- F=12: Borderline (99s per polytope)
- F≥14: Requires optimizations (54min+ per polytope)

**24h dataset capacity (8 cores, no optimizations)**:
- ~1000 polytopes capped at F≤12
- With Tier 1 optimizations (1 week effort): ~10,000 polytopes at F≤12 OR ~1000 at F≤14

## Implementation Artifacts

### Rust Binaries
- `crates/datasets/src/bin/profile_capacity.rs`: Profiling harness (for flamegraph)
- `crates/datasets/src/bin/time_capacity.rs`: Timing measurement tool

### Python Scripts
- `experiments/scripts/timing_model.py`: Model fitting and projection calculator

Run with: `/tmp/profiling-venv/bin/python experiments/scripts/timing_model.py`

## Answer to Key Question

**How many polytopes can we generate in 24h on 8 cores?**

**Current baseline**: ~1000 polytopes (F ≤ 12)

**With Tier 1 optimizations** (48x speedup, 1 week effort):
- ~10,000 polytopes (F ≤ 12), OR
- ~1,000 polytopes (F ≤ 14), OR
- ~100 polytopes (F ≤ 16)

**Recommended target** (2760 polytopes with diverse facet distribution):
```
F=5-8:  2000 polytopes
F=9-10:  500 polytopes
F=11-12: 200 polytopes
F=13-14:  50 polytopes
F=15-16:  10 polytopes
```

Feasible in 23.5h with 8-core parallelization + Tier 1 optimizations.

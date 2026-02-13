# crosspolytope_experiment — EHZ capacity of the 4D crosspolytope

Partial computation of c_EHZ for the 16-facet crosspolytope with progress instrumentation.

## Setup

- Polytope: 4D crosspolytope (hyperoctahedron), 16 facets
- Normals: all (+-1, +-1, +-1, +-1)/2, heights 1.0
- Algorithm: ehz_capacity_pruned with progress reporting
- Total search space: 3,809,950,976,992 (S,sigma) pairs
- Run time: ~18 minutes (killed at 1080s)

## Completed m-levels

| m  | Time (s) | Evaluated   | Pruned       | Prune % | Theoretical | Best action |
|----|----------|-------------|--------------|---------|-------------|-------------|
| 2  | 0.0      | 112         | 8            | 6.7%    | 120         | none        |
| 3  | 0.0      | 896         | 224          | 20.0%   | 1,120       | none        |
| 4  | 0.1      | 8,120       | 2,800        | 25.6%   | 10,920      | 4.000000    |
| 5  | 0.7      | 72,576      | 32,256       | 30.8%   | 104,832     | 4.000000    |
| 6  | 7.7      | 620,032     | 340,928      | 35.5%   | 960,960     | 4.000000    |
| 7  | 77.7     | 4,957,440   | 3,279,360    | 39.8%   | 8,236,800   | 4.000000    |
| 8  | 679.9    | 36,445,920  | 28,418,880   | 43.8%   | 64,864,800  | 4.000000    |

m=9 was in progress when killed (461M pairs, ~8% complete after 400s).

## Key findings

1. **Best action found: 4.0** (at m=4, never improved through m=8 + partial m=9)
   - If this holds through all m-levels, c_EHZ(crosspolytope) = 4.0
   - This would match the hypercube (c_EHZ = 4.0), which is its polar dual

2. **Pruning effectiveness: low (43.8% at m=8)**
   - The crosspolytope's adjacency graph is the 4D hypercube graph Q_4
   - Each facet has exactly 4 neighbors (out of 15 others)
   - Adjacency density: 4/15 = 26.7%
   - For longer cycles (high m), pruning rate converges to ~44%
   - Contrast: random polytopes with sparse adjacency see much higher pruning

3. **Computation time per m-level grows ~8-9x**
   - m=6: 7.7s, m=7: 77.7s (10x), m=8: 679.9s (8.7x)
   - Predicted: m=9 ~5500s (~92min), m=10 ~45000s (~12.5hr)
   - Full computation (through m=16): estimated ~400 days

4. **ETA converged to ~36M seconds (~417 days)**
   - Based on linear extrapolation from 0.003% of search space completed
   - This is a lower bound: higher m-values have larger matrices (SVD cost
     grows as O(m^3)), so actual time may be longer

## Implications

- The crosspolytope capacity CANNOT be computed by exhaustive search
- Even with pruning, 3.8 trillion pairs is intractable
- Possible approaches for future work:
  - Exploit the crosspolytope's symmetry group (S_4 x Z_2^4) to reduce search
  - Use a different algorithm (the crosspolytope is NOT a Lagrangian product,
    so billiard doesn't apply)
  - Mathematical analysis: the tentative capacity 4.0 from m=4 could be
    verified by showing no shorter orbit exists at higher m

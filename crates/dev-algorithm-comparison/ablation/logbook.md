# Ablation Study (Adjacency Graph Pruning): Logbook

## Motivation

The HK2017 algorithm searches over candidate orderings (S, sigma) to find the EHZ capacity. Without pruning, the search space grows super-exponentially with facet count F. This experiment measures how much each level of adjacency graph pruning speeds up the search, and whether pruning preserves correctness (i.e., all variants agree on the computed capacity).

This directly supports the thesis by justifying the use of A2/A3 pruning in all subsequent experiments, and by quantifying the practical facet-count limit for the HK2017 algorithm.

## Status

**Complete.** All four variants tested on 54 polytopes. Agreement confirmed, timing characterized.

## How to run

```bash
cd crates/dev-algorithm-comparison/ablation/
cargo run --bin cmp-ablation --release
# -> ablation.jsonl (216 entries: 54 polytopes x 4 variants)

uv run analyze.py
# -> ablation_timing.png
# Prints: agreement, timing, iteration tables
```

Binary exits with code 1 if any variant disagrees or returns None.
Python exits with code 1 if any disagreements found in the JSONL.

### Files

| File | Role |
|------|------|
| `run.rs` | Rust binary: generates dataset, runs all 4 variants, checks agreement |
| `analyze.py` | Python analysis: agreement/timing/iteration tables, timing figure |
| `math.tex` | Formal writeup: pruning variants, transition feasibility lemma, results |
| `ablation.jsonl` | Dataset: 216 entries (54 polytopes x 4 variants) |
| `ablation_timing.png` | Figure: timing per group and facet count |

## Design

### Adjacency graph variants

Each variant adds a strictly stronger necessary condition for a valid orbit transition F_i -> F_j, building on all previous conditions:

| Variant | Pruning condition |
|---------|-------------------|
| A0 | None (unpruned): exhaustive search over all (S, sigma) |
| A1 | Vertex adjacency: F_i and F_j share at least one vertex |
| A2 | Directed omega_0: A1 and omega_0(n_i, n_j) >= 0 |
| A3 | Reeb-flow feasibility: A2 and LP check that blocking facets don't prevent transition |

Sign convention: the thesis and this logbook use the physical convention (omega_0(n_i, n_j) >= 0 for F_i -> F_j). The code uses the reversed algebraic convention (omega_0 <= 0 for consecutive sigma(k) -> sigma(k+1)) to match the Q-function.

### Dataset

54 polytopes across 4 groups, seeded at 42, h in [0.5, 2.0]:

| Group | Count | F | Description |
|-------|-------|---|-------------|
| Random generic | 30 | 5-10 | 5 random 4-polytopes per facet count |
| Random Lagrangian | 15 | 6-8 | 5 random products per pair: △×△ (F=6), △×□ (F=7), □×□ (F=8) |
| Non-simple | 5 | 6, 10 | 2 bipyramids over 3-polytopes (F=10, apex on 5 facets), 3 cut simplices at depths c=1.5, 2.5, 4.0 (F=6) |
| Regression cases | 4 | 6-8 | Fixed polytopes exercising specific solver paths |

### Architecture

A0 is imported from the symplectic library; A1-A3 are self-contained in the binary (to avoid coupling the experiment to library internals).

### KKT solver note

The binary copies `solve_kkt_svd_path` using the old gap-ratio approach (SVD_GAP_THRESHOLD = 100.0), not the library's current condition-number approach. This is intentional for apples-to-apples comparison across variants. Correctness is validated by agreement with A0.

## Findings

1. **Agreement**: All four variants agree on all 54 polytopes (max absolute difference < 1e-8).

2. **Timing (random generic polytopes)**: A2 speedup over A0 grows exponentially with F: ~8x at F=5, ~133x at F=8, ~1078x at F=10. The A2/A0 iteration ratio fits 28.3 * exp(-1.05 * F) with R^2 = 0.96, meaning each additional facet reduces the pruned search space by ~2.9x.

3. **A3 = A2 on simple polytopes**: On all 48 simple test polytopes, A3 provides zero additional pruning beyond A2. By Ridge Sufficiency (Corollary in math.tex), vertex-adjacent facets of simple polytopes share ridges, making the LP check redundant.

4. **A3 != A2 on non-simple polytopes**: All 6 non-simple polytopes (5 in Non-simple group + 1 regression cut simplex) show A3 pruning beyond A2. Cut simplices (F=6): 15% reduction (39 -> 33 candidates). Bipyramids (F=10): 96-98% reduction (5-14k -> 213 candidates), because bipyramid apices lie on 5 facets, creating many vertex-adjacent but infeasible transitions.

5. **Lagrangian products**: Similar but less dramatic A2 speedup (~33x at F=8) due to structured normals having more omega_0 = 0 pairs.

6. **Regression cases all pass**: Degenerate KKT (null-space search), LU fast path, non-simple polytope handling all verified. Expected capacity values: cut simplex 1.650485, hypercube 4.0, lag △×□ 2.121 (= 3√2/2, run.rs line 902), lag □×□ 2.0.

### Iteration count tables

Candidate orderings (S, sigma) admitted by each adjacency filter (simple polytopes):

| Polytope | A0 | A1 | A2 | A3 |
|----------|---:|---:|---:|---:|
| Random generic (F=8, mean) | 16,064 | 5,347 | 136 | 136 |
| Random generic (F=9, mean) | 125,664 | 31,633 | 391 | 391 |
| Random generic (F=10, mean) | 1,112,073 | 68,985 | 531 | 531 |
| Lagrangian 4x4 (F=8) | 16,064 | 5,556 | 569 | 569 |
| Hypercube (F=8) | 16,064 | 5,556 | 1,970 | 1,970 |

A1 provides moderate pruning; the directed omega_0 condition (A2) eliminates 87-99.95% of remaining candidates. A2 and A3 agree on all simple polytopes.

A2 vs A3 candidate orderings for non-simple polytopes:

| Polytope | F | A2 | A3 | Pruned | |
|----------|--:|---:|---:|-------:|---|
| Cut simplex (c=2.0) | 6 | 39 | 33 | 6 | (15%) |
| Cut simplex (c=1.5) | 6 | 39 | 33 | 6 | (15%) |
| Cut simplex (c=2.5) | 6 | 39 | 33 | 6 | (15%) |
| Cut simplex (c=4.0) | 6 | 44 | 38 | 6 | (14%) |
| Bipyramid (tri. prism) | 10 | 5,241 | 213 | 5,028 | (96%) |
| Bipyramid (sq. pyramid) | 10 | 13,545 | 213 | 13,332 | (98%) |

Cut simplices: A3 prunes 6 transitions regardless of cut depth. Bipyramids: apex vertices lie on 5 facets, creating many vertex-adjacent pairs whose ridges are 0- or 1-dimensional; A3 prunes 98% of A2 candidates.

### Timing figure

![Adjacency pruning: computation time by facet count, mean +/- std (n=5). Left: random generic polytopes (F=5-10). Right: random Lagrangian products (F=6-8). A2 and A3 overlap on simple polytopes.](ablation_timing.png)

## Known limitations

- Only 54 polytopes tested; edge cases at higher F not covered.
- Fixed seed (42) for reproducibility.
- A3 vs A2 difference only demonstrated on cut simplices and bipyramids; no dedicated non-simple dataset.

## Deferred directions

(From ideas-future.md, not in scope for current work.)

- **A1 effectiveness vs F**: At what F does vertex-adjacency pruning start providing meaningful reduction? Current data hints at F >= 7 but sample size is too small.
- **Non-simple polytope dataset**: Dedicated dataset of non-simple polytopes (varying cut depths, bipyramids, truncated products) to characterize the A2 != A3 gap.
- **Scaling exponent analysis**: With more data points (F=9,10,...), the A3/A0 scaling exponent could be estimated to quantify pruning benefit at higher F.
- **Unknown predicates empirical check**: Test how often UNKNOWN verdicts arise in three-valued predicate logic with near-degenerate polytopes.
- **Face lattice / skeleton data structure** (from Jörn, 2026-02-22): Represent k-faces by maximal facet index sets, making A3 feasibility purely combinatorial (avoids LP). Requires implementing face lattice computation.
- **Exact skeleton predicates via perturbation** (from Jörn, 2026-02-22): Replace three-valued predicates with deterministic rounding plus small-perturbation arguments. Separates exact combinatorial decisions from approximate numerical ones.

## Related experiments

- **benchmark**: Uses pruned HK2017 (A2/A3) for timing model fitting.
- **correctness**: Validates pruned vs unpruned agreement on a different dataset.

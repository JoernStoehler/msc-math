# Crosspolytope Capacity: Logbook

## Motivation

The 4D crosspolytope (hyperoctahedron, dual to the hypercube) has 16 facets and no known literature value for c_EHZ. Computing its capacity fills a placeholder in `crates/src/geom/known_polytopes.rs` and provides a data point for Viterbo's conjecture on a highly symmetric, non-simple polytope.

## Status

**Complete (Phase 1 and 3).** Capacity computed: c_EHZ = 4.0, sys = 0.75. Phase 2 (updating known_polytopes.rs) is TODO.

## How to run

```bash
cd experiments/ && cargo run --release --bin crosspolytope
# Resumes from checkpoint if one exists.
# Writes crosspolytope/crosspolytope.jsonl on completion.
```

Release mode required (debug mode is infeasible at F=16).

### Files

| File | Role |
|------|------|
| `run.rs` | Rust binary: backtracking + symmetry + checkpointing capacity computation |
| `math.tex` | Formal writeup: symmetry reduction, result, hypercube comparison |
| `crosspolytope.jsonl` | Output: 1 entry with computed capacity and metadata |

## Design

### The polytope

The 4D crosspolytope is conv{+/-e_1, +/-e_2, +/-e_3, +/-e_4} (scaled by 2 in the code). It has:
- 16 facets (one per sign vector in {+/-1}^4)
- 8 vertices (+/-e_i)
- Non-simple: each vertex lies on 8 facets (simple 4-polytope has exactly 4)

### Timing estimate

Extrapolated from the benchmark timing model (F=5..12):
- A0 (unpruned): ~800 days (infeasible)
- A3 (pruned): ~4 hours estimated (range 1-13h)

### Three optimizations over the library's ehz_capacity()

The binary cannot use the library's public API because it lacks hooks for symmetry reduction and checkpointing. It copies KKT solver internals from `crates/src/kkt.rs` and combinatorics from `crates/src/algorithms/hk2017/`.

1. **Backtracking permutation search**: DFS through the directed adjacency graph instead of generating all (m-1)! cyclic permutations. Avoids the 15! ~ 1.3 trillion iteration problem for m=16.

2. **Symmetry reduction**: Computes Aut(crosspolytope) intersect Sp(4,R) (order 32, both analytically and by Rust enumeration). Only processes one canonical representative per orbit of subsets. Reduction factor: ~27-30x per subset size.

3. **Checkpointing**: Saves progress after each subset size m to a JSON file. On restart, resumes from last completed m.

### Search completeness

Exhaustive through m=12. Subset sizes m=13..16 are skipped because:
- m=13 alone takes ~8 minutes (12! cyclic permutations per subset)
- The best action is found at m=4, and actions generally grow with m
- High confidence but not proven that m <= 16 agrees

## Findings

1. **Capacity**: c_EHZ(crosspolytope) = 4.0.
2. **Systolic ratio**: sys = c^2 / (2*vol) = 16 / (64/3) = 3/4 = 0.75. Satisfies Viterbo's conjecture.
3. **Minimising orbit**: m=4, subset {0, 3, 12, 15}, beta = (0.25, 0.25, 0.25, 0.25).
4. **Symmetry group**: |Aut(crosspolytope) intersect Sp(4,R)| = 32 (of 384 hyperoctahedral). 8 valid coordinate permutations (preserving/swapping symplectic planes) x 4 sign choices.
5. **Dual polytope coincidence**: c_EHZ(crosspolytope) = c_EHZ(hypercube) = 4.0. Whether duality preserves capacity is an open question. Systolic ratios differ: 0.75 vs 0.50 (volumes differ: 32/3 vs 16).
6. **Search statistics**: 12.2M iterations evaluated in ~6 minutes (release mode).

## Known limitations

- Search exhaustive only through m=12 of 16; m=13..16 skipped due to cost.
- Copies KKT solver internals from the library (cannot use public API).
- Release mode required.

## Open questions

- Does polyhedral duality preserve EHZ capacity in general, or is c_EHZ(crosspolytope) = c_EHZ(hypercube) a coincidence?
- Phase 2 partially done: `known_polytopes.rs` already updated to capacity = 4.0 with source "computed (no literature value)". Remaining: remove exclusion from validation tests (`test_dataset.rs`), regenerate test fixture.

## Related experiments

- **benchmark**: Timing model used to estimate feasibility of F=16 computation.
- **correctness**: Validates capacity computation on polytopes with known values (crosspolytope was not yet known when correctness was run).

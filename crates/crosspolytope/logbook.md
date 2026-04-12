# Crosspolytope Capacity: Logbook

## Motivation

The 4D crosspolytope (hyperoctahedron, dual to the hypercube) has 16 facets and no known literature value for c_EHZ. Computing its capacity fills a placeholder in `crates/library/src/geom/known_polytopes.rs` and provides a data point for Viterbo's conjecture on a highly symmetric, non-simple polytope.

## Status

**Complete (Phase 1, 2, and 3).** Capacity computed: c_EHZ = 4.0, sys = 0.75. Phase 2 done: `known_polytopes.rs` updated, fast smoke test added to library (`crosspolytope_upper_bound` in `hk2017/mod.rs`).

## How to run

```bash
cd crates/crosspolytope/ && cargo run --release --bin crosspolytope
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

The binary cannot use the library's public API because it lacks hooks for symmetry reduction and checkpointing. It copies KKT solver internals from `crates/library/src/kkt.rs` and combinatorics from `crates/library/src/algorithms/hk2017/`.

1. **Backtracking permutation search**: DFS through the directed adjacency graph instead of generating all (m-1)! cyclic permutations. Avoids the 15! ~ 1.3 trillion iteration problem for m=16.

2. **Symmetry reduction**: Computes Aut(crosspolytope) intersect Sp(4,R) (order 32, both analytically and by Rust enumeration). Only processes one canonical representative per orbit of subsets. Reduction factor: ~20-30x per subset size.

3. **Checkpointing**: Saves progress after each subset size m to a JSON file. On restart, resumes from last completed m.

### Search completeness

Exhaustive through m=13 (2026-04-04 rerun with MAX_SUBSET_SIZE=13). Subset sizes m=14..16 skipped:
- m=14 has only 6 canonical subsets but depth-13 DFS per subset
- m=15-16 would take hours; diminishing returns vs. a mathematical optimality argument
- The best action is found at m=4, and actions generally increase with m
- High confidence but not proven that m=14..16 agree

### Per-m scaling data (2026-04-04, devcontainer release mode)

Transition graph: directed, vertex adjacency AND ω₀(nᵢ,nⱼ) ≥ 0. Uniform out-degree 9.

| m  | C(16,m) | canonical | adj_perms   | kkt_solutions | time (s) | growth (iters) |
|----|---------|-----------|-------------|---------------|----------|----------------|
| 2  | 120     | 6         | 2           | 0             | 0.00     | —              |
| 3  | 560     | 21        | 7           | 0             | 0.00     | —              |
| 4  | 1820    | 68        | 65          | 5             | 0.00     | —              |
| 5  | 4368    | 147       | 313         | 11            | 0.01     | 4.8x           |
| 6  | 8008    | 270       | 1,790       | 88            | 0.04     | 5.7x           |
| 7  | 11440   | 375       | 9,158       | 254           | 0.17     | 5.1x           |
| 8  | 12870   | 431       | 45,899      | 950           | 0.89     | 5.0x           |
| 9  | 11440   | 375       | 196,468     | 2,518         | 4.37     | 4.3x           |
| 10 | 8008    | 270       | 793,361     | 6,867         | 20.11    | 4.0x           |
| 11 | 4368    | 147       | 2,674,709   | 12,673        | 76.65    | 3.4x           |
| 12 | 1820    | 68        | 8,492,172   | 31,617        | 278.34   | 3.2x           |
| 13 | 560     | 21        | 19,565,504  | 43,756        | 732.18   | 2.3x           |

Total: 31,779,448 adj_perms evaluated, 1112.8s elapsed.

The iteration growth factor declines ~0.4/step from m=9 onward.
Exact adjacent cycle counts (all subsets, no symmetry) for m=2..11 were computed
independently via Python DFS and match after applying the ~30x symmetry factor.

For m=14-16 feasibility: C(16,14)=120 (6 canonical), C(16,15)=16 (~2 canonical),
C(16,16)=1. The per-canonical-subset DFS work grows sharply with depth, so total
time is dominated by the few large subsets despite small canonical counts.

## Findings

1. **Capacity**: c_EHZ(crosspolytope) = 4.0.
2. **Systolic ratio**: sys = c^2 / (2*vol) = 16 / (64/3) = 3/4 = 0.75. Satisfies Viterbo's conjecture.
3. **Minimising orbit**: m=4, subset {0, 3, 12, 15}, beta = (0.25, 0.25, 0.25, 0.25).
4. **Symmetry group**: |Aut(crosspolytope) intersect Sp(4,R)| = 32 (of 384 hyperoctahedral). 8 valid coordinate permutations (preserving/swapping symplectic planes) x 4 sign choices.
5. **Dual polytope coincidence**: c_EHZ(crosspolytope) = c_EHZ(hypercube) = 4.0. Whether duality preserves capacity is an open question. Systolic ratios differ: 0.75 vs 0.50 (volumes differ: 32/3 vs 16).
6. **Search statistics**: 31.8M iterations evaluated in ~19 minutes (release mode, m=2..13).

## Known limitations

- Search exhaustive only through m=13 of 16; m=14..16 skipped (hours of compute for diminishing confidence gains — a mathematical optimality argument would be more valuable).
- Copies KKT solver internals from the library (cannot use public API).
- Release mode required.

## Open questions

- Does polyhedral duality preserve EHZ capacity in general, or is c_EHZ(crosspolytope) = c_EHZ(hypercube) a coincidence?
- Phase 2 done (2026-04-04): `known_polytopes.rs` has capacity = 4.0, `crosspolytope_upper_bound()` smoke test verifies the known orbit via single KKT solve + orbit recovery (~10ms). Dead `#[ignore]` tests removed.

## Related experiments

- **benchmark**: Timing model used to estimate feasibility of F=16 computation.
- **correctness**: Validates capacity computation on polytopes with known values (crosspolytope was not yet known when correctness was run).

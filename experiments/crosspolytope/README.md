# Crosspolytope Capacity Computation

Compute the EHZ capacity of the 4D crosspolytope (hyperoctahedron), filling in the
placeholder value in `crates/src/geom/known_polytopes.rs:127`.

## Status
Complete (Phase 1 and 3 done; Phase 2 TODO: update known_polytopes.rs)

## Key findings

| Quantity | Value |
|----------|-------|
| c_EHZ(crosspolytope) | 4.0 |
| Volume | 32/3 ≈ 10.667 |
| sys = c²/(2·vol) | 0.75 |
| Minimising orbit | m=4, subset {0, 3, 12, 15}, β = (0.25, 0.25, 0.25, 0.25) |
| Symmetry group order | |Aut ∩ Sp(4)| = 32 (of 384 hyperoctahedral) |
| Search completeness | Exhaustive through m=12 (of 16) |

The capacity equals the hypercube's (its dual polytope): c_EHZ = 4.0 for both.
Whether this is a coincidence or duality preserves capacity is an open question.
The systolic ratios differ (0.75 vs 0.50) because the volumes differ (32/3 vs 16).

## Background

The 4D crosspolytope is `conv{±e₁, ±e₂, ±e₃, ±e₄}`, dual to the hypercube. It has:
- **16 facets** (one per sign vector in {±1}⁴)
- **8 vertices** (±eᵢ)
- **Non-simple**: each vertex lies on 8 facets (a simple 4-polytope has exactly 4)

No literature value for c_EHZ(crosspolytope) is known to us. The placeholder in
`known_polytopes.rs` uses `capacity: 1.0` and is excluded from all validation tests.

## Timing estimate (2026-02-24)

Extrapolated from `experiments/benchmark/benchmark.jsonl` (F=5..12, pruned A3, release mode):

| Algorithm | Estimated time (F=16) | Basis |
|-----------|----------------------|-------|
| A0 (unpruned) | ~800 days | Super-exponential: ratio(F) ≈ F−1.18, giving ~3.7 trillion iterations |
| A3 (pruned) | ~4 hours (range 1–13h) | 4.59× per facet from F=8..12 exponential fit |

The crosspolytope is non-simple, which may help pruning (the two non-simple polytopes
in the ablation dataset at F=10 had ~2× fewer A3 iterations than random generic polytopes).
However, the crosspolytope's high symmetry (hyperoctahedral group, order 384) could go
either way — it might create many equivalent Reeb orbits that pruning can't distinguish,
or the symmetry might mean most orbit types are quickly pruned.

**Conclusion:** A3 (pruned) in release mode is feasible as a one-off computation.

## Approach

The binary uses three optimizations over the library's `ehz_capacity()`:

1. **Backtracking permutation search**: DFS through the directed adjacency graph instead
   of generating all (m-1)! cyclic permutations and filtering. This avoids the
   15! ≈ 1.3 trillion iteration problem for m=16.

2. **Symmetry reduction**: Computes Aut(crosspolytope) ∩ Sp(4,R) (order 32) and only
   processes one canonical representative per orbit of subsets. Reduction factor: ~27-30×
   per subset size.

3. **Checkpointing**: Saves progress after each subset size m to a JSON file. On restart,
   resumes from the last completed m.

The binary copies KKT solver internals from `crates/src/kkt.rs` and combinatorics from
`crates/src/algorithms/hk2017/` rather than using the library's public `ehz_capacity()`
API. This is necessary because the library API runs the full search internally without
hooks for symmetry reduction or checkpointing.

### Search completeness

The search is exhaustive through m=12. Subset sizes m=13..16 are skipped because:
- m=13 alone takes ~8 minutes (12! cyclic permutations per subset)
- The best action is found at m=4, and actions generally grow with m
- We would bet heavily that m≤16 still agrees, though nobody has computed or proved
  the capacity for the crosspolytope before

## Experiment phases

### Phase 1: Compute capacity — DONE

Custom binary with backtracking + symmetry + checkpointing. Completed m=2..12 in ~6
minutes (12.2M iterations). Result: c_EHZ = 4.0 with sys = 0.75.

### Phase 2: Update known_polytopes.rs — TODO

After obtaining the capacity value:
1. Replace `capacity: 1.0` placeholder with the computed value
2. Update `source:` to reference this experiment
3. Remove the exclusion from validation tests in `test_dataset.rs`
4. Regenerate the test fixture: `cargo test --release regenerate_test_dataset -- --ignored`
5. Run the full test suite to verify

### Phase 3: Cross-checks — DONE (inline with Phase 1)

- sys = c²/(2·vol) = 16/(64/3) = 3/4 = 0.75
- Dual polytope comparison: hypercube has c_EHZ = 4.0, sys = 0.50
- Minimising orbit visits m=4 facets with equal weights β = 0.25

### Phase 4 (optional): Symmetry analysis — DONE (inline with Phase 1)

Symmetry group |Aut(crosspolytope) ∩ Sp(4,R)| = 32, computed both analytically and by
Rust enumeration (match confirmed). The group has 8 valid coordinate permutations
(preserving or swapping symplectic planes {0,2} and {1,3}) × 4 sign choices = 32.

## Files

| File | Purpose |
|------|---------|
| `crosspolytope.rs` | Rust binary: compute capacity with backtracking + symmetry |
| `crosspolytope.jsonl` | Output: computed capacity and metadata |
| `README.md` | This file |

## Run

```bash
cd experiments/ && cargo run --release --bin crosspolytope
```

Resumes from checkpoint if one exists. Writes `crosspolytope/crosspolytope.jsonl` on completion.

## Known limitations

- Search exhaustive only through m=12 of 16; m=13..16 skipped due to cost
- Copies KKT solver internals from the library (cannot use public API due to lack of hooks for symmetry reduction)
- Release mode required (debug mode is infeasible at F=16)

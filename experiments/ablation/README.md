# Ablation Study: Adjacency Pruning Baseline Comparison

**Purpose:** Validate that adjacency pruning (Corollary `cor:adjacency-pruning` in the
thesis) discards no optimal orbit, and measure its speedup over the unpruned baseline.

**Status:** Phase A complete. V0 (unpruned) and V1 (pruned) agree on all 38 test
polytopes. Phase B (improvement and dismissed variants) follows in a later session.

## Files

| File | Description |
|------|-------------|
| `ablation.rs` | Rust binary: generates dataset, checks agreement, writes JSONL |
| `ablation.py` | Python analysis: agreement table, timing comparison figure |
| `ablation.tex` | Thesis subsection (LaTeX) |
| `ablation.jsonl` | Generated dataset: 76 entries (38 polytopes × 2 variants) |
| `ablation_timing.png` | Figure: V0 vs V1 timing per group and facet count |

## Dataset

38 polytopes across 3 groups, seeded at 42, h ∈ [0.5, 2.0]:

| Group | Count | F | Description |
|-------|-------|---|-------------|
| Random generic | 20 | 5–8 | 5 random 4-polytopes per facet count |
| Random Lagrangian | 15 | 6–8 | 5 random products per pair: △×△ (F=6), △×□ (F=7), □×□ (F=8) |
| Regression cases | 3 | 7–8 | Fixed polytopes with known degenerate behavior (see below) |

## Algorithm Variants

| Variant | Description |
|---------|-------------|
| V0 | Unpruned HK2017 — exhaustive search over all orderings |
| V1 | Pruned HK2017 — skips orderings with non-adjacent consecutive facets (production) |

## Key Findings

**Agreement:** V0 = V1 on all 38 polytopes. Max absolute difference: 0.0 (bitwise identical capacities).

**Timing speedup** (V0 unpruned vs V1 pruned, median ms, 5 samples each):

| F | Generic V0 | Generic V1 | Speedup | Lagrangian V0 | Lagrangian V1 | Speedup |
|---|-----------|-----------|---------|--------------|--------------|---------|
| 5 | 0.5 | 0.5 | 1.0× | — | — | — |
| 6 | 3.6 | 2.5 | 1.4× | 3.0 | 3.1 | 1.0× |
| 7 | 29.1 | 16.7 | 1.7× | 28.6 | 15.1 | 1.9× |
| 8 | 254.0 | 81.5 | 3.1× | 262.2 | 89.5 | 2.9× |

Pruning is neutral at small F (adjacency graph nearly complete) and gives ~3× speedup at F=8.

## Regression Cases

These three polytopes exercise specific code paths in the KKT solver:

| Polytope | F | Expected capacity | Result | Code path exercised |
|----------|---|------------------:|--------|---------------------|
| (3,4) θ=0° | 7 | 3√2/2 ≈ 2.121320 | ✓ | Degenerate KKT: null-space search |
| (4,4) θ=0° | 8 | 2.000000 | ✓ | Degenerate KKT: SVD gap detection |
| Hypercube | 8 | 4.000000 | ✓ | Non-degenerate: LU fast path |

The (3,4) and (4,4) cases previously returned `None` before the null-space fix. Both now
return the correct capacity via the null-space search in `kkt.rs`.

## Regeneration

```bash
cd experiments/
cargo run --bin ablation --release
# → ablation/ablation.jsonl

python3 ablation/ablation.py
# → ablation/ablation_timing.png
# Prints: agreement table, timing table, regression case results
```

Binary exits with code 1 if any variant disagrees or returns None.
Python exits with code 1 if any disagreements found in the JSONL.

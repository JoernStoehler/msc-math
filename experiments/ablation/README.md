# Ablation Study: Algorithm Variant Comparison

**Purpose:** Compare four variants of the HK2017 capacity algorithm — testing
adjacency pruning strategies and solver alternatives — on a fixed 38-polytope
dataset. Verify correctness (all variants agree) and measure speedup.

**Status:** Phase A+B complete. All four variants agree on all 38 test polytopes.

## Files

| File | Description |
|------|-------------|
| `ablation.rs` | Rust binary: generates dataset, runs all variants, checks agreement |
| `ablation.py` | Python analysis: agreement/timing/iteration tables, timing figure |
| `ablation.tex` | Thesis subsection (LaTeX) |
| `ablation.jsonl` | Generated dataset: 152 entries (38 polytopes × 4 variants) |
| `ablation_timing.png` | Figure: all variants timing per group and facet count |

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
| V1 | Undirected adjacency pruning — skips non-adjacent cycles (production) |
| V4 | Directed Reeb-flow pruning — requires vertex adjacency AND ω₀(n_i, n_j) ≤ 0 |
| V5 | SVD-only solver — same pruning as V1, but skips LU fast path |

V0 and V1 are imported from the symplectic library. V4 and V5 are self-contained
in the binary (library internals copied, not modified).

## Key Findings

**Agreement:** All four variants agree on all 38 polytopes (max absolute difference < 10⁻⁸).

**Timing** (mean ms, n=5 per group per F):

| F | V0 | V1 | V4 | V5 | V1/V0 | V4/V0 |
|---|---:|---:|---:|---:|------:|------:|
| 5 | 0.5 | 0.5 | 0.1 | 0.6 | 1.0× | 7.4× |
| 6 | 3.4 | 2.7 | 0.2 | 2.8 | 1.3× | 15.6× |
| 7 | 28.2 | 14.7 | 0.6 | 13.3 | 1.9× | 45.9× |
| 8 | 250.4 | 78.4 | 1.8 | 67.7 | 3.2× | 142.4× |

(Random generic polytopes. Lagrangian products show similar but less dramatic V4 speedup: ~40× at F=8.)

**Iteration counts** (V4 prunes ~99% of iterations at F=8):

| F | V0 iters | V1 iters | V4 iters | V1/V0 | V4/V0 |
|---|--------:|--------:|--------:|------:|------:|
| 5 | 84 | 84 | 8 | 100% | 9.5% |
| 6 | 409 | 332 | 26 | 81% | 6.5% |
| 7 | 2,365 | 1,265 | 57 | 54% | 2.4% |
| 8 | 16,064 | 5,347 | 136 | 33% | 0.8% |

## Regression Cases

| Polytope | F | Expected capacity | Result | Code path exercised |
|----------|---|------------------:|--------|---------------------|
| △×□ θ=0° | 7 | 3√2/2 ≈ 2.121320 | ✓ | Degenerate KKT: null-space search |
| □×□ θ=0° | 8 | 2.000000 | ✓ | Degenerate KKT: SVD gap detection |
| Hypercube | 8 | 4.000000 | ✓ | Non-degenerate: LU fast path |

## Regeneration

```bash
cd experiments/
cargo run --bin ablation --release
# → ablation/ablation.jsonl (152 entries)

python3 ablation/ablation.py
# → ablation/ablation_timing.png
# Prints: agreement, timing, iteration tables
```

Binary exits with code 1 if any variant disagrees or returns None.
Python exits with code 1 if any disagreements found in the JSONL.

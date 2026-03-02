# Ablation Study: Adjacency Graph Pruning

**Purpose:** Iteratively refine the adjacency graph used to prune the search
space of the HK2017 algorithm, measuring correctness and speedup at each step.

**Status:** A-axis complete. Four variants (A0–A3) agree on all 54 test polytopes.

## Files

| File | Description |
|------|-------------|
| `ablation.rs` | Rust binary: generates dataset, runs all variants, checks agreement |
| `ablation.py` | Python analysis: agreement/timing/iteration tables, timing figure |
| `ablation.tex` | Thesis subsection (LaTeX) |
| `ablation.jsonl` | Generated dataset: 216 entries (54 polytopes × 4 variants) |
| `ablation_timing.png` | Figure: all variants timing per group and facet count |

## Dataset

54 polytopes across 4 groups, seeded at 42, h ∈ [0.5, 2.0]:

| Group | Count | F | Description |
|-------|-------|---|-------------|
| Random generic | 30 | 5–10 | 5 random 4-polytopes per facet count |
| Random Lagrangian | 15 | 6–8 | 5 random products per pair: △×△ (F=6), △×□ (F=7), □×□ (F=8) |
| Non-simple | 5 | 6–9 | Bipyramids (F=6,7) and cut simplices (c=1.5,2.5,4.0) |
| Regression cases | 4 | 6–8 | Fixed polytopes: degenerate KKT, LU fast path, cut simplex (c=2.0) |

## Adjacency Graph Variants

Each variant adds a strictly stronger necessary condition for a valid orbit
transition F_i → F_j (physical direction), building on all previous conditions:

| Variant | Pruning condition for transition F_i → F_j |
|---------|---------------------------------------------|
| A0 | None (unpruned): exhaustive search over all (S, σ) |
| A1 | Vertex adjacency: F_i ∩ F_j ≠ ∅ (Corollary `cor:adjacency-pruning`) |
| A2 | Directed ω₀: A1 and ω₀(n_i, n_j) ≥ 0 (Reeb flow on F_i carries orbit toward F_j) |
| A3 | Reeb-flow feasibility: A2 and ∃x ∈ F_i ∩ F_j with x−εV_i ∈ F_i, x+εV_j ∈ F_j |

A0 is imported from the symplectic library. A1, A2, and A3 are self-contained
in the binary (library internals copied where needed).

**Sign convention:** The thesis and this README use the physical convention
(ω₀(n_i, n_j) ≥ 0 for F_i → F_j). The code uses the reversed algebraic
convention (ω₀ ≤ 0 for consecutive σ(k) → σ(k+1)) to match the Q-function.

## Key Findings

**Agreement:** All four variants agree on all 54 polytopes (max absolute difference < 10⁻⁸).

**Timing** (mean ms, random generic polytopes, n=5 per F):

| F | A0 | A1 | A2 | A3 | A2/A0 speedup |
|---|---:|---:|---:|---:|-------------:|
| 5 | 0.8 | 0.7 | 0.1 | 0.1 | ~8× |
| 6 | 4.2 | 3.1 | 0.3 | 0.3 | ~16× |
| 7 | 25.6 | 13.3 | 0.6 | 0.6 | ~44× |
| 8 | 227.4 | 71.7 | 1.7 | 1.7 | ~133× |
| 9 | 2100.1 | 507.7 | 6.6 | 6.6 | ~317× |
| 10 | 22210.7 | 1298.6 | 20.6 | 20.8 | ~1078× |

Lagrangian products show similar but less dramatic A2 speedup (~33× at F=8)
due to structured normals having more ω₀ = 0 pairs.

**A3 = A2 on simple polytopes:** On all 48 simple test polytopes, A3 provides
zero additional pruning beyond A2. Every vertex lies on exactly 4 facets,
so adjacent facets share ridges. By Ridge Sufficiency (`[cor:ridge-sufficiency]`),
ω₀ ≥ 0 alone guarantees feasibility at ridges, making the LP check redundant.

**A3 ≠ A2 on the cut simplex:** The cut simplex (non-simple, F=6) has A2=39
vs A3=33 candidates — A3 prunes 6 orderings (15% beyond A2). The sole
non-ridge pair (F₁,F₅) shares only one vertex; a blocking facet closes off
the transition. See `[ex:a3-prunes]` in ablation.tex.

## Regression Cases

| Polytope | F | Expected capacity | Result | Code path exercised |
|----------|---|------------------:|--------|---------------------|
| △×□ θ=0° | 7 | 3√2/2 ≈ 2.121320 | ✓ | Degenerate KKT: null-space search |
| □×□ θ=0° | 8 | 2.000000 | ✓ | Degenerate KKT: SVD gap detection |
| Hypercube | 8 | 4.000000 | ✓ | Non-degenerate: LU fast path |
| Cut simplex | 6 | 1.650485 | ✓ | Non-simple polytope: A2≠A3 |

## Regeneration

```bash
cd experiments/
cargo run --bin ablation --release
# → ablation/ablation.jsonl (156 entries)

python3 ablation/ablation.py
# → ablation/ablation_timing.png
# Prints: agreement, timing, iteration tables
```

Binary exits with code 1 if any variant disagrees or returns None.
Python exits with code 1 if any disagreements found in the JSONL.

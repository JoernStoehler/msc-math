# gradient-descent

Gradient ascent on the systolic ratio `sys = c_EHZ^2 / (2 vol)` for F=10 polytopes.

## Goal

Scale up the sys-optimization experiment: run gradient ascent on a much larger starting set of polytopes, including both general random polytopes and Lagrangian products. The objective is to find polytopes with high sys values, ideally approaching or exceeding sys = 1 (Viterbo's conjecture bound).

## Design

Two classes of starting polytopes, both with F=10 facets:

1. **General random polytopes** (N=500): Random normals on S^3, heights in [0.8, 1.2]. Gradient ascent uses instrumented HK2017 (exponential enumeration with adjacency pruning).

2. **Lagrangian products** (N=500): Random polygon pairs (K_q x_L K_p) with splits (3,7), (4,6), (5,5). Gradient ascent uses instrumented HK2017 (same as general polytopes) with Lagrangian-constrained normal projection.

### Why HK2017, not billiard?

Initially we planned to use the billiard algorithm for Lagrangian products (since it restricts to block-structured σ with k∈{2,3}). However, the benchmark data shows billiard is ~7x **slower** than HK2017 pruned at F=10 — the block enumeration generates ~10x more candidates (50,400 vs 4,819) than HK2017's adjacency pruning eliminates. Since HK2017 works correctly on Lagrangian products, we use it for everything.

### Instrumented HK2017

The instrumented HK2017 extracts the full KKT multipliers (nu, lambda) needed for analytical gradient computation via the envelope theorem:

- `d(sys)/d(h_k)` uses nu (multiplier for eta^T beta = 1)
- `d(sys)/d(n_k)` uses lambda (multiplier for N^T beta = 0)

### Lagrangian-constrained gradient

For Lagrangian products, the normal gradient is projected to preserve the product structure:
- Q-facet normals: zero out p-components [2,3], renormalize in q-plane
- P-facet normals: zero out q-components [0,1], renormalize in p-plane
- Heights: unconstrained

### Step bounds

Same conservative step bounds as sys-optimization:
- Vertex-crossing checks (no facet hits a non-incident vertex)
- Height positivity (all h_k > 0)
- omega_0 sign preservation for ridge-adjacent pairs

## Files

| File | Purpose |
|------|---------|
| `gradient_descent.rs` | Rust binary: polytope generation + gradient ascent |
| `kkt_instrumented.rs` | Shared instrumented KKT solver and orbit enumeration |
| `gradient_descent.py` | Python: histogram and scatter figures |
| `gradient-descent.jsonl` | Output: per-iteration trajectory data |

## Running

```bash
cd experiments/
cargo run --release --bin gradient_descent
python3 gradient-descent/gradient_descent.py
```

## Findings

(To be filled after experiment completes)

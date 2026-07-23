# Combinatorial Cell Widths

Question: how far can one move each facet normal in sampled directions before
the incidence or symplectic-sign description of the polytope changes?

`main.rs` consumes `../polytopes.jsonl`, probes directions in each facet's
four-dimensional coordinate block, and writes
`combinatorial-boundaries-profiling.jsonl`. The retained rows describe sampled
cell widths and anisotropy; they do not prove the full cell geometry.

The Rust producer and Python analyzer both rewrite tracked evidence:

```bash
cargo run -p exp-combinatorial-cells --release --bin cell-widths
uv run analyze.py
```

This packet and `../convexity/` use related perturbation scaffolding but answer
different questions. `../gradient-discontinuity/` consumes this packet's
profiling rows together with first-boundary gradient data.

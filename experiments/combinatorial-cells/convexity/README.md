# Combinatorial Cell Convexity

Question: for sampled near-boundary points that retain the starting
combinatorial type, does their midpoint retain incidence, symplectic-sign, and
transition-matrix structure?

`main.rs` consumes `../polytopes.jsonl`, repeats the packet's declared
near-boundary sampling, and writes
`combinatorial-boundaries-convexity.jsonl`.

```bash
cargo run -p exp-combinatorial-cells --release --bin cell-convexity
uv run analyze.py
```

Both commands rewrite tracked outputs. The retained sample is diagnostic
evidence only: do not infer global convexity in dual-vertex space. This packet
shares perturbation ideas with `../cell-widths/`; similarity of scaffolding
does not make their evidence roles interchangeable.

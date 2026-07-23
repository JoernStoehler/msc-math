# Repeated Boundary Crossings

Question: along selected gradient, negative-gradient, and random directions,
what happens when the stepping kernel repeatedly crosses combinatorial
boundaries within a fixed distance budget?

`main.rs` consumes `../polytopes.jsonl`, repeatedly calls the shared
`compute_step_bound_detailed` implementation, and writes
`combinatorial-boundaries-sweep.jsonl`.

```bash
cargo run -p exp-combinatorial-cells --release --bin cell-multiple-crossings
uv run analyze.py
```

Both commands rewrite tracked outputs. Construction failures are informative
censoring about numerical robustness. The retained rows do not support
uncensored transition-rate estimates, a single-boundary model, or monotonicity
of repeated `sys` improvement, and they lack a per-step
producer-selected-best-sigma identity.

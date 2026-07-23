# Exact Closed-Word Spike

Question: how does the exploratory exact endpoint-set representation classify
the fixed set and action for selected closed words on one deterministic
generated polytope?

`main.rs` generates the polytope from the CLI fixture selector and writes one
JSON row per selected sigma to stdout. Its defaults are master seed `20260605`,
`F=7`, attempt `31`, and the three source-declared words. Supplying `--sigma`
adds a word to that default panel rather than replacing it.

```bash
cargo run -p exp-dev-flow-graph --release --bin flow-graph-closed-word-spike \
  > /tmp/flow-graph-closed-word-spike.jsonl
```

This is an isolated representation spike, not the supported crate resolver.
The packet retains no canonical output. A selected classification becomes
durable evidence only after promotion to a crate regression or
`../../verification/` packet. Changes to fixture generation, endpoint
geometry, fixed-point classification, action evaluation, or the default word
panel change the question answered by scratch output.

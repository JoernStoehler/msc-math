# Flow-Graph Frontier Counts

Question: how large are the transition-pruned half-word and closed-word
frontiers before exact tube arithmetic, and does the input satisfy the exact
structural no-zero-omega support predicate?

`main.rs` reads `../../combinatorial-cells/polytopes.jsonl` by default. It
reports directed transition edges, half-cache counts by plus depth, closed
cycle and failed-split counts, and the exact structural support predicate.
This isolates combinatorial frontier size from tube construction and fixed
point arithmetic.

The packet has no canonical retained output. JSONL goes to stdout unless an
explicit output is supplied:

```bash
cargo run -p exp-dev-flow-graph --release --bin flow-graph-frontier -- \
  --max-facets 5 --output /tmp/flow-graph-frontier-smoke.jsonl
```

Scratch rows are development measurements, not thesis evidence. If a stable
runtime or counter comparison becomes the result, give it a retained
performance packet with its input and interpretation. Changes to transition
construction, word enumeration/splitting, the structural support predicate, or
the input producer change what these counts mean.

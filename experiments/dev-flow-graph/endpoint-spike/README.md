# Exact Endpoint-Set Spike

Question: is a rational-halfspace polygon representation sufficient to carry
the exact endpoint sets and operation-level empty/nonempty outcomes needed by
an exploratory flow-graph tube construction?

`main.rs` reads `../../combinatorial-cells/polytopes.jsonl` by default,
enumerates transition-pruned words, and reports representation and operation
counters. This is a measurement spike, not the supported exact implementation
in the `symplectic` crate and not a theorem verifier.

The packet has no canonical retained output. A bounded scratch run is:

```bash
cargo run -p exp-dev-flow-graph --release --bin flow-graph-endpoint-spike -- \
  --max-facets 5 --max-rows 1 \
  --output /tmp/flow-graph-endpoint-spike-smoke.jsonl
```

JSONL goes to stdout if `--output` is omitted. Changes to the endpoint-set
representation, exact polygon operations, transition-pruned word stream, or
input producer invalidate comparisons with older scratch rows. Promote a
selected stable case to crate tests or `../../verification/` if it becomes
correctness evidence.

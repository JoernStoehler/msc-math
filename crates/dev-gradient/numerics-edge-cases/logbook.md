# Edge Cases: Logbook

Split from the original `gradient-validation/` experiment (Q3 + Q4).

## How to run

```bash
cargo run -p dev-gradient --release --bin dev_numerics_edge_cases
```

Produces `gradient-correctness-q3-degeneracy.jsonl` and `gradient-correctness-q4-redundant.jsonl`.

## Contents

Q3: Near-degeneracy — gradient correctness near degenerate configurations (beta_k → 0).
Q4: Barely-cutting facets — gradient correctness when a facet barely intersects the polytope.

Note: Q4 panics partway through on KKT Q error bound (pre-existing issue, see TASKS.md `lem:q-error-bound`).

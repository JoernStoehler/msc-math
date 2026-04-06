# Basic Validation: Logbook

Split from the original `gradient-validation/` experiment (Q1 + Q2).

## How to run

```bash
cd crates/ && cargo run -p dev-gradient --release --bin dev_numerics
```

Produces `gradient-correctness-q1-generic.jsonl` and `gradient-correctness-q2-nongeneric.jsonl`.

## Contents

Q1: Generic random polytopes — first-order test on random polytopes at F=5..10.
Q2: Non-generic Lagrangian products — first-order test on LP(n,n) polytopes with exact orbit ties.

Note: Q2 panics on KKT Q error bound (pre-existing issue, see TASKS.md `lem:q-error-bound`).

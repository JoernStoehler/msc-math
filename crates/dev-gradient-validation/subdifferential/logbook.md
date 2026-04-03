# Subdifferential: Logbook

Split from the original `gradient-validation/` experiment (Q5 + Q5b + Q5c).

## How to run

```bash
cargo run -p dev-gradient-validation --release --bin dev-subdifferential
```

Produces `gradient-correctness-q5-subdiff.jsonl` and `gradient-correctness-q5b-symmetric.jsonl`.

## Contents

Q5: Orbit-switching — subdifferential prediction at near-switching boundaries.
Q5b: Exact switching boundaries — subdifferential on LP(n,n) polytopes with exact orbit ties.
Q5c: Direction-filtered subdifferential — negative result (filtering doesn't improve predictions).

Note: Q5b panics on KKT Q error bound for some polytopes (pre-existing issue, see TASKS.md `lem:q-error-bound`).

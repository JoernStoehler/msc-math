# Capacity Correctness

Question: do the retained legacy/research pruned and unpruned HK2017 and
billiard capacity implementations satisfy the checked conformality,
symplectic-invariance, monotonicity, continuity, literature, and
cross-implementation agreement properties on this retained suite? This packet
does not currently exercise those properties end to end through the production
`capacity_4d` dispatcher.

The Rust producer is `main.rs`. A full run refreshes the tracked
`correctness.jsonl`; it is not a disposable smoke command:

```bash
cargo run -p dev-capacity-validation --release --bin axioms-correctness
```

Use the retained-artifact test when checking the current code without
rewriting evidence:

```bash
cargo test -p dev-capacity-validation --release --bin axioms-correctness
```

`correctness.jsonl` is a small tracked regression fixture and is available in a
normal clone. It needs no external artifact materialization.

`--help` prints the write contract and exits without changing the artifact.

The six proposition families are finite regression and literature checks on
the recorded cases. Passing them does not prove the corresponding properties
for every convex body. After a shared solver or target-pool change, inspect
this packet together with `../all-minimum/` and `../orbit-recovery/`.

# Capacity Verification

This package is the experiment-level correctness and regression home for local
Rust validation commands, capacity algorithms, minimum-orbit result semantics,
and geometric orbit recovery. Use
`research/verification.md`, `tasks/current-state.md`, and
`tasks/planning-notes.md` for interpretation and remaining task state.

Use crate tests for fast local API regressions. Use `experiments/numerics/` for
structured numerical error-audit runs over KKT variables and predicates. Use
`experiments/performance/` only for runtime and memory measurement targets.

## Rust Command Contract

- `axioms-correctness` is a full-output producer. Running it refreshes
  `correctness/correctness.jsonl`. Do not run it as a quick smoke command unless
  intentionally refreshing that tracked evidence file. `--help` prints this
  contract and exits without writing data.
- `cargo test -p dev-capacity-validation --bin axioms-correctness --release`
  reads `correctness/correctness.jsonl` and checks the stored proposition rows.
- `axioms-all-minimum` defaults to smoke mode and writes
  `all-minimum/smoke-all-minimum.jsonl` plus
  `all-minimum/smoke-all-minimum-orbits.jsonl`. Use `--full` only when
  refreshing `all-minimum/all-minimum.jsonl` and
  `all-minimum/all-minimum-orbits.jsonl`.
- `axioms-orbit-recovery` defaults to smoke mode. It consumes
  `all-minimum/smoke-all-minimum-orbits.jsonl` and writes the smoke recovery
  outputs. Use `--full` only after the full all-minimum outputs are current.
- `experiments/verification/algorithm-comparison/README.md` documents the
  algorithm-comparison package.
- `experiments/verification/sage/README.md` documents Sage validation helpers.

Tracked JSONL files in this package are evidence artifacts. Use `--help`,
compile checks, or documented smoke mode for local command validation unless the
task explicitly asks to refresh tracked evidence.

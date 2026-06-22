# Capacity Verification

This package is the experiment-level correctness and regression home for local
Rust validation commands, capacity algorithms, minimum-orbit result semantics,
and geometric orbit recovery. Use
`research/verification.md`, `tasks/current-state.md`, and
`tasks/planning-notes.md` for interpretation and remaining task state.

Use crate tests for fast local API regressions. Use
`experiments/dev-quadratic-program/numerics-audit/` for structured QP/KKT
numerical error-audit runs over KKT variables and predicates. Use
`experiments/performance/` only for stable runtime and memory measurement
targets.

The rerunnable f64 capacity verification/audit packet is
[`../dev-quadratic-program/verification/README.md`](../dev-quadratic-program/verification/README.md).
It runs a small manifest of generated, retained, and code-owned edge-fixture
rows through the current f64 scanner and compares scan outputs against
handwritten expectations, with pass/fail status separate from full versus
limited f64 claim scope.

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
- `experiments/verification/sage/README.md` documents Sage validation helpers.

Tracked JSONL files in this package are evidence artifacts. Use `--help`,
compile checks, or documented smoke mode for local command validation unless the
task explicitly asks to refresh tracked evidence.

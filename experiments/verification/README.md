# Capacity Verification

This package contains slower, artifact-backed capacity and orbit validation.
Use crate tests for fast API regressions. Use
`../dev-quadratic-program/numerics-audit/` for structured QP/KKT numerical
error audits and `../performance/` for runtime or memory measurements.

The separate rerunnable f64 route packet is
[`../dev-quadratic-program/verification/`](../dev-quadratic-program/verification/).

## Packet inventory

This list covers every immediate child directory:

- `correctness/`: capacity properties, literature agreement, and agreement
  between current capacity implementations;
- `all-minimum/`: production of trusted minimum-orbit rows from the shared
  local-first target pool;
- `orbit-recovery/`: geometric reconstruction checks consuming the
  `all-minimum` orbit rows;
- `flow-graph-proof-risk/`: exact public-output falsifiers for flow-graph proof
  risks;
- `ch2021-six-vertex/`: exact exhaustive reproduction of the displayed
  Chaidez--Hutchings rational six-vertex example;
- `sage/`: reusable Sage comparison and validation surfaces;
- `src/`: shared target-pool, cache, run-mode, and JSONL support for this Cargo
  package.

Read the packet-local README before running a binary. Tracked JSONL and JSON
files are evidence artifacts; some full commands overwrite them.

## Producer and consumer chain

```text
shared target pool
      |
      +--> correctness
      |
      `--> all-minimum
                |
                `--> orbit-recovery
```

`all-minimum` generates and action-checks minimum sets.
`orbit-recovery` consumes those rows and checks geometric reconstruction.
The consumer declares the exact input paths in its source and README, so
repository search derives the reverse impact relation without a separate
producer-side consumer registry.

The retained full chain contains 28 selected polytopes and 469 minimum-orbit
rows; the retained recovery output reports success for all 469. These are
tolerance-based regression and reconstruction results, not an exhaustive
proof surface.

Changes to shared capacity/orbit solvers, target selection, the trusted-row
schema, or recovery tolerances should inspect all three packets. Algorithm
comparison reasoning lives in `../algorithm-comparisons.md`.

## Package commands

```bash
cargo test -p dev-capacity-validation --release

# Smoke outputs:
cargo run -p dev-capacity-validation --release --bin axioms-all-minimum
cargo run -p dev-capacity-validation --release --bin axioms-orbit-recovery
cargo run -p dev-capacity-validation --release --bin flow-graph-proof-risk
```

Use `--help` and the packet-local README before a full refresh.

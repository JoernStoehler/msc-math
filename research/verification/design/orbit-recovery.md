# Orbit Recovery Validation: Logbook

## Motivation

The older orbit-recovery experiment answered a narrow question: given one
chosen minimum-action orbit `(sigma, beta)`, can we recover a geometric orbit
that closes, stays on the boundary of `K`, remains inside `K`, and has the
correct action?

The current experiment keeps that geometric question, but now runs it on the
full trusted minimum set produced by the separate all-minimum packet.

Experiment boundary:

- `research/verification/design/all-minimum.md`: compute and validate trusted
  minimum sigma rows from polytopes;
- this packet: given those trusted rows, rebuild one-sigma KKT data and test
  geometric recovery only.

## Status

**Geometry-only split landed on 2026-04-17.** The experiment no longer
recomputes minimum sets from polytopes. It consumes the trusted minimum-orbit
rows written by `experiments/verification/all-minimum/`.

The binary has two run modes:

- default invocation writes untracked smoke outputs for infrastructure checks;
- `--full` refreshes the canonical local-first dataset.

Output surface:

- `orbit-recovery.jsonl`: one summary row per polytope
- `orbit-recovery-orbits.jsonl`: one detail row per recovered minimum orbit
- smoke variants mirror those names with `smoke-` prefixes

## How to run

```bash
cargo run -p dev-capacity-validation --release --bin axioms-all-minimum
cargo run -p dev-capacity-validation --release --bin axioms-orbit-recovery
cargo run -p dev-capacity-validation --release --bin axioms-all-minimum --full
cargo run -p dev-capacity-validation --release --bin axioms-orbit-recovery --full
uv run analyze.py
uv run analyze.py --smoke
uv run plot_orbit_recovery.py
```

Run order:

- refresh `all-minimum` first;
- then run `orbit-recovery` on the trusted rows that were just written.

### Files

| File | Role |
|------|------|
| `main.rs` | Rust binary: rebuild one-sigma KKT data and validate geometric recovery |
| `analyze.py` | Python: summary statistics from polytope-level and per-orbit outputs |
| `plot_orbit_recovery.py` | Python: worst closure/action errors by facet count |
| `orbit-recovery.jsonl` | Canonical polytope-level summary dataset |
| `orbit-recovery-orbits.jsonl` | Canonical per-minimum-orbit detail dataset |
| `smoke-orbit-recovery.jsonl` | Untracked smoke summary dataset |
| `smoke-orbit-recovery-orbits.jsonl` | Untracked smoke per-orbit detail dataset |
| `orbit_recovery_errors.png` | Error plot generated from summary rows |

## Design

### Computation policy

For every selected trusted orbit row:

1. load the row from `all-minimum-orbits.jsonl`;
2. rebuild one-sigma KKT data with `solve_orbit_sigma(...)`;
3. recover the geometric orbit with `recover_and_verify(...)`;
4. check closure / on-facet / inside-`K` / action propositions.

Important boundary:

- this packet does **not** decide which sigmas are minima;
- trusted rows come from the separate all-minimum packet;
- extra producer fields in the trusted rows are ignored unless recovery needs
  them, so nullable interval metadata does not affect the geometry-only read
  path.

### Validation procedure

Each trusted minimum orbit is checked against the same finite propositions as
the older one-best experiment:

1. **Closure error:** `||gamma(T) - gamma(0)||`
2. **On-facet error:** max breakpoint deviation from its assigned facet
3. **Inside-`K` violation:** max halfspace violation over all breakpoints
4. **Action error:** `|A(gamma) - c_EHZ(K)|`

Thresholds:

- `1e-6` for closure, on-facet, and inside-`K`
- `1e-5` for action

Polytope-level summary rows also record:

- trusted minimum-orbit count
- solution dimensions across recovered minima
- timings for KKT rebuild and recovery stages
- worst sigma-action rebuild discrepancy against the trusted row

This packet assumes the trusted multiplicities are already checked upstream in
`all-minimum`.

## Current local-first results

Canonical run on 2026-04-17:

- `28/28` selected polytopes pass
- `469/469` trusted minimum orbits rebuild and recover successfully
- worst observed sigma-action rebuild discrepancy: `2.66e-15`
- worst observed closure / on-facet / inside-`K` / action errors:
  `3.37e-11`, `4.69e-12`, `2.43e-11`, `8.08e-14`

## Current limitations

- The packet still rebuilds one-sigma KKT data from the trusted sigma rows, so
  it is not a pure serialization-deserialization smoke test.
- The packet trusts the `all-minimum` output schema and target names to stay in
  sync with the shared target-pool helper.

# Minimum-Orbit Recovery

Question: can each trusted minimum row from `../all-minimum/` be rebuilt from
its one-sigma KKT data and recovered as a geometrically valid closed orbit?

`main.rs` consumes:

- `../all-minimum/smoke-all-minimum-orbits.jsonl` in default smoke mode;
- `../all-minimum/all-minimum-orbits.jsonl` with `--full`.

It writes matching `orbit-recovery*.jsonl` summary and orbit rows in this
directory. If an input is absent, the error tells the reader to run the
corresponding `axioms-all-minimum` mode first.

Smoke:

```bash
cargo run -p dev-capacity-validation --release --bin axioms-all-minimum
cargo run -p dev-capacity-validation --release --bin axioms-orbit-recovery
```

Tracked full refresh:

```bash
cargo run -p dev-capacity-validation --release --bin axioms-all-minimum -- --full
cargo run -p dev-capacity-validation --release --bin axioms-orbit-recovery -- --full
```

Analysis and the retained diagnostic plot:

```bash
uv run analyze.py
uv run plot_orbit_recovery.py
```

The plot command rewrites the tracked `orbit_recovery_errors.png`; use it only
when that artifact refresh is intended.

The checks rebuild KKT data and call `recover_and_verify`, then test closure,
facet adherence, inside-body compliance, and action agreement. The retained
full output reports successful recovery of all 469 input minima. The strict
thresholds are `1e-6` for closure/facet/inside checks and `1e-5` for action.
These are empirical stability tolerances, not proof margins.

This packet deliberately ignores nonessential trusted-row payload fields. A
change to the producer schema, target pool, solver behavior, or recovery
tolerances must inspect both this packet and `../all-minimum/`.

# All Minimum Orbits

Question: on the package's shared local-first target pool, which solved orbit
rows fall within the packet's absolute `1e-12` action window above the reported
minimum, and do their actions agree with the packet-local scalar result?

`main.rs` builds the target pool, enumerates trusted near-minimum rows, and writes:

- `all-minimum.jsonl`: one full-run summary row per selected polytope;
- `all-minimum-orbits.jsonl`: full-run near-minimum orbit rows consumed by
  `../orbit-recovery/`;
- `smoke-all-minimum.jsonl` and `smoke-all-minimum-orbits.jsonl`: disposable
  smoke counterparts.

The default command writes smoke outputs:

```bash
cargo run -p dev-capacity-validation --release --bin axioms-all-minimum
```

`--full` refreshes both tracked full-run artifacts. Run it only when that
evidence refresh is intended:

```bash
cargo run -p dev-capacity-validation --release --bin axioms-all-minimum -- --full
```

Summarize either mode from this directory:

```bash
uv run analyze.py
uv run analyze.py --smoke
```

The retained full packet contains 28 selected polytopes and 469 trusted
near-minimum orbit rows. It checks their actions against the packet-local legacy
`capacity_auto` route (billiard for classified products, otherwise pruned HK);
it is not a complete geometric ground-truth verifier. The target pool may use
optional catalog inputs from `../orbit-recovery/polytopes.jsonl`,
`../../combinatorial-cells/polytopes.jsonl`, and
`../../sys-landscape/cache.jsonl`; missing optional catalogs contribute no
rows. Smoke mode nevertheless fails unless the assembled pool contains all five
named smoke targets, including `random_F5_seeded` from a loaded random catalog.

Changes to target selection, capacity aggregation, orbit-row schema, or action
tolerances should also inspect the consuming `../orbit-recovery/` packet.

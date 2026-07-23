# All Minimum Orbits

Question: on the package's shared local-first target pool, which solved orbit
rows attain the minimum action, and do their actions agree with the ordinary
capacity result?

`main.rs` builds the target pool, enumerates trusted minimum rows, and writes:

- `all-minimum.jsonl`: one full-run summary row per selected polytope;
- `all-minimum-orbits.jsonl`: full-run minimum-orbit rows consumed by
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
minimum-orbit rows. It checks minimum actions against `ehz_capacity`; it is not
a complete geometric ground-truth verifier. The target pool may use optional
catalog inputs from `orbit-recovery/polytopes.jsonl`,
`../../combinatorial-cells/polytopes.jsonl`, and
`../../sys-landscape/cache.jsonl`; missing optional catalogs contribute no
rows.

Changes to target selection, capacity aggregation, orbit-row schema, or action
tolerances should also inspect the consuming `../orbit-recovery/` packet.

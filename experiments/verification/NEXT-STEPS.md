# Verification Next Steps

Active objective: keep the three packets aligned after any algorithm changes in shared solver code.

1. Run in sequence: `correctness`, then `all-minimum`, then `orbit-recovery`, then the local `analyze.py` scripts for each packet.
   - `cargo run -p dev-capacity-validation --release --bin axioms-correctness`
   - `cargo run -p dev-capacity-validation --release --bin axioms-all-minimum --full`
   - `cargo run -p dev-capacity-validation --release --bin axioms-orbit-recovery --full`
   - `cd experiments/verification/<packet> && uv run analyze.py` for `all-minimum`, `orbit-recovery`; run `uv run plot_orbit_recovery.py` for `orbit-recovery` visuals.

2. Watch for boundary-sensitive breakages first:
   - schema changes between `all-minimum-orbits.jsonl` and `orbit-recovery`.
   - changes to shared polytope selection that alter reproducibility of the local-first pool.
   - tolerance regressions in closure/on-facet/inside/action checks.

3. Stop condition:
   - all three packets run successfully in full mode (or clear justification for smoke-only).
   - `all-minimum` and `orbit-recovery` datasets are coherent and parseable for the current schema.
   - `correctness` continues to pass all six propositions and literature comparisons in `correctness/correctness.jsonl`.

4. If a failure appears, inspect in this order:
   - rerun the smallest failing packet with smoke mode and compare against tracked smoke outputs,
   - confirm no command/path regression in package entrypoints under `experiments/verification/{all-minimum,orbit-recovery,correctness}`,
   - only then touch source.

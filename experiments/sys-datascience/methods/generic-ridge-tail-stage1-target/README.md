# Generic ridge-tail stage-one target evaluation

This sibling packet is the only stage-one component that evaluates targets. It
consumes exactly the frozen 200-row panel after the target-free full gate and
does not generate, substitute, or rerank candidates.

The target evaluation is complete. Its compact target-free inputs are tracked
as ordinary files and need no external materialization before analysis.

The following preflight is target-free and remains safe to reproduce:

```bash
cargo run --release --manifest-path experiments/sys-datascience/methods/generic-ridge-tail-stage1-target/Cargo.toml -- \
  preflight --out-dir experiments/sys-datascience/methods/generic-ridge-tail-stage1/artifacts/stage1
```

The irreversible command used for the completed evaluation was:

```bash
cargo run --release --manifest-path experiments/sys-datascience/methods/generic-ridge-tail-stage1-target/Cargo.toml -- \
  evaluate --out-dir experiments/sys-datascience/methods/generic-ridge-tail-stage1/artifacts/stage1 --workers 12
```

This command is historical provenance, not a current instruction. Do not run
it again and do not evaluate a new arm without a new portfolio decision. The
exact evaluator bytes used by that run are preserved at
`provenance/target-run-main.rs`; `provenance/target-run.json` binds them to the
target-free dependency commit, package lock, retained rows, and recovery
audit. The active `src/main.rs` includes later target-free manifest-repair
logic and is not claimed to be the target-run source.

The evaluator is hard-capped at 200 input rows. It reconstructs each rational
geometry, verifies the frozen `poly_id`, f64 incidence volume, ridge mean,
proxy, role, band, and rank, then calls `exp_sys_landscape::capacity_auto`
(billiard or pruned HK2017 route) and computes
`sys = capacity^2 / (2 * f64_incidence_volume)`. It records the minimizing
sigma/action and backend provenance for every row.

Artifacts are `preflight.json`, `target-rows.jsonl`,
`evaluation-manifest.json`, and `analysis.json`. Reproduce deterministic
20,000-resample percentile bootstrap intervals and Wilson intervals with:

```bash
python3 experiments/sys-datascience/methods/generic-ridge-tail-stage1-target/analyze.py \
  experiments/sys-datascience/methods/generic-ridge-tail-stage1/artifacts/stage1/target-rows.jsonl \
  experiments/sys-datascience/methods/generic-ridge-tail-stage1/artifacts/stage1/analysis.json
python3 experiments/sys-datascience/methods/generic-ridge-tail-stage1-target/verify.py \
  experiments/sys-datascience/methods/generic-ridge-tail-stage1/artifacts/stage1
```

The completed stage-one result has no `sys > 1` row. The selected-minus-
baseline mean contrast is material, while the primary 0-.1% minus .1-1%
hardening contrast is negative; the frozen 100k rule therefore stops at 10k.
This is measured transfer evidence, not a mechanism or an endpoint estimate.

# 100k Promising-Scalars Generated-Candidate Proposer Packet

Workflow-test: no.

Objective: run the durable 100k `promising-scalars` scalar-filter proposer for generated candidates. Selection must happen before `sys` computation, using cheap generated-candidate features. The run is intended to produce reviewable generated-candidate proposer evidence, not only a workflow test.

Worktree: `/workspaces/msc-math/.worktrees/thesis-datascience-integration` on branch `thesis-datascience-integration`.

Write scope:
- `experiments/sys-datascience/methods/extreme-scalar-rejection-proposer/configs/100k-promising-scalars-durable.json`
- `experiments/sys-datascience/methods/extreme-scalar-rejection-proposer/artifacts/100k-promising-scalars/`
- method README or artifact note only if needed for reproducibility and interpretation.

Primary sources:
- `experiments/sys-datascience/meta/NEXT-SESSIONS.md`
- `experiments/sys-datascience/meta/topics/generated-candidate-proposers.md`
- `experiments/sys-datascience/meta/PROMPT-TEMPLATES.md`
- `experiments/sys-datascience/methods/extreme-scalar-rejection-proposer/README.md`
- `experiments/sys-datascience/methods/extreme-scalar-rejection-proposer/configs/100k-promising-scalars.json`
- `experiments/sys-datascience/methods/extreme-scalar-rejection-proposer/check_no_target_fields.py`
- `experiments/sys-datascience/methods/extreme-scalar-rejection-proposer/artifacts/ridge-tail-1m-summary/README.md`
- `experiments/sys-datascience/methods/extreme-scalar-rejection-proposer/artifacts/ridge-tail-1m-summary/role-summary.tsv`

Run command:

```bash
cargo run --manifest-path experiments/sys-datascience/methods/extreme-scalar-rejection-proposer/Cargo.toml --release -- \
  --config experiments/sys-datascience/methods/extreme-scalar-rejection-proposer/configs/100k-promising-scalars-durable.json
```

Audit command:

```bash
uv run --script experiments/sys-datascience/methods/extreme-scalar-rejection-proposer/check_no_target_fields.py \
  experiments/sys-datascience/methods/extreme-scalar-rejection-proposer/artifacts/100k-promising-scalars/candidate-geometry-cache.jsonl \
  experiments/sys-datascience/methods/extreme-scalar-rejection-proposer/artifacts/100k-promising-scalars/candidate-feature-table.jsonl \
  experiments/sys-datascience/methods/extreme-scalar-rejection-proposer/artifacts/100k-promising-scalars/selected-candidates-before-sys.jsonl
```

Review gates:
- Stage boundary preserved: geometry/features/selection artifacts expose no target fields, and `sys` is computed only for the frozen selected-or-baseline union.
- Target-field audit passes on `candidate-geometry-cache.jsonl`, `candidate-feature-table.jsonl`, and `selected-candidates-before-sys.jsonl`.
- Artifact set is complete and reproducible.
- Selected/baseline union size is intentional and documented.
- Interpretation is bucket/per-selection matched, not pooled.
- No claim stronger than 100k generated-candidate evidence supports.

Stopping conditions:
- Stop immediately if target leakage appears in pre-target artifacts.
- Stop if cache behavior or output reuse is ambiguous.
- Stop if selected-or-baseline union is unexpectedly huge before expensive `sys` calls.
- Stop immediately if a `sys > 1` candidate appears.
- Stop after the configured 100k scale if no credible near-counterexample signal appears.

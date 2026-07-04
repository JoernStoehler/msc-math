# 100k Promising-Scalars Generated-Candidate Proposer

Status: durable generated-candidate proposer evidence packet.

Question: can the current `promising-scalars` cheap scalar rule set select promising generated random-product candidates before `sys` is computed?

Source config:
- `experiments/sys-datascience/methods/extreme-scalar-rejection-proposer/configs/100k-promising-scalars-durable.json`

Primary command:

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

The audit was run before trimming the full pre-target caches from the tracked
artifact directory. To rerun the exact audit, regenerate the full artifact
directory from the durable config or use the parked local caches if they are
still available under `/tmp/sys-ds-100k-promising-scalars-large-caches/`.

Artifacts:
- `prompt.md`: executor packet brief.
- `command-output.txt`: captured runner output.
- `resolved-run-config.json`: resolved run configuration.
- `candidate-geometry-cache.jsonl`: omitted from the tracked compact artifact
  because it is about 406 MB. Regenerate from the durable config for row-level
  geometry inspection.
- `candidate-feature-table.jsonl`: omitted from the tracked compact artifact
  because it is about 241 MB. Regenerate from the durable config for row-level
  full-feature inspection.
- `selected-candidates-before-sys.jsonl`: 1675 selected-or-baseline rows frozen before `sys`.
- `selection-plan.json`: frozen selection rules, budgets, and union counts.
- `sys-evaluation-cache.jsonl`: 1675 evaluated selected-or-baseline rows.
- `selection-summary.tsv`: per-selection and union matched summaries.
- `evaluation-report.json` and `pipeline-summary.json`: compact generated reports.
- `target-field-audit.txt`: target-field audit output.

Run notes:
- The output directory did not exist before the run, so no stale artifacts or `sys` cache were reused.
- Runner output reports `cached_rows=0` for the `sys` stage and appended 1675 rows.
- Selection happened before `sys`: geometry/features/selection artifacts were produced first, then `sys` was computed only for the frozen selected-or-baseline union.
- Target-field audit passed: `checked 3 pre-target JSONL artifact(s): no forbidden keys`.
- Retention decision: keep compact tracked provenance and result artifacts here;
  do not track the full 653 MB generated cache bundle unless future row-level
  analysis needs it enough to justify the LFS cost.

Interpretation boundary:
- This is 100k generated random-product candidate evidence for the configured `promising-scalars` rule set, not a theorem and not evidence outside this generator/configuration.
- Interpret comparisons through the per-selection and matched-baseline rows in `selection-summary.tsv`; avoid pooled proposer claims.
- The selected-or-baseline union size is intentional for the current rule set: 30 selection sets, 1200 selected rows summed over sets, 485 unique selected rows, 1195 unique baseline rows, and 1675 unique selected-or-baseline rows.
- No evaluated candidate had `sys > 1`; `evaluation-report.json` reports maximum evaluated `sys` 0.867546058507634 and maximum selected `sys` 0.867546058507634.
- The run supports the claim that the current scalar rules can be evaluated as pre-`sys` generated-candidate proposers at 100k scale. It does not by itself support a near-counterexample claim.

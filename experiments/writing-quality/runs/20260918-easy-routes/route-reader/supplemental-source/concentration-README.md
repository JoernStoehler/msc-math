# 100k Ridge-Concentration Independent Validation

Status: independently generated and reviewed packet. The rule, decision
criterion, config, and analysis contract were frozen before target evaluation
on 2026-07-11. See `review.md` for the bounded verdict.

## Question And Value

Does low symplectic two-face maximum share add independent pre-target
enrichment inside the already supported low ridge-area-sum tail on a new
random-product candidate sample?

This packet was selected because it can change the retained proposer claim and
the prior 100k pipeline shows it is cheap locally. Another retained-table model,
blind scale-up, or same-rule replication cannot answer the incremental
concentration question.

## Frozen Design

Source config:
`../../../configs/100k-ridge-concentration-validation.json`.

- independent seed `1618033`;
- 10,000 candidates in each of the 10 retained product buckets;
- height interval `[0.8, 1.2]`;
- stage 1: lowest 1% per bucket by
  `ridge_symp_area_sum_over_volume_sqrt`;
- cascade: lowest 50% by `ridge_symp_area_max_share` within stage 1;
- emit and evaluate both the full stage-1 comparator and the cascade;
- one deterministic disjoint bucket-matched baseline per selection;
- select and audit before computing `sys`.

Call the concentration add-on descriptively validated only if the cascade mean
`sys` exceeds the complementary upper half of stage 1 overall and in at least 7
of 10 product buckets. Any evaluated `sys > 1` is an immediate escalation. The
criterion is descriptive generated-candidate validation, not a significance
test or mechanism claim.

## Commands

```bash
cargo run --manifest-path experiments/sys-datascience/methods/extreme-scalar-rejection-proposer/Cargo.toml --release -- \
  --config experiments/sys-datascience/methods/extreme-scalar-rejection-proposer/configs/100k-ridge-concentration-validation.json \
  --stage geometry

cargo run --manifest-path experiments/sys-datascience/methods/extreme-scalar-rejection-proposer/Cargo.toml --release -- \
  --config experiments/sys-datascience/methods/extreme-scalar-rejection-proposer/configs/100k-ridge-concentration-validation.json \
  --stage features

cargo run --manifest-path experiments/sys-datascience/methods/extreme-scalar-rejection-proposer/Cargo.toml --release -- \
  --config experiments/sys-datascience/methods/extreme-scalar-rejection-proposer/configs/100k-ridge-concentration-validation.json \
  --stage selection

uv run --script experiments/sys-datascience/methods/extreme-scalar-rejection-proposer/check_no_target_fields.py \
  experiments/sys-datascience/methods/extreme-scalar-rejection-proposer/artifacts/100k-ridge-concentration-validation/candidate-geometry-cache.jsonl \
  experiments/sys-datascience/methods/extreme-scalar-rejection-proposer/artifacts/100k-ridge-concentration-validation/candidate-feature-table.jsonl \
  experiments/sys-datascience/methods/extreme-scalar-rejection-proposer/artifacts/100k-ridge-concentration-validation/selected-candidates-before-sys.jsonl

cargo run --manifest-path experiments/sys-datascience/methods/extreme-scalar-rejection-proposer/Cargo.toml --release -- \
  --config experiments/sys-datascience/methods/extreme-scalar-rejection-proposer/configs/100k-ridge-concentration-validation.json \
  --stage sys

cargo run --manifest-path experiments/sys-datascience/methods/extreme-scalar-rejection-proposer/Cargo.toml --release -- \
  --config experiments/sys-datascience/methods/extreme-scalar-rejection-proposer/configs/100k-ridge-concentration-validation.json \
  --stage reports

uv run --script experiments/sys-datascience/methods/extreme-scalar-rejection-proposer/analyze_cascade_validation.py \
  experiments/sys-datascience/methods/extreme-scalar-rejection-proposer/artifacts/100k-ridge-concentration-validation
```

The final compact packet retains the frozen selection, evaluated target cache,
generated reports, incremental validation, command output, and target-field
audit. The large deterministic geometry/feature caches may be omitted after
their hashes and audit are recorded; regenerate them from the config when
row-level feature or geometry inspection is needed.

## Result

The frozen descriptive criterion passed: the cascade improved mean `sys` over
the stage-1 complement overall and in at least the predeclared number of product
buckets. No evaluated candidate had `sys > 1`. Use
`incremental-validation.tsv` and `validation-verdict.json` for detailed
generated results; do not copy their metric rows into control prose.

## Claim Boundary

Allowed: generated-candidate evidence for the exact frozen cascade and source
configuration, including a negative conclusion if the criterion fails.

Prohibited: a geometric mechanism, transfer beyond this random-product
generator, a calibrated probability of `sys > 1`, or a claim that in-table
prediction independently validates candidates.

# standard-baseline-p2

## Research Question

Does the P2 set of missing ordinary retained-table baselines change the
standard-method story for the retained random/product method table?

P2 covers:

- lasso regression;
- elastic-net regression;
- gradient boosting regression;
- elastic-net logistic high-tail classification;
- gradient boosting high-tail classification;
- feature-family ablation for combinatorial-count and ridge symplectic-area
  feature families.

This packet does not validate a generated-candidate proposer. All evaluated
rows already have `sys` computed.

## Method

Use trusted random/product rows from the retained prepared table. Use active
invariant numeric features from `methods/_shared/random_only.py`; do not use
source/provenance metadata as model input. Split with grouped holdout by
`capacity_source:facet_count`, matching the existing prediction-ranking packet.

The high-tail classification target is `sys` above the train-split quantile.
The default is the train top 10% cutoff. Score metrics are reported on held-out
groups. Top-score rows mean the held-out rows with the largest model score;
this is finite retained-table ranking, not generated-candidate selection.

Feature-family ablations fit the same gradient boosting models on:

- all invariant features;
- combinatorial-count features only;
- ridge symplectic-area features only;
- other invariant features only;
- all features except combinatorial-count features;
- all features except ridge symplectic-area features.

## Command

Build the current-schema input from the tracked canonical producer artifacts.
The prepared output may be scratch because it is a deterministic derived table:

```bash
TABLES_DIR="$(mktemp -d /tmp/sys-ds-p2-current-full.XXXXXX)"
experiments/polytope-invariant-table/build-random-only-slice.sh full \
  "$TABLES_DIR"
```

The recorded input is source-reproducible from these registered shared
producer objects:

- `experiments/polytope-datasets/random.jsonl`: sha256
  `a21ac62ba5c9496ef631d3cce74e8b663764516b76e9d4725f1e517d8dd55f9f`;
- `experiments/polytope-datasets/random-product.jsonl`: sha256
  `66bf82010e92e0f26b0df226f4e6c0eef05d21eb22a0967c7f669530f6545736`.

The derived tables must have the hashes recorded under **Current Status**
below. This contract avoids tracking a second copy of the 14,336-row prepared
table solely for P2 while removing any dependency on a surviving `/tmp` path.

Analysis command:

```bash
OPENBLAS_NUM_THREADS=1 OMP_NUM_THREADS=1 MKL_NUM_THREADS=1 \
  uv run --script experiments/sys-datascience/methods/standard-baseline-p2/analyze.py \
  --tables-dir "$TABLES_DIR"
```

The registered `experiments/polytope-invariant-table/` table has hash
`607c8731fa03d190d497edc3e8f1b4cca88f7d238260cce527680f568bc33d59` and is
missing six active invariant ridge columns. Use the rebuilt current-schema
table above, or publish and register a refreshed prepared-table snapshot before
rerunning P2 without `--tables-dir`.

## Artifacts

`analyze.py` writes:

- `artifacts/summary.json`;
- `artifacts/regression-metrics.tsv`;
- `artifacts/high-tail-classification-metrics.tsv`;
- `artifacts/feature-family-ablation.tsv`;
- `artifacts/linear-top-coefficients.tsv`;
- `artifacts/command.txt`.

## Current Status

P2 has a reviewed current-schema full retained-table run. A fresh rebuild from
the tracked producer objects was checked during Phase 0 normalization.

Prepared table:

- `polytope-table.jsonl`: 14,336 rows, sha256
  `49825d7636246f71f4ebd419cf0ccbc86e39e6b7f43d4b03e889bb85e4887aea`;
- `polytope-provenance-table.jsonl`: 14,336 rows, sha256
  `6ff88a5accce9a7ec7e5a494107350b0974b2ce0268ea44caae36a18a7494ef2`;
- feature count: 45 active invariant numeric features, split as 27
  combinatorial-count features and 18 ridge symplectic-area features;
- grouping: `capacity_source:facet_count`;
- maximum observed `sys`: `0.86258589584944`;
- rows with `sys > 1`: 0.

Use the generated artifacts for train/test counts, cutoffs, model metrics,
feature-family ablations, and coefficient rows. The reviewed interpretation is
only that P2 closes the named missing retained-table standard baselines without
producing a positive row or a validated generated-candidate proposer. Ridge
symplectic-area features carry most of the held-out association under this
split, but that is diagnostic and does not establish a geometric mechanism.

## Interpretation Rules

- Treat this as retained-table standard-method coverage only.
- Do not use P2 to claim a generated-candidate proposer.
- Do not use P2 to claim arbitrary random-distribution coverage.
- Compare P2 against `prediction-ranking/` and `tail-rule-mining/` before
  changing thesis wording.
- If P2 finds a strong new interaction, create a separate generated-candidate
  design packet before any proposer wording.

# tail-rule-mining

## Research Question

Can a shallow, interpretable rule learned from active invariant features isolate
the upper `sys` tail of the trusted random/product sample better than source,
stratum, or generator-provenance labels alone?

This is an in-table diagnostic. It is not a validated candidate proposer,
because train and test rows already have computed `sys` values.

## Current Schema

The active `polytope-table.jsonl` is invariant-only. This packet consumes:

- combinatorial invariant columns;
- ridge symplectic two-face area summaries normalized by `volume.sqrt()`;
- categorical controls built from `capacity_source`, `facet_count`, product
  bucket, product bounce count, and sample height range.

The old Euclidean size/spread, all-pair omega, omega-matrix,
transition-graph, `capacity`, and `volume` method inputs are not active. Their
historical results live in git history rather than this README.

## Method

Train shallow decision-tree classifiers for three labels:

- top decile: `sys` at or above the full-table 90th percentile;
- top five percent: `sys` at or above the full-table 95th percentile;
- top one percent: `sys` at or above the full-table 99th percentile.

Feature sources:

- `invariant_features`: all active numeric invariant covariates selected by
  the shared random-only selector;
- `ridge_symp_area_only`: ridge symplectic-area summaries only;
- `combinatorial_invariants_only`: counts, simplicity, incidence summaries,
  ridge sizes, facet-neighbor summaries, and edge density;
- `strata_only`: `capacity_source`, `facet_count`, and product bucket;
- `generator_provenance_only`: `capacity_source`, product bounce count, and
  sample height range.

Rows are split by `capacity_source:facet_count`, matching the other
random-only method packets. Outputs report grouped-holdout precision, recall,
enrichment over base rate, leaf rules, stability summaries, coarse baselines,
and a permutation-null diagnostic when requested.

## Command

```bash
uv run --script experiments/sys-datascience/methods/tail-rule-mining/analyze.py
```

Smoke against a scratch prepared table:

```bash
uv run --script experiments/sys-datascience/methods/tail-rule-mining/analyze.py \
  --tables-dir /tmp/ds-prepare-invariant-smoke \
  --out-dir /tmp/ds-method-tail-smoke \
  --stability-runs 0 \
  --permutations 0 \
  --min-bucket-rows 2
```

## Generated Artifacts

- `summary.json`
- `leaf-rules.tsv`
- `stability-runs.tsv`
- `stability-split-features.tsv`
- `bucket-interpretation-diagnostics.tsv`

These artifacts are generated outputs. Regenerate them from the current table;
do not patch-edit them by hand.

## Interpretation Contract

Use this packet to ask whether simple invariant covariates carry tail signal
beyond coarse source/stratum controls. Do not claim that a leaf rule is a
mechanism, theorem, or unevaluated-row proposer without a separate generated
candidate experiment.

Feature definitions for the active numeric inputs are owned by
`../../prepare/invariant_features.rs` and
`../../prepare/features_face_symplectic.rs`.

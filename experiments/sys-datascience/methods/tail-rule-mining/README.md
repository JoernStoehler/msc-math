# tail-rule-mining

## Research Question

Root search question:

> Can cheap invariant features guide search effort toward higher `sys`, possibly
> enough to justify a later generated-candidate proposer test?

This packet does **not** validate a generated-candidate proposer and does not
estimate top-`1e-6` cheap-tail behavior or a `sys > 1` hit probability.

Retained-table question answered here:

> In the already evaluated random/product table, do current invariant prepared
> features enrich for the high observed `sys` tail, and do ridge symplectic
> two-face summaries survive source/facet/provenance and combinatorial controls?

The high-tail labels are quantiles of the retained evaluated table or retained
evaluation scope. Every evaluated row already has `sys` computed, so this is a
retained-table diagnostic, not unevaluated-candidate validation.

## Feature Set

This method uses only current active invariant prepared features plus
categorical controls. The exact numeric feature tuple is
`ACTIVE_INVARIANT_NUMERIC_FEATURES` in
`experiments/sys-datascience/methods/_shared/random_only.py`.

The producer definitions for ridge symplectic area fields are in
`experiments/sys-datascience/prepare/features_face_symplectic.rs`, assembled
into method-facing rows by
`experiments/sys-datascience/prepare/invariant_features.rs` and
`experiments/sys-datascience/prepare/rows.rs`.

Active invariant numeric families:

- combinatorial counts and incidence summaries;
- ridge symplectic two-face area summaries divided by `sqrt(volume)`;
- ridge symplectic area concentration summaries such as max/top-3 share;
- ridge symplectic small-area fractions at normalized thresholds `1e-3`,
  `1e-2`, and `1e-1`;
- ridge symplectic entropy, effective face count, and normalized entropy.

Controls:

- `capacity_source`;
- `facet_count`;
- product bucket;
- product bounce count;
- sample height range.

Non-invariant coordinate features are intentionally not used: raw coordinate
norms, pairwise Euclidean distances, singular values, Euclidean facet/ridge
volumes, raw omega matrices, and raw `volume` are not method inputs.

## Method

The method is an adversarial retained-table filter-overlap screen. It defines
cheap filters from invariant features and controls, then measures how much each
filter overlaps the high-`sys` tail. Single-column filters are the
one-feature special case; shallow tree leaves are conjunction filters such as
`X1 <= a and X2 > b`.

Before analysis, the script checks that:

- `poly_id` is present and unique;
- `sys` is finite;
- `capacity_source` is a current random/product source;
- `facet_count` is an integer;
- every retained polytope has exactly one provenance row;
- provenance fields needed by source/product controls are present;
- required current invariant features are present and finite.

Outputs:

1. **Single-feature tail filters.** For each active invariant numeric feature,
   the script evaluates low/high 10%, 15%, 20%, and 30% filters. For one-hot
   categorical controls, it evaluates category membership rules. Holdout rows
   use thresholds and directions frozen on a disjoint hash split.

2. **Shallow tree filters.** Grouped-holdout decision trees test multi-feature
   conjunction filters over each feature family and control family.

3. **Feature-set predictive power.** Balanced logistic models and shallow
   trees compare predeclared feature tuples on a grouped train/test split.
   Output columns distinguish the train-score threshold fraction from the
   actual held-out selected fraction.

4. **Retained-table budget sanity.** Fitted scores rank held-out rows, and
   finite retained-table prefixes such as 0.2%, 0.5%, and 1% are checked for
   whether they contain the best observed held-out `sys` rows. This is not a
   top-`1e-6` generated-candidate statement.

5. **Feature attribution and redundancy.** For each active invariant numeric
   feature, raw single-feature enrichment is compared with residualized
   enrichment after source/facet/provenance controls, strongest combinatorial
   controls, strongest other-family invariant controls, and strongest non-self
   invariant controls.

## Commands

Build a current invariant random/product prepared table:

```bash
PRODUCE_DIR=/workspaces/msc-math/experiments/sys-datascience/produce \
  experiments/sys-datascience/prepare/build-random-only-slice.sh full \
  /tmp/sys-ds-tail-invariant-current-full
```

Development smoke run:

```bash
uv run --script experiments/sys-datascience/methods/tail-rule-mining/analyze.py \
  --tables-dir /tmp/sys-ds-tail-invariant-current-full \
  --out-dir /tmp/sys-ds-tail-invariant-smoke \
  --max-rows 1024 \
  --max-filter-features 64 \
  --stability-runs 0 \
  --permutations 0
```

Full local diagnostic:

```bash
rm -rf /tmp/sys-ds-tail-invariant-analysis
uv run --script experiments/sys-datascience/methods/tail-rule-mining/analyze.py \
  --tables-dir /tmp/sys-ds-tail-invariant-current-full \
  --out-dir /tmp/sys-ds-tail-invariant-analysis \
  --stability-runs 0 \
  --permutations 0
```

Run production/full data generation only after smoke validation. Generated
artifacts are not tracked in this method directory; regenerate them from the
current prepared table.

## Current Full Run

Production prepare command run locally:

```bash
rm -rf /tmp/sys-ds-tail-invariant-current-full /tmp/sys-ds-tail-invariant-analysis
PRODUCE_DIR=/workspaces/msc-math/experiments/sys-datascience/produce \
  experiments/sys-datascience/prepare/build-random-only-slice.sh full \
  /tmp/sys-ds-tail-invariant-current-full
```

Production analysis command run locally:

```bash
OPENBLAS_NUM_THREADS=1 OMP_NUM_THREADS=1 MKL_NUM_THREADS=1 \
  uv run --script experiments/sys-datascience/methods/tail-rule-mining/analyze.py \
  --tables-dir /tmp/sys-ds-tail-invariant-current-full \
  --out-dir /tmp/sys-ds-tail-invariant-analysis \
  --stability-runs 0 \
  --permutations 0
```

Prepared table:

- path: `/tmp/sys-ds-tail-invariant-current-full`;
- `polytope-table.jsonl`: 14,336 rows, sha256
  `49825d7636246f71f4ebd419cf0ccbc86e39e6b7f43d4b03e889bb85e4887aea`;
- `polytope-provenance-table.jsonl`: 14,336 rows, sha256
  `6ff88a5accce9a7ec7e5a494107350b0974b2ce0268ea44caae36a18a7494ef2`;
- source counts: 4,096 generic random rows and 10,240 random product rows;
- emitted polytope fields: 51;
- raw `volume` is not emitted by the current invariant method-facing table;
- maximum observed `sys`: `0.86258589584944`;
- rows with `sys > 1`: 0.

Analysis output:

- path: `/tmp/sys-ds-tail-invariant-analysis`;
- preflight:
  `passed_current_invariant_schema_structural_provenance_feature_check`;
- `summary.json` records feature counts, artifact row counts, preflight status,
  and candidate-proposer disposition for the run;
- permutations and stability reruns were disabled for this local production
  pass.

### Reading The Run

Use the generated TSV/JSON artifacts as the source of truth for detailed
rankings, effect sizes, budget-prefix behavior, and attribution rows. This
README records the run provenance and interpretation boundaries only.

Stable facts from the recorded prepared table:

- exact table: `/tmp/sys-ds-tail-invariant-current-full`;
- maximum observed `sys`: `0.86258589584944`;
- rows with `sys > 1`: 0.

Stable boundaries for this run:

- all high-tail labels are retained-table labels; every row being evaluated
  already has `sys`;
- `feature-set-predictive-power.tsv` is the source for held-out predictive
  metrics by feature set and model;
- `single-feature-filter-holdout-top-by-scope-label.tsv` is the source for
  frozen-threshold single-feature filters;
- `retained-table-budget-sanity.tsv` is the source for finite retained-table
  prefix behavior;
- `feature-attribution-redundancy.tsv` is the source for residualized
  attribution checks;
- no artifact in this packet validates a generated-candidate proposer,
  estimates top-`1e-6` cheap-feature behavior, or estimates a `sys > 1` hit
  probability.

## Artifact Contract

`analyze.py` writes:

- `summary.json`;
- `tree-filter-leaves.tsv`;
- `bucket-interpretation-diagnostics.tsv`;
- `single-feature-filter-leaderboard.tsv`;
- `single-feature-filter-holdout-rules.tsv`;
- `single-feature-filter-holdout-top-by-scope-label.tsv`;
- `feature-set-predictive-power.tsv`;
- `retained-table-budget-sanity.tsv`;
- `feature-attribution-redundancy.tsv`;
- `stability-runs.tsv` when `--stability-runs > 0`;
- `stability-split-features.tsv` when `--stability-runs > 0`.

Use `single-feature-filter-leaderboard.tsv` as descriptive retained-table
ranking only. Use `single-feature-filter-holdout-rules.tsv` when discussing
per-filter thresholds whose direction and threshold were fixed on a disjoint
train split. Do not present post-hoc best rows as multiple-comparison-corrected
validation.

Use `feature-set-predictive-power.tsv` for the question "does this invariant
feature tuple predict the retained high tail?" Compare rows using
`actual_test_selection_fraction` when budget size matters. Use
`retained-table-budget-sanity.tsv` only as a finite-table budget sanity check.
Use `feature-attribution-redundancy.tsv` for the attribution question "does this
invariant feature still enrich the retained high tail after residualizing
against these controls?"

## Current Status

The method has been rewired to the current invariant schema, the prepare stage
emits additional invariant ridge symplectic-area distribution summaries, and
the local full current-schema invariant run above has been recorded.

## Interpretation Rules

- If no `sys > 1` rows are present, state the exact table and maximum `sys`;
  do not generalize beyond that table.
- Do not present retained-table enrichment as generated-candidate validation.
- Do not extrapolate retained-table 0.1%, 0.2%, 0.5%, or 1% prefix behavior to
  top-`1e-6` generated-candidate behavior.
- Treat source/facet/provenance-only lift as generator/debugging information,
  not a geometry mechanism.
- Check `feature-attribution-redundancy.tsv` before making any mechanism or
  construction-hint statement from a raw feature filter.

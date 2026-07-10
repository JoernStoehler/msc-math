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

Current-schema table rebuild used for the recorded run:

```bash
rm -rf /tmp/sys-ds-p2-current-full
experiments/sys-datascience/prepare/build-random-only-slice.sh full \
  /tmp/sys-ds-p2-current-full
```

Recorded analysis command:

```bash
OPENBLAS_NUM_THREADS=1 OMP_NUM_THREADS=1 MKL_NUM_THREADS=1 \
  uv run --script experiments/sys-datascience/methods/standard-baseline-p2/analyze.py \
  --tables-dir /tmp/sys-ds-p2-current-full
```

The default `prepare/` LFS table in this worktree has hash
`607c8731fa03d190d497edc3e8f1b4cca88f7d238260cce527680f568bc33d59` and is
missing six active invariant ridge columns. Use the rebuilt current-schema
table above, or refresh the in-place prepared LFS artifact before rerunning P2
without `--tables-dir`.

## Artifacts

`analyze.py` writes:

- `artifacts/summary.json`;
- `artifacts/regression-metrics.tsv`;
- `artifacts/high-tail-classification-metrics.tsv`;
- `artifacts/feature-family-ablation.tsv`;
- `artifacts/linear-top-coefficients.tsv`;
- `artifacts/command.txt`.

## Current Status

P2 has a current-schema full retained-table run.

Prepared table:

- path: `/tmp/sys-ds-p2-current-full`;
- `polytope-table.jsonl`: 14,336 rows, sha256
  `49825d7636246f71f4ebd419cf0ccbc86e39e6b7f43d4b03e889bb85e4887aea`;
- `polytope-provenance-table.jsonl`: 14,336 rows, sha256
  `6ff88a5accce9a7ec7e5a494107350b0974b2ce0268ea44caae36a18a7494ef2`;
- feature count: 45 active invariant numeric features, split as 27
  combinatorial-count features and 18 ridge symplectic-area features;
- maximum observed `sys`: `0.86258589584944`;
- rows with `sys > 1`: 0.

Grouped holdout:

- grouping: `capacity_source:facet_count`;
- train/test rows: 8,704 / 5,632;
- train-derived top-10% high-tail cutoff: `0.5697652833472453`.

Main retained-table results:

- lasso regression: `R^2 = 0.6110233222992789`, MAE
  `0.10476866676771376`;
- elastic-net regression: `R^2 = 0.620742184420513`, MAE
  `0.10287766814738514`;
- histogram gradient boosting regression: `R^2 = 0.8784238138483205`, MAE
  `0.05203365520874717`;
- elastic-net logistic high-tail classifier: ROC-AUC
  `0.8737068187147978`, average precision `0.55296791481307`;
- histogram gradient boosting high-tail classifier: ROC-AUC
  `0.9359727673037687`, average precision `0.7034314626146508`.

Feature-family ablation says the held-out signal is almost entirely in the
ridge symplectic-area feature family under this split:

- ridge-only gradient boosting regression: `R^2 = 0.8872276246501958`;
- combinatorial-only gradient boosting regression: `R^2 =
  0.04314696727456602`;
- ridge-only high-tail classifier average precision:
  `0.7054296104152254`;
- combinatorial-only high-tail classifier average precision:
  `0.19185015669933783`.

Use generated artifacts for detailed metrics and coefficient rows; this README
records only run provenance and interpretation boundaries.

## Interpretation Rules

- Treat this as retained-table standard-method coverage only.
- Do not use P2 to claim a generated-candidate proposer.
- Do not use P2 to claim arbitrary random-distribution coverage.
- Compare P2 against `prediction-ranking/` and `tail-rule-mining/` before
  changing thesis wording.
- If P2 finds a strong new interaction, create a separate generated-candidate
  design packet before any proposer wording.

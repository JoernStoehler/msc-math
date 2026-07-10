# standard-baseline-p2 Review

Review date: 2026-07-08.

Verdict: accepted as a retained-table P2 standard-baseline packet. It can update
method-surface coverage after parent synthesis. It should not be used as thesis
evidence without the claim boundary below.

## Motivating Question

Does the compact P2 set of missing ordinary retained-table methods change the
current method-table story?

Answer: no positive row or candidate-proposer appears. P2 adds standard-method
coverage and confirms strong retained-table signal, mainly from ridge
symplectic-area features, but it remains in-table evidence.

## Source And Reproducibility Checks

- Script: `analyze.py`.
- Artifacts: `artifacts/summary.json`, `regression-metrics.tsv`,
  `high-tail-classification-metrics.tsv`, `feature-family-ablation.tsv`,
  `linear-top-coefficients.tsv`, `command.txt`.
- Input table: `/tmp/sys-ds-p2-current-full`, built from retained producer
  files because the in-place prepared LFS table is stale relative to the active
  invariant schema.
- Input hashes:
  - `polytope-table.jsonl`:
    `49825d7636246f71f4ebd419cf0ccbc86e39e6b7f43d4b03e889bb85e4887aea`;
  - `polytope-provenance-table.jsonl`:
    `6ff88a5accce9a7ec7e5a494107350b0974b2ce0268ea44caae36a18a7494ef2`.
- Re-run completed cleanly after fixing the scikit-learn 1.9 logistic API and
  convergence warning.

## Claim Boundary

Allowed update:

- P2 covers lasso/elastic-net, gradient boosting, high-tail classification, and
  feature-family ablation for the retained random/product table.
- It supports the standard-method coverage story for the retained method table.
- It reinforces that ridge symplectic-area features carry most of the
  held-out in-table signal under this split.

Not allowed:

- no generated-candidate proposer claim;
- no arbitrary random-distribution claim;
- no calibrated hit-rate or density claim;
- no claim that all data-science methods have been exhausted.

## Remaining Risk

The packet uses one grouped holdout split and fixed hyperparameters. That is
enough for the P2 coverage role because the result is not a positive proposer
claim. Reopen only if thesis text wants stronger comparative model language or
if P2 becomes the basis for a generated-candidate design.
